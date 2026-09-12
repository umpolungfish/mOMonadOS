//! gpu_rho_ml.rs — multi-limb GPU Pollard's rho, the width gpu_rho.rs's
//! fixed 4-limb (256-bit) kernel can't reach. Same CIOS Montgomery
//! primitives as gpu_gnfs_cong_ml.rs, templated on LIMBS, with the
//! Pollard rho walker from gpu_rho.rs ported onto them unchanged in
//! structure: each GPU thread walks its own (c, x0) sequence, gcd every
//! 128 steps, first thread to land a nontrivial factor wins.
//!
//! This doesn't change what rho can reach in principle — it's still
//! O(N^(1/4)) work, the same wall the CPU and 256-bit GPU versions hit —
//! it only removes the 256-bit cap, so the real GPU-parallel search now
//! runs at any width unbraid or native_numeral hand it, real thousands
//! of parallel walkers instead of a single CPU thread past 256 bits.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};
use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::gpu_rho::n0_inv;

fn limbs_of(n: &BigUint, limbs: usize) -> Vec<u64> {
    let d = n.to_u64_digits();
    let mut v = alloc::vec![0u64; limbs];
    for i in 0..limbs.min(d.len()) {
        v[i] = d[i];
    }
    v
}

fn from_limbs(l: &[u64]) -> BigUint {
    use crate::native_numeral::{add_via_word, multiply_via_word, pow2};
    let mut n = BigUint::zero();
    let base = pow2(64);
    for i in (0..l.len()).rev() {
        n = add_via_word(&multiply_via_word(&n, &base), &BigUint::from(l[i]));
    }
    n
}

/// Same cap gpu_gnfs_cong_ml uses: (bits+63)/64, +1 guard limb, min 2, max 32
/// (2048 bits) — the CIOS montmul needs one spare limb of headroom.
fn pick_limbs(n: &BigUint) -> usize {
    let need = ((n.bits() as usize) + 63) / 64;
    (need + 1).max(2).min(32)
}

fn selected_device() -> u32 {
    std::env::var("GNFS_GPU").ok().and_then(|s| s.parse().ok()).unwrap_or(0)
}

