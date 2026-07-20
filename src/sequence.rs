#![allow(dead_code)]
// sequence.rs — Dynamic IMASM sequence builder
//
// Fix 2 (2026-06-22): aggregate_votes() replaced with FAMILY_TOKEN_AFFINITY
// const matrix. Each primitive family has a row of base affinities to tokens;
// the specific variant's ordinal scales the row, making vote weights derived
// from the live tuple state rather than compiled-in per-variant constants.
//
// Fix 3 (2026-06-22): build_via_substrate() runs a canonical IMASM program
// on a MiniKernel seeded from the IgTuple, reads the post-execution register
// state, maps it through TOKEN_REG_AFFINITY, and combines those
// substrate-derived scores with the family affinity scores. The sequence
// builder is itself an IMASM execution on the kernel's own substrate.

use crate::imas_ig::{IgTuple, IgPrim};
use crate::tokens::{Token, Program};
use crate::belnap::{B4, b4_join, b4_meet};

// f32::round() is unavailable in no_std without libm
#[inline(always)]
fn ord_round(x: f32) -> i32 {
    if x >= 0.0 { (x + 0.5) as i32 } else { (x - 0.5) as i32 }
}

// ─── Vote table ──────────────────────────────────────────────────

type Scores = [i32; 12];
// ─── Substrate weight (configurable) ────────────────────────────
// Controls the relative influence of substrate execution vs. family
// affinity in build_via_substrate. Default 3 (= 3:1 substrate:family).
// Modify at runtime via set_substrate_weight() to explore bifurcation.
static mut SUBSTRATE_WEIGHT: i32 = 3;

/// Get the current substrate weight multiplier.
pub fn substrate_weight() -> i32 {
    unsafe { SUBSTRATE_WEIGHT }
}

/// Set substrate weight at runtime. Returns the previous weight.
pub fn set_substrate_weight(w: i32) -> i32 {
    let prev = unsafe { SUBSTRATE_WEIGHT };
    unsafe { SUBSTRATE_WEIGHT = w };
    prev
}



// FAMILY_TOKEN_AFFINITY[family][token] — base affinity weight.
// Rows: 12 IG primitive families in IgTuple field order
//   (D, T, R, P, F, K, G, C, Phi, H, S, Omega).
// Columns: 12 tokens in Token discriminant order
//   (VINIT, TANCH, AFWD, AREV, CLINK, IMSCRIB, FSPLIT, FFUSE, EVALT, EVALF, ENGAGR, IFIX).
//
// Vote weight per primitive = affinity[family][token] × ordinal_of_variant.
// Higher-ordinal variants push harder toward their family's preferred tokens,
// so production rules are genuinely derived from the live tuple state.
const FAMILY_TOKEN_AFFINITY: [[i32; 12]; 12] = [
    // VINIT TANCH AFWD AREV CLINK IMSCRIB FSPLIT FFUSE EVALT EVALF ENGAGR IFIX
    [  2,    0,    1,   1,   0,    2,      0,     0,    0,    0,    0,     1 ], // D Dimensionality
    [  0,    0,    0,   0,   1,    1,      2,     2,    0,    0,    1,     0 ], // T Topology
    [  0,    0,    2,   1,   2,    0,      0,     1,    0,    0,    0,     0 ], // R Coupling
    [  0,    0,    0,   0,   0,    0,      2,     2,    1,    1,    1,     0 ], // P Parity
    [  0,    0,    1,   1,   0,    1,      0,     0,    2,    2,    0,     0 ], // F Fidelity
    [  0,    1,    2,   2,   2,    0,      0,     0,    0,    0,    0,     2 ], // K Kinetics
    [  0,    0,    0,   0,   1,    2,      1,     0,    0,    0,    0,     0 ], // G Granularity
    [  0,    0,    1,   0,   1,    0,      2,     2,    0,    0,    2,     0 ], // C Composition
    [  0,    0,    0,   0,   0,    0,      0,     0,    2,    2,    2,     0 ], // Phi Criticality
    [  0,    0,    2,   0,   2,    2,      0,     0,    0,    0,    0,     2 ], // H Chirality
    [  0,    0,    1,   0,   0,    1,      1,     1,    0,    0,    0,     0 ], // S Stoichiometry
    [  1,    0,    1,   0,   0,    2,      0,     0,    1,    1,    2,     0 ], // Omega Winding
];

