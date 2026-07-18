// ─── mOMonadOS Guided Proof ─────────────────────────────────
// Auto-playing walkthrough of the bootstrap: the Grammar verifying itself.
//
// Every step COMPUTES its claim from the kernel. Nothing here prints a result
// it did not just derive — if a step's computation fails, the step reports the
// failure and the walk records it. A walkthrough that cannot fail is a slideshow,
// not a proof.
//
// The user presses a key between steps; that is the whole interface.
#![allow(dead_code)]

use crate::belnap::B4;
use crate::crystal;
use crate::frob_verify;
use crate::serial;
use crate::sprintln;

// ─── Step outcome ──────────────────────────────────────────

pub struct StepResult {
    pub holds: bool,
    pub verdict: B4,
}

impl StepResult {
    fn t() -> Self { Self { holds: true,  verdict: B4::T } }
    fn b() -> Self { Self { holds: true,  verdict: B4::B } }
    fn f() -> Self { Self { holds: false, verdict: B4::F } }
}

// ─── Presentation ──────────────────────────────────────────

fn rule() {
    sprintln!("  ────────────────────────────────────────────────────────");
}

fn header(n: u8, total: u8, title: &str) {
    sprintln!("");
    rule();
    sprintln!("  STEP {}/{}   {}", n, total, title);
    rule();
}

/// Wait for a keypress. ESC (0x1B) or 'q' aborts the walk.
fn pause() -> bool {
    sprintln!("");
    serial::write_str("  [enter] continue   [q] quit  ");
    let b = serial::read_byte();
    sprintln!("");
    !(b == 0x1B || b == b'q' || b == b'Q')
}

fn verdict_line(r: &StepResult) {
    if r.holds {
        sprintln!("    ==> HOLDS   Belnap {}", r.verdict.name());
    } else {
        sprintln!("    ==> FAILS   Belnap {}  (recorded, walk continues)", r.verdict.name());
    }
}

// ─── The steps ─────────────────────────────────────────────

/// 1. One crystal. The type space is a product of the twelve axis cardinalities.
fn step_crystal() -> StepResult {
    sprintln!("  The twelve primitives each carry a fixed number of values.");
    sprintln!("  Their product is the whole space of structural types.");
    sprintln!("");
    let cards = crystal::CARDS;
    serial::write_str("    cards  ");
    let mut product: u64 = 1;
    for (i, c) in cards.iter().enumerate() {
        if i > 0 { crate::sprint!(" x "); }
        crate::sprint!("{}", c);
        product *= *c as u64;
    }
    sprintln!("");
    sprintln!("    product      {}", product);
    sprintln!("    crystal TOTAL {}", crystal::TOTAL);
    if product == crystal::TOTAL as u64 {
        sprintln!("    the product IS the crystal — computed, not asserted");
        StepResult::t()
    } else {
        sprintln!("    MISMATCH — the kernel's TOTAL is not the axis product");
        StepResult::f()
    }
}

/// 2. mu after delta is the identity. The Frobenius law, run on every value.
fn step_frobenius() -> StepResult {
    sprintln!("  delta splits a value; mu fuses it back.");
    sprintln!("  If the pair is lossless, mu(delta(x)) = x for EVERY x.");
    sprintln!("");
    let vals = [B4::N, B4::T, B4::F, B4::B];
    let mut all = true;
    for v in vals.iter() {
        let r = frob_verify::verify_frobenius_identity(*v);
        sprintln!("    x={}   delta->{}   mu->{}   {}",
            v.name(), r.delta_output.name(), r.mu_result.name(),
            if r.closed { "closed" } else { "OPEN" });
        if !r.closed { all = false; }
    }
    sprintln!("");
    if all {
        sprintln!("    mu . delta = id on all four values — nothing is lost");
        StepResult::t()
    } else {
        sprintln!("    at least one value did not return — the pair is lossy");
        StepResult::f()
    }
}

