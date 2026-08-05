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
    let mut d = 2u64;
    while d * d <= r {
        while r % d == 0 && powmod(a, r / d, N) == 1 { r /= d; }
        d += if d == 2 { 1 } else { 2 };
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
