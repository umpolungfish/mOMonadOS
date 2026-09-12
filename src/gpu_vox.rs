//! gpu_vox.rs — the vox control-flow closure verdict on the GPU.
//!
//! One word per thread. Each glyph reduces to a class the verdict needs: a fork
//! (in), a fuse (out), a work opcode, or other. The thread rotates to the
//! least-balance cut, pairs forks with fuses cyclically from there, and reads
//! the verdict: N when nothing forked or a paired region did no work, T when
//! every fork pairs and some region did work, B when a fork is left open, F
//! when fuses outnumber forks. This is vox::verdict, run on the device and
//! checked against the CPU auditor word for word.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

const WLEN: usize = 256;   // per-word cap

/// Glyph -> class: 1 fork, 2 fuse, 3 work, 0 other.
fn code_of(c: char) -> u8 {
    use crate::vox::{FSPLIT, FFUSE, AFWD, AREV, CLINK, EVALT, EVALF, ENGAGR, IFIX};
    if c == FSPLIT { 1 }
    else if c == FFUSE { 2 }
    else if c == AFWD || c == AREV || c == CLINK || c == EVALT || c == EVALF || c == ENGAGR || c == IFIX { 3 }
    else { 0 }
}

const KERNEL_SRC: &str = r#"
// codes: 1 fork, 2 fuse, 3 work, 0 other. One word per thread. Output verdict
// as 0=N, 1=T, 2=F, 3=B.
extern "C" __global__ void vox_verdict(
    const unsigned char* codes, const unsigned int* lens,
    unsigned char* out, unsigned int n)
{
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n) return;
    const int WLEN = 256;
    unsigned char w[256];
    unsigned int L = lens[gid];
    if (L > 256) L = 256;
    for (unsigned int i=0;i<L;i++) w[i] = codes[gid*WLEN + i];

    unsigned int nSplit=0, nFuse=0;
    for (unsigned int i=0;i<L;i++){ if (w[i]==1) nSplit++; else if (w[i]==2) nFuse++; }
    if (nSplit==0 && nFuse==0) { out[gid]=0; return; }   // N

    // least-balance rotation
    long long bal=0, least=0; unsigned int start=0;
    for (unsigned int i=0;i<L;i++){
        bal += (w[i]==1)?1:((w[i]==2)?-1:0);
        if (bal < least) { least=bal; start=(i+1)%L; }
    }

    // cyclic pairing from start
    unsigned int stack[256]; int top=0;
    unsigned int unFuse=0, pairs=0; unsigned char anyWork=0;
    for (unsigned int off=0; off<L; off++){
        unsigned int i=(start+off)%L;
        if (w[i]==1) { stack[top++]=i; }
        else if (w[i]==2) {
            if (top>0) {
                unsigned int si=stack[--top];
                pairs++;
                unsigned int len=(i + L - si - 1)%L;   // interior length
                for (unsigned int j=0;j<len;j++){ if (w[(si+1+j)%L]==3) { anyWork=1; break; } }
            } else { unFuse++; }
        }
    }
    unsigned int unSplit=(unsigned int)top;

    unsigned char v;
    if (pairs==0 && unSplit==0 && unFuse==0) v=0;      // N
    else if (unFuse > unSplit) v=2;                     // F
    else if (unSplit > 0) v=3;                          // B
    else if (anyWork) v=1;                              // T
    else v=0;                                           // N
    out[gid]=v;
}
"#;

fn vchar(v: u8) -> char { match v { 1=>'T', 2=>'F', 3=>'B', _=>'N' } }

struct Xs(u64);
impl Xs { fn n(&mut self)->u64{ let mut x=self.0; x^=x<<13; x^=x>>7; x^=x<<17; self.0=x; x } }