/// Derive token votes from the IgTuple via the family affinity matrix.
/// contribution per primitive = affinity[family][token] × ordinal.
fn aggregate_votes(tuple: &IgTuple) -> Scores {
    let mut s: Scores = [0; 12];
    let fields: [IgPrim; 12] = [
        tuple.d, tuple.t, tuple.r, tuple.p, tuple.f, tuple.k,
        tuple.g, tuple.c, tuple.phi, tuple.h, tuple.s, tuple.omega,
    ];
    for (fam, prim) in fields.iter().enumerate() {
        let ord = ord_round(prim.ordinal());
        if ord <= 0 { continue; }
        let row = &FAMILY_TOKEN_AFFINITY[fam];
        for tok in 0..12 {
            s[tok] += row[tok] * ord;
        }
    }
    s
}

// ─── MiniKernel (substrate execution) ────────────────────────────
//
// A lightweight IMASM execution environment.  Registers R0-R3 are seeded
// from paired IgTuple primitives; running a canonical program transforms
// them.  Post-execution state is mapped to token scores via
// TOKEN_REG_AFFINITY, making the sequence builder an IMASM execution.
//
// Token semantics mirror kernel.rs except: FSPLIT copies the stack top
// (no fork tracking); FFUSE joins the two top values (no saved right_val);
// IFIX accumulates into R2 instead of writing to memory.  All other
// semantics are exact to kernel.rs.

fn tuple_to_b4(a: IgPrim, b: IgPrim) -> B4 {
    let combined = (ord_round(a.ordinal()) + ord_round(b.ordinal())) & 3;
    match combined { 0 => B4::N, 1 => B4::T, 2 => B4::F, _ => B4::B }
}

fn b4_score(v: B4) -> i32 {
    match v { B4::N => 0, B4::T | B4::F => 1, B4::B => 2 }
}

struct MiniKernel {
    stack: [B4; 64],
    sp:    usize,
    r:     [B4; 4],  // R0=Dim×Crit  R1=Top×Wind  R2=Kin×Fid  R3=Chir×Par
}

impl MiniKernel {
    fn from_tuple(tuple: &IgTuple) -> Self {
        Self {
            stack: [B4::N; 64],
            sp: 0,
            r: [
                tuple_to_b4(tuple.d,   tuple.phi),
                tuple_to_b4(tuple.t,   tuple.omega),
                tuple_to_b4(tuple.k,   tuple.f),
                tuple_to_b4(tuple.h,   tuple.p),
            ],
        }
    }

    fn push(&mut self, v: B4) {
        if self.sp < 64 { self.stack[self.sp] = v; self.sp += 1; }
    }
    fn pop(&mut self) -> B4 {
        if self.sp > 0 { self.sp -= 1; self.stack[self.sp] } else { B4::N }
    }
    fn peek(&self) -> B4 {
        if self.sp > 0 { self.stack[self.sp - 1] } else { B4::N }
    }

    fn run(&mut self, prog: &Program) {
        for tok in prog.as_slice() { self.step(*tok); }
    }

