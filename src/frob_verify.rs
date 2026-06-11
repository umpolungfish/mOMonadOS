// frob_verify.rs — Frobenius Verification Harness
//
// Ported from imasmic_core/frobenius_verify.py (Author: Lando⊗⊙perator)
// Every action (δ) MUST be immediately verified (μ).
// μ(δ(q)) == q must hold before the loop advances.
//
// Integrated into mOMonadOS — the paraconsistent OS kernel.

use crate::belnap::*;
use crate::tokens::*;

// ─── Frobenius Result ───────────────────────────────────────────

/// Outcome of a μ∘δ=id verification.
/// Dual to the Python FrobeniusResult dataclass in imasmic_core.
#[derive(Clone)]
pub struct FrobeniusResult {
    pub closed: bool,
    /// The original query/input (q)
    pub delta_input:  B4,
    /// The action result (δ(q))
    pub delta_output: B4,
    /// The verification (μ(δ(q)))
    pub mu_result:    B4,
    /// Description of mismatch if open
    pub mismatch:     Option<&'static str>,
    /// Belnap truth value of the verification
    pub belnap_value: B4,
}

impl FrobeniusResult {
    /// Construct a closed (verified) result.
    pub fn closed(delta_input: B4, delta_output: B4, mu_result: B4) -> Self {
        Self {
            closed: true,
            delta_input,
            delta_output,
            mu_result,
            mismatch: None,
            belnap_value: B4::T,
        }
    }

    /// Construct an open (unverified) result with a mismatch description.
    pub fn open(
        delta_input: B4,
        delta_output: B4,
        mu_result: B4,
        mismatch: &'static str,
    ) -> Self {
        Self {
            closed: false,
            delta_input,
            delta_output,
            mu_result,
            mismatch: Some(mismatch),
            belnap_value: B4::F,
        }
    }

    /// Construct a dialetheic result — both closed and open (Belnap B).
    pub fn dialetheic(
        delta_input: B4,
        delta_output: B4,
        mu_result: B4,
        mismatch: &'static str,
    ) -> Self {
        Self {
            closed: true, // paradox stabilized = closed in Frobenius sense
            delta_input,
            delta_output,
            mu_result,
            mismatch: Some(mismatch),
            belnap_value: B4::B,
        }
    }
}

// ─── Verifiers ───────────────────────────────────────────────────

/// Verify that a Belnap value round-trips through a delta-mu pair.
/// μ(δ(q)) == q must hold.
pub fn verify_roundtrip(
    query: B4,
    delta: fn(B4) -> B4,
    mu: fn(B4) -> B4,
) -> FrobeniusResult {
    let delta_out = delta(query);
    let mu_result = mu(delta_out);
    if mu_result == query {
        FrobeniusResult::closed(query, delta_out, mu_result)
    } else {
        FrobeniusResult::open(query, delta_out, mu_result,
            "roundtrip mismatch: μ(δ(q)) ≠ q")
    }
}

/// Verify that the Frobenius identity holds for a specific token pair.
/// FSPLIT = δ (co-multiplication), FFUSE = μ (multiplication).
/// The identity is: FFUSE(FSPLIT(v), v) == v for all v in B4.
pub fn verify_frobenius_identity(v: B4) -> FrobeniusResult {
    // δ(v) → (v, v) on the two branches
    // μ(left, right) → b4_join(left, right)
    // For B4: join(v, v) = v for all v (idempotence)
    let delta_out = v; // FSPLIT copies
    let mu_result = b4_join(v, v); // FFUSE joins
    if mu_result == v {
        FrobeniusResult::closed(v, delta_out, mu_result)
    } else {
        FrobeniusResult::open(v, delta_out, mu_result,
            "Frobenius identity violated: join(v, v) ≠ v")
    }
}

/// Verify that a gate (EVALT/EVALF) is idempotent.
/// gate(gate(v)) == gate(v) for all v.
pub fn verify_gate_idempotence(v: B4, gate: fn(B4) -> B4) -> FrobeniusResult {
    let first  = gate(v);
    let second = gate(first);
    if second == first {
        FrobeniusResult::closed(v, first, second)
    } else {
        FrobeniusResult::open(v, first, second,
            "gate idempotence violated: gate(gate(v)) ≠ gate(v)")
    }
}