fn kernel_src(limbs: usize) -> String {
    format!(
        r#"
#define LIMBS {L}
typedef unsigned long long u64;
typedef unsigned __int128 u128;

__device__ __noinline__ void montmul(const u64* a, const u64* b, const u64* N, u64 n0, u64* r){{
    u64 t[LIMBS+2]; for (int i=0;i<LIMBS+2;i++) t[i]=0;
    for (int i=0;i<LIMBS;i++){{
        u128 C=0;
        for (int j=0;j<LIMBS;j++){{ u128 x=(u128)a[j]*(u128)b[i]+(u128)t[j]+C; t[j]=(u64)x; C=x>>64; }}
        u128 s=(u128)t[LIMBS]+C; t[LIMBS]=(u64)s; t[LIMBS+1]=(u64)(s>>64);
        u64 m=t[0]*n0;
        u128 c2; {{ u128 x=(u128)m*(u128)N[0]+(u128)t[0]; c2=x>>64; }}
        for (int j=1;j<LIMBS;j++){{ u128 x=(u128)m*(u128)N[j]+(u128)t[j]+c2; t[j-1]=(u64)x; c2=x>>64; }}
        u128 s2=(u128)t[LIMBS]+c2; t[LIMBS-1]=(u64)s2; c2=s2>>64; t[LIMBS]=t[LIMBS+1]+(u64)c2; t[LIMBS+1]=0;
    }}
    bool ge; if (t[LIMBS]!=0) ge=true; else {{ ge=true; for(int j=LIMBS-1;j>=0;j--){{ if(t[j]!=N[j]){{ ge=t[j]>N[j]; break; }} }} }}
    if (ge){{ u128 br=0; for(int j=0;j<LIMBS;j++){{ u128 x=(u128)t[j]-(u128)N[j]-br; r[j]=(u64)x; br=(x>>64)&1; }} }}
    else {{ for(int j=0;j<LIMBS;j++) r[j]=t[j]; }}
}}
__device__ bool geqN(const u64* a, const u64* b){{ for(int j=LIMBS-1;j>=0;j--){{ if(a[j]!=b[j]) return a[j]>b[j]; }} return true; }}
__device__ void subN(const u64* a, const u64* b, u64* r){{ u128 br=0; for(int j=0;j<LIMBS;j++){{ u128 x=(u128)a[j]-(u128)b[j]-br; r[j]=(u64)x; br=(x>>64)&1; }} }}
__device__ void montadd(const u64* a, const u64* b, const u64* N, u64* r){{
    u128 c=0; u64 t[LIMBS]; for(int j=0;j<LIMBS;j++){{ u128 x=(u128)a[j]+(u128)b[j]+c; t[j]=(u64)x; c=x>>64; }}
    if (c!=0 || geqN(t,N)) subN(t,N,r); else {{ for(int j=0;j<LIMBS;j++) r[j]=t[j]; }}
}}
__device__ void montsub(const u64* a, const u64* b, const u64* N, u64* r){{
    if (geqN(a,b)) subN(a,b,r); else {{ u64 t[LIMBS]; subN(b,a,t); subN(N,t,r); }}
}}
__device__ bool is_zeroN(const u64* a){{ u64 o=0; for(int j=0;j<LIMBS;j++) o|=a[j]; return o==0; }}
__device__ void shr1N(u64* a){{ for(int j=0;j<LIMBS-1;j++) a[j]=(a[j]>>1)|(a[j+1]<<63); a[LIMBS-1]>>=1; }}
__device__ void shl1N(u64* a){{ for(int j=LIMBS-1;j>0;j--) a[j]=(a[j]<<1)|(a[j-1]>>63); a[0]<<=1; }}
__device__ __noinline__ void gcdN(const u64* aa, const u64* bb, u64* g){{
    u64 a[LIMBS],b[LIMBS]; for(int j=0;j<LIMBS;j++){{a[j]=aa[j];b[j]=bb[j];}}
    if (is_zeroN(a)){{ for(int j=0;j<LIMBS;j++) g[j]=b[j]; return; }}
    if (is_zeroN(b)){{ for(int j=0;j<LIMBS;j++) g[j]=a[j]; return; }}
    int shift=0;
    while ((((a[0]|b[0]))&1)==0){{ shr1N(a); shr1N(b); shift++; }}
    while ((a[0]&1)==0) shr1N(a);
    while (!is_zeroN(b)){{
        while ((b[0]&1)==0) shr1N(b);
        if (geqN(a,b)){{ u64 t[LIMBS]; subN(a,b,t); for(int j=0;j<LIMBS;j++) a[j]=b[j]; for(int j=0;j<LIMBS;j++) b[j]=t[j]; }}
        else {{ u64 t[LIMBS]; subN(b,a,t); for(int j=0;j<LIMBS;j++) b[j]=t[j]; }}
    }}
    for (int s=0;s<shift;s++) shl1N(a);
    for (int j=0;j<LIMBS;j++) g[j]=a[j];
}}

// Pollard rho over Montgomery arithmetic, LIMBS wide. Each thread walks its
// own sequence (distinct c and start), accumulates the product of
// differences, gcd every 128 steps. First thread to find a nontrivial
// factor writes it. Structurally identical to gpu_rho.rs's fixed 4-limb
// `rho` kernel — only the arithmetic width changed.
extern "C" __global__ void rho_ml(
    const u64* N, const u64* R2, u64 n0, const u64* one_mont,
    u64 iters, u64 seed_base,
    u64* out_factor, int* found)
{{
    unsigned int gid = blockIdx.x*blockDim.x + threadIdx.x;
    if (*found != 0) return;
    u64 cs[LIMBS]; for(int j=0;j<LIMBS;j++) cs[j]=0; cs[0]=seed_base+gid+1;
    u64 xs0[LIMBS]; for(int j=0;j<LIMBS;j++) xs0[j]=0; xs0[0]=seed_base+gid+2;
    u64 c[LIMBS], x[LIMBS], y[LIMBS], q[LIMBS];
    montmul(cs, R2, N, n0, c);
    montmul(xs0, R2, N, n0, x);
    for (int j=0;j<LIMBS;j++){{ y[j]=x[j]; q[j]=one_mont[j]; }}
    for (u64 step=1; step<=iters; step++){{
        if ((step & 1023)==0 && *found != 0) return;
        u64 t[LIMBS]; montmul(x,x,N,n0,t); montadd(t,c,N,x);
        montmul(y,y,N,n0,t); montadd(t,c,N,y);
        montmul(y,y,N,n0,t); montadd(t,c,N,y);
        u64 d[LIMBS]; montsub(x,y,N,d);
        if (is_zeroN(d)) return;
        montmul(q,d,N,n0,t); for(int j=0;j<LIMBS;j++) q[j]=t[j];
        if ((step & 127)==0){{
            u64 g[LIMBS]; gcdN(q,N,g);
            bool g1=true; for(int j=1;j<LIMBS;j++) if(g[j]!=0) g1=false; if(g[0]!=1) g1=false;
            bool gN = geqN(g,N) && geqN(N,g);
            if (!g1 && !gN && !is_zeroN(g)){{
                if (atomicCAS(found,0,1)==0){{ for(int j=0;j<LIMBS;j++) out_factor[j]=g[j]; }}
                return;
            }}
            if (gN) return;
        }}
    }}
}}

// Diagnostic only: a*b mod N via Montgomery round trip, one thread per case.
extern "C" __global__ void mont_verify_ml(
    const u64* A, const u64* B, const u64* NN, const u64* R2, const u64* n0s,
    u64* OUT, unsigned int cnt)
{{
    unsigned int g = blockIdx.x*blockDim.x + threadIdx.x;
    if (g >= cnt) return;
    const u64* a=&A[g*LIMBS]; const u64* b=&B[g*LIMBS]; const u64* N=&NN[g*LIMBS]; const u64* r2=&R2[g*LIMBS];
    u64 n0=n0s[g];
    u64 am[LIMBS], bm[LIMBS], pm[LIMBS], one[LIMBS], p[LIMBS];
    montmul(a, r2, N, n0, am);
    montmul(b, r2, N, n0, bm);
    montmul(am, bm, N, n0, pm);
    for(int j=0;j<LIMBS;j++) one[j]=0; one[0]=1;
    montmul(pm, one, N, n0, p);
    for (int j=0;j<LIMBS;j++) OUT[g*LIMBS+j]=p[j];
}}
"#,
        L = limbs
    )
}

