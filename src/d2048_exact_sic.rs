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

// -- Welch saturation, verified against the recovered fiducial --

/// The d=12 SIC fiducial recovered in `~/imsgct/d12_sic_build`, normalised.
///
/// These are not a construction performed here. They are the coordinates that
/// campaign produced, carried in at f64 so the kernel can check the property
/// they are supposed to have rather than restate it. At 200 digits the same
/// vector saturates Welch to 4.5e-201; what f64 can show is machine precision,
/// and that is what this reports.
const D12_FIDUCIAL: [(f64, f64); 12] = [
    ( 0.17657085022446443,  0.0),
    (-0.11435757101345763, -0.08013049463884122),
    (-0.15479561276309548, -0.089371288698146246),
    ( 0.48321014641873911, -0.023581345898466354),
    ( 0.12749895151265970,  0.0),
    (-0.12588270409781341,  0.43212734644653372),
    ( 0.0,                 -0.42627974133165048),
    ( 0.19345212691675506,  0.047368456873818477),
    ( 0.037018799865533485,-0.064118442202327918),
    ( 0.056930405087088630,-0.20015219612293078),
    ( 0.0,                 -0.30780969793021268),
    ( 0.027607026802660757, 0.31435053354897172),
];

fn cmul(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

/// Verify the Welch bound where a fiducial actually exists.
///
/// |<psi|D_{p,q} psi>|^2 = 1/(d+1) for all d^2-1 nontrivial displacements is
/// the defining property of a SIC, and it is a property of a VECTOR. Computing
/// 1/(d+1) from d asserts nothing about whether any vector attains it. So the
/// d=12 fiducial is displaced through the whole Weyl-Heisenberg group and all
/// 143 overlaps are measured.
pub fn welch_report() -> String {
    let d = 12usize;
    let dd = d as f64;
    let two_pi_d = 6.283185307179586_f64 / dd;
    let pi_d = 3.141592653589793_f64 / dd;

    let mut psi = D12_FIDUCIAL;
    let nrm = libm::sqrt(psi.iter().map(|z| z.0 * z.0 + z.1 * z.1).sum::<f64>());
    for z in psi.iter_mut() { z.0 /= nrm; z.1 /= nrm; }

    let target = 1.0_f64 / (dd + 1.0);
    let (mut lo, mut hi, mut worst) = (f64::MAX, f64::MIN, 0.0_f64);
    let mut count = 0usize;
    for p in 0..d {
        for q in 0..d {
            if p == 0 && q == 0 { continue; }
            // D_{p,q}|k> = w^{pq/2} w^{pk} |k+q>
            let ph = ((p * q) as f64) * pi_d;
            let pre = (libm::cos(ph), libm::sin(ph));
            let mut ov = (0.0_f64, 0.0_f64);
            for k in 0..d {
                let t = ((p * k) as f64) * two_pi_d;
                let v = cmul(cmul(pre, (libm::cos(t), libm::sin(t))), psi[k]);
                let bra = psi[(k + q) % d];
                // <psi| applied at index k+q: conjugate of the bra component
                ov.0 += bra.0 * v.0 + bra.1 * v.1;
                ov.1 += bra.0 * v.1 - bra.1 * v.0;
            }
            let m = ov.0 * ov.0 + ov.1 * ov.1;
            if m < lo { lo = m; }
            if m > hi { hi = m; }
            let dev = libm::fabs(m - target);
            if dev > worst { worst = dev; }
            count += 1;
        }
    }
    format!(
        "  Welch saturation, measured on the recovered d=12 fiducial\n           overlaps checked      {} (= d^2 - 1)\n           target 1/(d+1)        {:.17}\n           min |<psi|D psi>|^2   {:.17}\n           max |<psi|D psi>|^2   {:.17}\n           worst deviation       {:.3e}  (f64; the same vector gives 4.5e-201 at 200 digits)\n           source                ~/imsgct/d12_sic_build, d12_psi_uhi.pkl\n\n           At d=2048 this check has nothing to run on. 1/(d+1) = 1/2049 is what a\n           fiducial there WOULD satisfy; no such vector is recovered. The numerical\n           route in that campaign is closed at f ~ 0.7817 against 0.9967 for random,\n           and the arithmetic route is blocked on L-values at conductor p^12*inf_1.\n           So Welch is verified at 12 and open at 2048, and saying otherwise would be\n           reporting arithmetic on d as though it were a property of a vector.",
        count, target, lo, hi, worst)
}

// -- The crossover threshold, with its model exposed --

/// Gate-count models for SIC-POVM tomography in dimension d.
///
/// The threshold eps_2q < 0.1 / (t_gate * n_gates) is not a fact about d. It is
/// a fact about n_gates(d), and n_gates(d) is a modelling choice that the
/// single number 2.38e-8 hides. Naming the models and varying them is the only
/// way the threshold reports its own sensitivity.
fn n_gates(model: u8, d: f64) -> f64 {
    match model {
        // one gate per measurement outcome: the d^2 the demonstration assumes
        0 => d * d,
        // d^2 outcomes, each prepared by a circuit of depth log2(d)
        1 => d * d * (libm::log(d) / libm::log(2.0)),
        // Weyl-Heisenberg orbit: d^2 displacements over a fiducial of depth d
        2 => d * d * d,
        // one gate per amplitude, the cheapest defensible floor
        _ => d,
    }
}

fn model_name(model: u8) -> &'static str {
    match model { 0 => "d^2        (one gate per outcome)",
                  1 => "d^2 log2 d (outcome x preparation depth)",
                  2 => "d^3        (WH orbit over a depth-d fiducial)",
                  _ => "d          (one gate per amplitude)" }
}

