// iuft_teichmuller.rs — Inter-Universal Teichmüller Theory ↔ IUFT QC Gate Bridge
//
// Connects the IUFT QC gate encoding (12→3 Euler-angle SU(2) projection)
// to Inter-universal Teichmüller Theory (IUTT): promotion paths between
// Frobenius-closed universes become trajectories through SU(2) gate space.
//
// Core bridge:
//   Teichmüller deformation = promotion path preserving Frobenius structure
//   Promotion signature [<,∋,⊥,⊡] → gate parameter deltas
//     < (Parity)  → φ (azimuthal)
//     ⊥ (Chirality) → φ (azimuthal)
//     ⊡ (Winding) → θ (latitude)
//     ∋ (Composition) → latent (affects structure but not the 3 encoded angles)
//
//   Étale deformation: pinned primitives (P,F,K,G,Gm,Ph) unchanged
//   Anabelian deformation: core Frobenius structure transforms
//
//   The Great P-Gap: O₂† → O_∞ transition (d=4.38) driven by < — the
//   Frobenius-special symmetry emergence — maps to the largest single
//   jump in gate space.
//
// Reference:
//   - ig-docs/iuft_landscape_complete.md
//   - src/iuft_v2.py (teichmuller_deformation function)
//   - IUTT: Mochizuki's Inter-universal Teichmüller Theory

#![allow(dead_code)]

use alloc::vec::Vec;
use alloc::string::String;

use crate::iuft_qc::{IuftQcGate, encode_entry, gate_for, gate_distance};
use crate::imas_ig::{IgPrim, IgTuple};
use crate::sprintln;

// ═══════════════════════════════════════════════════════════════
// DATA TYPES
// ═══════════════════════════════════════════════════════════════

/// Deformation classification.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeformationType {
    /// Pinned primitives unchanged — structure-preserving deformation.
    Etale,
    /// Core Frobenius structure transforms — the "usual" case.
    Anabelian,
}

/// A single primitive promotion/demotion step in a Teichmüller path.
#[derive(Clone, Debug)]
pub struct PromotionStep {
    pub primitive: &'static str,  // Family name: "<", "∋", "⊥", "⊡", etc.
    pub from_ord: f32,
    pub to_ord: f32,
    pub delta: i32,               // Positive = promotion, negative = demotion
}

/// A Teichmüller deformation path between two Frobenius universes,
/// encoded as a trajectory through SU(2) gate space.
#[derive(Clone, Debug)]
pub struct TeichmullerPath {
    /// Source universe name.
    pub source: String,
    /// Target universe name.
    pub target: String,
    /// Source IUFT gate encoding.
    pub source_gate: IuftQcGate,
    /// Target IUFT gate encoding.
    pub target_gate: IuftQcGate,
    /// Promotion/demotion steps along the path.
    pub steps: Vec<PromotionStep>,
    /// Deformation type.
    pub deformation_type: DeformationType,
    /// Gate parameter deltas: (Δθ°, Δφ°, Δψ°).
    pub gate_delta: (f64, f64, f64),
    /// Number of interpolated trajectory points (excluding endpoints).
    pub trajectory_points: usize,
}

/// Tier transition data — the gate-space jump between ouroboricity tiers.
#[derive(Clone, Debug)]
pub struct TierTransition {
    pub from_tier: &'static str,
    pub to_tier: &'static str,
    pub distance: f64,             // Crystal lattice distance
    pub driver_primitive: &'static str,
    pub gate_jump: (f64, f64, f64), // (Δθ°, Δφ°, Δψ°)
    pub is_p_gap: bool,            // The Great P-Gap: O₂† → O_∞
}

// ═══════════════════════════════════════════════════════════════
// TIER → GATE MAPPING
// ═══════════════════════════════════════════════════════════════