/// Diagnostic: a*b mod N via the GPU Montgomery round trip, checked against
/// BigUint, at a chosen LIMBS width — isolates the primitives from the rho
/// search logic.
pub fn verify_mont(limbs: usize, count: usize, device: usize) -> String {
    let ctx = match CudaContext::new(device) { Ok(c) => c, Err(e) => return format!("gpu_rho_ml: no ctx: {e}") };
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = match compile_ptx_with_opts(kernel_src(limbs), opts) { Ok(p) => p, Err(e) => return format!("gpu_rho_ml: nvrtc: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m) => m, Err(e) => return format!("gpu_rho_ml: module: {e}") };
    let func = match module.load_function("mont_verify_ml") { Ok(f) => f, Err(e) => return format!("gpu_rho_ml: load: {e}") };

    use crate::native_numeral::{modulo_via_word, multiply_via_word, pow2};
    let two_pow = pow2(64 * limbs);
    let mut rng_state: u64 = 0x1234_5678 ^ (limbs as u64);
    let mut next = || { rng_state ^= rng_state << 13; rng_state ^= rng_state >> 7; rng_state ^= rng_state << 17; rng_state };

    let (mut a_f, mut b_f, mut n_f, mut r2_f) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut n0s = Vec::with_capacity(count);
    let mut refs: Vec<BigUint> = Vec::with_capacity(count);
    for _ in 0..count {
        let mut nl: Vec<u64> = (0..limbs).map(|_| next()).collect();
        nl[limbs - 1] |= 1u64 << 63;
        nl[0] |= 1;
        let bn = from_limbs(&nl);
        let a = modulo_via_word(&from_limbs(&(0..limbs).map(|_| next()).collect::<Vec<_>>()), &bn).unwrap();
        let b = modulo_via_word(&from_limbs(&(0..limbs).map(|_| next()).collect::<Vec<_>>()), &bn).unwrap();
        let r2 = modulo_via_word(&multiply_via_word(&two_pow, &two_pow), &bn).unwrap();
        a_f.extend(limbs_of(&a, limbs));
        b_f.extend(limbs_of(&b, limbs));
        n_f.extend(nl.clone());
        r2_f.extend(limbs_of(&r2, limbs));
        n0s.push(n0_inv(nl[0]));
        refs.push(modulo_via_word(&multiply_via_word(&a, &b), &bn).unwrap());
    }

    let d_a = stream.clone_htod(&a_f).unwrap();
    let d_b = stream.clone_htod(&b_f).unwrap();
    let d_n = stream.clone_htod(&n_f).unwrap();
    let d_r2 = stream.clone_htod(&r2_f).unwrap();
    let d_n0 = stream.clone_htod(&n0s).unwrap();
    let mut d_out = stream.alloc_zeros::<u64>(count * limbs).unwrap();
    let block: u32 = 128;
    let grid = ((count as u32) + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid, 1, 1), block_dim: (block, 1, 1), shared_mem_bytes: 0 };
    let cc = count as u32;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_a); b.arg(&d_b); b.arg(&d_n); b.arg(&d_r2); b.arg(&d_n0); b.arg(&mut d_out); b.arg(&cc);
    if let Err(e) = unsafe { b.launch(cfg) } { return format!("gpu_rho_ml: launch: {e}"); }
    let out = stream.clone_dtoh(&d_out).unwrap();
    let mut wrong = 0usize;
    for i in 0..count {
        let got = from_limbs(&out[i * limbs..i * limbs + limbs]);
        if got != refs[i] { wrong += 1; }
    }
    format!("gpu_rho_ml verify_mont: limbs={} count={} montmul wrong={}", limbs, count, wrong)
}

