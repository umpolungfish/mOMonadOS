// nesting.rs — The Observable That Completes the Split
//
// The Fixed-Point Nesting Rule is decidable in advance from the pre-nest
// residual ‖B(A) − A‖: where it vanishes the class is one-shot, and that has
// never misread, including on an adversarial set built to fool it. But the
// residual is faithful to a TWO-way split only. Any nonzero value returns the
// same verdict — attracted-or-open — and the separation of iterated closure
// (finite budget) from no closure (never arrives) has to be supplied from
// outside, as knowledge of whether the outer is conservative or dissipative.
// So the class that costs a budget and the class that costs everything are
// told apart by a fact the pairing itself is never asked for.
//
// This module asks the pairing. The completing observable is the RATIO of two
// successive residuals:
//
//     r1 = ‖B(A)  − A‖          the residual the rule already uses
//     r2 = ‖B(B(A)) − B(A)‖     the same residual one step downstream
//     q  = r2 / r1              the local contraction factor
//
// q < 1 : the gap is shrinking — the inner is being walked home. Attracted.
// q ≈ 1 : the gap is preserved — a translation or an orbit. Never arrives.
// q > 1 : the gap is growing — repelled. Never arrives.
//
// Attraction is a property of how the residual CHANGES, which is why one step
// cannot carry it and two can. The minimum is two: a single residual is one
// number and the split needs a comparison. The cost is one extra application of
// the outer, against a nest that otherwise runs to an iteration cap.
//
// Surface: mOMonadOS kernel, no_std, scratch f64 from constant_closure.
// Author: Quantum⊙perator (Lando⊗⊙perator team)

#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::constant_closure::f64_sqrt;

/// Residual below this counts as zero: the inner IS the fixed point.
const TOL: f64 = 1e-9;
/// How close q must sit to 1 for the gap to count as preserved rather than
/// shrinking. A near-fixed point must NOT be read as one-shot, and a genuine
/// translation must NOT be read as attracted, so this band is deliberately tight.
const FLAT_BAND: f64 = 1e-6;

fn abs(x: f64) -> f64 { if x < 0.0 { -x } else { x } }

/// Distance in R^n. Scalars are the n=1 case, so one metric serves both.
fn dist(a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    for i in 0..a.len() { let d = a[i] - b[i]; s += d * d; }
    f64_sqrt(s)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Class { OneShot, Attracted, NeverArrives }

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::OneShot      => "one-shot",
            Class::Attracted    => "attracted (finite budget)",
            Class::NeverArrives => "never arrives",
        }
    }
}

/// What the two-step reading says, before any nest is run.
pub struct Reading {
    pub r1: f64,
    pub r2: f64,
    /// The completing observable. None when r1 is zero — a possessed fixed
    /// point needs no ratio, and dividing by zero to get one would invent a
    /// number the pairing never produced.
    pub q: Option<f64>,
    pub class: Class,
    /// What the residual ALONE could say. Kept beside the new reading so the
    /// gain is visible rather than claimed.
    pub one_step: &'static str,
}

/// Read the pairing in two steps. This is the whole content of the module.
pub fn read(f: fn(&[f64], &mut [f64]), a: &[f64]) -> Reading {
    let n = a.len();
    let mut b1 = alloc::vec![0.0; n];
    let mut b2 = alloc::vec![0.0; n];
    f(a, &mut b1);
    let r1 = dist(&b1, a);

    if r1 < TOL {
        return Reading { r1, r2: 0.0, q: None, class: Class::OneShot, one_step: "one-shot" };
    }

    f(&b1, &mut b2);
    let r2 = dist(&b2, &b1);
    let q = r2 / r1;

    // The gap is shrinking, preserved, or growing. Only the first arrives.
    let class = if q < 1.0 - FLAT_BAND { Class::Attracted } else { Class::NeverArrives };

    Reading { r1, r2, q: Some(q), class, one_step: "attracted-or-open" }
}

