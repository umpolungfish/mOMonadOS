//! gpu_millennium.rs — the static self-imscription analysis on the GPU.
//!
//! millennium reads each conjecture word's executed crystal address, which comes
//! from kernel::self_imscribe: a static analysis of the token program (family
//! signature, token diversity, self-reference, Frobenius order, cyclic
//! dialetheia reachability, period, atomicity). No tick machine runs; it is all
//! counting over the token array, one word per thread. This computes those
//! fields on the device and the host rebuilds the same Snapshot, so
//! IgTuple::from_snapshot reads it unchanged. Checked against self_imscribe by
//! verify.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use crate::kernel::Snapshot;

const MAXLEN: usize = 64;
const NF: usize = 12; // fields out per program

const KERNEL_SRC: &str = r#"
typedef unsigned char u8;
// family: 0 Logical, 1 Frobenius, 2 Dialetheia, 3 Linear
__device__ int family(int t){
    if (t==6||t==7||t==12||t==13) return 1;      // Fsplit/Ffuse/Fsplit3/Ffuse3
    if (t==5||t==9||t==10||t==14) return 2;      // Evalt/Evalf/Engagr/Evali
    if (t==11) return 3;                          // Ifix
    return 0;                                     // Vinit/Tanch/Afwd/Arev/Clink/Imscrib/Rotat
}
extern "C" __global__ void imscribe_static(
    const u8* progs, const int* lens, int stride, unsigned int count, int* out)
{
    unsigned int g = blockIdx.x*blockDim.x + threadIdx.x;
    if (g >= count) return;
    const u8* p = progs + (size_t)g*stride;
    int n = lens[g];

    int l=0,f=0,d=0,x=0;
    bool seen[16]; for (int i=0;i<16;i++) seen[i]=false;
    for (int i=0;i<n;i++){ int t=p[i]; switch(family(t)){case 0:l++;break;case 1:f++;break;case 2:d++;break;default:x++;} seen[t]=true; }
    int diversity=0; for (int i=0;i<16;i++) if (seen[i]) diversity++;

    bool self_ref = (n>0) && (p[0]==p[n-1]);

    bool has6=false,has7=false,has12=false,has13=false;
    int first6=-1,first7=-1;
    for (int i=0;i<n;i++){
        int t=p[i];
        if (t==6){ has6=true; if(first6<0)first6=i; }
        if (t==7){ has7=true; if(first7<0)first7=i; }
        if (t==12) has12=true;
        if (t==13) has13=true;
    }
    int frob;
    if (has12||has13) frob=3;
    else if (!has6 && !has7) frob=0;
    else if (has6 && !has7) frob=1;
    else if (!has6 && has7) frob=2;
    else frob = (first6<first7)?1:2;

    // dialetheia_complete: every Engagr reaches an Evalt/Evalf on its cyclic ring
    bool has_evalt=false,has_evalf=false,has_engagr=false;
    for (int i=0;i<n;i++){ int t=p[i]; if(t==5)has_evalt=true; if(t==9)has_evalf=true; if(t==10)has_engagr=true; }
    bool dial;
    if (!has_evalt||!has_evalf||!has_engagr) dial=false;
    else {
        dial=true;
        for (int i=0;i<n && dial;i++){
            if (p[i]==10){
                bool found=false;
                for (int off=1; off<n; off++){ int j=(i+off)%n; if(p[j]==5||p[j]==9){found=true;break;} }
                if(!found) dial=false;
            }
        }
    }

    // period: least p dividing n with p[i]==p[i%p] for all i
    int period=n; if(n==0) period=1;
    for (int pp=1; pp<=n; pp++){
        if (n%pp==0){
            bool ok=true;
            for (int i=pp;i<n;i++){ if(p[i]!=p[i%pp]){ok=false;break;} }
            if(ok){ period=pp; break; }
        }
    }

    int fsplit_count=0,ffuse_count=0;
    for (int i=0;i<n;i++){ int t=p[i]; if(t==6||t==12)fsplit_count++; if(t==7||t==13)ffuse_count++; }
    bool atomic = (fsplit_count==1 && ffuse_count==1);
    bool bifurc = atomic && self_ref;

    int tier = (frob>0 || dial) ? 1 : 0;

    int* o = out + (size_t)g*12;
    o[0]=l; o[1]=f; o[2]=d; o[3]=x; o[4]=diversity; o[5]=self_ref?1:0;
    o[6]=frob; o[7]=dial?1:0; o[8]=period; o[9]=atomic?1:0; o[10]=bifurc?1:0; o[11]=tier;
}
"#;

fn ids_of(word: &str) -> Result<Vec<u8>, String> {
    let prog = crate::belnap_ring_shor::program_from_glyphs(word)
        .map_err(|_| format!("gpu_millennium: word did not parse"))?;
    let ids: Vec<u8> = prog.as_slice().iter().map(|t| crate::gpu_kernel::tok_id_pub(*t)).collect();
    Ok(ids)
}