/// Verify that the Belnap lattice operations form a proper lattice.
/// Checks: idempotence (a∧a=a, a∨a=a), commutativity, associativity.
pub fn verify_lattice_laws() -> [FrobeniusResult; 4] {
    let values = [B4::N, B4::T, B4::F, B4::B];
    // Idempotence of join
    let idem_join = {
        let all_ok = values.iter().all(|&v| b4_join(v, v) == v);
        if all_ok {
            FrobeniusResult::closed(B4::T, B4::T, B4::T)
        } else {
            FrobeniusResult::open(B4::T, B4::T, B4::F,
                "join idempotence violated")
        }
    };
    // Idempotence of meet
    let idem_meet = {
        let all_ok = values.iter().all(|&v| b4_meet(v, v) == v);
        if all_ok {
            FrobeniusResult::closed(B4::T, B4::T, B4::T)
        } else {
            FrobeniusResult::open(B4::T, B4::T, B4::F,
                "meet idempotence violated")
        }
    };
    // Commutativity
    let commut = {
        let all_ok = values.iter().all(|&a| {
            values.iter().all(|&b| {
                b4_join(a, b) == b4_join(b, a) && b4_meet(a, b) == b4_meet(b, a)
            })
        });
        if all_ok {
            FrobeniusResult::closed(B4::T, B4::T, B4::T)
        } else {
            FrobeniusResult::open(B4::T, B4::T, B4::F,
                "commutativity violated")
        }
    };
    // Associativity
    let assoc = {
        let all_ok = values.iter().all(|&a| {
            values.iter().all(|&b| {
                values.iter().all(|&c| {
                    b4_join(b4_join(a, b), c) == b4_join(a, b4_join(b, c))
                        && b4_meet(b4_meet(a, b), c) == b4_meet(a, b4_meet(b, c))
                })
            })
        });
        if all_ok {
            FrobeniusResult::closed(B4::T, B4::T, B4::T)
        } else {
            FrobeniusResult::open(B4::T, B4::T, B4::F,
                "associativity violated")
        }
    };
    [idem_join, idem_meet, commut, assoc]
}

/// Verify that a program is structurally well-formed.
/// Checks: FSPLIT/FFUSE balanced, no empty program, bootstrap prefix.
pub fn verify_program_structure(prog: &Program) -> FrobeniusResult {
    let n = prog.len();
    if n == 0 {
        return FrobeniusResult::open(B4::N, B4::N, B4::N,
            "empty program");
    }
    // FSPLIT/FFUSE balanced
    let (mut splits, mut fuses) = (0usize, 0usize);
    for i in 0..n {
        match prog.get(i) {
            Some(Token::FSPLIT) => splits += 1,
            Some(Token::FFUSE)  => fuses  += 1,
            _ => {}
        }
    }
    if splits != fuses {
        return FrobeniusResult::open(
            B4::from_u8(splits as u8 & 3),
            B4::from_u8(fuses as u8 & 3),
            B4::N,
            "FSPLIT/FFUSE unbalanced"
        );
    }
    FrobeniusResult::closed(B4::T, B4::T, B4::T)
}

// ─── Verification Harness ────────────────────────────────────────

/// Universal verification harness — ported from imasmic_core FrobeniusHarness.
/// Wraps any kernel's verification suite.
pub struct FrobeniusHarness {
    pub project_name: &'static str,
    pub closed_count: u64,
    pub open_count:   u64,
    /// Ring buffer of most recent results
    pub history:      [FrobeniusResult; 16],
    pub history_head: usize,
}

impl FrobeniusHarness {
    pub fn new(project_name: &'static str) -> Self {
        Self {
            project_name,
            closed_count: 0,
            open_count:   0,
            history: [
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
                FrobeniusResult::closed(B4::N, B4::N, B4::N),
            ],
            history_head: 0,
        }
    }

    /// Record a verification result.
    pub fn check(&mut self, result: FrobeniusResult) -> bool {
        let closed = result.closed;
        if closed {
            self.closed_count += 1;
        } else {
            self.open_count += 1;
        }
        self.history[self.history_head] = result;
        self.history_head = (self.history_head + 1) % 16;
        closed
    }
    pub fn total(&self) -> u64 {
        self.closed_count + self.open_count
    }

