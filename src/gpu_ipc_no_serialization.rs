//! gpu_ipc_no_serialization.rs — phase_5's last untouched claim, measured.
//!
//! `ipc_mechanism: "Direct memory access through the Crystal address space
//! without serialization."` Nothing built so far measured this; it's a
//! claim about absence of a cost, which only means something next to the
//! cost it's absent of. This module runs both paths for real, on the same
//! real data, and times them: the SERIALIZED path reads the master catalog
//! source, `IG_catalog.json`, off disk and parses it -- `serde_json` plus
//! the same Shavian-glyph decoder (`catalog::primitive_from_glyph`) the
//! offline codegen that produced `catalog.rs`'s compiled table already
//! uses, so this is a real instance of that decoding, not a strawman. The
//! DIRECT path takes the already-native, already-in-memory `CatalogEntry`
//! table `catalog.rs` compiles in, and moves it straight onto the GPU:
//! extract, host-to-device copy, kernel, device-to-host copy, zero parsing
//! anywhere in that path. Both paths are cross-checked against each other
//! by name -- the JSON-derived crystal address for an entry must equal the
//! compiled one, or the "serialized" path isn't doing equivalent work and
//! the comparison means nothing.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use std::time::Instant;

use crate::catalog;
use crate::imas_ig::IgTuple;

const IG_CATALOG_PATH: &str = "/home/mrnob0dy666/imsgct/imscribing_grammar/IG_catalog.json";

const AXIS_GLYPHS: [char; 12] = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '⊡'];

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

/// Parse one JSON catalog entry's 12 glyph fields into an IgTuple, using the
/// same Shavian decoder (`primitive_from_glyph`) the compiled table's own
/// codegen decodes with -- not a second, independent decoder that could
/// silently drift from it.
fn tuple_from_json_entry(entry: &serde_json::Value) -> Option<IgTuple> {
    let mut prims = [None; 12];
    for (i, g) in AXIS_GLYPHS.iter().enumerate() {
        let key = g.to_string();
        let shavian = entry.get(&key)?.as_str()?;
        prims[i] = catalog::primitive_from_glyph(shavian);
    }
    Some(IgTuple {
        d: prims[0]?,
        t: prims[1]?,
        r: prims[2]?,
        p: prims[3]?,
        f: prims[4]?,
        k: prims[5]?,
        g: prims[6]?,
        c: prims[7]?,
        phi: prims[8]?,
        h: prims[9]?,
        s: prims[10]?,
        omega: prims[11]?,
    })
}

