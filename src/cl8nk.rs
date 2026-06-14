#![allow(dead_code)]
// cl8nk.rs — Full CL8NK Navigator (CLINK Layer 8 — Organism)
//
// CATALOG-NATIVE: All structural data sourced from catalog.rs.
// Matches the Python cl8nk_navigator.py feature-for-feature.
//
// CLINK L8 canonical: ⟨𐑦⋅𐑸⋅𐑾⋅𐑹⋅𐑐⋅𐑧⋅𐑲⋅𐑵⋅⊙⋅𐑫⋅𐑳⋅𐑟⟩
// O_∞⁺ terminal ontological layer. Exceeds ZFC_fe at Ω/ɢ.
//
// Actions:
//   entry  <name>    — Full CL8NK formula decomposition
//   promotions        — 3-stage ladder: ZFC→ZFCₜ→ZFC_fe→CLINK L8
//   distance <name>   — d(name, CLINK L8) + per-primitive conflicts
//   transcendence     — Ω/ɢ transcendence analysis
//   tensor  <name>    — CLINK L8 ⊗ name (absorption test)
//   meet    <name>    — CLINK L8 ⊓ name (shared structural floor)
//   join    <name>    — CLINK L8 ⊔ name (minimal ceiling)
//   tier    <name>    — Ouroboricity tier assessment
//   chain             — Full CLINK chain L0→L8 distance ladder
//   systems           — All catalog systems
//   stats             — Catalog statistics + reference tuples

use crate::imas_ig::{IgPrim, IgTuple};
use crate::catalog;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
// ═══════════════════════════════════════════════════════════════
// CL8NK REFERENCE — single source of truth
// ═══════════════════════════════════════════════════════════════

/// Get the CLINK L8 reference tuple from catalog.
pub fn cl8nk_ref() -> IgTuple { catalog::clink_l8_tuple() }

/// Get the ZFC_fe reference tuple from catalog.
pub fn zfc_fe_ref() -> IgTuple { catalog::zfc_fe_tuple() }

/// Get the ZFCₜ reference tuple from catalog.
pub fn zfc_t_ref() -> IgTuple { catalog::zfc_t_tuple() }

/// Get the ZFC baseline tuple (O₀ floor).
pub fn zfc_baseline_ref() -> IgTuple { catalog::zfc_baseline_tuple() }

// ═══════════════════════════════════════════════════════════════
// PRIMITIVE KEY NAMES
// ═══════════════════════════════════════════════════════════════

pub static PRIMITIVE_KEYS: [&str; 12] = ["D","T","R","P","F","K","G","C","Φ","H","S","Ω"];

/// Get a primitive value from a tuple by key name.
pub fn get_prim(t: &IgTuple, key: &str) -> Option<IgPrim> {
    match key {
        "D" => Some(t.d), "T" => Some(t.t), "R" => Some(t.r),
        "P" => Some(t.p), "F" => Some(t.f), "K" => Some(t.k),
        "G" => Some(t.g), "C" => Some(t.c), "Φ" => Some(t.phi),
        "H" => Some(t.h), "S" => Some(t.s), "Ω" => Some(t.omega),
        _ => None,
    }
}

/// Get the ordinal table for a primitive family by key.
pub fn ord_table_for(key: &str) -> &'static [IgPrim] {
    match key {
        "D" => &catalog::D_ORD, "T" => &catalog::T_ORD,
        "R" => &catalog::R_ORD, "P" => &catalog::P_ORD,
        "F" => &catalog::F_ORD, "K" => &catalog::K_ORD,
        "G" => &catalog::G_ORD, "C" => &catalog::C_ORD,
        "Φ" => &catalog::PHI_ORD, "H" => &catalog::H_ORD,
        "S" => &catalog::S_ORD, "Ω" => &catalog::OMEGA_ORD,
        _ => &catalog::D_ORD,
    }
}

// ═══════════════════════════════════════════════════════════════
// WEIGHTED DISTANCE — matching Python compute_distance
// ═══════════════════════════════════════════════════════════════

/// Per-primitive weight + max delta for normalized distance.
pub struct DistSpec { pub weight: f32, pub max_delta: f32 }

pub static DIST_SPECS: [(&str, DistSpec); 12] = [
    ("D", DistSpec { weight: 0.8, max_delta: 3.0 }),
    ("T", DistSpec { weight: 0.9, max_delta: 4.0 }),
    ("R", DistSpec { weight: 0.7, max_delta: 3.0 }),
    ("P", DistSpec { weight: 0.9, max_delta: 4.0 }),
    ("F", DistSpec { weight: 0.6, max_delta: 2.0 }),
    ("K", DistSpec { weight: 0.7, max_delta: 3.5 }),
    ("G", DistSpec { weight: 0.6, max_delta: 2.0 }),
    ("C", DistSpec { weight: 0.8, max_delta: 3.0 }),
    ("Φ", DistSpec { weight: 1.0, max_delta: 2.0 }),
    ("H", DistSpec { weight: 0.9, max_delta: 3.0 }),
    ("S", DistSpec { weight: 0.5, max_delta: 2.0 }),
    ("Ω", DistSpec { weight: 0.7, max_delta: 3.0 }),
];

