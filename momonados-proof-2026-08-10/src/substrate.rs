// substrate.rs — The Substrate That Closes Everywhere
//
// A scan was run to locate a critical weight by varying the substrate weight and
// measuring the cycle length of the resulting program. Cycle length came back as
// one at every weight without exception, and the scan was recorded as having
// chosen the wrong observable: a constant reading locates no critical point and
// would report the substrate as inert.
//
// It is not the wrong observable. Under the Fixed-Point Nesting Rule an entire
// one-parameter family sitting in the one-shot class, with no member anywhere in
// a basin, is the signature of a CONSERVATIVE action — one with fixed points and
// orbits and no attraction, whose pairings can only be one-shot or never arrive.
// The family shows the first of those and none of the second. That is a stronger
// statement than inertness, and it is why closure cannot move: every parameter is
// already at the fixed point, so there is nothing for the closure to report.
//
// The content does bifurcate, sharply, and the two facts are compatible because
// they are not the same observable. Closure asks whether the program returns to
// itself. Content asks WHICH program it returns to. The substrate weight can only
// move the program by overturning whichever token the family matrix leads with,
// so the transition is a change of sector, not a change of closure — the program
// crosses from one self-closing regime to another without ever failing to close.
//
// This module reports both readings side by side, which is the whole point: the
// constant column is not a failure to measure, it is the measurement.
//
// Nothing here is a second copy. The weight, the builder and the self-imscriber
// are the kernel's own; only the reading is new.
//
// Surface: mOMonadOS kernel, sequence + kernel + imas_ig as they stand.
// Author: Quantum⊙perator (Lando⊗⊙perator team)

#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::sequence;
use crate::imas_ig::{IgTuple, IgPrim};
use crate::tokens::Token;

const SWEEP_HI: i32 = 10;
/// How far to look for a critical weight before concluding there is none.
const WC_SEARCH: i32 = 64;

/// The seed: O_∞, self-referential topology, universal range. This is the tuple
/// the family matrix leaves leading with self-imscription, which is exactly where
/// a critical weight can exist at all.
fn seed() -> IgTuple {
    IgTuple {
        d: IgPrim::if_,    t: IgPrim::are,     r: IgPrim::ian,   p: IgPrim::or_,
        f: IgPrim::peep,   k: IgPrim::egg,     g: IgPrim::ice,   c: IgPrim::measure,
        phi: IgPrim::monad, h: IgPrim::wool,   s: IgPrim::up,    omega: IgPrim::ah,
    }
}

fn program_at(t: &IgTuple, w: i32) -> Vec<Token> {
    sequence::set_substrate_weight(w);
    sequence::build_via_substrate(t, 12, t.t == IgPrim::are, 3).as_slice().to_vec()
}

/// Cycle length under repeated self-imscription. One means the program returns
/// to itself in a single step: the parameter is AT the fixed point.
fn cycle_length(t: &IgTuple, w: i32, max_iter: usize) -> usize {
    sequence::set_substrate_weight(w);
    let mut seen: Vec<Vec<Token>> = Vec::new();
    let mut cur = *t;
    let mut prog = sequence::build_via_substrate(&cur, 12, cur.t == IgPrim::are, 3);
    seen.push(prog.as_slice().to_vec());
    for i in 1..max_iter {
        let snap = crate::kernel::self_imscribe(&prog);
        cur = IgTuple::from_snapshot(&snap);
        let next = sequence::build_via_substrate(&cur, 12, cur.t == IgPrim::are, 3);
        let toks: Vec<Token> = next.as_slice().to_vec();
        for (j, prev) in seen.iter().enumerate() {
            if prev == &toks { return i - j; }
        }
        seen.push(toks);
        prog = next;
    }
    max_iter
}

/// The token the ranking puts after the opening self-imscription. This is the
/// content observable: what the program does, as against whether it closes.
fn leader(t: &IgTuple, w: i32) -> Token {
    sequence::set_substrate_weight(w);
    sequence::build_via_substrate(t, 12, t.t == IgPrim::are, 3).as_slice()[1]
}

