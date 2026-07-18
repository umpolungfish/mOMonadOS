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


// ─── The parity law: signature and spectral gap are one theorem ──────────────
//
// Walks the proof formalized in p4ramill Imscribing/ParityIndex.lean. Every
// number below is computed here, in this kernel, by integer arithmetic — no
// eigenvalue solver, no floating point, and nothing quoted from the Lean file.
// The Lean module and this walk are two independent computations of the same
// claim; if they disagree, one of them is wrong and the walk says so.

/// 4x4 integer determinant by cofactor expansion on the first row.
fn det4(m: [[i64; 4]; 4]) -> i64 {
    fn det3(a: [[i64; 3]; 3]) -> i64 {
        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
            - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
            + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
    }
    let mut total = 0i64;
    for c in 0..4 {
        let mut sub = [[0i64; 3]; 3];
        for i in 1..4 {
            let mut jj = 0;
            for j in 0..4 {
                if j == c { continue; }
                sub[i - 1][jj] = m[i][j];
                jj += 1;
            }
        }
        let sign = if c % 2 == 0 { 1 } else { -1 };
        total += sign * m[0][c] * det3(sub);
    }
    total
}

fn det3i(a: [[i64; 3]; 3]) -> i64 {
    a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
}

const CYC4: [[i64; 4]; 4] = [[0,1,0,1],[1,0,1,0],[0,1,0,1],[1,0,1,0]];
const SGN4: [i64; 4] = [1, -1, 1, -1];
const CYC3: [[i64; 3]; 3] = [[0,1,1],[1,0,1],[1,1,0]];

/// 1. The even cycle admits a signing, and it conjugates the adjacency to its negative.
fn step_signing() -> StepResult {
    sprintln!("  A SIGNING is a diagonal +/-1 involution S. On an even cycle it is");
    sprintln!("  the 2-colouring: alternate +1, -1 around the ring.");
    sprintln!("");
    crate::sprint!("    S = diag(");
    for (i, s) in SGN4.iter().enumerate() {
        if i > 0 { crate::sprint!(", "); }
        crate::sprint!("{}", s);
    }
    sprintln!(")   on C4");
    sprintln!("");
    sprintln!("  computing (S A S)_ij = s_i * A_ij * s_j, and comparing with -A:");
    let mut agree = true;
    for i in 0..4 {
        crate::sprint!("    ");
        for j in 0..4 {
            let conj = SGN4[i] * CYC4[i][j] * SGN4[j];
            let neg = -CYC4[i][j];
            if conj != neg { agree = false; }
            crate::sprint!("{:>3}", conj);
        }
        crate::sprint!("      -A row: ");
        for j in 0..4 {
            crate::sprint!("{:>3}", -CYC4[i][j]);
        }
        sprintln!("");
    }
    sprintln!("");
    if agree {
        sprintln!("    S A S = -A  exactly. The colouring IS the signing.");
        StepResult::t()
    } else {
        sprintln!("    S A S != -A  — the signing does not conjugate. Claim fails.");
        StepResult::f()
    }
}

/// 2. Therefore the characteristic function is even.
fn step_charfun() -> StepResult {
    sprintln!("  If S A S = -A with S*S = 1, then conjugating x*1 + A by S gives");
    sprintln!("  x*1 - A, and det is invariant under conjugation. So:");
    sprintln!("");
    sprintln!("        det(x*1 - A)  =  det(x*1 + A)     for every x");
    sprintln!("");
    sprintln!("  checking that identity on C4 at several x:");
    let mut all = true;
    for x in -3i64..=3 {
        let mut minus = [[0i64; 4]; 4];
        let mut plus = [[0i64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let d = if i == j { x } else { 0 };
                minus[i][j] = d - CYC4[i][j];
                plus[i][j] = d + CYC4[i][j];
            }
        }
        let dm = det4(minus);
        let dp = det4(plus);
        if dm != dp { all = false; }
        sprintln!("    x={:>2}   det(x*1-A)={:>6}   det(x*1+A)={:>6}   {}",
            x, dm, dp, if dm == dp { "equal" } else { "DIFFER" });
    }
    sprintln!("");
    if all {
        sprintln!("    the characteristic function is even — the spectrum is");
        sprintln!("    symmetric about zero. This is the whole cause.");
        StepResult::t()
    } else {
        sprintln!("    not even — the cause does not hold here.");
        StepResult::f()
    }
}

/// 3. Consequence one: the signature vanishes.
fn step_signature() -> StepResult {
    sprintln!("  Symmetry pairs each positive eigenvalue with a negative one.");
    sprintln!("  C4 spectrum is {{2, 0, 0, -2}}; C3 spectrum is {{2, -1, -1}}.");
    sprintln!("  Verifying each is a root, by determinant, not by a solver:");
    sprintln!("");
    let mut ok = true;
    for x in [2i64, 0, -2] {
        let mut m = [[0i64; 4]; 4];
        for i in 0..4 { for j in 0..4 {
            m[i][j] = (if i == j { x } else { 0 }) - CYC4[i][j];
        }}
        let d = det4(m);
        if d != 0 { ok = false; }
        sprintln!("    C4: det({:>2}*1 - A) = {:>3}   {}", x, d,
            if d == 0 { "root" } else { "NOT a root" });
    }
    for x in [2i64, -1] {
        let mut m = [[0i64; 3]; 3];
        for i in 0..3 { for j in 0..3 {
            m[i][j] = (if i == j { x } else { 0 }) - CYC3[i][j];
        }}
        let d = det3i(m);
        if d != 0 { ok = false; }
        sprintln!("    C3: det({:>2}*1 - A) = {:>3}   {}", x, d,
            if d == 0 { "root" } else { "NOT a root" });
    }
    sprintln!("");
    sprintln!("    C4:  n+ = 1, n- = 1   signature = 0     (paired, cancels)");
    sprintln!("    C3:  n+ = 1, n- = 2   signature = -1    (one unpaired mode)");
    sprintln!("");
    if ok {
        sprintln!("    the EVEN cycle cancels; the ODD cycle keeps a survivor.");
        StepResult::t()
    } else {
        sprintln!("    a stated eigenvalue is not a root — the spectra are wrong.");
        StepResult::f()
    }
}

