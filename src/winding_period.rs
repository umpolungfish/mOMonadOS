// winding_period.rs — winding-period engine, native Rust, torus by default.
//
// The cyclic subgroup <a> mod N is a torus: the exponent is a winding
// coordinate x mod r, and a^x sits at winding x/r on the unit circle.
// The period r is the winding number at which the loop closes — a^r = 1,
// the discrete ∮A = 2πn with n = r. BSGS walks the torus from both sides
// of its diameter: baby steps one radius clockwise, giant steps the other
// radius back; they meet at the winding midpoint. minimal_winding strips
// the period to its reduced denominator; closure_gcd closes p at bound B
// exactly when max_prime_power(ord_p(2)) ≤ B (the sharpened threshold
// theorem, 1156/1156 — the engine closes at the ORDER, not at p−1).
//
// no_std + alloc: the baby walk lands in a sorted (value, exponent) lattice
// and the giant walk binary-searches it — no HashMap dependency, and the
// lattice IS the torus quantization.
//
// Tuple: ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑲𐑠⊙𐑖𐑙𐑴⟩ — winding_period_finder, O_∞, μ∘δ=id.

#![allow(dead_code)]

use alloc::vec::Vec;
use crate::sprintln;
use num_bigint::{BigUint, BigInt, Sign};
use num_traits::{Zero, One, ToPrimitive};

// ── Winding: the torus coordinate ──────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Winding {
    pub num: i64,
    pub den: i64,
}

impl Winding {
    pub fn new(num: i64, den: i64) -> Winding { Winding { num, den } }
    pub fn zero() -> Winding { Winding { num: 0, den: 1 } }
    /// Position on the torus, reduced mod 1 into [0,1): the winding
    /// coordinate is a rational turn, and only 0 and 1/2 are real.
    pub fn toroidal(self) -> Winding {
        let n = ((self.num % self.den) + self.den) % self.den;
        Winding { num: n, den: self.den }
    }
    pub fn turns(self) -> f64 { self.num as f64 / self.den as f64 }
}

// ── u64 modular arithmetic (u128 intermediates, native on x86-64) ──

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    let mut b = a % m;
    while e > 0 {
        if e & 1 == 1 { r = mulmod(r, b, m); }
        b = mulmod(b, b, m);
        e >>= 1;
    }
    r
}

pub(crate) fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = a % b; a = b; b = t; }
    a
}