/// Normalized ordinal distance between two primitive values.
pub fn ordinal_distance(key: &str, v1: IgPrim, v2: IgPrim) -> f32 {
    let table = ord_table_for(key);
    let i1 = catalog::ord_index(table, v1).unwrap_or(0) as f32;
    let i2 = catalog::ord_index(table, v2).unwrap_or(0) as f32;
    let max_d = DIST_SPECS.iter().find(|(k,_)| *k == key).map(|(_,s)| s.max_delta).unwrap_or(3.0);
    (i2 - i1).abs() / max_d
}

/// A single conflict entry.
#[derive(Clone, Debug)]
pub struct Conflict {
    pub primitive: &'static str,
    pub cl8nk_val: IgPrim,
    pub sys_val: IgPrim,
    pub delta: f32,
}

/// Simple sqrt via Newton's method (no_std, no libm).
fn sqrt_f32(x: f32) -> f32 {
    if x <= 0.0 { return 0.0; }
    let mut y = x;
    let mut prev;
    loop {
        prev = y;
        y = 0.5 * (y + x / y);
        if (y - prev).abs() < 1e-6 { break; }
    }
    y
}

/// Weighted Euclidean distance between two tuples (matching Python algorithm).
pub fn tuple_distance_cl8nk(t1: &IgTuple, t2: &IgTuple) -> (f32, Vec<Conflict>) {
    let mut total: f32 = 0.0;
    let mut conflicts: Vec<Conflict> = Vec::new();
    for (key, spec) in &DIST_SPECS {
        let v1 = get_prim(t1, key).unwrap_or(IgPrim::D_wedge);
        let v2 = get_prim(t2, key).unwrap_or(IgPrim::D_wedge);
        if v1 != v2 {
            let d = ordinal_distance(key, v1, v2);
            total += spec.weight * d * d;
            conflicts.push(Conflict {
                primitive: key,
                cl8nk_val: v2,
                sys_val: v1,
                delta: d,
            });
        }
    }
    (sqrt_f32(total), conflicts)
}// ═══════════════════════════════════════════════════════════════
// TIER ASSESSMENT
// ═══════════════════════════════════════════════════════════════

pub fn assess_tier(t: &IgTuple) -> &'static str {
    let mut score: u8 = 0;
    if t.phi == IgPrim::Phi_c { score += 1; }
    if t.p == IgPrim::P_pmsym { score += 1; }
    if t.h == IgPrim::H_inf { score += 1; }
    if t.omega == IgPrim::Omega_z || t.omega == IgPrim::Omega_na { score += 1; }
    if t.d == IgPrim::D_odot { score += 1; }
    if t.k == IgPrim::K_slow { score += 1; }
    if t.t == IgPrim::T_odot { score += 1; }
    if t.r == IgPrim::R_lr { score += 1; }
    match score {
        s if s >= 7 => "O_∞",
        s if s >= 5 => "O₂",
        s if s >= 3 => "O₁",
        _ => "O₀",
    }
}

// ═══════════════════════════════════════════════════════════════
// FORMULA GENERATION — full CL8NK decomposition
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PrimFragment {
    pub primitive: &'static str,
    pub value_glyph: &'static str,
    pub clink_fragment: &'static str,
    pub promoted_atom: Option<&'static str>,
    pub proximity: &'static str,
}

#[derive(Clone, Debug)]
pub struct AtomDetail {
    pub atom: &'static str,
    pub primitive: &'static str,
    pub value_glyph: &'static str,
    pub clink_fragment: &'static str,
    pub is_transcendence: bool,
}

#[derive(Clone, Debug)]
pub struct PromoNeeded {
    pub primitive: &'static str,
    pub from_glyph: &'static str,
    pub to_glyph: &'static str,
    pub gap: f32,
}

#[derive(Clone, Debug)]
pub struct EntryResult {
    pub system_name: String,
    pub description: String,
    pub fragments: Vec<PrimFragment>,
    pub full_formula: String,
    pub promoted_atom_count: usize,
    pub promoted_atoms: Vec<&'static str>,
    pub atom_details: Vec<AtomDetail>,
    pub distance: f32,
    pub conflicts: Vec<Conflict>,
    pub tier: &'static str,
    pub match_count: u8,
    pub close_count: u8,
    pub distant_count: u8,
    pub has_transcendence: bool,
    pub transcendence_keys: Vec<&'static str>,
    pub promotions_needed: Vec<PromoNeeded>,
    pub promotions_count: usize,
}

