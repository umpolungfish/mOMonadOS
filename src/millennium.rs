// millennium.rs — the Millennium Prize conjectures read as OVM addresses.
//
// Each entry below is a word an ob3ect minted for one conjecture's structural
// promotion (the assignment that would carry the open Ω-value if it moved).
// What the ob3ect's own JSON stores as `grounded_tuple` is the address that
// word's DESCRIPTION grounds to — read off the reasoning text, never run. This
// tool answers a different question: what address does the word itself
// EXECUTE to, through the same kernel-backed pipeline `crystal_address_of` in
// repair.rs already uses (program_from_glyphs -> self_imscribe ->
// IgTuple::from_snapshot -> crystal_address). The two addresses are not
// comparable and this tool never treats them as if they were — it only ever
// prints the executed one, then runs weight | banked | insert on the raw word
// the same way `combo` does for any other word.

#![allow(dead_code)]
extern crate alloc;

use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// name, one-line note on which value would move if the conjecture closed, glyph word
const PROBLEMS: &[(&str, &str, &str)] = &[
    (
        "rh_positivity_promotion",
        "positivity route: the promoted word if the explicit formula's positivity closes",
        "⊢∈≻⊤≺⊥⊞⋈∋⊡⊙⊣",
    ),
    (
        "rh_winding_lift",
        "winding route: the promoted word if the zero-counting winding number lifts",
        "⊢∈≻⊤≺⊥⊞⋈⊙∋⊡⊣",
    ),
    (
        "bsd_parity_promotion",
        "rank-parity promotion for the Birch–Swinnerton-Dyer conjecture",
        "⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣",
    ),
    (
        "hodge_parity_promotion",
        "algebraic-cycle parity promotion for the Hodge conjecture",
        "⊢∈⊥≺⊤≻⋈⊞∋⊙⊡⊣",
    ),
    (
        "collatz_or_closure",
        "halving and tripling arms fused Frobenius-self-dual",
        "⊢∈≻⊤≺⊥⋈⊞⊙∋⊡⋈⊙⊣",
    ),
    (
        "navier_stokes",
        "smoothness/blowup promotion for Navier–Stokes existence and smoothness",
        "⊢⋈∈≻⊤≺⊥⊞⊡∋⊙⊣",
    ),
    (
        "yang_mills",
        "mass-gap promotion for Yang–Mills existence and mass gap",
        "⊢⊣∈≻⊤≺⊥⋈⊞⊙∋⊡⋈≻≺⊤⊥⊞⊡⊣",
    ),
];

/// word -> tuple -> crystal address, run live through the kernel. Same
/// pipeline as repair.rs's `crystal_address_of`, called directly on the
/// underlying functions rather than through the CLI path law 17 flags.
fn executed_tuple(word: &str) -> Option<crate::imas_ig::IgTuple> {
    let prog = crate::belnap_ring_shor::program_from_glyphs(word).ok()?;
    let snap = crate::kernel::self_imscribe(&prog);
    Some(crate::imas_ig::IgTuple::from_snapshot(&snap))
}

fn slot_line(label: &str, mark: &str, val: crate::imas_ig::IgPrim) -> String {
    format!("  {:<16} {}  {}", label, mark, val.short())
}

fn report_one(name: &str, note: &str, word: &str) -> String {
    let mut out = String::new();
    out.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    out.push_str(&format!("{}\n", name));
    out.push_str(&format!("{}\n", note));
    out.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    out.push_str(&format!("word: {}\n", word));

    match executed_tuple(word) {
        Some(t) => {
            out.push_str(&format!("executed crystal address: {}\n", t.crystal_address()));
            out.push_str("executed tuple:\n");
            out.push_str(&slot_line("dimensionality", "⊢", t.d)); out.push('\n');
            out.push_str(&slot_line("topology", "⊣", t.t)); out.push('\n');
            out.push_str(&slot_line("recognition", "≻", t.r)); out.push('\n');
            out.push_str(&slot_line("parity", "≺", t.p)); out.push('\n');
            out.push_str(&slot_line("fidelity", "⋈", t.f)); out.push('\n');
            out.push_str(&slot_line("kinetics", "⊤", t.k)); out.push('\n');
            out.push_str(&slot_line("granularity", "∈", t.g)); out.push('\n');
            out.push_str(&slot_line("grammar", "∋", t.c)); out.push('\n');
            out.push_str(&slot_line("criticality", "⊙", t.phi)); out.push('\n');
            out.push_str(&slot_line("chirality", "⊥", t.h)); out.push('\n');
            out.push_str(&slot_line("stoichiometry", "⊞", t.s)); out.push('\n');
            out.push_str(&slot_line("protection", "⊡", t.omega)); out.push('\n');
        }
        None => {
            out.push_str("executed tuple: word did not parse as a program (bad glyph)\n");
        }
    }

    out.push('\n');
    out.push_str("┌─ weight ────────────────────────────────────────────────────\n");
    for line in imasm_core::lattice_flow::weight_report(word).lines() {
        out.push_str("│ "); out.push_str(line); out.push('\n');
    }
    out.push_str("├─ banked ────────────────────────────────────────────────────\n");
    for line in imasm_core::lattice_flow::banked_report(word).lines() {
        out.push_str("│ "); out.push_str(line); out.push('\n');
    }
    out.push_str("└─ insert ────────────────────────────────────────────────────\n");
    for line in imasm_core::lattice_flow::insert_report(word).lines() {
        out.push_str("│ "); out.push_str(line); out.push('\n');
    }
    out
}

/// millennium [name]   run every problem's word, or just one by name.
pub fn millennium_main(args: &[&str]) -> String {
    let mut out = String::new();
    if args.first().copied() == Some("list") {
        out.push_str("millennium [name]   run the OVM instrument suite on a Millennium\n");
        out.push_str("                    conjecture's promotion word (or every one, with\n");
        out.push_str("                    no argument). names:\n");
        for (name, note, _) in PROBLEMS {
            out.push_str(&format!("  {:<24} {}\n", name, note));
        }
        return out;
    }

    let selected: Vec<&(&str, &str, &str)> = if let Some(want) = args.first() {
        PROBLEMS.iter().filter(|(name, _, _)| name == want).collect()
    } else {
        PROBLEMS.iter().collect()
    };

    if selected.is_empty() {
        out.push_str(&format!(
            "no problem named {:?} — run `millennium list` for names\n", args.first()
        ));
        return out;
    }

    for (i, (name, note, word)) in selected.iter().enumerate() {
        if i > 0 { out.push('\n'); }
        out.push_str(&report_one(name, note, word));
    }
    out
}

pub fn repl_millennium(args: &[&str]) {
    sprintln!("{}", millennium_main(args));
}
