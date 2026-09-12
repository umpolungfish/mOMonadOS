// ─── axis_values.rs ────────────────────────────────────────────────────
// Kernel-sourced tuple derivation for the harvester tools. No value table is
// hand-written here: a glyph word is imscribed through the kernel's own
// structural witness (program_from_glyphs → self_imscribe → from_snapshot), and
// per-axis value lists come from catalog::ordinal_table, the single source of
// truth. Arbitrary bytes (a hex key, a text fragment) become an ORDERED IMASM
// program via the canonical mark→glyph→token route and are imscribed the same
// way — so order matters and the value assignment is entirely the kernel's.
#![allow(dead_code)]
extern crate alloc;
use alloc::string::String;
use crate::belnap_ring_shor::{glyph_to_token, program_from_glyphs, Glyph};
use crate::catalog::ordinal_table;
use crate::counterfactual::MARKS;
use crate::imas_ig::{IgPrim, IgTuple};
use crate::kernel::self_imscribe;
use crate::tokens::Program;

/// The canonical value list of axis `i` (0..11 in MARKS order), from the kernel.
pub fn axis_values(i: usize) -> &'static [IgPrim] {
    let mut buf = [0u8; 4];
    ordinal_table(MARKS[i % 12].encode_utf8(&mut buf))
}

/// Imscribe an ordered IMASM program through the kernel's structural witness,
/// under canonical composition.
fn imscribe_program(p: &Program) -> IgTuple {
    IgTuple::from_snapshot(&self_imscribe(p))
}

/// Same, but composed under a given dialect's own period scale
/// (IgTuple::from_snapshot_under) rather than canonical's fixed one.
pub fn imscribe_program_under(dialect: u8, p: &Program) -> IgTuple {
    IgTuple::from_snapshot_under(dialect, &self_imscribe(p))
}

/// A glyph word → its tuple. Order-sensitive: this runs the program. Falls back
/// to the byte route only when the word is not a clean mark sequence.
pub fn word_to_tuple(word: &str) -> IgTuple {
    match program_from_glyphs(word) {
        Ok(prog) => imscribe_program(&prog),
        Err(_) => bytes_to_tuple(word.as_bytes()),
    }
}

/// Same, composed under a given dialect.
pub fn word_to_tuple_under(dialect: u8, word: &str) -> IgTuple {
    match program_from_glyphs(word) {
        Ok(prog) => imscribe_program_under(dialect, &prog),
        Err(_) => IgTuple::from_snapshot_under(dialect, &self_imscribe(&{
            let mut p = Program::empty();
            for &b in word.as_bytes() {
                let mark = MARKS[(b as usize) % MARKS.len()];
                if let Some(g) = Glyph::from_char(mark) {
                    p.push(glyph_to_token(g));
                }
            }
            p
        })),
    }
}

/// Arbitrary bytes → an ordered program via the canonical mark set → its tuple.
pub fn bytes_to_tuple(bytes: &[u8]) -> IgTuple {
    let mut p = Program::empty();
    for &b in bytes {
        let mark = MARKS[(b as usize) % MARKS.len()];
        if let Some(g) = Glyph::from_char(mark) {
            p.push(glyph_to_token(g));
        }
    }
    imscribe_program(&p)
}

pub fn text_to_tuple(text: &str) -> IgTuple {
    bytes_to_tuple(text.as_bytes())
}

/// Decode a hex string to bytes, then imscribe them as a program.
pub fn hex_to_tuple(hex: &str) -> IgTuple {
    let nibbles: alloc::vec::Vec<u8> = hex
        .trim_start_matches("0x")
        .bytes()
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        })
        .collect();
    let bytes: alloc::vec::Vec<u8> = nibbles
        .chunks(2)
        .map(|c| c[0] * 16 + c.get(1).copied().unwrap_or(0))
        .collect();
    bytes_to_tuple(&bytes)
}

pub fn glyphs(t: &IgTuple) -> String {
    alloc::format!("⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
        t.d.glyph(), t.t.glyph(), t.r.glyph(), t.p.glyph(), t.f.glyph(), t.k.glyph(),
        t.g.glyph(), t.c.glyph(), t.phi.glyph(), t.h.glyph(), t.s.glyph(), t.omega.glyph())
}

#[cfg(test)]
mod kin_chi_probe {
    // A native_numeral word is a run of packets, one per bit
    // (encode_biguint, native_numeral.rs:82-89): ≻⋈∈[⊤|⊥]∋. Four of those
    // five marks are fixed scaffolding, identical for a 0 bit and a 1 bit;
    // only the fifth (Kin=⊤ for a 0 bit, Chi=⊥ for a 1 bit) carries the
    // bit's own value. This probes, on the REAL packet shape (not a bare
    // ⊤/⊥ string), whether varying the underlying bit pattern at a FIXED
    // bit-length moves the imscribed tuple, or whether self_imscribe's
    // composition washes the packets' one variable mark out and converges
    // on the scaffolding alone -- which is what the crystal-address
    // collapse across very different numbers (2, 10, 12345, 8051, 97, a
    // 100-bit value all landing on 16389838) would look like from inside.
    use super::word_to_tuple;
    use num_bigint::BigUint;

    fn probe(label: &str, n: &BigUint) {
        let word = crate::native_numeral::encode(&n.to_string());
        let ig = word_to_tuple(&word);
        crate::nested_println!(
            "{label:>18}  n={n:<18} bits={:<4} {}  addr={}",
            n.bits(),
            super::glyphs(&ig),
            ig.crystal_address(),
        );
    }

    fn all_ones(k: u32) -> BigUint {
        (BigUint::from(1u32) << k) - BigUint::from(1u32)
    }
    fn top_bit_only(k: u32) -> BigUint {
        BigUint::from(1u32) << (k - 1)
    }
    fn alternating(k: u32) -> BigUint {
        // 0b101010...  MSB=1, alternating down, k bits long.
        let mut v = BigUint::from(1u32);
        for i in 1..k {
            v = (v << 1) | BigUint::from((i % 2 == 0) as u32);
        }
        v
    }

    #[test]
    fn same_bit_length_different_content_reported() {
        for k in [4u32, 8, 12, 20, 40, 64] {
            crate::nested_println!("--- k={k} bits ---");
            probe("all-ones", &all_ones(k));
            probe("top-bit-only", &top_bit_only(k));
            probe("alternating", &alternating(k));
        }
    }
}
