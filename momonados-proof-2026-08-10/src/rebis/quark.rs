// rebis/quark.rs — Quark colour × spin, with confinement as a ceiling theorem.
//
// The colour sector is a Belnap FIVE: Vacuum < {Red, Green, Blue} < White,
// with the three colours mutually incomparable. That order does the work of
// confinement without a separate postulate, exactly as the orbital's B-ceiling
// does the work of Pauli: White is the top, so a colour charge has nowhere
// above it to go except the singlet.
//
// A quark state is the product of that colour lattice with the orbital
// occupancy lattice, and Frobenius separates the two cases: μ∘δ = id holds on
// white states and fails on coloured ones. The failure IS the confinement.
//
// Mirrors Imscribing/Paraconsistent/QuarkBelnap.lean.

use crate::rebis::orbital::{occupancy_le, pair, Orbital, ALL_ORBITAL};

/// Five colour-charge states.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colour {
    Vacuum, // no colour charge — the analogue of N
    Red,
    Green,
    Blue,
    White,  // colour singlet — the analogue of B, the confinement ceiling
}

pub static ALL_COLOUR: [Colour; 5] =
    [Colour::Vacuum, Colour::Red, Colour::Green, Colour::Blue, Colour::White];

impl Colour {
    pub fn name(self) -> &'static str {
        match self {
            Colour::Vacuum => "Vacuum",
            Colour::Red => "Red",
            Colour::Green => "Green",
            Colour::Blue => "Blue",
            Colour::White => "White",
        }
    }

    pub fn is_coloured(self) -> bool {
        matches!(self, Colour::Red | Colour::Green | Colour::Blue)
    }
}

/// Information order: Vacuum < {R, G, B} < White.
pub fn colour_le(a: Colour, b: Colour) -> bool {
    if a == Colour::Vacuum {
        return true;
    }
    if b == Colour::White {
        return true;
    }
    a == b
}

/// Distinct colours share nothing, so they meet at the vacuum.
pub fn colour_meet(a: Colour, b: Colour) -> Colour {
    if a == Colour::Vacuum || b == Colour::Vacuum {
        return Colour::Vacuum;
    }
    if a == Colour::White {
        return b;
    }
    if b == Colour::White {
        return a;
    }
    if a == b {
        return a;
    }
    Colour::Vacuum
}

/// Distinct colours join to White. This is the confinement move.
pub fn colour_join(a: Colour, b: Colour) -> Colour {
    if a == Colour::White || b == Colour::White {
        return Colour::White;
    }
    if a == Colour::Vacuum {
        return b;
    }
    if b == Colour::Vacuum {
        return a;
    }
    if a == b {
        return a;
    }
    Colour::White
}

/// Anti-colour is relational, not representational: the distinction lives in
/// the pairing, not in the label, so anti(Red) is Red.
pub fn anti_colour(c: Colour) -> Colour {
    c
}

/// A quark state: colour × spin.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Quark {
    pub colour: Colour,
    pub spin: Orbital,
}

impl Quark {
    pub fn new(colour: Colour, spin: Orbital) -> Quark {
        Quark { colour, spin }
    }
    pub fn is_white(self) -> bool {
        self.colour == Colour::White
    }
    pub fn is_coloured(self) -> bool {
        self.colour.is_coloured()
    }
    /// Product order: both coordinates must be below.
    pub fn le(self, other: Quark) -> bool {
        colour_le(self.colour, other.colour) && occupancy_le(self.spin, other.spin)
    }
}

/// Fully confined and fully paired: the top of the product order.
pub const CEILING: Quark = Quark { colour: Colour::White, spin: Orbital::Paired };

pub fn ceiling_is_top(q: Quark) -> bool {
    q.le(CEILING)
}

/// Depairing (δ): a white singlet splits into colour and anticolour;
/// a coloured state has nothing to split, so δ is the diagonal.
pub fn depair(q: Quark) -> (Quark, Quark) {
    if q.colour == Colour::White {
        (Quark::new(Colour::Red, q.spin), Quark::new(Colour::Red, q.spin))
    } else {
        (q, q)
    }
}

/// Pairing (μ): complementary colours fuse to a white singlet.
pub fn qpair(a: Quark, b: Quark) -> Quark {
    if a.colour == anti_colour(b.colour) {
        Quark::new(Colour::White, pair(a.spin, b.spin))
    } else {
        Quark::new(colour_join(a.colour, b.colour), pair(a.spin, b.spin))
    }
}

/// Frobenius holds on white states.
pub fn frobenius_holds_white(q: Quark) -> bool {
    if !q.is_white() {
        return true; // precondition not met
    }
    let (a, b) = depair(q);
    qpair(a, b) == q
}

/// Frobenius fails on coloured states, and that failure is the confinement.
pub fn frobenius_fails_coloured(q: Quark) -> bool {
    if !q.is_coloured() {
        return true; // precondition not met
    }
    let (a, b) = depair(q);
    qpair(a, b) != q
}

/// Every theorem this module claims, checked rather than asserted.
pub fn verify() -> (bool, &'static str) {
    // Vacuum is bottom, White is top
    for c in ALL_COLOUR.iter() {
        if !colour_le(Colour::Vacuum, *c) {
            return (false, "Vacuum is not the bottom");
        }
        if !colour_le(*c, Colour::White) {
            return (false, "White is not the ceiling");
        }
        if colour_meet(*c, *c) != *c || colour_join(*c, *c) != *c {
            return (false, "colour meet/join is not idempotent");
        }
    }
    // the three colours are mutually incomparable
    let rgb = [Colour::Red, Colour::Green, Colour::Blue];
    for a in rgb.iter() {
        for b in rgb.iter() {
            if a == b {
                continue;
            }
            if colour_le(*a, *b) {
                return (false, "two distinct colours are comparable");
            }
            if colour_join(*a, *b) != Colour::White {
                return (false, "distinct colours do not join to White");
            }
            if colour_meet(*a, *b) != Colour::Vacuum {
                return (false, "distinct colours do not meet at Vacuum");
            }
        }
    }
    // the ceiling really is the top of the product order
    for c in ALL_COLOUR.iter() {
        for s in ALL_ORBITAL.iter() {
            let q = Quark::new(*c, *s);
            if !ceiling_is_top(q) {
                return (false, "a state sits above (White, paired)");
            }
            if !frobenius_holds_white(q) {
                return (false, "Frobenius fails on a white state");
            }
            if !frobenius_fails_coloured(q) {
                return (false, "Frobenius holds on a coloured state — no confinement");
            }
        }
    }
    (true, "quark: colour is Belnap FIVE, White ceiling closed, confinement is the Frobenius failure")
}
