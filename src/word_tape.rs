//! WordTape — the numeral whose value IS its IMASM glyph word.
//!
//! `native_numeral::encode` writes n as ⊢ [≻⋈∈[⊤|⊥]∋]*(LSB first) ⊙⊡⊣ — one
//! closed parity branch per bit. `WordTape` carries that word verbatim, and
//! every arithmetic operation walks the word's own cells: read each cell's
//! parity arm (⊤ even, ⊥ odd), ripple the carry/borrow across cells, and
//! write cells back. No `BigUint` and no u64 limb vector appears inside any
//! operation — the glyph word is the only value. The bits an operation
//! reads are the cells' parity arms, not a conversion to another numeral
//! system; `from_bits` is the exact inverse walk that writes arms back.
//!
//! Layout (byte-identical to native_numeral::encode):
//!   ⊢            VINIT      open the value
//!   ≻⋈∈[⊤|⊥]∋   one bit    advance, chain, parity-branch, deposit, rejoin
//!   ...                      bits least-significant first
//!   ⊙⊡⊣         IMSCRIB/IFIX/TANCH  recognize, fix, close
//!   zero = ⊢⊙⊡⊣

use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::BigUint;
use num_traits::{One, Zero};

const VINIT: char = '⊢';
const AFWD: char = '≻';
const CLINK: char = '⋈';
const FSPLIT: char = '∈';
const EVALT: char = '⊤';
const EVALF: char = '⊥';
const FFUSE: char = '∋';
const IMSCRIB: char = '⊙';
const IFIX: char = '⊡';
const TANCH: char = '⊣';

/// A numeral held as its own IMASM word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WordTape(pub(crate) String);

// ── entry/exit: the only two places BigUint appears ──────────────────────
fn biguint_to_bits(n: &BigUint) -> Vec<bool> {
    let mut v = Vec::new();
    let mut m = n.clone();
    let two = BigUint::from(2u32);
    while !m.is_zero() {
        let q = &m / &two;
        let r = &m % &two;
        v.push(!r.is_zero());
        m = q;
    }
    v
}

fn bits_to_biguint(bits: &[bool]) -> BigUint {
    let mut n = BigUint::zero();
    let mut p = BigUint::one();
    let two = BigUint::from(2u32);
    for &b in bits {
        if b { n += &p; }
        p *= &two;
    }
    n
}

// ── cell walks: read the word's parity arms, write them back ─────────────
fn trim(bits: &[bool]) -> &[bool] {
    let mut hi = bits.len();
    while hi > 0 && !bits[hi - 1] { hi -= 1; }
    &bits[..hi]
}

impl WordTape {
    pub(crate) fn zero() -> Self { Self(String::from("⊢⊙⊡⊣")) }
    pub(crate) fn one() -> Self { Self::from_small(1) }
    pub(crate) fn from_small(n: u64) -> Self { Self::from_biguint(&BigUint::from(n)) }
    pub(crate) fn from_biguint(n: &BigUint) -> Self {
        if n.is_zero() { return Self::zero(); }
        let bits = biguint_to_bits(n);
        Self::from_bits(&bits)
    }
    pub(crate) fn to_biguint(&self) -> BigUint {
        bits_to_biguint(&self.bits_low_first())
    }
    pub(crate) fn is_zero(&self) -> bool { self.0 == "⊢⊙⊡⊣" }

    /// Walk the word's bit cells (⊢ prefix and ⊙⊡⊣ suffix dropped) and read
    /// each parity arm: ⊥ → true, ⊤ → false. Least-significant bit first.
    fn bits_low_first(&self) -> Vec<bool> {
        let c: Vec<char> = self.0.chars().collect();
        if c.len() == 4 && c[1] == IMSCRIB && c[2] == IFIX && c[3] == TANCH {
            return Vec::new();
        }
        let body = &c[1..c.len() - 3];
        let mut bits = Vec::with_capacity(body.len() / 5);
        for cell in body.chunks(5) {
            bits.push(cell[3] == EVALF);
        }
        bits
    }

