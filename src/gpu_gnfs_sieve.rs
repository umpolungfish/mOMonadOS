//! gpu_gnfs_sieve.rs — ≻∈⊤⊥⊞⊡≺ classical log-line sieving and relations.
//!
//! Rational side: |a − b m|. Algebraic side: homogeneous |N_f(a,b)|.
//! Both must be B-smooth. Device runs the classical log-line sieve
//! (per-prime / per-ideal hits, including p^k, then log|norm| thresholds).
//! Host assigns ideal exponents / sign and exact-smoothness-checks survivors.
//! Arbitrary-size m is first-class: hits use (m mod p); thresholds use log(m).

use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use cudarc::driver::{
    CudaContext, CudaFunction, CudaSlice, CudaStream, LaunchConfig, PushKernelArg,
};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions, Ptx};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed, Zero};

use crate::gpu_gnfs_poly::PolyPair;

/// Algebraic prime ideal (p, r) with f(r) ≡ 0 (mod p).
#[derive(Clone, Copy, Debug)]
pub struct AlgIdeal {
    pub p: u64,
    pub r: u64,
}

#[derive(Clone, Debug)]
pub struct FactorBases {
    pub rational: Vec<u64>,
    pub algebraic: Vec<AlgIdeal>,
}

impl FactorBases {
    pub fn build(poly: &PolyPair, b: u64) -> Self {
        let rational = primes_upto(b);
        let mut algebraic = Vec::new();
        for &p in &rational {
            if p < 2 {
                continue;
            }
            for r in roots_mod_p(poly, p) {
                algebraic.push(AlgIdeal { p, r });
            }
        }
        Self {
            rational,
            algebraic,
        }
    }

    pub fn ncols(&self) -> usize {
        1 + self.rational.len() + self.algebraic.len()
    }
}

#[derive(Clone, Debug)]
pub struct Relation {
    pub a: i64,
    pub b: i64,
    pub exps: Vec<u8>,
    pub rat_signed: BigInt,
    pub rat_abs: BigUint,
    pub alg_abs: BigUint,
}

