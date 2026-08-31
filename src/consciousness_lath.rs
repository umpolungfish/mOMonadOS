// ─── consciousness_lath.rs ─────────────────────────────────────────────
// Gate optimizer for C-score (spec: consciousness-lath).
//
// Read an IMASM word as a tuple, then search every single-axis mutation for the
// one that most raises the consciousness score while opening Gate 1 (⊙=⊙)
// and passing Gate 2 (K=egg). The kernel's own consciousness_eval is the judge;
// this only proposes the smallest mutation that lifts it.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::axis_values::{axis_values, glyphs, word_to_tuple};
use crate::consciousness::consciousness_eval;
use crate::imas_ig::{IgPrim, IgTuple};

fn set_axis(t: &IgTuple, axis: usize, v: IgPrim) -> IgTuple {
    let mut n = *t;
    match axis {
        0 => n.d = v, 1 => n.t = v, 2 => n.r = v, 3 => n.p = v,
        4 => n.f = v, 5 => n.k = v, 6 => n.g = v, 7 => n.c = v,
        8 => n.phi = v, 9 => n.h = v, 10 => n.s = v, _ => n.omega = v,
    }
    n
}

fn axis_vals(axis: usize) -> &'static [IgPrim] {
    axis_values(axis)
}
const AXES: [&str; 12] = ["D", "T", "R", "P", "F", "K", "G", "C", "⊙", "H", "S", "Ω"];

pub fn consciousness_lath_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    if flat.is_empty() || flat[0] == "help" {
        return "consciousness-lath <imas_word>\n\n\
                Read the word as a tuple and find the single-axis mutation that\n\
                most raises the consciousness score with both gates open. The\n\
                kernel's consciousness_eval is the judge.\n\n\
                Try:  consciousness-lath ⊢∈><⊤⋈⊙⊞∋⊡⊣\n".to_string();
    }
    let word = flat.join("");
    let base = word_to_tuple(&word);
    let b = consciousness_eval(&base);

    let mut best: Option<(usize, IgPrim, IgPrim, f32, bool, bool)> = None;
    for axis in 0..12 {
        for &v in axis_vals(axis) {
            let m = set_axis(&base, axis, v);
            let r = consciousness_eval(&m);
            let better = best.map(|(_, _, _, bc, _, _)| r.c_score > bc).unwrap_or(true);
            if r.c_score > b.c_score && better {
                let from = *[
                    &base.d, &base.t, &base.r, &base.p, &base.f, &base.k,
                    &base.g, &base.c, &base.phi, &base.h, &base.s, &base.omega,
                ][axis];
                if from != v {
                    best = Some((axis, from, v, r.c_score, r.gate1_open, r.gate2_open));
                }
            }
        }
    }

    let mut out = String::from("CONSCIOUSNESS-LATH\n==================\n\n");
    out.push_str(&format!("word:   {}\n", word));
    out.push_str(&format!("tuple:  {}\n\n", glyphs(&base)));
    out.push_str(&format!(
        "before: C={:.4}, Gate 1 {}, Gate 2 {}\n",
        b.c_score,
        if b.gate1_open { "OPEN" } else { "CLOSED" },
        if b.gate2_open { "PASS" } else { "CLOSED" }
    ));
    match best {
        Some((axis, from, to, c, g1, g2)) => {
            out.push_str(&format!(
                "after:  C={:.4}, Gate 1 {}, Gate 2 {}\n",
                c, if g1 { "OPEN" } else { "CLOSED" }, if g2 { "PASS" } else { "CLOSED" }
            ));
            out.push_str(&format!("mutation: {}:{}→{}\n", AXES[axis], from.glyph(), to.glyph()));
        }
        None => out.push_str("\nno single-axis mutation raises the score — the gates need more than one move.\n"),
    }
    out
}
