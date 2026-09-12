//! gpu_ecm.rs — Lenstra elliptic-curve factorization on the GPU, any width.
//!
//! One Montgomery curve per thread over a Montgomery field whose limb count is
//! chosen to fit N, so there is no size ceiling on the input. Each thread scales
//! its point by the product of prime powers up to B1 with the x-only ladder,
//! then takes gcd(Z, N); a curve whose order mod a factor p is B1-smooth drives
//! Z to 0 mod p and the gcd reveals p. Thousands of curves at once, first factor
//! wins. The arithmetic is verified against BigUint at each width before use.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions, Ptx};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use crate::gpu_rho::n0_inv;

/// The ~250-line CIOS kernel had no PTX cache at all — every call to
/// factor_once/factor_once_bsgs/verify_width re-ran NVRTC from scratch on
/// the full source, which is why a 100-case verify at 4 limbs took minutes
/// instead of the near-instant runs gpu_rho_ml and gpu_gnfs already get
/// from caching. Same pattern as gpu_gnfs_fb.rs: hash the generated source
/// (it depends on `limbs`, so different widths get different cache files
/// automatically), load the cached PTX if present, else compile once and
/// write it out.
fn ptx_path(limbs: usize, src: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in src.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("target/gpu_ecm_{limbs}_{h:016x}.ptx")
}

fn load_ptx_cached(limbs: usize) -> Result<Ptx, String> {
    let src = kernel_src(limbs);
    let path = ptx_path(limbs, &src);
    if std::path::Path::new(&path).is_file() {
        return Ok(Ptx::from_file(&path));
    }
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = compile_ptx_with_opts(src, opts).map_err(|e| format!("NVRTC: {e}"))?;
    let out = ptx.to_src();
    let _ = std::fs::write(&path, out.as_bytes());
    Ok(Ptx::from_src(out))
}

