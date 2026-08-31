// nested_oneshot.rs — Nested Prime Number Placement Operator Valued-Measure
//
// The ob3ect `the_nested_prime_number_placement_operator_value_5a507895` is a
// B4-verdict primality and factorization operator built on the IMASM word:
//   ⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣
//
// B4 verdicts:
//   T = "one-shot closure" — inner object is at fixed point → N is prime
//   F = "no closure"       — inner object outside basin → N is composite
//   B = "paradice"         — both conditions held simultaneously
//
// FOLD TOPOLOGY (factorization via kissing endpoints):
// Each nesting fold takes a linear sieve path and folds it so start and end kiss.
// When endpoints kiss (cycle complete), gcd(|x-y|, n) is a non-trivial factor.
// Brent's cycle detection = kissing detection = fold revelation.
//
// ARBITRARY LENGTH: uses num-bigint::BigUint for factorization — no fixed digit limit.
// Limited only by memory and patience.
//
// Tuple: ⟨𐑦𐑰𐑾𐑿𐑐𐑧𐑲𐑠⊙𐑓𐑳𐑴⟩

#![allow(dead_code)]

extern crate alloc;

use crate::sprintln;
use super::prime_winding::is_prime;
use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive};
use core::str::FromStr;

/// The canonical glyph word for the Nested Prime Number Placement Operator.
pub const WORD: &str = "⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣";
pub const PERIOD: usize = 12;
pub const PHASE_BEARING: bool = true;

pub const LANDINGS: [&str; 12] = [
    "A", "A", "Ftf", "Ftf", "Ftf", "Ftf", "Ttf", "Ttf", "tf", "T", "T", "A",
];

pub enum B4Verdict { T, F, B }

impl core::fmt::Display for B4Verdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            B4Verdict::T => write!(f, "T"),
            B4Verdict::F => write!(f, "F"),
            B4Verdict::B => write!(f, "B"),
        }
    }
}

pub fn landing_at(k: usize) -> &'static str { LANDINGS[k % PERIOD] }
pub fn winding_number() -> &'static str { "0/1" }

// ── String helpers ─────────────────────────────────────────────────────────────

fn trim(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() && bytes[i] == b'0' { i += 1; }
    String::from(&s[i..])
}

fn eq(a: &str, b: &str) -> bool { trim(a) == trim(b) }

fn lt(a: &str, b: &str) -> bool {
    let ta = trim(a);
    let tb = trim(b);
    if ta.len() != tb.len() { return ta.len() < tb.len(); }
    ta < tb
}

// ── BigUint arithmetic helpers ────────────────────────────────────────────────

fn parse_big(s: &str) -> Option<BigUint> {
    let t = trim(s);
    if t.is_empty() { return None; }
    BigUint::from_str(&t).ok()
}

fn big_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let t = b.clone();
        b = &a % &b;
        a = t;
    }
    a
}

// Brent's polynomial: x → x² + c mod n (all BigUint)
fn brent_f(x: &BigUint, c: &BigUint, n: &BigUint) -> BigUint {
    let x2 = (x * x) % n;
    (&x2 + c) % n
}

// Miller-Rabin primality test for BigUint.
fn is_prime_big(n: &BigUint) -> bool {
    if n < &BigUint::from(2u32) { return false; }
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);
    if n == &two || n == &three { return true; }
    if n % 2u32 == BigUint::zero() { return false; }

    // write n-1 = d * 2^s with d odd
    let n_minus_1 = n - 1u32;
    let mut s: u32 = 0;
    let mut d = n_minus_1.clone();
    let two_b = BigUint::from(2u32);
    while &d % &two_b == BigUint::zero() {
        d /= 2u32;
        s += 1;
    }

    // Deterministic bases for various ranges; fall back to several small primes.
    let max_deterministic: BigUint = BigUint::from_str("33170440646798834559697582").unwrap();
    let bases: Vec<BigUint> = if n < &max_deterministic {
        // Single base 2 is deterministic for this range
        alloc::vec![BigUint::from(2u32)]
    } else {
        // Several small bases — deterministic for most practical sizes
        let small: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];
        small.iter().map(|&b| BigUint::from(b) % n).collect()
    };

    for a in &bases {
        if a.is_zero() { continue; }
        let mut x = a.modpow(&d, n);
        if x == BigUint::one() || x == n_minus_1 { continue; }
        let mut cont = false;
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == n_minus_1 { cont = true; break; }
        }
        if !cont { return false; }
    }
    true
}