/// Generate full CL8NK formula decomposition for a tuple.
/// Transcendence atoms — primitives where CLINK L8 exceeds ZFC_fe.
pub static TRANSCENDENCE_ATOMS: [&str; 2] = ["BROADCAST_TRANSCENDENCE", "BRAID_TRANSCENDENCE"];

/// Helper: check if a string is in TRANSCENDENCE_ATOMS.
pub fn is_transcendence_atom(s: &str) -> bool {
    TRANSCENDENCE_ATOMS.iter().any(|&a| a == s)
}

/// CL8NK formula table per primitive per value.
/// Returns (CLINK_ZFC_fragment, promoted_atom_or_none, proximity).
/// Per-primitive CLINK formula entry.
#[derive(Clone, Debug)]
pub struct FormulaEntry {
    pub fragment: &'static str,
    pub atom: Option<&'static str>,
    pub proximity: &'static str,
}

pub fn cl8nk_formula(key: &str, val: IgPrim) -> Option<FormulaEntry> {
    match key {
        "D" => match val {
            IgPrim::D_odot     => Some(FormulaEntry { fragment: "V = L(x) ∧ selfmodel(x) ∧ x ∈ V", atom: Some("HOLOGRAPHIC_STATE"), proximity: "match" }),
            IgPrim::D_infty    => Some(FormulaEntry { fragment: "∀n∃y( y ∈ x ∧ rank(y) > n )", atom: None, proximity: "close" }),
            IgPrim::D_triangle => Some(FormulaEntry { fragment: "dim(x) = 2 ∧ sur(x)", atom: None, proximity: "distant" }),
            IgPrim::D_wedge    => Some(FormulaEntry { fragment: "dim(x) = 0 ∧ fin(x)", atom: None, proximity: "distant" }),
            _ => None,
        },
        "T" => match val {
            IgPrim::T_odot     => Some(FormulaEntry { fragment: "bound_⊙(a, f) ∧ Refl(a, f) ∧ holo(x, a)", atom: Some("HOLOBOUND"), proximity: "match" }),
            IgPrim::T_bowtie   => Some(FormulaEntry { fragment: "cross(x, y) ∧ ¬ meet(x, y)", atom: None, proximity: "close" }),
            IgPrim::T_boxtimes => Some(FormulaEntry { fragment: "x ⊠ y ∧ irreducible(x, y)", atom: None, proximity: "distant" }),
            IgPrim::T_net      => Some(FormulaEntry { fragment: "graph(x) ∧ branch(x)", atom: None, proximity: "distant" }),
            IgPrim::T_in       => Some(FormulaEntry { fragment: "x ⊆ y ∧ cont(y)", atom: None, proximity: "distant" }),
            _ => None,
        },
        "R" => match val {
            IgPrim::R_lr     => Some(FormulaEntry { fragment: "lr⇔(x, y) ∧ Θ(x, y) ∧ ¬ Θ(y, x)", atom: Some("LR_DUAL"), proximity: "match" }),
            IgPrim::R_dagger => Some(FormulaEntry { fragment: "f ⊣ g ∧ L Adj(f, g)", atom: None, proximity: "close" }),
            IgPrim::R_cat    => Some(FormulaEntry { fragment: "Fun(x, y) ∧ Nat(y, z) → Fun(x, z)", atom: None, proximity: "distant" }),
            IgPrim::R_super  => Some(FormulaEntry { fragment: "x ↑ y ∧ ¬(y ↑ x)", atom: None, proximity: "distant" }),
            _ => None,
        },
        "P" => match val {
            IgPrim::P_pmsym => Some(FormulaEntry { fragment: "ℤ₂(x) ∧ ∀g∈G( gx = x ) ∧ μ∘δ = id", atom: Some("PM_Z2"), proximity: "match" }),
            IgPrim::P_psi   => Some(FormulaEntry { fragment: "|ψ⟩ = Σ c_i |e_i⟩", atom: None, proximity: "close" }),
            IgPrim::P_pm    => Some(FormulaEntry { fragment: "ℤ₂(x) ∧ ¬(x = -x)", atom: None, proximity: "close" }),
            IgPrim::P_sym   => Some(FormulaEntry { fragment: "∀g∈G( gx = x )", atom: None, proximity: "distant" }),
            IgPrim::P_asym  => Some(FormulaEntry { fragment: "¬∃sym(x)", atom: None, proximity: "distant" }),
            _ => None,
        },
        "F" => match val {
            IgPrim::F_hbar => Some(FormulaEntry { fragment: "ℏ(x) ∧ [x, p] = iℏ", atom: None, proximity: "match" }),
            IgPrim::F_eth  => Some(FormulaEntry { fragment: "Tr(ρ²) < 1 ∧ ρ = Σ p_i |i⟩⟨i|", atom: None, proximity: "close" }),
            IgPrim::F_ell  => Some(FormulaEntry { fragment: "P(x) ∈ {0,1} ∧ det(x)", atom: None, proximity: "distant" }),
            _ => None,
        },
        "K" => match val {
            IgPrim::K_slow => Some(FormulaEntry { fragment: "τ ≫ T ∧ eq(x) ∧ gate_open(x)", atom: None, proximity: "match" }),
            IgPrim::K_mod  => Some(FormulaEntry { fragment: "τ ∼ T ∧ noisy(x)", atom: None, proximity: "close" }),
            IgPrim::K_fast => Some(FormulaEntry { fragment: "τ ≪ T ∧ ∂_t x = f(x)", atom: None, proximity: "distant" }),
            IgPrim::K_trap => Some(FormulaEntry { fragment: "τ = ∞ ∧ ord(x)", atom: None, proximity: "distant" }),
            IgPrim::K_mbl  => Some(FormulaEntry { fragment: "τ = ∞ ∧ dis(x) ∧ MBL", atom: None, proximity: "distant" }),
            _ => None,
        },
        "G" => match val {
            IgPrim::G_aleph => Some(FormulaEntry { fragment: "∀y( y ⊂ x → |y| < |x| )", atom: None, proximity: "match" }),
            IgPrim::G_gimel => Some(FormulaEntry { fragment: "∃y∈x( |y| ∼ |x| )", atom: None, proximity: "close" }),
            IgPrim::G_beth  => Some(FormulaEntry { fragment: "∀y∈x( |y| < |x| )", atom: None, proximity: "distant" }),
            _ => None,
        },
        "C" => match val {
            IgPrim::C_broad => Some(FormulaEntry { fragment: "f → all(x) ∧ broadcast(x, f)", atom: Some("BROADCAST_TRANSCENDENCE"), proximity: "match" }),
            IgPrim::C_seq   => Some(FormulaEntry { fragment: "seq!(f, g) ∧ ⟨→⟩(f, g, τ) ∧ ¬ ⟨→⟩(g, f, τ)", atom: Some("SEQAX"), proximity: "close" }),
            IgPrim::C_or    => Some(FormulaEntry { fragment: "f ∨ g ∨ h", atom: None, proximity: "distant" }),
            IgPrim::C_and   => Some(FormulaEntry { fragment: "f ∧ g ∧ h", atom: None, proximity: "distant" }),
            _ => None,
        },
        "Φ" => match val {
            IgPrim::Phi_c         => Some(FormulaEntry { fragment: "ξ → ∞ ∧ μ∘δ = id", atom: Some("PHI_C"), proximity: "match" }),
            IgPrim::Phi_c_complex => Some(FormulaEntry { fragment: "ξ ∈ ℂ ∧ Im(ξ) → ∞", atom: None, proximity: "close" }),
            IgPrim::Phi_ep        => Some(FormulaEntry { fragment: "H(λ) non-Herm ∧ det(H - λI) = 0 ∧ ∂_λ H = 0", atom: None, proximity: "distant" }),
            IgPrim::Phi_super     => Some(FormulaEntry { fragment: "ξ → ∞ ∧ chaotic(x)", atom: None, proximity: "distant" }),
            IgPrim::Phi_sub       => Some(FormulaEntry { fragment: "¬∃ξ( diverges(ξ) )", atom: None, proximity: "distant" }),
            _ => None,
        },
        "H" => match val {
            IgPrim::H_inf => Some(FormulaEntry { fragment: "∀n∃φ( rank(φ) > n ∧ φ fixed by μ∘δ ∧ φ ∈ V )", atom: Some("ETERNAL_FIXEDPOINT"), proximity: "match" }),
            IgPrim::H2    => Some(FormulaEntry { fragment: "∃y∃z( y ∈ x ∧ z ∈ y ∧ ¬ z ∈ x ∧ rank(z) < rank(y) )", atom: Some("TEMPD2"), proximity: "close" }),
            IgPrim::H1    => Some(FormulaEntry { fragment: "∃y( P(y) ↔ P(S²(y)) )", atom: None, proximity: "distant" }),
            IgPrim::H0    => Some(FormulaEntry { fragment: "∀x( P(x) ↔ P(S(x)) )", atom: None, proximity: "distant" }),
            _ => None,
        },
        "S" => match val {
            IgPrim::S_nm => Some(FormulaEntry { fragment: "∃a∈A∃b∈B( type(a) ≠ type(b) )", atom: None, proximity: "match" }),
            IgPrim::S_nn => Some(FormulaEntry { fragment: "∀a∈A∀b∈B( type(a) = type(b) )", atom: None, proximity: "close" }),
            IgPrim::S_11 => Some(FormulaEntry { fragment: "|A| = 1 ∧ |B| = 1", atom: None, proximity: "distant" }),
            _ => None,
        },
        "Ω" => match val {
            IgPrim::Omega_na => Some(FormulaEntry { fragment: "Braid(σ_i) ∧ R_matrix ≠ 0 ∧ nonAbelian(x)", atom: Some("BRAID_TRANSCENDENCE"), proximity: "match" }),
            IgPrim::Omega_z  => Some(FormulaEntry { fragment: "∮_γ A = 2πn ∧ n ∈ ℤ ∧ wind(γ) ≠ 0", atom: Some("ZWIND"), proximity: "close" }),
            IgPrim::Omega_z2 => Some(FormulaEntry { fragment: "∮_γ A = nπ ∧ n ∈ ℤ₂", atom: None, proximity: "distant" }),
            IgPrim::Omega_0  => Some(FormulaEntry { fragment: "∮_γ dx = 0", atom: None, proximity: "distant" }),
            _ => None,
        },
        _ => None,
    }
}

