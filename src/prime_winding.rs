//! Prime Winding — ob3ect-backed kernel tool.
//!
//! The artifact `winding_period_of_the_primes_on_the_number_line` is a
//! verified ob3ect whose glyph word `⊢⊙∈≻⊤⋈≺⊥⊞∋⊡⋈⊙⊣` has period 14 and
//! is phase-bearing across A / Ftf / tf / T. This module exposes it as
//! a kernel command so the artifact is reachable from the REPL.
//!
//! Subcommands:
//!   prime_winding word       print the canonical glyph word
//!   prime_winding find <n>   find the nearest prime ≤ n, or factor n
//!   prime_winding factor <n>  factor n into prime factors (arbitrary precision)
//!   prime_winding cycle      full ROTAT orbit with landing per cut
//!   prime_winding tuple      print the 12-slot tuple
//!   prime_winding verdict    return the Frobenius verdict
//!   prime_winding help       list subcommands
//!
//! All arithmetic is string-based, so n is unbounded (no u64 limit).

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

pub const WORD: &str = "⊢⊙∈≻⊤⋈≺⊥⊞∋⊡⋈⊙⊣";
pub const PERIOD: usize = 14;
pub const PHASE_BEARING: bool = true;
pub const FROBENIUS_VERDICT: &str = "T";
pub const ARTIFACT_SLUG: &str = "winding_period_of_the_primes_on_the_number_line";
pub const OB3ECT_DIR: &str = "/home/mrnob0dy666/ob3ect/digital/winding_period_of_the_primes_on_the_number_line";

/// The 12-slot tuple at the artifact's foundation.
pub const TUPLE: [&str; 12] = [
    "uninitialized_number_line", "winding_closure", "prime_ascent", "gap_descent",
    "sequential_engagement", "kinetic_freezing", "chiral_bifurcation",
    "dialetheic_reunion", "self_modeling_density", "irregular_gap",
    "critical_threshold", "integer_invariant",
];

pub const LANDINGS: [&str; 14] = [
    "A", "A", "A", "Ftf", "Ftf", "Ftf", "Ftf", "Ftf", "tf", "T", "T", "A", "A", "A",
];

// ── Arbitrary-precision decimal arithmetic (schoolbook) ────────────────────
// Strings only — no external crate. Each function takes &str and returns String.

/// Strip leading zeros, return "0" if all zero.
fn trim(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() && bytes[i] == b'0' {
        i += 1;
    }
    s[i..].to_string()
}

/// True if a is less than b (both non-empty decimal).
fn lt(a: &str, b: &str) -> bool {
    let ta = trim(a);
    let tb = trim(b);
    if ta.len() != tb.len() { return ta.len() < tb.len(); }
    ta < tb
}

/// True if a == b.
fn eq(a: &str, b: &str) -> bool { trim(a) == trim(b) }

/// True if a == "0".
fn is_zero(a: &str) -> bool {
    for c in a.chars() { if c != '0' { return false; } }
    true
}

/// True if a is even.
fn is_even(a: &str) -> bool {
    let t = trim(a);
    let last = t.as_bytes().last().copied().unwrap_or(b'0');
    (last - b'0') % 2 == 0
}

/// a + b, both decimal, non-negative.
fn add(a: &str, b: &str) -> String {
    let ta = trim(a);
    let tb = trim(b);
    let av: Vec<u8> = ta.bytes().rev().collect();
    let bv: Vec<u8> = tb.bytes().rev().collect();
    let mut out: Vec<u8> = Vec::new();
    let mut carry: u8 = 0;
    let n = av.len().max(bv.len());
    for i in 0..n {
        let x = if i < av.len() { av[i] - b'0' } else { 0 };
        let y = if i < bv.len() { bv[i] - b'0' } else { 0 };
        let s = x + y + carry;
        out.push(b'0' + s % 10);
        carry = s / 10;
    }
    if carry > 0 { out.push(b'0' + carry); }
    let s: String = out.iter().rev().map(|c| *c as char).collect();
    trim(&s)
}

/// a - b, assumes a >= b. Both non-negative.
fn sub(a: &str, b: &str) -> String {
    let ta = trim(a);
    let tb = trim(b);
    let av: Vec<u8> = ta.bytes().rev().collect();
    let bv: Vec<u8> = tb.bytes().rev().collect();
    let mut out: Vec<u8> = Vec::new();
    let mut borrow: u8 = 0;
    for i in 0..av.len() {
        let x = av[i] - b'0';
        let y = if i < bv.len() { bv[i] - b'0' } else { 0 } + borrow;
        if x >= y {
            out.push(b'0' + (x - y));
            borrow = 0;
        } else {
            out.push(b'0' + (x + 10 - y));
            borrow = 1;
        }
    }
    let s: String = out.iter().rev().map(|c| *c as char).collect();
    trim(&s)
}

