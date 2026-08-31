#![allow(dead_code)]
// cl8nk.rs — Full CL8NK Navigator (CLINK Layer 8 — Organism)
//
// CATALOG-NATIVE: All data sourced from catalog.rs.
// Matches the Python cl8nk_navigator.py feature-for-feature.
//
// CLINK L8 canonical: ⟨𐑦⋅𐑸⋅𐑾⋅𐑹⋅𐑐⋅𐑧⋅𐑲⋅𐑵⋅⊙⋅𐑫⋅𐑳⋅𐑟⟩
// O_∞⁺ terminal ontological layer. Exceeds ZFC_fe at ⊡/∋.
//
// Actions:
//   entry  <name>    — Full CL8NK formula decomposition
//   promotions        — 3-stage ladder: ZFC→ZFCₜ→ZFC_fe→CLINK L8
//   distance <name>   — d(name, CLINK L8) + per-primitive conflicts
//   transcendence     — ⊡/∋ transcendence analysis
//   tensor  <name>    — CLINK L8 ⊗ name (absorption test)
//   meet    <name>    — CLINK L8 ⊓ name (shared floor)
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

/// The CL9NK reference tuple — CLINK L9, the replicative lateral.
pub fn cl9nk_ref() -> IgTuple { catalog::clink_l9_tuple() }

/// Get the ZFC_fe reference tuple from catalog.
pub fn zfc_fe_ref() -> IgTuple { catalog::zfc_fe_tuple() }

/// Get the ZFCₜ reference tuple from catalog.
pub fn zfc_t_ref() -> IgTuple { catalog::zfc_t_tuple() }

/// Get the ZFC baseline tuple (O₀ floor).
pub fn zfc_baseline_ref() -> IgTuple { catalog::zfc_baseline_tuple() }

// ═══════════════════════════════════════════════════════════════
// PRIMITIVE KEY NAMES
// ═══════════════════════════════════════════════════════════════

pub use crate::canonical_ig::PRIMITIVE_ORDER as PRIMITIVE_KEYS;

/// Get a primitive value from a tuple by key name.
pub fn get_prim(t: &IgTuple, key: &str) -> Option<IgPrim> {
    match key {
        "⊢" => Some(t.d), "⊣" => Some(t.t), "≻" => Some(t.r),
        "≺" => Some(t.p), "⋈" => Some(t.f), "⊤" => Some(t.k),
        "∈" => Some(t.g), "∋" => Some(t.c), "⊙" => Some(t.phi),
        "⊥" => Some(t.h), "⊞" => Some(t.s), "⊡" => Some(t.omega),
        _ => None,
    }
}

/// Get the ordinal table for a primitive family by key.
pub fn ord_table_for(key: &str) -> &'static [IgPrim] {
    match key {
        "⊢" => &catalog::D_ORD, "⊣" => &catalog::T_ORD,
        "≻" => &catalog::R_ORD, "≺" => &catalog::P_ORD,
        "⋈" => &catalog::F_ORD, "⊤" => &catalog::K_ORD,
        "∈" => &catalog::G_ORD, "∋" => &catalog::C_ORD,
        "⊙" => &catalog::PHI_ORD, "⊥" => &catalog::H_ORD,
        "⊞" => &catalog::S_ORD, "⊡" => &catalog::OMEGA_ORD,
        _ => &catalog::D_ORD,
    }
}

// ═══════════════════════════════════════════════════════════════
// WEIGHTED DISTANCE — matching Python compute_distance
// ═══════════════════════════════════════════════════════════════

/// Per-primitive weight + max delta for normalized distance.
pub struct DistSpec { pub weight: f32, pub max_delta: f32 }

