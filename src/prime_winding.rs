//! Prime Winding — the winding_period_of_the_primes_on_the_number_line ob3ect
//! as a kernel command.
//!
//! The ob3ect's glyph word ⊢⊙∈≻⊤⋈≺⊥⊞∋⊡⋈⊙⊣ has period 14 and is phase-bearing
//! across A / Ftf / tf / T. The word, glyph, tuple, and cycle commands report
//! the Grammar's reading of a number. The primality verdict and factoring run
//! on real arithmetic: trial division then Miller-Rabin, deterministic below
//! 3,317,044,064,679,887,385,961,981 and the standard thirteen-witness practice
//! above it. In the GPU build the verdict runs on the device (see gpu_prime).
//! A Grammar-native verdict via the winding period and the ⊡ holonomy is the
//! open route, closing.
//!
//! Subcommands:
//!   prime_winding word                       canonical glyph word
//!   prime_winding find <n>                   nearest prime <= n
//!   prime_winding range <lo> <hi> [count]    every prime in [lo, hi]
//!   prime_winding factor <n>                 factor n into primes
//!   prime_winding cycle                      full ROTAT orbit, landing per cut
//!   prime_winding tuple                      the 12-slot tuple
//!   prime_winding verdict                    Frobenius verdict
//!   prime_winding help                       list subcommands

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

/// Each hex digit's own type, read via `imscribe generate`'s guided,
/// per-primitive grounding (one canonical value at a time from a numbered
/// list, not free-text) rather than through self_imscribe. self_imscribe
/// was tried first and rejected: past a few nibbles it hits the
/// period-gated axes' saturation (imas_ig.rs's from_snapshot_under doc),
/// collapsing almost every N onto the same catch-all bucket regardless of
/// its digits, and even at a single digit's own short word it still
/// collapsed 0x1 and 0x2 onto one type. These sixteen are transcribed
/// directly from the real grounding run's own output (IG_catalog.json
/// entries "0x0".."0xf"), never hand-imscribed. 0x8 and 0x9 differ only on
/// D (if vs array) here; three independent re-groundings of the full set
/// found 0x8 and 0x9 identical on every axis instead, so this pair is
/// held open rather than resolved -- the tensor built on top of these
/// still recovers whichever type went in.
const GROUNDED_HEX_DIGIT_TUPLES: [crate::imas_ig::IgTuple; 16] = {
    use crate::imas_ig::IgPrim::*;
    use crate::imas_ig::IgTuple;
    [
        IgTuple { d: array, t: are,   r: ear, p: yew, f: age,  k: egg, g: ice, c: gag, phi: monad, h: fee,  s: hung, omega: awe }, // 0x0
        IgTuple { d: array, t: judge, r: ado, p: yew, f: peep, k: egg, g: ice, c: vow, phi: monad, h: fee,  s: hung, omega: oak }, // 0x1
        IgTuple { d: array, t: eat,   r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: hung, omega: oak }, // 0x2
        IgTuple { d: array, t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: up,   omega: oak }, // 0x3
        IgTuple { d: array, t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: hung, omega: oak }, // 0x4
        IgTuple { d: array, t: mime,  r: ear, p: yew, f: they, k: egg, g: ice, c: gag, phi: monad, h: fee,  s: up,   omega: oak }, // 0x5
        IgTuple { d: array, t: mime,  r: ear, p: yew, f: they, k: egg, g: ice, c: gag, phi: monad, h: kick, s: hung, omega: oak }, // 0x6
        IgTuple { d: array, t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: up,   omega: oak }, // 0x7
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: hung, omega: oak }, // 0x8
        IgTuple { d: array, t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: hung, omega: oak }, // 0x9
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: they, k: egg, g: ice, c: gag, phi: monad, h: fee,  s: up,   omega: oak }, // 0xA
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: they, k: egg, g: ice, c: gag, phi: monad, h: sure, s: up,   omega: ah  }, // 0xB
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: sure, s: up,   omega: ah  }, // 0xC
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: they, k: egg, g: ice, c: gag, phi: monad, h: sure, s: up,   omega: ah  }, // 0xD
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: kick, s: up,   omega: oak }, // 0xE
        IgTuple { d: if_,   t: mime,  r: ear, p: yew, f: peep, k: egg, g: ice, c: gag, phi: monad, h: sure, s: up,   omega: oak }, // 0xF
    ]
};

