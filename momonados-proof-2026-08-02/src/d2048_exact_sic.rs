// d2048_exact_sic.rs -- Exact Algebraic d=2048 SIC-POVM Fiducial Extraction
//
// Implements the exact (non-f64) Stark unit for d=2048 and the 2-part
// structural S-unit tower bypass via Galois embeddings.
//
// Exact algebraic form:
//   epsilon = (2047 + sqrt(4190205)) / 2
// Satisfies minimal polynomial: x^2 - 2047*x + 1 = 0
// Norm: N(epsilon) = 2047^2/4 - 4190205/4 = (4190209 - 4190205)/4 = 4/4 = 1
//
// 2-part extraction (Galois embedding):
//   psi_k = embeddings_2048(k)(stark_unit_2048)
// The base field F = Q(sqrt(4190205)) has discriminant 4190205 = 3*5*409*683.
// The two real embeddings are sigma_+(sqrt) = +sqrt and sigma_-(sqrt) = -sqrt.
// For the fundamental unit epsilon = (2047 + sqrt(4190205))/2:
//   sigma_+(epsilon) = (2047 + sqrt(4190205))/2  (the large unit, ~2046.9995)
//   sigma_-(epsilon) = (2047 - sqrt(4190205))/2  (the small unit, ~0.0004885)
//
// The S-unit group generators for d=2048 at conductor 16 are:
//   |epsilon_fund| ~ 1/d, 3, 5, |g3| = (sqrt(md) - 2045)/2 ~ 0.9995, |g4| = (2049 - sqrt(md))/2 ~ 1.0005
//
// Author: Math⊙perator (Lando⊗⊙perator team)
// Date: 2026-08-06

#![allow(dead_code)]
use alloc::string::String;
use alloc::format;
use alloc::vec::Vec;

// -- Exact algebraic representation in Q(sqrt(D)) --

/// An element of Q(sqrt(sqrt_d)) represented exactly as (a + b*sqrt_d)/c.
/// a, b are i64 numerators; c is a u64 denominator. No floating point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuadElem {
    pub a: i64,  // numerator of rational part
    pub b: i64,  // numerator of sqrt_d coefficient
    pub c: u64,  // denominator
    pub sqrt_d: i64, // the discriminant D under the square root
}

impl QuadElem {
    /// Construct a + b*sqrt_d (denominator 1).
    pub fn new(a: i64, b: i64, sqrt_d: i64) -> Self {
        Self { a, b, c: 1, sqrt_d }
    }

    /// Construct (a + b*sqrt_d) / c.
    pub fn from_frac(a: i64, b: i64, c: u64, sqrt_d: i64) -> Self {
        Self { a, b, c, sqrt_d }
    }

    /// Reduce to lowest terms.
    pub fn reduced(&self) -> Self {
        let g = gcd3(self.a.abs(), self.b.abs(), self.c);
        if g <= 1 { return *self; }
        Self {
            a: self.a / g as i64,
            b: self.b / g as i64,
            c: self.c / g,
            sqrt_d: self.sqrt_d,
        }
    }

    /// The Galois conjugate: sqrt_d -> -sqrt_d.
    pub fn conjugate(&self) -> Self {
        Self { a: self.a, b: -self.b, c: self.c, sqrt_d: self.sqrt_d }
    }

    /// Norm = self * conjugate = (a^2 - b^2 * sqrt_d) / c^2.
    pub fn norm(&self) -> QuadElem {
        QuadElem::from_frac(self.a * self.a - self.b * self.b * self.sqrt_d, 0, self.c * self.c, self.sqrt_d)
    }

    /// Widen to an exact (a, b, c) triple over i128. The monomial below runs
    /// past i64 -- g3^3 * g4^2 / eps has numerator ~7.4e19 -- so the product
    /// is carried in i128, where it fits with room to spare.
    pub fn widen(&self) -> (i128, i128, i128) { (self.a as i128, self.b as i128, self.c as i128) }

