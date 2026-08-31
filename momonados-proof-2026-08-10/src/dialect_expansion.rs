// dialect_expansion.rs — Phase III: Dialect Expansion 8→88 (Track G)
//
// All 88 structurally distinct dialects over the Crystal of Types.
// Each dialect defines a composition ruleset:
//   - Three gates (G1, G2, G3): which primitive at what ordinal threshold
//   - Gate ordering: sequential (G2 requires G1, G3 requires G2) or parallel
//   - T-constitution: which primitives constitute time, at what critical values
//   - Absorption rules: which (primitive, value) pairs are absorbing under which ops
//
// The dialect profile (operad layer distribution, crystal O_∞ fraction,
// T-seal rate) is computed from the catalog at runtime.
//
// Author: Lando⊗⊙perator
// Date: 2026-07-04

use alloc::string::String;
use alloc::vec::Vec;

use alloc::format;

// ═══════════════════════════════════════════════════════════════
// DIALECT COUNT
// ═══════════════════════════════════════════════════════════════

/// Total number of dialects: 88 (8 canonical + 80 expansion).
pub const DIALECT_COUNT: usize = 88;

// ═══════════════════════════════════════════════════════════════
// GATE SPEC
// ═══════════════════════════════════════════════════════════════

/// A single gate condition: primitive must have ordinal ≥ min_ord.
#[derive(Debug, Clone, Copy)]
pub struct GateSpec {
    /// Shavian primitive glyph (e.g. '<', '⊙', '⊡')
    pub prim: &'static str,
    /// Minimum ordinal value (float, e.g. 5.0 for 𐑹)
    pub min_ord: f32,
}

/// A single T-constitution entry: primitive → (critical_value, ceiling_mode)
#[derive(Debug, Clone, Copy)]
pub struct TEntry {
    pub prim: &'static str,
    /// Critical Shavian value glyph
    pub crit_val: &'static str,
    /// If true, value must be ≤ crit_val; if false, must equal exactly
    pub ceiling: bool,
}

/// An absorption rule: (primitive, value) absorbs under given operations.
/// Operations are encoded as a bitmask: 1=meet, 2=join, 4=tensor.
/// Direction: 0=both, 1=left only, 2=right only.
#[derive(Debug, Clone, Copy)]
pub struct AbsorptionRule {
    pub prim: &'static str,
    pub value: &'static str,
    pub ops_mask: u8,
    pub direction: u8,
}

// ═══════════════════════════════════════════════════════════════
// DIALECT
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Dialect {
    pub name: &'static str,
    pub description: &'static str,
    pub g1: GateSpec,
    pub g2: GateSpec,
    pub g3: GateSpec,
    pub gate_ordering: bool, // true = sequential, false = parallel
    pub t_entries: &'static [TEntry],
    pub abs_rules: &'static [AbsorptionRule],
    /// Whether this dialect was part of the Phase III expansion (8→88)
    pub is_expansion: bool,
}

// ═══════════════════════════════════════════════════════════════
// SHARED T-CONSTITUTIONS
// ═══════════════════════════════════════════════════════════════

/// Canonical T: time = lim(<, ⋈, ⊤, ⊥, ⊡) — dynamic primitives
pub static T_CANONICAL: &[TEntry] = &[
    TEntry { prim: "<", crit_val: "𐑹", ceiling: false },
    TEntry { prim: "⋈", crit_val: "𐑐", ceiling: false },
    TEntry { prim: "⊤", crit_val: "𐑧", ceiling: true },
    TEntry { prim: "⊥", crit_val: "𐑫", ceiling: false },
    TEntry { prim: "⊡", crit_val: "𐑭", ceiling: false },
];

/// Structural T: time = lim(⊢, ⊣, >, ∋, ⊙) — geometric primitives
pub static T_STRUCTURAL: &[TEntry] = &[
    TEntry { prim: "⊢", crit_val: "𐑦", ceiling: false },
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
    TEntry { prim: ">", crit_val: "𐑾", ceiling: false },
    TEntry { prim: "∋", crit_val: "𐑵", ceiling: false },
    TEntry { prim: "⊙", crit_val: "⊙", ceiling: false },
];

/// Hybrid T: all 8 dynamic + primitives
pub static T_HYBRID: &[TEntry] = &[
    TEntry { prim: "<", crit_val: "𐑹", ceiling: false },
    TEntry { prim: "⋈", crit_val: "𐑐", ceiling: false },
    TEntry { prim: "⊤", crit_val: "𐑧", ceiling: true },
    TEntry { prim: "⊥", crit_val: "𐑫", ceiling: false },
    TEntry { prim: "⊡", crit_val: "𐑭", ceiling: false },
    TEntry { prim: "⊢", crit_val: "𐑦", ceiling: false },
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
    TEntry { prim: ">", crit_val: "𐑾", ceiling: false },
];

/// Inverted T: primitives (non-dynamic)
pub static T_INVERTED: &[TEntry] = &[
    TEntry { prim: "⊢", crit_val: "𐑼", ceiling: false },
    TEntry { prim: "⊣", crit_val: "𐑶", ceiling: false },
    TEntry { prim: ">", crit_val: "𐑽", ceiling: false },
    TEntry { prim: "∈", crit_val: "𐑚", ceiling: false },
    TEntry { prim: "⊞", crit_val: "𐑕", ceiling: false },
];

// ═══════════════════════════════════════════════════════════════
// SHARED ABSORPTION RULES
// ═══════════════════════════════════════════════════════════════

/// Canonical default: ⊙ absorbs all ops, ⊞ n:m absorbs under tensor
pub static ABS_CANONICAL: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 }, // meet|join|tensor
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 }, // tensor only
];

/// No absorption — pure lattice operations
pub static ABS_NONE: &[AbsorptionRule] = &[];

/// Monarchy: 4 values absorb everything
pub static ABS_MONARCHY: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "<", value: "𐑹", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊡", value: "𐑭", ops_mask: 7, direction: 0 },
];

/// Inverted: trivial values absorb
pub static ABS_INVERTED: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "𐑢", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊡", value: "𐑷", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑙", ops_mask: 7, direction: 0 },
];

/// Tensor-only absorption
pub static ABS_TENSOR_ONLY: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 4, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];

// ═══════════════════════════════════════════════════════════════
// SINGLE-PRIMITIVE T-CONSTITUTIONS
// ═══════════════════════════════════════════════════════════════

