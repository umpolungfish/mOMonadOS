// ─── crystal_scope.rs ──────────────────────────────────────────────────
// Primitive substitution microscope (build.txt, useful §crystal-scope).
//
// One diagnostic where three commands stood before: for a change to a 12-glyph
// tuple, report the distance, the tier transition, the entropy delta, the
// crystal-address jump, and WHICH PRIMITIVE DROVE IT.
//
// The driver is measured, not guessed. For each of the twelve axes it forms the
// tuple that differs from A in that axis alone and takes its distance under the
// same weighted metric the rest of the kernel uses. That marginal is the axis's
// actual contribution, so the largest one is the driver by construction rather
// than by intuition about which primitives "matter".
//
// Everything else is a lookup into machinery that already exists:
//   algebra::tuple_distance, algebra::primitive_mismatches   weighted + Hamming
//   cl8nk::tuple_distance_cl8nk                              graded conflicts
//   cl8nk::assess_tier -> entropy::Tier                      tier and S = ln N
//   IgTuple::crystal_address                                 gate-space address
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::algebra::{primitive_mismatches, tuple_distance};
use crate::cl8nk::{assess_tier, tuple_distance_cl8nk};
use crate::entropy::Tier;
use crate::imas_ig::{IgPrim, IgTuple};

/// The twelve axes, in canonical order, with a getter and a setter each.
/// Written out rather than derived so the order is the catalog's order and
/// cannot drift from it silently.
const AXES: [&str; 12] = [
    "D", "T", "R", "P", "F", "K", "G", "C", "Phi", "H", "S", "Omega",
];

fn get_axis(t: &IgTuple, i: usize) -> IgPrim {
    match i {
        0 => t.d,
        1 => t.t,
        2 => t.r,
        3 => t.p,
        4 => t.f,
        5 => t.k,
        6 => t.g,
        7 => t.c,
        8 => t.phi,
        9 => t.h,
        10 => t.s,
        _ => t.omega,
    }
}

fn set_axis(t: &IgTuple, i: usize, v: IgPrim) -> IgTuple {
    let mut o = *t;
    match i {
        0 => o.d = v,
        1 => o.t = v,
        2 => o.r = v,
        3 => o.p = v,
        4 => o.f = v,
        5 => o.k = v,
        6 => o.g = v,
        7 => o.c = v,
        8 => o.phi = v,
        9 => o.h = v,
        10 => o.s = v,
        _ => o.omega = v,
    }
    o
}

/// Map the tier NAME onto the entropy tier, so S is a lookup and not a guess.
/// `cl8nk::assess_tier` and `entropy::Tier::name` already agree on these
/// strings; if they ever stop agreeing this returns None and the report says
/// the entropy is undetermined rather than inventing a number.
fn tier_of(t: &IgTuple) -> Option<Tier> {
    let name = assess_tier(t);
    Tier::all().into_iter().find(|x| x.name() == name)
}

pub struct AxisMove {
    pub axis: &'static str,
    pub from: IgPrim,
    pub to: IgPrim,
    /// Distance contributed by moving THIS axis alone.
    pub marginal: f32,
}

pub struct Scope {
    pub a: IgTuple,
    pub b: IgTuple,
    pub mismatches: u8,
    pub weighted: f32,
    pub cl8nk: f32,
    pub tier_a: Option<Tier>,
    pub tier_b: Option<Tier>,
    pub addr_a: u32,
    pub addr_b: u32,
    pub moves: Vec<AxisMove>,
    /// The driver, lifted out of `moves` for callers that want it directly.
    /// `moves` is sorted by marginal, so this is `moves[0]` and stays in step
    /// with it — one computation, two spellings, never two answers.
    pub driver_axis: Option<&'static str>,
    pub driver_marginal: f32,
    /// S(to) - S(from) over the entropy tiers, 0.0 when either tier is unknown.
    pub entropy_delta: f32,
}

pub fn scope(a: &IgTuple, b: &IgTuple) -> Scope {
    let mut moves: Vec<AxisMove> = Vec::new();
    for i in 0..12 {
        let (from, to) = (get_axis(a, i), get_axis(b, i));
        if from != to {
            // A differing in this axis ALONE: the axis's own contribution.
            let only = set_axis(a, i, to);
            moves.push(AxisMove {
                axis: AXES[i],
                from,
                to,
                marginal: tuple_distance(a, &only),
            });
        }
    }
    // Largest marginal first: the driver is the head of this list.
    moves.sort_by(|x, y| {
        y.marginal
            .partial_cmp(&x.marginal)
            .unwrap_or(core::cmp::Ordering::Equal)
    });

    let (cl8nk, _conflicts) = tuple_distance_cl8nk(a, b);

    let driver_axis = moves.first().map(|m| m.axis);
    let driver_marginal = moves.first().map(|m| m.marginal).unwrap_or(0.0);
    let (ta, tb) = (tier_of(a), tier_of(b));
    let entropy_delta = match (ta, tb) {
        (Some(x), Some(y)) => y.entropy() - x.entropy(),
        _ => 0.0,
    };

    Scope {
        a: *a,
        b: *b,
        mismatches: primitive_mismatches(a, b),
        weighted: tuple_distance(a, b),
        cl8nk,
        tier_a: ta,
        tier_b: tb,
        addr_a: a.crystal_address(),
        addr_b: b.crystal_address(),
        moves,
        driver_axis,
        driver_marginal,
        entropy_delta,
    }
}

