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
//! at every step. The order search is baby-step giant-step, the conventional
//! decomposition of the ⊡ winding through the ∈/∋ pair: the fork lays the baby
//! table, the giant stride walks, the fuse is their collision, so the order
//! comes back in O(sqrt order) time and memory where the plain walk was
//! O(order). There is no bound on n's size; the reach is the baby table's cap
//! squared.
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

/// The baby table's size, which is the real thing at stake here: baby-step
/// giant-step needs a table of m residues to reach an order of m^2, so the table
/// IS sqrt(order) memory, the algorithm's own cost, not a policy dial. This sets
/// how much of that memory to spend, a few GB at this size, reaching orders near
/// its square. Past it the winding route falls through to the bridge, the
/// squares route, and rho, none of which hold a table. The table-free sqrt-time
/// order search is Pollard's kangaroo; a naive small-stride rho does not do it,
/// it drifts at O(order), not sqrt.
const ORDER_TABLE_CAP: u64 = 20_000_000;

/// The multiplicative order of `a` modulo `n`: the least r > 0 with a^r ≡ 1,
/// found by baby-step giant-step, the conventional decomposition of the ⊡
/// winding holonomy through the ∈/∋ pair. The ∈ fork lays down the baby table
/// a^0, a^1, ..., a^{m-1}; the giant stride walks a^m, a^{2m}, ... and the ∋
/// fuse is the collision a^{im} = a^j, which gives r = im - j. This meets in
/// the middle in O(sqrt r) steps and O(sqrt r) memory, where the plain walk was
/// O(r). Returns None if the order exceeds m^2 for the capped table.
fn order_bsgs(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    let one = BigUint::one();
    let a = a % n;
    if a == one { return Some(one); }

    // m = ceil(sqrt(n)), capped. Order divides λ(n) < n, so m^2 ≥ n covers
    // every order when the cap does not bind.
    let mut m = n.sqrt() + &one;
    let cap = BigUint::from(ORDER_TABLE_CAP);
    if m > cap { m = cap; }
    let m_u64 = m.to_u64_digits().first().copied().unwrap_or(1);

    // ∈ fork: the baby table, a^j keyed by its byte value, smallest j kept.
    // A small order shows here directly as a^j = 1 with j > 0.
    let mut baby: BTreeMap<Vec<u8>, u64> = BTreeMap::new();
    let mut val = one.clone();
    for j in 0..m_u64 {
        if j > 0 && val == one { return Some(BigUint::from(j)); }
        baby.entry(val.to_bytes_le()).or_insert(j);
        val = (&val * &a) % n;
    }
    // val is now a^m, the giant stride.
    let giant_stride = val.clone();
    let mut giant = giant_stride.clone(); // a^{m·1}
    // ∋ fuse: the first giant hit in the baby table, a^{im} = a^j, gives r.
    for i in 1..=m_u64 {
        if let Some(&j) = baby.get(&giant.to_bytes_le()) {
            let e = BigUint::from(i) * &m - BigUint::from(j);
            if e > BigUint::zero() { return Some(e); }
        }
        giant = (&giant * &giant_stride) % n;
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
    o.push_str("  bridge <n> [B]   factor n off a smooth winding-bridge (⊞/⊡, Pollard p-1)\n");
    o.push_str("  squares <n>   factor n off a comparable-size bridge (∈, difference of squares)\n");
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
        let r = match order_bsgs(&a, &nv) {
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
            "  order exceeds the capped baby table (reach ~{}^2); hand to `trilattice_factor factor {}`",
            ORDER_TABLE_CAP, n
        ));
    } else {
        o.push_str(&format!(
            "  no base in the set gave an even winding with a non-trivial split; hand to `trilattice_factor factor {}`",
            n
        ));
    }
    o
}

/// The default smoothness bound for the winding-bridge route.
const BRIDGE_BOUND: u64 = 300_000;

/// The winding-bridge, the ⊞/⊡ decomposition past the order ceiling. When one
/// factor p has a smooth winding, meaning p-1 has only small prime factors, the
/// accumulated winding M = product of e for e up to the bound is a multiple of
/// p-1, so a^M ≡ 1 mod p while a^M is generic mod the other factor. That is the
/// MOAT_BRIDGE_TYPE mismatch: the winding bridges to p and stays across the moat
/// to q. gcd(a^M - 1, n) reads the bridged factor out. This never touches the
/// order's size, so it breaks the sqrt-order ceiling for a smooth-winding
/// factor; it finds nothing when both factors' windings are non-smooth, which
/// is the case an RSA modulus is chosen to be. This is Pollard's p-1.
fn winding_bridge(n: &BigUint, bound: u64) -> Option<BigUint> {
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let mut a = two % n;
    let mut e: u64 = 2;
    // Check the gcd at every step. The first factor whose winding divides the
    // accumulated M sends a to 1 mod that factor while it is still generic mod
    // the other, so gcd(a-1, n) crosses to it. Checking only in coarse batches
    // can let the second factor bridge inside the same batch, collapsing the
    // gcd to n and losing the split; per-step checking catches the first
    // crossing exactly. Cost stays dominated by the modular power, not the gcd.
    while e <= bound {
        a = a.modpow(&BigUint::from(e), n);
        e += 1;
        if a.is_zero() || a == one { break; } // bridged out or collapsed; no readable split
        let g = big_gcd(&a - &one, n.clone());
        if g > one && &g < n { return Some(g); }
    }
    None
}