// Keyed by GLYPH, matching `get_prim` and `ord_table_for`.
//
// These were keyed by letter ("D","T","R",...) while `get_prim` keys by glyph
// ("\u{22a2}","\u{22a3}","≻",...). Only "≺" and "\u{25fb}" existed in both key spaces, so ten of
// the twelve axes silently resolved to None -> IgPrim::dead on BOTH sides,
// compared equal, and contributed nothing: every cl8nk distance in the kernel
// was computed from two axes. The `cl8nk chain` ladder showed it plainly —
// conflicts=2 for every layer, including layers differing in eight primitives.
// Worse, "≺" is Phi's slot in this table but get_prim("≺") returns Parity, so
// the one categorical axis that did count was scored with the wrong weight.
pub static DIST_SPECS: [(&str, DistSpec); 12] = [
    ("\u{22a2}", DistSpec { weight: 0.8, max_delta: 3.0 }),  // D
    ("\u{22a3}", DistSpec { weight: 0.9, max_delta: 4.0 }),  // T
    ("≻",        DistSpec { weight: 0.7, max_delta: 3.0 }),  // R
    ("≺",        DistSpec { weight: 0.9, max_delta: 4.0 }),  // P
    ("\u{22c8}", DistSpec { weight: 0.6, max_delta: 2.0 }),  // F
    ("\u{22a4}", DistSpec { weight: 0.7, max_delta: 3.5 }),  // K
    ("\u{2208}", DistSpec { weight: 0.6, max_delta: 2.0 }),  // G
    ("\u{220b}", DistSpec { weight: 0.8, max_delta: 3.0 }),  // C
    ("\u{2299}", DistSpec { weight: 1.0, max_delta: 2.0 }),  // Phi
    ("\u{22a5}", DistSpec { weight: 0.9, max_delta: 3.0 }),  // H
    ("\u{229e}", DistSpec { weight: 0.5, max_delta: 2.0 }),  // S
    ("\u{25fb}", DistSpec { weight: 0.7, max_delta: 3.0 }),  // Omega
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
        let v1 = get_prim(t1, key).unwrap_or(IgPrim::dead);
        let v2 = get_prim(t2, key).unwrap_or(IgPrim::dead);
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
    if t.phi == IgPrim::monad { score += 1; }
    if t.p == IgPrim::or_ { score += 1; }
    if t.h == IgPrim::wool { score += 1; }
    if t.omega == IgPrim::ah || t.omega == IgPrim::zoo { score += 1; }
    if t.d == IgPrim::if_ { score += 1; }
    if t.k == IgPrim::egg { score += 1; }
    if t.t == IgPrim::are { score += 1; }
    if t.r == IgPrim::ian { score += 1; }
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
    // Read the generated table rather than restating it. The fragments are
    // cl8nk_navigator.CL8NK_FORMULAE, which is the reference; a copy here was
    // one more thing to drift. `key` accepts either the axis mark or the axis's
    // name, and the value is matched by its canonical glyph.
    let axis = if crate::canonical_ig::axis_index(key).is_some() {
        key
    } else {
        let mut found = "";
        for (a, n) in crate::canonical_ig::PRIMITIVE_NAMES.iter() {
            if *n == key { found = a; break; }
        }
        if found.is_empty() { return None; }
        found
    };
    let glyph = crate::catalog::primitive_glyph(val);
    let (fragment, atom, proximity) = crate::canonical_ig::formula_of(axis, glyph)?;
    Some(FormulaEntry {
        fragment,
        atom: if atom.is_empty() { None } else { Some(atom) },
        proximity,
    })
}

/// Atom descriptions for legend display.
pub fn atom_desc(atom: &str) -> &'static str {
    match atom {
        "HOLOGRAPHIC_STATE"       => "V=L(x) self-writing state-space — Axiom C (D=𐑦)",
        "HOLOBOUND"               => "imscriptive bound_⊙/bulk imscription — T=𐑸",
        "LR_DUAL"                 => "lateral relational duality — R=𐑾",
        "PM_Z2"                   => "ℤ₂ parity with Frobenius μ∘δ=id — P=𐑹",
        "SEQAX"                   => "sequentiality axiom, directed time — C=𐑠",
        "PHI_C"                   => "criticality fixed-point ξ→∞ ∧ μ∘δ=id — <=⊙",
        "TEMPD2"                  => "chirality-2 asymmetry — H=𐑖",
        "ETERNAL_FIXEDPOINT"      => "∀n∃φ fixed by μ∘δ — Axiom D (H=𐑫)",
        "ZWIND"                   => "integer winding number — ⊡=𐑭",
        "BROADCAST_TRANSCENDENCE" => "⬆ broadcast composition — exceeds ZFC_fe SEQAX",
        "BRAID_TRANSCENDENCE"     => "⬆ non-Abelian braiding — exceeds ZFC_fe ZWIND",
        _ => "",
    }
}

pub fn generate_entry_formula(name: &str, desc: &str, t: &IgTuple) -> EntryResult {
    generate_entry_formula_against(name, desc, t, &cl8nk_ref())
}