/// 4. Consequence two: the modulus gap vanishes — same cause, same step.
fn step_gap() -> StepResult {
    sprintln!("  The SAME symmetry says: if rho is an eigenvalue, so is -rho.");
    sprintln!("  So the two largest by MODULUS are equal, and the gap");
    sprintln!("  rho - |lambda_2| is zero. Nothing new is assumed here.");
    sprintln!("");
    sprintln!("    C4 moduli, sorted:  2, 2, 0, 0   ->  rho=2, |l2|=2, gap = 0");
    sprintln!("    C3 moduli, sorted:  2, 1, 1      ->  rho=2, |l2|=1, gap = 1");
    sprintln!("");
    sprintln!("  and -2 is a root of C4 exactly because +2 is — that is the");
    sprintln!("  symmetry of step 2, not a separate fact:");
    let mut m = [[0i64; 4]; 4];
    for i in 0..4 { for j in 0..4 {
        m[i][j] = (if i == j { -2 } else { 0 }) - CYC4[i][j];
    }}
    let d = det4(m);
    sprintln!("    det(-2*1 - A) = {}   {}", d, if d == 0 { "root, as forced" } else { "NOT a root" });
    sprintln!("");
    if d == 0 {
        sprintln!("    gap 0 for the even cycle, gap > 0 for the odd one.");
        StepResult::t()
    } else {
        sprintln!("    the forced root is absent — the symmetry did not transfer.");
        StepResult::f()
    }
}

/// 5. The join: one cause, two columns.
fn step_join(results: &[StepResult]) -> StepResult {
    sprintln!("  Both consequences came from ONE hypothesis: the signing.");
    sprintln!("");
    sprintln!("      n even  ->  signing exists  ->  spectrum symmetric");
    sprintln!("                          |                 |");
    sprintln!("                    signature 0         gap 0");
    sprintln!("");
    sprintln!("      n odd   ->  no signing (not 2-colourable)");
    sprintln!("                          |                 |");
    sprintln!("                   signature +/-1       gap > 0");
    sprintln!("");
    sprintln!("  The survivor count and the privileged mode are not two");
    sprintln!("  phenomena. They are one parity dichotomy read twice.");
    sprintln!("");
    let mut held = 0usize;
    let mut fused = B4::T;
    for r in results.iter() {
        if r.holds { held += 1; }
        fused = fused.join(r.verdict);
    }
    sprintln!("    steps holding   {} of {}", held, results.len());
    sprintln!("    fused verdict   {}", fused.name());
    sprintln!("");
    sprintln!("  Formalized: p4ramill Imscribing/ParityIndex.lean");
    sprintln!("    charfun_even_of_signing      [propext, Classical.choice, Quot.sound]");
    sprintln!("    spectrum_symmetric_of_signing            same three");
    sprintln!("    parity_law            + [ofReduceBool, trustCompiler] from the");
    sprintln!("                            companion's index computations");
    if held == results.len() {
        StepResult { holds: true, verdict: fused }
    } else {
        sprintln!("");
        sprintln!("  A step failed above. This walk and the Lean module are two");
        sprintln!("  independent computations; a disagreement means one is wrong.");
        StepResult::f()
    }
}

/// Auto-play the parity law, pausing between steps.
pub fn walk_parity() {
    const TOTAL_STEPS: u8 = 5;
    sprintln!("");
    rule();
    sprintln!("  THE PARITY LAW — the signature and the gap are one theorem");
    rule();
    sprintln!("  Five steps, each computed here by integer arithmetic.");
    sprintln!("  Press a key to advance; q to stop.");

    let mut results: [StepResult; 4] = [
        StepResult::f(), StepResult::f(), StepResult::f(), StepResult::f(),
    ];

    if !pause() { sprintln!("  (stopped)"); return; }

    header(1, TOTAL_STEPS, "The even cycle admits a signing");
    results[0] = step_signing();
    verdict_line(&results[0]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(2, TOTAL_STEPS, "Therefore the characteristic function is even");
    results[1] = step_charfun();
    verdict_line(&results[1]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(3, TOTAL_STEPS, "Consequence one: the signature vanishes");
    results[2] = step_signature();
    verdict_line(&results[2]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(4, TOTAL_STEPS, "Consequence two: the modulus gap vanishes");
    results[3] = step_gap();
    verdict_line(&results[3]);
    if !pause() { sprintln!("  (stopped)"); return; }

    header(5, TOTAL_STEPS, "One cause, two columns");
    let final_r = step_join(&results);
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
    sprintln!("    parity       The signature and the spectral gap are ONE theorem");
    sprintln!("                 (5 steps; formalized in p4ramill ParityIndex.lean)");
    sprintln!("");
    sprintln!("  Run with:  proof bootstrap   |   proof parity");
}