/// The bridge subcommand: factor n by the winding-bridge (Pollard p-1) at the
/// given smoothness bound, reading the split off the bridged winding.
pub fn bridge(n: &str, bound_opt: Option<u64>) -> String {
    let bound = bound_opt.unwrap_or(BRIDGE_BOUND);
    let nv: BigUint = match n.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("trilattice_factor bridge {}: not a non-negative integer", n),
    };
    let two = BigUint::from(2u32);
    if nv < two {
        return format!("trilattice_factor bridge {}: n < 2, nothing to bridge", n);
    }
    let mut o = format!("trilattice_factor bridge {}:\n", n);
    match winding_bridge(&nv, bound) {
        Some(g) => {
            let cof = &nv / &g;
            let (lo, hi) = if g <= cof { (g, cof) } else { (cof, g) };
            o.push_str(&format!(
                "  ⊞ MOAT_BRIDGE_TYPE: one factor's winding is {}-smooth, the other is not\n",
                bound
            ));
            o.push_str(&format!(
                "  ⊡ WIND_BRIDGE: gcd(a^M - 1, n) crosses to the bridged factor\n"
            ));
            o.push_str(&format!("  {} = {} × {}   read off the winding-bridge", nv, lo, hi));
        }
        None => o.push_str(&format!(
            "  no factor with a {}-smooth winding at this bound; raise the bound or hand to `trilattice_factor factor {}`",
            bound, n
        )),
    }
    o
}

/// The bridge-of-comparable-size, the ∈ BRIDGE_EXIST decomposition. The
/// fragment ∃y∈x(|y|∼|x|) is a factor of the same magnitude as sqrt(n): write
/// n = a^2 - b^2 = (a-b)(a+b), the congruence of squares. Walk a up from
/// ceil(sqrt n) until a^2 - n is itself a square b^2; then a-b and a+b are the
/// two comparable factors. The step count is a - sqrt(n), small when the two
/// factors are close and growing as they spread.
///
/// No arbitrary step cap: the walk self-terminates. Every odd composite n = p·q
/// has a valid a = (p+q)/2, no larger than (n+1)/2, so a walk that reaches
/// (n+1)/2 without a split means n has none, which for an odd number means it is
/// prime. Callers guard with a primality test first, so this only ever runs on a
/// composite and always returns a factor; the natural bound is a safety, not a
/// budget. It is slow when the factors are far apart, which is the case rho
/// handles, so a caller wanting speed on an unbalanced n reaches for `factor`.
fn difference_of_squares(n: &BigUint) -> Option<(BigUint, BigUint)> {
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let limit = (n + &one) / &two; // a > this means no factorization remains
    let mut a = n.sqrt();
    if &a * &a < *n { a += &one; } // ceil(sqrt n)
    while a <= limit {
        let a2 = &a * &a;
        let b2 = &a2 - n;
        let b = b2.sqrt();
        if &b * &b == b2 {
            let lo = &a - &b;
            let hi = &a + &b;
            if lo > one && lo < *n { return Some((lo, hi)); }
        }
        a += &one;
    }
    None
}

/// The squares subcommand: factor n by the difference-of-squares bridge,
/// reading the split off two factors of comparable size (∈ BRIDGE_EXIST).
pub fn squares(n: &str) -> String {
    let nv: BigUint = match n.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("trilattice_factor squares {}: not a non-negative integer", n),
    };
    let two = BigUint::from(2u32);
    if nv < BigUint::from(3u32) {
        return format!("trilattice_factor squares {}: n < 3, nothing to bridge", n);
    }
    if (&nv % &two).is_zero() {
        return format!(
            "trilattice_factor squares {}: {} = 2 × {}   (even; the squares bridge is for odd n)",
            n, nv, &nv / &two
        );
    }
    // The walk self-terminates only at (n+1)/2, which is unreachable for a large
    // prime, so rule primes out first with the real primality test.
    if crate::prime_winding::is_prime(&nv.to_str_radix(10)) == crate::prime_winding::PrimeVerdict::Prime {
        return format!("trilattice_factor squares {}: {} is prime, no difference of squares", n, nv);
    }
    let mut o = format!("trilattice_factor squares {}:\n", n);
    match difference_of_squares(&nv) {
        Some((lo, hi)) => {
            o.push_str("  ∈ BRIDGE_EXIST: a bridge of comparable size, |y| ∼ |x| ∼ √n\n");
            o.push_str("  ⊙ n = a² - b² = (a-b)(a+b), the congruence of squares\n");
            o.push_str(&format!("  {} = {} × {}   read off the difference of squares", nv, lo, hi));
        }
        None => o.push_str(&format!(
            "  no non-trivial difference of squares below (n+1)/2; hand to `trilattice_factor factor {}`",
            n
        )),
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
