//! gpu_shor.rs — classical period (multiplicative order) finding on the GPU.
//!
//! The order of a modulo n is the least r with a^r == 1 (mod n), the period
//! Shor's algorithm extracts. The sequential scan multiplies by a until it
//! returns to 1, a dependent chain. This splits the range into chunks: a thread
//! jumps to the start of its chunk with one modular exponentiation, walks the
//! chunk multiplying, and reports any r that hits 1 by an atomic minimum, so the
//! true order (the smallest such r) wins. The range is swept in windows so no
//! one launch runs long enough to trip the display-driver watchdog.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};

const KERNEL_SRC: &str = r#"
typedef unsigned long long u64;

// (a+b) mod n for a,b already < n -- overflow-safe without a wider integer
// type, the same "carry, don't widen" discipline bits_add/bits_subtract use.
__device__ u64 addmod(u64 a, u64 b, u64 n){
    u64 s = a + b;
    if (s < a || s >= n) s -= n;
    return s;
}

// a mod n by long division, walking a's bits from the top down: double the
// running remainder and bring in the next bit, exactly bits_divmod's own
// walk (there over whole limbs, here over the 64 bits of one). This is what
// replaces the hardware %/__int128 reduction the original kernel used.
__device__ u64 modn(u64 a, u64 n){
    u64 rem = 0;
    for (int i = 63; i >= 0; i--){
        rem = addmod(rem, rem, n);
        if ((a >> i) & 1) rem = addmod(rem, 1, n);
    }
    return rem;
}

// a*b mod n by shift-and-add: walk b's bits low to high, doubling a (mod n)
// each step and adding it in wherever that bit of b is set -- the same
// shape as bits_multiply, replacing the __int128 widen-then-reduce the
// original kernel used.
__device__ u64 mulm(u64 a, u64 b, u64 n){
    u64 result = 0;
    a = modn(a, n);
    while (b){
        if (b & 1) result = addmod(result, a, n);
        a = addmod(a, a, n);
        b >>= 1;
    }
    return result;
}
__device__ u64 powm(u64 a, u64 e, u64 n){ u64 r=modn(1,n); a=modn(a,n); while(e){ if(e&1) r=mulm(r,a,n); a=mulm(a,a,n); e>>=1; } return r; }
extern "C" __global__ void order_search(u64 a, u64 n, u64 base, u64 chunk, u64 window_end, u64* result){
    u64 t = (u64)blockIdx.x*blockDim.x + threadIdx.x;
    u64 start = base + t*chunk;
    if (start >= window_end) return;
    if (*result <= start) return;          // a smaller r already found
    u64 end = start + chunk; if (end > window_end) end = window_end;
    u64 val = powm(a, start - 1, n);       // a^(start-1); start>=1 so no underflow
    for (u64 r=start; r<end; r++){
        val = mulm(val, a, n);             // val = a^r
        if (val == 1){ atomicMin(result, r); return; }
    }
}

// The BSGS pair: every baby step a^j and every giant step g^i is its own
// independent modular exponentiation, not a link in a chain depending on
// the step before it -- unlike order_search's sequential walk, thread j
// here never needs thread j-1's result. That is the actual structural
// difference from brute force, not a faster multiply.
extern "C" __global__ void baby_step(u64 a, u64 n, u64 m, u64* out){
    u64 j = (u64)blockIdx.x*blockDim.x + threadIdx.x;
    if (j >= m) return;
    out[j] = powm(a, j, n);
}
extern "C" __global__ void giant_step(u64 g, u64 n, u64 m, u64* out){
    u64 i = (u64)blockIdx.x*blockDim.x + threadIdx.x + 1;
    if (i > m) return;
    out[i-1] = powm(g, i, n);
}
"#;

/// Multiplicative order of a modulo n on the GPU: least r>=1 with a^r==1 (mod
/// n), or 0 if none up to n (matching the CPU scan). Returns None if there is no
/// device.
pub fn order(a: u64, n: u64) -> Option<u64> {
    if n <= 1 { return Some(0); }
    let a = a % n;
    if a == 0 { return Some(0); }
    let ctx = CudaContext::new(0).ok()?;
    let stream = ctx.default_stream();
    let opts = CompileOptions::default();
    let ptx = compile_ptx_with_opts(KERNEL_SRC, opts).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("order_search").ok()?;

    let init: Vec<u64> = alloc::vec![u64::MAX];
    let mut d_res = stream.clone_htod(&init).ok()?;

    let chunk: u64 = 64;
    let threads: u32 = 256;
    let window: u64 = 1u64 << 26;                  // candidates per wave
    let blocks: u32 = ((window / chunk) as u32 + threads - 1) / threads;
    let cfg = LaunchConfig { grid_dim: (blocks,1,1), block_dim: (threads,1,1), shared_mem_bytes: 0 };

    let mut base: u64 = 1;
    while base < n.saturating_add(1) {
        let window_end = core::cmp::min(base.saturating_add(window), n.saturating_add(1));
        let mut b = stream.launch_builder(&func);
        b.arg(&a); b.arg(&n); b.arg(&base); b.arg(&chunk); b.arg(&window_end); b.arg(&mut d_res);
        unsafe { b.launch(cfg) }.ok()?;
        let r = stream.clone_dtoh(&d_res).ok()?;
        if r[0] != u64::MAX { return Some(r[0]); }
        base = window_end;
    }
    Some(0)
}