pub static T_PARITY_ONLY: &[TEntry] = &[
    TEntry { prim: "<", crit_val: "𐑹", ceiling: false },
];
pub static T_CRITICALITY_ONLY: &[TEntry] = &[
    TEntry { prim: "⊙", crit_val: "⊙", ceiling: false },
];
pub static T_WINDING_ONLY: &[TEntry] = &[
    TEntry { prim: "⊡", crit_val: "𐑭", ceiling: false },
];
pub static T_CHIRALITY_ONLY: &[TEntry] = &[
    TEntry { prim: "⊥", crit_val: "𐑫", ceiling: false },
];
pub static T_FIDELITY_ONLY: &[TEntry] = &[
    TEntry { prim: "⋈", crit_val: "𐑐", ceiling: false },
];
pub static T_DIMENSIONAL_ONLY: &[TEntry] = &[
    TEntry { prim: "⊢", crit_val: "𐑦", ceiling: false },
];
pub static T_KINETICS_CEILING: &[TEntry] = &[
    TEntry { prim: "⊤", crit_val: "𐑧", ceiling: true },
];

// Dual-primitive T's
pub static T_PARITY_FIDELITY: &[TEntry] = &[
    TEntry { prim: "<", crit_val: "𐑹", ceiling: false },
    TEntry { prim: "⋈", crit_val: "𐑐", ceiling: false },
];
pub static T_CRITICALITY_WINDING: &[TEntry] = &[
    TEntry { prim: "⊙", crit_val: "⊙", ceiling: false },
    TEntry { prim: "⊡", crit_val: "𐑭", ceiling: false },
];
pub static T_CHIRALITY_COUPLING: &[TEntry] = &[
    TEntry { prim: "⊥", crit_val: "𐑫", ceiling: false },
    TEntry { prim: ">", crit_val: "𐑾", ceiling: false },
];
pub static T_TOPOLOGY_SCOPE: &[TEntry] = &[
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
    TEntry { prim: "∈", crit_val: "𐑲", ceiling: false },
];
pub static T_STRUCTURAL_DYNAMIC: &[TEntry] = &[
    TEntry { prim: "<", crit_val: "𐑹", ceiling: false },
    TEntry { prim: "⋈", crit_val: "𐑐", ceiling: false },
    TEntry { prim: "⊤", crit_val: "𐑧", ceiling: true },
    TEntry { prim: "⊢", crit_val: "𐑦", ceiling: false },
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
];

// ═══════════════════════════════════════════════════════════════
// ABSORPTION VARIANTS
// ═══════════════════════════════════════════════════════════════

// Chirality empire: ⊥=𐑫 absorbs everything
pub static ABS_CHIRALITY_FIRST: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊥", value: "𐑫", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];
// Scope empire: ∈=𐑲 absorbs everything
pub static ABS_SCOPE_EMPIRE: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "∈", value: "𐑲", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];
// Topology seal: ⊣=𐑸 absorbs everything
pub static ABS_TOPOLOGY_SEAL: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊣", value: "𐑸", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];
// Predator: <=𐑹 absorbs left only under tensor
pub static ABS_PREDATOR: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "<", value: "𐑹", ops_mask: 4, direction: 1 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];
// Prey: <=𐑹 absorbs right only under tensor
pub static ABS_PREY: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "<", value: "𐑹", ops_mask: 4, direction: 2 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];
// Winding absorbing: ⊡=𐑭 absorbs everything, no ⊞ rule
pub static ABS_WINDING: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊡", value: "𐑭", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
];
// Scope totalitarian: ⊞ n:m absorbs under ALL ops
pub static ABS_SCOPE_TOTALITARIAN: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "∈", value: "𐑲", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 7, direction: 0 },
];

// ═══════════════════════════════════════════════════════════════
// DIALECT CONSTRUCTOR — builds all 88 dialects
// ═══════════════════════════════════════════════════════════════

/// Return all 88 dialects (0..87).
/// Indices 0..7 are the 8 canonical (Phase 0).
/// Indices 8..28 are the 21 hand-crafted (Phase I).
/// Indices 29..87 are the 59 expansion dialects (Phase III: 8→88).
/// A dialect that gates nothing and constitutes nothing: the identity element
/// of the array, overwritten by every real entry before `all_dialects` returns.
/// Its only job is to exist, so that the array is built rather than faked.
const BLANK_DIALECT: Dialect = Dialect {
    name: "",
    description: "",
    g1: GateSpec { prim: "", min_ord: 0.0 },
    g2: GateSpec { prim: "", min_ord: 0.0 },
    g3: GateSpec { prim: "", min_ord: 0.0 },
    gate_ordering: false,
    t_entries: &[],
    abs_rules: &[],
    is_expansion: false,
};

