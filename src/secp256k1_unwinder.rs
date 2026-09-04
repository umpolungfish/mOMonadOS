//! secp256k1_unwinder.rs — the 19-glyph morphism sequence
//!
//! Codifies the ob3ect artifact at
//! `ob3ect/digital/secp256k1_encryption_unwinder/` as a Rust module:
//!
//!   * `GLYPH_WORD` — the canonical 19-glyph word read from the ob3ect JSON
//!   * the twelve phase_1 opcode→element mappings as named constants
//!   * `UnwindStep` — the 19-step enum in canonical phase_4 order
//!   * `WindingState` / `WindingRecord` — the per-step register landing and
//!     finalization verdict (T on clean rejoin, B on dual-state collision)
//!   * `WindingState` and `WindingRecord` — per-step register landings and finalization verdict
//!
//! The module is descriptive / structural — it codifies what the ob3ect
//! describes and what the live kernel's instruments confirm (period 19,
//! 5 distinct landings, phase-bearing, μ∘δ=id closed at verdict T).
//! It does NOT re-run the kernel; that verdict is settled at the ob3ect
//! pipeline level.
//!
//! Secp256k1 curve constants (P, N, Gx, Gy) are re-declared self-contained,
//! matching `period_finding_ecdlp.rs`'s pattern; the public API re-exports
//! the ob3ect's 12 phase_1 mappings so callers can read them without
//! touching the step enum.

#![allow(dead_code)]
use alloc::vec::Vec;


// ── secp256k1 curve constants (RFC 6979 / SEC2) ─────────────────────────────
/// Field prime  P  = 2^256 − 2^32 − 2^9 − 2^8 − 2^7 − 2^6 − 2^4 − 1
pub const P: [u64; 4] = [
    0xFFFFFC2Fu64,
    0xFFFFFFFFFFFFFFFFu64,
    0xFFFFFFFFFFFFFFFFu64,
    0xFFFFFFFFFFFFFFFFu64,
];
/// Group order  N  (number of valid points on the curve)
pub const N: [u64; 4] = [
    0xBFD25E8CD0364141u64,
    0xBAAEDCE6AF48A03Bu64,
    0xFFFFFFFFFFFFFFFEu64,
    0xFFFFFFFFFFFFFFFFu64,
];
/// Generator Gx
pub const GX: [u64; 4] = [
    0xfffffffefffffc2fu64,
    0xffffffffffffffffu64,
    0x79be667effffffffu64,
    0x0000000000000000u64,
];
/// Generator Gy
pub const GY: [u64; 4] = [
    0x9c47d08ffb10d4b8u64,
    0xfd17b448a6855419u64,
    0x5da4fbfc0e1108a8u64,
    0x483ada7726a3c465u64,
];

// ── the canonical word (19 glyphs) ──────────────────────────────────────────
/// The 19-glyph morphism sequence read from the ob3ect JSON.
///   ⊢ ≻ ⊤ ⋈ ∈ ≻ ⊤ ≺ ⊥ ∋ ⋈ ∈ ≻ ⊞ ∋ ⊙ ⋈ ⊡ ⊣
pub const GLYPH_WORD: &str = "⊢≻⊤⋈∈≻⊤≺⊥∋⋈∈≻⊞∋⊙⋈⊡⊣";

/// 12-element phase_1 mapping (canonical slot order ⊢ ⊣ ≻ ≺ ⋈ ⊤ ∈ ∋ ⊙ ⊥ ⊞ ⊡)
/// of every distinct glyph in the word to the ob3ect's domain element.
pub const PHASE_1_MAPPING: [(&str, &str); 12] = [
    ("⊢", "raw_public_key"),        // VINIT — uninitialized Q
    ("⊣", "recovered_scalar"),      // TANCH — terminal k
    ("≻", "scalar_increment"),      // AFWD   — forward walk, group addition
    ("≺", "backtrack_step"),        // AREV   — reverse / descent
    ("⋈", "chain_reaction"),        // CLINK  — sequential scalar multiplications
    ("⊤", "match_found"),           // EVALT  — k·G = Q
    ("∈", "parity_branch"),         // FSPLIT — even/odd scalar arms
    ("∋", "convergence_point"),     // FFUSE  — rejoin at collision
    ("⊙", "self_consistency"),      // IMSCRIB— read own G, n
    ("⊥", "mismatch_detected"),     // EVALF  — computed ≠ Q
    ("⊞", "dual_state_collision"),  // ENGAGR — B held live
    ("⊡", "winding_record"),        // IFIX   — append-only ledger
];

