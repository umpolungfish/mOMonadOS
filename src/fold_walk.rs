//! The fold verdict of a word, computed on the kernel.
//!
//! Four steps, each computed here rather than printed: the pairing procedure
//! against the closed form over a corpus of deposited structures, the surplus
//! identity, the witness that the enclosure condition is about work and not
//! about length, and the codon lane run against the structural lane.

use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use crate::erdos_walks::{finish, rule, show, Step};
use crate::sprintln;

/// The seven marks that transform a state. The boundaries, self-reference, and
/// the fork and fuse themselves do not.
fn does_work(m: char) -> bool {
    matches!(m, '\u{227b}' | '\u{227a}' | '\u{22c8}' | '\u{22a4}' | '\u{22a5}' | '\u{229e}' | '\u{25fb}')
}
const FORK: char = '\u{2208}';
const FUSE: char = '\u{220b}';

/// Pair forks against fuses around the cycle. The word is read from the point
/// where the running balance of forks against fuses is least, which is the
/// rotation at which the stack never underflows more than it must; the pairing
/// then leaves unmatched only the surplus species. Returns the unmatched counts
/// and the paired regions as index pairs into the word.
fn pair_cyclic(w: &[char]) -> (usize, usize, Vec<(usize, usize)>) {
    let n = w.len();
    if n == 0 {
        return (0, 0, Vec::new());
    }
    let mut b: i64 = 0;
    let mut best: i64 = 0;
    let mut start = 0usize;
    for (i, &m) in w.iter().enumerate() {
        b += if m == FORK { 1 } else if m == FUSE { -1 } else { 0 };
        if b < best {
            best = b;
            start = i + 1;
        }
    }
    let mut stack: Vec<usize> = Vec::new();
    let mut regions: Vec<(usize, usize)> = Vec::new();
    let mut unmatched_fuse = 0usize;
    for off in 0..n {
        let i = (start + off) % n;
        if w[i] == FORK {
            stack.push(i);
        } else if w[i] == FUSE {
            match stack.pop() {
                Some(s) => regions.push((s, i)),
                None => unmatched_fuse += 1,
            }
        }
    }
    (stack.len(), unmatched_fuse, regions)
}

/// A region is substantial when it encloses a mark that does work.
fn substantial(w: &[char], r: (usize, usize)) -> bool {
    let n = w.len();
    let span = (r.1 + n - r.0 - 1) % n;
    (0..span).any(|j| does_work(w[(r.0 + 1 + j) % n]))
}

/// The readout by the procedure of the definition.
fn verdict_procedure(w: &[char]) -> char {
    let (uf, ug, regions) = pair_cyclic(w);
    let forks = w.iter().filter(|&&m| m == FORK).count();
    let fuses = w.iter().filter(|&&m| m == FUSE).count();
    if forks == 0 && fuses == 0 {
        return 'N';
    }
    if ug > uf {
        return 'F';
    }
    if uf > 0 {
        return 'B';
    }
    if regions.iter().any(|&r| substantial(w, r)) {
        'T'
    } else {
        'N'
    }
}

/// The readout by the closed form: a sign, and one existential condition.
fn verdict_closed(w: &[char]) -> char {
    let forks = w.iter().filter(|&&m| m == FORK).count() as i64;
    let fuses = w.iter().filter(|&&m| m == FUSE).count() as i64;
    let d = forks - fuses;
    if d > 0 {
        return 'B';
    }
    if d < 0 {
        return 'F';
    }
    let (_, _, regions) = pair_cyclic(w);
    if regions.iter().any(|&r| substantial(w, r)) {
        'T'
    } else {
        'N'
    }
}

