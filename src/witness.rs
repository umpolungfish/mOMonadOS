// ─── witness.rs ────────────────────────────────────────────────────────
// The shared primitive (build.txt §598):
//
//   "Don't merely tell me the answer. Give me the smallest executable object
//    that witnesses why you think the answer is true, false, invariant,
//    impossible, or unresolved."
//
// This is a router, not a fourth witness. The kernel already carries the
// witnesses: clay_witness holds executable IMASM programs for the closed Clay
// problems, clay_status holds the graded verdicts, and lean_census holds what
// the Lean corpus actually discharged. `witness` finds the smallest object
// among them that stands behind a claim, and says plainly when none does.
//
// The verdict is graded, never a bool: a claim with no witness is UNRESOLVED
// (nothing was found), which is not the same as IMPOSSIBLE (a blocker is
// named and proved).
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::clay_status::{self, ClayReport, ClayVerdict};
use crate::clay_witness;
use crate::lean_census::LEAN_CENSUS;

/// What kind of standing the witness gives the claim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Standing {
    /// An executable object exists and closes the claim.
    Witnessed,
    /// Closure reached the gate but a named ceiling blocks it.
    OneBumpShort,
    /// A blocker is named and proved: the claim cannot close as stated.
    Blocked,
    /// Nothing in the kernel stands behind it. Not a denial — an absence.
    Unresolved,
}

impl Standing {
    pub fn name(&self) -> &'static str {
        match self {
            Standing::Witnessed => "WITNESSED",
            Standing::OneBumpShort => "ONE-BUMP-SHORT",
            Standing::Blocked => "BLOCKED (blocker named and proved)",
            Standing::Unresolved => "UNRESOLVED (no witness found; not a denial)",
        }
    }
}

pub struct Witness {
    pub claim: String,
    pub standing: Standing,
    /// The smallest executable object, if one exists.
    pub program: Option<String>,
    pub program_name: Option<&'static str>,
    /// Why the standing is what it is.
    pub because: String,
    /// Lean files bearing on the claim: (path, theorems, sorries, axioms).
    pub lean_support: Vec<(&'static str, u16, u16, u16)>,
}

fn clay_lookup(claim: &str) -> Option<(ClayReport, usize)> {
    // Returns the report and the clay_witness program index, when one exists.
    let c = claim.to_lowercase();
    let c = c.trim();
    match c {
        "bsd" | "birch" | "swinnerton-dyer" => Some((clay_status::bsd_report(), 0)),
        "hodge" => Some((clay_status::hodge_report(), 1)),
        "ym" | "yang-mills" | "yang_mills" => Some((clay_status::ym_report(), 2)),
        // These three have verdicts but no executable witness program: usize::MAX
        // marks "graded verdict, no program", which is the honest state.
        "rh" | "riemann" => Some((clay_status::rh_report(), usize::MAX)),
        "ns" | "navier-stokes" => Some((clay_status::ns_report(), usize::MAX)),
        "pnp" | "p-vs-np" | "p=np" => Some((clay_status::pnp_report(), usize::MAX)),
        _ => None,
    }
}

/// Lean files whose path mentions the claim, worst-supported first.
fn lean_support(claim: &str) -> Vec<(&'static str, u16, u16, u16)> {
    let needle = claim.to_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<(&'static str, u16, u16, u16)> = LEAN_CENSUS
        .iter()
        .filter(|f| f.path.to_lowercase().contains(&needle))
        .map(|f| (f.path, f.theorems, f.sorries, f.axioms))
        .collect();
    // Most theorems first, then fewest sorries: the strongest support on top.
    hits.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
    hits.truncate(5);
    hits
}

pub fn witness(claim: &str) -> Witness {
    let support = lean_support(claim);

    if let Some((report, prog_idx)) = clay_lookup(claim) {
        let (standing, because) = match report.verdict {
            ClayVerdict::Closed => (
                Standing::Witnessed,
                format!(
                    "closes under {} dialect(s), T_CEILING-consistent",
                    report.closer_dialects.len()
                ),
            ),
            ClayVerdict::OneBumpShort => (
                Standing::OneBumpShort,
                report
                    .blocker
                    .unwrap_or("gate closed, T_CEILING blocked")
                    .to_string(),
            ),
            ClayVerdict::Unclosed => (
                Standing::Blocked,
                report
                    .blocker
                    .unwrap_or("resists closure under all known dialects")
                    .to_string(),
            ),
        };

        let (program, program_name) = if prog_idx == usize::MAX {
            (None, None)
        } else {
            let toks = clay_witness::witness_program(prog_idx);
            (
                toks.map(|t| format!("{} tokens", t.len())),
                Some(clay_witness::witness_name(prog_idx)),
            )
        };

        return Witness {
            claim: report.name.to_string(),
            standing,
            program,
            program_name,
            because,
            lean_support: support,
        };
    }

    // No Clay verdict. The census is then the only ground, and it can only
    // ever report support, never closure.
    let because = if support.is_empty() {
        "no Clay verdict and no Lean file mentions it".to_string()
    } else {
        let sorries: u32 = support.iter().map(|s| s.2 as u32).sum();
        format!(
            "{} Lean file(s) bear on it, carrying {} sorry — support, not closure",
            support.len(),
            sorries
        )
    };

    Witness {
        claim: claim.to_string(),
        standing: Standing::Unresolved,
        program: None,
        program_name: None,
        because,
        lean_support: support,
    }
}

pub fn format_witness(claim: &str) -> String {
    let w = witness(claim);
    let mut out = String::new();

    out.push_str("WITNESS\n");
    out.push_str("=======\n\n");
    out.push_str(&format!("claim:      {}\n", w.claim));
    out.push_str(&format!("standing:   {}\n", w.standing.name()));
    out.push_str(&format!("because:    {}\n\n", w.because));

    match (&w.program_name, &w.program) {
        (Some(name), Some(size)) => {
            out.push_str("smallest executable object:\n");
            out.push_str(&format!("    {} ({})\n", name, size));
            out.push_str("    run it:  clay witness <problem>\n\n");
        }
        _ => {
            out.push_str("smallest executable object:\n    none in the kernel\n\n");
        }
    }

    if w.lean_support.is_empty() {
        out.push_str("Lean support:\n    none\n");
    } else {
        out.push_str("Lean support (strongest first):\n");
        for (path, t, s, a) in &w.lean_support {
            out.push_str(&format!(
                "    {:<52} {} thm, {} sorry, {} axiom\n",
                path, t, s, a
            ));
        }
    }

    out.push_str("\nA witness is an object, not a proof of the claim it stands under.\n");
    out
}

pub fn witness_main(args: &[&str]) -> String {
    if args.is_empty() {
        return format!(
            "witness <claim> — the smallest executable object standing behind a claim\n\
             \n\
             Routes to the kernel's existing witnesses rather than inventing one:\n\
               clay_witness   executable IMASM witness programs\n\
               clay_status    graded Clay verdicts and named blockers\n\
               lean_census    what the Lean corpus actually discharged\n\
             \n\
             Try:  witness bsd | witness hodge | witness ym\n\
                   witness rh  | witness ns    | witness pnp\n\
                   witness <any substring matching a Lean path>\n"
        );
    }
    format_witness(args[0])
}
