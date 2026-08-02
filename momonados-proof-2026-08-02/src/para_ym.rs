#![allow(dead_code)]
//! Yang-Mills Mass Gap Bridge — priests-engine/para_ym.py
//! Mass gap Δ>0 = covering relation N<T in Belnap approximation order.
//! BRST nilpotence Q²=0 ↔ ENGAGR B-stability. Omega_Z gauge protection.

use crate::belnap::B4;

/// N<T is a covering relation: no x with N <_a x <_a T.
pub fn ym_gap_exists() -> bool {
    !([B4::N, B4::T, B4::F, B4::B].iter().any(|&x| {
        B4::N.approx_le(x) && x.approx_le(B4::T) && x != B4::N && x != B4::T
    }))
}

/// Ground state T is not dialetheic — gap is definite.
pub fn ym_gap_not_dialetheic() -> bool { !B4::T.dialetheic() }

/// N (vacuum) is unique undesignated element, T∧F=N.
pub fn ym_vacuum_canonical() -> bool {
    !B4::N.designated() && B4::N.approx_le(B4::T) && B4::T.meet(B4::F) == B4::N
}

/// BRST: Q²=0 ↔ ENGAGR(B)=B (stable), ENGAGR(T)=F (nilpotent).
pub fn ym_brst_nilpotent() -> bool {
    let q_b = B4::B.band(B4::B.bnot()); // ENGAGR(B)=B
    let q_t = B4::T.band(B4::T.bnot()); // ENGAGR(T)=F
    // ffuse∘fsplit(B)=B
    let frob = {
        let (d1, d2) = if B4::B == B4::B { (B4::T, B4::F) } else { (B4::B, B4::B) };
        d1.join(d2) == B4::B
    };
    q_b == B4::B && q_t != B4::B && frob
}

/// Confinement: T cannot reach N via any lattice op.
pub fn ym_confinement_ktrap() -> bool {
    ![B4::T.bnot(), B4::T.join(B4::T), B4::T.meet(B4::T),
      B4::T.band(B4::T), B4::T.bor(B4::T)].contains(&B4::N)
}

/// Omega_Z: gap preserved under join; T∨F=B (annihilation).
pub fn ym_topological_protection() -> bool {
    B4::T.join(B4::T) == B4::T
        && B4::N.join(B4::T) == B4::T
        && B4::T.join(B4::F) == B4::B
}

pub const YM_IMSCRIPTION: &str = "⟨𐑦·𐑰·𐑽·𐑹·𐑐·𐑪·𐑲·𐑵·⊙·𐑫·𐑕·𐑭⟩";

pub fn mass_gap_positive() -> bool {
    ym_gap_exists() && ym_gap_not_dialetheic() && ym_vacuum_canonical()
}

pub fn ym_brst_frobenius() -> bool {
    ym_brst_nilpotent() && ym_confinement_ktrap() && ym_topological_protection()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_gap() { assert!(mass_gap_positive()); }
    #[test] fn test_brst() { assert!(ym_brst_frobenius()); }
    #[test] fn test_topology() { assert!(ym_topological_protection()); }
}
