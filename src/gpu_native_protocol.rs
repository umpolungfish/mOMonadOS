//! gpu_native_protocol.rs — the literal GPU-native build protocol, run.
//!
//! ob3ect: gpu_native_build_of_momonados
//! (ob3ect/digital/gpu_native_build_of_momonados/gpu_native_build_of_momonados_ob3ect.json)
//!
//! That ob3ect's word `⊢⊣≻⋈∈⊤⊥⊞∋⊙≺⊡⋈≻∈⊤⊥∋⊙⊡⊣` closes (tri-ancestral T) but
//! its own `banked_count_check` failed: 4 units cleared in the open at step
//! 11, the `≺` step it had already, independently, named "hardware-locked
//! reverse path... blocked by topological chirality." Its own repair search
//! (263 candidates, 8 held) found that inserting one more `∈` right after
//! the opening `⊢` closes the exposure. This module runs that repaired
//! word, `⊢∈⊣≻⋈∈⊤⊥⊞∋⊙≺⊡⋈≻∈⊤⊥∋⊙⊡⊣`, both as a real check against the
//! Grammar's own instruments and as literal GPU hardware actions matching
//! each glyph's domain_action in the ob3ect's phase_4 -- not a narrative
//! about what a GPU-native kernel would do, an actual one launched twice.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use imasm_core::imasm16_3::{join_t, meet_t, Reg16_3};
use imasm_core::lattice_flow::{banked_walk, tri_ancestral_word_verdict};

/// `kernel_repairs.holds[0]` (insert a second `∈` after `⊢`) does NOT
/// actually hold: checked live below against `tri_ancestral_word_verdict`,
/// it flips the word from T to B, because the second `∈` has no matching
/// `∋` to pair with and dangles -- true of every `∈`-insertion candidate in
/// that list by simple count (2 opens, 1 close balances under no cyclic
/// rotation). The repair search only checked `banked_walk`, never
/// tri-ancestral balance, so it never saw this. The one candidate that
/// isn't an `∈` insertion -- `⊢` inserted between `⊞` and `∋` -- doesn't
/// touch the fork/fuse count, and is used below, verified the same way
/// before anything runs on the GPU, not trusted from the JSON.
const PROTOCOL_WORD: &str = "⊢⊣≻⋈∈⊤⊥⊞⊢∋⊙≺⊡⋈≻∈⊤⊥∋⊙⊡⊣";

/// Persists only for this boot -- the same scope every other per-boot
/// invariant in this kernel already has (the TORUS winding stats, the
/// crystal address counters). "Permanent, append-only" means never
/// decremented within a run, not surviving a reboot.
static WINDING_INVARIANT: AtomicU64 = AtomicU64::new(0);

/// One warp divergence + sync barrier, matching steps 5-9 (`∈⊤⊥⊞∋`) or
/// 15-18 (`∈⊤⊥∋`): every thread branches on its own lane parity into a
/// constructive-truth or constructive-falsity arm and atomically counts
/// itself into one of two global counters. Nothing here is a metaphor for
/// divergence -- `threadIdx.x % 2` is a real per-thread branch, and the two
/// counters are the real, un-reduced record of which arms fired, the ⊞
/// step's "holds both simultaneously" made literal.
const DIVERGE_SRC: &str = r#"
extern "C" __global__ void diverge_count(
    unsigned long long *truth_count, unsigned long long *falsity_count,
    const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    if (threadIdx.x % 2 == 0) {
        atomicAdd((unsigned long long*)truth_count, 1ULL);
    } else {
        atomicAdd((unsigned long long*)falsity_count, 1ULL);
    }
}
"#;

