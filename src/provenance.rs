// ─── provenance.rs ─────────────────────────────────────────────────────
// Epistemic type checking (build.txt §231).
//
// Every result carries a provenance, and the provenances form a lattice. The
// rule that makes this useful rather than decorative:
//
//     a claim is only as proved as its weakest dependency
//
// so the root's provenance is the MEET over its children, never the best thing
// found among them. That is the whole point of an epistemic type checker — it
// must not let a machine-checked sibling launder an axiom-dependent one.
//
// The evidence is the same evidence the rest of the kernel uses:
//   lean_census   theorems, sorries, axioms, decide/native_decide per file
//   clay_status   graded Clay verdicts and named blockers (via witness)
//   clay_witness  executable witness programs (via witness)
//
// Nothing is inferred about a file the census did not see. A claim with no
// evidence is UNRESOLVED, which is an absence and not a denial.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::lean_census::{CENSUS_DATE, LEAN_CENSUS};
use crate::witness::{witness, Standing};

/// The provenance lattice, ordered worst to best. `meet` takes the minimum,
/// so a single axiom-dependent dependency pulls the whole claim down to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Provenance {
    /// Nothing in the kernel bears on it.
    Unresolved = 0,
    /// Stated as an open target.
    Conjectural = 1,
    /// Argued but with obligations still open (`sorry` present).
    Heuristic = 2,
    /// Granted to itself as an axiom.
    AxiomDependent = 3,
    /// Closed by finite evaluation (`decide` / `native_decide`).
    Computed = 4,
    /// Follows from machine-checked results without new assumptions.
    Derived = 5,
    /// Machine-checked, no sorry, no axiom.
    LeanProved = 6,
}

impl Provenance {
    pub fn name(&self) -> &'static str {
        match self {
            Provenance::Unresolved => "UNRESOLVED",
            Provenance::Conjectural => "CONJECTURAL",
            Provenance::Heuristic => "HEURISTIC",
            Provenance::AxiomDependent => "AXIOM-DEPENDENT",
            Provenance::Computed => "COMPUTED",
            Provenance::Derived => "DERIVED",
            Provenance::LeanProved => "LEAN-PROVED",
        }
    }

    /// Why a result carries this provenance, in one line.
    pub fn gloss(&self) -> &'static str {
        match self {
            Provenance::Unresolved => "no evidence found — an absence, not a denial",
            Provenance::Conjectural => "stated as a target, not established",
            Provenance::Heuristic => "obligations remain open (sorry present)",
            Provenance::AxiomDependent => "rests on assumptions the corpus grants itself",
            Provenance::Computed => "closed by finite evaluation; holds for its decidable instance",
            Provenance::Derived => "follows from machine-checked results",
            Provenance::LeanProved => "machine-checked: no sorry, no axiom",
        }
    }

    /// The lattice meet: the weaker of two provenances.
    pub fn meet(self, other: Provenance) -> Provenance {
        if self <= other {
            self
        } else {
            other
        }
    }
}

/// One node of the dependency DAG.
pub struct Node {
    pub label: String,
    pub prov: Provenance,
    pub detail: String,
}

pub struct Dag {
    pub claim: String,
    pub root: Provenance,
    pub children: Vec<Node>,
    /// Total axioms and sorries across the dependencies.
    pub axioms: u32,
    pub sorries: u32,
    pub truncated: usize,
}

/// Grade a single Lean file from its census counts.
///
/// `clean` is the count of theorems closed WITHOUT sorry, decide or
/// native_decide — attributed per theorem, not per file. That distinction is
/// the whole point: Crystal.lean holds `crystal_total_size` (an arithmetic
/// identity, legitimately `by decide`) beside `crystal_roundtrip` (universally
/// quantified over every Imscription, closed by omega). Grading the file by its
/// weakest tactic called the bijection COMPUTED, which understates a proof.
fn grade_file(theorems: u16, sorries: u16, axioms: u16, decide: u16, native: u16, clean: u16) -> Provenance {
    if theorems == 0 && sorries == 0 && axioms == 0 {
        return Provenance::Unresolved;
    }
    // An open obligation still outranks everything: a file with a sorry has
    // something unproved in it whatever else it also proves.
    if sorries > 0 {
        return Provenance::Heuristic;
    }
    if axioms > 0 {
        return Provenance::AxiomDependent;
    }
    // A theorem closed without any finite check is machine-checked, and it
    // stays machine-checked whatever OTHER theorems in the file used decide.
    if clean > 0 {
        return Provenance::LeanProved;
    }
    if decide > 0 || native > 0 {
        return Provenance::Computed;
    }
    Provenance::Conjectural
}

