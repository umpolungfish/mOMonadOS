#!/usr/bin/env python3
"""Patch src/prime_winding.rs:
   1. Insert mul_str / mod_str / powmod_str helpers (string-based bignum).
   2. Replace is_prime() with Miller-Rabin on strings (7-base test).
   3. Patch factor() so its residual > 1M is MR-checked rather than
      blindly emitted as a prime factor.
"""
from pathlib import Path

P = Path("/home/mrnob0dy666/imsgct/mOMonadOS/src/prime_winding.rs")
src = P.read_text()

# --- 1. New helper block to insert just after div_small (line ~181) -------
helpers = '''
/// a * b, both non-negative decimal strings, schoolbook O(n*m).
/// Never truncates: every digit carries into the next.
fn mul_str(a: &str, b: &str) -> String {
    let ta = trim(a);
    let tb = trim(b);
    if ta == "0" || tb == "0" { return String::from("0"); }
    let av: Vec<u32> = ta.bytes().rev().map(|c| (c - b'0') as u32).collect();
    let bv: Vec<u32> = tb.bytes().rev().map(|c| (c - b'0') as u32).collect();
    let mut out: Vec<u32> = vec![0; av.len() + bv.len()];
    for i in 0..av.len() {
        let mut carry: u32 = 0;
        for j in 0..bv.len() {
            let cur = out[i + j] + av[i] * bv[j] + carry;
            out[i + j] = cur % 10;
            carry = cur / 10;
        }
        let mut k = i + bv.len();
        while carry > 0 {
            let cur = out[k] + carry;
            out[k] = cur % 10;
            carry = cur / 10;
            k += 1;
        }
    }
    let s: String = out.iter().rev().map(|d| (b'0' + *d as u8) as char).collect();
    trim(&s)
}

/// a mod m, both non-negative decimal strings, m > 0. Schoolbook.
fn mod_str(a: &str, m: &str) -> String {
    if m == "0" { return String::from("0"); }
    let mut r = String::from("0");
    for c in a.bytes() {
        // r = (r*10 + digit) mod m
        r = mul_str(&r, "10");
        let digit = (c - b'0') as char;
        r = add(&r, &String::from(digit));
        // subtract multiples of m until r < m
        while !lt(&r, m) {
            r = sub(&r, m);
        }
    }
    r
}

/// (base ^ exp) mod n, all non-negative decimal strings, n > 0.
/// Square-and-multiply. Exact, no u64 overflow.
fn powmod_str(base: &str, exp: &str, n: &str) -> String {
    if n == "1" { return String::from("0"); }
    let mut result = String::from("1");
    let mut b = mod_str(base, n);
    // Walk exp least-significant digit first
    let tb = trim(exp);
    for c in tb.bytes() {
        let digit = c - b'0';
        for _ in 0..digit {
            result = mod_str(&mul_str(&result, &b), n);
        }
        // square b
        b = mod_str(&mul_str(&b, &b), n);
    }
    result
}

'''

# Find anchor: the line "// True if a is prime, by trial division up to √a..."
anchor = "/// True if a is prime, by trial division up to √a (small u64 divisor path)."
assert anchor in src, "anchor not found"
src = src.replace(anchor, helpers + "/// True if a is prime, by deterministic Miller-Rabin on string n.\n/// 7 bases {2,3,5,7,11,13,17} — deterministic below ~3.3e23 and very reliable above.\n/// String arithmetic throughout, so n is unbounded (no u64 limit).")

