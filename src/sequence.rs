#![allow(dead_code)]
// sequence.rs — Dynamic IMASM sequence builder
//
// Instead of hand-authored preset programs, the kernel derives its next
// execution sequence from its current IgTuple: the 12 primitive values
// vote on token preferences, composition constraints enforce arity balance
// and FSPLIT/FFUSE matching, and the resulting Program is built fresh each
// time the kernel wraps around.
//
// This mirrors the ob3ect pipeline shift: opcodes + composition rules +
// edge semantics, not fixed scripts.

use crate::imas_ig::{IgTuple, IgPrim};
use crate::tokens::{Token, Program};

// ─── Vote table ──────────────────────────────────────────────────

/// Score array indexed by Token discriminant (0-11 = VINIT..IFIX).
type Scores = [i32; 12];

fn add(scores: &mut Scores, tok: Token, weight: i32) {
    scores[tok as usize] += weight;
}

/// Each IG primitive casts weighted votes for preferred tokens.
fn aggregate_votes(tuple: &IgTuple) -> Scores {
    let mut s: Scores = [0; 12];

    // Ð Dimensionality
    match tuple.d {
        IgPrim::D_wedge    => add(&mut s, Token::VINIT,   2),
        IgPrim::D_triangle => { add(&mut s, Token::AFWD, 1); add(&mut s, Token::AREV, 1); }
        IgPrim::D_infty    => add(&mut s, Token::IMSCRIB, 2),
        IgPrim::D_odot     => add(&mut s, Token::IFIX,    2),
        _ => {}
    }

    // Þ Topology
    match tuple.t {
        IgPrim::T_net      => add(&mut s, Token::FSPLIT,  2),
        IgPrim::T_in       => add(&mut s, Token::CLINK,   2),
        IgPrim::T_bowtie   => add(&mut s, Token::FFUSE,   2),
        IgPrim::T_boxtimes => add(&mut s, Token::ENGAGR,  2),
        IgPrim::T_odot     => add(&mut s, Token::IMSCRIB, 2),
        _ => {}
    }

    // Φ Parity
    match tuple.p {
        IgPrim::P_pmsym => { add(&mut s, Token::FSPLIT, 2); add(&mut s, Token::FFUSE, 2); }
        IgPrim::P_sym   => add(&mut s, Token::FFUSE,   2),
        IgPrim::P_pm    => { add(&mut s, Token::FSPLIT, 1); add(&mut s, Token::FFUSE, 1); }
        IgPrim::P_psi   => { add(&mut s, Token::EVALT,  2); add(&mut s, Token::EVALF, 2); }
        IgPrim::P_asym  => add(&mut s, Token::EVALT,   1),
        _ => {}
    }

    // ⊙ Criticality
    match tuple.phi {
        IgPrim::Phi_c         => { add(&mut s, Token::ENGAGR, 2); add(&mut s, Token::EVALT, 1); add(&mut s, Token::EVALF, 1); }
        IgPrim::Phi_sub       => add(&mut s, Token::EVALT,  2),
        IgPrim::Phi_super     => add(&mut s, Token::EVALF,  2),
        IgPrim::Phi_ep        => add(&mut s, Token::ENGAGR, 2),
        IgPrim::Phi_c_complex => { add(&mut s, Token::EVALT, 2); add(&mut s, Token::EVALF, 2); }
        _ => {}
    }

    // ƒ Fidelity
    match tuple.f {
        IgPrim::F_hbar => { add(&mut s, Token::EVALT, 1); add(&mut s, Token::EVALF, 1); }
        IgPrim::F_ell  => add(&mut s, Token::AFWD, 1),
        IgPrim::F_eth  => add(&mut s, Token::AREV, 1),
        _ => {}
    }

    // Ç Kinetics
    match tuple.k {
        IgPrim::K_fast => add(&mut s, Token::AFWD,   2),
        IgPrim::K_slow => { add(&mut s, Token::AREV, 1); add(&mut s, Token::CLINK, 1); }
        IgPrim::K_mod  => add(&mut s, Token::CLINK,  2),
        IgPrim::K_trap => add(&mut s, Token::IFIX,   2),
        IgPrim::K_mbl  => add(&mut s, Token::TANCH,  2),
        _ => {}
    }

    // Γ Granularity
    match tuple.g {
        IgPrim::G_aleph => add(&mut s, Token::IMSCRIB, 2),
        IgPrim::G_gimel => add(&mut s, Token::FSPLIT,  1),
        IgPrim::G_beth  => add(&mut s, Token::CLINK,   1),
        _ => {}
    }

    // ɢ Coupling
    match tuple.c {
        IgPrim::C_seq   => { add(&mut s, Token::AFWD, 1); add(&mut s, Token::CLINK, 1); }
        IgPrim::C_and   => add(&mut s, Token::FFUSE,  2),
        IgPrim::C_or    => add(&mut s, Token::FSPLIT, 2),
        IgPrim::C_broad => add(&mut s, Token::ENGAGR, 2),
        _ => {}
    }

    // Ħ Chirality
    match tuple.h {
        IgPrim::H0    => add(&mut s, Token::AFWD,    2),
        IgPrim::H1    => add(&mut s, Token::CLINK,   2),
        IgPrim::H2    => add(&mut s, Token::IMSCRIB, 2),
        IgPrim::H_inf => add(&mut s, Token::IFIX,    2),
        _ => {}
    }

    // Σ Stoichiometry
    match tuple.s {
        IgPrim::S_11 => add(&mut s, Token::AFWD,    1),
        IgPrim::S_nn => add(&mut s, Token::IMSCRIB, 1),
        IgPrim::S_nm => { add(&mut s, Token::FSPLIT, 1); add(&mut s, Token::FFUSE, 1); }
        _ => {}
    }

    // Ω Winding
    match tuple.omega {
        IgPrim::Omega_0  => { add(&mut s, Token::VINIT, 1); add(&mut s, Token::AFWD, 1); }
        IgPrim::Omega_z2 => { add(&mut s, Token::EVALT, 1); add(&mut s, Token::EVALF, 1); }
        IgPrim::Omega_z  => add(&mut s, Token::IMSCRIB, 2),
        IgPrim::Omega_na => add(&mut s, Token::ENGAGR,  2),
        _ => {}
    }

    // Ř Recognition/Coupling
    match tuple.r {
        IgPrim::R_lr     => add(&mut s, Token::FFUSE,   1),
        IgPrim::R_dagger => { add(&mut s, Token::AFWD, 1); add(&mut s, Token::AREV, 1); }
        IgPrim::R_cat    => add(&mut s, Token::CLINK,   1),
        IgPrim::R_super  => add(&mut s, Token::IMSCRIB, 1),
        _ => {}
    }

    s
}

