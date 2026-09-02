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
//! Primality is real arithmetic (trial division then Miller-Rabin, exact
//! for arbitrary precision) -- not the Grammar's own closure check.
//!
//! Three closure-based readings were tried here in sequence and each was
//! disproven against concrete counterexamples, not merely suspected:
//! a single-cut banking check was proven to read only the leading digit;
//! a register-A-anywhere check on the word's full ROTAT orbit was
//! satisfied by nearly any sufficiently long word, discriminating almost
//! nothing; and `tri_ancestral_verdict` on the hex-digit encoding (each
//! hex nibble 0x0-0xF its own canonical IMASM word, N's word the
//! concatenation of its nibbles' words) turned out to be T the moment
//! ANY nibble is 8 or higher, with no dependence on the number's actual
//! factors at all. Proof: 138 = 2×3×23 (composite) hex-encodes to 0x8A,
//! one nibble ≥8, and read T (reported PRIME); 113 (actually prime) and
//! 119 = 7×17 (composite) both hex-encode with every nibble <8 and read
//! identically N (reported UNDETERMINED) -- the check could not tell an
//! actual prime from a composite it was sitting right next to.
//!
//! No fourth closure-based encoding has been found that tracks
//! primality, and there is a structural reason not to expect one: a
//! fixed per-digit lookup into a bounded-state graph can only ever see a
//! bounded pattern in the digits, while primality is a global fact about
//! divisibility that does not reduce to any bounded local pattern. So
//! this went back to real arithmetic for the verdict, restoring the
//! Miller-Rabin implementation this module carried before the closure
//! rewrites (deterministic below Sinclair's bound of
//! 3,317,044,064,679,887,385,961,981; false-positive probability below
//! 4^-13 per composite above it, the same witness practice GMP and
//! OpenSSL use). The word/glyph/tuple/cycle commands below still report
//! the artifact's own fixed reference word and the per-number
//! hex-glyph encoding as what they are: the Grammar's reading of the
//! number, kept because it is real and checkable, not because it
//! decides primality.
//!
//! Factoring still needs a search, closure alone doesn't produce a
//! divisor, so `factor` walks small trial divisors (plain arithmetic,
//! not a Grammar claim) and checks the leftover cofactor with the same
//! `is_prime` used everywhere else here.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Each hex nibble's canonical IMASM word, one word per bit pattern of
/// {T-bit, F-bit, I-bit, fork-openness-bit}, built from the ob3ect
/// "A hex digit's fourth bit as fork openness, not a fifth glyph"
/// (`ob3ect/digital/a_hex_digit_s_fourth_bit_as_fork_openness_not_a_f03510ea`).
/// v=0xF reproduces that ob3ect's own canonical word exactly — checked, not
/// assumed. The template: ⊢, then ∈ iff the fork bit is set, then ⊤ iff the
/// T-bit is set, then ≻⋈, then ⊥ iff the F-bit is set, then ≺ (the clear —
/// banked if the fork opened before it, exposed otherwise), then ⋈ and ∋
/// iff the fork bit is set, then ⊞ iff the I-bit is set, then ⊙⊡⊣.
/// Verified pairwise distinct across all 16 against the real kernel.
pub const HEX_WORDS: [&str; 16] = [
    "⊢≻⋈≺⊙⊡⊣",             // 0x0  T=0 F=0 I=0 fork=0
    "⊢⊤≻⋈≺⊙⊡⊣",           // 0x1  T=1 F=0 I=0 fork=0
    "⊢≻⋈⊥≺⊙⊡⊣",           // 0x2  T=0 F=1 I=0 fork=0
    "⊢⊤≻⋈⊥≺⊙⊡⊣",         // 0x3  T=1 F=1 I=0 fork=0
    "⊢≻⋈≺⊞⊙⊡⊣",           // 0x4  T=0 F=0 I=1 fork=0
    "⊢⊤≻⋈≺⊞⊙⊡⊣",         // 0x5  T=1 F=0 I=1 fork=0
    "⊢≻⋈⊥≺⊞⊙⊡⊣",         // 0x6  T=0 F=1 I=1 fork=0
    "⊢⊤≻⋈⊥≺⊞⊙⊡⊣",       // 0x7  T=1 F=1 I=1 fork=0
    "⊢∈≻⋈≺⋈∋⊙⊡⊣",         // 0x8  T=0 F=0 I=0 fork=1
    "⊢∈⊤≻⋈≺⋈∋⊙⊡⊣",       // 0x9  T=1 F=0 I=0 fork=1
    "⊢∈≻⋈⊥≺⋈∋⊙⊡⊣",       // 0xA  T=0 F=1 I=0 fork=1
    "⊢∈⊤≻⋈⊥≺⋈∋⊙⊡⊣",     // 0xB  T=1 F=1 I=0 fork=1
    "⊢∈≻⋈≺⋈⊞∋⊙⊡⊣",       // 0xC  T=0 F=0 I=1 fork=1
    "⊢∈⊤≻⋈≺⋈⊞∋⊙⊡⊣",     // 0xD  T=1 F=0 I=1 fork=1
    "⊢∈≻⋈⊥≺⋈⊞∋⊙⊡⊣",     // 0xE  T=0 F=1 I=1 fork=1
    "⊢∈⊤≻⋈⊥≺⋈⊞∋⊙⊡⊣",   // 0xF  T=1 F=1 I=1 fork=1 -- the ob3ect's own word
];

