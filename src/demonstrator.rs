// ─── demonstrator.rs ───────────────────────────────────────────────────
// Turn a claim into an executable experiment (build.txt §510).
//
// For any claim the kernel can actually test, print the ladder — INPUT,
// OPERATION, INTERMEDIATE, OPERATION, OUTPUT, CHECK — with every value
// COMPUTED at the moment of printing. Nothing here is a stored transcript: if
// the kernel's behaviour changes, the demonstration changes with it, which is
// the only way a demonstration is worth anything.
//
// A demonstration that fails prints FAIL and says what differed. It is not an
// advertisement for the claim; it is a run of it.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::counterfactual::{apply, read, Perturbation};
use crate::lattice_flow::banked_walk;

pub struct Demo {
    pub claim: &'static str,
    pub lines: Vec<(String, String)>,
    pub check: String,
    pub passed: bool,
}

/// μ∘δ = id on braids: compile a braid word to IMASM (δ), read the tangle back
/// (μ), and compare the generators. This is the kernel's own closure claim.
pub fn demo_mu_delta(gens: &[i32]) -> Demo {
    let mut lines = Vec::new();
    lines.push(("INPUT".to_string(), format!("braid word {:?}", gens)));

    let prog = crate::braid_protocol::braid_to_imasm(gens, 1, false);
    let rendered: Vec<&'static str> = prog
        .iter()
        .map(|t| crate::braid_protocol::token_name(t))
        .collect();
    lines.push(("OPERATION".to_string(), "δ — braid_to_imasm".to_string()));
    lines.push(("INTERMEDIATE".to_string(), rendered.join(" ")));

    lines.push(("OPERATION".to_string(), "μ — read_tangle".to_string()));
    let (out, passed, note) = match crate::braid_protocol::read_tangle(&prog, gens.len() + 2, 1) {
        Ok(r) => {
            let same = r.generators == gens;
            (
                format!(
                    "{:?}   writhe {}, {} crossings",
                    r.generators, r.writhe, r.crossings
                ),
                same,
                if same {
                    "output == input — μ∘δ = id on this word".to_string()
                } else {
                    format!("output != input: {:?} vs {:?}", r.generators, gens)
                },
            )
        }
        Err(e) => (format!("μ refused: {}", e), false, format!("μ refused: {}", e)),
    };
    lines.push(("OUTPUT".to_string(), out));

    Demo {
        claim: "μ∘δ = id (braid → IMASM → braid)",
        lines,
        check: note,
        passed,
    }
}

/// ROTAT invariance: a word is a ring, so the verdict and topology hold across
/// the orbit while the final register need not.
pub fn demo_rotat(word: &str) -> Demo {
    let mut lines = Vec::new();
    let before = read(word);
    let rotated = apply(word, Perturbation::Rotate(1));
    let after = read(&rotated);

    match (before, after) {
        (Some(b), Some(a)) => {
            lines.push(("INPUT".to_string(), format!("{}   verdict {}, register {}", b.word, b.verdict, b.register)));
            lines.push(("OPERATION".to_string(), "ROTAT — cyclic shift by 1".to_string()));
            lines.push(("OUTPUT".to_string(), format!("{}   verdict {}, register {}", a.word, a.verdict, a.register)));

            let verdict_held = a.verdict == b.verdict;
            let register_held = a.register == b.register;
            let check = if verdict_held {
                format!(
                    "verdict {} across the shift{}",
                    if verdict_held { "HELD" } else { "MOVED" },
                    if register_held {
                        "; the register held too on this word"
                    } else {
                        "; the register MOVED, which is the phase dependence"
                    }
                )
            } else {
                format!("verdict MOVED {} -> {} — the claim FAILS here", b.verdict, a.verdict)
            };
            Demo { claim: "ROTAT preserves the verdict", lines, check, passed: verdict_held }
        }
        _ => Demo {
            claim: "ROTAT preserves the verdict",
            lines: alloc::vec![("INPUT".to_string(), format!("{} — unreadable", word))],
            check: "the word parses to no tokens".to_string(),
            passed: false,
        },
    }
}

/// Banking: weight held in a frame survives a clear; weight left open does not.
pub fn demo_banking(word: &str) -> Demo {
    let mut lines = Vec::new();
    lines.push(("INPUT".to_string(), word.to_string()));
    lines.push(("OPERATION".to_string(), "banked_walk — deposit, fork, clear".to_string()));

    match banked_walk(word) {
        Some(b) => {
            lines.push((
                "OUTPUT".to_string(),
                format!(
                    "{} deposits, {} live clears, {} exposed, {} inert",
                    b.deposits,
                    b.live_clears,
                    b.exposed.len(),
                    b.inert
                ),
            ));
            let check = if b.holds() {
                "HOLDS — every clear that fired had weight banked behind it".to_string()
            } else if b.vacuous() {
                "VACUOUS — no clear ever fired, so nothing was ever at risk".to_string()
            } else {
                let (step, glyph, lost) = b.exposed[0];
                format!(
                    "EXPOSED — {} at step {} cleared {} unit(s) with nothing banked",
                    glyph, step, lost
                )
            };
            // Vacuity is not success: the claim is about weight surviving, and a
            // word that risked nothing demonstrated nothing.
            let passed = b.holds();
            Demo { claim: "banked weight survives a clear", lines, check, passed }
        }
        None => Demo {
            claim: "banked weight survives a clear",
            lines,
            check: "the word parses to no tokens".to_string(),
            passed: false,
        },
    }
}