    /// The inverse walk: write bits back as parity cells. Trailing high zero
    /// bits are dropped so the word is canonical (zero is ⊢⊙⊡⊣).
    fn from_bits(bits: &[bool]) -> Self {
        let bits = trim(bits);
        let mut w = String::new();
        w.push(VINIT);
        for &b in bits {
            w.push(AFWD);
            w.push(CLINK);
            w.push(FSPLIT);
            w.push(if b { EVALF } else { EVALT });
            w.push(FFUSE);
        }
        w.push(IMSCRIB);
        w.push(IFIX);
        w.push(TANCH);
        Self(w)
    }

    pub(crate) fn bit_len(&self) -> usize {
        self.bits_low_first().len()
    }

    /// Low `k` bit-cells — x mod 2^k — read from the value's own word, not a
    /// conversion to another numeral system. `k` past the bit length returns
    /// the value unchanged (its low bits are all of it).
    pub(crate) fn truncate(&self, k: usize) -> Self {
        let bits = self.bits_low_first();
        let k = k.min(bits.len());
        Self::from_bits(&bits[..k])
    }
}

// ── bit walks shared by the arithmetic (no BigUint, no limbs) ────────────
fn bits_add(a: &[bool], b: &[bool]) -> Vec<bool> {
    let n = a.len().max(b.len());
    let mut out = Vec::with_capacity(n + 1);
    let mut carry = false;
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(false);
        let y = b.get(i).copied().unwrap_or(false);
        out.push((x ^ y) ^ carry);
        carry = (x & y) | (x & carry) | (y & carry);
    }
    if carry { out.push(true); }
    out
}

/// a − b. None if a < b (borrow runs out).
fn bits_sub(a: &[bool], b: &[bool]) -> Option<Vec<bool>> {
    let n = a.len().max(b.len());
    let mut out = Vec::with_capacity(n);
    let mut borrow = false;
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(false);
        let y = b.get(i).copied().unwrap_or(false);
        let (d, bo) = match (x, y, borrow) {
            (false, false, false) => (false, false),
            (false, false, true) => (true, true),
            (false, true, false) => (true, true),
            (false, true, true) => (false, true),
            (true, false, false) => (true, false),
            (true, false, true) => (false, false),
            (true, true, false) => (false, false),
            (true, true, true) => (true, true),
        };
        out.push(d);
        borrow = bo;
    }
    if borrow { None } else { Some(out) }
}

fn bits_cmp(a: &[bool], b: &[bool]) -> core::cmp::Ordering {
    let a = trim(a);
    let b = trim(b);
    if a.len() != b.len() { return a.len().cmp(&b.len()); }
    for i in (0..a.len()).rev() {
        if a[i] != b[i] { return a[i].cmp(&b[i]); }
    }
    core::cmp::Ordering::Equal
}

/// Schoolbook shift-and-add on the bit list.
fn bits_mul(a: &[bool], b: &[bool]) -> Vec<bool> {
    let a = trim(a);
    let b = trim(b);
    let mut acc = vec![false; a.len() + b.len()];
    for i in 0..a.len() {
        if a[i] {
            let mut carry = false;
            for j in 0..b.len() {
                let idx = i + j;
                let x = acc[idx];
                let y = b[j];
                acc[idx] = (x ^ y) ^ carry;
                carry = (x & y) | (x & carry) | (y & carry);
            }
            let mut k = i + b.len();
            while carry {
                let x = acc[k];
                acc[k] = x ^ carry;
                carry = x & carry;
                k += 1;
            }
        }
    }
    trim(&acc).to_vec()
}

/// Restoring long division. None if b is zero.
fn bits_divmod(a: &[bool], b: &[bool]) -> Option<(Vec<bool>, Vec<bool>)> {
    let bt = trim(b);
    if bt.is_empty() { return None; }
    let at = trim(a);
    let mut rem: Vec<bool> = Vec::new();
    let mut q = Vec::with_capacity(at.len());
    for i in (0..at.len()).rev() {
        rem.insert(0, at[i]); // rem = rem·2 + bring-down bit
        match bits_sub(&rem, bt) {
            Some(r) => { rem = r; q.push(true); }
            None => q.push(false),
        }
    }
    q.reverse();
    Some((trim(&q).to_vec(), rem))
}

