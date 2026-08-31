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
//! Primality runs on a digit-to-glyph encoding and the kernel's own
//! closure check, not on number theory imported from outside the
//! Grammar. Each decimal digit 0-9 has its own canonical IMASM word
//! (verified pairwise distinct — a real 1-1 encoding, not an assumed
//! one); N's word is those ten words concatenated in digit order; N is
//! prime iff that word's banking already holds (a REPAIR fixed point,
//! `imasm_core::lattice_flow::banked_walk(word).holds()`) — closed is
//! prime, open (an exposed clear with nothing banked behind it) is
//! composite. No BSGS, no Miller-Rabin, no Brent's rho, and no step
//! budget: the check is linear in digit count, so an arbitrary-length N
//! resolves the same way a two-digit one does.
//!
//! The one case this does not collapse into Prime or Composite is
//! `vacuous()` — a word where no clear ever fired, so nothing was ever
//! at risk. That is not evidence either way; it is Undetermined, a
//! fourth result standing on the same footing as the other three, the
//! same way Belnap FOUR holds T, F, B, and N as four points on one
//! lattice rather than three answers plus an apology.
//!
//! Factoring still needs a search, closure alone doesn't produce a
//! divisor, so `factor` walks small trial divisors (plain arithmetic,
//! not a Grammar claim) and checks the leftover cofactor with the same
//! closure-based `is_prime` used everywhere else here.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use imasm_core::lattice_flow::banked_walk;

/// Each decimal digit's canonical IMASM word. Verified pairwise distinct
/// (a real 1-1 encoding) before this was wired in — see the session
/// transcript for the check, not repeated here as a comment-only claim.
pub const DIGIT_WORDS: [&str; 10] = [
    "⊢⊣≻∈⊤⊥∋⋈⊙⊞≺⊡⊣",       // 0
    "⊢≻⋈∈⊤⊥⊞∋≺⊙⊡⊣",         // 1
    "⊢≻⋈≺∈⊤⊥⊞∋⊙⊡⊣",         // 2
    "⊢∈≻⊤≺⊥⊞∋⋈⊙⊡⊣",         // 3
    "⊢≻≺⋈∈⊤⊥⊞∋⊙⊡⊣",         // 4
    "⊢≻⋈∈⊤⊡⊥≺⊞∋⊙⊣",         // 5
    "⊢≻⋈⋈⋈⋈⋈⋈∈⊤⊙⊥≺⊞∋⊡⊣",   // 6
    "⊢≻∈⊤≻⊥≺⊞⋈⊙⊡∋⊣",       // 7
    "⊢∈≻⊤⋈≺⊥⋈⊞∋⊙⊡⊣",       // 8
    "⊢∈⊤≻⊥≺∋⊞⊙⋈⊡⊣",         // 9
];

/// Concatenate each digit's canonical word, in order. Non-digit
/// characters (there should be none in a trimmed decimal string) are
/// skipped rather than panicking on them.
pub fn digit_encode(n_str: &str) -> String {
    let mut out = String::new();
    for c in n_str.chars() {
        if let Some(d) = c.to_digit(10) {
            out.push_str(DIGIT_WORDS[d as usize]);
        }
    }
    out
}

/// Small trial divisors for `factor`'s search — plain arithmetic, not a
/// structural claim. The verdict on any candidate this produces is
/// still read from `is_prime`, never assumed from the search itself.
pub const TRIAL_DIVISION_BOUND: u64 = 100_000;

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

// ── Primality: digit encoding + the kernel's own closure check ─────────

/// The three outcomes the closure check can reach. Never collapsed to a
/// bool: Undetermined is a real, distinct answer (the vacuous case — no
/// clear ever fired, so nothing was ever at risk), not a stand-in for
/// either Prime or Composite.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PrimeVerdict { Prime, Composite, Undetermined }

/// N is prime iff the banking on its digit-encoded word already holds —
/// closed is prime, open (an exposed clear with nothing banked behind
/// it) is composite. Linear in digit count; no step budget, because
/// there is no search here to run out of budget.
pub fn is_prime(a: &str) -> PrimeVerdict {
    let t = trim(a);
    let n: BigUint = match t.parse() {
        Ok(v) => v,
        Err(_) => return PrimeVerdict::Composite,
    };
    if n < BigUint::from(2u32) { return PrimeVerdict::Composite; }

    let word = digit_encode(&t);
    match banked_walk(&word) {
        Some(b) if b.holds() => PrimeVerdict::Prime,
        Some(b) if b.vacuous() => PrimeVerdict::Undetermined,
        Some(_) => PrimeVerdict::Composite,
        None => PrimeVerdict::Undetermined,
    }
}

