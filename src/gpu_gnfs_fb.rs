//! GPU ⊢/alfs — prime sieve + algebraic roots of f on device.
//!
//! One block per prime: threads stride r ∈ [0,p) evaluating f(r) ≡ 0 (mod p).
//! Parallel across the factor-base bound so classical B no longer serializes on host.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use std::sync::Arc;

use cudarc::driver::{CudaContext, CudaFunction, CudaStream, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions, Ptx};
use num_bigint::BigUint;

use crate::gpu_gnfs_poly::PolyPair;
use crate::gpu_gnfs_sieve::{primes_upto, AlgIdeal, FactorBases};

const FB_KERNEL: &str = r#"
typedef unsigned long long u64;

extern "C" __global__ void gnfs_sieve_primes(unsigned char* is_p, int n) {
    int i = (int)(blockIdx.x * blockDim.x + threadIdx.x);
    if (i >= n + 1) return;
    is_p[i] = (i >= 2) ? 1 : 0;
}

extern "C" __global__ void gnfs_mark_composites(unsigned char* is_p, int n, int p) {
    long long start = (long long)p * (long long)p;
    if (start > n) return;
    for (long long m = start + (long long)threadIdx.x * p;
         m <= n;
         m += (long long)blockDim.x * p) {
        is_p[(int)m] = 0;
    }
}

// One block per prime. fcoef is either shared (stride=deg+1) or per-prime
// packed (stride = nprimes*(deg+1) layout: prime-major). packed=1 selects latter.
extern "C" __global__ void gnfs_roots_mod_p(
    const u64* fcoef, int deg, int packed,
    const u64* primes, int nprimes,
    u64* out_roots, int* out_counts, int max_roots)
{
    int pi = (int)blockIdx.x;
    if (pi >= nprimes) return;
    u64 p = primes[pi];
    if (p < 2) {
        if (threadIdx.x == 0) out_counts[pi] = 0;
        return;
    }
    __shared__ int count;
    if (threadIdx.x == 0) count = 0;
    __syncthreads();

    const u64* fc = packed ? &fcoef[pi * (deg + 1)] : fcoef;

    for (u64 r = (u64)threadIdx.x; r < p; r += (u64)blockDim.x) {
        unsigned __int128 acc = 0;
        unsigned __int128 xp = 1;
        unsigned __int128 pm = p;
        for (int i = 0; i <= deg; i++) {
            u64 c = fc[i] % p;
            acc = (acc + (unsigned __int128)c * xp) % pm;
            xp = (xp * (unsigned __int128)r) % pm;
        }
        if ((u64)acc == 0) {
            int slot = atomicAdd(&count, 1);
            if (slot < max_roots) {
                out_roots[pi * max_roots + slot] = r;
            }
        }
    }
    __syncthreads();
    if (threadIdx.x == 0) {
        out_counts[pi] = count < max_roots ? count : max_roots;
    }
}
"#;

struct FbGpu {
    stream: Arc<CudaStream>,
    sieve_init: CudaFunction,
    mark: CudaFunction,
    roots: CudaFunction,
    device: u32,
}

fn selected_device() -> u32 {
    std::env::var("GNFS_GPU")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn ptx_path() -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in FB_KERNEL.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("target/gnfs_fb_{h:016x}.ptx")
}

fn load_ptx() -> Result<Ptx, String> {
    let path = ptx_path();
    if std::path::Path::new(&path).is_file() {
        eprintln!("gpu_gnfs ⊢/alfs GPU: cached PTX → {path}");
        return Ok(Ptx::from_file(&path));
    }
    eprintln!("gpu_gnfs ⊢/alfs GPU: NVRTC compiling prime/root kernels…");
    let t0 = std::time::Instant::now();
    let opts = CompileOptions {
        options: alloc::vec!["--device-int128".into()],
        ..Default::default()
    };
    let ptx = compile_ptx_with_opts(FB_KERNEL, opts).map_err(|e| format!("NVRTC: {e}"))?;
    let src = ptx.to_src();
    let _ = std::fs::write(&path, src.as_bytes());
    eprintln!(
        "gpu_gnfs ⊢/alfs GPU: NVRTC {} ms — cached {path}",
        t0.elapsed().as_millis()
    );
    Ok(Ptx::from_src(src))
}