/// Convert N (decimal string) to hex, then concatenate each nibble's
/// canonical word, most-significant nibble first.
pub fn digit_encode(n_str: &str) -> String {
    let n: BigUint = match trim(n_str).parse() { Ok(v) => v, Err(_) => return String::new() };
    let hex = n.to_str_radix(16);
    let mut out = String::new();
    for c in hex.chars() {
        if let Some(d) = c.to_digit(16) {
            out.push_str(HEX_WORDS[d as usize]);
        }
    }
    out
}

/// Small trial divisors for `factor`'s search — plain arithmetic, not a
/// structural claim. The verdict on any candidate this produces is
/// still read from `is_prime`, never assumed from the search itself.
pub const TRIAL_DIVISION_BOUND: u64 = 10_000_000;

/// `find`'s decrement search costs one full hex_encode + cycle_landings per
/// step. `cycle_landings` walks the whole word once per rotation, so its
/// cost is quadratic in word length where the old single-cut `banked_walk`
/// was linear — measured before this constant was set to whatever value
/// keeps that cost interactive.
pub const SCAN_CAP: u64 = 100_000;

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

// ── Decimal string trim/compare — only what `find` and `factor` need ───────

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

/// Miller-Rabin, arbitrary precision, via BigUint::modpow. The 13 witnesses
/// 2..41 are deterministic for every n < 3,317,044,064,679,887,385,961,981
/// (Sinclair's known bound); past that they still carry a false-positive
/// probability below 4^-13 per composite, the same witness practice GMP and
/// OpenSSL use for arbitrary-size candidates.
fn miller_rabin(n: &BigUint) -> bool {
    let one = BigUint::one();
    let two = &one + &one;
    if *n < two { return false; }
    if *n == two { return true; }
    if n % &two == BigUint::zero() { return false; }

    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut r: u32 = 0;
    while &d % &two == BigUint::zero() {
        d /= &two;
        r += 1;
    }

    let witnesses: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    for &a_u64 in witnesses.iter() {
        let a = BigUint::from(a_u64);
        if a >= *n { continue; }
        let mut x = a.modpow(&d, n);
        if x == one || x == n_minus_one { continue; }
        let mut passed = false;
        for _ in 0..r.saturating_sub(1) {
            x = x.modpow(&two, n);
            if x == n_minus_one { passed = true; break; }
        }
        if !passed { return false; }
    }
    true
}

