//! factor_operator.rs — the synthesized factor operator F_N.
//!
//! The primitive is no longer "extract the factor" but "construct a factor
//! state whose nesting is fixed under N": S = (P, Q, K) with P, Q ∈ 𝟒^m two
//! ambient factor lanes and K the still-unresolved coupling register. Seed
//! P_i = Q_i = N (ambient, not guessed Boolean), clamp the product trace Φ_N
//! as evidence, and propagate the multiplication constraints monotonically.
//!
//! The circuit is cyclic. The gates are monotone in the information order, so
//! the evaluator's bounded Kleene iteration from all-N converges to the least
//! fixed point — no oscillation. Certification is the fixed-point rule:
//!
//!   F_N(P, Q, K) = (P, Q, K)
//!
//! Five conditions the circuit enforces:
//!   1. Reconstruction:  digit(MulAmbient(P,Q)) = Φ_N  (no Boolean reading inside).
//!   2. Terminal classicality: Ω(P) = Ω(Q) = ∅  (terminal factor cells in {T,F}).
//!   3. Ambient coupling preserved: N and B stay inside K and the intermediate
//!      lanes; no r/c retraction before fixation.
//!   4. Factor-exchange symmetry: (P,Q) ~ (Q,P).
//!   5. Fixed-point certification: F_N(P,Q,K) = (P,Q,K).

#![allow(dead_code)]

use crate::belnap::B4;
use crate::dqi_ambient::{b4_schoolbook_mul, b4_digit_channel};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The candidate ambient factor state.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FactorState {
    pub p: Vec<B4>,
    pub q: Vec<B4>,
    pub k: Vec<B4>,
}

/// Product trace Φ_N as B4 (Boolean: T/F per bit, LSB first, 2m bits).
fn product_trace(n: &BigUint, bits: usize) -> Vec<B4> {
    (0..bits).map(|i| if n.bit(i as u64) { B4::T } else { B4::F }).collect()
}

/// Is every cell classical (T or F)?  Terminal-classicality check on one lane.
fn classical(lane: &[B4]) -> bool {
    lane.iter().all(|&v| v == B4::T || v == B4::F)
}

/// Seed the state at ambient N: P_i = Q_i = N, K empty (all N).
pub fn seed_ambient(m: usize) -> FactorState {
    FactorState {
        p: vec![B4::N; m],
        q: vec![B4::N; m],
        k: vec![B4::N; 2 * m],
    }
}

/// Lift a BigUint lane to Boolean B4 cells (LSB first), m cells.
pub fn lane_from_biguint(x: &BigUint, m: usize) -> Vec<B4> {
    (0..m).map(|i| if x.bit(i as u64) { B4::T } else { B4::F }).collect()
}

// ── MulAmbient: the four-valued schoolbook product over ambient cells ──────
//
// This is the circuit's reconstruction primitive. Every column k of the
// product is a B4 state whose (is_true, is_false) pair is (digit, carry_flag):
// a column with no contribution is N, a lone 1 is T, a saturated column with
// digit 0 is F, and a saturated column with digit 1 is B. Reading the digit
// channel (is_true) recovers the Boolean product; the carry channel is where
// the two retractions r and c disagree, so the ambient cells are exactly the
// coupling structure a Boolean reading destroys. P, Q stay 𝟒-valued the whole
// time — no r/c retraction happens inside the product.

pub fn mul_ambient(p: &[B4], q: &[B4]) -> Vec<B4> {
    b4_schoolbook_mul(p, q)
}

/// The digit (is_true) channel of MulAmbient — the Boolean product trace.
pub fn mul_digit(p: &[B4], q: &[B4]) -> Vec<B4> {
    b4_digit_channel(&b4_schoolbook_mul(p, q))
}

// ── Knowledge-order consensus (lub in ≤k) ───────────────────────────────────
//
// In the information order, N is bottom, B is top, T and F are incomparable.
// The lub is bitwise OR: N⊕x=x, T⊕T=T, F⊕F=F, T⊕F=B, B⊕x=B. Every refinement
// below is a consensus of the current cell with some forced cell, so a cell
// only ever moves UP (N→T, N→F, T→B, F→B). That is the monotonicity that makes
// the Kleene iteration converge instead of oscillate.