/// Run `count` random glyph words on the GPU and the CPU auditor, report the
/// first verdict that differs.
pub fn verify(count: usize, seed: u64, device: usize) -> String {
    use crate::vox::{VINIT, TANCH, AFWD, AREV, IMSCRIB, IFIX, CLINK, EVALT, EVALF, ENGAGR, FSPLIT, FFUSE};
    let glyphs: [char; 12] = [VINIT, TANCH, AFWD, AREV, CLINK, EVALT, FSPLIT, FFUSE, IMSCRIB, EVALF, ENGAGR, IFIX];

    let ctx = match CudaContext::new(device) { Ok(c)=>c, Err(e)=>return format!("gpu_vox: no CUDA context: {e}") };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(KERNEL_SRC) { Ok(p)=>p, Err(e)=>return format!("gpu_vox: NVRTC: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_vox: module: {e}") };
    let func = match module.load_function("vox_verdict") { Ok(f)=>f, Err(e)=>return format!("gpu_vox: load: {e}") };

    let mut rng = Xs(seed ^ 0xA5A5_5A5A_1234_9876);
    let n = count;
    let mut codes = alloc::vec![0u8; n * WLEN];
    let mut lens = alloc::vec![0u32; n];
    let mut words: Vec<Vec<char>> = Vec::with_capacity(n);
    for g in 0..n {
        let len = 2 + (rng.n() % 60) as usize;   // 2..62
        let mut w = Vec::with_capacity(len);
        for j in 0..len {
            let c = glyphs[(rng.n() % 12) as usize];
            codes[g*WLEN + j] = code_of(c);
            w.push(c);
        }
        lens[g] = len as u32;
        words.push(w);
    }

    let d_codes = match stream.clone_htod(&codes) { Ok(d)=>d, Err(e)=>return format!("gpu_vox: htod codes: {e}") };
    let d_lens = match stream.clone_htod(&lens) { Ok(d)=>d, Err(e)=>return format!("gpu_vox: htod lens: {e}") };
    let mut d_out = match stream.alloc_zeros::<u8>(n) { Ok(d)=>d, Err(e)=>return format!("gpu_vox: alloc: {e}") };

    let block: u32 = 128;
    let grid = ((n as u32) + block - 1) / block;
    let cfg = LaunchConfig { grid_dim:(grid,1,1), block_dim:(block,1,1), shared_mem_bytes:0 };
    let nn = n as u32;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_codes); b.arg(&d_lens); b.arg(&mut d_out); b.arg(&nn);
    if let Err(e) = unsafe { b.launch(cfg) } { return format!("gpu_vox: launch: {e}"); }
    let g_out = match stream.clone_dtoh(&d_out) { Ok(v)=>v, Err(e)=>return format!("gpu_vox: dtoh: {e}") };

    let mut mism = 0usize; let mut first = String::new();
    for g in 0..n {
        let cpu = crate::vox::verdict(&words[g]);
        let gpu = vchar(g_out[g]);
        if cpu != gpu {
            mism += 1;
            if first.is_empty() {
                let s: String = words[g].iter().collect();
                first = format!("  first mismatch: word '{}'  CPU={} GPU={}", s, cpu, gpu);
            }
        }
    }
    if mism == 0 {
        format!("gpu_vox verify: {} words, GPU verdict == CPU vox::verdict on every one", n)
    } else {
        format!("gpu_vox verify: {}/{} words DIVERGED\n{}", mism, n, first)
    }
}

/// Verdict one glyph word on the device, with the CPU verdict beside it.
pub fn run(word: &str, device: usize) -> String {
    let w: Vec<char> = word.chars().filter(|c| !c.is_whitespace()).collect();
    if w.is_empty() { return "gpu_vox verdict <glyph-word>".into(); }
    if w.len() > WLEN { return format!("gpu_vox: word longer than {} marks", WLEN); }

    let ctx = match CudaContext::new(device) { Ok(c)=>c, Err(e)=>return format!("gpu_vox: no CUDA context: {e}") };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(KERNEL_SRC) { Ok(p)=>p, Err(e)=>return format!("gpu_vox: NVRTC: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_vox: module: {e}") };
    let func = match module.load_function("vox_verdict") { Ok(f)=>f, Err(e)=>return format!("gpu_vox: load: {e}") };

    let mut codes = alloc::vec![0u8; WLEN];
    for (i,&c) in w.iter().enumerate() { codes[i] = code_of(c); }
    let lens = alloc::vec![w.len() as u32];
    let d_codes = match stream.clone_htod(&codes) { Ok(d)=>d, Err(e)=>return format!("gpu_vox: htod: {e}") };
    let d_lens = match stream.clone_htod(&lens) { Ok(d)=>d, Err(e)=>return format!("gpu_vox: htod: {e}") };
    let mut d_out = match stream.alloc_zeros::<u8>(1) { Ok(d)=>d, Err(e)=>return format!("gpu_vox: alloc: {e}") };
    let cfg = LaunchConfig { grid_dim:(1,1,1), block_dim:(1,1,1), shared_mem_bytes:0 };
    let nn = 1u32;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_codes); b.arg(&d_lens); b.arg(&mut d_out); b.arg(&nn);
    if let Err(e) = unsafe { b.launch(cfg) } { return format!("gpu_vox: launch: {e}"); }
    let g_out = match stream.clone_dtoh(&d_out) { Ok(v)=>v, Err(e)=>return format!("gpu_vox: dtoh: {e}") };

    let gpu = vchar(g_out[0]);
    let cpu = crate::vox::verdict(&w);
    format!("gpu_vox verdict on device: {}   (CPU vox::verdict: {}, matches: {})", gpu, cpu, gpu == cpu)
}
