//! gpu_rho.rs — fast modular arithmetic for factoring on the device.
//!
//! Stage one: a 256-bit Montgomery multiply (CIOS, four 64-bit limbs), the
//! primitive Pollard rho and ECM ride on. Verified here against BigUint before
//! anything is built on it: `gpu_rho verify` multiplies random pairs modulo
//! random odd 256-bit moduli on the GPU and checks each against the reference.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};
use num_bigint::BigUint;
use num_traits::{Zero, One};

const KERNEL_SRC: &str = r#"
typedef unsigned long long u64;
typedef unsigned __int128 u128;

// CIOS Montgomery multiply: r = a*b*R^{-1} mod N, R = 2^256. All 4-limb LE.
__device__ void montmul(const u64* a, const u64* b, const u64* N, u64 n0, u64* r){
    u64 t[6]; for (int i=0;i<6;i++) t[i]=0;
    for (int i=0;i<4;i++){
        u128 C=0;
        for (int j=0;j<4;j++){
            u128 x = (u128)a[j]*(u128)b[i] + (u128)t[j] + C;
            t[j]=(u64)x; C = x>>64;
        }
        u128 s = (u128)t[4] + C; t[4]=(u64)s; t[5]=(u64)(s>>64);
        u64 m = t[0]*n0;
        u128 c2;
        { u128 x=(u128)m*(u128)N[0] + (u128)t[0]; c2 = x>>64; }
        for (int j=1;j<4;j++){
            u128 x=(u128)m*(u128)N[j] + (u128)t[j] + c2;
            t[j-1]=(u64)x; c2 = x>>64;
        }
        u128 s2=(u128)t[4]+c2; t[3]=(u64)s2; c2=s2>>64;
        t[4]=t[5]+(u64)c2; t[5]=0;
    }
    // t[0..4] is the result (up to 5 limbs). Conditional subtract N.
    // borrow-compare t (5 limbs, t[4] is 0/1) against N (4 limbs).
    bool ge;
    if (t[4] != 0) ge = true;
    else {
        ge = true;
        for (int j=3;j>=0;j--){ if (t[j]!=N[j]){ ge = t[j]>N[j]; break; } }
    }
    if (ge){
        u128 br=0;
        for (int j=0;j<4;j++){
            u128 x = (u128)t[j] - (u128)N[j] - br;
            r[j]=(u64)x; br = (x>>64)&1;
        }
    } else {
        for (int j=0;j<4;j++) r[j]=t[j];
    }
}

__device__ bool geq4(const u64* a, const u64* b){
    for (int j=3;j>=0;j--){ if (a[j]!=b[j]) return a[j]>b[j]; }
    return true;
}
__device__ void sub4(const u64* a, const u64* b, u64* r){
    u128 br=0;
    for (int j=0;j<4;j++){ u128 x=(u128)a[j]-(u128)b[j]-br; r[j]=(u64)x; br=(x>>64)&1; }
}
__device__ void montadd(const u64* a, const u64* b, const u64* N, u64* r){
    u128 c=0; u64 t[4];
    for (int j=0;j<4;j++){ u128 x=(u128)a[j]+(u128)b[j]+c; t[j]=(u64)x; c=x>>64; }
    if (c!=0 || geq4(t,N)) sub4(t,N,r); else { for(int j=0;j<4;j++) r[j]=t[j]; }
}
__device__ void montsub(const u64* a, const u64* b, const u64* N, u64* r){
    // a,b in [0,N). If a>=b: a-b. Else N-(b-a), both in range, no 257-bit overflow.
    if (geq4(a,b)) sub4(a,b,r);
    else { u64 t[4]; sub4(b,a,t); sub4(N,t,r); }
}
__device__ bool is_zero4(const u64* a){ return (a[0]|a[1]|a[2]|a[3])==0; }
__device__ void shr1_4(u64* a){
    for (int j=0;j<3;j++) a[j]=(a[j]>>1)|(a[j+1]<<63);
    a[3]>>=1;
}
__device__ void shl1_4(u64* a){
    for (int j=3;j>0;j--) a[j]=(a[j]<<1)|(a[j-1]>>63);
    a[0]<<=1;
}
// binary gcd of two 256-bit numbers
__device__ void gcd4(const u64* aa, const u64* bb, u64* g){
    u64 a[4],b[4]; for(int j=0;j<4;j++){a[j]=aa[j];b[j]=bb[j];}
    if (is_zero4(a)){ for(int j=0;j<4;j++) g[j]=b[j]; return; }
    if (is_zero4(b)){ for(int j=0;j<4;j++) g[j]=a[j]; return; }
    int shift=0;
    while (((a[0]|b[0])&1)==0){ shr1_4(a); shr1_4(b); shift++; }
    while ((a[0]&1)==0) shr1_4(a);
    while (!is_zero4(b)){
        while ((b[0]&1)==0) shr1_4(b);
        // a and b both odd; keep a <= b, then b -= a (b-a is even, >= 0)
        if (geq4(a,b)){
            u64 t[4]; sub4(a,b,t);              // t = a - b
            for (int j=0;j<4;j++) a[j]=b[j];    // a = old b
            for (int j=0;j<4;j++) b[j]=t[j];    // b = old a - old b
        } else {
            u64 t[4]; sub4(b,a,t);
            for (int j=0;j<4;j++) b[j]=t[j];    // b = b - a
        }
    }
    for (int s=0;s<shift;s++) shl1_4(a);
    for (int j=0;j<4;j++) g[j]=a[j];
}