/// Map an ouroboricity tier name to a numeric score.
pub fn tier_score(tier: &str) -> f32 {
    match tier {
        "O_0" | "O0" => 0.0,
        "O_1" | "O1" => 1.0,
        "O_2" | "O2" => 2.0,
        "O_2d" | "O2d" | "O_2†" | "O2t" => 3.0,
        "O_inf" | "O_∞" | "Oinf" => 4.0,
        _ => 0.0,
    }
}

/// The Great P-Gap: O₂† → O_∞ transition.
/// d=4.38, driven by < (parity), weighted²=19.2.
/// This is the largest structural discontinuity on the crystal.
pub const P_GAP_TRANSITION: TierTransition = TierTransition {
    from_tier: "O₂†",
    to_tier: "O_∞",
    distance: 4.38,
    driver_primitive: "<",
    gate_jump: (60.0, 180.0, 0.0),  // θ+60°, φ+180°, ψ unchanged
    is_p_gap: true,
};

/// All known tier transitions with their gate-space jumps.
/// From the crystal tier gap ladder (iuft_landscape_complete.md §3.3).
pub fn tier_transitions() -> &'static [TierTransition] {
    &[
        TierTransition {
            from_tier: "O₀", to_tier: "O₁",
            distance: 1.05, driver_primitive: "⊙",
            gate_jump: (30.0, 45.0, 90.0),  // Birth of self-reference → ψ activates
            is_p_gap: false,
        },
        TierTransition {
            from_tier: "O₁", to_tier: "O₂",
            distance: 1.30, driver_primitive: "⊢+⊡",
            gate_jump: (45.0, 90.0, 0.0),    // Topological expansion
            is_p_gap: false,
        },
        TierTransition {
            from_tier: "O₂", to_tier: "O₂†",
            distance: 1.00, driver_primitive: "⊢",
            gate_jump: (30.0, 0.0, 0.0),     // Dimensional refinement
            is_p_gap: false,
        },
        P_GAP_TRANSITION,
    ]
}

// ═══════════════════════════════════════════════════════════════
// PRIMITIVE → GATE PARAMETER MAPPING
// ═══════════════════════════════════════════════════════════════

/// Which gate parameter a primitive contributes to.
/// The 12→3 encoding groups primitives into angle contributions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GateParam {
    Theta,   // θ: latitude
    Phi,     // φ: azimuthal
    Psi,     // ψ: self-modeling
    Latent,  // Does not directly affect the 3 encoded angles
}

/// Map a primitive family to the gate parameter it controls.
pub fn primitive_to_gate_param(family: &str) -> GateParam {
    match family {
        "⊢" | "⊡" | "⊞" => GateParam::Theta,
        ">" | "<" | "⊥" => GateParam::Phi,
        "⊙" | "Ph" => GateParam::Psi,
        _ => GateParam::Latent,  // ⊣, ⋈, ⊤, ∈, ∋ — carried by the dialect sheaf
    }
}

/// Check if a primitive family is "pinned" (Frobenius-core invariant).
/// Pinned primitives should not change in an étale deformation.
pub fn is_pinned(family: &str) -> bool {
    matches!(family, "<" | "⋈" | "⊤" | "∈" | "∋" | "⊙" | "Ph" | "P" | "F" | "K" | "G" | "Gm")
}

/// The per-step angular contribution of a primitive promotion/demotion
/// to its associated gate parameter.
/// Returns (Δθ°, Δφ°, Δψ°) for a single ordinal step.
fn primitive_step_to_gate_delta(family: &str) -> (f64, f64, f64) {
    match family {
        // θ contributors (share 180° range, 3 families)
        "⊢" => (60.0, 0.0, 0.0),   // 180°/3 families = 60° per full-range step
        "⊡" => (60.0, 0.0, 0.0),
        "⊞" => (60.0, 0.0, 0.0),
        // φ contributors (share 360° range, 3 families)
        ">" => (0.0, 120.0, 0.0),  // 360°/3 families = 120° per full-range step
        "<" => (0.0, 120.0, 0.0),
        "⊥" => (0.0, 120.0, 0.0),
        // ψ contributor
        "⊙" | "Ph" => (0.0, 0.0, 180.0), // Full 180° range
        _ => (0.0, 0.0, 0.0),     // Latent — no direct gate effect
    }
}

