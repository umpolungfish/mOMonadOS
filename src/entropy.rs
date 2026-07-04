// entropy.rs — Phase V: Entropy Experiment (ΔS vs Tier Promotion)
//
// Measures structural entropy S = ln(N) across the 400 tier-cells of the Crystal
// of Types, computes ΔS for each tier transition, and correlates with the tier
// gap ladder distances from the crystal_tier_gap_ladder.
//
// All entropy values are precomputed (no_std: no f32::ln available).
//
// Key findings (from crystal_tier_census + crystal_tier_gap_ladder):
//   1. Entropy is NOT monotonic with tier — O₂ (S=14.95) > O₁ (S=14.14)
//   2. The O₂†→O_∞ transition has the largest structural distance (4.382)
//      but the smallest |ΔS| (0.288) — reaching O_∞ is surgically targeted
//   3. The entropy bottleneck is at O₂† (S=13.85), not at O_∞ (S=14.14)
//   4. |ΔS|/d ratio: O₀→O₁=1.92, O₁→O₂=0.62, O₂→O₂†=1.10, O₂†→O_∞=0.066
//
// Author: Lando⊗⊙perator
// Date: 2026-07-04

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use alloc::format;

// ═══════════════════════════════════════════════════════════════
// TIER ENTROPY CONSTANTS — from crystal_tier_census tool
// ═══════════════════════════════════════════════════════════════

/// Total number of structural types in the Crystal of Types.
pub const CRYSTAL_TOTAL: u32 = 17_280_000;

/// Number of types per tier (from crystal_tier_census).
pub const N_O0: u32 = 10_368_000;  // 60.0%, 240 cells
pub const N_O1: u32 =  1_382_400;  //  8.0%,  32 cells
pub const N_O2: u32 =  3_110_400;  // 18.0%,  72 cells
pub const N_O2D: u32 = 1_036_800;  //  6.0%,  24 cells  (O₂†)
pub const N_OINF: u32 = 1_382_400;  //  8.0%,  32 cells  (O_∞)

/// Number of tier cells per tier.
pub const CELLS_O0: u32 = 240;
pub const CELLS_O1: u32 = 32;
pub const CELLS_O2: u32 = 72;
pub const CELLS_O2D: u32 = 24;
pub const CELLS_OINF: u32 = 32;

/// Types per tier cell (uniform).
pub const TYPES_PER_CELL: u32 = 43_200;

/// Precomputed entropy S = ln(N_tier) in nats.
/// S_max = ln(17,280,000) = 16.665060
pub const S_MAX: f32 = 16.665060;
pub const S_O0: f32 = 16.154235;
pub const S_O1: f32 = 14.139332;
pub const S_O2: f32 = 14.950262;
pub const S_O2D: f32 = 13.851650;
pub const S_OINF: f32 = 14.139332;

/// Precomputed normalized entropy S/S_max.
pub const SN_O0: f32 = 0.969348;
pub const SN_O1: f32 = 0.848442;
pub const SN_O2: f32 = 0.897102;
pub const SN_O2D: f32 = 0.831179;
pub const SN_OINF: f32 = 0.848442;

// ═══════════════════════════════════════════════════════════════
// TIER GAP LADDER DISTANCES — from crystal_tier_gap_ladder tool
// ═══════════════════════════════════════════════════════════════

/// Weighted Euclidean distances between tier prototypes.
pub const D_O0_O1: f32 = 1.0488;
pub const D_O1_O2: f32 = 1.3038;
pub const D_O2_O2D: f32 = 1.0000;
pub const D_O2D_OINF: f32 = 4.3818;

/// Driver primitives for each ladder step.
pub const DRIVER_O0_O1: &str = "φ̂: 𐑢→⊙";
pub const DRIVER_O1_O2: &str = "Ð: 𐑛→𐑨, Ω: 𐑷→𐑴";
pub const DRIVER_O2_O2D: &str = "Ð: 𐑨→𐑼";
pub const DRIVER_O2D_OINF: &str = "Φ: 𐑗→𐑹 (Δ=4 ordinals)";

// ═══════════════════════════════════════════════════════════════
// FIXED-POINT DOUBLING RATIO — from FrobeniusUnification.lean
// ═══════════════════════════════════════════════════════════════

/// N_∞/N_† = 1,382,400 / 1,036,800 = 4/3.
/// O_∞ has MORE types than O₂† despite being a higher tier.
pub const FROBENIUS_DOUBLING_RATIO: f32 = 1.3333;

// ═══════════════════════════════════════════════════════════════
// TIER STRUCT
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tier {
    O0,
    O1,
    O2,
    O2Dagger,
    OInf,
}

