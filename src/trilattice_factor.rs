//! Trilattice-native factorization — ob3ect-backed kernel tool.
//!
//! The artifact "Trilattice-native factorization" is a verified ob3ect whose
//! glyph word `⊢≻⊙∈⊤⋈⊥≺⊞⊡∋⋈⊣` grounds each of the twelve marks to a role in
//! factoring a number:
//!
//!   ⊢ VINIT   void_bulk                the empty register before scale-collapse
//!   ≻ AFWD    scale_ascent             one forward step up the number line
//!   ⊙ IMSCRIB critical_phase           the self-reference checkpoint
//!   ∈ FSPLIT  asymmetric_factorization the fork: n splits into two unequal poles
//!   ⊤ EVALT   kinetic_freeze           the frozen pole, held on the fork's T arm
//!   ⋈ CLINK   fidelity_chain           carry
//!   ⊥ EVALF   polarity_interface       the moving pole, held on the fork's F arm
//!   ≺ AREV    chiral_descent           the descent that clears the open register
//!   ⊞ ENGAGR  transition_stabilization the both-state holding the accumulated pair
//!   ⊡ IFIX    cluster_propagation      the winding commit, the one fixation
//!   ∋ FFUSE   winding_reconstitution   the fuse: the two poles rejoin as a factor
//!   ⋈ CLINK   fidelity_chain           carry
//!   ⊣ TANCH   hierarchical_container   the closed boundary
//!
//! What this tool reads off the word, and what it does not.
//!
//! The reading is real and checkable. Run the word as a ring and its landing
//! register is `tf` at every cut but one; the single exception is the cut led
//! by the winding fixation ⊡, which commits to a clean `T`. That single-commit
//! cut is the ob3ect's cluster_propagation event, and it survives the control:
//! scrambling the marks smears the commit across several cuts, so the
//! one-commit-per-winding reading is in the word's arrangement, not the marks.
//! The fork ∈ (asymmetric_factorization) banks its two poles, the T pole
//! (kinetic_freeze) and the F pole (polarity_interface), so they survive the
//! chiral descent ≺ that clears the open register, and fuse at ∋
//! (winding_reconstitution). That is the factor pair held across the descent,
//! read straight from `weight` / `cycle`.
//!
//! The divisor search is not the word. As `prime_winding` establishes at
//! length against concrete counterexamples, closure alone never produces a
//! divisor: a bounded per-mark reading sees a bounded pattern in the digits,
//! and a factor is a global fact about divisibility. So the actual factoring
//! here IS `prime_winding`'s search, called directly, never re-copied. This
//! module carries the trilattice reading of the number and of the word; the
//! arithmetic that splits n lives one place, next door.
//!
//! The winding route closes part of that gap. `winding` reads the factor off a
//! real winding: the multiplicative order r of a base a mod n, the ROTAT period
//! of the ring a generates, from which gcd(a^(r/2) ± 1, n) splits n. That is
//! Shor's classical core, native to the SIC-phase structure this codebase runs
//! on (belnap_phase_shor), and it is the factor read from a winding rather than
//! from a rho search. It runs at arbitrary precision, carrying the full residue
//! at every step; there is no bound on n's size, only a step budget on the
//! order search, since finding an order classically is an O(order) walk.
//!
//! The number and the winding both enter the Grammar's own marks now. `read`
//! shows n's native parity-graded word (Native IMASM-Numeral Mapping), the
//! bijective encoding, beside the hex one; and when the winding route finds an
//! order r, it records r as its own native numeral, the ⊡ immutable_record of
//! the winding value.
//!
//! The rung still standing, and which method bounds it: reading the order r off
//! the register of n's word alone. Parity, the Z2 grade the native word
//! carries, does not determine the order: many powers a^k mod n share a parity,
//! so the parity sequence closes on a shorter cycle than r. The order is a fact
//! about the full residue, not its parity bit. So the register of the parity
//! word cannot report r by itself; a mapping carrying the full residue, not
//! only its Z2 grade, is the method to build next. Named, not walled.
//!
//! Subcommands:
//!   trilattice_factor word            the canonical glyph word and its marks
//!   trilattice_factor cert            the winding-commit certificate of the word
//!   trilattice_factor read <n>        the trilattice reading of n's digit word
//!   trilattice_factor winding <n> [a] factor n off a multiplicative-order winding
//!   trilattice_factor factor <n>      factor n (arbitrary precision) with the reading
//!   trilattice_factor help            list subcommands

use alloc::string::String;
use alloc::format;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use imasm_core::lattice_flow::cycle_landings;
use crate::prime_winding::{digit_encode, factor_bounded, big_gcd};
use crate::native_numeral::encode as native_encode;

/// Bases tried for the order winding, small units first. Any base sharing a
/// factor with n is a collision that hands the factor over directly; the
/// others are asked for their multiplicative order.
const WINDING_BASES: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

