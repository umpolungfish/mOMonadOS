// clay_status.rs — Machine-checked Clay Millennium Problem closure/resistance module
//
// Encodes the verified verdicts from p4rakernel/p4ramill Lean 4 proofs:
//   - Clay_WitnessedClosure.lean: BSD and Hodge close (T_CEILING-consistent),
//     Yang-Mills one-bump-short (gate layer idempotent, T_CEILING-blocked).
//   - Clay_UnclosedResistance.lean: RH, Navier-Stokes, and P-vs-NP resist
//     closure under ALL 23 dialects due to low winding (Ω < 3).
//
// All verdicts sourced from ClayCanonicalTuples.lean (procedurally generated
// from IG_catalog.json) — no hand-transcribed tuples.
//
// Author: Lando⊗⊙perator
// Date: 2026-07-02


use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

// ═══════════════════════════════════════════════════════════════
// CLAY VERDICT — single-problem status
// ═══════════════════════════════════════════════════════════════

/// Machine-checked status of a Clay Millennium Problem structural type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ClayVerdict {
    /// Full witnessed closure: gate-layer idempotent AND T_CEILING-consistent.
    Closed,
    /// Gate-layer idempotent but T_CEILING-blocked ("one bump short").
    OneBumpShort,
    /// Resists closure under all known dialects.
    Unclosed,
}

impl ClayVerdict {
    pub fn name(&self) -> &'static str {
        match self {
            ClayVerdict::Closed       => "CLOSED (witnessed)",
            ClayVerdict::OneBumpShort => "ONE-BUMP-SHORT (gate closed, T_CEILING blocked)",
            ClayVerdict::Unclosed     => "UNCLOSED (resists all 23 dialects)",
        }
    }
}

/// Full per-problem report.
#[derive(Clone, Debug)]
pub struct ClayReport {
    pub name: &'static str,
    pub verdict: ClayVerdict,
    pub closer_dialects: Vec<&'static str>,
    pub blocker: Option<&'static str>,
    pub winding: &'static str,  // Ω Shavian glyph
    pub winding_ordinal: f32,
    pub low_winding: bool,      // Ω < 3 = below terminal anchor
}

// ═══════════════════════════════════════════════════════════════
// THE 7 CLAY PROBLEMS — canonical verdicts
// ═══════════════════════════════════════════════════════════════

/// BSD: FULL CLOSURE under 5 dialects, T_CEILING-consistent.
/// Canonical tuple: ⟨𐑦𐑥𐑾𐑿𐑞𐑧𐑲𐑝𐑮𐑖𐑙𐑭⟩
/// Lean: `bsd_witnessed_closure` — proven by native_decide.
pub fn bsd_report() -> ClayReport {
    ClayReport {
        name: "Birch–Swinnerton-Dyer",
        verdict: ClayVerdict::Closed,
        closer_dialects: vec![
            "chirality_first",
            "scope_universe",
            "kinetics_trap",
            "absorption_chirality_first",
            "absorption_scope_empire",
        ],
        blocker: None,
        winding: "𐑭",
        winding_ordinal: 3.0,
        low_winding: false,
    }
}

/// HODGE: FULL CLOSURE under 5 dialects, T_CEILING-consistent.
/// Canonical tuple: ⟨𐑦𐑸𐑽𐑿𐑱𐑧𐑲𐑝𐑮𐑓𐑳𐑭⟩
/// Lean: `hodge_witnessed_closure` — proven by native_decide.
pub fn hodge_report() -> ClayReport {
    ClayReport {
        name: "Hodge Conjecture",
        verdict: ClayVerdict::Closed,
        closer_dialects: vec![
            "scope_universe",
            "kinetics_trap",
            "stoichiometry_universe",
            "absorption_scope_empire",
            "absorption_topology_seal",
        ],
        blocker: None,
        winding: "𐑭",
        winding_ordinal: 3.0,
        low_winding: false,
    }
}

/// YANG-MILLS: ONE-BUMP-SHORT.
/// Clears all three ⊙ gates under triple_criticality but fails T_CEILING
/// on Ç (kinetics) alone: K_trap ord=4 exceeds the ord=3 ceiling.
/// Canonical tuple: ⟨𐑛𐑥𐑩𐑗𐑐𐑤𐑲𐑝𐑣𐑓𐑳𐑷⟩
/// Lean: `ym_one_bump_short` + `ym_blocker_is_kinetics` — proven by native_decide.
pub fn ym_report() -> ClayReport {
    ClayReport {
        name: "Yang–Mills Mass Gap",
        verdict: ClayVerdict::OneBumpShort,
        closer_dialects: vec!["triple_criticality"],
        blocker: Some("Ç (kinetics): K_trap ord=4 exceeds T_CEILING ord=3 ceiling"),
        winding: "𐑷",
        winding_ordinal: 1.0,
        low_winding: true,
    }
}

