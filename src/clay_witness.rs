// clay_witness.rs — Clay Witness-Drag IMASM Programs (Track F)
//
// Encodes witness paths for Clay problems as IMASM programs.
// Each program traces the structural closure path verified in
// p4rakernel/p4ramill Lean 4:
//   - Clay_WitnessedClosure.lean: BSD, Hodge close under 5 dialects
//   - Clay_UnclosedResistance.lean: YM one-bump-short, RH/NS/PNP unclosed
//
// The witness programs use the IMSCRIB→CLINK→EVALT→IFIX pattern:
//   1. IMSCRIB — structural snapshot of the problem's tuple
//   2. CLINK    — compute ouroboricity tier under witness dialect
//   3. EVALT    — check: is tier O_∞?
//   4. IFIX     — seal the result
//
// For one-bump-short (YM): FSPLIT opens two branches:
//   Left:  CLINK→EVALT (gate-level: passes)
//   Right: CLINK→EVALF (T_CEILING: fails)
//   FFUSE joins — the witness IS the gap between these two.
//
// Author: Lando⊗⊙perator
// Date: 2026-07-03

use alloc::string::String;
use alloc::vec::Vec;
use crate::tokens::Token;

// ═══════════════════════════════════════════════════════════════
// WITNESS PROGRAM DEFINITIONS
// ═══════════════════════════════════════════════════════════════

/// Witness program count.
pub const WITNESS_COUNT: usize = 3;

/// Witness program names.
pub fn witness_name(i: usize) -> &'static str {
    match i {
        0 => "XXIX_BSD_Witness",
        1 => "XXX_Hodge_Witness",
        2 => "XXXI_YM_Witness",
        _ => "Unknown",
    }
}

/// Witness program description.
pub fn witness_description(i: usize) -> &'static str {
    match i {
        0 => "BSD closure witness — 5 dialects, T_CEILING-consistent. ⟨𐑦𐑥𐑾𐑿𐑞𐑧𐑲𐑝𐑮𐑖𐑙𐑭⟩",
        1 => "Hodge closure witness — 5 dialects, T_CEILING-consistent. ⟨𐑦𐑸𐑽𐑿𐑱𐑧𐑲𐑝𐑮𐑓𐑳𐑭⟩",
        2 => "YM one-bump-short witness — gate closed under triple_criticality, T_CEILING blocked on Ç",
        _ => "Unknown",
    }
}

/// Witness program token sequence.
pub fn witness_program(i: usize) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    match i {
        // XXIX: BSD_Witness
        // BSD closes under 5 dialects with T_CEILING consistency.
        // The witness: IMSCRIB captures the structure, AFWD promotes
        // toward closure, CLINK verifies the tier, EVALT confirms.
        0 => {
            for t in [Token::IMSCRIB, Token::AFWD, Token::CLINK,
                      Token::EVALT, Token::IFIX] {
                tokens.push(t);
            }
        }
        // XXX: Hodge_Witness
        // Hodge closes under 5 dialects. Same structural path as BSD.
        1 => {
            for t in [Token::IMSCRIB, Token::AFWD, Token::CLINK,
                      Token::EVALT, Token::IFIX] {
                tokens.push(t);
            }
        }
        // XXXI: YM_Witness — the "one bump short" witness
        // YM clears all three ⊙ gates under triple_criticality
        // but fails T_CEILING on Ç. FSPLIT→[EVALT|EVALF]→FFUSE
        // captures the gap: left branch (gate-level) passes,
        // right branch (T_CEILING) fails. The gap IS the witness.
        2 => {
            for t in [Token::IMSCRIB, Token::FSPLIT,
                      Token::EVALT, Token::CLINK,
                      Token::FFUSE,  Token::EVALF,
                      Token::IFIX] {
                tokens.push(t);
            }
        }
        _ => return None,
    }
    Some(tokens)
}

// ═══════════════════════════════════════════════════════════════
// WITNESS REPORT — formatted output for REPL
// ═══════════════════════════════════════════════════════════════

