/// The 12 IMASM opcodes — categorical duals of the 12 IG primitives.
/// No control-flow extensions. Looping, halting, and conditional branching
/// are constructed from the graph arity of the tokens themselves:
///
///   Token   In  Out   Graph role
///   VINIT   0   1     source (always ready)
///   TANCH   1   0     sink (terminates a wire → halting)
///   AFWD    1   1     forward morphism
///   AREV    1   1     contravariant inversion
///   CLINK   1   1     composition / meet
///   ISCRIB  1   1     identity / self-imscription (loop-back)
///   FSPLIT  1   2     fork — bifurcation (→ conditional)
///   EVALT   1   1     T-gate — passes T, blocks non-T
///   EVALF   1   1     F-gate — passes F, blocks non-F
///   FFUSE   2   1     join — recombination (← conditional)
///   ENGAGR  1   1     Both — paradox stabilized
///   IFIX    1   1     linear ! exponential
///
/// Loop: end-of-program wraps to start (cyclic graph). No YIELD needed.
/// Halt: TANCH sinks a value at root depth → empty frontier. No HALT needed.
/// Jump: FSPLIT→[EVALT|EVALF]→...→FFUSE selects branches. No JNZ/JZ needed.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Token {
    VINIT  = 0x0, // Initial object ∅ — void; 0→1 source
    TANCH  = 0x1, // Terminal anchor ⊤ — boundary; 1→0 sink
    AFWD   = 0x2, // Forward morphism → — directed transition
    AREV   = 0x3, // Contravariant inversion ← — reversal
    CLINK  = 0x4, // Composition ∘ — linkage (meet R1∧R2→R3)
    ISCRIB = 0x5, // Identity id — self-imscription
    FSPLIT = 0x6, // Co-multiplication δ — bifurcation (1→2 fork)
    FFUSE  = 0x7, // Multiplication μ — recombination (2→1 join)
    EVALT  = 0x8, // True — affirmation; T-gate
    EVALF  = 0x9, // False — negation; F-gate
    ENGAGR = 0xA, // Both — paradox stabilized
    IFIX   = 0xB, // Permanent brand — linear ! exponential
}