fn pow_mod_u64(a: u64, mut e: u64, n: u64) -> u64 {
    if n == 1 { return 0; }
    let mut r: u128 = 1 % n as u128;
    let mut ab: u128 = (a % n) as u128;
    while e > 0 {
        if e & 1 == 1 { r = (r * ab) % n as u128; }
        ab = (ab * ab) % n as u128;
        e >>= 1;
    }
    r as u64
}

/// Multiplicative order via GPU BSGS. Baby steps (a^j, j=0..m-1) and giant
/// steps (g^i, i=1..=m) are each generated by their own kernel, and within
/// each kernel every thread computes its own exponentiation directly --
/// thread j never waits on thread j-1's result, unlike order_search's
/// sequential per-thread chain. The candidates touched total O(sqrt(order)),
/// not O(order): a search-space reduction, not a faster multiply, which is
/// the actual reason this reaches orders order_search's own wave-by-wave
/// brute force cannot in practical time. step_cap bounds the baby-step
/// table size (and so the order this can see), same contract as
/// winding_period::winding_order_big's own step_cap.
pub fn order_bsgs(a: u64, n: u64, step_cap: u64) -> Option<u64> {
    if n <= 1 { return Some(0); }
    let a = a % n;
    if a == 0 { return Some(0); }
    if crate::winding_period::gcd(a, n) != 1 { return None; }
    if a == 1 { return Some(1); }

    let m = core::cmp::min(crate::winding_period::isqrt(n) + 1, step_cap);
    if m == 0 { return None; }

    let ctx = CudaContext::new(0).ok()?;
    let stream = ctx.default_stream();
    let opts = CompileOptions::default();
    let ptx = compile_ptx_with_opts(KERNEL_SRC, opts).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let baby_fn = module.load_function("baby_step").ok()?;
    let giant_fn = module.load_function("giant_step").ok()?;

    let threads: u32 = 256;
    let blocks: u32 = ((m as u32).saturating_add(threads - 1)) / threads;
    let cfg = LaunchConfig { grid_dim: (blocks, 1, 1), block_dim: (threads, 1, 1), shared_mem_bytes: 0 };

    // Baby steps: a^j mod n for j=0..m-1, each thread its own exponentiation.
    let zeros: Vec<u64> = alloc::vec![0u64; m as usize];
    let mut d_baby = stream.clone_htod(&zeros).ok()?;
    let mut bb = stream.launch_builder(&baby_fn);
    bb.arg(&a); bb.arg(&n); bb.arg(&m); bb.arg(&mut d_baby);
    unsafe { bb.launch(cfg) }.ok()?;
    let baby: Vec<u64> = stream.clone_dtoh(&d_baby).ok()?;

    let mut indexed: Vec<(u64, u64)> = baby.iter().enumerate().map(|(j, &v)| (v, j as u64)).collect();
    indexed.sort_unstable();

    // Giant steps: g^i mod n for i=1..=m, g = a^{-m} mod n -- again every
    // thread its own exponentiation, no dependency on the step before it.
    let a_inv = crate::winding_period::modinv(a, n);
    let g = pow_mod_u64(a_inv, m, n);

    let mut d_giant = stream.clone_htod(&zeros).ok()?;
    let mut gb = stream.launch_builder(&giant_fn);
    gb.arg(&g); gb.arg(&n); gb.arg(&m); gb.arg(&mut d_giant);
    unsafe { gb.launch(cfg) }.ok()?;
    let giant: Vec<u64> = stream.clone_dtoh(&d_giant).ok()?;

    for (idx, &gv) in giant.iter().enumerate() {
        let i = (idx as u64) + 1;
        if let Ok(pos) = indexed.binary_search_by_key(&gv, |&(v, _)| v) {
            let j = indexed[pos].1;
            let cand = i.saturating_mul(m).saturating_add(j);
            if cand > 0 && pow_mod_u64(a, cand, n) == 1 {
                return Some(crate::winding_period::minimal_winding(a, n, cand));
            }
        }
    }
    None
}

/// Self-check: GPU order against the plain CPU scan on small n.
pub fn verify() -> String {
    fn cpu_order(a: u64, n: u64) -> u64 {
        if n <= 1 { return 0; }
        let mut val = 1u64 % n;
        for r in 1..=n { val = (((val as u128) * (a as u128)) % n as u128) as u64; if val == 1 { return r; } }
        0
    }
    let cases: [(u64,u64); 8] = [(7,15),(5,21),(2,35),(2,77),(3,101),(10,1000003),(2,1048573),(6,999983)];
    let mut out = String::new();
    let mut all = true;
    for (a,n) in cases {
        let cpu = cpu_order(a,n);
        let gpu = order(a,n).unwrap_or(u64::MAX);
        let ok = cpu == gpu;
        all &= ok;
        out.push_str(&format!("  a={} n={}: CPU r={} GPU r={} -- match {}\n", a, n, cpu, gpu, ok));
    }
    format!("gpu_shor verify (order finding):\n{}  all match: {}", out, all)
}
