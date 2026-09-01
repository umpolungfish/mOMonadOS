//! gpu_catalog_crystal.rs — batch the real catalog's crystal addresses on GPU.
//!
//! Not `imasm_cycle_gpu` (that ob3ect's scope is the full primitives -> word
//! -> primitives round trip, which needs real per-word parsing and
//! ancestry-pairing, genuinely variable-length work). This is the one piece
//! of what `imasm cycle` touches that IS fixed-width and batches cleanly:
//! `crystal::encode`, a 12-term dot product (index[i] * STRIDE[i], STRIDE
//! values `[4320000, 864000, 216000, 43200, 14400, 2880, 960, 240, 48, 12,
//! 4, 1]`) with no branching and no dependence on any other entry. Every
//! live catalog entry (8600+ as of this run, not synthetic data) gets its
//! address computed this way; this batches that computation for the whole
//! catalog in one GPU kernel launch, checked against
//! `IgTuple::crystal_address()` (the real, existing CPU function) entry by
//! entry.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

const CRYSTAL_SRC: &str = r#"
extern "C" __global__ void crystal_encode(
    unsigned int *out, const unsigned char *indices, const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    const unsigned int strides[12] = {4320000u, 864000u, 216000u, 43200u, 14400u,
                                       2880u, 960u, 240u, 48u, 12u, 4u, 1u};
    unsigned int addr = 0;
    #pragma unroll
    for (int k = 0; k < 12; k++) {
        addr += (unsigned int)indices[i * 12 + k] * strides[k];
    }
    out[i] = addr;
}
"#;

/// Batches `crystal::encode` for every entry in the live catalog on GPU,
/// checked against `IgTuple::crystal_address()` for each one. Real catalog
/// data, not generated -- the point is the whole thing, not a sample.
pub fn run() -> String {
    let entries: Vec<&crate::catalog::CatalogEntry> = crate::catalog::catalog_entries(None).collect();
    let n = entries.len();
    let mut out = format!("gpu_catalog_crystal run: {n} live catalog entries\n\n");

    let mut indices_flat: Vec<u8> = Vec::with_capacity(n * 12);
    let mut cpu_addrs: Vec<u32> = Vec::with_capacity(n);
    for e in entries.iter() {
        let idx = e.tuple.to_crystal_indices();
        indices_flat.extend_from_slice(&idx);
        cpu_addrs.push(e.tuple.crystal_address());
    }

    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}  no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(CRYSTAL_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  NVRTC compile failed: {e}\n"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  module load failed: {e}\n"),
    };
    let f = match module.load_function("crystal_encode") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load crystal_encode failed: {e}\n"),
    };
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n"));

    let d_indices = match stream.clone_htod(&indices_flat) {
        Ok(d) => d,
        Err(e) => return format!("{out}  htod indices failed: {e}"),
    };
    let mut d_out = match stream.alloc_zeros::<u32>(n) {
        Ok(d) => d,
        Err(e) => return format!("{out}  alloc out failed: {e}"),
    };
    let n_u64 = n as u64;
    let mut builder = stream.launch_builder(&f);
    builder.arg(&mut d_out);
    builder.arg(&d_indices);
    builder.arg(&n_u64);
    let cfg = LaunchConfig::for_num_elems(n as u32);
    if let Err(e) = unsafe { builder.launch(cfg) } {
        return format!("{out}  launch crystal_encode failed: {e}");
    }
    let gpu_addrs: Vec<u32> = match stream.clone_dtoh(&d_out) {
        Ok(v) => v,
        Err(e) => return format!("{out}  dtoh out failed: {e}"),
    };

    let mut mismatches = 0usize;
    let mut first_mismatch: Option<(usize, &str, u32, u32)> = None;
    for i in 0..n {
        if gpu_addrs[i] != cpu_addrs[i] {
            mismatches += 1;
            if first_mismatch.is_none() {
                first_mismatch = Some((i, entries[i].name, cpu_addrs[i], gpu_addrs[i]));
            }
        }
    }

    out.push_str(&format!(
        "  ⋈ batched crystal_encode for all {n} entries in one kernel launch\n"
    ));
    out.push_str(&format!(
        "  {n} checked against IgTuple::crystal_address() (CPU), {mismatches} mismatch(es)\n"
    ));
    if let Some((i, name, cpu, gpu)) = first_mismatch {
        out.push_str(&format!("    first mismatch: entry {i} '{name}' cpu={cpu} gpu={gpu}\n"));
    }
    out.push_str(&format!(
        "  -- {}\n",
        if mismatches == 0 {
            "every live catalog entry's crystal address matches, batched"
        } else {
            "MISMATCH FOUND"
        }
    ));
    out
}
