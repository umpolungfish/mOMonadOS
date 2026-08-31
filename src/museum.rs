// ─── museum.rs ─────────────────────────────────────────────────────────
// Preserve failed ideas (build.txt §481).
//
// "The point isn't nostalgia. Failed constructions contain NEGATIVE KNOWLEDGE
// that ordinary catalogs throw away."
//
// Every other tool in this kernel discards its failures: the audit reports a
// defect and the defect gets fixed, and then nothing remembers that the defect
// was ever possible. This is the one place that keeps them.
//
// The museum is APPEND-ONLY, and not as a convenience. An enumerated address is
// never removed; only the imscription carried at it may be revised. A failure
// that has since been repaired keeps its address and gets its status updated —
// it is never deleted, because the fact that this failure was reachable is the
// knowledge being preserved.
//
// The seeded exhibits below are real, each one found and verified in this
// kernel, with the observation that exposed it. Nothing here is illustrative.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    /// Repaired. Kept because the failure was reachable, and could be again.
    Repaired,
    /// Still open: no repair is known.
    Open,
    /// Not a defect after all — the expectation was wrong, not the system.
    Withdrawn,
}

impl Status {
    pub fn name(&self) -> &'static str {
        match self {
            Status::Repaired => "REPAIRED",
            Status::Open => "OPEN",
            Status::Withdrawn => "WITHDRAWN (the expectation was wrong)",
        }
    }
}

pub struct Exhibit {
    pub name: &'static str,
    pub conjecture: &'static str,
    pub method: &'static str,
    pub input: &'static str,
    pub expected: &'static str,
    pub observed: &'static str,
    pub failure_mode: &'static str,
    pub minimal_counterexample: &'static str,
    pub repair: &'static str,
    pub status: Status,
}