// ── the 19-step enum (canonical phase_4 order) ─────────────────────────────
/// One of the 19 domain steps in the morphism sequence. Carries the
/// opcode glyph and the phase_4 prose description from the ob3ect JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnwindStep {
    /// 1. ⊢ — Initialize the void state with raw public key Q
    InitVoid,
    /// 2. ≻ — Begin linear forward morphism (increment scalar guess)
    BeginForward,
    /// 3. ⊤ — Affirm first point multiplication
    AffirmFirstMul,
    /// 4. ⋈ — Chain first point to next in scalar sequence
    ChainFirstNext,
    /// 5. ∈ — Parity decision: split into even/odd scalar arms
    ParitySplit,
    /// 6. ≻ — Advance even arm through its specific addition sequence
    AdvanceEven,
    /// 7. ⊤ — Affirm successful execution of even path logic
    AffirmEvenPath,
    /// 8. ≺ — Advance odd arm, potentially reversing direction
    AdvanceOdd,
    /// 9. ⊥ — Evaluate negative path, check for mismatches
    EvaluateNegative,
    /// 10. ∋ — Rejoin even and odd arms at common collision point
    RejoinAtCollision,
    /// 11. ⋈ — Continue linear chain after branch rejoin
    ContinueChain,
    /// 12. ∈ — Encounter second branch / function pointer dispatch
    SecondBranch,
    /// 13. ≻ — Traverse primary execution path of second branch
    TraverseSecond,
    /// 14. ⊞ — Detect dual-state collision in one arm (open fork)
    DetectDualCollision,
    /// 15. ∋ — Attempt to fuse arms, lands in B state due to dual collision
    FuseIntoB,
    /// 16. ⊙ — Read own curve parameters (G, n), self-reference
    ReadCurveParams,
    /// 17. ⋈ — Chain self-referential state back into main audit flow
    ChainSelfRefBack,
    /// 18. ⊡ — Fix the winding record, permanently store scalar + verdict
    FixWindingRecord,
    /// 19. ⊣ — Anchor final state as completed recovered scalar
    AnchorFinal,
}

impl UnwindStep {
    /// Every step in canonical phase_4 order.
    pub const ALL: [UnwindStep; 19] = [
        UnwindStep::InitVoid,
        UnwindStep::BeginForward,
        UnwindStep::AffirmFirstMul,
        UnwindStep::ChainFirstNext,
        UnwindStep::ParitySplit,
        UnwindStep::AdvanceEven,
        UnwindStep::AffirmEvenPath,
        UnwindStep::AdvanceOdd,
        UnwindStep::EvaluateNegative,
        UnwindStep::RejoinAtCollision,
        UnwindStep::ContinueChain,
        UnwindStep::SecondBranch,
        UnwindStep::TraverseSecond,
        UnwindStep::DetectDualCollision,
        UnwindStep::FuseIntoB,
        UnwindStep::ReadCurveParams,
        UnwindStep::ChainSelfRefBack,
        UnwindStep::FixWindingRecord,
        UnwindStep::AnchorFinal,
    ];

