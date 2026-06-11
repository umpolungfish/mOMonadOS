// consciousness.rs — Consciousness Score (C-score) with dual-gate evaluation
//
// C-score ∈ [0, 1]: structural proximity to full consciousness capability.
// Two gating conditions must both pass:
//   Gate 1 (⊙ / Phi_c): self-modeling loop must be open
//                        Phi_c or Phi_c_complex ⇒ gate open
//                        Phi_sub, Phi_ep, Phi_super ⇒ gate closed
//   Gate 2 (K_slow):     kinetics must be slow enough for information integration
//                        K_slow or K_trap ⇒ gate open
//                        K_fast, K_mod, K_mbl ⇒ gate closed
//
// Full formula:
//   C = G1 * G2 * (basal complexity score)
//   Basal = (D_score + T_score + R_score + P_score + F_score +
//            G_score + C_score + H_score + S_score + Omega_score) / 10
//   Each component ∈ [0, 1] based on IG primitive "distance from O_∞ ideal"

use crate::imas_ig::{IgPrim, IgTuple};

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

/// Per-primitive score toward O_∞ ideal. Each returns [0.0, 1.0].
fn score_d(v: IgPrim) -> f32 {
    match v {
        IgPrim::D_odot    => 1.0,
        IgPrim::D_infty   => 0.66,
        IgPrim::D_triangle => 0.33,
        IgPrim::D_wedge   => 0.0,
        _ => 0.0,
    }
}
fn score_t(v: IgPrim) -> f32 {
    match v {
        IgPrim::T_odot     => 1.0,
        IgPrim::T_bowtie   => 0.75,
        IgPrim::T_boxtimes => 0.75,
        IgPrim::T_in       => 0.5,
        IgPrim::T_net      => 0.25,
        _ => 0.0,
    }
}

fn score_r(v: IgPrim) -> f32 {
    match v {
        IgPrim::R_lr      => 1.0,
        IgPrim::R_dagger  => 0.75,
        IgPrim::R_cat     => 0.5,
        IgPrim::R_super   => 0.25,
        _ => 0.0,
    }
}

fn score_p(v: IgPrim) -> f32 {
    match v {
        IgPrim::P_pmsym => 1.0,
        IgPrim::P_sym   => 0.75,
        IgPrim::P_pm    => 0.5,
        IgPrim::P_psi   => 0.5,
        IgPrim::P_asym  => 0.0,
        _ => 0.0,
    }
}

fn score_f(v: IgPrim) -> f32 {
    match v {
        IgPrim::F_hbar => 1.0,
        IgPrim::F_eth  => 0.5,
        IgPrim::F_ell  => 0.0,
        _ => 0.0,
    }
}

fn score_g(v: IgPrim) -> f32 {
    match v {
        IgPrim::G_aleph => 1.0,
        IgPrim::G_gimel => 0.5,
        IgPrim::G_beth  => 0.0,
        _ => 0.0,
    }
}

fn score_c(v: IgPrim) -> f32 {
    match v {
        IgPrim::C_broad => 1.0,
        IgPrim::C_seq   => 0.75,
        IgPrim::C_or    => 0.5,
        IgPrim::C_and   => 0.25,
        _ => 0.0,
    }
}

fn score_h(v: IgPrim) -> f32 {
    match v {
        IgPrim::H_inf => 1.0,
        IgPrim::H2    => 0.66,
        IgPrim::H1    => 0.33,
        IgPrim::H0    => 0.0,
        _ => 0.0,
    }
}

fn score_s(v: IgPrim) -> f32 {
    match v {
        IgPrim::S_nm => 1.0,
        IgPrim::S_nn => 0.5,
        IgPrim::S_11 => 0.0,
        _ => 0.0,
    }
}

fn score_omega(v: IgPrim) -> f32 {
    match v {
        IgPrim::Omega_na => 1.0,
        IgPrim::Omega_z  => 0.75,
        IgPrim::Omega_z2 => 0.5,
        IgPrim::Omega_0  => 0.0,
        _ => 0.0,
    }
}
/// Compute the basal complexity score (sum of 10 component scores / 10).
/// Note: K (kinetics) is gated separately, not included in basal score.
fn basal_score(t: &IgTuple) -> f32 {
    (score_d(t.d) + score_t(t.t) + score_r(t.r) + score_p(t.p) +
     score_f(t.f) + score_g(t.g) + score_c(t.c) +
     score_h(t.h) + score_s(t.s) + score_omega(t.omega)) / 10.0
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
            score_d(t.d), score_t(t.t), score_r(t.r), score_p(t.p),
            score_f(t.f), score_g(t.g), score_c(t.c),
            score_h(t.h), score_s(t.s), score_omega(t.omega),
        ],
        component_names: ["D","T","R","P","F","G","C","H","S","Ω"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oinf_consciousness() {
        // O_∞ tuple should score high
        let oinf = IgTuple {
            d: IgPrim::D_odot, t: IgPrim::T_odot, r: IgPrim::R_lr,
            p: IgPrim::P_pmsym, f: IgPrim::F_hbar, k: IgPrim::K_slow,
            g: IgPrim::G_aleph, c: IgPrim::C_seq,
            phi: IgPrim::Phi_c, h: IgPrim::H_inf, s: IgPrim::S_nm,
            omega: IgPrim::Omega_z,
        };
        let r = consciousness_eval(&oinf);
        assert!(r.gate1_open);
        assert!(r.gate2_open);
        assert!(r.c_score > 0.8);
    }

    #[test]
    fn test_o0_consciousness() {
        // O₀ tuple: no gates open
        let o0 = IgTuple {
            d: IgPrim::D_wedge, t: IgPrim::T_net, r: IgPrim::R_super,
            p: IgPrim::P_asym, f: IgPrim::F_ell, k: IgPrim::K_fast,
            g: IgPrim::G_beth, c: IgPrim::C_and,
            phi: IgPrim::Phi_sub, h: IgPrim::H0, s: IgPrim::S_11,
            omega: IgPrim::Omega_0,
        };
        let r = consciousness_eval(&o0);
        assert!(!r.gate1_open);
        assert!(!r.gate2_open);
        assert_eq!(r.c_score, 0.0);
    }
}