/// Run the nest to confirm what the reading predicted. This is the check, not
/// the instrument: the reading is made first and the nest is allowed to
/// disagree with it.
///
/// There is no iteration budget. The loop ends on a condition the run itself
/// produces: either the gap closes, or a step fails to shrink it. A gap that
/// stops shrinking is the run saying it will not arrive, which is the answer
/// being asked for rather than a limit being hit — so "never arrives" is
/// reported at the step that demonstrates it, not after an arbitrary wait.
///
/// The honest edge: a map that converges non-monotonically, widening the gap on
/// some step before closing in later, would be stopped at that widening and
/// called open. Every map offered here shrinks monotonically inside its basin,
/// so none of them meets that edge, and a map that did would need its own
/// stopping condition rather than a bigger number.
pub fn confirm(f: fn(&[f64], &mut [f64]), a: &[f64]) -> (bool, u32) {
    let n = a.len();
    let mut x: Vec<f64> = a.to_vec();
    let mut nx = alloc::vec![0.0; n];
    let mut prev_gap = f64::INFINITY;
    let mut k: u32 = 0;
    loop {
        f(&x, &mut nx);
        let gap = dist(&nx, &x);
        if gap < TOL { return (true, k); }
        if gap >= prev_gap { return (false, k); }
        prev_gap = gap;
        x.copy_from_slice(&nx);
        k += 1;
    }
}

// ── The operator families, one per axis ─────────────────────────────────────
// Dissipative maps populate all three classes; conservative maps populate only
// the extremes. Both are here, because the observable has to hold in both.

/// Contraction toward v = 3: x ↦ v + (x−v)/2. Dissipative — has a basin.
fn contraction(x: &[f64], out: &mut [f64]) { out[0] = 3.0 + (x[0] - 3.0) * 0.5; }

/// Newton on x³ − 2x − 5. Dissipative — quadratic convergence in the basin.
fn newton(x: &[f64], out: &mut [f64]) {
    let v = x[0];
    let fx = v * v * v - 2.0 * v - 5.0;
    let dfx = 3.0 * v * v - 2.0;
    out[0] = v - fx / dfx;
}

/// Translation x ↦ x + 1. Fixed-point free, and its residual is CONSTANT at 1 —
/// the trap that would lure a predictor reading magnitude instead of change.
fn translate(x: &[f64], out: &mut [f64]) { out[0] = x[0] + 1.0; }

/// Rotation by a third of a turn in the plane. Conservative: a genuine limit
/// cycle, whose residual is also constant and which never arrives anywhere.
fn rotate3(x: &[f64], out: &mut [f64]) {
    let c = -0.5;
    let s = 0.866_025_403_784_438_6; // sin(2π/3)
    out[0] = c * x[0] - s * x[1];
    out[1] = s * x[0] + c * x[1];
}

/// Projection onto the first coordinate. Dissipative and instant: one step to
/// the fixed set from anywhere.
fn project(x: &[f64], out: &mut [f64]) { out[0] = x[0]; out[1] = 0.0; }

/// Greedy unit-fraction removal — the Erdős–Straus action.
///
/// The outer action is one greedy Fibonacci–Sylvester step: from `x`, subtract
/// the largest unit fraction that fits, `1/⌈1/x⌉`. The nested point is `4/n`,
/// and zero is the fixed point: a representation of `4/n` as unit fractions IS
/// an orbit that arrives.
///
/// So the conjecture is a BUDGET on this nesting, not a question of whether it
/// arrives — Fibonacci–Sylvester always arrives, for every rational in (0,1).
/// Erdős and Straus ask whether the budget is three for every `n ≥ 2`. That is
/// the shape the fixed-point rule assigns it: not the "never" bin, where the ctc
/// machinery would be needed, but the attracted bin with a step count asked to
/// be uniform. Reading it here is what makes the distinction measurable rather
/// than asserted.
fn greedy_unit(x: &[f64], out: &mut [f64]) {
    let v = x[0];
    if v <= 0.0 {
        out[0] = 0.0;
        return;
    }
    // ⌈1/v⌉ without libm: 1/v is well inside f64 range for the sizes read here.
    let inv = 1.0 / v;
    let mut d = inv as i64;
    if (d as f64) < inv { d += 1; }
    if d < 1 { d = 1; }
    let r = v - 1.0 / (d as f64);
    out[0] = if r < 1e-15 { 0.0 } else { r };
}


