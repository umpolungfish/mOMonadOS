//! yz.rs — the retract-vocabulary translation of Yamakawa-Zhandry's
//! "Verifiable Quantum Advantage without Structure," run as real code
//! against `belnap.rs`'s r/c/Inc.
//!
//! Scope, stated plainly so this is never read as more than it is: this
//! implements and runs the STRUCTURAL claims of the YZ-retranslation --
//! r and c as distinct adjoints (Theorem 5.5), Inc as the constant
//! paraconsistent closure, Corollary 11.2 (r∘Inc=T, c∘Inc=F, both
//! constant), and §1's walker picture (a {r,c}-strategy's register never
//! leaves the Boolean core; an Inc-seeded register carries a value the
//! core cannot). It does NOT implement Theorem 11.1's actual separation
//! bound (the 2^{-Ω(λ)} soundness number, list-recoverability, the folded
//! Reed-Solomon code) -- that is real cryptographic content the
//! YZ-retranslation document itself marks Tier-3, open, not yet derived
//! here. Anything below that isn't that bound is a structural fact,
//! checked, not a substitute for it.

#![allow(dead_code)]

use crate::belnap::{c, corollary_11_2_report, inc, r, theorem_5_5_report, B4};
use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

struct Xorshift(u64);
impl Xorshift {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_b4(&mut self) -> B4 {
        match self.next_u64() & 3 {
            0 => B4::N,
            1 => B4::T,
            2 => B4::F,
            _ => B4::B,
        }
    }
    /// Draws only from {N, T, F} -- never B. Used wherever the point is
    /// to show what Inc itself contributes: if the ambient stream can
    /// hand out B on its own, then an accumulator reaching B proves
    /// nothing about Inc specifically, since plain join(T, F) = B too
    /// (T=01, F=10, OR=11=B in the encoding) -- an ordinary information-
    /// order collision, not evidence of the closure. Excluding B from the
    /// stream is what makes "the accumulator reached B" attributable to
    /// Inc alone.
    fn next_b4_no_b(&mut self) -> B4 {
        match self.next_u64() % 3 {
            0 => B4::N,
            1 => B4::T,
            _ => B4::F,
        }
    }
    /// Draws only from {N, T} -- excludes both B and F. A join-chain of
    /// nothing but N and T can only ever be N or T; it needs an F to ever
    /// reach B (T=01, F=10, OR needs both bits). Used strictly before a
    /// walk's closure step, so B provably cannot appear before Inc
    /// applies -- not just probably won't, given next_b4_no_b alone could
    /// still collide a T and an F by chance and reach B before Inc ever
    /// ran.
    fn next_b4_no_b_no_f(&mut self) -> B4 {
        if self.next_u64() & 1 == 0 { B4::N } else { B4::T }
    }
}

/// §1's CROM adversary, run for real: at every step an ambient value
/// arrives (stood in here by a random draw over all four B4 values --
/// the adversary does not get to choose what the ambient offers) and the
/// walker retracts it through its one fixed adjoint before ever touching
/// it. Checks, at every single step, that the retracted value actually
/// landed in the Boolean core {T,F} -- not asserted from r/c's type,
/// measured over the walk.
pub fn crom_walk(steps: usize, seed: u64, use_coreflector: bool) -> String {
    let mut rng = Xorshift(seed ^ 0xA24BAED4963EE407);
    let mut off_core = 0usize;
    let mut sample = String::new();
    for i in 0..steps {
        let ambient = rng.next_b4();
        let retracted = if use_coreflector { c(ambient) } else { r(ambient) };
        let confined = matches!(retracted, B4::T | B4::F);
        if !confined {
            off_core += 1;
        }
        if i < 8 {
            sample.push_str(&format!(
                "    step {}: ambient={:<2} -> {}(ambient)={:<2}\n",
                i,
                ambient.name(),
                if use_coreflector { "c" } else { "r" },
                retracted.name()
            ));
        }
    }
    let mut out = String::new();
    out.push_str(&format!(
        "crom_walk: {} steps, adjoint={}\n",
        steps,
        if use_coreflector { "c (coreflector)" } else { "r (reflector)" }
    ));
    out.push_str(&sample);
    if steps > 8 {
        out.push_str(&format!("    ... ({} more steps)\n", steps - 8));
    }
    out.push_str(&format!(
        "  steps where the retracted register left {{T,F}}: {} / {}\n",
        off_core, steps
    ));
    out.push_str(&format!(
        "  register never left the Boolean core: {}\n",
        off_core == 0
    ));
    out
}