// ═══════════════════════════════════════════════════════════════
// TEICHMÜLLER DEFORMATION ENGINE
// ═══════════════════════════════════════════════════════════════

/// Compute the Teichmüller deformation path between two universes.
///
/// A Teichmüller deformation is a promotion path that preserves Frobenius
/// structure while changing ouroboricity tier. In gate space, this becomes
/// a trajectory through SU(2) — the sequence of gate encodings along
/// the promotion path.
/// Look up a gate by name: hardcoded table first, then catalog encode.
fn gate_or_encode(name: &str) -> Option<IuftQcGate> {
    if let Some(g) = gate_for(name) {
        return Some(g);
    }
    // Try catalog encode
    crate::catalog::catalog_entries(None)
        .find(|e| e.name == name)
        .map(|e| encode_entry(e))
}

pub fn teichmuller_path(source_name: &str, target_name: &str) -> Option<TeichmullerPath> {
    // Try hardcoded gate first, then catalog encode
    let src_gate = gate_or_encode(source_name)?;
    let tgt_gate = gate_or_encode(target_name)?;

    // Find catalog entries for primitive-level analysis
    let src_entry = crate::catalog::catalog_entries(None)
        .find(|e| e.name == source_name);
    let tgt_entry = crate::catalog::catalog_entries(None)
        .find(|e| e.name == target_name);

    let mut steps = Vec::new();

    if let (Some(src), Some(tgt)) = (src_entry, tgt_entry) {
        // Compare primitives and record promotion/demotion steps
        compare_primitives(&src.tuple, &tgt.tuple, &mut steps);
    }

    // Classify deformation type
    let pinned_changed = steps.iter()
        .filter(|s| is_pinned(s.primitive) && s.delta != 0)
        .count();
    let def_type = if pinned_changed == 0 {
        DeformationType::Etale
    } else {
        DeformationType::Anabelian
    };

    // Compute gate parameter deltas from steps
    let (mut dtheta, mut dphi, mut dpsi) = (0.0f64, 0.0f64, 0.0f64);
    let mut active_steps = 0usize;
    for step in &steps {
        let max_ord = max_ordinal_for_family(step.primitive);
        let frac = (step.delta.abs() as f64) / (max_ord as f64 - 1.0).max(1.0);
        let (dt, dp, ds) = primitive_step_to_gate_delta(step.primitive);
        let sign = if step.delta > 0 { 1.0 } else { -1.0 };
        dtheta += dt * frac * sign;
        dphi   += dp * frac * sign;
        dpsi   += ds * frac * sign;
        if dt.abs() > 0.0 || dp.abs() > 0.0 || ds.abs() > 0.0 {
            active_steps += 1;
        }
    }

    let trajectory_points = active_steps.max(1);

    Some(TeichmullerPath {
        source: String::from(source_name),
        target: String::from(target_name),
        source_gate: src_gate,
        target_gate: tgt_gate,
        steps,
        deformation_type: def_type,
        gate_delta: (dtheta, dphi, dpsi),
        trajectory_points,
    })
}

/// Compute the Teichmüller gate trajectory: interpolated SU(2) gates
/// along the deformation path.
pub fn teichmuller_trajectory(path: &TeichmullerPath) -> Vec<IuftQcGate> {
    let n = path.trajectory_points;
    if n <= 1 {
        return alloc::vec![path.source_gate, path.target_gate];
    }
    let mut gates = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = (i as f64) / (n as f64);
        let theta = path.source_gate.theta_deg + t * path.gate_delta.0;
        let phi   = path.source_gate.phi_deg   + t * path.gate_delta.1;
        let psi   = path.source_gate.psi_deg   + t * path.gate_delta.2;
        gates.push(IuftQcGate::new(theta, phi, psi));
    }
    gates
}

