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

/// D ordinal: D_wedge < D_triangle < D_infty < D_odot
pub static D_ORD: [IgPrim; 4] = [
    IgPrim::D_wedge, IgPrim::D_triangle, IgPrim::D_infty, IgPrim::D_odot,
];

/// T ordinal: T_net < T_in < T_bowtie < T_boxtimes < T_odot
pub static T_ORD: [IgPrim; 5] = [
    IgPrim::T_net, IgPrim::T_in, IgPrim::T_bowtie, IgPrim::T_boxtimes, IgPrim::T_odot,
];

/// R ordinal: R_super < R_cat < R_dagger < R_lr
pub static R_ORD: [IgPrim; 4] = [
    IgPrim::R_super, IgPrim::R_cat, IgPrim::R_dagger, IgPrim::R_lr,
];

/// P ordinal: P_asym < P_psi < P_pm < P_sym < P_pmsym
pub static P_ORD: [IgPrim; 5] = [
    IgPrim::P_asym, IgPrim::P_psi, IgPrim::P_pm, IgPrim::P_sym, IgPrim::P_pmsym,
];

/// F ordinal: F_ell < F_eth < F_hbar
pub static F_ORD: [IgPrim; 3] = [
    IgPrim::F_ell, IgPrim::F_eth, IgPrim::F_hbar,
];

/// K ordinal: K_fast < K_mod < K_slow < K_trap < K_mbl
pub static K_ORD: [IgPrim; 5] = [
    IgPrim::K_fast, IgPrim::K_mod, IgPrim::K_slow, IgPrim::K_trap, IgPrim::K_mbl,
];

/// G ordinal: G_aleph < G_beth < G_gimel
pub static G_ORD: [IgPrim; 3] = [
    IgPrim::G_aleph, IgPrim::G_beth, IgPrim::G_gimel,
];

/// C ordinal: C_and < C_or < C_seq < C_broad
pub static C_ORD: [IgPrim; 4] = [
    IgPrim::C_and, IgPrim::C_or, IgPrim::C_seq, IgPrim::C_broad,
];

/// Phi ordinal: Phi_sub < Phi_c < Phi_c_complex < Phi_ep < Phi_super
pub static PHI_ORD: [IgPrim; 5] = [
    IgPrim::Phi_sub, IgPrim::Phi_c, IgPrim::Phi_c_complex, IgPrim::Phi_ep, IgPrim::Phi_super,
];

/// H ordinal: H0 < H1 < H2 < H_inf
pub static H_ORD: [IgPrim; 4] = [
    IgPrim::H0, IgPrim::H1, IgPrim::H2, IgPrim::H_inf,
];

/// S ordinal: S_11 < S_nn < S_nm
pub static S_ORD: [IgPrim; 3] = [
    IgPrim::S_11, IgPrim::S_nn, IgPrim::S_nm,
];