/// The tuple round trip: tuple → word → tuple. The kernel already knows this
/// map is many-to-one, so the demonstration reports WHICH fixed point held.
pub fn demo_roundtrip(tuple_glyphs: &str) -> Demo {
    use crate::imas_ig::{IgPrim, IgTuple};
    let mut lines = Vec::new();

    let t = match IgTuple::from_glyphs(tuple_glyphs) {
        Ok(t) => t,
        Err((i, m)) => {
            return Demo {
                claim: "tuple → word → tuple",
                lines: alloc::vec![("INPUT".to_string(), format!("bad tuple at {}: {}", i, m))],
                check: "not a tuple".to_string(),
                passed: false,
            }
        }
    };
    lines.push(("INPUT".to_string(), format!("{}", t.display())));

    let prog = crate::sequence::build_via_substrate(&t, 12, t.t == IgPrim::are, 3);
    let word = crate::belnap_ring_shor::glyphs_from_program(&prog);
    lines.push(("OPERATION".to_string(), "build_via_substrate — the tuple writes a word".to_string()));
    lines.push(("INTERMEDIATE".to_string(), word.clone()));

    lines.push(("OPERATION".to_string(), "self_imscribe — the word reads back a tuple".to_string()));
    let back = IgTuple::from_snapshot(&crate::kernel::self_imscribe(&prog));
    lines.push(("OUTPUT".to_string(), format!("{}", back.display())));

    let prog2 = crate::sequence::build_via_substrate(&back, 12, back.t == IgPrim::are, 3);
    let word2 = crate::belnap_ring_shor::glyphs_from_program(&prog2);

    let (check, passed) = if back == t {
        ("the TUPLE returns itself — μ∘δ = id on the tuple".to_string(), true)
    } else if word2 == word {
        (
            format!(
                "the WORD is fixed, the tuple is not: it writes {} again.\n\
                 \x20         The map is many-to-one — what the imscription DOES is\n\
                 \x20         recoverable, what it CLAIMS is not.",
                word2
            ),
            true,
        )
    } else {
        (
            format!("OPEN — neither returns; the second word is {}", word2),
            false,
        )
    };

    Demo { claim: "tuple → word → tuple", lines, check, passed }
}

pub fn format_demo(d: &Demo) -> String {
    let mut out = String::new();
    out.push_str("DEMONSTRATION\n=============\n\n");
    out.push_str(&format!("claim:  {}\n\n", d.claim));
    for (label, value) in &d.lines {
        out.push_str(&format!("  {:<13} {}\n", label, value));
    }
    out.push_str(&format!(
        "\n  {:<13} {}  {}\n",
        "CHECK",
        if d.passed { "✓" } else { "✗" },
        d.check
    ));
    out.push_str("\nEvery value above was computed by this run, not stored.\n");
    out
}

pub fn demonstrate_main(args: &[&str]) -> String {
    if args.is_empty() {
        return "demonstrate <claim> [argument]\n\
                \n\
                Run a claim as an experiment and print the ladder: INPUT,\n\
                OPERATION, INTERMEDIATE, OUTPUT, CHECK. A failing claim prints\n\
                FAIL and says what differed.\n\
                \n\
                claims:\n\
                \x20 mu-delta [gens...]   braid → IMASM → braid, μ∘δ = id\n\
                \x20 rotat <word>         ROTAT preserves the verdict\n\
                \x20 banking <word>       banked weight survives a clear\n\
                \x20 roundtrip <tuple>    tuple → word → tuple\n\
                \n\
                Try:  demonstrate mu-delta 1 2 -1\n"
            .to_string();
    }

    match args[0] {
        "mu-delta" | "frobenius" | "mudelta" => {
            // Same splitn(4) tail as blackbox: "mu-delta 1 2 -1" delivers "2 -1"
            // as one argument, which silently truncated the braid word.
            let gens: Vec<i32> = args[1..]
                .iter()
                .flat_map(|s| s.split_whitespace())
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            let gens = if gens.is_empty() { alloc::vec![1, 2, -1] } else { gens };
            format_demo(&demo_mu_delta(&gens))
        }
        "rotat" => {
            let w = args.get(1).copied().unwrap_or("⊢∈⊤∋<⊣");
            format_demo(&demo_rotat(w))
        }
        "banking" => {
            let w = args.get(1).copied().unwrap_or("⊢∈⊤∋<⊣");
            format_demo(&demo_banking(w))
        }
        "roundtrip" => {
            let joined: String = args[1..].join("");
            format_demo(&demo_roundtrip(&joined))
        }
        other => format!("no demonstration named '{}'. Try: mu-delta, rotat, banking, roundtrip\n", other),
    }
}
