// ─── dialect_necromancer.rs ────────────────────────────────────────────
// Text → nearest catalog ghost (spec: dialect-necromancer).
//
// A fragment of text, or a bare 12-glyph tuple, is imscribed to a tuple and
// matched against the whole catalog by real tuple distance. Text derives its
// tuple deterministically (FNV-1a per axis); a tuple is parsed as given.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::algebra::tuple_distance;
use crate::catalog::catalog_entries;
use crate::imas_ig::{IgPrim, IgTuple};

const D: [IgPrim; 4] = [IgPrim::dead, IgPrim::ash, IgPrim::array, IgPrim::if_];
const T: [IgPrim; 5] = [IgPrim::judge, IgPrim::eat, IgPrim::mime, IgPrim::oil, IgPrim::are];
const R: [IgPrim; 4] = [IgPrim::ado, IgPrim::tot, IgPrim::ear, IgPrim::ian];
const P: [IgPrim; 5] = [IgPrim::church, IgPrim::yew, IgPrim::out, IgPrim::nun, IgPrim::or_];
const F: [IgPrim; 3] = [IgPrim::age, IgPrim::they, IgPrim::peep];
const K: [IgPrim; 5] = [IgPrim::yea, IgPrim::loll, IgPrim::egg, IgPrim::on, IgPrim::air];
const G: [IgPrim; 3] = [IgPrim::bib, IgPrim::thigh, IgPrim::ice];
const C: [IgPrim; 4] = [IgPrim::vow, IgPrim::gag, IgPrim::measure, IgPrim::ooze];
const PH: [IgPrim; 5] = [IgPrim::woe, IgPrim::monad, IgPrim::roar, IgPrim::err, IgPrim::haha];
const H: [IgPrim; 4] = [IgPrim::fee, IgPrim::kick, IgPrim::sure, IgPrim::wool];
const S: [IgPrim; 3] = [IgPrim::hung, IgPrim::so, IgPrim::up];
const OM: [IgPrim; 4] = [IgPrim::awe, IgPrim::oak, IgPrim::ah, IgPrim::zoo];

fn text_to_tuple(text: &str) -> IgTuple {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut b = [0usize; 12];
    for (i, byte) in text.bytes().enumerate() {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
        b[i % 12] = b[i % 12].wrapping_add((h & 0xff) as usize);
    }
    IgTuple {
        d: D[b[0] % 4], t: T[b[1] % 5], r: R[b[2] % 4], p: P[b[3] % 5],
        f: F[b[4] % 3], k: K[b[5] % 5], g: G[b[6] % 3], c: C[b[7] % 4],
        phi: PH[b[8] % 5], h: H[b[9] % 4], s: S[b[10] % 3], omega: OM[b[11] % 4],
    }
}

fn glyphs(t: &IgTuple) -> String {
    format!("⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
        t.d.glyph(), t.t.glyph(), t.r.glyph(), t.p.glyph(), t.f.glyph(), t.k.glyph(),
        t.g.glyph(), t.c.glyph(), t.phi.glyph(), t.h.glyph(), t.s.glyph(), t.omega.glyph())
}

pub fn dialect_necromancer_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    if flat.is_empty() || flat[0] == "help" {
        return "dialect-necromancer <text or 12-glyph tuple>\n\n\
                Imscribe a fragment and recover its nearest catalog ghost by real\n\
                tuple distance. A bracketed tuple is parsed as given; text is\n\
                imscribed deterministically.\n\n\
                Try:  dialect-necromancer the boundary imscribes the bulk\n".to_string();
    }
    let joined = flat.join(" ");
    let (tuple, how) = match IgTuple::from_glyphs(&joined) {
        Ok(t) => (t, "parsed tuple"),
        Err(_) => (text_to_tuple(&joined), "imscribed from text"),
    };

    let mut best: Option<(&'static str, &'static str, f32)> = None;
    for e in catalog_entries(None) {
        let d = tuple_distance(&tuple, &e.tuple);
        if best.map(|(_, _, bd)| d < bd).unwrap_or(true) {
            best = Some((e.name, e.description, d));
        }
    }

    let mut out = String::from("DIALECT-NECROMANCER\n===================\n\n");
    out.push_str(&format!("fragment:  {}\n", joined));
    out.push_str(&format!("imscribed: {}  ({})\n\n", glyphs(&tuple), how));
    match best {
        Some((name, desc, d)) => {
            out.push_str(&format!("nearest ghost: {}\n", name));
            out.push_str(&format!("distance:      {:.4}\n", d));
            out.push_str(&format!("its tuple:     — {}\n", desc));
        }
        None => out.push_str("the catalog is empty; no ghost to raise.\n"),
    }
    out
}