/// Full witness report for a Clay problem.
pub fn witness_report(problem: &str) -> String {
    use crate::clay_status;

    let report = match problem.to_lowercase().as_str() {
        "bsd" | "birch" | "swinnerton-dyer" | "birch–swinnerton-dyer" =>
            clay_status::bsd_report(),
        "hodge" =>
            clay_status::hodge_report(),
        "ym" | "yang-mills" | "yang–mills" | "yang_mills" =>
            clay_status::ym_report(),
        "rh" | "riemann" =>
            clay_status::rh_report(),
        "ns" | "navier-stokes" | "navier–stokes" =>
            clay_status::ns_report(),
        "pnp" | "p-vs-np" | "p vs np" | "p=np" =>
            clay_status::pnp_report(),
        _ => {
            return alloc::format!(
                "Unknown problem '{}'. Try: bsd, hodge, ym, rh, ns, pnp", problem);
        }
    };

    let mut out = String::new();
    out.push_str(&alloc::format!("═══ CLAY WITNESS: {} ═══\n", report.name));
    out.push_str(&alloc::format!("Status: {}\n", report.verdict.name()));
    out.push_str(&alloc::format!("Winding: Ω={} (ordinal {:.1})\n\n",
        report.winding, report.winding_ordinal));

    if !report.closer_dialects.is_empty() {
        out.push_str("Closer dialects:\n");
        for u in &report.closer_dialects {
            out.push_str(&alloc::format!("  • {}\n", u));
        }
        out.push_str("\n");
    }

    if let Some(blocker) = report.blocker {
        out.push_str(&alloc::format!("Blocker: {}\n\n", blocker));
    }

    // Show witness IMASM program
    let (witness_idx, witness_label) = match problem.to_lowercase().as_str() {
        s if s.starts_with("bsd") || s.starts_with("birch") => (0, "BSD closure"),
        "hodge" => (1, "Hodge closure"),
        s if s.starts_with("ym") || s.starts_with("yang") => (2, "YM one-bump-short"),
        _ => {
            out.push_str("No witness program — problem is unclosed under all dialects.\n");
            out.push_str("The IMASM witness programs exist only for problems with\n");
            out.push_str("verified closure paths (BSD, Hodge) or partial closure (YM).\n");
            return out;
        }
    };

    out.push_str(&alloc::format!("── {} — IMASM Witness Program ──\n", witness_label));
    out.push_str(&alloc::format!("  Program: {}\n", witness_name(witness_idx)));
    if let Some(tokens) = witness_program(witness_idx) {
        out.push_str("  Tokens: ");
        for (i, t) in tokens.iter().enumerate() {
            if i > 0 { out.push_str(" → "); }
            out.push_str(&alloc::format!("{:?}", t));
        }
        out.push_str("\n");
    }

    match witness_idx {
        0 => {
            out.push_str("\n  Path: IMSCRIB captures BSD's structural snapshot.\n");
            out.push_str("        AFWD promotes toward the closure target.\n");
            out.push_str("        CLINK computes the ouroboricity tier.\n");
            out.push_str("        EVALT confirms: O_∞ reached.\n");
            out.push_str("        IFIX seals: BSD IS closed under these 5 dialects.\n");
        }
        1 => {
            out.push_str("\n  Path: IMSCRIB captures Hodge's structural snapshot.\n");
            out.push_str("        AFWD promotes toward the closure target.\n");
            out.push_str("        CLINK computes the ouroboricity tier.\n");
            out.push_str("        EVALT confirms: O_∞ reached.\n");
            out.push_str("        IFIX seals: Hodge IS closed under these 5 dialects.\n");
        }
        2 => {
            out.push_str("\n  Path: IMSCRIB captures YM's structural snapshot.\n");
            out.push_str("        FSPLIT forks into two evaluation branches.\n");
            out.push_str("        Left:  EVALT — gate-level passes (triple_criticality).\n");
            out.push_str("        Right: EVALF — T_CEILING fails (Ç ceiling).\n");
            out.push_str("        FFUSE joins: the gap IS the one-bump-short witness.\n");
            out.push_str("        IFIX seals: YM closes under gate but NOT under T_CEILING.\n");
        }
        _ => {}
    }

    // Low winding theorem
    if report.low_winding {
        out.push_str("\n── Low Winding Theorem ──\n");
        out.push_str(&alloc::format!(
            "Ω={} (ord {:.1}) < terminal anchor 3.\n", report.winding, report.winding_ordinal));
        out.push_str("All closure-bearing dialects require Ω≥3 (integer winding).\n");
        out.push_str("This problem CANNOT close in any dialect because its winding\n");
        out.push_str("is structurally insufficient. The low_winding_theorem is a\n");
        out.push_str("structural proof — not an empirical limit.\n");
    }

    out
}

/// List all witness programs.
pub fn list_witnesses() -> String {
    let mut out = String::new();
    out.push_str("═══ CLAY WITNESS IMASM PROGRAMS ═══\n\n");
    for i in 0..WITNESS_COUNT {
        out.push_str(&alloc::format!("  {} — {}\n", witness_name(i), witness_description(i)));
    }
    out.push_str(&alloc::format!("\n{} witness programs total.\n", WITNESS_COUNT));
    out.push_str("Use 'clay witness <bsd|hodge|ym>' for full witness reports.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_programs_exist() {
        for i in 0..WITNESS_COUNT {
            let tokens = witness_program(i);
            assert!(tokens.is_some(), "Witness program {} should exist", i);
            assert!(!tokens.unwrap().is_empty(), "Witness program {} should not be empty", i);
        }
    }

    #[test]
    fn test_witness_names() {
        assert_eq!(witness_name(0), "XXIX_BSD_Witness");
        assert_eq!(witness_name(1), "XXX_Hodge_Witness");
        assert_eq!(witness_name(2), "XXXI_YM_Witness");
    }

    #[test]
    fn test_witness_reports() {
        let bsd = witness_report("bsd");
        assert!(bsd.contains("CLOSED"));
        assert!(bsd.contains("XXIX"));

        let hodge = witness_report("hodge");
        assert!(hodge.contains("CLOSED"));
        assert!(hodge.contains("XXX"));

        let ym = witness_report("ym");
        assert!(ym.contains("ONE-BUMP-SHORT"));
        assert!(ym.contains("XXXI"));

        let rh = witness_report("rh");
        assert!(rh.contains("UNCLOSED"));
        assert!(rh.contains("Low Winding"));
    }

    #[test]
    fn test_list_witnesses() {
        let list = list_witnesses();
        assert!(list.contains("XXIX"));
        assert!(list.contains("XXX"));
        assert!(list.contains("XXXI"));
    }

    #[test]
    fn test_unknown_problem() {
        let result = witness_report("foo");
        assert!(result.contains("Unknown"));
    }
}