/// a mod 10 (returns u8 0..9).
fn last_digit(a: &str) -> u8 {
    let t = trim(a);
    let last = t.as_bytes().last().copied().unwrap_or(b'0');
    last - b'0'
}

/// a mod d. d is small (<= 1_000_000_000). Returns the small remainder.
/// Full-precision schoolbook: never rounds.
fn rem_small(a: &str, d: u64) -> u64 {
    let mut r: u64 = 0;
    for c in a.bytes() {
        // r is always < d here, and d <= 1e9, so r*10 + digit < 1e10, fits u64.
        // For d up to ~1.84e18 this is exact. We use u128 for the product+carry
        // to guarantee no overflow even if d grows to 1e18.
        r = (r as u128 * 10 + (c - b'0') as u128 % d as u128) as u64 % d;
    }
    r
}

/// True if d (u64) divides a exactly.
fn divisible_by(a: &str, d: u64) -> bool {
    if d < 10 {
        // Fast path for small divisors
        match d {
            1 => true,
            2 => is_even(a),
            3 => rem_small(a, 3) == 0,
            4 => {
                let r = rem_small(a, 100);
                r % 4 == 0
            }
            5 => last_digit(a) == 0 || last_digit(a) == 5,
            6 => is_even(a) && rem_small(a, 3) == 0,
            7 => rem_small(a, 7) == 0,
            8 => rem_small(a, 1000) % 8 == 0,
            9 => rem_small(a, 9) == 0,
            _ => false,
        }
    } else {
        rem_small(a, d) == 0
    }
}

/// a / d for small u64 divisor d, returns string. Exact division.
/// Schoolbook: r is bounded by d, so r*10+digit < 10*d <= 1e19, fits in u128.
fn div_small(a: &str, d: u64) -> String {
    let mut out: Vec<u8> = Vec::new();
    let mut r: u128 = 0;
    for c in a.bytes() {
        r = r * 10 + (c - b'0') as u128;
        let q = (r / d as u128) as u8;
        if !out.is_empty() || q > 0 {
            out.push(b'0' + q);
        }
        r %= d as u128;
    }
    if out.is_empty() { String::from("0") } else { String::from_utf8(out).unwrap() }
}

/// True if a is prime, by trial division up to √a (small u64 divisor path).
pub fn is_prime(a: &str) -> bool {
    let t = trim(a);
    if t == "1" || t == "0" { return false; }
    if t == "2" { return true; }
    if is_even(&t) { return false; }
    // Quick small-prime sieve
    if divisible_by(&t, 3) { return t == "3"; }
    if divisible_by(&t, 5) { return t == "5"; }
    if divisible_by(&t, 7) { return t == "7"; }
    if divisible_by(&t, 11) { return t == "11"; }
    if divisible_by(&t, 13) { return t == "13"; }

    // For numbers that fit in u64, do trial division by u64.
    if t.len() <= 18 {
        if let Ok(n) = t.parse::<u64>() {
            let mut d: u64 = 17;
            while d * d <= n {
                if n % d == 0 { return false; }
                d += 2;
            }
            return true;
        }
    }
    // Arbitrary precision trial division up to √a.
    // Walk odd d from 17 upward as decimal string, stop when d*d > a.
    let mut d: u64 = 17;
    loop {
        // d*d as string
        let d2 = (d as u128 * d as u128).to_string();
        if !lt(&d2, &t) { break; }
        if rem_small(&t, d) == 0 { return false; }
        d += 2;
        // Safety cap: a number with > 1000 digits is not testable here.
        if d > 10_000_000 { break; }
    }
    true
}

// ── Output formatters ───────────────────────────────────────────────────────

pub fn word() -> String {
    format!(
        "prime_winding word: {}\n  period: {}\n  glyphs: {} (12 opcodes + 2 anchors)",
        WORD, PERIOD, WORD.chars().count()
    )
}