/// The permanent collection. Append only; never reorder, never remove.
pub static EXHIBITS: &[Exhibit] = &[
    Exhibit {
        name: "cl8nk_two_axis_metric",
        conjecture: "tuple_distance_cl8nk measures all twelve primitive axes",
        method: "cl8nk chain — the CLINK ladder's distances from L8",
        input: "every CLINK layer L0..L8 against the L8 reference",
        expected: "conflict counts varying with how far a layer sits from L8",
        observed: "conflicts=2 for EVERY layer, including layers differing in eight primitives",
        failure_mode: "key-space mismatch: DIST_SPECS keyed by letter (\"D\",\"T\",...), get_prim keyed by glyph. Only \"<\" and \"⊡\" existed in both, so ten axes resolved to None -> IgPrim::dead on BOTH sides and compared equal",
        minimal_counterexample: "any two tuples differing only in D: distance 0.0000, conflicts 0",
        repair: "re-key DIST_SPECS to glyphs. Ladder became coherent: L0/L1/L2 farthest at ~2.0, L7 nearest at 0.74. Before the fix L0 sat CLOSER to L8 than L4 cell did",
        status: Status::Repaired,
    },
    Exhibit {
        name: "provenance_file_granularity",
        conjecture: "a file's weakest tactic is a fair grade for what the file proves",
        method: "prov Primitives/Crystal",
        input: "Crystal.lean — crystal_total_size (by decide) beside crystal_roundtrip (omega)",
        expected: "LEAN-PROVED: the roundtrip is universally quantified over every Imscription",
        observed: "COMPUTED — one arithmetic decide dragged a proved bijection down with it",
        failure_mode: "grading per FILE when the claim is per THEOREM; the meet was too coarse",
        minimal_counterexample: "one `by decide` lemma added to any clean file demotes the whole file",
        repair: "census now attributes each theorem to the tactic closing IT and counts `clean`. Crystal.lean grades LEAN-PROVED; ArsCrossDomain stays COMPUTED (14 theorems, all native_decide, zero clean)",
        status: Status::Repaired,
    },
    Exhibit {
        name: "provenance_absent_evidence",
        conjecture: "the meet over dependencies is the honest grade for a claim",
        method: "prov bsd",
        input: "28 dependencies including an empty scaffold stub",
        expected: "HEURISTIC — 96 axioms and 171 open sorries are present",
        observed: "UNRESOLVED, glossed 'no evidence found', printed directly above those counts",
        failure_mode: "an empty file (0 thm, 0 sorry, 0 axiom) graded Unresolved and joined the meet as the weakest node. Absence of evidence was treated as weak evidence",
        minimal_counterexample: "any claim with one empty scaffold file among its dependencies",
        repair: "empty files excluded from the meet. Caught because the output contradicted itself",
        status: Status::Repaired,
    },
    Exhibit {
        name: "provenance_truncated_meet",
        conjecture: "grading the displayed dependencies grades the claim",
        method: "prov Frobenius",
        input: "dependencies sorted weakest-first, eight displayed",
        expected: "HEURISTIC — 7 axioms and 6 sorries were counted",
        observed: "UNRESOLVED with those counts printed beneath it",
        failure_mode: "counts covered every dependency but the fold covered only the eight shown; empty scaffolds sort first and filled all eight slots, so the files carrying the sorries fell outside the fold entirely",
        minimal_counterexample: "any claim with more than eight dependencies whose first eight are empty",
        repair: "root folded over every dependency; the eight-item limit is presentation only",
        status: Status::Repaired,
    },
    Exhibit {
        name: "basin_repair_enumeration",
        conjecture: "12^4 words can be enumerated under any action",
        method: "basin <seed> --action REPAIR",
        input: "the full space of length-4 words",
        expected: "an exact basin size, as ROTAT gives",
        observed: "the kernel hung; QEMU sat busy past the timeout with no output",
        failure_mode: "one REPAIR step is itself an exhaustive 12*n insertion sweep with a walk per candidate, so 20,736 seeds became millions of walks. Cost is per-ACTION, not per-length",
        minimal_counterexample: "basin with any length-4 seed under REPAIR",
        repair: "per-action enumeration limits: ROTAT to length 4, REPAIR to length 3. Past the limit it prints 'not enumerated' and gives no estimate",
        status: Status::Repaired,
    },
    Exhibit {
        name: "catalog_silent_name_drop",
        conjecture: "deduplicating the catalog merge by name loses nothing",
        method: "catalog_init merging ASK_CATALOG_SUBSET into STATIC_CATALOG",
        input: "8076 entries, 20 of whose names already existed in the static ladder",
        expected: "a clean merge",
        observed: "20 second imscriptions discarded in silence, including photon, graviton, hodge_conjecture and navier_stokes",
        failure_mode: "dedup by name alone; the dropped copy was never compared to the kept one, so nine differing TUPLES vanished unnoticed",
        minimal_counterexample: "photon: kept tier=3 domain=Physics, dropped tier=0 domain=General",
        repair: "the merge records every drop with a flag for whether it disagreed. Entropy verdict dS~0: hold the contradiction rather than discard it. The audit reports it WITHOUT calling it a defect — a second imscription at a live address is the normal case",
        status: Status::Repaired,
    },
    Exhibit {
        name: "repl_splitn_tail_truncation",
        conjecture: "a REPL command receives its arguments as separate tokens",
        method: "blackbox 1 4 9 16 25, and demonstrate mu-delta 1 2 -1",
        input: "five integers, and a three-generator braid word",
        expected: "five observations parsed; a braid word of three generators",
        observed: "the usage text, as though the command had been mistyped",
        failure_mode: "the dispatcher splits with `line.splitn(4, \' \')`, so everything past the third field arrives as ONE argument still carrying its spaces: \"9 16 25\". Parsing arguments as given dropped every term after the third SILENTLY, and the too-few-arguments branch then printed usage — the failure named the wrong cause",
        minimal_counterexample: "blackbox 1 4 9 16 25 -> parsed [1, 4], reported usage",
        repair: "both tools re-split their arguments on whitespace. Any future command taking more than three variadic arguments needs the same, or it will truncate in the same silent way",
        status: Status::Repaired,
    },
    Exhibit {
        name: "fuzzer_survivors_geometric",
        conjecture: "the count of Frobenius-stable programs is 2*3^n",
        method: "frobenius-fuzzer at each length, then blackbox on the counts",
        input: "survivor counts 2, 6, 18 at word lengths 2, 3, 4",
        expected: "54 survivors at length 5 — blackbox ranked a_n = 2*3^n above the degree-2 polynomial on complexity",
        observed: "58",
        failure_mode: "three points admitted a geometric law that the fourth refuted. The law was reported as a prediction to TEST rather than a property, and testing it is what broke it — the tool behaved correctly and the conjecture was wrong",
        minimal_counterexample: "length 5: 58 != 54",
        repair: "none needed for the tool. The sequence 2, 6, 18, 58 is still unidentified; a degree-3 polynomial fits it only by interpolation",
        status: Status::Withdrawn,
    },
    Exhibit {
        name: "blackbox_interpolation_wins",
        conjecture: "penalising complexity is enough to keep interpolation from winning",
        method: "blackbox 2 6 18 58",
        input: "four observations, no simple law among them",
        expected: "no law reported, or a law flagged as uninformative",
        observed: "polynomial of degree 3, fit 1.000, presented as best with a confident prediction of 146",
        failure_mode: "a degree n-1 polynomial fits ANY n points exactly, so its fit of 1.000 carries no information. The complexity penalty only ranks candidates against EACH OTHER — when nothing simpler exists, the vacuous law wins by default and looks like a discovery",
        minimal_counterexample: "any four integers with no simpler law: degree-3 always fits",
        repair: "any law whose complexity >= the number of observations is marked INTERPOLATION and licenses no prediction. It is kept and marked rather than hidden, because 'only interpolation fits' is itself the finding",
        status: Status::Repaired,
    },
    Exhibit {
        name: "fuzzer_length_6_by_reasoning",
        conjecture: "the fuzzer's enumeration limit can be raised by reasoning about per-candidate cost",
        method: "frobenius-fuzzer --len 6, to get a fifth term for the survivor sequence",
        input: "12^6 = 2,985,984 candidate programs",
        expected: "about 30 seconds — 12x the candidates of length 5, which took 2.5s, at a constant cost per candidate",
        observed: "abandoned after 11 minutes with no output",
        failure_mode: "the per-candidate cost is not constant: ~264x the time for 12x the candidates. Allocation churn over millions of iterations does not stay flat. The limit had been raised on the strength of an argument (one parse, one read_tangle, one braid_to_imasm, no nested sweep) instead of a measurement — the SAME shape as basin_repair_enumeration, made again after that exhibit was already in this collection",
        minimal_counterexample: "fuzz --len 6",
        repair: "MAX_LEN back to 5, the measured value, with the three timings recorded beside it. The rule earned twice over: an enumeration limit is a measurement, never an extrapolation",
        status: Status::Repaired,
    },
    Exhibit {
        name: "cl9nk_distance_5_63",
        conjecture: "d(L8, L9) = 5.63, as recorded in CL9NK_ASCENT.md",
        method: "cl9nk — the L9 tuple against the L8 reference, after the metric was repaired",
        input: "CLINK L8 and CLINK L9, differing in 8 of 12 primitives",
        expected: "5.63",
        observed: "1.7596",
        failure_mode: "the figure is not reachable by this metric at all. sqrt(sum w*d^2) with normalized d<=1 and weights summing to 9.1 has a CEILING of 3.0166; the Python navigator's variant tops out near 3.46. 5.63 exceeds both",
        minimal_counterexample: "the ceiling itself: no pair of tuples can exceed 3.0166",
        repair: "none. The document's number comes from a metric neither implementation carries, and nothing here was tuned toward it",
        status: Status::Open,
    },
    Exhibit {
        name: "catalog_tuple_degeneracy_as_defect",
        conjecture: "distinct concepts sharing one 12-primitive tuple is a catalog defect",
        method: "shadow, then a full census of collision classes",
        input: "8076 entries over 5676 distinct tuples; 842 collision classes",
        expected: "widespread carelessness from bulk generation",
        observed: "828 of 842 classes (98.3%) agree on Domain AND tier — 85 cherokee syllabary characters on one tuple, 44 adaptogenic herbs on another",
        failure_mode: "the expectation was wrong. The tuple names a KIND, not an individual; d=0 is an equivalence class, not a collision",
        minimal_counterexample: "cherokee_a, cherokee_e, cherokee_i — the same kind of object, correctly identified as such",
        repair: "not a repair: shadow reports d=0 as an equivalence class with a coherence check. Only the 14 classes disagreeing on Domain or tier are flagged, and even those are held rather than corrected",
        status: Status::Withdrawn,
    },
];