/// The Collatz action, as a nesting.
///
/// The raw shortcut map `n/2 | (3n+1)/2` is not a contraction: an odd step
/// raises the value, so the two-gap reading would call almost every point
/// "never arrives" and the confirm loop would stop at the first widening. That
/// is the documented edge of this module, and it is a fact about which action
/// was nested, not about the map.
///
/// The action that nests is the BLOCK: from `n`, apply the shortcut map until
/// the value first falls below `n`. Every block strictly decreases by
/// construction, so the gap shrinks monotonically and the reading is licensed.
/// One is the fixed point, held outright — `T(1) = 2`, `T(2) = 1`, so the orbit
/// through one never drops below it and the block returns one unchanged.
///
/// Shaped like `greedy_unit`: the outer action is one block, arriving is
/// reaching one, and the conjecture is the BUDGET on blocks rather than a
/// question of whether the nest arrives. What the depth split measures is
/// whether the first block terminates at all, which is where the open arm sits.
/// A block that does not close within its step allowance returns the value
/// unchanged, so the reading reports a preserved gap rather than inventing an
/// arrival.
fn collatz_block(x: &[f64], out: &mut [f64]) {
    let n0 = x[0];
    if !(n0 > 1.0) {
        out[0] = 1.0;
        return;
    }
    let start = n0 as u64;
    let mut n = start;
    // The allowance is a spend, not a truth: a block that exceeds it is
    // reported as a held gap and the caller sees the budget it cost.
    let mut steps = 0u32;
    while steps < 4096 {
        n = if n % 2 == 0 { n / 2 } else { (3 * n + 1) / 2 };
        steps += 1;
        if n < start {
            out[0] = n as f64;
            return;
        }
    }
    out[0] = n0;
}


/// Some actions arrive at a named point rather than at a vanishing step. The
/// Collatz block is one: it decreases its VALUE monotonically by construction,
/// while the size of the step it takes does not decrease at all. Reading such an
/// action by step size stops it at the first widening and calls it open, which
/// measures the observable rather than the action. Naming the target lets the
/// gap be read where the action actually closes.
pub fn target_by_name(name: &str) -> Option<f64> {
    match name {
        "collatz" => Some(1.0),
        _ => None,
    }
}

/// Read a pairing whose gap is the distance to a named target. Two gaps still
/// do the work; only what a gap MEANS changes.
pub fn read_to_target(f: fn(&[f64], &mut [f64]), a: &[f64], target: f64) -> Reading {
    let n = a.len();
    let mut b1 = alloc::vec![0.0; n];
    let mut b2 = alloc::vec![0.0; n];
    let t = [target];
    let r0 = dist(a, &t);
    if r0 < TOL {
        return Reading { r1: 0.0, r2: 0.0, q: None, class: Class::OneShot, one_step: "one-shot" };
    }
    f(a, &mut b1);
    let r1 = dist(&b1, &t);
    if r1 < TOL {
        return Reading { r1, r2: 0.0, q: None, class: Class::Attracted, one_step: "attracted-or-open" };
    }
    f(&b1, &mut b2);
    let r2 = dist(&b2, &t);
    let q = r2 / r1;
    let class = if q < 1.0 - FLAT_BAND { Class::Attracted } else { Class::NeverArrives };
    Reading { r1, r2, q: Some(q), class, one_step: "attracted-or-open" }
}

/// Run the nest against a named target. The loop ends on arrival or on a step
/// that fails to close the gap, and the step count IS the budget the action
/// spends — the quantity a conjecture about this action is about.
pub fn confirm_to_target(f: fn(&[f64], &mut [f64]), a: &[f64], target: f64) -> (bool, u32) {
    let n = a.len();
    let t = [target];
    let mut cur = a.to_vec();
    let mut nxt = alloc::vec![0.0; n];
    let mut prev = dist(&cur, &t);
    let mut steps = 0u32;
    loop {
        if prev < TOL { return (true, steps); }
        f(&cur, &mut nxt);
        let d = dist(&nxt, &t);
        if d >= prev { return (false, steps); }
        for i in 0..n { cur[i] = nxt[i]; }
        prev = d;
        steps += 1;
    }
}

