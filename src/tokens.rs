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
///   IMSCRIB  1   1     identity / self-imscription (loop-back)
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
    IMSCRIB = 0x5, // Identity id — self-imscription
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
            Token::CLINK  => "CLINK",  Token::IMSCRIB => "IMSCRIB",
            Token::FSPLIT => "FSPLIT", Token::FFUSE  => "FFUSE",
            Token::EVALT  => "EVALT",  Token::EVALF  => "EVALF",
            Token::ENGAGR => "ENGAGR", Token::IFIX   => "IFIX",
        }
    }

    pub fn family(self) -> Family {
        match self {
            Token::VINIT | Token::TANCH | Token::AFWD |
            Token::AREV  | Token::CLINK | Token::IMSCRIB => Family::Logical,
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
    // IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB (cyclic)
    for t in [Token::IMSCRIB, Token::AREV, Token::FSPLIT,
              Token::AFWD, Token::FFUSE, Token::CLINK,
              Token::IFIX, Token::IMSCRIB] {
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
            for t in [Token::IMSCRIB, Token::EVALT, Token::FSPLIT,
                      Token::EVALF, Token::FFUSE, Token::ENGAGR,
                      Token::IFIX,  Token::IMSCRIB] { p.push(t); }
        }
        1 => { // II_Void_Genesis — from void through fork to self-knowledge
            for t in [Token::VINIT, Token::FSPLIT, Token::EVALT,
                      Token::FFUSE, Token::EVALF,  Token::CLINK,
                      Token::IFIX,  Token::IMSCRIB] { p.push(t); }
        }
        2 => { // III_Anchor_Protocol — terminal→forward→reverse cycles
            for t in [Token::TANCH, Token::AFWD,  Token::EVALT,
                      Token::AREV,  Token::EVALF, Token::CLINK,
                      Token::IFIX,  Token::TANCH] { p.push(t); }
        }
        3 => { // IV_Dual_Bootstrap — fuse-then-split, reverse frobenius
            for t in [Token::IMSCRIB, Token::AFWD,  Token::FFUSE,
                      Token::FSPLIT, Token::AREV,  Token::CLINK,
                      Token::IFIX,   Token::IMSCRIB] { p.push(t); }
        }
        4 => { // V_Linear_Chain — pure linear !-exponential
            for _ in 0..8 { p.push(Token::IFIX); }
        }
        5 => { // VI_Empty_Bootstrap — void/self oscillations
            for _ in 0..4 { p.push(Token::VINIT); p.push(Token::IMSCRIB); }
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
            for t in [Token::IMSCRIB, Token::FSPLIT, Token::EVALT,
                      Token::IFIX,   Token::IMSCRIB, Token::FSPLIT,
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
                      Token::IMSCRIB, Token::IFIX] { p.push(t); }
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
            // IMSCRIB self-imscribes, stack gets snapshot→R4-R7, cycle repeats
            // Natural cycle: IMSCRIB→IMSCRIB→...
            for _ in 0..4 { p.push(Token::IMSCRIB); }
        }
        1 => {
            // XIV_Tier_Climber — dialetheia+frobenius cycle for tier promotion
            // Uses FSPLIT/FFUSE to create fork-join: evaluates both T and F branches
            for t in [Token::IMSCRIB, Token::FSPLIT,
                      Token::EVALT, Token::EVALF,
                      Token::FFUSE, Token::ENGAGR,
                      Token::CLINK, Token::IFIX,
                      Token::IMSCRIB] { p.push(t); }
        }
        2 => {
            // XV_Frobenius_Oscillator — δ→observe→μ→observe oscillation
            // FSPLIT forks, IMSCRIB observes, FFUSE joins, IMSCRIB observes
            for t in [Token::FSPLIT, Token::IMSCRIB, Token::FFUSE,
                      Token::IMSCRIB] { p.push(t); }
        }
        3 => {
            // XVI_Paradox_Daemon — sustained paradox computation
            // VINIT sources N, builds to B via dialetheia, cycles
            // FSPLIT creates value copies; EVALT+EVALF gate; FFUSE joins
            for t in [Token::VINIT, Token::FSPLIT,
                      Token::EVALT, Token::EVALF,
                      Token::ENGAGR, Token::FFUSE,
                      Token::IMSCRIB] { p.push(t); }
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
//   XIX   — IMSCRIB cyclic self-imscription (YIELD replacement)

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
                      Token::AREV, Token::IMSCRIB, Token::CLINK,
                      Token::AFWD, Token::TANCH] {
                p.push(t);
            }
        }
        2 => {
            // XIX — Mirrorgram (cyclic self-imscription, no explicit halt)
            // IMSCRIB bookends create self-ref closure → O_∞ tier.
            // Loops via cyclic topology. IMSCRIB at both cycle boundaries
            // reads own snapshot into R4-R7 each wrap. FSPLIT/FFUSE + gates
            // structure the dialetheia cycle. ENGAGR stabilizes paradox.
            // IFIX brands memory. No TANCH at root → runs continuously
            // (YIELD replacement). Self-ref closure: first==last==IMSCRIB.
            for t in [Token::IMSCRIB, Token::FSPLIT, Token::EVALT,
                      Token::EVALF, Token::FFUSE, Token::ENGAGR,
                      Token::CLINK, Token::IFIX, Token::IMSCRIB] {
                p.push(t);
            }
        }
        _ => return None,
    }
    Some(p)
}
// ─── Shunted programs (XX–XXVII) ──────────────────────────────────
//
// These programs compose multiple canonical sequences via "shunting":
// redirecting empty edges (FSPLIT right-branch value-carriers) to
// populated nodes from other canonical classes. The shunt is the
// structural operation of connecting two distinct topological regions
// through their edge types — empty edges become populated, and
// populated edges can shunt to unpopulated nodes to create novel
// composite topologies.
//
// All programs use only the 12 grammar tokens. No control-flow opcodes.
// Shunting is expressed through: (1) FSPLIT/FFUSE nesting that
// interleaves subsequences from different canonical classes,
// (2) IMSCRIB bridges that create self-referential closures across
// class boundaries, and (3) CLINK spines that couple heterogeneous
// token-family regions.