    fn step(&mut self, tok: Token) {
        use B4::*;
        match tok {
            Token::Vinit   => self.push(N),
            Token::Tanch   => { let _ = self.pop(); }
            Token::Afwd    => {
                // kernel: R0 = B4::from_u8(R0.wrapping_add(1))
                self.r[0] = B4::from_u8((self.r[0] as u8).wrapping_add(1));
            }
            Token::Arev    => {
                // kernel: R0 = B4::from_u8(R0.wrapping_sub(1))
                self.r[0] = B4::from_u8((self.r[0] as u8).wrapping_sub(1));
            }
            Token::Clink   => {
                // kernel: R3 = meet(R1, R2)
                self.r[3] = b4_meet(self.r[1], self.r[2]);
            }
            Token::Imscrib => {
                // kernel: R4-R7 from snapshot; here: R3 accumulates stack top
                self.r[3] = b4_join(self.r[3], self.peek());
            }
            Token::Fsplit  => {
                // kernel: copy top, save as right_val; simplified: just copy
                let v = self.peek();
                self.push(v);
            }
            Token::Ffuse   => {
                // kernel: join linear left with saved right_val; simplified: join top two
                let a = self.pop();
                let b = self.pop();
                self.push(b4_join(a, b));
            }
            Token::Evalt   => {
                let v = self.pop();
                self.push(if v == T { T } else { N });
            }
            Token::Evalf   => {
                let v = self.pop();
                self.push(if v == F { F } else { N });
            }
            Token::Engagr  => {
                self.push(B);
                self.r[1] = b4_join(self.r[1], B);
            }
            Token::Ifix    => {
                // kernel: store to memory[R0]; here: accumulate into R2
                let v = self.pop();
                self.r[2] = b4_join(self.r[2], v);
            }
        }
    }

    fn register_scores(&self) -> Scores {
        // TOKEN_REG_AFFINITY[token][reg]: how strongly does register reg's
        // post-execution value vote for token?
        // Columns: R0(Dim×Crit), R1(Top×Wind), R2(Kin×Fid), R3(Chir×Par).
        const TOKEN_REG_AFFINITY: [[i32; 4]; 12] = [
            // R0  R1  R2  R3
            [  2,  0,  0,  1 ], // VINIT
            [  0,  0,  2,  0 ], // TANCH
            [  1,  2,  2,  0 ], // AFWD
            [  0,  1,  1,  2 ], // AREV
            [  2,  1,  0,  0 ], // CLINK
            [  0,  0,  0,  3 ], // IMSCRIB
            [  0,  2,  0,  0 ], // FSPLIT
            [  0,  2,  0,  1 ], // FFUSE
            [  1,  0,  2,  0 ], // EVALT
            [  1,  0,  2,  0 ], // EVALF
            [  2,  0,  0,  2 ], // ENGAGR
            [  0,  0,  3,  1 ], // IFIX
        ];
        let rv = [b4_score(self.r[0]), b4_score(self.r[1]),
                  b4_score(self.r[2]), b4_score(self.r[3])];
        let mut s: Scores = [0; 12];
        for tok in 0..12 {
            for reg in 0..4 {
                s[tok] += TOKEN_REG_AFFINITY[tok][reg] * rv[reg];
            }
        }
        s
    }
}

/// Derive token scores by executing a canonical program on a MiniKernel
/// seeded from the IgTuple.  Canonical selection is tier-driven:
/// O_0→I, O_1→IV, O_2→VII, O_∞→XII.
fn substrate_votes(tuple: &IgTuple, tier: u8) -> Scores {
    let idx = match tier { 3 => 11, 2 => 6, 1 => 3, _ => 0 };
    let mut mk = MiniKernel::from_tuple(tuple);
    if let Some(prog) = crate::tokens::canonical(idx) {
        mk.run(&prog);
    }
    mk.register_scores()
}

// ─── Sorted token order ───────────────────────────────────────────

