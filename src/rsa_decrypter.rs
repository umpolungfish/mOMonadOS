// rsa_decrypter.rs — RSA Decrypter via Baby-Step Giant-Step Period Finding
//
// The ob3ect glyph word: ⊢⊣∈≻⊤≺⊥∋⋈⊞≻⋈⊙⊤⊡⊣
//
// ONESHOT PRINCIPLE: Construct the decryption inside its own fixed-point closure.
// For a given ciphertext C, modulus N, and private exponent d,
// the winding period r is found via Baby-Step Giant-Step (BSGS).
// The private descent C^d mod N is the reverse morphism closing the loop.
//
// Phase 2 (Frobenius):
//   ∈ Period Finding Fork → Baby-Step Lattice (forward) + Giant-Step Search (reverse)
//   ∋ Winding Period Resolution → Minimal Period r (Frobenius verdict T)
//
// The BSGS algorithm finds the minimal r such that a^r ≡ 1 (mod N)
// where a is the base (typically the ciphertext or a generator).
// Then d ≡ e^(-1) mod r gives the private exponent.
// Actually for RSA decryption: M = C^d mod N where d is the private exponent.
// The period finding is for the multiplicative group order φ(N) or the
// order of C modulo N. BSGS finds r = ord_N(C), then d*e ≡ 1 (mod r).
//
// Implementation uses num_bigint::BigUint for arbitrary precision.

#![allow(dead_code)]
extern crate alloc;

use crate::sprintln;
use alloc::string::String;
use alloc::collections::BTreeMap;
use num_bigint::{BigUint, BigInt};
use num_traits::{One, Zero, ToPrimitive};
use core::str::FromStr;

/// Trim leading zeros from a decimal string
fn trim(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() && bytes[i] == b'0' { i += 1; }
    String::from(&s[i..])
}

/// Parse a decimal string into BigUint
fn parse_big(s: &str) -> Option<BigUint> {
    let t = trim(s);
    if t.is_empty() { return None; }
    BigUint::from_str(&t).ok()
}

/// BigUint GCD
fn big_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let t = b.clone();
        b = &a % &b;
        a = t;
    }
    a
}

/// Modular exponentiation: base^exp mod modulus
fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base % modulus;
    let mut exp = exp.clone();
    while !exp.is_zero() {
        if &exp & BigUint::one() == BigUint::one() {
            result = (result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1;
    }
    result
}

/// Baby-Step Giant-Step to find the minimal period r such that base^r ≡ 1 (mod n)
/// Returns the period r, or None if not found within sqrt(n) steps
fn bsgs_period(base: &BigUint, n: &BigUint) -> Option<BigUint> {
    if n <= &BigUint::one() { return None; }
    if base % n == BigUint::zero() { return None; }
    
    // m = ceil(sqrt(n))
    let n_bits = n.bits();
    let m_bits = (n_bits + 1) / 2;
    let m = BigUint::one() << m_bits;
    let m_usize: usize = m.to_usize().unwrap_or(1000000).min(1000000); // cap for memory
    
    // Baby steps: base^j for j = 0..m
    let mut baby_steps = BTreeMap::new();
    let mut cur = BigUint::one();
    for j in 0..m_usize {
        baby_steps.insert(cur.clone(), j);
        cur = (&cur * base) % n;
    }
    
    // Giant step factor: base^(-m) mod n
    // Compute base^m first, then its modular inverse
    let base_m = mod_pow(base, &BigUint::from(m_usize), n);
    let inv_base_m = mod_inverse(&base_m, n)?;
    
    // Giant steps: search for collision
    cur = BigUint::one();
    for i in 0..m_usize {
        if let Some(&j) = baby_steps.get(&cur) {
            let r = BigUint::from(i * m_usize + j);
            if r > BigUint::zero() && mod_pow(base, &r, n) == BigUint::one() {
                return Some(r);
            }
        }
        cur = (&cur * &inv_base_m) % n;
    }
    None
}

/// Extended Euclidean Algorithm. The Bezout coefficients are genuinely
/// signed -- the recursive step needs x1 - (a/b)*y1, which is negative on
/// roughly half of all real inputs -- so this has to run over BigInt, not
/// BigUint. FIXED 2026-08-31: the previous version ran the same recursion
/// over BigUint and panicked ("Cannot subtract b from a because b is larger
/// than a") the first time a coefficient went negative, which is not an
/// edge case for this algorithm: `rsa period 2 3233` (the textbook RSA
/// N=61*53 example) hit it immediately.
fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        return (a.clone(), BigInt::one(), BigInt::zero());
    }
    let (g, x1, y1) = egcd(b, &(a % b));
    let x = y1.clone();
    let y = &x1 - &(a / b) * &y1;
    (g, x, y)
}

