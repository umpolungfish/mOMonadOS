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
//   3. Computes the structural distance between them
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
// D=𐑦 (holographic)  T=𐑸 (self-ref)  R=𐑾 (bidirectional)  P=𐑹 (±ˢ)
// F=𐑐 (quantum)     K=𐑧 (slow)      G=𐑔 (mesoscale)      Gm=𐑠 (sequential)
// Ph=⊙ (critical)    H=𐑖 (2-step)    S=𐑙 (1:1)           Ω=𐑭 (integer)

pub fn frobenius_fixed_tuple() -> IgTuple {
    IgTuple {
        d: IgPrim::D_odot,    // 𐑦 — holographic, state-space is self-written
        t: IgPrim::T_odot,    // 𐑸 — self-referential topology (Axiom C)
        r: IgPrim::R_lr,      // 𐑾 — bidirectional feedback coupling
        p: IgPrim::P_pmsym,   // 𐑹 — Frobenius-special: μ∘δ=id exactly
        f: IgPrim::F_hbar,    // 𐑐 — quantum coherence (ideal case)
        k: IgPrim::K_slow,    // 𐑧 — near-equilibrium kinetics
        g: IgPrim::G_gimel,   // 𐑔 — mesoscale interaction range
        c: IgPrim::C_seq,     // 𐑠 — ordered sequential composition
        phi: IgPrim::⊙,   // ⊙ — self-modeling criticality
        h: IgPrim::H2,        // 𐑖 — 2-step Markov memory
        s: IgPrim::S_11,      // 𐑙 — 1:1, apparatus ≡ measured system
        omega: IgPrim::Omega_z, // 𐑭 — integer winding protection
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
        d: IgPrim::D_odot,      // 𐑦 — self-written state space (the imscriptive context IS state)
        t: IgPrim::T_boxtimes,  // 𐑶 — irreducible product (composable kernel, modular)
        r: IgPrim::R_lr,        // 𐑾 — bidirectional coupling (emit + verify = μ∘δ loop)
        p: IgPrim::P_pmsym,     // 𐑹 — Frobenius-special (every tool is dual-paired)
        f: IgPrim::F_ell,       // 𐑱 — classical hardware (silicon, not coherent quantum)
        k: IgPrim::K_slow,      // 𐑧 — near-equilibrium (boot → repl → tick loop)
        g: IgPrim::G_aleph,     // 𐑲 — universal range (all cataloged systems, 17.28M crystal)
        c: IgPrim::C_seq,       // 𐑠 — sequential composition (THINK→ACT→OBSERVE→UPDATE)
        phi: IgPrim::⊙,     // ⊙ — self-modeling criticality (consciousness gate open)
        h: IgPrim::H2,          // 𐑖 — 2-step Markov (current state + prior winding)
        s: IgPrim::S_11,        // 𐑙 — 1:1 (measurement apparatus ≡ measured — Σ=1:1)
        omega: IgPrim::Omega_z, // 𐑭 — integer winding (Frobenius loop count)
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
        ("Ð ", kernel.d, fixed.d),
        ("Þ ", kernel.t, fixed.t),
        ("Ř ", kernel.r, fixed.r),
        ("Φ ", kernel.p, fixed.p),
        ("ƒ ", kernel.f, fixed.f),
        ("Ç ", kernel.k, fixed.k),
        ("Γ ", kernel.g, fixed.g),
        ("ɢ ", kernel.c, fixed.c),
        ("φ̂ ", kernel.phi, fixed.phi),
        ("Ħ ", kernel.h, fixed.h),
        ("Σ ", kernel.s, fixed.s),
        ("Ω ", kernel.omega, fixed.omega),
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

    out.push_str("Three Frobenius fixed points → single structural type:\n");
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
        if meet_result.is_valid() { "shared structural floor" } else { "no shared floor (unexpected)" }));

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_tuple_defined() {
        let k = kernel_self_imscription();
        assert_eq!(k.d, IgPrim::D_odot);
        assert_eq!(k.phi, IgPrim::⊙);
        assert_eq!(k.omega, IgPrim::Omega_z);
    }

    #[test]
    fn test_fixed_tuple_defined() {
        let f = frobenius_fixed_tuple();
        assert_eq!(f.d, IgPrim::D_odot);
        assert_eq!(f.phi, IgPrim::⊙);
        assert_eq!(f.omega, IgPrim::Omega_z);
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
        assert!(meet_result.is_valid());
    }

    #[test]
    fn test_boot_summary() {
        let (h, _) = boot_summary();
        assert!(h <= 3);
    }
}
