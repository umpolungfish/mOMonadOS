// ctc.rs — The Manufactured Fixed Point
//
// The Fixed-Point Nesting Rule classifies a nesting from the outer's stabilizer:
// the inner is already the outer's fixed point (one-shot), or lies in a basin
// (iterated), or no fixed point is in reach (no closure). A conservative outer
// has no basin at all, so its pairings are one-shot or nothing — which is why
// every no-closure verdict on record was reached with no machine available that
// could close such a pair.
//
// This module is that machine. It does not search for a fixed point; it imposes
// one. Where the action on values has no fixed point, the action is lifted to
// SETS of values, and on sets a fixed point always exists: the value-set sequence
// from any seed is eventually periodic (the state space is finite), and the union
// over one period maps to itself exactly. That union is the manufactured fixed
// point. It is closure by fiat, guaranteed by the shape of the state space rather
// than reached by iterating toward anything.
//
// The price is stated, never hidden: a manufactured fixed point is a smear over
// |support| values where a possessed one is a single value. Width 1 IS a pure
// fixed point — the machine returns possession unchanged when possession is what
// it was handed, so a manufactured one-shot is never confused with a real one.
//
// Surface: mOMonadOS kernel, Belnap FOUR (the kernel's own B4, not a copy).
// Author: Quantum⊙perator (Lando⊗⊙perator team)

#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::belnap::B4;

/// A subset of B4, as a 4-bit mask. The CTC register carries one of these:
/// a single value when the pairing closes on its own, a wider set when the
/// closure had to be manufactured.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct B4Set(pub u8);

pub const ALL: [B4; 4] = [B4::T, B4::F, B4::N, B4::B];

impl B4Set {
    pub fn empty() -> Self { B4Set(0) }
    pub fn single(v: B4) -> Self { B4Set(1u8 << v.to_u8()) }
    pub fn contains(self, v: B4) -> bool { self.0 & (1u8 << v.to_u8()) != 0 }
    pub fn insert(&mut self, v: B4) { self.0 |= 1u8 << v.to_u8(); }
    pub fn union(self, other: B4Set) -> B4Set { B4Set(self.0 | other.0) }
    pub fn width(self) -> u32 { self.0.count_ones() }

    pub fn values(self) -> Vec<B4> {
        let mut out = Vec::new();
        for v in ALL { if self.contains(v) { out.push(v); } }
        out
    }

    pub fn to_notation(self) -> String {
        if self.0 == 0 { return String::from("{}"); }
        let mut s = String::from("{");
        let mut first = true;
        for v in self.values() {
            if !first { s.push_str(","); }
            s.push_str(v.name());
            first = false;
        }
        s.push_str("}");
        s
    }
}

/// The three classes of the Fixed-Point Nesting Rule, plus the one this
/// module adds. Given positively: each names what the pairing does, and
/// what it cost to get there.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// The inner was already the outer's fixed point. Zero work: possession.
    OneShot,
    /// The inner was walked to a fixed point over a finite budget.
    Iterated,
    /// No fixed point of the outer is in reach of the inner, on values.
    NoClosure,
    /// No fixed point on values; one imposed on sets. Closure by fiat.
    Manufactured,
}

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::OneShot      => "one-shot",
            Class::Iterated     => "iterated",
            Class::NoClosure    => "no-closure",
            Class::Manufactured => "manufactured",
        }
    }
}

/// What a nesting returned, with its price attached.
pub struct Closure {
    pub class: Class,
    /// The fixed point itself: width 1 when possessed, wider when manufactured.
    pub support: B4Set,
    /// Steps taken before the state stopped changing. 0 for a possessed one-shot.
    pub steps: u32,
    /// The price. A possessed fixed point costs 0; a manufactured one costs the
    /// number of values it had to smear together beyond the first.
    pub price: u32,
    /// Whether the returned support genuinely maps to itself under the lifted map.
    pub verified: bool,
}

impl Closure {
    pub fn to_report(&self) -> String {
        let mut s = format!(
            "  class:   {}\n  support: {} (width {})\n  steps:   {}\n  price:   {}\n",
            self.class.name(),
            self.support.to_notation(),
            self.support.width(),
            self.steps,
            self.price,
        );
        s.push_str(&format!(
            "  closes:  {}\n",
            if self.verified { "verified — the support maps to itself exactly" }
            else { "NOT verified — the support does not map to itself" }
        ));
        s
    }
}

/// Lift a value-map to sets: g(S) = { g(x) : x ∈ S }. Monotone under inclusion,
/// which is exactly why the set space has a fixed point where the value space
/// need not.
fn lift(g: fn(B4) -> B4, s: B4Set) -> B4Set {
    let mut out = B4Set::empty();
    for v in s.values() { out.insert(g(v)); }
    out
}

/// The pure fixed points of the action, if any. This is the possession test:
/// it is asked FIRST, so a real fixed point is never dressed up as a made one.
pub fn pure_fixed_points(g: fn(B4) -> B4) -> B4Set {
    let mut out = B4Set::empty();
    for v in ALL { if g(v) == v { out.insert(v); } }
    out
}