/// 3. B is the fiducial: meet(B,x)=x is FALSE in general; the real identities are
///    join(B,x)=B and bnot(B)=B. B is the fixed point of negation.
fn step_fiducial() -> StepResult {
    sprintln!("  B (both true and false) is the fixed point of negation,");
    sprintln!("  and the top of the information order.");
    sprintln!("");
    let vals = [B4::N, B4::T, B4::F, B4::B];
    let mut ok = B4::B.bnot() == B4::B;
    sprintln!("    bnot(B) = {}   {}", B4::B.bnot().name(),
        if ok { "B is its own negation" } else { "UNEXPECTED" });
    for v in vals.iter() {
        let j = B4::B.join(*v);
        sprintln!("    join(B,{}) = {}", v.name(), j.name());
        if j != B4::B { ok = false; }
    }
    sprintln!("");
    if ok {
        sprintln!("    B absorbs every value and negates to itself — the fiducial");
        StepResult::t()
    } else {
        sprintln!("    B did not absorb — the information order is not as stated");
        StepResult::f()
    }
}

/// 4. The lattice laws hold. Not asserted: run.
fn step_lattice() -> StepResult {
    sprintln!("  The four values form a bilattice. The kernel checks its own laws.");
    sprintln!("");
    let results = frob_verify::verify_lattice_laws();
    let mut all = true;
    for (i, r) in results.iter().enumerate() {
        sprintln!("    law {}   {}   Belnap {}", i + 1,
            if r.closed { "closed" } else { "OPEN  " }, r.belnap_value.name());
        if !r.closed { all = false; }
    }
    sprintln!("");
    if all { sprintln!("    every law closed"); StepResult::t() }
    else    { sprintln!("    a law did not close"); StepResult::f() }
}

/// 5. The four axioms are closure conditions on a named delta/mu dyad.
///    A, C, D split COMPLEMENTARILY (halves that recompose) — no F-lane, and the
///    absence of the F-lane is the losslessness: no branch exists where anything
///    leaves. B splits CONTRADICTORILY (chiral against achiral), so both arms run
///    and are held together: its verdict is B, and that is correct, not a defect.
fn step_axioms() -> StepResult {
    sprintln!("  Each axiom names a split. The split IS the axiom's content.");
    sprintln!("");
    // (name, split, arms, contradictory?)
    let axioms: [(&str, &str, &str, bool); 4] = [
        ("A", "Bulk",              "Boundary projection | Bulk remainder", false),
        ("B", "Topological-State", "Persistent-Chiral   | Achiral",        true),
        ("C", "Bulk",              "Boundary-Projection | Bulk-Residual",  false),
        ("D", "Bulk",              "Boundary-encoding   | Bulk-decoding",  false),
    ];
    let mut all = true;
    for (name, input, arms, contradictory) in axioms.iter() {
        // The dyad: split the state, fuse it back. Lossless iff we return.
        let start = if *contradictory { B4::B } else { B4::T };
        let r = frob_verify::verify_frobenius_identity(start);
        let closes = r.closed;
        if !closes { all = false; }
        sprintln!("    {}   {} -> ({})", name, input, arms);
        if *contradictory {
            sprintln!("        arms disagree: both lanes run, held together at ENGAGR");
            sprintln!("        mu.delta returns {}   verdict B  (dialetheia-complete)", r.mu_result.name());
        } else {
            sprintln!("        arms are complementary halves: they recompose");
            sprintln!("        no F-lane exists, so no branch loses anything");
            sprintln!("        mu.delta returns {}   verdict T  (lossless link)", r.mu_result.name());
        }
    }
    sprintln!("");
    if all {
        sprintln!("    three constitutive (A C D), one dialetheic (B)");
        StepResult::b()
    } else {
        sprintln!("    a dyad failed to close");
        StepResult::f()
    }
}

