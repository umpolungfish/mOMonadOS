//! gpu_dqi.rs — the DQI minimum-weight coset-leader search on the GPU.
//!
//! syndrome_decode_bounded reduces to: over every mask in [0, 2^k), form the
//! candidate e0 XOR (sum of the basis vectors the mask selects) and keep the
//! one of least Hamming weight. The vectors are bit-packed into 64-bit words,
//! one mask per thread with a grid stride, and the least weight is taken by an
//! atomic minimum on a value that packs the weight above the mask, so the
//! smallest weight wins and ties break to the smaller mask. The host unpacks
//! the winning mask and rebuilds the candidate.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

const KERNEL_SRC: &str = r#"
typedef unsigned long long u64;
extern "C" __global__ void coset_min(
    const u64* e0, const u64* basis, int W, int k, u64 total, u64* best)
{
    u64 stride = (u64)gridDim.x * blockDim.x;
    for (u64 mask = (u64)blockIdx.x * blockDim.x + threadIdx.x; mask < total; mask += stride) {
        int weight = 0;
        for (int w = 0; w < W; w++) {
            u64 acc = e0[w];
            for (int i = 0; i < k; i++) {
                if ((mask >> i) & 1ULL) acc ^= basis[(u64)i * W + w];
            }
            weight += __popcll(acc);
        }
        // weight (< 2^24) in the high bits, mask (< 2^40) in the low bits.
        u64 packed = ((u64)weight << 40) | mask;
        atomicMin(best, packed);
    }
}
"#;

/// Least-Hamming-weight vector in the coset e0 + span(basis), searched over all
/// 2^k masks on the GPU. Returns the winning (candidate, weight). `basis` holds
/// k vectors each packed into W = ceil(num_vars/64) little-endian u64 words;
/// `e0` is one such packed vector. Falls back to None if no device is present.
pub fn coset_min_weight(e0_words: &[u64], basis_words: &[u64], w: usize, k: usize,
                        num_vars: usize) -> Option<(Vec<bool>, usize)> {
    if k > 40 { return None; }
    let total: u64 = 1u64 << k;
    let ctx = CudaContext::new(0).ok()?;
    let stream = ctx.default_stream();
    let ptx = compile_ptx(KERNEL_SRC).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("coset_min").ok()?;

    let d_e0 = stream.clone_htod(e0_words).ok()?;
    let d_basis = stream.clone_htod(basis_words).ok()?;
    let init: Vec<u64> = alloc::vec![u64::MAX];
    let mut d_best = stream.clone_htod(&init).ok()?;

    let wl = w as i32;
    let kl = k as i32;
    // A grid large enough to cover the machine; the stride loop handles the rest.
    let threads = 256u32;
    let blocks = core::cmp::min(65535u32, ((total + threads as u64 - 1) / threads as u64) as u32).max(1);
    let cfg = LaunchConfig { grid_dim: (blocks,1,1), block_dim: (threads,1,1), shared_mem_bytes: 0 };
    let mut b = stream.launch_builder(&func);
    b.arg(&d_e0); b.arg(&d_basis); b.arg(&wl); b.arg(&kl); b.arg(&total); b.arg(&mut d_best);
    unsafe { b.launch(cfg) }.ok()?;
    let best = stream.clone_dtoh(&d_best).ok()?;
    let packed = best[0];
    let weight = (packed >> 40) as usize;
    let mask = packed & ((1u64 << 40) - 1);

    // Rebuild the winning candidate from the mask.
    let mut cand = alloc::vec![false; num_vars];
    let mut words: Vec<u64> = e0_words.to_vec();
    for i in 0..k {
        if (mask >> i) & 1 == 1 {
            for wi in 0..w { words[wi] ^= basis_words[i*w + wi]; }
        }
    }
    for j in 0..num_vars {
        if (words[j/64] >> (j%64)) & 1 == 1 { cand[j] = true; }
    }
    Some((cand, weight))
}

/// Pack a boolean vector into little-endian 64-bit words.
pub fn pack(v: &[bool], w: usize) -> Vec<u64> {
    let mut words = alloc::vec![0u64; w];
    for (j, &b) in v.iter().enumerate() {
        if b { words[j/64] |= 1u64 << (j%64); }
    }
    words
}

/// Self-check: build a random coset, search it on the GPU, and confirm the
/// weight matches an exhaustive CPU scan.
pub fn verify(num_vars: usize, k: usize, seed: u64) -> String {
    let mut s = seed ^ 0xD1_D1_D1;
    let mut nx = || { s ^= s<<13; s ^= s>>7; s ^= s<<17; s };
    let w = (num_vars + 63) / 64;
    let e0: Vec<bool> = (0..num_vars).map(|_| nx() & 1 == 1).collect();
    let basis: Vec<Vec<bool>> = (0..k).map(|_| (0..num_vars).map(|_| nx() & 1 == 1).collect()).collect();

    // CPU exhaustive minimum.
    let mut cpu_best = usize::MAX;
    for mask in 0u64..(1u64<<k) {
        let mut cand = e0.clone();
        for i in 0..k { if (mask>>i)&1==1 { for j in 0..num_vars { cand[j] ^= basis[i][j]; } } }
        let wt = cand.iter().filter(|&&b| b).count();
        if wt < cpu_best { cpu_best = wt; }
    }

    let e0w = pack(&e0, w);
    let mut basisw: Vec<u64> = Vec::new();
    for bv in &basis { basisw.extend(pack(bv, w)); }
    match coset_min_weight(&e0w, &basisw, w, k, num_vars) {
        Some((cand, gpu_w)) => {
            let recomputed = cand.iter().filter(|&&b| b).count();
            format!("gpu_dqi verify num_vars={} k={}: GPU min weight {} (candidate weight {}), CPU min {} -- match {}",
                    num_vars, k, gpu_w, recomputed, cpu_best, gpu_w == cpu_best && gpu_w == recomputed)
        }
        None => format!("gpu_dqi verify: no CUDA device"),
    }
}
