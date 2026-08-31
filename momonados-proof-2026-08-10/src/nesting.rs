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
/// Budget for the confirming nest.
const MAXIT: u32 = 10_000;

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
pub fn confirm(f: fn(&[f64], &mut [f64]), a: &[f64]) -> (bool, u32) {
    let n = a.len();
    let mut x: Vec<f64> = a.to_vec();
    let mut nx = alloc::vec![0.0; n];
    for k in 0..MAXIT {
        f(&x, &mut nx);
        if dist(&nx, &x) < TOL { return (true, k); }
        x.copy_from_slice(&nx);
    }
    (false, MAXIT)
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

pub struct Nesting;

impl Nesting {
    pub fn run_all() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        // label, map, inner, dimension
        let cases: [(&str, fn(&[f64], &mut [f64]), [f64; 2], usize); 7] = [
            ("contraction, inner AT the fixed point",  contraction, [3.0, 0.0], 1),
            ("contraction, inner near it",             contraction, [3.01, 0.0], 1),
            ("contraction, inner far",                 contraction, [203.0, 0.0], 1),
            ("Newton, inner in the basin",             newton,      [2.0, 0.0], 1),
            ("translation x+1 (constant residual)",    translate,   [0.0, 0.0], 1),
            ("rotation by a third turn (limit cycle)", rotate3,     [1.0, 0.0], 2),
            ("projection, inner off the range",        project,     [1.0, 5.0], 2),
        ];

        for (label, f, seed, dim) in cases {
            let a = &seed[..dim];
            let rd = read(f, a);
            let (closed, steps) = confirm(f, a);

            let mut s = format!("{}\n", label);
            s.push_str(&format!("  r1 = {:.6e}\n", rd.r1));
            match rd.q {
                Some(q) => s.push_str(&format!("  r2 = {:.6e}   q = r2/r1 = {:.6}\n", rd.r2, q)),
                None    => s.push_str("  r2 = —            q = —  (residual already zero)\n"),
            }
            s.push_str(&format!("  one-step reading:  {}\n", rd.one_step));
            s.push_str(&format!("  two-step reading:  {}\n", rd.class.name()));
            s.push_str(&format!(
                "  nest confirms:     {}\n",
                if closed { format!("closed in {} step(s)", steps) }
                else { format!("open at the {}-step cap", steps) }
            ));

            // The prediction is scored against the nest, not assumed to match it.
            let agrees = match rd.class {
                Class::OneShot      => closed && steps == 0,
                Class::Attracted    => closed,
                Class::NeverArrives => !closed,
            };
            s.push_str(&format!("  prediction:        {}\n", if agrees { "HELD" } else { "MISSED" }));
            out.push(s);
        }
        out
    }

    pub fn report() -> String {
        let mut s = String::from("The Observable That Completes the Split\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("r1 alone splits two ways. q = r2/r1 splits three, at the cost\n");
        s.push_str("of one more application of the outer.\n\n");
        let rows = Self::run_all();
        let held = rows.iter().filter(|r| r.contains("HELD")).count();
        for r in &rows { s.push_str(r); s.push_str("\n"); }
        s.push_str(&format!("{}/{} predictions held.\n", held, rows.len()));
        s
    }
}