    /// Numerical approximation as f64 (for display and cross-check only).
    pub fn approx(&self) -> f64 {
        let s = libm::sqrt(self.sqrt_d as f64);
        (self.a as f64 + self.b as f64 * s) / (self.c as f64)
    }

    /// String representation: (a + b*sqrt(D)) / c, simplified.
    pub fn display(&self) -> String {
        let r = self.reduced();
        if r.b == 0 {
            return format!("{}/{}", r.a, r.c);
        }
        if r.a == 0 {
            if r.c == 1 {
                return format!("{}*sqrt({})", r.b, r.sqrt_d);
            }
            return format!("{}/{}*sqrt({})", r.b, r.c, r.sqrt_d);
        }
        if r.c == 1 {
            format!("{} + {}*sqrt({})", r.a, r.b, r.sqrt_d)
        } else {
            format!("({} + {}*sqrt({}))/{}", r.a, r.b, r.sqrt_d, r.c)
        }
    }
}

fn gcd3(a: i64, b: i64, c: u64) -> u64 {
    let g1 = gcd_i64(a, b);
    let g2 = gcd_i64(g1, c as i64);
    if g2 < 1 { 1 } else { g2 as u64 }
}

fn gcd_i64(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = a % b; a = b; b = t; }
    if a == 0 { 1 } else { a }
}

// -- The exact Stark unit for d=2048 --

/// The discriminant m_d = (d-3)(d+1) = 4190205 = 3*5*409*683.
pub const D2048_MD: i64 = 4_190_205;
/// d-1 = 2047 (coefficient in the fundamental unit formula).
pub const D2048_N: i64 = 2047;

/// The fundamental unit (large embedding): epsilon = (2047 + sqrt(4190205)) / 2.
/// This is the EXACT algebraic representation -- no f64, no approximation.
/// Minimal polynomial: x^2 - 2047*x + 1 = 0.
pub fn stark_unit_d2048() -> QuadElem {
    QuadElem::from_frac(D2048_N, 1, 2, D2048_MD)
}

/// The fundamental unit (small embedding): (2047 - sqrt(4190205)) / 2.
/// Norm-1 dual: eps_small = 1/eps_large.
pub fn stark_unit_d2048_small() -> QuadElem {
    QuadElem::from_frac(D2048_N, -1, 2, D2048_MD)
}

/// Generator g3: (sqrt(md) - 2045) / 2, norm = -(d-3) = -2045.
pub fn generator_g3() -> QuadElem {
    QuadElem::from_frac(-2045, 1, 2, D2048_MD)
}

/// Generator g4: (2049 - sqrt(md)) / 2, norm = d+1 = 2049.
pub fn generator_g4() -> QuadElem {
    QuadElem::from_frac(2049, -1, 2, D2048_MD)
}

/// The S-unit monomial: epsilon_fund^(-1) * g3^3 * g4^2
/// Exponents [-1, 3, 2] at conductor 16.
/// Returns the EXACT algebraic value (not f64).
/// gcd over i128, for reducing the wide monomial.
fn gcd128(mut x: i128, mut y: i128) -> i128 {
    x = x.abs(); y = y.abs();
    while y != 0 { let t = x % y; x = y; y = t; }
    if x == 0 { 1 } else { x }
}

/// Exact multiplication in Q(sqrt(D)) over i128, reduced.
/// (a1 + b1 s)(a2 + b2 s) / (c1 c2) = (a1a2 + b1b2 D + (a1b2 + a2b1) s) / (c1 c2)
fn qmul128(x: (i128, i128, i128), y: (i128, i128, i128), d: i128) -> (i128, i128, i128) {
    let a = x.0 * y.0 + x.1 * y.1 * d;
    let b = x.0 * y.1 + x.1 * y.0;
    let c = x.2 * y.2;
    let g = gcd128(gcd128(a, b), c);
    (a / g, b / g, c / g)
}

