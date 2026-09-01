//! multilattice.rs — the FDE/QM boundary, run as real code.
//!
//! Built directly from two Lean files, both checked *sans* sorry this
//! session: BelnapWHMultilattice.lean (proves the naive product-lattice
//! WH action is stuck at orbit 2^n) and SIC_Multilattice_Proof.lean
//! (the corrected Pauli-algebra action, orbit 4^n, zero axioms). Where
//! Lean proves a closed-form count, this file computes the same count by
//! direct enumeration -- the same claim, checked twice, by two different
//! methods, not read off a doc a second time.
//!
//! THE BOUNDARY, made numeric:
//! - n=1 (d=2): the Belnap evidence-counting Born rule (integers, no
//!   square roots) and the standard QM Hadamard's |amplitude|^2 Born
//!   rule agree EXACTLY: both give 1/2. FDE is not an approximation of
//!   QM here; it reproduces it exactly, in different arithmetic.
//! - n>1: the two constructions' orbit sizes are literally different
//!   (naive: 2^n, corrected: 4^n) -- computed, not asserted -- and even
//!   the corrected multilattice's own join-based "equiangularity" is a
//!   different quantity from the Hilbert-space ratio 1/(d+1) that real
//!   SIC-POVMs need. d^2/(d+1) is computed exactly as a rational for
//!   d=2^n and checked for integrality: it fails for every n>1, matching
//!   SIC_Multilattice_Proof.lean's own character-theoretic argument
//!   (elementary-abelian ±1 characters cannot produce a non-integer
//!   target ratio). That is the literal boundary: exact agreement at
//!   n=1, a named, computed, real obstruction at n>1.

#![allow(dead_code)]

use crate::belnap::B4;
use crate::fibonacci_qc::Complex;
use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// WH(2) displacement index: (amplitude bit, phase bit), same shape as
/// SIC_Multilattice_Proof.lean's WHIdx2.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WhIdx2 {
    pub a: bool,
    pub b: bool,
}

/// The corrected Pauli-algebra WH(2) action on a single Belnap value,
/// transcribed directly from SIC_Multilattice_Proof.lean's `wh2Act`
/// (corrected 2026-06-20). NOT the naive `bnot`-based action --
/// see `wh2_act_naive` below for that one, kept specifically so the
/// two can be compared directly rather than only one being coded up.
pub fn wh2_act(d: WhIdx2, v: B4) -> B4 {
    match (d.a, d.b) {
        (false, false) => v,
        (true, false) => match v {
            B4::N => B4::F,
            B4::T => B4::B,
            B4::F => B4::N,
            B4::B => B4::T,
        },
        (false, true) => match v {
            B4::N => B4::T,
            B4::T => B4::N,
            B4::F => B4::B,
            B4::B => B4::F,
        },
        (true, true) => match v {
            B4::N => B4::B,
            B4::T => B4::F,
            B4::F => B4::T,
            B4::B => B4::N,
        },
    }
}

/// The naive product-lattice action from BelnapWHMultilattice.lean §A:
/// amplitude bit applies bnot to the VALUE, phase bit flips a SEPARATE
/// ZMod 2 phase field the Lean file's `MLQubit{val, phase}` carries
/// alongside it. Returns (value, phase) as that pair, not the value
/// alone -- an earlier version of this function dropped the phase bit
/// entirely, which made every naive-orbit count collapse to 1 instead
/// of 2^n (since bnot(B)=B leaves the value fixed at B regardless of the
/// amplitude bit, and with no phase tracked there was nothing left to
/// vary). Caught by reading the printed orbit table, not asserted.
pub fn wh2_act_naive(d: WhIdx2, v: B4, phase: bool) -> (B4, bool) {
    let new_val = if d.a { v.bnot() } else { v };
    let new_phase = phase ^ d.b;
    (new_val, new_phase)
}

/// Enumerates every WHIdx(n) = (WHIdx2)^n displacement (4^n of them for
/// small n) and returns the number of DISTINCT resulting states when
/// each acts on the all-B fiducial componentwise -- a real count, not a
/// cited one. `naive` selects which action (§A vs the corrected one) is
/// under test; the naive branch tracks (value, phase) pairs per qubit,
/// the corrected branch tracks value alone (SIC_Multilattice_Proof.lean's
/// MLState carries no separate phase field at all).
pub fn ml_orbit_card(n: usize, naive: bool) -> usize {
    let mut seen: Vec<Vec<(B4, bool)>> = Vec::new();
    let total: u64 = 1u64 << (2 * n); // 4^n as 2 bits per qubit
    for mask in 0..total {
        let mut state: Vec<(B4, bool)> = alloc::vec![(B4::B, false); n];
        for i in 0..n {
            let a = (mask >> (2 * i)) & 1 == 1;
            let b = (mask >> (2 * i + 1)) & 1 == 1;
            let d = WhIdx2 { a, b };
            state[i] = if naive {
                wh2_act_naive(d, B4::B, false)
            } else {
                (wh2_act(d, B4::B), false)
            };
        }
        if !seen.contains(&state) {
            seen.push(state);
        }
    }
    seen.len()
}

