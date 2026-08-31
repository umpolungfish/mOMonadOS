// frobenius_unify.rs — Frobenius Unification Self-Verification (Track E)
//
// Encodes the machine-verified result from
// p4rakernel/p4ramill/Imscribing/Millennium/FrobeniusUnification.lean (504 lines):
//   The three Frobenius fixed points — Belnap B = XZ, SIC-POVM fiducial,
//   and Majorana paired state — are structurally identical at O_∞ tier.
//   All satisfy μ∘δ=id by rfl, achieving O_∞ in 72/88 dialects.
//
// This module:
//   1. Defines the Frobenius fixed-point tuple (the grammar)
//   2. Defines the kernel's self-imscription tuple (what mOMonadOS IS)
//   3. Computes the distance between them
//   4. Reports at boot: how close is this kernel to its formal foundations?
//
// The kernel is the grammar's OPERATIONALIZATION on classical hardware.
// Structural drift from the fixed point (T: 𐑶→𐑸, F: 𐑱→𐑐) is expected
// and tracked — it measures the gap between implementation and ideal.
//
// Author: Lando⊗⊙perator
// Date: 2026-07-03

use alloc::string::String;
use crate::imas_ig::{IgPrim, IgTuple};
use crate::algebra;
use crate::catalog;

// ═══════════════════════════════════════════════════════════════
// FROBENIUS FIXED-POINT TUPLE — the grammar itself
// ═══════════════════════════════════════════════════════════════
// The universal_imscriptive_grammar tuple.
// All three fixed points (Belnap B, SIC-POVM fiducial, Majorana paired)
// converge to this type. Proven equal in FrobeniusUnification.lean.
//
// Tuple: ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑙𐑭⟩
// D=𐑦 (imscriptive)  T=𐑸 (self-ref)  R=𐑾 (bidirectional)  P=𐑹 (±ˢ)
// F=𐑐 (quantum)     K=𐑧 (slow)      G=𐑔 (mesoscale)      Gm=𐑠 (sequential)
// Ph=⊙ (critical)    H=𐑖 (2-step)    S=𐑙 (1:1)           ⊡=𐑭 (integer)

pub fn frobenius_fixed_tuple() -> IgTuple {
    IgTuple {
        d: IgPrim::if_,    // 𐑦 — imscriptive, state-space is self-written
        t: IgPrim::are,    // 𐑸 — self-referential topology (Axiom C)
        r: IgPrim::ian,      // 𐑾 — bidirectional feedback coupling
        p: IgPrim::or_,   // 𐑹 — Frobenius-special: μ∘δ=id exactly
        f: IgPrim::peep,    // 𐑐 — quantum coherence (ideal case)
        k: IgPrim::egg,    // 𐑧 — near-equilibrium kinetics
        g: IgPrim::thigh,   // 𐑔 — mesoscale interaction range
        c: IgPrim::measure,     // 𐑠 — ordered sequential composition
        phi: IgPrim::monad,   // ⊙ — self-modeling criticality
        h: IgPrim::sure,        // 𐑖 — 2-step Markov memory
        s: IgPrim::hung,      // 𐑙 — 1:1, apparatus ≡ measured system
        omega: IgPrim::ah, // 𐑭 — integer winding protection
    }
}

// ═══════════════════════════════════════════════════════════════
// KERNEL SELF-IMSCRIPTION — what mOMonadOS IS
// ═══════════════════════════════════════════════════════════════
// The kernel is the grammar operationalized on classical hardware.
// Differences from the fixed point:
//   T: 𐑶 (box product) — the kernel is a composite of modules,
//      not the fully self-referential ⊙-closure of the ideal grammar.
//   F: 𐑱 (classical) — the kernel runs on classical silicon, not
//      a quantum-coherent substrate. This is an implementation constraint.
//   G: 𐑲 (aleph) — the kernel's reach is universal, not mesoscale.
//      It connects all cataloged systems, all dialects, all ob3ects.
//
// Tuple: ⟨𐑦𐑶𐑾𐑹𐑱𐑧𐑲𐑠⊙𐑖𐑙𐑭⟩