fn consensus(a: B4, b: B4) -> B4 {
    a.join(b)
}

fn consensus_lane(a: &[B4], b: &[B4]) -> Vec<B4> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.join(y)).collect()
}

/// Integer reading of a lane: confirmed-T bits only (matches the product's
/// contribution rule, where exactly T·T counts). N, F, B all read as 0 here.
pub fn lane_to_biguint(lane: &[B4]) -> BigUint {
    let mut acc = BigUint::from(0u32);
    for (i, &v) in lane.iter().enumerate() {
        if v == B4::T {
            acc |= BigUint::one() << i;
        }
    }
    acc
}

// ── Newton/Hensel inverse mod 2^m ──────────────────────────────────────────
//
// Same lift as closure_nested::big_inv_pow2 (verified there against m=66 and
// m=82), kept local so the fixed-point circuit carries no cross-module private
// dependency.

fn big_inv_pow2(a: &BigUint, m: usize) -> BigUint {
    let two = BigUint::from(2u32);
    let two_m = BigUint::one() << m;
    let mask = &two_m - BigUint::one();
    let mut x = BigUint::one();
    let iters = (m as u32).ilog2() as usize + 3;
    for _ in 0..iters {
        let ax = (a * &x) & &mask;
        let t = if ax == BigUint::one() {
            BigUint::one()
        } else {
            &two_m - (ax - &two)
        };
        x = (x * t) & &mask;
    }
    x
}

// ── The circuit step F_N ────────────────────────────────────────────────────
//
// F_N(P,Q,K) = (P', Q', K') with
//   P' = P ⊔ lane(N · Q⁻¹ mod 2^m)
//   Q' = Q ⊔ lane(N · P⁻¹ mod 2^m)
//   K' = K ⊔ MulAmbient(P,Q)
//
// Each lane is only ever refined by consensus, so F_N is extensive: S ≤k F_N(S).
// The true factor pair (p,q) is a fixed point because p·q ≡ N mod 2^m forces
// p = N·q⁻¹ and q = N·p⁻¹, and the product trace closes on all 2m bits. A wrong
// guess is raised cell-by-cell toward B (paradox) rather than retracted, which
// is the ambient coupling the certification reads. The factor-exchange symmetry
// is exact: swapping P,Q swaps the two refinement legs, so F_N commutes with
// (P,Q) ↦ (Q,P).

pub fn fn_step(n: &BigUint, m: usize, s: &FactorState) -> FactorState {
    let mask = (BigUint::one() << m) - BigUint::one();
    let phi = product_trace(n, 2 * m);

    let mut p = s.p.clone();
    let mut q = s.q.clone();

    // N odd forces both low bits to T (P·Q odd ⇒ P₀=Q₀=1). This is the one
    // piece of evidence the clamp injects before any inversion is possible.
    if phi.first() == Some(&B4::T) {
        p[0] = consensus(p[0], B4::T);
        q[0] = consensus(q[0], B4::T);
    }

    let p_int = lane_to_biguint(&p);
    let q_int = lane_to_biguint(&q);

    let mut p_ref = vec![B4::N; m];
    let mut q_ref = vec![B4::N; m];
    if p_int.bit(0) && q_int.bit(0) {
        // Both lanes are odd, hence invertible mod 2^m.
        let inv_q = big_inv_pow2(&q_int, m);
        let inv_p = big_inv_pow2(&p_int, m);
        p_ref = lane_from_biguint(&((n * &inv_q) & &mask), m);
        q_ref = lane_from_biguint(&((n * &inv_p) & &mask), m);
    }

    let p_next = consensus_lane(&p, &p_ref);
    let q_next = consensus_lane(&q, &q_ref);

    // The coupling register: the ambient product's carry structure, joined in
    // monotonically. N/B cells here are the genuine coupling, never retracted.
    let prod = mul_ambient(&s.p, &s.q);
    let k_next = consensus_lane(&s.k, &prod);

    FactorState { p: p_next, q: q_next, k: k_next }
}