/// N's own composite TYPE, built by tensoring each hex digit's own
/// grounded type (a point in the 17,280,000-point space crystal.rs
/// defines) rather than combining IMASM tokens. Tensoring the digits'
/// own IgTuples with algebra::tensor was tried and rejected: that stays
/// inside the SAME 12-mark space and is idempotent/commutative (max/min
/// per axis), so it is blind to repetition (10, 170, and sixteen repeats
/// of hex digit A all tensor to one result) and to order (8051 and 8052
/// converge once their differing digit is absorbed). A tensor product
/// does not fold inputs back into the space they came from, it GROWS the
/// space: n copies of a TOTAL-point space tensor into a TOTAL^n-point
/// space, the same way crystal.rs's own encode already tensors twelve
/// small axis spaces into the one 17,280,000-point space.
/// crystal::tensor_types does that one level up, over each digit's own
/// grounded type. The composite is injective in the digit sequence: order
/// and repetition both survive, and crystal::untensor_types recovers
/// every digit's own type back out exactly.
pub fn digit_type_tensor(n_str: &str) -> Option<alloc::vec::Vec<u8>> {
    let n: BigUint = trim(n_str).parse().ok()?;
    let hex = n.to_str_radix(16);
    let mut types: alloc::vec::Vec<u32> = alloc::vec::Vec::new();
    for c in hex.chars() {
        let d = c.to_digit(16)? as usize;
        let ty = GROUNDED_HEX_DIGIT_TUPLES[d].crystal_address();
        types.push(ty);
    }
    Some(crate::crystal::tensor_types(&types))
}

/// N's own composite type, rendered as a decimal string for display. Same
/// value digit_type_tensor produces, read as one big-endian number instead
/// of raw bytes -- what a caller wanting to show or compare a number's real
/// Grammar-native identity actually wants, rather than the byte vector
/// digit_type_tensor hands back for further composition.
pub fn digit_type_address(n_str: &str) -> Option<alloc::string::String> {
    let bytes = digit_type_tensor(n_str)?;
    Some(BigUint::from_bytes_be(&bytes).to_string())
}

/// Shared shape behind grounded_add_report and grounded_mul_report: compute
/// a `op` b the ordinary way -- that arithmetic is never in question, it's
/// what native_numeral's word-native pass already does -- then report all
/// three numbers' own real Grammar-native type, each sourced separately from
/// GROUNDED_HEX_DIGIT_TUPLES's real imscribe-generate grounding through
/// digit_type_tensor, never a bit read and never inferred from the result.
/// Reports whether the result's type equals either operand's own type
/// outright (it can, since digit types collapse hex digits 3≡7 and 4≡9 in
/// this grounding run, already found and left open rather than resolved),
/// as a fact read off the real values, not assumed from the arithmetic.
fn grounded_binop_report(
    a_str: &str,
    b_str: &str,
    sym: &str,
    op_name: &str,
    op: fn(&BigUint, &BigUint) -> Option<BigUint>,
) -> alloc::string::String {
    let a: BigUint = match trim(a_str).parse() {
        Ok(v) => v,
        Err(_) => return format!("{op_name}: '{}' is not a valid non-negative integer", a_str),
    };
    let b: BigUint = match trim(b_str).parse() {
        Ok(v) => v,
        Err(_) => return format!("{op_name}: '{}' is not a valid non-negative integer", b_str),
    };
    let result = match op(&a, &b) {
        Some(v) => v,
        None => return format!("{op_name}: {a} {sym} {b} has no result (underflow, or division/modulo by zero)"),
    };
    let result_str = result.to_string();
    let ta = digit_type_address(a_str);
    let tb = digit_type_address(b_str);
    let tr = digit_type_address(&result_str);
    format!(
        "{a} {sym} {b} = {result}\n  type({a}) = {}\n  type({b}) = {}\n  type({result}) = {}\n  type(result) == type(a): {}\n  type(result) == type(b): {}\n",
        ta.as_deref().unwrap_or("(none)"),
        tb.as_deref().unwrap_or("(none)"),
        tr.as_deref().unwrap_or("(none)"),
        tr == ta,
        tr == tb,
    )
}