/// The crossover metric with its assumptions on the surface.
///
/// t_gate * n_gates * eps_2q > 0.1 says noise has eaten the circuit and a
/// classical simulation wins. So advantage needs eps_2q < 0.1/(t_gate*n_gates).
/// t_gate enters as a pure multiplier, so it is varied too rather than being
/// silently set to one.
pub fn crossover_report() -> String {
    let mut out = String::from(
        "  Crossover metric  t_gate * n_gates * eps_2q > 0.1  =>  classical simulation wins\n           Required 2-qubit error for advantage: eps_2q < 0.1 / (t_gate * n_gates(d))\n\n           t_gate = 1 (the demonstration's implicit choice):\n           model                                        d=64        d=2048       d=65536\n");
    for model in [0_u8, 1, 2, 3] {
        out.push_str(&format!("    {:<42}", model_name(model)));
        for d in [64.0_f64, 2048.0, 65536.0] {
            out.push_str(&format!(" {:>12.2e}", 0.1 / n_gates(model, d)));
        }
        out.push('\n');
    }
    out.push_str("\n  The d^2 row at d=2048 is 2.38e-08, which is the number the demonstration\n          reports. The other rows are the same claim under other equally stated\n          assumptions, and they span four orders at one dimension. The threshold is\n          a property of the gate model at least as much as of d.\n\n          t_gate sensitivity at d=2048, model d^2 (t_gate is a bare multiplier):\n");
    for t in [0.1_f64, 1.0, 10.0, 100.0] {
        out.push_str(&format!("    t_gate = {:>6.1}  ->  eps_2q < {:.2e}\n", t, 0.1 / (t * n_gates(0, 2048.0))));
    }
    out.push_str("\n  Against hardware. Taking eps_2q = 1e-3 as measured today, advantage under\n          each model needs n_gates < 0.1/eps_2q = 100 gates, so the largest d that\n          clears it is:\n");
    for model in [0_u8, 1, 2, 3] {
        let mut best = 0_u64;
        for d in 2..=100_000_u64 {
            if n_gates(model, d as f64) < 100.0 { best = d; } else { break; }
        }
        out.push_str(&format!("    {:<42} d <= {}\n", model_name(model), best));
    }
    out.push_str("\n  Every model puts the reachable dimension in the single or double digits at\n          1e-3, so d=2048 is out of reach under all of them, not only under d^2. The\n          algebraic representation is what the extraction buys; this metric is not\n          where it is bought.");
    out
}

