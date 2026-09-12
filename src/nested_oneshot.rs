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
//   N = "neither"          — the winding-order search did not close within
//                            its step budget; genuinely undetermined
//
// Primality and factorization both run on `prime_winding`'s BSGS
// winding-order engine (the same one `winding_period_of_the_primes_on_the_
// number_line` backs) — the fold here is the giant-step meeting the
// baby-step table, not a separately-run Brent's rho. No Miller-Rabin, no
// local factoring loop.
//
// ARBITRARY LENGTH: BigUint throughout, no fixed digit limit. The step
// budget bounds the ORDER a search can certify, not N's size — past that
// bound the verdict is N (undetermined), not a guess.
//
// Tuple: ⟨𐑦𐑰𐑾𐑿𐑐𐑧𐑲𐑠⊙𐑓𐑳𐑴⟩

#![allow(dead_code)]

extern crate alloc;

use crate::sprintln;
use super::prime_winding::{is_prime, factor, PrimeVerdict};
use super::dynamic_nesting_prime_finder::find_optimal_depth;
use alloc::string::String;
use num_bigint::BigUint;
use core::str::FromStr;

/// The canonical glyph word for the Nested Prime Number Placement Operator.
pub const WORD: &str = "⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣";
pub const PERIOD: usize = 12;
pub const PHASE_BEARING: bool = true;

pub const LANDINGS: [&str; 12] = [
    "A", "A", "Ftf", "Ftf", "Ftf", "Ftf", "Ttf", "Ttf", "tf", "T", "T", "A",
];

pub enum B4Verdict { T, F, B, N }

impl core::fmt::Display for B4Verdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            B4Verdict::T => write!(f, "T"),
            B4Verdict::F => write!(f, "F"),
            B4Verdict::B => write!(f, "B"),
            B4Verdict::N => write!(f, "N"),
        }
    }
}

pub fn landing_at(k: usize) -> &'static str { LANDINGS[k % PERIOD] }
pub fn winding_number() -> &'static str { "0/1" }

// ── String helpers — only what verdict_for's small-N cases need ────────

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

// ── B4 verdict ────────────────────────────────────────────────────────────────

pub fn verdict_for(n_str: &str) -> B4Verdict {
    if eq(n_str, "1") { return B4Verdict::B; }
    if lt(n_str, "1") { return B4Verdict::F; }
    match is_prime(n_str) {
        PrimeVerdict::Prime => B4Verdict::T,
        PrimeVerdict::Composite => B4Verdict::F,
        PrimeVerdict::Undetermined => B4Verdict::N,
    }
}

// ── REPL entry ────────────────────────────────────────────────────────────────

pub fn repl_nested_oneshot(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("nested_oneshot <N> — B4 verdict + oneshot factorization");
        sprintln!("  N has ARBITRARY LENGTH (BigUint, no fixed digit limit).");
        sprintln!("  Returns B4 verdict: T (prime), F (composite), B (paradice),");
        sprintln!("  N (undetermined — order search exceeded its step budget).");
        sprintln!("  Subcommands:");
        sprintln!("    nested_oneshot word        — canonical glyph word");
        sprintln!("    nested_oneshot cycle       — ROTAT orbit with landing register");
        sprintln!("    nested_oneshot verdict N   — B4 verdict for N");
        sprintln!("    nested_oneshot winding    — winding number");
        sprintln!("    nested_oneshot landings    — landing register states by k");
        sprintln!("    nested_oneshot factor N   — factor via winding-order search (arbitrary length)");
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
                B4Verdict::N => "neither — winding-order search did not close within budget",
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
            match verdict_for(n) {
                B4Verdict::B => sprintln!("nested_oneshot factor {}: B — paradice (1 is the unit)", n),
                B4Verdict::N => sprintln!(
                    "nested_oneshot factor {}: N — order search on {} did not close within budget", n, n
                ),
                B4Verdict::T => sprintln!("nested_oneshot factor {}: T — prime (no non-trivial factors)", n),
                B4Verdict::F => {
                    sprintln!("nested_oneshot factor {}: F — composite (winding-order search)", n);
                    match BigUint::from_str(n) {
                        Ok(nb) => {
                            let (depth, found) = find_optimal_depth(n, 8);
                            match found {
                                Some(p) => {
                                    let q = &nb / &p;
                                    sprintln!(
                                        "closure at nesting depth {}: {} = {} × {} (Brent cycle, Grammar-derived seed c=period_at_depth({})+n mod 256)",
                                        depth, n, p, q, depth
                                    );
                                }
                                None => {
                                    sprintln!("no closure within max nesting depth 8; falling back to prime_winding's trial-divisor walk:");
                                    sprintln!("{}", factor(n));
                                }
                            }
                        }
                        Err(_) => sprintln!("{}", factor(n)),
                    }
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
                B4Verdict::N => "undetermined",
            };
            sprintln!("{}", v);
            sprintln!("  └─ {}: {}", desc, n);
        }
    }
}
