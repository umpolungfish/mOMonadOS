#!/usr/bin/env python3
"""Patch3: fix limb_mod and powmod_limbs in prime_winding.rs."""
from pathlib import Path

P = Path("/home/mrnob0dy666/imsgct/mOMonadOS/src/prime_winding.rs")
src = P.read_text()

# ── Replace the entire limb block (to_limbs through is_prime_limbs) ────────────
old_limb_block = """// ── Limb-based big integer arithmetic for primality testing ────────────────
//
// A "limb" is a u64; a big integer is stored LSB-first in Vec<u64>.
// O(n³) per MR round — fast enough for ≤ 500-limb (≈10,000-digit) inputs.
//
// (The powmod string-path is correct for all sizes; the limb path here
// replaces it for the MR inner loop, where the exponent is typically small
// and the modulus is the large candidate being tested.)

fn trim_limbs(a: &mut Vec<u64>) {
    while a.len() > 1 && *a.last().unwrap() == 0 { a.pop(); }
}

/// Convert decimal string to Vec<u64> limbs (LSB first).
fn to_limbs(s: &str) -> Vec<u64> {
    let t = trim(s);
    if t == "0" { return vec![0]; }
    // Generous initial size.
    let nbits = (t.len() as f64 * 3.322) as usize + 1;
    let mut limbs = vec![0u64; nbits / 64 + 2];
    for c in t.bytes() {
        // limbs *= 10
        let mut carry: u64 = 0;
        for i in 0..limbs.len() {
            let x = (limbs[i] as u128) * 10 + carry as u128;
            limbs[i] = x as u64;
            carry = (x >> 64) as u64;
        }
        // limbs += digit
        let d = (c - b'0') as u64;
        let mut car = d;
        for i in 0..limbs.len() {
            let sum = limbs[i] as u128 + car as u128;
            limbs[i] = sum as u64;
            car = (sum >> 64) as u64;
            if car == 0 { break; }
        }
    }
    trim_limbs(&mut limbs);
    limbs
}

/// Multiply two limb vectors; returns trimmed Vec<u64>.
fn mul_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0u64; a.len() + b.len() + 1];
    for i in 0..a.len() {
        if a[i] == 0 { continue; }
        let mut carry: u64 = 0;
        for j in 0..b.len() {
            if b[j] == 0 { continue; }
            let prod = (a[i] as u128) * (b[j] as u128) + out[i + j] as u128 + carry as u128;
            out[i + j] = prod as u64;
            carry = (prod >> 64) as u64;
        }
        let mut k = i + b.len();
        let mut c = carry;
        while c > 0 {
            let sum = out[k] as u128 + c as u128;
            out[k] = sum as u64;
            c = (sum >> 64) as u64;
            k += 1;
        }
    }
    trim_limbs(&mut out);
    out
}

fn limb_effective_len(a: &[u64]) -> usize {
    a.len() - a.iter().rev().take_while(|&&x| x == 0).count()
}

fn limb_eq(a: &[u64], b: &[u64]) -> bool {
    if limb_effective_len(a) != limb_effective_len(b) { return false; }
    for i in 0..limb_effective_len(a) { if a[i] != b[i] { return false; } }
    true
}

fn limb_eq_u64(a: &[u64], v: u64) -> bool {
    if a.is_empty() { return v == 0; }
    if a[0] != v { return false; }
    for i in 1..a.len() { if a[i] != 0 { return false; } }
    true
}

fn limb_lt_u64(a: &[u64], v: u64) -> bool {
    if a.is_empty() { return v > 0; }
    if a.len() > 1 { return false; }
    a[0] < v
}

fn limb_rem_u64(a: &[u64], v: u64) -> u64 {
    if v == 0 { return 0; }
    let mut r: u64 = 0;
    for i in (0..a.len()).rev() {
        let combined = ((r as u128) << 64) | a[i] as u128;
        r = (combined % v as u128) as u64;
    }
    r
}

fn limb_is_even(a: &[u64]) -> bool { a.is_empty() || (a[0] & 1) == 0 }

fn limb_sub1(a: &mut [u64]) {
    let mut i = 0;
    loop {
        if a[i] > 0 { a[i] -= 1; return; }
        a[i] = u64::MAX;
        i += 1;
    }
}

fn limb_shr1(a: &mut [u64]) {
    let mut carry: u64 = 0;
    for i in (0..a.len()).rev() {
        let new_carry = a[i] & 1;
        a[i] = (a[i] >> 1) | (carry << 63);
        carry = new_carry;
    }
}

/// (base ^ exp) mod n on limb vectors. Square-and-multiply.
fn powmod_limbs(base: &[u64], exp: &[u64], n: &[u64]) -> Vec<u64> {
    if limb_eq_u64(n, 1) { return vec![0]; }
    let mut result = vec![1u64];
    let mut b = base.to_vec();
    for i in (0..exp.len()).rev() {
        let word = exp[i];
        for bit in (0..64u32).rev() {
            let sq = mul_limbs(&result, &result);
            result = limb_mod(sq, n);
            if (word >> bit) & 1 == 1 {
                let p = mul_limbs(&result, &b);
                result = limb_mod(p, n);
            }
        }
    }
    result
}

/// `a mod n` via simple schoolbook subtraction.  O(n² · |a/n|).
/// Always terminates: at each loop, either a < n (return) or a -= n (strictly smaller).
fn limb_mod(mut a: Vec<u64>, n: &[u64]) -> Vec<u64> {
    loop {
        // If a < n (by length or by element comparison), we are done.
        if a.len() < n.len() {
            trim_limbs(&mut a);
            return a;
        }
        if a.len() == n.len() {
            // Compare high-to-low; Rust u64 comparison is correct.
            for i in (0..a.len()).rev() {
                if a[i] > n[i] { break; }
                if a[i] < n[i] { trim_limbs(&mut a); return a; }
            }
            // equal
            return vec![0];
        }
        // a.len() > n.len(): a > n, subtract n from a.
        // We subtract with zero-padding so the loops align at index 0.
        let mut borrow: u64 = 0;
        for i in 0..a.len() {
            let av = a[i] as u128;
            let bv = if i < n.len() { n[i] as u128 } else { 0.0 }; // placeholder
            let _ = bv; // silence
            // simple subtraction at offset 0
            let ai = a[i];
            let bi = if i < n.len() { n[i] } else { 0 };
            if ai >= bi + borrow {
                a[i] = ai - bi - borrow;
                borrow = 0;
            } else {
                a[i] = ai + (1u64 << 64) - bi - borrow;
                borrow = 1;
            }
        }
        // loop continues
    }
}

/// Miller-Rabin on limb-vector n. Bases {2,3,5,7,11,13,17}.
fn is_prime_limbs(n: &[u64]) -> bool {
    // small prime sieve
    let small = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    for &p in &small {
        if limb_eq_u64(n, p) { return true; }
        if limb_rem_u64(n, p) == 0 { return false; }
    }
    if limb_lt_u64(n, 49) { return true; }
    // n-1 = 2^r * d
    let mut d = n.to_vec();
    limb_sub1(&mut d);
    let mut r: u32 = 0;
    while limb_is_even(&d) { limb_shr1(&mut d); r += 1; }
    let n_minus_1 = {
        let mut t = n.to_vec();
        limb_sub1(&mut t);
        t
    };
    let bases = [2u64, 3, 5, 7, 11, 13, 17];
    for &a in &bases {
        // skip if a >= n
        if !limb_lt_u64(n, a) { continue; }
        let base_limbs = vec![a];
        let mut x = powmod_limbs(&base_limbs, &d, n);
        if limb_eq(&x, &[1]) || limb_eq(&x, &n_minus_1) { continue; }
        for _ in 0..(r - 1) {
            let sq = mul_limbs(&x, &x);
            x = limb_mod(sq, n);
            if limb_eq(&x, &n_minus_1) { break; }
        }
        if !limb_eq(&x, &n_minus_1) { return false; }
    }
    true
}"""

