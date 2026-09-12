//! Protocol-wired, nested IMASM graphs executed on CUDA with the full carrier.
use alloc::{format, string::String, vec, vec::Vec};
use cudarc::{driver::{CudaContext, LaunchConfig, PushKernelArg}, nvrtc::compile_ptx};
use imasm_core::{check::{Graph, ClosureState, from_sequence, match_pairs}, classic::Token,
    flow::{gate_out, flow_values}, imasm16_3::Reg16_3};

const MAX_NODES: usize = 64;
const MAX_EDGES: usize = 192;
const SRC: &str = r#"
extern "C" __global__ void protocol_flow(
    const unsigned char* source, const unsigned char* gates,
    const unsigned int* from, const unsigned int* to,
    unsigned int nodes, unsigned int edges,
    unsigned char* node_out, unsigned char* edge_out, unsigned int* status)
{
    unsigned int seed = blockIdx.x * blockDim.x + threadIdx.x;
    if (seed >= 16) return;
    unsigned char ev[192] = {0}, ni[64] = {0};
    unsigned int cap = 4 * (edges ? edges : 1) + 4;
    unsigned int rounds = 0, settled = 0;
    for (unsigned int round=0; round<cap; ++round) {
        unsigned int changed = 0;
        ++rounds;
        for (unsigned int i=0; i<nodes; ++i) {
            unsigned char value = 0;
            for (unsigned int e=0; e<edges; ++e)
                if (to[e] == i) value |= ev[e];
            ni[i] = value;
            unsigned int slot = 0;
            for (unsigned int e=0; e<edges; ++e) {
                if (from[e] != i) continue;
                unsigned char next = source[i] ? (unsigned char)seed
                    : gates[(i*16 + value)*3 + slot];
                if (next != ev[e]) { ev[e]=next; changed=1; }
                ++slot;
            }
        }
        if (!changed) { settled=1; break; }
    }
    for (unsigned int i=0; i<nodes; ++i) node_out[seed*64+i]=ni[i];
    for (unsigned int e=0; e<edges; ++e) edge_out[seed*192+e]=ev[e];
    status[seed*2]=settled;
    status[seed*2+1]=rounds;
}
"#;

fn pack(v: Reg16_3) -> u8 {
    v.big_t as u8 | (v.big_f as u8)<<1 | (v.small_t as u8)<<2 | (v.small_f as u8)<<3
}
fn unpack(v: u8) -> Reg16_3 {
    Reg16_3 { big_t:v&1!=0, big_f:v&2!=0, small_t:v&4!=0, small_f:v&8!=0 }
}

fn graph(word: &str, arity: usize, depth: usize) -> Result<Graph, String> {
    if !(arity == 2 || arity == 3) || depth > 8 {
        return Err("arity must be 2 or 3; enclosing depth must be 0..8".into());
    }
    let mut ops = Vec::new();
    for ch in word.chars().filter(|c| !c.is_whitespace()) {
        let t = Token::parse(&ch.to_string()).ok_or_else(|| format!("unrecognized mark {ch}"))?;
        ops.push(match (arity,t) {
            (3,Token::Fsplit)=>Token::Fsplit3, (3,Token::Ffuse)=>Token::Ffuse3,
            (3,Token::Engagr)=>Token::Evali, _=>t,
        });
    }
    if ops.is_empty() { return Err("provide a committed IMASM word".into()); }
    if depth > 0 {
        if ops.first()!=Some(&Token::Vinit) || ops.last()!=Some(&Token::Tanch) {
            return Err("enclosure requires a word bounded by ⊢ and ⊣".into());
        }
        let (fork,fuse) = if arity==3 {(Token::Fsplit3,Token::Ffuse3)} else {(Token::Fsplit,Token::Ffuse)};
        let mut nested = vec![Token::Vinit];
        nested.extend(core::iter::repeat(fork).take(depth));
        nested.extend_from_slice(&ops[1..ops.len()-1]);
        nested.extend(core::iter::repeat(fuse).take(depth));
        nested.push(Token::Tanch);
        ops = nested;
    }
    if ops.len() > MAX_NODES { return Err("graph exceeds 64 nodes".into()); }
    let g=from_sequence(&ops,&match_pairs(&ops));
    let errors=g.validate();
    if !errors.is_empty() { return Err(format!("grammar F: {errors:?}")); }
    if g.edges.len()>MAX_EDGES { return Err("graph exceeds 192 edges".into()); }
    Ok(g)
}