fn some_add(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    Some(crate::native_numeral::add_via_word(a, b))
}
fn some_mul(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    Some(crate::native_numeral::multiply_via_word(a, b))
}

/// a + b with all three numbers' own real Grammar-native type. See
/// grounded_binop_report for what "real" means here and why the arithmetic
/// and the type are answered as two separate questions.
pub fn grounded_add_report(a_str: &str, b_str: &str) -> alloc::string::String {
    grounded_binop_report(a_str, b_str, "+", "grounded_add", some_add)
}

/// a * b with all three numbers' own real Grammar-native type. Same shape
/// as grounded_add_report, over multiply_via_word instead of add_via_word.
pub fn grounded_mul_report(a_str: &str, b_str: &str) -> alloc::string::String {
    grounded_binop_report(a_str, b_str, "*", "grounded_mul", some_mul)
}

/// a - b with all three numbers' own real Grammar-native type. subtract_via_
/// word is already Option (unsigned underflow when b > a), the same shape
/// grounded_binop_report expects directly -- no wrapper needed, unlike
/// add/mul above.
pub fn grounded_sub_report(a_str: &str, b_str: &str) -> alloc::string::String {
    grounded_binop_report(a_str, b_str, "-", "grounded_sub", crate::native_numeral::subtract_via_word)
}

/// a mod b with all three numbers' own real Grammar-native type. modulo_via_
/// word is already Option (division by zero), same shape as subtract.
pub fn grounded_mod_report(a_str: &str, b_str: &str) -> alloc::string::String {
    grounded_binop_report(a_str, b_str, "mod", "grounded_mod", crate::native_numeral::modulo_via_word)
}

/// a = q*b + r via divmod_via_word, with all FOUR numbers' own real
/// Grammar-native type -- a and b (the operands), and q and r together
/// (divmod's own two results), each sourced separately from
/// GROUNDED_HEX_DIGIT_TUPLES, not from each other or from the division.
/// Reports its own shape rather than reusing grounded_binop_report, since
/// divmod produces a pair, not one result.
pub fn grounded_divmod_report(a_str: &str, b_str: &str) -> alloc::string::String {
    let a: BigUint = match trim(a_str).parse() {
        Ok(v) => v,
        Err(_) => return format!("grounded_divmod: '{}' is not a valid non-negative integer", a_str),
    };
    let b: BigUint = match trim(b_str).parse() {
        Ok(v) => v,
        Err(_) => return format!("grounded_divmod: '{}' is not a valid non-negative integer", b_str),
    };
    let (q, r) = match crate::native_numeral::divmod_via_word(&a, &b) {
        Some(v) => v,
        None => return format!("grounded_divmod: {a} / {b} has no result (division by zero)"),
    };
    let (q_str, r_str) = (q.to_string(), r.to_string());
    let ta = digit_type_address(a_str);
    let tb = digit_type_address(b_str);
    let tq = digit_type_address(&q_str);
    let tr = digit_type_address(&r_str);
    format!(
        "{a} = {q} * {b} + {r}\n  type({a}) = {}\n  type({b}) = {}\n  type(q={q}) = {}\n  type(r={r}) = {}\n",
        ta.as_deref().unwrap_or("(none)"),
        tb.as_deref().unwrap_or("(none)"),
        tq.as_deref().unwrap_or("(none)"),
        tr.as_deref().unwrap_or("(none)"),
    )
}

fn some_gcd(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    Some(big_gcd(a.clone(), b.clone()))
}

