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

    /// Truth-functional conjunction — the bilattice's *other* axis.
    /// Mirrors `Belnap.lean` `band` and `belnap.py` `band`: F absorbs,
    /// B absorbs N. Distinct from `band` above, which is the knowledge
    /// consensus: T ∧ F = F here, where T ⊗ F = B there.
    pub fn truth_and(self, other: B4) -> B4 {
        use B4::*;
        match (self, other) {
            (F, _) | (_, F) => F,
            (B, T) | (T, B) | (B, N) | (N, B) => B,
            (T, T) => T,
            (T, N) | (N, T) => N,
            (N, N) => N,
            (B, B) => B,
        }
    }

    /// Truth-functional disjunction. Mirrors `Belnap.lean` `bor`:
    /// T absorbs, B absorbs N. T ∨ F = T here, where T ⊕ F = N for `bor`.
    pub fn truth_or(self, other: B4) -> B4 {
        use B4::*;
        match (self, other) {
            (T, _) | (_, T) => T,
            (B, F) | (F, B) | (B, N) | (N, B) => B,
            (F, F) => F,
            (F, N) | (N, F) => N,
            (N, N) => N,
            (B, B) => B,
        }
    }

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
    /// x ≤k y iff x ⊓k y = x, and the knowledge glb is `meet`.
    pub fn approx_le(self, other: B4) -> bool {
        self.meet(other) == self
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

// ── The Boolean core's two adjoints, and the closure they can't reach ──
//
// UNO_Reverse.tex: the inclusion of the Boolean core B_4={F,T} into FOUR
// has a left adjoint (reflector r) and a right adjoint (coreflector c),
// not the same map -- checked directly against the manuscript's own
// theorems, not read off a summary: line ~419 "r(F)=F, r(N)=r(B)=T" (the
// truth-order reflection theorem, proved by cases at F and T); the
// coreflection theorem gives c(N)=c(B)=F by the dual argument, c(T)=T.
// `Inc(v)=B` is the paraconsistent closure: constant, image {B}, a fixed
// point (`Inc(B)=B`) outside the Boolean core, idempotent.
//
// Naming note: the manuscript's "B_4" names the two-element classical
// SUBSET {F,T} of FOUR. This file's `B4` type is the full four-valued
// carrier {N,T,F,B}. Same three letters, two different objects -- do not
// conflate this type with the paper's classical core when reading the
// functions below.

/// The reflector: the least Boolean value above v in the truth order.
/// Sends every nonclassical value (N, B) up to T; T and F are fixed.
pub fn r(v: B4) -> B4 {
    if v == B4::F { B4::F } else { B4::T }
}

/// The coreflector: the greatest Boolean value below v in the truth
/// order. Sends every nonclassical value (N, B) down to F; T and F are
/// fixed. The same two inputs (N, B) that `r` sends to T, `c` sends to
/// F -- the manuscript's "erase the same nonclassical values in opposite
/// directions" theorem, checked below in `theorem_5_5_report`.
pub fn c(v: B4) -> B4 {
    if v == B4::T { B4::T } else { B4::F }
}

/// The paraconsistent closure: constant at B, for every input. Not a
/// retraction into the Boolean core -- its one output value sits outside
/// {T, F} entirely, which is the whole content of Corollary 11.2 below:
/// no Boolean endomap (certainly not r or c) can undo this constancy and
/// recover what v was.
pub fn inc(_v: B4) -> B4 {
    B4::B
}

/// Corollary 11.2 (UNO_Reverse.tex, the corollary right after the
/// retraction-square figure): r∘Inc and c∘Inc are constant maps, T and F
/// respectively, and neither recovers Inc(v)=B. Checked exhaustively over
/// all four inputs -- only four to check, so "exhaustive" here is a real
/// claim, not a sampling one.
pub fn corollary_11_2_report() -> alloc::string::String {
    use alloc::format;
    use alloc::string::String;
    let mut out = String::new();
    let mut r_inc_constant = true;
    let mut c_inc_constant = true;
    for v in [B4::N, B4::T, B4::F, B4::B] {
        let ri = r(inc(v));
        let ci = c(inc(v));
        if ri != B4::T { r_inc_constant = false; }
        if ci != B4::F { c_inc_constant = false; }
        out.push_str(&format!(
            "  v={:<2} Inc(v)={:<2} r(Inc(v))={:<2} c(Inc(v))={:<2}\n",
            v.name(), inc(v).name(), ri.name(), ci.name()
        ));
    }
    out.push_str(&format!(
        "  r∘Inc constant at T over all 4 inputs: {}\n",
        r_inc_constant
    ));
    out.push_str(&format!(
        "  c∘Inc constant at F over all 4 inputs: {}\n",
        c_inc_constant
    ));
    out.push_str("  neither equals Inc(v)=B for any v: true by inspection (T≠B, F≠B)\n");
    out
}

/// Theorem 5.5 in the YZ-retranslation document's numbering (the paper's
/// own "the two universal collapses are distinct" theorem in
/// UNO_Reverse.tex): r and c send the same two nonclassical inputs to
/// opposite classical outputs, and agree on the two classical inputs.
pub fn theorem_5_5_report() -> alloc::string::String {
    use alloc::format;
    use alloc::string::String;
    let mut out = String::new();
    let mut opposite_on_nonclassical = true;
    let mut agree_on_classical = true;
    for v in [B4::N, B4::T, B4::F, B4::B] {
        let rv = r(v);
        let cv = c(v);
        let nonclassical = matches!(v, B4::N | B4::B);
        if nonclassical && rv == cv { opposite_on_nonclassical = false; }
        if !nonclassical && rv != cv { agree_on_classical = false; }
        out.push_str(&format!("  v={:<2} r(v)={:<2} c(v)={:<2}\n", v.name(), rv.name(), cv.name()));
    }
    out.push_str(&format!(
        "  r and c disagree on both nonclassical inputs (N, B): {}\n",
        opposite_on_nonclassical
    ));
    out.push_str(&format!(
        "  r and c agree on both classical inputs (T, F): {}\n",
        agree_on_classical
    ));
    out
}

// Convenience aliases matching Python b4_* conventions.
pub type Belnap = B4;

pub fn meet(a: B4, b: B4) -> B4 { a.meet(b) }
pub fn join(a: B4, b: B4) -> B4 { a.join(b) }
pub fn band(a: B4, b: B4) -> B4 { a.band(b) }
pub fn bor(a: B4, b: B4) -> B4 { a.bor(b) }
pub fn truth_and(a: B4, b: B4) -> B4 { a.truth_and(b) }
pub fn truth_or(a: B4, b: B4) -> B4 { a.truth_or(b) }
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

    pub fn clear(&mut self) { self.top = 0; }

    /// ROTAT — cyclic shift of the stack by k positions (mod depth).
    /// The element at position i moves to position (i + k) % depth.
    pub fn rotate(&mut self, k: usize) {
        let n = self.top;
        if n <= 1 { return; }
        let k = k % n;
        if k == 0 { return; }
        // Cyclic shift right by k: reverse whole, reverse first k, reverse rest
        self.data[..n].reverse();
        self.data[..k].reverse();
        self.data[k..n].reverse();
    }

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

    pub fn clear(&mut self) {
        self.regs = [B4::N; 8];
        self.engagr = false;
    }
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
    // The two axes are different operations, and nothing above would catch
    // one being reached for in place of the other. T ∧ F = F and T ∨ F = T
    // on the truth axis, against T ⊗ F = B and T ⊕ F = N on the knowledge one.
    assert_eq!(T.truth_and(F), F);
    assert_eq!(T.truth_or(F), T);
    assert_ne!(T.truth_and(F), T.band(F));
    assert_ne!(T.truth_or(F), T.bor(F));
    // B absorbs N on both truth operations; F and T absorb respectively.
    assert_eq!(B.truth_and(N), B);
    assert_eq!(B.truth_or(N), B);
    for &x in &[N, T, F, B] {
        assert_eq!(F.truth_and(x), F);
        assert_eq!(T.truth_or(x), T);
        assert_eq!(x.truth_and(x), x);
        assert_eq!(x.truth_or(x), x);
    }
}
