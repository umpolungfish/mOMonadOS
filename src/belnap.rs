#![allow(dead_code)]
/// Belnap FOUR truth values.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum B4 {
    N = 0, // None  — void, absence
    T = 1, // True  — affirmation
    F = 2, // False — negation
    B = 3, // Both  — paradox stabilized
}

impl B4 {
    pub fn name(self) -> &'static str {
        match self { B4::N => "N", B4::T => "T", B4::F => "F", B4::B => "B" }
    }

    pub fn from_u8(v: u8) -> Self {
        match v & 0b11 { 1 => B4::T, 2 => B4::F, 3 => B4::B, _ => B4::N }
    }

    pub fn to_u8(self) -> u8 { self as u8 }

    /// Truth-order meet (greatest lower bound): bitwise AND.
    pub fn meet(self, other: B4) -> B4 {
        B4::from_u8(self as u8 & other as u8)
    }

    /// Truth-order join (least upper bound): bitwise OR.
    pub fn join(self, other: B4) -> B4 {
        B4::from_u8(self as u8 | other as u8)
    }

    /// Knowledge-order consensus (lub in ≤k): bitwise OR = join.
    /// T ⊗ F = B (both affirmations → paradox).
    pub fn band(self, other: B4) -> B4 { self.join(other) }

    /// Knowledge-order gullibility (glb in ≤k): bitwise AND = meet.
    /// T ⊕ F = N (no shared ground).
    pub fn bor(self, other: B4) -> B4 { self.meet(other) }

    /// Negation: swap T↔F, preserve N and B.
    pub fn bnot(self) -> B4 {
        let b = self as u8;
        B4::from_u8(((b & 1) << 1) | ((b & 2) >> 1))
    }

    /// Is this value paradox-stabilized (Both)?
    pub fn dialetheic(self) -> bool { self == B4::B }

    /// Is this value designated (T or B) in Belnap logic?
    pub fn designated(self) -> bool { matches!(self, B4::T | B4::B) }

    /// Knowledge-order comparison: self ≤k other.
    /// x ≤k y iff x ⊕ y = x (bor returns x).
    pub fn approx_le(self, other: B4) -> bool {
        self.bor(other) == self
    }

    /// Encode to WH2 pair: (is_true, is_false).
    pub fn to_wh2(self) -> (bool, bool) {
        let b = self as u8;
        ((b & 1) != 0, (b & 2) != 0)
    }

    /// Decode from WH2 pair.
    pub fn from_wh2(t: bool, f: bool) -> Self {
        B4::from_u8((t as u8) | ((f as u8) << 1))
    }
}

// Convenience aliases matching Python b4_* conventions.
pub type Belnap = B4;

pub fn meet(a: B4, b: B4) -> B4 { a.meet(b) }
pub fn join(a: B4, b: B4) -> B4 { a.join(b) }
pub fn band(a: B4, b: B4) -> B4 { a.band(b) }
pub fn bor(a: B4, b: B4) -> B4 { a.bor(b) }
pub fn bnot(a: B4) -> B4 { a.bnot() }
pub fn dialetheic(a: B4) -> bool { a.dialetheic() }
pub fn designated(a: B4) -> bool { a.designated() }
pub fn approx_le(a: B4, b: B4) -> bool { a.approx_le(b) }
pub fn to_wh2(a: B4) -> (bool, bool) { a.to_wh2() }
pub fn from_wh2(t: bool, f: bool) -> B4 { B4::from_wh2(t, f) }

// Legacy aliases for existing kernel code.
pub fn b4_meet(a: B4, b: B4) -> B4 { a.meet(b) }
pub fn b4_join(a: B4, b: B4) -> B4 { a.join(b) }

/// 4096-cell B4 memory, 2 bits per cell packed into a byte array.
pub struct B4Memory {
    data: [u8; 1024], // 4096 cells × 2 bits = 1024 bytes
}

impl B4Memory {
    pub const fn new() -> Self {
        Self { data: [0u8; 1024] }
    }

    pub fn read(&self, addr: usize) -> B4 {
        let addr = addr & 0xFFF;
        let byte = self.data[addr / 4];
        let shift = (addr % 4) * 2;
        B4::from_u8((byte >> shift) & 0b11)
    }

    pub fn write(&mut self, addr: usize, val: B4) {
        let addr = addr & 0xFFF;
        let shift = (addr % 4) * 2;
        let mask = !(0b11u8 << shift);
        self.data[addr / 4] = (self.data[addr / 4] & mask) | ((val as u8) << shift);
    }
}

/// 256-deep B4 stack.
pub struct B4Stack {
    data: [B4; 256],
    top: usize,
}

impl B4Stack {
    pub const fn new() -> Self {
        Self { data: [B4::N; 256], top: 0 }
    }

    pub fn push(&mut self, v: B4) {
        if self.top < 256 {
            self.data[self.top] = v;
            self.top += 1;
        }
    }

    pub fn pop(&mut self) -> B4 {
        if self.top == 0 { return B4::N; }
        self.top -= 1;
        self.data[self.top]
    }

    pub fn peek(&self) -> B4 {
        if self.top == 0 { B4::N } else { self.data[self.top - 1] }
    }

    pub fn peek_at(&self, offset: usize) -> B4 {
        if offset >= self.top { B4::N } else { self.data[offset] }
    }

    pub fn depth(&self) -> usize { self.top }
}

/// 8 × B4 register file.
pub struct B4Registers {
    regs: [B4; 8],
    pub engagr: bool,
}

impl B4Registers {
    pub const fn new() -> Self {
        Self { regs: [B4::N; 8], engagr: false }
    }

    pub fn read(&self, i: usize) -> B4 { self.regs[i & 7] }

    pub fn write(&mut self, i: usize, v: B4) { self.regs[i & 7] = v; }
}

// ── Module-level self-verification ──────────────────────────────
#[test]
fn belnap_invariants() {
    use B4::*;
    // Frobenius: join(B, x) = B  ∀x (B absorbs join)
    for &x in &[N, T, F, B] { assert_eq!(B.join(x), B); }
    // B meet = identity
    for &x in &[N, T, F, B] { assert_eq!(B.meet(x), x); }
    // B fixed-point negation
    assert_eq!(B.bnot(), B);
    // N fixed-point negation
    assert_eq!(N.bnot(), N);
    // T↔F swap
    assert_eq!(T.bnot(), F);
    assert_eq!(F.bnot(), T);
    // bnot(bnot(x)) = x
    for &x in &[N, T, F, B] { assert_eq!(x.bnot().bnot(), x); }
    // dialetheic: only B
    assert!(B.dialetheic());
    assert!(!N.dialetheic() && !T.dialetheic() && !F.dialetheic());
    // designated: T and B
    assert!(T.designated() && B.designated());
    assert!(!N.designated() && !F.designated());
    // WH2 round-trip
    for &x in &[N, T, F, B] {
        let (t, f) = x.to_wh2();
        assert_eq!(B4::from_wh2(t, f), x);
    }
    // approx_le: N ≤k everything, everything ≤k B
    for &x in &[N, T, F, B] {
        assert!(N.approx_le(x));
        assert!(x.approx_le(B));
    }
    // band: T ⊗ F = B, N ⊗ x = x
    assert_eq!(T.band(F), B);
    assert_eq!(N.band(T), T);
    // bor: T ⊕ F = N, B ⊕ x = x
    assert_eq!(T.bor(F), N);
    assert_eq!(B.bor(T), T);
}