fn setup(device: u32) -> Result<FbGpu, String> {
    let ctx = CudaContext::new(device as usize).map_err(|e| format!("CudaContext({device}): {e}"))?;
    let stream = ctx.default_stream();
    let module = ctx
        .load_module(load_ptx()?)
        .map_err(|e| format!("load_module: {e}"))?;
    Ok(FbGpu {
        stream,
        sieve_init: module
            .load_function("gnfs_sieve_primes")
            .map_err(|e| format!("sieve_init: {e}"))?,
        mark: module
            .load_function("gnfs_mark_composites")
            .map_err(|e| format!("mark: {e}"))?,
        roots: module
            .load_function("gnfs_roots_mod_p")
            .map_err(|e| format!("roots: {e}"))?,
        device,
    })
}

fn big_mod_u64(n: &BigUint, p: u64) -> u64 {
    if p < 2 {
        return 0;
    }
    crate::native_numeral::modulo_small_via_word(n, p).unwrap_or(0)
}

/// Build factor bases on device. Returns (fb, used_gpu, ms).
pub fn build_factor_bases_gpu(poly: &PolyPair, b: u64) -> (FactorBases, bool, u64) {
    let t0 = std::time::Instant::now();
    match build_inner(poly, b) {
        Ok(fb) => (fb, true, t0.elapsed().as_millis() as u64),
        Err(e) => {
            eprintln!("gpu_gnfs ⊢/alfs GPU failed — host fallback: {e}");
            (
                FactorBases::build(poly, b),
                false,
                t0.elapsed().as_millis() as u64,
            )
        }
    }
}

fn build_inner(poly: &PolyPair, b: u64) -> Result<FactorBases, String> {
    if b < 2 {
        return Ok(FactorBases {
            rational: Vec::new(),
            algebraic: Vec::new(),
        });
    }
    // Cap device bit-sieve memory (~1 byte per integer).
    const MAX_GPU_SIEVE: u64 = 50_000_000;
    let gpu = setup(selected_device())?;
    let deg = poly.degree as i32;
    if deg < 1 {
        return Err(String::from("deg < 1"));
    }

    let rational = if b <= MAX_GPU_SIEVE {
        gpu_primes(&gpu, b)?
    } else {
        eprintln!("gpu_gnfs ⊢/alfs: B={b} > {MAX_GPU_SIEVE} — host primes, GPU roots");
        primes_upto(b)
    };

    let wide = poly.f.iter().any(|c| c.bits() > 64);
    let algebraic = if wide {
        gpu_roots_packed(&gpu, poly, &rational, deg as usize)?
    } else {
        let mut fcoef = alloc::vec![0u64; (deg as usize) + 1];
        for (i, c) in poly.f.iter().enumerate() {
            fcoef[i] = c.to_u64_digits().first().copied().unwrap_or(0);
        }
        gpu_roots(&gpu, &fcoef, deg, &rational, false)?
    };

    eprintln!(
        "gpu_gnfs ⊢/alfs device: B={b} |rat|={} |alg|={} on device {} (gpu=true)",
        rational.len(),
        algebraic.len(),
        gpu.device
    );
    Ok(FactorBases {
        rational,
        algebraic,
    })
}