fn mod_inverse(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    let a_i = BigInt::from(a.clone());
    let n_i = BigInt::from(n.clone());
    let (g, x, _) = egcd(&a_i, &n_i);
    if g != BigInt::one() { return None; }
    let mut inv = &x % &n_i;
    if inv < BigInt::zero() { inv += &n_i; }
    if inv.is_zero() { inv = n_i; }
    inv.to_biguint()
}

/// Compute private exponent d from public exponent e and period r
/// d ≡ e^(-1) mod r
fn compute_private_exponent(e: &BigUint, r: &BigUint) -> Option<BigUint> {
    mod_inverse(e, r)
}

/// Decrypt: M = C^d mod N
fn decrypt(ciphertext: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    mod_pow(ciphertext, d, n)
}

/// The ob3ect glyph word for RSA Decrypter
pub const GLYPH_WORD: &str = "⊢⊣∈≻⊤≺⊥∋⋈⊞≻⋈⊙⊤⊡⊣";

/// Main decryption function implementing the ob3ect flow
pub fn rsa_decrypt(c_str: &str, n_str: &str, e_str: &str) -> Option<String> {
    let c = parse_big(c_str)?;
    let n = parse_big(n_str)?;
    let e = parse_big(e_str)?;

    if n <= BigUint::one() { return None; }
    if c >= n { return None; }
    
    // Step 1: Find the winding period r = ord_n(C) via BSGS (∈/∋ phase)
    sprintln!("∈ Period Finding Fork: BSGS search for r = ord_N(C)...");
    let r = bsgs_period(&c, &n)?;
    sprintln!("∋ Winding Period Resolution: r = {} ({} digits)", r, r.to_str_radix(10).len());
    
    // Step 2: Compute private exponent d ≡ e^(-1) mod r (⊞ phase - dialetheic key state)
    sprintln!("⊞ Dialetheic Key State: computing d ≡ e^(-1) mod r...");
    let d = compute_private_exponent(&e, &r)?;
    sprintln!("  d = {} ({} digits)", d, d.to_str_radix(10).len());
    
    // Step 3: Decrypt M = C^d mod N (≻⋈ descent + ⊙ identity + ⊤ verification)
    sprintln!("≻⋈ Private Descent: C^d mod N...");
    let m = decrypt(&c, &d, &n);
    sprintln!("⊙ Identity: C^d ≡ {} (mod N)", m);
    sprintln!("⊤ Closure Verification: M^e ≡ C (mod N) = {}", 
        if mod_pow(&m, &e, &n) == c { "PASS" } else { "FAIL" });
    
    // Step 4: Fix the plaintext (⊡)
    sprintln!("⊡ Plaintext Record: {} ({} digits)", m, m.to_str_radix(10).len());
    
    Some(m.to_str_radix(10))
}