/// Atom descriptions for legend display.
pub fn atom_desc(atom: &str) -> &'static str {
    match atom {
        "HOLOGRAPHIC_STATE"       => "V=L(x) self-writing state-space — Axiom C (D=𐑦)",
        "HOLOBOUND"               => "holographic bound_⊙/bulk encoding — T=𐑸",
        "LR_DUAL"                 => "lateral relational duality — R=𐑾",
        "PM_Z2"                   => "ℤ₂ parity with Frobenius μ∘δ=id — P=𐑹",
        "SEQAX"                   => "sequentiality axiom, directed time — C=𐑠",
        "PHI_C"                   => "criticality fixed-point ξ→∞ ∧ μ∘δ=id — Φ=⊙",
        "TEMPD2"                  => "chirality-2 asymmetry — H=𐑖",
        "ETERNAL_FIXEDPOINT"      => "∀n∃φ fixed by μ∘δ — Axiom D (H=𐑫)",
        "ZWIND"                   => "integer winding number — Ω=𐑭",
        "BROADCAST_TRANSCENDENCE" => "⬆ broadcast composition — exceeds ZFC_fe SEQAX",
        "BRAID_TRANSCENDENCE"     => "⬆ non-Abelian braiding — exceeds ZFC_fe ZWIND",
        _ => "",
    }
}

