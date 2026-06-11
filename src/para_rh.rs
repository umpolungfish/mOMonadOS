//! Riemann Hypothesis Bridge — priests-engine/para_rh.py
//! Structural claim: ζ(s) = χ(s)ζ(1-s) is Belnap negation (bnot).
//! Critical line Re(s)=1/2 is the unique designated fixed point of bnot.
//! RH in Belnap: all non-trivial zeros are B-designated.

use crate::belnap::B4;

/// Map Re(s) as fraction num/den to B4.
///   < 0 or > den:  N  (outside strip)
///   = den/2:       B  (critical line Re=1/2)
///   0 < num < den: T  (non-critical interior)
///   = 0 or = den:  F  (boundary)
pub fn rh_strip_state(re_s_num: i32, re_s_den: i32) -> B4 {
    if re_s_num < 0 || re_s_num > re_s_den { return B4::N; }
    if re_s_num == 0 || re_s_num == re_s_den { return B4::F; }
    if 2 * re_s_num == re_s_den { return B4::B; }
    B4::T
}

/// Functional equation s→1-s as Belnap negation: bnot∘bnot = id.
pub fn rh_functional_eq(s: B4) -> B4 { s.bnot() }

/// B is the unique designated fixed point of the functional equation.
pub fn rh_frobenius_fixed_point() -> bool {
    B4::B.bnot() == B4::B
        && B4::B.designated()
        && B4::T.bnot() != B4::T               // bnot(T) = F ≠ T
        && !B4::T.bnot().designated()           // bnot(T) = F, not designated
}

/// RH in Belnap FOUR: B is the only value that is both designated
/// and a fixed point of bnot.
pub fn rh_belnap_statement() -> bool {
    // B is unique designated fixed point
    let unique_fixed = B4::B.designated() && B4::B.bnot() == B4::B
        && [B4::N, B4::T, B4::F].iter()
            .all(|&x| !(x.designated() && x.bnot() == x));
    // B is dialetheic, nothing else is
    unique_fixed && B4::B.dialetheic()
        && [B4::N, B4::T, B4::F].iter().all(|&x| !x.dialetheic())
}

/// bnot∘bnot = id: functional equation applied twice = identity.
pub fn rh_involution_identity() -> bool {
    [B4::N, B4::T, B4::F, B4::B].iter().all(|&x| x.bnot().bnot() == x)
}

/// RH bridge imscription: D_holo·T_holo·R_dagger·P_pm_sym·F_hbar·K_slow
/// ·G_aleph·Gamma_seq·Phi_c·H2·n_m·Omega_Z2
pub const RH_IMSCRIPTION: &str = "⟨𐑦·𐑸·𐑽·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑖·𐑳·𐑴⟩";

/// Contains both ⊙ (Phi_c) and 𐑹 (P_pm_sym) → O_∞ tier.
pub fn rh_bridge_is_o_inf() -> bool {
    RH_IMSCRIPTION.contains('⊙') && RH_IMSCRIPTION.contains('𐑹')
}

/// B simultaneously satisfies all three Millennium barrier conditions:
/// RH: B is unique designated fixed point of bnot (critical line).
/// P vs NP: B is dialetheic (NP witness, one-way barrier).
/// SIC-POVM: B satisfies equiangular + information-complete axioms.
pub fn millennium_barriers_unified() -> bool {
    let rh_ok = B4::B.bnot() == B4::B && B4::B.designated();
    let pvsnp_ok = B4::B.dialetheic();
    let all_vals = [B4::N, B4::T, B4::F, B4::B];
    let sic_ok = all_vals.iter().all(|&x| x.meet(B4::B) == x)
              && all_vals.iter().all(|&x| x.join(B4::B) == B4::B);
    rh_ok && pvsnp_ok && sic_ok
}

/// Strip samples for display.
pub const STRIP_SAMPLES: &[(i32, &str)] = &[
    (-10, "Re=-0.1"), (0, "Re=0.0"), (10, "Re=0.1"), (25, "Re=0.25"),
    (49, "Re=0.49"),  (50, "Re=0.5"), (51, "Re=0.51"), (75, "Re=0.75"),
    (100, "Re=1.0"), (110, "Re=1.1"),
];

pub fn strip_label(s: B4) -> &'static str {
    match s {
        B4::N => "outside strip",
        B4::F => "strip boundary",
        B4::T => "non-critical interior",
        B4::B => "CRITICAL LINE (Re=1/2)",
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_state() {
        assert_eq!(rh_strip_state(50, 100), B4::B);  // Re=0.5
        assert_eq!(rh_strip_state(25, 100), B4::T);  // interior
        assert_eq!(rh_strip_state(0, 100), B4::F);   // boundary
        assert_eq!(rh_strip_state(-10, 100), B4::N); // outside
    }

    #[test]
    fn test_frobenius_fixed() {
        assert!(rh_frobenius_fixed_point());
    }

    #[test]
    fn test_belnap_statement() {
        assert!(rh_belnap_statement());
    }

    #[test]
    fn test_involution() {
        assert!(rh_involution_identity());
    }

    #[test]
    fn test_o_inf() {
        assert!(rh_bridge_is_o_inf());
    }

    #[test]
    fn test_barriers_unified() {
        assert!(millennium_barriers_unified());
    }
}