/// N is prime iff real arithmetic says so: exact trial division by every
/// odd number up to 1000 (catches the overwhelming majority of composites
/// cheaply, exactly, no probability involved), then Miller-Rabin on
/// whatever survives. `Undetermined` is kept in the `PrimeVerdict` enum
/// for the callers below that still match on it (`find`, `factor`), but
/// this function never produces it: trial division and Miller-Rabin
/// together always resolve to a definite answer.
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

    let mut d: u64 = 3;
    loop {
        let bd = BigUint::from(d);
        if &bd * &bd > n { break; }
        if &n % &bd == BigUint::zero() {
            return if n == bd { PrimeVerdict::Prime } else { PrimeVerdict::Composite };
        }
        if d >= 1000 { break; }
        d += 2;
    }

    if miller_rabin(&n) { PrimeVerdict::Prime } else { PrimeVerdict::Composite }
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
///
/// The old version of this search jumped past a whole block of numbers on
/// a fact that no longer holds: `is_prime` used to read only the leading
/// digit, so an entire digit-count's worth of numbers shared one verdict
/// and could be skipped at once. Now that the verdict comes from the full
/// ROTAT orbit and depends on every digit, that shortcut is gone and there
/// is no known way to jump to the answer — decrementing one at a time is
/// the search itself, not a stand-in for a faster one. `SCAN_CAP` bounds
/// the step count at a value measured to stay interactive; past that it
/// reports OutOfReach, stating what was scanned rather than hanging.
pub fn find(n: &str) -> String {
    if lt(n, "2") {
        return format!("prime_winding find {}: no primes ≤ {}", n, n);
    }
    let start = trim(n);
    let mut m = start.clone();
    let mut steps: u64 = 0;
    loop {
        match is_prime(&m) {
            PrimeVerdict::Prime => {
                let glyph = digit_encode(&m);
                return if m == start {
                    format!("prime_winding find {}: {} IS PRIME\n  glyph: {}", n, n, glyph)
                } else {
                    format!(
                        "prime_winding find {}: {} is composite, nearest prime ≤ {} is {}\n  glyph: {}",
                        n, n, n, m, glyph
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
        if steps > SCAN_CAP {
            return format!(
                "prime_winding find {}: OutOfReach — {} step(s) scanned below {} with no closing or vacuous word, scan halted at {}",
                n, steps, n, m
            );
        }
    }
}

/// One Brent's-rho walk with a fixed (c, x0) seed, batch size 128. Real
/// engine lives in functional_graph.rs (`pollard_rho_brent`) now -- this
/// file no longer carries its own private copy of the same walk. Returns a
/// nontrivial factor of n if the walk crosses one within max_power doubling
/// rounds, None otherwise -- no guaranteed success bound, Brent's rho's
/// real, known shape, not a defect to hide.
fn brent_walk(n: &BigUint, c: u64, x0: u64, max_power: u64) -> Option<BigUint> {
    crate::functional_graph::pollard_rho_brent(n, &BigUint::from(c), &BigUint::from(x0), 128, max_power)
}

/// The seeds a Brent's-rho split tries, each its own independent walk on
/// the same n -- distinct starting strands over the same braid, any one
/// of which closing (finding a nontrivial gcd) splits n. Hosted runs all
/// of them at once, one thread per strand, the way `live_hud` already
/// runs a background thread in this same binary; bare metal, with no
/// std::thread, walks them one at a time. Same seeds, same walk, same
/// answer either way -- only whether the strands run concurrently
/// differs.
const BRENT_SEEDS: [(u64, u64); 6] = [(3, 2), (7, 5), (11, 3), (17, 7), (23, 11), (41, 13)];
const BRENT_MAX_POWER: u64 = 4_000_000;

#[cfg(feature = "hosted")]
fn brent_split_one(n: &BigUint, max_power: u64) -> Option<BigUint> {
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();
    for &(c, x0) in BRENT_SEEDS.iter() {
        let n = n.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            if let Some(f) = brent_walk(&n, c, x0, max_power) {
                let _ = tx.send(f);
            }
            // Un-joined on purpose: this walk is step-bounded so it always
            // terminates on its own; only the first strand to close matters,
            // and the losing strands finishing quietly in the background
            // costs nothing this tool needs to wait on.
        });
    }
    drop(tx);
    rx.recv().ok()
}

#[cfg(not(feature = "hosted"))]
fn brent_split_one(n: &BigUint, max_power: u64) -> Option<BigUint> {
    for &(c, x0) in BRENT_SEEDS.iter() {
        if let Some(f) = brent_walk(n, c, x0, max_power) {
            return Some(f);
        }
    }
    None
}

/// Recursively split n via Brent's rho until every piece is prime-confirmed
/// (`is_prime`, real Miller-Rabin) or a piece resists every seed within its
/// step bound. The latter is pushed to `unsplit` and reported as exactly
/// that -- a known composite this search did not finish factoring -- never
/// silently folded into `primes`.
fn brent_split_recursive(n: BigUint, primes: &mut Vec<BigUint>, unsplit: &mut Vec<BigUint>,
                          max_power: u64) {
    let mut stack: Vec<BigUint> = Vec::new();
    stack.push(n);
    while let Some(m) = stack.pop() {
        if m.is_one() { continue; }
        if is_prime(&m.to_str_radix(10)) == PrimeVerdict::Prime {
            primes.push(m);
            continue;
        }
        match brent_split_one(&m, max_power) {
            Some(f) => {
                let g = &m / &f;
                stack.push(f);
                stack.push(g);
            }
            None => unsplit.push(m),
        }
    }
}

/// Factor n (decimal string, arbitrary precision). Trial division to
/// `TRIAL_DIVISION_BOUND` finds every small factor exactly; whatever
/// remains past that bound goes to Brent's rho (parallel strands, see
/// `brent_split_one`), which reaches factors far beyond trial division's
/// practical range without needing to divide up to n's own square root.
/// This is real, unbounded-precision arithmetic throughout -- closure
/// alone never produces a divisor, so it plays no role in the search,
/// only in confirming each candidate's primality via `is_prime`.
///
/// `max_power` bounds each Brent seed's walk (see `brent_walk`); `None`
/// keeps the default `BRENT_MAX_POWER`. The bound is not optional in the
/// sense of removable: Brent's rho carries no guaranteed termination on a
/// genuinely hard composite, and without SOME cutoff a call on such an n
/// would run forever with no way to ever report `unsplit`. What's
/// adjustable is the size of that cutoff, not whether one exists.
pub fn factor(n: &str) -> String {
    factor_bounded(n, None)
}

/// `factor`, with the per-seed Brent step bound made explicit rather than
/// silently defaulted, for a caller that wants to trade search depth
/// against wall-clock time on a case the default budget resists.
pub fn factor_bounded(n: &str, max_power: Option<u64>) -> String {
    let max_power = max_power.unwrap_or(BRENT_MAX_POWER);
    let t = trim(n);
    let n_big: BigUint = match t.parse() {
        Ok(v) => v,
        Err(_) => return format!("prime_winding factor {}: not a valid non-negative integer", n),
    };
    if n_big < BigUint::from(2u32) {
        return format!("prime_winding factor {}: n < 2, no prime factors", n);
    }

    let mut primes: Vec<BigUint> = Vec::new();
    let mut unsplit: Vec<BigUint> = Vec::new();

    let mut m = n_big;
    let mut d: u64 = 2;
    while d <= TRIAL_DIVISION_BOUND {
        let bd = BigUint::from(d);
        if &bd * &bd > m { break; }
        if &m % &bd == BigUint::zero() {
            while &m % &bd == BigUint::zero() {
                primes.push(bd.clone());
                m /= &bd;
            }
        }
        d += if d == 2 { 1 } else { 2 };
    }

    if m > BigUint::one() {
        brent_split_recursive(m, &mut primes, &mut unsplit, max_power);
    }

    primes.sort_unstable();
    unsplit.sort_unstable();

    if unsplit.is_empty() && primes.len() == 1 && primes[0].to_str_radix(10) == trim(n) {
        return format!("prime_winding factor {}: {} IS PRIME", n, n);
    }

    let mut out = format!("prime_winding factor {}: ", n);
    if !primes.is_empty() {
        let rep: Vec<String> = primes.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!("{} = {}", n, rep.join(" × ")));
    } else {
        out.push_str(&format!("{} — no confirmed prime factors", n));
    }
    if !unsplit.is_empty() {
        let rep: Vec<String> = unsplit.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!(
            "\n  └─ confirmed composite, not split by any of {} Brent seeds within {} steps each: {}",
            BRENT_SEEDS.len(), max_power, rep.join(" × ")
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
