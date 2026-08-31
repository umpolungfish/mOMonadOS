//! lattice_flow — the word instruments. The implementation moved into
//! `imasm_core` so the host `ask` binary can reach them; a second copy is drift.
//!
//! What stays here is the part that is genuinely the kernel's: `insert_sweep_all`
//! walks THIS kernel's program table, which the shared crate has no notion of.
//! The reports below are one-line wrappers that print what the shared function
//! returns.

use alloc::string::String;
use crate::sprintln;

pub use imasm_core::lattice_flow::{normalize, banked_walk};

macro_rules! wrap {
    ($name:ident) => {
        pub fn $name(word: &str) {
            sprintln!("{}", imasm_core::lattice_flow::$name(word));
        }
    };
}
wrap!(cycle_report);
wrap!(transitions_report);
wrap!(banked_report);
wrap!(insert_report);
wrap!(weight_report);

/// Render a program as a glyph word, in the alphabet the walkers parse.
fn program_word(p: &crate::tokens::Program) -> String {
    let mut w = String::new();
    for t in p.as_slice() { w.push_str(t.code()); }
    normalize(&w)
}

/// Every built-in program, put to the same question.
///
/// The words are not typed in; the kernel hands over its own programs and each
/// is rendered from its tokens. A word that holds is left alone, and a word
/// that is exposed is asked how many single glyphs would close it -- which is
/// the interesting number, because a word with no repair at all is exposed for
/// a structural reason rather than a missing symbol.
pub fn insert_sweep_all()  {
    use crate::tokens::*;

    let families: [(&str, usize); 5] = [
        ("canonical",  canonical_count()),
        ("continuous", continuous_count()),
        ("novel",      novel_count()),
        ("shunted",    shunted_count()),
        ("compound",   compound_count()),
    ];

    let mut total = 0u32;
    let mut holding = 0u32;
    let mut vacuous = 0u32;
    let mut repairable = 0u32;
    let mut stuck = 0u32;

    for (fam, count) in families.iter() {
        sprintln!("── {} ──", fam);
        for i in 0..*count {
            let prog = match *fam {
                "canonical"  => canonical(i),
                "continuous" => continuous_program(i),
                "novel"      => novel_program(i),
                "shunted"    => shunted_program(i),
                _            => compound_program(i),
            };
            let prog = match prog { Some(p) => p, None => continue };
            let name = match *fam {
                "canonical"  => canonical_name(i),
                "continuous" => continuous_name(i),
                "novel"      => novel_name(i),
                "shunted"    => shunted_name(i),
                _            => compound_name(i),
            };
            let word = program_word(&prog);
            if word.is_empty() { continue; }
            total += 1;

            match banked_walk(&word) {
                Some(b) if b.holds() => {
                    holding += 1;
                    sprintln!("  {:<28} holds        {}", name, word);
                }
                Some(b) if b.vacuous() => {
                    vacuous += 1;
                    sprintln!("  {:<28} vacuous — no clear fired", name);
                }
                Some(b) => {
                    let lost: u32 = b.exposed.iter().map(|e| e.2).sum();
                    let r = imasm_core::lattice_flow::repair_count(&word);
                    if r == 0 { stuck += 1; } else { repairable += 1; }
                    sprintln!("  {:<28} exposed {} — {} repair(s)   {}", name, lost, r, word);
                }
                None => { total -= 1; }
            }
        }
    }

    sprintln!("");
    sprintln!("{} programs: {} hold, {} vacuous, {} exposed-and-repairable, {} exposed-with-no-repair",
        total, holding, vacuous, repairable, stuck);
}