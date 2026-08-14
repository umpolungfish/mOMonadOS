// ─── universe_wormhole.rs ──────────────────────────────────────────────
// Inter-framework deformation path (spec: universe-wormhole).
//
// Given two frameworks from `hop`, take the real gate-space distance between
// their tuples, render the gap as a braid word (one crossing per differing
// axis), and read its Jones magnitude as the topological invariant of the path.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String};
use alloc::vec::Vec;
use crate::fibonacci_qc::jones_polynomial;
use crate::hop::{distance, find_framework, FRAMEWORKS};

fn count_mismatches(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

pub fn universe_wormhole_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    if flat.len() < 2 {
        let mut s = String::from(
            "universe-wormhole <origin> <target>\n\n\
             The minimum gate-space path between two hop frameworks, rendered as\n\
             a braid whose Jones magnitude is the path invariant.\n\n\
             frameworks:\n",
        );
        for (n, _, d) in FRAMEWORKS { s.push_str(&format!("    {:10} {}\n", n, d)); }
        s.push_str("\nTry:  universe-wormhole hqe fibonacci\n");
        return s;
    }
    let (o, t) = (flat[0], flat[1]);
    let (on, ot) = match find_framework(o) {
        Some(x) => x, None => return format!("no framework '{}'\n", o),
    };
    let (tn, tt) = match find_framework(t) {
        Some(x) => x, None => return format!("no framework '{}'\n", t),
    };
    let d = distance(ot, tt);
    let mism = count_mismatches(ot, tt);

    // One crossing per differing axis, alternating strand — the coarsest braid
    // realizing a path of that many moves.
    let mut braid: Vec<i32> = Vec::new();
    for k in 0..mism { braid.push(if k % 2 == 0 { 1 } else { 2 }); }
    let jones = if braid.is_empty() { 1.0 } else { jones_polynomial(3, &braid).norm() };

    let mut bstr = String::new();
    for (k, g) in braid.iter().enumerate() {
        if k > 0 { bstr.push(' '); }
        bstr.push_str(&format!("σ{}", g));
    }
    if bstr.is_empty() { bstr.push_str("(identity — same tuple)"); }

    let mut out = String::from("UNIVERSE-WORMHOLE\n=================\n\n");
    out.push_str(&format!("origin:  {} — {}\n", on, ot));
    out.push_str(&format!("target:  {} — {}\n\n", tn, tt));
    out.push_str(&format!("differing axes:    {}\n", mism));
    out.push_str(&format!("deformation cost:  {:.4}  (gate-space distance)\n", d));
    out.push_str(&format!("braid:             {}\n", bstr));
    out.push_str(&format!("Jones invariant:   {:.6}\n", jones));
    out
}
