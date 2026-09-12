//! gpu_gnfs_linalg.rs — ⋈≻∋⊤⊣ matrix, dependency, congruence of squares, factor.
//!
//! φ: Z[α] → Z/NZ with α ↦ m is a ring homomorphism because f(m)=N≡0 (mod N).
//! There is no ring hom into Z (f(m)=N≠0), so φ(β) mod N can be a CRT-mixed
//! square root of u² and gcd(|u−φ(β)|, N) can split N.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed, Zero};

use crate::gpu_gnfs_poly::PolyPair;
use crate::gpu_gnfs_sieve::Relation;
use crate::prime_winding::big_gcd;

pub fn isqrt(n: &BigUint) -> BigUint {
    use crate::native_numeral::{add_via_word, divmod_small_via_word, divmod_via_word};
    if n < &BigUint::from(2u32) {
        return n.clone();
    }
    let mut x = n.clone();
    let mut y = divmod_small_via_word(&add_via_word(&x, &BigUint::from(1u32)), 2).unwrap().0;
    while y < x {
        x = y.clone();
        let q = divmod_via_word(n, &x).unwrap().0;
        y = divmod_small_via_word(&add_via_word(&x, &q), 2).unwrap().0;
    }
    x
}

fn is_perfect_square(n: &BigUint) -> Option<BigUint> {
    let r = isqrt(n);
    if crate::native_numeral::multiply_via_word(&r, &r) == *n {
        Some(r)
    } else {
        None
    }
}

fn bigint_mod_n(a: &BigInt, n: &BigUint) -> BigUint {
    use crate::native_numeral::{signed_add_via_word_general, signed_divmod_via_word};
    let n_bi = BigInt::from_biguint(Sign::Plus, n.clone());
    let mut r = signed_divmod_via_word(a, &n_bi).unwrap().1;
    if r.sign() == Sign::Minus {
        r = signed_add_via_word_general(&r, &n_bi);
    }
    r.to_biguint().unwrap_or_else(BigUint::zero)
}

fn biguint_to_i64(u: &BigUint) -> Option<i64> {
    u.to_u64_digits()
        .first()
        .copied()
        .and_then(|v| i64::try_from(v).ok())
}

type ModElem = Vec<BigUint>;

fn mod_zero(d: usize) -> ModElem {
    alloc::vec![BigUint::zero(); d]
}

fn mod_one(d: usize) -> ModElem {
    let mut e = mod_zero(d);
    e[0] = BigUint::one();
    e
}

fn mod_linear(a: i64, b: i64, d: usize, n: &BigUint) -> ModElem {
    let mut e = mod_zero(d);
    e[0] = bigint_mod_n(&BigInt::from(a), n);
    if d > 1 {
        e[1] = bigint_mod_n(&-BigInt::from(b), n);
    }
    e
}

fn mod_inv(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{
        modulo_via_word, signed_add_via_word_general, signed_divmod_via_word,
        signed_multiply_via_word, signed_subtract_via_word_general,
    };
    let mut r0 = BigInt::from_biguint(Sign::Plus, n.clone());
    let mut r1 = BigInt::from_biguint(Sign::Plus, a.clone());
    let mut s0 = BigInt::zero();
    let mut s1 = BigInt::one();
    while !r1.is_zero() {
        let q = signed_divmod_via_word(&r0, &r1).unwrap().0;
        let r2 = signed_subtract_via_word_general(&r0, &signed_multiply_via_word(&q, &r1));
        r0 = r1;
        r1 = r2;
        let s2 = signed_subtract_via_word_general(&s0, &signed_multiply_via_word(&q, &s1));
        s0 = s1;
        s1 = s2;
    }
    if r0.abs() != BigInt::one() {
        return None;
    }
    if s0.sign() == Sign::Minus {
        s0 = signed_add_via_word_general(&s0, &BigInt::from_biguint(Sign::Plus, n.clone()));
    }
    Some(modulo_via_word(&s0.to_biguint().unwrap_or_else(BigUint::zero), n).unwrap())
}

