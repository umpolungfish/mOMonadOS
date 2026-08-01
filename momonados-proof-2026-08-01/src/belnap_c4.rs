// rebis/belnap_c4.rs — Belnap Complex Plane C₄
// Full port of red-hot_rebis/rhr_p4rky/belnap_c4.py
//
// Extends Belnap FOUR-valued logic (B4) with the Belnap complex plane C₄,
// where quantum amplitudes live:
//   C₄ = {a + bi | a, b ∈ {T, F, B, N}}
//
// The imaginary unit satisfies i² = B (both true and false), reflecting the
// dialetheic structure of quantum superposition. The Born rule is a projection
// from C₄ to classical probability via P = proj_T(|ψ|²).

use alloc::string::String;
use crate::belnap::{B4, band, bor, bnot};
use crate::sprintln;

// ─── Belnap Complex Number ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BelnapComplex {
    pub real: B4,
    pub imag: B4,
}

impl BelnapComplex {
    pub const fn new(real: B4, imag: B4) -> Self {
        BelnapComplex { real, imag }
    }

    /// Zero: 0 + 0i (N + Ni)
    pub const fn zero() -> Self {
        BelnapComplex { real: B4::N, imag: B4::N }
    }

    /// One: 1 + 0i (T + Ni)
    pub const fn one() -> Self {
        BelnapComplex { real: B4::T, imag: B4::N }
    }

    /// Imaginary unit: 0 + 1i (N + Ti) — satisfies i² = B
    pub const fn i() -> Self {
        BelnapComplex { real: B4::N, imag: B4::T }
    }

    /// Both unit: B + 0i
    pub const fn both() -> Self {
        BelnapComplex { real: B4::B, imag: B4::N }
    }

    /// Conjugate: (a+bi)* = a + (¬b)i
    pub fn conjugate(&self) -> BelnapComplex {
        BelnapComplex { real: self.real, imag: bnot(self.imag) }
    }

    /// Magnitude squared: |a+bi|² = a² ⊕ b² using Belnap arithmetic
    /// a² = band(a, a) = a (since band is idempotent for B4)
    /// So |a+bi|² = bor(a, b)
    pub fn magnitude_squared(&self) -> B4 {
        bor(self.real, self.imag)
    }

    /// Born probability: projection of |ψ|² onto classical truth probability
    /// P = proj_T(|ψ|²) — maps {T↦1.0, B↦0.5, F↦0.0, N↦0.0}
    pub fn born_probability(&self) -> f32 {
        match self.magnitude_squared() {
            B4::T => 1.0,
            B4::B => 0.5,
            B4::F => 0.0,
            B4::N => 0.0,
        }
    }

    /// All 16 elements of C₄
    pub fn all_elements() -> [BelnapComplex; 16] {
        let vals = [B4::N, B4::F, B4::T, B4::B]; // N=0, F=1, T=2, B=3 ordering
        let mut result: [BelnapComplex; 16] = [BelnapComplex::zero(); 16];
        let mut idx = 0;
        for &r in &vals {
            for &i in &vals {
                result[idx] = BelnapComplex { real: r, imag: i };
                idx += 1;
            }
        }
        result
    }

    pub fn symbol(&self) -> &'static str {
        match (self.real, self.imag) {
            (B4::N, B4::N) => "0",
            (B4::T, B4::N) => "1",
            (B4::N, B4::T) => "i",
            (B4::T, B4::T) => "1+i",
            (B4::F, B4::N) => "f",
            (B4::N, B4::F) => "fi",
            (B4::B, B4::N) => "b",
            (B4::N, B4::B) => "bi",
            _ => "c4",
        }
    }
}// ─── Arithmetic ───────────────────────────────────────────────────────────────

/// (a+bi) + (c+di) = (a⊕c) + (b⊕d)i using Belnap join (bor)
pub fn c4_add(a: &BelnapComplex, b: &BelnapComplex) -> BelnapComplex {
    BelnapComplex {
        real: bor(a.real, b.real),
        imag: bor(a.imag, b.imag),
    }
}

/// (a+bi) - (c+di) = (a∧¬c) + (b∧¬d)i
pub fn c4_sub(a: &BelnapComplex, b: &BelnapComplex) -> BelnapComplex {
    BelnapComplex {
        real: band(a.real, bnot(b.real)),
        imag: band(a.imag, bnot(b.imag)),
    }
}

/// (a+bi)(c+di) = (ac − bd) + (ad + bc)i. i² = B (dialetheic)
/// ac = band(a, c), bd = band(b, d)
/// bd * i² = band(bd, B)
/// real = band(ac, ¬(band(bd, B)))
pub fn c4_mul(a: &BelnapComplex, b: &BelnapComplex) -> BelnapComplex {
    let ac = band(a.real, b.real);
    let bd = band(a.imag, b.imag);
    let bd_times_b = band(bd, B4::B);
    let real_part = band(ac, bnot(bd_times_b));
    let ad = band(a.real, b.imag);
    let bc = band(a.imag, b.real);
    let imag_part = bor(ad, bc);
    BelnapComplex { real: real_part, imag: imag_part }
}