/// Tokens in descending vote order (no_std insertion sort).
fn sorted_by_score(scores: &Scores) -> [Token; 12] {
    const ALL: [Token; 12] = [
        Token::VINIT,  Token::TANCH,  Token::AFWD,  Token::AREV,
        Token::CLINK,  Token::IMSCRIB, Token::FSPLIT, Token::FFUSE,
        Token::EVALT,  Token::EVALF,  Token::ENGAGR, Token::IFIX,
    ];
    let mut order: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut i = 1;
    while i < 12 {
        let mut j = i;
        while j > 0 && scores[order[j - 1]] < scores[order[j]] {
            let tmp = order[j - 1]; order[j - 1] = order[j]; order[j] = tmp;
            j -= 1;
        }
        i += 1;
    }
    let mut result = [Token::AFWD; 12];
    let mut k = 0;
    while k < 12 { result[k] = ALL[order[k]]; k += 1; }
    result
}

// ─── Stack-depth accounting ───────────────────────────────────────

/// Net change to estimated stack depth from emitting this token.
/// Based on actual kernel.rs behavior, not the arity_in/out table.
fn stack_delta(tok: Token) -> i32 {
    match tok {
        Token::VINIT  | Token::ENGAGR => 1,   // push without pop
        Token::FSPLIT                  => 1,   // copies top (push without pop)
        Token::TANCH  | Token::IFIX   => -1,   // pop without push
        Token::FFUSE                   => -1,   // pop left, push join (net -1)
        _                              =>  0,   // pop + push cancel
    }
}

// ─── Sequence builder ─────────────────────────────────────────────