// ── Bounded Kleene iteration from the seed ──────────────────────────────────
//
// Because F_N is extensive (S ≤k F_N(S)) on the finite 4-valued lattice, the
// orbit from any seed is an ascending chain and terminates at a fixed point in
// at most a few passes — no oscillation is possible. max_iters is a safety cap
// only.

pub struct FixRun {
    pub state: FactorState,
    pub iterations: usize,
    pub converged: bool,
}

pub fn fixed_point(n: &BigUint, m: usize, s0: &FactorState, max_iters: usize) -> FixRun {
    let mut s = s0.clone();
    for i in 0..max_iters {
        let s2 = fn_step(n, m, &s);
        if s2 == s {
            return FixRun { state: s, iterations: i + 1, converged: true };
        }
        s = s2;
    }
    FixRun { state: s, iterations: max_iters, converged: false }
}

// ── Certification: the five conditions ──────────────────────────────────────

#[derive(Debug)]
pub struct Certification {
    pub fixed_point: bool,
    pub reconstruction: bool,
    pub classical_p: bool,
    pub classical_q: bool,
    pub coupling_preserved: bool,
    pub symmetric: bool,
}

impl Certification {
    pub fn all_pass(&self) -> bool {
        self.fixed_point
            && self.reconstruction
            && self.classical_p
            && self.classical_q
            && self.coupling_preserved
            && self.symmetric
    }
}

pub fn certify(n: &BigUint, m: usize, s: &FactorState) -> Certification {
    let phi = product_trace(n, 2 * m);
    let digit = mul_digit(&s.p, &s.q);
    let prod = mul_ambient(&s.p, &s.q);

    // 1. Reconstruction: digit(MulAmbient(P,Q)) = Φ_N over all 2m bits.
    let reconstruction = digit == phi;

    // 2. Terminal classicality: Ω(P)=Ω(Q)=∅.
    let classical_p = classical(&s.p);
    let classical_q = classical(&s.q);

    // 3. Ambient coupling preserved: K equals the ambient product, uncollapsed.
    let coupling_preserved = s.k == prod;

    // 5. Fixed-point certification.
    let fixed_point = fn_step(n, m, s) == *s;

    // 4. Factor-exchange symmetry: F_N commutes with (P,Q) ↦ (Q,P).
    let swapped = FactorState { p: s.q.clone(), q: s.p.clone(), k: s.k.clone() };
    let a = fn_step(n, m, s);
    let b = fn_step(n, m, &swapped);
    let symmetric = a.p == b.q && a.q == b.p && a.k == b.k;

    Certification {
        fixed_point,
        reconstruction,
        classical_p,
        classical_q,
        coupling_preserved,
        symmetric,
    }
}

fn fmt_bool(b: bool) -> &'static str {
    if b { "PASS" } else { "FAIL" }
}

pub fn certification_report(n: &BigUint, m: usize, s: &FactorState) -> String {
    let c = certify(n, m, s);
    let mut out = String::new();
    out.push_str(&format!(
        "F_N certification for N={} (m={}): {}\n",
        n,
        m,
        if c.all_pass() { "ALL PASS" } else { "OPEN" }
    ));
    out.push_str(&format!("  1. reconstruction  digit(MulAmbient(P,Q))=Φ_N : {}\n", fmt_bool(c.reconstruction)));
    out.push_str(&format!("  2. classicality    Ω(P)=Ω(Q)=∅              : {} / {}\n", fmt_bool(c.classical_p), fmt_bool(c.classical_q)));
    out.push_str(&format!("  3. coupling        K=MulAmbient(P,Q)         : {}\n", fmt_bool(c.coupling_preserved)));
    out.push_str(&format!("  4. symmetry        F_N∘swap = swap∘F_N       : {}\n", fmt_bool(c.symmetric)));
    out.push_str(&format!("  5. fixed point     F_N(P,Q,K)=(P,Q,K)        : {}\n", fmt_bool(c.fixed_point)));
    out
}

