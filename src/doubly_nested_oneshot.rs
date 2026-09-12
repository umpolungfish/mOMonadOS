// doubly_nested_oneshot.rs — Doubly Nested Prime Number Placement Operator
//
// One level deeper than nested_oneshot. The word is the nested_oneshot word wrapped
// in an outer frame layer:
//   ⊢∈⊤⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣∋⊣
//   └── inner compute ──┘└─ outer hold (⊤ seed) ─┘
//
// Period: 17 (kernel-verified via imasm cycle; the ROTAT orbit under the
// frame open/close rules has period 17 — the outer hold frame deposits ⊤ first).
// Phase-bearing: true, 5 distinct landings; outer hold deposits before inner computes
//
// B4 verdicts (same as nested_oneshot):
//   T = "one-shot closure" — inner object is at fixed point → N is prime
//   F = "no closure"       — inner object outside basin → N is composite
//   B = "paradice"         — both conditions held simultaneously (N = 1)
//   N = "neither"          — winding-order search did not close within budget
//
// Primality and factorization both run on `prime_winding`'s BSGS
// winding-order engine, the same one the inner nested_oneshot word and the
// artifact `winding_period_of_the_primes_on_the_number_line` share. The
// outer frame now HOLDS a ⊤ deposit before the inner word computes, so the
// inner VINIT clear fires against a live register and inherits the work (2 live clears).
//
// Tuple: ⟨𐑦𐑰𐑾𐑿𐑐𐑧𐑲𐑠⊙𐑓𐑳𐑴⟩ (same tuple as nested_oneshot, but word has depth 2)

#![allow(dead_code)]

extern crate alloc;

use crate::sprintln;
use super::prime_winding::{is_prime, factor, PrimeVerdict};
use super::dynamic_nesting_prime_finder::find_optimal_depth;
use alloc::string::String;
use num_bigint::BigUint;
use core::str::FromStr;

/// The doubly-nested glyph word (inner word wrapped in outer frame).
pub const WORD: &str = "⊢∈⊤⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣∋⊣";
pub const PERIOD: usize = 17;
pub const PHASE_BEARING: bool = true;

/// Landing register states at each ROTAT cut k=0..16.
/// 5 distinct states: A, Ftf, Ttf, tf, T
/// (kernel-verified: period=17, 5 distinct, final=A, banked=OK, 2 live clears)
pub const LANDINGS: [&str; 17] = [
    "A",   // k=0
    "A",   // k=1
    "A",   // k=2
    "A",   // k=3
    "A",   // k=4
    "Ftf", // k=5
    "Ftf", // k=6
    "Ftf", // k=7
    "Ftf", // k=8
    "Ttf", // k=9
    "Ttf", // k=10
    "tf",  // k=11
    "T",   // k=12
    "T",   // k=13
    "A",   // k=14
    "A",   // k=15
    "A",   // k=16
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

pub fn repl_doubly_nested_oneshot(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("doubly_nested_oneshot <N> — one level deeper than nested_oneshot");
        sprintln!("  Word: ⊢∈⊤⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣∋⊣  (period 17, 5 distinct landings)");
        sprintln!("  N has ARBITRARY LENGTH (BigUint, no fixed digit limit).");
        sprintln!("  Returns B4 verdict: T (prime), F (composite), B (paradice),");
        sprintln!("  N (undetermined — order search exceeded its step budget).");
        sprintln!("  Subcommands:");
        sprintln!("    dnos word        — canonical glyph word");
        sprintln!("    dnos cycle       — ROTAT orbit with landing register");
        sprintln!("    dnos verdict N   — B4 verdict for N");
        sprintln!("    dnos winding    — winding number");
        sprintln!("    dnos landings    — landing register states by k");
        sprintln!("    dnos factor N   — factor via winding-order search (arbitrary length)");
        sprintln!("  Examples:");
        sprintln!("    dnos 17");
        sprintln!("    dnos factor 1234567");
        sprintln!("    dnos factor 99999999999999999999999999999999999999999");
        return;
    }

    let sub = args[0];
    match sub {
        "word" => {
            sprintln!("word            : {}", WORD);
            sprintln!("period          : {}", PERIOD);
            sprintln!("phase-bearing   : {}", PHASE_BEARING);
            sprintln!("nesting depth   : 2 (outer frame wraps inner)");
        }
        "cycle" => {
            sprintln!("word: {}   period {}", WORD, PERIOD);
            sprintln!("  k  landing");
            sprintln!("  ─── ───────");
            for k in 0..PERIOD { sprintln!("  {:3}  {}", k, landing_at(k)); }
            sprintln!("5 distinct landings: A, Ftf, Ttf, tf, T");
        }
        "verdict" => {
            if args.len() < 2 { sprintln!("Usage: dnos verdict <N>"); return; }
            let n = args[1];
            let v = verdict_for(n);
            let desc = match &v {
                B4Verdict::T => "one-shot closure (prime)",
                B4Verdict::F => "no closure (composite)",
                B4Verdict::B => "paradice (neither prime nor composite)",
                B4Verdict::N => "neither — winding-order search did not close within budget",
            };
            sprintln!("doubly_nested_oneshot verdict {}: {}", n, v);
            sprintln!("  └─ {}", desc);
        }
        "winding" => { sprintln!("winding: {}", winding_number()); }
        "landings" => {
            sprintln!("landing register by ROTAT cut (k=0..16):");
            for k in 0..PERIOD { sprintln!("  k={:2}: {}", k, landing_at(k)); }
        }
        "factor" => {
            if args.len() < 2 { sprintln!("Usage: dnos factor <N>"); return; }
            let n = args[1];
            match verdict_for(n) {
                B4Verdict::B => sprintln!("doubly_nested_oneshot factor {}: B — paradice (1 is the unit)", n),
                B4Verdict::N => sprintln!(
                    "doubly_nested_oneshot factor {}: N — order search on {} did not close within budget", n, n
                ),
                B4Verdict::T => sprintln!("doubly_nested_oneshot factor {}: T — prime (no non-trivial factors)", n),
                B4Verdict::F => {
                    sprintln!("doubly_nested_oneshot factor {}: F — composite (winding-order search)", n);
                    match BigUint::from_str(n) {
                        Ok(nb) => {
                            let (depth, found) = find_optimal_depth(n, 10);
                            match found {
                                Some(p) => {
                                    let q = &nb / &p;
                                    sprintln!(
                                        "closure at nesting depth {}: {} = {} × {} (Brent cycle, Grammar-derived seed c=period_at_depth({})+n mod 256)",
                                        depth, n, p, q, depth
                                    );
                                }
                                None => {
                                    sprintln!("no closure within max nesting depth 10; falling back to prime_winding's trial-divisor walk:");
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
            sprintln!("  └─ {}: {} (doubly-nested)", desc, n);
        }
    }
}
