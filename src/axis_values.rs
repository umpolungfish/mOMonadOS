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

/// Imscribe an ordered IMASM program through the kernel's structural witness.
fn imscribe_program(p: &Program) -> IgTuple {
    IgTuple::from_snapshot(&self_imscribe(p))
}

/// A glyph word → its tuple. Order-sensitive: this runs the program. Falls back
/// to the byte route only when the word is not a clean mark sequence.
pub fn word_to_tuple(word: &str) -> IgTuple {
    match program_from_glyphs(word) {
        Ok(prog) => imscribe_program(&prog),
        Err(_) => bytes_to_tuple(word.as_bytes()),
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
