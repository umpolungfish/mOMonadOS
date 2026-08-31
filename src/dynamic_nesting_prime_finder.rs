// dynamic_nesting_prime_finder.rs — Dynamic Nesting Prime Finder
//
// ONESHOT PRINCIPLE
// =================
// Construct the object inside its own fixed-point closure. For a given N,
// search nesting depths d=1,2,3,... and find the MINIMAL d at which the
// Brent cycle closes in the depth-d closure geometry.
//
// The depth-d word:
//   d=1: ⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣                     (period 12)
//   d=2: ⊢∈⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣∋⊣                (period 17)
//   d=3: ⊢∈⊢∈⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣⊡⊣∋⊣           (period 22)
//   ...
//
// Period growth: P(d) = 5d + 7
//
// At each depth d, the Brent seed c is DERIVED from the closure geometry
// (not iterated toward it):
//   c_d = (P(d) + n mod 256) mod n, with c_d >= 1
//
// The seed carries the period signature, so the iterate starts inside the
// depth-d closure rather than searching toward it. The first d where the
// cycle yields a non-trivial factor is the OPTIMAL nesting depth.

#![allow(dead_code)]
extern crate alloc;

use crate::sprintln;
use super::prime_winding::is_prime;
use alloc::string::String;
use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive};
use core::str::FromStr;

const INNER_BODY: &str = "≻⊤≺⊥⋈⊙⊞∋";
const PERIOD_BASE: usize = 12;
const PERIOD_STEP: usize = 5;

pub fn word_at_depth(d: usize) -> String {
    if d == 0 { return String::new(); }
    let mut s = String::from("⊢∈");
    for _ in 0..(d - 1) { s.push_str("⊢∈"); }
    s.push_str(INNER_BODY);
    if d == 1 {
        s.push_str("⊡⊣");
    } else {
        s.push_str("⊡⊣");
        for _ in 0..(d - 2) { s.push_str("∋⊡⊣"); }
        s.push_str("∋⊣");
    }
    s
}

pub fn period_at_depth(d: usize) -> usize {
    if d == 0 { return 0; }
    PERIOD_BASE + PERIOD_STEP * (d - 1)
}

fn trim(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() && bytes[i] == b'0' { i += 1; }
    String::from(&s[i..])
}

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

fn brent_f(x: &BigUint, c: &BigUint, n: &BigUint) -> BigUint {
    let x2 = (x * x) % n;
    (&x2 + c) % n
}

fn big_to_string(b: &BigUint) -> String { b.to_str_radix(10) }

/// Construct the depth-d closure seed:
///   c_d = (P(d) + (n mod 256)) mod n, ensure c_d >= 1
///
/// The P(d) term is the depth-d word's period. Mixing the period into the
/// seed means the iterate starts inside the depth-d closure geometry.
pub fn closure_seed(n: &BigUint, d: usize) -> BigUint {
    let period = period_at_depth(d) as u64;
    let n_low = (n % BigUint::from(256u32)).to_u64().unwrap_or(0);
    let raw = period.wrapping_add(n_low);
    let raw = if raw == 0 { 1 } else { raw };
    BigUint::from(raw)
}

/// Brent's cycle detection with depth-bounded power budget.
/// Higher depth = higher power budget: 2^(8+d) max steps.
fn brent_factor_at_depth(n: &BigUint, c: &BigUint, depth: usize) -> Option<BigUint> {
    let two = BigUint::from(2u32);
    if n % two.clone() == BigUint::zero() { return Some(two.clone()); }
    if n.is_one() { return None; }

    let n_mod_97 = (n % BigUint::from(97u32)).to_u64().unwrap_or(0);
    let x_init = BigUint::from(2u64 + (n_mod_97 % 95));
    let mut x: BigUint = x_init.clone();
    let m: u64 = 128;
    let max_power: u64 = 1u64 << (8 + depth.min(20) as u64);
    let mut power: u64 = 1;
    #[allow(unused_assignments)] let mut y: BigUint = x.clone();

    while power < max_power {
        let mut step: u64 = 0;
        while step < power {
            x = brent_f(&x, c, n);
            step += 1;
        }
        y = x.clone();
        let mut j: u64 = 0;
        let upper = power.min(m);
        while j < upper {
            x = brent_f(&x, c, n);
            let diff = if x > y { &x - &y } else { &y - &x };
            let g = big_gcd(diff, n.clone());
            if !g.is_one() && g != *n { return Some(g); }
            j += 1;
        }
        power = match power.checked_mul(2) {
            Some(p) => p,
            None => break,
        };
    }
    None
}