/// §1's QROM prover / §4's Inc-typed finder: at every step Inc seeds B
/// (its one, constant output) at one clearly marked step, and the
/// running accumulator absorbs it via `join` -- the same information-
/// order join belnap.rs already carries and already has an absorption
/// law for (B.join(x)=B, checked in `belnap_invariants`).
///
/// Before the closure step, the ambient is drawn from {N,T} only
/// (`next_b4_no_b_no_f`): a join-chain of N and T alone can never reach
/// B on its own, so the accumulator provably cannot be B yet when Inc
/// fires -- not just probably won't. (Drawing from {N,T,F} there would
/// still let an ordinary T-then-F collision reach B by chance, T=01,
/// F=10, OR=11=B, and the demonstration would sometimes show B arriving
/// before Inc ever ran.) Whatever the closure step's own ambient value
/// was (T or N -- never B, never F, by construction), Inc(ambient)=B
/// anyway: that IS Inc's constancy, shown rather than assumed. After the
/// closure, the stream widens to {N,T,F} (still never raw B) and
/// `pinned_after_seed` is checked at every remaining step, not assumed
/// from the algebra alone.
pub fn qrom_walk(steps: usize, seed: u64) -> String {
    let mut rng = Xorshift(seed ^ 0x9E3779B97F4A7C15);
    let mut accumulator = B4::N;
    let closure_step = steps / 3;
    let mut closure_ambient = B4::N;
    let mut pinned_after_seed = true;
    let mut b_before_closure = false;
    let mut sample = String::new();
    for i in 0..steps {
        let ambient = if i <= closure_step {
            rng.next_b4_no_b_no_f()
        } else {
            rng.next_b4_no_b()
        };
        if i == closure_step {
            closure_ambient = ambient;
            // Inc ignores what ambient actually was -- that's the point.
            accumulator = accumulator.join(inc(ambient));
        } else {
            accumulator = accumulator.join(ambient);
        }
        if i < closure_step && accumulator == B4::B {
            b_before_closure = true;
        }
        if i > closure_step && accumulator != B4::B {
            pinned_after_seed = false;
        }
        if i < 8 {
            sample.push_str(&format!(
                "    step {}: ambient={:<2}{} accumulator={:<2}\n",
                i,
                ambient.name(),
                if i == closure_step { "  <- Inc applied here" } else { "" },
                accumulator.name()
            ));
        }
    }
    let mut out = String::new();
    out.push_str(&format!(
        "qrom_walk: {} steps, ambient is {{N,T}} pre-closure, {{N,T,F}} post-closure -- B never appears in the raw ambient stream\n",
        steps
    ));
    out.push_str(&sample);
    if steps > 8 {
        out.push_str(&format!("    ... ({} more steps)\n", steps - 8));
    }
    out.push_str(&format!(
        "  accumulator ever reached B before step {} (should never happen): {}\n",
        closure_step, b_before_closure
    ));
    out.push_str(&format!(
        "  Inc applied at step {} to ambient={} (not B): Inc(v)=B regardless of v, shown not assumed\n",
        closure_step, closure_ambient.name()
    ));
    out.push_str(&format!(
        "  accumulator stayed at B for every step after: {}\n",
        pinned_after_seed
    ));
    out.push_str(&format!("  final accumulator: {}\n", accumulator.name()));
    out.push_str(&format!(
        "  r(final)={}  c(final)={}  -- both constant, neither recovers B (Corollary 11.2, applied to this walk's own end state)\n",
        r(accumulator).name(), c(accumulator).name()
    ));
    out
}

