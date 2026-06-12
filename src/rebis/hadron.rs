// rebis/hadron.rs — Hadron/Quark/Orbital Belnap Analysis
//
// Port of rhr_p4rky/hadron_belnap.py, quark_belnap.py,
// orbital_belnap.py, exotic_hadron_belnap.py.
//
// The Belnap FOUR lattice provides a natural language for QCD:
//   B4::B (Both)  — confinement regime: both quark AND gluon active
//   B4::T (True)  — asymptotic freedom: quark behavior dominates
//   B4::F (False) — chiral symmetry broken: gluon behavior dominates
//   B4::N (Neither)— deconfinement: neither quark nor hadron description

use crate::belnap::B4;

// ── Quark flavors as Belnap-encoded quantum numbers ────────────

/// Quark flavor with Belnap-encoded properties.
#[derive(Copy, Clone, Debug)]
pub struct Quark {
    pub flavor: QuarkFlavor,
    pub color: B4,   // color charge in B4 encoding
    pub spin: B4,    // spin state
    pub mass_regime: B4, // mass relative to Λ_QCD
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum QuarkFlavor {
    Up = 0, Down = 1, Strange = 2, Charm = 3, Bottom = 4, Top = 5,
}

impl QuarkFlavor {
    pub fn name(self) -> &'static str {
        match self { Self::Up => "u", Self::Down => "d", Self::Strange => "s",
            Self::Charm => "c", Self::Bottom => "b", Self::Top => "t" }
    }

    /// Light quark (u,d,s) or heavy (c,b,t)?
    pub fn is_light(self) -> bool {
        matches!(self, Self::Up | Self::Down | Self::Strange)
    }

    /// Belnap encoding: light → {B,T,F,N} mapped to chiral properties.
    pub fn belnap_type(self) -> B4 {
        match self {
            Self::Up      => B4::T,  // definite: only up-type
            Self::Down    => B4::F,  // definite: only down-type
            Self::Strange => B4::N,  // neither up nor down, but light
            Self::Charm   => B4::B,  // both: up-type AND heavy
            Self::Bottom  => B4::F,  // down-type, heavy
            Self::Top     => B4::T,  // up-type, ultra-heavy
        }
    }
}

// ── Hadron types ────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HadronType {
    Meson,        // quark-antiquark pair
    Baryon,       // three quarks
    Tetraquark,   // exotic: qq qbar qbar
    Pentaquark,   // exotic: qqqq qbar
    Glueball,     // no valence quarks
    Hybrid,       // quark-antiquark + excited gluon
}

impl HadronType {
    pub fn name(self) -> &'static str {
        match self { Self::Meson => "meson", Self::Baryon => "baryon",
            Self::Tetraquark => "tetraquark", Self::Pentaquark => "pentaquark",
            Self::Glueball => "glueball", Self::Hybrid => "hybrid" }
    }
}

// ── Belnap state of a hadron ────────────────────────────────────

/// Hadronic state encoded in Belnap FOUR.
#[derive(Copy, Clone, Debug)]
pub struct HadronState {
    pub htype: HadronType,
    pub confinement: B4,  // B=confined, N=deconfined, T=asymptotic, F=chiral-broken
    pub parity: B4,       // parity eigenvalue
    pub charge: B4,       // charge conjugation
    pub frobenius_ok: bool,
}

impl HadronState {
    /// Compute the hadronic Belnap state from constituent quarks.
    pub fn from_quarks(quarks: &[Quark], htype: HadronType) -> Self {
        // Aggregate quark Belnap values via meet (confinement)
        let mut confinement = B4::B; // start at top
        for q in quarks {
            confinement = crate::belnap::meet(confinement, q.color);
        }

        // Parity: product of individual quark parities
        let mut parity = B4::T; // even start
        for q in quarks {
            parity = crate::belnap::meet(parity, q.spin);
        }

        // Charge conjugation: join of quark properties
        let mut charge = B4::N; // start at bottom
        for q in quarks {
            charge = crate::belnap::join(charge, q.flavor.belnap_type());
        }

        // Frobenius check: ffuse∘fsplit on the confinement state
        let frobenius_ok = check_hadron_frobenius(confinement, parity);

        Self { htype, confinement, parity, charge, frobenius_ok }
    }
}

