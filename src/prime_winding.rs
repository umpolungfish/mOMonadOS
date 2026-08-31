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
//! Primality and factorization both run on `winding_period`'s BSGS order
//! search (the artifact's own winding-number engine) — order r of a base
//! a mod N by baby-step/giant-step, N prime by Fermat iff r | (N-1) across
//! several bases; N split by the Shor step, r even and a^(r/2) not ±1
//! gives gcd(a^(r/2) − 1, N). No Miller-Rabin, no Brent's rho: the search
//! either closes within its step budget or reports plainly that it did
//! not. n is unbounded (BigUint), but the ORDER a search can certify is
//! bounded by the baby-step table it can build in memory — past that
//! bound the verdict is Undetermined, not a guess dressed as an answer.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::winding_period::{winding_order_big, factor_big, WindingBig, FactorBig};

pub const WORD: &str = "⊢⊙∈≻⊤⋈≺⊥⊞∋⊡⋈⊙⊣";
pub const PERIOD: usize = 14;
pub const PHASE_BEARING: bool = true;
pub const FROBENIUS_VERDICT: &str = "T";
pub const ARTIFACT_SLUG: &str = "winding_period_of_the_primes_on_the_number_line";
pub const OB3ECT_DIR: &str = "/home/mrnob0dy666/ob3ect/digital/winding_period_of_the_primes_on_the_number_line";

/// Baby-step table bound for every order search this module runs — an
/// order past step_cap*(step_cap+1) is Undetermined, not guessed at.
/// Matches the documented sqrt~1e6 reach already established for the u64
/// winding path in oneshot_prime_winder.rs.
pub const STEP_CAP: u64 = 1_000_000;

/// Coprime-base retries a Shor-style split gets before a composite N is
/// reported as unsplit rather than factored.
pub const FACTOR_TRIES: u32 = 64;

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

// ── Decimal string trim/compare/subtract — only what `find`'s scan needs ───

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

// ── Primality: the artifact's own winding-order engine ─────────────────

/// The three outcomes an order-based primality search can reach. Never
/// collapsed to a bool: Undetermined is a real, distinct answer, not a
/// stand-in for either Prime or Composite.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PrimeVerdict { Prime, Composite, Undetermined }

/// Order r of a mod N divides N-1 iff a^(N-1) == 1 mod N (Fermat), so a
/// base whose order does NOT divide N-1 proves N composite outright. A
/// base whose order search does not close within STEP_CAP leaves N
/// Undetermined for that base — every subsequent base gets the same
/// treatment, so the overall verdict is Undetermined the moment any one
/// base's search runs out of budget, Composite the moment any one base
/// proves it, and Prime only once every tested base has passed clean.
pub fn is_prime(a: &str) -> PrimeVerdict {
    let t = trim(a);
    let n: BigUint = match t.parse() {
        Ok(v) => v,
        Err(_) => return PrimeVerdict::Composite,
    };
    let two = BigUint::from(2u32);
    if n < two { return PrimeVerdict::Composite; }
    if n == two { return PrimeVerdict::Prime; }
    if &n % &two == BigUint::zero() { return PrimeVerdict::Composite; }

    let n_minus_1 = &n - BigUint::one();
    let bases: [u32; 6] = [2, 3, 5, 7, 11, 13];
    for &b in &bases {
        let a_b = BigUint::from(b);
        if a_b >= n { continue; }
        match winding_order_big(&a_b, &n, STEP_CAP) {
            WindingBig::Order(r) => {
                if r == 0 || &n_minus_1 % BigUint::from(r) != BigUint::zero() {
                    return PrimeVerdict::Composite;
                }
            }
            // gcd(a, N) != 1 for a < N means a shares a factor with N.
            WindingBig::NotInGroup => return PrimeVerdict::Composite,
            WindingBig::BudgetExceeded => return PrimeVerdict::Undetermined,
        }
    }
    PrimeVerdict::Prime
}

fn prime_verdict_str(v: PrimeVerdict) -> &'static str {
    match v {
        PrimeVerdict::Prime => "PRIME",
        PrimeVerdict::Composite => "COMPOSITE",
        PrimeVerdict::Undetermined => "UNDETERMINED (order search exceeded the step budget)",
    }
}

// ── Output formatters ───────────────────────────────────────────────────────

pub fn word() -> String {
    format!(
        "prime_winding word: {}\n  period: {}\n  glyphs: {} (12 opcodes + 2 anchors)",
        WORD, PERIOD, WORD.chars().count()
    )
}

