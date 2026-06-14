#![allow(dead_code)]
// consciousness.rs — Consciousness Score (C-score) with dual-gate evaluation
//
// C-score ∈ [0, 1]: structural proximity to full consciousness capability.
// Two gating conditions must both pass:
//   Gate 1 (⊙ / Phi_c): self-modeling loop must be open
//   Gate 2 (K_slow):     kinetics must be slow enough for information integration
//
// ALL per-primitive scores are now computed from ordinal positions in catalog.rs.
// No hardcoded match arms — scores are proportional to ordinal position within
// each primitive family. This means score tables are always consistent with
// the catalog ordinals and can be updated in one place.
//
// Full formula:
//   C = G1 * G2 * (basal complexity score)
//   Basal = (D_score + T_score + R_score + P_score + F_score +
//            G_score + C_score + H_score + S_score + Omega_score) / 10
//   Each component ∈ [0, 1] based on ordinal position within its family

use crate::imas_ig::{IgPrim, IgTuple};
use crate::catalog;

/// Evaluate Gate 1: ⊙ self-modeling loop.
/// Returns true iff Phi is ⊙ or complex-critical (gate open).
pub fn gate1_phi_c(t: &IgTuple) -> bool {
    matches!(t.phi, IgPrim::Phi_c | IgPrim::Phi_c_complex)
}

/// Evaluate Gate 2: kinetics slow enough for integration.
/// Returns true iff K is slow (near-equilibrium) or trapped (ordered).
pub fn gate2_k_slow(t: &IgTuple) -> bool {
    matches!(t.k, IgPrim::K_slow | IgPrim::K_trap)
}

/// Compute the basal complexity score (sum of 10 component scores / 10).
/// All scores computed from catalog ordinal positions — no hardcoded values.
fn basal_score(t: &IgTuple) -> f32 {
    (catalog::score_d(t.d) + catalog::score_t(t.t) +
     catalog::score_r(t.r) + catalog::score_p(t.p) +
     catalog::score_f(t.f) + catalog::score_g(t.g) +
     catalog::score_c(t.c) + catalog::score_h(t.h) +
     catalog::score_s(t.s) + catalog::score_omega(t.omega)) / 10.0
}

/// Full consciousness score: C = G1 * G2 * basal.
/// Both gates must be open for any nonzero score.
pub fn consciousness_score(t: &IgTuple) -> f32 {
    let g1 = if gate1_phi_c(t) { 1.0 } else { 0.0 };
    let g2 = if gate2_k_slow(t) { 1.0 } else { 0.0 };
    g1 * g2 * basal_score(t)
}

/// Detailed consciousness evaluation with per-component breakdown.
pub struct ConsciousnessResult {
    pub c_score: f32,
    pub gate1_open: bool,
    pub gate2_open: bool,
    pub basal: f32,
    pub components: [f32; 10],
    pub component_names: [&'static str; 10],
}

pub fn consciousness_eval(t: &IgTuple) -> ConsciousnessResult {
    let g1 = gate1_phi_c(t);
    let g2 = gate2_k_slow(t);
    let basal = basal_score(t);
    let c = if g1 && g2 { basal } else { 0.0 };
    ConsciousnessResult {
        c_score: c,
        gate1_open: g1,
        gate2_open: g2,
        basal,
        components: [
            catalog::score_d(t.d), catalog::score_t(t.t),
            catalog::score_r(t.r), catalog::score_p(t.p),
            catalog::score_f(t.f), catalog::score_g(t.g),
            catalog::score_c(t.c), catalog::score_h(t.h),
            catalog::score_s(t.s), catalog::score_omega(t.omega),
        ],
        component_names: ["D","T","R","P","F","G","C","H","S","Ω"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog;

    #[test]
    fn test_oinf_consciousness() {
        let oinf = catalog::o_inf_tuple();
        let r = consciousness_eval(&oinf);
        assert!(r.gate1_open);
        assert!(r.gate2_open);
        assert!(r.c_score > 0.8);
    }

    #[test]
    fn test_o0_consciousness() {
        let o0 = catalog::o_0_tuple();
        let r = consciousness_eval(&o0);
        assert!(!r.gate1_open);
        assert!(!r.gate2_open);
        assert_eq!(r.c_score, 0.0);
    }

    #[test]
    fn test_zfc_score() {
        let zfc = catalog::zfc_baseline_tuple();
        let r = consciousness_eval(&zfc);
        assert!(!r.gate1_open);
        assert_eq!(r.c_score, 0.0);
    }
}