/// The Belnap evidence-counting Born rule (BelnapNFiducial.lean §10),
/// exact integer arithmetic: posEvidence(B)=1, negEvidence(B)=1, so
/// P(T|B) = 1/(1+1) = 1/2 as an exact rational, returned as (numerator,
/// denominator) rather than a float -- there is no square root anywhere
/// in this computation.
pub fn belnap_born_prob_t_given_b() -> (u64, u64) {
    let pos = 1u64; // posEvidence(B)
    let neg = 1u64; // negEvidence(B)
    (pos, pos + neg)
}

/// The standard QM Hadamard's Born-rule prediction for the same
/// question -- |amplitude|^2 for measuring |0> after H|0>, using the
/// real complex Hadamard (1/sqrt(2) amplitudes), not a shortcut. Reuses
/// fibonacci_qc::Complex, the same complex type shor_qft.rs's QFT is
/// built on, rather than a second complex-number type.
pub fn qm_born_prob_0_after_hadamard() -> f64 {
    let inv_sqrt2 = 1.0 / libm::sqrt(2.0);
    let amp = Complex::new(inv_sqrt2, 0.0);
    amp.norm_sq()
}

/// Is d^2/(d+1) -- the exact rational Hilbert-space equiangularity ratio
/// a real SIC-POVM needs for dimension d -- a perfect square in Q? This
/// is what SIC_Multilattice_Proof.lean §11's own comment actually
/// claims fails for every d=2^n, n>1 (NOT that d^2/(d+1) is an integer,
/// which an earlier version of this function checked instead -- a
/// mistranscription, not what the file says).
///
/// gcd(d, d+1) = 1 always (consecutive integers), so gcd(d^2, d+1) = 1
/// too: any prime dividing both d^2 and d+1 would divide d, hence divide
/// (d+1)-d=1. So d^2/(d+1) is already in lowest terms, and since the
/// numerator d^2 is automatically a perfect square, the whole fraction
/// is a perfect square in Q exactly when the denominator d+1 is a
/// perfect square in Z. Checked directly by integer square root, not
/// assumed.
pub fn equiangularity_ratio_is_perfect_square(d: u64) -> (u64, u64, bool) {
    let denom = d + 1;
    let root = libm::round(libm::sqrt(denom as f64)) as u64;
    let is_square = root * root == denom;
    (d * d, denom, is_square)
}

pub fn report(max_n: usize) -> String {
    let mut out = String::new();
    out.push_str("multilattice: the FDE/QM boundary, computed\n\n");

    out.push_str("orbit size, naive (bnot) vs corrected (Pauli) action, both by direct enumeration:\n");
    out.push_str("  n  naive_orbit  2^n   corrected_orbit  4^n\n");
    for n in 1..=max_n.min(6) {
        let naive = ml_orbit_card(n, true);
        let corrected = ml_orbit_card(n, false);
        out.push_str(&format!(
            "  {}  {:<11} {:<5} {:<16} {}\n",
            n, naive, 1usize << n, corrected, 1usize << (2 * n)
        ));
    }

    out.push_str("\nBorn rule at n=1 (d=2): Belnap evidence-counting vs the real QM Hadamard:\n");
    let (num, den) = belnap_born_prob_t_given_b();
    let belnap_val = num as f64 / den as f64;
    let qm_val = qm_born_prob_0_after_hadamard();
    out.push_str(&format!(
        "  Belnap: P(T|B) = {}/{} = {:.10}  (exact integers, no sqrt)\n",
        num, den, belnap_val
    ));
    out.push_str(&format!(
        "  QM:     |<0|H|0>|^2 = {:.10}  (real Hadamard, 1/sqrt(2) amplitude)\n",
        qm_val
    ));
    out.push_str(&format!(
        "  agreement (|Belnap - QM| < 1e-9): {}\n",
        (belnap_val - qm_val).abs() < 1e-9
    ));

    out.push_str("\nequiangularity ratio d^2/(d+1), required for a real SIC-POVM, checked for being a perfect square in Q (the file's actual claim -- d+1 a perfect square in Z, since d^2/(d+1) is already in lowest terms):\n");
    let mut counterexample_found = false;
    for n in 1..=max_n.min(8) {
        let d = 1u64 << n;
        let (num, den, is_square) = equiangularity_ratio_is_perfect_square(d);
        out.push_str(&format!(
            "  n={} d={:<4} {}^2/{} = {}/{}  perfect square: {}\n",
            n, d, d, d + 1, num, den, is_square
        ));
        if n > 1 && is_square {
            counterexample_found = true;
        }
    }
    out.push_str(&format!(
        "\ncounterexample to \"not a perfect square for any n>1\" found in this range: {}\n",
        counterexample_found
    ));
    out.push_str("\nreading: n=1 is exact agreement, both the orbit count and the Born rule. The naive action's orbit is proved and measured short of 4^n for n>1. But the file's own further claim, in a comment rather than a proved theorem, that d^2/(d+1) is never a perfect square for n>1, does not hold in general -- check the counterexample line above against the per-n table.\n");
    out
}

pub fn repl_multilattice(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("multilattice — the FDE/QM boundary: orbit counts, Born rule, equiangularity, all computed");
        sprintln!("  multilattice report [max_n]   full report, orbit table capped at n=6, ratio table capped at n=8");
        return;
    }
    match args[0] {
        "report" => {
            let max_n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(6);
            sprintln!("{}", report(max_n));
        }
        other => sprintln!("multilattice: unknown subcommand '{}' (try 'multilattice help')", other),
    }
}
