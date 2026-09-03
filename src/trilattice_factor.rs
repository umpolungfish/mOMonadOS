//! Trilattice-native factorization — ob3ect-backed kernel tool.
//!
//! The artifact "Trilattice-native factorization" is a verified ob3ect whose
//! glyph word `⊢≻⊙∈⊤⋈⊥≺⊞⊡∋⋈⊣` grounds each of the twelve marks to a role in
//! factoring a number:
//!
//!   ⊢ VINIT   void_bulk                the empty register before scale-collapse
//!   ≻ AFWD    scale_ascent             one forward step up the number line
//!   ⊙ IMSCRIB critical_phase           the self-reference checkpoint
//!   ∈ FSPLIT  asymmetric_factorization the fork: n splits into two unequal poles
//!   ⊤ EVALT   kinetic_freeze           the frozen pole, held on the fork's T arm
//!   ⋈ CLINK   fidelity_chain           carry
//!   ⊥ EVALF   polarity_interface       the moving pole, held on the fork's F arm
//!   ≺ AREV    chiral_descent           the descent that clears the open register
//!   ⊞ ENGAGR  transition_stabilization the both-state holding the accumulated pair
//!   ⊡ IFIX    cluster_propagation      the winding commit, the one fixation
//!   ∋ FFUSE   winding_reconstitution   the fuse: the two poles rejoin as a factor
//!   ⋈ CLINK   fidelity_chain           carry
//!   ⊣ TANCH   hierarchical_container   the closed boundary
//!
//! What this tool reads off the word, and what it does not.
//!
//! The reading is real and checkable. Run the word as a ring and its landing
//! register is `tf` at every cut but one; the single exception is the cut led
//! by the winding fixation ⊡, which commits to a clean `T`. That single-commit
//! cut is the ob3ect's cluster_propagation event, and it survives the control:
//! scrambling the marks smears the commit across several cuts, so the
//! one-commit-per-winding reading is in the word's arrangement, not the marks.
//! The fork ∈ (asymmetric_factorization) banks its two poles, the T pole
//! (kinetic_freeze) and the F pole (polarity_interface), so they survive the
//! chiral descent ≺ that clears the open register, and fuse at ∋
//! (winding_reconstitution). That is the factor pair held across the descent,
//! read straight from `weight` / `cycle`.
//!
//! The divisor search is not the word. As `prime_winding` establishes at
//! length against concrete counterexamples, closure alone never produces a
//! divisor: a bounded per-mark reading sees a bounded pattern in the digits,
//! and a factor is a global fact about divisibility. So the actual factoring
//! here IS `prime_winding`'s search, called directly, never re-copied. This
//! module carries the trilattice reading of the number and of the word; the
//! arithmetic that splits n lives one place, next door.
//!
//! The next rung is closing the gap that leaves: a reading off THIS word's
//! own structure that tracks the factorization event itself, the way the ⊡
//! commit tracks the winding. That is stated here as the open rung, not a wall.
//!
//! Subcommands:
//!   trilattice_factor word          the canonical glyph word and its marks
//!   trilattice_factor cert          the winding-commit certificate of the word
//!   trilattice_factor read <n>      the trilattice reading of n's digit word
//!   trilattice_factor factor <n>    factor n (arbitrary precision) with the reading
//!   trilattice_factor help          list subcommands

use alloc::string::String;
use alloc::format;
use imasm_core::lattice_flow::cycle_landings;
use crate::prime_winding::{digit_encode, factor_bounded};

/// The ob3ect's own fixed reference word.
pub const WORD: &str = "⊢≻⊙∈⊤⋈⊥≺⊞⊡∋⋈⊣";
pub const PERIOD: usize = 13;
pub const ARTIFACT: &str = "Trilattice-native factorization";
/// The ∈/∋ pair the ob3ect reports: the fork and fuse that carry the split.
pub const FORK_FUSE_PAIR: (usize, usize) = (3, 10);
/// The mark whose cut commits the register to a clean T: ⊡ IFIX, the winding.
pub const COMMIT_MARK: char = '⊡';