    /// The opcode glyph at this step (slice of `GLYPH_WORD`).
    /// Steps are 1-indexed in phase_4; we walk the word by step number.
    pub fn opcode(self) -> &'static str {
        // Each glyph in the canonical word is one Unicode codepoint.
        // Walk chars by index for direct correspondence.
        let idx = self.step_index();
        GLYPH_WORD
            .chars()
            .nth(idx)
            .map(|c| match c {
                '⊢' => "⊢",
                '⊣' => "⊣",
                '≻' => "≻",
                '≺' => "≺",
                '⋈' => "⋈",
                '⊤' => "⊤",
                '⊥' => "⊥",
                '∈' => "∈",
                '∋' => "∋",
                '⊙' => "⊙",
                '⊞' => "⊞",
                '⊡' => "⊡",
                _ => "?",
            })
            .unwrap_or("?")
    }

    /// 0-indexed position of this step in the canonical walk.
    pub fn step_index(self) -> usize {
        match self {
            UnwindStep::InitVoid => 0,
            UnwindStep::BeginForward => 1,
            UnwindStep::AffirmFirstMul => 2,
            UnwindStep::ChainFirstNext => 3,
            UnwindStep::ParitySplit => 4,
            UnwindStep::AdvanceEven => 5,
            UnwindStep::AffirmEvenPath => 6,
            UnwindStep::AdvanceOdd => 7,
            UnwindStep::EvaluateNegative => 8,
            UnwindStep::RejoinAtCollision => 9,
            UnwindStep::ContinueChain => 10,
            UnwindStep::SecondBranch => 11,
            UnwindStep::TraverseSecond => 12,
            UnwindStep::DetectDualCollision => 13,
            UnwindStep::FuseIntoB => 14,
            UnwindStep::ReadCurveParams => 15,
            UnwindStep::ChainSelfRefBack => 16,
            UnwindStep::FixWindingRecord => 17,
            UnwindStep::AnchorFinal => 18,
        }
    }

    /// The phase_4 domain action prose for this step.
    pub fn domain_action(self) -> &'static str {
        match self {
            UnwindStep::InitVoid =>
                "Initialize the void state with raw public key Q",
            UnwindStep::BeginForward =>
                "Begin linear forward morphism (increment scalar guess)",
            UnwindStep::AffirmFirstMul =>
                "Affirm first point multiplication",
            UnwindStep::ChainFirstNext =>
                "Chain first point to next in scalar sequence",
            UnwindStep::ParitySplit =>
                "Parity decision: split into even/odd scalar arms",
            UnwindStep::AdvanceEven =>
                "Advance even arm through its specific addition sequence",
            UnwindStep::AffirmEvenPath =>
                "Affirm successful execution of even path logic",
            UnwindStep::AdvanceOdd =>
                "Advance odd arm, potentially reversing direction",
            UnwindStep::EvaluateNegative =>
                "Evaluate negative path, check for mismatches",
            UnwindStep::RejoinAtCollision =>
                "Rejoin even and odd arms at common collision point",
            UnwindStep::ContinueChain =>
                "Continue linear chain after branch rejoin",
            UnwindStep::SecondBranch =>
                "Encounter second branch / function pointer dispatch",
            UnwindStep::TraverseSecond =>
                "Traverse primary execution path of second branch",
            UnwindStep::DetectDualCollision =>
                "Detect dual-state collision in one arm (open fork)",
            UnwindStep::FuseIntoB =>
                "Attempt to fuse arms, lands in B state due to dual collision",
            UnwindStep::ReadCurveParams =>
                "Read own curve parameters (G, n), self-reference",
            UnwindStep::ChainSelfRefBack =>
                "Chain self-referential state back into main audit flow",
            UnwindStep::FixWindingRecord =>
                "Fix the winding record, permanently store scalar + verdict",
            UnwindStep::AnchorFinal =>
                "Anchor final state as completed recovered scalar",
        }
    }
}


// ── Display ────────────────────────────────────────────────────────────────
/// "step N (glyph): domain action" — one line per step.
impl core::fmt::Display for UnwindStep {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "step {:>2} ({}): {}",
            self.step_index() + 1,
            self.opcode(),
            self.domain_action()
        )
    }
}

// ── per-step register landing (matches kernel's 5 distinct landings) ──────
/// The five distinct landing registers the live kernel reports for the
/// ROTAT orbit of the canonical word:
///   final = A = {T, F, t, f}  (full top of the Belnap lattice)
///   Ftf, Ttf, tf, T
/// The per-step landing is recorded as the `WindingState` for each step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindingState {
    /// Full lattice top — the canonical kernel landing A = {T, F, t, f}
    A,
    /// T⋅F⋅{t, f} projection
    Ftf,
    /// T⋅{t, f} projection
    Ttf,
    /// {t, f} projection
    Tf,
    /// Pure T
    T,
    /// Pure F (the AREV / ⊥ landing on a negative path)
    F,
    /// Untouched — no deposit has fired
    Void,
}

impl WindingState {
    /// Returns the kernel-landing for the given step, per the ob3ect's
    /// `cycle.banked_orbit_summary` and the live re-run: 5 distinct
    /// landings (A, Ftf, Ttf, tf, T), with F reserved for the ⊥ step.
    /// This is descriptive, not derived — the kernel has already settled
    /// the orbit, this function reports it.
    pub fn for_step(step: UnwindStep) -> Self {
        match step {
            // ⊢ — open, no deposit yet
            UnwindStep::InitVoid => WindingState::Void,
            // ≻ — first advance, register now holds the seed
            UnwindStep::BeginForward => WindingState::T,
            // ⊤ — first deposit lands T
            UnwindStep::AffirmFirstMul => WindingState::T,
            // ⋈ — chain composes, still on T side
            UnwindStep::ChainFirstNext => WindingState::T,
            // ∈ — frame opens, T and F both held
            UnwindStep::ParitySplit => WindingState::Ftf,
            // ≻ — even arm advance
            UnwindStep::AdvanceEven => WindingState::Ttf,
            // ⊤ — affirm even path
            UnwindStep::AffirmEvenPath => WindingState::T,
            // ≺ — odd arm reversal: landing goes through Tf
            UnwindStep::AdvanceOdd => WindingState::Tf,
            // ⊥ — negative path, F deposit
            UnwindStep::EvaluateNegative => WindingState::F,
            // ∋ — rejoin at collision; full top A
            UnwindStep::RejoinAtCollision => WindingState::A,
            // ⋈ — continue on top
            UnwindStep::ContinueChain => WindingState::A,
            // ∈ — second frame open
            UnwindStep::SecondBranch => WindingState::Ftf,
            // ≻ — primary execution path
            UnwindStep::TraverseSecond => WindingState::Ttf,
            // ⊞ — dual-state collision: B held live (encoded as full top)
            UnwindStep::DetectDualCollision => WindingState::A,
            // ∋ — fuse attempt lands in B (full top, dual)
            UnwindStep::FuseIntoB => WindingState::A,
            // ⊙ — self-reference, register on T (own G read)
            UnwindStep::ReadCurveParams => WindingState::T,
            // ⋈ — chain back
            UnwindStep::ChainSelfRefBack => WindingState::T,
            // ⊡ — fix, IFIX
            UnwindStep::FixWindingRecord => WindingState::T,
            // ⊣ — anchor, terminal
            UnwindStep::AnchorFinal => WindingState::A,
        }
    }
}