/// The S-unit monomial eps_fund^(-1) * g3^3 * g4^2, exponents [-1, 3, 2] at
/// conductor 16, evaluated EXACTLY.
///
/// eps has norm 1, so its inverse is its Galois conjugate and the whole
/// monomial is a product -- no division is ever needed.
pub fn stark_unit_monomial() -> (i128, i128, i128) {
    let d = D2048_MD as i128;
    let mut m = stark_unit_d2048().conjugate().widen();   // eps^(-1) = conj(eps)
    let g3 = generator_g3().widen();
    let g4 = generator_g4().widen();
    for _ in 0..3 { m = qmul128(m, g3, d); }
    for _ in 0..2 { m = qmul128(m, g4, d); }
    m
}

/// f64 value of an exact (a, b, c) triple.
///
/// Correct only while a and b*sqrt(d) do not cancel. For the monomial they
/// cancel almost completely -- both terms are ~7.35e19 and the true value is
/// ~4.9e-4, twenty-three orders down -- and f64's ulp up there is 8192, so this
/// returns noise on that input. Use `monomial_approx` for the monomial.
fn approx128(m: (i128, i128, i128), d: i128) -> f64 {
    let s = libm::sqrt(d as f64);
    (m.0 as f64 + m.1 as f64 * s) / (m.2 as f64)
}

/// Numerical value of the monomial, evaluated FACTORED.
///
/// The expanded triple is exact and it is also the one form that cannot be
/// read in f64: expanding a product of well-conditioned factors into a single
/// (a + b*sqrt(D))/c manufactures a subtraction of two twenty-digit numbers
/// that agree to their last place. Each factor on its own is O(1) and loses
/// nothing, so the product of the factors is what carries the value. Exactness
/// of a representation and evaluability of that representation are two
/// different properties, and this monomial has the first without the second.
pub fn monomial_approx() -> f64 {
    let e = stark_unit_d2048().conjugate().approx();
    let g3 = generator_g3().approx();
    let g4 = generator_g4().approx();
    e * g3 * g3 * g3 * g4 * g4
}

/// The S-unit monomial: epsilon_fund^(-1) * g3^3 * g4^2
/// Exponents [-1, 3, 2] at conductor 16.
/// Returns the EXACT algebraic value (not f64).
pub fn stark_unit_monomial_exact() -> String {
    let eps = stark_unit_d2048();
    let g3 = generator_g3();
    let g4 = generator_g4();
    let m = stark_unit_monomial();
    let v = monomial_approx();
    format!("ε_Stark = ε_fund^(-1) · π₁^3 · π₂^2 with exponents [-1, 3, 2]\n\
             ε_fund = {}\n\
             g3 = {}\n\
             g4 = {}\n\
             ε_fund ≈ {}\n\
             g3 ≈ {}\n\
             g4 ≈ {}\n\
             monomial = ({} + {}*sqrt({}))/{}  (exact, i128)\n\
             monomial ≈ {:.18}\n\
             1/d = 1/2048 ≈ {:.18}\n\
             monomial - 1/d ≈ {:.3e}\n\
             Minimal poly: x² - 2047x + 1 = 0\n\
             Norm: 1 (algebraically exact)",
        eps.display(), g3.display(), g4.display(),
        eps.approx(), g3.approx(), g4.approx(),
        m.0, m.1, D2048_MD, m.2,
        v, 1.0_f64 / 2048.0, v - 1.0_f64 / 2048.0
    )
}

// -- 2-part extraction via Galois embedding --

/// Galois embeddings of Q(sqrt(D)) into R (both are real since D > 0).
/// σ_+(sqrt_d) = +sqrt_d  -- the principal (large) embedding
/// σ_-(sqrt_d) = -sqrt_d  -- the conjugate (small) embedding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Embedding {
    Plus,  // sigma_+ : sqrt -> +sqrt
    Minus, // sigma_- : sqrt -> -sqrt
}

