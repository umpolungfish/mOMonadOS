// combo.rs — cycle a word, then run the full instrument suite on every
// distinct rotation.
//
// `cycle` alone answers one question: where does each cut land. It does not
// say whether any of those rotations is itself exposed, repairable, or
// already holding. This runs weight, banked, insert, and repair on each one,
// so the whole orbit gets the same standing audit a single word already gets
// from `imasm write`/`imasm derive` -- just walked across every rotation
// instead of the one the caller happened to write down.

#![allow(dead_code)]
extern crate alloc;

use crate::sprintln;
use alloc::string::String;
use alloc::vec::Vec;
use imasm_core::lattice_flow::normalize;

/// The rotations of a normalized word, as plain strings, one per cut.
/// Deduplicated: a word with rotational symmetry (period dividing its
/// length) repeats some cuts verbatim, and there is nothing new to learn
/// running the same string through the suite twice.
fn distinct_rotations(word: &str) -> Vec<(usize, String)> {
    let norm = normalize(word);
    let chars: Vec<char> = norm.chars().collect();
    let n = chars.len();
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for k in 0..n {
        let rot: String = chars[k..].iter().chain(chars[..k].iter()).collect();
        if seen.iter().any(|w| w == &rot) { continue; }
        seen.push(rot.clone());
        out.push((k, rot));
    }
    out
}

/// `brief`: repair prints only its cheapest candidate (`repair_best_report`)
/// instead of every candidate grouped by crystal address. Same search either
/// way; only what gets printed changes.
pub fn combo_main(args: &[&str], brief: bool) -> String {
    let mut out = String::new();
    if args.is_empty() {
        out.push_str("combo <word> [brief]   cycle the word, then run weight | banked |\n");
        out.push_str("                       insert | repair on every distinct rotation\n");
        out.push_str("                       it produces. brief: repair shows only its\n");
        out.push_str("                       cheapest candidate, not every one found.\n");
        return out;
    }
    let word = args[0];

    out.push_str("── cycle ─────────────────────────────────────────────────────\n");
    out.push_str(&imasm_core::lattice_flow::cycle_report(word));

    let rotations = distinct_rotations(word);
    out.push('\n');
    out.push_str(&alloc::format!(
        "auditing {} distinct rotation(s){}\n", rotations.len(),
        if brief { " (brief: cheapest repair only)" } else { "" }
    ));

    for (k, rot) in rotations.iter() {
        out.push('\n');
        out.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        out.push_str(&alloc::format!("cut k={:<3} {}\n", k, rot));
        out.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        out.push_str("┌─ weight ────────────────────────────────────────────────────\n");
        for line in imasm_core::lattice_flow::weight_report(rot).lines() {
            out.push_str("│ "); out.push_str(line); out.push('\n');
        }

        out.push_str("├─ banked ────────────────────────────────────────────────────\n");
        for line in imasm_core::lattice_flow::banked_report(rot).lines() {
            out.push_str("│ "); out.push_str(line); out.push('\n');
        }

        out.push_str("├─ insert ────────────────────────────────────────────────────\n");
        for line in imasm_core::lattice_flow::insert_report(rot).lines() {
            out.push_str("│ "); out.push_str(line); out.push('\n');
        }

        out.push_str("└─ repair ────────────────────────────────────────────────────\n");
        let rep = if brief {
            crate::repair::repair_best_report(rot)
        } else {
            crate::repair::repair_main(&[rot.as_str(), "program"])
        };
        for line in rep.lines() {
            out.push_str("  "); out.push_str(line); out.push('\n');
        }
    }

    out
}

pub fn repl_combo(args: &[&str]) {
    let brief = args.last().copied() == Some("brief");
    let word_args: &[&str] = if brief && !args.is_empty() { &args[..args.len()-1] } else { args };
    sprintln!("{}", combo_main(word_args, brief));
}
