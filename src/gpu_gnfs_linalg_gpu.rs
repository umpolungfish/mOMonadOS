//! GPU ⋈≻∋⊤ — GF(2) dependency + φ-congruence on device.
//!
//! After the log-line sieve, keep the same CUDA device hot: one-block GF(2) GE
//! for the nullspace, then modular ring products / gcd factor extraction
//! (u64 kernels here; multi-limb Montgomery in gpu_gnfs_cong_ml).

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use std::sync::Arc;

use cudarc::driver::{CudaContext, CudaFunction, CudaStream, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions, Ptx};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

use crate::gpu_gnfs_linalg::{self, SquareCongruence};
use crate::gpu_gnfs_poly::PolyPair;
use crate::gpu_gnfs_sieve::Relation;
use crate::prime_winding::big_gcd;

const LINALG_KERNEL: &str = r#"
typedef unsigned long long u64;
typedef long long i64;

__device__ u64 gnfs_gcd_u64(u64 a, u64 b) {
    while (b) { u64 t = a % b; a = b; b = t; }
    return a;
}

__device__ u64 gnfs_isqrt64(u64 v) {
    if (v < 2) return v;
    u64 x = v, y = (x + 1) >> 1;
    while (y < x) { x = y; y = (x + v / x) >> 1; }
    return x;
}

__device__ int gnfs_modinv_u64(u64 a, u64 n, u64* out) {
    i64 t = 0, nt = 1;
    i64 r = (i64)n, nr = (i64)(a % n);
    while (nr != 0) {
        i64 q = r / nr;
        i64 tmp = nt; nt = t - q * nt; t = tmp;
        tmp = nr; nr = r - q * nr; r = tmp;
    }
    if (!(r == 1 || r == -1)) return 0;
    if (t < 0) t += (i64)n;
    if (r == -1) t = ((i64)n - t) % (i64)n;
    *out = (u64)t;
    return 1;
}

extern "C" __global__ void gnfs_gf2_ge(
    u64* mat, u64* hist,
    int nrows, int words, int hwords, int ncols,
    int* out_rank)
{
    int t = (int)threadIdx.x;
    int nthreads = (int)blockDim.x;
    if (blockIdx.x != 0) return;
    __shared__ int pivot;
    __shared__ int row_i;
    if (t == 0) row_i = 0;
    __syncthreads();

    for (int col = 0; col < ncols; col++) {
        if (t == 0) pivot = -1;
        __syncthreads();
        int ri = row_i;
        if (ri >= nrows) break;
        for (int r = ri + t; r < nrows; r += nthreads) {
            u64 bit = (mat[r * words + (col >> 6)] >> (col & 63)) & 1ull;
            if (bit) atomicCAS(&pivot, -1, r);
        }
        __syncthreads();
        if (pivot < 0) continue;
        // swap pivot ↔ ri (strided across words)
        for (int w = t; w < words; w += nthreads) {
            u64 a = mat[pivot * words + w];
            u64 b = mat[ri * words + w];
            mat[ri * words + w] = a;
            mat[pivot * words + w] = b;
        }
        for (int w = t; w < hwords; w += nthreads) {
            u64 a = hist[pivot * hwords + w];
            u64 b = hist[ri * hwords + w];
            hist[ri * hwords + w] = a;
            hist[pivot * hwords + w] = b;
        }
        __syncthreads();
        for (int r = t; r < nrows; r += nthreads) {
            if (r == ri) continue;
            u64 bit = (mat[r * words + (col >> 6)] >> (col & 63)) & 1ull;
            if (bit) {
                for (int w = 0; w < words; w++)
                    mat[r * words + w] ^= mat[ri * words + w];
                for (int w = 0; w < hwords; w++)
                    hist[r * hwords + w] ^= hist[ri * hwords + w];
            }
        }
        __syncthreads();
        if (t == 0) row_i = ri + 1;
        __syncthreads();
    }
    if (t == 0) *out_rank = row_i;
}