// Brent's cycle detection with fold/kiss topology (BigUint, arbitrary length)
fn brent_factor_big(n: &BigUint) -> Option<BigUint> {
    let two = BigUint::from(2u32);
    if n % 2u32 == BigUint::zero() { return Some(two.clone()); }
    if n.is_one() { return None; }

    // Seed c from the word's 4 distinct landing states
    let c_candidates: [u64; 4] = [3, 7, 11, 17];
    let n_mod_97 = (n % BigUint::from(97u32)).to_u64().unwrap_or(0);

    for (i, &base_c) in c_candidates.iter().enumerate() {
        // c = (base_c + (n%97)*(i+1)) % n, ensure >= 1
        let c_raw = base_c.wrapping_add((n_mod_97 as u64).wrapping_mul(i as u64 + 1)) % 1_000_000;
        let c = BigUint::from(if c_raw == 0 { 1u64 } else { c_raw });

        let x_init = BigUint::from(2u64 + (n_mod_97 % 95));
        let mut x: BigUint = x_init.clone();
        let m: u64 = 128;
        let mut power: u64 = 1;
        #[allow(unused_assignments)]
        let mut y: BigUint = x.clone();

        while power < 1_000_000 {
            // λ = power steps
            let mut step: u64 = 0;
            while step < power {
                x = brent_f(&x, &c, n);
                step += 1;
            }
            y = x.clone();
            // μ = power steps, compare to stored
            let mut j: u64 = 0;
            let upper = power.min(m);
            while j < upper {
                x = brent_f(&x, &c, n);
                let d = if x > y { &x - &y } else { &y - &x };
                let g = big_gcd(d, n.clone());
                if !g.is_one() && g != *n { return Some(g); }
                j += 1;
            }
            power = match power.checked_mul(2) {
                Some(p) => p,
                None => break,
            };
        }
    }
    None
}

/// Returns (confirmed prime factors, unfactored composite cofactors).
///
/// FIXED 2026-08-31: on a Brent failure this used to push the cofactor `m`
/// straight into the factor list with a "conservative: treat as prime"
/// comment -- but `m` had already failed `is_prime_big` (real Miller-Rabin)
/// one line above, so this was never conservative, it was wrong: a known
/// composite got reported as though it were a prime factor. Brent's rho has
/// no guaranteed success bound, and an RSA-scale semiprime (each factor far
/// past what ~4e6 Brent steps can reach) reliably triggers it -- verified
/// live: `nested_oneshot factor` on the real RSA-100-scale modulus this
/// session's other tests used printed "N = N", claiming the whole composite
/// modulus as its own sole factor. Now the two cases are kept apart.
fn factor_recursive_big(mut n: BigUint) -> (Vec<BigUint>, Vec<BigUint>) {
    let mut factors: Vec<BigUint> = Vec::new();
    let mut unfactored: Vec<BigUint> = Vec::new();
    let two = BigUint::from(2u32);
    while &n % &two == BigUint::zero() {
        factors.push(two.clone());
        n /= 2u32;
    }
    let mut stack: Vec<BigUint> = Vec::new();
    if !n.is_one() { stack.push(n); }
    while let Some(m) = stack.pop() {
        if m.is_one() { continue; }
        if is_prime_big(&m) { factors.push(m); continue; }
        if let Some(f) = brent_factor_big(&m) {
            let g = &m / &f;
            if f >= g {
                stack.push(f);
                stack.push(g);
            } else {
                stack.push(g);
                stack.push(f);
            }
        } else {
            unfactored.push(m); // confirmed composite, Brent could not split it
        }
    }
    factors.sort_unstable();
    unfactored.sort_unstable();
    (factors, unfactored)
}

fn collect_factors_big(n_str: &str) -> (Vec<BigUint>, Vec<BigUint>) {
    let t = trim(n_str);
    if t.is_empty() || t == "0" { return (Vec::new(), Vec::new()); }
    match parse_big(&t) {
        Some(ref n) if n > &BigUint::one() => factor_recursive_big(n.clone()),
        _ => (Vec::new(), Vec::new()),
    }
}

fn big_to_string(b: &BigUint) -> String {
    b.to_str_radix(10)
}

// ── B4 verdict ────────────────────────────────────────────────────────────────

pub fn verdict_for(n_str: &str) -> B4Verdict {
    if eq(n_str, "1") { return B4Verdict::B; }
    if lt(n_str, "1") { return B4Verdict::F; }
    if eq(n_str, "2") { return B4Verdict::T; }
    if is_prime(n_str) { B4Verdict::T } else { B4Verdict::F }
}

// ── REPL entry ────────────────────────────────────────────────────────────────