/// How many multiplication steps the order search spends per base before it
/// gives the base up. The multiplicative order can be as large as n, and
/// finding it classically is a real O(order) walk, so a walk this long without
/// closing hands the number to the rho search rather than run without bound.
/// This is the cost ceiling of the native route, not a limit on n's size.
const ORDER_STEP_BUDGET: u64 = 40_000_000;

/// The multiplicative order of `a` modulo `n` at arbitrary precision: the least
/// r > 0 with a^r ≡ 1, walked in BigUint so no product wraps, or None if it
/// does not close within `budget` steps. This is the winding of the ring `a`
/// generates, the ROTAT period the factor is read off, carrying the full
/// residue at every step, not only its parity.
fn order_big(a: &BigUint, n: &BigUint, budget: u64) -> Option<BigUint> {
    let one = BigUint::one();
    let mut val = one.clone() % n;
    let mut r: u64 = 0;
    while r < budget {
        val = (&val * a) % n;
        r += 1;
        if val == one { return Some(BigUint::from(r)); }
    }
    None
}

/// The ob3ect's own fixed reference word.
pub const WORD: &str = "⊢≻⊙∈⊤⋈⊥≺⊞⊡∋⋈⊣";
pub const PERIOD: usize = 13;
pub const ARTIFACT: &str = "Trilattice-native factorization";
/// The ∈/∋ pair the ob3ect reports: the fork and fuse that carry the split.
pub const FORK_FUSE_PAIR: (usize, usize) = (3, 10);
/// The mark whose cut commits the register to a clean T: ⊡ IFIX, the winding.
pub const COMMIT_MARK: char = '⊡';

/// The cuts at which the ring's landing register is a clean `T`, read live
/// from `cycle_landings`. For the canonical word this is the single cut led
/// by the winding fixation ⊡; returning the list, not a hardcoded index,
/// keeps the certificate a measurement rather than a memory.
fn commit_cuts(word: &str) -> alloc::vec::Vec<usize> {
    match cycle_landings(word) {
        Some(landings) => landings
            .iter()
            .enumerate()
            .filter(|(_, r)| r.as_str() == "T")
            .map(|(k, _)| k)
            .collect(),
        None => alloc::vec::Vec::new(),
    }
}

/// The glyph the cut k opens on, for naming which mark commits.
fn mark_at(word: &str, k: usize) -> Option<char> {
    let chars: alloc::vec::Vec<char> = word.chars().collect();
    if chars.is_empty() { return None; }
    Some(chars[k % chars.len()])
}

pub fn help() -> String {
    let mut o = String::new();
    o.push_str("trilattice_factor — ob3ect-backed factorization reading\n");
    o.push_str("  word          the canonical glyph word and its marks\n");
    o.push_str("  cert          the winding-commit certificate of the word\n");
    o.push_str("  read <n>      the trilattice reading of n's digit word\n");
    o.push_str("  winding <n> [a]  factor n off a multiplicative-order winding (Shor's core)\n");
    o.push_str("  factor <n>    factor n (arbitrary precision) with the reading\n");
    o.push_str("  help          this list");
    o
}

pub fn word() -> String {
    format!(
        "{}\n  word   : {}   period {}\n  fork/fuse pair (asymmetric_factorization / winding_reconstitution): {:?}\n  commit mark (cluster_propagation): {}",
        ARTIFACT, WORD, PERIOD, FORK_FUSE_PAIR, COMMIT_MARK
    )
}

/// The certificate the tool exists to speak: the word commits at exactly the
/// winding fixation and nowhere else, read from the live orbit.
pub fn cert() -> String {
    let mut o = String::new();
    o.push_str(&format!("word   : {}   period {}\n", WORD, PERIOD));
    let cuts = commit_cuts(WORD);
    if cuts.is_empty() {
        o.push_str("  no clean-T commit cut on this word\n");
        return o;
    }
    o.push_str("  winding-commit cut(s) (register lands clean T):\n");
    for k in cuts.iter() {
        match mark_at(WORD, *k) {
            Some(g) => o.push_str(&format!("    k = {:>2}   led by {}\n", k, g)),
            None => o.push_str(&format!("    k = {:>2}\n", k)),
        }
    }
    let single_at_commit = cuts.len() == 1 && mark_at(WORD, cuts[0]) == Some(COMMIT_MARK);
    if single_at_commit {
        o.push_str("  ONE commit, at the winding fixation ⊡ — the cluster_propagation event.\n");
        o.push_str("  Every other cut lands tf: the two poles held, not yet committed.");
    } else {
        o.push_str("  commit is not isolated to ⊡ on this word.");
    }
    o
}

