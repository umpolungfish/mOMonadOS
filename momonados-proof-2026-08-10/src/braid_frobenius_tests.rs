//! The braid ↔ IMASM dual, closed.
//!
//! `braid_to_imasm` is δ and `read_tangle` is μ. They were written as a pair and
//! never gated as one. These tests are that gate: μ∘δ = id on the generator
//! word, and the braid relations checked as facts about the compiled programs
//! rather than as imported theorems.
//!
//! What is checked here is the generator word, not the token stream. δ chooses
//! a depth path between crossings and μ does not record which path was taken,
//! so the identity that can hold is on the braid, which is the object, and not
//! on the program, which is one presentation of it. Claiming the stronger
//! identity would be claiming something false.

use crate::braid_protocol::{braid_to_imasm, read_tangle};
use crate::tokens::Token;

/// δ then μ, returning the generator word that came back.
fn round_trip(word: &[i32], strands: usize) -> alloc::vec::Vec<i32> {
    let prog = braid_to_imasm(word, 1, false);
    read_tangle(&prog, strands, 1)
        .expect("a program δ emitted must be readable by μ")
        .generators
}

#[test]
fn frobenius_identity_on_the_generator_word() {
    // μ∘δ = id. The registration gate for the pair.
    for word in [
        &[][..],
        &[1][..],
        &[-1][..],
        &[1, 2, 1][..],
        &[-1, -2, -1][..],
        &[1, 2, 3, 2, 1][..],
        &[1, -2, 3, -1, 2][..],
        &[3, 1, 3, 1][..],
    ] {
        assert_eq!(round_trip(word, 8), word, "μ∘δ ≠ id on {:?}", word);
    }
}

#[test]
fn sign_is_carried_by_the_involution() {
    // σ and σ⁻¹ differ by exactly one gate, and that gate is AREV — the
    // involution that swaps T↔F and t↔f and fixes B and N. Handedness in the
    // braid is chirality in the register; it is not a label bolted on.
    let pos = braid_to_imasm(&[1], 1, false);
    let neg = braid_to_imasm(&[-1], 1, false);
    assert!(pos.contains(&Token::Afwd) && !pos.contains(&Token::Arev));
    assert!(neg.contains(&Token::Arev) && !neg.contains(&Token::Afwd));
    assert_eq!(pos.len(), neg.len(), "the two handednesses cost the same");
}

#[test]
fn writhe_survives_the_round_trip() {
    for word in [&[1, 2, 1][..], &[-1, -2, -1][..], &[1, -1][..], &[1, 1, 1][..]] {
        let direct: i32 = word.iter().map(|g: &i32| g.signum()).sum();
        let prog = braid_to_imasm(word, 1, false);
        let read = read_tangle(&prog, 8, 1).expect("readable");
        assert_eq!(read.writhe, direct, "writhe lost on {:?}", word);
        assert_eq!(read.crossings, word.len(), "crossing count lost on {:?}", word);
    }
}

#[test]
fn yang_baxter_is_a_fact_about_the_programs() {
    // σ₁σ₂σ₁ = σ₂σ₁σ₂. The two sides are distinct programs, so the relation
    // cannot be an identity of token streams. What it must be is an agreement
    // on every invariant μ recovers. Where it holds, the relation is checked
    // rather than assumed; where an invariant separates them, that separation
    // is the finding and is reported rather than smoothed.
    let a = read_tangle(&braid_to_imasm(&[1, 2, 1], 1, false), 4, 1).expect("readable");
    let b = read_tangle(&braid_to_imasm(&[2, 1, 2], 1, false), 4, 1).expect("readable");
    assert_eq!(a.writhe, b.writhe, "Yang-Baxter sides disagree on writhe");
    assert_eq!(a.crossings, b.crossings, "Yang-Baxter sides disagree on crossing count");
}

#[test]
fn far_commutativity_holds_on_the_invariants() {
    // σᵢσⱼ = σⱼσᵢ for |i−j| ≥ 2.
    let a = read_tangle(&braid_to_imasm(&[1, 3], 1, false), 5, 1).expect("readable");
    let b = read_tangle(&braid_to_imasm(&[3, 1], 1, false), 5, 1).expect("readable");
    assert_eq!(a.writhe, b.writhe);
    assert_eq!(a.crossings, b.crossings);
}

#[test]
fn closure_returns_the_depth_it_started_at() {
    // `close: true` is the trace closure. A closed braid must come back to the
    // depth it left from, or it is not closed and the word for it is wrong.
    for word in [&[1, 2, 1][..], &[1, 2, 3, 2, 1][..], &[-1, 2, -3][..]] {
        let prog = braid_to_imasm(word, 1, true);
        let read = read_tangle(&prog, 8, 1).expect("readable");
        assert!(read.closes, "closed braid {:?} did not return to its start depth", word);
    }
}

#[test]
fn an_open_braid_declares_itself_open() {
    // The dual of the above: without `close`, a word that leaves the stack
    // deep must say so. A tool that reported every tangle as closed would be
    // useless in exactly the case closure matters.
    let prog = braid_to_imasm(&[1, 2, 3], 1, false);
    let read = read_tangle(&prog, 8, 1).expect("readable");
    assert!(!read.closes, "an unclosed braid reported itself closed");
}
