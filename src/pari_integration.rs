//! pari_integration.rs — PARI/GP Tower Polynomial Integration (exact, arbitrary-precision)
//!
//! The PARI tower polynomials tower_C{1,4,8,16,32}.poly have coefficients up to
//! 530 bits (C32), far past i64/i128/u128. They are carried exactly as
//! little-endian magnitude limbs plus a sign bit (see tower_polynomials.rs), and
//! this module evaluates them modulo the secp256k1 group order n with
//! arbitrary-precision integer arithmetic — no i64 truncation, no placeholder
//! coefficients, no single-u64 "U256Mod" wrapper, no std::fs / println! at
//! runtime (the kernel is no_std).
//!
//! The C16 polynomial (degree 64) is the main fiducial moduli polynomial used
//! by sic_povm_d2048_fiducial_step.

#![allow(dead_code)]

use alloc::vec::Vec;
use alloc::vec;
use core::cmp::Ordering;
use crate::pk2sk::U256;
use crate::tower_polynomials::{TowerPoly, TOWER_C16};

// ── Arbitrary-precision unsigned integer (little-endian limbs, trimmed) ─────
// A minimal bigint for exact polynomial evaluation. Limbs are little-endian
// with no high zero limbs; the empty limb vector is zero. No fixed-width
// intermediate: every multiply is a schoolbook carry chain, every reduction is
// a restoring binary long division.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigUint {
    limbs: Vec<u64>,
}

impl BigUint {
    pub fn zero() -> Self { BigUint { limbs: vec![] } }

    pub fn from_u64(v: u64) -> Self {
        if v == 0 { Self::zero() } else { BigUint { limbs: vec![v] } }
    }

    pub fn from_u256(v: &U256) -> Self {
        let mut l = v.0.to_vec();
        while l.len() > 1 && l[l.len() - 1] == 0 { l.pop(); }
        if l.len() == 1 && l[0] == 0 { l.clear(); }
        BigUint { limbs: l }
    }

    /// Magnitude from a mag-limbs row (a slice of the tower polynomial mag table).
    pub fn from_mag(mag: &[u64]) -> BigUint {
        let mut l = mag.to_vec();
        while l.len() > 1 && l[l.len() - 1] == 0 { l.pop(); }
        if l.len() == 1 && l[0] == 0 { l.clear(); }
        BigUint { limbs: l }
    }

    pub fn is_zero(&self) -> bool { self.limbs.is_empty() }

    /// Low 256 bits (caller must ensure the value fits in 4 limbs).
    pub fn to_u256(&self) -> U256 {
        let mut a = [0u64; 4];
        for (i, &x) in self.limbs.iter().take(4).enumerate() { a[i] = x; }
        U256(a)
    }

    pub fn bit_len(&self) -> usize {
        match self.limbs.last() {
            None => 0,
            Some(&hi) => (self.limbs.len() - 1) * 64 + (64 - hi.leading_zeros() as usize),
        }
    }

    fn bit(&self, i: usize) -> bool {
        if i / 64 >= self.limbs.len() { return false; }
        (self.limbs[i / 64] >> (i % 64)) & 1 == 1
    }

    pub fn cmp(&self, o: &BigUint) -> Ordering {
        if self.limbs.len() != o.limbs.len() {
            return self.limbs.len().cmp(&o.limbs.len());
        }
        for i in (0..self.limbs.len()).rev() {
            match self.limbs[i].cmp(&o.limbs[i]) {
                Ordering::Equal => continue,
                c => return c,
            }
        }
        Ordering::Equal
    }

    pub fn add(&self, o: &BigUint) -> BigUint {
        let n = self.limbs.len().max(o.limbs.len());
        let mut out = vec![0u64; n];
        let mut carry = 0u128;
        for i in 0..n {
            let a = if i < self.limbs.len() { self.limbs[i] as u128 } else { 0 };
            let b = if i < o.limbs.len() { o.limbs[i] as u128 } else { 0 };
            let s = a + b + carry;
            out[i] = s as u64;
            carry = s >> 64;
        }
        if carry != 0 { out.push(carry as u64); }
        BigUint { limbs: out }
    }