/// Teichmüller distance in gate space: the projective SU(2) distance
/// between source and target gate encodings.
pub fn teichmuller_gate_distance(source: &str, target: &str) -> Option<f64> {
    gate_distance(source, target)
}

/// Compute the tier transition gate: the gate-space jump between two
/// ouroboricity tiers. Uses the catalog's crystal tier gap ladder.
pub fn tier_transition_gate(from_tier: &str, to_tier: &str) -> Option<TierTransition> {
    tier_transitions().iter()
        .find(|t| t.from_tier == from_tier && t.to_tier == to_tier)
        .cloned()
}

/// Compute gate for a tier: approximate encoding of an ouroboricity tier
/// as an SU(2) gate. Uses the cumulative tier transitions.
pub fn tier_to_gate(tier: &str) -> Option<IuftQcGate> {
    let score = tier_score(tier);
    if score <= 0.0 {
        return Some(IuftQcGate::new(0.0, 0.0, 0.0));
    }

    // Start from ZFC gate (O_0 proxy) and apply tier transitions
    let mut gate = IuftQcGate::new(45.0, 195.0, 0.0); // ZFC baseline

    for t in tier_transitions() {
        if tier_score(t.from_tier) < score {
            gate.theta_deg += t.gate_jump.0;
            gate.phi_deg   += t.gate_jump.1;
            gate.psi_deg   += t.gate_jump.2;
        }
    }

    Some(gate)
}

// ═══════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════

/// Max ordinal value for a primitive family.
fn max_ordinal_for_family(family: &str) -> f32 {
    match family {
        "⊢" => 4.0, "⊣" => 5.0, ">" => 4.0, "<" => 5.0,
        "⋈" => 3.0, "⊤" => 4.5, "∈" => 3.0, "∋" => 4.0,
        "⊙" | "Ph" => 3.0, "⊥" => 4.0, "⊞" => 3.0, "⊡" => 4.0,
        _ => 1.0,
    }
}

/// Compare two IgTuples and record promotion/demotion steps.
fn compare_primitives(src: &IgTuple, tgt: &IgTuple, steps: &mut Vec<PromotionStep>) {
    compare_one("⊢", src.d, tgt.d, steps);
    compare_one("⊣", src.t, tgt.t, steps);
    compare_one(">", src.r, tgt.r, steps);
    compare_one("<", src.p, tgt.p, steps);
    compare_one("⋈", src.f, tgt.f, steps);
    compare_one("⊤", src.k, tgt.k, steps);
    compare_one("∈", src.g, tgt.g, steps);
    compare_one("∋", src.c, tgt.c, steps);
    compare_one("⊙", src.phi, tgt.phi, steps);
    compare_one("⊥", src.h, tgt.h, steps);
    compare_one("⊞", src.s, tgt.s, steps);
    compare_one("⊡", src.omega, tgt.omega, steps);
}

fn compare_one(family: &'static str, a: IgPrim, b: IgPrim, steps: &mut Vec<PromotionStep>) {
    if a == b { return; }
    let from_ord = a.ordinal();
    let to_ord = b.ordinal();
    let delta = libm::roundf(to_ord - from_ord) as i32;
    steps.push(PromotionStep {
        primitive: family,
        from_ord,
        to_ord,
        delta,
    });
}

// ═══════════════════════════════════════════════════════════════
// REPORTERS
// ═══════════════════════════════════════════════════════════════