fn mod_mul(x: &ModElem, y: &ModElem, f: &[BigUint], n: &BigUint) -> Result<ModElem, BigUint> {
    use crate::native_numeral::{add_via_word, modulo_via_word, multiply_via_word, subtract_via_word};
    let d = f.len() - 1;
    let lead = &f[d];
    let lead_mod = modulo_via_word(lead, n).unwrap();
    let g_lead = big_gcd(lead_mod.clone(), n.clone());
    if g_lead > BigUint::one() && &g_lead < n {
        return Err(g_lead);
    }
    let lead_inv = mod_inv(&lead_mod, n).ok_or_else(|| big_gcd(lead_mod.clone(), n.clone()))?;

    let mut prod = alloc::vec![BigUint::zero(); 2 * d];
    for i in 0..d {
        for j in 0..d {
            let xi = x.get(i).cloned().unwrap_or_else(BigUint::zero);
            let yj = y.get(j).cloned().unwrap_or_else(BigUint::zero);
            prod[i + j] = modulo_via_word(&add_via_word(&prod[i + j], &modulo_via_word(&multiply_via_word(&xi, &yj), n).unwrap()), n).unwrap();
        }
    }
    for deg in (d..=2 * d - 1).rev() {
        if prod[deg].is_zero() {
            continue;
        }
        let coef = prod[deg].clone();
        prod[deg] = BigUint::zero();
        let shift = deg - d;
        let scale = modulo_via_word(&multiply_via_word(&coef, &lead_inv), n).unwrap();
        for k in 0..d {
            let ak = modulo_via_word(&f[k], n).unwrap();
            let term = modulo_via_word(&multiply_via_word(&scale, &ak), n).unwrap();
            let idx = shift + k;
            if prod[idx] >= term {
                prod[idx] = subtract_via_word(&prod[idx], &term).unwrap();
            } else {
                prod[idx] = modulo_via_word(&subtract_via_word(&add_via_word(&prod[idx], n), &term).unwrap(), n).unwrap();
            }
        }
    }
    prod.truncate(d);
    Ok(prod)
}

fn mod_pow2(x: &ModElem, f: &[BigUint], n: &BigUint) -> Result<ModElem, BigUint> {
    mod_mul(x, x, f, n)
}

fn mod_eq(x: &ModElem, y: &ModElem) -> bool {
    let d = x.len().max(y.len());
    for i in 0..d {
        let xi = x.get(i).cloned().unwrap_or_else(BigUint::zero);
        let yi = y.get(i).cloned().unwrap_or_else(BigUint::zero);
        if xi != yi {
            return false;
        }
    }
    true
}

fn phi_mod(elem: &ModElem, m: &BigUint, n: &BigUint) -> BigUint {
    use crate::native_numeral::{add_via_word, modulo_via_word, multiply_via_word};
    let mut acc = BigUint::zero();
    let mut pow = BigUint::one();
    for c in elem {
        acc = modulo_via_word(&add_via_word(&acc, &modulo_via_word(&multiply_via_word(c, &pow), n).unwrap()), n).unwrap();
        pow = modulo_via_word(&multiply_via_word(&pow, m), n).unwrap();
    }
    acc
}

pub fn product_algebraic_mod(
    relations: &[Relation],
    dep: &[usize],
    poly: &PolyPair,
    n: &BigUint,
) -> Result<ModElem, BigUint> {
    let d = poly.degree as usize;
    let mut acc = mod_one(d);
    for &i in dep {
        let r = &relations[i];
        let lin = mod_linear(r.a, r.b, d, n);
        acc = mod_mul(&acc, &lin, &poly.f, n)?;
    }
    Ok(acc)
}