/// A representative codon for each residue, and the mark that codon emits.
/// Unpromoted residues emit nothing.
fn residue_codon(aa: char) -> &'static str {
    match aa {
        'M' => "AUG", 'W' => "UGG", 'C' => "UGC", 'Y' => "UAU", 'F' => "UUU",
        'I' => "AUC", 'N' => "AAC", 'Q' => "CAG", 'H' => "CAU", 'D' => "GAC",
        'K' => "AAA", 'E' => "GAA",
        'A' => "GCU", 'R' => "CGU", 'G' => "GGU", 'L' => "CUU", 'P' => "CCU",
        'S' => "UCU", 'T' => "ACU", 'V' => "GUU",
        _ => "NNN",
    }
}

/// The lift from a codon to a mark. Twenty-three of the sixty-four are promoted.
fn lift_codon(c: &str) -> Option<char> {
    Some(match c {
        "AUG" => '\u{22a2}',
        "UGG" => '\u{22a3}',
        "UGU" | "UGC" => '\u{227b}',
        "UAU" | "UAC" => '\u{227a}',
        "UUU" | "UUC" => '\u{22c8}',
        "AUU" | "AUC" | "AUA" => '\u{22a4}',
        "AAU" | "AAC" => FORK,
        "CAA" | "CAG" => FUSE,
        "CAU" | "CAC" => '\u{2299}',
        "GAU" | "GAC" => '\u{22a5}',
        "AAA" | "AAG" => '\u{229e}',
        "GAA" | "GAG" => '\u{25fb}',
        _ => return None,
    })
}