pub fn primes_upto(bound: u64) -> Vec<u64> {
    if bound < 2 {
        return Vec::new();
    }
    let n = bound as usize;
    let mut is_p = alloc::vec![true; n + 1];
    is_p[0] = false;
    is_p[1] = false;
    let mut p = 2usize;
    while p * p <= n {
        if is_p[p] {
            let mut m = p * p;
            while m <= n {
                is_p[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    let mut out = Vec::new();
    for i in 2..=n {
        if is_p[i] {
            out.push(i as u64);
        }
    }
    out
}

fn poly_trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && a.last() == Some(&0) {
        a.pop();
    }
    if a.is_empty() {
        a.push(0);
    }
    a
}

fn poly_deg(a: &[u64]) -> isize {
    let mut i = a.len() as isize - 1;
    while i > 0 && a[i as usize] == 0 {
        i -= 1;
    }
    if i == 0 && a.first().copied().unwrap_or(0) == 0 {
        -1
    } else {
        i
    }
}

fn poly_mul_fp(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if poly_deg(a) < 0 || poly_deg(b) < 0 {
        return alloc::vec![0];
    }
    let mut c = alloc::vec![0u64; a.len() + b.len()];
    let pm = p as u128;
    for i in 0..a.len() {
        if a[i] == 0 {
            continue;
        }
        for j in 0..b.len() {
            if b[j] == 0 {
                continue;
            }
            let s = (c[i + j] as u128 + (a[i] as u128) * (b[j] as u128)) % pm;
            c[i + j] = s as u64;
        }
    }
    poly_trim(c)
}

fn poly_mod_fp(mut a: Vec<u64>, m: &[u64], p: u64) -> Vec<u64> {
    let dm = poly_deg(m);
    if dm <= 0 {
        return alloc::vec![0];
    }
    let lead = m[dm as usize];
    let inv_lead = match inv_mod_u64(lead, p) {
        Some(v) => v,
        None => return alloc::vec![0],
    };
    let pm = p as u128;
    loop {
        let da = poly_deg(&a);
        if da < dm {
            break;
        }
        let coef = ((a[da as usize] as u128 * inv_lead as u128) % pm) as u64;
        let shift = (da - dm) as usize;
        for i in 0..=dm as usize {
            let idx = i + shift;
            let sub = ((coef as u128 * m[i] as u128) % pm) as u64;
            a[idx] = ((a[idx] as u128 + pm - sub as u128) % pm) as u64;
        }
    }
    poly_trim(a)
}

fn poly_pow_fp(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Vec<u64> {
    let mut result = alloc::vec![1u64];
    let mut b = poly_mod_fp(base.to_vec(), m, p);
    while e > 0 {
        if e & 1 == 1 {
            result = poly_mod_fp(poly_mul_fp(&result, &b, p), m, p);
        }
        b = poly_mod_fp(poly_mul_fp(&b, &b, p), m, p);
        e >>= 1;
    }
    result
}

fn poly_gcd_fp(mut a: Vec<u64>, mut b: Vec<u64>, p: u64) -> Vec<u64> {
    while poly_deg(&b) >= 0 {
        let r = poly_mod_fp(a, &b, p);
        a = b;
        b = r;
    }
    let d = poly_deg(&a);
    if d < 0 {
        return alloc::vec![0];
    }
    let inv = inv_mod_u64(a[d as usize], p).unwrap_or(1);
    let pm = p as u128;
    for x in a.iter_mut() {
        *x = ((*x as u128 * inv as u128) % pm) as u64;
    }
    poly_trim(a)
}

fn eval_u64_coeffs(f: &[u64], x: u64, p: u64) -> u64 {
    let pm = p as u128;
    let mut acc: u128 = 0;
    let mut pow: u128 = 1;
    for &a in f {
        acc = (acc + (a as u128) * pow) % pm;
        pow = (pow * x as u128) % pm;
    }
    acc as u64
}

/// All roots of f in F_p (degree ≤ 16). Brute for small p; else gcd(f, x^p−x) + split.
fn roots_mod_p(poly: &PolyPair, p: u64) -> Vec<u64> {
    if p < 2 {
        return Vec::new();
    }
    let mut f: Vec<u64> = poly.f.iter().map(|c| big_mod_u64(c, p)).collect();
    f = poly_trim(f);
    if poly_deg(&f) < 1 {
        return Vec::new();
    }
    // Make monic modulus for reduction.
    let d = poly_deg(&f) as usize;
    if let Some(inv) = inv_mod_u64(f[d], p) {
        let pm = p as u128;
        for x in f.iter_mut() {
            *x = ((*x as u128 * inv as u128) % pm) as u64;
        }
    }

    if p <= 8192 {
        let mut roots = Vec::new();
        for r in 0..p {
            if eval_u64_coeffs(&f, r, p) == 0 {
                roots.push(r);
            }
        }
        return roots;
    }

    // g = gcd(f, x^p − x) = product of distinct linear factors over F_p.
    let xp = poly_pow_fp(&[0, 1], p, &f, p);
    let mut xp_minus_x = xp;
    if xp_minus_x.len() < 2 {
        xp_minus_x.resize(2, 0);
    }
    xp_minus_x[1] = (xp_minus_x[1] + p - 1) % p;
    let g = poly_gcd_fp(f.clone(), xp_minus_x, p);
    let gd = poly_deg(&g);
    if gd <= 0 {
        return Vec::new();
    }
    if gd == 1 {
        let c1 = g[1];
        let c0 = g[0];
        if let Some(inv) = inv_mod_u64(c1, p) {
            let r = ((p as u128 - (c0 as u128 * inv as u128 % p as u128)) % p as u128) as u64;
            return alloc::vec![r];
        }
        return Vec::new();
    }

    split_linear_factors(&g, p)
}

fn split_linear_factors(g: &[u64], p: u64) -> Vec<u64> {
    let mut stack = alloc::vec![g.to_vec()];
    let mut roots = Vec::new();
    let mut seed = p ^ 0x9e3779b97f4a7c15;
    while let Some(h) = stack.pop() {
        let d = poly_deg(&h);
        if d <= 0 {
            continue;
        }
        if d == 1 {
            let c1 = h[1];
            let c0 = h[0];
            if let Some(inv) = inv_mod_u64(c1, p) {
                let r = ((p as u128 - (c0 as u128 * inv as u128 % p as u128)) % p as u128) as u64;
                roots.push(r);
            }
            continue;
        }
        let mut split = false;
        for _ in 0..48 {
            seed = seed
                .wrapping_mul(0xbf58476d1ce4e5b9)
                .wrapping_add(0x94d049bb133111eb);
            let a = seed % p;
            let base = alloc::vec![a, 1];
            let mut t = poly_pow_fp(&base, (p - 1) / 2, &h, p);
            if t.is_empty() {
                t.push(0);
            }
            t[0] = (t[0] + p - 1) % p;
            let d1 = poly_gcd_fp(h.clone(), t, p);
            let dd = poly_deg(&d1);
            if dd > 0 && dd < d {
                if let Some(qpoly) = poly_div_exact(&h, &d1, p) {
                    stack.push(d1);
                    stack.push(qpoly);
                    split = true;
                    break;
                }
            }
        }
        if !split && (p as u128) * (d as u128) < 4_000_000 {
            for r in 0..p {
                if eval_u64_coeffs(&h, r, p) == 0 {
                    roots.push(r);
                }
            }
        }
    }
    roots.sort_unstable();
    roots.dedup();
    roots
}

fn poly_div_exact(a: &[u64], b: &[u64], p: u64) -> Option<Vec<u64>> {
    let db = poly_deg(b);
    let da = poly_deg(a);
    if db < 0 || da < db {
        return None;
    }
    let mut rem = a.to_vec();
    let mut q = alloc::vec![0u64; (da - db + 1) as usize];
    let inv_lead = inv_mod_u64(b[db as usize], p)?;
    let pm = p as u128;
    loop {
        let cur_d = poly_deg(&rem);
        if cur_d < db {
            break;
        }
        let coef = ((rem[cur_d as usize] as u128 * inv_lead as u128) % pm) as u64;
        let shift = (cur_d - db) as usize;
        q[shift] = coef;
        for i in 0..=db as usize {
            let idx = i + shift;
            let sub = ((coef as u128 * b[i] as u128) % pm) as u64;
            rem[idx] = ((rem[idx] as u128 + pm - sub as u128) % pm) as u64;
        }
    }
    if rem.iter().any(|&x| x != 0) {
        return None;
    }
    Some(poly_trim(q))
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

/// Natural log of a positive BigUint via leading mantissa + bit length.
pub fn log_biguint(n: &BigUint) -> f64 {
    if n.is_zero() {
        return f64::NEG_INFINITY;
    }
    let bits = n.bits();
    let shift = bits.saturating_sub(53);
    let top = if shift == 0 {
        n.to_u64_digits().first().copied().unwrap_or(1)
    } else {
        let limbs = crate::native_numeral::word_bits(n);
        let drop = (shift / 64) as usize;
        let sub = (shift % 64) as u32;
        let rest = if drop < limbs.len() { &limbs[drop..] } else { &[][..] };
        crate::native_numeral::shift_right_bits(rest, sub).first().copied().unwrap_or(1)
    };
    (top as f64).ln() + (shift as f64) * core::f64::consts::LN_2
}

/// Convert BigUint to u512 limbs (max 8 limbs = 512 bits, little-endian).
fn biguint_to_u512(n: &BigUint) -> [u64; 8] {
    let mut limbs = [0u64; 8];
    let digits = n.to_u64_digits();
    let nlimbs = digits.len().min(8);
    limbs[..nlimbs].copy_from_slice(&digits[..nlimbs]);
    limbs
}

/// Number of limbs used in u512 (ignoring trailing zeros).
fn u512_nlimbs(limbs: &[u64; 8]) -> u32 {
    for i in (0..8).rev() {
        if limbs[i] != 0 {
            return i as u32 + 1;
        }
    }
    0
}

fn big_mod_u64(n: &BigUint, p: u64) -> u64 {
    if p < 2 {
        return 0;
    }
    crate::native_numeral::modulo_small_via_word(n, p).unwrap_or(0)
}

fn inv_mod_u64(a: u64, p: u64) -> Option<u64> {
    if p < 2 || a % p == 0 {
        return None;
    }
    let mut t: i128 = 0;
    let mut newt: i128 = 1;
    let mut r: i128 = p as i128;
    let mut newr: i128 = (a % p) as i128;
    while newr != 0 {
        let q = r / newr;
        let (nt, nr) = (t - q * newt, r - q * newr);
        t = newt;
        newt = nt;
        r = newr;
        newr = nr;
    }
    if r > 1 {
        return None;
    }
    if t < 0 {
        t += p as i128;
    }
    Some(t as u64)
}

fn eval_f_mod_u64(f: &[BigUint], x: u64, m: u64) -> u64 {
    if m < 2 {
        return 0;
    }
    let mm = m as u128;
    let mut acc: u128 = 0;
    let mut pow: u128 = 1;
    for a in f {
        let ai = big_mod_u64(a, m) as u128;
        acc = (acc + ai * pow) % mm;
        pow = (pow * x as u128) % mm;
    }
    acc as u64
}

fn eval_fprime_mod_u64(f: &[BigUint], x: u64, m: u64) -> u64 {
    if m < 2 || f.len() < 2 {
        return 0;
    }
    let mm = m as u128;
    let mut acc: u128 = 0;
    for i in 1..f.len() {
        let ai = big_mod_u64(&f[i], m) as u128;
        let coef = ((i as u128) % mm) * ai % mm;
        let mut xp: u128 = 1;
        for _ in 0..(i - 1) {
            xp = (xp * x as u128) % mm;
        }
        acc = (acc + coef * xp) % mm;
    }
    acc as u64
}

/// Lift a simple root r₀ of f ≡ 0 (mod p) to a root mod pᵉ (Hensel).
fn hensel_lift_root(poly: &PolyPair, p: u64, r0: u64, pe: u64) -> Option<u64> {
    if p < 2 || pe < p || pe % p != 0 {
        return None;
    }
    let mut r = r0 % p;
    if eval_f_mod_u64(&poly.f, r, p) != 0 {
        return None;
    }
    if pe == p {
        return Some(r);
    }
    let fp = eval_fprime_mod_u64(&poly.f, r, p) % p;
    let inv = inv_mod_u64(fp, p)?;
    let mut pk = p;
    while pk < pe {
        let next = pk.checked_mul(p)?;
        let fr = eval_f_mod_u64(&poly.f, r, next);
        let q = fr / pk;
        let t = ((p as u128 - ((q as u128 * inv as u128) % p as u128)) % p as u128) as u64;
        r = r + t * pk;
        pk = next;
    }
    Some(r % pe)
}

fn smooth_rational(n: &BigUint, primes: &[u64], out: &mut [u8]) -> bool {
    debug_assert_eq!(out.len(), primes.len());
    for e in out.iter_mut() {
        *e = 0;
    }
    if n.is_zero() {
        return false;
    }
    let mut rest = n.clone();
    for (i, &p) in primes.iter().enumerate() {
        loop {
            let (q, r) = crate::native_numeral::divmod_small_via_word(&rest, p).unwrap();
            if r != 0 { break; }
            rest = q;
            out[i] ^= 1;
        }
    }
    rest.is_one()
}

fn smooth_algebraic(
    n: &BigUint,
    a: i64,
    b: i64,
    ideals: &[AlgIdeal],
    out: &mut [u8],
) -> bool {
    debug_assert_eq!(out.len(), ideals.len());
    for e in out.iter_mut() {
        *e = 0;
    }
    if n.is_zero() {
        return false;
    }
    let mut rest = n.clone();
    let mut i = 0;
    while i < ideals.len() {
        let p = ideals[i].p;
        let mut exp = 0u32;
        loop {
            let (q, r) = crate::native_numeral::divmod_small_via_word(&rest, p).unwrap();
            if r != 0 { break; }
            rest = q;
            exp += 1;
        }
        if exp > 0 {
            let mut assigned = false;
            let mut j = i;
            while j < ideals.len() && ideals[j].p == p {
                let r = ideals[j].r as i64;
                let lhs = a.rem_euclid(p as i64);
                let rhs = (b.rem_euclid(p as i64) * r).rem_euclid(p as i64);
                if lhs == rhs {
                    out[j] ^= (exp & 1) as u8;
                    assigned = true;
                    break;
                }
                j += 1;
            }
            if !assigned {
                return false;
            }
        }
        while i < ideals.len() && ideals[i].p == p {
            i += 1;
        }
    }
    rest.is_one()
}

pub fn rational_value(a: i64, b: i64, m: &BigUint) -> BigInt {
    use crate::native_numeral::{signed_multiply_via_word, signed_subtract_via_word_general};
    let aa = BigInt::from(a);
    let bb = BigInt::from(b);
    let mm = BigInt::from_biguint(Sign::Plus, m.clone());
    signed_subtract_via_word_general(&aa, &signed_multiply_via_word(&bb, &mm))
}

fn try_relation(poly: &PolyPair, fb: &FactorBases, a: i64, b: i64) -> Option<Relation> {
    if b <= 0 || gcd_i64(a, b) != 1 {
        return None;
    }
    let rat_signed = rational_value(a, b, &poly.m);
    if rat_signed.is_zero() {
        return None;
    }
    let rat = rat_signed.abs().to_biguint().unwrap_or_else(BigUint::zero);
    let alg = poly.homogeneous_norm(a, b);
    if alg.is_zero() {
        return None;
    }

    let ncols = fb.ncols();
    let mut exps = alloc::vec![0u8; ncols];
    exps[0] = if rat_signed.sign() == Sign::Minus { 1 } else { 0 };

    let rat_slice = &mut exps[1..1 + fb.rational.len()];
    if !smooth_rational(&rat, &fb.rational, rat_slice) {
        return None;
    }
    let alg_slice = &mut exps[1 + fb.rational.len()..];
    if !smooth_algebraic(&alg, a, b, &fb.algebraic, alg_slice) {
        return None;
    }
    Some(Relation {
        a,
        b,
        exps,
        rat_signed,
        rat_abs: rat,
        alg_abs: alg,
    })
}

/// Sieving parameters — window scales with B and bit-length; no toy ceilings
/// that silently truncate a mandated full search.
#[derive(Clone, Debug)]
pub struct SieveParams {
    pub a_bound: i64,
    pub b_bound: i64,
    pub target_relations: usize,
    pub max_pairs: u64,
    /// log|norm| − sieve_score fudge (nats). Classical λ·log(B) scale.
    pub log_fudge: f64,
}

impl SieveParams {
    pub fn from_b(b: u64, bits: u64, ncols: usize) -> Self {
        let scale = if bits < 16 {
            8
        } else if bits < 32 {
            6
        } else if bits < 64 {
            4
        } else if bits < 128 {
            3
        } else if bits < 256 {
            2
        } else {
            2
        };
        let a_bound = (b as i64).saturating_mul(scale).max(100);
        let b_bound = (b as i64).saturating_mul(2).max(40);
        let target = ncols + 32 + (bits as usize / 8);
        // Full window; soft cap only to bound VRAM tiling time, not correctness.
        let max_pairs = (a_bound as u64)
            .saturating_mul(b_bound as u64)
            .saturating_mul(2)
            .max(1_000_000);
        let log_b = (b.max(3) as f64).ln();
        let log_fudge = 1.8 * log_b;
        Self {
            a_bound,
            b_bound,
            target_relations: target,
            max_pairs,
            log_fudge,
        }
    }
}

pub struct SieveReport {
    pub relations: Vec<Relation>,
    pub pairs_tested: u64,
    pub params: SieveParams,
    pub used_gpu: bool,
    pub gpu_ms: u64,
    pub cells: u64,
    pub hit_launches: u64,
    pub device: u32,
}

/// Powers p, p², … while p^k fits in the a-line and in u64.
fn prime_power_steps(p: u64, a_bound: i64) -> Vec<(u64, f32)> {
    let mut out = Vec::new();
    if p < 2 {
        return out;
    }
    let lim = (2 * a_bound as u64).saturating_add(2).max(p);
    let logp = (p as f32).ln();
    let mut pe = p;
    loop {
        out.push((pe, logp));
        let next = pe.checked_mul(p);
        match next {
            Some(n) if n <= lim && n > pe => pe = n,
            _ => break,
        }
    }
    out
}

/// Algebraic line hits: (pᵉ, root mod pᵉ, log p) via Hensel when f'(r) invertible.
fn algebraic_power_hits(poly: &PolyPair, id: &AlgIdeal, a_bound: i64) -> Vec<(u64, u64, f32)> {
    let mut out = Vec::new();
    if id.p < 2 {
        return out;
    }
    let lim = (2 * a_bound as u64).saturating_add(2).max(id.p);
    let logp = (id.p as f32).ln();
    let mut pe = id.p;
    loop {
        match hensel_lift_root(poly, id.p, id.r, pe) {
            Some(r) => out.push((pe, r, logp)),
            None => {
                if pe == id.p {
                    // Multiple root / failed lift — still hit the prime ideal once.
                    out.push((id.p, id.r % id.p, logp));
                }
                break;
            }
        }
        match pe.checked_mul(id.p) {
            Some(n) if n <= lim && n > pe => pe = n,
            _ => break,
        }
    }
    out
}

pub fn collect_relations(poly: &PolyPair, fb: &FactorBases, params: &SieveParams) -> SieveReport {
    let mut relations = Vec::new();
    let mut pairs = 0u64;
    let mut gpu_ms = 0u64;
    let mut cells = 0u64;
    let mut hit_launches = 0u64;
    let device = selected_device();

    let m_bits = poly.m.bits();
    let max_f_bits = poly.f.iter().map(|c| c.bits()).max().unwrap_or(0);
    let exact_mark_ok = m_bits <= 64 && max_f_bits <= 64;

    let gpu = match gpu_setup(device) {
        Ok(g) => Some(g),
        Err(e) => {
            eprintln!("gpu_gnfs ≻ GPU setup failed — host log-line sieve: {e}");
            None
        }
    };
    let mut used_gpu = false;
    if let Some(ref g) = gpu {
        eprintln!(
            "gpu_gnfs ≻ GPU classical log-line |A|={} Bmax={} fudge={:.3} mark={} on device {} ({})",
            params.a_bound,
            params.b_bound,
            params.log_fudge,
            if exact_mark_ok { "exact+log" } else { "log|norm|" },
            device,
            device_label(device)
        );
        match gpu_sieve_collect(
            g,
            poly,
            fb,
            params,
            exact_mark_ok,
            &mut relations,
            &mut pairs,
            &mut gpu_ms,
            &mut cells,
            &mut hit_launches,
        ) {
            Ok(()) => used_gpu = true,
            Err(e) => {
                eprintln!("gpu_gnfs ≻ GPU sieve failed — continuing on host log-line: {e}");
                host_log_sieve(poly, fb, params, &mut relations, &mut pairs);
            }
        }
    } else {
        host_log_sieve(poly, fb, params, &mut relations, &mut pairs);
    }

    SieveReport {
        relations,
        pairs_tested: pairs,
        params: params.clone(),
        used_gpu,
        gpu_ms,
        cells,
        hit_launches,
        device,
    }
}

/// Keep the selected GPU under load for `secs` wall seconds (batched log-line tiles).
/// Returns a stdout-visible status string so engagement is not buried on stderr.
pub fn soak(secs: u64, b: u64) -> String {
    let secs = secs.max(1).min(600);
    let b = if b == 0 { 2000 } else { b };
    let device = selected_device();
    let n = BigUint::from(10403u32);
    let poly = match crate::gpu_gnfs_poly::select_polynomial(&n, Some(3)) {
        Ok(p) => p,
        Err(e) => return format!("gpu_gnfs soak: poly failed: {e}"),
    };
    let fb = FactorBases::build(&poly, b);
    let mut params = SieveParams::from_b(b, 14, fb.ncols());
    // Force a multi-tile window so soak has something to chew.
    params.a_bound = params.a_bound.max(20_000);
    params.b_bound = params.b_bound.max(8_000);
    params.target_relations = usize::MAX / 4;
    params.max_pairs = u64::MAX / 4;

    let gpu = match gpu_setup(device) {
        Ok(g) => g,
        Err(e) => return format!("gpu_gnfs soak: GPU setup failed on device {device}: {e}"),
    };
    let exact = poly.m.bits() <= 64;
    let t0 = std::time::Instant::now();
    let mut relations = Vec::new();
    let mut pairs = 0u64;
    let mut gpu_ms = 0u64;
    let mut cells = 0u64;
    let mut hits = 0u64;
    let mut rounds = 0u64;
    while t0.elapsed().as_secs() < secs {
        relations.clear();
        let mut p = 0u64;
        let mut gm = 0u64;
        let mut c = 0u64;
        let mut h = 0u64;
        if let Err(e) = gpu_sieve_collect(
            &gpu,
            &poly,
            &fb,
            &params,
            exact,
            &mut relations,
            &mut p,
            &mut gm,
            &mut c,
            &mut h,
        ) {
            return format!("gpu_gnfs soak: sieve failed: {e}");
        }
        pairs += p;
        gpu_ms += gm;
        cells += c;
        hits += h;
        rounds += 1;
        eprintln!(
            "gpu_gnfs soak: round {rounds} cells_cum={cells} gpu_ms_cum={gpu_ms} elapsed={}s",
            t0.elapsed().as_secs()
        );
    }
    format!(
        "gpu_gnfs soak: OK device={} ({}) rounds={} pairs={} cells={} hit-launches={} gpu_ms={} wall_s={} gpu=true — watch nvidia-smi -i {}",
        device,
        device_label(device),
        rounds,
        pairs,
        cells,
        hits,
        gpu_ms,
        t0.elapsed().as_secs(),
        device
    )
}

fn selected_device() -> u32 {
    std::env::var("GNFS_GPU")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn device_label(idx: u32) -> &'static str {
    match idx {
        0 => "RTX 4070",
        1 => "RTX 3060",
        _ => "CUDA device",
    }
}

/// Host classical log-line sieve — same mathematics as the device path.
fn host_log_sieve(
    poly: &PolyPair,
    fb: &FactorBases,
    params: &SieveParams,
    relations: &mut Vec<Relation>,
    pairs: &mut u64,
) {
    let a_bound = params.a_bound;
    let a_count = (2 * a_bound + 1) as usize;
    let log_m = log_biguint(&poly.m);
    let log_f: Vec<f64> = poly
        .f
        .iter()
        .map(|c| {
            if c.is_zero() {
                f64::NEG_INFINITY
            } else {
                log_biguint(c)
            }
        })
        .collect();
    let deg = poly.degree as usize;
    let fudge = params.log_fudge;

    let mut rat_s = alloc::vec![0f32; a_count];
    let mut alg_s = alloc::vec![0f32; a_count];

    'b_loop: for b in 1..=params.b_bound {
        if relations.len() >= params.target_relations || *pairs >= params.max_pairs {
            break;
        }
        for x in rat_s.iter_mut() {
            *x = 0.0;
        }
        for x in alg_s.iter_mut() {
            *x = 0.0;
        }

        // Rational hits: a ≡ b·m (mod pᵉ)
        for &p in &fb.rational {
            for (pe, logp) in prime_power_steps(p, a_bound) {
                let mul = big_mod_u64(&poly.m, pe);
                let r = ((b as i128) * (mul as i128)).rem_euclid(pe as i128) as i64;
                let pp = pe as i64;
                let rem = (-a_bound).rem_euclid(pp);
                let delta = (r - rem).rem_euclid(pp);
                let mut a = -a_bound + delta;
                while a <= a_bound {
                    let idx = (a + a_bound) as usize;
                    rat_s[idx] += logp;
                    a += pp;
                }
            }
        }
        // Algebraic hits: a ≡ b·rₑ (mod pᵉ), Hensel-lifted roots
        for id in &fb.algebraic {
            for (pe, r_pe, logp) in algebraic_power_hits(poly, id, a_bound) {
                let r = ((b as i128) * (r_pe as i128)).rem_euclid(pe as i128) as i64;
                let pp = pe as i64;
                let rem = (-a_bound).rem_euclid(pp);
                let delta = (r - rem).rem_euclid(pp);
                let mut a = -a_bound + delta;
                while a <= a_bound {
                    let idx = (a + a_bound) as usize;
                    alg_s[idx] += logp;
                    a += pp;
                }
            }
        }

        for a_idx in 0..a_count {
            *pairs += 1;
            let a = -a_bound + a_idx as i64;
            if gcd_i64(a, b) != 1 {
                continue;
            }
            let lr = log_rat_norm(a, b, log_m);
            let la = log_alg_norm(a, b, deg, &log_f);
            if (rat_s[a_idx] as f64) + fudge < lr || (alg_s[a_idx] as f64) + fudge < la {
                continue;
            }
            if let Some(rel) = try_relation(poly, fb, a, b) {
                relations.push(rel);
                if relations.len() >= params.target_relations {
                    break 'b_loop;
                }
            }
            if *pairs >= params.max_pairs {
                break 'b_loop;
            }
        }

        if b % 64 == 0 {
            eprintln!(
                "gpu_gnfs ≻ host log-line: b={b}/{} relations={}/{} pairs={}",
                params.b_bound,
                relations.len(),
                params.target_relations,
                *pairs
            );
        }
    }
}

fn log_rat_norm(a: i64, b: i64, log_m: f64) -> f64 {
    // |a − b m| = b·m · |a/(b m) − 1|. For m ≫ |a|/b: ≈ log(b)+log(m).
    if b <= 0 {
        return f64::INFINITY;
    }
    let lb = (b as f64).ln();
    if log_m > 40.0 {
        // |a|/(b m) ≪ 1
        return log_m + lb;
    }
    // m fits comfortably in f64 mantissa range for the product scale
    let m = log_m.exp();
    let v = (a as f64 - (b as f64) * m).abs();
    if v < 1.0 {
        return 0.0;
    }
    v.ln()
}

fn log_alg_norm(a: i64, b: i64, deg: usize, log_f: &[f64]) -> f64 {
    let la = if a == 0 {
        f64::NEG_INFINITY
    } else {
        (a.abs() as f64).ln()
    };
    let lb = (b as f64).ln();
    let mut best = f64::NEG_INFINITY;
    for (i, &lf) in log_f.iter().enumerate() {
        if !lf.is_finite() {
            continue;
        }
        let t = lf
            + if i == 0 {
                0.0
            } else if la.is_finite() {
                (i as f64) * la
            } else {
                f64::NEG_INFINITY
            }
            + ((deg - i) as f64) * lb;
        if t > best {
            best = t;
        }
    }
    best
}

// ─── GPU classical log-line sieve ───

const SMOOTH_KERNEL: &str = r#"
typedef unsigned long long u64;
typedef long long i64;

__device__ u64 abs_i128_to_u64(__int128 v) {
    if (v < 0) v = -v;
    if (v == 0 || v > (__int128)0xFFFFFFFFFFFFFFFFull) return 0;
    return (u64)v;
}

__device__ int fb_smooth(u64 rest, const u64* fb, int nfb) {
    if (rest <= 1) return rest == 1;
    for (int k = 0; k < nfb; k++) {
        u64 p = fb[k];
        if (p < 2) continue;
        while (rest % p == 0) rest /= p;
    }
    return rest == 1;
}

__device__ u64 alg_norm_u64(i64 a, i64 b, const u64* f, int deg) {
    __int128 pos = 0, neg = 0;
    u64 aa = (u64)(a < 0 ? -a : a);
    u64 bb = (u64)(b < 0 ? -b : b);
    for (int i = 0; i <= deg; i++) {
        if (f[i] == 0) continue;
        __int128 term = (__int128)f[i];
        for (int k = 0; k < i; k++) term *= (__int128)aa;
        for (int k = 0; k < deg - i; k++) term *= (__int128)bb;
        int sign_neg = ((a < 0) && (i & 1)) ^ ((b < 0) && ((deg - i) & 1));
        if (sign_neg) neg += term; else pos += term;
    }
    __int128 diff = pos >= neg ? pos - neg : neg - pos;
    return abs_i128_to_u64(diff);
}

// Multi-limb (base 2^64) helpers for wide integers on device.
typedef struct { u64 limbs[8]; int n; } u512;

__device__ void u512_zero(u512* x) {
    x->n = 0;
    for (int i = 0; i < 8; i++) x->limbs[i] = 0;
}

__device__ void u512_set_u64(u512* x, u64 v) {
    x->limbs[0] = v;
    x->n = (v == 0) ? 0 : 1;
}

__device__ void u512_copy(u512* dst, const u512* src) {
    dst->n = src->n;
    for (int i = 0; i < src->n; i++) dst->limbs[i] = src->limbs[i];
}

__device__ int u512_cmp(const u512* a, const u512* b) {
    int na = a->n, nb = b->n;
    if (na != nb) return na < nb ? -1 : 1;
    for (int i = na - 1; i >= 0; i--) {
        if (a->limbs[i] != b->limbs[i]) return a->limbs[i] < b->limbs[i] ? -1 : 1;
    }
    return 0;
}

__device__ void u512_sub(u512* res, const u512* a, const u512* b) {
    // assumes a >= b
    __int128 borrow = 0;
    int n = a->n > b->n ? a->n : b->n;
    for (int i = 0; i < n; i++) {
        u64 ai = (i < a->n) ? a->limbs[i] : 0;
        u64 bi = (i < b->n) ? b->limbs[i] : 0;
        __int128 diff = (__int128)ai - (__int128)bi - borrow;
        res->limbs[i] = (u64)diff;
        borrow = (diff < 0) ? 1 : 0;
    }
    res->n = n;
    while (res->n > 0 && res->limbs[res->n - 1] == 0) res->n--;
}

__device__ void u512_mul_u64(u512* res, const u512* a, u64 b) {
    if (b == 0 || a->n == 0) { u512_zero(res); return; }
    __int128 carry = 0;
    for (int i = 0; i < a->n; i++) {
        __int128 prod = (__int128)a->limbs[i] * (__int128)b + carry;
        res->limbs[i] = (u64)prod;
        carry = prod >> 64;
    }
    int n = a->n;
    while (carry && n < 8) {
        res->limbs[n] = (u64)carry;
        carry >>= 64;
        n++;
    }
    res->n = n;
}

__device__ void u512_add(u512* res, const u512* a, const u512* b) {
    __int128 carry = 0;
    int n = a->n > b->n ? a->n : b->n;
    for (int i = 0; i < n; i++) {
        u64 ai = (i < a->n) ? a->limbs[i] : 0;
        u64 bi = (i < b->n) ? b->limbs[i] : 0;
        __int128 sum = (__int128)ai + (__int128)bi + carry;
        res->limbs[i] = (u64)sum;
        carry = sum >> 64;
    }
    int i = n;
    while (carry && i < 8) {
        res->limbs[i] = (u64)carry;
        carry >>= 64;
        i++;
    }
    res->n = i;
    while (res->n > 0 && res->limbs[res->n - 1] == 0) res->n--;
}

__device__ void u512_from_i64(u512* x, i64 v) {
    if (v < 0) v = -v;
    u512_set_u64(x, (u64)v);
}

__device__ int u512_is_zero(const u512* x) {
    return x->n == 0;
}

__device__ int u512_fits_u64(const u512* x) {
    return x->n <= 1;
}

__device__ u64 u512_to_u64(const u512* x) {
    return (x->n == 0) ? 0 : x->limbs[0];
}

// log(|x|) for u512: leading limb + bit length
__device__ double u512_log(const u512* x) {
    if (x->n == 0) return -1.0e300;
    int msb_limb = x->n - 1;
    u64 top = x->limbs[msb_limb];
    int msb_bit = 63 - __clzll(top);
    int bits = msb_limb * 64 + msb_bit + 1;
    double mantissa = (double)top / (1ull << msb_bit);
    return log(mantissa) + (double)bits * 0.6931471805599453;
}

// Check if u512 is B-smooth by trial dividing against factor base
__device__ int u512_smooth(const u512* x, const u64* fb, int nfb) {
    if (u512_is_zero(x)) return 0;
    if (x->n == 1 && x->limbs[0] == 1) return 1;
    // For simplicity on device, we only check exact smoothness when it fits in u64.
    // For wide norms, we rely on the log-threshold cut.
    return u512_fits_u64(x) && fb_smooth(u512_to_u64(x), fb, nfb);
}

// Rational norm: |a - b * m| where m is wide
__device__ void rational_norm_wide(u512* res, i64 a, i64 b, const u512* m) {
    // Compute b * m
    u512 bm;
    u512_mul_u64(&bm, m, (u64)(b < 0 ? -b : b));
    // a as u512
    u512 a_u512;
    u512_from_i64(&a_u512, a);
    // |a - b*m|
    int cmp = u512_cmp(&a_u512, &bm);
    if (cmp >= 0) u512_sub(res, &a_u512, &bm);
    else u512_sub(res, &bm, &a_u512);
}

// Algebraic norm: |f(a/b) * b^deg| = |sum f_i * a^i * b^(deg-i)|
__device__ void algebraic_norm_wide(u512* res, i64 a, i64 b, const u512* fcoef, int deg) {
    u512 pos, neg, term, aa, bb;
    u512_zero(&pos);
    u512_zero(&neg);
    u512_from_i64(&aa, a < 0 ? -a : a);
    u512_from_i64(&bb, b < 0 ? -b : b);

    for (int i = 0; i <= deg; i++) {
        if (u512_is_zero(&fcoef[i])) continue;
        // term = fcoef[i] * aa^i * bb^(deg-i)
        u512_copy(&term, &fcoef[i]);
        for (int k = 0; k < i; k++) u512_mul_u64(&term, &term, u512_to_u64(&aa));
        for (int k = 0; k < deg - i; k++) u512_mul_u64(&term, &term, u512_to_u64(&bb));
        int sign_neg = ((a < 0) && (i & 1)) ^ ((b < 0) && ((deg - i) & 1));
        if (sign_neg) u512_add(&neg, &neg, &term);
        else u512_add(&pos, &pos, &term);
    }
    int cmp = u512_cmp(&pos, &neg);
    if (cmp >= 0) u512_sub(res, &pos, &neg);
    else u512_sub(res, &neg, &pos);
}

extern "C" __global__ void gnfs_hit_line(
    i64 A, i64 b_count, i64 b_lo, u64 residue_mul, u64 add, u64 p, float logp,
    float* sieve, int a_count)
{
    int step = blockIdx.x * blockDim.x + threadIdx.x;
    int b_idx = blockIdx.y * blockDim.y + threadIdx.y;
    if (b_idx >= b_count || p < 2) return;
    i64 b = b_lo + (i64)b_idx;
    __int128 t = (__int128)b * (__int128)residue_mul + (__int128)add;
    i64 r = (i64)((t % (__int128)p + (__int128)p) % (__int128)p);
    i64 pp = (i64)p;
    i64 negA = -A;
    i64 rem = ((negA % pp) + pp) % pp;
    i64 delta = (r - rem + pp) % pp;
    i64 a0 = negA + delta;
    i64 a = a0 + (i64)step * pp;
    if (a > A) return;
    int a_idx = (int)(a + A);
    if (a_idx >= 0 && a_idx < a_count)
        atomicAdd(&sieve[b_idx * a_count + a_idx], logp);
}

// One launch for the whole factor-base side: z = prime index.
extern "C" __global__ void gnfs_hit_batch(
    i64 A, i64 b_count, i64 b_lo,
    const u64* residue_mul, const u64* p, const float* logp, int nprime,
    float* sieve, int a_count)
{
    int pi = blockIdx.z;
    int step = blockIdx.x * blockDim.x + threadIdx.x;
    int b_idx = blockIdx.y * blockDim.y + threadIdx.y;
    if (pi >= nprime || b_idx >= b_count) return;
    u64 pp_u = p[pi];
    if (pp_u < 2) return;
    i64 b = b_lo + (i64)b_idx;
    u64 mul = residue_mul[pi];
    float lp = logp[pi];
    __int128 t = (__int128)b * (__int128)mul;
    i64 r = (i64)((t % (__int128)pp_u + (__int128)pp_u) % (__int128)pp_u);
    i64 pp = (i64)pp_u;
    i64 negA = -A;
    i64 rem = ((negA % pp) + pp) % pp;
    i64 delta = (r - rem + pp) % pp;
    i64 a0 = negA + delta;
    i64 a = a0 + (i64)step * pp;
    if (a > A) return;
    int a_idx = (int)(a + A);
    if (a_idx >= 0 && a_idx < a_count)
        atomicAdd(&sieve[b_idx * a_count + a_idx], lp);
}

extern "C" __global__ void gnfs_mark_exact(
    i64 A, i64 b_count, i64 b_lo, u64 m,
    const float* rat_s, const float* alg_s,
    const u64* fb, int nfb,
    const u64* fcoef, int deg,
    unsigned char* out, int a_count)
{
    int a_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int b_idx = blockIdx.y * blockDim.y + threadIdx.y;
    if (a_idx >= a_count || b_idx >= b_count) return;
    int slot = b_idx * a_count + a_idx;
    if (rat_s[slot] < 1.0f || alg_s[slot] < 1.0f) { out[slot] = 0; return; }
    i64 a = -A + (i64)a_idx;
    i64 b = b_lo + (i64)b_idx;
    i64 x = a < 0 ? -a : a;
    i64 y = b;
    while (y) { i64 t = x % y; x = y; y = t; }
    if (x != 1) { out[slot] = 0; return; }
    __int128 rdiff = (__int128)a - (__int128)b * (__int128)m;
    u64 rat = abs_i128_to_u64(rdiff);
    if (rat == 0 || !fb_smooth(rat, fb, nfb)) { out[slot] = 0; return; }
    u64 alg = alg_norm_u64(a, b, fcoef, deg);
    if (alg == 0 || !fb_smooth(alg, fb, nfb)) { out[slot] = 0; return; }
    out[slot] = 1;
}

// Classical threshold: sieve_score + fudge >= log|norm|.
extern "C" __global__ void gnfs_mark_lognorm(
    i64 A, i64 b_count, i64 b_lo,
    const float* rat_s, const float* alg_s,
    double log_m, const double* log_f, int deg,
    double fudge,
    unsigned char* out, int a_count)
{
    int a_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int b_idx = blockIdx.y * blockDim.y + threadIdx.y;
    if (a_idx >= a_count || b_idx >= b_count) return;
    int slot = b_idx * a_count + a_idx;
    i64 a = -A + (i64)a_idx;
    i64 b = b_lo + (i64)b_idx;
    i64 x = a < 0 ? -a : a;
    i64 y = b;
    while (y) { i64 t = x % y; x = y; y = t; }
    if (x != 1) { out[slot] = 0; return; }

    double lr;
    if (log_m > 40.0) {
        lr = log_m + log((double)b);
    } else {
        double m = exp(log_m);
        double v = fabs((double)a - (double)b * m);
        lr = (v < 1.0) ? 0.0 : log(v);
    }

    double la_a = (a == 0) ? -1.0e300 : log(fabs((double)a));
    double la_b = log((double)b);
    double la = -1.0e300;
    for (int i = 0; i <= deg; i++) {
        double lf = log_f[i];
        if (lf < -1.0e200) continue;
        double t = lf;
        if (i > 0) t += (double)i * la_a;
        t += (double)(deg - i) * la_b;
        if (t > la) la = t;
    }

    float rs = rat_s[slot];
    float as = alg_s[slot];
    out[slot] = ((double)rs + fudge >= lr && (double)as + fudge >= la) ? 1 : 0;
}

// Faithful survivor cut for wide m and f: compute true log|norm| via multi-limb.
// m_limbs: m_n limbs of m (little-endian, base 2^64)
// fcoef_limbs: (deg+1)*8 limbs, each coefficient has 8 limbs (little-endian)
extern "C" __global__ void gnfs_mark_wide(
    i64 A, i64 b_count, i64 b_lo,
    const float* rat_s, const float* alg_s,
    const u64* m_limbs, int m_n,
    const u64* fcoef_limbs, int deg,
    double fudge,
    unsigned char* out, int a_count)
{
    int a_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int b_idx = blockIdx.y * blockDim.y + threadIdx.y;
    if (a_idx >= a_count || b_idx >= b_count) return;
    int slot = b_idx * a_count + a_idx;
    i64 a = -A + (i64)a_idx;
    i64 b = b_lo + (i64)b_idx;
    i64 x = a < 0 ? -a : a;
    i64 y = b;
    while (y) { i64 t = x % y; x = y; y = t; }
    if (x != 1) { out[slot] = 0; return; }

    // Rational norm: |a - b*m|
    // Compute b*m into local array
    u64 bm[8];
    for (int i = 0; i < 8; i++) bm[i] = 0;
    u64 b_abs = (u64)(b < 0 ? -b : b);
    __int128 carry = 0;
    for (int i = 0; i < m_n; i++) {
        __int128 prod = (__int128)m_limbs[i] * (__int128)b_abs + carry;
        bm[i] = (u64)prod;
        carry = prod >> 64;
    }
    int bm_n = m_n;
    while (carry && bm_n < 8) {
        bm[bm_n] = (u64)carry;
        carry >>= 64;
        bm_n++;
    }

    // a as u64
    u64 a_abs = (u64)(a < 0 ? -a : a);
    // Compare a and bm
    int cmp = 0;
    for (int i = 7; i >= 0; i--) {
        u64 ai = (i == 0) ? a_abs : 0;
        u64 bi = (i < bm_n) ? bm[i] : 0;
        if (ai != bi) { cmp = (ai < bi) ? -1 : 1; break; }
    }

    // rat = |a - bm|
    u64 rat[8];
    if (cmp >= 0) {
        // a >= bm: rat = a - bm
        __int128 borrow = 0;
        for (int i = 0; i < 8; i++) {
            u64 ai = (i == 0) ? a_abs : 0;
            u64 bi = (i < bm_n) ? bm[i] : 0;
            __int128 diff = (__int128)ai - (__int128)bi - borrow;
            rat[i] = (u64)diff;
            borrow = (diff < 0) ? 1 : 0;
        }
    } else {
        // bm > a: rat = bm - a
        __int128 borrow = 0;
        for (int i = 0; i < 8; i++) {
            u64 ai = (i == 0) ? a_abs : 0;
            u64 bi = (i < bm_n) ? bm[i] : 0;
            __int128 diff = (__int128)bi - (__int128)ai - borrow;
            rat[i] = (u64)diff;
            borrow = (diff < 0) ? 1 : 0;
        }
    }

    // log(|rat|)
    double lr = -1.0e300;
    for (int i = 7; i >= 0; i--) {
        if (rat[i] != 0) {
            int msb_bit = 63 - __clzll(rat[i]);
            int bits = i * 64 + msb_bit + 1;
            double mantissa = (double)rat[i] / (1ull << msb_bit);
            lr = log(mantissa) + (double)bits * 0.6931471805599453;
            break;
        }
    }

    // Algebraic norm: |sum f_i * a^i * b^(deg-i)|
    // pos and neg accumulators
    u64 pos[8], neg[8];
    for (int i = 0; i < 8; i++) { pos[i] = 0; neg[i] = 0; }
    u64 aa = (u64)(a < 0 ? -a : a);
    u64 bb = (u64)(b < 0 ? -b : b);

    for (int i = 0; i <= deg; i++) {
        // fcoef_limbs[i*8 + j] for j=0..7
        int coef_offset = i * 8;
        // Skip if coefficient is zero
        int coef_n = 0;
        for (int j = 7; j >= 0; j--) {
            if (fcoef_limbs[coef_offset + j] != 0) { coef_n = j + 1; break; }
        }
        if (coef_n == 0) continue;

        // term = f_i * aa^i * bb^(deg-i)
        u64 term[8];
        for (int j = 0; j < 8; j++) term[j] = fcoef_limbs[coef_offset + j];
        int term_n = coef_n;

        // Multiply by aa^i
        for (int k = 0; k < i; k++) {
            carry = 0;
            for (int j = 0; j < term_n; j++) {
                __int128 prod = (__int128)term[j] * (__int128)aa + carry;
                term[j] = (u64)prod;
                carry = prod >> 64;
            }
            while (carry && term_n < 8) {
                term[term_n] = (u64)carry;
                carry >>= 64;
                term_n++;
            }
        }
        // Multiply by bb^(deg-i)
        for (int k = 0; k < deg - i; k++) {
            carry = 0;
            for (int j = 0; j < term_n; j++) {
                __int128 prod = (__int128)term[j] * (__int128)bb + carry;
                term[j] = (u64)prod;
                carry = prod >> 64;
            }
            while (carry && term_n < 8) {
                term[term_n] = (u64)carry;
                carry >>= 64;
                term_n++;
            }
        }

        int sign_neg = ((a < 0) && (i & 1)) ^ ((b < 0) && ((deg - i) & 1));
        if (sign_neg) {
            // neg += term
            carry = 0;
            int n = term_n > 8 ? 8 : term_n;
            for (int j = 0; j < n; j++) {
                __int128 sum = (__int128)neg[j] + (__int128)term[j] + carry;
                neg[j] = (u64)sum;
                carry = sum >> 64;
            }
        } else {
            // pos += term
            carry = 0;
            int n = term_n > 8 ? 8 : term_n;
            for (int j = 0; j < n; j++) {
                __int128 sum = (__int128)pos[j] + (__int128)term[j] + carry;
                pos[j] = (u64)sum;
                carry = sum >> 64;
            }
        }
    }

    // alg = |pos - neg|
    u64 alg[8];
    // Compare pos and neg
    cmp = 0;
    for (int i = 7; i >= 0; i--) {
        if (pos[i] != neg[i]) { cmp = (pos[i] < neg[i]) ? -1 : 1; break; }
    }
    if (cmp >= 0) {
        __int128 borrow = 0;
        for (int i = 0; i < 8; i++) {
            u64 pi = pos[i];
            u64 ni = neg[i];
            __int128 diff = (__int128)pi - (__int128)ni - borrow;
            alg[i] = (u64)diff;
            borrow = (diff < 0) ? 1 : 0;
        }
    } else {
        __int128 borrow = 0;
        for (int i = 0; i < 8; i++) {
            u64 pi = pos[i];
            u64 ni = neg[i];
            __int128 diff = (__int128)ni - (__int128)pi - borrow;
            alg[i] = (u64)diff;
            borrow = (diff < 0) ? 1 : 0;
        }
    }

    // log(|alg|)
    double la = -1.0e300;
    for (int i = 7; i >= 0; i--) {
        if (alg[i] != 0) {
            int msb_bit = 63 - __clzll(alg[i]);
            int bits = i * 64 + msb_bit + 1;
            double mantissa = (double)alg[i] / (1ull << msb_bit);
            la = log(mantissa) + (double)bits * 0.6931471805599453;
            break;
        }
    }

    float rs = rat_s[slot];
    float as = alg_s[slot];
    out[slot] = ((double)rs + fudge >= lr && (double)as + fudge >= la) ? 1 : 0;
}
"#;

struct GpuSmooth {
    stream: Arc<CudaStream>,
    hit_batch: CudaFunction,
    mark_exact: CudaFunction,
    mark_log: CudaFunction,
    mark_wide: CudaFunction,
}

fn ptx_cache_path() -> String {
    // FNV-1a over kernel source so edits invalidate the cache.
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in SMOOTH_KERNEL.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("target/gnfs_smooth_{h:016x}.ptx")
}

fn load_or_compile_ptx() -> Result<Ptx, String> {
    let path = ptx_cache_path();
    if std::path::Path::new(&path).is_file() {
        eprintln!("gpu_gnfs ≻ GPU: cached PTX → {path}");
        return Ok(Ptx::from_file(&path));
    }
    eprintln!("gpu_gnfs ≻ GPU: NVRTC compiling log-line kernels (CPU, once)…");
    let t0 = std::time::Instant::now();
    let opts = CompileOptions {
        options: alloc::vec!["--device-int128".into()],
        ..Default::default()
    };
    let ptx = compile_ptx_with_opts(SMOOTH_KERNEL, opts).map_err(|e| format!("NVRTC: {e}"))?;
    let src = ptx.to_src();
    match std::fs::write(&path, src.as_bytes()) {
        Ok(()) => eprintln!(
            "gpu_gnfs ≻ GPU: NVRTC {} ms — cached {path}",
            t0.elapsed().as_millis()
        ),
        Err(e) => eprintln!(
            "gpu_gnfs ≻ GPU: NVRTC {} ms — cache write failed ({e})",
            t0.elapsed().as_millis()
        ),
    }
    Ok(Ptx::from_src(src))
}

fn gpu_setup(device: u32) -> Result<GpuSmooth, String> {
    let ctx = CudaContext::new(device as usize).map_err(|e| format!("CudaContext({device}): {e}"))?;
    let stream = ctx.default_stream();
    let ptx = load_or_compile_ptx()?;
    let module = ctx.load_module(ptx).map_err(|e| format!("load_module: {e}"))?;
    let hit_batch = module
        .load_function("gnfs_hit_batch")
        .map_err(|e| format!("load hit_batch: {e}"))?;
    let mark_exact = module
        .load_function("gnfs_mark_exact")
        .map_err(|e| format!("load mark_exact: {e}"))?;
    let mark_log = module
        .load_function("gnfs_mark_lognorm")
        .map_err(|e| format!("load mark_log: {e}"))?;
    let mark_wide = module
        .load_function("gnfs_mark_wide")
        .map_err(|e| format!("load mark_wide: {e}"))?;
    Ok(GpuSmooth {
        stream,
        hit_batch,
        mark_exact,
        mark_log,
        mark_wide,
    })
}

fn launch_hit_batch(
    gpu: &GpuSmooth,
    a_bound: i64,
    b_count: i64,
    b_lo: i64,
    muls: &[u64],
    ps: &[u64],
    logps: &[f32],
    sieve: &mut CudaSlice<f32>,
    a_count: i32,
) -> Result<u64, String> {
    let n = muls.len();
    if n == 0 {
        return Ok(0);
    }
    debug_assert_eq!(ps.len(), n);
    debug_assert_eq!(logps.len(), n);
    // CUDA gridDim.z max is 65535; keep chunks well under that.
    const Z_CHUNK: usize = 4096;
    let mut launches = 0u64;
    let mut off = 0;
    while off < n {
        let end = (off + Z_CHUNK).min(n);
        let muls_c = &muls[off..end];
        let ps_c = &ps[off..end];
        let log_c = &logps[off..end];
        let nc = end - off;
        let min_p = ps_c.iter().copied().filter(|&p| p >= 2).min().unwrap_or(2);
        let steps = ((2 * a_bound) as u64 / min_p + 2) as u32;
        let bc = b_count as u32;
        let tx = 32u32;
        let ty = 8u32;
        let cfg = LaunchConfig {
            grid_dim: ((steps + tx - 1) / tx, (bc + ty - 1) / ty, nc as u32),
            block_dim: (tx, ty, 1),
            shared_mem_bytes: 0,
        };
        let d_mul = gpu
            .stream
            .clone_htod(muls_c)
            .map_err(|e| format!("htod mul: {e}"))?;
        let d_p = gpu
            .stream
            .clone_htod(ps_c)
            .map_err(|e| format!("htod p: {e}"))?;
        let d_log = gpu
            .stream
            .clone_htod(log_c)
            .map_err(|e| format!("htod logp: {e}"))?;
        let n_i = nc as i32;
        let mut launch = gpu.stream.launch_builder(&gpu.hit_batch);
        unsafe {
            launch.arg(&a_bound);
            launch.arg(&b_count);
            launch.arg(&b_lo);
            launch.arg(&d_mul);
            launch.arg(&d_p);
            launch.arg(&d_log);
            launch.arg(&n_i);
            launch.arg(&mut *sieve);
            launch.arg(&a_count);
            launch
                .launch(cfg)
                .map_err(|e| format!("hit_batch n={nc}: {e}"))?;
        }
        launches += 1;
        off = end;
    }
    Ok(launches)
}

fn gpu_sieve_collect(
    gpu: &GpuSmooth,
    poly: &PolyPair,
    fb: &FactorBases,
    params: &SieveParams,
    exact_mark: bool,
    relations: &mut Vec<Relation>,
    pairs: &mut u64,
    gpu_ms_out: &mut u64,
    cells_out: &mut u64,
    hits_out: &mut u64,
) -> Result<(), String> {
    let m_u64 = if exact_mark {
        poly.m
            .to_u64_digits()
            .first()
            .copied()
            .ok_or_else(|| String::from("m empty"))?
    } else {
        0
    };

    let mut fcoef = alloc::vec![0u64; poly.f.len().max(1)];
    if exact_mark {
        fcoef.clear();
        for c in &poly.f {
            fcoef.push(c.to_u64_digits().first().copied().unwrap_or(0));
        }
    }
    let deg = poly.degree as i32;
    let log_m = log_biguint(&poly.m);
    let log_f: Vec<f64> = poly
        .f
        .iter()
        .map(|c| {
            if c.is_zero() {
                f64::NEG_INFINITY
            } else {
                log_biguint(c)
            }
        })
        .collect();
    let fudge = params.log_fudge;

    let a_bound = params.a_bound;
    let b_max_full = params.b_bound;
    let a_count = (2 * a_bound + 1) as usize;
    if a_count == 0 || b_max_full <= 0 {
        return Err(String::from("empty sieve window"));
    }

    // Hit tables are independent of the b-tile — build once (was redoing Hensel every tile).
    let t_prep = std::time::Instant::now();
    let mut rat_muls = Vec::new();
    let mut rat_ps = Vec::new();
    let mut rat_logs = Vec::new();
    for &p in &fb.rational {
        for (pe, logp) in prime_power_steps(p, a_bound) {
            rat_muls.push(big_mod_u64(&poly.m, pe));
            rat_ps.push(pe);
            rat_logs.push(logp);
        }
    }
    let mut alg_muls = Vec::new();
    let mut alg_ps = Vec::new();
    let mut alg_logs = Vec::new();
    for id in &fb.algebraic {
        for (pe, r_pe, logp) in algebraic_power_hits(poly, id, a_bound) {
            alg_muls.push(r_pe % pe);
            alg_ps.push(pe);
            alg_logs.push(logp);
        }
    }
    let n_rat = rat_ps.len();
    let n_alg = alg_ps.len();
    eprintln!(
        "gpu_gnfs ≻ hit tables: rat={} alg={} prep={}ms (once, not per tile)",
        n_rat,
        n_alg,
        t_prep.elapsed().as_millis()
    );

    // ~9 bytes/cell; tile so device memory stays bounded.
    const MAX_TILE_CELLS: usize = 20_000_000;
    let b_tile = ((MAX_TILE_CELLS / a_count).max(1) as i64).min(b_max_full);
    let n_tiles = ((b_max_full + b_tile - 1) / b_tile) as usize;

    let fb_u = fb.rational.clone();
    let nfb = fb_u.len() as i32;
    let fb_dev = gpu
        .stream
        .clone_htod(&fb_u)
        .map_err(|e| format!("htod fb: {e}"))?;
    let f_dev = gpu
        .stream
        .clone_htod(&fcoef)
        .map_err(|e| format!("htod f: {e}"))?;
    let log_f_dev = gpu
        .stream
        .clone_htod(&log_f)
        .map_err(|e| format!("htod log_f: {e}"))?;

    // Wide (multi-limb) m and f coefficients for faithful survivor cut when m_bits > 64.
    // m_limbs: 8 limbs of m (little-endian, base 2^64)
    // fcoef_limbs: (deg+1) * 8 limbs, each coefficient has 8 limbs
    let m_limbs: [u64; 8] = biguint_to_u512(&poly.m);
    let m_n = u512_nlimbs(&m_limbs);
    let mut fcoef_limbs = Vec::with_capacity((poly.f.len().max(1)) * 8);
    for c in &poly.f {
        let limbs = biguint_to_u512(c);
        fcoef_limbs.extend_from_slice(&limbs);
    }
    let m_dev = if m_n > 0 {
        Some(gpu.stream.clone_htod(&m_limbs).map_err(|e| format!("htod m_limbs: {e}"))?)
    } else {
        None
    };
    let f_dev_wide = if !fcoef_limbs.is_empty() {
        Some(gpu.stream.clone_htod(&fcoef_limbs).map_err(|e| format!("htod fcoef_limbs: {e}"))?)
    } else {
        None
    };

    let a_count_i = a_count as i32;
    let a_bound_ll = a_bound;
    let tx = 16u32;
    let ty = 16u32;
    let mut cells_total = 0u64;
    let mut hit_launches = 0u64;
    let mut gpu_kernel_ms = 0u64;
    let mut host_verify_ms = 0u64;
    let t_wall = std::time::Instant::now();

    // Pending marks from the previous tile — verified on host while the next tile runs on GPU.
    let mut pending: Option<(i64, usize, Vec<u8>)> = None;

    for tile in 0..n_tiles {
        if relations.len() >= params.target_relations || *pairs >= params.max_pairs {
            break;
        }
        let b_lo = 1 + tile as i64 * b_tile;
        let b_hi = (b_lo + b_tile - 1).min(b_max_full);
        let b_count = (b_hi - b_lo + 1) as usize;
        let total = a_count * b_count;
        cells_total += total as u64;
        let b_count_ll = b_count as i64;

        let t_k = std::time::Instant::now();
        let mut rat_s = gpu
            .stream
            .alloc_zeros::<f32>(total)
            .map_err(|e| format!("alloc rat_s: {e}"))?;
        let mut alg_s = gpu
            .stream
            .alloc_zeros::<f32>(total)
            .map_err(|e| format!("alloc alg_s: {e}"))?;
        let mut out_dev = gpu
            .stream
            .alloc_zeros::<u8>(total)
            .map_err(|e| format!("alloc out: {e}"))?;

        hit_launches += launch_hit_batch(
            gpu,
            a_bound_ll,
            b_count_ll,
            b_lo,
            &rat_muls,
            &rat_ps,
            &rat_logs,
            &mut rat_s,
            a_count_i,
        )?;
        hit_launches += launch_hit_batch(
            gpu,
            a_bound_ll,
            b_count_ll,
            b_lo,
            &alg_muls,
            &alg_ps,
            &alg_logs,
            &mut alg_s,
            a_count_i,
        )?;

        let cfg_mark = LaunchConfig {
            grid_dim: (
                (a_count as u32 + tx - 1) / tx,
                (b_count as u32 + ty - 1) / ty,
                1,
            ),
            block_dim: (tx, ty, 1),
            shared_mem_bytes: 0,
        };
        {
            let mut launch = if exact_mark {
                gpu.stream.launch_builder(&gpu.mark_log)
            } else {
                gpu.stream.launch_builder(&gpu.mark_wide)
            };
            unsafe {
                launch.arg(&a_bound_ll);
                launch.arg(&b_count_ll);
                launch.arg(&b_lo);
                launch.arg(&rat_s);
                launch.arg(&alg_s);
                if exact_mark {
                    launch.arg(&log_m);
                    launch.arg(&log_f_dev);
                    launch.arg(&deg);
                } else {
                    launch.arg(m_dev.as_ref().unwrap());
                    launch.arg(&m_n);
                    launch.arg(f_dev_wide.as_ref().unwrap());
                    launch.arg(&deg);
                }
                launch.arg(&fudge);
                launch.arg(&mut out_dev);
                launch.arg(&a_count_i);
                launch
                    .launch(cfg_mark)
                    .map_err(|e| format!("mark kernel: {e}"))?;
            }
        }

        let mask = if exact_mark {
            let mut exact_dev = gpu
                .stream
                .alloc_zeros::<u8>(total)
                .map_err(|e| format!("alloc exact: {e}"))?;
            {
                let mut launch = gpu.stream.launch_builder(&gpu.mark_exact);
                unsafe {
                    launch.arg(&a_bound_ll);
                    launch.arg(&b_count_ll);
                    launch.arg(&b_lo);
                    launch.arg(&m_u64);
                    launch.arg(&rat_s);
                    launch.arg(&alg_s);
                    launch.arg(&fb_dev);
                    launch.arg(&nfb);
                    launch.arg(&f_dev);
                    launch.arg(&deg);
                    launch.arg(&mut exact_dev);
                    launch.arg(&a_count_i);
                    launch
                        .launch(cfg_mark)
                        .map_err(|e| format!("mark_exact: {e}"))?;
                }
            }
            // Overlap: verify previous tile while this tile's kernels run.
            if let Some((pb_lo, pb_count, pmask)) = pending.take() {
                let th = std::time::Instant::now();
                let (surv, _) = verify_mask_relations(
                    poly,
                    fb,
                    a_bound,
                    a_count,
                    pb_lo,
                    pb_count,
                    &pmask,
                    relations,
                    pairs,
                    params,
                );
                host_verify_ms += th.elapsed().as_millis() as u64;
                let _ = surv;
            }
            gpu.stream
                .synchronize()
                .map_err(|e| format!("sync: {e}"))?;
            let log_host = gpu
                .stream
                .clone_dtoh(&out_dev)
                .map_err(|e| format!("dtoh log: {e}"))?;
            let exact_host = gpu
                .stream
                .clone_dtoh(&exact_dev)
                .map_err(|e| format!("dtoh exact: {e}"))?;
            let mut anded = log_host;
            for (l, e) in anded.iter_mut().zip(exact_host.iter()) {
                *l &= *e;
            }
            anded
        } else {
            // Overlap: verify previous tile while this tile's kernels run.
            if let Some((pb_lo, pb_count, pmask)) = pending.take() {
                let th = std::time::Instant::now();
                let (surv, _) = verify_mask_relations(
                    poly,
                    fb,
                    a_bound,
                    a_count,
                    pb_lo,
                    pb_count,
                    &pmask,
                    relations,
                    pairs,
                    params,
                );
                host_verify_ms += th.elapsed().as_millis() as u64;
                let _ = surv;
            }
            gpu.stream
                .synchronize()
                .map_err(|e| format!("sync: {e}"))?;
            gpu.stream
                .clone_dtoh(&out_dev)
                .map_err(|e| format!("dtoh: {e}"))?
        };
        let tile_gpu_ms = t_k.elapsed().as_millis() as u64;
        gpu_kernel_ms += tile_gpu_ms;

        let survivors = mask.iter().filter(|&&x| x != 0).count();
        pending = Some((b_lo, b_count, mask));

        eprintln!(
            "gpu_gnfs ≻ tile {}/{}: relations={}/{} cells={} gpu_ms={} host_verify_ms_cum={} survivors_queued={} (rat≈{} alg≈{})",
            tile + 1,
            n_tiles,
            relations.len(),
            params.target_relations,
            cells_total,
            tile_gpu_ms,
            host_verify_ms,
            survivors,
            n_rat,
            n_alg
        );

        if relations.len() >= params.target_relations || *pairs >= params.max_pairs {
            break;
        }
    }

    // Drain last tile's marks on the host.
    if let Some((pb_lo, pb_count, pmask)) = pending.take() {
        let th = std::time::Instant::now();
        let (surv, _) = verify_mask_relations(
            poly,
            fb,
            a_bound,
            a_count,
            pb_lo,
            pb_count,
            &pmask,
            relations,
            pairs,
            params,
        );
        let drain_ms = th.elapsed().as_millis() as u64;
        host_verify_ms += drain_ms;
        eprintln!(
            "gpu_gnfs ≻ host drain: survivors={} relations={} verify_ms={}",
            surv,
            relations.len(),
            drain_ms
        );
    }

    let wall_ms = t_wall.elapsed().as_millis() as u64;
    *gpu_ms_out = gpu_kernel_ms;
    *cells_out = cells_total;
    *hits_out = hit_launches;
    eprintln!(
        "gpu_gnfs ≻ GPU log-line done: cells={} hit-launches={} gpu_ms={} host_verify_ms={} wall_ms={} relations={}/{}",
        cells_total,
        hit_launches,
        gpu_kernel_ms,
        host_verify_ms,
        wall_ms,
        relations.len(),
        params.target_relations
    );
    Ok(())
}

/// Host exact-smoothness check of GPU-marked (a,b) cells.
fn verify_mask_relations(
    poly: &PolyPair,
    fb: &FactorBases,
    a_bound: i64,
    a_count: usize,
    b_lo: i64,
    b_count: usize,
    mask: &[u8],
    relations: &mut Vec<Relation>,
    pairs: &mut u64,
    params: &SieveParams,
) -> (u64, u64) {
    let mut survivors = 0u64;
    let mut accepted = 0u64;
    for b_idx in 0..b_count {
        if relations.len() >= params.target_relations || *pairs >= params.max_pairs {
            break;
        }
        let b = b_lo + b_idx as i64;
        for a_idx in 0..a_count {
            *pairs += 1;
            if mask[b_idx * a_count + a_idx] == 0 {
                continue;
            }
            survivors += 1;
            let a = -a_bound + a_idx as i64;
            if let Some(rel) = try_relation(poly, fb, a, b) {
                relations.push(rel);
                accepted += 1;
                if relations.len() >= params.target_relations {
                    break;
                }
            }
        }
    }
    (survivors, accepted)
}

pub fn sieve_status_line(rep: &SieveReport, fb: &FactorBases) -> String {
    format!(
        "gpu_gnfs ≻ sieve: pairs={} relations={}/{} |A|={} Bmax={} ncols={} gpu={} device={} gpu_ms={} cells={}",
        rep.pairs_tested,
        rep.relations.len(),
        rep.params.target_relations,
        rep.params.a_bound,
        rep.params.b_bound,
        fb.ncols(),
        rep.used_gpu,
        rep.device,
        rep.gpu_ms,
        rep.cells
    )
}