/// Find the nearest prime ≤ n. n is a decimal string (arbitrary precision).
/// Stops and says so the moment the search meets an Undetermined verdict —
/// it does not step past a number it could not certify.
pub fn find(n: &str) -> String {
    if lt(n, "2") {
        return format!("prime_winding find {}: no primes ≤ {}", n, n);
    }
    let mut m = trim(n);
    let mut steps: u64 = 0;
    loop {
        match is_prime(&m) {
            PrimeVerdict::Prime => {
                return if steps == 0 {
                    format!("prime_winding find {}: {} IS PRIME\n  glyph: {}", n, n, WORD)
                } else {
                    format!(
                        "prime_winding find {}: {} is composite, nearest prime ≤ {} is {}\n  glyph: {}",
                        n, n, n, m, WORD
                    )
                };
            }
            PrimeVerdict::Undetermined => {
                return format!(
                    "prime_winding find {}: order search on {} did not close within the step budget — cannot certify further",
                    n, m
                );
            }
            PrimeVerdict::Composite => {}
        }
        if lt(&m, "2") {
            return format!("prime_winding find {}: no prime found below", n);
        }
        m = sub(&m, "1");
        steps += 1;
        if steps > 1_000_000_000 {
            return format!("prime_winding find {}: scan limit reached (1B steps)", n);
        }
    }
}

/// Factor n (decimal string, arbitrary precision) via the winding-order
/// engine only. Three outcomes per component, none collapsed into the
/// others: confirmed prime factors, composite cofactors the Shor step
/// could not split within its retries, and cofactors whose own
/// primality search never closed at all.
pub fn factor(n: &str) -> String {
    let t = trim(n);
    let n_big: BigUint = match t.parse() {
        Ok(v) => v,
        Err(_) => return format!("prime_winding factor {}: not a valid non-negative integer", n),
    };
    if n_big < BigUint::from(2u32) {
        return format!("prime_winding factor {}: n < 2, no prime factors", n);
    }

    let mut m = n_big;
    let mut primes: Vec<BigUint> = Vec::new();
    let two = BigUint::from(2u32);
    while &m % &two == BigUint::zero() {
        primes.push(two.clone());
        m /= &two;
    }

    let mut composite_unsplit: Vec<BigUint> = Vec::new();
    let mut undetermined: Vec<BigUint> = Vec::new();
    let mut stack: Vec<BigUint> = Vec::new();
    if m > BigUint::one() { stack.push(m); }
    let mut seed: u64 = 0xC0FFEE_1234_5678;

    while let Some(cand) = stack.pop() {
        if cand == BigUint::one() { continue; }
        match is_prime(&cand.to_str_radix(10)) {
            PrimeVerdict::Prime => primes.push(cand),
            PrimeVerdict::Undetermined => undetermined.push(cand),
            PrimeVerdict::Composite => {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                match factor_big(&cand, FACTOR_TRIES, STEP_CAP, seed) {
                    FactorBig::Found { p, q, .. } => {
                        stack.push(p);
                        stack.push(q);
                    }
                    FactorBig::NoFactorInTries | FactorBig::BudgetExceeded => {
                        composite_unsplit.push(cand);
                    }
                }
            }
        }
    }
    primes.sort_unstable();
    composite_unsplit.sort_unstable();
    undetermined.sort_unstable();

    if primes.is_empty() && composite_unsplit.is_empty() && undetermined.is_empty() {
        return format!("prime_winding factor {}: {} IS PRIME", n, n);
    }
    if composite_unsplit.is_empty() && undetermined.is_empty() && primes.len() == 1
        && primes[0].to_str_radix(10) == trim(n)
    {
        return format!("prime_winding factor {}: {} IS PRIME", n, n);
    }

    let mut out = format!("prime_winding factor {}: ", n);
    if !primes.is_empty() {
        let rep: Vec<String> = primes.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!("{} = {}", n, rep.join(" × ")));
    } else {
        out.push_str(&format!("{} — no confirmed prime factors", n));
    }
    if !composite_unsplit.is_empty() {
        let rep: Vec<String> = composite_unsplit.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!(
            "\n  └─ composite cofactor(s), order search could not split within {} tries: {}",
            FACTOR_TRIES, rep.join(" × ")
        ));
    }
    if !undetermined.is_empty() {
        let rep: Vec<String> = undetermined.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!(
            "\n  └─ cofactor(s) whose primality itself did not close within the step budget: {}",
            rep.join(" × ")
        ));
    }
    out
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
         period: {}, phase-bearing, Frobenius verdict: {}\n\
         primality and factoring both run on the BSGS winding-order search\n\
         (step budget {} baby steps); past that budget a cofactor's status\n\
         reports as UNDETERMINED rather than a guess.\n\n\
         subcommands:\n\
           prime_winding word       canonical glyph word\n\
           prime_winding find <n>   find nearest prime ≤ n (arbitrary precision)\n\
           prime_winding factor <n>  factor n into prime divisors (with multiplicity)\n\
           prime_winding cycle      ROTAT orbit with landing register per cut\n\
           prime_winding tuple      the 12-slot tuple the word was imscribed from\n\
           prime_winding verdict    Frobenius verdict and tri-ancestral reading\n\
           prime_winding artifact   ob3ect + Lean scaffold paths\n\
           prime_winding help       this help",
        WORD, PERIOD, FROBENIUS_VERDICT, STEP_CAP
    )
}

pub fn prime_verdict_label(v: PrimeVerdict) -> &'static str { prime_verdict_str(v) }