/// One nontrivial factor of odd composite n via GPU parallel rho at
/// whatever width n needs, up to 2048 bits (32 limbs). Beyond the 256-bit
/// kernel's reach, same complexity class (~N^(1/4) work), just parallel
/// and not capped at 256 bits — the wall on a genuinely balanced large
/// semiprime is unchanged, this only extends where the search can run at
/// all.
pub fn factor_once_ml(n: &BigUint, device: usize) -> Option<BigUint> {
    if n.bits() > 2048 { return None; }
    let limbs = pick_limbs(n);
    let nl = limbs_of(n, limbs);
    if nl[0] & 1 == 0 { return Some(BigUint::from(2u32)); }

    use crate::native_numeral::{modulo_via_word, multiply_via_word, pow2};
    let two_pow = pow2(64 * limbs);
    let r2 = modulo_via_word(&multiply_via_word(&two_pow, &two_pow), n).unwrap();
    let one_mont = modulo_via_word(&two_pow, n).unwrap();
    let n0 = n0_inv(nl[0]);

    let ctx = CudaContext::new(device).ok()?;
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = match compile_ptx_with_opts(kernel_src(limbs), opts) {
        Ok(p) => p,
        Err(e) => { crate::nested_eprintln!("gpu_rho_ml: NVRTC compile failed: {e}"); return None; }
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => { crate::nested_eprintln!("gpu_rho_ml: load_module failed: {e}"); return None; }
    };
    let func = match module.load_function("rho_ml") {
        Ok(f) => f,
        Err(e) => { crate::nested_eprintln!("gpu_rho_ml: load_function failed: {e}"); return None; }
    };

    let d_n = stream.clone_htod(&nl).ok()?;
    let d_r2 = stream.clone_htod(&limbs_of(&r2, limbs)).ok()?;
    let d_om = stream.clone_htod(&limbs_of(&one_mont, limbs)).ok()?;

    let threads: u32 = 4096;
    let iters: u64 = 1 << 20;
    let max_launches = 12u64;
    for l in 0..max_launches {
        let init_found: Vec<i32> = alloc::vec![0];
        let mut d_found = stream.clone_htod(&init_found).ok()?;
        let mut d_out = stream.alloc_zeros::<u64>(limbs).ok()?;
        let seed_base: u64 = 1 + l * (threads as u64) * 4;
        let cfg = LaunchConfig { grid_dim: ((threads+127)/128,1,1), block_dim:(128,1,1), shared_mem_bytes:0 };
        let mut b = stream.launch_builder(&func);
        b.arg(&d_n); b.arg(&d_r2); b.arg(&n0); b.arg(&d_om);
        b.arg(&iters); b.arg(&seed_base); b.arg(&mut d_out); b.arg(&mut d_found);
        if unsafe { b.launch(cfg) }.is_err() { return None; }
        let found = stream.clone_dtoh(&d_found).ok()?;
        if found[0] != 0 {
            let out = stream.clone_dtoh(&d_out).ok()?;
            let f = from_limbs(&out);
            if f > BigUint::one() && &f < n && crate::native_numeral::modulo_via_word(n, &f).unwrap().is_zero() { return Some(f); }
        }
    }
    None
}

pub fn run_factor(n_str: &str, device: usize) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("gpu_rho_ml: '{}' is not a valid non-negative integer", n_str),
    };
    if n.is_zero() || n == BigUint::one() {
        return format!("gpu_rho_ml: N={} has no nontrivial factor\n", n_str);
    }
    let limbs = pick_limbs(&n);
    let dev = if device != 0 { device } else { selected_device() as usize };
    match factor_once_ml(&n, dev) {
        Some(p) => {
            use crate::native_numeral::{divmod_via_word, multiply_via_word};
            let q = divmod_via_word(&n, &p).unwrap().0;
            format!(
                "gpu_rho_ml {}: bits={} limbs={} — factor {} × {} (device {}, verified p*q=N: {})\n",
                n_str, n.bits(), limbs, p, q, dev, multiply_via_word(&p, &q) == n
            )
        }
        None => format!(
            "gpu_rho_ml {}: bits={} limbs={} — no factor found within the launch budget (device {})\n",
            n_str, n.bits(), limbs, dev
        ),
    }
}
