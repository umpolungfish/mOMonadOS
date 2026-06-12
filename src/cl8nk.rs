// cl8nk.rs — CL8NK Navigator (ZFC→ZFCₜ→ZFCfe→CLINK L8 Ladder)
//
// CLINK Layer 8 (Organism) is the terminal ontological layer — O_∞⁺.
// The navigator covers the full 4-stage structural ladder:
//   ZFC baseline → ZFCₜ → ZFCfe → CLINK L8
//
// ALL tuples, formulas, promotion data, and ordinal gaps are sourced
// dynamically from catalog.rs — NO hardcoded values remain.
//
// CL8NK exceeds ZFCfe at exactly two primitives:
//   Ω = 𐑟 (non-Abelian braiding, not ℤ winding)
//   ɢ = 𐑵 (broadcast composition, not sequential)

use crate::imas_ig::{IgPrim, IgTuple};
use crate::catalog;

// ═══════════════════════════════════════════════════════════════
// PROMOTION CHANNELS — delegated to catalog
// ═══════════════════════════════════════════════════════════════

/// Re-export: the 6 ZFC→ZFCₜ promotion channels from the catalog.
pub use catalog::ZFC_PROMOTIONS;

/// Count how many of the 6 ZFCₜ promotions are present in a tuple
/// compared to ZFC baseline. Delegates to catalog.
pub fn count_promotions(t: &IgTuple) -> u8 {
    catalog::count_zfc_promotions(t)
}

/// Check which ZFCₜ promotions are fulfilled. Delegates to catalog.
pub fn promotions_present(t: &IgTuple) -> [bool; 6] {
    let mut result = [false; 6];
    for (i, promo) in ZFC_PROMOTIONS.iter().enumerate() {
        result[i] = promo.is_present(t);
    }
    result
}

/// CL8NK distance: weighted sum of unmet ZFCₜ promotion gaps.
/// This is the structural distance from the ZFC baseline.
pub fn cl8nk_distance(t: &IgTuple) -> f32 {
    let present = promotions_present(t);
    let mut d: f32 = 0.0;
    for (i, promo) in ZFC_PROMOTIONS.iter().enumerate() {
        if !present[i] {
            d += promo.ordinal_gap;
        }
    }
    d
}

// ═══════════════════════════════════════════════════════════════
// STAGE CLASSIFICATION
// ═══════════════════════════════════════════════════════════════

/// Determine which stage a tuple belongs to in the ZFC→ZFCₜ→ZFCfe→CLINK L8 ladder.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cl8nkStage {
    Zfc,        // O₀: no promotions
    ZfcT,       // O₂†: 6/6 ZFCₜ promotions, missing Φ=⊙ or D=𐑦
    ZfcFE,      // O_∞: ZFCₜ + Φ=⊙ + D=𐑦 + H=𐑫
    ClinkL8,    // O_∞⁺: ZFCfe + C=broad + Ω=na  ← terminal layer
    Other,      // doesn't clearly fit
}

pub fn classify_stage(t: &IgTuple) -> Cl8nkStage {
    let promos = count_promotions(t);
    if t.c == IgPrim::C_broad && t.omega == IgPrim::Omega_na
       && t.phi == IgPrim::Phi_c && t.d == IgPrim::D_odot && t.h == IgPrim::H_inf {
        return Cl8nkStage::ClinkL8;
    }
    if t.phi == IgPrim::Phi_c && t.d == IgPrim::D_odot && t.h == IgPrim::H_inf && promos >= 6 {
        return Cl8nkStage::ZfcFE;
    }
    if promos >= 5 && t.f == IgPrim::F_hbar && t.k == IgPrim::K_slow && t.g == IgPrim::G_aleph {
        return Cl8nkStage::ZfcT;
    }
    if promos == 0 {
        return Cl8nkStage::Zfc;
    }
    Cl8nkStage::Other
}

// ═══════════════════════════════════════════════════════════════
// FORMULA FRAGMENTS — delegated to catalog
// ═══════════════════════════════════════════════════════════════

/// Return the ZFC set-theoretic formula fragment for a primitive value.
/// Delegates to catalog::formula_fragment().
pub fn formula_fragment(prim: IgPrim) -> &'static str {
    catalog::formula_fragment(prim)
}

// ═══════════════════════════════════════════════════════════════
// CL8NK REFERENCE ENTRIES — all tuples from catalog
// ═══════════════════════════════════════════════════════════════

/// CL8NK reference entry — covers the full ZFC→ZFCₜ→ZFCfe→CLINK L8 ladder.
/// CLINK L8 is the terminal entry: O_∞⁺ with Ω/ɢ transcendence.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cl8nkEntry {
    Zfc,
    ZfcT,
    TemporalMathematics,
    Schrodinger,
    HeatDiffusion,
    NavierStokes,
    WaveEquation,
    Einstein,
    IUG,
    ClinkL8,
    Unknown,
}