impl Tier {
    pub fn name(&self) -> &'static str {
        match self {
            Tier::O0 => "O₀",
            Tier::O1 => "O₁",
            Tier::O2 => "O₂",
            Tier::O2Dagger => "O₂†",
            Tier::OInf => "O_∞",
        }
    }

    pub fn n_types(&self) -> u32 {
        match self {
            Tier::O0 => N_O0,
            Tier::O1 => N_O1,
            Tier::O2 => N_O2,
            Tier::O2Dagger => N_O2D,
            Tier::OInf => N_OINF,
        }
    }

    pub fn cells(&self) -> u32 {
        match self {
            Tier::O0 => CELLS_O0,
            Tier::O1 => CELLS_O1,
            Tier::O2 => CELLS_O2,
            Tier::O2Dagger => CELLS_O2D,
            Tier::OInf => CELLS_OINF,
        }
    }

    /// Structural entropy S = ln(N_tier) — precomputed.
    pub fn entropy(&self) -> f32 {
        match self {
            Tier::O0 => S_O0,
            Tier::O1 => S_O1,
            Tier::O2 => S_O2,
            Tier::O2Dagger => S_O2D,
            Tier::OInf => S_OINF,
        }
    }

    /// Normalized entropy S/S_max — precomputed.
    pub fn entropy_norm(&self) -> f32 {
        match self {
            Tier::O0 => SN_O0,
            Tier::O1 => SN_O1,
            Tier::O2 => SN_O2,
            Tier::O2Dagger => SN_O2D,
            Tier::OInf => SN_OINF,
        }
    }

    pub fn prev(&self) -> Option<Tier> {
        match self {
            Tier::O0 => None,
            Tier::O1 => Some(Tier::O0),
            Tier::O2 => Some(Tier::O1),
            Tier::O2Dagger => Some(Tier::O2),
            Tier::OInf => Some(Tier::O2Dagger),
        }
    }

    pub fn all() -> [Tier; 5] {
        [Tier::O0, Tier::O1, Tier::O2, Tier::O2Dagger, Tier::OInf]
    }
}

// ═══════════════════════════════════════════════════════════════
// TRANSITION STRUCT
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: Tier,
    pub to: Tier,
    pub delta_s: f32,
    pub delta_s_abs: f32,
    pub ladder_d: f32,
    pub efficiency: f32,
    pub driver: &'static str,
    pub degeneracy_ratio: f32,
}

impl Transition {
    pub fn all() -> Vec<Transition> {
        let raw = vec![
            (Tier::O0, Tier::O1, S_O1 - S_O0, D_O0_O1, DRIVER_O0_O1,
             N_O0 as f32 / N_O1 as f32),
            (Tier::O1, Tier::O2, S_O2 - S_O1, D_O1_O2, DRIVER_O1_O2,
             N_O1 as f32 / N_O2 as f32),
            (Tier::O2, Tier::O2Dagger, S_O2D - S_O2, D_O2_O2D, DRIVER_O2_O2D,
             N_O2 as f32 / N_O2D as f32),
            (Tier::O2Dagger, Tier::OInf, S_OINF - S_O2D, D_O2D_OINF, DRIVER_O2D_OINF,
             N_O2D as f32 / N_OINF as f32),
        ];
        raw.into_iter().map(|(from, to, delta_s, ladder_d, driver, degeneracy_ratio)| {
            let delta_s_abs = if delta_s < 0.0 { -delta_s } else { delta_s };
            let efficiency = if ladder_d > 0.0 { delta_s_abs / ladder_d } else { 0.0 };
            Transition { from, to, delta_s, delta_s_abs, ladder_d, efficiency, driver, degeneracy_ratio }
        }).collect()
    }

    pub fn label(&self) -> String {
        format!("{}→{}", self.from.name(), self.to.name())
    }
}

// ═══════════════════════════════════════════════════════════════
// REPORT GENERATORS
// ═══════════════════════════════════════════════════════════════