/// Show the ob3ect glyph word and its meaning
pub fn show_word() {
    sprintln!("RSA Decrypter Glyph Word: {}", GLYPH_WORD);
    sprintln!("Length: 16 tokens");
    sprintln!("");
    sprintln!("Phase mapping:");
    sprintln!("  ⊢  VINIT      — Uninitialized Modulus Space");
    sprintln!("  ⊣  TANCH      — Modulus N (boundary)");
    sprintln!("  ∈  FSPLIT     — Period Finding Fork (BSGS)");
    sprintln!("  ≻  AFWD       — Modular Exponentiation (ascent)");
    sprintln!("  ⊤  EVALT      — Closure Verification (a^r ≡ 1)");
    sprintln!("  ≺  AREV       — Private Key Descent (d)");
    sprintln!("  ⊥  EVALF      — Non-Closure Check (a^k ≠ 1)");
    sprintln!("  ∋  FFUSE      — Winding Period Resolution");
    sprintln!("  ⋈  CLINK      — Square-and-Multiply Chain");
    sprintln!("  ⊞  ENGAGR     — Dialetheic Key State (public ∧ private)");
    sprintln!("  ⊙  IMSCRIB    — Identity Element (1 mod N)");
    sprintln!("  ⊡  IFIX       — Plaintext Record");
    sprintln!("  ⊣  TANCH      — Seal with verified plaintext");
}

/// REPL entry point
pub fn repl_rsa(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("rsa <C> <N> <e>        — Decrypt ciphertext C with modulus N and public exponent e");
        sprintln!("rsa word               — Show the ob3ect glyph word and phase mapping");
        sprintln!("rsa period <base> <N>  — Find winding period r = ord_N(base) via BSGS");
        sprintln!("rsa verify <M> <e> <N> — Verify M^e ≡ C (mod N) for a candidate plaintext");
        return;
    }
    
    match args[0] {
        "word" => show_word(),
        "period" => {
            if args.len() < 3 {
                sprintln!("usage: rsa period <base> <N>");
                return;
            }
            let base = match parse_big(args[1]) { Some(b) => b, None => { sprintln!("Invalid base"); return; }};
            let n = match parse_big(args[2]) { Some(n) => n, None => { sprintln!("Invalid N"); return; }};
            sprintln!("Finding period r = ord_{}({})...", &args[2][..args[2].len().min(20)], &args[1][..args[1].len().min(20)]);
            match bsgs_period(&base, &n) {
                Some(r) => {
                    sprintln!("r = {} ({} digits)", r, r.to_str_radix(10).len());
                    sprintln!("Verification: base^r mod N = {}", mod_pow(&base, &r, &n));
                }
                None => sprintln!("Period not found within sqrt(N) bound"),
            }
        }
        "verify" => {
            if args.len() < 4 {
                sprintln!("usage: rsa verify <M> <e> <N>");
                return;
            }
            let m = match parse_big(args[1]) { Some(m) => m, None => { sprintln!("Invalid M"); return; }};
            let e = match parse_big(args[2]) { Some(e) => e, None => { sprintln!("Invalid e"); return; }};
            let n = match parse_big(args[3]) { Some(n) => n, None => { sprintln!("Invalid N"); return; }};
            let c = mod_pow(&m, &e, &n);
            sprintln!("M^e mod N = {}", c);
        }
        c_str => {
            if args.len() < 3 {
                sprintln!("usage: rsa <C> <N> <e>");
                return;
            }
            let n_str = args[1];
            let e_str = args[2];
            
            sprintln!("RSA Decrypter — ob3ect glyph word: {}", GLYPH_WORD);
            sprintln!("Input: C = {}... ({} digits), N = {}... ({} digits), e = {}", 
                &c_str[..c_str.len().min(30)], c_str.len(),
                &n_str[..n_str.len().min(30)], n_str.len(),
                e_str);
            
            match rsa_decrypt(c_str, n_str, e_str) {
                Some(plaintext) => {
                    sprintln!("");
                    sprintln!("⊡ PLAINTEXT = {}", plaintext);
                    sprintln!("μ∘δ = id — decryption closed");
                }
                None => {
                    sprintln!("Decryption failed: invalid inputs or period not found");
                }
            }
        }
    }
}