// Pollard rho over Montgomery arithmetic. Each thread walks its own sequence
// (distinct c and start), accumulates the product of differences, and takes a
// gcd every 128 steps. First thread to find a nontrivial factor writes it.
extern "C" __global__ void rho(
    const u64* N, const u64* R2, u64 n0, const u64* one_mont,
    u64 iters, u64 seed_base,
    u64* out_factor, int* found)
{
    unsigned int gid = blockIdx.x*blockDim.x + threadIdx.x;
    if (*found != 0) return;
    // c = to_mont(seed_base+gid+1); x = to_mont(seed_base+gid+2)
    u64 cs[4]={seed_base+gid+1,0,0,0};
    u64 xs0[4]={seed_base+gid+2,0,0,0};
    u64 c[4], x[4], y[4], q[4];
    montmul(cs, R2, N, n0, c);
    montmul(xs0, R2, N, n0, x);
    for (int j=0;j<4;j++){ y[j]=x[j]; q[j]=one_mont[j]; }
    for (u64 step=1; step<=iters; step++){
        if ((step & 1023)==0 && *found != 0) return;   // someone won; bail
        // x = x*x + c
        u64 t[4]; montmul(x,x,N,n0,t); montadd(t,c,N,x);
        // y = f(f(y))
        montmul(y,y,N,n0,t); montadd(t,c,N,y);
        montmul(y,y,N,n0,t); montadd(t,c,N,y);
        // diff = x - y (mod N)
        u64 d[4]; montsub(x,y,N,d);
        if (is_zero4(d)) return;   // sequence merged, this walker is done
        // q = q * diff
        montmul(q,d,N,n0,t); for(int j=0;j<4;j++) q[j]=t[j];
        if ((step & 127)==0){
            u64 g[4]; gcd4(q,N,g);
            bool g1 = (g[0]==1 && g[1]==0 && g[2]==0 && g[3]==0);
            bool gN = geq4(g,N) && geq4(N,g); // g==N
            if (!g1 && !gN && !is_zero4(g)){
                if (atomicCAS(found,0,1)==0){ for(int j=0;j<4;j++) out_factor[j]=g[j]; }
                return;
            }
            if (gN) return;
        }
    }
}

// Verify the rho primitives against the host: a,b < N.
extern "C" __global__ void prim_verify(
    const u64* A, const u64* B, const u64* NN,
    u64* OADD, u64* OSUB, u64* OGCD, unsigned int cnt)
{
    unsigned int g = blockIdx.x*blockDim.x + threadIdx.x;
    if (g >= cnt) return;
    const u64* a=&A[g*4]; const u64* b=&B[g*4]; const u64* N=&NN[g*4];
    montadd(a,b,N,&OADD[g*4]);
    montsub(a,b,N,&OSUB[g*4]);
    gcd4(a,b,&OGCD[g*4]);
}