/// §3's soundness claim, run as an experiment rather than read as a
/// theorem: many independent trials, each with its own random ambient
/// sequence and its own point where Inc fires. Pre-closure ambient is
/// {N,T} only, same reason and same guarantee as `qrom_walk`: a
/// join-chain of N and T alone cannot reach B by chance, so
/// post-closure's B is attributable to Inc alone, not to an accidental
/// T/F collision that happened to land before the closure step.
///
/// Two different things are measured, deliberately kept apart: the
/// PRE-closure accumulator (built only from ordinary ambient values)
/// genuinely varies trial to trial, and r/c of THAT state also varies --
/// a {r,c} walker reading the computation before the closure fires is
/// not blind at all. Only the POST-closure state is constant, in both
/// its raw value (always B, once Inc has fired and pinned it) and its
/// r/c readout. The soundness claim is about that second part
/// specifically: it is the closure event, not the whole computation,
/// that becomes permanently invisible to a {r,c} walker.
pub fn soundness_trials(trials: usize, steps: usize) -> String {
    let mut pre_varies_count = 0usize;
    let mut pre_r_varies_count = 0usize;
    let mut post_all_b = true;
    let mut post_r_constant_t = true;
    let mut post_c_constant_f = true;
    let mut first_pre: Option<B4> = None;
    let mut first_pre_r: Option<B4> = None;
    let mut first_pre_c: Option<B4> = None;
    let mut pre_c_varies_count = 0usize;
    let mut pre_reached_b_count = 0usize;
    let mut distinct_t_counts: Vec<usize> = Vec::new();
    let closure_step = steps / 3;

    for t in 0..trials {
        let mut rng = Xorshift((t as u64).wrapping_mul(0xD1B54A32D192ED03) ^ 0x2545F4914F6CDD1D);
        let mut pre_accumulator = B4::N;
        let mut post_accumulator = B4::N;
        let mut pre_t_count = 0usize;
        for i in 0..steps {
            if i < closure_step {
                let ambient = rng.next_b4_no_b_no_f();
                if ambient == B4::T { pre_t_count += 1; }
                pre_accumulator = pre_accumulator.join(ambient);
                post_accumulator = pre_accumulator;
            } else if i == closure_step {
                let ambient = rng.next_b4_no_b_no_f();
                post_accumulator = post_accumulator.join(inc(ambient));
            } else {
                let ambient = rng.next_b4_no_b();
                post_accumulator = post_accumulator.join(ambient);
            }
        }
        if !distinct_t_counts.contains(&pre_t_count) {
            distinct_t_counts.push(pre_t_count);
        }
        if pre_accumulator == B4::B { pre_reached_b_count += 1; }
        match first_pre {
            None => first_pre = Some(pre_accumulator),
            Some(v) if v != pre_accumulator => pre_varies_count += 1,
            _ => {}
        }
        let pre_r = r(pre_accumulator);
        match first_pre_r {
            None => first_pre_r = Some(pre_r),
            Some(v) if v != pre_r => pre_r_varies_count += 1,
            _ => {}
        }
        let pre_c = c(pre_accumulator);
        match first_pre_c {
            None => first_pre_c = Some(pre_c),
            Some(v) if v != pre_c => pre_c_varies_count += 1,
            _ => {}
        }
        if post_accumulator != B4::B { post_all_b = false; }
        if r(post_accumulator) != B4::T { post_r_constant_t = false; }
        if c(post_accumulator) != B4::F { post_c_constant_f = false; }
    }

    let mut out = String::new();
    out.push_str(&format!(
        "soundness_trials: {} independent trials, {} steps each, closure at step {}\n",
        trials, steps, closure_step
    ));
    out.push_str(&format!(
        "  pre-closure accumulator reached B in {} / {} trials (should be 0 -- {{N,T}} alone cannot produce it)\n",
        pre_reached_b_count, trials
    ));
    out.push_str(&format!(
        "  pre-closure accumulator (joined, saturating) differed from trial 0 in {} / {} trials\n",
        pre_varies_count, trials
    ));
    out.push_str(&format!(
        "  pre-closure T-count (non-saturating) took {} distinct values across {} trials -- a full-access reader sees real, varying content here\n",
        distinct_t_counts.len(), trials
    ));
    out.push_str(&format!(
        "  r(pre-closure accumulator) differed from trial 0 in {} / {} trials\n",
        pre_r_varies_count, trials
    ));
    out.push_str(&format!(
        "  c(pre-closure accumulator) differed from trial 0 in {} / {} trials\n",
        pre_c_varies_count, trials
    ));
    out.push_str("  reading: r(N)=r(T)=T (see 'yz report'), so the r-walker is constant on this {N,T} domain too -- not because of Inc, just because r already collapses N and T together. The c-walker (c(N)=F, c(T)=T) is the one that can still distinguish pre-closure content; both adjoints go constant only once the closure has fired.\n");
    out.push_str(&format!(
        "  post-closure accumulator was B in every trial: {}\n",
        post_all_b
    ));
    out.push_str(&format!(
        "  r(post-closure) constant at T in every trial: {}   c(post-closure) constant at F in every trial: {}\n",
        post_r_constant_t, post_c_constant_f
    ));
    out.push_str("  reading: the underlying content genuinely varies before the closure (the T-count line), c can still see some of that (its own domain-specific blindness aside), and only the closure event itself -- and everything downstream of it, for both adjoints -- goes constant. That is Corollary 11.2's content, isolated to the specific event it applies to, not the whole walk.\n");
    out.push_str("  this is Corollary 11.2's structural content, run; it is NOT Theorem 11.1's quantitative bound (list size, 2^-Omega(lambda)), which stays open here.\n");
    out
}

