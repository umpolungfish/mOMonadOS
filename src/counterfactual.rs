// ─── counterfactual.rs ─────────────────────────────────────────────────
// "What if I perturb reality?" (build.txt §7)
//
// A structural sensitivity analyzer. Perturb one glyph and read what moved:
// which invariants held, which broke, whether the perturbation can be undone,
// and the smallest repair that puts the word back in its original basin.
//
// It owns no semantics of its own. The verdict and register come from
// imasm_core, the banking walk from lattice_flow, and the repair search is the
// same exact single-glyph sweep `insert` uses. This tool only *compares* two
// walks — which is the whole point: discovering which symbols actually matter
// rather than merely correlating with an outcome.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use imasm_core::imasm16_3::{parse_glyph_word, run_word_register, tri_ancestral_verdict};

use crate::lattice_flow::{banked_walk, normalize};

/// The twelve marks, in canonical order.
pub const MARKS: [char; 12] = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '⊡'];

/// Everything one walk knows about a word. Compared field-by-field against
/// the perturbed walk to produce the held/broken split.
#[derive(Clone, PartialEq)]
pub struct Reading {
    pub word: String,
    pub length: usize,
    pub verdict: char,
    pub verdict_why: String,
    pub register: String,
    pub holds: bool,
    pub vacuous: bool,
    pub live_clears: u32,
    pub deposits: u32,
    pub inert: u32,
    pub exposed: usize,
}

