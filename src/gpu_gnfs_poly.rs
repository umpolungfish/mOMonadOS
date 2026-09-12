//! gpu_gnfs_poly.rs — ⊙ Self-Referential Polynomial (base-m selection).

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::BigUint;
use num_traits::{One, Zero};

const M_SEARCH_RADIUS: u64 = 64;

/// Algebraic/rational polynomial pair from ⊙.
///
/// Classical base-m selection: expand N in base m to get
///   f(x) = Σ a_i x^i    with  f(m) = N
/// and take the linear rational side
///   g(x) = x − m.
#[derive(Clone, Debug)]
pub struct PolyPair {
    pub degree: u32,
    pub m: BigUint,
    /// Little-endian coefficients of f: a0 + a1 x + … + ad x^d.
    pub f: Vec<BigUint>,
}

impl PolyPair {
    pub fn eval_at_m(&self) -> BigUint {
        use crate::native_numeral::{add_via_word, multiply_via_word};
        let mut acc = BigUint::zero();
        let mut pow = BigUint::one();
        for a in &self.f {
            acc = add_via_word(&acc, &multiply_via_word(a, &pow));
            pow = multiply_via_word(&pow, &self.m);
        }
        acc
    }

    /// Homogeneous norm |N_f(a,b)| = |b^d f(a/b)| = |Σ a_i a^i b^{d-i}|.
    pub fn homogeneous_norm(&self, a: i64, b: i64) -> BigUint {
        use crate::native_numeral::{add_via_word, multiply_via_word, subtract_via_word};
        let d = self.degree as usize;
        let aa = BigUint::from(a.unsigned_abs());
        let bb = BigUint::from(b.unsigned_abs());
        let mut pos = BigUint::zero();
        let mut neg = BigUint::zero();
        for (i, coef) in self.f.iter().enumerate() {
            if coef.is_zero() {
                continue;
            }
            let mut term = coef.clone();
            for _ in 0..i {
                term = multiply_via_word(&term, &aa);
            }
            for _ in 0..(d - i) {
                term = multiply_via_word(&term, &bb);
            }
            let sign_neg = ((a < 0) && (i % 2 == 1)) ^ ((b < 0) && ((d - i) % 2 == 1));
            if sign_neg {
                neg = add_via_word(&neg, &term);
            } else {
                pos = add_via_word(&pos, &term);
            }
        }
        if pos >= neg {
            subtract_via_word(&pos, &neg).unwrap()
        } else {
            subtract_via_word(&neg, &pos).unwrap()
        }
    }

    pub fn format_f(&self) -> String {
        let mut parts = Vec::new();
        for (i, a) in self.f.iter().enumerate().rev() {
            if a.is_zero() && self.f.len() > 1 {
                continue;
            }
            let term = match i {
                0 => format!("{a}"),
                1 => {
                    if a.is_one() {
                        String::from("x")
                    } else {
                        format!("{a}*x")
                    }
                }
                _ => {
                    if a.is_one() {
                        format!("x^{i}")
                    } else {
                        format!("{a}*x^{i}")
                    }
                }
            };
            parts.push(term);
        }
        if parts.is_empty() {
            String::from("0")
        } else {
            parts.join(" + ")
        }
    }

    pub fn format_g(&self) -> String {
        format!("x - {}", self.m)
    }

    pub fn coeff_score(&self) -> u64 {
        self.f.iter().map(|a| a.bits().max(1)).sum()
    }

    /// f(r) mod p for p fitting in u64 (algebraic FB root hunt).
    pub fn eval_mod_u64(&self, r: u64, p: u64) -> u64 {
        let mut acc: u128 = 0;
        let mut pow: u128 = 1;
        let pm = p as u128;
        for a in &self.f {
            let ai = (a % p).to_u64_digits().first().copied().unwrap_or(0) as u128;
            acc = (acc + ai * pow) % pm;
            pow = (pow * r as u128) % pm;
        }
        acc as u64
    }
}

pub fn choose_degree(bits: u64) -> u32 {
    // Classical GNFS: d ≈ (3 ln N / ln ln N)^{1/3}.
    let ln_n = (bits as f64) * core::f64::consts::LN_2;
    let ln_ln = ln_n.ln().max(2.0);
    let d = ((3.0 * ln_n) / ln_ln).powf(1.0 / 3.0);
    let d = d.round() as u32;
    d.clamp(3, 16)
}

fn pow_u32(base: &BigUint, exp: u32) -> BigUint {
    use crate::native_numeral::multiply_via_word;
    let mut acc = BigUint::one();
    let mut b = base.clone();
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            acc = multiply_via_word(&acc, &b);
        }
        b = multiply_via_word(&b, &b);
        e >>= 1;
    }
    acc
}