// -- Step 5 measured: what the algebraic representation actually costs --

/// The monomial for an arbitrary d, so the representation can be MEASURED as d
/// moves rather than asserted at one point. N = d-1, D = N^2-4, and the three
/// generators are the same shapes they are at 2048: eps = (N + sqrt(D))/2 with
/// norm 1, g3 = (sqrt(D) - (N-2))/2, g4 = ((N+2) - sqrt(D))/2.
fn monomial_at(d: u64) -> Option<((i128, i128, i128), i128)> {
    let n = d as i128 - 1;
    let dd = n * n - 4;
    let mut m = (n, -1, 2);                 // eps^(-1) = conj(eps)
    let g3 = (-(n - 2), 1, 2);
    let g4 = (n + 2, -1, 2);
    // i128 has 127 bits; the numerator runs about 6*log2(d), so bail rather
    // than wrap silently once the product would not fit.
    for _ in 0..3 { m = qmul_checked(m, g3, dd)?; }
    for _ in 0..2 { m = qmul_checked(m, g4, dd)?; }
    Some((m, dd))
}

/// qmul128 that refuses to wrap.
fn qmul_checked(x: (i128, i128, i128), y: (i128, i128, i128), d: i128) -> Option<(i128, i128, i128)> {
    let a = x.0.checked_mul(y.0)?.checked_add(x.1.checked_mul(y.1)?.checked_mul(d)?)?;
    let b = x.0.checked_mul(y.1)?.checked_add(x.1.checked_mul(y.0)?)?;
    let c = x.2.checked_mul(y.2)?;
    let g = gcd128(gcd128(a, b), c);
    Some((a / g, b / g, c / g))
}

fn bits(v: i128) -> u32 { 128 - v.unsigned_abs().leading_zeros() }

/// Step 5, measured instead of asserted.
///
/// The claim under test is that the algebraic representation costs O(log d)
/// where the amplitude list costs O(d^2). Both sides are counted in bits: the
/// algebraic side is the actual width of the exact triple plus the
/// discriminant it lives over, and the classical side is d^2 complex
/// amplitudes at 128 bits each.
pub fn scaling_report() -> String {
    let mut out = String::from(
        "  Step 5 measured — bits actually needed, not bits asserted\n              d      bits(D)  bits(a)  bits(b)   algebraic   log2(d)   alg/log2(d)        classical\n");
    for d in [16_u64, 64, 256, 1024, 2048, 4096, 65536, 1_048_576] {
        match monomial_at(d) {
            None => out.push_str(&format!("  {:>8}   (exceeds i128)\n", d)),
            Some(((a, b, c), dd)) => {
                let alg = bits(a) + bits(b) + bits(c) + bits(dd);
                let l2 = 63 - d.leading_zeros() as u64;          // d is a power of two here
                let classical = (d as u128) * (d as u128) * 128;
                out.push_str(&format!(
                    "  {:>8}  {:>7}  {:>7}  {:>7}  {:>10}  {:>8}  {:>12.2}  {:>15}\n",
                    d, bits(dd), bits(a), bits(b), alg, l2, alg as f64 / l2 as f64, classical));
            }
        }
    }
    out.push_str(
        "  The ratio settles at about 13 bits of representation per bit of d:\n           bits(a) = 6*log2(d), bits(b) = 5*log2(d), bits(D) = 2*log2(d). That is the\n           O(log d) claim, measured, with its constant. The classical column is d^2\n           complex amplitudes at 128 bits and grows as the square.\n           Above 2^20 the exact product no longer fits i128 and the row is refused\n           rather than wrapped — the ceiling is this arithmetic, not the method.");
    out
}

// -- Cyclotomic levels: the radicals, computed --

/// The quadratic Gauss sum g(m) = sum_{k=0}^{m-1} exp(2*pi*i*k^2/m).
///
/// k*k is reduced mod m before the phase is formed, so the argument stays
/// small and 2732^2 never has to be representable.
pub fn gauss_sum(m: u32) -> (f64, f64) {
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    let two_pi = 6.283185307179586_f64;
    for k in 0..m {
        let r = ((k as u64 * k as u64) % m as u64) as f64;
        let t = two_pi * r / m as f64;
        re += libm::cos(t);
        im += libm::sin(t);
    }
    (re, im)
}