/// Floor square root by Newton descent from 2^ceil(bits/2), with a final
/// safety sweep so the result is the exact floor under floor division.
fn bits_isqrt(a: &[bool]) -> Vec<bool> {
    let at = trim(a);
    if at.is_empty() { return Vec::new(); }
    let bl = at.len();
    let mut x = vec![false; bl / 2 + 2];
    x[bl / 2 + 1] = true; // 2^ceil(bl/2), strictly above sqrt
    loop {
        let (q, _) = bits_divmod(&at, &x).unwrap();
        let y = bits_add(&x, &q)[1..].to_vec(); // (x + a/x) >> 1
        if bits_cmp(&y, &x) != core::cmp::Ordering::Less { break; }
        x = y;
    }
    while bits_cmp(&bits_mul(&x, &x), &at) == core::cmp::Ordering::Greater {
        x = bits_sub(&x, &[true]).unwrap();
    }
    while bits_cmp(&bits_mul(&bits_add(&x, &[true]), &bits_add(&x, &[true])), &at)
        != core::cmp::Ordering::Greater {
        x = bits_add(&x, &[true]);
    }
    trim(&x).to_vec()
}

// ── arithmetic: word-walks over the cell bits ────────────────────────────
impl WordTape {
    pub(crate) fn add(&self, b: &Self) -> Self {
        Self::from_bits(&bits_add(&self.bits_low_first(), &b.bits_low_first()))
    }
    pub(crate) fn sub(&self, b: &Self) -> Option<Self> {
        bits_sub(&self.bits_low_first(), &b.bits_low_first()).map(|r| Self::from_bits(&r))
    }
    pub(crate) fn mul(&self, b: &Self) -> Self {
        Self::from_bits(&bits_mul(&self.bits_low_first(), &b.bits_low_first()))
    }
    pub(crate) fn divmod(&self, b: &Self) -> Option<(Self, Self)> {
        bits_divmod(&self.bits_low_first(), &b.bits_low_first())
            .map(|(q, r)| (Self::from_bits(&q), Self::from_bits(&r)))
    }
    pub(crate) fn ge(&self, b: &Self) -> bool {
        bits_cmp(&self.bits_low_first(), &b.bits_low_first()) != core::cmp::Ordering::Less
    }
    pub(crate) fn gt(&self, b: &Self) -> bool {
        bits_cmp(&self.bits_low_first(), &b.bits_low_first()) == core::cmp::Ordering::Greater
    }

    /// Shift left: insert n empty low bit-cells before the existing ones.
    pub(crate) fn shl(&self, bits: usize) -> Self {
        let mut v = self.bits_low_first();
        let mut out = vec![false; bits];
        out.append(&mut v);
        Self::from_bits(&out)
    }
    /// Shift right one: drop the least-significant bit-cell.
    pub(crate) fn shr1(&self) -> Self {
        let v = self.bits_low_first();
        if v.is_empty() { return Self::zero(); }
        Self::from_bits(&v[1..])
    }
    /// A one-hot bit at a binary depth.
    pub(crate) fn one_at(bit: usize) -> Self {
        let mut v = vec![false; bit + 1];
        v[bit] = true;
        Self::from_bits(&v)
    }

    /// Both inputs canonical below m, so one compare/subtract is the full
    /// reduction walk.
    pub(crate) fn add_mod_canonical(&self, b: &Self, m: &Self) -> Self {
        let sum = self.add(b);
        if sum.ge(m) { sum.sub(m).unwrap() } else { sum }
    }