// Per test: am=mont(a,R2), bm=mont(b,R2), pm=mont(am,bm), p=mont(pm,1) = a*b mod N.
extern "C" __global__ void mont_verify(
    const u64* A, const u64* B, const u64* NN, const u64* R2, const u64* n0s,
    u64* OUT, unsigned int cnt)
{
    unsigned int g = blockIdx.x*blockDim.x + threadIdx.x;
    if (g >= cnt) return;
    const u64* a=&A[g*4]; const u64* b=&B[g*4]; const u64* N=&NN[g*4]; const u64* r2=&R2[g*4];
    u64 n0=n0s[g];
    u64 am[4], bm[4], pm[4], one[4], p[4];
    montmul(a, r2, N, n0, am);
    montmul(b, r2, N, n0, bm);
    montmul(am, bm, N, n0, pm);
    one[0]=1; one[1]=0; one[2]=0; one[3]=0;
    montmul(pm, one, N, n0, p);
    for (int j=0;j<4;j++) OUT[g*4+j]=p[j];
}
"#;

pub fn limbs4(n: &BigUint) -> [u64; 4] {
    let d = n.to_u64_digits();
    let mut r = [0u64; 4];
    for i in 0..4 { if i < d.len() { r[i] = d[i]; } }
    r
}

pub fn from_limbs4(l: &[u64]) -> BigUint {
    use crate::native_numeral::{add_via_word, multiply_via_word, pow2};
    let mut n = BigUint::zero();
    let base = pow2(64);
    for i in (0..4).rev() { n = add_via_word(&multiply_via_word(&n, &base), &BigUint::from(l[i])); }
    n
}

/// -N^{-1} mod 2^64 (word inverse for CIOS), by Newton on the low limb.
pub fn n0_inv(n_low: u64) -> u64 {
    let mut inv: u64 = 1;
    for _ in 0..6 { inv = inv.wrapping_mul(2u64.wrapping_sub(n_low.wrapping_mul(inv))); }
    inv.wrapping_neg()
}

struct Xs(u64);
impl Xs { fn n(&mut self)->u64{ let mut x=self.0; x^=x<<13; x^=x>>7; x^=x<<17; self.0=x; x } }

/// One nontrivial factor of odd composite n (n < 2^256) via GPU parallel rho,
/// or None within the launch budget. Reach is roughly a 2^60 smallest factor;
/// beyond that needs ECM or distinguished-point rho.
pub fn factor_once(n: &BigUint, device: usize) -> Option<BigUint> {
    if n.bits() > 256 { return None; }
    let nl = limbs4(n);
    if nl[0] & 1 == 0 { return Some(BigUint::from(2u32)); }
    use crate::native_numeral::{modulo_via_word, multiply_via_word, pow2};
    let two256 = pow2(256);
    let r2 = modulo_via_word(&multiply_via_word(&two256, &two256), n).unwrap();
    let one_mont = modulo_via_word(&two256, n).unwrap();   // R mod N
    let n0 = n0_inv(nl[0]);

    let ctx = CudaContext::new(device).ok()?;
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = compile_ptx_with_opts(KERNEL_SRC, opts).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("rho").ok()?;

    let d_n = stream.clone_htod(&nl.to_vec()).ok()?;
    let d_r2 = stream.clone_htod(&limbs4(&r2).to_vec()).ok()?;
    let d_om = stream.clone_htod(&limbs4(&one_mont).to_vec()).ok()?;

    let threads: u32 = 4096;
    let iters: u64 = 1 << 20;
    let max_launches = 12u64;
    for l in 0..max_launches {
        let init_found: Vec<i32> = alloc::vec![0];
        let mut d_found = stream.clone_htod(&init_found).ok()?;
        let mut d_out = stream.alloc_zeros::<u64>(4).ok()?;
        let seed_base: u64 = 1 + l * (threads as u64) * 4;
        let cfg = LaunchConfig { grid_dim: ((threads+127)/128,1,1), block_dim:(128,1,1), shared_mem_bytes:0 };
        let mut b = stream.launch_builder(&func);
        b.arg(&d_n); b.arg(&d_r2); b.arg(&n0); b.arg(&d_om);
        b.arg(&iters); b.arg(&seed_base); b.arg(&mut d_out); b.arg(&mut d_found);
        if unsafe { b.launch(cfg) }.is_err() { return None; }
        let found = stream.clone_dtoh(&d_found).ok()?;
        if found[0] != 0 {
            let out = stream.clone_dtoh(&d_out).ok()?;
            let f = from_limbs4(&out);
            if f > BigUint::one() && &f < n && crate::native_numeral::modulo_via_word(n, &f).unwrap().is_zero() { return Some(f); }
        }
    }
    None
}