fn ith_root(n: &BigUint, d: u32) -> BigUint {
    use crate::native_numeral::{add_via_word, divmod_small_via_word, divmod_via_word, multiply_via_word, pow2, subtract_via_word};
    assert!(d >= 1);
    if n.is_zero() || n.is_one() {
        return n.clone();
    }
    if d == 1 {
        return n.clone();
    }
    let one = BigUint::one();
    let bits = n.bits();
    let mut x = pow2((((bits + d as u64 - 1) / d as u64).max(1)) as usize);
    if x.is_zero() {
        x = one.clone();
    }
    let dm1 = d - 1;
    // d and dm1 always fit in a u64 with room to spare (bounded well under
    // u32::MAX in practice, this is GNFS polynomial degree, at most a
    // couple dozen), so the small-divisor and small-multiplier primitives
    // apply directly; n / &x_dm1 stays big-by-big, since x_dm1 can be as
    // large as n itself.
    loop {
        let xd = pow_u32(&x, d);
        if &xd == n {
            return x;
        }
        let x_dm1 = pow_u32(&x, dm1);
        let sum = add_via_word(&multiply_via_word(&x, &BigUint::from(dm1)), &divmod_via_word(n, &x_dm1).unwrap().0);
        let next = divmod_small_via_word(&sum, d as u64).unwrap().0;
        if next == x || next.is_zero() {
            let mut y = if next.is_zero() {
                one.clone()
            } else {
                next
            };
            while pow_u32(&y, d) > *n {
                y = subtract_via_word(&y, &one).unwrap();
            }
            loop {
                let yp1 = add_via_word(&y, &one);
                if pow_u32(&yp1, d) > *n {
                    return y;
                }
                y = yp1;
            }
        }
        if &next > &x && pow_u32(&x, d) <= *n {
            let mut y = x;
            while pow_u32(&y, d) > *n {
                y = subtract_via_word(&y, &one).unwrap();
            }
            loop {
                let yp1 = add_via_word(&y, &one);
                if pow_u32(&yp1, d) > *n {
                    return y;
                }
                y = yp1;
            }
        }
        x = next;
    }
}

fn base_m_digits(n: &BigUint, m: &BigUint) -> Vec<BigUint> {
    use crate::native_numeral::divmod_via_word;
    assert!(!m.is_zero() && !m.is_one());
    let mut rest = n.clone();
    let mut digits = Vec::new();
    while !rest.is_zero() {
        let (q, r) = divmod_via_word(&rest, m).unwrap();
        digits.push(r);
        rest = q;
    }
    if digits.is_empty() {
        digits.push(BigUint::zero());
    }
    digits
}

fn poly_from_m(n: &BigUint, m: &BigUint) -> Option<PolyPair> {
    if m < &BigUint::from(2u32) {
        return None;
    }
    let f = base_m_digits(n, m);
    let degree = (f.len().saturating_sub(1)) as u32;
    if degree < 2 {
        return None;
    }
    let pair = PolyPair {
        degree,
        m: m.clone(),
        f,
    };
    if pair.eval_at_m() != *n {
        return None;
    }
    Some(pair)
}

pub fn select_polynomial(n: &BigUint, degree: Option<u32>) -> Result<PolyPair, String> {
    if n < &BigUint::from(8u32) {
        return Err(format!("gpu_gnfs ⊙: N={n} too small for GNFS poly selection"));
    }
    let d = degree.unwrap_or_else(|| choose_degree(n.bits()));
    if d < 3 {
        return Err(format!("gpu_gnfs ⊙: degree {d} < 3"));
    }
    let m0 = ith_root(n, d);
    if m0 < BigUint::from(2u32) {
        return Err(format!("gpu_gnfs ⊙: ith_root returned m={m0} for d={d}"));
    }

    use crate::native_numeral::add_via_word;
    let mut best: Option<PolyPair> = None;
    let mut best_score = u64::MAX;
    let radius = BigUint::from(M_SEARCH_RADIUS);
    let lo = if m0 > radius {
        crate::native_numeral::subtract_via_word(&m0, &radius).unwrap()
    } else {
        BigUint::from(2u32)
    };
    let hi = add_via_word(&m0, &radius);
    let mut m = lo;
    while m <= hi {
        if let Some(pair) = poly_from_m(n, &m) {
            let score = pair.coeff_score() + if pair.degree == d { 0 } else { 1_000_000 };
            if score < best_score {
                best_score = score;
                best = Some(pair);
            }
        }
        m = add_via_word(&m, &BigUint::one());
    }
    best.ok_or_else(|| format!("gpu_gnfs ⊙: no valid base-m poly near m0={m0} for d={d}"))
}