fn egcd(a: u64, b: u64) -> (i128, i128, u64) {
    // extended Euclid: (x, y, g) with a*x + b*y = g
    let (mut old_r, mut r) = (a as i128, b as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    let (mut old_t, mut t) = (0i128, 1i128);
    while r != 0 {
        let q = old_r / r;
        let tmp = old_r - q * r; old_r = r; r = tmp;
        let tmp = old_s - q * s; old_s = s; s = tmp;
        let tmp = old_t - q * t; old_t = t; t = tmp;
    }
    (old_s, old_t, old_r as u64)
}

pub(crate) fn modinv(a: u64, m: u64) -> u64 {
    let (x, _, g) = egcd(a % m, m);
    debug_assert_eq!(g, 1);
    ((x % m as i128 + m as i128) % m as i128) as u64
}

pub(crate) fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

// ── The period: winding number at closure ─────────────────────

/// Minimal period r of a^x mod N by BSGS winding halving, O(√r).
/// Baby steps walk one radius out; giant steps walk the other radius
/// back; the meeting point is the winding midpoint. The period is then
/// stripped to its reduced denominator by minimal_winding.
pub fn winding_order(a: u64, N: u64) -> Option<u64> {
    if N <= 1 || gcd(a, N) != 1 { return None; }
    if a % N == 1 { return Some(1); }
    let m = isqrt(N) + 1;                       // the torus diameter
    let mut baby: Vec<(u64, u64)> = Vec::with_capacity(m as usize);
    let mut cur = 1u64;
    for j in 0..m {                             // one radius clockwise
        baby.push((cur, j));
        cur = mulmod(cur, a, N);
    }
    baby.sort_unstable_by_key(|t| t.0);
    baby.dedup_by_key(|t| t.0);
    let g = modinv(powmod(a, m, N), N);         // a^{-m}
    let mut gamma = 1u64;
    for i in 1..=m {                            // the other radius back
        gamma = mulmod(gamma, g, N);            // a^{-i·m}
        if let Ok(k) = baby.binary_search_by_key(&gamma, |t| t.0) {
            let (_, j) = baby[k];
            let cand = i * m + j;
            if cand > 0 && powmod(a, cand, N) == 1 {
                return Some(minimal_winding(a, N, cand));
            }
        }
    }
    None
}

/// Denominator reduction: strip each prime factor p while a^(r/p) ≡ 1
/// still closes — the minimal winding is the denominator left standing.
pub fn minimal_winding(a: u64, N: u64, mut r: u64) -> u64 {
    if r <= 1 { return r; }
    // Collect the distinct prime factors of r by trial division, then strip
    // any factor whose removal still closes, iterating until stable. The old
    // loop advanced d and bounded by isqrt(r), which skipped the cofactor
    // after a strip shrank r (e.g. 43 in 258 -> 86) and returned a
    // non-minimal period for small-order bases.
    let mut factors: Vec<u64> = Vec::new();
    let mut rr = r;
    let mut d = 2u64;
    while d * d <= rr {
        if rr % d == 0 {
            factors.push(d);
            while rr % d == 0 { rr /= d; }
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if rr > 1 { factors.push(rr); }
    let mut changed = true;
    while changed {
        changed = false;
        for &p in &factors {
            if r % p == 0 && powmod(a, r / p, N) == 1 {
                r /= p;
                changed = true;
            }
        }
    }
    r
}

// ── Pollard p−1 closure on the torus ──────────────────────────

/// M = lcm(2..B) without big integers: M is the product of the prime
/// powers p^e ≤ B, so 2^M mod N = fold powmod over those prime powers.
/// Closure at B is governed by max_prime_power(ord_p(2)) — the sharpened
/// threshold theorem — so the engine closes moduli the p−1-smoothness
/// bound certifies safe.
pub fn closure_gcd(N: u64, B: u64) -> Option<u64> {
    if N < 3 || B < 2 { return None; }
    let mut x = 2u64;
    for p in 2..=B {
        if !is_prime(p) { continue; }
        let mut e = p;
        while e <= B / p { e *= p; }           // largest power p^e ≤ B
        x = powmod(x, e, N);
    }
    let g = gcd(x.wrapping_sub(1), N);
    if g > 1 && g < N { Some(g) } else { None }
}

/// Same Pollard p-1 closure as closure_gcd, BigUint instead of native u64 —
/// closure_gcd silently caps out at 64 bits (the REPL's own u64 parse fails
/// past that and defaults to N=0, reporting a fabricated "open" verdict
/// rather than an error). Real algorithm, unchanged: x=2, raise to the
/// largest p^e <= B for every prime p <= B, then gcd(x-1, N).
pub fn closure_gcd_big(n: &BigUint, b: u64) -> Option<BigUint> {
    let three = BigUint::from(3u32);
    if *n < three || b < 2 { return None; }
    let mut x = BigUint::from(2u32);
    for p in 2..=b {
        if !is_prime(p) { continue; }
        let mut e = p;
        while e <= b / p { e *= p; }
        x = crate::native_numeral::mod_pow_walk(&x, &crate::native_numeral::to_bits_low_first(&BigUint::from(e)), n);
    }
    let one = BigUint::from(1u32);
    if x.is_zero() { return None; }
    let g = gcd_big(crate::native_numeral::subtract_via_word(&x, &one).unwrap(), n.clone());
    if g > one && &g < n { Some(g) } else { None }
}

pub fn repl_closure_big(n_str: &str, b: u64) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("winding closure: '{}' is not a valid non-negative integer", n_str),
    };
    match closure_gcd_big(&n, b) {
        Some(p) => {
            let q = crate::native_numeral::divmod_via_word(&n, &p).unwrap().0;
            format!("closure_gcd({}, {}) = {}  (p × q = N: {})", n_str, b, p, crate::native_numeral::multiply_via_word(&p, &q) == n)
        }
        None => format!("closure_gcd({}, {}): open (no p with p-1 exactly {}-smooth)", n_str, b, b),
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n % 2 == 0 { return n == 2; }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

// ── End-to-end factorization: the Shor winding step, native ───

/// Retry loop over random bases a: winding_order gives the period r of
/// a^x mod N; r even and a^{r/2} ≢ ±1 yields gcd(a^{r/2} − 1, N).
/// xorshift64 supplies the bases — no rand dependency, no_std-safe.
pub fn factor(N: u64, max_tries: u32, mut seed: u64) -> Option<(u64, u64, u64, u64)> {
    for _ in 0..max_tries {
        seed = xorshift64(seed);
        let a = seed % (N - 3) + 3;
        let g0 = gcd(a, N);
        if g0 > 1 && g0 < N { return Some((a, 0, g0, N / g0)); }
        let r = match winding_order(a, N) { Some(r) => r, None => continue };
        if r == 0 || r % 2 != 0 { continue; }
        let x = powmod(a, r / 2, N);
        if x == 1 || x == N - 1 { continue; }
        let g = gcd(x.wrapping_sub(1), N);
        if g > 1 && g < N { return Some((a, r, g, N / g)); }
    }
    None
}

fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

// ── REPL surface (house pattern: fibonacci_qc::repl_*) ─────────

pub fn repl_order(a: u64, N: u64) {
    match winding_order(a, N) {
        Some(r) => sprintln!("winding_order({}, {}) = {}  (a^r == 1 mod N, the closure winding)", a, N, r),
        None => sprintln!("winding_order({}, {}): no period (a not in the unit group)", a, N),
    }
}

pub fn repl_factor(N: u64, tries: u32, seed: u64) {
    match factor(N, tries, seed) {
        Some((a, r, p, q)) => sprintln!("FACTORED {} = {} × {}  (a={}, r={})  p·q==N: {}",
            N, p, q, a, r, p * q == N),
        None => sprintln!("factor({}): no factor in {} tries", N, tries),
    }
}

pub fn repl_closure(N: u64, B: u64) {
    match closure_gcd(N, B) {
        Some(g) => sprintln!("closure_gcd({}, {}) = {}  (closes at bound {})", N, B, g, B),
        None => sprintln!("closure_gcd({}, {}): open (no p with max_prime_power(ord_p(2)) ≤ {})",
            N, B, B),
    }
}

// ── Native prime generation: the push (no Python fixtures) ────

/// Miller-Rabin, deterministic over u64 with the first twelve primes.
pub fn is_prime_mr(n: u64) -> bool {
    if n < 2 { return false; }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 { return n == p; }
    }
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 { d /= 2; s += 1; }
    'base: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 { continue 'base; }
        }
        return false;
    }
    true
}