/// Inner product: ⟨ψ|φ⟩ = conj(ψ)·φ (as C₄)
pub fn c4_inner(psi: &BelnapComplex, phi: &BelnapComplex) -> BelnapComplex {
    c4_mul(&psi.conjugate(), phi)
}

/// Tensor product: ψ ⊗ φ = (ψ·real ⊕ φ·real) + (ψ·imag ⊕ φ·imag)i
pub fn c4_tensor(a: &BelnapComplex, b: &BelnapComplex) -> BelnapComplex {
    BelnapComplex {
        real: bor(a.real, b.real),
        imag: bor(a.imag, b.imag),
    }
}

/// Display a C₄ element
pub fn c4_format(z: &BelnapComplex) -> alloc::string::String {
    let r = match z.real { B4::T => "T", B4::F => "F", B4::B => "B", B4::N => "N" };
    let i = match z.imag { B4::T => "+Ti", B4::F => "+Fi", B4::B => "+Bi", B4::N => "" };
    if i.is_empty() && z.real == B4::N {
        alloc::format!("N")
    } else if i.is_empty() {
        alloc::format!("{}", r)
    } else if z.real == B4::N {
        alloc::format!("{}", i.trim_start_matches('+'))
    } else {
        alloc::format!("{}{}", r, i)
    }
}

/// Multiplication table for C₄
pub fn c4_multiplication_table() -> alloc::vec::Vec<alloc::vec::Vec<String>> {
    let vals = [B4::N, B4::F, B4::T, B4::B];
    let mut table = alloc::vec::Vec::new();
    for &r1 in &vals {
        for &i1 in &vals {
            let mut row = alloc::vec::Vec::new();
            let a = BelnapComplex::new(r1, i1);
            for &r2 in &vals {
                for &i2 in &vals {
                    let b = BelnapComplex::new(r2, i2);
                    let prod = c4_mul(&a, &b);
                    row.push(c4_format(&prod));
                }
            }
            table.push(row);
        }
    }
    table
}

// ─── Born Rule Analysis ───────────────────────────────────────────────────────

/// Full Born rule table: for each C₄ element, show |ψ|² and P(T|ψ)
pub fn c4_born_table() {
    
    sprintln!("═════════════════════════════════════════════════════════");
    sprintln!("  C₄ Born Rule — Full Projection Table");
    sprintln!("  P = proj_T(|ψ|²)  where |ψ|² = a ⊕ b  (Belnap join)");
    sprintln!("═════════════════════════════════════════════════════════");
    sprintln!("  ψ = a+bi     |ψ|²     P(T)    Born outcome");
    sprintln!("  ──────────  ──────  ──────  ─────────────");

    for z in BelnapComplex::all_elements() {
        let msq = z.magnitude_squared();
        let prob = z.born_probability();
        let label = match msq {
            B4::T => "certain",
            B4::B => "both/and",
            B4::F => "impossible",
            B4::N => "undetermined",
        };
        sprintln!("  {:<10}  {:<6}  {:>5.2}  {}",
            c4_format(&z), c4_format(&BelnapComplex::new(msq, B4::N)), prob, label);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let z = BelnapComplex::zero();
        assert_eq!(z.real, B4::N);
        assert_eq!(z.imag, B4::N);
    }

    #[test]
    fn test_one() {
        let z = BelnapComplex::one();
        assert_eq!(z.real, B4::T);
        assert_eq!(z.imag, B4::N);
    }

    #[test]
    fn test_i_squared() {
        let i = BelnapComplex::i();
        let i2 = c4_mul(&i, &i);
        // i² = B (both true and false)
        assert_eq!(i2.real, B4::B);
        assert_eq!(i2.imag, B4::N);
    }

    #[test]
    fn test_conjugate() {
        let z = BelnapComplex::new(B4::T, B4::F);
        let c = z.conjugate();
        assert_eq!(c.real, B4::T);
        assert_eq!(c.imag, B4::T); // ¬F = T
    }

    #[test]
    fn test_born_probability() {
        let z = BelnapComplex::new(B4::T, B4::N); // |1|² = T → P=1
        assert!((z.born_probability() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_add_commutative() {
        let a = BelnapComplex::new(B4::T, B4::N);
        let b = BelnapComplex::new(B4::N, B4::T);
        assert_eq!(c4_add(&a, &b), c4_add(&b, &a));
    }

    #[test]
    fn test_all_elements_count() {
        assert_eq!(BelnapComplex::all_elements().len(), 16);
    }
}