/// Runtime additions. Also append-only.
static mut ADDED: Vec<(String, String)> = Vec::new();

fn contains_ci(hay: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let h: Vec<char> = hay.chars().flat_map(|c| c.to_lowercase()).collect();
    let n: Vec<char> = needle.chars().flat_map(|c| c.to_lowercase()).collect();
    if n.len() > h.len() {
        return false;
    }
    (0..=(h.len() - n.len())).any(|i| h[i..i + n.len()] == n[..])
}

/// Every field is searched: a failure is found by its symptom as often as by
/// its name, and the symptom is what a caller remembers.
pub fn search(term: &str) -> Vec<&'static Exhibit> {
    EXHIBITS
        .iter()
        .filter(|e| {
            contains_ci(e.name, term)
                || contains_ci(e.conjecture, term)
                || contains_ci(e.method, term)
                || contains_ci(e.observed, term)
                || contains_ci(e.failure_mode, term)
                || contains_ci(e.minimal_counterexample, term)
                || contains_ci(e.repair, term)
        })
        .collect()
}

pub fn format_exhibit(e: &Exhibit) -> String {
    let mut s = String::new();
    s.push_str(&format!("EXHIBIT  {}\n", e.name));
    for _ in 0..(9 + e.name.len()) {
        s.push('=');
    }
    s.push_str("\n\n");
    s.push_str(&format!("  CONJECTURE     {}\n", e.conjecture));
    s.push_str(&format!("  METHOD         {}\n", e.method));
    s.push_str(&format!("  INPUT          {}\n", e.input));
    s.push_str(&format!("  EXPECTED       {}\n", e.expected));
    s.push_str(&format!("  OBSERVED       {}\n", e.observed));
    s.push_str(&format!("  FAILURE MODE   {}\n", e.failure_mode));
    s.push_str(&format!("  MINIMAL C-EX   {}\n", e.minimal_counterexample));
    s.push_str(&format!("  REPAIR         {}\n", e.repair));
    s.push_str(&format!("  STATUS         {}\n", e.status.name()));
    s
}