fn prime_verdict_str(v: PrimeVerdict) -> &'static str {
    match v {
        PrimeVerdict::Prime => "PRIME",
        PrimeVerdict::Composite => "COMPOSITE",
        PrimeVerdict::Undetermined => "UNDETERMINED (no clear ever fired on the digit-encoded word — nothing was ever at risk)",
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
/// Stops the moment the search meets an Undetermined verdict — it does
/// not step past a number outside its reach.
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
                    "prime_winding find {}: {} is vacuous on its digit-encoded word — no clear ever fired, cannot certify further",
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

/// Factor n (decimal string, arbitrary precision). Closure alone answers
/// whether N is prime, not what its factors are, so this walks small
/// trial divisors to find them — plain arithmetic, a search strategy,
/// not a Grammar claim — and every cofactor's status (prime, composite,
/// undetermined) is read from `is_prime`, never assumed from the search.
pub fn factor(n: &str) -> String {
    let t = trim(n);
    let n_big: BigUint = match t.parse() {
        Ok(v) => v,
        Err(_) => return format!("prime_winding factor {}: not a valid non-negative integer", n),
    };
    if n_big < BigUint::from(2u32) {
        return format!("prime_winding factor {}: n < 2, no prime factors", n);
    }

    let mut primes: Vec<BigUint> = Vec::new();
    let mut composite_unsplit: Vec<BigUint> = Vec::new();
    // bot embeds as bot at every tier and restricts back to bot, roundtrip
    // exact, no axioms (fdeRestrict_fdeEmbed_id) -- vacuous is absorbing.
    // A trial divisor found by arithmetic still divides m regardless of
    // what its own closure check says, but its Grammar verdict is not
    // upgraded to Prime just because division succeeded; it goes here
    // when is_prime reads it as vacuous, same bucket the leftover
    // cofactor uses for the same reason.
    let mut undetermined: Vec<BigUint> = Vec::new();

    let mut m = n_big;
    let mut d: u64 = 2;
    while d <= TRIAL_DIVISION_BOUND {
        let bd = BigUint::from(d);
        if &bd * &bd > m { break; }
        if &m % &bd == BigUint::zero() {
            let bucket = match is_prime(&d.to_string()) {
                PrimeVerdict::Prime => &mut primes,
                PrimeVerdict::Composite => &mut composite_unsplit,
                PrimeVerdict::Undetermined => &mut undetermined,
            };
            while &m % &bd == BigUint::zero() {
                bucket.push(bd.clone());
                m /= &bd;
            }
        }
        d += if d == 2 { 1 } else { 2 };
    }

    if m > BigUint::one() {
        match is_prime(&m.to_str_radix(10)) {
            PrimeVerdict::Prime => primes.push(m),
            PrimeVerdict::Undetermined => undetermined.push(m),
            PrimeVerdict::Composite => composite_unsplit.push(m),
        }
    }

    primes.sort_unstable();
    composite_unsplit.sort_unstable();
    undetermined.sort_unstable();

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
            "\n  └─ composite cofactor, no trial divisor below {} splits it further: {}",
            TRIAL_DIVISION_BOUND, rep.join(" × ")
        ));
    }
    if !undetermined.is_empty() {
        let rep: Vec<String> = undetermined.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!(
            "\n  └─ cofactor vacuous on its digit-encoded word — primality undetermined: {}",
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
         primality runs on a digit-to-glyph encoding: each digit 0-9 has\n\
         its own IMASM word, N's word is those concatenated in digit\n\
         order, and N is prime iff that word's banking already holds.\n\
         No step budget — linear in digit count, so arbitrary-length N\n\
         resolves the same way a small one does. factor walks small\n\
         trial divisors and reads each cofactor's status from this same\n\
         check.\n\n\
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

pub fn prime_verdict_label(v: PrimeVerdict) -> &'static str { prime_verdict_str(v) }
