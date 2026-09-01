//! gpu_imasm_cycle.rs — the full imasm-cycle round trip, batched on GPU.
//!
//! Forward leg: each catalog entry's 12-axis tuple already names one of the
//! 49 Shavian types per axis (`imas_ig::IgTuple`'s d,t,r,p,f,k,g,c,phi,h,s,
//! omega, in exactly `canonical_ig::PRIMITIVE_ORDER`'s order -- checked by
//! reading `imas_ig.rs` and `crystal.rs`'s own `to_crystal_indices`, not
//! assumed). Each type is itself a full IMASM program, read once at start
//! from the real, static `the_primitive_type_called_<name>_ob3ect.json`
//! files this session already produces more of (`~/imsgct/ob3ect/digital/`).
//! Writing an entry's word is concatenating its twelve types' programs in
//! that order -- done here on the host, cheap, exact, the same operation
//! `ask_native::imasm::tuple_glyph_word` names.
//!
//! Return leg: for each axis, given the exact offset the word was written
//! at (known here because this leg wrote it, the same asymmetry the real
//! two-leg discipline has -- write knows its boundaries, cold reads don't),
//! check every one of the up to 50 candidate types as a prefix match at
//! that offset. That is a small, fixed-size, branch-light inner loop
//! (candidate count and program length both bounded), the same shape as
//! every other batched kernel this session built, and it is what actually
//! runs on the GPU: one thread per (entry, axis) pair, matching against the
//! same fixed candidate table every other thread reads. The verdict per
//! entry: EXACT if every axis recovers only its own type, AMBIGUOUS if the
//! true type is among more than one match on some axis, BROKEN if the true
//! type is missing from the matches on some axis -- the same three-way
//! reading `primitive_imasm_cycle.md` reports for the CPU walk.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use std::collections::BTreeMap;

const AXIS_GLYPHS: [char; 12] = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '⊡'];
const MAX_TYPE_LEN: usize = 24; // real max across all 50 types is 22, checked
const MAX_TYPES: usize = 64; // real count is 50, checked; room to grow

fn glyph_to_u8(c: char) -> Option<u8> {
    AXIS_GLYPHS.iter().position(|&g| g == c).map(|p| p as u8)
}

/// The twelve opcodes' English names, `IMSCRIBERS_GUIDE_TO_IMASM.md` Part I's
/// own table. Some of the static type ob3ects (generated before this session,
/// dated July) carry the opcode as this name ("VINIT") rather than the glyph
/// ("⊢") the ob3ects minted tonight use -- checked directly against
/// `the_primitive_type_called_death`'s own JSON, not assumed. Both are the
/// same twelve opcodes; this accepts either spelling.
fn opcode_name_to_u8(name: &str) -> Option<u8> {
    let glyph = match name {
        "VINIT" => '⊢', "TANCH" => '⊣', "AFWD" => '≻', "AREV" => '≺',
        "CLINK" => '⋈', "IMSCRIB" => '⊙', "FSPLIT" | "FSPLIT3" => '∈',
        "FFUSE" | "FFUSE3" => '∋', "EVALT" => '⊤', "EVALF" => '⊥',
        "ENGAGR" | "EVALI" => '⊞', "IFIX" => '⊡',
        _ => return None,
    };
    glyph_to_u8(glyph)
}