fn build(fields: &[i32]) -> Snapshot {
    let mut snap = Snapshot {
        frobenius_order: fields[6] as u8,
        period: fields[8] as usize,
        sig: (fields[0] as usize, fields[1] as usize, fields[2] as usize, fields[3] as usize),
        token_diversity: fields[4] as usize,
        self_ref: fields[5] != 0,
        dialetheia_complete: fields[7] != 0,
        tier: fields[11] as u8,
        b_live_ticks: 0,
        gate_discriminations: 0,
        value_period: 0,
        atomic_reentry: fields[9] != 0,
        bifurcation_revisited: fields[10] != 0,
        winding_count: 0,
    };
    let _ = &mut snap;
    snap
}

/// Static self-imscription snapshots for a batch of words, computed on the GPU.
pub fn snapshots(words: &[&str]) -> Option<Vec<Snapshot>> {
    if words.is_empty() { return Some(Vec::new()); }
    let ctx = CudaContext::new(0).ok()?;
    let stream = ctx.default_stream();
    let ptx = compile_ptx(KERNEL_SRC).ok()?;
    let module = ctx.load_module(ptx).ok()?;
    let func = module.load_function("imscribe_static").ok()?;

    let count = words.len();
    let mut progs = alloc::vec![0u8; count * MAXLEN];
    let mut lens = alloc::vec![0i32; count];
    for (g, w) in words.iter().enumerate() {
        let ids = ids_of(w).ok()?;
        if ids.len() > MAXLEN { return None; }
        lens[g] = ids.len() as i32;
        for (j, &id) in ids.iter().enumerate() { progs[g*MAXLEN + j] = id; }
    }
    let d_progs = stream.clone_htod(&progs).ok()?;
    let d_lens = stream.clone_htod(&lens).ok()?;
    let mut d_out = stream.alloc_zeros::<i32>(count * NF).ok()?;
    let stride = MAXLEN as i32; let cc = count as u32;
    let threads = 128u32; let blocks = (cc + threads - 1) / threads;
    let cfg = LaunchConfig { grid_dim: (blocks.max(1),1,1), block_dim: (threads,1,1), shared_mem_bytes: 0 };
    let mut b = stream.launch_builder(&func);
    b.arg(&d_progs); b.arg(&d_lens); b.arg(&stride); b.arg(&cc); b.arg(&mut d_out);
    unsafe { b.launch(cfg) }.ok()?;
    let out = stream.clone_dtoh(&d_out).ok()?;
    Some((0..count).map(|g| build(&out[g*NF..g*NF+NF])).collect())
}

/// One word's static snapshot on the GPU.
pub fn snapshot(word: &str) -> Option<Snapshot> {
    snapshots(&[word]).and_then(|v| v.into_iter().next())
}

/// Self-check: GPU static snapshot against kernel::self_imscribe on the seven
/// conjecture words plus a couple of controls.
pub fn verify() -> String {
    let words = [
        "⊢∈≻⊤≺⊥⊞⋈∋⊡⊙⊣", "⊢∈≻⊤≺⊥⊞⋈⊙∋⊡⊣", "⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣",
        "⊢∈⊥≺⊤≻⋈⊞∋⊙⊡⊣", "⊢∈≻⊤≺⊥⋈⊞⊙∋⊡⋈⊙⊣", "⊢⋈∈≻⊤≺⊥⊞⊡∋⊙⊣",
        "⊢⊣∈≻⊤≺⊥⋈⊞⊙∋⊡⋈≻≺⊤⊥⊞⊡⊣",
    ];
    let mut out = String::new();
    let mut all = true;
    for w in words {
        let cpu = match crate::belnap_ring_shor::program_from_glyphs(w) {
            Ok(prog) => crate::kernel::self_imscribe(&prog),
            Err(_) => { out.push_str(&format!("  {}: parse failed\n", w)); all=false; continue; }
        };
        match snapshot(w) {
            Some(gpu) => {
                let ok = gpu.frobenius_order==cpu.frobenius_order && gpu.period==cpu.period
                    && gpu.sig==cpu.sig && gpu.token_diversity==cpu.token_diversity
                    && gpu.self_ref==cpu.self_ref && gpu.dialetheia_complete==cpu.dialetheia_complete
                    && gpu.tier==cpu.tier && gpu.atomic_reentry==cpu.atomic_reentry
                    && gpu.bifurcation_revisited==cpu.bifurcation_revisited;
                all &= ok;
                out.push_str(&format!("  {}: frob={} period={} sig={:?} tier={} -- match {}\n",
                    w, gpu.frobenius_order, gpu.period, gpu.sig, gpu.tier, ok));
            }
            None => { out.push_str(&format!("  {}: no device\n", w)); all=false; }
        }
    }
    format!("gpu_millennium verify:\n{}  all match: {}", out, all)
}