    /// self - o; requires self >= o.
    pub fn sub(&self, o: &BigUint) -> BigUint {
        debug_assert!(self.cmp(o) != Ordering::Less);
        let mut out = vec![0u64; self.limbs.len()];
        let mut borrow = 0u128;
        for i in 0..self.limbs.len() {
            let a = self.limbs[i] as u128;
            let b = (if i < o.limbs.len() { o.limbs[i] as u128 } else { 0 }) + borrow;
            if a >= b {
                out[i] = (a - b) as u64;
                borrow = 0;
            } else {
                out[i] = (a + (1u128 << 64) - b) as u64;
                borrow = 1;
            }
        }
        while out.len() > 1 && out[out.len() - 1] == 0 { out.pop(); }
        if out.len() == 1 && out[0] == 0 { out.clear(); }
        BigUint { limbs: out }
    }

    pub fn mul(&self, o: &BigUint) -> BigUint {
        if self.is_zero() || o.is_zero() { return BigUint::zero(); }
        let mut out = vec![0u64; self.limbs.len() + o.limbs.len()];
        for i in 0..self.limbs.len() {
            let mut carry = 0u128;
            for j in 0..o.limbs.len() {
                let t = self.limbs[i] as u128 * o.limbs[j] as u128
                    + out[i + j] as u128 + carry;
                out[i + j] = t as u64;
                carry = t >> 64;
            }
            let mut k = i + o.limbs.len();
            while carry != 0 {
                let t = out[k] as u128 + carry;
                out[k] = t as u64;
                carry = t >> 64;
                k += 1;
            }
        }
        while out.len() > 1 && out[out.len() - 1] == 0 { out.pop(); }
        if out.len() == 1 && out[0] == 0 { out.clear(); }
        BigUint { limbs: out }
    }

    fn shl_bits(&self, n: usize) -> BigUint {
        if self.is_zero() { return BigUint::zero(); }
        let word_shift = n / 64;
        let bit_shift = n % 64;
        let mut out = vec![0u64; self.limbs.len() + word_shift + 1];
        let mut carry = 0u64;
        for i in 0..self.limbs.len() {
            let cur = self.limbs[i];
            out[i + word_shift] = (cur << bit_shift) | carry;
            carry = if bit_shift == 0 { 0 } else { cur >> (64 - bit_shift) };
        }
        out[self.limbs.len() + word_shift] = carry;
        while out.len() > 1 && out[out.len() - 1] == 0 { out.pop(); }
        if out.len() == 1 && out[0] == 0 { out.clear(); }
        BigUint { limbs: out }
    }

    /// self >> n bits (exact floor division by 2^n). Used to rescale
    /// Q64.64 fixed-point products (Q128.128 -> Q64.64).
    pub fn shr_bits(&self, n: usize) -> BigUint {
        if self.is_zero() { return BigUint::zero(); }
        let word_shift = n / 64;
        let bit_shift = n % 64;
        if word_shift >= self.limbs.len() { return BigUint::zero(); }
        let out_len = self.limbs.len() - word_shift;
        let mut out = vec![0u64; out_len];
        for i in 0..out_len {
            let src = i + word_shift;
            let hi = self.limbs[src];
            let lo = if src + 1 < self.limbs.len() { self.limbs[src + 1] } else { 0 };
            out[i] = if bit_shift == 0 { hi } else { (hi >> bit_shift) | (lo << (64 - bit_shift)) };
        }
        while out.len() > 1 && out[out.len() - 1] == 0 { out.pop(); }
        if out.len() == 1 && out[0] == 0 { out.clear(); }
        BigUint { limbs: out }
    }

    fn set_bit(&mut self, i: usize) {
        while i / 64 >= self.limbs.len() { self.limbs.push(0); }
        self.limbs[i / 64] |= 1u64 << (i % 64);
    }

    /// Restoring binary long division: returns (quotient, remainder).
    pub fn div_rem(&self, d: &BigUint) -> (BigUint, BigUint) {
        debug_assert!(!d.is_zero());
        if self.cmp(d) == Ordering::Less { return (BigUint::zero(), self.clone()); }
        let mut q = BigUint::zero();
        let mut r = BigUint::zero();
        for i in (0..self.bit_len()).rev() {
            r = r.shl_bits(1);
            if self.bit(i) { r = r.add(&BigUint::from_u64(1)); }
            if r.cmp(d) != Ordering::Less {
                r = r.sub(d);
                q.set_bit(i);
            }
        }
        (q, r)
    }
}

// ── Moduli polynomial data (real, backed by tower_polynomials) ──────────────

#[derive(Clone, Debug)]
pub struct ModuliPolynomialData {
    pub degree: u32,
    pub limbs: u32,
    pub mag: &'static [u64],
    pub neg: &'static [u8],
}

