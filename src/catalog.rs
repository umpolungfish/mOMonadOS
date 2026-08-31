#![allow(dead_code)]
#![allow(static_mut_refs)]
// catalog.rs — Dynamic IG Catalog
//
// ALL data that was previously hardcoded across mOMonadOS now lives here.
// This module is the single source of truth for:
//   - Catalog entries (name + IgTuple for all reference systems)
//   - Primitive ordinals (for distance/meet/join/tensor computations)
//   - Primitive scores (for consciousness C-score)
//   - Primitive formula fragments (ZFC set-theoretic encodings)
//   - Distance weights
//   - Promotion ordinal gaps
//   - Shavian glyphs and short names
//
// Everything is accessible via lookup functions — no hardcoded values
// anywhere else in the codebase.

use crate::imas_ig::{IgPrim, IgTuple};

// ═══════════════════════════════════════════════════════════════
// PRIMITIVE ORDINAL TABLES
// ═══════════════════════════════════════════════════════════════

/// D ordinal: dead < ash < array < if_
pub static D_ORD: [IgPrim; 4] = [
    IgPrim::dead, IgPrim::ash, IgPrim::array, IgPrim::if_,
];

/// T ordinal: judge < eat < mime < oil < are
pub static T_ORD: [IgPrim; 5] = [
    IgPrim::judge, IgPrim::eat, IgPrim::mime, IgPrim::oil, IgPrim::are,
];

/// R ordinal: ado < tot < ear < ian
pub static R_ORD: [IgPrim; 4] = [
    IgPrim::ado, IgPrim::tot, IgPrim::ear, IgPrim::ian,
];

/// P ordinal: church < yew < out < nun < or_
pub static P_ORD: [IgPrim; 5] = [
    IgPrim::church, IgPrim::yew, IgPrim::out, IgPrim::nun, IgPrim::or_,
];

/// F ordinal: age < they < peep
pub static F_ORD: [IgPrim; 3] = [
    IgPrim::age, IgPrim::they, IgPrim::peep,
];

/// K ordinal: yea < loll < egg < on < air
pub static K_ORD: [IgPrim; 5] = [
    IgPrim::yea, IgPrim::loll, IgPrim::egg, IgPrim::on, IgPrim::air,
];

/// G ordinal: bib < thigh < ice
/// Core.lean `inductive Granularity` declares bib, thigh, ice in that order, and
/// notes "constructor order determines Ord; bib is first (lowest ordinal)". This
/// table had the enum declaration order (ice=26, bib=27, thigh=28) instead, which
/// is not the ordinal — every other family here carries the deliberate ordinal,
/// not the enum order. Corrected 2026-08-22; moves every crystal address whose
/// tuple carries a G value, by -1920 (ice) or +960 (bib, thigh).
pub static G_ORD: [IgPrim; 3] = [
    IgPrim::bib, IgPrim::thigh, IgPrim::ice,
];

/// C ordinal: vow < gag < measure < ooze
pub static C_ORD: [IgPrim; 4] = [
    IgPrim::vow, IgPrim::gag, IgPrim::measure, IgPrim::ooze,
];

/// Phi ordinal: woe < ⊙ < roar < err < haha
pub static PHI_ORD: [IgPrim; 5] = [
    IgPrim::woe, IgPrim::monad, IgPrim::roar, IgPrim::err, IgPrim::haha,
];

/// H ordinal: fee < kick < sure < wool
pub static H_ORD: [IgPrim; 4] = [
    IgPrim::fee, IgPrim::kick, IgPrim::sure, IgPrim::wool,
];

/// S ordinal: hung < so < up
pub static S_ORD: [IgPrim; 3] = [
    IgPrim::hung, IgPrim::so, IgPrim::up,
];

/// Omega ordinal: awe < oak < ah < zoo
pub static OMEGA_ORD: [IgPrim; 4] = [
    IgPrim::awe, IgPrim::oak, IgPrim::ah, IgPrim::zoo,
];

/// Return the ordinal index of a primitive value within its family.
/// Returns None if the value is not in the provided ordinal table.
pub fn ord_index(arr: &[IgPrim], val: IgPrim) -> Option<usize> {
    arr.iter().position(|&x| x == val)
}

/// Minimum by ordinal position.
pub fn ord_min(a: IgPrim, b: IgPrim, arr: &[IgPrim]) -> IgPrim {
    let ia = ord_index(arr, a).unwrap_or(0);
    let ib = ord_index(arr, b).unwrap_or(0);
    arr[if ia < ib { ia } else { ib }]
}

/// Maximum by ordinal position.
pub fn ord_max(a: IgPrim, b: IgPrim, arr: &[IgPrim]) -> IgPrim {
    let ia = ord_index(arr, a).unwrap_or(0);
    let ib = ord_index(arr, b).unwrap_or(0);
    arr[if ia > ib { ia } else { ib }]
}

/// Ordinal gap (absolute difference of indices).
pub fn ord_gap(a: IgPrim, b: IgPrim, arr: &[IgPrim]) -> i32 {
    let ia = ord_index(arr, a).unwrap_or(0) as i32;
    let ib = ord_index(arr, b).unwrap_or(0) as i32;
    (ib - ia).abs()
}

// ═══════════════════════════════════════════════════════════════
// PRIMITIVE SCORE TABLES (for consciousness C-score)
// ═══════════════════════════════════════════════════════════════