fn gpu_primes(gpu: &FbGpu, b: u64) -> Result<Vec<u64>, String> {
    let n = b as i32;
    let mut d_isp = gpu
        .stream
        .alloc_zeros::<u8>((n as usize) + 1)
        .map_err(|e| format!("alloc isp: {e}"))?;
    {
        let threads = 256u32;
        let blocks = ((n as u32 + 1) + threads - 1) / threads;
        let mut launch = gpu.stream.launch_builder(&gpu.sieve_init);
        unsafe {
            launch.arg(&mut d_isp);
            launch.arg(&n);
            launch
                .launch(LaunchConfig {
                    grid_dim: (blocks, 1, 1),
                    block_dim: (threads, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("sieve_init: {e}"))?;
        }
    }
    let lim = ((b as f64).sqrt() as u64) + 1;
    let drivers = primes_upto(lim.max(2));
    for &p in &drivers {
        if p < 2 {
            continue;
        }
        let p_i = p as i32;
        let mut launch = gpu.stream.launch_builder(&gpu.mark);
        unsafe {
            launch.arg(&mut d_isp);
            launch.arg(&n);
            launch.arg(&p_i);
            launch
                .launch(LaunchConfig {
                    grid_dim: (1, 1, 1),
                    block_dim: (256, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("mark p={p}: {e}"))?;
        }
    }
    gpu.stream
        .synchronize()
        .map_err(|e| format!("sieve sync: {e}"))?;
    let isp = gpu
        .stream
        .clone_dtoh(&d_isp)
        .map_err(|e| format!("dtoh isp: {e}"))?;
    let mut out = Vec::new();
    for (i, &v) in isp.iter().enumerate() {
        if v != 0 {
            out.push(i as u64);
        }
    }
    Ok(out)
}

fn gpu_roots(
    gpu: &FbGpu,
    fcoef: &[u64],
    deg: i32,
    primes: &[u64],
    packed: bool,
) -> Result<Vec<AlgIdeal>, String> {
    if primes.is_empty() {
        return Ok(Vec::new());
    }
    let max_roots = (deg as usize + 1).max(1);
    let nprimes = primes.len();
    let d_f = gpu
        .stream
        .clone_htod(fcoef)
        .map_err(|e| format!("htod f: {e}"))?;
    let d_p = gpu
        .stream
        .clone_htod(primes)
        .map_err(|e| format!("htod p: {e}"))?;
    let mut d_roots = gpu
        .stream
        .alloc_zeros::<u64>(nprimes * max_roots)
        .map_err(|e| format!("alloc roots: {e}"))?;
    let mut d_counts = gpu
        .stream
        .alloc_zeros::<i32>(nprimes)
        .map_err(|e| format!("alloc counts: {e}"))?;
    let nprimes_i = nprimes as i32;
    let max_roots_i = max_roots as i32;
    let packed_i = if packed { 1i32 } else { 0i32 };
    let threads = 256u32;
    {
        let mut launch = gpu.stream.launch_builder(&gpu.roots);
        unsafe {
            launch.arg(&d_f);
            launch.arg(&deg);
            launch.arg(&packed_i);
            launch.arg(&d_p);
            launch.arg(&nprimes_i);
            launch.arg(&mut d_roots);
            launch.arg(&mut d_counts);
            launch.arg(&max_roots_i);
            launch
                .launch(LaunchConfig {
                    grid_dim: (nprimes as u32, 1, 1),
                    block_dim: (threads, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("roots launch: {e}"))?;
        }
    }
    gpu.stream
        .synchronize()
        .map_err(|e| format!("roots sync: {e}"))?;
    let roots = gpu
        .stream
        .clone_dtoh(&d_roots)
        .map_err(|e| format!("dtoh roots: {e}"))?;
    let counts = gpu
        .stream
        .clone_dtoh(&d_counts)
        .map_err(|e| format!("dtoh counts: {e}"))?;
    let mut algebraic = Vec::new();
    for (pi, &p) in primes.iter().enumerate() {
        let c = counts[pi].max(0) as usize;
        for s in 0..c.min(max_roots) {
            algebraic.push(AlgIdeal {
                p,
                r: roots[pi * max_roots + s],
            });
        }
    }
    Ok(algebraic)
}

/// Wide coefficients: pack f mod p for every prime, one batched root launch.
fn gpu_roots_packed(
    gpu: &FbGpu,
    poly: &PolyPair,
    primes: &[u64],
    deg: usize,
) -> Result<Vec<AlgIdeal>, String> {
    if primes.is_empty() {
        return Ok(Vec::new());
    }
    let mut flat = alloc::vec![0u64; primes.len() * (deg + 1)];
    for (pi, &p) in primes.iter().enumerate() {
        if p < 2 {
            continue;
        }
        for (i, c) in poly.f.iter().enumerate() {
            flat[pi * (deg + 1) + i] = big_mod_u64(c, p);
        }
    }
    gpu_roots(gpu, &flat, deg as i32, primes, true)
}