fn execute(g: &Graph, device: usize) -> Result<(Vec<u8>,Vec<u8>,Vec<u32>),String> {
    let ctx=CudaContext::new(device).map_err(|e|format!("CUDA: {e}"))?;
    let stream=ctx.default_stream();
    let module=ctx.load_module(compile_ptx(SRC).map_err(|e|format!("NVRTC: {e}"))?)
        .map_err(|e|format!("module: {e}"))?;
    let func=module.load_function("protocol_flow").map_err(|e|format!("entry: {e}"))?;
    let source:Vec<u8>=g.nodes.iter().map(|t|(*t==Token::Vinit) as u8).collect();
    // Gate tables are generated from the shared engine. Edge aggregation,
    // partition slots, iteration, and every carrier update execute on device.
    let mut gates=vec![0u8;g.nodes.len()*16*3];
    for (i,&t) in g.nodes.iter().enumerate() {
        for v in 0..16 { for slot in 0..3 {
            gates[(i*16+v)*3+slot]=pack(gate_out(t,unpack(v as u8),Reg16_3::default(),slot,g.out_degree(i)));
        }}
    }
    let from:Vec<u32>=g.edges.iter().map(|&(a,_)|a as u32).collect();
    let to:Vec<u32>=g.edges.iter().map(|&(_,b)|b as u32).collect();
    let ds=stream.clone_htod(&source).map_err(|e|e.to_string())?;
    let dg=stream.clone_htod(&gates).map_err(|e|e.to_string())?;
    let df=stream.clone_htod(&from).map_err(|e|e.to_string())?;
    let dt=stream.clone_htod(&to).map_err(|e|e.to_string())?;
    let mut dn=stream.alloc_zeros::<u8>(16*MAX_NODES).map_err(|e|e.to_string())?;
    let mut de=stream.alloc_zeros::<u8>(16*MAX_EDGES).map_err(|e|e.to_string())?;
    let mut status=stream.alloc_zeros::<u32>(32).map_err(|e|e.to_string())?;
    let nodes=g.nodes.len() as u32; let edges=g.edges.len() as u32;
    let mut launch=stream.launch_builder(&func);
    launch.arg(&ds).arg(&dg).arg(&df).arg(&dt).arg(&nodes).arg(&edges)
        .arg(&mut dn).arg(&mut de).arg(&mut status);
    unsafe {launch.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(32,1,1),shared_mem_bytes:0})}
        .map_err(|e|e.to_string())?;
    Ok((stream.clone_dtoh(&dn).map_err(|e|e.to_string())?,
        stream.clone_dtoh(&de).map_err(|e|e.to_string())?,
        stream.clone_dtoh(&status).map_err(|e|e.to_string())?))
}

pub fn run(word: &str, arity: usize, depth: usize, device: usize) -> String {
    let g=match graph(word,arity,depth) {Ok(g)=>g,Err(e)=>return format!("gpu_kernel flow: {e}")};
    let state=g.closure_state();
    let verdict=match state {
        ClosureState::Closed(_) if g.nodes.contains(&Token::Engagr)=>'B',
        ClosureState::Closed(_)=>'T', ClosureState::Open=>'B', _=>'N',
    };
    let mut out=format!("gpu_kernel flow: arity={arity} enclosure_depth={depth}\n  word: {}\n  check: {verdict} {state:?}\n",
        g.nodes.iter().map(|t|t.code()).collect::<String>());
    let (gpu_nodes,gpu_edges,status)=match execute(&g,device) {
        Ok(r)=>r,Err(e)=>return format!("{out}  device execution failed: {e}")
    };
    let (pairs,_)=g.frobenius_closures();
    for &(f,j) in &pairs {
        out.push_str(&format!("  pair {f}->{j}: {}\n",if g.transforms_between(f,j){"transforms"}else{"identity"}));
    }
    let mut mismatches=0;
    for seed in 0..16u8 {
        let (ni,ev)=flow_values(&g,unpack(seed));
        let ns=&gpu_nodes[seed as usize*MAX_NODES..seed as usize*MAX_NODES+g.nodes.len()];
        let es=&gpu_edges[seed as usize*MAX_EDGES..seed as usize*MAX_EDGES+g.edges.len()];
        let matches=status[seed as usize*2]==1
            && ni.iter().map(|v|pack(*v)).eq(ns.iter().copied())
            && ev.iter().map(|v|pack(*v)).eq(es.iter().copied());
        if !matches {mismatches+=1;}
        let dyads=pairs.iter().map(|&(f,j)|format!("{f}->{j}:{}=>{}:{}",
            unpack(ns[f]).name(),unpack(ns[j]).name(),if ns[f]==ns[j]{"recovered"}else{"changed"})).collect::<Vec<_>>().join(" ");
        out.push_str(&format!("  seed={} settled={} CPU_match={} {dyads}\n",unpack(seed).name(),status[seed as usize*2]==1,matches));
    }
    out.push_str(&format!("  CUDA/core parity: {mismatches} mismatches across all 16 seeds\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_carrier_recovers_at_every_nested_dyad() {
        for arity in [2,3] {for depth in 0..=8 {
            let g=graph("⊢∈≻∋⊣",arity,depth).unwrap();
            assert!(matches!(g.closure_state(),ClosureState::Closed(_)));
            for seed in 0..16 {
                let (ni,_)=flow_values(&g,unpack(seed));
                for (f,j) in g.frobenius_closures().0 {assert!(ni[f]==ni[j]);}
            }
        }}
    }
    #[test]
    fn identity_and_loss_are_distinct() {
        let g=graph("⊢∈⊙∋⊣",3,0).unwrap();
        assert!(matches!(g.closure_state(),ClosureState::Identity));
        let g=graph("⊢∈≺∋⊣",3,0).unwrap();
        assert!(matches!(g.closure_state(),ClosureState::Closed(_)));
        let (ni,_)=flow_values(&g,unpack(5));
        assert!(g.frobenius_closures().0.iter().any(|&(f,j)|ni[f]!=ni[j]));
    }
    #[test]
    fn reject_unknown_marks_and_invalid_graphs() {
        assert!(graph("not a word",3,0).is_err());
        assert!(graph("⊢⊢⊣",3,0).is_err());
        assert!(graph("⊢∈≻∋⊣",4,0).is_err());
    }
}
