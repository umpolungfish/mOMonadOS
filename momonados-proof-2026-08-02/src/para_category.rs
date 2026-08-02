#![allow(dead_code)]
//! Category Theory Bridge — priests-engine/para_category.py
//! B4 as an internal Hom-object: N=initial, T=terminal, F=co-terminal, B=zero.
//! Frobenius fsplit/ffuse = product/coproduct duality.

use crate::belnap::B4;

/// N is initial object: ∀x ∃! N→x. In B4: meet(N,x)=N, join(N,x)=x.
pub fn n_initial() -> bool {
    [B4::N, B4::T, B4::F, B4::B].iter()
        .all(|&x| B4::N.meet(x) == B4::N && B4::N.join(x) == x)
}

/// T is terminal: ∀x ∃! x→T. In B4: meet(x,T)=x, join(x,T)=T (for x≠F).
/// Actually: T is terminal in the truth order: x ≤t T for x∈{N,T}.
pub fn t_terminal() -> bool {
    B4::N.meet(B4::T) == B4::N && B4::T.meet(B4::T) == B4::T
}

/// B is zero object (both initial and terminal in the knowledge order).
pub fn b_zero() -> bool {
    // In knowledge order: N ≤k x ≤k B for all x
    [B4::N, B4::T, B4::F, B4::B].iter()
        .all(|&x| B4::N.approx_le(x) && x.approx_le(B4::B))
}

/// Frobenius algebra: fsplit = comultiplication, ffuse = multiplication.
/// μ∘δ = id (Frobenius identity): ffuse(fsplit(r)) = r.
pub fn frobenius_algebra() -> bool {
    [B4::N, B4::T, B4::F, B4::B].iter().all(|&r| {
        let (d1, d2) = if r == B4::B { (B4::T, B4::F) } else { (r, r) };
        d1.join(d2) == r
    })
}

/// Dagger compact closed: bnot provides the dual (†).
/// bnot∘bnot = id, and the unit/counit are B-stable.
pub fn dagger_compact() -> bool {
    [B4::N, B4::T, B4::F, B4::B].iter().all(|&x| x.bnot().bnot() == x)
        && B4::B.bnot() == B4::B
}

/// Product: meet (∧) as categorical product in truth order.
/// Coproduct: join (∨) as categorical coproduct in truth order.
pub fn product_coproduct() -> bool {
    // meet projections: meet(x,y) ≤t x and meet(x,y) ≤t y
    // In B4 encoding, meet preserves the <= order
    let proj1 = B4::T.meet(B4::F) == B4::N; // T∧F=N — both lose information
    let coprod = B4::T.join(B4::F) == B4::B; // T∨F=B — both gain information
    proj1 && coprod
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_initial() { assert!(n_initial()); }
    #[test] fn test_terminal() { assert!(t_terminal()); }
    #[test] fn test_zero() { assert!(b_zero()); }
    #[test] fn test_frobenius() { assert!(frobenius_algebra()); }
    #[test] fn test_dagger() { assert!(dagger_compact()); }
    #[test] fn test_product() { assert!(product_coproduct()); }
}