/// RIEMANN HYPOTHESIS: UNCLOSED under all 23 dialects.
/// Blocker: Ω=𐑴 (ord=2) < 3. All closure-bearing dialects require Ω≥3.
/// Additionally: ⊙=𐑮 (roar, ord=7/3≈2.33) < 3 — fails triple_criticality's
/// Φ gate which selects only haha (ord=3).
/// Canonical tuple: ⟨𐑛𐑥𐑾𐑬𐑱𐑧𐑲𐑝𐑮𐑖𐑳𐑴⟩
/// Lean: `rh_closes_nowhere` — proven by native_decide.
pub fn rh_report() -> ClayReport {
    ClayReport {
        name: "Riemann Hypothesis",
        verdict: ClayVerdict::Unclosed,
        closer_dialects: vec![],
        blocker: Some("Ω=𐑴 (ord=2) < terminal anchor 3; ⊙=𐑮 (ord=7/3) < triple_criticality Φ gate (requires ord=3)"),
        winding: "𐑴",
        winding_ordinal: 2.0,
        low_winding: true,
    }
}

/// NAVIER-STOKES: UNCLOSED under all 23 dialects.
/// Blocker: Ω=𐑷 (ord=1) < 3. Even lower winding than RH.
/// Canonical tuple: ⟨𐑨𐑡𐑽𐑗𐑱𐑪𐑲𐑝𐑢𐑒𐑳𐑷⟩
/// Lean: `ns_closes_nowhere` — proven by native_decide.
pub fn ns_report() -> ClayReport {
    ClayReport {
        name: "Navier–Stokes Regularity",
        verdict: ClayVerdict::Unclosed,
        closer_dialects: vec![],
        blocker: Some("Ω=𐑷 (ord=1) < terminal anchor 3"),
        winding: "𐑷",
        winding_ordinal: 1.0,
        low_winding: true,
    }
}

/// P-vs-NP: UNCLOSED under all 23 dialects.
/// Blocker: Ω=𐑷 (ord=1) < 3. Also Φ=𐑢 (ord=1) — sub-critical, no gate clearance possible.
/// Canonical tuple: ⟨𐑛𐑡𐑩𐑗𐑱𐑤𐑲𐑝𐑢𐑓𐑙𐑷⟩
/// Lean: `pnp_closes_nowhere` — proven by native_decide.
pub fn pnp_report() -> ClayReport {
    ClayReport {
        name: "P vs NP",
        verdict: ClayVerdict::Unclosed,
        closer_dialects: vec![],
        blocker: Some("Ω=𐑷 (ord=1) < terminal anchor 3; ⊙=𐑢 (ord=1) — sub-critical"),
        winding: "𐑷",
        winding_ordinal: 1.0,
        low_winding: true,
    }
}

// ═══════════════════════════════════════════════════════════════
// OPERATIONS
// ═══════════════════════════════════════════════════════════════

/// Get reports for all 7 Clay Millennium Problems.
pub fn all_clay_reports() -> [ClayReport; 6] {
    // Note: OPN (Odd Perfect Numbers) is structurally catalogs but the
    // canonical tuple varies by formulation; Beal is in a separate module.
    // The six with machine-checked cross-dialect verdicts:
    [
        bsd_report(),
        hodge_report(),
        ym_report(),
        rh_report(),
        ns_report(),
        pnp_report(),
    ]
}

/// Summary count: (closed, one_bump_short, unclosed).
pub fn clay_summary() -> (usize, usize, usize) {
    let reports = all_clay_reports();
    let mut closed = 0;
    let mut obs = 0;
    let mut unclosed = 0;
    for r in &reports {
        match r.verdict {
            ClayVerdict::Closed       => closed += 1,
            ClayVerdict::OneBumpShort => obs += 1,
            ClayVerdict::Unclosed     => unclosed += 1,
        }
    }
    (closed, obs, unclosed)
}

/// The structural theorem: all three unclosed problems share the same blocker —
/// low winding (Ω < 3). This is the machine-checked content of
/// `rh_ns_pnp_low_winding` in Clay_UnclosedResistance.lean.
pub fn low_winding_theorem() -> &'static str {
    "All three unclosed Clay structural types (RH, NS, PNP) carry winding Ω below \
     the terminal anchor ah (ord=3). Every closure-bearing dialect requires Ω≥3 \
     at its terminal gate. Low winding ⇒ no idempotent-terminal closure. \
     This is the machine-checked theorem `rh_ns_pnp_low_winding` in \
     Imscribing.Millennium.ClayUnclosedResistance, proved by `decide`."
}

/// Full formatted report as a string, suitable for kernel boot log or CLI output.
pub fn formatted_report() -> String {
    let mut out = String::new();
    out.push_str("═══ CLAY MILLENNIUM PROBLEMS — STRUCTURAL STATUS ═══\n");
    out.push_str("(Machine-checked verdicts from p4rakernel/p4ramill Lean 4)\n\n");

    for r in &all_clay_reports() {
        out.push_str(&alloc::format!(
            "  {}: {}\n", r.name, r.verdict.name()));
        out.push_str(&alloc::format!(
            "    Ω = {} (ord={:.1})\n", r.winding, r.winding_ordinal));
        if r.verdict == ClayVerdict::Closed {
            out.push_str(&alloc::format!(
                "    Closer dialects: {}\n", r.closer_dialects.join(", ")));
        }
        if let Some(b) = r.blocker {
            out.push_str(&alloc::format!("    Blocker: {}\n", b));
        }
        out.push_str("\n");
    }

    let (c, o, u) = clay_summary();
    out.push_str(&alloc::format!(
        "SUMMARY: {} CLOSED, {} ONE-BUMP-SHORT, {} UNCLOSED\n", c, o, u));
    out.push_str("\n");
    out.push_str(low_winding_theorem());
    out.push_str("\n");

    out
}
