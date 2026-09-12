//! GPU ∋ multi-limb φ-congruence — Montgomery ring product mod N on device.
//!
//! Same CIOS / gcdN primitives as gpu_ecm, templated on limb count so N beyond
//! u64 keeps the CUDA device hot through dependency → factor extraction.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use cudarc::driver::{CudaContext, CudaFunction, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions, Ptx};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

use crate::gpu_gnfs_linalg::{self, SquareCongruence};
use crate::gpu_gnfs_poly::PolyPair;
use crate::gpu_gnfs_sieve::Relation;
use crate::gpu_rho::n0_inv;
use crate::prime_winding::big_gcd;

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

fn pick_limbs(n: &BigUint) -> usize {
    let need = ((n.bits() as usize) + 63) / 64;
    (need + 1).max(2).min(32)
}

fn selected_device() -> u32 {
    std::env::var("GNFS_GPU")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn kernel_src(limbs: usize) -> String {
    format!(
        r#"
#define LIMBS {L}
typedef unsigned long long u64;
typedef long long i64;
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
__device__ bool nontrivial(const u64* g, const u64* N){{
    bool g1=true; for(int j=1;j<LIMBS;j++) if(g[j]!=0) g1=false; if(g[0]!=1) g1=false;
    bool gN=geqN(g,N)&&geqN(N,g);
    return !g1 && !gN && !is_zeroN(g);
}}
__device__ void from_mont(const u64* a, const u64* N, u64 n0, u64* r){{
    u64 one[LIMBS]; for(int j=0;j<LIMBS;j++) one[j]=0; one[0]=1;
    montmul(a, one, N, n0, r);
}}
__device__ void i64_to_mont(i64 v, const u64* N, const u64* R2, u64 n0, u64* out){{
    // reduce v mod N into a small limb vector, then to Montgomery
    u64 t[LIMBS]; for(int j=0;j<LIMBS;j++) t[j]=0;
    if (v >= 0) {{
        t[0] = (u64)v;
    }} else {{
        // -|v| ≡ N - (|v| % N). |v| fits u64 for relation a,b.
        u64 absv = (u64)(-(v + 1)) + 1ull;
        // absv < 2^63 typically; subtract from N if needed
        u64 tmp[LIMBS]; for(int j=0;j<LIMBS;j++) tmp[j]=0; tmp[0]=absv;
        if (geqN(tmp, N)) {{
            // rare for relation coords
            subN(tmp, N, tmp);
        }}
        subN(N, tmp, t);
    }}
    // if t >= N (only if v huge positive), reduce once
    if (geqN(t, N)) subN(t, N, t);
    montmul(t, R2, N, n0, out);
}}

extern "C" __global__ void gnfs_cong_ml(
    const int* dep_offs, const int* dep_idx, int ndeps,
    const i64* rel_a, const i64* rel_b,
    const u64* fcoef_mont, int deg,
    const u64* N, const u64* R2, u64 n0,
    const u64* inv_lead_mont, const u64* m_mont,
    const u64* out_x, // precomputed x = √(∏ rat) % N, LIMBS each
    u64* out_y, u64* out_factor, int* out_status)
{{
    int d = (int)blockIdx.x;
    if (d >= ndeps) return;
    if (threadIdx.x != 0) return;

    out_status[d] = 0;
    for (int j=0;j<LIMBS;j++) {{ out_y[d*LIMBS+j]=0; out_factor[d*LIMBS+j]=0; }}

    int lo = dep_offs[d];
    int hi = dep_offs[d + 1];
    if (hi <= lo) return;

    // lead factor check (normal form from mont)
    u64 lead_n[LIMBS];
    from_mont(&fcoef_mont[deg * LIMBS], N, n0, lead_n);
    u64 g0[LIMBS]; gcdN(lead_n, N, g0);
    if (nontrivial(g0, N)) {{
        for (int j=0;j<LIMBS;j++) out_factor[d*LIMBS+j]=g0[j];
        out_status[d] = 1;
        return;
    }}

    // acc = 1 in Montgomery
    u64 acc[8][LIMBS];
    for (int i=0;i<8;i++) for (int j=0;j<LIMBS;j++) acc[i][j]=0;
    // one_mont = R mod N = montmul(1, R2)
    u64 one[LIMBS]; for(int j=0;j<LIMBS;j++) one[j]=0; one[0]=1;
    montmul(one, R2, N, n0, acc[0]);

    for (int k = lo; k < hi; k++) {{
        int ri = dep_idx[k];
        i64 a = rel_a[ri];
        i64 b = rel_b[ri];
        u64 lin[8][LIMBS];
        for (int i=0;i<8;i++) for (int j=0;j<LIMBS;j++) lin[i][j]=0;
        i64_to_mont(a, N, R2, n0, lin[0]);
        if (deg > 1) i64_to_mont(-b, N, R2, n0, lin[1]);

        u64 prod[16][LIMBS];
        for (int i=0;i<16;i++) for (int j=0;j<LIMBS;j++) prod[i][j]=0;
        for (int i=0;i<deg;i++) {{
            for (int jj=0;jj<deg;jj++) {{
                u64 term[LIMBS];
                montmul(acc[i], lin[jj], N, n0, term);
                u64 sum[LIMBS];
                montadd(prod[i+jj], term, N, sum);
                for (int t=0;t<LIMBS;t++) prod[i+jj][t]=sum[t];
            }}
        }}
        for (int e = 2*deg-2; e >= deg; e--) {{
            int allz = 1;
            for (int t=0;t<LIMBS;t++) if (prod[e][t]) allz=0;
            if (allz) continue;
            u64 coef[LIMBS]; for(int t=0;t<LIMBS;t++) coef[t]=prod[e][t];
            for (int t=0;t<LIMBS;t++) prod[e][t]=0;
            u64 scale[LIMBS];
            montmul(coef, inv_lead_mont, N, n0, scale);
            int shift = e - deg;
            for (int t2=0;t2<deg;t2++) {{
                u64 term[LIMBS];
                montmul(scale, &fcoef_mont[t2*LIMBS], N, n0, term);
                u64 cur[LIMBS];
                montsub(prod[shift+t2], term, N, cur);
                for (int t=0;t<LIMBS;t++) prod[shift+t2][t]=cur[t];
            }}
        }}
        for (int i=0;i<deg;i++) for (int t=0;t<LIMBS;t++) acc[i][t]=prod[i][t];
        for (int i=deg;i<8;i++) for (int t=0;t<LIMBS;t++) acc[i][t]=0;
    }}

    // φ = Σ acc[i] * m^i  (Montgomery); powv starts as one_mont
    u64 phi_m[LIMBS]; for(int t=0;t<LIMBS;t++) phi_m[t]=0;
    u64 powv[LIMBS];
    {{ u64 one2[LIMBS]; for(int j=0;j<LIMBS;j++) one2[j]=0; one2[0]=1; montmul(one2, R2, N, n0, powv); }}
    for (int i=0;i<deg;i++) {{
        u64 term[LIMBS];
        montmul(acc[i], powv, N, n0, term);
        u64 sum[LIMBS];
        montadd(phi_m, term, N, sum);
        for (int t=0;t<LIMBS;t++) phi_m[t]=sum[t];
        u64 np[LIMBS];
        montmul(powv, m_mont, N, n0, np);
        for (int t=0;t<LIMBS;t++) powv[t]=np[t];
    }}

    u64 y[LIMBS];
    from_mont(phi_m, N, n0, y);
    for (int t=0;t<LIMBS;t++) out_y[d*LIMBS+t]=y[t];

    const u64* x = &out_x[d*LIMBS];
    // candidates: |x-y|, (x+y) mod N, x, y — then gcd with N
    u64 cands[4][LIMBS];
    if (geqN(x, y)) subN(x, y, cands[0]); else subN(y, x, cands[0]);
    {{
        // x,y < N ⇒ x+y < 2N: add then conditional subtract N (may need high limb)
        u128 c=0;
        for (int j=0;j<LIMBS;j++) {{
            u128 s=(u128)x[j]+(u128)y[j]+c; cands[1][j]=(u64)s; c=s>>64;
        }}
        if (c!=0 || geqN(cands[1], N)) {{
            u64 tmp[LIMBS]; subN(cands[1], N, tmp);
            for (int j=0;j<LIMBS;j++) cands[1][j]=tmp[j];
            // if carry was set, x+y = low + 2^(64L); after one N-sub still need another
            // because 2^(64L) ≡ 0 in our limb width — carry means value = low + 2^(64L) ≥ 2^(64L) > N
            // Correct: value = low + c*2^(64L). Since x+y < 2N < 2^(64L) for N fitting in LIMBS-1
            // with spare limb, c should be 0. If c==1, N uses full width: value-N = low+(2^(64L)-N).
            if (c!=0) {{
                u64 twoL_minus_N[LIMBS];
                // 2^(64L) - N = -N mod 2^(64L) = bitwise... sub from zero with borrow chain:
                // compute ~N+1
                u128 br=1;
                for (int j=0;j<LIMBS;j++) {{
                    u128 x=~(u128)N[j] + br; twoL_minus_N[j]=(u64)x; br=x>>64;
                }}
                u128 cc=0;
                for (int j=0;j<LIMBS;j++) {{
                    u128 s=(u128)cands[1][j]+(u128)twoL_minus_N[j]+cc;
                    cands[1][j]=(u64)s; cc=s>>64;
                }}
            }}
        }}
    }}
    for (int t=0;t<LIMBS;t++) {{ cands[2][t]=x[t]; cands[3][t]=y[t]; }}

    for (int i=0;i<4;i++) {{
        if (is_zeroN(cands[i])) continue;
        u64 g[LIMBS]; gcdN(cands[i], N, g);
        if (nontrivial(g, N)) {{
            for (int j=0;j<LIMBS;j++) out_factor[d*LIMBS+j]=g[j];
            out_status[d] = 1;
            return;
        }}
    }}
    out_status[d] = 2;
}}
"#,
        L = limbs
    )
}

fn ptx_path(limbs: usize) -> String {
    let src = kernel_src(limbs);
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in src.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("target/gnfs_cong_ml_L{limbs}_{h:016x}.ptx")
}

fn load_ptx(limbs: usize) -> Result<Ptx, String> {
    let path = ptx_path(limbs);
    if std::path::Path::new(&path).is_file() {
        eprintln!("gpu_gnfs ∋ GPU: cached multi-limb PTX → {path}");
        return Ok(Ptx::from_file(&path));
    }
    eprintln!("gpu_gnfs ∋ GPU: NVRTC compiling multi-limb congruence (L={limbs})…");
    let t0 = std::time::Instant::now();
    let opts = CompileOptions {
        options: alloc::vec!["--device-int128".into()],
        ..Default::default()
    };
    let ptx = compile_ptx_with_opts(kernel_src(limbs), opts).map_err(|e| format!("NVRTC: {e}"))?;
    let src = ptx.to_src();
    let _ = std::fs::write(&path, src.as_bytes());
    eprintln!(
        "gpu_gnfs ∋ GPU: NVRTC {} ms — cached {path}",
        t0.elapsed().as_millis()
    );
    Ok(Ptx::from_src(src))
}

/// Multi-limb φ-congruence on device. Host still builds √(rational product) as x.
pub fn congruence_multilimb(
    n: &BigUint,
    poly: &PolyPair,
    relations: &[Relation],
    deps: &[Vec<usize>],
) -> Result<SquareCongruence, String> {
    use crate::native_numeral::{modulo_via_word, multiply_via_word, pow2};
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

    let limbs = pick_limbs(n);
    let deg = poly.degree as usize;
    if deg == 0 || deg > 7 {
        return Err(format!("deg={deg} unsupported on multi-limb device path"));
    }
    if n.to_u64_digits().first().copied().unwrap_or(0) & 1 == 0 {
        return Err(String::from("N even"));
    }

    let device = selected_device();
    let ctx = CudaContext::new(device as usize).map_err(|e| format!("CudaContext: {e}"))?;
    let stream = ctx.default_stream();
    let module = ctx
        .load_module(load_ptx(limbs)?)
        .map_err(|e| format!("load_module: {e}"))?;
    let cong: CudaFunction = module
        .load_function("gnfs_cong_ml")
        .map_err(|e| format!("cong_ml: {e}"))?;

    let r = pow2(64 * limbs);
    let r2 = modulo_via_word(&multiply_via_word(&r, &r), n).unwrap();
    let n0 = n0_inv(n.to_u64_digits()[0]);
    let nl = limbs_of(n, limbs);
    let r2l = limbs_of(&r2, limbs);

    // fcoef into Montgomery: (c % N) * R2 * R^{-1} = to_mont(c%N)
    // Host: to_mont(c) = (c % N) * R % N  … actually montmul(c, R2) on device;
    // here precompute (c%N * R) % N = to_mont which equals c*R mod N.
    let mut fcoef_mont = alloc::vec![0u64; (deg + 1) * limbs];
    for (i, c) in poly.f.iter().enumerate() {
        let cm = modulo_via_word(&multiply_via_word(&modulo_via_word(c, n).unwrap(), &r), n).unwrap();
        let L = limbs_of(&cm, limbs);
        fcoef_mont[i * limbs..(i + 1) * limbs].copy_from_slice(&L);
    }
    let lead = &poly.f[deg];
    let lead_mod = modulo_via_word(lead, n).unwrap();
    let inv_lead = mod_inv_big(&lead_mod, n).ok_or_else(|| {
        format!(
            "no inv lead — gcd={}",
            big_gcd(lead_mod.clone(), n.clone())
        )
    })?;
    let inv_lead_mont = limbs_of(&modulo_via_word(&multiply_via_word(&inv_lead, &r), n).unwrap(), limbs);
    let m_mont = limbs_of(&modulo_via_word(&multiply_via_word(&modulo_via_word(&poly.m, n).unwrap(), &r), n).unwrap(), limbs);

    let mut dep_offs = alloc::vec![0i32; deps.len() + 1];
    let mut dep_idx: Vec<i32> = Vec::new();
    let mut out_x = alloc::vec![0u64; deps.len().max(1) * limbs];
    let mut rel_a = Vec::with_capacity(relations.len());
    let mut rel_b = Vec::with_capacity(relations.len());
    for r in relations {
        rel_a.push(r.a);
        rel_b.push(r.b);
    }

    for (di, dep) in deps.iter().enumerate() {
        dep_offs[di] = dep_idx.len() as i32;
        let mut rat = BigInt::one();
        for &i in dep {
            dep_idx.push(i as i32);
            rat = crate::native_numeral::signed_multiply_via_word(&rat, &relations[i].rat_signed);
        }
        let ra = rat.abs().to_biguint().unwrap_or_else(BigUint::zero);
        let root = gpu_gnfs_linalg::isqrt(&ra);
        let x = if multiply_via_word(&root, &root) == ra {
            modulo_via_word(&root, n).unwrap()
        } else {
            BigUint::zero()
        };
        let xl = limbs_of(&x, limbs);
        out_x[di * limbs..(di + 1) * limbs].copy_from_slice(&xl);
    }
    dep_offs[deps.len()] = dep_idx.len() as i32;

    let ndeps = deps.len() as i32;
    let deg_i = deg as i32;

    let d_offs = stream
        .clone_htod(&dep_offs)
        .map_err(|e| format!("htod offs: {e}"))?;
    let d_idx = stream
        .clone_htod(&dep_idx)
        .map_err(|e| format!("htod idx: {e}"))?;
    let d_a = stream
        .clone_htod(&rel_a)
        .map_err(|e| format!("htod a: {e}"))?;
    let d_b = stream
        .clone_htod(&rel_b)
        .map_err(|e| format!("htod b: {e}"))?;
    let d_f = stream
        .clone_htod(&fcoef_mont)
        .map_err(|e| format!("htod f: {e}"))?;
    let d_n = stream
        .clone_htod(&nl)
        .map_err(|e| format!("htod N: {e}"))?;
    let d_r2 = stream
        .clone_htod(&r2l)
        .map_err(|e| format!("htod R2: {e}"))?;
    let d_inv = stream
        .clone_htod(&inv_lead_mont)
        .map_err(|e| format!("htod inv: {e}"))?;
    let d_m = stream
        .clone_htod(&m_mont)
        .map_err(|e| format!("htod m: {e}"))?;
    let d_x = stream
        .clone_htod(&out_x)
        .map_err(|e| format!("htod x: {e}"))?;
    let mut d_y = stream
        .alloc_zeros::<u64>(deps.len().max(1) * limbs)
        .map_err(|e| format!("alloc y: {e}"))?;
    let mut d_fac = stream
        .alloc_zeros::<u64>(deps.len().max(1) * limbs)
        .map_err(|e| format!("alloc fac: {e}"))?;
    let mut d_st = stream
        .alloc_zeros::<i32>(deps.len().max(1))
        .map_err(|e| format!("alloc st: {e}"))?;

    eprintln!(
        "gpu_gnfs ∋ device: multi-limb φ-congruence {} deps limbs={} bits={} on device {}…",
        deps.len(),
        limbs,
        n.bits(),
        device
    );

    {
        let mut launch = stream.launch_builder(&cong);
        unsafe {
            launch.arg(&d_offs);
            launch.arg(&d_idx);
            launch.arg(&ndeps);
            launch.arg(&d_a);
            launch.arg(&d_b);
            launch.arg(&d_f);
            launch.arg(&deg_i);
            launch.arg(&d_n);
            launch.arg(&d_r2);
            launch.arg(&n0);
            launch.arg(&d_inv);
            launch.arg(&d_m);
            launch.arg(&d_x);
            launch.arg(&mut d_y);
            launch.arg(&mut d_fac);
            launch.arg(&mut d_st);
            launch
                .launch(LaunchConfig {
                    grid_dim: (deps.len().max(1) as u32, 1, 1),
                    block_dim: (32, 1, 1),
                    shared_mem_bytes: 0,
                })
                .map_err(|e| format!("cong_ml launch: {e}"))?;
        }
    }
    stream
        .synchronize()
        .map_err(|e| format!("cong_ml sync: {e}"))?;

    let factors = stream
        .clone_dtoh(&d_fac)
        .map_err(|e| format!("dtoh fac: {e}"))?;
    let statuses = stream
        .clone_dtoh(&d_st)
        .map_err(|e| format!("dtoh st: {e}"))?;
    let ys = stream
        .clone_dtoh(&d_y)
        .map_err(|e| format!("dtoh y: {e}"))?;

    for (di, dep) in deps.iter().enumerate() {
        if statuses[di] == 1 {
            let f = from_limbs(&factors[di * limbs..(di + 1) * limbs]);
            if f > BigUint::one() && &f < n {
                let x = from_limbs(&out_x[di * limbs..(di + 1) * limbs]);
                let y = from_limbs(&ys[di * limbs..(di + 1) * limbs]);
                return Ok(SquareCongruence {
                    x,
                    y,
                    factor: Some(f),
                    dep: dep.clone(),
                });
            }
        }
        if statuses[di] == 2 {
            let x = from_limbs(&out_x[di * limbs..(di + 1) * limbs]);
            let y = from_limbs(&ys[di * limbs..(di + 1) * limbs]);
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

    Err(format!(
        "gpu_gnfs ∋: multi-limb device found no factor across {} deps (L={limbs})",
        deps.len()
    ))
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

fn mod_inv_big(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{
        signed_add_via_word_general, signed_divmod_via_word, signed_multiply_via_word,
        signed_subtract_via_word_general,
    };
    // Extended Euclid
    let mut t = BigInt::zero();
    let mut nt = BigInt::one();
    let mut r = BigInt::from_biguint(num_bigint::Sign::Plus, n.clone());
    let mut nr = BigInt::from_biguint(num_bigint::Sign::Plus, a.clone());
    while !nr.is_zero() {
        let q = signed_divmod_via_word(&r, &nr).unwrap().0;
        let tmp = nt.clone();
        nt = signed_subtract_via_word_general(&t, &signed_multiply_via_word(&q, &nt));
        t = tmp;
        let tmp = nr.clone();
        nr = signed_subtract_via_word_general(&r, &signed_multiply_via_word(&q, &nr));
        r = tmp;
    }
    if r != BigInt::one() && r != -BigInt::one() {
        return None;
    }
    if r == -BigInt::one() {
        t = -t;
    }
    if t.sign() == num_bigint::Sign::Minus {
        t = signed_add_via_word_general(&t, &BigInt::from_biguint(num_bigint::Sign::Plus, n.clone()));
    }
    t.to_biguint()
}
