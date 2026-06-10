/// The 12 IMASM opcodes — categorical duals of the 12 IG primitives.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Token {
    VINIT  = 0x0, // Initial object ∅ — void
    TANCH  = 0x1, // Terminal anchor ⊤ — boundary
    AFWD   = 0x2, // Forward morphism → — directed transition
    AREV   = 0x3, // Contravariant inversion ← — reversal
    CLINK  = 0x4, // Composition ∘ — linkage
    ISCRIB = 0x5, // Identity id — self-imscription
    FSPLIT = 0x6, // Co-multiplication δ — bifurcation
    FFUSE  = 0x7, // Multiplication μ — recombination
    EVALT  = 0x8, // True — affirmation
    EVALF  = 0x9, // False — negation
    ENGAGR = 0xA, // Both — paradox stabilized
    IFIX   = 0xB, // Permanent brand — linear ! exponential
}

impl Token {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Token::VINIT),  1 => Some(Token::TANCH),
            2 => Some(Token::AFWD),   3 => Some(Token::AREV),
            4 => Some(Token::CLINK),  5 => Some(Token::ISCRIB),
            6 => Some(Token::FSPLIT), 7 => Some(Token::FFUSE),
            8 => Some(Token::EVALT),  9 => Some(Token::EVALF),
            10 => Some(Token::ENGAGR), 11 => Some(Token::IFIX),
            _ => None,
        }
    }

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

    pub fn is_empty(&self) -> bool { self.len == 0 }

    pub fn as_slice(&self) -> &[Token] { &self.buf[..self.len] }

    pub fn inject(&mut self, pos: usize, t: Token) {
        if self.len >= 64 { return; }
        let pos = pos.min(self.len);
        // shift right
        let mut i = self.len;
        while i > pos { self.buf[i] = self.buf[i - 1]; i -= 1; }
        self.buf[pos] = t;
        self.len += 1;
    }
}

// ─── Bootstrap + 12 Canonicals ───────────────────────────────────

pub fn bootstrap_loop() -> Program {
    let mut p = Program::empty();
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
        0 => { // I_Dialetheic_Bootstrap
            for t in [Token::ISCRIB, Token::EVALT, Token::FSPLIT,
                      Token::EVALF, Token::FFUSE, Token::ENGAGR,
                      Token::IFIX,  Token::ISCRIB] { p.push(t); }
        }
        1 => { // II_Void_Genesis
            for t in [Token::VINIT, Token::FSPLIT, Token::EVALT,
                      Token::FFUSE, Token::EVALF,  Token::CLINK,
                      Token::IFIX,  Token::ISCRIB] { p.push(t); }
        }
        2 => { // III_Anchor_Protocol
            for t in [Token::TANCH, Token::AFWD,  Token::EVALT,
                      Token::AREV,  Token::EVALF, Token::CLINK,
                      Token::IFIX,  Token::TANCH] { p.push(t); }
        }
        3 => { // IV_Dual_Bootstrap
            for t in [Token::ISCRIB, Token::AFWD,  Token::FFUSE,
                      Token::FSPLIT, Token::AREV,  Token::CLINK,
                      Token::IFIX,   Token::ISCRIB] { p.push(t); }
        }
        4 => { // V_Linear_Chain
            for _ in 0..8 { p.push(Token::IFIX); }
        }
        5 => { // VI_Empty_Bootstrap
            for _ in 0..4 { p.push(Token::VINIT); p.push(Token::ISCRIB); }
        }
        6 => { // VII_Parakernel
            for t in [Token::ENGAGR, Token::AFWD,  Token::FSPLIT,
                      Token::EVALT,  Token::FFUSE, Token::EVALF,
                      Token::IFIX,   Token::ENGAGR] { p.push(t); }
        }
        7 => { // VIII_Frobenius_Kernel
            for _ in 0..2 { p.push(Token::FSPLIT); p.push(Token::FFUSE); }
        }
        8 => { // IX_Chiral_Pairs
            for _ in 0..4 { p.push(Token::AFWD); p.push(Token::AREV); }
        }
        9 => { // X_Truth_Machine
            for t in [Token::ISCRIB, Token::FSPLIT, Token::EVALT,
                      Token::IFIX,   Token::ISCRIB, Token::FSPLIT,
                      Token::EVALF,  Token::IFIX] { p.push(t); }
        }
        10 => { // XI_Eternal_Return
            for t in [Token::TANCH, Token::AFWD,  Token::AREV,
                      Token::TANCH, Token::AFWD,  Token::AREV,
                      Token::TANCH, Token::AFWD] { p.push(t); }
        }
        11 => { // XII_ROM_Burn
            for t in [Token::EVALT,  Token::IFIX, Token::EVALF,
                      Token::IFIX,   Token::ENGAGR, Token::IFIX,
                      Token::ISCRIB, Token::IFIX] { p.push(t); }
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

/// Minimal period of the program (smallest p such that prog is periodic with period p).
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