fn bgcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() { let t = crate::native_numeral::modulo_via_word(&a, &b).unwrap(); a = b; b = t; }
    a
}

/// Verify montadd, montsub, gcd on the device against BigUint.
pub fn verify_prims(count: usize, seed: u64, device: usize) -> String {
    let ctx = match CudaContext::new(device) { Ok(c)=>c, Err(e)=>return format!("gpu_rho: no ctx: {e}") };
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = match compile_ptx_with_opts(KERNEL_SRC, opts) { Ok(p)=>p, Err(e)=>return format!("gpu_rho: nvrtc: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_rho: module: {e}") };
    let func = match module.load_function("prim_verify") { Ok(f)=>f, Err(e)=>return format!("gpu_rho: load: {e}") };
    let mut rng = Xs(seed ^ 0x0F0F_1234);
    let n = count;
    let (mut af, mut bf, mut nf) = (Vec::new(), Vec::new(), Vec::new());
    let mut ar=Vec::new(); let mut br=Vec::new(); let mut nr=Vec::new();
    for _ in 0..n {
        let mut nl=[rng.n(),rng.n(),rng.n(),rng.n()|(1<<63)]; nl[0]|=1;
        let bn=from_limbs4(&nl);
        let a=crate::native_numeral::modulo_via_word(&from_limbs4(&[rng.n(),rng.n(),rng.n(),rng.n()]), &bn).unwrap();
        let b=crate::native_numeral::modulo_via_word(&from_limbs4(&[rng.n(),rng.n(),rng.n(),rng.n()]), &bn).unwrap();
        for v in limbs4(&a){af.push(v);} for v in limbs4(&b){bf.push(v);} for v in nl{nf.push(v);}
        ar.push(a); br.push(b); nr.push(bn);
    }
    let d_a=stream.clone_htod(&af).unwrap(); let d_b=stream.clone_htod(&bf).unwrap(); let d_n=stream.clone_htod(&nf).unwrap();
    let mut d_add=stream.alloc_zeros::<u64>(n*4).unwrap();
    let mut d_sub=stream.alloc_zeros::<u64>(n*4).unwrap();
    let mut d_gcd=stream.alloc_zeros::<u64>(n*4).unwrap();
    let cc=n as u32;
    let cfg=LaunchConfig{grid_dim:((cc+127)/128,1,1),block_dim:(128,1,1),shared_mem_bytes:0};
    let mut bd=stream.launch_builder(&func);
    bd.arg(&d_a);bd.arg(&d_b);bd.arg(&d_n);bd.arg(&mut d_add);bd.arg(&mut d_sub);bd.arg(&mut d_gcd);bd.arg(&cc);
    if let Err(e)=unsafe{bd.launch(cfg)}{return format!("gpu_rho: launch: {e}");}
    let oadd=stream.clone_dtoh(&d_add).unwrap();
    let osub=stream.clone_dtoh(&d_sub).unwrap();
    let ogcd=stream.clone_dtoh(&d_gcd).unwrap();
    let (mut ba,mut bs,mut bg)=(0usize,0usize,0usize);
    for i in 0..n {
        let add=from_limbs4(&oadd[i*4..i*4+4]);
        let sub=from_limbs4(&osub[i*4..i*4+4]);
        let gcd=from_limbs4(&ogcd[i*4..i*4+4]);
        use crate::native_numeral::{add_via_word, modulo_via_word, subtract_via_word};
        if add != modulo_via_word(&add_via_word(&ar[i], &br[i]), &nr[i]).unwrap() { ba+=1; }
        let want_sub = modulo_via_word(&subtract_via_word(&add_via_word(&ar[i], &nr[i]), &br[i]).unwrap(), &nr[i]).unwrap();
        if sub != want_sub { bs+=1; }
        if gcd != bgcd(ar[i].clone(), br[i].clone()) { bg+=1; }
    }
    format!("gpu_rho prims: {} cases -- montadd wrong {}, montsub wrong {}, gcd wrong {}", n, ba, bs, bg)
}