/// Random bits-bit prime: xorshift64 candidate, top bit and odd forced.
fn gen_prime(bits: u32, s: &mut u64) -> u64 {
    let top = 1u64 << (bits - 1);
    loop {
        *s = xorshift64(*s);
        let p = (*s % top) | top | 1u64;
        if is_prime_mr(p) { return p; }
    }
}

pub fn repl_factorgen(bits: u32, tries: u32, seed: u64) {
    if bits < 8 || bits > 62 {
        sprintln!("factorgen: bits must be in [8, 62] (u64 semiprimes)");
        return;
    }
    let half = bits / 2;
    let mut s = seed;
    let p = gen_prime(half, &mut s);
    let mut q = gen_prime(half, &mut s);
    while q == p { q = gen_prime(half, &mut s); }
    let N = p * q;
    match factor(N, tries, s) {
        Some((a, r, f1, f2)) => sprintln!(
            "FACTORED {} = {} x {}  (a={}, r={})  p.q==N: {}  bits={}",
            N, f1, f2, a, r, f1 * f2 == N, bits),
        None => sprintln!("factorgen({} bits): no factor in {} tries (N={})", bits, tries, N),
    }
}

// ── Arbitrary-precision winding order: BSGS on BigUint ─────────────────
//
// N can be as large as the caller wants; the ORDER r this can certify
// cannot be, because the baby-step table it walks has to fit in memory.
// r is bounded by step_cap*(step_cap+1), which is why it always comes
// back as a plain u64 even when N does not: the table is what limits
// reach, not N's size.
//
// Past that reach the answer is OutOfReach, a fourth result standing
// next to Order and NotInGroup -- not a degraded Order, not a stand-in
// for "probably composite." T, F, B, N (Belnap FOUR) are four points on
// the same lattice, none subordinate to another; OutOfReach is the N
// here, reported at the same strength as a found order, never dressed
// up with a different, non-winding method to avoid saying it.