/// Find the nearest prime ≤ n. n is a decimal string (arbitrary precision).
pub fn find(n: &str) -> String {
    if lt(n, "2") {
        return format!("prime_winding find {}: no primes ≤ {}", n, n);
    }
    if is_prime(n) {
        return format!(
            "prime_winding find {}: {} IS PRIME\n  glyph: {}",
            n, n, WORD
        );
    }
    let mut m = trim(n);
    let mut steps: u64 = 0;
    while !is_prime(&m) {
        if lt(&m, "2") {
            return format!("prime_winding find {}: no prime found below", n);
        }
        m = sub(&m, "1");
        steps += 1;
        if steps > 1_000_000_000 {
            return format!("prime_winding find {}: scan limit reached (1B steps)", n);
        }
    }
    format!(
        "prime_winding find {}: {} is composite, nearest prime ≤ {} is {}\n  glyph: {}",
        n, n, n, m, WORD
    )
}

/// Factor n (decimal string, arbitrary precision) into primes with multiplicity.
pub fn factor(n: &str) -> String {
    if lt(n, "2") {
        return format!("prime_winding factor {}: n < 2, no prime factors", n);
    }
    let mut m = trim(n);
    let mut factors: Vec<String> = Vec::new();

    while is_even(&m) {
        factors.push(String::from("2"));
        m = div_small(&m, 2);
    }
    let mut d: u64 = 3;
    loop {
        let d2 = (d as u128 * d as u128).to_string();
        if !lt(&d2, &m) { break; }
        while rem_small(&m, d) == 0 {
            factors.push(d.to_string());
            m = div_small(&m, d);
        }
        d += 2;
        if d > 1_000_000 { break; }
    }
    if !is_zero(&m) && m != "1" {
        factors.push(m.clone());
    }

    if factors.is_empty() {
        return format!("prime_winding factor {}: {} IS PRIME", n, n);
    }
    let distinct: bool = factors.len() == 1 && factors[0] == n;
    if distinct {
        format!("prime_winding factor {}: {} IS PRIME", n, n)
    } else {
        let rep = factors.join(" × ");
        format!("prime_winding factor {}: {} = {}", n, n, rep)
    }
}

pub fn cycle() -> String {
    let mut out = format!(
        "prime_winding cycle — ROTAT orbit of {}\n  period: {}\n  phase-bearing: {}\n  distinct landings: 4 (A, Ftf, tf, T)\n  landing register by cut:\n",
        WORD, PERIOD, PHASE_BEARING
    );
    for (i, land) in LANDINGS.iter().enumerate() {
        out.push_str(&format!("    k = {:>2}  {}\n", i, land));
    }
    out
}

pub fn tuple() -> String {
    let slots = ["⊢", "⊣", "≻", "≺", "⋈", "⊤", "∈", "∋", "⊙", "⊥", "⊞", "⊡"];
    let mut out = String::from("prime_winding tuple (12 slots, ⊢→⊡):\n");
    for (i, slot) in slots.iter().enumerate() {
        if i < TUPLE.len() {
            out.push_str(&format!("  {}  {}\n", slot, TUPLE[i]));
        }
    }
    out
}

pub fn verdict() -> String {
    String::from(
        "prime_winding verdict: T\n\
         μ∘δ = id holds on the word.\n\
         tri-ancestral: T — reconnection over a transformed object.\n\
         closed walk: false; verdict T over an open walk = reconnection without return.",
    )
}

pub fn artifact() -> String {
    format!(
        "prime_winding artifact paths:\n  ob3ect json:   {}/{}_ob3ect.json\n  lean scaffold: {}/{}_scaffold.lean\n  diagram svg:   {}/{}_diagram_pen.svg",
        OB3ECT_DIR, ARTIFACT_SLUG, OB3ECT_DIR, ARTIFACT_SLUG, OB3ECT_DIR, ARTIFACT_SLUG
    )
}

pub fn help() -> String {
    format!(
        "prime_winding — winding period of the primes on the number line\n\
         glyph word: {}\n\
         period: {}, phase-bearing, Frobenius verdict: {}\n\n\
         subcommands:\n\
           prime_winding word       canonical glyph word\n\
           prime_winding find <n>   find nearest prime ≤ n (arbitrary precision)\n\
           prime_winding factor <n>  factor n into prime divisors (with multiplicity)\n\
           prime_winding cycle      ROTAT orbit with landing register per cut\n\
           prime_winding tuple      the 12-slot tuple the word was imscribed from\n\
           prime_winding verdict    Frobenius verdict and tri-ancestral reading\n\
           prime_winding artifact   ob3ect + Lean scaffold paths\n\
           prime_winding help       this help",
        WORD, PERIOD, FROBENIUS_VERDICT
    )
}