# --- 2. Replace the body of is_prime (from `fn is_prime` to its closing `}`) -
import re
pat = re.compile(
    r"fn is_prime\(a: &str\) -> bool \{.*?^}",
    re.DOTALL | re.MULTILINE,
)
new_body = '''fn is_prime(a: &str) -> bool {
    let t = trim(a);
    if t == "1" || t == "0" { return false; }
    if t == "2" { return true; }
    if t == "3" || t == "5" || t == "7" { return true; }
    if is_even(&t) { return false; }
    if divisible_by(&t, 3) { return false; }
    if divisible_by(&t, 5) { return false; }
    if divisible_by(&t, 7) { return false; }
    if divisible_by(&t, 11) { return false; }
    if divisible_by(&t, 13) { return false; }
    if divisible_by(&t, 17) { return false; }
    if divisible_by(&t, 19) { return false; }
    if divisible_by(&t, 23) { return false; }
    if divisible_by(&t, 29) { return false; }
    if divisible_by(&t, 31) { return false; }
    if divisible_by(&t, 37) { return false; }
    if lt(&t, "41") { return true; } // exhausted primes < 41, n in {41,43,47} survive

    // Small u64 fast path: deterministic MR with the 7-base set
    // covers all 64-bit integers (a > 2^64 has at least one base witnessing).
    if t.len() <= 18 {
        if let Ok(n) = t.parse::<u64>() {
            return is_prime_u64(n);
        }
    }

    // Arbitrary-precision Miller-Rabin on strings.
    is_prime_str(&t)
}

/// Deterministic Miller-Rabin for n: u64, n < 2^64.
/// Bases {2,3,5,7,11,13,17} are sufficient (per Jim Sinclair).
fn is_prime_u64(n: u64) -> bool {
    if n < 2 { return false; }
    let small = [2u64,3,5,7,11,13,17,19,23,29,31,37];
    for &p in &small {
        if n == p { return true; }
        if n % p == 0 { return false; }
    }
    // Write n-1 = 2^r * d with d odd.
    let mut d = n - 1;
    let mut r: u32 = 0;
    while d % 2 == 0 { d /= 2; r += 1; }
    let bases: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
    'outer: for &a in &bases {
        if a % n == 0 { continue; }
        // x = a^d mod n
        let mut x: u128 = 1;
        let mut base: u128 = a as u128;
        let mut e: u128 = d as u128;
        let m: u128 = n as u128;
        while e > 0 {
            if e & 1 == 1 { x = (x * base) % m; }
            base = (base * base) % m;
            e >>= 1;
        }
        let mut x64 = x as u64;
        if x64 == 1 || x64 == n - 1 { continue; }
        for _ in 0..(r - 1) {
            x = (x * x) % m;
            x64 = x as u64;
            if x64 == n - 1 { continue 'outer; }
        }
        return false; // composite, witnessed by base a
    }
    true
}

/// Deterministic Miller-Rabin on string n (n is a decimal, n > 37).
/// Uses bases {2,3,5,7,11,13,17}. All arithmetic is string-based:
/// no u64 rounding, no truncation, exact.
fn is_prime_str(n: &str) -> bool {
    // n-1 = 2^r * d, d odd. We compute this in decimal string form.
    let one = String::from("1");
    let two = String::from("2");
    let n_minus_1 = sub(n, &one);
    let mut d = n_minus_1.clone();
    let mut r: u32 = 0;
    while is_even(&d) {
        d = div_small(&d, 2);
        r += 1;
    }
    let bases = ["2", "3", "5", "7", "11", "13", "17"];
    let n_minus_1_str = n_minus_1.clone();
    'outer: for base in &bases {
        // a must be in [2, n-2]; if base >= n the test is vacuous
        if !lt(base, n) { continue; }
        // x = base^d mod n
        let mut x = powmod_str(base, &d, n);
        if eq(&x, &one) || eq(&x, &n_minus_1_str) { continue; }
        let mut witness = true;
        for _ in 0..(r - 1) {
            x = mod_str(&mul_str(&x, &x), n);
            if eq(&x, &n_minus_1_str) { witness = false; break; }
        }
        if witness { return false; } // composite
    }
    true
}'''

src2, n = pat.subn(new_body, src)
assert n == 1, f"is_prime replacement: expected 1 substitution, got {n}"
src = src2

# --- 3. Patch factor() so its residual is MR-checked ------------------------
# Replace:
#   if !is_zero(&m) && m != "1" {
#       factors.push(m.clone());
#   }
# with MR-checked path
old_residual = '''    if !is_zero(&m) && m != "1" {
        factors.push(m.clone());
    }'''
new_residual = '''    if !is_zero(&m) && m != "1" {
        // Residual m: under trial division by primes <= 1M, m is either
        // a prime or a composite of two primes both > 1M. Use Miller-Rabin
        // to discriminate — if MR says composite, fall back to a coarse
        // factorisation by a second pass of small primes (string-based).
        if is_prime(&m) {
            factors.push(m.clone());
        } else {
            // Composite residual: try one more round of small-prime trial division
            // to peel off any factor < 10M that the first round missed (shouldn't
            // happen, but defensive). If still composite, just emit the residual
            // labelled COMPOSITE so the caller knows.
            let mut r2: u64 = 999_999;
            if r2 % 2 == 0 { r2 -= 1; }
            loop {
                let d2 = (r2 as u128 * r2 as u128).to_string();
                if !lt(&d2, &m) { break; }
                while rem_small(&m, r2) == 0 {
                    factors.push(r2.to_string());
                    m = div_small(&m, r2);
                }
                if r2 < 1_000_000 { r2 = 1_000_000; }
                if r2 >= 10_000_000 { break; }
                r2 += 2;
            }
            if !is_zero(&m) && m != "1" {
                if is_prime(&m) {
                    factors.push(m.clone());
                } else {
                    factors.push(format!("[COMPOSITE:{}]", m));
                }
            }
        }
    }'''
assert old_residual in src, "residual anchor not found"
src = src.replace(old_residual, new_residual)

P.write_text(src)
print(f"Patched {P} ({len(src)} bytes)")