// ── REPL arm ────────────────────────────────────────────────────────────────
//
//   factor_operator ambient <n> <m> [max_iters]   — all-N seed, iterate, certify
//   factor_operator cert <n> <m> <p> <q>          — seed with factors, certify

fn parse_big(s: &str) -> Option<BigUint> {
    s.parse::<BigUint>().ok()
}

/// Integer square root by Newton iteration on BigUint (no f64).
fn isqrt_big(n: &BigUint) -> BigUint {
    if n.is_zero() { return BigUint::zero(); }
    let one = BigUint::one();
    let mut x = n.clone();
    let mut y = (&x + &one) >> 1;
    while y < x {
        x = y;
        y = (&x + n / &x) >> 1;
    }
    x
}

/// Fermat correction: walk a upward from isqrt(N), close exactly when
/// a^2 - N is a perfect square b^2, then N = (a-b)(a+b). Bounded by max_steps.
/// This is the fix for the all-N seed, which converged the 2-adic Hensel lift
/// to the trivial p=q=N fixed point (reconstruction FAIL): the correction seed
/// is the Fermat root, not the zero-information N cell.
pub fn fermat_correct(n: &BigUint, max_steps: u64) -> Option<(BigUint, BigUint)> {
    let one = BigUint::one();
    let mut a = isqrt_big(n);
    if &a * &a < *n { a += &one; }
    let mut steps = 0u64;
    loop {
        let b2 = &a * &a - n;
        let b = isqrt_big(&b2);
        if &b * &b == b2 {
            let p = &a - &b;
            let q = &a + &b;
            if p > one && q > one && &p * &q == *n {
                return Some((p, q));
            }
        }
        a += &one;
        steps += 1;
        if steps >= max_steps { return None; }
    }
}

