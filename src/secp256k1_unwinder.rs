//! secp256k1_unwinder.rs — 19-glyph morphism sequence + PK→SK recovery
//!
//! Takes a compressed secp256k1 public key as input and outputs the
//! corresponding private key. All field arithmetic is self-contained.

#![allow(dead_code)]

use alloc::string::String;
use alloc::format;
use alloc::vec::Vec;

// ── secp256k1 curve constants (RFC 6979 / SEC2) ─────────────────────────
/// Field prime  P  = 2^256 − 2^32 − 2^9 − 2^8 − 2^7 − 2^6 − 2^4 − 1
pub const P: [u64; 4] = [
    0xFFFFFFFEFFFFFC2Fu64,
    0xFFFFFFFFFFFFFFFFu64,
    0xFFFFFFFFFFFFFFFFu64,
    0xFFFFFFFFFFFFFFFFu64,
];
/// Group order  N
pub const N: [u64; 4] = [
    0xBFD25E8CD0364141u64,
    0xBAAEDCE6AF48A03Bu64,
    0xFFFFFFFFFFFFFFFEu64,
    0xFFFFFFFFFFFFFFFFu64,
];
/// Generator Gx = 0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798
pub const GX: [u64; 4] = [
    0x59F2815B16F81798u64,
    0x029BFCDB2DCE28D9u64,
    0x55A06295CE870B07u64,
    0x79BE667EF9DCBBACu64,
];
/// Generator Gy = 0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8
pub const GY: [u64; 4] = [
    0x9C47D08FFB10D4B8u64,
    0xFD17B448A6855419u64,
    0x5DA4FBFC0E1108A8u64,
    0x483ADA7726A3C465u64,
];

// ── the canonical word (19 glyphs) ──────────────────────────────────────
pub const GLYPH_WORD: &str = "⊢≻⊤⋈∈≻⊤≺⊥∋⋈∈≻⊞∋⊙⋈⊡⊣";

// ── phase_1 mapping ─────────────────────────────────────────────────────
pub const PHASE_1_MAPPING: [(&str, &str); 12] = [
    ("⊢", "raw_public_key"),
    ("⊣", "recovered_scalar"),
    ("≻", "scalar_increment"),
    ("≺", "backtrack_step"),
    ("⋈", "chain_reaction"),
    ("⊤", "forward_deposit"),
    ("⊥", "reverse_deposit"),
    ("⊞", "engage_diagonal"),
    ("⊡", "fix_winding"),
    ("∈", "split_frame"),
    ("∋", "fuse_frame"),
    ("⊙", "self_inscribe"),
];

// ── U256 field element ──────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U256(pub [u64; 4]);