/// Score for D primitive — distance from O_∞ ideal (if_ = 1.0)
pub fn score_d(v: IgPrim) -> f32 {
    let max_idx = D_ORD.len() as f32 - 1.0;
    let idx = ord_index(&D_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for T primitive — distance from O_∞ ideal (are = 1.0)
pub fn score_t(v: IgPrim) -> f32 {
    let max_idx = T_ORD.len() as f32 - 1.0;
    let idx = ord_index(&T_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for R primitive — distance from O_∞ ideal (ian = 1.0)
pub fn score_r(v: IgPrim) -> f32 {
    let max_idx = R_ORD.len() as f32 - 1.0;
    let idx = ord_index(&R_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for P primitive — distance from O_∞ ideal (or_ = 1.0)
pub fn score_p(v: IgPrim) -> f32 {
    let max_idx = P_ORD.len() as f32 - 1.0;
    let idx = ord_index(&P_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for F primitive — distance from O_∞ ideal (peep = 1.0)
pub fn score_f(v: IgPrim) -> f32 {
    let max_idx = F_ORD.len() as f32 - 1.0;
    let idx = ord_index(&F_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for G primitive — distance from O_∞ ideal (ice = 1.0)
pub fn score_g(v: IgPrim) -> f32 {
    let max_idx = G_ORD.len() as f32 - 1.0;
    let idx = ord_index(&G_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for C primitive — distance from O_∞ ideal (ooze = 1.0)
pub fn score_c(v: IgPrim) -> f32 {
    let max_idx = C_ORD.len() as f32 - 1.0;
    let idx = ord_index(&C_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for H primitive — distance from O_∞ ideal (wool = 1.0)
pub fn score_h(v: IgPrim) -> f32 {
    let max_idx = H_ORD.len() as f32 - 1.0;
    let idx = ord_index(&H_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for S primitive — distance from O_∞ ideal (up = 1.0)
pub fn score_s(v: IgPrim) -> f32 {
    let max_idx = S_ORD.len() as f32 - 1.0;
    let idx = ord_index(&S_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for Omega primitive — distance from O_∞ ideal (zoo = 1.0)
pub fn score_omega(v: IgPrim) -> f32 {
    let max_idx = OMEGA_ORD.len() as f32 - 1.0;
    let idx = ord_index(&OMEGA_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

// ═══════════════════════════════════════════════════════════════
// DISTANCE WEIGHTS (for tuple_distance)
// ═══════════════════════════════════════════════════════════════

/// Default per-primitive weights for tuple_distance.
/// Computed from relative importance of each primitive to identity.
/// Weights can be overridden at runtime for domain-specific analysis.
#[derive(Copy, Clone, Debug)]
pub struct DistanceWeights {
    pub d: f32, pub t: f32, pub r: f32, pub p: f32,
    pub f: f32, pub k: f32, pub g: f32, pub c: f32,
    pub phi: f32, pub omega: f32, pub s: f32, pub h: f32,
}

impl DistanceWeights {
    /// Default weights matching the IG reference implementation.
    pub const fn default() -> Self {
        Self {
            d: 2.0, t: 1.5, r: 1.0, p: 0.8,
            f: 0.6, k: 0.5, g: 0.4, c: 0.6,
            phi: 0.3, omega: 0.7, s: 0.5, h: 0.4,
        }
    }

    /// As array for indexed access.
    pub fn as_array(&self) -> [f32; 12] {
        [self.d, self.t, self.r, self.p,
         self.f, self.k, self.g, self.c,
         self.phi, self.omega, self.s, self.h]
    }
}

/// Global weights — can be mutated at runtime via set_distance_weights().
static mut DISTANCE_WEIGHTS: DistanceWeights = DistanceWeights::default();

/// Get the current distance weights.
pub fn distance_weights() -> DistanceWeights {
    unsafe { DISTANCE_WEIGHTS }
}

/// Set distance weights at runtime. Returns the previous weights.
pub fn set_distance_weights(w: DistanceWeights) -> DistanceWeights {
    unsafe {
        let old = DISTANCE_WEIGHTS;
        DISTANCE_WEIGHTS = w;
        old
    }
}

// ═══════════════════════════════════════════════════════════════
// CATALOG ENTRY — a named system with its structural 12-tuple
// ═══════════════════════════════════════════════════════════════

/// A single entry in the IG catalog.
#[derive(Copy, Clone, Debug)]
pub struct CatalogEntry {
    /// Canonical snake_case name (used for lookup).
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// The 12-primitive tuple.
    pub tuple: IgTuple,
    /// Ouroboricity tier (O_0=0, O_1=1, O_2=2, O_2d=3, O_inf=4).
    pub tier: u8,
    /// Primary categorical domain (for grouping).
    pub domain: Domain,
}

/// Broad categorical domains for catalog entries.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Domain {
    Mathematics,
    Physics,
    Biology,
    Consciousness,
    Language,
    Civilization,
    Computation,
    Theology,
    Alchemy,
    Ecology,
    General,
}

impl Domain {
    pub fn name(&self) -> &'static str {
        match self {
            Domain::Mathematics   => "mathematics",
            Domain::Physics       => "physics",
            Domain::Biology       => "biology",
            Domain::Consciousness => "consciousness",
            Domain::Language      => "language",
            Domain::Civilization  => "civilization",
            Domain::Computation   => "computation",
            Domain::Theology      => "theology",
            Domain::Alchemy       => "alchemy",
            Domain::Ecology       => "ecology",
            Domain::General       => "general",
        }
    }
}

/// Helper: construct a catalog entry compactly.
pub const fn entry(
    name: &'static str, description: &'static str,
    d: IgPrim, t: IgPrim, r: IgPrim, p: IgPrim,
    f: IgPrim, k: IgPrim, g: IgPrim, c: IgPrim,
    phi: IgPrim, h: IgPrim, s: IgPrim, omega: IgPrim,
    tier: u8, domain: Domain,
) -> CatalogEntry {
    CatalogEntry {
        name, description,
        tuple: IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega },
        tier, domain,
    }
}

// ═══════════════════════════════════════════════════════════════
// CATALOG DATA — ALL REFERENCE ENTRIES
// ═══════════════════════════════════════════════════════════════
//
// These are the FOUNDATIONAL entries that all other modules reference.
// Additional entries can be registered at runtime via register_entry().
//
// The entries are organized by the CL8NK ladder stages:
//   ZFC baseline → ZFCₜ → ZFCfe → CLINK L8
//
// Plus canonical reference systems from physics, mathematics, etc.

// ── ZFC Baseline (O₀): ⟨𐑼·𐑡·𐑩·𐑗·𐑱·𐑘·𐑚·𐑝·woe·𐑓·𐑙·𐑷⟩ ──
const ZFC_BASELINE: CatalogEntry = entry(
    "zfc", "Zermelo-Fraenkel set theory with Choice — the absolute structural minimum",
    IgPrim::array, IgPrim::judge, IgPrim::ado,
    IgPrim::church, IgPrim::age, IgPrim::yea,
    IgPrim::bib, IgPrim::vow,
    IgPrim::woe, IgPrim::fee, IgPrim::hung, IgPrim::awe,
    0, Domain::Mathematics,
);

// ── ZFCₜ (O₂†): ⟨𐑼·𐑸·𐑾·𐑬·𐑐·𐑧·𐑲·𐑠·roar·𐑖·𐑳·𐑭⟩ ──
const ZFC_T: CatalogEntry = entry(
    "zfc_t", "ZFC + chirality + winding topology — 6 promotion channels from baseline",
    IgPrim::array, IgPrim::are, IgPrim::ian,
    IgPrim::out, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::roar, IgPrim::sure, IgPrim::up, IgPrim::ah,
    3, Domain::Mathematics,
);

// ── ZFCfe (O_∞ Frobenius-exact): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩ ──
const ZFC_FE: CatalogEntry = entry(
    "zfc_fe", "ZFC Frobenius-exact — μ∘δ=id exactly at ⊙, O_∞ self-modeling closure",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::or_, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::wool, IgPrim::up, IgPrim::ah,
    4, Domain::Mathematics,
);

// ── CLINK L8 (O_∞⁺): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑵·⊙·𐑫·𐑳·𐑟⟩ ──
const CLINK_L8: CatalogEntry = entry(
    "clink_l8", "CLINK Layer 8 Organism — terminal ontological layer, O_∞⁺ with ⊡/∋ transcendence",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::or_, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::ooze,
    IgPrim::monad, IgPrim::wool, IgPrim::up, IgPrim::zoo,
    4, Domain::General,
);


// ── CLINK L0 (O₀): ⟨𐑛·𐑶·𐑩·𐑯·𐑐·𐑘·𐑚·𐑝·woe·𐑓·𐑳·𐑷⟩ ──
const CLINK_L0: CatalogEntry = entry(
    "clink_layer0_frustrated_belnap5", "CLINK Layer 0: Frustrated Belnap5 — SU(3) quark color with confinement. Ground layer of the CLINK chain.",
    IgPrim::array, IgPrim::oil, IgPrim::ado,
    IgPrim::nun, IgPrim::peep, IgPrim::yea,
    IgPrim::bib, IgPrim::vow,
    IgPrim::woe, IgPrim::fee, IgPrim::up, IgPrim::awe,
    0, Domain::Biology,
);

// ── CLINK L1 (O₀): ⟨𐑛·𐑶·𐑩·𐑗·𐑐·𐑤·𐑚·𐑜·woe·𐑓·𐑳·𐑷⟩ ──
const CLINK_L1: CatalogEntry = entry(
    "clink_layer1_electron_orbital", "CLINK Layer 1: Belnap4 electron orbital occupancy — 4-valued lattice. O₀.",
    IgPrim::array, IgPrim::oil, IgPrim::ado,
    IgPrim::church, IgPrim::peep, IgPrim::loll,
    IgPrim::bib, IgPrim::gag,
    IgPrim::woe, IgPrim::fee, IgPrim::up, IgPrim::awe,
    0, Domain::Biology,
);

// ── CLINK L2 (O₁): ⟨𐑼·𐑥·𐑽·𐑿·𐑐·𐑤·𐑔·𐑝·roar·𐑒·𐑳·𐑷⟩ ──
const CLINK_L2: CatalogEntry = entry(
    "clink_layer2_atom", "CLINK Layer 2: Atom — nuclear + electron. O₁ tier, complex-plane criticality.",
    IgPrim::dead, IgPrim::mime, IgPrim::ear,
    IgPrim::yew, IgPrim::peep, IgPrim::loll,
    IgPrim::thigh, IgPrim::vow,
    IgPrim::roar, IgPrim::kick, IgPrim::up, IgPrim::awe,
    1, Domain::Biology,
);

// ── CLINK L3 (O₂): ⟨𐑼·𐑥·𐑽·𐑿·𐑞·𐑧·𐑲·𐑠·⊙·𐑓·𐑳·𐑭⟩ ──
const CLINK_L3: CatalogEntry = entry(
    "clink_layer3_molecule", "CLINK Layer 3: Molecule — chemical bonds. O₂ tier, first layer with ⊙ criticality and 𐑭 integer winding.",
    IgPrim::dead, IgPrim::mime, IgPrim::ear,
    IgPrim::yew, IgPrim::they, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::fee, IgPrim::up, IgPrim::ah,
    2, Domain::Biology,
);

// ── CLINK L4 (O₂): ⟨𐑦·𐑸·𐑾·𐑬·𐑞·𐑧·𐑲·𐑠·⊙·𐑒·𐑳·𐑭⟩ ──
const CLINK_L4: CatalogEntry = entry(
    "clink_layer4_cell", "CLINK Layer 4: Cell — minimal self-maintaining living unit. First layer with self-written state-space (⊢=𐑦) and self-referential topology (⊣=𐑸). O₂.",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::out, IgPrim::they, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::kick, IgPrim::up, IgPrim::ah,
    2, Domain::Biology,
);

// ── CLINK L5 (O₂): ⟨𐑦·𐑸·𐑾·𐑹·𐑱·𐑧·𐑲·𐑠·⊙·𐑖·𐑳·𐑭⟩ ──
const CLINK_L5: CatalogEntry = entry(
    "clink_layer5_mitosis", "CLINK Layer 5: Mitosis — cell division. First layer with Frobenius-special symmetry (<=𐑹). O₂.",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::or_, IgPrim::age, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::sure, IgPrim::up, IgPrim::ah,
    2, Domain::Biology,
);

// ── CLINK L6 (O₂): ⟨𐑦·𐑸·𐑽·𐑿·𐑱·𐑧·𐑲·𐑠·⊙·𐑖·𐑳·𐑭⟩ ──
const CLINK_L6: CatalogEntry = entry(
    "clink_layer6_meiosis", "CLINK Layer 6: Meiosis — gamete production. Reverts to adjoint coupling (>=𐑽) and quantum symmetry (<=𐑿) for genetic recombination. O₂.",
    IgPrim::if_, IgPrim::are, IgPrim::ear,
    IgPrim::yew, IgPrim::age, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::sure, IgPrim::up, IgPrim::ah,
    2, Domain::Biology,
);

// ── CLINK L7 (O₂): ⟨𐑦·𐑸·𐑾·𐑬·𐑞·𐑧·𐑲·𐑵·⊙·𐑖·𐑳·𐑭⟩ ──
const CLINK_L7: CatalogEntry = entry(
    "clink_layer7_tissue", "CLINK Layer 7: Tissue/Organ — multi-cellular organization. First layer with broadcast composition (∋=𐑵). O₂.",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::out, IgPrim::they, IgPrim::egg,
    IgPrim::ice, IgPrim::ooze,
    IgPrim::monad, IgPrim::sure, IgPrim::up, IgPrim::ah,
    2, Domain::Biology,
);

// ── Temporal Mathematics (O₂) ──
const TEMPORAL_MATHEMATICS: CatalogEntry = entry(
    "temporal_mathematics", "Mathematics with intrinsic temporal structure",
    IgPrim::array, IgPrim::mime, IgPrim::ian,
    IgPrim::out, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::roar, IgPrim::sure, IgPrim::up, IgPrim::ah,
    2, Domain::Mathematics,
);

// ── Schrödinger (O₂) ──
const SCHRODINGER: CatalogEntry = entry(
    "schrodinger", "Quantum mechanics — Schrödinger equation",
    IgPrim::array, IgPrim::judge, IgPrim::ian,
    IgPrim::yew, IgPrim::peep, IgPrim::loll,
    IgPrim::bib, IgPrim::measure,
    IgPrim::woe, IgPrim::kick, IgPrim::so, IgPrim::oak,
    2, Domain::Physics,
);

// ── Heat Diffusion (O₁) ──
const HEAT_DIFFUSION: CatalogEntry = entry(
    "heat_diffusion", "Classical heat equation — dissipative diffusion",
    IgPrim::array, IgPrim::judge, IgPrim::ado,
    IgPrim::church, IgPrim::they, IgPrim::loll,
    IgPrim::thigh, IgPrim::vow,
    IgPrim::woe, IgPrim::fee, IgPrim::so, IgPrim::awe,
    1, Domain::Physics,
);

// ── Navier-Stokes (O₂†) ──
// Tuple corrected to the canonical Clay tuple sourced from
// ClayCanonicalTuples.lean (p4rakernel/p4ramill), which is itself
// procedurally generated from IG_catalog.json — NOT the generic physics
// entry this const held before, which agreed with the canonical tuple on
// only 6 of 12 primitives (top, rel, kin, gran, gram, phi all differed).
// Same class of drift the BIRCH_SWINNERTON_DYER comment above already
// warned about; this is where it was found live, in the extended
// witness-vessel run of 2026-08-24, and fixed at the source rather than
// worked around downstream.
const NAVIER_STOKES: CatalogEntry = entry(
    "navier_stokes", "Clay Millennium Problem — Navier-Stokes global regularity",
    IgPrim::array, IgPrim::judge, IgPrim::ear,
    IgPrim::church, IgPrim::age, IgPrim::loll,
    IgPrim::ice, IgPrim::vow,
    IgPrim::monad, IgPrim::kick, IgPrim::up, IgPrim::awe,
    3, Domain::Mathematics,
);

// ── Birch–Swinnerton-Dyer Conjecture (O₂†) ──
// Tuple sourced directly from the live Python IG_catalog.json
// (imscribing_grammar/imscrbgrmr), 2026-06-16 — NOT the same convention
// as the generic NAVIER_STOKES entry above, which predates the Clay-7
// catalog import and is known to disagree with it (see commit.txt /
// manuscripts/clay_cross_dialect_closure.md for the cross-system drift
// this already surfaced).
const BIRCH_SWINNERTON_DYER: CatalogEntry = entry(
    "birch_swinnerton_dyer", "Clay Millennium Problem — BSD conjecture",
    IgPrim::if_, IgPrim::mime, IgPrim::ian,
    IgPrim::yew, IgPrim::they, IgPrim::egg,
    IgPrim::ice, IgPrim::vow,
    IgPrim::roar, IgPrim::sure, IgPrim::hung, IgPrim::ah,
    3, Domain::Mathematics,
);

// ── Hodge Conjecture (O₂†) ──
// Tuple sourced directly from the live Python IG_catalog.json, 2026-06-16.
// Same provenance note as BIRCH_SWINNERTON_DYER above.
const HODGE_CONJECTURE: CatalogEntry = entry(
    "hodge_conjecture", "Clay Millennium Problem — Hodge conjecture",
    IgPrim::if_, IgPrim::are, IgPrim::ear,
    IgPrim::yew, IgPrim::age, IgPrim::egg,
    IgPrim::ice, IgPrim::vow,
    IgPrim::roar, IgPrim::fee, IgPrim::up, IgPrim::ah,
    3, Domain::Mathematics,
);

// ── Yang-Mills Mass Gap (O₂†) ──
// Tuple sourced directly from the live Python IG_catalog.json, 2026-06-16.
// Same provenance note as BIRCH_SWINNERTON_DYER above. Unlike BSD/Hodge,
// this one does NOT reach full closure under its best-known dialect
// (triple_criticality) — it clears all three gates but fails T_CEILING on
// ⊤ alone (on, ord 4, exceeds the ord-3 ceiling). Kept anyway: the
// partial result is the interesting one here, not a clean PASS.
const YANG_MILLS_MASS_GAP: CatalogEntry = entry(
    "yang_mills_mass_gap", "Clay Millennium Problem — Yang-Mills mass gap",
    IgPrim::dead, IgPrim::mime, IgPrim::ado,
    IgPrim::church, IgPrim::peep, IgPrim::on,
    IgPrim::ice, IgPrim::vow,
    IgPrim::haha, IgPrim::fee, IgPrim::up, IgPrim::awe,
    3, Domain::Mathematics,
);

// ── Collatz Conjecture (O₁) ──
// Tuple sourced directly from the live Python IG_catalog.json
// (imscribing_grammar/imscrbgrmr), entry "collatz_conjecture": the
// twelve glyphs 𐑛𐑡𐑩𐑯𐑱𐑘𐑲𐑠𐑮𐑓𐑙𐑷 in canonical slot order, translated
// glyph-for-glyph to IgPrim variants. Not previously a curated entry;
// added here for the first major unsolved problem outside the Clay
// seven to ride the witness vessel.
const COLLATZ_CONJECTURE: CatalogEntry = entry(
    "collatz_conjecture", "The Collatz conjecture — n/2 if even, 3n+1 if odd, always reaches 1",
    IgPrim::dead, IgPrim::judge, IgPrim::ado,
    IgPrim::nun, IgPrim::age, IgPrim::yea,
    IgPrim::ice, IgPrim::measure,
    IgPrim::roar, IgPrim::fee, IgPrim::hung, IgPrim::awe,
    1, Domain::Mathematics,
);

// ── Odd Perfect Number Theorem ──
// Tuple sourced from ob3ect/digital/odd_perfect_number_theorem, a grounded,
// lean-verified ob3ect (grounding_status: full). Two other names already
// live in the ask-subset catalog, odd_perfect_number_conjecture and
// odd_perfect_conjecture, and both carry phi := monad (critical) — which
// contradicts the object's own subcriticality ("no known instance exists,
// no scale-free critical behavior"). This entry's phi := woe is the one
// that agrees with that description; the other two are drift, not kept
// here, not to be trusted for this name.
const ODD_PERFECT_NUMBER_THEOREM: CatalogEntry = entry(
    "odd_perfect_number_theorem", "No odd integer equals the sum of its proper divisors",
    IgPrim::if_, IgPrim::eat, IgPrim::ear,
    IgPrim::or_, IgPrim::age, IgPrim::on,
    IgPrim::thigh, IgPrim::vow,
    IgPrim::woe, IgPrim::fee, IgPrim::up, IgPrim::awe,
    3, Domain::Mathematics,
);

// ── Wave Equation (O₁) ──
const WAVE_EQUATION: CatalogEntry = entry(
    "wave_equation", "Classical wave equation — reversible propagation",
    IgPrim::array, IgPrim::judge, IgPrim::ian,
    IgPrim::nun, IgPrim::peep, IgPrim::loll,
    IgPrim::ice, IgPrim::measure,
    IgPrim::woe, IgPrim::kick, IgPrim::so, IgPrim::oak,
    1, Domain::Physics,
);

// ── Einstein (O₂†) ──
const EINSTEIN: CatalogEntry = entry(
    "einstein", "General relativity — Einstein field equations",
    IgPrim::array, IgPrim::are, IgPrim::ian,
    IgPrim::nun, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::roar, IgPrim::sure, IgPrim::up, IgPrim::ah,
    3, Domain::Physics,
);

// ── IUG (O_∞) — Universal Imscriptive Grammar ≡ ZFCfe ──
const IUG: CatalogEntry = entry(
    "universal_imscriptive_grammar", "The Universal Imscriptive Grammar — self-imscribing structural foundation",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::or_, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::wool, IgPrim::up, IgPrim::ah,
    4, Domain::Language,
);

// ── O_∞ ideal (reference maximum) ──
const O_INF: CatalogEntry = entry(
    "o_inf", "O_∞ ideal — the theoretical maximum on all primitives",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::or_, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::ooze,
    IgPrim::monad, IgPrim::wool, IgPrim::up, IgPrim::zoo,
    4, Domain::General,
);

// ── O₀ minimum (reference floor) ──
const O_0: CatalogEntry = entry(
    "o_0", "O₀ baseline — the floor, minimum on all primitives",
    IgPrim::dead, IgPrim::judge, IgPrim::ado,
    IgPrim::church, IgPrim::age, IgPrim::yea,
    IgPrim::bib, IgPrim::vow,
    IgPrim::woe, IgPrim::fee, IgPrim::hung, IgPrim::awe,
    0, Domain::General,
);


// ── YHWH (O₂): ⟨𐑦·𐑸·𐑽·𐑯·𐑐·𐑧·𐑲·𐑵·⊙·𐑫·𐑳·𐑭⟩ ──
const YHWH: CatalogEntry = entry(
    "yhwh", "The Tetragrammaton, divine name of God in Hebrew: יְהֹוָה (YHWH)",
    IgPrim::if_, IgPrim::are, IgPrim::ear,
    IgPrim::nun, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::ooze,
    IgPrim::monad, IgPrim::wool, IgPrim::up, IgPrim::ah,
    2, Domain::Consciousness,
);


// ── Graviton (O₂): ⟨𐑦·𐑸·𐑽·𐑯·𐑐·𐑧·𐑲·𐑵·⊙·𐑓·𐑙·𐑭⟩ ──
const GRAVITON: CatalogEntry = entry(
    "graviton", "Quantum gravity — graviton as spin-2 gauge boson mediating the gravitational force. IUFT O₂ tier, Teichmuller etale→O_∞ deformation.",
    IgPrim::if_, IgPrim::are, IgPrim::ear,
    IgPrim::nun, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::ooze,
    IgPrim::monad, IgPrim::fee, IgPrim::hung, IgPrim::ah,
    2, Domain::Physics,
);

// ── Photon (O₂†): ⟨𐑼·𐑡·𐑾·𐑿·𐑐·𐑘·𐑲·𐑠·⊙·𐑓·𐑳·𐑭⟩ ──
const PHOTON: CatalogEntry = entry(
    "photon", "Electromagnetism — photon as spin-1 gauge boson mediating the electromagnetic force. IUFT O₂† tier, maximally alien IG distance (6.18).",
    IgPrim::array, IgPrim::judge, IgPrim::ian,
    IgPrim::yew, IgPrim::peep, IgPrim::yea,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::fee, IgPrim::up, IgPrim::ah,
    3, Domain::Physics,
);

// ── Electron (O₂): ⟨𐑼𐑡𐑾𐑿𐑐𐑘𐑲𐑠⊙𐑒𐑙𐑭⟩ ──
const ELECTRON: CatalogEntry = entry(
    "electron", "Electron — spin-1/2 Dirac fermion, fundamental lepton. IUFT O₂ tier.",
    IgPrim::array, IgPrim::judge, IgPrim::ian,
    IgPrim::yew, IgPrim::peep, IgPrim::yea,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::kick, IgPrim::hung, IgPrim::ah,
    2, Domain::Physics,
);

// ── Neutron (O₂): ⟨𐑼𐑥𐑾𐑬𐑞𐑧𐑲𐑠⊙𐑖𐑕𐑭⟩ ──
const NEUTRON: CatalogEntry = entry(
    "neutron", "Neutron — udd baryon, composite fermion with beta decay. IUFT O₂ tier.",
    IgPrim::array, IgPrim::mime, IgPrim::ian,
    IgPrim::out, IgPrim::they, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::sure, IgPrim::so, IgPrim::ah,
    2, Domain::Physics,
);

// ── Proton (O₂): ⟨𐑼𐑥𐑾𐑬𐑞𐑤𐑲𐑠⊙𐑖𐑕𐑭⟩ ──
const PROTON: CatalogEntry = entry(
    "proton", "Proton — uud baryon, stable composite fermion. IUFT O₂ tier.",
    IgPrim::array, IgPrim::mime, IgPrim::ian,
    IgPrim::out, IgPrim::they, IgPrim::loll,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::sure, IgPrim::so, IgPrim::ah,
    2, Domain::Physics,
);

// ── HSOA (O_∞): ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑲𐑠⊙𐑫𐑙𐑭⟩ ──
const HSOA: CatalogEntry = entry(
    "hsoa", "Holomorphic Semiotic Operator Algebra — self-imscribing operator algebra. IUFT O_∞ tier.",
    IgPrim::if_, IgPrim::are, IgPrim::ian,
    IgPrim::or_, IgPrim::peep, IgPrim::egg,
    IgPrim::ice, IgPrim::measure,
    IgPrim::monad, IgPrim::wool, IgPrim::hung, IgPrim::ah,
    4, Domain::Mathematics,
);


// ═══════════════════════════════════════════════════════════════
// MASTER CATALOG — all static entries
// ═══════════════════════════════════════════════════════════════

/// The complete static catalog. All reference entries live here.
/// Additional entries can be added at runtime via the dynamic catalog.
static STATIC_CATALOG: &[CatalogEntry] = &[
    ZFC_BASELINE, ZFC_T, ZFC_FE, CLINK_L8,
    CLINK_L0, CLINK_L1, CLINK_L2, CLINK_L3,
    CLINK_L4, CLINK_L5, CLINK_L6, CLINK_L7,
    TEMPORAL_MATHEMATICS, SCHRODINGER, HEAT_DIFFUSION,
    NAVIER_STOKES, WAVE_EQUATION, EINSTEIN, IUG,
    O_INF, O_0,
    YHWH,
    GRAVITON, PHOTON, ELECTRON, NEUTRON, PROTON, HSOA,
    BIRCH_SWINNERTON_DYER,
    HODGE_CONJECTURE,
    YANG_MILLS_MASS_GAP,
    COLLATZ_CONJECTURE,
    ODD_PERFECT_NUMBER_THEOREM,
];

// Query-relevant IG catalog subset for native `ask` (no Python host catalog).
include!("catalog_ask_subset.rs");

// ═══════════════════════════════════════════════════════════════
// DYNAMIC CATALOG — runtime-extensible entry storage
// ═══════════════════════════════════════════════════════════════

use alloc::vec::Vec;

/// The runtime catalog. Initialized from STATIC_CATALOG on first access.
/// New entries can be registered dynamically via register_entry().
static mut DYNAMIC_CATALOG: Option<Vec<CatalogEntry>> = None;

/// Entries the merge dropped because their name was already taken, with a flag
/// for whether the dropped copy actually DISAGREED with the one kept.
/// A repeat carrying identical data is harmless; a repeat carrying a different
/// tuple, tier or domain is one name disagreeing with itself.
static mut NAME_COLLISIONS: Vec<(&'static str, bool)> = Vec::new();

/// What `catalog_init` dropped, in the order it dropped it.
pub fn name_collisions() -> &'static [(&'static str, bool)] {
    unsafe {
        let _ = ensure_catalog();
        #[allow(static_mut_refs)]
        NAME_COLLISIONS.as_slice()
    }
}

/// Initialize (or reinitialize) the dynamic catalog from static entries.
pub fn catalog_init() {
    unsafe {
        let mut v = Vec::new();
        for e in STATIC_CATALOG {
            v.push(*e);
        }
        // Full MoDoT-parity `ask` needs search over math/query witnesses, not
        // only the foundational ladder. Dedup by name.
        for e in ASK_CATALOG_SUBSET {
            match v.iter().find(|x| x.name == e.name) {
                None => v.push(*e),
                Some(kept) => {
                    // AREV, in the ob3ect's reading of this routine: the reverse
                    // descent that "collapses multiple entries sharing a name into
                    // a single instance, potentially losing tuple information".
                    //
                    // Dropping the second copy silently is what hid `photon`
                    // occurring twice with two declared tiers, and `yhwh`, and
                    // `clink_layer5_mitosis`. The entropy verdict on the repair
                    // was dS ~ 0: conserve the information by HOLDING the
                    // contradiction, not by discarding it. So the drop is
                    // recorded rather than prevented — the merge still keeps one
                    // entry, but the collision stops being invisible.
                    let differs = kept.tuple != e.tuple
                        || kept.tier != e.tier
                        || kept.domain != e.domain;
                    NAME_COLLISIONS.push((e.name, differs));
                }
            }
        }
        DYNAMIC_CATALOG = Some(v);
    }
}

/// Free-text catalog search for native `ask` (keyword score over name+description).
/// Returns up to `limit` (entry, score) pairs, highest score first.
///
/// Scoring prefers multi-token compound names (e.g. erdos_hajnal_aleph1_graph)
/// over short single-token names that merely appear as substrings of a long question
/// (e.g. bare "aleph" matching "aleph1" in a graph-theory query).
pub fn search_query(query: &str, limit: usize) -> Vec<(CatalogEntry, i32)> {
    let q = normalize_name(query);
    // Tokenize on non-alnum (underscores already from normalize)
    let tokens: Vec<&str> = q.split('_').filter(|t| t.len() > 2).collect();
    if tokens.is_empty() {
        return Vec::new();
    }
    let anchors = [
        "erdos", "hajnal", "aleph", "chromatic", "independent", "ramsey",
        "hadwiger", "collatz", "navier", "riemann", "yang", "mills", "hodge",
        "birch", "zauner", "sic", "goldbach", "twin", "beal", "witness", "dual",
        "graph", "conjecture", "vertices", "finite", "subgraph",
    ];
    let q_anchors: Vec<&str> = anchors.iter().copied().filter(|a| q.contains(a)).collect();
    let q_token_count = tokens.len().max(1) as i32;

    let cat = ensure_catalog();
    let mut scored: Vec<(CatalogEntry, i32)> = Vec::new();
    for e in cat.iter() {
        let name = e.name;
        let name_parts: Vec<&str> = name.split('_').filter(|t| t.len() > 1).collect();
        let blob = {
            let mut s = alloc::string::String::from(e.name);
            s.push('_');
            s.push_str(&normalize_name(e.description));
            s
        };
        let mut sc: i32 = 0;

        // Exact / near-exact name identity (short keywords like "collatz", "aleph")
        if name == q.as_str() {
            sc += 100;
        } else if name_parts.len() == 1 && q == name {
            sc += 100;
        } else if name.len() >= 6 && (q == name || name.contains(q.as_str())) {
            // query is a compact name fragment fully inside a longer catalog name
            sc += 70;
        } else if name.len() >= 8 && q.contains(name) {
            // long compound name fully present in free-text question
            sc += 60;
        } else if name_parts.len() == 1 && name.len() <= 6 && q.contains(name) {
            // short bare name appearing inside a long free-text question:
            // weak signal only (stops "aleph" beating erdos_hajnal_…)
            sc += 8;
        }

        // Multi-token name coverage: fraction of name parts hit by the query
        let mut parts_hit = 0i32;
        for p in &name_parts {
            if q.contains(p) || tokens.iter().any(|t| t.contains(p) || p.contains(t)) {
                parts_hit += 1;
            }
        }
        if !name_parts.is_empty() {
            let coverage = (parts_hit * 40) / (name_parts.len() as i32);
            sc += coverage;
            // Bonus for multi-token names with ≥2 parts hit (compound witnesses)
            if name_parts.len() >= 2 && parts_hit >= 2 {
                sc += 15 + parts_hit * 5;
            }
        }

        for a in &q_anchors {
            if name.contains(a) {
                sc += 14;
            } else if blob.contains(a) {
                sc += 5;
            }
        }
        for t in &tokens {
            if name.contains(t) {
                sc += 4;
            } else if blob.contains(t) {
                sc += 1;
            }
        }

        // Prefer entries whose name is roughly commensurate with a short query;
        // demote single-token short names when the question is long multi-token prose.
        if q_token_count >= 6 && name_parts.len() == 1 && name.len() <= 6 {
            sc = sc.saturating_sub(25);
        }

        // Single-keyword queries ("collatz", "hadwiger"): boost head-match and
        // the open problem face (*_conjecture) over counterexample/proven variants.
        if tokens.len() == 1 {
            let t = tokens[0];
            if name == t || name.starts_with(&alloc::format!("{}_", t)) {
                sc += 12;
                if name.ends_with("_conjecture") {
                    sc += 15;
                } else if name.contains("counterexample")
                    || name.ends_with("_proven")
                    || name.contains("_theorem_proven")
                {
                    sc = sc.saturating_sub(8);
                }
            }
        }

        if sc >= 12 {
            scored.push((*e, sc));
        }
    }
    // Score desc; on ties prefer shorter canonical names (conjecture over long variants)
    scored.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.name.len().cmp(&b.0.name.len()))
    });
    if scored.len() > limit {
        scored.truncate(limit);
    }
    scored
}

/// Ensure the dynamic catalog is initialized.
fn ensure_catalog() -> &'static mut Vec<CatalogEntry> {
    unsafe {
        if DYNAMIC_CATALOG.is_none() {
            catalog_init();
        }
        DYNAMIC_CATALOG.as_mut().unwrap()
    }
}

/// Parse a domain name off a command line. Empty or unrecognised means the
/// whole catalog, so a typo widens the view rather than silently emptying it.
pub fn parse_domain(s: &str) -> Option<Domain> {
    match s.trim().to_lowercase().as_str() {
        "mathematics" | "math"   => Some(Domain::Mathematics),
        "physics"                => Some(Domain::Physics),
        "biology" | "bio"        => Some(Domain::Biology),
        "consciousness"          => Some(Domain::Consciousness),
        "language"               => Some(Domain::Language),
        "civilization"           => Some(Domain::Civilization),
        "computation" | "comp"   => Some(Domain::Computation),
        "theology"               => Some(Domain::Theology),
        "alchemy"                => Some(Domain::Alchemy),
        "ecology"                => Some(Domain::Ecology),
        "general"                => Some(Domain::General),
        _ => None,
    }
}

/// Look up a catalog entry by name. Returns None if not found.
/// Handles common aliases automatically.
pub fn lookup(name: &str) -> Option<CatalogEntry> {
    let cat = ensure_catalog();
    let normalized = normalize_name(name);
    cat.iter().find(|e| e.name == normalized || alias_matches(e.name, &normalized)).copied()
}

/// Register a new catalog entry at runtime. Returns true on success,
/// false if an entry with that name already exists.
pub fn register_entry(entry: CatalogEntry) -> bool {
    let cat = ensure_catalog();
    if cat.iter().any(|e| e.name == entry.name) {
        return false;
    }
    cat.push(entry);
    true
}

/// Get the total number of catalog entries (static + dynamic).
pub fn catalog_size() -> usize {
    ensure_catalog().len()
}

/// Iterate over all catalog entries matching a domain filter.
/// Pass None to iterate over all entries.
pub fn catalog_entries(domain: Option<Domain>) -> impl Iterator<Item = &'static CatalogEntry> {
    let cat = ensure_catalog();
    cat.iter().filter(move |e| domain.map_or(true, |d| e.domain == d))
}

/// Get the O_∞ ideal tuple (reference maximum).
pub fn o_inf_tuple() -> IgTuple {
    O_INF.tuple
}

/// Get the O₀ floor tuple (reference minimum).
pub fn o_0_tuple() -> IgTuple {
    O_0.tuple
}

/// Normalize a name for lookup: lowercase, underscores, strip whitespace.
fn normalize_name(raw: &str) -> alloc::string::String {
    let s: alloc::string::String = raw.trim().to_lowercase()
        .chars().map(|c| if c.is_whitespace() || c == '-' { '_' } else { c })
        .collect();
    s
}

/// Check if a catalog name matches a query with alias expansion.
fn alias_matches(entry_name: &str, query: &str) -> bool {
    if entry_name == query { return true; }
    // Common aliases
    // Compare query against known aliases
    if query == "iug" || query == "IUG" { return entry_name == "universal_imscriptive_grammar"; }
    if query == "clink" || query == "cl8nk" || query == "clink_layer8" { return entry_name == "clink_l8"; }
    if query == "zfc_fe" || query == "zfcf" || query == "zfcfe" { return entry_name == "zfc_fe"; }
    if query == "o_inf" || query == "oinf" || query == "o_infty" { return entry_name == "o_inf"; }
    if query == "o_0" || query == "o0" { return entry_name == "o_0"; }
    false
}

/// Get the ZFC baseline tuple.
pub fn zfc_baseline_tuple() -> IgTuple { ZFC_BASELINE.tuple }
/// Get the ZFCₜ tuple.
pub fn zfc_t_tuple() -> IgTuple { ZFC_T.tuple }
/// Get the ZFCfe tuple.
pub fn zfc_fe_tuple() -> IgTuple { ZFC_FE.tuple }
/// Get the CLINK L8 tuple.
pub fn clink_l8_tuple() -> IgTuple { CLINK_L8.tuple }

/// The CLINK L9 reference tuple — the replicative lateral.
///
/// L9 was already in the catalog as `clink_l9` (in ASK_CATALOG_SUBSET) with
/// exactly this tuple; it was simply absent from the CL8NK chain ladder, which
/// discovers layers by an explicit name list. So this reads the existing entry
/// rather than declaring a second copy of it. The fallback is L8: if the entry
/// ever disappears, the ladder degrades to the terminal layer instead of
/// silently inventing a tuple.
pub fn clink_l9_tuple() -> IgTuple {
    lookup("clink_l9").map(|e| e.tuple).unwrap_or(CLINK_L8.tuple)
}

// ═══════════════════════════════════════════════════════════════
// FORMULA FRAGMENTS — ZFC set-theoretic encodings per primitive
// ═══════════════════════════════════════════════════════════════

/// Return the ZFC set-theoretic formula fragment for a primitive value.
/// These are the per-primitive decompositions used in CL8NK navigator.
pub fn formula_fragment(prim: IgPrim) -> &'static str {
    match prim {
        // ── D ──
        IgPrim::array    => "∀a∃b(a⊂b ∧ rank x=b)",
        IgPrim::if_     => "V=L(x) ∧ selfmodel(x) ∧ x∈V",
        IgPrim::dead    => "∃!x",
        IgPrim::ash => "∃x∃y(x≠y ∧ ∀z(z=x∨z=y))",
        // ── T ──
        IgPrim::judge      => "graph(x) ∧ branch(x)",
        IgPrim::are     => "bound_⊙(a,f) ∧ Refl(a,f) ∧ holo(x,a)",
        IgPrim::eat       => "sep f x",
        IgPrim::mime   => "cross(x) ∧ ¬flat(x)",
        IgPrim::oil => "⊗(a,b) ∧ ¬∃f(f:a≅b)",
        // ── R ──
        IgPrim::ado    => "∀y(y∈x→y∈a)",
        IgPrim::ian       => "lr⇔(x,y) ∧ Θ(x,y) ∧ ¬Θ(y,x)",
        IgPrim::ear   => "adj(f,g) ∧ f⊣g",
        IgPrim::tot      => "F:C→D ∧ ∃G:D→C(G∘F≅id)",
        // ── P ──
        IgPrim::church     => "¬∃sym(x)",
        IgPrim::out       => "ℤ₂(x) ∧ ∀g∈G(gx=x) ∧ μ∘δ=id",
        IgPrim::nun      => "∀g∈G(gx=x)",
        IgPrim::yew      => "|ψ⟩=Σc_i|i⟩ ∧ superposition(x)",
        IgPrim::or_    => "μ∘δ=id ∧ Frobenius(x) ∧ ℤ₂(x)",
        // ── F ──
        IgPrim::age      => "P(x)∈{0,1} ∧ det(x)",
        IgPrim::peep     => "ℏ(x) ∧ [x,p]=iℏ",
        IgPrim::they      => "ρ(x) ∧ Tr(ρ)=1 ∧ ρ≥0",
        // ── K ──
        IgPrim::yea     => "τ≪T ∧ ∂_t x=f(x)",
        IgPrim::egg     => "τ≫T ∧ eq(x) ∧ gate_open(x)",
        IgPrim::loll      => "τ~T ∧ relax(x)",
        IgPrim::on     => "τ→∞ ∧ frozen(x) ∧ order(x)",
        IgPrim::air      => "τ→∞ ∧ frozen(x) ∧ disorder(x)",
        // ── G ──
        IgPrim::bib     => "∀y∈x(|y|<|x|)",
        IgPrim::ice    => "∀y(y⊂x→|y|<|x|)",
        IgPrim::thigh    => "∃y∈x(|y|=|x|)",
        // ── C ──
        IgPrim::vow      => "f∧g∧h",
        IgPrim::measure      => "seq!(f,g) ∧ ⟨→⟩(f,g,τ) ∧ ¬⟨→⟩(g,f,τ)",
        IgPrim::gag       => "f∨g∨h",
        IgPrim::ooze    => "f→all(x) ∧ broadcast(x,f)",
        // ── Phi ──
        IgPrim::woe    => "¬∃ξ(diverges(ξ))",
        IgPrim::monad      => "ξ→∞ ∧ μ∘δ=id",
        IgPrim::roar => "ξ∈ℂ ∧ Im(ξ)→∞",
        IgPrim::err     => "H=H₀+λV ∧ λ∈EP",
        IgPrim::haha  => "ξ→∞ ∧ ¬(μ∘δ=id)",
        // ── H ──
        IgPrim::fee         => "∀x(P(x)↔P(S(x)))",
        IgPrim::sure         => "∃y∃z(y∈x∧z∈y∧¬z∈x ∧ rank(z)<rank(y))",
        IgPrim::kick         => "∃y(y∈x∧P(y)↔¬P(S(y)))",
        IgPrim::wool      => "∀n∃φ(rank(φ)>n ∧ φ fixed by μ∘δ ∧ φ∈V)",
        // ── S ──
        IgPrim::hung       => "|A|=1 ∧ |B|=1",
        IgPrim::so       => "|A|=n ∧ |B|=n ∧ ∀a∈A∃!b∈B",
        IgPrim::up       => "∃a∈A∃b∈B(type(a)≠type(b))",
        // ── Omega ──
        IgPrim::awe    => "∮_γ dx = 0",
        IgPrim::ah    => "∮_γ A = 2πn ∧ n∈ℤ ∧ wind(γ)≠0",
        IgPrim::oak   => "∮_γ A = πn ∧ n∈ℤ₂",
        IgPrim::zoo   => "Braid(σ_i) ∧ R_matrix≠0 ∧ nonAbelian(x)",
    }
}

// ═══════════════════════════════════════════════════════════════
// PROMOTION CHANNELS — ZFC→ZFCₜ→ZFCfe→CLINK L8
// ═══════════════════════════════════════════════════════════════

/// A promotion channel: source primitive → target primitive with ordinal gap.
#[derive(Copy, Clone, Debug)]
pub struct PromotionChannel {
    pub name: &'static str,
    pub zfc_prim: IgPrim,
    pub promoted_prim: IgPrim,
    /// Ordinal gap weight for distance computation.
    pub ordinal_gap: f32,
}

/// The 6 ZFC→ZFCₜ promotion channels.
pub static ZFC_PROMOTIONS: [PromotionChannel; 6] = [
    PromotionChannel { name: "HOLOBOUND", zfc_prim: IgPrim::judge,    promoted_prim: IgPrim::are,  ordinal_gap: 4.382 },
    PromotionChannel { name: "LR_DUAL",   zfc_prim: IgPrim::ado,  promoted_prim: IgPrim::ian,    ordinal_gap: 3.000 },
    PromotionChannel { name: "PM_Z2",     zfc_prim: IgPrim::church,   promoted_prim: IgPrim::out,    ordinal_gap: 2.000 },
    PromotionChannel { name: "SEQAX",     zfc_prim: IgPrim::vow,    promoted_prim: IgPrim::measure,   ordinal_gap: 2.191 },
    PromotionChannel { name: "TEMPD2",    zfc_prim: IgPrim::fee,       promoted_prim: IgPrim::sure,      ordinal_gap: 2.191 },
    PromotionChannel { name: "ZWIND",     zfc_prim: IgPrim::awe,  promoted_prim: IgPrim::ah, ordinal_gap: 2.191 },
];

/// The 2 additional ZFCfe→CLINK L8 transcendence channels.
pub static CLINK_TRANSCENDENCE: [PromotionChannel; 2] = [
    PromotionChannel { name: "BROADCAST", zfc_prim: IgPrim::measure,   promoted_prim: IgPrim::ooze, ordinal_gap: 1.0 },
    PromotionChannel { name: "NONABELIAN",zfc_prim: IgPrim::ah, promoted_prim: IgPrim::zoo, ordinal_gap: 1.0 },
];

/// All 8 promotion channels (6 ZFCₜ + 2 CLINK).
pub fn all_promotions() -> [PromotionChannel; 8] {
    let mut result = [ZFC_PROMOTIONS[0]; 8];
    for i in 0..6 { result[i] = ZFC_PROMOTIONS[i]; }
    result[6] = CLINK_TRANSCENDENCE[0];
    result[7] = CLINK_TRANSCENDENCE[1];
    result
}

/// Count how many ZFCₜ promotions are present in a tuple.
pub fn count_zfc_promotions(t: &IgTuple) -> u8 {
    let mut count = 0u8;
    for promo in &ZFC_PROMOTIONS {
        if promo.is_present(t) { count += 1; }
    }
    count
}

impl PromotionChannel {
    /// Check if this promotion is fulfilled in the given tuple.
    pub fn is_present(&self, t: &IgTuple) -> bool {
        // The promoted primitive must be at the target value
        match self.name {
            "HOLOBOUND" => t.t == self.promoted_prim,
            "LR_DUAL"   => t.r == self.promoted_prim,
            "PM_Z2"     => t.p == self.promoted_prim,
            "SEQAX"     => t.c == self.promoted_prim,
            "TEMPD2"    => t.h == self.promoted_prim,
            "ZWIND"     => t.omega == self.promoted_prim,
            "BROADCAST" => t.c == self.promoted_prim,
            "NONABELIAN"=> t.omega == self.promoted_prim,
            _ => false,
        }
    }

    /// Which primitive family this promotion targets.
    pub fn target_family(&self) -> u8 {
        match self.name {
            "HOLOBOUND" => 1,  // T
            "LR_DUAL"   => 2,  // R
            "PM_Z2"     => 3,  // P
            "SEQAX"     => 7,  // C
            "TEMPD2"    => 10, // H
            "ZWIND"     => 11, // Omega
            "BROADCAST" => 7,  // C
            "NONABELIAN"=> 11, // Omega
            _ => 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// SHAVIAN GLYPH & SHORT NAME TABLES
// ═══════════════════════════════════════════════════════════════

/// Return the Shavian glyph for any primitive value.
/// This is the canonical mapping — used by IgPrim::glyph().
pub fn primitive_glyph(prim: IgPrim) -> &'static str {
    match prim {
        IgPrim::if_ => "𐑦", IgPrim::dead => "𐑛",
        IgPrim::ash => "𐑨", IgPrim::array => "𐑼",
        IgPrim::are => "𐑸", IgPrim::judge => "𐑡",
        IgPrim::eat => "𐑰", IgPrim::mime => "𐑥",
        IgPrim::oil => "𐑶",
        IgPrim::ian => "𐑾", IgPrim::ear => "𐑽",
        IgPrim::tot => "𐑑", IgPrim::ado => "𐑩",
        IgPrim::or_ => "𐑹", IgPrim::nun => "𐑯",
        IgPrim::out => "𐑬", IgPrim::yew => "𐑿",
        IgPrim::church => "𐑗",
        IgPrim::peep => "𐑐", IgPrim::age => "𐑱",
        IgPrim::they => "𐑞",
        IgPrim::on => "𐑪", IgPrim::egg => "𐑧",
        IgPrim::loll => "𐑤", IgPrim::yea => "𐑘",
        IgPrim::air => "𐑺",
        IgPrim::ice => "𐑲", IgPrim::bib => "𐑚",
        IgPrim::thigh => "𐑔",
        IgPrim::measure => "𐑠", IgPrim::vow => "𐑝",
        IgPrim::gag => "𐑜", IgPrim::ooze => "𐑵",
        IgPrim::monad => "⊙", IgPrim::roar => "𐑮",
        IgPrim::err => "𐑻", IgPrim::woe => "𐑢",
        IgPrim::haha => "𐑣",
        IgPrim::wool => "𐑫", IgPrim::sure => "𐑖",
        IgPrim::kick => "𐑒", IgPrim::fee => "𐑓",
        IgPrim::up => "𐑳", IgPrim::so => "𐑕",
        IgPrim::hung => "𐑙",
        IgPrim::ah => "𐑭", IgPrim::oak => "𐑴",
        IgPrim::awe => "𐑷", IgPrim::zoo => "𐑟",
    }
}
/// The inverse of `primitive_glyph`: a glyph back to its primitive.
///
/// Built from the same pairs as the forward table and kept beside it, because a
/// glyph table with only one direction is why `imasm write` and `imasm derive`
/// were promised in the agent rider for months and never existed — nothing
/// could read a tuple back out of its glyphs.
pub fn primitive_from_glyph(g: &str) -> Option<IgPrim> {
    match g {
        "𐑦" => Some(IgPrim::if_),
        "𐑛" => Some(IgPrim::dead),
        "𐑨" => Some(IgPrim::ash),
        "𐑼" => Some(IgPrim::array),
        "𐑸" => Some(IgPrim::are),
        "𐑡" => Some(IgPrim::judge),
        "𐑰" => Some(IgPrim::eat),
        "𐑥" => Some(IgPrim::mime),
        "𐑶" => Some(IgPrim::oil),
        "𐑾" => Some(IgPrim::ian),
        "𐑽" => Some(IgPrim::ear),
        "𐑑" => Some(IgPrim::tot),
        "𐑩" => Some(IgPrim::ado),
        "𐑹" => Some(IgPrim::or_),
        "𐑯" => Some(IgPrim::nun),
        "𐑬" => Some(IgPrim::out),
        "𐑿" => Some(IgPrim::yew),
        "𐑗" => Some(IgPrim::church),
        "𐑐" => Some(IgPrim::peep),
        "𐑱" => Some(IgPrim::age),
        "𐑞" => Some(IgPrim::they),
        "𐑪" => Some(IgPrim::on),
        "𐑧" => Some(IgPrim::egg),
        "𐑤" => Some(IgPrim::loll),
        "𐑘" => Some(IgPrim::yea),
        "𐑺" => Some(IgPrim::air),
        "𐑲" => Some(IgPrim::ice),
        "𐑚" => Some(IgPrim::bib),
        "𐑔" => Some(IgPrim::thigh),
        "𐑠" => Some(IgPrim::measure),
        "𐑝" => Some(IgPrim::vow),
        "𐑜" => Some(IgPrim::gag),
        "𐑵" => Some(IgPrim::ooze),
        "⊙" => Some(IgPrim::monad),
        "𐑮" => Some(IgPrim::roar),
        "𐑻" => Some(IgPrim::err),
        "𐑢" => Some(IgPrim::woe),
        "𐑣" => Some(IgPrim::haha),
        "𐑫" => Some(IgPrim::wool),
        "𐑖" => Some(IgPrim::sure),
        "𐑒" => Some(IgPrim::kick),
        "𐑓" => Some(IgPrim::fee),
        "𐑳" => Some(IgPrim::up),
        "𐑕" => Some(IgPrim::so),
        "𐑙" => Some(IgPrim::hung),
        "𐑭" => Some(IgPrim::ah),
        "𐑴" => Some(IgPrim::oak),
        "𐑷" => Some(IgPrim::awe),
        "𐑟" => Some(IgPrim::zoo),
        _ => None,
    }
}


/// Return the short display name for any primitive value.
pub fn primitive_short(prim: IgPrim) -> &'static str {
    match prim {
        IgPrim::if_ => "⊢_⊙", IgPrim::dead => "⊢_∨",
        IgPrim::ash => "⊢_△", IgPrim::array => "⊢_∞",
        IgPrim::are => "⊣_⊙", IgPrim::judge => "⊣_net",
        IgPrim::eat => "⊣_in", IgPrim::mime => "⊣_bow",
        IgPrim::oil => "⊣_⊠",
        IgPrim::ian => ">_lr", IgPrim::ear => ">_†",
        IgPrim::tot => ">_cat", IgPrim::ado => ">_sup",
        IgPrim::or_ => "<_⊙", IgPrim::nun => "<_sym",
        IgPrim::out => "<_±", IgPrim::yew => "<_ψ",
        IgPrim::church => "<_∅",
        IgPrim::peep => "⋈_ℏ", IgPrim::age => "⋈_ℓ",
        IgPrim::they => "⋈_ð",
        IgPrim::on => "⊤_trap", IgPrim::egg => "⊤_↓",
        IgPrim::loll => "⊤_~", IgPrim::yea => "⊤_↑",
        IgPrim::air => "⊤_MBL",
        IgPrim::ice => "∈_univ", IgPrim::bib => "∈_loc",
        IgPrim::thigh => "∈_meso",
        IgPrim::measure => "∋_seq", IgPrim::vow => "∋_∧",
        IgPrim::gag => "∋_∨", IgPrim::ooze => "∋_⊛",
        IgPrim::monad => "⊙_⊙", IgPrim::roar => "⊙_ℂ",
        IgPrim::err => "⊙_EP", IgPrim::woe => "⊙_sub",
        IgPrim::haha => "⊙_sup",
        IgPrim::wool => "⊥_∞", IgPrim::sure => "⊥_2",
        IgPrim::kick => "⊥_1", IgPrim::fee => "⊥_0",
        IgPrim::up => "⊞_n:m", IgPrim::so => "⊞_n:n",
        IgPrim::hung => "⊞_1:1",
        IgPrim::ah => "⊡_ℤ", IgPrim::oak => "⊡_ℤ₂",
        IgPrim::awe => "⊡_0", IgPrim::zoo => "⊡_NA",
    }
}

/// Return the primitive family name for a primitive value.
pub fn primitive_family(prim: IgPrim) -> &'static str {
    match prim {
        IgPrim::if_ | IgPrim::dead | IgPrim::ash | IgPrim::array => "⊢",
        IgPrim::are | IgPrim::judge | IgPrim::eat | IgPrim::mime | IgPrim::oil => "⊣",
        IgPrim::ian | IgPrim::ear | IgPrim::tot | IgPrim::ado => "≻",
        IgPrim::or_ | IgPrim::nun | IgPrim::out | IgPrim::yew | IgPrim::church => "≺",
        IgPrim::peep | IgPrim::age | IgPrim::they => "⋈",
        IgPrim::on | IgPrim::egg | IgPrim::loll | IgPrim::yea | IgPrim::air => "⊤",
        IgPrim::ice | IgPrim::bib | IgPrim::thigh => "∈",
        IgPrim::measure | IgPrim::vow | IgPrim::gag | IgPrim::ooze => "∋",
        IgPrim::monad | IgPrim::roar | IgPrim::err | IgPrim::woe | IgPrim::haha => "⊙",
        IgPrim::wool | IgPrim::sure | IgPrim::kick | IgPrim::fee => "⊥",
        IgPrim::up | IgPrim::so | IgPrim::hung => "⊞",
        IgPrim::ah | IgPrim::oak | IgPrim::awe | IgPrim::zoo => "⊡",
    }
}

/// Return the ordinal table for a primitive family.
pub fn ordinal_table(family: &str) -> &'static [IgPrim] {
    match family {
        "⊢" => &D_ORD, "⊣" => &T_ORD, "≻" => &R_ORD,
        "≺" => &P_ORD, "⋈" => &F_ORD, "⊤" => &K_ORD,
        "∈" => &G_ORD, "∋" => &C_ORD, "⊙" => &PHI_ORD,
        "⊥" => &H_ORD, "⊞" => &S_ORD, "⊡" => &OMEGA_ORD,
        _ => &D_ORD,
    }
}