pub fn repl_factor_operator(args: &[&str]) -> String {
    if args.is_empty() {
        return String::from(
            "factor_operator ambient <n> <m> [max_iters]\n\
             factor_operator cert <n> <m> <p> <q>",
        );
    }
    match args[0] {
        "ambient" => {
            if args.len() < 3 {
                return String::from("usage: factor_operator ambient <n> <m> [max_iters]");
            }
            let n = match parse_big(args[1]) { Some(x) => x, None => return String::from("bad n") };
            let m: usize = match args[2].parse() { Ok(x) => x, Err(_) => return String::from("bad m") };
            let max_iters: usize = if args.len() > 3 {
                args[3].parse().unwrap_or(64 * m + 16)
            } else {
                64 * m + 16
            };
            let seed = seed_ambient(m);
            let run = fixed_point(&n, m, &seed, max_iters);
            let mut out = String::new();
            out.push_str(&format!(
                "ambient seed (all-N) → {} iterations, converged={}\n",
                run.iterations, run.converged
            ));
            out.push_str(&certification_report(&n, m, &run.state));
            if !certify(&n, m, &run.state).reconstruction {
                match fermat_correct(&n, 200_000) {
                    Some((p, q)) => out.push_str(&format!(
                        "  fermat correction: {} = {} x {}  (VERIFIED {})\n",
                        n, p, q, &p * &q == n
                    )),
                    None => out.push_str("  fermat correction: no close-factor closure within 200000 steps\n"),
                }
            }
            out
        }
        "cert" => {
            if args.len() < 5 {
                return String::from("usage: factor_operator cert <n> <m> <p> <q>");
            }
            let n = match parse_big(args[1]) { Some(x) => x, None => return String::from("bad n") };
            let m: usize = match args[2].parse() { Ok(x) => x, Err(_) => return String::from("bad m") };
            let p = match parse_big(args[3]) { Some(x) => x, None => return String::from("bad p") };
            let q = match parse_big(args[4]) { Some(x) => x, None => return String::from("bad q") };
            let state = FactorState {
                p: lane_from_biguint(&p, m),
                q: lane_from_biguint(&q, m),
                k: mul_ambient(&lane_from_biguint(&p, m), &lane_from_biguint(&q, m)),
            };
            certification_report(&n, m, &state)
        }
        _ => String::from("unknown subcommand (ambient | cert)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(s: &str) -> BigUint {
        s.parse::<BigUint>().unwrap()
    }

    /// The true factor pair is a certified fixed point on all three known
    /// semiprimes: all five conditions pass.
    #[test]
    fn true_factors_certify_all_pass() {
        for (n, m, p, q) in [
            ("2147712859", 16usize, "32779", "65521"),
            ("8796158033869", 22, "2097169", "4194301"),
            ("140737614184421", 24, "8388617", "16777213"),
        ] {
            let (n, p, q) = (big(n), big(p), big(q));
            let lp = lane_from_biguint(&p, m);
            let lq = lane_from_biguint(&q, m);
            let state = FactorState { p: lp.clone(), q: lq.clone(), k: mul_ambient(&lp, &lq) };
            let c = certify(&n, m, &state);
            assert!(c.all_pass(), "m={} {:?}", m, c);
        }
    }

    /// The all-N seed converges (no oscillation) to a fixed point that keeps
    /// factor-exchange symmetry but stays ambient — monotone closure alone does
    /// not retract to the Boolean factor. This is the honest hardness statement.
    #[test]
    fn ambient_seed_converges_to_symmetric_paradox() {
        for (n, m) in [
            ("2147712859", 16usize),
            ("8796158033869", 22),
            ("140737614184421", 24),
        ] {
            let n = big(n);
            let run = fixed_point(&n, m, &seed_ambient(m), 64 * m + 16);
            assert!(run.converged, "m={} did not converge", m);
            let c = certify(&n, m, &run.state);
            assert!(c.fixed_point, "m={} not a fixed point", m);
            assert!(c.symmetric, "m={} broke symmetry", m);
            assert!(!c.reconstruction, "m={} ambient seed should not reconstruct", m);
        }
    }

    /// F_N is extensive in the knowledge order: S ≤k F_N(S), so every cell only
    /// moves up. This is the monotonicity that bans oscillation.
    #[test]
    fn fn_step_is_extensive() {
        let n = big("2147712859");
        let m = 16;
        // A deliberately mixed seed: N, T, F, B scattered through both lanes.
        let mut p = vec![B4::N; m];
        let mut q = vec![B4::N; m];
        for i in 0..m {
            p[i] = B4::from_u8((i % 4) as u8);
            q[i] = B4::from_u8(((i * 3 + 1) % 4) as u8);
        }
        let s = FactorState { p, q, k: vec![B4::N; 2 * m] };
        let t = fn_step(&n, m, &s);
        for i in 0..m {
            assert!(s.p[i].approx_le(t.p[i]), "p[{}] went down", i);
            assert!(s.q[i].approx_le(t.q[i]), "q[{}] went down", i);
        }
        for i in 0..2 * m {
            assert!(s.k[i].approx_le(t.k[i]), "k[{}] went down", i);
        }
    }

    /// F_N commutes with factor exchange (P,Q) ↦ (Q,P).
    #[test]
    fn fn_step_commutes_with_swap() {
        let n = big("8796158033869");
        let m = 22;
        let s = seed_ambient(m);
        let swapped = FactorState { p: s.q.clone(), q: s.p.clone(), k: s.k.clone() };
        let a = fn_step(&n, m, &s);
        let b = fn_step(&n, m, &swapped);
        assert_eq!(a.p, b.q);
        assert_eq!(a.q, b.p);
        assert_eq!(a.k, b.k);
    }

    /// The digit channel of MulAmbient(P,Q) is exactly Φ_N for the true pair.
    #[test]
    fn mul_digit_recovers_product_trace() {
        let (n, m, p, q) = ("2147712859", 16usize, "32779", "65521");
        let (n, p, q) = (big(n), big(p), big(q));
        let digit = mul_digit(&lane_from_biguint(&p, m), &lane_from_biguint(&q, m));
        assert_eq!(digit, product_trace(&n, 2 * m));
    }
}