pub fn provenance_of(claim: &str) -> Dag {
    let needle = claim.to_lowercase();
    let mut children: Vec<Node> = Vec::new();
    let mut axioms = 0u32;
    let mut sorries = 0u32;

    // The Clay/witness leg, when the claim names one.
    let w = witness(claim);
    match w.standing {
        Standing::Witnessed => children.push(Node {
            label: format!("witness: {}", w.program_name.unwrap_or("executable object")),
            prov: Provenance::Derived,
            detail: w.because.clone(),
        }),
        Standing::OneBumpShort | Standing::Blocked => children.push(Node {
            label: "clay verdict".to_string(),
            prov: Provenance::Conjectural,
            detail: w.because.clone(),
        }),
        Standing::Unresolved => {}
    }

    // The Lean leg: every census file whose path mentions the claim.
    let mut hits: Vec<&'static crate::lean_census::LeanFile> = LEAN_CENSUS
        .iter()
        .filter(|f| !needle.is_empty() && f.path.to_lowercase().contains(&needle))
        .collect();
    // Weakest first: the meet is decided by these, so show them in the order
    // that explains the verdict.
    hits.sort_by_key(|f| grade_file(f.theorems, f.sorries, f.axioms, f.decide, f.native_decide, f.clean));

    let total_hits = hits.len();
    for f in hits.iter().take(8) {
        axioms += f.axioms as u32;
        sorries += f.sorries as u32;
        children.push(Node {
            label: f.path.to_string(),
            prov: grade_file(f.theorems, f.sorries, f.axioms, f.decide, f.native_decide, f.clean),
            detail: format!(
                "{} thm ({} clean), {} sorry, {} axiom, {} decide, {} native_decide",
                f.theorems, f.clean, f.sorries, f.axioms, f.decide, f.native_decide
            ),
        });
    }
    // Counts must cover every dependency, not just the ones displayed.
    for f in hits.iter().skip(8) {
        axioms += f.axioms as u32;
        sorries += f.sorries as u32;
    }

    // The meet must ALSO cover every dependency. Displaying only eight is a
    // presentation limit; grading only eight would be a lie, and a subtle one:
    // hits are sorted weakest-first, so empty scaffolds filled all eight slots
    // and the files actually carrying the sorries fell outside the fold.
    let lean_meet = hits
        .iter()
        .map(|f| grade_file(f.theorems, f.sorries, f.axioms, f.decide, f.native_decide, f.clean))
        .filter(|p| *p != Provenance::Unresolved)
        .fold(None::<Provenance>, |acc, p| {
            Some(match acc {
                Some(a) => a.meet(p),
                None => p,
            })
        });

    // A file carrying no theorem, no sorry and no axiom is an EMPTY file: it
    // bears no evidence either way, so it must not join the meet. Letting it in
    // made a claim with 96 axioms and 171 open obligations report UNRESOLVED —
    // "no evidence found" — because one scaffold stub was the weakest node.
    // Absence of evidence is not weak evidence. If every node is empty the root
    // is Unresolved anyway, by the unwrap_or below.
    let root = children
        .iter()
        .map(|c| c.prov)
        .filter(|p| *p != Provenance::Unresolved)
        .chain(lean_meet.into_iter())
        .fold(None::<Provenance>, |acc, p| {
            Some(match acc {
                Some(a) => a.meet(p),
                None => p,
            })
        })
        .unwrap_or(Provenance::Unresolved);

    Dag {
        claim: claim.to_string(),
        root,
        children,
        axioms,
        sorries,
        truncated: total_hits.saturating_sub(8),
    }
}

pub fn format_dag(d: &Dag) -> String {
    let mut out = String::new();
    out.push_str("PROVENANCE\n==========\n\n");
    out.push_str(&format!("{}\n\n", d.claim));
    out.push_str(&format!("{}\n", d.root.name()));

    if d.children.is_empty() {
        out.push_str(" (no dependencies found)\n\n");
    } else {
        for (i, c) in d.children.iter().enumerate() {
            let last = i + 1 == d.children.len();
            out.push_str(&format!(
                " {}─ [{:<15}] {}\n",
                if last { "└" } else { "├" },
                c.prov.name(),
                c.label
            ));
            out.push_str(&format!(
                " {}     {}\n",
                if last { " " } else { "│" },
                c.detail
            ));
        }
        if d.truncated > 0 {
            out.push_str(&format!(
                " … {} further dependencies counted but not listed\n",
                d.truncated
            ));
        }
        out.push('\n');
    }

    out.push_str(&format!(
        "assumptions:  {}\n",
        if d.axioms == 0 {
            "none".to_string()
        } else {
            format!("{} axiom declarations across the dependencies", d.axioms)
        }
    ));
    out.push_str(&format!(
        "obligations:  {}\n",
        if d.sorries == 0 {
            "none open".to_string()
        } else {
            format!("{} sorry still open", d.sorries)
        }
    ));
    out.push_str(&format!("status:       {}\n", d.root.gloss()));

    out.push_str(&format!(
        "\nThe root is the MEET over its dependencies, not the best among them:\n\
         a claim is only as proved as its weakest link. Census {}.\n",
        CENSUS_DATE
    ));
    out
}

/// Alias for `provenance_of`. Kept because callers reach for this name; it
/// forwards rather than reimplementing, so there is one grading path.
pub fn provenance_check(claim: &str) -> Dag {
    provenance_of(claim)
}

pub fn provenance_main(args: &[&str]) -> String {
    if args.is_empty() {
        let mut s = String::from(
            "provenance <claim>   (alias: prov)\n\
             \n\
             The dependency DAG for a claim, graded on the provenance lattice.\n\
             The root is the MEET over its dependencies — a machine-checked\n\
             sibling never launders an axiom-dependent one.\n\
             \n\
             the lattice, weakest first:\n",
        );
        for p in [
            Provenance::Unresolved,
            Provenance::Conjectural,
            Provenance::Heuristic,
            Provenance::AxiomDependent,
            Provenance::Computed,
            Provenance::Derived,
            Provenance::LeanProved,
        ] {
            s.push_str(&format!("    {:<16} {}\n", p.name(), p.gloss()));
        }
        s.push_str("\nTry:  prov bsd | prov RH | prov SIC | prov Frobenius\n");
        return s;
    }
    format_dag(&provenance_of(args[0]))
}