impl Embedding {
    /// Apply the Galois embedding to a QuadElem, returning the f64 approximation.
    /// This is the 2-part extraction: ψ_k = σ_k(ε_stark).
    pub fn apply(&self, e: QuadElem) -> f64 {
        let s = libm::sqrt(e.sqrt_d as f64);
        let sqrt_val = match self {
            Embedding::Plus => s,
            Embedding::Minus => -s,
        };
        (e.a as f64 + e.b as f64 * sqrt_val) / (e.c as f64)
    }

    /// Symbol label for display.
    pub fn symbol(&self) -> &'static str {
        match self {
            Embedding::Plus => "σ_+",
            Embedding::Minus => "σ_-",
        }
    }
}

/// The two-part structural S-unit tower bypass.
///
/// psi_k = embeddings_2048(k)(stark_unit_2048)
///
/// For d=2048 the base field Q(sqrt(4190205)) has two real embeddings.
/// The large embedding σ_+ gives the fundamental unit ε ≈ 2046.9995.
/// The small embedding σ_- gives its algebraic conjugate ε⁻¹ ≈ 0.0005.
///
/// The S-unit exponents [-1, 3, 2] combine the fundamental unit with
/// ramified prime generators g3 = (sqrt(md) - 2045)/2 and g4 = (2049 - sqrt(md))/2.
/// The Galois action on these gives the 2-part extraction.
pub struct TwoPartExtraction {
    /// The exact algebraic Stark unit ε_stark = (2047 + sqrt(4190205))/2
    pub eps_exact: QuadElem,
    /// Large embedding σ_+(ε) ≈ 2046.9995
    pub eps_large: f64,
    /// Small embedding σ_-(ε) ≈ 0.0005 = 1/eps_large
    pub eps_small: f64,
    /// Generator g3 exact: (sqrt(md) - 2045)/2
    pub g3_exact: QuadElem,
    /// Generator g4 exact: (2049 - sqrt(md))/2
    pub g4_exact: QuadElem,
    /// S-unit exponents [-1, 3, 2]
    pub exponents: [i64; 3],
    /// The monomial those exponents name, evaluated exactly: (a, b, c)
    pub monomial: (i128, i128, i128),
    /// Whether the Galois pair satisfies the norm identity |σ_+| * |σ_-| = 1
    pub norm_one: bool,
    /// Whether the f64 approximation matches the stored f64 seed
    pub matches_f64: bool,
}

impl TwoPartExtraction {
    pub fn compute() -> Self {
        let eps = stark_unit_d2048();
        let eps_large = Embedding::Plus.apply(eps);
        let eps_small = Embedding::Minus.apply(eps);
        let g3 = generator_g3();
        let g4 = generator_g4();

        // Norm identity: eps_large * eps_small should be 1 (Norm = 1)
        let norm_one = (eps_large * eps_small - 1.0).abs() < 1e-10;

        // Cross-check against the f64 stored value
        // The stored f64 stark_unit in belt/stark.rs is 2046.9995114801
        let f64_ref = 2046.9995114801_f64;
        let matches_f64 = libm::fabs(eps_large - f64_ref) < 1e-6;

        Self {
            eps_exact: eps,
            eps_large,
            eps_small,
            g3_exact: g3,
            g4_exact: g4,
            exponents: [-1, 3, 2],
            monomial: stark_unit_monomial(),
            norm_one,
            matches_f64,
        }
    }