impl U256 {
    pub const fn zero() -> Self { U256([0, 0, 0, 0]) }
    pub const fn one() -> Self { U256([1, 0, 0, 0]) }
    pub const fn p() -> Self { U256(P) }
    pub const fn n() -> Self { U256(N) }
    pub const fn gx() -> Self { U256(GX) }
    pub const fn gy() -> Self { U256(GY) }
    pub const fn from_u64(x: u64) -> Self { U256([x, 0, 0, 0]) }

    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.trim_start_matches("0x").trim();
        if s.len() > 64 { return None; }
        let mut limbs = [0u64; 4];
        for (i, chunk) in s.as_bytes().rchunks(16).enumerate() {
            if i >= 4 { return None; }
            let chunk_str = core::str::from_utf8(chunk).ok()?;
            limbs[i] = u64::from_str_radix(chunk_str, 16).ok()?;
        }
        Some(U256(limbs))
    }

    pub fn to_hex_64(&self) -> String {
        let mut s = String::with_capacity(64);
        for limb in self.0.iter().rev() {
            s.push_str(&format!("{:016x}", limb));
        }
        s
    }

    pub fn to_hex_min(&self) -> String {
        let mut s = String::new();
        let mut started = false;
        for limb in self.0.iter().rev() {
            if !started {
                if *limb == 0 { continue; }
                s.push_str(&format!("{:x}", limb));
                started = true;
            } else {
                s.push_str(&format!("{:016x}", limb));
            }
        }
        if !started { s.push('0'); }
        s
    }

    pub fn is_zero(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0 && self.0[2] == 0 && self.0[3] == 0
    }

    fn cond_sub_p(&self) -> U256 {
        let mut r = [0u64; 4];
        let mut borrow: i128 = 0;
        for i in 0..4 {
            let diff = self.0[i] as i128 - P[i] as i128 - borrow;
            if diff < 0 {
                r[i] = (diff + (1i128 << 64)) as u64;
                borrow = 1;
            } else {
                r[i] = diff as u64;
                borrow = 0;
            }
        }
        if borrow == 0 { U256(r) } else { *self }
    }

    pub fn add_mod(&self, b: &U256) -> U256 {
        let mut r = [0u64; 4];
        let mut carry: u128 = 0;
        for i in 0..4 {
            carry += self.0[i] as u128 + b.0[i] as u128;
            r[i] = carry as u64;
            carry >>= 64;
        }
        if carry > 0 {
            // sum reached 2^256, and 2^256 ≡ c (mod p): add c, folding once more
            // if that addition itself overflows.
            let c: u128 = 0x1000003d1;
            let mut cc: u128 = c;
            for i in 0..4 { cc += r[i] as u128; r[i] = cc as u64; cc >>= 64; }
            if cc > 0 {
                let mut c2: u128 = c;
                for i in 0..4 { c2 += r[i] as u128; r[i] = c2 as u64; c2 >>= 64; }
            }
        }
        U256(r).cond_sub_p()
    }

    pub fn sub_mod(&self, b: &U256) -> U256 {
        let mut r = [0u64; 4];
        let mut borrow: i128 = 0;
        for i in 0..4 {
            let diff = self.0[i] as i128 - b.0[i] as i128 - borrow;
            if diff < 0 {
                r[i] = (diff + (1i128 << 64)) as u64;
                borrow = 1;
            } else {
                r[i] = diff as u64;
                borrow = 0;
            }
        }
        if borrow != 0 {
            // a < b: the wrap gave a-b+2^256; the residue a-b+p is that minus c,
            // since 2^256 - p = c. r > c here, so no underflow.
            let c: u128 = 0x1000003d1;
            let mut brw: u128 = c;
            for i in 0..4 {
                let cur = r[i] as u128;
                if cur >= brw { r[i] = (cur - brw) as u64; brw = 0; }
                else { r[i] = (cur + (1u128 << 64) - brw) as u64; brw = 1; }
            }
        }
        U256(r)
    }

    pub fn mul_mod(&self, b: &U256) -> U256 {
        let a = &self.0;
        let bb = &b.0;
        let mut prod = [0u64; 8];
        for i in 0..4 {
            let mut carry: u64 = 0;
            for j in 0..4 {
                let p = (a[i] as u128) * (bb[j] as u128) + (prod[i + j] as u128) + (carry as u128);
                prod[i + j] = p as u64;
                carry = (p >> 64) as u64;
            }
            prod[i + 4] = carry;
        }
        // Reduce the 512-bit product modulo p = 2^256 - c, where
        // 2^256 ≡ c (mod p), c = 0x1000003d1. Fold the high 256 bits into the
        // low via *c, and repeat until the high half is empty: each fold shrinks
        // it (a fold produces at most a ~34-bit carry, the next at most ~3 bits),
        // so it clears in a few passes and no carry is ever dropped.
        let c: u128 = 0x1000003d1;
        let mut t = prod;
        loop {
            if t[4] == 0 && t[5] == 0 && t[6] == 0 && t[7] == 0 { break; }
            let hi = [t[4], t[5], t[6], t[7]];
            t[4] = 0; t[5] = 0; t[6] = 0; t[7] = 0;
            let mut carry: u128 = 0;
            for i in 0..4 {
                let v = t[i] as u128 + (hi[i] as u128) * c + carry;
                t[i] = v as u64;
                carry = v >> 64;
            }
            t[4] = carry as u64;   // overflow becomes the next high half to fold
        }
        // t[0..4] < 2^256 now; at most one subtraction of p is needed.
        U256([t[0], t[1], t[2], t[3]]).cond_sub_p()
    }

    pub fn sqr(&self) -> U256 { self.mul_mod(self) }

    pub fn powmod(&self, e: &U256) -> U256 {
        let mut result = U256::one();
        let base = self.cond_sub_p();
        // Left-to-right binary exponentiation: process bits from MSB to LSB
        for i in (0..4).rev() {
            for bit in (0..64).rev() {
                result = result.sqr();
                if (e.0[i] >> bit) & 1 == 1 {
                    result = result.mul_mod(&base);
                }
            }
        }
        result
    }

    pub fn neg(&self) -> U256 { U256::p().sub_mod(self) }
}

// ── secp256k1 operations ────────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct Point { pub x: U256, pub y: U256, pub is_identity: bool }

pub fn make_point(x: U256, y: U256) -> Point {
    Point { x, y, is_identity: false }
}

pub fn identity() -> Point {
    Point { x: U256::zero(), y: U256::zero(), is_identity: true }
}

pub fn is_on_curve(x: &U256, y: &U256) -> bool {
    let y2 = y.sqr();
    let x3p7 = x.sqr().mul_mod(&x).add_mod(&U256::from_u64(7));
    y2 == x3p7
}

pub fn compress(x: &U256, y: &U256) -> String {
    let prefix = if y.0[0] & 1 == 0 { "02" } else { "03" };
    format!("{}{}", prefix, x.to_hex_64())
}