/// Build a dynamic program of `len` tokens driven by the current IgTuple.
///
/// Rules (in priority order):
///   1. If self_ref: IMSCRIB at position 0, IMSCRIB at last position.
///   2. Before the last `open_forks` slots: emit FFUSE to close all open forks.
///   3. Never emit a depth-reducing token (TANCH, IFIX, FFUSE) when est_depth <= 0.
///   4. Never emit FFUSE when open_forks == 0.
///   5. Limit new FSPLIT ops so remaining capacity can close them all.
///   6. Otherwise: pick highest-voted valid token.
pub fn build_next_program(tuple: &IgTuple, len: usize, self_ref: bool) -> Program {
    let len = len.max(4).min(62); // safe range
    let scores = aggregate_votes(tuple);
    let preferred = sorted_by_score(&scores);

    let mut p = Program::empty();
    let mut est_depth: i32 = 1; // assume at least 1 value carried from prior cycle
    let mut open_forks: u32 = 0;

    let mut i = 0;
    while i < len {
        let remaining = len - i;
        let is_first = i == 0;
        let is_last  = remaining == 1;

        // Rule 1a: self-ref opening
        if is_first && self_ref {
            p.push(Token::IMSCRIB);
            i += 1;
            continue;
        }

        // Rule 1b: self-ref close (only if forks are all closed)
        if is_last && self_ref && open_forks == 0 {
            p.push(Token::IMSCRIB);
            i += 1;
            continue;
        }

        // Rule 2: must close open forks before we run out of space
        if open_forks > 0 && remaining <= open_forks as usize {
            p.push(Token::FFUSE);
            open_forks -= 1;
            est_depth -= 1;
            i += 1;
            continue;
        }

        // Rule 3-6: pick from preferred list
        let mut chosen = Token::AFWD; // safe fallback (no stack change)
        let mut found = false;
        let mut pi = 0;
        while pi < 12 {
            let tok = preferred[pi];
            let depth_after = est_depth + stack_delta(tok);

            // Never go below depth 0
            if depth_after < 0 {
                pi += 1;
                continue;
            }
            // FFUSE requires an open fork
            if tok == Token::FFUSE && open_forks == 0 {
                pi += 1;
                continue;
            }
            // FSPLIT: only if remaining capacity can accommodate closing it
            // (need at least 1 more slot for the matching FFUSE, plus open_forks already pending)
            if tok == Token::FSPLIT && (remaining as u32) <= open_forks + 2 {
                pi += 1;
                continue;
            }
            // TANCH: halts the kernel — only allow if this is intentional (last slot, no self_ref)
            if tok == Token::TANCH && (!is_last || self_ref || open_forks > 0) {
                pi += 1;
                continue;
            }

            chosen = tok;
            found = true;
            break;
        }

        if !found {
            // Absolute fallback: IMSCRIB never changes depth, always safe
            chosen = Token::IMSCRIB;
        }

        p.push(chosen);
        if chosen == Token::FSPLIT { open_forks += 1; }
        if chosen == Token::FFUSE  { open_forks -= 1; }
        est_depth += stack_delta(chosen);
        i += 1;
    }

    // Safety: close any forks we couldn't close in the main loop
    while open_forks > 0 && p.len() < 64 {
        p.push(Token::FFUSE);
        open_forks -= 1;
    }

    p
}

/// Recommended sequence length for the next program, based on tier and Frobenius order.
pub fn next_seq_len(snap: &crate::kernel::Snapshot) -> usize {
    match snap.tier {
        3 => 12, // O_∞ → longer sequences
        2 => 10,
        1 => 8,
        _ => 6,  // O_0 → shorter
    }
}

// ─── Debug display ────────────────────────────────────────────────

/// One-line summary of the vote table (top 4 tokens with scores > 0).
pub fn vote_summary(tuple: &IgTuple) -> VoteSummary {
    VoteSummary { scores: aggregate_votes(tuple) }
}

pub struct VoteSummary { scores: Scores }

impl core::fmt::Display for VoteSummary {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const NAMES: [&str; 12] = [
            "VINIT","TANCH","AFWD","AREV","CLINK","IMSCRIB",
            "FSPLIT","FFUSE","EVALT","EVALF","ENGAGR","IFIX",
        ];
        let s = &self.scores;
        let mut order = [0usize; 12];
        let mut i = 0;
        while i < 12 { order[i] = i; i += 1; }
        let mut i = 1;
        while i < 12 {
            let mut j = i;
            while j > 0 && s[order[j-1]] < s[order[j]] {
                let tmp = order[j-1]; order[j-1] = order[j]; order[j] = tmp;
                j -= 1;
            }
            i += 1;
        }
        write!(f, "votes:")?;
        let mut shown = 0;
        let mut i = 0;
        while i < 12 && shown < 5 {
            if s[order[i]] > 0 {
                write!(f, " {}={}", NAMES[order[i]], s[order[i]])?;
                shown += 1;
            }
            i += 1;
        }
        Ok(())
    }
}