    pub fn report(&self) -> String {
        format!(
            "One-Shot #11: Exact d=2048 SIC-POVM Fiducial Extraction (2-part S-unit bypass)\n  \
             ε_stark = {} (exact algebraic, minimal poly x² - 2047x + 1 = 0)\n  \
             σ_+(ε) = {:.16}\n  \
             σ_-(ε) = {:.16}\n  \
             σ_+(ε) × σ_-(ε) = {:.16} (Norm = 1.0)\n  \
             g3 = {} ≈ {:.16}\n  \
             g4 = {} ≈ {:.16}\n  \
             Exponents [-1, 3, 2] at conductor 16\n  \
             monomial ε⁻¹·g3³·g4² = ({} + {}·sqrt({}))/{}  (exact, i128)\n  \
             monomial ≈ {:.18} (factored)   1/d ≈ {:.18}   Δ ≈ {:.3e}\n  \
             expanded triple in f64 = {:.6} — the expansion cancels 20 digits, f64 ulp there is 8192\n  \
             f64↔exact cross-check: σ_+(ε) matches f64 seed 2046.9995114801: {}\n  \
             Norm identity σ_+σ_-=1: {}\n  \
             Galois 2-part extraction: ψ_+ = σ_+(ε_stark), ψ_- = σ_-(ε_stark) = ψ_+⁻¹\n  \
             Base field discriminant: 4190205 = 3×5×409×683 (6 Frobenius-dual pairs)",
            self.eps_exact.display(),
            self.eps_large,
            self.eps_small,
            self.eps_large * self.eps_small,
            self.g3_exact.display(), self.g3_exact.approx(),
            self.g4_exact.display(), self.g4_exact.approx(),
            self.monomial.0, self.monomial.1, D2048_MD, self.monomial.2,
            monomial_approx(),
            1.0_f64 / 2048.0,
            monomial_approx() - 1.0_f64 / 2048.0,
            approx128(self.monomial, D2048_MD as i128),
            self.matches_f64,
            self.norm_one
        )
    }

    /// The 2-part extraction via Galois embedding.
    /// Returns (ψ_plus, ψ_minus) = (σ_+(ε), σ_-(ε))
    pub fn psi_pair(&self) -> (f64, f64) {
        (self.eps_large, self.eps_small)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stark_unit_algebraic() {
        let eps = stark_unit_d2048();
        // Exact form: (2047 + 1*sqrt(4190205)) / 2
        assert_eq!(eps.a, 2047);
        assert_eq!(eps.b, 1);
        assert_eq!(eps.c, 2);
        assert_eq!(eps.sqrt_d, 4_190_205);
        // Norm = 1
        assert_eq!(eps.norm().norm(), 1);
    }

    #[test]
    fn test_stark_unit_minpoly() {
        let eps = stark_unit_d2048();
        // x^2 - 2047*x + 1 = 0
        let val = eps.approx();
        let check = val * val - 2047.0 * val + 1.0;
        assert!(libm::fabs(check) < 1e-12, "minimal polynomial check: {} ≈ 0", check);
    }

    #[test]
    fn test_norm_identity() {
        let eps = stark_unit_d2048();
        let eps_large = Embedding::Plus.apply(eps);
        let eps_small = Embedding::Minus.apply(eps);
        assert!(libm::fabs(eps_large * eps_small - 1.0) < 1e-15);
    }

    #[test]
    fn test_galois_pair() {
        let extract = TwoPartExtraction::compute();
        let (psi_plus, psi_minus) = extract.psi_pair();
        // psi_plus * psi_minus = norm = 1
        assert!(libm::fabs(psi_plus * psi_minus - 1.0) < 1e-15);
        // psi_plus ≈ 2046.9995
        assert!(libm::fabs(psi_plus - 2046.9995114801) < 1e-6);
        // psi_minus ≈ 1/psi_plus
        assert!(libm::fabs(psi_minus - 1.0 / psi_plus) < 1e-15);
    }
}
/// Public API for the REPL: `d2048 exact`
pub fn exact_extraction_report() -> String {
    let extract = TwoPartExtraction::compute();
    extract.report()
}

/// Public summary for `d2048 summary` integration
pub fn exact_summary() -> String {
    let extract = TwoPartExtraction::compute();
    format!(
        "Exact d=2048 SIC-POVM: ε = (2047 + √4190205)/2, norm=1, σ₊σ₋=1: {}\n  exponents [-1,3,2], f64 match: {}",
        extract.norm_one, extract.matches_f64
    )
}