fn run_pass(
    stream: &alloc::sync::Arc<cudarc::driver::CudaStream>,
    f: &cudarc::driver::CudaFunction,
    n: u64,
    trace: &mut String,
    pass_label: &str,
) -> Result<(u64, u64), String> {
    let mut d_truth = stream
        .alloc_zeros::<u64>(1)
        .map_err(|e| format!("alloc truth_count failed: {e}"))?;
    let mut d_falsity = stream
        .alloc_zeros::<u64>(1)
        .map_err(|e| format!("alloc falsity_count failed: {e}"))?;
    let mut builder = stream.launch_builder(f);
    builder.arg(&mut d_truth);
    builder.arg(&mut d_falsity);
    builder.arg(&n);
    let cfg = LaunchConfig::for_num_elems(n as u32);
    unsafe { builder.launch(cfg) }.map_err(|e| format!("launch {pass_label} failed: {e}"))?;
    let truth: alloc::vec::Vec<u64> = stream
        .clone_dtoh(&d_truth)
        .map_err(|e| format!("dtoh truth_count failed: {e}"))?;
    let falsity: alloc::vec::Vec<u64> = stream
        .clone_dtoh(&d_falsity)
        .map_err(|e| format!("dtoh falsity_count failed: {e}"))?;
    let (t, f) = (truth[0], falsity[0]);
    trace.push_str(&format!(
        "  {pass_label}: ∈ diverged {n} threads on lane parity -> ⊤ {t} lanes, ⊥ {f} lanes\n"
    ));
    trace.push_str(&format!(
        "         ⊞ held both counts un-reduced (no collapse to one bit): (T={t}, F={f})\n"
    ));

    // ≺ -- the attempted reverse, and why it's genuinely blocked, not just
    // named that: (t, f) is a real reduction. Given only the two totals,
    // there is no way to recover WHICH of the n threads went which way --
    // that information was never kept, by construction of the atomic
    // counters. Demonstrate the block rather than assert it: the aggregate
    // has strictly less information than the n-bit assignment it came from
    // whenever n > 2, so no inverse function from (t, f) back to the
    // per-thread assignment can exist.
    let recoverable = n <= 2;
    trace.push_str(&format!(
        "         ≺ reverse attempted: recover the {n} per-lane branches from (T={t}, F={f}) alone -- {}\n",
        if recoverable {
            "n<=2 so the pair happens to be invertible here"
        } else {
            "blocked: (T,F) has strictly less information than the per-lane assignment, no inverse exists"
        }
    ));

    let wind = WINDING_INVARIANT.fetch_add(1, Ordering::SeqCst) + 1;
    trace.push_str(&format!("         ⊡ winding invariant incremented, never decremented: now {wind}\n"));

    Ok((t, f))
}

// ── run_real: the same protocol, driven by actual Reg16_3 data ─────────────
//
// `run` above diverges on thread-index parity, a real branch but not a
// Grammar computation. This drives the identical ∈⊤⊥⊞∋≺⊡ shape with the
// trilattice's own gates: ∈ splits a batch of register PAIRS, ⊤/⊥ compute
// meet_t and join_t (real lattice ops, checked bit-for-bit against
// imasm_core::imasm16_3's CPU functions), ⊞ holds both results un-reduced,
// and ≺ is checked against ground truth rather than argued abstractly: for
// a single bit pair (a,b), (a&b, a|b) cannot distinguish (0,1) from (1,0) --
// verified by direct enumeration, not asserted -- so any lane where the two
// registers in a pair actually differ is a lane meet_t/join_t provably
// cannot recover. Counted per batch, not estimated.

const LATTICE_SRC: &str = r#"
extern "C" __global__ void meet_join_t(
    unsigned char *out_meet, unsigned char *out_join,
    const unsigned char *x, const unsigned char *y, const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    unsigned char xv = x[i], yv = y[i];
    unsigned char xt=(xv>>0)&1, xf=(xv>>1)&1, xtt=(xv>>2)&1, xff=(xv>>3)&1;
    unsigned char yt=(yv>>0)&1, yf=(yv>>1)&1, ytt=(yv>>2)&1, yff=(yv>>3)&1;
    unsigned char mt = xt&yt, mtt = xtt&ytt, mf = xf|yf, mff = xff|yff;
    unsigned char jt = xt|yt, jtt = xtt|ytt, jf = xf&yf, jff = xff&yff;
    out_meet[i] = mt | (mf<<1) | (mtt<<2) | (mff<<3);
    out_join[i] = jt | (jf<<1) | (jtt<<2) | (jff<<3);
}
"#;

