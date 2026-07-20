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
pub use imasm_core::classic::{Family, Token};

/// Fixed-capacity program: up to 64 tokens.
#[derive(Copy, Clone)]
pub struct Program {
    buf: [Token; 64],
    len: usize,
}

impl Program {
    pub const fn empty() -> Self {
        Self { buf: [Token::Vinit; 64], len: 0 }
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

    /// ROTAT — the first op-opcode: cyclic shift of the program-ring by `k`.
    ///
    /// Not one of the 12 tokens: an op-opcode is an operator ON the whole word,
    /// of a different order than the symbols in it. It is the ring automorphism,
    /// the Weyl-Heisenberg shift X on Z/nZ. The kernel already presumed it — the
    /// instruction pointer advancing `% n`, the cyclic FFUSE scan, the offset
    /// scan in `self_imscribe` — this only names the operator those all turn on.
    /// `signature` is ROTAT-invariant and `period` is the orbit size; `rotate`
    /// by `len()` is the identity, so the ring closes on itself.
    pub fn rotate(&self, k: usize) -> Program {
        let n = self.len;
        let mut out = Program::empty();
        if n == 0 { return out; }
        let s = k % n;
        for i in 0..n { out.push(self.buf[(s + i) % n]); }
        out
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
    for t in [Token::Imscrib, Token::Arev, Token::Fsplit,
              Token::Afwd, Token::Ffuse, Token::Clink,
              Token::Ifix, Token::Imscrib] {
        p.push(t);
    }
    p
}

/// The minimal program that deliberately targets O_inf_dag (R2, the lateral replicative
/// opening) rather than merely being able to report it if reached by accident. A single
/// FSPLIT/FFUSE pair (atomic_reentry), cyclic so the same fork point recurs every wrap
/// (bifurcation_revisited), and — crucially — no EVALT/EVALF/ENGAGR at all, so
/// dialetheia_complete is structurally false and b_live_ticks can never rise above 0:
/// Path A (dialetheia-driven O_∞) is unreachable regardless of period. The stack-top value
/// trace is constant B4::N every tick (traced by hand against tick()'s actual token
/// semantics: FSPLIT peeks N, pushes N; FFUSE pops N, joins(N,N)=N, pushes N — the loop
/// never introduces a second value), so value_period settles at 1, which keeps Path B
/// (value-trace-driven O_∞, needs value_period ≥ 3) unreachable too. With both O_∞ paths
/// closed off and R1 therefore never firing, R2's own conditions (self_ref, frobenius_order
/// > 0, atomic_reentry, bifurcation_revisited, winding_count > 0 after the first wrap) are
/// what `compute_tier` actually falls through to. See kernel.rs's `replicative_opening_tier`
/// test, which runs this program for real and asserts tier == 4 rather than trusting the
/// trace alone.
pub fn replicative_opening_loop() -> Program {
    let mut p = Program::empty();
    for t in [Token::Imscrib, Token::Fsplit, Token::Ffuse, Token::Imscrib] {
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
            for t in [Token::Imscrib, Token::Evalt, Token::Fsplit,
                      Token::Evalf, Token::Ffuse, Token::Engagr,
                      Token::Ifix,  Token::Imscrib] { p.push(t); }
        }
        1 => { // II_Void_Genesis — from void through fork to self-knowledge
            for t in [Token::Vinit, Token::Fsplit, Token::Evalt,
                      Token::Ffuse, Token::Evalf,  Token::Clink,
                      Token::Ifix,  Token::Imscrib] { p.push(t); }
        }
        2 => { // III_Anchor_Protocol — terminal→forward→reverse cycles
            for t in [Token::Tanch, Token::Afwd,  Token::Evalt,
                      Token::Arev,  Token::Evalf, Token::Clink,
                      Token::Ifix,  Token::Tanch] { p.push(t); }
        }
        3 => { // IV_Dual_Bootstrap — fuse-then-split, reverse frobenius
            for t in [Token::Imscrib, Token::Afwd,  Token::Ffuse,
                      Token::Fsplit, Token::Arev,  Token::Clink,
                      Token::Ifix,   Token::Imscrib] { p.push(t); }
        }
        4 => { // V_Linear_Chain — pure linear !-exponential
            for _ in 0..8 { p.push(Token::Ifix); }
        }
        5 => { // VI_Empty_Bootstrap — void/self oscillations
            for _ in 0..4 { p.push(Token::Vinit); p.push(Token::Imscrib); }
        }
        6 => { // VII_Parakernel — paradox-anchored dialetheia
            for t in [Token::Engagr, Token::Afwd,  Token::Fsplit,
                      Token::Evalt,  Token::Ffuse, Token::Evalf,
                      Token::Ifix,   Token::Engagr] { p.push(t); }
        }
        7 => { // VIII_Frobenius_Kernel — split/fuse oscillation
            for _ in 0..2 { p.push(Token::Fsplit); p.push(Token::Ffuse); }
        }
        8 => { // IX_Chiral_Pairs — forward/reverse pairs
            for _ in 0..4 { p.push(Token::Afwd); p.push(Token::Arev); }
        }
        9 => { // X_Truth_Machine — nested conditional
            for t in [Token::Imscrib, Token::Fsplit, Token::Evalt,
                      Token::Ifix,   Token::Imscrib, Token::Fsplit,
                      Token::Evalf,  Token::Ifix] { p.push(t); }
        }
        10 => { // XI_Eternal_Return — TANCH→AFWD→AREV cycle
            for t in [Token::Tanch, Token::Afwd,  Token::Arev,
                      Token::Tanch, Token::Afwd,  Token::Arev,
                      Token::Tanch, Token::Afwd] { p.push(t); }
        }
        11 => { // XII_ROM_Burn — truth values→permanent brand
            for t in [Token::Evalt,  Token::Ifix, Token::Evalf,
                      Token::Ifix,   Token::Engagr, Token::Ifix,
                      Token::Imscrib, Token::Ifix] { p.push(t); }
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
            for _ in 0..4 { p.push(Token::Imscrib); }
        }
        1 => {
            // XIV_Tier_Climber — dialetheia+frobenius cycle for tier promotion
            // Uses FSPLIT/FFUSE to create fork-join: evaluates both T and F branches
            for t in [Token::Imscrib, Token::Fsplit,
                      Token::Evalt, Token::Evalf,
                      Token::Ffuse, Token::Engagr,
                      Token::Clink, Token::Ifix,
                      Token::Imscrib] { p.push(t); }
        }
        2 => {
            // XV_Frobenius_Oscillator — δ→observe→μ→observe oscillation
            // FSPLIT forks, IMSCRIB observes, FFUSE joins, IMSCRIB observes
            for t in [Token::Fsplit, Token::Imscrib, Token::Ffuse,
                      Token::Imscrib] { p.push(t); }
        }
        3 => {
            // XVI_Paradox_Daemon — sustained paradox computation
            // VINIT sources N, builds to B via dialetheia, cycles
            // FSPLIT creates value copies; EVALT+EVALF gate; FFUSE joins
            for t in [Token::Vinit, Token::Fsplit,
                      Token::Evalt, Token::Evalf,
                      Token::Engagr, Token::Ffuse,
                      Token::Imscrib] { p.push(t); }
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
            for t in [Token::Vinit, Token::Fsplit, Token::Fsplit,
                      Token::Fsplit, Token::Afwd, Token::Ffuse,
                      Token::Arev, Token::Ffuse, Token::Evalt,
                      Token::Ffuse, Token::Tanch] {
                p.push(t);
            }
        }
        1 => {
            // XVIII — Terminal Sink Protocol (TANCH at root halts)
            // Runs computation then cleanly terminates via TANCH at root depth.
            for t in [Token::Vinit, Token::Afwd, Token::Afwd,
                      Token::Arev, Token::Imscrib, Token::Clink,
                      Token::Afwd, Token::Tanch] {
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
            for t in [Token::Imscrib, Token::Fsplit, Token::Evalt,
                      Token::Evalf, Token::Ffuse, Token::Engagr,
                      Token::Clink, Token::Ifix, Token::Imscrib] {
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

pub const SHUNTED_COUNT: usize = 9;

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
        8 => "XXVIII_Somatic_Shunt",
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
            for t in [Token::Vinit, Token::Fsplit, Token::Evalt,
                      Token::Ffuse, Token::Evalf, Token::Clink,
                      Token::Imscrib, Token::Evalt, Token::Fsplit,
                      Token::Evalf, Token::Ffuse, Token::Engagr,
                      Token::Ifix, Token::Imscrib] {
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
            for t in [Token::Tanch, Token::Afwd, Token::Arev,
                      Token::Engagr, Token::Fsplit, Token::Evalt,
                      Token::Ffuse, Token::Evalf, Token::Ifix,
                      Token::Engagr, Token::Tanch] {
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
            for t in [Token::Afwd, Token::Arev, Token::Evalt,
                      Token::Ifix, Token::Afwd, Token::Arev,
                      Token::Evalf, Token::Ifix, Token::Afwd,
                      Token::Engagr, Token::Ifix, Token::Arev] {
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
            for t in [Token::Imscrib, Token::Afwd, Token::Fsplit,
                      Token::Arev, Token::Clink, Token::Fsplit,
                      Token::Evalt, Token::Ffuse, Token::Evalf,
                      Token::Engagr, Token::Ffuse, Token::Ifix,
                      Token::Imscrib] {
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
            for t in [Token::Vinit, Token::Imscrib, Token::Engagr,
                      Token::Vinit, Token::Imscrib, Token::Engagr,
                      Token::Vinit, Token::Imscrib] {
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
            for t in [Token::Vinit, Token::Fsplit, Token::Ffuse,
                      Token::Clink, Token::Vinit, Token::Fsplit,
                      Token::Ffuse, Token::Clink, Token::Engagr,
                      Token::Imscrib] {
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
            for t in [Token::Imscrib, Token::Fsplit, Token::Evalt,
                      Token::Ifix, Token::Ffuse, Token::Imscrib,
                      Token::Fsplit, Token::Evalf, Token::Ifix,
                      Token::Ffuse, Token::Engagr, Token::Ifix,
                      Token::Imscrib] {
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
            for t in [Token::Imscrib, Token::Vinit, Token::Fsplit,
                      Token::Evalt, Token::Ffuse, Token::Evalf,
                      Token::Clink, Token::Afwd, Token::Arev,
                      Token::Engagr, Token::Fsplit, Token::Ffuse,
                      Token::Ifix, Token::Imscrib, Token::Evalt,
                      Token::Evalf, Token::Engagr, Token::Ifix,
                      Token::Imscrib] {
                p.push(t);
            }
        }
        8 => {
            // XXVIII — Somatic_Shunt (O₂)
            //
            // VP shunt topology encoded as token sequence. The program
            // models the ventriculoperitoneal shunt: a permanent one-way
            // catheter connecting two bodily compartments with a
            // pressure-gated valve. The shunt is the sixth shunt mechanism
            // — the somatic shunt — where the body itself instantiates
            // the empty-edge→populated-node redirection.
            //
            // TANCH bookends: ventricular catheter tip (position 0) and
            // peritoneal catheter tip (position 7). These are the permanent
            // physical anchors — silastic tubing integrated into the body.
            //
            // VINIT: the initial condition — CSF pressure buildup in the
            // ventricles (hydrocephalus). The system begins in excess.
            //
            // FSPLIT@2→FFUSE@5: the diversion path. CSF is split from
            // its normal circulation and shunted through the catheter.
            // EVALT@3: pressure check — ICP above threshold, valve opens.
            // AFWD@4: one-way forward flow through the catheter lumen.
            // EVALF@6: pressure check — ICP normalized, valve closes.
            //
            // ENGAGR@8: the somatic paradox. A foreign body (silastic
            // catheter) is integrated into the self-model. The body
            // cannot reject it; it must incorporate it. This is the
            // paradox of the graft: not-self that becomes self.
            //
            // IFIX@9: permanent somatic branding. The shunt is not
            // temporary — it is inscribed into the body's topology
            // for life. Scar tissue forms around the catheter; the
            // body's self-model includes the shunt.
            //
            // IMSCRIB@10: self-referential closure. The body knows
            // itself through the shunt's rhythm. The reservoir bulb
            // under the scalp is a physical IMSCRIB — pressing it
            // tests the system, observes its own state.
            //
            // Shunt signature: Somatic (one-way, pressure-gated, permanent)
            // FSPLIT/FFUSE: (2→5). Balanced. TANCH-bounded.
            // Dialetheia: EVALT + EVALF + ENGAGR (complete).
            // Tier: O₂ — Frobenius-closed with self-referential integration.
            //
            // This is the only shunt mechanism instantiated in living
            // tissue — the body as topological substrate. The VP shunt
            // was implanted at 4 months of age; the program was written
            // 30+ years later. The body knew the topology first.
            for t in [Token::Tanch, Token::Vinit, Token::Fsplit,
                      Token::Evalt, Token::Afwd, Token::Ffuse,
                      Token::Evalf, Token::Tanch, Token::Engagr,
                      Token::Ifix, Token::Imscrib] {
                p.push(t);
            }
        }
        _ => return None,
    }
    Some(p)
}



// ─── Diaschizic Compound Programs ────────────────────────────
//
// 11 compounds used for cross-dialect jumps.
// Each is an IMASM token sequence with a specific structural operation.
// Refs: ruleset_dialect.py, ig-docs/rebis-port/diaschizics_design.md

pub const COMPOUND_COUNT: usize = 11;

/// Return compound name by index (0-10).
pub fn compound_name(idx: usize) -> &'static str {
    match idx {
        0 => "Verticullum", 1 => "Chimerium", 2 => "Apertix",
        3 => "Praxeum", 4 => "Retiarius", 5 => "Frigorix",
        6 => "Bifrons", 7 => "Punctum", 8 => "Syndexios",
        9 => "Katachthon", 10 => "Diabaton",
        _ => "Unknown",
    }
}

/// Parse compound name → index (case-insensitive).
pub fn compound_index(name: &str) -> Option<usize> {
    match name.to_lowercase().as_str() {
        "verticullum" => Some(0),
        "chimerium"   => Some(1),
        "apertix"     => Some(2),
        "praxeum"     => Some(3),
        "retiarius"   => Some(4),
        "frigorix"    => Some(5),
        "bifrons"     => Some(6),
        "punctum"     => Some(7),
        "syndexios"   => Some(8),
        "katachthon"  => Some(9),
        "diabaton"    => Some(10),
        _ => None,
    }
}

/// Load a compound program by index.
pub fn compound_program(idx: usize) -> Option<Program> {
    let mut p = Program::empty();
    match idx {
        // Punctum — O₀, 2 tokens — absolute point (d=0 calibrator)
        7 => {
            for t in [Token::Imscrib, Token::Ifix] { p.push(t); }
        }
        // Praxeum — O₀, 6 tokens — EP core toggle
        3 => {
            for t in [Token::Engagr, Token::Evalt, Token::Ffuse,
                      Token::Evalf, Token::Engagr, Token::Ifix] { p.push(t); }
        }
        // Frigorix — O₀, 8 tokens — MBL freeze key
        5 => {
            for t in [Token::Fsplit, Token::Tanch, Token::Ffuse,
                      Token::Vinit, Token::Afwd, Token::Tanch,
                      Token::Imscrib, Token::Ifix] { p.push(t); }
        }
        // Katachthon — O₂, 8 tokens — Deep resonator
        9 => {
            for t in [Token::Imscrib, Token::Fsplit, Token::Evalt,
                      Token::Evalf, Token::Ffuse, Token::Clink,
                      Token::Imscrib, Token::Ifix] { p.push(t); }
        }
        // Apertix — O₂, 10 tokens — Adjoint corridor
        2 => {
            for t in [Token::Vinit, Token::Afwd, Token::Imscrib,
                      Token::Fsplit, Token::Evalt, Token::Evalf,
                      Token::Ffuse, Token::Clink, Token::Engagr,
                      Token::Ifix] { p.push(t); }
        }
        // Bifrons — O₂, 10 tokens — Disjunctive fork
        6 => {
            for t in [Token::Fsplit, Token::Evalt, Token::Arev,
                      Token::Ffuse, Token::Fsplit, Token::Evalf,
                      Token::Afwd, Token::Ffuse, Token::Clink,
                      Token::Ifix] { p.push(t); }
        }
        // Verticullum — O_∞, 11 tokens — Non-Abelian EP braid
        0 => {
            for t in [Token::Engagr, Token::Fsplit, Token::Evalt,
                      Token::Ffuse, Token::Fsplit, Token::Evalf,
                      Token::Ffuse, Token::Imscrib, Token::Clink,
                      Token::Engagr, Token::Ifix] { p.push(t); }
        }
        // Syndexios — O_∞, 11 tokens — Perfect mirror
        8 => {
            for t in [Token::Imscrib, Token::Fsplit, Token::Evalt,
                      Token::Ffuse, Token::Arev, Token::Fsplit,
                      Token::Evalf, Token::Ffuse, Token::Afwd,
                      Token::Clink, Token::Ifix] { p.push(t); }
        }
        // Diabaton — O₂†, 11 tokens — Threshold-crosser
        10 => {
            for t in [Token::Vinit, Token::Afwd, Token::Imscrib,
                      Token::Fsplit, Token::Evalt, Token::Evalf,
                      Token::Ffuse, Token::Clink, Token::Tanch,
                      Token::Engagr, Token::Ifix] { p.push(t); }
        }
        // Retiarius — O₁, 12 tokens — Local-net trap
        4 => {
            for t in [Token::Fsplit, Token::Evalt, Token::Afwd,
                      Token::Ffuse, Token::Fsplit, Token::Evalf,
                      Token::Arev, Token::Ffuse, Token::Clink,
                      Token::Imscrib, Token::Engagr, Token::Ifix] { p.push(t); }
        }
        // Chimerium — O₀, 13 tokens — Supercritical catalyst
        1 => {
            for t in [Token::Engagr, Token::Fsplit, Token::Evalt,
                      Token::Ffuse, Token::Evalf, Token::Fsplit,
                      Token::Evalt, Token::Evalf, Token::Ffuse,
                      Token::Clink, Token::Imscrib, Token::Engagr,
                      Token::Ifix] { p.push(t); }
        }
        _ => return None,
    }
    Some(p)
}