    /// Newton square root, entirely on the word.
    pub(crate) fn isqrt(&self) -> Self {
        Self::from_bits(&bits_isqrt(&self.bits_low_first()))
    }
    /// Concavity-bounded Newton continuation from a lower floor root.
    pub(crate) fn isqrt_from(&self, lower: &Self) -> Self {
        let bits = self.bits_low_first();
        let at = trim(&bits);
        if at.is_empty() { return Self::zero(); }
        let one = Self::one();
        let two = Self::from_small(2);
        let lower_sq = lower.mul(lower);
        let rise = self.sub(&lower_sq).unwrap_or_else(Self::zero);
        let denom = lower.add(lower).add(&one);
        let jump = rise.divmod(&denom).map(|x| x.0).unwrap_or_else(Self::zero);
        let mut x = lower.add(&jump).add(&one);
        loop {
            let q = self.divmod(&x).unwrap().0;
            let y = x.add(&q).divmod(&two).unwrap().0;
            if y.ge(&x) { break; }
            x = y;
        }
        while x.mul(&x).gt(self) { x = x.sub(&one).unwrap(); }
        x
    }

    /// Solve the nonlinear remainder of the carried Fermat root; statistics
    /// are word counts in the order q=u, q=u−1, general, bracket width,
    /// bounded refinements, divisions.
    pub(crate) fn root_deficit_from(base: &Self, excess: &Self) -> (Self, [Self; 6]) {
        let zero = Self::zero();
        let one = Self::one();
        let two_b = base.add(base);
        let (u, r) = excess.divmod(&two_b).unwrap();
        if u.is_zero() {
            return (zero, [one, Self::zero(), Self::zero(), Self::zero(), Self::zero(), Self::one()]);
        }
        let u2 = u.mul(&u);
        if !u2.gt(&r) {
            return (u, [one, Self::zero(), Self::zero(), Self::zero(), Self::zero(), Self::one()]);
        }
        let x = u2.sub(&r).unwrap();
        let two_u = u.add(&u);
        let d1_capacity = two_b.add(&two_u).sub(&one).unwrap();
        if !x.gt(&d1_capacity) {
            return (u.sub(&one).unwrap(), [Self::zero(), one, Self::zero(), Self::zero(), Self::zero(), Self::one()]);
        }
        let (mut lo, _) = excess.divmod(&two_b.add(&u)).unwrap();
        let mut hi = u;
        let mut lo_value = lo.mul(&two_b.add(&lo));
        debug_assert!(!lo_value.gt(excess));
        debug_assert!(hi.mul(&two_b.add(&hi)).gt(excess));
        let bracket = hi.sub(&lo).unwrap();
        let mut refinements = Self::zero();
        while hi.sub(&lo).unwrap().gt(&one) {
            let half = hi.sub(&lo).unwrap().shr1();
            let mid = lo.add(&half);
            let mid_value = lo_value.add(&half.mul(&two_b.add(&lo.add(&lo)).add(&half)));
            if mid_value.gt(excess) { hi = mid; } else { lo = mid; lo_value = mid_value; }
            debug_assert!(!lo_value.gt(excess));
            debug_assert!(hi.mul(&two_b.add(&hi)).gt(excess));
            refinements = refinements.add(&one);
        }
        (lo, [Self::zero(), Self::zero(), one, bracket, refinements, Self::from_small(2)])
    }