pub struct Nesting;

/// Resolve a map by name, with the dimension it acts on. Naming the map is what
/// lets a caller read a pairing they care about rather than the ones I picked.
pub fn map_by_name(name: &str) -> Option<(fn(&[f64], &mut [f64]), usize)> {
    match name {
        "halve"   => Some((contraction, 1)),
        "newton"  => Some((newton, 1)),
        "shift"   => Some((translate, 1)),
        "rotate"  => Some((rotate3, 2)),
        "project" => Some((project, 2)),
        "greedy"  => Some((greedy_unit, 1)),
        "collatz" => Some((collatz_block, 1)),
        _ => None,
    }
}

pub const MAPS: [&str; 7] = ["halve", "newton", "shift", "rotate", "project", "greedy", "collatz"];

fn describe(label: &str, f: fn(&[f64], &mut [f64]), a: &[f64]) -> String {
    describe_to(label, f, a, None)
}

/// `target` names the point the action arrives AT, when it has one. Without it
/// the gap is the size of the step, which is the right reading for every map
/// that closes by standing still and the wrong one for an action that walks a
/// value down while its stride varies.
fn describe_to(label: &str, f: fn(&[f64], &mut [f64]), a: &[f64], target: Option<f64>) -> String {
    let (rd, closed, steps) = match target {
        Some(t) => {
            let r = read_to_target(f, a, t);
            let (c, n) = confirm_to_target(f, a, t);
            (r, c, n)
        }
        None => {
            let r = read(f, a);
            let (c, n) = confirm(f, a);
            (r, c, n)
        }
    };
    let mut s = format!("{}\n", label);
    s.push_str(&format!("  first gap  r1 = {:.6e}\n", rd.r1));
    match rd.q {
        Some(q) => s.push_str(&format!("  second gap r2 = {:.6e}   q = r2/r1 = {:.6}\n", rd.r2, q)),
        None    => s.push_str("  second gap —              q = —  (first gap already zero)\n"),
    }
    s.push_str(&format!("  from one gap:  {}\n", rd.one_step));
    s.push_str(&format!("  from two gaps: {}\n", rd.class.name()));
    if let Some(t) = target {
        s.push_str(&format!("  gap read to:   the target {}\n", t));
    }
    s.push_str(&format!("  running it:    {}\n",
        if closed { format!("settled in {} step(s)", steps) }
        else { format!("stopped shrinking at step {} — it never arrives", steps) }));
    let agrees = match rd.class {
        Class::OneShot      => closed && steps == 0,
        Class::Attracted    => closed,
        Class::NeverArrives => !closed,
    };
    s.push_str(&format!("  prediction:    {}\n", if agrees { "HELD" } else { "MISSED" }));
    s
}

impl Nesting {
    /// What this command does and every form it takes, from the command itself.
    pub fn help() -> String {
        let mut s = String::from("nesting — read a point against a map before running it\n\n");
        s.push_str("One gap tells you whether the point is already the answer, and nothing\n");
        s.push_str("more: every other case reads the same. Two gaps tell you the rest. If\n");
        s.push_str("the gap shrank it is being drawn in and will arrive; if it held or grew\n");
        s.push_str("it never will. The cost is one extra application of the map.\n\n");
        s.push_str("  nesting                    the reference pairings\n");
        s.push_str("  nesting <map> <x>          one point, for a map on the line\n");
        s.push_str("  nesting <map> <x> <y>      one point, for a map on the plane\n");
        s.push_str("  nesting help               this\n\n");
        s.push_str("maps:\n");
        for (name, dim, what) in [
            ("greedy",  1, "greedy unit-fraction removal — Erdős–Straus is its BUDGET, not its arrival"),
            ("collatz", 1, "one Collatz block, down to the first value below n — Collatz is its BUDGET too"),
            ("halve",   1, "halve the distance to 3 — arrives from anywhere, q = 0.5"),
            ("newton",  1, "Newton on x³−2x−5 — arrives fast in range, q well under 1"),
            ("shift",   1, "add one forever — never arrives, and its gap never changes"),
            ("rotate",  2, "turn a third of a circle — a closed orbit, never arrives"),
            ("project", 2, "flatten onto the first axis — arrives in a single step"),
        ] {
            s.push_str(&format!("  {:<8} {} coord{}  {}\n",
                name, dim, if dim == 1 { " " } else { "s" }, what));
        }
        s.push_str("\nexample:  nesting shift 0   — gap 1.0 both times, so it never arrives\n");
        s
    }