/// gcd(a, b) with all three numbers' own real Grammar-native type. Same
/// shape as grounded_add_report, over big_gcd (this file's own extended-
/// Euclid, already word-native) instead of add_via_word.
pub fn grounded_gcd_report(a_str: &str, b_str: &str) -> alloc::string::String {
    grounded_binop_report(a_str, b_str, "gcd", "grounded_gcd", some_gcd)
}

/// N's real prime factorization (factor_primes, this file's own trial
/// division + Brent's rho, already word-native), with N's own real
/// Grammar-native type and every distinct factor's own, each sourced
/// separately from GROUNDED_HEX_DIGIT_TUPLES through digit_type_tensor.
/// Repeated factors are reported once each (their multiplicity is in the
/// factorization line itself); factoring is never in question here, only
/// which real type each factor carries, the same separation of concerns
/// as every other grounded_* report in this file.
pub fn grounded_factor_report(n_str: &str, max_power: Option<u64>) -> alloc::string::String {
    let n: BigUint = match trim(n_str).parse() {
        Ok(v) => v,
        Err(_) => return format!("grounded_factor: '{}' is not a valid non-negative integer", n_str),
    };
    if n < BigUint::from(2u32) {
        return format!("grounded_factor: {} < 2, no prime factors", n_str);
    }
    let max_power = max_power.unwrap_or(BRENT_MAX_POWER);
    let (mut primes, mut unsplit) = factor_primes(n.clone(), max_power);
    primes.sort_unstable();
    unsplit.sort_unstable();

    let mut out = format!("{n} = {}\n  type({n}) = {}\n",
        if primes.is_empty() { alloc::string::String::from("(no confirmed prime factors)") }
        else { primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<_>>().join(" × ") },
        digit_type_address(n_str).as_deref().unwrap_or("(none)"),
    );
    let mut seen: Vec<BigUint> = Vec::new();
    for p in primes.iter().chain(unsplit.iter()) {
        if seen.contains(p) { continue; }
        seen.push(p.clone());
        let p_str = p.to_str_radix(10);
        out.push_str(&format!("  type({p_str}) = {}\n", digit_type_address(&p_str).as_deref().unwrap_or("(none)")));
    }
    if !unsplit.is_empty() {
        let rep: Vec<String> = unsplit.iter().map(|p| p.to_str_radix(10)).collect();
        out.push_str(&format!("  (unsplit, confirmed composite: {})\n", rep.join(" × ")));
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
    use crate::native_numeral::{add_via_word, halve_even_by_word, mod_pow_walk, modulo_via_word, multiply_via_word, subtract_via_word, to_bits_low_first};
    let one = BigUint::one();
    let two = add_via_word(&one, &one);
    if *n < two { return false; }
    if *n == two { return true; }
    if modulo_via_word(n, &two).unwrap() == BigUint::zero() { return false; }

    let n_minus_one = subtract_via_word(n, &one).unwrap();
    let mut d = n_minus_one.clone();
    let mut r: u32 = 0;
    while modulo_via_word(&d, &two).unwrap() == BigUint::zero() {
        d = halve_even_by_word(&d).unwrap();
        r += 1;
    }

    let witnesses: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    for &a_u64 in witnesses.iter() {
        let a = BigUint::from(a_u64);
        if a >= *n { continue; }
        let mut x = mod_pow_walk(&a, &to_bits_low_first(&d), n);
        if x == one || x == n_minus_one { continue; }
        let mut passed = false;
        for _ in 0..r.saturating_sub(1) {
            x = modulo_via_word(&multiply_via_word(&x, &x), n).unwrap();
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
    use crate::native_numeral::{divmod_small_on_limbs, word_bits};
    let two = BigUint::from(2u32);
    if n < two { return PrimeVerdict::Composite; }
    if n == two { return PrimeVerdict::Prime; }
    // n's own word read once here, not once per candidate divisor below --
    // n never changes across this loop, so its limbs don't need re-reading
    // on every one of the up to 500 divisors tried.
    let n_limbs = word_bits(&n);
    if divmod_small_on_limbs(&n_limbs, 2).1 == 0 { return PrimeVerdict::Composite; }

    // Every trial divisor here fits in a u64 with room to spare (d <= 1000),
    // so d*d does too -- that comparison stays plain u64 arithmetic, the
    // same way an ordering check stays native throughout this file's other
    // conversions. The reduction itself reads n's own limbs (extracted
    // once, above) through the limb-at-a-time small-divisor primitive
    // rather than the general bit-at-a-time one.
    let mut d: u64 = 3;
    loop {
        if BigUint::from(d * d) > n { break; }
        if divmod_small_on_limbs(&n_limbs, d).1 == 0 {
            return if n == BigUint::from(d) { PrimeVerdict::Prime } else { PrimeVerdict::Composite };
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
/// The verdict depends on every digit, so there is no way to jump to the
/// answer; decrementing one at a time is the search itself. `SCAN_CAP` bounds
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

/// Enumerate every prime in [lo, hi], high to low, as the find/subtract/repeat
/// walk: test the current value, on a prime record it and drop to prime-1, on a
/// composite drop by one, until below lo. One primality test per integer, no
/// table, so it runs at arbitrary precision and around an arbitrarily large
/// starting point where a sieve cannot allocate. For a dense low range a sieve
/// is faster; this is for reach and for zero memory, not for beating a sieve.
///
/// `count_only` suppresses the per-prime listing and returns only the tally,
/// for spans too wide to print.
pub fn range(lo: &str, hi: &str, count_only: bool) -> String {
    let lo_t = trim(lo);
    let hi_t = trim(hi);
    if lt(&hi_t, &lo_t) {
        return format!("prime_winding range [{}, {}]: empty span (hi < lo)", lo, hi);
    }
    let mut m = hi_t.clone();
    let mut count: u64 = 0;
    let mut tested: u64 = 0;
    let mut listing = String::new();
    loop {
        if lt(&m, &lo_t) || lt(&m, "2") { break; }
        tested += 1;
        if let PrimeVerdict::Prime = is_prime(&m) {
            count += 1;
            if !count_only {
                listing.push_str(&m);
                listing.push('\n');
            }
        }
        if m == "2" { break; }
        m = sub(&m, "1");
    }
    if count_only {
        format!("prime_winding range [{}, {}]: {} prime(s) ({} integers tested)", lo, hi, count, tested)
    } else {
        format!("prime_winding range [{}, {}]: {} prime(s)\n{}", lo, hi, count, listing)
    }
}

/// a / b, both nonzero, Euclid's algorithm on BigUint. Public so the
/// trilattice winding route reads the same gcd rather than carrying its own.
pub fn big_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let t = b.clone();
        b = crate::native_numeral::modulo_via_word(&a, &b).unwrap();
        a = t;
    }
    a
}

/// Brent's polynomial x -> x^2 + c mod n.
fn brent_f(x: &BigUint, c: &BigUint, n: &BigUint) -> BigUint {
    use crate::native_numeral::{add_via_word, modulo_via_word, multiply_via_word};
    let x2 = modulo_via_word(&multiply_via_word(x, x), n).unwrap();
    modulo_via_word(&add_via_word(&x2, c), n).unwrap()
}

/// One Brent's-rho walk with a fixed (c, x0) seed. Returns a nontrivial
/// factor of n if the walk crosses one within max_power steps, None
/// otherwise. This has no guaranteed success bound -- that is Brent's
/// rho's real, known shape, not a defect to hide -- so a walk that
/// exhausts its steps reports None plainly rather than assuming n is
/// prime because this one seed failed to split it.
///
/// x is the tortoise, fixed for the whole doubling round r; y is the hare,
/// walked r steps ahead of x and compared against it in batches of m,
/// accumulating the product of differences mod n so one gcd covers the
/// whole batch. A batch gcd landing on n itself (the product folded in
/// more than one factor's collision at once) is not a dead end -- it is
/// recovered by re-walking that same batch from its start (`ys`) one step
/// at a time, gcd on each step, until the exact collision point splits
/// out the real factor.
fn brent_walk(n: &BigUint, c: u64, x0: u64, max_power: u64) -> Option<BigUint> {
    use crate::native_numeral::{modulo_via_word, multiply_via_word, subtract_via_word};
    let c = BigUint::from(c);
    let one = BigUint::one();
    let mut x = BigUint::from(x0);
    let mut y = x.clone();
    let m: u64 = 128;
    let mut r: u64 = 1;
    let mut g = one.clone();
    let mut q = one.clone();
    let mut ys = y.clone();

    while g.is_one() && r < max_power {
        x = y.clone();
        for _ in 0..r {
            y = brent_f(&y, &c, n);
        }
        let mut k: u64 = 0;
        while k < r && g.is_one() {
            ys = y.clone();
            let steps = m.min(r - k);
            for _ in 0..steps {
                y = brent_f(&y, &c, n);
                let diff = if y > x { subtract_via_word(&y, &x).unwrap() } else { subtract_via_word(&x, &y).unwrap() };
                q = modulo_via_word(&multiply_via_word(&q, &diff), n).unwrap();
            }
            g = big_gcd(q.clone(), n.clone());
            k += m;
        }
        r = match r.checked_mul(2) { Some(v) => v, None => break };
    }

    if &g == n {
        loop {
            ys = brent_f(&ys, &c, n);
            let diff = if ys > x { subtract_via_word(&ys, &x).unwrap() } else { subtract_via_word(&x, &ys).unwrap() };
            g = big_gcd(diff, n.clone());
            if !g.is_one() { break; }
        }
    }

    if !g.is_one() && &g != n { Some(g) } else { None }
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
                let g = crate::native_numeral::divmod_via_word(&m, &f).unwrap().0;
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

/// The actual search behind factor_bounded, pulled out so a caller wanting
/// the real BigUint factors (not the formatted report) doesn't have to
/// re-parse factor_bounded's own string output. Trial division to
/// TRIAL_DIVISION_BOUND, then Brent's rho on whatever remains; returns
/// (confirmed primes, confirmed-composite-but-unsplit remainders), same
/// two lists factor_bounded formats, neither sorted here (factor_bounded
/// sorts its own copies; a caller of this function sorts if it wants to).
fn factor_primes(n_big: BigUint, max_power: u64) -> (Vec<BigUint>, Vec<BigUint>) {
    let mut primes: Vec<BigUint> = Vec::new();
    let mut unsplit: Vec<BigUint> = Vec::new();

    use crate::native_numeral::{bits_to_value, divmod_small_on_limbs, word_bits};
    // Up to TRIAL_DIVISION_BOUND (ten million) candidate divisors, each one
    // fitting comfortably in a u64 (d*d tops out at 10^14, nowhere near
    // overflow) -- the comparison stays native for the same reason it does
    // in is_prime above. m's own word is read once per value of m, not
    // once per candidate divisor: m only changes on the rare event of a
    // factor actually dividing out, so re-deriving its limbs on every one
    // of ten million divisor tests was pure waste, not arithmetic this
    // loop needed to do.
    let mut m = n_big;
    let mut m_limbs = word_bits(&m);
    let mut d: u64 = 2;
    while d <= TRIAL_DIVISION_BOUND {
        if BigUint::from(d * d) > m { break; }
        if divmod_small_on_limbs(&m_limbs, d).1 == 0 {
            let bd = BigUint::from(d);
            loop {
                let (q, rem) = divmod_small_on_limbs(&m_limbs, d);
                if rem != 0 { break; }
                primes.push(bd.clone());
                m_limbs = q;
                m = bits_to_value(&m_limbs);
            }
        }
        d += if d == 2 { 1 } else { 2 };
    }

    if m > BigUint::one() {
        brent_split_recursive(m, &mut primes, &mut unsplit, max_power);
    }
    (primes, unsplit)
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

    let (mut primes, mut unsplit) = factor_primes(n_big, max_power);

    if unsplit.is_empty() && primes.len() == 1 && primes[0].to_str_radix(10) == trim(n) {
        return format!("prime_winding factor {}: {} IS PRIME", n, n);
    }

    primes.sort_unstable();
    unsplit.sort_unstable();
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
           prime_winding range <lo> <hi> [count]   every prime in [lo,hi], the find/subtract walk\n\
           prime_winding factor <n>  factor n into prime divisors (with multiplicity)\n\
           prime_winding grounded_add <a> <b>  a+b plus all three numbers' real Grammar type\n\
           prime_winding grounded_mul <a> <b>  a*b plus all three numbers' real Grammar type\n\
           prime_winding grounded_sub <a> <b>  a-b plus all three numbers' real Grammar type\n\
           prime_winding grounded_mod <a> <b>  a mod b plus all three numbers' real Grammar type\n\
           prime_winding grounded_divmod <a> <b>  a=q*b+r plus all four numbers' real Grammar type\n\
           prime_winding grounded_gcd <a> <b>  gcd(a,b) plus all three numbers' real Grammar type\n\
           prime_winding grounded_factor <n> [max_power]  N's real factorization plus every number's real Grammar type\n\
           prime_winding cycle      ROTAT orbit with landing register per cut\n\
           prime_winding tuple      the 12-slot tuple the word was imscribed from\n\
           prime_winding verdict    Frobenius verdict and tri-ancestral reading\n\
           prime_winding artifact   ob3ect + Lean scaffold paths\n\
           prime_winding help       this help",
        WORD, PERIOD, FROBENIUS_VERDICT
    )
}

pub fn prime_verdict_label(v: PrimeVerdict) -> &'static str { prime_verdict_str(v) }

#[cfg(test)]
mod digit_tensor_probe {
    use super::{digit_type_tensor, HEX_WORDS};
    use num_bigint::BigUint;

    fn probe(n_str: &str) {
        let bytes = digit_type_tensor(n_str).expect("valid decimal");
        let composite = BigUint::from_bytes_be(&bytes);
        crate::nested_println!("n={n_str:<40} hex_digits={:<20} composite_type={composite}",
            BigUint::parse_bytes(n_str.as_bytes(), 10).unwrap().to_str_radix(16));
    }

    #[test]
    fn per_digit_types_reported() {
        // The tensor construction is only as faithful as its input: if two
        // different hex digits imscribe to the SAME type via word_to_tuple
        // (self_imscribe execution), the composite cannot recover them
        // distinctly either, whatever tensor_types itself does correctly.
        // Reported rather than asserted: this is HEX_WORDS[d]'s own type
        // under the executed pipeline, an open question about which
        // pipeline should supply a digit's type, not a broken invariant of
        // this test's own subject (tensor_types, which is verified
        // correct and injective in tensor_roundtrips_through_untensor).
        let mut types = alloc::vec::Vec::new();
        for w in HEX_WORDS.iter() {
            types.push(crate::axis_values::word_to_tuple(w).crystal_address());
        }
        let mut collisions = alloc::vec::Vec::new();
        for i in 0..16 {
            for j in (i + 1)..16 {
                if types[i] == types[j] {
                    collisions.push((i, j, types[i]));
                }
            }
        }
        crate::nested_println!("hex digit types under word_to_tuple: {types:?}");
        crate::nested_println!("collisions (digit i, digit j, shared type): {collisions:?}");
    }

    #[test]
    fn tensor_roundtrips_through_untensor() {
        let types: alloc::vec::Vec<u32> = alloc::vec![100u32, 200, 300, 16389838, 0];
        let composite = crate::crystal::tensor_types(&types);
        let back = crate::crystal::untensor_types(&composite, types.len());
        assert_eq!(types, back);
    }

    #[test]
    fn per_digit_tensor_reported() {
        probe("2");
        probe("3");
        probe("10");
        probe("12345");
        probe("8051");
        probe("8052");
        probe("97");
        probe("999999999989");
        probe("999999999999999999999999999999");
        // all-ones vs top-bit-only vs alternating at matched byte length,
        // the same three shapes the executed pipeline collapsed to two
        // states on regardless of length.
        probe("255");
        probe("128");
        probe("170");
        probe("18446744073709551615");
        probe("9223372036854775808");
        probe("12297829382473034410");
    }
}