/// Print a full Teichmüller deformation report.
pub fn print_teichmuller_report(source: &str, target: &str) {
    match teichmuller_path(source, target) {
        Some(path) => {
            sprintln!("══ Teichmüller Deformation Report ══");
            sprintln!("  Source: {}  →  Target: {}", path.source, path.target);
            sprintln!("");
            sprintln!("  Source Gate: θ={:.1}°  φ={:.1}°  ψ={:.1}°",
                path.source_gate.theta_deg, path.source_gate.phi_deg, path.source_gate.psi_deg);
            sprintln!("  Target Gate: θ={:.1}°  φ={:.1}°  ψ={:.1}°",
                path.target_gate.theta_deg, path.target_gate.phi_deg, path.target_gate.psi_deg);
            sprintln!("  Gate Delta:  Δθ={:+.1}°  Δφ={:+.1}°  Δψ={:+.1}°",
                path.gate_delta.0, path.gate_delta.1, path.gate_delta.2);
            sprintln!("  Gate Distance (projective SU(2)): {:.4}",
                path.source_gate.distance_to(&path.target_gate));
            sprintln!("");
            sprintln!("  Deformation Type: {}",
                match path.deformation_type {
                    DeformationType::Etale => "ÉTALE (structure-preserving)",
                    DeformationType::Anabelian => "ANABELIAN (core transforms)",
                });
            sprintln!("");
            if path.steps.is_empty() {
                sprintln!("  No primitive-level steps (identical or unknown tuples).");
            } else {
                sprintln!("  Promotion/Demotion Steps:");
                for step in &path.steps {
                    let dir = if step.delta > 0 { "↑" } else { "↓" };
                    let param = match primitive_to_gate_param(step.primitive) {
                        GateParam::Theta => "θ",
                        GateParam::Phi => "φ",
                        GateParam::Psi => "ψ",
                        GateParam::Latent => "·",
                    };
                    sprintln!("    {} {}: {:.1}→{:.1}  (Δ={})  [→{}]",
                        dir, step.primitive, step.from_ord, step.to_ord, step.delta, param);
                }
            }
            sprintln!("");
            sprintln!("  Trajectory ({} points):", path.trajectory_points + 1);
            let traj = teichmuller_trajectory(&path);
            for (i, g) in traj.iter().enumerate() {
                sprintln!("    {:>2}: θ={:.1}°  φ={:.1}°  ψ={:.1}°",
                    i, g.theta_deg, g.phi_deg, g.psi_deg);
            }
        }
        None => {
            sprintln!("Teichmüller path '{}' → '{}': one or both universes lack IUFT gate encodings.", source, target);
        }
    }
}

/// Print the tier transition ladder with gate-space jumps.
pub fn print_tier_ladder() {
    sprintln!("══ Crystal Tier Ladder → Gate Space ══");
    for t in tier_transitions() {
        let p_gap = if t.is_p_gap { " ★ THE GREAT P-GAP" } else { "" };
        sprintln!("  {} → {}:  d={:.2}  driver={}  gate jump: Δθ={:+.0}° Δφ={:+.0}° Δψ={:+.0}°{}",
            t.from_tier, t.to_tier, t.distance, t.driver_primitive,
            t.gate_jump.0, t.gate_jump.1, t.gate_jump.2, p_gap);
    }
}

/// Print all known Teichmüller paths between canonical universes.
pub fn print_canonical_paths() {
    let universes = ["monad", "topos", "poincare_hopf_theorem", "imscribing_grammar", "CLINK L8"];
    sprintln!("══ Canonical Teichmüller Paths ══");
    sprintln!("{:>22} → {:<22}  {:>7}  {:>8}  {:>10}",
        "Source", "Target", "d(gate)", "Δθ", "Type");
    sprintln!("{:-<80}", "");
    for src in &universes {
        for tgt in &universes {
            if src == tgt { continue; }
            if let Some(path) = teichmuller_path(src, tgt) {
                let d = path.source_gate.distance_to(&path.target_gate);
                let dt = match path.deformation_type {
                    DeformationType::Etale => "étale",
                    DeformationType::Anabelian => "anabel",
                };
                sprintln!("  {:>22} → {:<22}  {:.4}   {:+.0}°     {}",
                    path.source, path.target, d, path.gate_delta.0, dt);
            }
        }
    }
}