extern "C" __global__ void gnfs_cong_u64(
    const int* dep_offs, const int* dep_idx, int ndeps,
    const i64* rel_a, const i64* rel_b,
    const u64* fcoef, int deg,
    u64 n, u64 m,
    const u64* rat_abs, const u64* alg_abs,
    u64* out_x, u64* out_y, u64* out_factor, int* out_status)
{
    int d = (int)blockIdx.x;
    if (d >= ndeps) return;
    if (threadIdx.x != 0) return;

    out_status[d] = 0;
    out_factor[d] = 0;
    out_y[d] = 0;

    int lo = dep_offs[d];
    int hi = dep_offs[d + 1];
    if (hi <= lo) return;

    u64 lead = fcoef[deg] % n;
    u64 g0 = gnfs_gcd_u64(lead, n);
    if (g0 > 1 && g0 < n) {
        out_factor[d] = g0;
        out_status[d] = 1;
        return;
    }

    u64 inv_lead = 0;
    if (!gnfs_modinv_u64(lead, n, &inv_lead)) {
        u64 g = gnfs_gcd_u64(lead, n);
        if (g > 1 && g < n) {
            out_factor[d] = g;
            out_status[d] = 1;
        }
        return;
    }

    u64 acc[8];
    int i;
    for (i = 0; i < 8; i++) acc[i] = 0;
    acc[0] = 1;

    for (int k = lo; k < hi; k++) {
        int ri = dep_idx[k];
        i64 a = rel_a[ri];
        i64 b = rel_b[ri];
        u64 au = (u64)((a % (i64)n + (i64)n) % (i64)n);
        u64 bu = (u64)(((-b) % (i64)n + (i64)n) % (i64)n);

        u64 lin[8];
        for (i = 0; i < 8; i++) lin[i] = 0;
        lin[0] = au;
        if (deg > 1) lin[1] = bu;

        u64 prod[16];
        for (i = 0; i < 16; i++) prod[i] = 0;
        for (i = 0; i < deg; i++) {
            for (int j = 0; j < deg; j++) {
                unsigned __int128 p = (unsigned __int128)acc[i] * (unsigned __int128)lin[j]
                    + (unsigned __int128)prod[i + j];
                prod[i + j] = (u64)(p % n);
            }
        }
        for (int e = 2 * deg - 2; e >= deg; e--) {
            if (prod[e] == 0) continue;
            u64 coef = prod[e];
            prod[e] = 0;
            u64 scale = (u64)(((unsigned __int128)coef * (unsigned __int128)inv_lead) % n);
            int shift = e - deg;
            for (int t2 = 0; t2 < deg; t2++) {
                u64 ak = fcoef[t2] % n;
                u64 term = (u64)(((unsigned __int128)scale * (unsigned __int128)ak) % n);
                int idx = shift + t2;
                u64 cur = prod[idx];
                prod[idx] = cur >= term ? cur - term : cur + n - term;
            }
        }
        for (i = 0; i < deg; i++) acc[i] = prod[i];
        for (i = deg; i < 8; i++) acc[i] = 0;
    }

    u64 phi = 0, powv = 1;
    for (i = 0; i < deg; i++) {
        phi = (u64)(((unsigned __int128)phi + (unsigned __int128)acc[i] * (unsigned __int128)powv) % n);
        powv = (u64)(((unsigned __int128)powv * (unsigned __int128)m) % n);
    }

    u64 pa = 1;
    for (int k = lo; k < hi; k++) {
        pa = (u64)((unsigned __int128)pa * (unsigned __int128)alg_abs[dep_idx[k]]);
    }
    u64 ys = gnfs_isqrt64(pa);
    out_y[d] = (ys * ys == pa) ? (ys % n) : (phi % n);

    u64 x = out_x[d];
    u64 y = out_y[d];
    u64 cands[4];
    cands[0] = x >= y ? x - y : y - x;
    cands[1] = (x + y) % n;
    cands[2] = x;
    cands[3] = y;
    for (i = 0; i < 4; i++) {
        u64 c = cands[i] % n;
        if (c == 0) continue;
        u64 g = gnfs_gcd_u64(c, n);
        if (g > 1 && g < n) {
            out_factor[d] = g;
            out_status[d] = 1;
            return;
        }
    }
    u64 gphi = gnfs_gcd_u64(phi % n, n);
    if (gphi > 1 && gphi < n) {
        out_factor[d] = gphi;
        out_status[d] = 1;
        return;
    }
    (void)rat_abs;
    out_status[d] = 2;
}