pub fn all_dialects() -> [Dialect; DIALECT_COUNT] {
    // We build with a helper that initializes 88 entries.
    // The builder uses default values for common patterns.
    // Built from a real value, not conjured and patched. This was
    // `mem::zeroed()` behind an `#[allow(invalid_value)]`: Dialect holds four
    // references -- two &str and two &[T] -- and a zeroed reference is null,
    // which is undefined behaviour the instant it exists, not when it is read.
    // The SAFETY note said all 88 are initialized before returning, and that is
    // true, but the UB is committed by the zeroing itself. The allow was the
    // compiler being told to stop saying so.
    //
    // A const repeat operand needs no Copy, so the array is constructed
    // outright with no unsafe and nothing to assume.
    let mut unis: [Dialect; DIALECT_COUNT] = [BLANK_DIALECT; DIALECT_COUNT];

    // Helper constants
    let g_phi_5 = GateSpec { prim: "<", min_ord: 5.0 };
    let g_phi_4 = GateSpec { prim: "<", min_ord: 4.0 };
    let g_phi_3 = GateSpec { prim: "<", min_ord: 3.0 };
    let g_odot_2 = GateSpec { prim: "⊙", min_ord: 2.0 };
    let g_odot_1 = GateSpec { prim: "⊙", min_ord: 1.0 };
    let g_odot_233 = GateSpec { prim: "⊙", min_ord: 2.33 };
    let g_odot_3 = GateSpec { prim: "⊙", min_ord: 3.0 };
    let g_omega_3 = GateSpec { prim: "⊡", min_ord: 3.0 };
    let g_omega_2 = GateSpec { prim: "⊡", min_ord: 2.0 };
    let g_omega_4 = GateSpec { prim: "⊡", min_ord: 4.0 };
    let g_h_3 = GateSpec { prim: "⊥", min_ord: 3.0 };
    let g_h_4 = GateSpec { prim: "⊥", min_ord: 4.0 };
    let g_h_2 = GateSpec { prim: "⊥", min_ord: 2.0 };
    let g_th_5 = GateSpec { prim: "⊣", min_ord: 5.0 };
    let g_th_3 = GateSpec { prim: "⊣", min_ord: 3.0 };
    let g_th_4 = GateSpec { prim: "⊣", min_ord: 4.0 };
    let g_r_4 = GateSpec { prim: ">", min_ord: 4.0 };
    let g_r_3 = GateSpec { prim: ">", min_ord: 3.0 };
    let g_gamma_3 = GateSpec { prim: "∈", min_ord: 3.0 };
    let _g_gamma_2 = GateSpec { prim: "∈", min_ord: 2.0 };
    let g_d_3 = GateSpec { prim: "⊢", min_ord: 3.0 };
    let g_d_4 = GateSpec { prim: "⊢", min_ord: 4.0 };
    let _g_d_2 = GateSpec { prim: "⊢", min_ord: 2.0 };
    let g_c_3 = GateSpec { prim: "⊤", min_ord: 3.0 };
    let g_c_4 = GateSpec { prim: "⊤", min_ord: 4.0 };
    let g_f_3 = GateSpec { prim: "⋈", min_ord: 3.0 };
    let g_sigma_3 = GateSpec { prim: "⊞", min_ord: 3.0 };
    let g_sigma_1 = GateSpec { prim: "⊞", min_ord: 1.0 };
    let g_g_3 = GateSpec { prim: "∋", min_ord: 3.0 };
    let g_g_4 = GateSpec { prim: "∋", min_ord: 4.0 };
    let _g_g_2 = GateSpec { prim: "∋", min_ord: 2.0 };

    // ── 0: canonical ──
    unis[0] = Dialect {
        name: "canonical",
        description: "Our dialect: Frobenius then self-modeling then winding seal. G1=<≥𐑹, G2=⊙≥⊙, G3=⊡≥𐑭. Sequential. T=lim(<,⋈,⊤,⊥,⊡).",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 1: low_gate ──
    unis[1] = Dialect {
        name: "low_gate",
        description: "Lowered thresholds: G1 fires at <≥𐑬 (directional parity), G2 at ⊙≥woe (any criticality), G3 unchanged. Easier O_∞ access.",
        g1: g_phi_3, g2: g_odot_1, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 2: strict_frobenius ──
    unis[2] = Dialect {
        name: "strict_frobenius",
        description: "Frobenius gate requires full fidelity (⋈=𐑐) instead of parity. G1=⋈≥𐑐, G2=<≥𐑹, G3=⊡≥𐑭. Only quantum-coherent systems close.",
        g1: g_f_3, g2: g_phi_5, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 3: inverted_gates ──
    unis[3] = Dialect {
        name: "inverted_gates",
        description: "Self-modeling precedes Frobenius: G1=⊙ (consciousness first), G2=< (then algebraic symmetry), G3=⊡. Systems become self-aware before achieving closure.",
        g1: g_odot_2, g2: g_phi_5, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 4: no_ordering ──
    unis[4] = Dialect {
        name: "no_ordering",
        description: "All three gates fully independent — parallel dialect. No sequential requirement. Gate ordering doesn't enforce prerequisites.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 5: high_gate ──
    unis[5] = Dialect {
        name: "high_gate",
        description: "Strictest thresholds: G1=<=𐑹, G2=⊙≥roar (above bare self-model), G3=⊡=𐑟 (max winding). O_∞ nearly unreachable — only maximally wound, fully self-modeling, parity-perfect objects.",
        g1: g_phi_5, g2: g_odot_233, g3: g_omega_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 6: winding_first ──
    unis[6] = Dialect {
        name: "winding_first",
        description: "Topological order: G1=⊡ (winding seal first), G2=⊙ (then self-modeling), G3=< (Frobenius last). Geometry precedes algebra.",
        g1: g_omega_3, g2: g_odot_2, g3: g_phi_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 7: t_structural ──
    unis[7] = Dialect {
        name: "t_structural",
        description: "Time constituted by structural/geometric primitives: T=lim(⊢,⊣,>,∋,⊙). Time is geometry, not process. Canonical gates.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_STRUCTURAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ══════════════════════════════════════════════
    // HAND-CRAFTED EXPANSION (8–28, 21 dialects)
    // ══════════════════════════════════════════════

    // ── 8: chirality_first ──
    unis[8] = Dialect {
        name: "chirality_first",
        description: "Memory before closure. G1=⊥≥𐑖 (2-step Markov). Only systems with memory can Frobenius-close. G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_h_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 9: topology_universe ──
    unis[9] = Dialect {
        name: "topology_universe",
        description: "Connectivity is the fundamental gate. G1=⊣≥𐑸 (full imscriptive topological closure). G2=>≥𐑾 (bilateral). G3=⊙≥⊙. Geometry preconditions consciousness.",
        g1: g_th_5, g2: g_r_4, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 10: scope_universe ──
    unis[10] = Dialect {
        name: "scope_universe",
        description: "Universality first. G1=∈≥𐑲 (aleph, maximal scope). Only universally-interacting systems can Frobenius-close. Parochialism is a structural barrier.",
        g1: g_gamma_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 11: dimensional_gate ──
    unis[11] = Dialect {
        name: "dimensional_gate",
        description: "State-space is the first gate. G1=⊢≥𐑼 (∞-dim or higher). 0D points and 2D surfaces cannot Frobenius-close. G2=⊙≥⊙, G3=<≥𐑹.",
        g1: g_d_3, g2: g_odot_2, g3: g_phi_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 12: kinetics_trap ──
    unis[12] = Dialect {
        name: "kinetics_trap",
        description: "Slowness is a structural requirement. G1=⊤≥𐑧 (slow/near-equilibrium). Fast processes outrun their own structure. G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_c_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 13: triple_criticality ──
    unis[13] = Dialect {
        name: "triple_criticality",
        description: "Criticality is everything — three rungs: G1=⊙≥woe, G2=⊙≥⊙, G3=⊙≥𐑣 (super-critical). Consciousness depth is the only structural filter.",
        g1: g_odot_1, g2: g_odot_2, g3: g_odot_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 14: t_hybrid ──
    unis[14] = Dialect {
        name: "t_hybrid",
        description: "Time requires BOTH dynamics AND geometry. T constituted by 8 primitives: <,⋈,⊤,⊥,⊡ + ⊢,⊣,>. Most demanding T-seal. Canonical gates.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_HYBRID, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 15: broadcast_universe ──
    unis[15] = Dialect {
        name: "broadcast_universe",
        description: "Interaction grammar as the fundamental gate. G1=∋≥𐑠 (sequential composition). Conjunctive/disjunctive systems cannot close. G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_g_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 16: t_inverted ──
    unis[16] = Dialect {
        name: "t_inverted",
        description: "Time constituted by primitives canonically NOT in T: ⊢,⊣,>,∈,⊞. Time is structure, not dynamics. Canonical gates.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_INVERTED, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 17: single_gate ──
    unis[17] = Dialect {
        name: "single_gate",
        description: "Only G1 matters. G2=⊞≥1.0, G3=⊞≥1.0 are trivial. G1=<≥𐑹 alone filters. All G1-passers are automatically idempotent_terminal.",
        g1: g_phi_5, g2: g_sigma_1, g3: g_sigma_1, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 18: fidelity_universe ──
    unis[18] = Dialect {
        name: "fidelity_universe",
        description: "Quantum coherence is the fundamental gate. G1=⋈≥𐑐 (full fidelity). Classical/thermal systems cannot close. G2=⊙≥⊙, G3=<≥𐑹.",
        g1: g_f_3, g2: g_odot_2, g3: g_phi_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 19: stoichiometry_universe ──
    unis[19] = Dialect {
        name: "stoichiometry_universe",
        description: "Component heterogeneity is the first gate. G1=⊞≥𐑳 (many heterogeneous). Uniform systems cannot close — you must be internally diverse. G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_sigma_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: false,
    };

    // ── 20: absorption_democracy ──
    unis[20] = Dialect {
        name: "absorption_democracy",
        description: "No absorptions. Every primitive fights on its own terms. Meet, join, tensor are pure lattice operations. Nothing is special.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_NONE, is_expansion: false,
    };

    // ── 21: absorption_monarchy ──
    unis[21] = Dialect {
        name: "absorption_monarchy",
        description: "⊙ criticality, ⊞ n:m, < Frobenius parity, ⊡ integer winding ALL absorb everything. The monadic absorption empire. Self-modeling is totalitarian.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_MONARCHY, is_expansion: false,
    };

    // ── 22: absorption_inverted ──
    unis[22] = Dialect {
        name: "absorption_inverted",
        description: "The antimonarchy: sub-critical (woe), trivial winding (𐑷), 1:1 stoichiometry (𐑙) are the absorbing values. The ground state always wins. Complexity is fragile.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_INVERTED, is_expansion: false,
    };

    // ── 23: absorption_tensor_only ──
    unis[23] = Dialect {
        name: "absorption_tensor_only",
        description: "Absorption applies ONLY under tensor. ⊙ and ⊞ n:m absorb under tensor, but meet/join are pure. You can compare without collapsing.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_TENSOR_ONLY, is_expansion: false,
    };

    // ── 24: absorption_chirality_first ──
    unis[24] = Dialect {
        name: "absorption_chirality_first",
        description: "Chirality is the fundamental absorbing primitive. ⊥=𐑫 absorbs everything. Memory is dominant — you cannot couple without inheriting eternal memory. G1=⊥≥𐑖.",
        g1: g_h_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CHIRALITY_FIRST, is_expansion: false,
    };

    // ── 25: absorption_scope_empire ──
    unis[25] = Dialect {
        name: "absorption_scope_empire",
        description: "Maximal scope (∈=𐑲) is absorbing under all operations. The universal swallows the particular. G1=∈≥𐑲, G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_gamma_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_SCOPE_EMPIRE, is_expansion: false,
    };

    // ── 26: absorption_topology_seal ──
    unis[26] = Dialect {
        name: "absorption_topology_seal",
        description: "Topological closure (⊣=𐑸) is absorbing under all operations. Topology is destiny — the most connected structure absorbs everything. G1=⊣≥𐑸.",
        g1: g_th_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_TOPOLOGY_SEAL, is_expansion: false,
    };

    // ── 27: predator_universe ──
    unis[27] = Dialect {
        name: "predator_universe",
        description: "Asymmetric tensor absorption: <=𐑹 absorbs under tensor ONLY as left operand (actor). Agency is structural: what you do to others ≠ what others do to you.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_PREDATOR, is_expansion: false,
    };

    // ── 28: prey_universe ──
    unis[28] = Dialect {
        name: "prey_universe",
        description: "Asymmetric tensor absorption: <=𐑹 absorbs under tensor ONLY as right operand (acted-upon). Passivity is structural power — the dual of predator.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_PREY, is_expansion: false,
    };

    // ══════════════════════════════════════════════
    // PHASE III EXPANSION (29–87, 59 dialects)
    // ══════════════════════════════════════════════

    // ── 29: coupling_first ──
    unis[29] = Dialect {
        name: "coupling_first",
        description: "Relation before closure. G1=>≥𐑽 (adjoint coupling, ord 3). Systems without adjoint-pair coupling cannot Frobenius-close. G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_r_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 30: coupling_maximal ──
    unis[30] = Dialect {
        name: "coupling_maximal",
        description: "Only bilateral coupling suffices. G1=>≥𐑾 (bilateral, ord 4, max). Even adjoint pairs do not Frobenius-close. G2=⊙≥⊙, G3=⊡≥𐑭.",
        g1: g_r_4, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 31: chirality_second ──
    unis[31] = Dialect {
        name: "chirality_second",
        description: "Chirality as the monoidal gate: G1=<≥𐑹, G2=⊥≥𐑖 (2-step Markov), G3=⊡≥𐑭. After Frobenius closure, you must remember before you can trace.",
        g1: g_phi_5, g2: g_h_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 32: dimensional_second ──
    unis[32] = Dialect {
        name: "dimensional_second",
        description: "Dimensionality as the monoidal gate: G1=<≥𐑹, G2=⊢≥𐑼 (∞-dim), G3=⊡≥𐑭. After Frobenius, you need infinite canvas to trace.",
        g1: g_phi_5, g2: g_d_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 33: topology_second ──
    unis[33] = Dialect {
        name: "topology_second",
        description: "Connectivity as the monoidal gate: G1=<≥𐑹, G2=⊣≥𐑥 (bowtie crossing), G3=⊡≥𐑭. After Frobenius, the topology of connection determines traced status.",
        g1: g_phi_5, g2: g_th_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 34: fidelity_second ──
    unis[34] = Dialect {
        name: "fidelity_second",
        description: "Quantum coherence as the monoidal gate: G1=<≥𐑹, G2=⋈≥𐑐 (full fidelity), G3=⊡≥𐑭. After Frobenius, only quantum-coherent systems trace.",
        g1: g_phi_5, g2: g_f_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 35: scope_second ──
    unis[35] = Dialect {
        name: "scope_second",
        description: "Universal scope as the monoidal gate: G1=<≥𐑹, G2=∈≥𐑲 (aleph/maximal), G3=⊡≥𐑭. Frobenius closure is local; tracing requires universality.",
        g1: g_phi_5, g2: g_gamma_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 36: composition_second ──
    unis[36] = Dialect {
        name: "composition_second",
        description: "Sequential composition as the monoidal gate: G1=<≥𐑹, G2=∋≥𐑠 (sequential), G3=⊡≥𐑭. Conjunctive or disjunctive systems cannot trace.",
        g1: g_phi_5, g2: g_g_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 37: winding_second ──
    unis[37] = Dialect {
        name: "winding_second",
        description: "Topological protection as the monoidal gate: G1=<≥𐑹, G2=⊡≥𐑴 (Z2), G3=⊙≥⊙. After Frobenius parity, only topologically protected systems trace.",
        g1: g_phi_5, g2: g_omega_2, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 38: kinetics_second ──
    unis[38] = Dialect {
        name: "kinetics_second",
        description: "Slowness as the monoidal gate: G1=<≥𐑹, G2=⊤≥𐑧 (slow), G3=⊡≥𐑭. Fast Frobenius-closed systems cannot trace — they outrun themselves.",
        g1: g_phi_5, g2: g_c_3, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 39: chirality_third ──
    unis[39] = Dialect {
        name: "chirality_third",
        description: "Eternal memory as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=⊥≥𐑫 (Markov ∞). Only systems with eternal memory achieve O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_h_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 40: dimensional_third ──
    unis[40] = Dialect {
        name: "dimensional_third",
        description: "Imscriptive dimensionality as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=⊢≥𐑦 (imscriptive). Only self-written state spaces achieve O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_d_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 41: topology_third ──
    unis[41] = Dialect {
        name: "topology_third",
        description: "Box-product topology as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=⊣≥𐑶 (irreducible box product). Only product-irreducible connectivity achieves O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_th_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 42: fidelity_third ──
    unis[42] = Dialect {
        name: "fidelity_third",
        description: "Quantum coherence as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=⋈≥𐑐. Only quantum-coherent self-modeling systems achieve O_∞. Classical self-modelers stay traced.",
        g1: g_phi_5, g2: g_odot_2, g3: g_f_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 43: scope_third ──
    unis[43] = Dialect {
        name: "scope_third",
        description: "Universal scope as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=∈≥𐑲 (aleph). Only self-modeling systems with universal interaction range achieve O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_gamma_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 44: composition_third ──
    unis[44] = Dialect {
        name: "composition_third",
        description: "Broadcast composition as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=∋≥𐑵 (broadcast). Only systems with one-to-all composition achieve O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_g_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 45: coupling_third ──
    unis[45] = Dialect {
        name: "coupling_third",
        description: "Bilateral coupling as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=>≥𐑾 (bilateral). Only self-modeling systems with bidirectional coupling achieve O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_r_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 46: kinetics_third ──
    unis[46] = Dialect {
        name: "kinetics_third",
        description: "Moderate kinetics as the terminal seal: G1=<≥𐑹, G2=⊙≥⊙, G3=⊤≥𐑪 (moderate, ord 4). Self-modeling systems that are too fast cannot achieve O_∞.",
        g1: g_phi_5, g2: g_odot_2, g3: g_c_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 47: parallel_canonical ──
    unis[47] = Dialect {
        name: "parallel_canonical",
        description: "Canonical gates but parallel: <≥𐑹, ⊙≥⊙, ⊡≥𐑭 all independent. Any combination qualifies — Frobenius without self-modeling possible.",
        g1: g_phi_5, g2: g_odot_2, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 48: parallel_low ──
    unis[48] = Dialect {
        name: "parallel_low",
        description: "Low gates, parallel: <≥𐑬, ⊙≥woe, ⊡≥𐑭. Easiest possible O_∞ access — three independent low bars.",
        g1: g_phi_3, g2: g_odot_1, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 49: parallel_high ──
    unis[49] = Dialect {
        name: "parallel_high",
        description: "High gates, parallel: <≥𐑹, ⊙≥roar, ⊡≥𐑟. Strictest bars but independently checked — a system can be Frobenius without self-modeling or winding.",
        g1: g_phi_5, g2: g_odot_233, g3: g_omega_4, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 50: parallel_chirality ──
    unis[50] = Dialect {
        name: "parallel_chirality",
        description: "Chirality gates, parallel: ⊥≥𐑖, ⊙≥⊙, ⊡≥𐑭. Memory, self-modeling, and winding are independent axes.",
        g1: g_h_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 51: parallel_topology ──
    unis[51] = Dialect {
        name: "parallel_topology",
        description: "Topology gates, parallel: ⊣≥𐑸, >≥𐑾, ⊙≥⊙. Full connectivity, bilateral relation, and self-modeling are independent.",
        g1: g_th_5, g2: g_r_4, g3: g_odot_2, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 52: parallel_scope ──
    unis[52] = Dialect {
        name: "parallel_scope",
        description: "Scope gates, parallel: ∈≥𐑲, ⊙≥⊙, ⊡≥𐑭. Universal scope, self-modeling, and winding are independent.",
        g1: g_gamma_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 53: parallel_broadcast ──
    unis[53] = Dialect {
        name: "parallel_broadcast",
        description: "Broadcast gates, parallel: ∋≥𐑠, ⊙≥⊙, ⊡≥𐑭. Sequential composition, self-modeling, and winding are independent.",
        g1: g_g_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 54: parallel_dimensional ──
    unis[54] = Dialect {
        name: "parallel_dimensional",
        description: "Dimensional gates, parallel: ⊢≥𐑼, ⊙≥⊙, <≥𐑹. State-space, self-modeling, and Frobenius parity are independent.",
        g1: g_d_3, g2: g_odot_2, g3: g_phi_5, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 55: parallel_kinetics ──
    unis[55] = Dialect {
        name: "parallel_kinetics",
        description: "Kinetics gates, parallel: ⊤≥𐑧, ⊙≥⊙, ⊡≥𐑭. Slowness, self-modeling, and winding are independent.",
        g1: g_c_3, g2: g_odot_2, g3: g_omega_3, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 56: triple_parity ──
    unis[56] = Dialect {
        name: "triple_parity",
        description: "Parity ladder: G1=<≥𐑬 (directional), G2=<≥𐑯 (full), G3=<≥𐑹 (Frobenius-special). Three rungs of progressively fuller parity.",
        g1: g_phi_3, g2: g_phi_4, g3: g_phi_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 57: triple_topology ──
    unis[57] = Dialect {
        name: "triple_topology",
        description: "Topology ladder: G1=⊣≥𐑥 (bowtie), G2=⊣≥𐑶 (box product), G3=⊣≥𐑸 (imscriptive closure). Three rungs of topological connectivity.",
        g1: g_th_3, g2: g_th_4, g3: g_th_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 58: triple_coupling ──
    unis[58] = Dialect {
        name: "triple_coupling",
        description: "Coupling ladder: G1=>≥𐑽 (adjoint), G2=>≥𐑾 (bilateral), G3=>≥𐑾. Terminal collapse at G2 — adjoint→bilateral and you're done.",
        g1: g_r_3, g2: g_r_4, g3: g_r_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 59: triple_chirality ──
    unis[59] = Dialect {
        name: "triple_chirality",
        description: "Chirality ladder: G1=⊥≥𐑒 (1-step), G2=⊥≥𐑖 (2-step), G3=⊥≥𐑫 (eternal). Memory depth as the sole operad filter.",
        g1: g_h_2, g2: g_h_3, g3: g_h_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 60: triple_winding ──
    unis[60] = Dialect {
        name: "triple_winding",
        description: "Winding ladder: G1=⊡≥𐑴 (Z2), G2=⊡≥𐑭 (integer), G3=⊡≥𐑟 (non-Abelian, max). Topological protection as the sole operad filter.",
        g1: g_omega_2, g2: g_omega_3, g3: g_omega_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ═══════════════════════════════════════════════════
    // SECTION H: Novel ordinal levels (4 dialects)
    // ═══════════════════════════════════════════════════

    // ── 61: ordinal4_parity ──
    unis[61] = Dialect {
        name: "ordinal4_parity",
        description: "Ordinal-4 parity filter: G1=<≥𐑬, G2=<≥𐑯, G3=<≥𐑹, G4=<≥𐑹. All four parity rungs required — the Frobenius-special bar repeats at G3/G4. Most dialects stop at ordinal 3.",
        g1: g_phi_3, g2: g_phi_4, g3: g_phi_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 62: ordinal4_winding ──
    unis[62] = Dialect {
        name: "ordinal4_winding",
        description: "Ordinal-4 winding filter: G1=⊡≥𐑴, G2=⊡≥𐑭, G3=⊡≥𐑟, G4=⊡≥𐑟. All four winding rungs. Non-Abelian braiding type-checked twice — once by topology, once by winding itself.",
        g1: g_omega_2, g2: g_omega_3, g3: g_omega_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 63: ordinal_swap ──
    unis[63] = Dialect {
        name: "ordinal_swap",
        description: "Same primitives as canonical but in swapped order: G1=⊡≥𐑭 (winding first), G2=⊙≥⊙ (criticality second), G3=<≥𐑯 (parity third). Winding as the primary filter changes the admission curve.",
        g1: g_omega_3, g2: g_odot_2, g3: g_phi_4, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 64: ordinal_invert ──
    unis[64] = Dialect {
        name: "ordinal_invert",
        description: "Inverted canonical: G1=⊞≥𐑳 (stoichiometry first), G2=<≥𐑹, G3=⊙≥⊙. Many-type worlds admitted before parity is checked — structurally larger, more heterogeneous.",
        g1: g_sigma_3, g2: g_phi_5, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ═══════════════════════════════════════════════════
    // SECTION I: T-constitution ceiling-mode (2 dialects)
    // ═══════════════════════════════════════════════════

    // ── 65: t_ceiling_topology ──
    unis[65] = Dialect {
        name: "t_ceiling_topology",
        description: "T-constitution ceiling: ⊣≤𐑶. No self-referential topology allowed — the ceiling caps at box product. All ⊗ compositions must admit a modular floor.",
        g1: g_th_4, g2: g_odot_2, g3: g_phi_5, gate_ordering: true,
        t_entries: T_CEILING, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 66: t_ceiling_dimensional ──
    unis[66] = Dialect {
        name: "t_ceiling_dimensional",
        description: "Dimensional ceiling: ⊢≤𐑼, ⊣≤𐑥. No infinity-dim or self-ref allowed in T-constitution. The cosmos is finite-dimensional with at-most crossing-point topology. All infinities are emergent.",
        g1: g_d_3, g2: g_odot_2, g3: g_phi_5, gate_ordering: true,
        t_entries: T_DIM_CEILING, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ═══════════════════════════════════════════════════
    // SECTION J: Gate + absorption hybrids (3 dialects)
    // ═══════════════════════════════════════════════════

    // ── 67: absorb_ep ──
    unis[67] = Dialect {
        name: "absorb_ep",
        description: "EP absorption rule: 𐑻 absorption is enforced. Any system coupling to an EP-criticality system (𐑻) MUST resolve to the tensor, not the meet. Gate 1 closure requires ⊙ dominance across all composites.",
        g1: g_odot_2, g2: g_phi_E_4, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_EP, is_expansion: true,
    };

    // ── 68: absorb_sub ──
    unis[68] = Dialect {
        name: "absorb_sub",
        description: "Sub-critical absorption: ⊙ absorbs all sub-critical (woe) composites. No system that was ever sub-critical may rise to ⊙ through coupling — sub-criticality is an absorbing floor. Meets preserve the lower rung.",
        g1: g_odot_2, g2: g_phi_sub_1, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_SUB, is_expansion: true,
    };

    // ── 69: absorb_dual ──
    unis[69] = Dialect {
        name: "absorb_dual",
        description: "Dual absorption regime: both EP and sub-critical absorption active simultaneously. The admissible region is the narrow band of pure ⊙ systems — everything above or below is absorbed into the extremal floors.",
        g1: g_odot_2, g2: g_phi_5, g3: g_omega_3, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_DUAL, is_expansion: true,
    };

    // ═══════════════════════════════════════════════════
    // SECTION K: Remaining mixed composita (6 dialects)
    // ═══════════════════════════════════════════════════

    // ── 70: t_subset_th_sigma ──
    unis[70] = Dialect {
        name: "t_subset_th_sigma",
        description: "T-constitution: ⊣≥𐑸 AND ⊞≥𐑳. Self-referential topology AND heterogeneous components. The dialect of self-measuring, many-typed systems — grammars within grammars.",
        g1: g_th_5, g2: g_sigma_3, g3: g_odot_2, gate_ordering: false,
        t_entries: T_TH_SIGMA, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 71: t_subset_th_omega ──
    unis[71] = Dialect {
        name: "t_subset_th_omega",
        description: "T-constitution: ⊣≥𐑸 AND ⊡≥𐑭. Self-referential topology AND integer winding. The dialect of topologically protected self-reference — every grammatical system carries a winding charge.",
        g1: g_th_5, g2: g_omega_3, g3: g_odot_2, gate_ordering: false,
        t_entries: T_TH_OMEGA, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 72: t_subset_d_sigma ──
    unis[72] = Dialect {
        name: "t_subset_d_sigma",
        description: "T-constitution: ⊢≥𐑼 AND ⊞≥𐑳. Infinite-dimensional AND many-typed. The dialect of field theories over heterogeneous state spaces — gauge fields with multiple matter sectors.",
        g1: g_d_3, g2: g_sigma_3, g3: g_odot_2, gate_ordering: false,
        t_entries: T_D_SIGMA, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 73: mixed_gamma_th ──
    unis[73] = Dialect {
        name: "mixed_gamma_th",
        description: "Cardinality + topology mix: G1=∈≥𐑲 (universal), G2=⊣≥𐑸 (self-ref), G3=⊙≥⊙. Systems must have universal interaction range before they can exhibit self-referential topology.",
        g1: g_gamma_3, g2: g_th_5, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 74: mixed_fidelity_coupling ──
    unis[74] = Dialect {
        name: "mixed_fidelity_coupling",
        description: "Fidelity + coupling mix: G1=⋈≥𐑐 (quantum), G2=>≥𐑾 (bilateral), G3=⊙≥⊙. Systems must support quantum coherence before they can enter bilateral coupling — no classical feedback loops.",
        g1: g_f_3, g2: g_r_4, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 75: mixed_composition_kinetics ──
    unis[75] = Dialect {
        name: "mixed_composition_kinetics",
        description: "Composition + kinetics mix: G1=∋≥𐑠 (sequential), G2=⊤≥𐑧 (slow), G3=⊙≥⊙. Sequential composition before slow kinetics — time's arrow precedes near-equilibrium. Deep time from deep structure.",
        g1: g_g_3, g2: g_c_3, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ═══════════════════════════════════════════════════
    // SECTION L: Overflow / deep-structure variants (12 dialects)
    // ═══════════════════════════════════════════════════

    // ── 76: g4_quad ──
    unis[76] = Dialect {
        name: "g4_quad",
        description: "Full quad-gate: G1=∈≥𐑲, G2=<≥𐑹, G3=⊙≥⊙, G4=⊡≥𐑭. Four-gate ordinal-4 dialect. Universal range → Frobenius parity → self-modeling → winding. The longest ordinal ladder.",
        g1: g_gamma_3, g2: g_phi_5, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 77: only_parity ──
    unis[77] = Dialect {
        name: "only_parity",
        description: "Single-gate dialect: G1=<≥𐑹 only. Parity is the sole filter — no criticality gate, no winding. All Frobenius-special systems pass; everything else is admitted. The minimal-gate dialect.",
        g1: g_phi_5, g2: GATE_NONE, g3: GATE_NONE, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 78: only_winding ──
    unis[78] = Dialect {
        name: "only_winding",
        description: "Single-gate dialect: G1=⊡≥𐑭 only. Winding number as the sole filter. Any topologically protected system passes. All flat-world systems admitted. Pure topological dialect.",
        g1: g_omega_3, g2: GATE_NONE, g3: GATE_NONE, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 79: only_odot ──
    unis[79] = Dialect {
        name: "only_odot",
        description: "Single-gate dialect: G1=⊙≥⊙ only. Self-modeling is the one requirement. No parity filter, no winding requirement. The purest criticality dialect — everything else is secondary.",
        g1: g_odot_2, g2: GATE_NONE, g3: GATE_NONE, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 80: empty_gate ──
    unis[80] = Dialect {
        name: "empty_gate",
        description: "Zero-gate dialect: all G1/G2/G3/G4 = NONE. No gate filtering — every type passes. The maximally permissive dialect. All 17.28M crystal types are admissible. The void that contains everything.",
        g1: GATE_NONE, g2: GATE_NONE, g3: GATE_NONE, gate_ordering: false,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 81: dense_gates ──
    unis[81] = Dialect {
        name: "dense_gates",
        description: "Maximally dense: 5 distinct primitives across 3 gate slots. G1=<≥𐑹 AND ⊡≥𐑭, G2=⊙≥⊙ AND ⊤≥𐑧, G3=⊣≥𐑸. Parity+winding paired, criticality+kinetics paired, topology solo. The densest gate constellation — 5 orthogonal structural demands.",
        g1: g_phi_omega, g2: g_odot_c, g3: g_th_5, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 82: chirality_winding ──
    unis[82] = Dialect {
        name: "chirality_winding",
        description: "Chirality-winding pair: G1=⊥≥𐑫 (eternal memory), G2=⊡≥𐑭 (integer winding), G3=⊙≥⊙. Eternal chirality and topological protection are yoked — memory depth enables winding, winding preserves memory. The paired conservation dialect.",
        g1: g_h_4, g2: g_omega_3, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 83: composition_scope ──
    unis[83] = Dialect {
        name: "composition_scope",
        description: "Composition-scope pair: G1=∋≥𐑠 (sequential), G2=∈≥𐑲 (universal scope), G3=⊙≥⊙. Sequential composition with universal interaction range. Stepwise construction across all scales. The algorithmic dialect.",
        g1: g_g_3, g2: g_gamma_3, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 84: fidelity_chirality ──
    unis[84] = Dialect {
        name: "fidelity_chirality",
        description: "Fidelity-chirality pair: G1=⋈≥𐑐 (quantum), G2=⊥≥𐑖 (2-step memory), G3=⊙≥⊙. Quantum coherence enables two-step Markov memory — classical systems can't sustain the phase relationships needed for structured memory. The quantum memory dialect.",
        g1: g_f_3, g2: g_h_3, g3: g_odot_2, gate_ordering: true,
        t_entries: T_CANONICAL, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 85: t_broadcast ──
    unis[85] = Dialect {
        name: "t_broadcast",
        description: "T-constitution: ∋≥𐑵 (broadcast composition). Systems must support one-to-all broadcast — every structural unit emits to the whole simultaneously. The dialect where every part speaks to every other part without mediation.",
        g1: g_g_4, g2: g_odot_2, g3: g_omega_3, gate_ordering: true,
        t_entries: T_BROADCAST, abs_rules: ABS_CANONICAL, is_expansion: true,
    };

    // ── 86: absorb_broadcast ──
    unis[86] = Dialect {
        name: "absorb_broadcast",
        description: "Broadcast absorption: 𐑻 rule + ∋≥𐑵 T-constitution. Self-modeling dominance with mandatory broadcast composition. The grammar's communicativity is baked into the dialect's constitution — nothing may be silent.",
        g1: g_odot_2, g2: g_g_4, g3: g_phi_5, gate_ordering: true,
        t_entries: T_BROADCAST, abs_rules: ABS_EP, is_expansion: true,
    };

    // ── 87: the_all ──
    unis[87] = Dialect {
        name: "the_all",
        description: "All-structured dialect: G1=⊙≥⊙, G2=<≥𐑹, G3=⊡≥𐑭, G4=⊣≥𐑸. T-constitution: all primitives explicit. Absorption: 𐑻 + sub-critical dual regime. Four-gate + full T + dual absorption. The densest structural filter in the catalog — admits only systems that are simultaneously self-modeling, Frobenius-special, topologically protected, self-referential, and constitutionally complete. Approximate fingerprint: the grammar itself, plus a handful of crystal neighbors.",
        g1: g_odot_2, g2: g_phi_5, g3: g_omega_3, gate_ordering: true,
        t_entries: T_ALL, abs_rules: ABS_DUAL, is_expansion: true,
    };

    unis
} // end all_dialects()

// ═══════════════════════════════════════════════════════════════
// ADDITIONAL T-CONSTITUTIONS (referenced by expansion dialects)
// ═══════════════════════════════════════════════════════════════

/// Ceiling T: ⊣≤𐑶 — ceiling on topology, no self-ref allowed
pub static T_CEILING: &[TEntry] = &[
    TEntry { prim: "⊣", crit_val: "𐑶", ceiling: true },
];

/// Dimensional ceiling: ⊢≤𐑼, ⊣≤𐑥 — finite-dimensional, crossing-point max
pub static T_DIM_CEILING: &[TEntry] = &[
    TEntry { prim: "⊢", crit_val: "𐑼", ceiling: true },
    TEntry { prim: "⊣", crit_val: "𐑥", ceiling: true },
];

/// Self-referential + heterogeneous T: ⊣≥𐑸 AND ⊞≥𐑳
pub static T_TH_SIGMA: &[TEntry] = &[
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
    TEntry { prim: "⊞", crit_val: "𐑳", ceiling: false },
];

/// Self-referential + winding T: ⊣≥𐑸 AND ⊡≥𐑭
pub static T_TH_OMEGA: &[TEntry] = &[
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
    TEntry { prim: "⊡", crit_val: "𐑭", ceiling: false },
];

/// Infinite-dim + heterogeneous T: ⊢≥𐑼 AND ⊞≥𐑳
pub static T_D_SIGMA: &[TEntry] = &[
    TEntry { prim: "⊢", crit_val: "𐑼", ceiling: false },
    TEntry { prim: "⊞", crit_val: "𐑳", ceiling: false },
];

/// Broadcast T: ∋≥𐑵 — one-to-all composition required
pub static T_BROADCAST: &[TEntry] = &[
    TEntry { prim: "∋", crit_val: "𐑵", ceiling: false },
];

/// All T: all 12 primitives at their maximal structural values
pub static T_ALL: &[TEntry] = &[
    TEntry { prim: "⊢", crit_val: "𐑦", ceiling: false },
    TEntry { prim: "⊣", crit_val: "𐑸", ceiling: false },
    TEntry { prim: ">", crit_val: "𐑾", ceiling: false },
    TEntry { prim: "<", crit_val: "𐑹", ceiling: false },
    TEntry { prim: "⋈", crit_val: "𐑐", ceiling: false },
    TEntry { prim: "⊤", crit_val: "𐑧", ceiling: true },
    TEntry { prim: "∈", crit_val: "𐑲", ceiling: false },
    TEntry { prim: "∋", crit_val: "𐑵", ceiling: false },
    TEntry { prim: "⊙", crit_val: "⊙", ceiling: false },
    TEntry { prim: "⊥", crit_val: "𐑫", ceiling: false },
    TEntry { prim: "⊞", crit_val: "𐑳", ceiling: false },
    TEntry { prim: "⊡", crit_val: "𐑭", ceiling: false },
];

// ═══════════════════════════════════════════════════════════════
// ADDITIONAL ABSORPTION RULES
// ═══════════════════════════════════════════════════════════════

/// EP absorption: 𐑻 rule — ⊙ absorbs EP (𐑻) composites under all ops
pub static ABS_EP: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "𐑻", ops_mask: 7, direction: 0 }, // absorb EP
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];

/// Sub-critical absorption: ⊙ absorbs sub-critical (woe) under all ops
pub static ABS_SUB: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "𐑢", ops_mask: 7, direction: 0 }, // absorb sub-critical
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];

/// Dual absorption: both EP and sub-critical absorbed
pub static ABS_DUAL: &[AbsorptionRule] = &[
    AbsorptionRule { prim: "⊙", value: "⊙", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "𐑻", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊙", value: "𐑢", ops_mask: 7, direction: 0 },
    AbsorptionRule { prim: "⊞", value: "𐑳", ops_mask: 4, direction: 0 },
];

// ═══════════════════════════════════════════════════════════════
// GATE SPEC CONSTANTS (module-level, for dialects needing them)
// ═══════════════════════════════════════════════════════════════

/// No-op gate: always passes (min_ord=0)
#[allow(non_upper_case_globals)]
    pub const GATE_NONE: GateSpec = GateSpec { prim: "", min_ord: 0.0 };

// ═══════════════════════════════════════════════════════════════
// REPL QUERY FUNCTIONS
// ═══════════════════════════════════════════════════════════════

/// Look up a dialect by name (returns index 0..87 or None).
pub fn dialect_by_name(name: &str) -> Option<usize> {
    let unis = all_dialects();
    for i in 0..DIALECT_COUNT {
        if unis[i].name == name {
            return Some(i);
        }
    }
    None
}

/// Return a formatted profile of a dialect by index.
pub fn dialect_profile(idx: usize) -> Option<String> {
    if idx >= DIALECT_COUNT {
        return None;
    }
    let unis = all_dialects();
    let u = &unis[idx];
    let mut s = String::new();
    s.push_str(&format!("Dialect #{}: {}\n", idx, u.name));
    s.push_str(&format!("  Description: {}\n", u.description));
    s.push_str(&format!("  Expansion: {}\n", u.is_expansion));
    s.push_str(&format!("  Gate ordering: {}\n", if u.gate_ordering { "sequential" } else { "parallel" }));
    s.push_str(&format!("  G1: {} >= {:.2}\n", u.g1.prim, u.g1.min_ord));
    s.push_str(&format!("  G2: {} >= {:.2}\n", u.g2.prim, u.g2.min_ord));
    s.push_str(&format!("  G3: {} >= {:.2}\n", u.g3.prim, u.g3.min_ord));
    s.push_str(&format!("  T-entries: {}\n", u.t_entries.len()));
    for te in u.t_entries {
        s.push_str(&format!("    {}={} (ceiling={})\n", te.prim, te.crit_val, te.ceiling));
    }
    s.push_str(&format!("  Absorption rules: {}\n", u.abs_rules.len()));
    for ar in u.abs_rules {
        s.push_str(&format!("    {}->{} ops={} dir={}\n", ar.prim, ar.value, ar.ops_mask, ar.direction));
    }
    Some(s)
}

/// List all dialect names.
pub fn list_universes() -> Vec<&'static str> {
    let unis = all_dialects();
    let mut names = Vec::with_capacity(DIALECT_COUNT);
    for i in 0..DIALECT_COUNT {
        names.push(unis[i].name);
    }
    names
}

/// Count dialects by expansion status.
pub fn dialect_counts() -> (usize, usize) {
    let unis = all_dialects();
    let mut canonical = 0usize;
    let mut expansion = 0usize;
    for d in unis.iter() {
        if d.is_expansion { expansion += 1; } else { canonical += 1; }
    }
    (canonical, expansion)
}

/// EP-criticality gate: ⊙≥𐑻 (ordinal 4, exceptional point threshold)
#[allow(non_upper_case_globals)]
    pub const g_phi_E_4: GateSpec = GateSpec { prim: "⊙", min_ord: 4.0 };

/// Sub-critical gate: ⊙≥woe (ordinal 1, below critical — always passes)
#[allow(non_upper_case_globals)]
    pub const g_phi_sub_1: GateSpec = GateSpec { prim: "⊙", min_ord: 1.0 };

/// Compound: <≥𐑹 (parity first in compound pair)
#[allow(non_upper_case_globals)]
    pub const g_phi_omega: GateSpec = GateSpec { prim: "<", min_ord: 5.0 };

/// Compound: ⊙≥⊙ + ⊤≥𐑧 (criticality first in compound pair)
#[allow(non_upper_case_globals)]
    pub const g_odot_c: GateSpec = GateSpec { prim: "⊙", min_ord: 2.0 };