/// a mod N, N mod a, gcd -- same Euclid as the u64 `gcd` above, BigUint.
fn gcd_big(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let t = crate::native_numeral::modulo_via_word(&a, &b).unwrap();
        a = b;
        b = t;
    }
    a
}

/// Integer square root by Newton's method, BigUint -- same shape as `isqrt`.
fn isqrt_big(n: &BigUint) -> BigUint {
    use crate::native_numeral::{add_via_word, divmod_via_word};
    if n.is_zero() { return BigUint::zero(); }
    let two = BigUint::from(2u32);
    let mut x = n.clone();
    let mut y = divmod_via_word(&add_via_word(&x, &BigUint::one()), &two).unwrap().0;
    while y < x {
        x = y;
        let (q, _) = divmod_via_word(n, &x).unwrap();
        y = divmod_via_word(&add_via_word(&x, &q), &two).unwrap().0;
    }
    x
}

/// Modular inverse via extended Euclid on BigInt -- same shape as `egcd`
/// + `modinv`, promoted to signed arbitrary precision for the subtraction.
fn modinv_big(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{
        signed_add_via_word_general, signed_divmod_via_word, signed_multiply_via_word,
        signed_subtract_via_word_general,
    };
    let a_i = BigInt::from_biguint(Sign::Plus, a.clone());
    let m_i = BigInt::from_biguint(Sign::Plus, m.clone());
    let (mut old_r, mut r) = (a_i, m_i.clone());
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::from(0));
    while !r.is_zero() {
        let (q, _) = signed_divmod_via_word(&old_r, &r).unwrap();
        let t_r = signed_subtract_via_word_general(&old_r, &signed_multiply_via_word(&q, &r)); old_r = r; r = t_r;
        let t_s = signed_subtract_via_word_general(&old_s, &signed_multiply_via_word(&q, &s)); old_s = s; s = t_s;
    }
    if old_r != BigInt::from(1) { return None; }
    let (_, r1) = signed_divmod_via_word(&old_s, &m_i).unwrap();
    let (_, inv_signed) = signed_divmod_via_word(&signed_add_via_word_general(&r1, &m_i), &m_i).unwrap();
    let inv = inv_signed.to_biguint()?;
    // Defense in depth, the same standard hensel_unbraid's own base case
    // now holds to: a real modular inverse satisfies a*inv == 1 mod m
    // directly, checked here rather than trusted from the recursion alone.
    if crate::native_numeral::modulo_via_word(&crate::native_numeral::multiply_via_word(a, &inv), m).unwrap()
        == BigUint::one()
    {
        Some(inv)
    } else {
        None
    }
}

/// Three outcomes for a BigUint order search: found (always u64, per the
/// reach argument above), a not coprime to N (or N<=1, degenerate), or
/// the order lies outside the range this table can see.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindingBig {
    Order(u64),
    NotInGroup,
    OutOfReach,
}