/// Find the optimal nesting depth: minimal d at which the Brent cycle
/// closes in the depth-d closure geometry with a non-trivial factor.
/// Returns (depth, factor). depth=0 means "no closure found" or "trivial".
/// depth=1 with factor=None means "prime" (closure at the trivial d=1).
pub fn find_optimal_depth(n_str: &str, max_depth: usize) -> (usize, Option<BigUint>) {
    let n = match parse_big(n_str) {
        Some(n) if n > BigUint::one() => n,
        _ => return (0, None),
    };
    if is_prime(n_str) { return (1, None); }
    for d in 1..=max_depth {
        let c = closure_seed(&n, d);
        if let Some(factor) = brent_factor_at_depth(&n, &c, d) {
            return (d, Some(factor));
        }
    }
    (0, None)
}

pub fn repl_dyn(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("dyn_nest <N> [max_depth=8] — find optimal oneshot nesting depth");
        sprintln!("  Returns minimal d at which the Brent cycle closes in the");
        sprintln!("  depth-d closure geometry, yielding a non-trivial factor.");
        sprintln!("  Subcommands:");
        sprintln!("    dyn_nest word <d>         — show the depth-d word");
        sprintln!("    dyn_nest period <d>       — show the depth-d period P(d)=5d+7");
        sprintln!("    dyn_nest seed <N> <d>     — show the closure seed for depth d");
        sprintln!("    dyn_nest <N> [max_d]      — search optimal depth for N");
        return;
    }

    match args[0] {
        "word" => {
            let d: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            sprintln!("depth-{}: {} (period {})", d, word_at_depth(d), period_at_depth(d));
        }
        "period" => {
            let d: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            sprintln!("period(d={}) = 5*{} + 7 = {}", d, d, period_at_depth(d));
        }
        "seed" => {
            if args.len() < 3 {
                sprintln!("usage: dyn_nest seed <N> <d>");
                return;
            }
            let n = match parse_big(args[1]) {
                Some(n) => n,
                None => { sprintln!("Invalid N: {}", args[1]); return; }
            };
            let d: usize = match args[2].parse() {
                Ok(d) => d,
                Err(_) => { sprintln!("Invalid depth: {}", args[2]); return; }
            };
            let c = closure_seed(&n, d);
            sprintln!("closure_seed(d={}, n={}...) = {}", d, &args[1][..args[1].len().min(20)], c);
        }
        n_str => {
            let max_depth: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(8);

            let n = match parse_big(n_str) {
                Some(n) if n > BigUint::one() => n,
                _ => { sprintln!("Invalid or trivial N: {}", n_str); return; }
            };

            // PIPE: oneshot prime winder as pre-filter
            let oneshot = crate::oneshot_prime_winder::oneshot_verdict(n_str);
            match oneshot {
                crate::oneshot_prime_winder::B4Verdict::T => {
                    sprintln!("oneshot[winder] verdict: T (PRIME) — closure at d=0, no nesting");
                    sprintln!("  word(d=0) = (no fold required — already in fixed-point class)");
                    return;
                }
                crate::oneshot_prime_winder::B4Verdict::B => {
                    sprintln!("oneshot[winder] verdict: B (paradice) — N=1 is its own closure");
                    return;
                }
                crate::oneshot_prime_winder::B4Verdict::F => {
                    sprintln!("oneshot[winder] verdict: F (composite) — proceeding to dynamic nesting");
                }
            }

            sprintln!("Searching optimal nesting depth for N ({} digits), max_d={}",
                      big_to_string(&n).len(), max_depth);

            for d in 1..=max_depth {
                let c = closure_seed(&n, d);
                sprintln!("  d={}: word={} (period={})", d, word_at_depth(d), period_at_depth(d));
                match brent_factor_at_depth(&n, &c, d) {
                    Some(factor) => {
                        sprintln!("  → CLOSURE at d={}! factor = {}", d, factor);
                        sprintln!("  Witness: c={} (depth-d closure seed).", c);
                        sprintln!("  Result: μ∘δ = id at depth {} with non-trivial fold.", d);
                        return;
                    }
                    None => {
                        sprintln!("    no closure at d={} (cycle did not yield a factor)", d);
                    }
                }
            }
            sprintln!("No factor found up to depth {} (N may have large prime factors).", max_depth);
        }
    }
}