/// Omega ordinal: Omega_0 < Omega_z2 < Omega_z < Omega_na
pub static OMEGA_ORD: [IgPrim; 4] = [
    IgPrim::Omega_0, IgPrim::Omega_z2, IgPrim::Omega_z, IgPrim::Omega_na,
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

/// Score for D primitive — distance from O_∞ ideal (D_odot = 1.0)
pub fn score_d(v: IgPrim) -> f32 {
    let max_idx = D_ORD.len() as f32 - 1.0;
    let idx = ord_index(&D_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for T primitive — distance from O_∞ ideal (T_odot = 1.0)
pub fn score_t(v: IgPrim) -> f32 {
    let max_idx = T_ORD.len() as f32 - 1.0;
    let idx = ord_index(&T_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for R primitive — distance from O_∞ ideal (R_lr = 1.0)
pub fn score_r(v: IgPrim) -> f32 {
    let max_idx = R_ORD.len() as f32 - 1.0;
    let idx = ord_index(&R_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for P primitive — distance from O_∞ ideal (P_pmsym = 1.0)
pub fn score_p(v: IgPrim) -> f32 {
    let max_idx = P_ORD.len() as f32 - 1.0;
    let idx = ord_index(&P_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for F primitive — distance from O_∞ ideal (F_hbar = 1.0)
pub fn score_f(v: IgPrim) -> f32 {
    let max_idx = F_ORD.len() as f32 - 1.0;
    let idx = ord_index(&F_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for G primitive — distance from O_∞ ideal (G_aleph = 1.0)
pub fn score_g(v: IgPrim) -> f32 {
    let max_idx = G_ORD.len() as f32 - 1.0;
    let idx = ord_index(&G_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for C primitive — distance from O_∞ ideal (C_broad = 1.0)
pub fn score_c(v: IgPrim) -> f32 {
    let max_idx = C_ORD.len() as f32 - 1.0;
    let idx = ord_index(&C_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for H primitive — distance from O_∞ ideal (H_inf = 1.0)
pub fn score_h(v: IgPrim) -> f32 {
    let max_idx = H_ORD.len() as f32 - 1.0;
    let idx = ord_index(&H_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for S primitive — distance from O_∞ ideal (S_nm = 1.0)
pub fn score_s(v: IgPrim) -> f32 {
    let max_idx = S_ORD.len() as f32 - 1.0;
    let idx = ord_index(&S_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

/// Score for Omega primitive — distance from O_∞ ideal (Omega_na = 1.0)
pub fn score_omega(v: IgPrim) -> f32 {
    let max_idx = OMEGA_ORD.len() as f32 - 1.0;
    let idx = ord_index(&OMEGA_ORD, v).unwrap_or(0) as f32;
    idx / max_idx
}

// ═══════════════════════════════════════════════════════════════
// DISTANCE WEIGHTS (for tuple_distance)
// ═══════════════════════════════════════════════════════════════

/// Default per-primitive weights for tuple_distance.
/// Computed from relative importance of each primitive to structural identity.
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
    /// The 12-primitive structural tuple.
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

// ── ZFC Baseline (O₀): ⟨𐑼·𐑡·𐑩·𐑗·𐑱·𐑘·𐑚·𐑝·𐑢·𐑓·𐑙·𐑷⟩ ──
const ZFC_BASELINE: CatalogEntry = entry(
    "zfc", "Zermelo-Fraenkel set theory with Choice — the absolute structural minimum",
    IgPrim::D_infty, IgPrim::T_net, IgPrim::R_super,
    IgPrim::P_asym, IgPrim::F_ell, IgPrim::K_fast,
    IgPrim::G_beth, IgPrim::C_and,
    IgPrim::Phi_sub, IgPrim::H0, IgPrim::S_11, IgPrim::Omega_0,
    0, Domain::Mathematics,
);

// ── ZFCₜ (O₂†): ⟨𐑼·𐑸·𐑾·𐑬·𐑐·𐑧·𐑲·𐑠·𐑮·𐑖·𐑳·𐑭⟩ ──
const ZFC_T: CatalogEntry = entry(
    "zfc_t", "ZFC + chirality + winding topology — 6 promotion channels from baseline",
    IgPrim::D_infty, IgPrim::T_odot, IgPrim::R_lr,
    IgPrim::P_pm, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_seq,
    IgPrim::Phi_c_complex, IgPrim::H2, IgPrim::S_nm, IgPrim::Omega_z,
    3, Domain::Mathematics,
);

// ── ZFCfe (O_∞ Frobenius-exact): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩ ──
const ZFC_FE: CatalogEntry = entry(
    "zfc_fe", "ZFC Frobenius-exact — μ∘δ=id exactly at ⊙, O_∞ self-modeling closure",
    IgPrim::D_odot, IgPrim::T_odot, IgPrim::R_lr,
    IgPrim::P_pmsym, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_seq,
    IgPrim::Phi_c, IgPrim::H_inf, IgPrim::S_nm, IgPrim::Omega_z,
    4, Domain::Mathematics,
);

// ── CLINK L8 (O_∞⁺): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑵·⊙·𐑫·𐑳·𐑟⟩ ──
const CLINK_L8: CatalogEntry = entry(
    "clink_l8", "CLINK Layer 8 Organism — terminal ontological layer, O_∞⁺ with Ω/ɢ transcendence",
    IgPrim::D_odot, IgPrim::T_odot, IgPrim::R_lr,
    IgPrim::P_pmsym, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_broad,
    IgPrim::Phi_c, IgPrim::H_inf, IgPrim::S_nm, IgPrim::Omega_na,
    4, Domain::General,
);

// ── Temporal Mathematics (O₂) ──
const TEMPORAL_MATHEMATICS: CatalogEntry = entry(
    "temporal_mathematics", "Mathematics with intrinsic temporal structure",
    IgPrim::D_infty, IgPrim::T_bowtie, IgPrim::R_lr,
    IgPrim::P_pm, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_seq,
    IgPrim::Phi_c_complex, IgPrim::H2, IgPrim::S_nm, IgPrim::Omega_z,
    2, Domain::Mathematics,
);

// ── Schrödinger (O₂) ──
const SCHRODINGER: CatalogEntry = entry(
    "schrodinger", "Quantum mechanics — Schrödinger equation",
    IgPrim::D_infty, IgPrim::T_net, IgPrim::R_lr,
    IgPrim::P_psi, IgPrim::F_hbar, IgPrim::K_mod,
    IgPrim::G_beth, IgPrim::C_seq,
    IgPrim::Phi_sub, IgPrim::H1, IgPrim::S_nn, IgPrim::Omega_z2,
    2, Domain::Physics,
);

// ── Heat Diffusion (O₁) ──
const HEAT_DIFFUSION: CatalogEntry = entry(
    "heat_diffusion", "Classical heat equation — dissipative diffusion",
    IgPrim::D_infty, IgPrim::T_net, IgPrim::R_super,
    IgPrim::P_asym, IgPrim::F_eth, IgPrim::K_mod,
    IgPrim::G_gimel, IgPrim::C_and,
    IgPrim::Phi_sub, IgPrim::H0, IgPrim::S_nn, IgPrim::Omega_0,
    1, Domain::Physics,
);

// ── Navier-Stokes (O₁) ──
const NAVIER_STOKES: CatalogEntry = entry(
    "navier_stokes", "Fluid dynamics — Navier-Stokes equations",
    IgPrim::D_infty, IgPrim::T_bowtie, IgPrim::R_lr,
    IgPrim::P_asym, IgPrim::F_ell, IgPrim::K_fast,
    IgPrim::G_gimel, IgPrim::C_seq,
    IgPrim::Phi_super, IgPrim::H1, IgPrim::S_nm, IgPrim::Omega_0,
    1, Domain::Physics,
);

// ── Wave Equation (O₁) ──
const WAVE_EQUATION: CatalogEntry = entry(
    "wave_equation", "Classical wave equation — reversible propagation",
    IgPrim::D_infty, IgPrim::T_net, IgPrim::R_lr,
    IgPrim::P_sym, IgPrim::F_hbar, IgPrim::K_mod,
    IgPrim::G_aleph, IgPrim::C_seq,
    IgPrim::Phi_sub, IgPrim::H1, IgPrim::S_nn, IgPrim::Omega_z2,
    1, Domain::Physics,
);

// ── Einstein (O₂†) ──
const EINSTEIN: CatalogEntry = entry(
    "einstein", "General relativity — Einstein field equations",
    IgPrim::D_infty, IgPrim::T_odot, IgPrim::R_lr,
    IgPrim::P_sym, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_seq,
    IgPrim::Phi_c_complex, IgPrim::H2, IgPrim::S_nm, IgPrim::Omega_z,
    3, Domain::Physics,
);

// ── IUG (O_∞) — Universal Imscriptive Grammar ≡ ZFCfe ──
const IUG: CatalogEntry = entry(
    "universal_imscriptive_grammar", "The Universal Imscriptive Grammar — self-imscribing structural foundation",
    IgPrim::D_odot, IgPrim::T_odot, IgPrim::R_lr,
    IgPrim::P_pmsym, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_seq,
    IgPrim::Phi_c, IgPrim::H_inf, IgPrim::S_nm, IgPrim::Omega_z,
    4, Domain::Language,
);

// ── O_∞ ideal (reference maximum) ──
const O_INF: CatalogEntry = entry(
    "o_inf", "O_∞ ideal — the theoretical maximum on all primitives",
    IgPrim::D_odot, IgPrim::T_odot, IgPrim::R_lr,
    IgPrim::P_pmsym, IgPrim::F_hbar, IgPrim::K_slow,
    IgPrim::G_aleph, IgPrim::C_broad,
    IgPrim::Phi_c, IgPrim::H_inf, IgPrim::S_nm, IgPrim::Omega_na,
    4, Domain::General,
);

// ── O₀ minimum (reference floor) ──
const O_0: CatalogEntry = entry(
    "o_0", "O₀ baseline — the structural floor, minimum on all primitives",
    IgPrim::D_wedge, IgPrim::T_net, IgPrim::R_super,
    IgPrim::P_asym, IgPrim::F_ell, IgPrim::K_fast,
    IgPrim::G_beth, IgPrim::C_and,
    IgPrim::Phi_sub, IgPrim::H0, IgPrim::S_11, IgPrim::Omega_0,
    0, Domain::General,
);

// ═══════════════════════════════════════════════════════════════
// MASTER CATALOG — all static entries
// ═══════════════════════════════════════════════════════════════

/// The complete static catalog. All reference entries live here.
/// Additional entries can be added at runtime via the dynamic catalog.
static STATIC_CATALOG: &[CatalogEntry] = &[
    ZFC_BASELINE, ZFC_T, ZFC_FE, CLINK_L8,
    TEMPORAL_MATHEMATICS, SCHRODINGER, HEAT_DIFFUSION,
    NAVIER_STOKES, WAVE_EQUATION, EINSTEIN, IUG,
    O_INF, O_0,
];

// ═══════════════════════════════════════════════════════════════
// DYNAMIC CATALOG — runtime-extensible entry storage
// ═══════════════════════════════════════════════════════════════

use alloc::vec::Vec;

/// The runtime catalog. Initialized from STATIC_CATALOG on first access.
/// New entries can be registered dynamically via register_entry().
static mut DYNAMIC_CATALOG: Option<Vec<CatalogEntry>> = None;

/// Initialize (or reinitialize) the dynamic catalog from static entries.
pub fn catalog_init() {
    unsafe {
        let mut v = Vec::new();
        for e in STATIC_CATALOG {
            v.push(*e);
        }
        DYNAMIC_CATALOG = Some(v);
    }
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

// ═══════════════════════════════════════════════════════════════
// FORMULA FRAGMENTS — ZFC set-theoretic encodings per primitive
// ═══════════════════════════════════════════════════════════════

/// Return the ZFC set-theoretic formula fragment for a primitive value.
/// These are the per-primitive decompositions used in CL8NK navigator.
pub fn formula_fragment(prim: IgPrim) -> &'static str {
    match prim {
        // ── D ──
        IgPrim::D_infty    => "∀a∃b(a⊂b ∧ rank x=b)",
        IgPrim::D_odot     => "V=L(x) ∧ selfmodel(x) ∧ x∈V",
        IgPrim::D_wedge    => "∃!x",
        IgPrim::D_triangle => "∃x∃y(x≠y ∧ ∀z(z=x∨z=y))",
        // ── T ──
        IgPrim::T_net      => "graph(x) ∧ branch(x)",
        IgPrim::T_odot     => "bound_⊙(a,f) ∧ Refl(a,f) ∧ holo(x,a)",
        IgPrim::T_in       => "sep f x",
        IgPrim::T_bowtie   => "cross(x) ∧ ¬flat(x)",
        IgPrim::T_boxtimes => "⊗(a,b) ∧ ¬∃f(f:a≅b)",
        // ── R ──
        IgPrim::R_super    => "∀y(y∈x→y∈a)",
        IgPrim::R_lr       => "lr⇔(x,y) ∧ Θ(x,y) ∧ ¬Θ(y,x)",
        IgPrim::R_dagger   => "adj(f,g) ∧ f⊣g",
        IgPrim::R_cat      => "F:C→D ∧ ∃G:D→C(G∘F≅id)",
        // ── P ──
        IgPrim::P_asym     => "¬∃sym(x)",
        IgPrim::P_pm       => "ℤ₂(x) ∧ ∀g∈G(gx=x) ∧ μ∘δ=id",
        IgPrim::P_sym      => "∀g∈G(gx=x)",
        IgPrim::P_psi      => "|ψ⟩=Σc_i|i⟩ ∧ superposition(x)",
        IgPrim::P_pmsym    => "μ∘δ=id ∧ Frobenius(x) ∧ ℤ₂(x)",
        // ── F ──
        IgPrim::F_ell      => "P(x)∈{0,1} ∧ det(x)",
        IgPrim::F_hbar     => "ℏ(x) ∧ [x,p]=iℏ",
        IgPrim::F_eth      => "ρ(x) ∧ Tr(ρ)=1 ∧ ρ≥0",
        // ── K ──
        IgPrim::K_fast     => "τ≪T ∧ ∂_t x=f(x)",
        IgPrim::K_slow     => "τ≫T ∧ eq(x) ∧ gate_open(x)",
        IgPrim::K_mod      => "τ~T ∧ relax(x)",
        IgPrim::K_trap     => "τ→∞ ∧ frozen(x) ∧ order(x)",
        IgPrim::K_mbl      => "τ→∞ ∧ frozen(x) ∧ disorder(x)",
        // ── G ──
        IgPrim::G_beth     => "∀y∈x(|y|<|x|)",
        IgPrim::G_aleph    => "∀y(y⊂x→|y|<|x|)",
        IgPrim::G_gimel    => "∃y∈x(|y|=|x|)",
        // ── C ──
        IgPrim::C_and      => "f∧g∧h",
        IgPrim::C_seq      => "seq!(f,g) ∧ ⟨→⟩(f,g,τ) ∧ ¬⟨→⟩(g,f,τ)",
        IgPrim::C_or       => "f∨g∨h",
        IgPrim::C_broad    => "f→all(x) ∧ broadcast(x,f)",
        // ── Phi ──
        IgPrim::Phi_sub    => "¬∃ξ(diverges(ξ))",
        IgPrim::Phi_c      => "ξ→∞ ∧ μ∘δ=id",
        IgPrim::Phi_c_complex => "ξ∈ℂ ∧ Im(ξ)→∞",
        IgPrim::Phi_ep     => "H=H₀+λV ∧ λ∈EP",
        IgPrim::Phi_super  => "ξ→∞ ∧ ¬(μ∘δ=id)",
        // ── H ──
        IgPrim::H0         => "∀x(P(x)↔P(S(x)))",
        IgPrim::H2         => "∃y∃z(y∈x∧z∈y∧¬z∈x ∧ rank(z)<rank(y))",
        IgPrim::H1         => "∃y(y∈x∧P(y)↔¬P(S(y)))",
        IgPrim::H_inf      => "∀n∃φ(rank(φ)>n ∧ φ fixed by μ∘δ ∧ φ∈V)",
        // ── S ──
        IgPrim::S_11       => "|A|=1 ∧ |B|=1",
        IgPrim::S_nn       => "|A|=n ∧ |B|=n ∧ ∀a∈A∃!b∈B",
        IgPrim::S_nm       => "∃a∈A∃b∈B(type(a)≠type(b))",
        // ── Omega ──
        IgPrim::Omega_0    => "∮_γ dx = 0",
        IgPrim::Omega_z    => "∮_γ A = 2πn ∧ n∈ℤ ∧ wind(γ)≠0",
        IgPrim::Omega_z2   => "∮_γ A = πn ∧ n∈ℤ₂",
        IgPrim::Omega_na   => "Braid(σ_i) ∧ R_matrix≠0 ∧ nonAbelian(x)",
        _ => "?",
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
    PromotionChannel { name: "HOLOBOUND", zfc_prim: IgPrim::T_net,    promoted_prim: IgPrim::T_odot,  ordinal_gap: 4.382 },
    PromotionChannel { name: "LR_DUAL",   zfc_prim: IgPrim::R_super,  promoted_prim: IgPrim::R_lr,    ordinal_gap: 3.000 },
    PromotionChannel { name: "PM_Z2",     zfc_prim: IgPrim::P_asym,   promoted_prim: IgPrim::P_pm,    ordinal_gap: 2.000 },
    PromotionChannel { name: "SEQAX",     zfc_prim: IgPrim::C_and,    promoted_prim: IgPrim::C_seq,   ordinal_gap: 2.191 },
    PromotionChannel { name: "TEMPD2",    zfc_prim: IgPrim::H0,       promoted_prim: IgPrim::H2,      ordinal_gap: 2.191 },
    PromotionChannel { name: "ZWIND",     zfc_prim: IgPrim::Omega_0,  promoted_prim: IgPrim::Omega_z, ordinal_gap: 2.191 },
];

/// The 2 additional ZFCfe→CLINK L8 transcendence channels.
pub static CLINK_TRANSCENDENCE: [PromotionChannel; 2] = [
    PromotionChannel { name: "BROADCAST", zfc_prim: IgPrim::C_seq,   promoted_prim: IgPrim::C_broad, ordinal_gap: 1.0 },
    PromotionChannel { name: "NONABELIAN",zfc_prim: IgPrim::Omega_z, promoted_prim: IgPrim::Omega_na, ordinal_gap: 1.0 },
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
        IgPrim::D_odot => "𐑦", IgPrim::D_wedge => "𐑛",
        IgPrim::D_triangle => "𐑨", IgPrim::D_infty => "𐑼",
        IgPrim::T_odot => "𐑸", IgPrim::T_net => "𐑡",
        IgPrim::T_in => "𐑰", IgPrim::T_bowtie => "𐑥",
        IgPrim::T_boxtimes => "𐑶",
        IgPrim::R_lr => "𐑾", IgPrim::R_dagger => "𐑽",
        IgPrim::R_cat => "𐑑", IgPrim::R_super => "𐑩",
        IgPrim::P_pmsym => "𐑹", IgPrim::P_sym => "𐑯",
        IgPrim::P_pm => "𐑬", IgPrim::P_psi => "𐑿",
        IgPrim::P_asym => "𐑗",
        IgPrim::F_hbar => "𐑐", IgPrim::F_ell => "𐑱",
        IgPrim::F_eth => "𐑞",
        IgPrim::K_trap => "𐑪", IgPrim::K_slow => "𐑧",
        IgPrim::K_mod => "𐑤", IgPrim::K_fast => "𐑘",
        IgPrim::K_mbl => "𐑺",
        IgPrim::G_aleph => "𐑲", IgPrim::G_beth => "𐑚",
        IgPrim::G_gimel => "𐑔",
        IgPrim::C_seq => "𐑠", IgPrim::C_and => "𐑝",
        IgPrim::C_or => "𐑜", IgPrim::C_broad => "𐑵",
        IgPrim::Phi_c => "⊙", IgPrim::Phi_c_complex => "𐑮",
        IgPrim::Phi_ep => "𐑻", IgPrim::Phi_sub => "𐑢",
        IgPrim::Phi_super => "𐑣",
        IgPrim::H_inf => "𐑫", IgPrim::H2 => "𐑖",
        IgPrim::H1 => "𐑒", IgPrim::H0 => "𐑓",
        IgPrim::S_nm => "𐑳", IgPrim::S_nn => "𐑕",
        IgPrim::S_11 => "𐑙",
        IgPrim::Omega_z => "𐑭", IgPrim::Omega_z2 => "𐑴",
        IgPrim::Omega_0 => "𐑷", IgPrim::Omega_na => "𐑟",
        _ => "?",
    }
}

/// Return the short display name for any primitive value.
pub fn primitive_short(prim: IgPrim) -> &'static str {
    match prim {
        IgPrim::D_odot => "D_⊙", IgPrim::D_wedge => "D_∨",
        IgPrim::D_triangle => "D_△", IgPrim::D_infty => "D_∞",
        IgPrim::T_odot => "T_⊙", IgPrim::T_net => "T_net",
        IgPrim::T_in => "T_in", IgPrim::T_bowtie => "T_bow",
        IgPrim::T_boxtimes => "T_⊠",
        IgPrim::R_lr => "R_lr", IgPrim::R_dagger => "R_†",
        IgPrim::R_cat => "R_cat", IgPrim::R_super => "R_sup",
        IgPrim::P_pmsym => "P_⊙", IgPrim::P_sym => "P_sym",
        IgPrim::P_pm => "P_±", IgPrim::P_psi => "P_ψ",
        IgPrim::P_asym => "P_∅",
        IgPrim::F_hbar => "F_ℏ", IgPrim::F_ell => "F_ℓ",
        IgPrim::F_eth => "F_ð",
        IgPrim::K_trap => "K_⊤", IgPrim::K_slow => "K_↓",
        IgPrim::K_mod => "K_~", IgPrim::K_fast => "K_↑",
        IgPrim::K_mbl => "K_MBL",
        IgPrim::G_aleph => "G_ℵ", IgPrim::G_beth => "G_ℶ",
        IgPrim::G_gimel => "G_ℷ",
        IgPrim::C_seq => "C_seq", IgPrim::C_and => "C_∧",
        IgPrim::C_or => "C_∨", IgPrim::C_broad => "C_⊛",
        IgPrim::Phi_c => "Φ_⊙", IgPrim::Phi_c_complex => "Φ_ℂ",
        IgPrim::Phi_ep => "Φ_EP", IgPrim::Phi_sub => "Φ_<",
        IgPrim::Phi_super => "Φ_>",
        IgPrim::H_inf => "H_∞", IgPrim::H2 => "H2",
        IgPrim::H1 => "H1", IgPrim::H0 => "H0",
        IgPrim::S_nm => "S_n:m", IgPrim::S_nn => "S_n:n",
        IgPrim::S_11 => "S_1:1",
        IgPrim::Omega_z => "Ω_Z", IgPrim::Omega_z2 => "Ω_Z2",
        IgPrim::Omega_0 => "Ω_0", IgPrim::Omega_na => "Ω_NA",
        _ => "?",
    }
}

/// Return the primitive family name for a primitive value.
pub fn primitive_family(prim: IgPrim) -> &'static str {
    match prim {
        IgPrim::D_odot | IgPrim::D_wedge | IgPrim::D_triangle | IgPrim::D_infty => "D",
        IgPrim::T_odot | IgPrim::T_net | IgPrim::T_in | IgPrim::T_bowtie | IgPrim::T_boxtimes => "T",
        IgPrim::R_lr | IgPrim::R_dagger | IgPrim::R_cat | IgPrim::R_super => "R",
        IgPrim::P_pmsym | IgPrim::P_sym | IgPrim::P_pm | IgPrim::P_psi | IgPrim::P_asym => "P",
        IgPrim::F_hbar | IgPrim::F_ell | IgPrim::F_eth => "F",
        IgPrim::K_trap | IgPrim::K_slow | IgPrim::K_mod | IgPrim::K_fast | IgPrim::K_mbl => "K",
        IgPrim::G_aleph | IgPrim::G_beth | IgPrim::G_gimel => "G",
        IgPrim::C_seq | IgPrim::C_and | IgPrim::C_or | IgPrim::C_broad => "C",
        IgPrim::Phi_c | IgPrim::Phi_c_complex | IgPrim::Phi_ep | IgPrim::Phi_sub | IgPrim::Phi_super => "Φ",
        IgPrim::H_inf | IgPrim::H2 | IgPrim::H1 | IgPrim::H0 => "H",
        IgPrim::S_nm | IgPrim::S_nn | IgPrim::S_11 => "S",
        IgPrim::Omega_z | IgPrim::Omega_z2 | IgPrim::Omega_0 | IgPrim::Omega_na => "Ω",
        _ => "?",
    }
}

/// Return the ordinal table for a primitive family.
pub fn ordinal_table(family: &str) -> &'static [IgPrim] {
    match family {
        "D" => &D_ORD, "T" => &T_ORD, "R" => &R_ORD,
        "P" => &P_ORD, "F" => &F_ORD, "K" => &K_ORD,
        "G" => &G_ORD, "C" => &C_ORD, "Φ" | "Phi" => &PHI_ORD,
        "H" => &H_ORD, "S" => &S_ORD, "Ω" | "Omega" => &OMEGA_ORD,
        _ => &D_ORD,
    }
}