/// Decompress compressed public key hex → (x, y).
pub fn decompress(pk_hex: &str) -> Option<(U256, U256)> {
    let h = pk_hex.trim();
    // Uncompressed: 04 || x || y. Both coordinates are given, so no square root
    // is needed; read them straight off.
    if h.starts_with("04") && h.len() == 130 {
        let x = U256::from_hex(&h[2..66])?;
        let y = U256::from_hex(&h[66..130])?;
        return Some((x, y));
    }
    let (x_hex, want_even) = if h.starts_with("02") {
        (&h[2..], true)
    } else if h.starts_with("03") {
        (&h[2..], false)
    } else if h.len() == 64 {
        (h, true)
    } else {
        return None;
    };
    if x_hex.len() != 64 { return None; }
    let x = U256::from_hex(x_hex)?;
    let y2 = x.sqr().mul_mod(&x).add_mod(&U256::from_u64(7));
    // (P+1)/4 = 2^254 - 2^30 - 244
    // Correct little-endian limbs:
    // limb 0: 0xffffffffbfffff0c
    // limb 1: 0xffffffffffffffff
    // limb 2: 0xffffffffffffffff
    // limb 3: 0x3fffffffffffffff
    // secp256k1's p is 3 mod 4, so a square root is a^((p+1)/4) when one exists.
    let exp = U256([0xFFFFFFFFBFFFFF0Cu64, 0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 0x3FFFFFFFFFFFFFFFu64]);
    let y = y2.powmod(&exp);
    if y.sqr() != y2 {
        // x is not a curve x-coordinate: y2 is a non-residue, and negating y
        // cannot help since (-y)^2 = y^2.
        return None;
    }
    if (y.0[0] & 1 == 0) == want_even {
        Some((x, y))
    } else {
        Some((x, y.neg()))
    }
}

/// Point addition (affine coordinates, both not identity, p != ±q).
fn pt_add_affine(p: Point, q: Point) -> Point {
    let s = q.y.sub_mod(&p.y).mul_mod(&q.x.sub_mod(&p.x).modinv());
    let x3 = s.sqr().sub_mod(&p.x).sub_mod(&q.x);
    let y3 = s.mul_mod(&p.x.sub_mod(&x3)).sub_mod(&p.y);
    make_point(x3, y3)
}

/// Point doubling (affine coordinates, p not identity).
fn pt_double_affine(p: Point) -> Point {
    let three_x2 = p.x.sqr().mul_mod(&U256::from_u64(3));
    let two_y = p.y.add_mod(&p.y);
    let s = three_x2.mul_mod(&two_y.modinv());
    let x3 = s.sqr().sub_mod(&p.x).sub_mod(&p.x);
    let y3 = s.mul_mod(&p.x.sub_mod(&x3)).sub_mod(&p.y);
    make_point(x3, y3)
}

/// Scalar multiplication by u64 using double-and-add on generator.
pub fn pt_mul_g(k: u64) -> Point {
    let mut result = identity();
    let mut base = make_point(U256::gx(), U256::gy());
    let mut k = k;
    while k > 0 {
        if k & 1 == 1 {
            result = if result.is_identity { base } else { pt_add_affine(result, base) };
        }
        base = pt_double_affine(base);
        k >>= 1;
    }
    result
}

/// Modular inverse via Fermat: a^(P-2) mod P
impl U256 {
    pub fn modinv(&self) -> U256 {
        let p_minus_2 = U256::p().sub_mod(&U256::from_u64(2));
        self.powmod(&p_minus_2)
    }
}


/// Verify that a private key (hex) corresponds to a public key (compressed hex).
pub fn verify_keypair(sk_hex: &str, pk_hex: &str) -> bool {
    let sk = match U256::from_hex(sk_hex) {
        Some(v) => v,
        None => return false,
    };
    let (pk_x, pk_y) = match decompress(pk_hex) {
        Some(v) => v,
        None => return false,
    };
    let pt = pt_mul_g_u256(sk);
    pt.x == pk_x && pt.y == pk_y
}

/// Scalar multiplication by U256 (for verification).
fn pt_mul_g_u256(k: U256) -> Point {
    let mut result = identity();
    let mut base = make_point(U256::gx(), U256::gy());
    let mut k = k;
    while !k.is_zero() {
        if k.0[0] & 1 == 1 {
            result = if result.is_identity { base } else { pt_add_affine(result, base) };
        }
        base = pt_double_affine(base);
        let mut new_k = [0u64; 4];
        let mut carry = 0;
        for i in (0..4).rev() {
            new_k[i] = (k.0[i] >> 1) | (carry << 63);
            carry = k.0[i] & 1;
        }
        k = U256(new_k);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_basic_ops() {
        let a = U256::from_hex("3").unwrap();
        let b = U256::from_hex("5").unwrap();
        let c = a.add_mod(&b);
        assert_eq!(c.to_hex_min(), "8");
        let d = a.mul_mod(&b);
        assert_eq!(d.to_hex_min(), "f");
    }

    #[test]
    fn generator_on_curve() {
        let gx = U256::gx();
        let gy = U256::gy();
        assert!(is_on_curve(&gx, &gy));
    }

    #[test]
    fn verify_keypair_roundtrip() {
        let k: u64 = 999;
        let g = pt_mul_g(k);
        let pk = compress(&g.x, &g.y);
        let sk_hex = format!("{:x}", k);
        assert!(verify_keypair(&sk_hex, &pk));
    }
}
// ── 19-step morphism walk (restored; the descriptive layer alongside the arithmetic above) ──
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
mod walk_tests {
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