/// Frobenius check on hadronic state: simulate fsplit/ffuse.
fn check_hadron_frobenius(confinement: B4, parity: B4) -> bool {
    // fsplit: (confinement, parity) → (c1,c2), (p1,p2)
    let (c1, c2) = crate::rebis::frob_filter::fsplit_b4(confinement);
    let (p1, p2) = crate::rebis::frob_filter::fsplit_b4(parity);
    // ffuse: recombine
    let c_result = crate::rebis::frob_filter::ffuse_b4(c1, c2);
    let p_result = crate::rebis::frob_filter::ffuse_b4(p1, p2);
    c_result == confinement && p_result == parity
}

// ── Orbital Belnap analysis ─────────────────────────────────────

/// Orbital angular momentum state in Belnap encoding.
#[derive(Copy, Clone, Debug)]
pub struct Orbital {
    pub n: u8,       // principal quantum number
    pub l: u8,       // orbital angular momentum
    pub m: i8,       // magnetic quantum number
    pub spin: B4,    // spin state
}

impl Orbital {
    pub fn new(n: u8, l: u8, m: i8) -> Self {
        let spin = if (n + l) % 2 == 0 { B4::T } else { B4::F };
        Self { n, l, m, spin }
    }

    /// Belnap encoding of the orbital.
    /// n=1 → T (ground), n→∞ → N (continuum), intermediate → B
    pub fn belnap_state(&self) -> B4 {
        match self.n {
            1 => B4::T,       // ground state: definite
            2..=3 => B4::B,   // low excited: both localized and delocalized
            4..=7 => B4::F,   // Rydberg: more delocalized
            _ => B4::N,        // near continuum: neither bound nor free
        }
    }

    /// Frobenius condition on orbital: does it satisfy closure?
    pub fn frobenius_check(&self) -> bool {
        let state = self.belnap_state();
        let (c1, c2) = crate::rebis::frob_filter::fsplit_b4(state);
        let result = crate::rebis::frob_filter::ffuse_b4(c1, c2);
        result == state
    }
}

// ── Exotic hadron identification ────────────────────────────────

/// Is a combination of quarks an exotic hadron?
pub fn classify_exotic(quarks: &[Quark]) -> Option<HadronType> {
    match quarks.len() {
        2 => Some(HadronType::Meson),
        3 => Some(HadronType::Baryon),
        4 => Some(HadronType::Tetraquark),
        5 => Some(HadronType::Pentaquark),
        0 => Some(HadronType::Glueball),
        _ => None,
    }
}

// ── Standard hadrons (static data) ──────────────────────────────

/// Proton: uud
pub fn proton_quarks() -> [Quark; 3] {
    [
        Quark { flavor: QuarkFlavor::Up, color: B4::T, spin: B4::T, mass_regime: B4::T },
        Quark { flavor: QuarkFlavor::Up, color: B4::F, spin: B4::F, mass_regime: B4::T },
        Quark { flavor: QuarkFlavor::Down, color: B4::N, spin: B4::T, mass_regime: B4::T },
    ]
}

/// Neutron: udd
pub fn neutron_quarks() -> [Quark; 3] {
    [
        Quark { flavor: QuarkFlavor::Up, color: B4::T, spin: B4::T, mass_regime: B4::T },
        Quark { flavor: QuarkFlavor::Down, color: B4::F, spin: B4::F, mass_regime: B4::T },
        Quark { flavor: QuarkFlavor::Down, color: B4::N, spin: B4::T, mass_regime: B4::T },
    ]
}

/// Pion+: u dbar
pub fn pion_plus_quarks() -> [Quark; 2] {
    [
        Quark { flavor: QuarkFlavor::Up, color: B4::T, spin: B4::T, mass_regime: B4::T },
        Quark { flavor: QuarkFlavor::Down, color: B4::T, spin: B4::F, mass_regime: B4::T },
    ]
}
