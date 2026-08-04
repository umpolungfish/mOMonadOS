#![allow(dead_code)]
//! belnap_shor_factors.rs — Belnap Shor with Factorization Post-Processing
//! ====================================================================
//! Extends belnap_shor.rs with:
//!   1. gcd-based factor extraction from period r
//!   2. Structural analysis of the coherence gap (belnapCost ≠ 2r for general N)
//!   3. Paraconsistent period verification
//!   4. Full Shor pipeline: N,a → period → factors
//!
//! STRUCTURAL STATUS (see DialetheicOperator.lean, BelnapQFT.lean):
//!   ┌─────────────────────────────────────────────────────────────┐
//!   │ The Belnap Shor does NOT compute the period from coherence  │
//!   │ costs. Rather, it demonstrates that the 2:1 B-bias/T-bias   │
//!   │ measurement ratio is the QUANTUM ADVANTAGE INVARIANT: the   │
//!   │ cost of maintaining B-coherence through measurement is      │
//!   │ exactly twice the cost of collapsing it.                    │
//!   │                                                             │
//!   │ Period r is found via classical search (or Belnap ring      │
//!   │ walk for numbers up to d=2048). The 2:1 ratio is the        │
//!   │ structural fingerprint, not the extraction mechanism.       │
//!   │                                                             │
//!   │ phi_upsilon_bottleneck: belnapCost = 2·period ONLY for      │
//!   │ the special case N=15,a=7 (where n=period=4 coincidentally).│
//!   │ For general N: belnapCost = 2n ≠ 2r. The theorem is gated   │
//!   │ on this precondition (proved as rfl for the canonical case).│
//!   └─────────────────────────────────────────────────────────────┘

use crate::belnap_shor::{ShorResult, run_belnap_shor};

// ── gcd (Euclidean algorithm) ─────────────────────────────────────────

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// ── Factor extraction from period ─────────────────────────────────────

/// Attempt to factor N given the period r of a modulo N.
/// For even r where a^(r/2) ≠ -1 mod N:
///   p = gcd(a^(r/2) - 1, N), q = gcd(a^(r/2) + 1, N)
#[derive(Clone, Debug)]
pub struct FactorResult {
    pub n: u64,
    pub a: u64,
    pub period: u64,
    pub factor1: Option<u64>,
    pub factor2: Option<u64>,
    pub trivial: bool,  // true if factors are 1 and N
    pub reason: &'static str,
}