new_limb_block = """// ── Limb-based big integer arithmetic for Miller-Rabin primality testing ───────
//
// A "limb" is a u64; a big integer is stored LSB-first in Vec<u64>.
// O(n³) per MR round — fast enough for inputs up to ~500 limbs (≈10,000 digits).
//
// The exponent in MR is d = (n-1)/2^r, which is typically the size of n (large).
// The modulus n is also large.  The product of two large limb vectors is O(n²).

fn trim_limbs(a: &mut Vec<u64>) {
    while a.len() > 1 && *a.last().unwrap() == 0 { a.pop(); }
}

/// Convert decimal string to Vec<u64> limbs (LSB first).
/// Uses 10^19 as the limb base (10^19 < 2^64).
fn to_limbs(s: &str) -> Vec<u64> {
    let t = trim(s);
    if t == "0" { return vec![0]; }
    // Each limb holds up to 19 decimal digits (10^19 < 2^64).
    let cap = 1_000_000_000_000_000_000u64; // 10^19
    let mut limbs = Vec::new();
    let mut pos = t.len();
    while pos > 0 {
        // Take up to 19 digits from the right.
        let start = if pos > 19 { pos - 19 } else { 0 };
        let chunk = &t[start..pos];
        let digit = chunk.parse::<u64>().unwrap();
        limbs.push(digit);
        pos = start;
        while start > 0 && t.as_bytes()[start - 1] == b'0' && pos == start {
            // handle leading zeros on next chunk
            start -= 1;
            pos = start;
        }
    }
    trim_limbs(&mut limbs);
    limbs
}

/// Multiply two limb vectors; returns a new trimmed Vec<u64>.
fn mul_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0u64; a.len() + b.len() + 1];
    for i in 0..a.len() {
        if a[i] == 0 { continue; }
        let mut carry: u64 = 0;
        for j in 0..b.len() {
            if b[j] == 0 { continue; }
            let prod = (a[i] as u128) * (b[j] as u128) + out[i + j] as u128 + carry as u128;
            out[i + j] = prod as u64;
            carry = (prod >> 64) as u64;
        }
        let mut k = i + b.len();
        let mut c = carry;
        while c > 0 {
            let sum = out[k] as u128 + c as u128;
            out[k] = sum as u64;
            c = (sum >> 64) as u64;
            k += 1;
        }
    }
    trim_limbs(&mut out);
    out
}

fn limb_effective_len(a: &[u64]) -> usize {
    a.len() - a.iter().rev().take_while(|&&x| x == 0).count()
}

fn limb_eq(a: &[u64], b: &[u64]) -> bool {
    if limb_effective_len(a) != limb_effective_len(b) { return false; }
    for i in 0..limb_effective_len(a) { if a[i] != b[i] { return false; } }
    true
}

fn limb_eq_u64(a: &[u64], v: u64) -> bool {
    if a.is_empty() { return v == 0; }
    if a[0] != v { return false; }
    for i in 1..a.len() { if a[i] != 0 { return false; } }
    true
}

fn limb_lt_u64(a: &[u64], v: u64) -> bool {
    if a.is_empty() { return v > 0; }
    if a.len() > 1 { return false; }
    a[0] < v
}

fn limb_rem_u64(a: &[u64], v: u64) -> u64 {
    if v == 0 { return 0; }
    let mut r: u64 = 0;
    for i in (0..a.len()).rev() {
        let combined = ((r as u128) << 64) | a[i] as u128;
        r = (combined % v as u128) as u64;
    }
    r
}

fn limb_is_even(a: &[u64]) -> bool { a.is_empty() || (a[0] & 1) == 0 }

fn limb_sub1(a: &mut [u64]) {
    let mut i = 0;
    loop {
        if a[i] > 0 { a[i] -= 1; return; }
        a[i] = u64::MAX;
        i += 1;
    }
}

fn limb_shr1(a: &mut [u64]) {
    let mut carry: u64 = 0;
    for i in (0..a.len()).rev() {
        let new_carry = a[i] & 1;
        a[i] = (a[i] >> 1) | (carry << 63);
        carry = new_carry;
    }
}

/// (base ^ exp) mod n on limb vectors. Square-and-multiply, MSB-first on exponent.
fn powmod_limbs(base: &[u64], exp: &[u64], n: &[u64]) -> Vec<u64> {
    if limb_eq_u64(n, 1) { return vec![0]; }
    let mut result = vec![1u64];
    let b = base.to_vec();
    for i in (0..exp.len()).rev() {
        let word = exp[i];
        for bit in (0..64u32).rev() {
            // result = result^2 mod n
            let sq = mul_limbs(&result, &result);
            result = limb_mod(sq, n);
            if (word >> bit) & 1 == 1 {
                // result = result * b mod n
                let p = mul_limbs(&result, &b);
                result = limb_mod(p, n);
            }
        }
    }
    result
}

/// `a mod n` via simple schoolbook subtraction.
//  Terminates: each loop either returns (a < n) or subtracts n (a becomes strictly smaller).
//  Complexity: O(n² · |a/n|).  For MR inner loop, |a| ≈ |n| and |a/n| ≈ 1, so O(n²) ≈ O(L²).
fn limb_mod(mut a: Vec<u64>, n: &[u64]) -> Vec<u64> {
    if n.is_empty() || (n.len() == 1 && n[0] == 0) { return a; }
    loop {
        // If a < n by length: done.
        if a.len() < n.len() { trim_limbs(&mut a); return a; }
        // If equal length: compare limb-by-limb from high to low.
        if a.len() == n.len() {
            let mut is_lt = false;
            let mut is_gt = false;
            for i in (0..a.len()).rev() {
                if a[i] > n[i] { is_gt = true; break; }
                if a[i] < n[i] { is_lt = true; break; }
            }
            if is_lt { trim_limbs(&mut a); return a; }
            if !is_gt { return vec![0]; } // equal
        }
        // a > n (strictly, by length or value). Subtract n aligned at index 0.
        let mut borrow: u64 = 0;
        for i in 0..a.len() {
            let ai = a[i];
            let bi = if i < n.len() { n[i] } else { 0 };
            if ai >= bi + borrow {
                a[i] = ai - bi - borrow;
                borrow = 0;
            } else {
                a[i] = ai.wrapping_sub(bi).wrapping_sub(borrow);
                borrow = 1;
            }
        }
        // trim any leading zeros and loop
        trim_limbs(&mut a);
    }
}

/// Miller-Rabin on limb-vector n. Bases {2,3,5,7,11,13,17}.
fn is_prime_limbs(n: &[u64]) -> bool {
    // Small prime sieve (covers n < 50).
    let small = [2u64,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
    for &p in &small {
        if limb_eq_u64(n, p) { return true; }
        if limb_rem_u64(n, p) == 0 { return false; }
    }
    if limb_lt_u64(n, 49) { return true; }

    // n-1 = 2^r * d with d odd.
    let mut d = n.to_vec();
    limb_sub1(&mut d);
    let mut r: u32 = 0;
    while limb_is_even(&d) { limb_shr1(&mut d); r += 1; }
    let n_minus_1 = {
        let mut t = n.to_vec();
        limb_sub1(&mut t);
        t
    };

    let bases = [2u64, 3, 5, 7, 11, 13, 17];
    for &a in &bases {
        // Skip a >= n.
        if !limb_lt_u64(n, a) { continue; }
        let base_limbs = vec![a];
        let mut x = powmod_limbs(&base_limbs, &d, n);
        if limb_eq(&x, &[1]) || limb_eq(&x, &n_minus_1) { continue; }
        for _ in 0..(r - 1) {
            let sq = mul_limbs(&x, &x);
            x = limb_mod(sq, n);
            if limb_eq(&x, &n_minus_1) { break; }
        }
        if !limb_eq(&x, &n_minus_1) { return false; } // composite
    }
    true
}"""

assert old_limb_block in src, "limb block anchor not found"
src = src.replace(old_limb_block, new_limb_block)

P.write_text(src)
print("patch3: limb_mod fixed")