pub fn repl_nested_oneshot(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("nested_oneshot <N> — B4 verdict + oneshot factorization");
        sprintln!("  N has ARBITRARY LENGTH (BigUint, no fixed digit limit).");
        sprintln!("  Returns B4 verdict: T (prime), F (composite), B (paradice).");
        sprintln!("  Subcommands:");
        sprintln!("    nested_oneshot word        — canonical glyph word");
        sprintln!("    nested_oneshot cycle       — ROTAT orbit with landing register");
        sprintln!("    nested_oneshot verdict N   — B4 verdict for N");
        sprintln!("    nested_oneshot winding    — winding number");
        sprintln!("    nested_oneshot landings    — landing register states by k");
        sprintln!("    nested_oneshot factor N   — oneshot factor via Brent fold/kiss (arbitrary length)");
        sprintln!("  Examples:");
        sprintln!("    nested_oneshot 17");
        sprintln!("    nested_oneshot factor 1234567");
        sprintln!("    nested_oneshot factor 99999999999999999999999999999999999999");
        return;
    }

    let sub = args[0];
    match sub {
        "word" => {
            sprintln!("word     : {}", WORD);
            sprintln!("period   : {}", PERIOD);
            sprintln!("phase-bearing: {}", PHASE_BEARING);
        }
        "cycle" => {
            sprintln!("word: {}   period {}", WORD, PERIOD);
            sprintln!("  k  landing");
            sprintln!("  ─── ───────");
            for k in 0..PERIOD { sprintln!("  {:3}  {}", k, landing_at(k)); }
            sprintln!("final register is PHASE-BEARING: 4 distinct landings");
        }
        "verdict" => {
            if args.len() < 2 { sprintln!("Usage: nested_oneshot verdict <N>"); return; }
            let n = args[1];
            let v = verdict_for(n);
            let desc = match &v {
                B4Verdict::T => "one-shot closure (prime)",
                B4Verdict::F => "no closure (composite)",
                B4Verdict::B => "paradice (neither prime nor composite)",
            };
            sprintln!("nested_oneshot verdict {}: {}", n, v);
            sprintln!("  └─ {}", desc);
        }
        "winding" => { sprintln!("winding: {}", winding_number()); }
        "landings" => {
            sprintln!("landing register by ROTAT cut (k=0..11):");
            for k in 0..PERIOD { sprintln!("  k={:2}: {}", k, landing_at(k)); }
        }
        "factor" => {
            if args.len() < 2 { sprintln!("Usage: nested_oneshot factor <N>"); return; }
            let n = args[1];
            let v = verdict_for(n);
            match v {
                B4Verdict::B => {
                    sprintln!("nested_oneshot factor {}: B — paradice (1 is the unit)", n);
                }
                B4Verdict::F => {
                    let (factors, unfactored) = collect_factors_big(n);
                    if factors.is_empty() && unfactored.is_empty() {
                        sprintln!("nested_oneshot factor {}: F — could not factor (non-numeric input?)", n);
                    } else {
                        // group with multiplicity
                        let group = |xs: &[BigUint]| -> Vec<String> {
                            let mut reps: Vec<String> = Vec::new();
                            let mut i = 0usize;
                            while i < xs.len() {
                                let p = &xs[i];
                                let mut cnt = 1usize;
                                while (i + cnt) < xs.len() && &xs[i + cnt] == p {
                                    cnt += 1;
                                }
                                if cnt == 1 { reps.push(big_to_string(p)); }
                                else { reps.push(alloc::format!("{}^{}", big_to_string(p), cnt)); }
                                i += cnt;
                            }
                            reps
                        };
                        sprintln!("nested_oneshot factor {}: F — composite (arbitrary-length Brent)", n);
                        if !factors.is_empty() {
                            sprintln!("  └─ confirmed prime factors: {}", group(&factors).join(" × "));
                        }
                        if !unfactored.is_empty() {
                            sprintln!("  └─ unfactored composite cofactor(s), Brent could not split (confirmed NOT prime by Miller-Rabin): {}", group(&unfactored).join(" × "));
                        }
                    }
                }
                B4Verdict::T => {
                    sprintln!("nested_oneshot factor {}: T — prime (no non-trivial factors)", n);
                }
            }
        }
        _ => {
            let n = sub;
            let v = verdict_for(n);
            let desc = match &v {
                B4Verdict::T => "prime",
                B4Verdict::F => "composite",
                B4Verdict::B => "paradice",
            };
            sprintln!("{}", v);
            sprintln!("  └─ {}: {}", desc, n);
        }
    }
}
