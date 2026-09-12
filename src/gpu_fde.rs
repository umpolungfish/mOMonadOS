//! gpu_fde.rs — the FDE(n) tower theorems checked on the GPU.
//!
//! The tower ascends by embedding (identity on the tag) and descends by
//! restriction (a mid index still in range keeps its place, one past the range
//! collapses to top). Two theorems ride on that: restrict after embed is the
//! identity at any depth, and restriction factors through any intermediate tier.
//! This checks both over the whole finite space up to a bound, one tuple per
//! thread on the device, against the CPU functions in fde.rs.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use crate::fde::{FdeVal, fde_restrict, fde_embed};

const KERNEL_SRC: &str = r#"
// value: tag 0=Bot, 1=Top, 2=Mid(idx). embed is identity.
__device__ void rstr(int tag,int idx,int k,int* ot,int* oi){
    if (tag==0){ *ot=0; *oi=0; return; }
    if (tag==1){ *ot=1; *oi=0; return; }
    if (idx < k){ *ot=2; *oi=idx; } else { *ot=1; *oi=0; }
}
extern "C" __global__ void fde_check(
    const unsigned int* Kk, const unsigned int* Nn, const unsigned int* Jj,
    const unsigned int* TAG, const unsigned int* IDX,
    unsigned char* out, unsigned int cnt)
{
    unsigned int i = blockIdx.x*blockDim.x + threadIdx.x;
    if (i >= cnt) return;
    int k=Kk[i], j=Jj[i], tag=TAG[i], idx=IDX[i];
    // roundtrip: restrict(n,k, embed(x)) == x, embed = identity so restrict(_,k,x)==x
    int rt,ri; rstr(tag,idx,k,&rt,&ri);
    unsigned char round_ok = (rt==tag && ri==idx) ? 1 : 0;
    // trans: restrict(n,k,x) == restrict(j,k, restrict(n,j,x))
    int dt,di; rstr(tag,idx,k,&dt,&di);
    int mt,mi; rstr(tag,idx,j,&mt,&mi);
    int vt,vi; rstr(mt,mi,k,&vt,&vi);
    unsigned char trans_ok = (dt==vt && di==vi) ? 1 : 0;
    out[i] = (round_ok ? 1 : 0) | (trans_ok ? 2 : 0);
}
"#;

fn tag_idx(x: FdeVal) -> (u32, u32) {
    match x { FdeVal::Bot => (0,0), FdeVal::Top => (1,0), FdeVal::Mid(i) => (2, i as u32) }
}

/// Check both tower theorems over every (n<=N, k<=n, j in [k,n], x well-formed
/// at n) on the device, against the CPU fde functions.
pub fn verify(max_n: usize, device: usize) -> String {
    // Enumerate the finite space.
    let mut ks: Vec<u32> = Vec::new();
    let mut ns: Vec<u32> = Vec::new();
    let mut js: Vec<u32> = Vec::new();
    let mut tags: Vec<u32> = Vec::new();
    let mut idxs: Vec<u32> = Vec::new();
    let mut vals: Vec<FdeVal> = Vec::new();
    for n in 1..=max_n {
        // values well-formed at n: Bot, Top, Mid(0..n)
        let mut space: Vec<FdeVal> = alloc::vec![FdeVal::Bot, FdeVal::Top];
        for i in 0..n { space.push(FdeVal::Mid(i)); }
        for k in 0..=n {
            for j in k..=n {
                for &x in &space {
                    let (t, id) = tag_idx(x);
                    ns.push(n as u32); ks.push(k as u32); js.push(j as u32);
                    tags.push(t); idxs.push(id); vals.push(x);
                }
            }
        }
    }
    let cnt = vals.len();
    if cnt == 0 { return "gpu_fde verify: empty space".into(); }

    let ctx = match CudaContext::new(device) { Ok(c)=>c, Err(e)=>return format!("gpu_fde: no CUDA context: {e}") };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(KERNEL_SRC) { Ok(p)=>p, Err(e)=>return format!("gpu_fde: NVRTC: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_fde: module: {e}") };
    let func = match module.load_function("fde_check") { Ok(f)=>f, Err(e)=>return format!("gpu_fde: load: {e}") };

    let d_k = match stream.clone_htod(&ks) { Ok(d)=>d, Err(e)=>return format!("gpu_fde: htod: {e}") };
    let d_n = match stream.clone_htod(&ns) { Ok(d)=>d, Err(e)=>return format!("gpu_fde: htod: {e}") };
    let d_j = match stream.clone_htod(&js) { Ok(d)=>d, Err(e)=>return format!("gpu_fde: htod: {e}") };
    let d_t = match stream.clone_htod(&tags) { Ok(d)=>d, Err(e)=>return format!("gpu_fde: htod: {e}") };
    let d_i = match stream.clone_htod(&idxs) { Ok(d)=>d, Err(e)=>return format!("gpu_fde: htod: {e}") };
    let mut d_out = match stream.alloc_zeros::<u8>(cnt) { Ok(d)=>d, Err(e)=>return format!("gpu_fde: alloc: {e}") };

    let cfg = LaunchConfig::for_num_elems(cnt as u32);
    let cc = cnt as u32;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_k); b.arg(&d_n); b.arg(&d_j); b.arg(&d_t); b.arg(&d_i); b.arg(&mut d_out); b.arg(&cc);
    if let Err(e) = unsafe { b.launch(cfg) } { return format!("gpu_fde: launch: {e}"); }
    let g_out = match stream.clone_dtoh(&d_out) { Ok(v)=>v, Err(e)=>return format!("gpu_fde: dtoh: {e}") };

    // CPU reference and theorem assertions.
    let mut parity_bad = 0usize;
    let mut trans_fail = 0usize;
    let mut round_fail_wf = 0usize;
    for a in 0..cnt {
        let (n, k, j, x) = (ns[a] as usize, ks[a] as usize, js[a] as usize, vals[a]);
        // CPU: roundtrip restrict(n,k,embed(k,n,x))==x ; trans direct==via j
        let cpu_round = fde_restrict(n, k, fde_embed(k, n, x)) == x;
        let cpu_trans = fde_restrict(n, k, x) == fde_restrict(j, k, fde_restrict(n, j, x));
        let g = g_out[a];
        let gpu_round = (g & 1) != 0;
        let gpu_trans = (g & 2) != 0;
        if gpu_round != cpu_round || gpu_trans != cpu_trans { parity_bad += 1; }
        if !cpu_trans { trans_fail += 1; }
        // roundtrip theorem holds for x well-formed at k (mid idx < k)
        let wf_at_k = match x { FdeVal::Mid(i) => i < k, _ => true };
        if wf_at_k && !cpu_round { round_fail_wf += 1; }
    }

    if parity_bad == 0 && trans_fail == 0 && round_fail_wf == 0 {
        format!("gpu_fde verify: {} tuples over n<=1..{}, GPU == CPU; restriction transitivity holds on all, restrict-after-embed holds on every well-formed value", cnt, max_n)
    } else {
        format!("gpu_fde verify: {} tuples: parity mismatches {}, transitivity failures {}, roundtrip failures on well-formed {}", cnt, parity_bad, trans_fail, round_fail_wf)
    }
}