impl Token {
    pub fn name(self) -> &'static str {
        match self {
            Token::VINIT  => "VINIT",  Token::TANCH  => "TANCH",
            Token::AFWD   => "AFWD",   Token::AREV   => "AREV",
            Token::CLINK  => "CLINK",  Token::ISCRIB => "ISCRIB",
            Token::FSPLIT => "FSPLIT", Token::FFUSE  => "FFUSE",
            Token::EVALT  => "EVALT",  Token::EVALF  => "EVALF",
            Token::ENGAGR => "ENGAGR", Token::IFIX   => "IFIX",
        }
    }

    pub fn family(self) -> Family {
        match self {
            Token::VINIT | Token::TANCH | Token::AFWD |
            Token::AREV  | Token::CLINK | Token::ISCRIB => Family::Logical,
            Token::FSPLIT | Token::FFUSE                => Family::Frobenius,
            Token::EVALT  | Token::EVALF | Token::ENGAGR => Family::Dialetheia,
            Token::IFIX                                 => Family::Linear,
        }
    }

    /// Input arity — how many stack values this token consumes before firing.
    pub fn arity_in(self) -> u8 {
        match self {
            Token::VINIT  => 0,
            Token::FFUSE  => 2,
            _             => 1,
        }
    }

    /// Output arity — how many stack values this token produces.
    pub fn arity_out(self) -> u8 {
        match self {
            Token::TANCH  => 0,
            Token::FSPLIT => 2,
            _             => 1,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Family {
    Logical,
    Frobenius,
    Dialetheia,
    Linear,
}

/// Fixed-capacity program: up to 64 tokens.
#[derive(Copy, Clone)]
pub struct Program {
    buf: [Token; 64],
    len: usize,
}

impl Program {
    pub const fn empty() -> Self {
        Self { buf: [Token::VINIT; 64], len: 0 }
    }

    pub fn push(&mut self, t: Token) {
        if self.len < 64 { self.buf[self.len] = t; self.len += 1; }
    }

    pub fn get(&self, i: usize) -> Option<Token> {
        if i < self.len { Some(self.buf[i]) } else { None }
    }

    pub fn len(&self) -> usize { self.len }

    pub fn as_slice(&self) -> &[Token] { &self.buf[..self.len] }

    pub fn inject(&mut self, pos: usize, t: Token) {
        if self.len >= 64 { return; }
        let pos = pos.min(self.len);
        let mut i = self.len;
        while i > pos { self.buf[i] = self.buf[i - 1]; i -= 1; }
        self.buf[pos] = t;
        self.len += 1;
    }
}

// ─── Bootstrap + 12 Canonicals ───────────────────────────────────
//
// All programs are cyclic graphs — the last token's output wire connects
// back to the first token's input. Execution wraps naturally.
// FSPLIT/FFUSE pairs create fork-join subgraphs for conditional flow.
// TANCH at root depth sinks the wire, triggering halt.

pub fn bootstrap_loop() -> Program {
    let mut p = Program::empty();
    // ISCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→ISCRIB (cyclic)
    for t in [Token::ISCRIB, Token::AREV, Token::FSPLIT,
              Token::AFWD, Token::FFUSE, Token::CLINK,
              Token::IFIX, Token::ISCRIB] {
        p.push(t);
    }
    p
}

pub const CANONICAL_COUNT: usize = 12;

pub fn canonical_name(i: usize) -> &'static str {
    match i {
        0  => "I_Dialetheic_Bootstrap",
        1  => "II_Void_Genesis",
        2  => "III_Anchor_Protocol",
        3  => "IV_Dual_Bootstrap",
        4  => "V_Linear_Chain",
        5  => "VI_Empty_Bootstrap",
        6  => "VII_Parakernel",
        7  => "VIII_Frobenius_Kernel",
        8  => "IX_Chiral_Pairs",
        9  => "X_Truth_Machine",
        10 => "XI_Eternal_Return",
        11 => "XII_ROM_Burn",
        _  => "Unknown",
    }
}

pub fn canonical(i: usize) -> Option<Program> {
    let mut p = Program::empty();
    match i {
        0 => { // I_Dialetheic_Bootstrap — full dialetheia cycle, self-imscribing
            for t in [Token::ISCRIB, Token::EVALT, Token::FSPLIT,
                      Token::EVALF, Token::FFUSE, Token::ENGAGR,
                      Token::IFIX,  Token::ISCRIB] { p.push(t); }
        }
        1 => { // II_Void_Genesis — from void through fork to self-knowledge
            for t in [Token::VINIT, Token::FSPLIT, Token::EVALT,
                      Token::FFUSE, Token::EVALF,  Token::CLINK,
                      Token::IFIX,  Token::ISCRIB] { p.push(t); }
        }
        2 => { // III_Anchor_Protocol — terminal→forward→reverse cycles
            for t in [Token::TANCH, Token::AFWD,  Token::EVALT,
                      Token::AREV,  Token::EVALF, Token::CLINK,
                      Token::IFIX,  Token::TANCH] { p.push(t); }
        }
        3 => { // IV_Dual_Bootstrap — fuse-then-split, reverse frobenius
            for t in [Token::ISCRIB, Token::AFWD,  Token::FFUSE,
                      Token::FSPLIT, Token::AREV,  Token::CLINK,
                      Token::IFIX,   Token::ISCRIB] { p.push(t); }
        }
        4 => { // V_Linear_Chain — pure linear !-exponential
            for _ in 0..8 { p.push(Token::IFIX); }
        }
        5 => { // VI_Empty_Bootstrap — void/self oscillations
            for _ in 0..4 { p.push(Token::VINIT); p.push(Token::ISCRIB); }
        }
        6 => { // VII_Parakernel — paradox-anchored dialetheia
            for t in [Token::ENGAGR, Token::AFWD,  Token::FSPLIT,
                      Token::EVALT,  Token::FFUSE, Token::EVALF,
                      Token::IFIX,   Token::ENGAGR] { p.push(t); }
        }
        7 => { // VIII_Frobenius_Kernel — split/fuse oscillation
            for _ in 0..2 { p.push(Token::FSPLIT); p.push(Token::FFUSE); }
        }
        8 => { // IX_Chiral_Pairs — forward/reverse pairs
            for _ in 0..4 { p.push(Token::AFWD); p.push(Token::AREV); }
        }
        9 => { // X_Truth_Machine — nested conditional
            for t in [Token::ISCRIB, Token::FSPLIT, Token::EVALT,
                      Token::IFIX,   Token::ISCRIB, Token::FSPLIT,
                      Token::EVALF,  Token::IFIX] { p.push(t); }
        }
        10 => { // XI_Eternal_Return — TANCH→AFWD→AREV cycle
            for t in [Token::TANCH, Token::AFWD,  Token::AREV,
                      Token::TANCH, Token::AFWD,  Token::AREV,
                      Token::TANCH, Token::AFWD] { p.push(t); }
        }
        11 => { // XII_ROM_Burn — truth values→permanent brand
            for t in [Token::EVALT,  Token::IFIX, Token::EVALF,
                      Token::IFIX,   Token::ENGAGR, Token::IFIX,
                      Token::ISCRIB, Token::IFIX] { p.push(t); }
        }
        _ => return None,
    }
    Some(p)
}

// ─── Continuous programs (XIII–XVI) ───────────────────────────
//
// These use only the 12 tokens. Loop via cyclic graph topology.
// Conditional branching via FSPLIT→[EVALT|EVALF]→...→FFUSE.
// Halt via TANCH at root depth.
// No HALT/YIELD/JNZ/JZ.

pub const CONTINUOUS_COUNT: usize = 4;

pub fn continuous_name(i: usize) -> &'static str {
    match i {
        0 => "XIII_Heartbeat",
        1 => "XIV_Tier_Climber",
        2 => "XV_Frobenius_Oscillator",
        3 => "XVI_Paradox_Daemon",
        _ => "Unknown",
    }
}