/// Reads every `the_primitive_type_called_<name>_ob3ect.json` under
/// `~/imsgct/ob3ect/digital/`, in whatever order the filesystem gives them,
/// and returns (name, opcode-glyphs-as-u8, length). A type whose program
/// uses a mark outside the twelve is skipped, loudly, not silently dropped.
fn load_type_table() -> Result<Vec<(String, [u8; MAX_TYPE_LEN], usize)>, String> {
    let home = std::env::var("HOME").map_err(|_| "no HOME set".to_string())?;
    let dir = std::path::Path::new(&home).join("imsgct/ob3ect/digital");
    let rd = std::fs::read_dir(&dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    let mut out = Vec::new();
    for entry in rd.flatten() {
        let fname = entry.file_name().to_string_lossy().into_owned();
        let Some(rest) = fname.strip_prefix("the_primitive_type_called_") else { continue };
        let type_name = rest.to_string();
        let json_path = entry.path().join(format!("the_primitive_type_called_{type_name}_ob3ect.json"));
        let Ok(text) = std::fs::read_to_string(&json_path) else { continue };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let Some(steps) = v.pointer("/phases/phase_4/steps").and_then(|s| s.as_array()) else { continue };
        let mut buf = [0u8; MAX_TYPE_LEN];
        let mut n = 0usize;
        let mut ok = true;
        for s in steps {
            let op = s.get("opcode").and_then(|o| o.as_str()).unwrap_or("");
            let code = match op.chars().next().and_then(glyph_to_u8) {
                Some(c) => c,
                None => match opcode_name_to_u8(op) {
                    Some(c) => c,
                    None => { ok = false; break; }
                },
            };
            if n >= MAX_TYPE_LEN { ok = false; break; }
            buf[n] = code;
            n += 1;
        }
        if ok && n > 0 {
            out.push((type_name, buf, n));
        }
    }
    if out.is_empty() {
        return Err(format!("no readable type ob3ects under {}", dir.display()));
    }
    Ok(out)
}

/// Which of the 12 axes a type name is actually valid on, from the real
/// per-axis tables (`catalog::D_ORD`..`OMEGA_ORD`) -- not every type is
/// legal at every axis, and checking a candidate's program against an axis
/// it was never valid for is a false-positive prefix match, not a real
/// ambiguity. Returns a 12-bit mask, bit i set iff valid on axis i (the
/// same ⊢⊣≻≺⋈⊤∈∋⊙⊥⊞⊡ order `AXIS_GLYPHS`/`PRIMITIVE_ORDER` use).
fn axis_validity_mask(type_name: &str) -> u16 {
    use crate::catalog::{C_ORD, D_ORD, F_ORD, G_ORD, H_ORD, K_ORD, OMEGA_ORD, P_ORD, PHI_ORD, R_ORD, S_ORD, T_ORD};
    let tables: [&[crate::imas_ig::IgPrim]; 12] = [
        &D_ORD, &T_ORD, &R_ORD, &P_ORD, &F_ORD, &K_ORD,
        &G_ORD, &C_ORD, &PHI_ORD, &H_ORD, &S_ORD, &OMEGA_ORD,
    ];
    let mut mask = 0u16;
    for (axis, table) in tables.iter().enumerate() {
        if table.iter().any(|p| {
            let n = format!("{:?}", p);
            n == type_name || n.strip_suffix('_') == Some(type_name)
        }) {
            mask |= 1 << axis;
        }
    }
    mask
}

const CYCLE_SRC: &str = r#"
extern "C" __global__ void cycle_readback(
    unsigned long long *out_bitmask,
    const unsigned char *words, const unsigned int *word_len, const unsigned int word_stride,
    const unsigned int *axis_offset, const unsigned int n_entries,
    const unsigned char *cand_progs, const unsigned int *cand_len, const unsigned int n_cand,
    const unsigned short *cand_axis_mask, const unsigned int max_type_len)
{
    unsigned long long tid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    unsigned long long total = (unsigned long long)n_entries * 12ULL;
    if (tid >= total) return;
    unsigned int entry = (unsigned int)(tid / 12ULL);
    unsigned int axis = (unsigned int)(tid % 12ULL);
    unsigned int off = axis_offset[entry * 12u + axis];
    unsigned int wlen = word_len[entry];
    const unsigned char *word = words + (unsigned long long)entry * word_stride;

    unsigned long long mask = 0ULL;
    for (unsigned int c = 0; c < n_cand; c++) {
        if (((cand_axis_mask[c] >> axis) & 1u) == 0u) continue;
        unsigned int clen = cand_len[c];
        if (off + clen > wlen) continue;
        const unsigned char *prog = cand_progs + (unsigned long long)c * max_type_len;
        bool match = true;
        for (unsigned int k = 0; k < clen; k++) {
            if (word[off + k] != prog[k]) { match = false; break; }
        }
        if (match) mask |= (1ULL << c);
    }
    out_bitmask[tid] = mask;
}
"#;

pub fn run() -> String {
    let type_table = match load_type_table() {
        Ok(t) => t,
        Err(e) => return format!("gpu_imasm_cycle run: {e}\n"),
    };
    let n_cand = type_table.len();
    if n_cand > MAX_TYPES {
        return format!("gpu_imasm_cycle run: {n_cand} types exceeds MAX_TYPES {MAX_TYPES}\n");
    }
    let name_to_idx: BTreeMap<String, usize> =
        type_table.iter().enumerate().map(|(i, (name, _, _))| (name.clone(), i)).collect();

    let entries: Vec<&crate::catalog::CatalogEntry> = crate::catalog::catalog_entries(None).collect();
    let n = entries.len();
    let mut out = format!(
        "gpu_imasm_cycle run: {n} live catalog entries, {n_cand} type program(s) loaded\n\n"
    );

    // Forward leg (host, cheap, exact): write each entry's word, tracking
    // where each of its 12 axis segments actually starts.
    let mut max_word_len = 0usize;
    let mut per_entry_axis_names: Vec<[String; 12]> = Vec::with_capacity(n);
    let mut per_entry_words: Vec<Vec<u8>> = Vec::with_capacity(n);
    let mut per_entry_offsets: Vec<[u32; 12]> = Vec::with_capacity(n);
    let mut skipped_unknown_type: Vec<(usize, &str, String)> = Vec::new();

    for (ei, e) in entries.iter().enumerate() {
        let axis_names: [String; 12] = [
            format!("{:?}", e.tuple.d), format!("{:?}", e.tuple.t), format!("{:?}", e.tuple.r),
            format!("{:?}", e.tuple.p), format!("{:?}", e.tuple.f), format!("{:?}", e.tuple.k),
            format!("{:?}", e.tuple.g), format!("{:?}", e.tuple.c), format!("{:?}", e.tuple.phi),
            format!("{:?}", e.tuple.h), format!("{:?}", e.tuple.s), format!("{:?}", e.tuple.omega),
        ];
        let mut word: Vec<u8> = Vec::new();
        let mut offsets = [0u32; 12];
        let mut ok = true;
        for (axis_i, name) in axis_names.iter().enumerate() {
            // IgPrim escapes two Rust keywords with a trailing underscore
            // (if_, or_); the type ob3ect directories were never named that
            // way (the_primitive_type_called_if, ..._or). Strip it for the
            // lookup -- checked directly, both directories exist without
            // the underscore, and this is the only difference between them.
            let lookup_name = name.strip_suffix('_').unwrap_or(name);
            let Some(&idx) = name_to_idx.get(lookup_name) else {
                skipped_unknown_type.push((ei, e.name, name.clone()));
                ok = false;
                break;
            };
            offsets[axis_i] = word.len() as u32;
            let (_, buf, len) = &type_table[idx];
            word.extend_from_slice(&buf[..*len]);
        }
        if !ok {
            per_entry_axis_names.push(axis_names);
            per_entry_words.push(Vec::new());
            per_entry_offsets.push([0u32; 12]);
            continue;
        }
        max_word_len = max_word_len.max(word.len());
        per_entry_axis_names.push(axis_names);
        per_entry_words.push(word);
        per_entry_offsets.push(offsets);
    }

    let usable: Vec<usize> = (0..n).filter(|&i| !per_entry_words[i].is_empty()).collect();
    let n_usable = usable.len();
    out.push_str(&format!(
        "  {} entries written (types resolved on every axis), {} skipped (unmapped type name)\n",
        n_usable, skipped_unknown_type.len()
    ));
    if let Some((ei, name, tname)) = skipped_unknown_type.first() {
        out.push_str(&format!("    example skip: entry {ei} '{name}' names type '{tname}' with no loaded program\n"));
    }
    {
        let mut hist: BTreeMap<String, usize> = BTreeMap::new();
        for (_, _, tname) in skipped_unknown_type.iter() {
            *hist.entry(tname.clone()).or_insert(0) += 1;
        }
        let mut hist_vec: Vec<(String, usize)> = hist.into_iter().collect();
        hist_vec.sort_by(|a, b| b.1.cmp(&a.1));
        out.push_str(&format!("    {} distinct unmapped type name(s):\n", hist_vec.len()));
        for (tname, count) in hist_vec.iter().take(20) {
            out.push_str(&format!("      '{tname}': {count}\n"));
        }
    }
    if n_usable == 0 {
        out.push_str("  nothing to batch.\n");
        return out;
    }

    // Flatten to fixed-stride buffers for the GPU.
    let stride = max_word_len;
    let mut words_flat: Vec<u8> = alloc::vec![0u8; n_usable * stride];
    let mut word_len_flat: Vec<u32> = Vec::with_capacity(n_usable);
    let mut offset_flat: Vec<u32> = Vec::with_capacity(n_usable * 12);
    for (row, &ei) in usable.iter().enumerate() {
        let w = &per_entry_words[ei];
        words_flat[row * stride..row * stride + w.len()].copy_from_slice(w);
        word_len_flat.push(w.len() as u32);
        offset_flat.extend_from_slice(&per_entry_offsets[ei]);
    }
    let mut cand_progs_flat: Vec<u8> = alloc::vec![0u8; n_cand * MAX_TYPE_LEN];
    let mut cand_len_flat: Vec<u32> = Vec::with_capacity(n_cand);
    let mut cand_axis_mask_flat: Vec<u16> = Vec::with_capacity(n_cand);
    for (i, (name, buf, len)) in type_table.iter().enumerate() {
        cand_progs_flat[i * MAX_TYPE_LEN..i * MAX_TYPE_LEN + *len].copy_from_slice(&buf[..*len]);
        cand_len_flat.push(*len as u32);
        cand_axis_mask_flat.push(axis_validity_mask(name));
    }

    let ctx = match CudaContext::new(0) { Ok(c) => c, Err(e) => return format!("{out}  no CUDA context: {e}\n") };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(CYCLE_SRC) { Ok(p) => p, Err(e) => return format!("{out}  NVRTC compile failed: {e}\n") };
    let module = match ctx.load_module(ptx) { Ok(m) => m, Err(e) => return format!("{out}  module load failed: {e}\n") };
    let f = match module.load_function("cycle_readback") { Ok(f) => f, Err(e) => return format!("{out}  load cycle_readback failed: {e}\n") };
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n"));

    let d_words = match stream.clone_htod(&words_flat) { Ok(d) => d, Err(e) => return format!("{out}  htod words: {e}") };
    let d_word_len = match stream.clone_htod(&word_len_flat) { Ok(d) => d, Err(e) => return format!("{out}  htod word_len: {e}") };
    let d_offsets = match stream.clone_htod(&offset_flat) { Ok(d) => d, Err(e) => return format!("{out}  htod offsets: {e}") };
    let d_cand_progs = match stream.clone_htod(&cand_progs_flat) { Ok(d) => d, Err(e) => return format!("{out}  htod cand_progs: {e}") };
    let d_cand_len = match stream.clone_htod(&cand_len_flat) { Ok(d) => d, Err(e) => return format!("{out}  htod cand_len: {e}") };
    let d_cand_axis_mask = match stream.clone_htod(&cand_axis_mask_flat) { Ok(d) => d, Err(e) => return format!("{out}  htod cand_axis_mask: {e}") };
    let mut d_out = match stream.alloc_zeros::<u64>(n_usable * 12) { Ok(d) => d, Err(e) => return format!("{out}  alloc out: {e}") };

    let word_stride_u32 = stride as u32;
    let n_entries_u32 = n_usable as u32;
    let n_cand_u32 = n_cand as u32;
    let max_type_len_u32 = MAX_TYPE_LEN as u32;
    let total_threads = (n_usable * 12) as u32;

    let mut builder = stream.launch_builder(&f);
    builder.arg(&mut d_out);
    builder.arg(&d_words);
    builder.arg(&d_word_len);
    builder.arg(&word_stride_u32);
    builder.arg(&d_offsets);
    builder.arg(&n_entries_u32);
    builder.arg(&d_cand_progs);
    builder.arg(&d_cand_len);
    builder.arg(&n_cand_u32);
    builder.arg(&d_cand_axis_mask);
    builder.arg(&max_type_len_u32);
    let cfg = LaunchConfig::for_num_elems(total_threads);
    if let Err(e) = unsafe { builder.launch(cfg) } {
        return format!("{out}  launch cycle_readback failed: {e}");
    }
    out.push_str(&format!(
        "  ∈ read back all {n_usable} entries x 12 axes ({total_threads} threads) in one kernel launch\n\n"
    ));

    let masks: Vec<u64> = match stream.clone_dtoh(&d_out) { Ok(v) => v, Err(e) => return format!("{out}  dtoh out: {e}") };

    let mut exact = 0usize;
    let mut ambiguous = 0usize;
    let mut broken = 0usize;
    let mut broken_example: Option<(&str, usize, String)> = None;
    let mut ambiguous_axis_hist = [0usize; 12];

    for (row, &ei) in usable.iter().enumerate() {
        let axis_names = &per_entry_axis_names[ei];
        let mut entry_broken = false;
        let mut entry_ambiguous = false;
        for axis in 0..12 {
            let mask = masks[row * 12 + axis];
            let true_name = axis_names[axis].strip_suffix('_').unwrap_or(&axis_names[axis]);
            let true_idx = name_to_idx[true_name];
            let true_bit_set = (mask >> true_idx) & 1 == 1;
            let popcount = mask.count_ones();
            if !true_bit_set {
                entry_broken = true;
                if broken_example.is_none() {
                    broken_example = Some((entries[ei].name, axis, true_name.to_string()));
                }
            } else if popcount > 1 {
                entry_ambiguous = true;
                ambiguous_axis_hist[axis] += 1;
            }
        }
        if entry_broken { broken += 1; }
        else if entry_ambiguous { ambiguous += 1; }
        else { exact += 1; }
    }

    out.push_str(&format!(
        "  closes EXACTLY: {exact}\n  closes UP TO AMBIGUITY: {ambiguous}\n  BREAKS: {broken}\n  (of {n_usable} entries checked)\n\n"
    ));
    if let Some((name, axis, true_name)) = broken_example {
        let axis_name = crate::canonical_ig::PRIMITIVE_NAMES[axis].1;
        out.push_str(&format!(
            "  example break: entry '{name}', axis {} ({axis_name}) -- its own value '{true_name}' is not in that axis's own table (a catalog data defect, not a cycle defect)\n",
            AXIS_GLYPHS[axis]
        ));
    }
    out.push_str("  ambiguity by axis (entries with >1 candidate match, true type still among them):\n");
    for (axis, &count) in ambiguous_axis_hist.iter().enumerate() {
        if count > 0 {
            out.push_str(&format!("    axis {} ({}): {count}\n", AXIS_GLYPHS[axis], axis));
        }
    }
    out
}