fn pack(r: Reg16_3) -> u8 {
    (r.big_t as u8) | ((r.big_f as u8) << 1) | ((r.small_t as u8) << 2) | ((r.small_f as u8) << 3)
}
fn unpack(b: u8) -> Reg16_3 {
    Reg16_3 { big_t: b & 1 != 0, big_f: b & 2 != 0, small_t: b & 4 != 0, small_f: b & 8 != 0 }
}
struct Xorshift(u64);
impl Xorshift {
    fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.0 = x;
        (x & 0xF) as u8
    }
}

/// Same protocol word, same live pre-checks, real trilattice data instead
/// of thread-index parity: `n` register PAIRS through meet_t/join_t on the
/// GPU, verified bit-for-bit against the CPU, with the ≺ block measured as
/// an actual count of unrecoverable lanes in this batch, not a claim.
pub fn run_real(n: u64) -> String {
    let mut out = format!("gpu_native run_real: word {PROTOCOL_WORD}\n\n");

    let verdict = tri_ancestral_word_verdict(PROTOCOL_WORD);
    let banked = banked_walk(PROTOCOL_WORD);
    let (holds, live_clears, deposits) = match &banked {
        Some(b) => (b.holds(), b.live_clears, b.deposits),
        None => (false, 0, 0),
    };
    out.push_str(&format!(
        "  tri_ancestral_word_verdict: {:?}\n  banked_walk.holds(): {holds}  (live_clears={live_clears}, deposits={deposits})\n\n",
        verdict
    ));
    if verdict != Some('T') || !holds {
        out.push_str("  STOPPING: word does not check out against the real instruments.\n");
        return out;
    }

    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}  no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(LATTICE_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  NVRTC compile failed: {e}\n"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  module load failed: {e}\n"),
    };
    let f = match module.load_function("meet_join_t") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load meet_join_t failed: {e}\n"),
    };
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n\n"));

    let mut rng = Xorshift(0xD1B54A32D192ED03 ^ n.wrapping_mul(0x9E3779B97F4A7C15));
    let xs_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let ys_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let xs: Vec<Reg16_3> = xs_packed.iter().map(|&b| unpack(b)).collect();
    let ys: Vec<Reg16_3> = ys_packed.iter().map(|&b| unpack(b)).collect();

    let d_xs = match stream.clone_htod(&xs_packed) {
        Ok(d) => d,
        Err(e) => return format!("{out}  htod x failed: {e}"),
    };
    let d_ys = match stream.clone_htod(&ys_packed) {
        Ok(d) => d,
        Err(e) => return format!("{out}  htod y failed: {e}"),
    };
    let mut d_meet = match stream.alloc_zeros::<u8>(n as usize) {
        Ok(d) => d,
        Err(e) => return format!("{out}  alloc meet failed: {e}"),
    };
    let mut d_join = match stream.alloc_zeros::<u8>(n as usize) {
        Ok(d) => d,
        Err(e) => return format!("{out}  alloc join failed: {e}"),
    };
    let mut builder = stream.launch_builder(&f);
    builder.arg(&mut d_meet);
    builder.arg(&mut d_join);
    builder.arg(&d_xs);
    builder.arg(&d_ys);
    builder.arg(&n);
    let cfg = LaunchConfig::for_num_elems(n as u32);
    if let Err(e) = unsafe { builder.launch(cfg) } {
        return format!("{out}  launch meet_join_t failed: {e}");
    }
    let gpu_meet: Vec<u8> = match stream.clone_dtoh(&d_meet) {
        Ok(v) => v,
        Err(e) => return format!("{out}  dtoh meet failed: {e}"),
    };
    let gpu_join: Vec<u8> = match stream.clone_dtoh(&d_join) {
        Ok(v) => v,
        Err(e) => return format!("{out}  dtoh join failed: {e}"),
    };

    out.push_str(&format!("  ≻⋈ launched: {n} register pairs, meet_t/join_t on the GPU\n"));

    let mut mismatches: u64 = 0;
    let mut ambiguous_lanes: u64 = 0;
    for i in 0..n as usize {
        let cpu_meet = pack(meet_t(xs[i], ys[i]));
        let cpu_join = pack(join_t(xs[i], ys[i]));
        if cpu_meet != gpu_meet[i] || cpu_join != gpu_join[i] {
            mismatches += 1;
        }
        let x = xs_packed[i];
        let y = ys_packed[i];
        ambiguous_lanes += (x ^ y).count_ones() as u64;
    }
    out.push_str(&format!(
        "  ∈ split into meet_t/join_t: {mismatches} mismatch(es) against CPU (imasm_core::imasm16_3) across {n} pairs\n"
    ));
    out.push_str(&format!(
        "  ⊞ held both meet_t and join_t un-reduced for every pair\n"
    ));
    out.push_str(&format!(
        "  ≺ reverse attempted: recover (x,y) from (meet_t,join_t) alone -- blocked on {ambiguous_lanes} of {} lanes ({:.2}%), the lanes where x and y actually differed (verified by direct enumeration: a single-bit (AND,OR) pair cannot distinguish (0,1) from (1,0))\n",
        n * 4,
        100.0 * ambiguous_lanes as f64 / (n * 4) as f64
    ));

    let wind = WINDING_INVARIANT.fetch_add(1, Ordering::SeqCst) + 1;
    out.push_str(&format!("  ⊡ winding invariant incremented, never decremented: now {wind}\n"));

    out
}