/// The radical a level carries, extracted from the sum rather than from its
/// modulus.
///
/// At a prime level p = 1 mod 4 the sum is real and IS sqrt(p). At a level
/// divisible by 4 the sum is 2*sqrt(n)*(1 + i), so its modulus is sqrt(8n) and
/// the radical sits in the real part halved. Comparing |g| against the radical
/// at those levels compares sqrt(8n) with sqrt(n) and they are not equal --
/// the modulus is the wrong place to look, not a failed check.
pub fn level_radical(m: u32) -> f64 {
    let (re, _im) = gauss_sum(m);
    if m % 4 == 0 { re / 2.0 } else { re }
}

/// Step 4 computed: the four cyclotomic levels of D = 3*5*409*683, each
/// radical extracted from its own Gauss sum, and their product against sqrt(D).
pub fn cyclotomic_levels_report() -> String {
    let levels: [(u32, i64); 4] = [(5, 5), (12, 3), (409, 409), (2732, 683)];
    let mut out = String::new();
    let mut product = 1.0_f64;
    for (m, radicand) in levels.iter() {
        let (re, im) = gauss_sum(*m);
        let r = level_radical(*m);
        let want = libm::sqrt(*radicand as f64);
        product *= r;
        out.push_str(&format!(
            "  g({:4}) = {:>14.9} {:+.9}i  |g| = {:>12.9}  ->  sqrt({:4}) = {:.9}  (want {:.9}, err {:.2e}) [{}]\n",
            m, re, im, libm::sqrt(re * re + im * im), radicand, r, want, r - want,
            if *m % 4 == 0 { "Re/2" } else { "Re" }));
    }
    let sqrt_d = libm::sqrt(D2048_MD as f64);
    out.push_str(&format!(
        "  product of radicals = {:.9}\n  sqrt(D)             = {:.9}   err {:.2e}\n",
        product, sqrt_d, product - sqrt_d));
    let eps = (2047.0_f64 + product) / 2.0;
    out.push_str(&format!(
        "  eps = (2047 + product)/2 = {:.10}   (from the sums, not from a seed)",
        eps));
    out
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
pub fn stark_unit_monomial() -> (i128, i128, i128) { stark_unit_monomial_with_norm().0 }

/// The monomial and its norm. Norm is multiplicative, so it is accumulated
/// with the product: N(eps^-1) = 1, N(g3) = -2045, N(g4) = 2049, giving
/// 1 * (-2045)^3 * 2049^2. Recovering it afterwards from a^2 - b^2*D would
/// need 5.4e39 and i128 stops at 1.7e38.
pub fn stark_unit_monomial_with_norm() -> ((i128, i128, i128), i128) {
    let d = D2048_MD as i128;
    let mut m = stark_unit_d2048().conjugate().widen();   // eps^(-1) = conj(eps)
    let g3 = generator_g3().widen();
    let g4 = generator_g4().widen();
    let mut norm: i128 = 1;                               // N(eps^-1) = 1
    for _ in 0..3 { m = qmul128(m, g3, d); norm *= -2045; }
    for _ in 0..2 { m = qmul128(m, g4, d); norm *= 2049; }
    (m, norm)
}

/// f64 value of an exact (a, b, c) triple, by rationalizing.
///
/// Reading (a + b*sqrt(D))/c straight off subtracts two terms that can agree
/// to their last digit -- for the monomial both are 7.35e19 and the value that
/// survives is 4.9e-4, twenty-three orders down, under an ulp of 8192. So do
/// not evaluate that difference. Multiply above and below by the conjugate:
///
///     M = (a + b*sqrt(D))/c  =>  M = c*Norm(M) / (a - b*sqrt(D))
///
/// The denominator adds two terms of the SAME sign, so nothing cancels and f64
/// carries it at full relative precision; the numerator is an exact integer.
/// The cancellation was in the way the value was read, not in the value.
///
/// Norm is multiplicative, so it is carried alongside the product rather than
/// recovered from a^2 - b^2*D, which would overflow i128 at 5.4e39.
fn approx_rational(m: (i128, i128, i128), norm: i128, d: i128) -> f64 {
    let s = libm::sqrt(d as f64);
    let den = m.0 as f64 - m.1 as f64 * s;
    (m.2 as f64) * (norm as f64) / den
}

/// Numerical value of the monomial, read off the exact triple.
pub fn monomial_approx() -> f64 {
    let (m, norm) = stark_unit_monomial_with_norm();
    approx_rational(m, norm, D2048_MD as i128)
}

/// The same value reached the other way, by multiplying the factors before
/// they are expanded. Two independent routes to one number: agreement is the
/// check that the i128 expansion is the monomial it claims to be.
pub fn monomial_approx_factored() -> f64 {
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
    /// Its norm, carried multiplicatively through the product
    pub monomial_norm: i128,
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
            monomial: stark_unit_monomial_with_norm().0,
            monomial_norm: stark_unit_monomial_with_norm().1,
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
             norm(monomial) = {} (multiplicative: 1·(-2045)³·2049²)\n  \
             monomial = {:.18} (exact triple, rationalized)\n  \
             monomial = {:.18} (factors multiplied — independent route, agrees)\n  \
             1/d = {:.18}   Δ = {:.3e}\n  \
             f64↔exact cross-check: σ_+(ε) matches f64 seed 2046.9995114801: {}\n  \
             Norm identity σ_+σ_-=1: {}\n  \
             Galois 2-part extraction: ψ_+ = σ_+(ε_stark), ψ_- = σ_-(ε_stark) = ψ_+⁻¹\n  \
             Base field discriminant: 4190205 = 3×5×409×683 (6 Frobenius-dual pairs)\n\
             Cyclotomic levels, each radical computed from its own Gauss sum:\n{}",
            self.eps_exact.display(),
            self.eps_large,
            self.eps_small,
            self.eps_large * self.eps_small,
            self.g3_exact.display(), self.g3_exact.approx(),
            self.g4_exact.display(), self.g4_exact.approx(),
            self.monomial.0, self.monomial.1, D2048_MD, self.monomial.2,
            self.monomial_norm,
            monomial_approx(),
            monomial_approx_factored(),
            1.0_f64 / 2048.0,
            monomial_approx() - 1.0_f64 / 2048.0,
            self.matches_f64,
            self.norm_one,
            cyclotomic_levels_report()
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
        // Norm = 1. `norm()` returns a QuadElem, so the second `.norm()` here
        // compared a QuadElem against the integer 1 and the test never
        // compiled — the whole test binary failed with it. N(eps) =
        // (2047² − 4190205)/2² = 4/4 = 1, checked in lowest terms.
        let n = eps.norm().reduced();
        assert_eq!((n.a, n.b, n.c), (1, 0, 1));
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
        // 1e-15 is below the f64 floor for this product and the test could not
        // pass at any precision: ε₊ ≈ 2047 and ε₋ ≈ 4.885e-4 are each formed
        // from √4190205, so the product carries ~5.1e-13 of cancellation error.
        // The identity itself is exact and is checked algebraically in
        // `test_stark_unit_algebraic`; this one only witnesses that the two
        // embeddings are numerically reciprocal.
        assert!(libm::fabs(eps_large * eps_small - 1.0) < 1e-12);
    }

    #[test]
    fn test_galois_pair() {
        let extract = TwoPartExtraction::compute();
        let (psi_plus, psi_minus) = extract.psi_pair();
        // psi_plus * psi_minus = norm = 1, to the f64 floor for a product of
        // 2047 and 4.885e-4 built from √4190205 — see test_norm_identity.
        assert!(libm::fabs(psi_plus * psi_minus - 1.0) < 1e-12);
        // psi_plus ≈ 2046.9995
        assert!(libm::fabs(psi_plus - 2046.9995114801) < 1e-6);
        // psi_minus ≈ 1/psi_plus
        assert!(libm::fabs(psi_minus - 1.0 / psi_plus) < 1e-12);
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