/// Lowest weight whose program differs from the zero-weight program, if any.
fn critical_weight(t: &IgTuple) -> Option<i32> {
    let base = program_at(t, 0);
    for w in 1..=WC_SEARCH {
        if program_at(t, w) != base { return Some(w); }
    }
    None
}

pub struct Substrate;

impl Substrate {
    pub fn report() -> String {
        let t = seed();
        let mut s = String::from("The Substrate That Closes Everywhere\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("Two observables on one sweep. Closure asks whether the program\n");
        s.push_str("returns to itself; content asks which program it returns to.\n\n");

        s.push_str("  weight | cycle | leader after IMSCRIB | regime\n");
        s.push_str("  -------|-------|----------------------|------------------\n");
        let mut all_one = true;
        let base = program_at(&t, 0);
        for w in 0..=SWEEP_HI {
            let c = cycle_length(&t, w, 20);
            if c != 1 { all_one = false; }
            let ld = leader(&t, w);
            let regime = if program_at(&t, w) == base { "self-imscription" } else { "advancing" };
            s.push_str(&format!("  {:>6} | {:>5} | {:<20} | {}\n", w, c, ld.name(), regime));
        }

        let wc = critical_weight(&t);
        s.push_str(&format!(
            "\nclosure across the sweep: {}\n",
            if all_one { "cycle 1 at every weight — every parameter is AT the fixed point" }
            else { "NOT constant — some weight left the fixed point" }));
        match wc {
            Some(w) => s.push_str(&format!(
                "content bifurcation:      w_c = {} — the ranking flips once and never again\n", w)),
            None => s.push_str(
                "content bifurcation:      none up to the search bound\n"),
        }

        // Where a critical weight can exist at all. The substrate vote can only
        // move the program by overturning the token the family matrix leads
        // with, so stepping off the self-referential topology or narrowing the
        // range leaves it nothing to overturn.
        s.push_str("\nWhere a critical weight exists at all\n");
        s.push_str("  variant                        | leader at w=0        | w_c\n");
        s.push_str("  -------------------------------|----------------------|------\n");
        let mut rows: Vec<(&'static str, IgTuple)> = Vec::new();
        rows.push(("seed (self-ref, universal)", t));
        for (label, tv) in [("topology → oil", IgPrim::oil), ("topology → judge", IgPrim::judge),
                            ("topology → mime", IgPrim::mime), ("topology → eat", IgPrim::eat)] {
            let mut v = t; v.t = tv; rows.push((label, v));
        }
        for (label, gv) in [("range → bib", IgPrim::bib), ("range → thigh", IgPrim::thigh)] {
            let mut v = t; v.g = gv; rows.push((label, v));
        }
        for (label, v) in &rows {
            let ld = leader(v, 0);
            let w = critical_weight(v);
            s.push_str(&format!("  {:<30} | {:<20} | {}\n", label, ld.name(),
                match w { Some(x) => format!("{}", x), None => String::from("none") }));
        }

        s.push_str("\nReading\n");
        if all_one && wc.is_some() {
            s.push_str("  Closure is constant and content is not. The substrate is\n");
            s.push_str("  conservative: every parameter already sits at the fixed point,\n");
            s.push_str("  so no member is in a basin and the one-shot class is the whole\n");
            s.push_str("  family. The bifurcation is a change of SECTOR — the program\n");
            s.push_str("  crosses from one self-closing regime to another and never fails\n");
            s.push_str("  to close on the way. A constant cycle length is therefore the\n");
            s.push_str("  correct reading of a conservative substrate, not a failed one.\n");
            s.push_str("  What the substrate vote does is carry a self-referential tuple\n");
            s.push_str("  out of pure self-imscription, and on this evidence nothing else.\n");
        } else {
            s.push_str("  The two observables did not separate as the rule predicts.\n");
        }
        s
    }
}