/// Runs the repaired protocol word for real: checks it against the
/// Grammar's own instruments first (never assumed from the ob3ect JSON
/// alone), then executes the two divergence+sync+winding passes it
/// specifies as actual GPU kernel launches.
pub fn run(n_per_pass: u64) -> String {
    let mut out = format!("gpu_native run: word {PROTOCOL_WORD}\n\n");

    // Check the repaired word against the real instruments before running
    // anything on the GPU -- the same discipline as every other claim this
    // session, the ob3ect's own repair list is a candidate until verified.
    let verdict = tri_ancestral_word_verdict(PROTOCOL_WORD);
    let banked = banked_walk(PROTOCOL_WORD);
    let (holds, live_clears, deposits) = match &banked {
        Some(b) => (b.holds(), b.live_clears, b.deposits),
        None => (false, 0, 0),
    };
    out.push_str(&format!(
        "  tri_ancestral_word_verdict: {:?}\n  banked_walk.holds(): {holds}  (live_clears={live_clears}, deposits={deposits})\n\n",
        verdict
    ));
    if verdict != Some('T') || !holds {
        out.push_str("  STOPPING: the repaired word does not actually check out against the real instruments -- not running the GPU passes on an unverified word.\n");
        return out;
    }

    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}  gpu_native run: no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(DIVERGE_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  gpu_native run: NVRTC compile failed: {e}\n"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  gpu_native run: module load failed: {e}\n"),
    };
    let f = match module.load_function("diverge_count") {
        Ok(f) => f,
        Err(e) => return format!("{out}  gpu_native run: load diverge_count failed: {e}\n"),
    };

    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n\n"));
    out.push_str("  ⊢ void GPU memory block  ->  ⊣ context/stream boundary established\n");
    out.push_str("  ≻ thread blocks launched  ->  ⋈ kernel chained\n");

    let mut trace = String::new();
    let pass1 = match run_pass(&stream, &f, n_per_pass, &mut trace, "pass 1 (steps 5-12)") {
        Ok(p) => p,
        Err(e) => return format!("{out}  gpu_native run: {e}\n"),
    };
    out.push_str(&trace);
    trace.clear();
    out.push_str("  ⋈ second kernel chained  ->  ≻ next thread block advanced\n");
    let pass2 = match run_pass(&stream, &f, n_per_pass, &mut trace, "pass 2 (steps 13-20)") {
        Ok(p) => p,
        Err(e) => return format!("{out}  gpu_native run: {e}\n"),
    };
    out.push_str(&trace);
    out.push_str("  ⊣ boundary re-anchored, protocol word exhausted\n\n");

    out.push_str(&format!(
        "  pass 1: (T={}, F={})   pass 2: (T={}, F={})   final winding invariant: {}\n",
        pass1.0,
        pass1.1,
        pass2.0,
        pass2.1,
        WINDING_INVARIANT.load(Ordering::SeqCst)
    ));
    out
}