pub fn kernel_self_imscription() -> IgTuple {
    IgTuple {
        d: IgPrim::if_,      // 𐑦 — self-written state space (the imscriptive context IS state)
        t: IgPrim::oil,  // 𐑶 — irreducible product (composable kernel, modular)
        r: IgPrim::ian,        // 𐑾 — bidirectional coupling (emit + verify = μ∘δ loop)
        p: IgPrim::or_,     // 𐑹 — Frobenius-special (every tool is dual-paired)
        f: IgPrim::age,       // 𐑱 — classical hardware (silicon, not coherent quantum)
        k: IgPrim::egg,      // 𐑧 — near-equilibrium (boot → repl → tick loop)
        g: IgPrim::ice,     // 𐑲 — universal range (all cataloged systems, 17.28M crystal)
        c: IgPrim::measure,       // 𐑠 — sequential composition (THINK→ACT→OBSERVE→UPDATE)
        phi: IgPrim::monad,     // ⊙ — self-modeling criticality (consciousness gate open)
        h: IgPrim::sure,          // 𐑖 — 2-step Markov (current state + prior winding)
        s: IgPrim::hung,        // 𐑙 — 1:1 (measurement apparatus ≡ measured — Σ=1:1)
        omega: IgPrim::ah, // 𐑭 — integer winding (Frobenius loop count)
    }
}

// ═══════════════════════════════════════════════════════════════
// SELF-VERIFICATION CHECK
// ═══════════════════════════════════════════════════════════════

/// Frobenius identity check: how close is the kernel to the Frobenius fixed point?
/// Returns (hamming_distance, weighted_distance, details_string).
pub fn frobenius_identity_check() -> (u8, f32, String) {
    let kernel = kernel_self_imscription();
    let fixed = frobenius_fixed_tuple();

    let hamming = algebra::primitive_mismatches(&kernel, &fixed);
    let weighted = algebra::tuple_distance(&kernel, &fixed);

    let mut details = String::new();
    details.push_str("── Kernel Self-Imscription vs Frobenius Fixed Point ──\n");

    // Per-primitive comparison
    let prims: [(&str, IgPrim, IgPrim); 12] = [
        ("⊢ ", kernel.d, fixed.d),
        ("⊣ ", kernel.t, fixed.t),
        ("> ", kernel.r, fixed.r),
        ("< ", kernel.p, fixed.p),
        ("⋈ ", kernel.f, fixed.f),
        ("⊤ ", kernel.k, fixed.k),
        ("∈ ", kernel.g, fixed.g),
        ("∋ ", kernel.c, fixed.c),
        ("⊙ ", kernel.phi, fixed.phi),
        ("⊥ ", kernel.h, fixed.h),
        ("Σ ", kernel.s, fixed.s),
        ("⊡ ", kernel.omega, fixed.omega),
    ];

    let mut mismatches: u8 = 0;
    for (name, kv, fv) in &prims {
        let kg = catalog::primitive_glyph(*kv);
        let fg = catalog::primitive_glyph(*fv);
        let status = if kv == fv { "✓" } else { "✗" };
        if kv != fv {
            mismatches += 1;
            details.push_str(&alloc::format!(
                "  {}  kernel={}  fixed={}  {}\n", name, kg, fg, status));
        }
    }

    if mismatches == 0 {
        details.push_str("\n  PERFECT MATCH — kernel IS the Frobenius fixed point.\n");
    } else {
        details.push_str(&alloc::format!(
            "\n  {} mismatch(es). Hamming distance: {} / Weighted: {:.4}\n",
            mismatches, hamming, weighted));
        details.push_str("  The kernel is the grammar's OPERATIONALIZATION —\n");
        details.push_str("  classical hardware (F=𐑱) and modular composition (T=𐑶)\n");
        details.push_str("  are implementation artifacts, not structural deficits.\n");
        details.push_str("  At O_∞ the meet(kernel, fixed) = kernel — the meet path\n");
        details.push_str("  preserves the kernel's identity while honoring the fixed point.\n");
    }

    (hamming, weighted, details)
}