pub fn generate_entry_formula(name: &str, desc: &str, t: &IgTuple) -> EntryResult {
    let cl8 = cl8nk_ref();
    let mut fragments: Vec<PrimFragment> = Vec::new();
    let mut promoted_atoms: Vec<&'static str> = Vec::new();
    let mut atom_details: Vec<AtomDetail> = Vec::new();
    let mut match_count: u8 = 0;
    let mut close_count: u8 = 0;
    let mut distant_count: u8 = 0;
    let mut transcendence_keys: Vec<&'static str> = Vec::new();

    for key in &PRIMITIVE_KEYS {
        let val = get_prim(t, key).unwrap_or(IgPrim::D_wedge);
        let glyph = catalog::primitive_glyph(val);
        if let Some(fe) = cl8nk_formula(key, val) {
            let frag = PrimFragment {
                primitive: key,
                value_glyph: glyph,
                clink_fragment: fe.fragment,
                promoted_atom: fe.atom,
                proximity: fe.proximity,
            };
            if let Some(atom) = fe.atom {
                promoted_atoms.push(atom);
                let is_t = is_transcendence_atom(atom);
                atom_details.push(AtomDetail {
                    atom,
                    primitive: key,
                    value_glyph: glyph,
                    clink_fragment: fe.fragment,
                    is_transcendence: is_t,
                });
                if is_t { transcendence_keys.push(key); }
            }
            match fe.proximity {
                "match" => match_count += 1,
                "close" => close_count += 1,
                _ => distant_count += 1,
            }
            fragments.push(frag);
        } else {
            fragments.push(PrimFragment {
                primitive: key,
                value_glyph: glyph,
                clink_fragment: "?",
                promoted_atom: None,
                proximity: "unknown",
            });
            distant_count += 1;
        }
    }

    // Build full conjunction
    let mut full_parts: Vec<&str> = Vec::new();
    for f in &fragments {
        full_parts.push(f.clink_fragment);
    }
    let full_formula = full_parts.join(" ∧\n    ");

    let (d, conflicts) = tuple_distance_cl8nk(t, &cl8);
    let tier = assess_tier(t);

    // Promotions needed
    let mut promos: Vec<PromoNeeded> = Vec::new();
    for key in &PRIMITIVE_KEYS {
        let v1 = get_prim(t, key).unwrap_or(IgPrim::D_wedge);
        let v2 = get_prim(&cl8, key).unwrap_or(IgPrim::D_wedge);
        if v1 != v2 {
            promos.push(PromoNeeded {
                primitive: key,
                from_glyph: catalog::primitive_glyph(v1),
                to_glyph: catalog::primitive_glyph(v2),
                gap: ordinal_distance(key, v1, v2),
            });
        }
    }

    EntryResult {
        system_name: String::from(name),
        description: String::from(desc),
        fragments,
        full_formula,
        promoted_atom_count: promoted_atoms.len(),
        promoted_atoms,
        atom_details,
        distance: d,
        conflicts,
        tier,
        match_count,
        close_count,
        distant_count,
        has_transcendence: !transcendence_keys.is_empty(),
        transcendence_keys,
        promotions_count: promos.len(),
        promotions_needed: promos,
    }
}// ═══════════════════════════════════════════════════════════════
// TENSOR / MEET / JOIN — lattice operations with CLINK L8
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct TensorResult {
    pub tuple: IgTuple,
    pub distance_from_cl8nk: f32,
    pub absorbed: bool,
    pub interpretation: &'static str,
}

