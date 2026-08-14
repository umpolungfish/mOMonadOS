#![allow(dead_code)]
//! Temporal Logic Bridge — priests-engine/para_temporal.py
//! B4 temporal operators: always(□), eventually(◇), until(U).
//! B is the temporal fixed point — always-and-never simultaneously.

use crate::belnap::B4;

/// Always: □T=T, □F=F, □N=N, □B=B (all are fixed points).
pub fn always(p: B4) -> B4 { p }

/// Eventually: ◇T=T, ◇F=F, ◇N=N, ◇B=B (idempotent on B4 values).
pub fn eventually(p: B4) -> B4 { p }

/// Next: ○T=F (toggle), ○F=T, ○N=N, ○B=B.
pub fn next(p: B4) -> B4 { p.bnot() }

/// Until: p U q. B U x = B (B absorbs), N U x = x, T U F = F, etc.
pub fn until(p: B4, q: B4) -> B4 {
    if p == B4::B { return B4::B; }
    if p == B4::N { return q; }
    // T or F: release when q becomes true or contradiction
    if q == B4::B || q == p { return q; }
    p
}

/// B is the temporal fixed point: □B=◇B=○B=B.
pub fn b_temporal_fixed() -> bool {
    always(B4::B) == B4::B && eventually(B4::B) == B4::B && next(B4::B) == B4::B
}

/// Temporal involution: ○○p = p  ∀p.
pub fn next_involution() -> bool {
    [B4::N, B4::T, B4::F, B4::B].iter().all(|&p| next(next(p)) == p)
}

/// B absorbs until: B U x = B, x U B = B  ∀x.
pub fn b_absorbs_until() -> bool {
    [B4::N, B4::T, B4::F, B4::B].iter()
        .all(|&x| until(B4::B, x) == B4::B && until(x, B4::B) == B4::B)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_fixed() { assert!(b_temporal_fixed()); }
    #[test] fn test_involution() { assert!(next_involution()); }
    #[test] fn test_absorbs() { assert!(b_absorbs_until()); }
    #[test] fn test_until() {
        assert_eq!(until(B4::N, B4::T), B4::T);
        assert_eq!(until(B4::T, B4::F), B4::T);
        assert_eq!(until(B4::B, B4::N), B4::B);
    }
}