extern "C" __global__ void gnfs_mod_sqrt_search(
    const u64* delta, const u64* fcoef, int deg,
    u64 n, i64 bound, u64* out_beta, int* out_found)
{
    u64 tid = (u64)blockIdx.x * (u64)blockDim.x + (u64)threadIdx.x;
    i64 span = 2 * bound + 1;
    u64 space = 1;
    int i;
    for (i = 0; i < deg; i++) {
        if (space > 0xffffffffffffffffull / (u64)span) return;
        space *= (u64)span;
    }
    if (tid >= space) return;
    if (*out_found != 0) return;

    i64 beta[8];
    u64 rem = tid;
    for (i = 0; i < deg; i++) {
        i64 digit = (i64)(rem % (u64)span);
        rem /= (u64)span;
        beta[i] = digit - bound;
    }
    int all0 = 1;
    for (i = 0; i < deg; i++) if (beta[i] != 0) { all0 = 0; break; }
    if (all0) return;

    u64 inv_lead = 0;
    if (!gnfs_modinv_u64(fcoef[deg] % n, n, &inv_lead)) return;

    u64 bmod[8];
    for (i = 0; i < deg; i++) {
        i64 v = beta[i] % (i64)n;
        if (v < 0) v += (i64)n;
        bmod[i] = (u64)v;
    }
    u64 prod[16];
    for (i = 0; i < 16; i++) prod[i] = 0;
    for (i = 0; i < deg; i++) {
        for (int j = 0; j < deg; j++) {
            unsigned __int128 p = (unsigned __int128)bmod[i] * (unsigned __int128)bmod[j]
                + (unsigned __int128)prod[i + j];
            prod[i + j] = (u64)(p % n);
        }
    }
    for (int e = 2 * deg - 2; e >= deg; e--) {
        if (prod[e] == 0) continue;
        u64 coef = prod[e];
        prod[e] = 0;
        u64 scale = (u64)(((unsigned __int128)coef * (unsigned __int128)inv_lead) % n);
        int shift = e - deg;
        for (int k = 0; k < deg; k++) {
            u64 ak = fcoef[k] % n;
            u64 term = (u64)(((unsigned __int128)scale * (unsigned __int128)ak) % n);
            int idx = shift + k;
            u64 cur = prod[idx];
            prod[idx] = cur >= term ? cur - term : cur + n - term;
        }
    }
    for (i = 0; i < deg; i++) {
        if ((prod[i] % n) != (delta[i] % n)) return;
    }
    if (atomicCAS(out_found, 0, 1) == 0) {
        for (i = 0; i < deg; i++) out_beta[i] = bmod[i];
    }
}
"#;

struct LinalgGpu {
    stream: Arc<CudaStream>,
    gf2: CudaFunction,
    cong: CudaFunction,
    sqrt_search: CudaFunction,
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
    for &b in LINALG_KERNEL.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("target/gnfs_linalg_{h:016x}.ptx")
}