/// The corpus: one deposited structure per row, with the verdict it was
/// recorded at and the word it lifts to.
const CORPUS: &[(&str, &str)] = &[
    ("B", "⊢⊣≺⋈≻⊥∈⊞≺⊡⊞"),
    ("B", "⊞⊡⊡⋈⊤⊡⊥⊥∈⊙⊞⋈⊡⊡⊥≺⊞⊞⋈⊤≻⊞⊣⋈∋≻⋈≺⊥⊙⊞⊙⊥⋈⋈⊞⊡≺∋⊡⊤⋈⋈⊞⊥⊥∈≺⊞⊡⊞⋈⊡⊥∈⊤⊡⊞⊤⊥⋈⊞⊡⊥∈⊤⊙⊞⊡≺∈≺∈⊙∈≺⊤⊥⊞∋⊞∈⊤⊞∈⋈⊞⊤⊙∈⊤⊡⊥∋⊥⊙≺∋∋∈⊤⊥⊥∈⊙≺∋⊞⊥∈⊡⊞⊥⊙⊡⋈⊤"),
    ("B", "∈⊡∈≻⊤⊤∈⋈≻⋈⊡⊤⊡⊤∈∈⊡⊡∈⊡⊤⊢⊥∋⊥∈⊡⊞⊞∋⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊡⋈⊡⊤⊡⊞⊥"),
    ("B", "⊢⊣⊙⋈≺≻⊥∈⊞⊞≺⊡⊢"),
    ("B", "⊢⊞⊡⊡⋈⊤⊡⊥⊥∈⊙⊞⋈⊡⊡⊥≺⊞⊞⋈⊤≻⊞⊣⋈≺∋≻⋈≺⊥⊙⊢⊞∋⊙⊥⋈⋈⊞⊢⊡≺∋⊡⊤⋈⋈⊞⊥⊥∈≺⊞⊡⊞⋈⊡⊥∈⊤⊡⊞⊤⊥⋈⊞⊡⊥∈⊤⊙⊞⊡≺∈≺∈⊙∈≺⊤⊢⊥⊞∋⊞∈⊤⊞∈⋈⊞⊤⊙∈⊤⊡⊥∋⊥⊙≺∋∋∈⊤⊥⊥∈⊙≺∋⊞⊥∈⊡⊞⊥⊙⊢⊡⋈⊤⊙⊢⊥⊡≺⊞"),
    ("B", "⊢⊞⊡⊡⋈⊤⊡⊥⊥∈⊙⊞⋈⊡⊡⊥≺⊞⊞⋈⊤≻⊞⊣⋈≺∋≻⋈≺⊥⊙⊢⊞∋⊙⊥⋈⋈⊞⊢⊡≺∋⊡⊤⋈⋈⊞⊥⊥∈≺⊞⊡⊞⋈⊡⊥∈⊤⊡⊞⊤⊥⋈⊞⊡⊥∈⊤⊙⊞⊡≺∈≺∈⊙∈≺⊤⊢⊥⊞∋⊞∈⊤⊞∈⋈⊞⊤⊙∈⊤⊡⊥∋⊥⊙≺∋∋∈⊤⊥⊥∈⊙≺∋⊞⊥∈⊡⊞⊥⊙⊢⊡⋈⊤⊙⊢⊥⊡≺⊞"),
    ("B", "⊞⊡⊡⋈⊤⊡⊥⊥∈⊙⊞⋈⊡⊡⊥≺⊞⊞⋈⊤⊤≻⊞⊣∋≻⋈≺⊥⊙⊢⊞⊞∋⊙⊥⋈⋈⊞⊢⊡≺∋⊡⊤⋈⋈⊞⊞⊥⊥∈≺⊞⊞⊡⊞⋈⊡⊥⊥∈⊤⊡⊡⊞⊤⊥⋈⊞⊡⊥∈⊤⊙⊞⊡≺∈≺∈⊙∈≺⊤⊢⊥⊞∋⊞∈⊤⊞∈∈⋈⊞⊤⊙∈⊤⊡⊡⊥∋⊥⊙≺∋∋∋∈⊤⊥⊥⊥⊥∈⊙≺∋⊞⊥∈⊡⊞⊥⊙⊢⊡⊡⋈⊤"),
    ("B", "∈⊡⊙∈⊤≻≻≺⊡⋈⊡≻⋈≻≻⊢∈≺⋈⊢≺⋈∋∈≺≺⊣⊥≺≺≺≺≺≺≺≺≺≺≺≺≺≺≺≺≺⊙≺≺≺≺≺⊙≺≺≺≺≺⊙≺≺≺≺≺⊙≺≺≺≺⊙≺≺≺≺⊙≺≺≺≺≺⊙≺≺≺≺≺⊙≺≺≺⊙≺⊙≺≺≺⊙≺≺≺≺≺⊙≺≺≺⊙≺⊙≺≺≺⊙≺≺≺≺≺⊙≺≺≺⊙≺⊙≺≺≺⊙≺≺≺≺≺⊙≺≺≺⊙≺⊙≺≺≺⊙≺≺≺≺⊙≺≺≺≺≺≺≺≺⊙≺≺≺≺≺∈≺≺≺≺≺≺≺≺⊙≺≺≺≺≺≺≺≺⊙≺≺≺≺≺≺≺"),
    ("B", "≺⊢⊡⊙⋈⊣⊞⊞⊞⊞≺∈⊡∈⊡⊡⋈⊡⋈"),
    ("B", "≺⊢⊡⊙⋈⊣⊞⊞⊞⊞≺∈⊡∈⊡⊡⋈⊡⋈"),
    ("B", "⊤⊤∈⊡∈⊤⊤⊡⊙⊡≻⊡⋈⊞⊤∈⊤⊤∈⊢∈⊡∈⋈≺⊢∈⊞⋈⊡≺∋≺≺⊡⊣⊞≺≺⊢⋈⋈⊤⋈⊤∈⋈≺⊤∋⊙⊞⊞∈≺⊤∈⊥⋈⊢⋈⋈⊢≺⊢⊙≺⋈⋈≻∈⊡⋈⋈⊡⊤⊣⊤⊡≺≻⊞⊢∈⋈⋈⊡∈⊙⊤⊢⋈⊣⊤⊢≻⊣≺⊤⊡⊢∋≻≻⊤⊥≺≺⊞⊡∈∈⊡⋈⊤≺⊢⋈⊙⋈⊤⊤⊤⋈⋈≻≺≻⊞⊡∋∋∋⊡∋⊞⊡⊞⊡⊢⊤⊤⊢⊤⋈⊤≻⊣≺⋈≺⊤⋈⊙∋⊡⋈⊤⋈⊢⋈⋈⊞≺∈⊢⊤≺⊤⊢⊢∈⊞∋⋈∈≻⊢⊤≻≻⊞∈⊥⊥⊡⊞⊡∋∋"),
    ("B", "∈⊤≻⊤∈⊤≻≻⊡≺≻⊙⊤∈⊡⊡≻⊡≻⊙⊢⊙⊡⊢≻∈⊢∋⊡⊥⋈∈≺⊤∈⊤⊥∈⋈⊡⋈⊥∈∋≺⊤⊥⊥⊥≺⊙⋈⋈≻⊡⊥≺∋⋈∈⊣⊢≺⊥⊞⊣⊞⊙⋈⊥≺∋∈≻⊤⋈⊡⊙≺≺⊥⋈⊥⊤⊙⊣⊡≺⊣⊡∈⋈⊡⊤∈⋈∈∋≺⊙≻⊡⊡⊥≺⋈⊥⊞≺⊤⋈≺⊥⋈⊡⊥⊥⊤∋⊥⊙⊥∋⊙≺⊥≺⊙⊥⊥⋈∋⊤⊢⊡⊡⋈≺⊞⊞⊥⊥⊡⊤≺⊥∋⋈⋈⊤≻⋈≻⋈⊙≺⊥∈⊢⊤⊞∋⊙∋⊤∋∈⊞⊥⊥⊥∈⊡⊥⊡⊞⊣⊞⊞⋈⊤≻⊙⋈⊞⊣∈⊣≻⋈⋈⊣∋⊥"),
    ("B", "⋈∈⊢∋≻∋⋈≺⊡⊙⊥∈∈⊡⊡∋∈⊞⊤⊞⊤⊥⊥≻"),
    ("B", "⊙≺⊥⊤≻⊞⊡⊡∈⊡⊤∈⋈⊥⊞⊤∈⊤⊡⊢⊥∈⊡⊥∈⋈⊥⊥⊥⋈⊥⊥⊡⊥⊥⊥∈⊥⊡⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥∈⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥∈⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥∈⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥∈⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥⊥"),
    ("B", "⊥⊞⊞⊡∋⊥⊥⊡⊤⊢∈⊥∈⊞⊥⊡≺≺⊙⊙⊡⊤⊡⊞∈⊥∈⊥⊤⋈⊙⋈⊤⊙⊡⊤⊡⊞⊙⊥∈∋⊥⊞⋈⊞⋈⊥⊤⊤⊥"),
    ("B", "⋈⊞≺⋈⊢⊙⊥≺⋈≺⊢⊡⊢≺∈∋⊢∋≺⊞⊢⊞⊙⊢⊡∈⊢⊞⊣⊢⊢⊤⊞⊢⊢⊥⊡≺≺⊢⊢⊞⊡∋∈⊡⊙⊣⊤≺⊣⊢⊣⊢∈⊤"),
    ("B", "⊞⋈≻⊡⊢⊞⊙⊥∈≺≺∈⊣≻⊞⋈⊡∈⋈∈∋∈∈⊥⊥≺⊤∋⊤∈⊣⊣≻∈⊥∈≻∈⊤≻⊥⊤∈≻⊞⊞⊤⊥∈⊢∈⊣⊣∈≻⊞⊥∋⊣⊤≻"),
    ("B", "⊞⋈≻⊡⊢⊞⊙⊥∈≺≺∈⊣≻⊞⋈⊡∈⋈∈∋∈∈⊥⊥≺⊤∋⊤∈⊣⊣≻∈⊥∈≻∈⊤≻⊥⊤∈≻⊞⊞⊤⊥∈⊢∈⊣⊣∈≻⊞⊥∋⊣⊤≻"),
    ("B", "⊞⋈≻⊡⊢⊞⊙⊥∈≺≺∈⊣≻⊞⋈⊡∈⋈∈∋∈∈⊥⊥≺⊤∋⊤∈⊣⊣≻∈⊥∈≻∈⊤≻⊥⊤∈≻⊞⊞⊤⊥∈⊢∈⊣⊣∈≻⊞⊥∋⊣⊤≻"),
    ("B", "⊥⊞⊞⊡∋⊥⊥⊡⊤⊢∈⊥∈⊥⊞⊥≺⊙⊞⊙⊡⊤⊡⊞⊥∈⊥⊞∈≺⊙⊣⊞⊙⊡⊤⊡⊞∈⊥∈⊥⊞⊥≺⊙≺⊙⊡⊤⊡⊞⊥∈∋⊥⊞⋈⊞⋈⊥⊤⊤⊥∈∈⊡⊥⊡⊤∋⊞∈"),
    ("B", "⊥⊞⊞⊡∋⊥⊥⊡⊤⊢∈⊥∈⊥⊞∈≺⊙⊢⊙⊡⊤⊡⊞∈⊥∈⊥⊞∈≺⊙⊣⊞⊙⊡⊤⊡⊞∈⊥∈⊥⊞⊥≺⊙⊙⊙⊡⊤⊡⊞⊥∈∋⊥⊞⋈⊞⋈⊥⊤⊤⊥∈∈⊡⊥⊡⊤∋⊞∈"),
    ("B", "⊢⊞⊞⊤⊞⊤⊤⊤⋈⋈≺⋈≺⊞⊞⊞⊡∈∈∋⊞⊥⊡⊢⊞⊤⊥∈⊞⊥∈∈∈∈∈∈⊞∈∈∈∈⊥∈⊞∈∈∈∈⊥∈⊞∈∈∈∈⊥∈⊞∈∈∈∈∈∈⊥∈⊞∈∈∈∈⊥∈⊞∈"),
    ("F", "⊢∈⋈⊣⊙⊣≺⊙⊙⊞⊣∋⊢⊡∋∈⊙⊙⊡⊞⋈⊢⊥≺∋≺≻⊙⊤⊡⊥⊤⋈∋⊡≺⊥⊡⊤⊡≺⊤⋈⊞≻⊢≻≻≻∈⊥⊡⊡≻⊡⊡∈⊤⊢∋⊤⊢⊤⊞⊙∋∋⊙⊤⊡⊢⋈∋⊙∈⊞≻⊡≻⊞⊞⊥∋⊡⊞⊞⊞⊞∋⊞⊞⊞⊞≺⊞⊣≺≻≻⊢⊣⊙"),
    ("F", "⊢∋⊤⋈⊞⊞⊤⊡⊡⊥⊤⊡∈⊞⊞⊤∋⊥⊞⊡⊤⊥∋∋⊤⋈⊞∋⊡⊥⊥≺∈⊤∋⊞⊡⊙"),
    ("F", "⊢⊣⊢⊣⊥⋈∈∋⊙≻⊙⊡≺≻⊡⋈⋈≺⊞⊡⊡⊥∋∋⊡∋⊡∋⊞⊤⊡∋≻≻⊤≻≺∋⊡∈≺≻∈"),
    ("F", "∋≺⊢⊣⋈⊡⊞⊣⊣⋈⋈⊡⊙⋈⊥⊣⊣⋈≺⊣⋈⊙⊢∈⊡⊢∋⊡∋⊢⊙⊢∋⊙∈≻⊣∋⊡⋈⊢⊢⊡⊤⊤⊢⊥∋⊞⊣⊣⊞"),
    ("F", "∈⋈⊢⋈⊥⊣⊡∋⊤⊞≺≻≺⊞⊙≺⊣∋∋⊣⊡⊡⊢⊣⊢≺∈⊣⊙⊢⊢⋈⊙≺⊡⊡≻⊢∋∋∋⊢≺⊞⊥⊤⊣⊥∋≺≺⊡≻⊤≺⊤∈⊡⋈⋈"),
    ("F", "⊢≻≻⋈≻∈⊥∈⊙⋈≺⊤∈≺⊡⊡⊞⊞⊞∈⊢⊤⊞∋∋⊞⊙⋈⊞⊤∋⊞⊞⊞⋈≺≺∋∈∋⊢⊤⊞∋≺≺⊡⊡⊙⊡⋈⊥∋⊤≻≻⊞⊥⊣∋≺"),
    ("F", "⊣∋⊢∋∈≻⊤⊙⊣≻⊡≻≻∋≺≻⊞⊢∋≺⊙∈⋈⊣⊞≺∋⊙⊢⊤⊡⊤⊙⊡∈⊡⊡⋈⊢⊞⊙⊞⋈⊤≺⊤∈⊣∋⋈⊡⊤⊡⋈⊢⊙⊡≻⊢⊤⊙⊙⊙"),
    ("F", "⊥⊢⋈≺∋⊞⊤≺∈≺⊣≻⊢⊙∋⊣≺⊡⊢≻⋈∋∋⊣⋈≻⊡⊡⊢≻⊡⋈⊙∈⋈≺≻⊙⊢≺⊞∋≺⊣⊙≺⊡⊞⊞⋈≺⊥∋⊡≺∋⊣⋈⊣∈⊞⊢⊙⊞"),
    ("F", "⊢⊡≺⊞⊞⊤∋⊤∋∈⊙⋈⊥⊡≺⊥⊤⊡⊥≺⊞∋⊤⊥⊡≻⊥⊤⊥∋⊡⊡≺⊢⊥∋≺⊢⊡⋈≻⋈⊤∈∈⊞⋈⊡⊥⊤⊙⊙≺⊡∋⊤⊞⊞⊥⊡⊥⊢∈⊞≻⊥⊥⊞∋∋⊥≺⊤⋈⊤⊡⊞∋⊥⊥⋈≺⊡⊤⊞⊙"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("F", "⊢⊥≻≺⊞⊥⊞⊤∋"),
    ("N", "⊢⊣⋈⊥⊙≺⊣⊣⊞⊥"),
    ("N", "⊢⊣⋈⊥⊙⋈≺⊣⊣⊞⊥"),
    ("N", "⊢⊣⋈⊥⊙⋈≺⊣⊣⊞⊥⊞"),
    ("N", "⊢⋈⊥≻⊙≺⊣⊣⊞≺⊥⊞"),
    ("N", "⊢⊣⋈⊥≻⊙≺⋈⊣⊞⊥⊞"),
    ("N", "⊢⊣⋈⊥⊙⋈≺⊣⊣⊞≺⊥"),
    ("N", "⊢⊣⋈⊥≻⊙⋈≺⊣⊞≺⊥"),
    ("N", "⊢⊣⋈⊥≻⊙⋈≺⊣⊞≺⊥⊞"),
    ("N", "⋈∈∋⊙≻⊙⊡≺≻⊡⋈⋈≺⊞"),
    ("N", "⋈∈∋⊙≻⊙⊡≺≻⊡⋈⋈≺⊞"),
    ("N", "≺⊢⊡⊙⋈⊣⊞"),
    ("N", "≺⊢⊡⊙⋈⊣⊞"),
    ("N", "⊤⋈⊤⊙⊙⊙⊡"),
    ("T", "⊤⊡∋≻≻⊤≻≺∋⊡∈≺≻∈"),
    ("T", "⊤⊡∋≻≻⊤≻≺∋⊡∈≺≻∈"),
    ("T", "≺⋈⊢⊡⊞∋⋈⊞∈⊤⊞∈⊙⊞⊞∋"),
    ("T", "≺⋈⊢⊡⊞∋⋈⊞∈⊤⊞∈⊙⊞⊞∋"),
    ("T", "⊢⊥⊡⊥⋈⊞⋈⊢⋈∈⊣⊞∋∋∈⊞⊞⊡⊞⋈"),
    ("T", "∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥∈≺⊤∋⊣⊞⊥"),
    ("T", "⊤⊡∋≻≻⊤≻≺∋⊡∈≺≻∈⋈∈∋⊙≻⊙⊡≺≻⊡⋈⋈≺⊞"),
    ("T", "⊣⊙⊡⋈≻∋⊣⊣⊥⊣⋈⊡⊡⊢⊡⊤⊢⋈⋈≻∈⊞⊞⊡≺⊞⊞≻≻⋈⊙⋈∋⋈≻⋈⊥∈⊣⊢∋⊡⊢⊞⊞∋⊞⋈∈⊞⊡⊙⊞∈"),
    ("T", "≻⊞⊤⊙⊡≺≺⊥∈⋈⊞⊥⊞≺⊡∈≺⋈⊞⊥⊣⊣⋈⊢⊙∈⋈≻⊥≺⊤⋈⋈≺⊡⊞⊥⊡≺≻⊤≺≻≻⊢⊢⊥⊤⊙≻⊥⊞⋈⊢≺⊤⊤⋈⊣∋⊣⊙∈⋈∋⊤≻⋈⊙⊣⊥⋈⊤∋⊤⊢⊥⊙≺⋈⊙≻⋈≺⊣∋⊥⊙")
];

/// Nine of those structures, with the residue sequence read from the deposited
/// coordinates, for the codon lane.
const SEQS: &[(&str, &str, &str)] = &[
    ("1L2Y", "T", "NLYIQWLKDGGPSSGRPPPS"),
    ("1VII", "T", "MLSDEDFKAVFGMTRSAFANLPLWKQQNLKKEKGLF"),
    ("3I40", "T", "GIVEQCCTSICSLYQLENYCNFVNQHLCGSHLVEALYLVCGERGFFYTPKA"),
    ("1UBQ", "F", "MQIFVKTLTGKTITLEVEPSDTIENVKAKIQDKEGIPPDQQRLIFAGKQLEDGRTLSDYNIQKESTLHLVLRLRGG"),
    ("1EMA", "B", "SKGEELFTGVVPILVELDGDVNGHKFSVSGEGEGDATYGKLTLKFICTTGKLPVPWPTLVTTFVQCFSRYPDHKRHDFFKSAPEGYVQERTIFFKDDGNYKTRAEVKFEGDTLVNRIELKGIDFKEDGNILGHKLEYNYNSHNVYIADKQKNGIKVNFKIRHNIEDGSVQLADHYQQNTPIGDGPVLLPDNHYLSTQSALSKDPNEKRDHVLLEFVTAAGI"),
    ("insulin_b_chain_platonic", "N", "FVNQHLCGSHLVEALYLVCGERGFFYTPKT"),
    ("beta_endorphin_platonic", "T", "YGGFMTSEKSQTPLVTLFKNAIVKNAHKKGQ"),
    ("pbp2a_binder_s179", "T", "ASGWLHRTVVEFCSQWSWDWFEEMASTEIMFRFCNKKVEYRKKLCLCFHFTQFCFDNLLWMRQTEMKALKQKFLLNRSKEAHKAN"),
    ("pbp2a_binder_s590", "F", "GATQYLAMWFEKWWTFAFEHLAFALDWWFRYWRFHLMNSELMSQLAREQSRMHMQRRHNCWSQEFRMMEIIMDQLRKWTRWLRAK")
];

pub fn walk_fold() {
    sprintln!("");
    rule();
    sprintln!("  THE FOLD VERDICT — a sign, and one condition on what a fork encloses");
    rule();
    sprintln!("  Four steps: the procedure against the closed form, the surplus");
    sprintln!("  identity, the witness at the enclosure condition, and the codon lane.");

    let mut steps: Vec<Step> = Vec::new();

    // 1 — the closed form reproduces the procedure, and both reproduce the record.
    let mut agree = 0usize;
    let mut recorded = 0usize;
    for (v, w) in CORPUS {
        let word: Vec<char> = w.chars().collect();
        let p = verdict_procedure(&word);
        let c = verdict_closed(&word);
        if p == c {
            agree += 1;
        }
        if c.to_string() == *v {
            recorded += 1;
        }
    }
    steps.push(Step {
        title: "The closed form is the procedure, on every word of the corpus",
        computed: format!(
            "{} of {} words agree between procedure and closed form; {} of {} match the recorded verdict",
            agree, CORPUS.len(), recorded, CORPUS.len()),
        holds: agree == CORPUS.len() && recorded == CORPUS.len(),
    });
    show(1, 4, &steps[0]);

    // 2 — the surplus identity, and one-sidedness.
    let mut ok = 0usize;
    for (_, w) in CORPUS {
        let word: Vec<char> = w.chars().collect();
        let (uf, ug, _) = pair_cyclic(&word);
        let forks = word.iter().filter(|&&m| m == FORK).count() as i64;
        let fuses = word.iter().filter(|&&m| m == FUSE).count() as i64;
        if (uf as i64) - (ug as i64) == forks - fuses && (uf == 0 || ug == 0) {
            ok += 1;
        }
    }
    steps.push(Step {
        title: "At most one unmatched species, and its size is the signed difference",
        computed: format!("{} of {} words carry the identity with one side empty", ok, CORPUS.len()),
        holds: ok == CORPUS.len(),
    });
    show(2, 4, &steps[1]);

    // 3 — the witness: a non-empty region that is not substantial.
    let w_odot: Vec<char> = "\u{22a2}\u{2208}\u{2299}\u{220b}\u{22a3}".chars().collect();
    let w_clink: Vec<char> = "\u{22a2}\u{2208}\u{22c8}\u{220b}\u{22a3}".chars().collect();
    let w_empty: Vec<char> = "\u{22a2}\u{2208}\u{220b}\u{22a3}".chars().collect();
    let v_odot = verdict_closed(&w_odot);
    let v_clink = verdict_closed(&w_clink);
    let v_empty = verdict_closed(&w_empty);
    let (_, _, r_odot) = pair_cyclic(&w_odot);
    let n_odot = w_odot.len();
    let nonempty = r_odot
        .iter()
        .any(|&r| (r.1 + n_odot - r.0 - 1) % n_odot > 0);
    steps.push(Step {
        title: "The enclosure condition is about work, not about length",
        computed: format!(
            "fork enclosing self-reference: region non-empty {}, verdict {}; enclosing composition: {}; enclosing nothing: {}",
            nonempty, v_odot, v_clink, v_empty),
        holds: nonempty && v_odot == 'N' && v_clink == 'T' && v_empty == 'N',
    });
    show(3, 4, &steps[2]);

    // 4 — the codon lane against the structural lane.
    let mut lane = 0usize;
    for (_, v, seq) in SEQS {
        let word: Vec<char> = seq
            .chars()
            .filter_map(|aa| lift_codon(residue_codon(aa)))
            .collect();
        if verdict_closed(&word).to_string() == *v {
            lane += 1;
        }
    }
    steps.push(Step {
        title: "The codon lane returns the structural lane's verdict",
        computed: format!(
            "{} of {} sequences back-translated, lifted codon by codon, and read out to the recorded verdict",
            lane, SEQS.len()),
        holds: lane == SEQS.len(),
    });
    show(4, 4, &steps[3]);

    finish("THE FOLD VERDICT — closed form, and its codon reading", &steps);
}