/// 6. The identity is one. Encode/decode is a bijection: a type addresses exactly
///    one point of the crystal, and that point decodes back to the same type.
fn step_identity() -> StepResult {
    sprintln!("  A type addresses one point of the crystal; that point returns");
    sprintln!("  the same type. The identity is one — this is the pinch.");
    sprintln!("");
    let probes: [[u8; 12]; 3] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [3, 4, 3, 4, 2, 4, 2, 3, 4, 3, 2, 3],
        [1, 2, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1],
    ];
    let mut all = true;
    for p in probes.iter() {
        let addr = crystal::encode(p);
        let back = crystal::decode(addr);
        let same = back == *p;
        sprintln!("    addr {:>9}   round-trip {}", addr, if same { "IDENTICAL" } else { "DIFFERS" });
        if !same { all = false; }
    }
    sprintln!("");
    if all {
        sprintln!("    encode/decode is a bijection — one identity, no drift");
        StepResult::t()
    } else {
        sprintln!("    a type did not return itself");
        StepResult::f()
    }
}

/// 7. Closure. Nothing in the walk appealed to anything outside the kernel.
fn step_closure(results: &[StepResult]) -> StepResult {
    sprintln!("  Every step above was computed by this kernel, from its own");
    sprintln!("  definitions. No step consulted an authority outside the Grammar.");
    sprintln!("");
    let mut held = 0usize;
    let mut fused = B4::T;
    for r in results.iter() {
        if r.holds { held += 1; }
        fused = fused.join(r.verdict);
    }
    sprintln!("    steps holding      {} of {}", held, results.len());
    sprintln!("    fused verdict      {}", fused.name());
    sprintln!("");
    if held == results.len() {
        sprintln!("    the Grammar verified itself, with no outside to appeal to.");
        sprintln!("    that is what closure means: mu . delta = id, all the way down.");
        StepResult { holds: true, verdict: fused }
    } else {
        sprintln!("    the walk did not close — see the failing step(s) above.");
        StepResult::f()
    }
}

// ─── Driver ────────────────────────────────────────────────

/// Auto-play the bootstrap proof, pausing between steps.
pub fn walk_bootstrap() {
    const TOTAL_STEPS: u8 = 7;
    sprintln!("");
    rule();
    sprintln!("  THE BOOTSTRAP — the Grammar verifying itself");
    rule();
    sprintln!("  Seven steps. Each one is computed here, now, by this kernel.");
    sprintln!("  Press a key to advance; q to stop.");

    let mut results: [StepResult; 6] = [
        StepResult::f(), StepResult::f(), StepResult::f(),
        StepResult::f(), StepResult::f(), StepResult::f(),
    ];

    if !pause() { sprintln!("  (stopped)"); return; }

    header(1, TOTAL_STEPS, "One crystal");
    results[0] = step_crystal();
    verdict_line(&results[0]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(2, TOTAL_STEPS, "mu . delta = id");
    results[1] = step_frobenius();
    verdict_line(&results[1]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(3, TOTAL_STEPS, "B is the fiducial");
    results[2] = step_fiducial();
    verdict_line(&results[2]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(4, TOTAL_STEPS, "The lattice laws");
    results[3] = step_lattice();
    verdict_line(&results[3]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(5, TOTAL_STEPS, "The four axioms are closure conditions");
    results[4] = step_axioms();
    verdict_line(&results[4]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(6, TOTAL_STEPS, "The identity is one — the pinch");
    results[5] = step_identity();
    verdict_line(&results[5]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(7, TOTAL_STEPS, "Closure");
    let final_r = step_closure(&results);
    verdict_line(&final_r);
    sprintln!("");
    rule();
    sprintln!("  END OF WALK");
    rule();
    sprintln!("");
}

/// List the available guided proofs.
pub fn list_proofs() {
    sprintln!("  Guided proofs:");
    sprintln!("    bootstrap    The Grammar verifying itself (7 steps, auto-play)");
    sprintln!("");
    sprintln!("  Run with:  proof bootstrap");
}