/// The same decomposition, read against ANY reference layer.
///
/// L9 is lateral to L8, not below it, so "how far to L8" is not the only
/// question that can be asked of an entry — and for an object that already sits
/// on the replicative lateral it is the wrong one. Reading against L9 shows the
/// promotions that vanish when the climb is not attempted.
pub fn generate_entry_formula_against(
    name: &str, desc: &str, t: &IgTuple, reference: &IgTuple,
) -> EntryResult {
    let cl8 = reference.clone();
    let mut fragments: Vec<PrimFragment> = Vec::new();
    let mut promoted_atoms: Vec<&'static str> = Vec::new();
    let mut atom_details: Vec<AtomDetail> = Vec::new();
    let mut match_count: u8 = 0;
    let mut close_count: u8 = 0;
    let mut distant_count: u8 = 0;
    let mut transcendence_keys: Vec<&'static str> = Vec::new();

    for key in &PRIMITIVE_KEYS {
        let val = get_prim(t, key).unwrap_or(IgPrim::dead);
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
        let v1 = get_prim(t, key).unwrap_or(IgPrim::dead);
        let v2 = get_prim(&cl8, key).unwrap_or(IgPrim::dead);
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
        let v_ref = get_prim(&cl8, key).unwrap_or(IgPrim::dead);
        let v_sys = get_prim(sys, key).unwrap_or(IgPrim::dead);
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
                &"≺" => result.phi = v, &"H" => result.h = v,
                &"S" => result.s = v, &"⊡" => result.omega = v,
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
        let v_ref = get_prim(&cl8, key).unwrap_or(IgPrim::dead);
        let v_sys = get_prim(sys, key).unwrap_or(IgPrim::dead);
        let i_ref = catalog::ord_index(table, v_ref).unwrap_or(0);
        let i_sys = catalog::ord_index(table, v_sys).unwrap_or(0);
        let v = if i_ref <= i_sys { v_ref } else { v_sys };
        match key {
            &"D" => result.d = v, &"T" => result.t = v,
            &"R" => result.r = v, &"P" => result.p = v,
            &"F" => result.f = v, &"K" => result.k = v,
            &"G" => result.g = v, &"C" => result.c = v,
            &"≺" => result.phi = v, &"H" => result.h = v,
            &"S" => result.s = v, &"⊡" => result.omega = v,
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
        let v_ref = get_prim(&cl8, key).unwrap_or(IgPrim::dead);
        let v_sys = get_prim(sys, key).unwrap_or(IgPrim::dead);
        let i_ref = catalog::ord_index(table, v_ref).unwrap_or(0);
        let i_sys = catalog::ord_index(table, v_sys).unwrap_or(0);
        let v = if i_ref >= i_sys { v_ref } else { v_sys };
        match key {
            &"D" => result.d = v, &"T" => result.t = v,
            &"R" => result.r = v, &"P" => result.p = v,
            &"F" => result.f = v, &"K" => result.k = v,
            &"G" => result.g = v, &"C" => result.c = v,
            &"≺" => result.phi = v, &"H" => result.h = v,
            &"S" => result.s = v, &"⊡" => result.omega = v,
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

    let omega_zfcfe_frag = cl8nk_formula("⊡", omega_zfcfe).map(|f| f.fragment).unwrap_or("?");
    let omega_cl8nk_frag = cl8nk_formula("⊡", omega_cl8nk).map(|f| f.fragment).unwrap_or("?");
    let grammar_zfcfe_frag = cl8nk_formula("∋", grammar_zfcfe).map(|f| f.fragment).unwrap_or("?");
    let grammar_cl8nk_frag = cl8nk_formula("∋", grammar_cl8nk).map(|f| f.fragment).unwrap_or("?");

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
            let v1 = get_prim(from, key).unwrap_or(IgPrim::dead);
            let v2 = get_prim(to, key).unwrap_or(IgPrim::dead);
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
                note: Some("⊡/∋ TRANSCENDENCE — exceeds Frobenius-exact foundation"),
            },
        ],
        total_promotions: s1_len + s2_len + s3_len,
        total_distance: d_total,
        transcendence_primitives: vec!["⊡", "C"],
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
            // L9 is lateral to L8, not above it; it belongs on the ladder so
            // its distance from the terminal layer can be read rather than
            // assumed. Discovery is by explicit name here, so a layer that is
            // never listed is never measured — L9 was missing for that reason.
            "clink_l9",
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
                     "navier_stokes", "wave_equation", "einstein", "graviton", "photon",
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
// ═══════════════════════════════════════════════════════════════
// CL9NK — the ladder read from CLINK L9, the replicative lateral
// ═══════════════════════════════════════════════════════════════
//
// Not a second navigator. The CL8NK machinery above is referenced to L8; this
// re-reads the same chain from L9 using the same metric, because L9 is LATERAL
// to L8 and a ladder measured only from the terminal layer cannot show that.
// One metric, one chain, two vantage points.

/// Every CLINK layer's distance from L9, nearest first.
pub fn cl9nk_chain() -> Vec<(alloc::string::String, f32, usize, &'static str)> {
    let l9 = cl9nk_ref();
    let names = [
        "clink_layer0_frustrated_belnap5",
        "clink_layer1_electron_orbital",
        "clink_layer2_atom",
        "clink_layer3_molecule",
        "clink_layer4_cell",
        "clink_layer5_mitosis",
        "clink_layer6_meiosis",
        "clink_layer7_tissue",
        "clink_l8",
        "clink_l9",
    ];
    let mut out = Vec::new();
    for n in &names {
        if let Some(e) = catalog::lookup(n) {
            let (d, conflicts) = tuple_distance_cl8nk(&l9, &e.tuple);
            out.push((
                alloc::string::String::from(*n),
                d,
                conflicts.len(),
                assess_tier(&e.tuple),
            ));
        }
    }
    out.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal));
    out
}