pub fn format_scope(s: &Scope) -> String {
    let mut out = String::new();
    out.push_str("CRYSTAL-SCOPE\n=============\n\n");
    out.push_str(&format!("from:  {}\n", s.a.display()));
    out.push_str(&format!("to:    {}\n\n", s.b.display()));

    if s.mismatches == 0 {
        out.push_str("The two tuples are identical. Nothing moved.\n");
        return out;
    }

    // Tier transition and entropy delta.
    match (s.tier_a, s.tier_b) {
        (Some(ta), Some(tb)) => {
            out.push_str(&format!(
                "transition:      {} -> {}{}\n",
                ta.name(),
                tb.name(),
                if ta == tb { "  (no tier change)" } else { "" }
            ));
            let ds = tb.entropy() - ta.entropy();
            out.push_str(&format!(
                "delta-S:         {}{:.4}   (S = ln N_tier: {:.4} -> {:.4})\n",
                if ds > 0.0 { "+" } else { "" },
                ds,
                ta.entropy(),
                tb.entropy()
            ));
        }
        _ => {
            out.push_str(
                "transition:      UNDETERMINED — the tier name did not match an\n\
                 \x20                entropy tier, so no delta-S is reported.\n",
            );
        }
    }

    out.push_str(&format!(
        "distance:        {:.4} weighted, {:.4} cl8nk, {} of 12 primitives differ\n",
        s.weighted, s.cl8nk, s.mismatches
    ));
    out.push_str(&format!(
        "gate-space jump: {} -> {}  (delta {})\n\n",
        s.addr_a,
        s.addr_b,
        if s.addr_b >= s.addr_a {
            format!("+{}", s.addr_b - s.addr_a)
        } else {
            format!("-{}", s.addr_a - s.addr_b)
        }
    ));

    // The driver, and every other axis that moved, by measured marginal.
    match s.moves.first() {
        Some(d) => {
            out.push_str(&format!(
                "driver:          {}  ({} -> {}), marginal {:.4}\n",
                d.axis,
                d.from.glyph(),
                d.to.glyph(),
                d.marginal
            ));
            if s.moves.len() > 1 {
                out.push_str("\nevery axis that moved, by measured marginal:\n");
                for m in &s.moves {
                    out.push_str(&format!(
                        "    {:<6} {} -> {}   {:.4}\n",
                        m.axis,
                        m.from.glyph(),
                        m.to.glyph(),
                        m.marginal
                    ));
                }
            }
        }
        None => {
            out.push_str("driver:          none — the tuples differ in no axis\n");
        }
    }

    out.push_str(
        "\nThe marginal is the distance moving that axis ALONE, under the same\n\
         weighted metric as the total. Marginals need not sum to the total: the\n\
         metric mixes categorical mismatch with ordinal gaps.\n",
    );
    out
}

pub fn crystal_scope_main(args: &[&str]) -> String {
    if args.len() < 2 {
        return "crystal-scope <tuple-A> <tuple-B>   (alias: cscope)\n\
                \n\
                One diagnostic for a change to a 12-glyph tuple: distance, tier\n\
                transition, delta-S, gate-space jump, and which primitive drove it.\n\
                \n\
                The driver is measured — each axis is moved alone and its own\n\
                distance taken — not inferred from which primitives look important.\n\
                \n\
                Both tuples are twelve primitive glyphs; paste them from `ig`.\n"
            .to_string();
    }

    // Two tuples of twelve glyphs each. Split the joined glyph run in half so a
    // caller can paste them with or without the bracket-and-dot decoration.
    let joined: String = args.join(" ");
    let cleaned: String = joined
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '⟨' && *c != '⟩' && *c != '·' && *c != ',' && *c != ';')
        .collect();
    let n = cleaned.chars().count();
    if n != 24 {
        return format!(
            "expected two tuples of twelve glyphs (24 total), got {}.\n",
            n
        );
    }
    let first: String = cleaned.chars().take(12).collect();
    let second: String = cleaned.chars().skip(12).collect();

    let a = match IgTuple::from_glyphs(&first) {
        Ok(t) => t,
        Err((i, m)) => return format!("tuple A bad at {}: {}\n", i, m),
    };
    let b = match IgTuple::from_glyphs(&second) {
        Ok(t) => t,
        Err((i, m)) => return format!("tuple B bad at {}: {}\n", i, m),
    };

    format_scope(&scope(&a, &b))
}
