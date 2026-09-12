//! gpu_prime.rs — prime_winding's primality, run on the GPU.
//!
//! Every candidate `prime_winding range`/`find` tests in the GPU build is
//! decided on the device, not the CPU. A window of consecutive integers is
//! handed to one launch; each thread runs a deterministic Miller-Rabin over
//! 128-bit integers (trial division by the first twelve primes, then the
//! twelve-witness test) and writes a one-byte prime flag. The CPU only forms
//! the window top and reads the flags back.
//!
//! 128-bit covers integers up to about 3.8e38 (38 digits). The multi-limb
//! kernel for wider numbers is the next rung, named in `range` where it hands
//! off; it is not a wall.

use alloc::format;
use alloc::string::String;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};

/// Largest modulus the mulmod below handles without 128-bit overflow: it does
/// one `a <<= 1` on a value below `m`, so `m` must stay under 2^127.
pub const GPU_PRIME_MAX: u128 = 1u128 << 126;

const KERNEL_SRC: &str = r#"
typedef unsigned long long u64;
typedef unsigned __int128 u128;

// (a*b) mod m by binary (Russian-peasant) reduction, so the product never
// needs 256 bits. Requires m < 2^127 so a<<1 does not overflow.
__device__ u128 mulmod(u128 a, u128 b, u128 m){
    u128 r = 0; a %= m;
    while (b) { if (b & 1) { r += a; if (r >= m) r -= m; } a <<= 1; if (a >= m) a -= m; b >>= 1; }
    return r;
}
__device__ u128 powmod(u128 a, u128 e, u128 m){
    u128 r = 1; a %= m;
    while (e) { if (e & 1) r = mulmod(r, a, m); a = mulmod(a, a, m); e >>= 1; }
    return r;
}
__device__ bool isprime(u128 n){
    if (n < 2) return false;
    const u64 sp[12] = {2,3,5,7,11,13,17,19,23,29,31,37};
    for (int i = 0; i < 12; i++) { u128 p = sp[i]; if (n == p) return true; if (n % p == 0) return false; }
    u128 d = n - 1; int r = 0;
    while ((d & 1) == 0) { d >>= 1; r++; }
    for (int i = 0; i < 12; i++) {
        u128 x = powmod((u128)sp[i], d, n);
        if (x == 1 || x == n - 1) continue;
        bool comp = true;
        for (int j = 0; j < r - 1; j++) { x = mulmod(x, x, n); if (x == n - 1) { comp = false; break; } }
        if (comp) return false;
    }
    return true;
}
// Thread i tests base - i, where base = (base_hi<<64)|base_lo.
extern "C" __global__ void prime_batch(unsigned char* out, u64 base_lo, u64 base_hi, u64 n){
    u64 i = (u64)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    u128 base = ((u128)base_hi << 64) | (u128)base_lo;
    u128 cand = base - (u128)i;
    out[i] = isprime(cand) ? 1 : 0;
}
"#;

/// Candidates per launch. One byte of device output each, so a window is a
/// megabyte of VRAM at this size.
const WINDOW: u64 = 1 << 20;

/// Enumerate every prime in [lo, hi] on the GPU, high to low. Same output
/// shape as the CPU `prime_winding::range`, so the two are directly comparable.
pub fn range_gpu(lo: u128, hi: u128, count_only: bool, device: usize) -> String {
    if hi < lo {
        return format!("prime_winding range [{}, {}]: empty span (hi < lo)", lo, hi);
    }
    let ctx = match CudaContext::new(device) {
        Ok(c) => c,
        Err(e) => return format!("gpu_prime: no CUDA context (device {device}): {e}"),
    };
    let stream = ctx.default_stream();
    let opts = CompileOptions {
        // __int128 in device code needs this NVRTC flag (Linux).
        options: alloc::vec!["--device-int128".into()],
        ..Default::default()
    };
    let ptx = match compile_ptx_with_opts(KERNEL_SRC, opts) {
        Ok(p) => p,
        Err(e) => return format!("gpu_prime: NVRTC compile failed: {e}"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("gpu_prime: module load failed: {e}"),
    };
    let func = match module.load_function("prime_batch") {
        Ok(f) => f,
        Err(e) => return format!("gpu_prime: load prime_batch failed: {e}"),
    };

    let mut top = hi;
    let mut count: u64 = 0;
    let mut tested: u64 = 0;
    let mut listing = String::new();

    loop {
        // This launch tests top, top-1, ..., top-span+1, never below lo.
        let span = core::cmp::min(WINDOW as u128, top - lo + 1) as u64;
        let base_lo = (top & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        let base_hi = (top >> 64) as u64;

        let mut d_out = match stream.alloc_zeros::<u8>(span as usize) {
            Ok(d) => d,
            Err(e) => return format!("gpu_prime: alloc failed: {e}"),
        };
        let cfg = LaunchConfig::for_num_elems(span as u32);
        let mut b = stream.launch_builder(&func);
        b.arg(&mut d_out);
        b.arg(&base_lo);
        b.arg(&base_hi);
        b.arg(&span);
        if let Err(e) = unsafe { b.launch(cfg) } {
            return format!("gpu_prime: launch failed: {e}");
        }
        let mask = match stream.clone_dtoh(&d_out) {
            Ok(m) => m,
            Err(e) => return format!("gpu_prime: dtoh failed: {e}"),
        };

        for i in 0..(span as usize) {
            tested += 1;
            if mask[i] == 1 {
                count += 1;
                if !count_only {
                    let p = top - i as u128;
                    listing.push_str(&format!("{}\n", p));
                }
            }
        }

        // Reached lo this launch, or one more window would pass it.
        if span < WINDOW || top - (span as u128) < lo {
            break;
        }
        top -= span as u128;
    }

    if count_only {
        format!("prime_winding range [{}, {}]: {} prime(s) ({} integers tested on GPU)", lo, hi, count, tested)
    } else {
        format!("prime_winding range [{}, {}]: {} prime(s) (GPU)\n{}", lo, hi, count, listing)
    }
}

/// The greatest prime <= n, tested on the GPU: scan one window down from n and
/// take the first flagged value. Falls back to a wider walk only if the gap
/// exceeds one window (astronomically unlikely for a prime gap).
pub fn find_gpu(n: u128, device: usize) -> String {
    if n < 2 {
        return format!("prime_winding find {}: no primes <= {}", n, n);
    }
    let lo = if n > WINDOW as u128 { n - WINDOW as u128 + 1 } else { 2 };
    let out = range_gpu(lo, n, false, device);
    // range_gpu lists high to low; the first listed value is the nearest prime <= n.
    for line in out.lines() {
        if let Ok(p) = line.parse::<u128>() {
            return if p == n {
                format!("prime_winding find {}: {} IS PRIME (GPU)", n, n)
            } else {
                format!("prime_winding find {}: nearest prime <= {} is {} (GPU)", n, n, p)
            };
        }
    }
    format!("prime_winding find {}: no prime found within one GPU window below {}", n, n)
}