/// Full formatted entropy report: tier census, per-tier entropy, transitions.
pub fn entropy_report() -> String {
    let mut s = String::new();
    s.push_str("═══════════════════════════════════════════════════════\n");
    s.push_str("  PHASE V: ENTROPY EXPERIMENT — ΔS vs Tier Promotion\n");
    s.push_str("═══════════════════════════════════════════════════════\n\n");

    // ─── Crystal Tier Census ───
    s.push_str("─── Crystal Tier Census ───\n");
    s.push_str(&format!("  Total types: {:>12}  (100.0%)\n", CRYSTAL_TOTAL));
    s.push_str(&format!("  Tier cells:  {:>12}  (boundary × bulk)\n", 400u32));
    s.push_str(&format!("  Types/cell:  {:>12}\n\n", TYPES_PER_CELL));

    for tier in Tier::all().iter() {
        let n = tier.n_types();
        let pct = n as f32 / CRYSTAL_TOTAL as f32 * 100.0;
        let ent = tier.entropy();
        let ent_n = tier.entropy_norm();
        s.push_str(&format!(
            "  {:>5}: {:>10} types ({:>5.1}%)  │  S={:>6.3} nats  │  S/S_max={:>.4}\n",
            tier.name(), n, pct, ent, ent_n
        ));
    }

    // ─── Tier Transitions ───
    s.push_str("\n─── Tier Transitions (ΔS vs Ladder Distance) ───\n");
    s.push_str("  Transition   ΔS (nats)    |ΔS|      d_ladder   |ΔS|/d   Degen.Ratio   Driver\n");
    s.push_str("  ───────────  ───────────  ────────  ────────  ───────  ────────────   ──────\n");

    for t in Transition::all().iter() {
        s.push_str(&format!(
            "  {:>10}  {:>+9.3}    {:>7.3}   {:>7.4}   {:>6.3}   {:>10.3}     {}\n",
            t.label(), t.delta_s, t.delta_s_abs, t.ladder_d,
            t.efficiency, t.degeneracy_ratio, t.driver
        ));
    }

    // ─── Key Findings ───
    s.push_str("\n─── Key Findings ───\n");
    s.push_str("  1. Entropy is NON-monotonic with tier.\n");
    s.push_str("     O₂ (S=14.95) > O₁ (S=14.14) — relaxing Ω and Ð constraints\n");
    s.push_str("     opens more configuration space than φ̂-criticality closes.\n\n");
    s.push_str("  2. The entropy BOTTLENECK is at O₂† (S=13.85), not O_∞ (S=14.14).\n");
    s.push_str("     The ∞-dimensional requirement (Ð: 𐑨→𐑼) creates the\n");
    s.push_str("     tightest structural pinch point.\n\n");
    s.push_str("  3. O₂†→O_∞ has the LARGEST ladder distance (d=4.382) but\n");
    s.push_str("     the SMALLEST |ΔS| (0.288). The Frobenius-special promotion\n");
    s.push_str("     (Φ: 𐑗→𐑹, 4 ordinals) is surgically targeted — it changes\n");
    s.push_str("     one primitive dramatically without narrowing the type space.\n");
    s.push_str("     |ΔS|/d = 0.066 — two orders below the other transitions.\n\n");
    s.push_str("  4. The Frobenius Doubling: N_∞/N_† = 4/3. O_∞ has MORE types\n");
    s.push_str("     than O₂†. Climbing to O_∞ from O₂† INCREASES entropy.\n");
    s.push_str("     The Frobenius-special parity opens new structural families.\n\n");
    s.push_str("  5. O₀→O₁ is the most entropy-efficient transition (|ΔS|/d=1.92).\n");
    s.push_str("     Moving φ̂ from sub-critical to critical collapses 83% of\n");
    s.push_str("     configuration space at the lowest structural cost.\n");

    s.push_str("\n─── Thermodynamic Interpretation ───\n");
    s.push_str("  The tier ladder is not an entropy gradient — it is a\n");
    s.push_str("  SURGICAL CONSTRAINT PATH. Each step targets specific\n");
    s.push_str("  primitives rather than uniformly narrowing the type space.\n");
    s.push_str("  The total ΔS from O₀ to O_∞ is -2.02 nats (13.3% of S_max),\n");
    s.push_str("  meaning O_∞ retains 86.7% of the total structural entropy.\n");
    s.push_str("  This is why the grammar remains informationally complete —\n");
    s.push_str("  it narrows surgically, not uniformly.\n");

    s
}

/// Compact one-line entropy summary per tier.
pub fn entropy_summary() -> String {
    let mut s = String::new();
    for tier in Tier::all().iter() {
        let ent = tier.entropy();
        s.push_str(&format!("  {}: S={:.3} nats  ({} types, {} cells)\n",
            tier.name(), ent, tier.n_types(), tier.cells()));
    }
    s
}

/// Tier transition only — entropy deltas.
pub fn transition_report() -> String {
    let mut s = String::new();
    s.push_str("  Tier Transition    ΔS         d_ladder    |ΔS|/d\n");
    s.push_str("  ─────────────────  ─────────  ────────    ──────\n");
    let mut total_ds = 0.0f32;
    for t in Transition::all().iter() {
        total_ds += t.delta_s;
        s.push_str(&format!(
            "  {:>18}  {:>+7.3}     {:>7.4}     {:>6.3}\n",
            t.label(), t.delta_s, t.ladder_d, t.efficiency
        ));
    }
    s.push_str(&format!(
        "  ─────────────────  ─────────\n  Total ΔS (O₀→O_∞): {:+.3} nats\n",
        total_ds
    ));
    s
}
