// ─── ouroboros.rs ──────────────────────────────────────────────────────
// Inverse grammar (build.txt, arcane §1).
//
// The loop is  tuple -> IMASM word -> braid word -> ... -> tuple.  Two of its
// legs already exist and are used here rather than rebuilt:
//
//   forward   word -> Program -> self_imscribe -> Snapshot -> IgTuple
//             (belnap_ring_shor::program_from_glyphs, kernel::self_imscribe,
//              imas_ig::IgTuple::from_snapshot)
//   braid     Program -> braid generators   (braid_protocol::read_tangle)
//
// What did NOT exist is the inverse. `sequence::build_via_substrate` CONSTRUCTS
// a word from a tuple, but construction is not inversion: it writes one word of
// a fixed length without asking whether a shorter one imscribes the same tuple.
// This module SEARCHES for the shortest, which is a different question and the
// one that measures how lossy the map really is.
//
// The search is exhaustive and therefore exact: every word of length 1, then
// every word of length 2, and so on. The first hit is genuinely shortest
// because nothing shorter was skipped. Past the budget it reports NOT FOUND
// rather than returning the constructive word and calling it minimal.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::belnap_ring_shor::{glyphs_from_program, program_from_glyphs};
use crate::counterfactual::MARKS;
use crate::imas_ig::{IgPrim, IgTuple};
use crate::kernel::self_imscribe;

/// Longest word length the search enumerates exhaustively by default.
/// 12^1+12^2+12^3+12^4 = 22,620 candidates, each one imscription: ~2.5s of
/// kernel time, measured. Length 5 is 12x that and is opt-in via --depth.
pub const MAX_SEARCH_LEN: usize = 4;
pub const MAX_SEARCH_LEN_CEILING: usize = 6;

/// The forward map, whole: a glyph word to the tuple it imscribes.
pub fn imscribes(word: &str) -> Option<IgTuple> {
    let prog = program_from_glyphs(word).ok()?;
    Some(IgTuple::from_snapshot(&self_imscribe(&prog)))
}

pub struct Inversion {
    pub target: IgTuple,
    /// The shortest word imscribing the target, if one exists within budget.
    pub shortest: Option<String>,
    pub searched: usize,
    pub exhausted_to: usize,
    /// What `build_via_substrate` writes for this tuple — construction, not search.
    pub constructive: String,
    /// Words found at the shortest length: the map's fibre, so its lossiness.
    pub siblings: usize,
    /// Braid generators read off the shortest word's program.
    pub braid: Option<Vec<i32>>,
    pub braid_refused: Option<String>,
}

fn each_word_of_len(n: usize, mut f: impl FnMut(&str) -> bool) -> usize {
    let base = MARKS.len();
    let total = base.pow(n as u32);
    let mut buf: Vec<char> = Vec::with_capacity(n);
    let mut seen = 0usize;
    for code in 0..total {
        buf.clear();
        let mut c = code;
        for _ in 0..n {
            buf.push(MARKS[c % base]);
            c /= base;
        }
        let w: String = buf.iter().collect();
        seen += 1;
        if f(&w) {
            break;
        }
    }
    seen
}

pub fn invert(target: &IgTuple) -> Inversion {
    invert_to_depth(target, MAX_SEARCH_LEN)
}

pub fn invert_to_depth(target: &IgTuple, depth: usize) -> Inversion {
    let mut shortest: Option<String> = None;
    let mut searched = 0usize;
    let mut exhausted_to = 0usize;
    let mut siblings = 0usize;

    for len in 1..=depth.min(MAX_SEARCH_LEN_CEILING) {
        let mut hit: Option<String> = None;
        let mut fibre = 0usize;
        searched += each_word_of_len(len, |w| {
            if let Some(t) = imscribes(w) {
                if t == *target {
                    fibre += 1;
                    if hit.is_none() {
                        hit = Some(w.to_string());
                    }
                }
            }
            false // never break: counting the whole fibre at this length
        });
        exhausted_to = len;
        if hit.is_some() {
            shortest = hit;
            siblings = fibre;
            break;
        }
    }

    // The constructive word, for comparison. This is what the kernel already
    // writes for a tuple; the point of the search is to find out whether it is
    // longer than it needs to be.
    let prog = crate::sequence::build_via_substrate(
        target,
        12,
        target.t == IgPrim::are,
        3,
    );
    let constructive = glyphs_from_program(&prog);

    // The braid leg, read off the shortest word when there is one.
    let (braid, braid_refused) = match &shortest {
        Some(w) => match program_from_glyphs(w) {
            Ok(p) => {
                let toks: Vec<crate::tokens::Token> = p.as_slice().to_vec();
                match crate::braid_protocol::read_tangle(&toks, toks.len() + 2, 1) {
                    Ok(r) => (Some(r.generators), None),
                    Err(e) => (None, Some(e)),
                }
            }
            Err((i, c)) => (None, Some(format!("glyph {} at {} is not a token", c, i))),
        },
        None => (None, None),
    };

    Inversion {
        target: *target,
        shortest,
        searched,
        exhausted_to,
        constructive,
        siblings,
        braid,
        braid_refused,
    }
}

