//! gpu_opi.rs — the OPI Prange trial loop on the GPU.
//!
//! run_opi repeats an independent Prange trial many times: keep a random n of
//! the m constraints, pick one satisfying value per kept constraint, Lagrange-
//! interpolate the degree-<n polynomial through those points over GF(p), then
//! count how many of the m constraints the polynomial satisfies. The best count
//! over all trials is the result. Each trial is independent, so one thread runs
//! one trial and the best is taken by an atomic maximum. The interpolation
//! matches opi.rs::lagrange_interpolate term for term, so the per-trial count is
//! the same arithmetic the CPU does, checked by opi_verify.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};

fn kernel_src(n: usize) -> String {
    format!(r#"
#define N {N}
typedef unsigned long long u64;
typedef unsigned __int128 u128;

__device__ u64 mulm(u64 a, u64 b, u64 p){{ return (u64)(((u128)a*(u128)b) % p); }}
__device__ u64 addm(u64 a, u64 b, u64 p){{ u64 s=a+b; if (s>=p||s<a) s-=p; return s%p; }}
__device__ u64 subm(u64 a, u64 b, u64 p){{ return (a>=b)?(a-b):(p-(b-a)); }}
__device__ u64 powm(u64 a, u64 e, u64 p){{ u64 r=1%p; a%=p; while(e){{ if(e&1) r=mulm(r,a,p); a=mulm(a,a,p); e>>=1; }} return r; }}
__device__ u64 invm(u64 a, u64 p){{ return powm(a, p-2, p); }}

// xorshift64 per-thread RNG
__device__ u64 xs(u64* s){{ u64 x=*s; x^=x<<13; x^=x>>7; x^=x<<17; *s=x; return x; }}

extern "C" __global__ void opi_trials(
    u64 p, int m, int n, const u64* points, const unsigned char* membership,
    u64 seed_base, unsigned int trials, int* best)
{{
    unsigned int tid = blockIdx.x*blockDim.x + threadIdx.x;
    if (tid >= trials) return;
    u64 s = (seed_base ^ ((u64)tid*0x9E3779B97F4A7C15ULL)) | 1ULL;
    for (int w=0; w<3; w++) xs(&s);

    // Prange: n distinct constraint indices, rejection-sampled.
    int chosen[N];
    u64 kx[N], ky[N];
    int cnt=0;
    while (cnt < n) {{
        int idx = (int)(xs(&s) % (u64)m);
        bool dup=false; for (int j=0;j<cnt;j++) if (chosen[j]==idx) {{ dup=true; break; }}
        if (dup) continue;
        chosen[cnt]=idx;
        kx[cnt]=points[idx];
        // one satisfying value for this constraint, rejection-sampled
        u64 v; do {{ v = xs(&s) % p; }} while (membership[(u64)idx*p + v]==0);
        ky[cnt]=v;
        cnt++;
    }}

    // Lagrange interpolation, matching opi.rs term for term.
    u64 coeffs[N]; for (int d=0;d<n;d++) coeffs[d]=0;
    for (int i=0;i<n;i++){{
        u64 xi=kx[i], yi=ky[i];
        u64 basis[N]; for (int d=0;d<n;d++) basis[d]=0; basis[0]=1;
        int degree=0; u64 denom=1;
        for (int j=0;j<n;j++){{
            if (j==i) continue;
            u64 xj=kx[j];
            u64 next[N]; for (int d=0;d<n;d++) next[d]=0;
            for (int d=0; d<=degree; d++){{
                if (d+1 < n) next[d+1]=addm(next[d+1], basis[d], p);
                next[d]=subm(next[d], mulm(basis[d], xj, p), p);
            }}
            for (int d=0;d<n;d++) basis[d]=next[d];
            degree++;
            denom=mulm(denom, subm(xi,xj,p), p);
        }}
        u64 scale=mulm(yi, invm(denom,p), p);
        for (int d=0;d<n;d++) coeffs[d]=addm(coeffs[d], mulm(basis[d], scale, p), p);
    }}

    // Count satisfied constraints (Horner eval at every point).
    int sat=0;
    for (int i=0;i<m;i++){{
        u64 x=points[i], acc=0;
        for (int d=n-1; d>=0; d--) acc=addm(mulm(acc,x,p), coeffs[d], p);
        if (membership[(u64)i*p + acc]) sat++;
    }}
    atomicMax(best, sat);
}}

// Fixed subset (kx,ky given): interpolate and count, no sampling. Used to check
// the interpolation and eval arithmetic against the CPU on identical inputs.
extern "C" __global__ void opi_fixed(
    u64 p, int m, int n, const u64* points, const unsigned char* membership,
    const u64* kx, const u64* ky, int* out)
{{
    if (blockIdx.x!=0 || threadIdx.x!=0) return;
    u64 coeffs[N]; for (int d=0;d<n;d++) coeffs[d]=0;
    for (int i=0;i<n;i++){{
        u64 xi=kx[i], yi=ky[i];
        u64 basis[N]; for (int d=0;d<n;d++) basis[d]=0; basis[0]=1;
        int degree=0; u64 denom=1;
        for (int j=0;j<n;j++){{
            if (j==i) continue;
            u64 xj=kx[j];
            u64 next[N]; for (int d=0;d<n;d++) next[d]=0;
            for (int d=0; d<=degree; d++){{
                if (d+1 < n) next[d+1]=addm(next[d+1], basis[d], p);
                next[d]=subm(next[d], mulm(basis[d], xj, p), p);
            }}
            for (int d=0;d<n;d++) basis[d]=next[d];
            degree++;
            denom=mulm(denom, subm(xi,xj,p), p);
        }}
        u64 scale=mulm(yi, invm(denom,p), p);
        for (int d=0;d<n;d++) coeffs[d]=addm(coeffs[d], mulm(basis[d], scale, p), p);
    }}
    int sat=0;
    for (int i=0;i<m;i++){{
        u64 x=points[i], acc=0;
        for (int d=n-1; d>=0; d--) acc=addm(mulm(acc,x,p), coeffs[d], p);
        if (membership[(u64)i*p + acc]) sat++;
    }}
    out[0]=sat;
}}
"#, N = n)
}

/// Best satisfied-constraint count over `trials` GPU Prange trials. points has m
/// entries, membership is row-major m*p bytes (1 = value in the constraint's
/// satisfying set). Returns None if no device is present.
pub fn run_trials(p: u64, m: usize, n: usize, points: &[u64], membership: &[u8],
                  trials: usize, seed: u64) -> Option<usize> {
    let ctx = CudaContext::new(0).ok()?;
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = compile_ptx_with_opts(kernel_src(n), opts).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("opi_trials").ok()?;

    let d_pts = stream.clone_htod(points).ok()?;
    let d_mem = stream.clone_htod(membership).ok()?;
    let init: Vec<i32> = alloc::vec![0];
    let mut d_best = stream.clone_htod(&init).ok()?;

    let ml = m as i32; let nl = n as i32; let tr = trials as u32;
    let threads = 128u32;
    let blocks = ((trials as u32) + threads - 1) / threads;
    let cfg = LaunchConfig { grid_dim: (blocks.max(1),1,1), block_dim: (threads,1,1), shared_mem_bytes: 0 };
    let mut b = stream.launch_builder(&func);
    b.arg(&p); b.arg(&ml); b.arg(&nl); b.arg(&d_pts); b.arg(&d_mem);
    b.arg(&seed); b.arg(&tr); b.arg(&mut d_best);
    unsafe { b.launch(cfg) }.ok()?;
    let best = stream.clone_dtoh(&d_best).ok()?;
    Some(best[0] as usize)
}

/// Count satisfied constraints for one fixed subset on the GPU (interpolate +
/// eval, no sampling), for checking against the CPU on identical inputs.
pub fn run_fixed(p: u64, m: usize, n: usize, points: &[u64], membership: &[u8],
                 kx: &[u64], ky: &[u64]) -> Option<usize> {
    let ctx = CudaContext::new(0).ok()?;
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = compile_ptx_with_opts(kernel_src(n), opts).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("opi_fixed").ok()?;
    let d_pts = stream.clone_htod(points).ok()?;
    let d_mem = stream.clone_htod(membership).ok()?;
    let d_kx = stream.clone_htod(kx).ok()?;
    let d_ky = stream.clone_htod(ky).ok()?;
    let init: Vec<i32> = alloc::vec![0];
    let mut d_out = stream.clone_htod(&init).ok()?;
    let ml = m as i32; let nl = n as i32;
    let cfg = LaunchConfig { grid_dim: (1,1,1), block_dim: (1,1,1), shared_mem_bytes: 0 };
    let mut b = stream.launch_builder(&func);
    b.arg(&p); b.arg(&ml); b.arg(&nl); b.arg(&d_pts); b.arg(&d_mem);
    b.arg(&d_kx); b.arg(&d_ky); b.arg(&mut d_out);
    unsafe { b.launch(cfg) }.ok()?;
    let out = stream.clone_dtoh(&d_out).ok()?;
    Some(out[0] as usize)
}

/// Self-check: build an OPI instance and a random subset, count satisfied
/// constraints for that subset on the GPU, and confirm it equals the CPU count
/// through opi.rs's own interpolate-and-evaluate on identical inputs.
pub fn verify(p: u64, n: usize, seed: u64) -> String {
    let inst = match crate::opi::build_instance(p, n, seed) {
        Ok(v) => v, Err(e) => return format!("gpu_opi verify: {}", e),
    };
    let (m, points, membership) = inst;
    // Pick a random subset of n distinct constraints and one satisfying value each.
    let mut s = seed ^ 0xABCD_1234;
    let mut nx = || { s ^= s<<13; s ^= s>>7; s ^= s<<17; s };
    let mut chosen: Vec<usize> = Vec::new();
    while chosen.len() < n {
        let idx = (nx() % m as u64) as usize;
        if !chosen.contains(&idx) { chosen.push(idx); }
    }
    let mut kx: Vec<u64> = Vec::new();
    let mut ky: Vec<u64> = Vec::new();
    for &idx in &chosen {
        kx.push(points[idx]);
        let mut v; loop { v = nx() % p; if membership[idx*(p as usize) + v as usize]==1 { break; } }
        ky.push(v);
    }
    let cpu = crate::opi::cpu_count_subset(p, m, n, &points, &membership, &kx, &ky);
    match run_fixed(p, m, n, &points, &membership, &kx, &ky) {
        Some(gpu) => format!("gpu_opi verify p={} n={}: GPU satisfied {} / {}  CPU {} -- match {}",
                             p, n, gpu, m, cpu, gpu == cpu),
        None => format!("gpu_opi verify: no CUDA device"),
    }
}