    /// Read one caller-chosen point against one caller-chosen map.
    pub fn run(map: &str, args: &[&str]) -> String {
        let (f, dim) = match map_by_name(map) {
            Some(x) => x,
            None => return format!(
                "nesting: no map named '{}'. Available: {}\n\
                 usage: nesting <map> <x> [y]\n", map, MAPS.join(", ")),
        };
        let mut pt = [0.0f64; 2];
        for i in 0..dim {
            match args.get(i).and_then(|s| parse_f64(s)) {
                Some(v) => pt[i] = v,
                None => return format!(
                    "nesting: '{}' needs {} coordinate(s), and they must be numbers.\n\
                     usage: nesting {} {}\n",
                    map, dim, map, if dim == 1 { "<x>" } else { "<x> <y>" }),
            }
        }
        let label = if dim == 1 { format!("'{}' at {}", map, pt[0]) }
                    else { format!("'{}' at ({}, {})", map, pt[0], pt[1]) };
        describe_to(&label, f, &pt[..dim], target_by_name(map))
    }

    /// The reference sweep, when no point is named.
    pub fn sweep() -> String {
        let cases: [(&str, &str, [f64; 2], usize); 7] = [
            ("halve, starting on the target",  "halve",   [3.0, 0.0], 1),
            ("halve, starting close",          "halve",   [3.01, 0.0], 1),
            ("halve, starting far",            "halve",   [203.0, 0.0], 1),
            ("newton, in range",               "newton",  [2.0, 0.0], 1),
            ("shift by one, forever",          "shift",   [0.0, 0.0], 1),
            ("rotate by a third turn",         "rotate",  [1.0, 0.0], 2),
            ("project, off the range",         "project", [1.0, 5.0], 2),
        ];
        let mut s = String::from("The Observable That Completes the Split\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("One gap splits two ways. q = r2/r1 splits three, for the cost\n");
        s.push_str("of one more application. Name a point to read just that one:\n");
        s.push_str("  nesting <map> <x> [y]     maps: ");
        s.push_str(&MAPS.join(", "));
        s.push_str("\n\n");
        let mut held = 0;
        for (label, name, seed, dim) in cases {
            let (f, _) = map_by_name(name).unwrap();
            let r = describe(label, f, &seed[..dim]);
            if r.contains("HELD") { held += 1; }
            s.push_str(&r); s.push_str("\n");
        }
        s.push_str(&format!("{}/{} predictions held.\n", held, cases.len()));
        s
    }
}

/// Minimal float parse — core has no `f64::from_str` in this no_std build, and
/// pulling one in for a REPL argument would be a second numeric stack.
fn parse_f64(s: &str) -> Option<f64> {
    let s = s.trim();
    let (neg, body) = match s.strip_prefix('-') { Some(r) => (true, r), None => (false, s) };
    let mut whole = 0.0f64;
    let mut frac = 0.0f64;
    let mut scale = 0.1f64;
    let mut seen_dot = false;
    let mut any = false;
    for c in body.chars() {
        match c {
            '0'..='9' => {
                any = true;
                let d = (c as u8 - b'0') as f64;
                if seen_dot { frac += d * scale; scale *= 0.1; }
                else { whole = whole * 10.0 + d; }
            }
            '.' if !seen_dot => seen_dot = true,
            _ => return None,
        }
    }
    if !any { return None; }
    let v = whole + frac;
    Some(if neg { -v } else { v })
}