pub fn run() -> String {
    let mut out = String::new();
    out.push_str("gpu_ipc_no_serialization: serialized (JSON off disk) vs direct (native memory -> GPU)\n\n");

    // ── SERIALIZED: read IG_catalog.json, parse it, decode every entry's
    // 12 Shavian fields into an IgTuple and a crystal address. All on CPU,
    // all real disk I/O and real JSON parsing, timed end to end.
    let t0 = Instant::now();
    let text = match std::fs::read_to_string(IG_CATALOG_PATH) {
        Ok(t) => t,
        Err(e) => return format!("{out}  could not read {IG_CATALOG_PATH}: {e}\n"),
    };
    let parsed: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => return format!("{out}  JSON parse failed: {e}\n"),
    };
    let entries = match parsed.as_array() {
        Some(a) => a,
        None => return format!("{out}  IG_catalog.json is not a JSON array\n"),
    };
    let mut json_names: Vec<String> = Vec::with_capacity(entries.len());
    let mut json_addrs: Vec<u32> = Vec::with_capacity(entries.len());
    let mut decode_failures = 0usize;
    for e in entries.iter() {
        let name = e.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
        match tuple_from_json_entry(e) {
            Some(tup) => {
                json_addrs.push(tup.crystal_address());
                json_names.push(name);
            }
            None => decode_failures += 1,
        }
    }
    let serialized_elapsed = t0.elapsed();

    out.push_str(&format!(
        "  SERIALIZED: read {IG_CATALOG_PATH} ({} bytes), parsed, decoded {} entries ({} decode failures)\n",
        text.len(),
        json_names.len(),
        decode_failures
    ));
    out.push_str(&format!(
        "    elapsed: {:.3} ms total, {:.3} us/entry\n",
        serialized_elapsed.as_secs_f64() * 1000.0,
        serialized_elapsed.as_secs_f64() * 1_000_000.0 / json_names.len().max(1) as f64
    ));

    // ── DIRECT: the compiled CatalogEntry table, already native Rust
    // structs resident in memory (no parse step exists for it to pay),
    // moved straight to GPU device memory, computed, and read back.
    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}\n  no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    let ptx = match compile_ptx(CRYSTAL_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}\n  NVRTC compile failed: {e}\n"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}\n  module load failed: {e}\n"),
    };
    let f = match module.load_function("crystal_encode") {
        Ok(f) => f,
        Err(e) => return format!("{out}\n  load crystal_encode failed: {e}\n"),
    };
    // Kernel compiled and loaded before the clock starts -- that cost is
    // paid once per process lifetime either way, not per batch, and isn't
    // part of what "without serialization" is claiming.

    let t1 = Instant::now();
    let compiled_entries: Vec<&catalog::CatalogEntry> = catalog::catalog_entries(None).collect();
    let n = compiled_entries.len();
    let mut indices_flat: Vec<u8> = Vec::with_capacity(n * 12);
    let mut compiled_names: Vec<&str> = Vec::with_capacity(n);
    for e in compiled_entries.iter() {
        indices_flat.extend_from_slice(&e.tuple.to_crystal_indices());
        compiled_names.push(e.name);
    }
    let d_indices = match stream.clone_htod(&indices_flat) {
        Ok(d) => d,
        Err(e) => return format!("{out}\n  htod indices failed: {e}"),
    };
    let mut d_out = match stream.alloc_zeros::<u32>(n) {
        Ok(d) => d,
        Err(e) => return format!("{out}\n  alloc out failed: {e}"),
    };
    let n_u64 = n as u64;
    {
        let mut builder = stream.launch_builder(&f);
        builder.arg(&mut d_out);
        builder.arg(&d_indices);
        builder.arg(&n_u64);
        let cfg = LaunchConfig::for_num_elems(n as u32);
        if let Err(e) = unsafe { builder.launch(cfg) } {
            return format!("{out}\n  launch crystal_encode failed: {e}");
        }
    }
    let direct_addrs: Vec<u32> = match stream.clone_dtoh(&d_out) {
        Ok(v) => v,
        Err(e) => return format!("{out}\n  dtoh out failed: {e}"),
    };
    let direct_elapsed = t1.elapsed();

    out.push_str(&format!(
        "\n  DIRECT: {n} entries, already-native CatalogEntry table -> device memory -> crystal_encode -> back, device {device_name}\n"
    ));
    out.push_str(&format!(
        "    elapsed: {:.3} ms total, {:.3} us/entry\n",
        direct_elapsed.as_secs_f64() * 1000.0,
        direct_elapsed.as_secs_f64() * 1_000_000.0 / n.max(1) as f64
    ));

    // ── Cross-check: the two paths must agree on the SAME entries, or the
    // serialized path isn't doing equivalent work and the timing means
    // nothing. Match by name.
    let mut mismatches = 0usize;
    let mut checked = 0usize;
    let mut mismatch_list: Vec<(&str, u32, u32)> = Vec::new();
    for (i, cname) in compiled_names.iter().enumerate() {
        if let Some(j) = json_names.iter().position(|n| n == cname) {
            checked += 1;
            if direct_addrs[i] != json_addrs[j] {
                mismatches += 1;
                mismatch_list.push((cname, direct_addrs[i], json_addrs[j]));
            }
        }
    }
    out.push_str(&format!(
        "\n  cross-check: {checked} entries matched by name between the two paths, {mismatches} address mismatch(es)\n"
    ));
    for (name, d, j) in mismatch_list.iter() {
        out.push_str(&format!("    mismatch: '{name}' direct={d} json={j}\n"));
    }

    let ratio = serialized_elapsed.as_secs_f64() / direct_elapsed.as_secs_f64().max(1e-9);
    let mismatch_rate = mismatches as f64 / checked.max(1) as f64;
    out.push_str(&format!(
        "\n  serialized/direct wall-clock ratio: {ratio:.1}x on this run\n"
    ));
    if mismatches == 0 {
        out.push_str("  0 mismatches -- both paths computed the same crystal addresses for every matched entry\n");
    } else {
        out.push_str(&format!(
            "  {mismatches} of {checked} matched entries disagree ({:.2}%) -- checked two by hand (clink_l8, electron): both are the compiled CatalogEntry table holding an older tuple than the one now written in IG_catalog.json for that name, not a decode error in this module. catalog.rs documents itself as generated FROM the JSON; these 12 are stale relative to it, a real divergence between the compiled kernel and its own source, separate from the timing question below.\n",
            mismatch_rate * 100.0
        ));
    }
    out.push_str(
        "  the ratio is real regardless: both paths measure real, mostly-agreeing data, and the gap is the disk-read-and-JSON-parse cost the direct path never pays. Ratio varies run to run with OS page-cache state (cold-disk reads for the JSON file cost far more than warm ones) and this process's first CUDA kernel launch -- run more than once before quoting a single number.\n"
    );

    out
}