/// `gpu_rho factor <n>`: one factor via GPU rho, with a sympy-checkable answer.
pub fn run_factor(n_str: &str, device: usize) -> String {
    let n: BigUint = match n_str.trim().parse() { Ok(v)=>v, Err(_)=>return format!("gpu_rho factor: '{}' not an integer", n_str) };
    match factor_once(&n, device) {
        Some(f) => format!("gpu_rho factor {} : found factor {}  (divides: {})", n, f, crate::native_numeral::modulo_via_word(&n, &f).unwrap().is_zero()),
        None => format!("gpu_rho factor {} : no factor within budget (needs ECM / distinguished-point rho)", n),
    }
}

pub fn verify(count: usize, seed: u64, device: usize) -> String {
    let ctx = match CudaContext::new(device) { Ok(c)=>c, Err(e)=>return format!("gpu_rho: no CUDA context: {e}") };
    let stream = ctx.default_stream();
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = match compile_ptx_with_opts(KERNEL_SRC, opts) { Ok(p)=>p, Err(e)=>return format!("gpu_rho: NVRTC: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_rho: module: {e}") };
    let func = match module.load_function("mont_verify") { Ok(f)=>f, Err(e)=>return format!("gpu_rho: load: {e}") };

    let mut rng = Xs(seed ^ 0xDEAD_BEEF_1234_5678);
    let n = count;
    let (mut a_f, mut b_f, mut n_f, mut r2_f) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut n0s = Vec::with_capacity(n);
    let mut refs: Vec<BigUint> = Vec::with_capacity(n);
    use crate::native_numeral::{modulo_via_word, multiply_via_word, pow2};
    let two256 = pow2(256);
    for _ in 0..n {
        // random odd 256-bit N (top limb nonzero-ish), a,b < N
        let mut nl = [rng.n(), rng.n(), rng.n(), rng.n() | (1u64<<63)];
        nl[0] |= 1; // odd
        let bn = from_limbs4(&nl);
        let a = modulo_via_word(&from_limbs4(&[rng.n(), rng.n(), rng.n(), rng.n()]), &bn).unwrap();
        let b = modulo_via_word(&from_limbs4(&[rng.n(), rng.n(), rng.n(), rng.n()]), &bn).unwrap();
        let r2 = modulo_via_word(&multiply_via_word(&two256, &two256), &bn).unwrap();   // R^2 mod N
        for v in limbs4(&a) { a_f.push(v); }
        for v in limbs4(&b) { b_f.push(v); }
        for v in nl { n_f.push(v); }
        for v in limbs4(&r2) { r2_f.push(v); }
        n0s.push(n0_inv(nl[0]));
        refs.push(modulo_via_word(&multiply_via_word(&a, &b), &bn).unwrap());
    }

    let d_a = stream.clone_htod(&a_f).unwrap();
    let d_b = stream.clone_htod(&b_f).unwrap();
    let d_n = stream.clone_htod(&n_f).unwrap();
    let d_r2 = stream.clone_htod(&r2_f).unwrap();
    let d_n0 = stream.clone_htod(&n0s).unwrap();
    let mut d_out = stream.alloc_zeros::<u64>(n*4).unwrap();

    let block: u32 = 128;
    let grid = ((n as u32)+block-1)/block;
    let cfg = LaunchConfig { grid_dim:(grid,1,1), block_dim:(block,1,1), shared_mem_bytes:0 };
    let cc = n as u32;
    let mut bd = stream.launch_builder(&func);
    bd.arg(&d_a); bd.arg(&d_b); bd.arg(&d_n); bd.arg(&d_r2); bd.arg(&d_n0); bd.arg(&mut d_out); bd.arg(&cc);
    if let Err(e) = unsafe { bd.launch(cfg) } { return format!("gpu_rho: launch: {e}"); }
    let out = stream.clone_dtoh(&d_out).unwrap();

    let mut bad = 0usize; let mut first = String::new();
    for g in 0..n {
        let got = from_limbs4(&out[g*4..g*4+4]);
        if got != refs[g] {
            bad += 1;
            if first.is_empty() { first = format!("  first: got {} expected {}", got, refs[g]); }
        }
    }
    if bad == 0 { format!("gpu_rho verify: {} montgomery products, GPU == BigUint on every one", n) }
    else { format!("gpu_rho verify: {}/{} WRONG\n{}", bad, n, first) }
}
