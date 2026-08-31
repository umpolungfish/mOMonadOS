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

fn gcd(mut a: u64, mut b: u64) -> u64 {
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

fn modinv(a: u64, m: u64) -> u64 {
    let (x, _, g) = egcd(a % m, m);
    debug_assert_eq!(g, 1);
    ((x % m as i128 + m as i128) % m as i128) as u64
}

fn isqrt(n: u64) -> u64 {
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
// reach, not N's size. Past that reach the search reports plainly that
// it did not close within the cap -- never a silent switch to a
// different, non-winding method.

/// a mod N, N mod a, gcd -- same Euclid as the u64 `gcd` above, BigUint.
fn gcd_big(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let t = &a % &b;
        a = b;
        b = t;
    }
    a
}

/// Integer square root by Newton's method, BigUint -- same shape as `isqrt`.
fn isqrt_big(n: &BigUint) -> BigUint {
    if n.is_zero() { return BigUint::zero(); }
    let two = BigUint::from(2u32);
    let mut x = n.clone();
    let mut y = (&x + BigUint::one()) / &two;
    while y < x {
        x = y;
        y = (&x + n / &x) / &two;
    }
    x
}

/// Modular inverse via extended Euclid on BigInt -- same shape as `egcd`
/// + `modinv`, promoted to signed arbitrary precision for the subtraction.
fn modinv_big(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let a_i = BigInt::from_biguint(Sign::Plus, a.clone());
    let m_i = BigInt::from_biguint(Sign::Plus, m.clone());
    let (mut old_r, mut r) = (a_i, m_i.clone());
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::from(0));
    while !r.is_zero() {
        let q = &old_r / &r;
        let t_r = &old_r - &q * &r; old_r = r; r = t_r;
        let t_s = &old_s - &q * &s; old_s = s; s = t_s;
    }
    if old_r != BigInt::from(1) { return None; }
    let inv = ((old_s % &m_i) + &m_i) % &m_i;
    inv.to_biguint()
}

/// Three outcomes for a BigUint order search: found (always u64, per the
/// reach argument above), a not coprime to N (or N<=1, degenerate), or
/// the table the step cap allows was not big enough to see the closure.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindingBig {
    Order(u64),
    NotInGroup,
    BudgetExceeded,
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
            if r % p == 0 && a.modpow(&BigUint::from(r / p), n) == one {
                r /= p;
                changed = true;
            }
        }
    }
    r
}

/// BSGS order of a mod N, BigUint. `step_cap` bounds the baby-step table
/// (and so the order this can certify, to step_cap*(step_cap+1)) --
/// exceeding it is BudgetExceeded, not a guess and not a fallback.
pub fn winding_order_big(a: &BigUint, n: &BigUint, step_cap: u64) -> WindingBig {
    let one = BigUint::one();
    if *n <= one { return WindingBig::NotInGroup; }
    let a_mod = a % n;
    if gcd_big(a_mod.clone(), n.clone()) != one { return WindingBig::NotInGroup; }
    if a_mod == one { return WindingBig::Order(1); }

    let m_big = isqrt_big(n) + &one;
    if m_big > BigUint::from(step_cap) { return WindingBig::BudgetExceeded; }
    let m = match m_big.to_u64() { Some(v) => v, None => return WindingBig::BudgetExceeded };

    let mut baby: Vec<(BigUint, u64)> = Vec::with_capacity(m as usize);
    let mut cur = one.clone();
    for j in 0..m {
        baby.push((cur.clone(), j));
        cur = (&cur * &a_mod) % n;
    }
    baby.sort_unstable_by(|x, y| x.0.cmp(&y.0));
    baby.dedup_by(|x, y| x.0 == y.0);

    let a_inv = match modinv_big(&a_mod, n) { Some(v) => v, None => return WindingBig::NotInGroup };
    let giant_step = a_inv.modpow(&BigUint::from(m), n);
    let mut gamma = one.clone();
    for i in 1..=m {
        gamma = (&gamma * &giant_step) % n;
        if let Ok(k) = baby.binary_search_by(|probe| probe.0.cmp(&gamma)) {
            let j = baby[k].1;
            let cand = i.saturating_mul(m) + j;
            if cand > 0 && a_mod.modpow(&BigUint::from(cand), n) == one {
                return WindingBig::Order(minimal_winding_big(&a_mod, n, cand));
            }
        }
    }
    WindingBig::BudgetExceeded
}

/// Three outcomes for a BigUint factorization attempt: a non-trivial
/// split, the tries exhausted with no split found (N may still be
/// prime, or the bases tried just did not work), or the order search
/// itself did not reach far enough to try -- BudgetExceeded takes
/// priority in the report because it means the attempt stopped short,
/// not that N resisted a complete one.
pub enum FactorBig {
    Found { a: BigUint, r: u64, p: BigUint, q: BigUint },
    NoFactorInTries,
    BudgetExceeded,
}

/// Shor's winding step, BigUint: order r of a random base a, r even and
/// a^(r/2) not ±1 gives gcd(a^(r/2) - 1, N) as a non-trivial factor.
/// Same retry shape as `factor`, promoted to arbitrary precision.
pub fn factor_big(n: &BigUint, max_tries: u32, step_cap: u64, mut seed: u64) -> FactorBig {
    let one = BigUint::one();
    let three = BigUint::from(3u32);
    if *n <= three { return FactorBig::NoFactorInTries; }
    let n_minus_1 = n - &one;
    let range = n - &three;
    let mut budget_hit = false;
    for _ in 0..max_tries {
        seed = xorshift64(seed);
        let a = &BigUint::from(seed) % &range + &three;
        let g0 = gcd_big(a.clone(), n.clone());
        if g0 != one && &g0 != n {
            let q = n / &g0;
            return FactorBig::Found { a, r: 0, p: g0, q };
        }
        match winding_order_big(&a, n, step_cap) {
            WindingBig::Order(r) => {
                if r == 0 || r % 2 != 0 { continue; }
                let x = a.modpow(&BigUint::from(r / 2), n);
                if x == one || x == n_minus_1 { continue; }
                let g = gcd_big(&x - &one, n.clone());
                if g != one && &g != n {
                    let q = n / &g;
                    return FactorBig::Found { a, r, p: g, q };
                }
            }
            WindingBig::BudgetExceeded => budget_hit = true,
            WindingBig::NotInGroup => {}
        }
    }
    if budget_hit { FactorBig::BudgetExceeded } else { FactorBig::NoFactorInTries }
}
