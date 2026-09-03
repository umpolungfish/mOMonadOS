//! Native IMASM-Numeral Mapping — ob3ect-backed kernel tool.
//!
//! The artifact "Native IMASM-Numeral Mapping" is a verified ob3ect whose
//! glyph word `⊢⊣≻⋈∈⊤⊥⊞∋⊙≺⊡⋈⊣` maps a number into the Grammar through its
//! parity grade. Its boundary is the Z2 topological invariant, parity, the
//! one bit that cannot deform to zero without crossing a phase boundary. The
//! marks are numeral roles:
//!
//!   ⊢ VINIT   void_numeral          the uninitialized value
//!   ⊣ TANCH   topological_boundary  the closed numeral system
//!   ≻ AFWD    increment_magnitude   one step up in magnitude
//!   ⋈ CLINK   digit_composition     chain a digit onto the numeral
//!   ∈ FSPLIT  parity_branch         split on parity, even arm and odd arm
//!   ⊤ EVALT   even_parity           the digit is even (0)
//!   ⊥ EVALF   odd_parity            the digit is odd (1)
//!   ⊞ ENGAGR  chiral_superposition  both parities held
//!   ∋ FFUSE   stoichiometric_rejoin the arms rejoin
//!   ⊙ IMSCRIB critical_state        the value recognizes itself
//!   ≺ AREV    decrement_magnitude   one step down
//!   ⊡ IFIX    immutable_record      the Z2 fixation, the value made permanent
//!
//! Why this and not the hex-nibble encoding. prime_winding maps a number by
//! looking each hex digit up in a fixed table, and its own header records that
//! this reads only a bounded local pattern in the digits: it went T the moment
//! any nibble was 8 or higher, tracking no global fact. This mapping is built
//! from the Z2 parity grade instead: a number is encoded bit by bit, each bit
//! its own parity branch, even on the ⊤ arm and odd on the ⊥ arm, so the whole
//! binary expansion is carried as a chain of closed parity branches. The
//! encoding is exact and reversible, and distinct numbers give distinct words.
//!
//! The raw blueprint word clears its parity superposition at the ≺ decrement
//! with nothing banked, which the ob3ect's own banked check flags. The
//! per-number encoding here is built to hold instead: each bit's branch opens,
//! works, and fuses before the next, so no count is left in the open.
//!
//! Subcommands:
//!   native_numeral word         the canonical glyph word and its marks
//!   native_numeral encode <n>   map n to its native parity-graded word
//!   native_numeral help         list subcommands

use alloc::string::String;
use alloc::format;
use num_bigint::BigUint;
use num_traits::Zero;
use imasm_core::lattice_flow::{tri_ancestral_word_verdict, cycle_landings};

/// The ob3ect's own fixed reference word.
pub const WORD: &str = "⊢⊣≻⋈∈⊤⊥⊞∋⊙≺⊡⋈⊣";
pub const PERIOD: usize = 14;
pub const ARTIFACT: &str = "Native IMASM-Numeral Mapping";

/// Map n to its native parity-graded IMASM word. Bits are read low to high;
/// each bit is one closed parity branch, ∈ then its parity arm then ∋, so the
/// branch works and fuses before the next bit opens, and nothing is left in
/// the open for a later clear. The magnitude step ≻ precedes each branch, the
/// value is recognized at ⊙ and fixed at ⊡, and the whole is bounded by ⊢ and
/// ⊣. n = 0 is the void numeral, ⊢⊙⊡⊣, a fixed empty magnitude.
pub fn encode(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let mut w = String::new();
    w.push('⊢');
    if n.is_zero() {
        w.push('⊙');
        w.push('⊡');
        w.push('⊣');
        return w;
    }
    let bits = to_bits_low_first(&n);
    for b in bits.iter() {
        w.push('≻');            // increment_magnitude: advance one position
        w.push('⋈');            // digit_composition: chain this digit on
        w.push('∈');            // parity_branch
        w.push(if *b { '⊥' } else { '⊤' }); // odd → ⊥, even → ⊤
        w.push('∋');            // stoichiometric_rejoin
    }
    w.push('⊙');                // critical_state: recognize the value
    w.push('⊡');                // immutable_record: the Z2 fixation
    w.push('⊣');                // topological_boundary
    w
}

/// n as bits, least significant first. `true` is an odd (1) bit.
fn to_bits_low_first(n: &BigUint) -> alloc::vec::Vec<bool> {
    let mut bits = alloc::vec::Vec::new();
    let mut m = n.clone();
    let two = BigUint::from(2u32);
    while !m.is_zero() {
        let bit = (&m % &two) != BigUint::zero();
        bits.push(bit);
        m /= &two;
    }
    bits
}

pub fn help() -> String {
    let mut o = String::new();
    o.push_str("native_numeral — ob3ect-backed parity-graded numeral mapping\n");
    o.push_str("  word         the canonical glyph word and its marks\n");
    o.push_str("  encode <n>   map n to its native parity-graded word\n");
    o.push_str("  help         this list");
    o
}

pub fn word() -> String {
    format!(
        "{}\n  word   : {}   period {}\n  boundary: the Z2 parity invariant\n  parity branch ∈ (4) / rejoin ∋ (8): even on ⊤, odd on ⊥",
        ARTIFACT, WORD, PERIOD
    )
}

/// Encode n and read the result back through the instruments: its length, its
/// tri-ancestral verdict as a loop, and whether every cut holds the same
/// verdict. This is the number's native word plus what the Grammar says of it.
pub fn encode_report(n: &str) -> String {
    let w = encode(n);
    if w.is_empty() {
        return format!("native_numeral encode {}: not a valid non-negative integer", n);
    }
    let mut o = format!("native_numeral encode {}:\n  word   : {}\n", n, w);
    match tri_ancestral_word_verdict(&w) {
        Some(v) => o.push_str(&format!("  tri-ancestral verdict (as a loop): {}\n", v)),
        None => o.push_str("  no IMASM glyphs\n"),
    }
    match cycle_landings(&w) {
        Some(l) => {
            let mut distinct: alloc::vec::Vec<&String> = alloc::vec::Vec::new();
            for r in l.iter() { if !distinct.iter().any(|s| *s == r) { distinct.push(r); } }
            o.push_str(&format!(
                "  period {}, {} distinct landing register(s) over the orbit",
                l.len(), distinct.len()
            ));
        }
        None => o.push_str("  no orbit"),
    }
    o
}
