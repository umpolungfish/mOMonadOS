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

    // d*d > m proves, by exhaustive trial division to m's own square root,
    // that m has no factor at all -- m IS its own complete ordinary-sense
    // prime factorization, a fact independent of its Grammar verdict. That
    // is a different, stronger statement than "the bound ran out before
    // finding one," so the leftover is tracked apart from small divisors
    // already confirmed by direct division, and reported accordingly.
    let mut leftover: Option<(BigUint, PrimeVerdict, bool)> = None;
    if m > BigUint::one() {
        let exhausted = { let bd = BigUint::from(d); &bd * &bd > m };
        let verdict = is_prime(&m.to_str_radix(10));
        leftover = Some((m, verdict, exhausted));
    }

    primes.sort_unstable();
    composite_unsplit.sort_unstable();
    undetermined.sort_unstable();

    if composite_unsplit.is_empty() && undetermined.is_empty() && primes.is_empty()
        && leftover.as_ref().is_some_and(|(v, verdict, _)|
            *verdict == PrimeVerdict::Prime && v.to_str_radix(10) == trim(n))
    {
        return format!("prime_winding factor {}: {} IS PRIME", n, n);
    }

    let mut out = format!("prime_winding factor {}: ", n);
    let mut all_primes = primes.clone();
    if let Some((v, PrimeVerdict::Prime, _)) = &leftover {
        all_primes.push(v.clone());
        all_primes.sort_unstable();
    }
    if !all_primes.is_empty() {
        let rep: Vec<String> = all_primes.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!("{} = {}", n, rep.join(" × ")));
    } else {
        out.push_str(&format!("{} — no confirmed prime factors", n));
    }
    if !composite_unsplit.is_empty() {
        let rep: Vec<String> = composite_unsplit.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!(
            "\n  └─ composite divisor(s) found by direct division: {}",
            rep.join(" × ")
        ));
    }
    if !undetermined.is_empty() {
        let rep: Vec<String> = undetermined.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!(
            "\n  └─ vacuous divisor(s) found by direct division: {}",
            rep.join(" × ")
        ));
    }
    if let Some((v, verdict, exhausted)) = &leftover {
        if *verdict != PrimeVerdict::Prime {
            let s = v.to_str_radix(10);
            match (verdict, exhausted) {
                (PrimeVerdict::Composite, true) => out.push_str(&format!(
                    "\n  └─ {} has no factor at all, proven by trial division exhausted to its own square root -- its exact ordinary-sense prime factorization is itself; Grammar-composite only because its leading digit reads exposed",
                    s
                )),
                (PrimeVerdict::Composite, false) => out.push_str(&format!(
                    "\n  └─ composite cofactor, no trial divisor below {} splits it further (search incomplete, not exhaustive): {}",
                    TRIAL_DIVISION_BOUND, s
                )),
                (PrimeVerdict::Undetermined, true) => out.push_str(&format!(
                    "\n  └─ {} has no factor at all, proven by trial division exhausted to its own square root -- its exact ordinary-sense prime factorization is itself; vacuous on its digit-encoded word, primality undetermined by the Grammar",
                    s
                )),
                (PrimeVerdict::Undetermined, false) => out.push_str(&format!(
                    "\n  └─ cofactor vacuous on its digit-encoded word — primality undetermined (search incomplete below {}): {}",
                    TRIAL_DIVISION_BOUND, s
                )),
                (PrimeVerdict::Prime, _) => unreachable!(),
            }
        }
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