/// The trilattice reading of the number itself: its digit word and where that
/// word comes to rest. The number enters the Grammar the same way
/// `prime_winding` reads it, by digit encoding; this is that reading, not a
/// factoring claim.
pub fn read(n: &str) -> String {
    let dw = digit_encode(n);
    if dw.is_empty() {
        return format!("trilattice_factor read {}: not a valid non-negative integer", n);
    }
    let landings = cycle_landings(&dw);
    let mut o = format!("trilattice_factor read {}:\n", n);
    o.push_str(&format!("  hex-digit word   : {}\n", dw));
    match landings {
        Some(l) => {
            let commits = l.iter().filter(|r| r.as_str() == "T").count();
            o.push_str(&format!(
                "  period {}, {} winding-commit cut(s) over the orbit\n",
                l.len(), commits
            ));
        }
        None => o.push_str("  no IMASM glyphs in the digit word\n"),
    }
    // The native parity-graded word: bijective with n, where the hex word is
    // not. This is the mapping the Native IMASM-Numeral Mapping ob3ect gives.
    let nw = native_encode(n);
    o.push_str(&format!("  native word      : {}", nw));
    o
}

/// The winding route: read the factor straight off a winding, the way the ⊡
/// commit names, at arbitrary precision. For a base a coprime to n, the
/// multiplicative order r is the period of the ring a generates under
/// multiplication mod n, the ROTAT period, carrying the full residue at every
/// step. When r is even and a^(r/2) is not -1, gcd(a^(r/2) ± 1, n) splits n.
/// This is Shor's classical core, native to the SIC-phase structure, the factor
/// read from the winding rather than from a rho search. There is no size bound
/// on n; the only ceiling is how many steps the order search spends per base
/// (`ORDER_STEP_BUDGET`), since finding an order classically is an O(order)
/// walk. A base whose order does not close in budget is given up, and if no
/// base closes, the number is handed to the rho search, which reaches factors
/// without needing the order at all.
pub fn winding(n: &str, a_opt: Option<u64>) -> String {
    let t = n.trim();
    let nv: BigUint = match t.parse() {
        Ok(v) => v,
        Err(_) => return format!("trilattice_factor winding {}: not a non-negative integer", n),
    };
    let two = BigUint::from(2u32);
    if nv < two {
        return format!("trilattice_factor winding {}: n < 2, no winding", n);
    }
    if (&nv % &two).is_zero() {
        return format!(
            "trilattice_factor winding {}: {} = 2 × {}   (even, split before any winding)",
            n, nv, &nv / &two
        );
    }

    let bases: alloc::vec::Vec<BigUint> = match a_opt {
        Some(a) => alloc::vec![BigUint::from(a)],
        None => WINDING_BASES.iter().map(|&a| BigUint::from(a)).collect(),
    };

    let one = BigUint::one();
    let mut o = format!("trilattice_factor winding {}:\n", n);
    let mut budget_hit = false;
    for a in bases.iter() {
        let a = a % &nv;
        if a < two { continue; }
        let g = big_gcd(a.clone(), nv.clone());
        if g > one {
            o.push_str(&format!(
                "  base {} shares a factor: gcd = {} → {} = {} × {}",
                a, g, nv, g, &nv / &g
            ));
            return o;
        }
        let r = match order_big(&a, &nv, ORDER_STEP_BUDGET) {
            Some(r) => r,
            None => { budget_hit = true; continue; }
        };
        // Even order with a^(r/2) not -1 gives the split.
        if (&r % &two).is_zero() {
            let half = &r / &two;
            let a_half = a.modpow(&half, &nv);
            if a_half != &nv - &one {
                let p = big_gcd(
                    if a_half > BigUint::zero() { &a_half - &one } else { &nv - &one },
                    nv.clone(),
                );
                let q = big_gcd(&a_half + &one, nv.clone());
                let nontrivial = p > one && q > one && p < nv && q < nv;
                if nontrivial {
                    o.push_str(&format!(
                        "  base {}: winding r = {} (the ROTAT period of {} mod {})\n",
                        a, r, a, nv
                    ));
                    o.push_str(&format!(
                        "  winding fixed as its native numeral (⊡ immutable_record): {}\n",
                        native_encode(&r.to_str_radix(10))
                    ));
                    o.push_str(&format!(
                        "  ⊡ ZWIND holonomy: ∮ A = 2π·{}, the integer winding of the orbit loop\n",
                        r
                    ));
                    let (lo, hi) = if p <= q { (p, q) } else { (q, p) };
                    o.push_str(&format!(
                        "  a^(r/2) ± 1 splits it: {} = {} × {}   read off the winding, no rho search",
                        nv, lo, hi
                    ));
                    return o;
                }
            }
        }
    }
    if budget_hit {
        o.push_str(&format!(
            "  order search hit its step budget before closing; hand to `trilattice_factor factor {}`",
            n
        ));
    } else {
        o.push_str(&format!(
            "  no base in the set gave an even winding with a non-trivial split; hand to `trilattice_factor factor {}`",
            n
        ));
    }
    o
}

/// Factor n, leading with the trilattice reading and then the real divisor
/// search from `prime_winding` (one copy of the arithmetic, called here).
pub fn factor(n: &str, max_power: Option<u64>) -> String {
    let mut o = String::new();
    o.push_str(&read(n));
    o.push('\n');
    o.push_str(&factor_bounded(n, max_power));
    o
}