/// The C16 moduli polynomial (degree 64), exact signed coefficients.
pub fn extract_moduli_polynomial_data() -> ModuliPolynomialData {
    ModuliPolynomialData {
        degree: TOWER_C16.degree,
        limbs: TOWER_C16.limbs,
        mag: TOWER_C16.mag,
        neg: TOWER_C16.neg,
    }
}

/// Horner evaluation of a tower polynomial at x, reduced mod the group order n.
/// Coefficients may be negative (neg bit) and wider than 256 bits; each is
/// reduced mod n first, then the running accumulator is kept in [0, n).
pub fn eval_poly_mod_n(poly: &TowerPoly, x: &U256) -> U256 {
    let n = BigUint::from_u256(&U256::n());
    let xb = BigUint::from_u256(x);
    let deg = poly.degree as usize;
    let limbs = poly.limbs as usize;
    let mut acc = BigUint::zero();
    for k in (0..=deg).rev() {
        // acc = acc * x (mod n)
        acc = acc.mul(&xb).div_rem(&n).1;
        // c_k (signed) mod n
        let mag = BigUint::from_mag(&poly.mag[k * limbs .. (k + 1) * limbs]);
        let c = mag.div_rem(&n).1;
        if poly.neg[k] == 1 {
            if acc.cmp(&c) != Ordering::Less {
                acc = acc.sub(&c);
            } else {
                acc = n.sub(&c.sub(&acc));
            }
        } else {
            acc = acc.add(&c);
            if acc.cmp(&n) != Ordering::Less {
                acc = acc.sub(&n);
            }
        }
    }
    acc.to_u256()
}

/// Add two U256 mod the group order n (the curve's scalar ring, not the field
/// prime P). n < 2^256, so the BigUint reduction is exact.
pub fn add_mod_n(a: &U256, b: &U256) -> U256 {
    let n = BigUint::from_u256(&U256::n());
    BigUint::from_u256(a).add(&BigUint::from_u256(b)).div_rem(&n).1.to_u256()
}

/// The sic_povm_d2048_fiducial step: the C16 moduli polynomial IS the portal.
/// The public key (x, y) maps into the fiducial winding through P(x) and P(y)
/// — the ±√m_d Galois pair — combined modulo the group order n.
pub fn sic_povm_d2048_fiducial_step(data: &ModuliPolynomialData, pk_x: &U256, pk_y: &U256) -> U256 {
    let poly = TowerPoly { degree: data.degree, limbs: data.limbs, mag: data.mag, neg: data.neg };
    let px = eval_poly_mod_n(&poly, pk_x);
    let py = eval_poly_mod_n(&poly, pk_y);
    add_mod_n(&px, &py)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower_polynomials::{TOWER_C1, TOWER_C4, TOWER_C8, TOWER_C32};

    #[test]
    fn test_big_arith() {
        // (2^64 - 1) * 2 = 2^65 - 2
        let a = BigUint::from_u64(0xffffffffffffffffu64).mul(&BigUint::from_u64(2));
        assert_eq!(a.to_u256(), U256([0xfffffffffffffffeu64, 1, 0, 0]));
        let (q, r) = BigUint::from_u64(17).div_rem(&BigUint::from_u64(5));
        assert_eq!(q.to_u256(), U256::from_u64(3));
        assert_eq!(r.to_u256(), U256::from_u64(2));
    }

    #[test]
    fn test_all_tower_polys_monic() {
        for poly in [&TOWER_C1, &TOWER_C4, &TOWER_C8, &TOWER_C16, &TOWER_C32] {
            let deg = poly.degree as usize;
            let l = poly.limbs as usize;
            assert_eq!(poly.neg[deg], 0, "leading coeff must be positive");
            let lead = BigUint::from_mag(&poly.mag[deg * l .. (deg + 1) * l]);
            assert_eq!(lead.to_u256(), U256::from_u64(1), "leading coeff must be 1");
        }
    }

    #[test]
    fn test_eval_c1() {
        // C1 = x, so P(x) = x mod n
        let x = U256::from_u64(12345);
        assert_eq!(eval_poly_mod_n(&TOWER_C1, &x), x);
    }

    #[test]
    fn test_moduli_data() {
        let d = extract_moduli_polynomial_data();
        assert_eq!(d.degree, 64);
        assert_eq!(d.limbs, 5);
    }
}