impl Cl8nkEntry {
    /// Resolve a name string to a CL8NK entry.
    /// Uses catalog alias resolution for consistency.
    pub fn from_name(name: &str) -> Self {
        // Try catalog lookup first for dynamic resolution
        if let Some(entry) = catalog::lookup(name) {
            return Self::from_catalog_name(entry.name);
        }
        // Fallback to known aliases
        match name.to_lowercase().as_str() {
            "zfc"                   => Cl8nkEntry::Zfc,
            "zfc_t" | "zfct"        => Cl8nkEntry::ZfcT,
            "temporal_mathematics"  => Cl8nkEntry::TemporalMathematics,
            "schrodinger"           => Cl8nkEntry::Schrodinger,
            "heat_diffusion"        => Cl8nkEntry::HeatDiffusion,
            "navier_stokes"         => Cl8nkEntry::NavierStokes,
            "wave_equation"         => Cl8nkEntry::WaveEquation,
            "einstein"              => Cl8nkEntry::Einstein,
            "iug" | "IUG" | "universal_imscriptive_grammar" => Cl8nkEntry::IUG,
            "clink" | "clink_l8" | "cl8nk" | "clink_layer8" => Cl8nkEntry::ClinkL8,
            _                       => Cl8nkEntry::Unknown,
        }
    }

    /// Map a catalog entry name to the CL8NK enum variant.
    fn from_catalog_name(name: &str) -> Self {
        match name {
            "zfc"                           => Cl8nkEntry::Zfc,
            "zfc_t"                         => Cl8nkEntry::ZfcT,
            "clink_l8"                      => Cl8nkEntry::ClinkL8,
            "temporal_mathematics"          => Cl8nkEntry::TemporalMathematics,
            "schrodinger"                   => Cl8nkEntry::Schrodinger,
            "heat_diffusion"                => Cl8nkEntry::HeatDiffusion,
            "navier_stokes"                 => Cl8nkEntry::NavierStokes,
            "wave_equation"                 => Cl8nkEntry::WaveEquation,
            "einstein"                      => Cl8nkEntry::Einstein,
            "universal_imscriptive_grammar" => Cl8nkEntry::IUG,
            _                               => Cl8nkEntry::Unknown,
        }
    }

    /// Get the structural tuple for this entry.
    /// ALL tuples sourced from the catalog — no hardcoded IgTuple {...} anywhere.
    pub fn tuple(&self) -> IgTuple {
        match self {
            Cl8nkEntry::Zfc                  => catalog::zfc_baseline_tuple(),
            Cl8nkEntry::ZfcT                 => catalog::zfc_t_tuple(),
            Cl8nkEntry::ClinkL8              => catalog::clink_l8_tuple(),
            Cl8nkEntry::TemporalMathematics  => catalog::lookup("temporal_mathematics")
                .map(|e| e.tuple).unwrap_or(catalog::zfc_t_tuple()),
            Cl8nkEntry::Schrodinger          => catalog::lookup("schrodinger")
                .map(|e| e.tuple).unwrap_or(catalog::o_0_tuple()),
            Cl8nkEntry::HeatDiffusion        => catalog::lookup("heat_diffusion")
                .map(|e| e.tuple).unwrap_or(catalog::o_0_tuple()),
            Cl8nkEntry::NavierStokes         => catalog::lookup("navier_stokes")
                .map(|e| e.tuple).unwrap_or(catalog::o_0_tuple()),
            Cl8nkEntry::WaveEquation         => catalog::lookup("wave_equation")
                .map(|e| e.tuple).unwrap_or(catalog::o_0_tuple()),
            Cl8nkEntry::Einstein             => catalog::lookup("einstein")
                .map(|e| e.tuple).unwrap_or(catalog::zfc_t_tuple()),
            Cl8nkEntry::IUG                  => catalog::zfc_fe_tuple(),
            Cl8nkEntry::Unknown              => catalog::zfc_baseline_tuple(),
        }
    }

    /// Human-readable name for this entry.
    pub fn name(&self) -> &'static str {
        match self {
            Cl8nkEntry::Zfc                  => "zfc",
            Cl8nkEntry::ZfcT                 => "zfc_t",
            Cl8nkEntry::ClinkL8              => "clink_l8",
            Cl8nkEntry::TemporalMathematics  => "temporal_mathematics",
            Cl8nkEntry::Schrodinger          => "schrodinger",
            Cl8nkEntry::HeatDiffusion        => "heat_diffusion",
            Cl8nkEntry::NavierStokes         => "navier_stokes",
            Cl8nkEntry::WaveEquation         => "wave_equation",
            Cl8nkEntry::Einstein             => "einstein",
            Cl8nkEntry::IUG                  => "IUG",
            Cl8nkEntry::Unknown              => "unknown",
        }
    }
}