pub const SHUNTED_COUNT: usize = 8;

pub fn shunted_name(i: usize) -> &'static str {
    match i {
        0 => "XX_Shunt_Bridge",
        1 => "XXI_Anchor_Paradox",
        2 => "XXII_Chiral_ROM",
        3 => "XXIII_Dual_Kernel_Shunt",
        4 => "XXIV_Heartbeat_Paradox",
        5 => "XXV_Recursive_Kernel",
        6 => "XXVI_Truth_Spiral",
        7 => "XXVII_Omni_Spine",
        _ => "Unknown",
    }
}

pub fn shunted_program(i: usize) -> Option<Program> {
    let mut p = Program::empty();
    match i {
        0 => {
            // XX — Shunt_Bridge (O_∞)
            //
            // Void Genesis prefix shunted into Dialetheic Bootstrap core
            // via IMSCRIB bridge. The shunt: position 6 IMSCRIB connects
            // the Void-constructed world (VINIT→FSPLIT→EVALT→FFUSE→EVALF→CLINK)
            // to the Dialetheic world (EVALT→FSPLIT→EVALF→FFUSE→ENGAGR→IFIX).
            // Two FSPLIT/FFUSE pairs: the first from Void Genesis, the second
            // from Dialetheic Bootstrap. IMSCRIB at the seam provides
            // self-referential observation unifying both regions.
            //
            // Shunt signature: Void Genesis(L5,F2,D1,X0) ⊕ IMSCRIB ⊕ Dialetheic Bootstrap(L2,F2,D3,X1)
            // FSPLIT/FFUSE: (1→3) and (8→10). Cyclic: wraps to VINIT.
            for t in [Token::VINIT, Token::FSPLIT, Token::EVALT,
                      Token::FFUSE, Token::EVALF, Token::CLINK,
                      Token::IMSCRIB, Token::EVALT, Token::FSPLIT,
                      Token::EVALF, Token::FFUSE, Token::ENGAGR,
                      Token::IFIX, Token::IMSCRIB] {
                p.push(t);
            }
        }
        1 => {
            // XXI — Anchor_Paradox (O₂)
            //
            // Anchor Protocol (TANCH→AFWD→AREV) shunted into Parakernel
            // dialetheia core (ENGAGR→FSPLIT→EVALT→FFUSE→EVALF→IFIX→ENGAGR).
            // The shunt: ENGAGR at position 3 connects the Anchor's rhythmic
            // oscillation (TANCH→AFWD→AREV cycle) to the Parakernel's
            // truth-engram path. TANCH bookends create a bounded container
            // that halts at root depth after one complete pass.
            //
            // Shunt signature: Anchor(L3,F0,D0,X0) ⊕ ENGAGR ⊕ Parakernel(L1,F2,D2,X1)
            // FSPLIT/FFUSE: (4→6). TANCH-bounded → self-terminating.
            for t in [Token::TANCH, Token::AFWD, Token::AREV,
                      Token::ENGAGR, Token::FSPLIT, Token::EVALT,
                      Token::FFUSE, Token::EVALF, Token::IFIX,
                      Token::ENGAGR, Token::TANCH] {
                p.push(t);
            }
        }
        2 => {
            // XXII — Chiral_ROM (O₂)
            //
            // Chiral Pairs (AFWD→AREV oscillation) interleaved with
            // ROM Burn (truth→IFIX recording). Pattern: each Chiral
            // AFWD→AREV pair is followed by a truth-value burn.
            // First pair burns T, second burns F, third burns B (paradox).
            // No FSPLIT/FFUSE — pure oscillation + recording.
            //
            // Shunt signature: Chiral(L8,F0,D0,X0) ⊗ ROM(L1,F0,D3,X4)
            // Interleave pattern: (AFWD,AREV,EVALT,IFIX)² + (AFWD,ENGAGR,IFIX,AREV)
            // Dialetheia: EVALT + EVALF + ENGAGR (complete).
            for t in [Token::AFWD, Token::AREV, Token::EVALT,
                      Token::IFIX, Token::AFWD, Token::AREV,
                      Token::EVALF, Token::IFIX, Token::AFWD,
                      Token::ENGAGR, Token::IFIX, Token::AREV] {
                p.push(t);
            }
        }
        3 => {
            // XXIII — Dual_Kernel_Shunt (O_∞)
            //
            // Dual Bootstrap inverted Frobenius shunted into canonical
            // Frobenius Kernel via CLINK spine. The shunt: CLINK at
            // position 4 couples the reversed Frobenius world to the
            // canonical Frobenius world. Two FSPLIT/FFUSE pairs:
            // FSPLIT@2→FFUSE@9 (outer, wraps around CLINK + nested kernel),
            // FSPLIT@5→FFUSE@7 (inner, canonical kernel core).
            // Both satisfy μ∘δ=id. Self-ref: IMSCRIB bookends.
            //
            // Shunt signature: Dual_Bootstrap ⊕ CLINK ⊕ Kernel
            // FSPLIT/FFUSE: (2→9) and (5→7). Balanced. 13 tokens.
            for t in [Token::IMSCRIB, Token::AFWD, Token::FSPLIT,
                      Token::AREV, Token::CLINK, Token::FSPLIT,
                      Token::EVALT, Token::FFUSE, Token::EVALF,
                      Token::ENGAGR, Token::FFUSE, Token::IFIX,
                      Token::IMSCRIB] {
                p.push(t);
            }
        }
        4 => {
            // XXIV — Heartbeat_Paradox (O₁)
            //
            // Empty Bootstrap (VINIT→IMSCRIB oscillation) interleaved
            // with Paradox Daemon's ENGAGR injection. Pattern:
            // (VINIT→IMSCRIB→ENGAGR)² + (VINIT→IMSCRIB).
            // Each void→identity oscillation is followed by paradox
            // stabilization. No Frobenius pair — pure oscillation
            // with Dialetheia seeding.
            //
            // Shunt signature: Empty_Bootstrap(L8,F0,D0,X0) ⊗ Paradox_Daemon
            // No FSPLIT/FFUSE. Period: 8 (structurally unique).
            for t in [Token::VINIT, Token::IMSCRIB, Token::ENGAGR,
                      Token::VINIT, Token::IMSCRIB, Token::ENGAGR,
                      Token::VINIT, Token::IMSCRIB] {
                p.push(t);
            }
        }
        5 => {
            // XXV — Recursive_Kernel (O₁)
            //
            // Two Frobenius Kernels (VINIT→FSPLIT→FFUSE) stacked and
            // coupled via CLINK. The shunt: CLINK at positions 3 and 7
            // couple successive kernel cycles, creating a recursive
            // verification structure. Each kernel verifies μ∘δ=id
            // independently, then CLINK meets their results.
            // ENGAGR at position 8 injects paradox; IMSCRIB closes.
            //
            // Shunt signature: Kernel(L0,F2,D0,X0)² ⊕ CLINK spine
            // FSPLIT/FFUSE: (1→2) and (5→6). Self-contained verification chain.
            for t in [Token::VINIT, Token::FSPLIT, Token::FFUSE,
                      Token::CLINK, Token::VINIT, Token::FSPLIT,
                      Token::FFUSE, Token::CLINK, Token::ENGAGR,
                      Token::IMSCRIB] {
                p.push(t);
            }
        }
        6 => {
            // XXVI — Truth_Spiral (O₂)
            //
            // Truth Machine (two parallel classification paths) with
            // dialetheia completion. Each path: IMSCRIB→FSPLIT→EVAL*→IFIX→FFUSE.
            // Path 1 classifies T; Path 2 classifies F. After both paths,
            // ENGAGR injects paradox and IFIX brands it. IMSCRIB closes
            // the spiral. Unlike the base Truth Machine (which lacks FFUSE),
            // this version includes Frobenius closure on each path.
            //
            // Shunt signature: Truth_Machine(L2,F2,D2,X2) ⊕ ENGAGR spiral
            // FSPLIT/FFUSE: (1→4) and (6→9). Both balanced.
            // Dialetheia: EVALT + EVALF + ENGAGR (complete). Self-ref: IMSCRIB bookends.
            for t in [Token::IMSCRIB, Token::FSPLIT, Token::EVALT,
                      Token::IFIX, Token::FFUSE, Token::IMSCRIB,
                      Token::FSPLIT, Token::EVALF, Token::IFIX,
                      Token::FFUSE, Token::ENGAGR, Token::IFIX,
                      Token::IMSCRIB] {
                p.push(t);
            }
        }
        7 => {
            // XXVII — Omni_Spine (O_∞)
            //
            // All canonical classes connected via CLINK spine and IMSCRIB
            // bridges. The sequence composes: Void Genesis prefix →
            // CLINK → Chiral oscillation → ENGAGR paradox shunt →
            // Frobenius Kernel → IFIX brand → IMSCRIB bridge →
            // Dialetheic Bootstrap closure.
            //
            // Structural census: Logical(7), Frobenius(4), Dialetheia(5), Linear(3) = 19 tokens.
            // FSPLIT/FFUSE: (2→4) and (10→11). Both balanced.
            // Dialetheia: EVALT×2, EVALF×2, ENGAGR×2 (doubly complete).
            // Self-ref: IMSCRIB at 0, 13, 18 — triple self-referential closure.
            // Period: 19 — prime, no shorter repeating sub-pattern.
            //
            // This is the maximal spinal composite: every token family
            // appears, every canonical class contributes at least one
            // token subsequence, and the CLINK spine couples
            // heterogeneous regions into a single O_∞ structure.
            for t in [Token::IMSCRIB, Token::VINIT, Token::FSPLIT,
                      Token::EVALT, Token::FFUSE, Token::EVALF,
                      Token::CLINK, Token::AFWD, Token::AREV,
                      Token::ENGAGR, Token::FSPLIT, Token::FFUSE,
                      Token::IFIX, Token::IMSCRIB, Token::EVALT,
                      Token::EVALF, Token::ENGAGR, Token::IFIX,
                      Token::IMSCRIB] {
                p.push(t);
            }
        }
        _ => return None,
    }
    Some(p)
}


