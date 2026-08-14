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
use crate::axis_values::{glyphs, text_to_tuple};
use crate::imas_ig::IgTuple;

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