pub fn read(word: &str) -> Option<Reading> {
    let norm = normalize(word);
    let steps = parse_glyph_word(&norm);
    if steps.is_empty() {
        return None;
    }
    let (verdict, why) = tri_ancestral_verdict(&steps);
    let b = banked_walk(&norm)?;
    Some(Reading {
        word: norm,
        length: steps.len(),
        verdict,
        verdict_why: why,
        register: run_word_register(&steps),
        holds: b.holds(),
        vacuous: b.vacuous(),
        live_clears: b.live_clears,
        deposits: b.deposits,
        inert: b.inert,
        exposed: b.exposed.len(),
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum Perturbation {
    Replace(char, char),
    Delete(char),
    Insert(char, usize),
    Rotate(isize),
}

impl Perturbation {
    pub fn describe(&self) -> String {
        match self {
            Perturbation::Replace(a, b) => format!("replace {} -> {}", a, b),
            Perturbation::Delete(a) => format!("delete {}", a),
            Perturbation::Insert(a, p) => format!("insert {} at {}", a, p),
            Perturbation::Rotate(k) => format!("rotate {}", k),
        }
    }

    /// A perturbation is reversible when an inverse perturbation exists that
    /// restores the word exactly. Rotation always has one. Replacement has one
    /// only when the replaced mark is not already present elsewhere, otherwise
    /// the inverse would also rewrite an untouched occurrence. Deletion loses
    /// the position and insertion of a duplicate loses which copy was added.
    pub fn reversible(&self, original: &str) -> (bool, &'static str) {
        match self {
            Perturbation::Rotate(_) => (true, "rotate by -k restores the word"),
            Perturbation::Replace(_a, b) => {
                let count_b = original.chars().filter(|c| c == b).count();
                if count_b == 0 {
                    (true, "the target mark was absent, so replacing back is unambiguous")
                } else {
                    (false, "the target mark already occurs, so the inverse cannot tell the copies apart")
                }
            }
            Perturbation::Delete(_) => (false, "deletion discards the position"),
            Perturbation::Insert(_, _) => (true, "delete at the same position restores the word"),
        }
    }
}

pub fn apply(word: &str, p: Perturbation) -> String {
    let chars: Vec<char> = normalize(word).chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    match p {
        Perturbation::Replace(from, to) => chars
            .iter()
            .map(|&c| if c == from { to } else { c })
            .collect(),
        Perturbation::Delete(g) => chars.iter().filter(|&&c| c != g).collect(),
        Perturbation::Insert(g, pos) => {
            let mut v = chars.clone();
            let at = pos.min(v.len());
            v.insert(at, g);
            v.into_iter().collect()
        }
        Perturbation::Rotate(k) => {
            let n = chars.len() as isize;
            let shift = ((k % n) + n) % n;
            let mut v: Vec<char> = Vec::with_capacity(chars.len());
            for i in 0..chars.len() {
                v.push(chars[((i as isize + shift) % n) as usize]);
            }
            v.into_iter().collect()
        }
    }
}

/// Which named invariants survived, and which did not.
pub fn compare(before: &Reading, after: &Reading) -> (Vec<String>, Vec<String>) {
    let mut held = Vec::new();
    let mut broke = Vec::new();

    let mut check = |name: &str, same: bool, detail: String| {
        if same {
            held.push(name.to_string());
        } else {
            broke.push(format!("{}: {}", name, detail));
        }
    };

    check(
        "verdict",
        before.verdict == after.verdict,
        format!("{} -> {}", before.verdict, after.verdict),
    );
    check(
        "final register",
        before.register == after.register,
        format!("{} -> {}", before.register, after.register),
    );
    check(
        "length",
        before.length == after.length,
        format!("{} -> {}", before.length, after.length),
    );
    check(
        "banking holds",
        before.holds == after.holds,
        format!("{} -> {}", before.holds, after.holds),
    );
    check(
        "live clears",
        before.live_clears == after.live_clears,
        format!("{} -> {}", before.live_clears, after.live_clears),
    );
    check(
        "deposits",
        before.deposits == after.deposits,
        format!("{} -> {}", before.deposits, after.deposits),
    );
    check(
        "inert tail",
        before.inert == after.inert,
        format!("{} -> {}", before.inert, after.inert),
    );
    check(
        "exposed clears",
        before.exposed == after.exposed,
        format!("{} -> {}", before.exposed, after.exposed),
    );

    (held, broke)
}

/// The exact single-glyph insertion sweep: the smallest edit putting the
/// perturbed word back in the original's basin (same verdict AND same banking
/// standing). Exhaustive over 12 marks x every position, so the answer is
/// found rather than argued.
pub fn smallest_repair(perturbed: &str, target: &Reading) -> Option<(String, char, usize)> {
    let chars: Vec<char> = normalize(perturbed).chars().collect();
    for pos in 0..=chars.len() {
        for &g in MARKS.iter() {
            let mut v = chars.clone();
            v.insert(pos, g);
            let cand: String = v.into_iter().collect();
            if let Some(r) = read(&cand) {
                if r.verdict == target.verdict && r.holds == target.holds {
                    return Some((cand, g, pos));
                }
            }
        }
    }
    None
}

fn parse_perturbation(args: &[&str]) -> Result<Perturbation, String> {
    if args.len() < 2 {
        return Err("missing perturbation".to_string());
    }
    let first = |s: &str| s.chars().next();
    match args[0] {
        "replace" => {
            if args.len() < 3 {
                return Err("replace needs two glyphs: replace <from> <to>".to_string());
            }
            match (first(args[1]), first(args[2])) {
                (Some(a), Some(b)) => Ok(Perturbation::Replace(a, b)),
                _ => Err("replace needs two glyphs".to_string()),
            }
        }
        "delete" => match first(args[1]) {
            Some(a) => Ok(Perturbation::Delete(a)),
            None => Err("delete needs a glyph".to_string()),
        },
        "insert" => {
            let g = first(args[1]).ok_or_else(|| "insert needs a glyph".to_string())?;
            let pos = if args.len() > 2 {
                args[2].parse::<usize>().unwrap_or(0)
            } else {
                0
            };
            Ok(Perturbation::Insert(g, pos))
        }
        "rotate" => {
            let k = args[1]
                .parse::<isize>()
                .map_err(|_| "rotate needs an integer".to_string())?;
            Ok(Perturbation::Rotate(k))
        }
        other => Err(format!("unknown perturbation '{}'", other)),
    }
}

pub fn format_counterfactual(word: &str, args: &[&str]) -> String {
    let p = match parse_perturbation(args) {
        Ok(p) => p,
        Err(e) => return format!("{}\n\n{}", e, usage()),
    };

    let before = match read(word) {
        Some(r) => r,
        None => return format!("'{}' parses to no tokens — nothing to perturb.\n", word),
    };
    let perturbed_word = apply(word, p);
    let after = match read(&perturbed_word) {
        Some(r) => r,
        None => {
            return format!(
                "COUNTERFACTUAL\n==============\n\n\
                 original:    {}\n\
                 perturbation: {}\n\
                 perturbed:   {} — parses to no tokens.\n\n\
                 The perturbation destroyed the word rather than moving it.\n",
                before.word,
                p.describe(),
                perturbed_word
            )
        }
    };

    let (held, broke) = compare(&before, &after);
    let (rev, rev_why) = p.reversible(&before.word);

    let mut out = String::new();
    out.push_str("COUNTERFACTUAL\n==============\n\n");
    out.push_str(&format!("original:     {}\n", before.word));
    out.push_str(&format!("perturbation: {}\n", p.describe()));
    out.push_str(&format!("perturbed:    {}\n\n", after.word));

    out.push_str(&format!(
        "               {:<16} {:<16}\n",
        "BEFORE", "AFTER"
    ));
    out.push_str(&format!(
        "  verdict      {:<16} {:<16}\n",
        before.verdict, after.verdict
    ));
    out.push_str(&format!(
        "  register     {:<16} {:<16}\n",
        before.register, after.register
    ));
    out.push_str(&format!(
        "  banking      {:<16} {:<16}\n",
        if before.holds { "holds" } else if before.vacuous { "vacuous" } else { "exposed" },
        if after.holds { "holds" } else if after.vacuous { "vacuous" } else { "exposed" }
    ));
    out.push_str(&format!(
        "  live clears  {:<16} {:<16}\n",
        before.live_clears, after.live_clears
    ));
    out.push_str(&format!(
        "  deposits     {:<16} {:<16}\n\n",
        before.deposits, after.deposits
    ));

    out.push_str(&format!("invariants held ({}):\n", held.len()));
    if held.is_empty() {
        out.push_str("    none — the perturbation moved everything\n");
    } else {
        for h in &held {
            out.push_str(&format!("    {}\n", h));
        }
    }

    out.push_str(&format!("\ninvariants broken ({}):\n", broke.len()));
    if broke.is_empty() {
        out.push_str("    none — the perturbation is invisible to every reading\n");
    } else {
        for b in &broke {
            out.push_str(&format!("    {}\n", b));
        }
    }

    out.push_str(&format!(
        "\nreversible:   {} — {}\n",
        if rev { "yes" } else { "no" },
        rev_why
    ));

    if broke.is_empty() {
        out.push_str("\nsmallest repair:\n    not needed — the word never left its basin\n");
    } else {
        match smallest_repair(&after.word, &before) {
            Some((restored, g, pos)) => {
                out.push_str("\nsmallest repair (exact sweep, 12 marks x every position):\n");
                out.push_str(&format!("    insert {} at {}\n", g, pos));
                out.push_str(&format!("    {}\n", restored));
                out.push_str("    restores the original verdict and banking standing\n");
            }
            None => {
                out.push_str(
                    "\nsmallest repair:\n    none — no single insertion returns it to the \
                     original basin\n",
                );
            }
        }
    }

    out
}

fn usage() -> String {
    "counterfactual <word> replace <from> <to>\n\
     counterfactual <word> delete <glyph>\n\
     counterfactual <word> insert <glyph> [position]\n\
     counterfactual <word> rotate <k>\n\
     \n\
     Perturb one glyph and read what moved: which invariants held, which broke,\n\
     whether it can be undone, and the smallest repair back to the basin.\n"
        .to_string()
}

pub fn counterfactual_main(args: &[&str]) -> String {
    if args.len() < 2 {
        return usage();
    }
    format_counterfactual(args[0], &args[1..])
}