pub fn continuous_program(i: usize) -> Option<Program> {
    let mut p = Program::empty();
    match i {
        0 => {
            // XIII_Heartbeat — minimal self-imscription loop
            // ISCRIB self-imscribes, stack gets snapshot→R4-R7, cycle repeats
            // Natural cycle: ISCRIB→ISCRIB→...
            for _ in 0..4 { p.push(Token::ISCRIB); }
        }
        1 => {
            // XIV_Tier_Climber — dialetheia+frobenius cycle for tier promotion
            // Uses FSPLIT/FFUSE to create fork-join: evaluates both T and F branches
            for t in [Token::ISCRIB, Token::FSPLIT,
                      Token::EVALT, Token::EVALF,
                      Token::FFUSE, Token::ENGAGR,
                      Token::CLINK, Token::IFIX,
                      Token::ISCRIB] { p.push(t); }
        }
        2 => {
            // XV_Frobenius_Oscillator — δ→observe→μ→observe oscillation
            // FSPLIT forks, ISCRIB observes, FFUSE joins, ISCRIB observes
            for t in [Token::FSPLIT, Token::ISCRIB, Token::FFUSE,
                      Token::ISCRIB] { p.push(t); }
        }
        3 => {
            // XVI_Paradox_Daemon — sustained paradox computation
            // VINIT sources N, builds to B via dialetheia, cycles
            // FSPLIT creates value copies; EVALT+EVALF gate; FFUSE joins
            for t in [Token::VINIT, Token::FSPLIT,
                      Token::EVALT, Token::EVALF,
                      Token::ENGAGR, Token::FFUSE,
                      Token::ISCRIB] { p.push(t); }
        }
        _ => return None,
    }
    Some(p)
}

/// Family signature (Logical, Frobenius, Dialetheia, Linear).
pub fn signature(prog: &Program) -> (usize, usize, usize, usize) {
    let (mut l, mut f, mut d, mut x) = (0, 0, 0, 0);
    for t in prog.as_slice() {
        match t.family() {
            Family::Logical    => l += 1,
            Family::Frobenius  => f += 1,
            Family::Dialetheia => d += 1,
            Family::Linear     => x += 1,
        }
    }
    (l, f, d, x)
}

/// Minimal period of the program.
pub fn period(prog: &Program) -> usize {
    let n = prog.len();
    if n == 0 { return 1; }
    for p in 1..=n {
        if n % p == 0 {
            let periodic = (p..n).all(|i| prog.get(i) == prog.get(i % p));
            if periodic { return p; }
        }
    }
    n
}
// ─── Novel programs (XVII–XIX) ──────────────────────────────
// Demonstrate the three reconstructed control-flow features:
//   XVII  — Nested FSPLIT/FFUSE (JNZ/JZ replacement, fork depth 3)
//   XVIII — TANCH root-depth halt (HALT replacement)
//   XIX   — ISCRIB cyclic self-imscription (YIELD replacement)

pub const NOVEL_COUNT: usize = 3;

pub fn novel_name(i: usize) -> &'static str {
    match i {
        0 => "XVII_Nested_Fork_Labyrinth",
        1 => "XVIII_Terminal_Sink_Protocol",
        2 => "XIX_Mirrorgram",
        _ => "Unknown",
    }
}

pub fn novel_program(i: usize) -> Option<Program> {
    let mut p = Program::empty();
    match i {
        0 => {
            // XVII — Nested Fork Labyrinth (fork depth 3)
            // Three nested FSPLIT/FFUSE pairs: balanced-parenthesis scanner
            // matches them correctly across 3 fork depths.
            for t in [Token::VINIT, Token::FSPLIT, Token::FSPLIT,
                      Token::FSPLIT, Token::AFWD, Token::FFUSE,
                      Token::AREV, Token::FFUSE, Token::EVALT,
                      Token::FFUSE, Token::TANCH] {
                p.push(t);
            }
        }
        1 => {
            // XVIII — Terminal Sink Protocol (TANCH at root halts)
            // Runs computation then cleanly terminates via TANCH at root depth.
            for t in [Token::VINIT, Token::AFWD, Token::AFWD,
                      Token::AREV, Token::ISCRIB, Token::CLINK,
                      Token::AFWD, Token::TANCH] {
                p.push(t);
            }
        }
        2 => {
            // XIX — Mirrorgram (cyclic self-imscription, no explicit halt)
            // ISCRIB bookends create self-ref closure → O_∞ tier.
            // Loops via cyclic topology. ISCRIB at both cycle boundaries
            // reads own snapshot into R4-R7 each wrap. FSPLIT/FFUSE + gates
            // structure the dialetheia cycle. ENGAGR stabilizes paradox.
            // IFIX brands memory. No TANCH at root → runs continuously
            // (YIELD replacement). Self-ref closure: first==last==ISCRIB.
            for t in [Token::ISCRIB, Token::FSPLIT, Token::EVALT,
                      Token::EVALF, Token::FFUSE, Token::ENGAGR,
                      Token::CLINK, Token::IFIX, Token::ISCRIB] {
                p.push(t);
            }
        }
        _ => return None,
    }
    Some(p)
}