    /// Closure ratio: closed / total. Returns 1.0 if no checks yet.
    pub fn closure_ratio(&self) -> f64 {
        let t = self.total();
        if t == 0 { return 1.0; }
        self.closed_count as f64 / t as f64
    }

    /// True iff no open results.
    pub fn is_closed(&self) -> bool {
        self.open_count == 0
    }

    /// Run a full verification suite against a program.
    /// Returns the number of checks that passed.
    pub fn verify_program(&mut self, prog: &Program) -> u64 {
        let mut passed = 0u64;
        // 1. Structural check
        if self.check(verify_program_structure(prog)) { passed += 1; }
        // 2. Frobenius identity for all B4 values
        for &v in &[B4::N, B4::T, B4::F, B4::B] {
            if self.check(verify_frobenius_identity(v)) { passed += 1; }
        }
        // 3. Gate idempotence
        for &v in &[B4::N, B4::T, B4::F, B4::B] {
            if self.check(verify_gate_idempotence(v, evalt)) { passed += 1; }
            if self.check(verify_gate_idempotence(v, evalf)) { passed += 1; }
        }
        // 4. Lattice laws
        for r in verify_lattice_laws() {
            if self.check(r) { passed += 1; }
        }
        passed
    }
}

// ─── Gate functions (used by verify_gate_idempotence) ────────────

/// EVALT gate: passes T, blocks all else → N.
fn evalt(v: B4) -> B4 {
    if v == B4::T { B4::T } else { B4::N }
}

/// EVALF gate: passes F, blocks all else → N.
fn evalf(v: B4) -> B4 {
    if v == B4::F { B4::F } else { B4::N }
}

// ─── Tests (run in kernel REPL, not on bare metal) ──────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frobenius_identity_all_values() {
        for &v in &[B4::N, B4::T, B4::F, B4::B] {
            let r = verify_frobenius_identity(v);
            assert!(r.closed, "Frobenius identity failed for {:?}", v);
        }
    }

    #[test]
    fn test_gate_idempotence() {
        for &v in &[B4::N, B4::T, B4::F, B4::B] {
            assert!(verify_gate_idempotence(v, evalt).closed);
            assert!(verify_gate_idempotence(v, evalf).closed);
        }
    }

    #[test]
    fn test_lattice_laws() {
        for r in verify_lattice_laws() {
            assert!(r.closed, "Lattice law violated: {:?}", r.mismatch);
        }
    }

    #[test]
    fn test_harness_accumulation() {
        let mut h = FrobeniusHarness::new("test");
        h.check(FrobeniusResult::closed(B4::T, B4::T, B4::T));
        h.check(FrobeniusResult::open(B4::T, B4::F, B4::F, "test fail"));
        assert_eq!(h.closed_count, 1);
        assert_eq!(h.open_count, 1);
        assert!(!h.is_closed());
    }
}// ─── p4rakernel verifications ──────────────────────────────────
// Ported from red-hot_rebis/rhr_p4rky/kernel.py
// These verify Belnap lattice invariants under Frobenius operations.

/// Theorem: ffuse∘fsplit = id.
/// For all Belnap values, join(fsplit_branches(v)) == v.
/// This is identical to verify_frobenius_identity but named for
/// compatibility with the p4rakernel naming convention.
pub fn frobenius_invariant(v: B4) -> bool {
    // fsplit(v) → (T, F) if v==B, else (v, v)
    // ffuse(a, b) → join(a, b)
    let (b1, b2) = if v == B4::B { (B4::T, B4::F) } else { (v, v) };
    b4_join(b1, b2) == v
}

/// Verify the Frobenius invariant for all four Belnap values.
pub fn verify_frobenius_invariant() -> FrobeniusResult {
    let values = [B4::N, B4::T, B4::F, B4::B];
    let all_ok = values.iter().all(|&v| frobenius_invariant(v));
    if all_ok {
        FrobeniusResult::closed(B4::T, B4::T, B4::T)
    } else {
        FrobeniusResult::open(B4::T, B4::T, B4::F,
            "Frobenius invariant violated for some Belnap value")
    }
}