fn load_ptx() -> Result<Ptx, String> {
    let path = ptx_path();
    if std::path::Path::new(&path).is_file() {
        eprintln!("gpu_gnfs ⋈≻ GPU: cached PTX → {path}");
        return Ok(Ptx::from_file(&path));
    }
    eprintln!("gpu_gnfs ⋈≻ GPU: NVRTC compiling GF(2)/congruence kernels…");
    let t0 = std::time::Instant::now();
    let opts = CompileOptions {
        options: alloc::vec!["--device-int128".into()],
        ..Default::default()
    };
    let ptx = compile_ptx_with_opts(LINALG_KERNEL, opts).map_err(|e| format!("NVRTC: {e}"))?;
    let src = ptx.to_src();
    match std::fs::write(&path, src.as_bytes()) {
        Ok(()) => eprintln!(
            "gpu_gnfs ⋈≻ GPU: NVRTC {} ms — cached {path}",
            t0.elapsed().as_millis()
        ),
        Err(e) => eprintln!(
            "gpu_gnfs ⋈≻ GPU: NVRTC {} ms — cache write failed ({e})",
            t0.elapsed().as_millis()
        ),
    }
    Ok(Ptx::from_src(src))
}

fn setup(device: u32) -> Result<LinalgGpu, String> {
    let ctx = CudaContext::new(device as usize).map_err(|e| format!("CudaContext({device}): {e}"))?;
    let stream = ctx.default_stream();
    let module = ctx
        .load_module(load_ptx()?)
        .map_err(|e| format!("load_module: {e}"))?;
    Ok(LinalgGpu {
        stream,
        gf2: module
            .load_function("gnfs_gf2_ge")
            .map_err(|e| format!("gf2: {e}"))?,
        cong: module
            .load_function("gnfs_cong_u64")
            .map_err(|e| format!("cong: {e}"))?,
        sqrt_search: module
            .load_function("gnfs_mod_sqrt_search")
            .map_err(|e| format!("sqrt: {e}"))?,
        device,
    })
}

fn big_to_u64(u: &BigUint) -> Option<u64> {
    if u.bits() > 64 {
        None
    } else {
        Some(u.to_u64_digits().first().copied().unwrap_or(0))
    }
}

/// GF(2) nullspace on device. Falls back to host on error.
pub fn find_dependencies_gpu(relations: &[Relation], max_deps: usize) -> (Vec<Vec<usize>>, bool, u64) {
    let t0 = std::time::Instant::now();
    match find_dependencies_gpu_inner(relations, max_deps) {
        Ok(deps) => (deps, true, t0.elapsed().as_millis() as u64),
        Err(e) => {
            eprintln!("gpu_gnfs ⋈≻ GPU GE failed — host fallback: {e}");
            (
                gpu_gnfs_linalg::find_dependencies(relations, max_deps),
                false,
                t0.elapsed().as_millis() as u64,
            )
        }
    }
}

