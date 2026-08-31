// rebis/orbital.rs — Electron orbital occupancy as a Belnap FOUR bilattice.
//
// The four occupancy states of an atomic orbital ARE Belnap FOUR: empty is N,
// a single up electron is T, a single down electron is F, and the filled pair
// is B. Pauli exclusion is then not a separate postulate but the statement
// that B is the ceiling of the information order — nothing sits strictly above
// the paired state, so a third electron has nowhere to go.
//
// Mirrors Imscribing/Paraconsistent/OrbitalBelnap.lean.

use crate::belnap::B4;

/// The four occupancy states of an orbital.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Orbital {
    Empty,     // no electrons     → N
    SpinUp,    // one electron ↑   → T
    SpinDown,  // one electron ↓   → F
    Paired,    // two electrons ↑↓ → B
}

pub static ALL_ORBITAL: [Orbital; 4] =
    [Orbital::Empty, Orbital::SpinUp, Orbital::SpinDown, Orbital::Paired];

impl Orbital {
    pub fn name(self) -> &'static str {
        match self {
            Orbital::Empty => "empty",
            Orbital::SpinUp => "spinUp",
            Orbital::SpinDown => "spinDown",
            Orbital::Paired => "paired",
        }
    }

    /// The occupancy states are Belnap FOUR, not merely like it.
    pub fn to_b4(self) -> B4 {
        match self {
            Orbital::Empty => B4::N,
            Orbital::SpinUp => B4::T,
            Orbital::SpinDown => B4::F,
            Orbital::Paired => B4::B,
        }
    }

    pub fn from_b4(b: B4) -> Orbital {
        match b {
            B4::N => Orbital::Empty,
            B4::T => Orbital::SpinUp,
            B4::F => Orbital::SpinDown,
            B4::B => Orbital::Paired,
        }
    }
}

/// Information order: how much occupancy is known. Empty is bottom, paired top.
pub fn occupancy_le(a: Orbital, b: Orbital) -> bool {
    use Orbital::*;
    match (a, b) {
        (Empty, _) => true,
        _ if a == b => true,
        (SpinUp, Paired) => true,
        (SpinDown, Paired) => true,
        _ => false,
    }
}

/// Paired is the maximum of the information order.
pub fn paired_is_top(s: Orbital) -> bool {
    occupancy_le(s, Orbital::Paired)
}

/// Pauli exclusion as a ceiling theorem: nothing lies strictly above paired.
/// If paired ≤ s then s is paired, so a third electron cannot be added.
pub fn pauli_exclusion(s: Orbital) -> bool {
    if !occupancy_le(Orbital::Paired, s) {
        return true; // vacuous
    }
    s == Orbital::Paired
}

/// Depairing (δ): resolve an orbital into its two spin components.
pub fn depair(s: Orbital) -> (Orbital, Orbital) {
    use Orbital::*;
    match s {
        Paired => (SpinUp, SpinDown),
        SpinUp => (SpinUp, Empty),
        SpinDown => (Empty, SpinDown),
        Empty => (Empty, Empty),
    }
}

/// Pairing (μ): combine two spin components. Opposite spins fill the orbital;
/// an already-filled state absorbs; same spin is Pauli-blocked and keeps one.
pub fn pair(a: Orbital, b: Orbital) -> Orbital {
    use Orbital::*;
    if (a == SpinUp && b == SpinDown) || (a == SpinDown && b == SpinUp) {
        return Paired;
    }
    if a == Paired || b == Paired {
        return Paired;
    }
    if a == b {
        return a;
    }
    if a == Empty {
        return b;
    }
    if b == Empty {
        return a;
    }
    Empty
}

/// Frobenius on the orbital: splitting and refusing recovers the state.
pub fn pair_depair_id(s: Orbital) -> bool {
    let (a, b) = depair(s);
    pair(a, b) == s
}

/// Meet in the information order.
pub fn occupancy_meet(a: Orbital, b: Orbital) -> Orbital {
    Orbital::from_b4(a.to_b4().meet(b.to_b4()))
}

/// Join in the information order.
pub fn occupancy_join(a: Orbital, b: Orbital) -> Orbital {
    Orbital::from_b4(a.to_b4().join(b.to_b4()))
}

/// Every theorem this module claims, checked rather than asserted.
pub fn verify() -> (bool, &'static str) {
    // the bijection with Belnap FOUR
    for s in ALL_ORBITAL.iter() {
        if Orbital::from_b4(s.to_b4()) != *s {
            return (false, "orbital ↔ B4 is not a bijection");
        }
    }
    // empty is bottom, paired is top
    for s in ALL_ORBITAL.iter() {
        if !occupancy_le(Orbital::Empty, *s) {
            return (false, "empty is not the bottom");
        }
        if !paired_is_top(*s) {
            return (false, "paired is not the top");
        }
        if !pauli_exclusion(*s) {
            return (false, "Pauli ceiling is open");
        }
        if !pair_depair_id(*s) {
            return (false, "Frobenius fails on an orbital state");
        }
    }
    // the two singly-occupied states are incomparable
    if occupancy_le(Orbital::SpinUp, Orbital::SpinDown)
        || occupancy_le(Orbital::SpinDown, Orbital::SpinUp)
    {
        return (false, "spinUp and spinDown are not incomparable");
    }
    // meet and join agree with the Belnap lattice they are
    for a in ALL_ORBITAL.iter() {
        for b in ALL_ORBITAL.iter() {
            if occupancy_meet(*a, *b).to_b4() != a.to_b4().meet(b.to_b4()) {
                return (false, "meet disagrees with B4");
            }
            if occupancy_join(*a, *b).to_b4() != a.to_b4().join(b.to_b4()) {
                return (false, "join disagrees with B4");
            }
        }
    }
    (true, "orbital: Belnap FOUR, Pauli ceiling closed, Frobenius holds")
}