pub fn compute_tensor_op(sys: &IgTuple) -> TensorResult {
    let cl8 = cl8nk_ref();
    let mut result = cl8;
    for key in &PRIMITIVE_KEYS {
        let table = ord_table_for(key);
        let v_ref = get_prim(&cl8, key).unwrap_or(IgPrim::D_wedge);
        let v_sys = get_prim(sys, key).unwrap_or(IgPrim::D_wedge);
        let i_ref = catalog::ord_index(table, v_ref).unwrap_or(0);
        let i_sys = catalog::ord_index(table, v_sys).unwrap_or(0);
        // For P and F: min; for others: max
        if key == &"P" || key == &"F" {
            let v = if i_sys <= i_ref { v_sys } else { v_ref };
            match key {
                &"P" => result.p = v,
                &"F" => result.f = v,
                _ => {}
            }
        } else {
            let v = if i_ref >= i_sys { v_ref } else { v_sys };
            match key {
                &"D" => result.d = v, &"T" => result.t = v,
                &"R" => result.r = v, &"K" => result.k = v,
                &"G" => result.g = v, &"C" => result.c = v,
                &"Φ" => result.phi = v, &"H" => result.h = v,
                &"S" => result.s = v, &"Ω" => result.omega = v,
                _ => {}
            }
        }
    }
    let (d, _) = tuple_distance_cl8nk(&result, &cl8);
    let absorbed = d == 0.0;
    TensorResult {
        tuple: result,
        distance_from_cl8nk: d,
        absorbed,
        interpretation: if absorbed { "CLINK L8 fully absorbed — strict superset" }
                        else { "d>0 — not fully absorbed" },
    }
}

#[derive(Clone, Debug)]
pub struct MeetJoinResult {
    pub tuple: IgTuple,
    pub d_from_cl8nk: f32,
    pub d_from_system: f32,
}

pub fn compute_meet_op(sys: &IgTuple) -> MeetJoinResult {
    let cl8 = cl8nk_ref();
    let mut result = cl8;
    for key in &PRIMITIVE_KEYS {
        let table = ord_table_for(key);
        let v_ref = get_prim(&cl8, key).unwrap_or(IgPrim::D_wedge);
        let v_sys = get_prim(sys, key).unwrap_or(IgPrim::D_wedge);
        let i_ref = catalog::ord_index(table, v_ref).unwrap_or(0);
        let i_sys = catalog::ord_index(table, v_sys).unwrap_or(0);
        let v = if i_ref <= i_sys { v_ref } else { v_sys };
        match key {
            &"D" => result.d = v, &"T" => result.t = v,
            &"R" => result.r = v, &"P" => result.p = v,
            &"F" => result.f = v, &"K" => result.k = v,
            &"G" => result.g = v, &"C" => result.c = v,
            &"Φ" => result.phi = v, &"H" => result.h = v,
            &"S" => result.s = v, &"Ω" => result.omega = v,
            _ => {}
        }
    }
    let (d_ref, _) = tuple_distance_cl8nk(&result, &cl8);
    let (d_sys, _) = tuple_distance_cl8nk(&result, sys);
    MeetJoinResult { tuple: result, d_from_cl8nk: d_ref, d_from_system: d_sys }
}

pub fn compute_join_op(sys: &IgTuple) -> MeetJoinResult {
    let cl8 = cl8nk_ref();
    let mut result = cl8;
    for key in &PRIMITIVE_KEYS {
        let table = ord_table_for(key);
        let v_ref = get_prim(&cl8, key).unwrap_or(IgPrim::D_wedge);
        let v_sys = get_prim(sys, key).unwrap_or(IgPrim::D_wedge);
        let i_ref = catalog::ord_index(table, v_ref).unwrap_or(0);
        let i_sys = catalog::ord_index(table, v_sys).unwrap_or(0);
        let v = if i_ref >= i_sys { v_ref } else { v_sys };
        match key {
            &"D" => result.d = v, &"T" => result.t = v,
            &"R" => result.r = v, &"P" => result.p = v,
            &"F" => result.f = v, &"K" => result.k = v,
            &"G" => result.g = v, &"C" => result.c = v,
            &"Φ" => result.phi = v, &"H" => result.h = v,
            &"S" => result.s = v, &"Ω" => result.omega = v,
            _ => {}
        }
    }
    let (d_ref, _) = tuple_distance_cl8nk(&result, &cl8);
    let (d_sys, _) = tuple_distance_cl8nk(&result, sys);
    MeetJoinResult { tuple: result, d_from_cl8nk: d_ref, d_from_system: d_sys }
}

