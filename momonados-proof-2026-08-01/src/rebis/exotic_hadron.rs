//! exotic_hadron.rs — Exotic Hadron Frobenius Verification
//! Port of rhr_p4rky/exotic_hadron_belnap.py
//!
//! Extends hadron.rs with:
//!   - Gluon color states (8 gluon colors)
//!   - Glueball with depair/pair Frobenius cycle
//!   - Tetraquark depair→2-meson→pair verification
//!   - Pentaquark structure
//!
//! All exotic hadrons satisfy μ∘δ=id via depair∘pair = id.

use alloc::collections::BTreeSet;
use crate::belnap::B4;

// ── Gluon color states ─────────────────────────────────────────────────

/// The 8 gluon color-anticolor combinations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GluonColor {
    RG,     // red-antigreen
    RB,     // red-antiblue
    GR,     // green-antired
    GB,     // green-antiblue
    BR,     // blue-antired
    BG,     // blue-antigreen
    RRDD,   // (rr̄ - dd̄)/√2  — diagonal combination
    BBRD,   // (bb̄ - rr̄)/√2  — orthogonal diagonal
}

impl GluonColor {
    pub const ALL: [GluonColor; 8] = [
        GluonColor::RG, GluonColor::RB,
        GluonColor::GR, GluonColor::GB,
        GluonColor::BR, GluonColor::BG,
        GluonColor::RRDD, GluonColor::BBRD,
    ];

    pub fn name(self) -> &'static str {
        match self {
            GluonColor::RG => "rg", GluonColor::RB => "rb",
            GluonColor::GR => "gr", GluonColor::GB => "gb",
            GluonColor::BR => "br", GluonColor::BG => "bg",
            GluonColor::RRDD => "rrdd", GluonColor::BBRD => "bbrd",
        }
    }

    /// Is this a diagonal (color-neutral) gluon?
    pub fn is_diagonal(self) -> bool {
        matches!(self, GluonColor::RRDD | GluonColor::BBRD)
    }
}

// ── Glueball ───────────────────────────────────────────────────────────

/// A glueball: bound state of gluons with no valence quarks.
/// Minimum 2 gluons; Frobenius-verified via depair∘pair = id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glueball {
    pub gluons: BTreeSet<GluonColor>,
}

impl Glueball {
    /// Create a new glueball. Must have at least 2 gluons.
    pub fn new(gluons: BTreeSet<GluonColor>) -> Option<Self> {
        if gluons.len() < 2 { return None; }
        Some(Glueball { gluons })
    }

    /// Create from a slice of gluon colors.
    pub fn from_slice(gluons: &[GluonColor]) -> Option<Self> {
        if gluons.len() < 2 { return None; }
        Some(Glueball { gluons: gluons.iter().copied().collect() })
    }

    /// Depair (δ): split the glueball into two copies of itself.
    /// In the Frobenius cycle, δ duplicates the state.
    pub fn depair(&self) -> (Glueball, Glueball) {
        (self.clone(), self.clone())
    }

    /// Pair (μ): merge two glueballs by union of their gluon sets.
    /// Returns None if the combined set has fewer than 2 gluons.
    pub fn pair(g1: &Glueball, g2: &Glueball) -> Option<Glueball> {
        let merged: BTreeSet<GluonColor> = g1.gluons.union(&g2.gluons).copied().collect();
        if merged.len() < 2 { None }
        else { Some(Glueball { gluons: merged }) }
    }

    /// Frobenius verification: depair∘pair should recover the original.
    pub fn verify_frobenius(&self) -> bool {
        let (d1, d2) = self.depair();
        match Glueball::pair(&d1, &d2) {
            Some(result) => result == *self,
            None => false,
        }
    }

    /// Belnap encoding of the glueball state.
    pub fn belnap_state(&self) -> B4 {
        let has_diag = self.gluons.iter().any(|g| g.is_diagonal());
        let has_charged = self.gluons.iter().any(|g| !g.is_diagonal());
        match (has_diag, has_charged) {
            (true, true) => B4::B,    // Both: mixed glueball
            (true, false) => B4::T,   // True: pure diagonal
            (false, true) => B4::F,   // False: pure charged
            (false, false) => B4::N,  // Neither (shouldn't happen with ≥2)
        }
    }
}

// ── Quark color (simplified for exotic hadron module) ──────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QColor { Red, Green, Blue, AntiRed, AntiGreen, AntiBlue, White }

impl QColor {
    pub fn anti(self) -> Self {
        match self {
            QColor::Red => QColor::AntiRed,
            QColor::Green => QColor::AntiGreen,
            QColor::Blue => QColor::AntiBlue,
            QColor::AntiRed => QColor::Red,
            QColor::AntiGreen => QColor::Green,
            QColor::AntiBlue => QColor::Blue,
            QColor::White => QColor::White,
        }
    }