pub fn museum_main(args: &[&str]) -> String {
    let sub = args.first().copied().unwrap_or("list");

    match sub {
        "list" | "" => {
            let mut s = String::from("MUSEUM — the permanent collection\n");
            s.push_str("=================================\n\n");
            s.push_str("Failed constructions carry negative knowledge that catalogs discard.\n");
            s.push_str("Append-only: a repaired failure keeps its exhibit, because the fact\n");
            s.push_str("that it was reachable is the knowledge being kept.\n\n");
            let (mut rep, mut open, mut wd) = (0, 0, 0);
            for e in EXHIBITS {
                match e.status {
                    Status::Repaired => rep += 1,
                    Status::Open => open += 1,
                    Status::Withdrawn => wd += 1,
                }
                s.push_str(&format!("  {:<34} {}\n", e.name, e.status.name()));
            }
            s.push_str(&format!(
                "\n{} exhibits: {} repaired, {} open, {} withdrawn.\n",
                EXHIBITS.len(),
                rep,
                open,
                wd
            ));
            s.push_str("\nmuseum <name>            the full record\n");
            s.push_str("museum search <term>     every field is searched\n");
            s.push_str("museum open              only what is still unresolved\n");
            s
        }
        "open" => {
            let mut s = String::from("OPEN FAILURES — no repair is known\n==================================\n\n");
            let open: Vec<&Exhibit> = EXHIBITS.iter().filter(|e| e.status == Status::Open).collect();
            if open.is_empty() {
                s.push_str("None. Every exhibit has a repair or was withdrawn.\n");
            } else {
                for e in open {
                    s.push_str(&format_exhibit(e));
                    s.push('\n');
                }
            }
            s
        }
        "search" => {
            let term = args.get(1).copied().unwrap_or("");
            let hits = search(term);
            if hits.is_empty() {
                return format!("Nothing in the collection mentions '{}'.\n", term);
            }
            let mut s = format!("{} exhibit(s) mention '{}':\n\n", hits.len(), term);
            for e in hits {
                s.push_str(&format_exhibit(e));
                s.push('\n');
            }
            s
        }
        name => match EXHIBITS.iter().find(|e| e.name == name) {
            Some(e) => format_exhibit(e),
            None => {
                let hits = search(name);
                if hits.is_empty() {
                    format!("No exhibit named '{}'. Try `museum` for the collection.\n", name)
                } else {
                    let mut s = format!("No exhibit named '{}', but {} mention it:\n\n", name, hits.len());
                    for e in hits {
                        s.push_str(&format_exhibit(e));
                        s.push('\n');
                    }
                    s
                }
            }
        },
    }
}