/// The cuts at which the ring's landing register is a clean `T`, read live
/// from `cycle_landings`. For the canonical word this is the single cut led
/// by the winding fixation ⊡; returning the list, not a hardcoded index,
/// keeps the certificate a measurement rather than a memory.
fn commit_cuts(word: &str) -> alloc::vec::Vec<usize> {
    match cycle_landings(word) {
        Some(landings) => landings
            .iter()
            .enumerate()
            .filter(|(_, r)| r.as_str() == "T")
            .map(|(k, _)| k)
            .collect(),
        None => alloc::vec::Vec::new(),
    }
}

/// The glyph the cut k opens on, for naming which mark commits.
fn mark_at(word: &str, k: usize) -> Option<char> {
    let chars: alloc::vec::Vec<char> = word.chars().collect();
    if chars.is_empty() { return None; }
    Some(chars[k % chars.len()])
}

pub fn help() -> String {
    let mut o = String::new();
    o.push_str("trilattice_factor — ob3ect-backed factorization reading\n");
    o.push_str("  word          the canonical glyph word and its marks\n");
    o.push_str("  cert          the winding-commit certificate of the word\n");
    o.push_str("  read <n>      the trilattice reading of n's digit word\n");
    o.push_str("  factor <n>    factor n (arbitrary precision) with the reading\n");
    o.push_str("  help          this list");
    o
}

pub fn word() -> String {
    format!(
        "{}\n  word   : {}   period {}\n  fork/fuse pair (asymmetric_factorization / winding_reconstitution): {:?}\n  commit mark (cluster_propagation): {}",
        ARTIFACT, WORD, PERIOD, FORK_FUSE_PAIR, COMMIT_MARK
    )
}

/// The certificate the tool exists to speak: the word commits at exactly the
/// winding fixation and nowhere else, read from the live orbit.
pub fn cert() -> String {
    let mut o = String::new();
    o.push_str(&format!("word   : {}   period {}\n", WORD, PERIOD));
    let cuts = commit_cuts(WORD);
    if cuts.is_empty() {
        o.push_str("  no clean-T commit cut on this word\n");
        return o;
    }
    o.push_str("  winding-commit cut(s) (register lands clean T):\n");
    for k in cuts.iter() {
        match mark_at(WORD, *k) {
            Some(g) => o.push_str(&format!("    k = {:>2}   led by {}\n", k, g)),
            None => o.push_str(&format!("    k = {:>2}\n", k)),
        }
    }
    let single_at_commit = cuts.len() == 1 && mark_at(WORD, cuts[0]) == Some(COMMIT_MARK);
    if single_at_commit {
        o.push_str("  ONE commit, at the winding fixation ⊡ — the cluster_propagation event.\n");
        o.push_str("  Every other cut lands tf: the two poles held, not yet committed.");
    } else {
        o.push_str("  commit is not isolated to ⊡ on this word.");
    }
    o
}

/// The trilattice reading of the number itself: its digit word and where that
/// word comes to rest. The number enters the Grammar the same way
/// `prime_winding` reads it, by digit encoding; this is that reading, not a
/// factoring claim.
pub fn read(n: &str) -> String {
    let dw = digit_encode(n);
    if dw.is_empty() {
        return format!("trilattice_factor read {}: not a valid non-negative integer", n);
    }
    let landings = cycle_landings(&dw);
    let mut o = format!("trilattice_factor read {}:\n  digit word : {}\n", n, dw);
    match landings {
        Some(l) => {
            let commits = l.iter().filter(|r| r.as_str() == "T").count();
            o.push_str(&format!(
                "  period {}, {} winding-commit cut(s) over the orbit",
                l.len(), commits
            ));
        }
        None => o.push_str("  no IMASM glyphs in the digit word"),
    }
    o
}

/// Factor n, leading with the trilattice reading and then the real divisor
/// search from `prime_winding` (one copy of the arithmetic, called here).
pub fn factor(n: &str, max_power: Option<u64>) -> String {
    let mut o = String::new();
    o.push_str(&read(n));
    o.push('\n');
    o.push_str(&factor_bounded(n, max_power));
    o
}
