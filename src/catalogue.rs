// ─── catalogue.rs ──────────────────────────────────────────────────────
// Synthesize candidate operators (build.txt §410).
//
// Sweep a bounded family of tuples, score each by NOVELTY (distance to its
// nearest catalog entry — far from everything known = novel), assess its tier,
// and name the closest known family. Program synthesis for mathematical
// structures: what is naturally expressible but not yet named?
//
// The sweep varies the four boundary primitives (D ⊙ Ω ∋) — the axes that move
// between tier cells — over their values, holding a fixed O_∞-ish spine, so the
// candidates are structurally distinct rather than lexical noise.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::algebra::tuple_distance;
use crate::axis_values::glyphs;
use crate::catalog::ordinal_table;
use crate::catalog::catalog_entries;
use crate::cl8nk::assess_tier;
use crate::imas_ig::{IgPrim, IgTuple};

fn nearest(t: &IgTuple) -> (&'static str, f32) {
    let mut best = ("", f32::INFINITY);
    for e in catalog_entries(None) {
        let d = tuple_distance(t, &e.tuple);
        if d < best.1 { best = (e.name, d); }
    }
    best
}

pub fn catalogue_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    let sub = flat.first().copied().unwrap_or("synthesize");
    if sub == "help" {
        return "catalogue synthesize [--top N]\n\n\
                Sweep candidate operators over the four boundary primitives\n\
                (D ⊙ Ω ∋), score each by novelty (distance to nearest catalog\n\
                entry), and rank. What is expressible but not yet named.\n\n\
                Try:  catalogue synthesize --top 5\n".to_string();
    }
    let mut top = 5usize;
    if let Some(i) = flat.iter().position(|&s| s == "--top") {
        top = flat.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(5);
    }

    // Fixed spine (a plausible high-tier body), boundary axes swept.
    let spine = IgTuple {
        d: IgPrim::if_, t: IgPrim::are, r: IgPrim::ian, p: IgPrim::or_,
        f: IgPrim::peep, k: IgPrim::egg, g: IgPrim::ice, c: IgPrim::measure,
        phi: IgPrim::monad, h: IgPrim::wool, s: IgPrim::hung, omega: IgPrim::ah,
    };

    let mut cands: Vec<(IgTuple, &'static str, f32)> = Vec::new();
    for &dv in ordinal_table("⊢") {
        for &pv in ordinal_table("⊙") {
            for &ov in ordinal_table("⊡") {
                for &cv in ordinal_table("∋") {
                    let mut t = spine;
                    t.d = dv; t.phi = pv; t.omega = ov; t.c = cv;
                    let (near, d) = nearest(&t);
                    cands.push((t, near, d));
                }
            }
        }
    }
    // Rank by novelty: farthest from the catalog first.
    cands.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(core::cmp::Ordering::Equal));

    let mut out = String::from("CATALOGUE — SYNTHESIS\n=====================\n\n");
    out.push_str(&format!("swept {} candidates over D ⊙ Ω ∋; top {} by novelty:\n\n", cands.len(), top));
    for (i, (t, near, d)) in cands.iter().take(top).enumerate() {
        out.push_str(&format!("candidate #{}\n", i + 1));
        out.push_str(&format!("  signature:      {}\n", glyphs(t)));
        out.push_str(&format!("  tier:           {}\n", assess_tier(t)));
        out.push_str(&format!("  novelty:        {:.4}  (distance to nearest known)\n", d));
        out.push_str(&format!("  closest family: {}\n\n", near));
    }
    out.push_str("Novelty is distance-to-nearest, not worth: a far candidate is\n\
                  un-named, which is a lead, not a verdict.\n");
    out
}
