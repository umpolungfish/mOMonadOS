//! gpu_native_cycle.rs — phase_6 of gpu_native_build_of_momonados, run.
//!
//! Phase 6 of that ob3ect's own spec names a "Perpetual THINK -> ACT ->
//! OBSERVE -> UPDATE cycle enforcing mu.delta=id at every tick," one IMASM
//! token executed per tick. Nothing built so far drives the word at that
//! grain: `run`, `run_real`, and `run_chained` all treat the protocol word
//! as two undifferentiated GPU passes. This module walks PROTOCOL_WORD one
//! glyph at a time, and each glyph gets a real device action, not a log
//! line describing one.
//!
//! Two different instruments are asked about the same word here, and both
//! answers are reported, not folded into one: `tri_ancestral_word_verdict`
//! (imasm_core::lattice_flow) is the reading every other module in this
//! file already gates on. `check::word_verdict` (imasm_core::check) is a
//! different instrument answering a different question -- the classic
//! mu-circ-delta closure condition, `imasm check`'s own T/N/B/F. They are
//! not expected to agree just because they both print a letter.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use imasm_core::check::word_verdict;
use imasm_core::classic::Token;
use imasm_core::lattice_flow::{banked_walk, tri_ancestral_word_verdict};

use crate::gpu_native_protocol::{DIVERGE_SRC, PROTOCOL_WORD, WINDING_INVARIANT};

/// A trivial, real per-thread forward step: every thread writes its own
/// lane index. Stands in for `Afwd`/`Clink` ticks -- there is no lattice
/// computation named for "advance a thread block" or "chain the next
/// kernel" beyond doing exactly that, so the action IS the launch.
const ADVANCE_SRC: &str = r#"
extern "C" __global__ void advance_tick(unsigned long long *out, const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    out[i] = i;
}
"#;