fn find_dependencies_gpu_inner(
    relations: &[Relation],
    max_deps: usize,
) -> Result<Vec<Vec<usize>>, String> {
    if relations.is_empty() {
        return Ok(Vec::new());
    }
    let gpu = setup(selected_device())?;
    let nrows = relations.len();
    let ncols = relations[0].exps.len();
    let words = (ncols + 63) / 64;
    let hwords = (nrows + 63) / 64;
    if nrows > 65536 {
        return Err(format!("nrows={nrows} > 65536 GE limit"));
    }

    let mut mat = alloc::vec![0u64; nrows * words];
    let mut hist = alloc::vec![0u64; nrows * hwords];
    for (r, rel) in relations.iter().enumerate() {
        for (c, &e) in rel.exps.iter().enumerate() {
            if e != 0 {
                mat[r * words + c / 64] |= 1u64 << (c % 64);
            }
        }
        hist[r * hwords + r / 64] |= 1u64 << (r % 64);
    }

    let mut d_mat = gpu
        .stream
        .clone_htod(&mat)
        .map_err(|e| format!("htod mat: {e}"))?;
    let mut d_hist = gpu
        .stream
        .clone_htod(&hist)
        .map_err(|e| format!("htod hist: {e}"))?;
    let mut d_rank = gpu
        .stream
        .alloc_zeros::<i32>(1)
        .map_err(|e| format!("alloc rank: {e}"))?;

    let nrows_i = nrows as i32;
    let words_i = words as i32;
    let hwords_i = hwords as i32;
    let ncols_i = ncols as i32;
    eprintln!(
        "gpu_gnfs ⋈≻ device: GF(2) GE {}×{} on device {}…",
        nrows, ncols, gpu.device
    );
    {
        let mut launch = gpu.stream.launch_builder(&gpu.gf2);
        unsafe {
            launch.arg(&mut d_mat);
            launch.arg(&mut d_hist);
            launch.arg(&nrows_i);
            launch.arg(&words_i);
            launch.arg(&hwords_i);
            launch.arg(&ncols_i);
            launch.arg(&mut d_rank);
            launch
                .launch(LaunchConfig {
                    grid_dim: (1, 1, 1),
                    block_dim: ((nrows.min(1024)).max(1) as u32, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("gf2 launch: {e}"))?;
        }
    }
    gpu.stream
        .synchronize()
        .map_err(|e| format!("gf2 sync: {e}"))?;

    let hist_out = gpu
        .stream
        .clone_dtoh(&d_hist)
        .map_err(|e| format!("dtoh hist: {e}"))?;
    let mat_out = gpu
        .stream
        .clone_dtoh(&d_mat)
        .map_err(|e| format!("dtoh mat: {e}"))?;

    let mut out = Vec::new();
    for r in 0..nrows {
        if out.len() >= max_deps {
            break;
        }
        if !mat_out[r * words..(r + 1) * words]
            .iter()
            .all(|&w| w == 0)
        {
            continue;
        }
        let mut idxs = Vec::new();
        for i in 0..nrows {
            if (hist_out[r * hwords + i / 64] >> (i % 64)) & 1 == 1 {
                idxs.push(i);
            }
        }
        if !idxs.is_empty() {
            out.push(idxs);
        }
    }
    eprintln!(
        "gpu_gnfs ⋈≻ device: {} dependencies from nullspace",
        out.len()
    );
    Ok(out)
}

fn try_gcd_host(x: &BigUint, y: &BigUint, n: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{add_via_word, modulo_via_word, subtract_via_word};
    let candidates = [
        if x >= y { subtract_via_word(x, y).unwrap() } else { subtract_via_word(y, x).unwrap() },
        add_via_word(x, y),
        x.clone(),
        y.clone(),
    ];
    for c in candidates {
        if c.is_zero() {
            continue;
        }
        let g = big_gcd(modulo_via_word(&c, n).unwrap(), n.clone());
        if g > BigUint::one() && &g < n {
            return Some(g);
        }
    }
    None
}

/// φ-congruence on device for any odd N (u64 fast path, multi-limb Montgomery otherwise).
pub fn congruence_factor_gpu(
    n: &BigUint,
    poly: &PolyPair,
    relations: &[Relation],
    deps: &[Vec<usize>],
) -> Result<(SquareCongruence, bool, u64), String> {
    let t0 = std::time::Instant::now();
    let n_u = match big_to_u64(n) {
        Some(v) if v >= 2 => Some(v),
        _ => None,
    };
    if let Some(n_u) = n_u {
        let force_ml = std::env::var("GNFS_FORCE_ML").ok().as_deref() == Some("1");
        if !force_ml {
            match congruence_u64(n, n_u, poly, relations, deps) {
                Ok(sq) => return Ok((sq, true, t0.elapsed().as_millis() as u64)),
                Err(e) => {
                    eprintln!("gpu_gnfs ∋ u64 congruence failed — trying multi-limb: {e}");
                }
            }
        }
    }
    match crate::gpu_gnfs_cong_ml::congruence_multilimb(n, poly, relations, deps) {
        Ok(sq) => Ok((sq, true, t0.elapsed().as_millis() as u64)),
        Err(e) => {
            // Host Q(α)/norm resolve only for small N — large-N host resolve_y is a hang.
            if n.bits() <= 64 {
                eprintln!("gpu_gnfs ∋ multi-limb failed — host fallback: {e}");
                let sq = gpu_gnfs_linalg::congruence_factor(n, poly, relations, deps)?;
                Ok((sq, false, t0.elapsed().as_millis() as u64))
            } else {
                Err(e)
            }
        }
    }
}

fn congruence_u64(
    n: &BigUint,
    n_u: u64,
    poly: &PolyPair,
    relations: &[Relation],
    deps: &[Vec<usize>],
) -> Result<SquareCongruence, String> {
    use crate::native_numeral::{modulo_via_word, multiply_via_word, signed_multiply_via_word};
    if let Some(lead) = poly.f.last() {
        let g = big_gcd(modulo_via_word(lead, n).unwrap(), n.clone());
        if g > BigUint::one() && &g < n {
            return Ok(SquareCongruence {
                x: BigUint::zero(),
                y: BigUint::zero(),
                factor: Some(g),
                dep: Vec::new(),
            });
        }
    }

    let gpu = setup(selected_device())?;
    let deg = poly.degree as usize;
    if deg == 0 || deg > 7 {
        return Err(format!("deg={deg} unsupported on device"));
    }

    let mut fcoef = alloc::vec![0u64; deg + 1];
    for (i, c) in poly.f.iter().enumerate() {
        fcoef[i] = big_to_u64(c).unwrap_or(0);
    }
    let m_u = big_to_u64(&poly.m).ok_or_else(|| String::from("m wider than u64"))?;

    let mut rel_a = Vec::with_capacity(relations.len());
    let mut rel_b = Vec::with_capacity(relations.len());
    let mut rat_abs = Vec::with_capacity(relations.len());
    let mut alg_abs = Vec::with_capacity(relations.len());
    for r in relations {
        rel_a.push(r.a);
        rel_b.push(r.b);
        rat_abs.push(big_to_u64(&r.rat_abs).unwrap_or(u64::MAX));
        alg_abs.push(big_to_u64(&r.alg_abs).unwrap_or(u64::MAX));
    }

    let mut dep_offs = alloc::vec![0i32; deps.len() + 1];
    let mut dep_idx: Vec<i32> = Vec::new();
    let mut out_x_host = alloc::vec![0u64; deps.len()];
    for (di, dep) in deps.iter().enumerate() {
        dep_offs[di] = dep_idx.len() as i32;
        let mut rat = BigInt::one();
        for &i in dep {
            dep_idx.push(i as i32);
            rat = signed_multiply_via_word(&rat, &relations[i].rat_signed);
        }
        let ra = rat.abs().to_biguint().unwrap_or_else(BigUint::zero);
        let root = gpu_gnfs_linalg::isqrt(&ra);
        out_x_host[di] = if multiply_via_word(&root, &root) == ra {
            big_to_u64(&modulo_via_word(&root, n).unwrap()).unwrap_or(0)
        } else {
            0
        };
    }
    dep_offs[deps.len()] = dep_idx.len() as i32;

    let ndeps = deps.len() as i32;
    let deg_i = deg as i32;

    let d_offs = gpu
        .stream
        .clone_htod(&dep_offs)
        .map_err(|e| format!("htod offs: {e}"))?;
    let d_idx = gpu
        .stream
        .clone_htod(&dep_idx)
        .map_err(|e| format!("htod idx: {e}"))?;
    let d_a = gpu
        .stream
        .clone_htod(&rel_a)
        .map_err(|e| format!("htod a: {e}"))?;
    let d_b = gpu
        .stream
        .clone_htod(&rel_b)
        .map_err(|e| format!("htod b: {e}"))?;
    let d_f = gpu
        .stream
        .clone_htod(&fcoef)
        .map_err(|e| format!("htod f: {e}"))?;
    let d_rat = gpu
        .stream
        .clone_htod(&rat_abs)
        .map_err(|e| format!("htod rat: {e}"))?;
    let d_alg = gpu
        .stream
        .clone_htod(&alg_abs)
        .map_err(|e| format!("htod alg: {e}"))?;
    let mut d_x = gpu
        .stream
        .clone_htod(&out_x_host)
        .map_err(|e| format!("htod x: {e}"))?;
    let mut d_y = gpu
        .stream
        .alloc_zeros::<u64>(deps.len().max(1))
        .map_err(|e| format!("alloc y: {e}"))?;
    let mut d_fac = gpu
        .stream
        .alloc_zeros::<u64>(deps.len().max(1))
        .map_err(|e| format!("alloc fac: {e}"))?;
    let mut d_st = gpu
        .stream
        .alloc_zeros::<i32>(deps.len().max(1))
        .map_err(|e| format!("alloc st: {e}"))?;

    eprintln!(
        "gpu_gnfs ∋ device: φ-congruence {} deps on device {} (N={n_u})…",
        deps.len(),
        gpu.device
    );
    {
        let mut launch = gpu.stream.launch_builder(&gpu.cong);
        unsafe {
            launch.arg(&d_offs);
            launch.arg(&d_idx);
            launch.arg(&ndeps);
            launch.arg(&d_a);
            launch.arg(&d_b);
            launch.arg(&d_f);
            launch.arg(&deg_i);
            launch.arg(&n_u);
            launch.arg(&m_u);
            launch.arg(&d_rat);
            launch.arg(&d_alg);
            launch.arg(&mut d_x);
            launch.arg(&mut d_y);
            launch.arg(&mut d_fac);
            launch.arg(&mut d_st);
            launch
                .launch(LaunchConfig {
                    grid_dim: (deps.len().max(1) as u32, 1, 1),
                    block_dim: (32, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("cong launch: {e}"))?;
        }
    }
    gpu.stream
        .synchronize()
        .map_err(|e| format!("cong sync: {e}"))?;

    let factors = gpu
        .stream
        .clone_dtoh(&d_fac)
        .map_err(|e| format!("dtoh fac: {e}"))?;
    let statuses = gpu
        .stream
        .clone_dtoh(&d_st)
        .map_err(|e| format!("dtoh st: {e}"))?;
    let xs = gpu
        .stream
        .clone_dtoh(&d_x)
        .map_err(|e| format!("dtoh x: {e}"))?;
    let ys = gpu
        .stream
        .clone_dtoh(&d_y)
        .map_err(|e| format!("dtoh y: {e}"))?;

    for (di, dep) in deps.iter().enumerate() {
        if statuses[di] == 1 && factors[di] > 1 && factors[di] < n_u {
            return Ok(SquareCongruence {
                x: BigUint::from(xs[di]),
                y: BigUint::from(ys[di]),
                factor: Some(BigUint::from(factors[di])),
                dep: dep.clone(),
            });
        }
        if statuses[di] == 2 {
            let x = BigUint::from(xs[di]);
            let y = BigUint::from(ys[di]);
            if let Some(f) = try_gcd_host(&x, &y, n) {
                return Ok(SquareCongruence {
                    x,
                    y,
                    factor: Some(f),
                    dep: dep.clone(),
                });
            }
        }
    }

    for dep in deps.iter().take(16) {
        if dep.is_empty() {
            continue;
        }
        if let Some(sq) = gpu_mod_sqrt_dep(&gpu, n, n_u, poly, relations, dep, m_u, &fcoef)? {
            return Ok(sq);
        }
    }

    Err(format!(
        "gpu_gnfs ∋: device found no factor across {} deps",
        deps.len()
    ))
}

fn gpu_mod_sqrt_dep(
    gpu: &LinalgGpu,
    n: &BigUint,
    n_u: u64,
    poly: &PolyPair,
    relations: &[Relation],
    dep: &[usize],
    m_u: u64,
    fcoef: &[u64],
) -> Result<Option<SquareCongruence>, String> {
    let delta = match gpu_gnfs_linalg::product_algebraic_mod(relations, dep, poly, n) {
        Ok(d) => d,
        Err(f) => {
            if f > BigUint::one() && &f < n {
                return Ok(Some(SquareCongruence {
                    x: BigUint::zero(),
                    y: BigUint::zero(),
                    factor: Some(f),
                    dep: dep.to_vec(),
                }));
            }
            return Ok(None);
        }
    };
    let deg = poly.degree as usize;
    let mut delta_u = alloc::vec![0u64; deg];
    for i in 0..deg {
        delta_u[i] = big_to_u64(delta.get(i).unwrap_or(&BigUint::zero())).unwrap_or(0);
    }

    let d_delta = gpu
        .stream
        .clone_htod(&delta_u)
        .map_err(|e| format!("htod delta: {e}"))?;
    let d_f = gpu
        .stream
        .clone_htod(fcoef)
        .map_err(|e| format!("htod f: {e}"))?;
    let mut d_beta = gpu
        .stream
        .alloc_zeros::<u64>(deg)
        .map_err(|e| format!("alloc beta: {e}"))?;
    let mut d_found = gpu
        .stream
        .alloc_zeros::<i32>(1)
        .map_err(|e| format!("alloc found: {e}"))?;

    let bound: i64 = if n_u <= 400 { 48 } else { 28 };
    let span = (2 * bound + 1) as u64;
    let mut space = 1u64;
    for _ in 0..deg {
        space = space.saturating_mul(span);
    }
    if space == 0 || space > 80_000_000 {
        return Ok(None);
    }
    let threads = 256u32;
    let blocks = ((space + threads as u64 - 1) / threads as u64) as u32;
    let deg_i = deg as i32;
    eprintln!(
        "gpu_gnfs ∋ device: mod-√ search space={} bound={}…",
        space, bound
    );
    {
        let mut launch = gpu.stream.launch_builder(&gpu.sqrt_search);
        unsafe {
            launch.arg(&d_delta);
            launch.arg(&d_f);
            launch.arg(&deg_i);
            launch.arg(&n_u);
            launch.arg(&bound);
            launch.arg(&mut d_beta);
            launch.arg(&mut d_found);
            launch
                .launch(LaunchConfig {
                    grid_dim: (blocks.max(1), 1, 1),
                    block_dim: (threads, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("sqrt launch: {e}"))?;
        }
    }
    gpu.stream
        .synchronize()
        .map_err(|e| format!("sqrt sync: {e}"))?;
    let found = gpu
        .stream
        .clone_dtoh(&d_found)
        .map_err(|e| format!("dtoh found: {e}"))?;
    if found[0] == 0 {
        return Ok(None);
    }
    let beta = gpu
        .stream
        .clone_dtoh(&d_beta)
        .map_err(|e| format!("dtoh beta: {e}"))?;

    use crate::native_numeral::{add_via_word, modulo_via_word, multiply_via_word, signed_multiply_via_word};
    let mut acc = BigUint::zero();
    let mut pow = BigUint::one();
    let m = BigUint::from(m_u);
    for i in 0..deg {
        acc = modulo_via_word(&add_via_word(&acc, &multiply_via_word(&BigUint::from(beta[i]), &pow)), n).unwrap();
        pow = modulo_via_word(&multiply_via_word(&pow, &m), n).unwrap();
    }
    let y = acc;

    let mut rat = BigInt::one();
    for &i in dep {
        rat = signed_multiply_via_word(&rat, &relations[i].rat_signed);
    }
    let ra = rat.abs().to_biguint().unwrap_or_else(BigUint::zero);
    let root = gpu_gnfs_linalg::isqrt(&ra);
    if multiply_via_word(&root, &root) != ra {
        return Ok(None);
    }
    let x = modulo_via_word(&root, n).unwrap();
    if let Some(f) = try_gcd_host(&x, &y, n) {
        return Ok(Some(SquareCongruence {
            x,
            y,
            factor: Some(f),
            dep: dep.to_vec(),
        }));
    }
    Ok(None)
}