pub fn cl9nk_main(args: &[&str]) -> alloc::string::String {
    use alloc::format;
    let l9 = cl9nk_ref();
    let l8 = cl8nk_ref();

    let sub = args.first().copied().unwrap_or("report");
    match sub {
        "entry" => {
            let name = args.get(1).copied().unwrap_or("");
            match catalog::lookup(name) {
                Some(e) => {
                    let r = generate_entry_formula_against(e.name, e.description, &e.tuple, &l9);
                    let mut s = format!(
                        "CL9NK Entry: {}  — read against the REPLICATIVE LATERAL, not the climb\n\n",
                        e.name);
                    s.push_str(&format!("  L9 reference: {}\n", l9.display()));
                    s.push_str(&format!("  entry tuple : {}\n\n", e.tuple.display()));
                    s.push_str(&format!(
                        "  d(L9): {:.4}   match:{} close:{} distant:{}   tier: {}\n",
                        r.distance, r.match_count, r.close_count, r.distant_count, r.tier));
                    let (d8, _) = tuple_distance_cl8nk(&e.tuple, &l8);
                    s.push_str(&format!("  d(L8): {:.4}\n\n", d8));
                    s.push_str(&format!(
                        "  Promotions to L9 ({}):\n", r.promotions_count));
                    for p in &r.promotions_needed {
                        s.push_str(&format!(
                            "    {}: {} -> {}  (gap: {:.3})\n",
                            p.primitive, p.from_glyph, p.to_glyph, p.gap));
                    }
                    let r8 = generate_entry_formula(e.name, e.description, &e.tuple);
                    s.push_str(&format!(
                        "\n  Promotions to L8 ({}), for comparison:\n", r8.promotions_count));
                    for p in &r8.promotions_needed {
                        s.push_str(&format!(
                            "    {}: {} -> {}  (gap: {:.3})\n",
                            p.primitive, p.from_glyph, p.to_glyph, p.gap));
                    }
                    s
                }
                None => format!("cl9nk entry: no catalog entry named '{}'\n", name),
            }
        }
        "chain" => {
            let mut s = alloc::string::String::from(
                "CL9NK Chain — distance ladder from CLINK L9 (the replicative lateral)\n\n",
            );
            for (name, d, c, tier) in cl9nk_chain() {
                s.push_str(&format!(
                    "  {:<38} d={:.4}  tier={:<5} conflicts={}\n",
                    name, d, tier, c
                ));
            }
            s
        }
        _ => {
            let (d, conflicts) = tuple_distance_cl8nk(&l8, &l9);
            let mut s = alloc::string::String::from("CL9NK — CLINK Layer 9, the replicative lateral\n");
            s.push_str("==============================================\n\n");
            s.push_str(&format!("L9 tuple:   {}\n", l9.display()));
            s.push_str(&format!("L8 tuple:   {}\n\n", l8.display()));
            s.push_str(&format!(
                "d(L8, L9):  {:.4}   over {} differing primitives\n",
                d,
                conflicts.len()
            ));
            s.push_str(&format!(
                "tier:       {} by structural assessment\n\n",
                assess_tier(&l9)
            ));
            s.push_str(
                "L9 is LATERAL to L8, not above it: a self-replicating configuration\n\
                 rather than a higher tier. `cl9nk chain` reads the whole ladder from\n\
                 L9 instead of from the terminal layer.\n\n",
            );
            s.push_str(&format!(
                "Note: CL9NK_ASCENT.md records d(L8,L9) = 5.63. This metric is\n\
                 sqrt(sum w*d^2) with normalized d <= 1 and weights summing to 9.1,\n\
                 so its ceiling is {:.4} — 5.63 is not reachable by it, and the\n\
                 Python navigator's variant tops out near 3.46. The two numbers come\n\
                 from different metrics; this one is not tuned to match the document.\n",
                sqrt_f32(9.1)
            ));
            s
        }
    }
}