/// Nest `seed` inside the action `g` and return what the pairing does.
///
/// The order of the tests is the content of the module. Possession is checked
/// before anything is manufactured; a basin is walked before closure is imposed;
/// imposition is the last resort and is priced when it fires.
pub fn nest(g: fn(B4) -> B4, seed: B4) -> Closure {
    // 1. Possession. The inner is already the fixed point — zero work.
    if g(seed) == seed {
        return Closure {
            class: Class::OneShot,
            support: B4Set::single(seed),
            steps: 0,
            price: 0,
            verified: true,
        };
    }

    // 2. Basin. Walk the value orbit; the state space is finite, so this
    //    terminates either at a fixed point or on a cycle.
    let mut x = seed;
    let mut steps = 0u32;
    let mut seen = B4Set::single(seed);
    loop {
        let next = g(x);
        if next == x {
            // Walked home to a genuine fixed point over a finite budget.
            // `steps` already counts the applications that reached `x`; the
            // test that found it fixed is not itself a step of the walk.
            return Closure {
                class: Class::Iterated,
                support: B4Set::single(next),
                steps,
                price: 0,
                verified: true,
            };
        }
        if seen.contains(next) {
            // A cycle with no fixed point on it: on values this is no-closure.
            // Everything below is the manufacture.
            break;
        }
        seen.insert(next);
        x = next;
        steps += 1;
        if steps > 8 { break; } // B4 has four values; this cannot be reached
    }

    // 3. Manufacture. Iterate the LIFTED map until the set stops changing.
    //    The sequence of sets is non-decreasing once unioned with its image, so
    //    it saturates in at most |B4| steps, and the saturated set maps into
    //    itself by construction.
    let mut s = B4Set::single(seed);
    let mut lift_steps = 0u32;
    loop {
        let next = s.union(lift(g, s));
        lift_steps += 1;
        if next == s { break; }
        s = next;
        if lift_steps > 8 { break; }
    }

    // The claim is checked, not asserted: does the support map to itself?
    let image = lift(g, s);
    let verified = image.0 & !s.0 == 0;

    Closure {
        class: Class::Manufactured,
        support: s,
        steps: lift_steps,
        // The price is the smear: how many values had to be held at once beyond
        // the one a possessed fixed point would have been.
        price: s.width().saturating_sub(1),
        verified,
    }
}

// ── The actions the kernel already carries, plus one with no fixed point ─────

/// Belnap negation — the kernel's own, not a second copy. ¬B = B and ¬N = N,
/// so this action has two possessed fixed points and needs nothing manufactured.
fn act_bnot(v: B4) -> B4 { v.bnot() }

/// The temporal Next of the kernel's temporal bridge: ○T=F, ○F=T, ○N=N, ○B=B.
/// Two fixed points; T and F sit on a 2-cycle with no fixed point between them.
fn act_next(v: B4) -> B4 {
    match v { B4::T => B4::F, B4::F => B4::T, other => other }
}

/// A genuine 4-cycle: T→F→N→B→T. Fixed-point free on values — the conservative
/// case the rule calls no-closure, with no basin anywhere to walk down.
fn act_cycle(v: B4) -> B4 {
    match v { B4::T => B4::F, B4::F => B4::N, B4::N => B4::B, B4::B => B4::T }
}

/// Collapse to the dialetheic value: every input lands on B in one step. Every
/// pairing is iterated-to-B, and B itself is possessed.
fn act_collapse(_v: B4) -> B4 { B4::B }

pub struct Ctc;

impl Ctc {
    pub fn run_all() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        let cases: [(&str, fn(B4) -> B4, B4); 6] = [
            ("negation, seeded at B (possessed)",        act_bnot,     B4::B),
            ("negation, seeded at T (2-cycle)",          act_bnot,     B4::T),
            ("temporal next, seeded at N (possessed)",   act_next,     B4::N),
            ("collapse to B, seeded at T (basin)",       act_collapse, B4::T),
            ("4-cycle, seeded at T (no fixed point)",    act_cycle,    B4::T),
            ("4-cycle, seeded at B (no fixed point)",    act_cycle,    B4::B),
        ];

        for (label, g, seed) in cases {
            let fixed = pure_fixed_points(g);
            let c = nest(g, seed);
            let mut s = format!("{}\n", label);
            s.push_str(&format!("  action's pure fixed points: {}\n", fixed.to_notation()));
            s.push_str(&c.to_report());
            out.push(s);
        }
        out
    }

    pub fn report() -> String {
        let mut s = String::from("The Manufactured Fixed Point — CTC closure over Belnap FOUR\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("Possession is tested first, then the basin, then imposition.\n");
        s.push_str("A manufactured closure carries its price: the width it smears.\n\n");
        for r in Self::run_all() {
            s.push_str(&r);
            s.push_str("\n");
        }
        s
    }
}