// ═══════════════════════════════════════════════════════════════
// TRANSCENDENCE ANALYSIS
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct TranscendenceResult {
    pub d_zfcfe_to_cl8nk: f32,
    pub omega_zfcfe: IgPrim,
    pub omega_cl8nk: IgPrim,
    pub omega_zfcfe_frag: &'static str,
    pub omega_cl8nk_frag: &'static str,
    pub grammar_zfcfe: IgPrim,
    pub grammar_cl8nk: IgPrim,
    pub grammar_zfcfe_frag: &'static str,
    pub grammar_cl8nk_frag: &'static str,
    pub tensor_absorbed: bool,
}

pub fn compute_transcendence() -> TranscendenceResult {
    let zfc_fe = zfc_fe_ref();
    let cl8 = cl8nk_ref();
    let (d, _) = tuple_distance_cl8nk(&zfc_fe, &cl8);
    let omega_zfcfe = zfc_fe.omega;
    let omega_cl8nk = cl8.omega;
    let grammar_zfcfe = zfc_fe.c;
    let grammar_cl8nk = cl8.c;

    let omega_zfcfe_frag = cl8nk_formula("Ω", omega_zfcfe).map(|f| f.fragment).unwrap_or("?");
    let omega_cl8nk_frag = cl8nk_formula("Ω", omega_cl8nk).map(|f| f.fragment).unwrap_or("?");
    let grammar_zfcfe_frag = cl8nk_formula("C", grammar_zfcfe).map(|f| f.fragment).unwrap_or("?");
    let grammar_cl8nk_frag = cl8nk_formula("C", grammar_cl8nk).map(|f| f.fragment).unwrap_or("?");

    let tensor = compute_tensor_op(&zfc_fe);

    TranscendenceResult {
        d_zfcfe_to_cl8nk: d,
        omega_zfcfe, omega_cl8nk,
        omega_zfcfe_frag, omega_cl8nk_frag,
        grammar_zfcfe, grammar_cl8nk,
        grammar_zfcfe_frag, grammar_cl8nk_frag,
        tensor_absorbed: tensor.absorbed,
    }
}// ═══════════════════════════════════════════════════════════════
// PROMOTION LADDER — ZFC → ZFCₜ → ZFC_fe → CLINK L8
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PromoDetail {
    pub primitive: &'static str,
    pub from_glyph: &'static str,
    pub to_glyph: &'static str,
    pub from_fragment: &'static str,
    pub to_fragment: &'static str,
    pub from_atom: Option<&'static str>,
    pub to_atom: Option<&'static str>,
    pub ordinal_gap: f32,
}