pub fn format_inversion(inv: &Inversion) -> String {
    let mut out = String::new();
    out.push_str("OUROBOROS-INVERSE\n=================\n\n");
    out.push_str(&format!("target tuple:  {}\n\n", inv.target.display()));

    match &inv.shortest {
        Some(w) => {
            let n = w.chars().count();
            out.push_str(&format!("IMASM (shortest):  {}   [{} glyphs]\n", w, n));

            let cn = inv.constructive.chars().count();
            out.push_str(&format!(
                "IMASM (constructed): {}   [{} glyphs]\n",
                inv.constructive, cn
            ));
            if n > 0 {
                out.push_str(&format!(
                    "gain:              {:.1}x shorter than the constructive word\n",
                    cn as f64 / n as f64
                ));
            }
            out.push_str(&format!(
                "fibre at length {}: {} word(s) imscribe this same tuple\n",
                n, inv.siblings
            ));
            if inv.siblings > 1 {
                out.push_str(
                    "                   the map is many-to-one here: what the imscription\n\
                     \x20                  DOES is recoverable, what it CLAIMS is not\n",
                );
            }

            match (&inv.braid, &inv.braid_refused) {
                (Some(gens), _) => {
                    out.push_str(&format!("braid:             {:?}\n", gens));
                }
                (None, Some(why)) => {
                    out.push_str(&format!("braid:             mu refused — {}\n", why));
                }
                (None, None) => {}
            }
        }
        None => {
            out.push_str(&format!(
                "IMASM (shortest):  NOT FOUND — {} words searched, every word of\n\
                 \x20                  length 1..{} enumerated exhaustively.\n",
                inv.searched, inv.exhausted_to
            ));
            out.push_str(&format!(
                "IMASM (constructed): {}\n",
                inv.constructive
            ));
            out.push_str(
                "\nNo word this short imscribes the target. The constructive word above\n\
                 is what the kernel writes, not a minimum — it is shown for comparison\n\
                 and is not claimed to be the answer.\n",
            );
        }
    }

    out.push_str(&format!(
        "\nsearched: {} words, exhaustive through length {}.\n",
        inv.searched, inv.exhausted_to
    ));
    out
}

pub fn ouroboros_main(args: &[&str]) -> String {
    if args.is_empty() {
        return "ouroboros-inverse <12-glyph tuple>   (alias: oinv)\n\
                \n\
                Given a tuple, SEARCH for the shortest IMASM word that imscribes\n\
                it, and read the braid off that word. Exhaustive through length\n\
                4, so the first hit is genuinely shortest — and NOT FOUND is\n\
                reported as not found, never filled in with the constructive word.\n\
                \n\
                Try:  oinv <paste a tuple from `classify` or `ig`>\n\
                      oinv --forward \u{22a2}\u{2208}\u{22a4}\u{220b}   (what tuple does this word imscribe?)\n\
                      oinv <tuple> --depth 5   (12x the search, opt-in)\n"
            .to_string();
    }

    // The forward leg, exposed: what tuple does this WORD imscribe? Needed to
    // read the loop in the direction it already runs, and to find targets the
    // inverse search can actually reach.
    if args[0] == "--forward" || args[0] == "-f" {
        if args.len() < 2 {
            return "ouroboros-inverse --forward <imasm word>\n".to_string();
        }
        let word = args[1];
        return match imscribes(word) {
            Some(t) => format!(
                "FORWARD\n=======\n\nIMASM:  {}\ntuple:  {}\n",
                word,
                t.display()
            ),
            None => format!("'{}' is not a readable IMASM word.\n", word),
        };
    }

    let mut depth = MAX_SEARCH_LEN;
    let mut glyphs: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--depth" || args[i] == "-d" {
            if i + 1 < args.len() {
                depth = args[i + 1].parse::<usize>().unwrap_or(MAX_SEARCH_LEN);
                i += 1;
            }
        } else {
            glyphs.push(args[i]);
        }
        i += 1;
    }

    let joined = glyphs.join("");
    match IgTuple::from_glyphs(&joined) {
        Ok(t) => format_inversion(&invert_to_depth(&t, depth)),
        Err((i, msg)) => format!(
            "not a tuple at position {}: {}\n\nGive twelve primitive glyphs.\n",
            i, msg
        ),
    }
}