pub fn repl_yz(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("yz — the YZ-retranslation's retract machinery (r/c/Inc), run as code, not read as a document");
        sprintln!("  yz report                    Corollary 11.2 and Theorem 5.5, checked exhaustively over all 4 inputs");
        sprintln!("  yz walk <n> [r|c] [seed]     CROM {{r,c}}-walker over n random ambient steps; confirms register stays in {{T,F}}");
        sprintln!("  yz inc-walk <n> [seed]       QROM Inc-walker; shows Inc's B seed gets pinned through join and outlives the walk");
        sprintln!("  yz soundness <trials> <n>    Corollary 11.2 as an experiment: r/c readouts are constant across independent trials");
        sprintln!("  Scope: structural claims only (Corollary 11.2, Theorem 5.5, the walker confinement picture).");
        sprintln!("  Theorem 11.1's actual separation bound is not implemented here -- Tier-3, open, per the source document.");
        return;
    }
    match args[0] {
        "report" => {
            sprintln!("Corollary 11.2 (r∘Inc=T, c∘Inc=F, both constant):");
            sprintln!("{}", corollary_11_2_report());
            sprintln!("Theorem 5.5 (r and c erase N,B in opposite directions):");
            sprintln!("{}", theorem_5_5_report());
        }
        "walk" => {
            let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20);
            let use_c = args.get(2).copied() == Some("c");
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
            sprintln!("{}", crom_walk(n, seed, use_c));
        }
        "inc-walk" => {
            let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20);
            let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            sprintln!("{}", qrom_walk(n, seed));
        }
        "soundness" => {
            let trials: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000);
            let steps: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
            sprintln!("{}", soundness_trials(trials, steps));
        }
        other => sprintln!("yz: unknown subcommand '{}' (try 'yz help')", other),
    }
}