    /// Join two colors: returns the resulting color or White if color-neutral.
    pub fn join(a: QColor, b: QColor) -> QColor {
        if a == b.anti() { return QColor::White; }
        if a == QColor::White { return b; }
        if b == QColor::White { return a; }
        // Non-matching non-white: could be a mixed state
        if a == QColor::Red && b == QColor::Blue { return QColor::AntiGreen; }
        if a == QColor::Red && b == QColor::Green { return QColor::AntiBlue; }
        if a == QColor::Green && b == QColor::Blue { return QColor::AntiRed; }
        if b == QColor::Red && a == QColor::Blue { return QColor::AntiGreen; }
        if b == QColor::Red && a == QColor::Green { return QColor::AntiBlue; }
        if b == QColor::Green && a == QColor::Blue { return QColor::AntiRed; }
        QColor::White
    }

    /// Join a slice of colors, returning the final color.
    pub fn join_all(colors: &[QColor]) -> QColor {
        let mut result = QColor::White;
        for &c in colors {
            result = QColor::join(result, c);
        }
        result
    }
}

// ── Tetraquark ─────────────────────────────────────────────────────────

/// A tetraquark: q₁ q₂ q̄₃ q̄₄
/// Frobenius-verified: depair → two mesons, pair → original tetraquark.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tetraquark {
    pub q1_color: QColor,
    pub q2_color: QColor,
    pub aq1_color: QColor,  // anti-quark 1
    pub aq2_color: QColor,  // anti-quark 2
}

impl Tetraquark {
    /// Create a tetraquark. Must be overall color-white.
    pub fn new(q1: QColor, q2: QColor, aq1: QColor, aq2: QColor) -> Option<Self> {
        // aq1 must be anti-color of something compatible
        let total = QColor::join_all(&[q1, q2, aq1, aq2]);
        if total != QColor::White { return None; }
        Some(Tetraquark { q1_color: q1, q2_color: q2, aq1_color: aq1, aq2_color: aq2 })
    }

    /// Depair (δ): split tetraquark into two (quark, antiquark) meson pairs.
    pub fn depair(&self) -> ((QColor, QColor), (QColor, QColor)) {
        ((self.q1_color, self.aq1_color), (self.q2_color, self.aq2_color))
    }

    /// Pair (μ): recombine two meson pairs into a tetraquark.
    pub fn pair(p1: (QColor, QColor), p2: (QColor, QColor)) -> Option<Tetraquark> {
        let (q1, aq1) = p1;
        let (q2, aq2) = p2;
        // Check each pair is color-neutral
        if QColor::join_all(&[q1, aq1]) != QColor::White { return None; }
        if QColor::join_all(&[q2, aq2]) != QColor::White { return None; }
        Tetraquark::new(q1, q2, aq1, aq2)
    }

    /// Frobenius verification: depair∘pair = id.
    pub fn verify_frobenius(&self) -> bool {
        let (p1, p2) = self.depair();
        match Tetraquark::pair(p1, p2) {
            Some(result) => result == *self,
            None => false,
        }
    }
}

// ── Pentaquark ─────────────────────────────────────────────────────────

/// A pentaquark: q₁ q₂ q₃ q₄ q̄₁
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pentaquark {
    pub quarks: [QColor; 4],
    pub antiquark: QColor,
}

impl Pentaquark {
    /// Create a pentaquark. Must be overall color-white.
    pub fn new(quarks: [QColor; 4], antiquark: QColor) -> Option<Self> {
        let all = alloc::vec![quarks[0], quarks[1], quarks[2], quarks[3], antiquark];
        let total = QColor::join_all(&all);
        if total != QColor::White { return None; }
        Some(Pentaquark { quarks, antiquark })
    }

    /// Belnap encoding of pentaquark stability.
    pub fn belnap_state(&self) -> B4 {
        // True = narrow resonance, False = broad, Both = molecular,
        // Neither = unbound
        let n_heavy: usize = self.quarks.iter()
            .filter(|&&c| matches!(c, QColor::Red | QColor::AntiRed))
            .count();
        let has_heavy_anti = matches!(self.antiquark, QColor::AntiRed);
        match (n_heavy, has_heavy_anti) {
            (4, true) => B4::T,   // Tightly bound: ccccc̄
            (3..=4, _) => B4::B,  // Molecular state
            (0..=2, false) => B4::F, // Broad resonance
            _ => B4::N,           // Unbound
        }
    }
}