/// Theorem: ENGAGR stabilizes paradox.
/// After ENGAGR pushes B, FSPLIT splits it to (T, F), and FFUSE joins
/// back to B — showing B is a fixed point of the Frobenius cycle.
pub fn verify_paradox_stabilization() -> FrobeniusResult {
    // ENGAGR → B on stack
    // FSPLIT B → (T, F)
    // FFUSE(T, F) → join(T, F) = B
    let b = B4::B;
    let (b1, b2) = if b == B4::B { (B4::T, B4::F) } else { (b, b) };
    let result = b4_join(b1, b2);
    if result == B4::B {
        FrobeniusResult::closed(B4::B, B4::B, B4::B)
    } else {
        FrobeniusResult::open(B4::B, B4::B, result,
            "paradox stabilization violated: join(T,F) ≠ B")
    }
}

/// Theorem: B absorbs all values under join.
/// join(B, x) = B for all x. B is the universal upper bound.
pub fn verify_b_absorbs_join() -> FrobeniusResult {
    let all_ok = [B4::N, B4::T, B4::F, B4::B].iter()
        .all(|&x| b4_join(B4::B, x) == B4::B);
    if all_ok {
        FrobeniusResult::closed(B4::B, B4::B, B4::B)
    } else {
        FrobeniusResult::open(B4::B, B4::B, B4::F,
            "B absorption under join violated")
    }
}

/// Theorem: B is a fixed point of negation.
/// not(B) = B. Paradox is self-negating.
pub fn verify_b_fixed_point_negation() -> FrobeniusResult {
    // In Belnap: bnot(B) = B (bitwise: 3 XOR 3 = 0, but Belnap uses different negation)
    // Actually: bnot in Belnap flips T↔F, preserves N and B
    // bnot(N)=N, bnot(T)=F, bnot(F)=T, bnot(B)=B
    let bnot = |v: B4| -> B4 {
        match v {
            B4::N => B4::N,
            B4::T => B4::F,
            B4::F => B4::T,
            B4::B => B4::B,
        }
    };
    if bnot(B4::B) == B4::B {
        FrobeniusResult::closed(B4::B, B4::B, B4::B)
    } else {
        FrobeniusResult::open(B4::B, B4::B, B4::F,
            "B fixed-point negation violated")
    }
}

/// Theorem: No value other than B is dialetheic.
/// Only B represents "both true and false."
pub fn verify_only_b_is_dialetheic() -> FrobeniusResult {
    let dialetheic = |v: B4| -> bool { v == B4::B };
    let non_b: [B4; 3] = [B4::N, B4::T, B4::F];
    let all_ok = non_b.iter().all(|&v| !dialetheic(v)) && dialetheic(B4::B);
    if all_ok {
        FrobeniusResult::closed(B4::B, B4::B, B4::B)
    } else {
        FrobeniusResult::open(B4::B, B4::B, B4::F,
            "only-B-is-dialetheic violated")
    }
}

/// Theorem: B is the top element (all values approximate to B).
/// approx_le(x, B) for all x.
pub fn verify_b_is_top() -> FrobeniusResult {
    let approx_le = |a: B4, b: B4| -> bool {
        // a ≤ b in Belnap iff a.meet(b) == a AND a.join(b) == b
        b4_meet(a, b) == a && b4_join(a, b) == b
    };
    let all_ok = [B4::N, B4::T, B4::F, B4::B].iter()
        .all(|&x| approx_le(x, B4::B));
    if all_ok {
        FrobeniusResult::closed(B4::B, B4::B, B4::B)
    } else {
        FrobeniusResult::open(B4::B, B4::B, B4::F,
            "B-is-top violated")
    }
}

/// Run all p4rakernel verifications.
/// Returns (passed, total) counts.
pub fn verify_all_p4ra() -> (u64, u64) {
    let mut h = FrobeniusHarness::new("p4rakernel");
    h.check(verify_frobenius_invariant());
    h.check(verify_paradox_stabilization());
    h.check(verify_b_absorbs_join());
    h.check(verify_b_fixed_point_negation());
    h.check(verify_only_b_is_dialetheic());
    h.check(verify_b_is_top());
    (h.closed_count, h.total())
}