#[derive(Clone, Debug)]
pub struct LadderStage {
    pub stage: &'static str,
    pub tier: &'static str,
    pub promotions: usize,
    pub distance: Option<f32>,
    pub details: Vec<PromoDetail>,
    pub note: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct PromotionsResult {
    pub ladder: Vec<LadderStage>,
    pub total_promotions: usize,
    pub total_distance: f32,
    pub transcendence_primitives: Vec<&'static str>,
    pub d_zfcfe_to_cl8nk: f32,
}

pub fn generate_promotions() -> PromotionsResult {
    let zfc_bl = zfc_baseline_ref();
    let zfc_t = zfc_t_ref();
    let zfc_fe = zfc_fe_ref();
    let cl8 = cl8nk_ref();

    fn promo_details(from: &IgTuple, to: &IgTuple) -> Vec<PromoDetail> {
        let mut details: Vec<PromoDetail> = Vec::new();
        for key in &PRIMITIVE_KEYS {
            let v1 = get_prim(from, key).unwrap_or(IgPrim::D_wedge);
            let v2 = get_prim(to, key).unwrap_or(IgPrim::D_wedge);
            if v1 != v2 {
                let f_info = cl8nk_formula(key, v1);
                let t_info = cl8nk_formula(key, v2);
                details.push(PromoDetail {
                    primitive: key,
                    from_glyph: catalog::primitive_glyph(v1),
                    to_glyph: catalog::primitive_glyph(v2),
                    from_fragment: f_info.as_ref().map(|f| f.fragment).unwrap_or("?"),
                    to_fragment: t_info.as_ref().map(|f| f.fragment).unwrap_or("?"),
                    from_atom: f_info.and_then(|f| f.atom),
                    to_atom: t_info.and_then(|f| f.atom),
                    ordinal_gap: ordinal_distance(key, v1, v2),
                });
            }
        }
        details
    }

    let stage1 = promo_details(&zfc_bl, &zfc_t);
    let stage2 = promo_details(&zfc_t, &zfc_fe);
    let stage3 = promo_details(&zfc_fe, &cl8);

    let (d1, _) = tuple_distance_cl8nk(&zfc_bl, &zfc_t);
    let (d2, _) = tuple_distance_cl8nk(&zfc_t, &zfc_fe);
    let (d3, _) = tuple_distance_cl8nk(&zfc_fe, &cl8);
    let (d_total, _) = tuple_distance_cl8nk(&zfc_bl, &cl8);

    let s1_len = stage1.len();
    let s2_len = stage2.len();
    let s3_len = stage3.len();
    PromotionsResult {
        ladder: vec![
            LadderStage {
                stage: "ZFC baseline", tier: "O₀", promotions: 0,
                distance: None, details: vec![], note: None,
            },
            LadderStage {
                stage: "→ ZFCₜ", tier: "O₂†", promotions: s1_len,
                distance: Some(d1), details: stage1, note: None,
            },
            LadderStage {
                stage: "→ ZFC_fe", tier: "O_∞", promotions: s2_len,
                distance: Some(d2), details: stage2, note: None,
            },
            LadderStage {
                stage: "→ CLINK L8", tier: "O_∞⁺", promotions: s3_len,
                distance: Some(d3), details: stage3,
                note: Some("Ω/ɢ TRANSCENDENCE — exceeds Frobenius-exact foundation"),
            },
        ],
        total_promotions: s1_len + s2_len + s3_len,
        total_distance: d_total,
        transcendence_primitives: vec!["Ω", "C"],
        d_zfcfe_to_cl8nk: d3,
    }
}

// ═══════════════════════════════════════════════════════════════
// CHAIN ANALYSIS — dynamically discovered from catalog
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ChainLayer {
    pub name: String,
    pub description: String,
    pub distance_from_l8: f32,
    pub tier: &'static str,
    pub conflicts_count: usize,
}

pub fn chain_analysis() -> Vec<ChainLayer> {
    let cl8 = cl8nk_ref();
    let mut layers: Vec<ChainLayer> = Vec::new();

    // Discover CLINK layers from catalog — matches Python dynamic discovery
    // Searches all known catalog entry names containing "clink_layer" or "clink_l"
    let all_entries: Vec<catalog::CatalogEntry> = {
        let mut v = Vec::new();
        let ref_names = [
            "clink_layer0_frustrated_belnap5",
            "clink_layer1_electron_orbital",
            "clink_layer2_atom",
            "clink_layer3_molecule",
            "clink_layer4_cell",
            "clink_layer5_mitosis",
            "clink_layer6_meiosis",
            "clink_layer7_tissue",
            "clink_l8",
        ];
        for rn in &ref_names {
            if let Some(entry) = catalog::lookup(rn) {
                v.push(entry);
            }
        }
        v
    };

    for entry in &all_entries {
        let (d, conflicts) = tuple_distance_cl8nk(&entry.tuple, &cl8);
        let tier = assess_tier(&entry.tuple);
        layers.push(ChainLayer {
            name: String::from(entry.name),
            description: String::from(entry.description),
            distance_from_l8: d,
            tier,
            conflicts_count: conflicts.len(),
        });
    }

    // Sort by distance from L8 (descending, so L0 first)
    layers.sort_by(|a, b| b.distance_from_l8.partial_cmp(&a.distance_from_l8).unwrap_or(core::cmp::Ordering::Equal));

    layers
}

// ═══════════════════════════════════════════════════════════════
// CATALOG SYSTEMS / STATS
// ═══════════════════════════════════════════════════════════════

pub fn catalog_systems() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    // Collect from the static catalog
    let ref_names = ["zfc", "zfc_t", "zfc_fe", "clink_l8",
                     "clink_layer0_frustrated_belnap5",
                     "clink_layer1_electron_orbital",
                     "clink_layer2_atom",
                     "clink_layer3_molecule",
                     "clink_layer4_cell",
                     "clink_layer5_mitosis",
                     "clink_layer6_meiosis",
                     "clink_layer7_tissue",
                     "temporal_mathematics", "schrodinger", "heat_diffusion",
                     "navier_stokes", "wave_equation", "einstein",
                     "universal_imscriptive_grammar", "o_inf", "o_0", "yhwh"];
    for rn in &ref_names {
        if let Some(_) = catalog::lookup(rn) {
            names.push(String::from(*rn));
        }
    }
    names.sort();
    names
}

pub fn catalog_stats() -> (usize, bool, bool) {
    let count = catalog_systems().len();
    let cl8_found = catalog::lookup("clink_l8").is_some();
    let zfcfe_found = catalog::lookup("zfc_fe").is_some();
    (count, cl8_found, zfcfe_found)
}