/// Brute β in (Z/NZ)^d. Cap keeps runtime sane (N≤400, d≤3).
fn find_sqrt_mod_brute(
    delta: &ModElem,
    f: &[BigUint],
    n: &BigUint,
) -> Result<Option<ModElem>, BigUint> {
    let d = f.len() - 1;
    let n_u = match biguint_to_i64(n) {
        Some(v) if v <= 400 && d <= 3 => v,
        _ => return Ok(None),
    };
    let mut idx = alloc::vec![0i64; d];
    loop {
        let beta: ModElem = idx.iter().map(|&c| BigUint::from(c as u64)).collect();
        let sq = mod_pow2(&beta, f, n)?;
        if mod_eq(&sq, delta) {
            return Ok(Some(beta));
        }
        let mut k = 0;
        while k < d {
            idx[k] += 1;
            if idx[k] < n_u {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
        if k == d {
            break;
        }
    }
    Ok(None)
}

// ─── Q(α) arithmetic for non-monic base-m f ─────────────────────────

#[derive(Clone, Debug)]
struct QElem {
    c: Vec<BigInt>,
    den: BigInt,
}

impl QElem {
    fn one(d: usize) -> Self {
        let mut c = alloc::vec![BigInt::zero(); d];
        c[0] = BigInt::one();
        Self {
            c,
            den: BigInt::one(),
        }
    }

    fn linear(a: i64, b: i64, d: usize) -> Self {
        let mut c = alloc::vec![BigInt::zero(); d];
        c[0] = BigInt::from(a);
        if d > 1 {
            c[1] = -BigInt::from(b);
        }
        Self {
            c,
            den: BigInt::one(),
        }
    }

    fn normalize(&mut self) {
        if self.den.sign() == Sign::Minus {
            self.den = -self.den.clone();
            for c in &mut self.c {
                *c = -c.clone();
            }
        }
        let mut g = self.den.abs().to_biguint().unwrap_or_else(BigUint::one);
        for c in &self.c {
            if !c.is_zero() {
                g = big_gcd(g, c.abs().to_biguint().unwrap_or_else(BigUint::one));
            }
        }
        if g > BigUint::one() {
            use crate::native_numeral::signed_divmod_via_word;
            let g_bi = BigInt::from_biguint(Sign::Plus, g);
            self.den = signed_divmod_via_word(&self.den, &g_bi).unwrap().0;
            for c in &mut self.c {
                *c = signed_divmod_via_word(c, &g_bi).unwrap().0;
            }
        }
    }
}

fn q_mul(x: &QElem, y: &QElem, f: &[BigUint]) -> Option<QElem> {
    use crate::native_numeral::{signed_add_via_word_general, signed_multiply_via_word, signed_subtract_via_word_general};
    let d = f.len() - 1;
    let lead = BigInt::from_biguint(Sign::Plus, f[d].clone());
    if lead.is_zero() {
        return None;
    }
    let mut prod = alloc::vec![BigInt::zero(); 2 * d];
    for i in 0..d {
        for j in 0..d {
            let xi = x.c.get(i).cloned().unwrap_or_else(BigInt::zero);
            let yj = y.c.get(j).cloned().unwrap_or_else(BigInt::zero);
            prod[i + j] = signed_add_via_word_general(&prod[i + j], &signed_multiply_via_word(&xi, &yj));
        }
    }
    let mut den = signed_multiply_via_word(&x.den, &y.den);
    for deg in (d..prod.len()).rev() {
        if prod[deg].is_zero() {
            continue;
        }
        let coef = prod[deg].clone();
        prod[deg] = BigInt::zero();
        let shift = deg - d;
        den = signed_multiply_via_word(&den, &lead);
        for i in 0..deg {
            prod[i] = signed_multiply_via_word(&prod[i], &lead);
        }
        for k in 0..d {
            let ak = BigInt::from_biguint(Sign::Plus, f[k].clone());
            prod[shift + k] = signed_subtract_via_word_general(&prod[shift + k], &signed_multiply_via_word(&coef, &ak));
        }
    }
    prod.truncate(d);
    let mut e = QElem { c: prod, den };
    e.normalize();
    Some(e)
}

fn product_algebraic_q(relations: &[Relation], dep: &[usize], poly: &PolyPair) -> Option<QElem> {
    let d = poly.degree as usize;
    let mut acc = QElem::one(d);
    for &i in dep {
        let r = &relations[i];
        acc = q_mul(&acc, &QElem::linear(r.a, r.b, d), &poly.f)?;
    }
    Some(acc)
}

fn q_phi_mod(e: &QElem, m: &BigUint, n: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{add_via_word, modulo_via_word, multiply_via_word};
    let den = bigint_mod_n(&e.den, n);
    let inv = mod_inv(&den, n)?;
    let mut acc = BigUint::zero();
    let mut pow = BigUint::one();
    for c in &e.c {
        let cm = bigint_mod_n(c, n);
        acc = modulo_via_word(&add_via_word(&acc, &modulo_via_word(&multiply_via_word(&cm, &pow), n).unwrap()), n).unwrap();
        pow = modulo_via_word(&multiply_via_word(&pow, m), n).unwrap();
    }
    Some(modulo_via_word(&multiply_via_word(&acc, &inv), n).unwrap())
}

fn int_ring_mul(x: &[BigInt], y: &[BigInt], f: &[BigUint]) -> Option<Vec<BigInt>> {
    use crate::native_numeral::{signed_add_via_word_general, signed_divmod_via_word, signed_multiply_via_word, signed_subtract_via_word_general};
    let d = f.len() - 1;
    let lead = BigInt::from_biguint(Sign::Plus, f[d].clone());
    if lead.is_zero() {
        return None;
    }
    let mut prod = alloc::vec![BigInt::zero(); 2 * d];
    for i in 0..x.len().min(d) {
        for j in 0..y.len().min(d) {
            prod[i + j] = signed_add_via_word_general(&prod[i + j], &signed_multiply_via_word(&x[i], &y[j]));
        }
    }
    for deg in (d..=2 * d - 1).rev() {
        if prod[deg].is_zero() {
            continue;
        }
        let coef = prod[deg].clone();
        prod[deg] = BigInt::zero();
        let (scale, rem) = signed_divmod_via_word(&coef, &lead).unwrap();
        if !rem.is_zero() {
            return None;
        }
        let shift = deg - d;
        for k in 0..d {
            let ak = BigInt::from_biguint(Sign::Plus, f[k].clone());
            prod[shift + k] = signed_subtract_via_word_general(&prod[shift + k], &signed_multiply_via_word(&scale, &ak));
        }
    }
    prod.truncate(d);
    Some(prod)
}

fn int_ring_pow2(x: &[BigInt], f: &[BigUint]) -> Option<Vec<BigInt>> {
    int_ring_mul(x, x, f)
}

fn find_sqrt_int_brute(delta_int: &[BigInt], f: &[BigUint], bound: i64) -> Option<Vec<BigInt>> {
    let d = f.len() - 1;
    let range: Vec<i64> = (-bound..=bound).collect();
    let nstates = range.len().saturating_pow(d as u32);
    if nstates == 0 || nstates > 2_000_000 {
        return None;
    }
    let mut idx = alloc::vec![0usize; d];
    for _ in 0..nstates {
        let beta: Vec<BigInt> = idx.iter().map(|&i| BigInt::from(range[i])).collect();
        if !beta.iter().all(|c| c.is_zero()) {
            if let Some(sq) = int_ring_pow2(&beta, f) {
                let ok = sq.len() >= delta_int.len()
                    && (0..delta_int.len()).all(|i| sq[i] == delta_int[i])
                    && sq.iter().skip(delta_int.len()).all(|c| c.is_zero());
                if ok {
                    return Some(beta);
                }
            }
        }
        let mut k = 0;
        while k < d {
            idx[k] += 1;
            if idx[k] < range.len() {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
        if k == d {
            break;
        }
    }
    None
}

fn q_sqrt(e: &QElem, f: &[BigUint], bound: i64) -> Option<QElem> {
    let den_abs = e.den.abs().to_biguint().unwrap_or_else(BigUint::one);
    let den_sq = is_perfect_square(&den_abs)?;
    let den_sqrt = BigInt::from_biguint(Sign::Plus, den_sq);
    // Prefer searching sqrt in Q-ring via integer pow2 on cleared content:
    // if den is square, find β with β² = e as QElems by searching numerator
    // against e.c after accounting for den — try direct int sqrt of e.c first
    // when den==1, else search β such that q_mul(β,β)=e.
    let d = f.len() - 1;
    let range: Vec<i64> = (-bound..=bound).collect();
    let nstates = range.len().saturating_pow(d as u32);
    if nstates == 0 || nstates > 2_000_000 {
        return None;
    }
    let mut idx = alloc::vec![0usize; d];
    for _ in 0..nstates {
        let mut beta = QElem {
            c: idx.iter().map(|&i| BigInt::from(range[i])).collect(),
            den: den_sqrt.clone(),
        };
        if beta.c.iter().all(|c| c.is_zero()) {
            // advance
        } else {
            beta.normalize();
            if let Some(sq) = q_mul(&beta, &beta, f) {
                let mut a = e.clone();
                a.normalize();
                if a.den == sq.den && a.c == sq.c {
                    return Some(beta);
                }
            }
        }
        let mut k = 0;
        while k < d {
            idx[k] += 1;
            if idx[k] < range.len() {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
        if k == d {
            break;
        }
    }
    // also try den=1 roots of numerator when den_sq path failed shape
    if let Some(beta_c) = find_sqrt_int_brute(&e.c, f, bound) {
        let mut out = QElem {
            c: beta_c,
            den: den_sqrt,
        };
        out.normalize();
        if let Some(sq) = q_mul(&out, &out, f) {
            let mut a = e.clone();
            a.normalize();
            if a.den == sq.den && a.c == sq.c {
                return Some(out);
            }
        }
    }
    None
}

pub fn find_dependencies(relations: &[Relation], max_deps: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    if relations.is_empty() {
        return out;
    }
    let nrows = relations.len();
    let ncols = relations[0].exps.len();
    let words = (ncols + 63) / 64;
    let mut mat: Vec<Vec<u64>> = relations
        .iter()
        .map(|r| {
            let mut row = alloc::vec![0u64; words];
            for (c, &e) in r.exps.iter().enumerate() {
                if e != 0 {
                    row[c / 64] |= 1u64 << (c % 64);
                }
            }
            row
        })
        .collect();
    let mut hist: Vec<Vec<u8>> = (0..nrows)
        .map(|i| {
            let mut h = alloc::vec![0u8; nrows];
            h[i] = 1;
            h
        })
        .collect();

    let mut row_i = 0usize;
    for col in 0..ncols {
        if row_i >= nrows {
            break;
        }
        let mut piv = None;
        for r in row_i..nrows {
            if (mat[r][col / 64] >> (col % 64)) & 1 == 1 {
                piv = Some(r);
                break;
            }
        }
        let Some(p) = piv else { continue };
        mat.swap(row_i, p);
        hist.swap(row_i, p);
        for r in 0..nrows {
            if r == row_i {
                continue;
            }
            if (mat[r][col / 64] >> (col % 64)) & 1 == 1 {
                for w in 0..words {
                    mat[r][w] ^= mat[row_i][w];
                }
                for k in 0..nrows {
                    hist[r][k] ^= hist[row_i][k];
                }
            }
        }
        row_i += 1;
    }

    for r in 0..nrows {
        if out.len() >= max_deps {
            break;
        }
        if !mat[r].iter().all(|&w| w == 0) {
            continue;
        }
        let idxs: Vec<usize> = hist[r]
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b != 0 { Some(i) } else { None })
            .collect();
        if !idxs.is_empty() {
            out.push(idxs);
        }
    }

    if out.is_empty() && nrows <= 22 {
        let lim = 1u32 << nrows;
        for mask in 1..lim {
            if out.len() >= max_deps {
                break;
            }
            let mut acc = alloc::vec![0u8; ncols];
            let mut idxs = Vec::new();
            for i in 0..nrows {
                if (mask >> i) & 1 == 1 {
                    idxs.push(i);
                    for c in 0..ncols {
                        acc[c] ^= relations[i].exps[c];
                    }
                }
            }
            if acc.iter().all(|&e| e == 0) {
                out.push(idxs);
            }
        }
    }
    out
}

pub struct SquareCongruence {
    pub x: BigUint,
    pub y: BigUint,
    pub factor: Option<BigUint>,
    pub dep: Vec<usize>,
}

fn try_gcd_factor(x: &BigUint, y: &BigUint, n: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{add_via_word, modulo_via_word, subtract_via_word};
    let candidates = [
        if x >= y { subtract_via_word(x, y).unwrap() } else { subtract_via_word(y, x).unwrap() },
        add_via_word(x, y),
        x.clone(),
        y.clone(),
    ];
    for c in candidates {
        if c.is_zero() {
            continue;
        }
        let g = big_gcd(modulo_via_word(&c, n).unwrap(), n.clone());
        if g > BigUint::one() && &g < n {
            return Some(g);
        }
    }
    None
}

fn resolve_y(
    n: &BigUint,
    poly: &PolyPair,
    relations: &[Relation],
    dep: &[usize],
) -> Result<BigUint, String> {
    // 1) Modular product + brute √ (tiny N)
    match product_algebraic_mod(relations, dep, poly, n) {
        Err(factor) => {
            // lead/inv revealed a factor — surface via Err as special? use Ok(0) marker
            // Caller checks gcd(factor). Return as y=0 and let gcd(x,0) path — better return Err with factor tag.
            return Err(format!("FACTOR:{factor}"));
        }
        Ok(delta) => match find_sqrt_mod_brute(&delta, &poly.f, n) {
            Err(factor) => return Err(format!("FACTOR:{factor}")),
            Ok(Some(beta)) => return Ok(phi_mod(&beta, &poly.m, n)),
            Ok(None) => {}
        },
    }

    // 2) Q(α) product + small √ (cheap bounds only — large bounds dominate runtime)
    if let Some(delta_q) = product_algebraic_q(relations, dep, poly) {
        for bound in [6i64, 12, 20] {
            if let Some(beta) = q_sqrt(&delta_q, &poly.f, bound) {
                if let Some(y) = q_phi_mod(&beta, &poly.m, n) {
                    return Ok(y);
                }
                let den = bigint_mod_n(&beta.den, n);
                let g = big_gcd(den, n.clone());
                if g > BigUint::one() && &g < n {
                    return Err(format!("FACTOR:{g}"));
                }
            }
        }
    }

    // 3) Norm-product fallback
    let mut alg = BigUint::one();
    for &i in dep {
        alg = crate::native_numeral::multiply_via_word(&alg, &relations[i].alg_abs);
    }
    match is_perfect_square(&alg) {
        Some(s) => Ok(crate::native_numeral::modulo_via_word(&s, n).unwrap()),
        None => Err(format!(
            "gpu_gnfs ∋: no alg √ for dep {}",
            format_dep(dep)
        )),
    }
}

/// ∋⊤ — from dependencies, build X² ≡ Y² (mod N) via φ and extract a factor.
pub fn congruence_factor(
    n: &BigUint,
    poly: &PolyPair,
    relations: &[Relation],
    deps: &[Vec<usize>],
) -> Result<SquareCongruence, String> {
    use crate::native_numeral::{modulo_via_word, signed_multiply_via_word};
    if let Some(lead) = poly.f.last() {
        let g = big_gcd(modulo_via_word(lead, n).unwrap(), n.clone());
        if g > BigUint::one() && &g < n {
            return Ok(SquareCongruence {
                x: BigUint::zero(),
                y: BigUint::zero(),
                factor: Some(g),
                dep: Vec::new(),
            });
        }
    }

    let mut last_err = String::from("gpu_gnfs ∋: no dependency yielded a factor");
    for dep in deps {
        if dep.is_empty() {
            continue;
        }
        let mut rat = BigInt::one();
        for &i in dep {
            rat = signed_multiply_via_word(&rat, &relations[i].rat_signed);
        }
        let rat_abs = rat.abs().to_biguint().unwrap_or_else(BigUint::zero);
        let u = match is_perfect_square(&rat_abs) {
            Some(s) => s,
            None => {
                last_err = format!(
                    "gpu_gnfs ∋: rational product not square (bits={}, |dep|={})",
                    rat_abs.bits(),
                    dep.len()
                );
                continue;
            }
        };
        let x = modulo_via_word(&u, n).unwrap();

        let y = match resolve_y(n, poly, relations, dep) {
            Ok(y) => y,
            Err(e) if e.starts_with("FACTOR:") => {
                let f: BigUint = e.trim_start_matches("FACTOR:").parse().unwrap_or_else(|_| BigUint::zero());
                if f > BigUint::one() && &f < n {
                    return Ok(SquareCongruence {
                        x: x.clone(),
                        y: BigUint::zero(),
                        factor: Some(f),
                        dep: dep.clone(),
                    });
                }
                last_err = e;
                continue;
            }
            Err(e) => {
                last_err = e;
                continue;
            }
        };

        if let Some(factor) = try_gcd_factor(&x, &y, n) {
            return Ok(SquareCongruence {
                x,
                y,
                factor: Some(factor),
                dep: dep.clone(),
            });
        }
        last_err = format!(
            "gpu_gnfs ∋: gcd trivial dep {} x={} y={}",
            format_dep(dep),
            x,
            y
        );
    }
    Err(last_err)
}

pub fn format_dep(dep: &[usize]) -> String {
    let mut s = String::from("[");
    for (i, &d) in dep.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&format!("{d}"));
    }
    s.push(']');
    s
}

#[cfg(test)]
mod word_native_conversion_tests {
    // These four functions (isqrt, is_perfect_square, mod_inv, mod_mul,
    // phi_mod) carry the pure-CPU host fallback resolve_y/congruence_factor
    // uses when both the u64 fast path and the multi-limb GPU path fail --
    // a branch not exercised by any live gpu_gnfs run this session (8051
    // and its multi-limb-forced variant both took a different path).
    // Checked directly here against values computed independently of the
    // converted primitives, not just against each other.
    use super::*;

    #[test]
    fn isqrt_matches_known_values() {
        assert_eq!(isqrt(&BigUint::from(0u32)), BigUint::from(0u32));
        assert_eq!(isqrt(&BigUint::from(1u32)), BigUint::from(1u32));
        assert_eq!(isqrt(&BigUint::from(99u32)), BigUint::from(9u32));
        assert_eq!(isqrt(&BigUint::from(100u32)), BigUint::from(10u32));
        assert_eq!(isqrt(&BigUint::from(101u32)), BigUint::from(10u32));
        // A large value: floor(sqrt(n))^2 <= n < (floor(sqrt(n))+1)^2.
        let n = BigUint::from(123456789123456789u64);
        let r = isqrt(&n);
        assert!(&r * &r <= n);
        assert!((&r + BigUint::from(1u32)) * (&r + BigUint::from(1u32)) > n);
    }

    #[test]
    fn is_perfect_square_matches_known_values() {
        assert_eq!(is_perfect_square(&BigUint::from(144u32)), Some(BigUint::from(12u32)));
        assert_eq!(is_perfect_square(&BigUint::from(145u32)), None);
        assert_eq!(is_perfect_square(&BigUint::from(0u32)), Some(BigUint::from(0u32)));
    }

    #[test]
    fn mod_inv_matches_known_inverse() {
        // 3 * 5 = 15 = 2*7 + 1, so 5 is 3's inverse mod 7.
        let inv = mod_inv(&BigUint::from(3u32), &BigUint::from(7u32)).unwrap();
        assert_eq!(inv, BigUint::from(5u32));
        assert_eq!((BigUint::from(3u32) * &inv) % BigUint::from(7u32), BigUint::from(1u32));
        // gcd(4, 8) != 1, no inverse.
        assert!(mod_inv(&BigUint::from(4u32), &BigUint::from(8u32)).is_none());
    }

    #[test]
    fn mod_mul_matches_hand_computed_ring_product() {
        // Ring Z[x]/(x^2+1) mod 13: f = [1, 0, 1] (x^2 + 1), so x^2 ≡ -1 ≡ 12.
        // (2 + 3x)(4 + 5x) = 8 + 10x + 12x + 15x^2 = 8 + 22x + 15x^2
        //   ≡ 8 + 22x + 15*12 (mod 13, replacing x^2)
        //   = 8 + 22x + 180 ≡ (8+180 mod 13) + (22 mod 13)x = (188 mod 13) + 9x
        // 188 = 14*13 + 6, so result is 6 + 9x.
        let n = BigUint::from(13u32);
        let f = alloc::vec![BigUint::from(1u32), BigUint::zero(), BigUint::from(1u32)];
        let x = alloc::vec![BigUint::from(2u32), BigUint::from(3u32)];
        let y = alloc::vec![BigUint::from(4u32), BigUint::from(5u32)];
        let prod = mod_mul(&x, &y, &f, &n).unwrap();
        assert_eq!(prod, alloc::vec![BigUint::from(6u32), BigUint::from(9u32)]);
    }

    #[test]
    fn phi_mod_matches_hand_computed_evaluation() {
        // elem = [2, 3, 4] read as 2 + 3*m + 4*m^2, m=5, n=1000: 2+15+100=117.
        let elem = alloc::vec![BigUint::from(2u32), BigUint::from(3u32), BigUint::from(4u32)];
        let got = phi_mod(&elem, &BigUint::from(5u32), &BigUint::from(1000u32));
        assert_eq!(got, BigUint::from(117u32));
    }
}