fn kernel_src(limbs: usize) -> String {
    // Independent stage-2 accumulators for latency hiding, fewer as limbs (and
    // so per-accumulator register cost) grow, to avoid spilling.
    let nacc = if limbs <= 6 { 6 } else if limbs <= 11 { 4 } else { 2 };
    format!(r#"
#define LIMBS {L}
#define NACC {A}
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
__device__ __noinline__ void xDBL(const u64* X,const u64* Z,const u64* a24,const u64* N,u64 n0,u64* OX,u64* OZ){{
    u64 t1[LIMBS],t2[LIMBS],tt[LIMBS],tmp[LIMBS],x2[LIMBS],z2[LIMBS];
    montadd(X,Z,N,t1); montmul(t1,t1,N,n0,t1);
    montsub(X,Z,N,t2); montmul(t2,t2,N,n0,t2);
    montmul(t1,t2,N,n0,x2);
    montsub(t1,t2,N,tt);
    montmul(a24,tt,N,n0,tmp); montadd(tmp,t2,N,tmp);
    montmul(tt,tmp,N,n0,z2);
    for(int j=0;j<LIMBS;j++){{ OX[j]=x2[j]; OZ[j]=z2[j]; }}
}}
__device__ __noinline__ void xADD(const u64* X0,const u64* Z0,const u64* X1,const u64* Z1,
                     const u64* Xd,const u64* Zd,const u64* N,u64 n0,u64* OX,u64* OZ){{
    u64 t1[LIMBS],t2[LIMBS],a[LIMBS],b[LIMBS],ox[LIMBS],oz[LIMBS];
    montsub(X0,Z0,N,t1); montadd(X1,Z1,N,t2); montmul(t1,t2,N,n0,a);
    montadd(X0,Z0,N,t1); montsub(X1,Z1,N,t2); montmul(t1,t2,N,n0,b);
    montadd(a,b,N,t1); montmul(t1,t1,N,n0,t1); montmul(Zd,t1,N,n0,ox);
    montsub(a,b,N,t2); montmul(t2,t2,N,n0,t2); montmul(Xd,t2,N,n0,oz);
    for(int j=0;j<LIMBS;j++){{ OX[j]=ox[j]; OZ[j]=oz[j]; }}
}}

// a*b mod N (via Montgomery) for verification.
extern "C" __global__ void mont_verify(const u64* A,const u64* B,const u64* NN,const u64* R2,const u64* n0s,u64* OUT,unsigned int cnt){{
    unsigned int g=blockIdx.x*blockDim.x+threadIdx.x; if(g>=cnt) return;
    const u64* a=&A[g*LIMBS]; const u64* b=&B[g*LIMBS]; const u64* N=&NN[g*LIMBS]; const u64* r2=&R2[g*LIMBS];
    u64 n0=n0s[g];
    u64 am[LIMBS],bm[LIMBS],pm[LIMBS],one[LIMBS],p[LIMBS];
    montmul(a,r2,N,n0,am); montmul(b,r2,N,n0,bm); montmul(am,bm,N,n0,pm);
    for(int j=0;j<LIMBS;j++) one[j]=0; one[0]=1;
    montmul(pm,one,N,n0,p);
    for(int j=0;j<LIMBS;j++) OUT[g*LIMBS+j]=p[j];
}}

// s*(PX:PZ) by the x-only ladder, s a positive u64.
__device__ __noinline__ void ladder_u64(u64 s, const u64* PX, const u64* PZ, const u64* a24,
                           const u64* N, u64 n0, u64* OX, u64* OZ){{
    int hb=63; while (hb>0 && !((s>>hb)&1ULL)) hb--;
    u64 R0X[LIMBS],R0Z[LIMBS],R1X[LIMBS],R1Z[LIMBS];
    for(int j=0;j<LIMBS;j++){{ R0X[j]=PX[j]; R0Z[j]=PZ[j]; }}
    xDBL(PX,PZ,a24,N,n0,R1X,R1Z);
    for (int i=hb-1;i>=0;i--){{
        if ((s>>i)&1ULL){{ xADD(R0X,R0Z,R1X,R1Z,PX,PZ,N,n0,R0X,R0Z); xDBL(R1X,R1Z,a24,N,n0,R1X,R1Z); }}
        else            {{ xADD(R0X,R0Z,R1X,R1Z,PX,PZ,N,n0,R1X,R1Z); xDBL(R0X,R0Z,a24,N,n0,R0X,R0Z); }}
    }}
    for(int j=0;j<LIMBS;j++){{ OX[j]=R0X[j]; OZ[j]=R0Z[j]; }}
}}
__device__ bool nontrivial(const u64* g, const u64* N){{
    bool g1=true; for(int j=1;j<LIMBS;j++) if(g[j]!=0) g1=false; if(g[0]!=1) g1=false;
    bool gN=geqN(g,N)&&geqN(N,g);
    return !g1 && !gN && !is_zeroN(g);
}}

extern "C" __global__ void ecm(
    const u64* N, const u64* R2, u64 n0,
    const u64* one_mont, const u64* two_mont, const u64* inv4_mont,
    const unsigned char* kbits, int nbits, u64 sigma_base, u64 b1v, u64 b2v,
    u64* out_factor, int* found)
{{
    unsigned int gid=blockIdx.x*blockDim.x+threadIdx.x;
    if (*found!=0) return;
    u64 A[LIMBS],a24[LIMBS],t[LIMBS];
    u64 sig[LIMBS]; for(int j=0;j<LIMBS;j++) sig[j]=0; sig[0]=sigma_base+gid+6;
    montmul(sig,R2,N,n0,A);
    montadd(A,two_mont,N,t);
    montmul(t,inv4_mont,N,n0,a24);
    u64 PX[LIMBS],PZ[LIMBS];
    u64 xs[LIMBS]; for(int j=0;j<LIMBS;j++) xs[j]=0; xs[0]=sigma_base+gid+2;
    montmul(xs,R2,N,n0,PX);
    for(int j=0;j<LIMBS;j++) PZ[j]=one_mont[j];
    // stage 1: Q = k*P
    u64 R0X[LIMBS],R0Z[LIMBS],R1X[LIMBS],R1Z[LIMBS];
    for(int j=0;j<LIMBS;j++){{ R0X[j]=PX[j]; R0Z[j]=PZ[j]; }}
    xDBL(PX,PZ,a24,N,n0,R1X,R1Z);
    for (int i=nbits-2;i>=0;i--){{
        unsigned char bit=kbits[i];
        if (bit){{ xADD(R0X,R0Z,R1X,R1Z,PX,PZ,N,n0,R0X,R0Z); xDBL(R1X,R1Z,a24,N,n0,R1X,R1Z); }}
        else    {{ xADD(R0X,R0Z,R1X,R1Z,PX,PZ,N,n0,R1X,R1Z); xDBL(R0X,R0Z,a24,N,n0,R0X,R0Z); }}
    }}
    u64 g[LIMBS]; gcdN(R0Z,N,g);
    if (nontrivial(g,N)){{ if(atomicCAS(found,0,1)==0){{ for(int j=0;j<LIMBS;j++) out_factor[j]=g[j]; }} return; }}

    // stage 2: walk odd multiples m*Q for m in (b1, b2], accumulate their Z, gcd.
    if (b2v > b1v){{
        u64 QX[LIMBS],QZ[LIMBS], twoX[LIMBS],twoZ[LIMBS];
        for(int j=0;j<LIMBS;j++){{ QX[j]=R0X[j]; QZ[j]=R0Z[j]; }}
        xDBL(QX,QZ,a24,N,n0,twoX,twoZ);
        u64 m0 = (b1v & 1ULL) ? (b1v + 2ULL) : (b1v + 1ULL);   // first odd > b1
        u64 curX[LIMBS],curZ[LIMBS], prevX[LIMBS],prevZ[LIMBS], nX[LIMBS],nZ[LIMBS];
        ladder_u64(m0, QX,QZ,a24,N,n0,curX,curZ);
        ladder_u64(m0-2ULL, QX,QZ,a24,N,n0,prevX,prevZ);
        u64 acc[LIMBS]; for(int j=0;j<LIMBS;j++) acc[j]=one_mont[j];
        u64 cnt=0;
        for (u64 m=m0; m<=b2v; m+=2ULL){{
            u64 tmp[LIMBS]; montmul(acc,curZ,N,n0,tmp); for(int j=0;j<LIMBS;j++) acc[j]=tmp[j];
            xADD(curX,curZ,twoX,twoZ,prevX,prevZ,N,n0,nX,nZ);
            for(int j=0;j<LIMBS;j++){{ prevX[j]=curX[j]; prevZ[j]=curZ[j]; curX[j]=nX[j]; curZ[j]=nZ[j]; }}
            if (((++cnt)&2047ULL)==0){{
                gcdN(acc,N,g);
                if (nontrivial(g,N)){{ if(atomicCAS(found,0,1)==0){{ for(int j=0;j<LIMBS;j++) out_factor[j]=g[j]; }} return; }}
                if (*found!=0) return;
                for(int j=0;j<LIMBS;j++) acc[j]=one_mont[j];
            }}
        }}
        gcdN(acc,N,g);
        if (nontrivial(g,N)){{ if(atomicCAS(found,0,1)==0){{ for(int j=0;j<LIMBS;j++) out_factor[j]=g[j]; }} }}
    }}
}}

// Per-block BSGS stage 2: ONE curve per block. The block's threads build a
// shared baby table j*Q for j coprime to D in (0,D/2], then split the giant
// steps i*D*Q over (B1,B2] among themselves. A giant meeting a baby modulo a
// factor p makes the cross-difference X_T*Z_j - X_j*Z_T vanish mod p; each
// thread accumulates that difference over its giants and takes one gcd.
extern "C" __global__ void ecm_bsgs(
    const u64* N, const u64* R2, u64 n0,
    const u64* one_mont, const u64* two_mont, const u64* inv4_mont,
    const unsigned char* kbits, int nbits, u64 sigma_base,
    const unsigned int* jlist, int nj, u64 D, u64 i_min, u64 i_max,
    u64* out_factor, int* found)
{{
    extern __shared__ u64 smem[];
    u64* babyX = smem;                       // nj*LIMBS
    u64* babyZ = babyX + (u64)nj*LIMBS;      // nj*LIMBS
    u64* Qsh   = babyZ + (u64)nj*LIMBS;      // LIMBS
    u64* QZsh  = Qsh + LIMBS;                 // LIMBS
    u64* DQXsh = QZsh + LIMBS;                // LIMBS
    u64* DQZsh = DQXsh + LIMBS;               // LIMBS
    u64* a24sh = DQZsh + LIMBS;               // LIMBS
    int t = threadIdx.x;
    unsigned int curve = blockIdx.x;
    if (*found!=0) return;

    if (t==0){{
        u64 A[LIMBS],a24[LIMBS],tt[LIMBS];
        u64 sig[LIMBS]; for(int j=0;j<LIMBS;j++) sig[j]=0; sig[0]=sigma_base+curve+6;
        montmul(sig,R2,N,n0,A);
        montadd(A,two_mont,N,tt);
        montmul(tt,inv4_mont,N,n0,a24);
        for(int j=0;j<LIMBS;j++) a24sh[j]=a24[j];
        u64 PX[LIMBS],PZ[LIMBS];
        u64 xs[LIMBS]; for(int j=0;j<LIMBS;j++) xs[j]=0; xs[0]=sigma_base+curve+2;
        montmul(xs,R2,N,n0,PX);
        for(int j=0;j<LIMBS;j++) PZ[j]=one_mont[j];
        u64 R0X[LIMBS],R0Z[LIMBS],R1X[LIMBS],R1Z[LIMBS];
        for(int j=0;j<LIMBS;j++){{ R0X[j]=PX[j]; R0Z[j]=PZ[j]; }}
        xDBL(PX,PZ,a24,N,n0,R1X,R1Z);
        for (int i=nbits-2;i>=0;i--){{
            if (kbits[i]){{ xADD(R0X,R0Z,R1X,R1Z,PX,PZ,N,n0,R0X,R0Z); xDBL(R1X,R1Z,a24,N,n0,R1X,R1Z); }}
            else    {{ xADD(R0X,R0Z,R1X,R1Z,PX,PZ,N,n0,R1X,R1Z); xDBL(R0X,R0Z,a24,N,n0,R0X,R0Z); }}
        }}
        for(int j=0;j<LIMBS;j++){{ Qsh[j]=R0X[j]; QZsh[j]=R0Z[j]; }}
        u64 g[LIMBS]; gcdN(R0Z,N,g);
        if (nontrivial(g,N)){{ if(atomicCAS(found,0,1)==0){{ for(int j=0;j<LIMBS;j++) out_factor[j]=g[j]; }} }}
        u64 dqx[LIMBS],dqz[LIMBS];
        ladder_u64(D, R0X,R0Z,a24,N,n0,dqx,dqz);
        for(int j=0;j<LIMBS;j++){{ DQXsh[j]=dqx[j]; DQZsh[j]=dqz[j]; }}
    }}
    __syncthreads();
    if (*found!=0) return;

    u64 a24[LIMBS]; for(int j=0;j<LIMBS;j++) a24[j]=a24sh[j];
    u64 QX[LIMBS],QZ[LIMBS]; for(int j=0;j<LIMBS;j++){{ QX[j]=Qsh[j]; QZ[j]=QZsh[j]; }}

    for (int idx=t; idx<nj; idx+=blockDim.x){{
        u64 bx[LIMBS],bz[LIMBS];
        ladder_u64((u64)jlist[idx], QX,QZ,a24,N,n0,bx,bz);
        for(int j=0;j<LIMBS;j++){{ babyX[idx*LIMBS+j]=bx[j]; babyZ[idx*LIMBS+j]=bz[j]; }}
    }}
    __syncthreads();

    u64 DQX[LIMBS],DQZ[LIMBS]; for(int j=0;j<LIMBS;j++){{ DQX[j]=DQXsh[j]; DQZ[j]=DQZsh[j]; }}
    // NACC independent product chains: consecutive folds hit different
    // accumulators, so several montmuls are in flight instead of one serial
    // chain waiting on itself.
    u64 acc[NACC][LIMBS];
    for(int a=0;a<NACC;a++) for(int j=0;j<LIMBS;j++) acc[a][j]=one_mont[j];
    int lane=0;
    for (u64 i=i_min+(u64)t; i<=i_max; i+=blockDim.x){{
        if (*found!=0) return;
        u64 TX[LIMBS],TZ[LIMBS];
        ladder_u64(i, DQX,DQZ,a24,N,n0,TX,TZ);
        for (int idx=0; idx<nj; idx++){{
            u64 p1[LIMBS],p2[LIMBS],df[LIMBS];
            montmul(TX,&babyZ[idx*LIMBS],N,n0,p1);
            montmul(&babyX[idx*LIMBS],TZ,N,n0,p2);
            montsub(p1,p2,N,df);
            u64 tmp[LIMBS]; montmul(acc[lane],df,N,n0,tmp); for(int j=0;j<LIMBS;j++) acc[lane][j]=tmp[j];
            lane++; if (lane==NACC) lane=0;
        }}
    }}
    u64 prod[LIMBS]; for(int j=0;j<LIMBS;j++) prod[j]=acc[0][j];
    for(int a=1;a<NACC;a++){{ u64 tmp[LIMBS]; montmul(prod,acc[a],N,n0,tmp); for(int j=0;j<LIMBS;j++) prod[j]=tmp[j]; }}
    u64 g[LIMBS]; gcdN(prod,N,g);
    if (nontrivial(g,N)){{ if(atomicCAS(found,0,1)==0){{ for(int j=0;j<LIMBS;j++) out_factor[j]=g[j]; }} }}
}}
"#, L = limbs, A = nacc)
}

fn limbs_of(n: &BigUint, limbs: usize) -> Vec<u64> {
    let d = n.to_u64_digits();
    let mut v = alloc::vec![0u64; limbs];
    for i in 0..limbs { if i < d.len() { v[i] = d[i]; } }
    v
}
fn from_limbs(l: &[u64]) -> BigUint {
    use crate::native_numeral::{add_via_word, multiply_via_word, pow2};
    let mut n = BigUint::zero();
    let base = pow2(64);
    for i in (0..l.len()).rev() { n = add_via_word(&multiply_via_word(&n, &base), &BigUint::from(l[i])); }
    n
}

/// Limb count to hold N with a spare limb of headroom.
fn pick_limbs(n: &BigUint) -> usize {
    let bits = n.bits() as usize;
    core::cmp::max(4, bits / 64 + 1)
}

/// (SM count, max threads per SM) for sizing a launch to the card.
fn device_geom(ctx: &std::sync::Arc<CudaContext>) -> (u32, u32) {
    use cudarc::driver::sys::CUdevice_attribute as A;
    let sm = ctx.attribute(A::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT).unwrap_or(16).max(1) as u32;
    let tpm = ctx.attribute(A::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR).unwrap_or(1536).max(256) as u32;
    (sm, tpm)
}

/// Devices to spread curves across: every CUDA device present.
fn devices() -> Vec<usize> {
    let mut v = Vec::new();
    let mut d = 0usize;
    while d < 8 { if CudaContext::new(d).is_ok() { v.push(d); } else { break; } d += 1; }
    if v.is_empty() { v.push(0); }
    v
}

/// ECM stage 1 on one device over its own sigma range, stopping when `stop` is
/// set. Returns a factor if this device finds one.
fn ecm_worker(n: &BigUint, b1: u64, b2: u64, device: usize, sigma_offset: u64,
              stop: &core::sync::atomic::AtomicBool) -> Option<BigUint> {
    use core::sync::atomic::Ordering;
    use crate::native_numeral::{add_via_word, halve_even_by_word, modulo_via_word, multiply_via_word, pow2};
    let limbs = pick_limbs(n);
    let n0 = n0_inv(n.to_u64_digits()[0]);
    let r = pow2(64 * limbs);
    let r2 = modulo_via_word(&multiply_via_word(&r, &r), n).unwrap();
    let one_mont = modulo_via_word(&r, n).unwrap();
    let two_mont = modulo_via_word(&multiply_via_word(&r, &BigUint::from(2u32)), n).unwrap();
    let inv2 = halve_even_by_word(&add_via_word(n, &BigUint::from(1u32))).unwrap();
    let inv4 = modulo_via_word(&multiply_via_word(&inv2, &inv2), n).unwrap();
    let inv4_mont = modulo_via_word(&multiply_via_word(&inv4, &r), n).unwrap();
    let k = smooth_k(b1);
    let nbits = k.bits() as i32;
    let kbits: Vec<u8> = (0..nbits).map(|i| if k.bit(i as u64) {1u8} else {0u8}).collect();

    let ctx = CudaContext::new(device).ok()?;
    let stream = ctx.default_stream();
    let ptx = load_ptx_cached(limbs).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("ecm").ok()?;

    let d_n = stream.clone_htod(&limbs_of(n, limbs)).ok()?;
    let d_r2 = stream.clone_htod(&limbs_of(&r2, limbs)).ok()?;
    let d_om = stream.clone_htod(&limbs_of(&one_mont, limbs)).ok()?;
    let d_tm = stream.clone_htod(&limbs_of(&two_mont, limbs)).ok()?;
    let d_i4 = stream.clone_htod(&limbs_of(&inv4_mont, limbs)).ok()?;
    let d_kb = stream.clone_htod(&kbits).ok()?;

    // Fill the card: one curve per thread, full occupancy across every SM.
    let (sm, tpm) = device_geom(&ctx);
    let threads: u32 = sm * tpm;
    let max_launches = 16u64;
    for l in 0..max_launches {
        if stop.load(Ordering::Relaxed) { return None; }
        let init_found: Vec<i32> = alloc::vec![0];
        let mut d_found = stream.clone_htod(&init_found).ok()?;
        let mut d_out = stream.alloc_zeros::<u64>(limbs).ok()?;
        let sigma_base = sigma_offset + 6 + l * (threads as u64);
        let cfg = LaunchConfig { grid_dim: ((threads+127)/128,1,1), block_dim:(128,1,1), shared_mem_bytes:0 };
        let mut b = stream.launch_builder(&func);
        b.arg(&d_n); b.arg(&d_r2); b.arg(&n0);
        b.arg(&d_om); b.arg(&d_tm); b.arg(&d_i4);
        b.arg(&d_kb); b.arg(&nbits); b.arg(&sigma_base); b.arg(&b1); b.arg(&b2);
        b.arg(&mut d_out); b.arg(&mut d_found);
        if unsafe { b.launch(cfg) }.is_err() { return None; }
        let found = stream.clone_dtoh(&d_found).ok()?;
        if found[0] != 0 {
            let out = stream.clone_dtoh(&d_out).ok()?;
            let f = from_limbs(&out);
            if f > BigUint::one() && &f < n && crate::native_numeral::modulo_via_word(n, &f).unwrap().is_zero() {
                stop.store(true, Ordering::Relaxed);
                return Some(f);
            }
        }
    }
    None
}

/// One factor of n via GPU ECM stage 1, spread across every device. Curves are
/// partitioned by a per-device sigma offset, and the first device to find a
/// factor stops the others.
pub fn factor_once(n: &BigUint, b1: u64, _device: usize) -> Option<BigUint> {
    factor_once_b2(n, b1, b1.saturating_mul(20), _device)
}

/// ECM with an explicit stage-2 bound b2, spread across every device.
pub fn factor_once_b2(n: &BigUint, b1: u64, b2: u64, _device: usize) -> Option<BigUint> {
    if n <= &BigUint::from(3u32) { return None; }
    let two = BigUint::from(2u32);
    if crate::native_numeral::modulo_via_word(n, &two).unwrap().is_zero() { return Some(two); }

    let devs = devices();
    let stop = std::sync::Arc::new(core::sync::atomic::AtomicBool::new(false));
    let result = std::sync::Arc::new(std::sync::Mutex::new(None::<BigUint>));
    std::thread::scope(|s| {
        for (i, &dev) in devs.iter().enumerate() {
            let stop = stop.clone();
            let result = result.clone();
            let sigma_offset = (i as u64) * 4_000_000_000;
            s.spawn(move || {
                if let Some(f) = ecm_worker(n, b1, b2, dev, sigma_offset, &stop) {
                    *result.lock().unwrap() = Some(f);
                }
            });
        }
    });
    let g = result.lock().unwrap().clone();
    g
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 { while b != 0 { let t = a % b; a = b; b = t; } a }

/// Pick a giant-step modulus D and its baby residues (j in (0, D/2] coprime to
/// D) so the shared baby table fits in `cap_bytes`. Larger D means fewer giant
/// steps for the same range; the residues coprime to D are the baby list.
fn pick_d(limbs: usize, cap_bytes: usize) -> (u64, Vec<u32>) {
    // candidates, richest primorial first
    for &d in &[30030u64, 2310, 210, 30, 6] {
        let js: Vec<u32> = (1..=d/2).filter(|&j| gcd_u64(j, d) == 1).map(|j| j as u32).collect();
        let bytes = (2 * js.len() + 5) * limbs * 8;
        if bytes <= cap_bytes { return (d, js); }
    }
    let d = 6u64;
    (d, (1..=d/2).filter(|&j| gcd_u64(j, d) == 1).map(|j| j as u32).collect())
}

/// Per-block BSGS stage-2 ECM on one device: one curve per block, a shared baby
/// table over j coprime to D, giant steps i*D*Q split across the block. Deeper
/// stage-2 reach per curve than the per-thread walk, at fewer curves.
fn bsgs_worker(n: &BigUint, b1: u64, b2: u64, device: usize, sigma_offset: u64,
               stop: &core::sync::atomic::AtomicBool) -> Option<BigUint> {
    use core::sync::atomic::Ordering;
    use crate::native_numeral::{add_via_word, halve_even_by_word, modulo_via_word, multiply_via_word, pow2};
    let limbs = pick_limbs(n);
    let n0 = n0_inv(n.to_u64_digits()[0]);
    let r = pow2(64 * limbs);
    let r2 = modulo_via_word(&multiply_via_word(&r, &r), n).unwrap();
    let one_mont = modulo_via_word(&r, n).unwrap();
    let two_mont = modulo_via_word(&multiply_via_word(&r, &BigUint::from(2u32)), n).unwrap();
    let inv2 = halve_even_by_word(&add_via_word(n, &BigUint::from(1u32))).unwrap();
    let inv4 = modulo_via_word(&multiply_via_word(&inv2, &inv2), n).unwrap();
    let inv4_mont = modulo_via_word(&multiply_via_word(&inv4, &r), n).unwrap();
    let k = smooth_k(b1);
    let nbits = k.bits() as i32;
    let kbits: Vec<u8> = (0..nbits).map(|i| if k.bit(i as u64) {1u8} else {0u8}).collect();

    let (d, jlist) = pick_d(limbs, 40_000);
    let nj = jlist.len() as i32;
    let i_min = core::cmp::max(1, b1 / d);
    let i_max = b2 / d + 1;
    let shared = ((2 * jlist.len() + 5) * limbs * 8) as u32;

    let ctx = CudaContext::new(device).ok()?;
    let stream = ctx.default_stream();
    let ptx = load_ptx_cached(limbs).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("ecm_bsgs").ok()?;

    let d_n = stream.clone_htod(&limbs_of(n, limbs)).ok()?;
    let d_r2 = stream.clone_htod(&limbs_of(&r2, limbs)).ok()?;
    let d_om = stream.clone_htod(&limbs_of(&one_mont, limbs)).ok()?;
    let d_tm = stream.clone_htod(&limbs_of(&two_mont, limbs)).ok()?;
    let d_i4 = stream.clone_htod(&limbs_of(&inv4_mont, limbs)).ok()?;
    let d_kb = stream.clone_htod(&kbits).ok()?;
    let d_jl = stream.clone_htod(&jlist).ok()?;

    // One curve per block; oversubscribe the SMs so many curves are resident and
    // the giant/baby loops have enough warps to hide the multiply latency.
    let (sm, _tpm) = device_geom(&ctx);
    let blocks: u32 = sm * 8;
    let block_dim: u32 = 256;
    let max_launches = 16u64;
    for l in 0..max_launches {
        if stop.load(Ordering::Relaxed) { return None; }
        let init_found: Vec<i32> = alloc::vec![0];
        let mut d_found = stream.clone_htod(&init_found).ok()?;
        let mut d_out = stream.alloc_zeros::<u64>(limbs).ok()?;
        let sigma_base = sigma_offset + 6 + l * (blocks as u64);
        let cfg = LaunchConfig { grid_dim: (blocks,1,1), block_dim: (block_dim,1,1), shared_mem_bytes: shared };
        let mut b = stream.launch_builder(&func);
        b.arg(&d_n); b.arg(&d_r2); b.arg(&n0);
        b.arg(&d_om); b.arg(&d_tm); b.arg(&d_i4);
        b.arg(&d_kb); b.arg(&nbits); b.arg(&sigma_base);
        b.arg(&d_jl); b.arg(&nj); b.arg(&d); b.arg(&i_min); b.arg(&i_max);
        b.arg(&mut d_out); b.arg(&mut d_found);
        if unsafe { b.launch(cfg) }.is_err() { return None; }
        let found = stream.clone_dtoh(&d_found).ok()?;
        if found[0] != 0 {
            let out = stream.clone_dtoh(&d_out).ok()?;
            let f = from_limbs(&out);
            if f > BigUint::one() && &f < n && crate::native_numeral::modulo_via_word(n, &f).unwrap().is_zero() {
                stop.store(true, Ordering::Relaxed);
                return Some(f);
            }
        }
    }
    None
}

/// One factor via per-block BSGS stage-2 ECM, spread across every device.
pub fn factor_once_bsgs(n: &BigUint, b1: u64, b2: u64) -> Option<BigUint> {
    if n <= &BigUint::from(3u32) { return None; }
    let two = BigUint::from(2u32);
    if crate::native_numeral::modulo_via_word(n, &two).unwrap().is_zero() { return Some(two); }
    let devs = devices();
    let stop = std::sync::Arc::new(core::sync::atomic::AtomicBool::new(false));
    let result = std::sync::Arc::new(std::sync::Mutex::new(None::<BigUint>));
    std::thread::scope(|s| {
        for (i, &dev) in devs.iter().enumerate() {
            let stop = stop.clone();
            let result = result.clone();
            let sigma_offset = (i as u64) * 4_000_000_000;
            s.spawn(move || {
                if let Some(f) = bsgs_worker(n, b1, b2, dev, sigma_offset, &stop) {
                    *result.lock().unwrap() = Some(f);
                }
            });
        }
    });
    let g = result.lock().unwrap().clone();
    g
}

fn primes_upto(limit: usize) -> Vec<u64> {
    let mut s = alloc::vec![true; limit + 1];
    let mut ps = Vec::new();
    let mut i = 2;
    while i <= limit {
        if s[i] { ps.push(i as u64); let mut j=i*i; while j<=limit { s[j]=false; j+=i; } }
        i += 1;
    }
    ps
}
fn smooth_k(b1: u64) -> BigUint {
    let mut k = BigUint::one();
    for p in primes_upto(b1 as usize) {
        let mut pe = p;
        while pe.saturating_mul(p) <= b1 { pe *= p; }
        k = crate::native_numeral::multiply_via_word(&k, &BigUint::from(pe));
    }
    k
}

struct Xs(u64);
impl Xs { fn n(&mut self)->u64{ let mut x=self.0; x^=x<<13; x^=x>>7; x^=x<<17; self.0=x; x } }

/// Verify the width-`limbs` Montgomery multiply against BigUint.
pub fn verify_width(limbs: usize, count: usize, seed: u64, device: usize) -> String {
    let ctx = match CudaContext::new(device) { Ok(c)=>c, Err(e)=>return format!("gpu_ecm: no ctx: {e}") };
    let stream = ctx.default_stream();
    let ptx = match load_ptx_cached(limbs) { Ok(p)=>p, Err(e)=>return format!("gpu_ecm: nvrtc: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_ecm: module: {e}") };
    let func = match module.load_function("mont_verify") { Ok(f)=>f, Err(e)=>return format!("gpu_ecm: load: {e}") };
    let mut rng = Xs(seed ^ 0xBEEF);
    let n = count;
    let (mut af,mut bf,mut nf,mut r2f)=(Vec::new(),Vec::new(),Vec::new(),Vec::new());
    let mut n0s=Vec::new(); let mut refs=Vec::new();
    let r = BigUint::one() << (64*limbs);
    for _ in 0..n {
        let mut nl: Vec<u64> = (0..limbs).map(|_| rng.n()).collect();
        nl[limbs-1] |= 1<<63; nl[0]|=1;
        let bn=from_limbs(&nl);
        let a=from_limbs(&(0..limbs).map(|_| rng.n()).collect::<Vec<_>>())%&bn;
        let b=from_limbs(&(0..limbs).map(|_| rng.n()).collect::<Vec<_>>())%&bn;
        let r2=(&r*&r)%&bn;
        af.extend(limbs_of(&a,limbs)); bf.extend(limbs_of(&b,limbs)); nf.extend(nl.clone()); r2f.extend(limbs_of(&r2,limbs));
        n0s.push(n0_inv(nl[0])); refs.push((&a*&b)%&bn);
    }
    let d_a=stream.clone_htod(&af).unwrap(); let d_b=stream.clone_htod(&bf).unwrap();
    let d_n=stream.clone_htod(&nf).unwrap(); let d_r2=stream.clone_htod(&r2f).unwrap();
    let d_n0=stream.clone_htod(&n0s).unwrap(); let mut d_out=stream.alloc_zeros::<u64>(n*limbs).unwrap();
    let cc=n as u32;
    let cfg=LaunchConfig{grid_dim:((cc+127)/128,1,1),block_dim:(128,1,1),shared_mem_bytes:0};
    let mut bd=stream.launch_builder(&func);
    bd.arg(&d_a);bd.arg(&d_b);bd.arg(&d_n);bd.arg(&d_r2);bd.arg(&d_n0);bd.arg(&mut d_out);bd.arg(&cc);
    if let Err(e)=unsafe{bd.launch(cfg)}{return format!("gpu_ecm: launch: {e}");}
    let out=stream.clone_dtoh(&d_out).unwrap();
    let mut bad=0; for i in 0..n { if from_limbs(&out[i*limbs..i*limbs+limbs])!=refs[i] { bad+=1; } }
    format!("gpu_ecm verify_width {}: {} products, wrong {}", limbs, n, bad)
}

pub fn run_factor(n_str: &str, b1: u64, device: usize) -> String {
    let n: BigUint = match n_str.trim().parse() { Ok(v)=>v, Err(_)=>return format!("gpu_ecm: '{}' not an integer", n_str) };
    let b2 = b1.saturating_mul(50);
    match factor_once_b2(&n, b1, b2, device) {
        Some(f) => format!("gpu_ecm factor {} (B1={}, B2={}): found factor {}  (divides: {})", n, b1, b2, f, (&n % &f).is_zero()),
        None => format!("gpu_ecm factor {} (B1={}, B2={}): no factor within curve budget", n, b1, b2),
    }
}

/// Per-block BSGS stage-2 ECM verb. B2 defaults to 100*B1, the deep stage-2
/// range the shared baby table is built to sweep.
pub fn run_bsgs(n_str: &str, b1: u64) -> String {
    let n: BigUint = match n_str.trim().parse() { Ok(v)=>v, Err(_)=>return format!("gpu_ecm: '{}' not an integer", n_str) };
    let b2 = b1.saturating_mul(100);
    let limbs = pick_limbs(&n);
    let (d, jlist) = pick_d(limbs, 40_000);
    let ndev = devices().len();
    // stderr so run_quit's stdout filter still passes only the final result,
    // while the operator sees that the GPU path has started.
    eprintln!("gpu_ecm bsgs: bits={} limbs={} devices={} B1={} B2={} D={} baby={} — NVRTC then launch",
              n.bits(), limbs, ndev, b1, b2, d, jlist.len());
    match factor_once_bsgs(&n, b1, b2) {
        Some(f) => format!("gpu_ecm bsgs {} (B1={}, B2={}, D={}, baby={}): found factor {}  (divides: {})",
                           n, b1, b2, d, jlist.len(), f, (&n % &f).is_zero()),
        None => format!("gpu_ecm bsgs {} (B1={}, B2={}, D={}, baby={}): no factor within curve budget",
                        n, b1, b2, d, jlist.len()),
    }
}