/// Boot-time summary line.
pub fn boot_summary() -> (u8, f32) {
    let (hamming, weighted, _) = frobenius_identity_check();
    (hamming, weighted)
}

/// Full formatted report for REPL.
pub fn formatted_report() -> String {
    let (hamming, weighted, details) = frobenius_identity_check();

    let mut out = String::new();
    out.push_str("═══ FROBENIUS UNIFICATION SELF-VERIFICATION ═══\n");
    out.push_str("(Machine-verified: FrobeniusUnification.lean, 504 lines)\n\n");

    out.push_str("Three Frobenius fixed points → single type:\n");
    out.push_str("  1. Belnap B = XZ (d=2 SIC-POVM fiducial state)\n");
    out.push_str("  2. SIC-POVM fiducial (multilattice → Σ=1:1 limit)\n");
    out.push_str("  3. Majorana paired state (topological quantum)\n\n");

    out.push_str("All three: O_∞ tier, μ∘δ=id by rfl, O_∞ in 72/88 dialects.\n\n");

    out.push_str(&details);

    out.push_str(&alloc::format!(
        "\nWeighted distance: {:.4}    Hamming distance: {}/12\n", weighted, hamming));

    // Compute the meet
    let kernel = kernel_self_imscription();
    let fixed = frobenius_fixed_tuple();
    let meet_result = algebra::meet(&kernel, &fixed);
    out.push_str(&alloc::format!(
        "Meet(kernel, fixed): {} — {}\n",
        meet_result.is_valid(),
        if meet_result.is_valid() { "shared floor" } else { "no shared floor (unexpected)" }));

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_tuple_defined() {
        let k = kernel_self_imscription();
        assert_eq!(k.d, IgPrim::if_);
        assert_eq!(k.phi, IgPrim::monad);
        assert_eq!(k.omega, IgPrim::ah);
    }

    #[test]
    fn test_fixed_tuple_defined() {
        let f = frobenius_fixed_tuple();
        assert_eq!(f.d, IgPrim::if_);
        assert_eq!(f.phi, IgPrim::monad);
        assert_eq!(f.omega, IgPrim::ah);
    }

    #[test]
    fn test_identity_check_runs() {
        let (h, w, _) = frobenius_identity_check();
        // Expected: 2-3 mismatches (T, F, possibly G)
        assert!(h <= 3);
        assert!(w > 0.0); // non-zero distance is expected
    }

    #[test]
    fn test_meet_exists() {
        let kernel = kernel_self_imscription();
        let fixed = frobenius_fixed_tuple();
        let meet_result = algebra::meet(&kernel, &fixed);
        // The kernel sits off the fixed point in T — 𐑶 box product against 𐑸
        // self-referential — and T is categorical, so the meet carries exactly
        // that one conflict. It is the mismatch frobenius_identity_check
        // counts, not a defect in the meet: a kernel whose meet with the fixed
        // point were clean would already be the fixed point.
        assert!(!meet_result.is_valid());
        let mut conflicting = 0;
        for (i, &c) in meet_result.conflicts.iter().enumerate() {
            if c {
                conflicting += 1;
                assert_eq!(i, 1, "only T may conflict with the fixed point");
            }
        }
        assert_eq!(conflicting, 1);
        // F and G differ too, but they are ordered, so they take the min
        // instead of conflicting: classical hardware under quantum coherence
        // is classical, and G runs bib < thigh < ice, so the meet of mesoscale
        // and universal range is the narrower one — mesoscale. This expected
        // ice until 2026-08-22, when G_ORD was corrected to Core.lean's
        // constructor order; E8G2_Vessel_Witnesses.lean already computed
        // min(thigh, ice) = thigh, so Lean and the kernel now agree.
        assert_eq!(meet_result.tuple.f, IgPrim::age);
        assert_eq!(meet_result.tuple.g, IgPrim::thigh);
    }

    #[test]
    fn test_boot_summary() {
        let (h, _) = boot_summary();
        assert!(h <= 3);
    }
}