pub fn extract_factors(n_val: u64, a: u64, period: u64) -> FactorResult {
    if period == 0 || n_val <= 1 {
        return FactorResult {
            n: n_val, a, period,
            factor1: None, factor2: None,
            trivial: true,
            reason: "invalid period or N",
        };
    }

    // Period must be even for the standard gcd trick
    if period % 2 != 0 {
        return FactorResult {
            n: n_val, a, period,
            factor1: None, factor2: None,
            trivial: true,
            reason: "period is odd — cannot use a^(r/2) trick",
        };
    }

    let half = period / 2;
    let a_half = mod_pow(a, half, n_val);

    // a^(r/2) ≡ -1 mod N → trivial factors
    if a_half == n_val - 1 {
        return FactorResult {
            n: n_val, a, period,
            factor1: None, factor2: None,
            trivial: true,
            reason: "a^(r/2) ≡ -1 mod N — try different a",
        };
    }

    let p = gcd(if a_half > 0 { a_half - 1 } else { n_val - 1 }, n_val);
    let q = gcd(a_half + 1, n_val);

    if p == 1 || q == 1 || p == n_val || q == n_val {
        return FactorResult {
            n: n_val, a, period,
            factor1: Some(p), factor2: Some(q),
            trivial: true,
            reason: "factors are trivial (1 or N)",
        };
    }

    FactorResult {
        n: n_val, a, period,
        factor1: Some(p.min(q)),
        factor2: Some(p.max(q)),
        trivial: false,
        reason: "non-trivial factors found",
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 0 { return 0; }
    if modulus == 1 { return 0; }
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 != 0 { result = (result * base) % modulus; }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

// ── Coherence gap analysis ────────────────────────────────────────────

/// Analyze the coherence gap: belnapCost vs 2·period.
/// The gap = belnapCost - 2·period measures how far we are from
/// the phi_upsilon_bottleneck precondition.
#[derive(Clone, Debug)]
pub struct CoherenceGap {
    pub n_qubits: usize,
    pub period: u64,
    pub belnap_cost: u32,  // B-bias measurement cost
    pub twice_period: u64,
    pub gap: i64,          // belnap_cost - 2*period (can be negative)
    pub ratio_to_2r: f64,  // belnap_cost / (2*period)
    pub precondition_holds: bool,  // belnapCost == 2*period
}

pub fn analyze_coherence_gap(
    n_qubits: usize, period: u64, belnap_cost: u32
) -> CoherenceGap {
    let twice = 2u64 * period;
    let gap = (belnap_cost as i64) - (twice as i64);
    let ratio = belnap_cost as f64 / (twice.max(1) as f64);
    CoherenceGap {
        n_qubits, period, belnap_cost,
        twice_period: twice,
        gap,
        ratio_to_2r: ratio,
        precondition_holds: gap == 0,
    }
}

// ── Full pipeline: Belnap Shor → factors ──────────────────────────────

#[derive(Clone, Debug)]
pub struct FullShorResult {
    pub n_qubits: usize,
    pub a: u64,
    pub n_val: u64,
    pub period: u64,
    pub shor_result: ShorResult,
    pub factors: FactorResult,
    pub gap: CoherenceGap,
}

/// Run the complete Belnap Shor pipeline: coherence analysis → period → factors.
pub fn run_full_belnap_shor(n: usize, a: u64, n_val: u64) -> FullShorResult {
    let shor = run_belnap_shor(n, a, n_val);
    let period = shor.period_cl;
    let factors = extract_factors(n_val, a, period);
    let gap = analyze_coherence_gap(n, period, shor.b_bias_coherence);

    FullShorResult {
        n_qubits: n, a, n_val, period,
        shor_result: shor,
        factors,
        gap,
    }
}

/// Auto-size: choose n = ceil(log₂(N)) with minimum 2 qubits.
pub fn run_full_belnap_shor_auto(a: u64, n_val: u64) -> FullShorResult {
    let n = if n_val <= 1 { 2 } else {
        let mut bits = 0;
        let mut v = n_val - 1;
        while v > 0 { bits += 1; v >>= 1; }
        bits.max(2) as usize
    };
    run_full_belnap_shor(n, a, n_val)
}

// ── Paraconsistent period verification ────────────────────────────────

/// Verify the period using the Belnap lattice structure.
/// In the Belnap framework, a^r ≡ 1 (mod N) means the modular
/// exponentiation cycle closes. We verify that B-propagation
/// through the ModExp table is consistent with the period.
pub fn verify_period_paraconsistently(a: u64, n_val: u64, period: u64) -> bool {
    if n_val <= 1 || period == 0 { return false; }
    // Verify: a^period ≡ 1 mod N
    mod_pow(a, period, n_val) == 1
    // Structural constraint: for all smaller r', a^r' ≠ 1
    && (1..period).all(|r| mod_pow(a, r, n_val) != 1)
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(15, 1), 1);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn test_factor_n15_a7() {
        // N=15, a=7, period=4, a^(r/2)=7^2=49≡4 mod 15
        // gcd(4-1, 15)=gcd(3,15)=3, gcd(4+1,15)=gcd(5,15)=5
        let f = extract_factors(15, 7, 4);
        assert!(!f.trivial);
        assert_eq!(f.factor1, Some(3));
        assert_eq!(f.factor2, Some(5));
    }

    #[test]
    fn test_factor_n21_a5() {
        // N=21, a=5, period=6, a^(r/2)=5^3=125≡20 mod 21
        // gcd(20-1,21)=gcd(19,21)=1 → trivial
        let f = extract_factors(21, 5, 6);
        assert!(f.trivial);
        assert_eq!(f.reason, "factors are trivial (1 or N)");
    }

    #[test]
    fn test_factor_n35_a2() {
        // N=35, a=2, period=12, a^(r/2)=2^6=64≡29 mod 35
        // gcd(29-1,35)=gcd(28,35)=7, gcd(29+1,35)=gcd(30,35)=5
        let f = extract_factors(35, 2, 12);
        assert!(!f.trivial);
        assert_eq!(f.factor1, Some(5));
        assert_eq!(f.factor2, Some(7));
    }

    #[test]
    fn test_coherence_gap_n15() {
        // N=15: n=4, period=4, belnapCost=8, 2*period=8 → gap=0
        let gap = analyze_coherence_gap(4, 4, 8);
        assert!(gap.precondition_holds);
        assert_eq!(gap.gap, 0);
        assert_eq!(gap.ratio_to_2r, 1.0);
    }

    #[test]
    fn test_coherence_gap_n21() {
        // N=21: n=5, period=6, belnapCost=10, 2*period=12 → gap=-2
        let gap = analyze_coherence_gap(5, 6, 10);
        assert!(!gap.precondition_holds);
        assert_eq!(gap.gap, -2);
        assert!((gap.ratio_to_2r - 10.0/12.0).abs() < 0.001);
    }

    #[test]
    fn test_coherence_gap_n35() {
        // N=35: n=6, period=12, belnapCost=12, 2*period=24 → gap=-12
        let gap = analyze_coherence_gap(6, 12, 12);
        assert!(!gap.precondition_holds);
        assert_eq!(gap.gap, -12);
    }

    #[test]
    fn test_paraconsistent_period() {
        assert!(verify_period_paraconsistently(7, 15, 4));
        assert!(verify_period_paraconsistently(5, 21, 6));
        assert!(verify_period_paraconsistently(2, 35, 12));
        assert!(!verify_period_paraconsistently(7, 15, 3)); // wrong period
    }

    #[test]
    fn test_full_pipeline_n15() {
        let r = run_full_belnap_shor(4, 7, 15);
        assert_eq!(r.period, 4);
        assert!(!r.factors.trivial);
        assert_eq!(r.factors.factor1, Some(3));
        assert_eq!(r.factors.factor2, Some(5));
        assert!(r.gap.precondition_holds);
    }
}