fn sorted_by_score(scores: &Scores) -> [Token; 12] {
    const ALL: [Token; 12] = [
        Token::Vinit,  Token::Tanch,   Token::Afwd,   Token::Arev,
        Token::Clink,  Token::Imscrib, Token::Fsplit,  Token::Ffuse,
        Token::Evalt,  Token::Evalf,   Token::Engagr,  Token::Ifix,
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
    let mut result = [Token::Afwd; 12];
    let mut k = 0;
    while k < 12 { result[k] = ALL[order[k]]; k += 1; }
    result
}

// ─── Stack-depth accounting ───────────────────────────────────────

fn stack_delta(tok: Token) -> i32 {
    match tok {
        Token::Vinit | Token::Engagr => 1,
        Token::Fsplit                 => 1,
        Token::Tanch | Token::Ifix   => -1,
        Token::Ffuse                  => -1,
        _                             =>  0,
    }
}

// ─── Program composition ─────────────────────────────────────────

/// Build a valid IMASM program of `len` tokens from the given score vector.
/// Rules (priority order):
///   1. Self-ref bookending (IMSCRIB at first/last positions).
///   2. Force FFUSE before running out of space for open forks.
///   3. Never reduce stack below 0.
///   4. Never emit FFUSE with no open fork.
///   5. Limit FSPLIT so remaining capacity can close all open forks.
///   6. Otherwise: highest-voted valid token.
fn build_program_from_scores(scores: &Scores, len: usize, self_ref: bool) -> Program {
    let len = len.max(4).min(62);
    let preferred = sorted_by_score(scores);

    let mut p = Program::empty();
    let mut est_depth: i32 = 1;
    let mut open_forks: u32 = 0;

    let mut i = 0;
    while i < len {
        let remaining = len - i;
        let is_first = i == 0;
        let is_last  = remaining == 1;

        if is_first && self_ref {
            p.push(Token::Imscrib);
            i += 1;
            continue;
        }
        if is_last && self_ref && open_forks == 0 {
            p.push(Token::Imscrib);
            i += 1;
            continue;
        }
        if open_forks > 0 && remaining <= open_forks as usize {
            p.push(Token::Ffuse);
            open_forks -= 1;
            est_depth -= 1;
            i += 1;
            continue;
        }

        let mut chosen = Token::Afwd;
        let mut found = false;
        let mut pi = 0;
        while pi < 12 {
            let tok = preferred[pi];
            let depth_after = est_depth + stack_delta(tok);
            if depth_after < 0 { pi += 1; continue; }
            if tok == Token::Ffuse && open_forks == 0 { pi += 1; continue; }
            if tok == Token::Fsplit && (remaining as u32) <= open_forks + 2 { pi += 1; continue; }
            if tok == Token::Tanch && (!is_last || self_ref || open_forks > 0) { pi += 1; continue; }
            chosen = tok;
            found = true;
            break;
        }
        if !found { chosen = Token::Imscrib; }

        p.push(chosen);
        if chosen == Token::Fsplit { open_forks += 1; }
        if chosen == Token::Ffuse  { open_forks -= 1; }
        est_depth += stack_delta(chosen);
        i += 1;
    }

    while open_forks > 0 && p.len() < 64 {
        p.push(Token::Ffuse);
        open_forks -= 1;
    }
    p
}

// ─── Public API ───────────────────────────────────────────────────

/// Build the next program via substrate execution: run a canonical IMASM
/// program on a MiniKernel seeded from the IgTuple (substrate scores),
/// combined with family affinity matrix scores (family scores).
///
/// This is the primary production function in dynamic mode.  The sequence
/// builder is itself an IMASM execution — the substrate constructs its own
/// continuation.
pub fn build_via_substrate(tuple: &IgTuple, len: usize, self_ref: bool, tier: u8) -> Program {
    let family_s = aggregate_votes(tuple);
    let sub_s    = substrate_votes(tuple, tier);
    let mut combined: Scores = [0; 12];
    for i in 0..12 {
        // Substrate execution is the primary signal (×3); family matrix is baseline.
        combined[i] = sub_s[i] * substrate_weight() + family_s[i];
    }
    build_program_from_scores(&combined, len, self_ref)
}

/// Build from family affinity scores only (no substrate execution).
/// Kept for compatibility; `build_via_substrate` is used in dynamic mode.
pub fn build_next_program(tuple: &IgTuple, len: usize, self_ref: bool) -> Program {
    let scores = aggregate_votes(tuple);
    build_program_from_scores(&scores, len, self_ref)
}

/// Recommended sequence length for the next program.
pub fn next_seq_len(snap: &crate::kernel::Snapshot) -> usize {
    match snap.tier { 3 => 12, 2 => 10, 1 => 8, _ => 6 }
}

// ─── Debug display ────────────────────────────────────────────────

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