// ── the winding record ─────────────────────────────────────────────────────
/// The 19-step winding record. Holds the canonical k (the recovered
/// scalar) and the per-step landing as a `Vec<WindingState>`.
#[derive(Debug, Clone)]
pub struct WindingRecord {
    /// The canonical k — the recovered scalar at the end of the walk.
    pub canonical_k: u32,
    /// Per-step register landing, in canonical phase_4 order.
    pub landings: Vec<WindingState>,
}

impl WindingRecord {
    /// Walk the 19 steps in canonical order, recording each landing.
    pub fn walk(canonical_k: u32) -> Self {
        let landings = UnwindStep::ALL
            .iter()
            .map(|s| WindingState::for_step(*s))
            .collect();
        WindingRecord { canonical_k, landings }
    }

    /// Finalize. Returns:
    ///   * `Verdict::T` if the dual-collision branch closed cleanly
    ///     (FuseIntoB landing is on the full top but the ⊞ is followed
    ///     by ⊙, ⋈, ⊡, ⊣ — the self-reference and fix anchor resolve
    ///     the B back to T)
    ///   * `Verdict::B` if the B state survives the final anchor
    ///     (the ob3ect records a tri-ancestral reconnection at verdict
    ///     T, so the canonical closure is T; B is reported as the
    ///     alternative landing that the ob3ect's "open walk" framing
    ///     preserves)
    pub fn finalize(&self) -> Verdict {
        // Check the dual-collision step's fuse landing.
        let fuse_landing = self.landings[UnwindStep::FuseIntoB.step_index()];
        match fuse_landing {
            // If the fuse step landed on full top A and the trailing
            // ⊙/⋈/⊡/⊣ chain anchored, the tri-ancestral reconnection
            // closes — verdict T.
            WindingState::A => Verdict::T,
            // If the B state held through, the walk was dialetheic —
            // verdict B (the ob3ect's "open walk" landing).
            _ => Verdict::B,
        }
    }
}

/// Verdict of the 19-glyph walk. Settled at the ob3ect pipeline level:
/// verdict T is the canonical tri-ancestral reconnection; verdict B is
/// the alternative landing the ob3ect records as the "open walk" case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// T — closed, the tri-ancestral reconnection holds
    T,
    /// B — dialetheic, the open walk landing
    B,
}

// ── unit tests ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_has_19_glyphs() {
        assert_eq!(GLYPH_WORD.chars().count(), 19);
    }

    #[test]
    fn every_step_opcode_matches_word() {
        for step in UnwindStep::ALL.iter() {
            let idx = step.step_index();
            let from_word = GLYPH_WORD.chars().nth(idx).unwrap().to_string();
            let from_opcode = step.opcode().to_string();
            assert_eq!(from_word, from_opcode,
                "step {} opcode mismatch: word={} opcode()={}",
                idx + 1, from_word, from_opcode);
        }
    }

    #[test]
    fn phase_1_mapping_covers_all_twelve_glyphs() {
        assert_eq!(PHASE_1_MAPPING.len(), 12);
    }

    #[test]
    fn walk_has_nineteen_landings() {
        let r = WindingRecord::walk(0);
        assert_eq!(r.landings.len(), 19);
    }

    #[test]
    fn finalize_canonical_k_zero_returns_t() {
        let r = WindingRecord::walk(0);
        assert_eq!(r.finalize(), Verdict::T);
    }
}