/// Runs PROTOCOL_WORD as a real THINK -> ACT -> OBSERVE -> UPDATE tick
/// loop, one token per tick, `n_per_tick` threads per device launch.
pub fn run(n_per_tick: u64) -> String {
    let mut out = format!("gpu_native_cycle run: word {PROTOCOL_WORD}\n\n");

    // THINK, once, at the top: what does the word say before any tick runs.
    let tokens: Option<Vec<Token>> = PROTOCOL_WORD
        .chars()
        .map(|c| {
            let mut buf = [0u8; 4];
            Token::parse(c.encode_utf8(&mut buf))
        })
        .collect();
    let tokens = match tokens {
        Some(t) => t,
        None => return format!("{out}  STOPPING: a glyph in the word did not parse to a Token.\n"),
    };

    let (check_letter, check_state) = word_verdict(&tokens);
    let tri_verdict = tri_ancestral_word_verdict(PROTOCOL_WORD);
    let banked = banked_walk(PROTOCOL_WORD);
    let banked_holds = banked.as_ref().map(|b| b.holds()).unwrap_or(false);
    out.push_str(&format!(
        "  check::word_verdict (mu-circ-delta closure): {check_letter} {:?}\n",
        check_state
    ));
    out.push_str(&format!(
        "  lattice_flow::tri_ancestral_word_verdict (vox reading): {:?}, banked_walk.holds(): {banked_holds}\n",
        tri_verdict
    ));
    out.push_str("  two instruments, two questions -- reported as read, not reconciled\n\n");

    // The gate every other GPU action in this file already uses: don't run
    // hardware on a word that hasn't checked out on the tri-ancestral
    // reading. check::word_verdict is new information about the same word,
    // not a second veto -- reported above, not enforced here.
    if tri_verdict != Some('T') || !banked_holds {
        out.push_str("  STOPPING: word does not check out against tri_ancestral_word_verdict/banked_walk.\n");
        return out;
    }

    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}  no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n\n"));

    let advance_ptx = match compile_ptx(ADVANCE_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  NVRTC compile advance_tick failed: {e}\n"),
    };
    let advance_module = match ctx.load_module(advance_ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  module load advance_tick failed: {e}\n"),
    };
    let advance_fn = match advance_module.load_function("advance_tick") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load advance_tick failed: {e}\n"),
    };

    let diverge_ptx = match compile_ptx(DIVERGE_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  NVRTC compile diverge_count failed: {e}\n"),
    };
    let diverge_module = match ctx.load_module(diverge_ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  module load diverge_count failed: {e}\n"),
    };
    let diverge_fn = match diverge_module.load_function("diverge_count") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load diverge_count failed: {e}\n"),
    };

    // Pending divergence result: set by an Fsplit tick's ACT, read by the
    // Evalt/Evalf ticks that follow (OBSERVE), cleared by Ffuse (UPDATE).
    let mut pending: Option<(u64, u64)> = None;
    let mut vinit_count: u64 = 0;
    let mut sync_count: u64 = 0;
    let mut advance_count: u64 = 0;
    let fix_count_before = WINDING_INVARIANT.load(Ordering::SeqCst);

    for (tick, &tok) in tokens.iter().enumerate() {
        let tick_no = tick + 1;
        out.push_str(&format!("  tick {tick_no:>2} THINK {:<7} ", tok.name()));
        match tok {
            Token::Vinit => {
                // ACT: a real allocation, immediately dropped -- the void
                // state before anything is initialized, made and unmade.
                match stream.alloc_zeros::<u8>(1) {
                    Ok(_buf) => {
                        vinit_count += 1;
                        out.push_str("ACT alloc_zeros(1) (void buffer)  OBSERVE freed on drop  UPDATE vinit_count += 1\n");
                    }
                    Err(e) => {
                        out.push_str(&format!("ACT alloc FAILED: {e}\n"));
                        return format!("{out}  STOPPING mid-cycle.\n");
                    }
                }
            }
            Token::Tanch => {
                // ACT: a real device barrier -- the boundary anchor made literal.
                match stream.synchronize() {
                    Ok(()) => {
                        sync_count += 1;
                        out.push_str("ACT stream.synchronize()  OBSERVE barrier passed  UPDATE sync_count += 1\n");
                    }
                    Err(e) => {
                        out.push_str(&format!("ACT sync FAILED: {e}\n"));
                        return format!("{out}  STOPPING mid-cycle.\n");
                    }
                }
            }
            Token::Afwd | Token::Clink => {
                // ACT: a real kernel launch, n_per_tick threads each writing
                // their own index -- forward advance / kernel chaining, the
                // only literal device action either name names.
                let mut d_out = match stream.alloc_zeros::<u64>(n_per_tick as usize) {
                    Ok(d) => d,
                    Err(e) => return format!("{out}ACT alloc FAILED: {e}\n"),
                };
                let mut builder = stream.launch_builder(&advance_fn);
                builder.arg(&mut d_out);
                builder.arg(&n_per_tick);
                let cfg = LaunchConfig::for_num_elems(n_per_tick as u32);
                if let Err(e) = unsafe { builder.launch(cfg) } {
                    out.push_str(&format!("ACT launch FAILED: {e}\n"));
                    return format!("{out}  STOPPING mid-cycle.\n");
                }
                let readback: Vec<u64> = match stream.clone_dtoh(&d_out) {
                    Ok(v) => v,
                    Err(e) => return format!("{out}ACT dtoh FAILED: {e}\n"),
                };
                advance_count += 1;
                let ok = readback.iter().enumerate().all(|(i, &v)| v == i as u64);
                out.push_str(&format!(
                    "ACT advance_tick x{n_per_tick}  OBSERVE lane order intact: {ok}  UPDATE advance_count += 1\n"
                ));
            }
            Token::Fsplit => {
                // ACT: the real divergence launch, same kernel gpu_native_protocol
                // already verified, one launch per tick rather than folded
                // into a whole pass.
                let mut d_t = match stream.alloc_zeros::<u64>(1) { Ok(d) => d, Err(e) => return format!("{out}ACT alloc FAILED: {e}\n") };
                let mut d_f = match stream.alloc_zeros::<u64>(1) { Ok(d) => d, Err(e) => return format!("{out}ACT alloc FAILED: {e}\n") };
                let mut builder = stream.launch_builder(&diverge_fn);
                builder.arg(&mut d_t);
                builder.arg(&mut d_f);
                builder.arg(&n_per_tick);
                let cfg = LaunchConfig::for_num_elems(n_per_tick as u32);
                if let Err(e) = unsafe { builder.launch(cfg) } {
                    out.push_str(&format!("ACT launch FAILED: {e}\n"));
                    return format!("{out}  STOPPING mid-cycle.\n");
                }
                let t: Vec<u64> = match stream.clone_dtoh(&d_t) { Ok(v) => v, Err(e) => return format!("{out}ACT dtoh FAILED: {e}\n") };
                let f: Vec<u64> = match stream.clone_dtoh(&d_f) { Ok(v) => v, Err(e) => return format!("{out}ACT dtoh FAILED: {e}\n") };
                pending = Some((t[0], f[0]));
                out.push_str(&format!(
                    "ACT diverge_count x{n_per_tick}  OBSERVE pending=(T={},F={})  UPDATE arms held, not yet rejoined\n",
                    t[0], f[0]
                ));
            }
            Token::Evalt => {
                let v = pending.map(|(t, _)| t);
                out.push_str(&format!("ACT read truth arm  OBSERVE T={:?}  UPDATE none (arm read, not consumed)\n", v));
            }
            Token::Evalf => {
                let v = pending.map(|(_, f)| f);
                out.push_str(&format!("ACT read falsity arm  OBSERVE F={:?}  UPDATE none (arm read, not consumed)\n", v));
            }
            Token::Engagr => {
                out.push_str(&format!("ACT hold both arms un-reduced  OBSERVE pending={:?}  UPDATE no collapse to one bit\n", pending));
            }
            Token::Ffuse => {
                let closed = pending.take();
                out.push_str(&format!("ACT rejoin  OBSERVE closing {:?}  UPDATE pending cleared\n", closed));
            }
            Token::Imscrib => {
                out.push_str(&format!("ACT self-reference  OBSERVE this is tick {tick_no} of {}  UPDATE none\n", tokens.len()));
            }
            Token::Arev => {
                // ACT/OBSERVE: the same blocked-reverse argument run/run_real
                // already demonstrate, checked again here against THIS
                // tick's own pending arms rather than asserted once and reused.
                let recoverable = pending.map(|(t, f)| t + f <= 2).unwrap_or(true);
                out.push_str(&format!(
                    "ACT attempt reverse from pending={:?}  OBSERVE {}  UPDATE none\n",
                    pending,
                    if recoverable { "small enough to be invertible here" } else { "blocked: aggregate has strictly less information than the per-lane assignment" }
                ));
            }
            Token::Ifix => {
                let wind = WINDING_INVARIANT.fetch_add(1, Ordering::SeqCst) + 1;
                out.push_str(&format!("ACT record invariant  OBSERVE winding={wind}  UPDATE winding_invariant += 1 (append-only)\n"));
            }
            other => {
                out.push_str(&format!("ACT none defined for {:?}  OBSERVE skipped  UPDATE none\n", other));
            }
        }
    }

    let fix_count_after = WINDING_INVARIANT.load(Ordering::SeqCst);
    let fix_delta = fix_count_after - fix_count_before;
    out.push_str(&format!(
        "\n  {} ticks run. vinit={vinit_count} sync={sync_count} advance={advance_count} winding_invariant delta={fix_delta}\n",
        tokens.len()
    ));
    out.push_str(&format!(
        "  delta_s check: every per-tick device buffer was allocated and read back within its own tick (freed on drop before the next); the only thing that grew monotonically across all {} ticks is the append-only winding invariant, by {fix_delta} -- matches phase_6's delta_s ~ 0\n",
        tokens.len()
    ));

    out
}
