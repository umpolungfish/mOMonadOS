// ─── axis_values.rs ────────────────────────────────────────────────────
// One canonical per-axis value table, low ordinal → high, sourced from the
// IgPrim ordinal() table. Shared so the several harvester/derivation tools do
// not each hand-copy it and drift.
#![allow(dead_code)]
extern crate alloc;
use crate::imas_ig::{IgPrim, IgTuple};

pub const D: [IgPrim; 4] = [IgPrim::dead, IgPrim::ash, IgPrim::array, IgPrim::if_];
pub const T: [IgPrim; 5] = [IgPrim::judge, IgPrim::eat, IgPrim::mime, IgPrim::oil, IgPrim::are];
pub const R: [IgPrim; 4] = [IgPrim::ado, IgPrim::tot, IgPrim::ear, IgPrim::ian];
pub const P: [IgPrim; 5] = [IgPrim::church, IgPrim::yew, IgPrim::out, IgPrim::nun, IgPrim::or_];
pub const F: [IgPrim; 3] = [IgPrim::age, IgPrim::they, IgPrim::peep];
pub const K: [IgPrim; 5] = [IgPrim::yea, IgPrim::loll, IgPrim::egg, IgPrim::on, IgPrim::air];
pub const G: [IgPrim; 3] = [IgPrim::bib, IgPrim::thigh, IgPrim::ice];
pub const C: [IgPrim; 4] = [IgPrim::vow, IgPrim::gag, IgPrim::measure, IgPrim::ooze];
pub const PH: [IgPrim; 5] = [IgPrim::woe, IgPrim::monad, IgPrim::roar, IgPrim::err, IgPrim::haha];
pub const H: [IgPrim; 4] = [IgPrim::fee, IgPrim::kick, IgPrim::sure, IgPrim::wool];
pub const S: [IgPrim; 3] = [IgPrim::hung, IgPrim::so, IgPrim::up];
pub const OM: [IgPrim; 4] = [IgPrim::awe, IgPrim::oak, IgPrim::ah, IgPrim::zoo];

pub fn from_indices(b: &[usize; 12]) -> IgTuple {
    IgTuple {
        d: D[b[0] % 4], t: T[b[1] % 5], r: R[b[2] % 4], p: P[b[3] % 5],
        f: F[b[4] % 3], k: K[b[5] % 5], g: G[b[6] % 3], c: C[b[7] % 4],
        phi: PH[b[8] % 5], h: H[b[9] % 4], s: S[b[10] % 3], omega: OM[b[11] % 4],
    }
}

/// Count the twelve opcode marks in an IMASM word; reduce each count into its
/// own axis. Uses the counterfactual MARKS order.
pub fn word_to_tuple(word: &str) -> IgTuple {
    let mut c = [0usize; 12];
    for ch in word.chars() {
        match ch {
            '⊢' => c[0] += 1, '⊣' => c[1] += 1, '>' => c[2] += 1, '<' => c[3] += 1,
            '⋈' => c[4] += 1, '⊙' => c[5] += 1, '∈' => c[6] += 1, '∋' => c[7] += 1,
            '⊤' => c[8] += 1, '⊥' => c[9] += 1, '⊞' => c[10] += 1, '◻' => c[11] += 1,
            _ => {}
        }
    }
    from_indices(&c)
}

pub fn text_to_tuple(text: &str) -> IgTuple {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut b = [0usize; 12];
    for (i, byte) in text.bytes().enumerate() {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
        b[i % 12] = b[i % 12].wrapping_add((h & 0xff) as usize);
    }
    from_indices(&b)
}

pub fn glyphs(t: &IgTuple) -> alloc::string::String {
    alloc::format!("⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
        t.d.glyph(), t.t.glyph(), t.r.glyph(), t.p.glyph(), t.f.glyph(), t.k.glyph(),
        t.g.glyph(), t.c.glyph(), t.phi.glyph(), t.h.glyph(), t.s.glyph(), t.omega.glyph())
}