    /// Close the unnested excess into whole recursive square-shell blocks,
    /// by tape shifts, additions, comparisons and subtractions only.
    pub(crate) fn nest_into_square_frontier(excess: &Self, base: &Self) -> (Self, Self, Self) {
        let mut remainder = excess.clone();
        let mut nested_base = base.clone();
        let mut q = Self::zero();
        let mut blocks = Self::zero();
        for r in (0..=excess.bit_len() / 2).rev() {
            let width = Self::one_at(r);
            let capacity = nested_base.shl(r + 1).add(&Self::one_at(2 * r));
            if remainder.ge(&capacity) {
                remainder = remainder.sub(&capacity).unwrap();
                nested_base = nested_base.add(&width);
                q = q.add(&width);
                blocks = blocks.add(&Self::one());
            }
        }
        debug_assert_eq!(nested_base, base.add(&q));
        (q, remainder, blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;

    fn b(n: u64) -> BigUint { BigUint::from(n) }

    fn check_roundtrip(vals: &[u64]) {
        for &v in vals {
            let t = WordTape::from_small(v);
            assert_eq!(t.to_biguint(), b(v), "roundtrip {v}");
            // canonical zero is the bare word, everything else has cells
            if v == 0 { assert_eq!(t.0, "⊢⊙⊡⊣"); }
        }
    }

    #[test]
    fn roundtrip_through_the_word() {
        check_roundtrip(&[0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 31, 32, 63, 64, 127, 255, 256, 1024, 4032, 65535]);
    }

    #[test]
    fn add_is_exact() {
        for a in 0..40u64 {
            for c in 0..40u64 {
                let r = WordTape::from_small(a).add(&WordTape::from_small(c));
                assert_eq!(r.to_biguint(), b(a + c), "add {a}+{c}");
            }
        }
    }

    #[test]
    fn sub_is_exact_or_none() {
        for a in 0..40u64 {
            for c in 0..40u64 {
                let r = WordTape::from_small(a).sub(&WordTape::from_small(c));
                if c > a { assert!(r.is_none(), "sub {a}-{c} should be none"); }
                else { assert_eq!(r.unwrap().to_biguint(), b(a - c), "sub {a}-{c}"); }
            }
        }
    }

    #[test]
    fn mul_is_exact() {
        for a in 0..25u64 {
            for c in 0..25u64 {
                let r = WordTape::from_small(a).mul(&WordTape::from_small(c));
                assert_eq!(r.to_biguint(), b(a * c), "mul {a}*{c}");
            }
        }
    }

    #[test]
    fn divmod_is_exact() {
        for a in 0..50u64 {
            for m in 1..50u64 {
                let (q, r) = WordTape::from_small(a).divmod(&WordTape::from_small(m)).unwrap();
                assert_eq!(q.to_biguint(), b(a / m), "div {a}/{m}");
                assert_eq!(r.to_biguint(), b(a % m), "mod {a}/{m}");
            }
        }
    }

    #[test]
    fn isqrt_is_floor_sqrt() {
        for v in 0..200u64 {
            let r = WordTape::from_small(v).isqrt();
            let x = r.to_biguint();
            let xs = x.clone() * x.clone();
            let xp1 = (x.clone() + BigUint::one()) * (x.clone() + BigUint::one());
            assert!(xs <= b(v), "isqrt({v})^2 <= v");
            assert!(xp1 > b(v), "(isqrt({v})+1)^2 > v");
        }
        for &sq in &[1u64, 4, 9, 16, 25, 36, 49, 64, 81, 100, 4032 * 4032 / 1] {
            let r = WordTape::from_small(sq).isqrt();
            // only assert exact for perfect squares
            if sq < 400 { assert_eq!(r.to_biguint(), BigUint::from((sq as f64).sqrt() as u64)); }
        }
    }

    #[test]
    fn add_mod_canonical_stays_below_modulus() {
        let m = WordTape::from_small(64);
        for a in 0..64u64 {
            for c in 0..64u64 {
                let r = WordTape::from_small(a).add_mod_canonical(&WordTape::from_small(c), &m);
                assert_eq!(r.to_biguint(), b((a + c) % 64), "amc {a}+{c} mod 64");
            }
        }
    }

    #[test]
    fn wide_roundtrip_and_arithmetic() {
        // a 128-bit-scale value exercised through add/mul/divmod/isqrt
        let big = BigUint::from(0xFFFFFFFFFFFFFFFFu64) * BigUint::from(0x123456789ABCDEFu64);
        let t = WordTape::from_biguint(&big);
        assert_eq!(t.to_biguint(), big);
        let doubled = t.add(&t);
        assert_eq!(doubled.to_biguint(), &big + &big);
        let squared = t.mul(&t);
        assert_eq!(squared.to_biguint(), &big * &big);
        let root = squared.isqrt();
        assert_eq!(root.to_biguint(), big, "isqrt of a perfect square recovers the root");
    }
}