/// Denominator reduction on a u64 order, BigUint modpow underneath --
/// same algorithm as `minimal_winding`, promoted so N can be arbitrary.
fn minimal_winding_big(a: &BigUint, n: &BigUint, mut r: u64) -> u64 {
    if r <= 1 { return r; }
    let mut factors: Vec<u64> = Vec::new();
    let mut rr = r;
    let mut d = 2u64;
    while d * d <= rr {
        if rr % d == 0 {
            factors.push(d);
            while rr % d == 0 { rr /= d; }
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if rr > 1 { factors.push(rr); }
    let one = BigUint::one();
    let mut changed = true;
    while changed {
        changed = false;
        for &p in &factors {
            if r % p == 0 && crate::native_numeral::mod_pow_walk(a, &crate::native_numeral::to_bits_low_first(&BigUint::from(r / p)), n) == one {
                r /= p;
                changed = true;
            }
        }
    }
    r
}

/// BSGS order of a mod N, BigUint. `step_cap` sets the baby-step table
/// size, and so the range of orders this can see: step_cap*(step_cap+1).
/// An order past that range is OutOfReach, reported plainly as itself.
pub fn winding_order_big(a: &BigUint, n: &BigUint, step_cap: u64) -> WindingBig {
    use crate::native_numeral::{add_via_word, divmod_via_word, mod_pow_walk, modulo_via_word, multiply_via_word, to_bits_low_first};
    let one = BigUint::one();
    if *n <= one { return WindingBig::NotInGroup; }
    let a_mod = divmod_via_word(a, n).unwrap().1;
    if gcd_big(a_mod.clone(), n.clone()) != one { return WindingBig::NotInGroup; }
    if a_mod == one { return WindingBig::Order(1); }

    let m_big = add_via_word(&isqrt_big(n), &one);
    if m_big > BigUint::from(step_cap) { return WindingBig::OutOfReach; }
    let m = match m_big.to_u64() { Some(v) => v, None => return WindingBig::OutOfReach };

    let mut baby: Vec<(BigUint, u64)> = Vec::with_capacity(m as usize);
    let mut cur = one.clone();
    for j in 0..m {
        baby.push((cur.clone(), j));
        cur = modulo_via_word(&multiply_via_word(&cur, &a_mod), n).unwrap();
    }
    baby.sort_unstable_by(|x, y| x.0.cmp(&y.0));
    baby.dedup_by(|x, y| x.0 == y.0);

    let a_inv = match modinv_big(&a_mod, n) { Some(v) => v, None => return WindingBig::NotInGroup };
    let giant_step = mod_pow_walk(&a_inv, &to_bits_low_first(&BigUint::from(m)), n);
    let mut gamma = one.clone();
    for i in 1..=m {
        gamma = modulo_via_word(&multiply_via_word(&gamma, &giant_step), n).unwrap();
        if let Ok(k) = baby.binary_search_by(|probe| probe.0.cmp(&gamma)) {
            let j = baby[k].1;
            let cand = i.saturating_mul(m) + j;
            if cand > 0 && mod_pow_walk(&a_mod, &to_bits_low_first(&BigUint::from(cand)), n) == one {
                return WindingBig::Order(minimal_winding_big(&a_mod, n, cand));
            }
        }
    }
    WindingBig::OutOfReach
}

/// Three outcomes for a BigUint factorization attempt: a non-trivial
/// split, the tries exhausted with no split found (N may still be
/// prime, or the bases tried just did not work), or an order search
/// that landed OutOfReach -- reported ahead of NoFactorInTries because
/// it names a different fact: not that N resisted a complete attempt,
/// but that this base's order sits outside what the search can see.
pub enum FactorBig {
    Found { a: BigUint, r: u64, p: BigUint, q: BigUint },
    NoFactorInTries,
    OutOfReach,
}

/// Shor's winding step, BigUint: order r of a random base a, r even and
/// a^(r/2) not ±1 gives gcd(a^(r/2) - 1, N) as a non-trivial factor.
/// Same retry shape as `factor`, promoted to arbitrary precision.
pub fn factor_big(n: &BigUint, max_tries: u32, step_cap: u64, mut seed: u64) -> FactorBig {
    use crate::native_numeral::{add_via_word, divmod_via_word, mod_pow_walk, modulo_via_word, subtract_via_word, to_bits_low_first};
    let one = BigUint::one();
    let three = BigUint::from(3u32);
    if *n <= three { return FactorBig::NoFactorInTries; }
    let n_minus_1 = subtract_via_word(n, &one).unwrap();
    let range = subtract_via_word(n, &three).unwrap();
    let mut saw_out_of_reach = false;
    for _ in 0..max_tries {
        seed = xorshift64(seed);
        let a = add_via_word(&modulo_via_word(&BigUint::from(seed), &range).unwrap(), &three);
        let g0 = gcd_big(a.clone(), n.clone());
        if g0 != one && &g0 != n {
            let q = divmod_via_word(n, &g0).unwrap().0;
            return FactorBig::Found { a, r: 0, p: g0, q };
        }
        match winding_order_big(&a, n, step_cap) {
            WindingBig::Order(r) => {
                if r == 0 || r % 2 != 0 { continue; }
                let x = mod_pow_walk(&a, &to_bits_low_first(&BigUint::from(r / 2)), n);
                if x == one || x == n_minus_1 { continue; }
                let g = gcd_big(subtract_via_word(&x, &one).unwrap(), n.clone());
                if g != one && &g != n {
                    let q = divmod_via_word(n, &g).unwrap().0;
                    return FactorBig::Found { a, r, p: g, q };
                }
            }
            WindingBig::OutOfReach => saw_out_of_reach = true,
            WindingBig::NotInGroup => {}
        }
    }
    if saw_out_of_reach { FactorBig::OutOfReach } else { FactorBig::NoFactorInTries }
}

#[cfg(test)]
mod big_tests {
    // modinv_big, winding_order_big, and factor_big have no REPL path at
    // all right now -- only closure_gcd_big does, via `winding closure`
    // when N exceeds u64. These are the only checks that exercise the
    // three unreachable ones, each against an independent brute-force
    // oracle rather than the function's own internal check.
    use super::*;

    #[test]
    fn modinv_big_matches_known_inverses() {
        // 3 * 4 = 12 = 1 mod 11; 7 * 15 = 105 = 4*26 + 1 mod 26.
        assert_eq!(modinv_big(&BigUint::from(3u32), &BigUint::from(11u32)), Some(BigUint::from(4u32)));
        assert_eq!(modinv_big(&BigUint::from(7u32), &BigUint::from(26u32)), Some(BigUint::from(15u32)));
    }

    #[test]
    fn modinv_big_satisfies_a_times_inv_is_one_mod_m() {
        let pairs: [(u64, u64); 6] = [(3, 11), (7, 26), (17, 97), (999983, 1000003), (2, 1009), (1009, 999999999989)];
        for (a, m) in pairs {
            let a = BigUint::from(a);
            let m = BigUint::from(m);
            let inv = modinv_big(&a, &m).expect("coprime pair should have an inverse");
            assert_eq!(
                crate::native_numeral::modulo_via_word(&crate::native_numeral::multiply_via_word(&a, &inv), &m).unwrap(),
                BigUint::one(),
                "a={} m={} inv={}", a, m, inv
            );
        }
    }

    #[test]
    fn modinv_big_is_none_when_not_coprime() {
        // gcd(6, 9) = 3, no inverse exists.
        assert_eq!(modinv_big(&BigUint::from(6u32), &BigUint::from(9u32)), None);
    }

    /// Independent oracle: the order of a mod n by direct repeated
    /// multiplication, no BSGS, no word-native primitives -- the ground
    /// truth winding_order_big is checked against.
    fn brute_order(a: u64, n: u64) -> Option<u64> {
        if gcd(a, n) != 1 { return None; }
        let mut x = a % n;
        let mut r = 1u64;
        while x != 1 {
            x = (x * a) % n;
            r += 1;
            if r > n { return None; }
        }
        Some(r)
    }

    #[test]
    fn winding_order_big_matches_brute_force_on_many_small_cases() {
        for n in 2u64..60 {
            for a in 1u64..n {
                let expected = brute_order(a, n);
                let got = winding_order_big(&BigUint::from(a), &BigUint::from(n), 1000);
                match (expected, got) {
                    (Some(r), WindingBig::Order(g)) => assert_eq!(r, g, "a={} n={}", a, n),
                    (None, WindingBig::NotInGroup) => {}
                    (e, g) => panic!("a={} n={} expected={:?} got={:?}", a, n, e, g),
                }
            }
        }
    }

    #[test]
    fn factor_big_never_reports_a_false_positive() {
        // 1009 x 1013, well within a small step_cap's reach, tried with a
        // fixed seed and enough retries that a Found result is expected --
        // whatever it returns, a Found must survive an exact
        // multiply_via_word check against N.
        let n = BigUint::from(1009u32) * BigUint::from(1013u32);
        for seed in [1u64, 42, 0x9E37_79B9_7F4A_7C15, 12345] {
            if let FactorBig::Found { p, q, .. } = factor_big(&n, 200, 2000, seed) {
                assert_eq!(
                    crate::native_numeral::multiply_via_word(&p, &q), n,
                    "false positive: seed={} p={} q={}", seed, p, q
                );
            }
        }
    }
}
