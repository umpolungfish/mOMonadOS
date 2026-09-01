//! opi.rs — Optimal Polynomial Intersection, matched to the real paper.
//!
//! First version of this file built a simpler problem (single target
//! value per point) and, worse, its report compared a satisfaction
//! FRACTION against a number that was actually an expected COUNT
//! (n/p), not a fraction (1/p) -- a units error that only looked
//! sane because n happened to be near p. Caught by reading the actual
//! paper (Optimization_by_Decoded_Quantum_Interferometry.pdf, ig-docs)
//! instead of continuing to guess at "balanced." Rebuilt from
//! Definition 2.2 and §11.3 directly.
//!
//! Definition 2.2 (OPI): fix prime p and n < p-1. Let F_1..F_{p-1} be
//! subsets of GF(p). Find Q of degree ≤ n-1 maximizing the count of
//! y ∈ {1,...,p-1} with Q(y) ∈ F_y. §5 re-expresses this using a
//! primitive root γ of GF(p): the m = p-1 constraint points are
//! γ^0, ..., γ^{p-2}, and "balanced" (§5, eq. 15) means every F_i has
//! size ⌊p/2⌋ -- stated in the paper itself, not a construction I chose.
//! The sets F_i here are built as independent uniformly random
//! size-⌊p/2⌋ subsets of GF(p), matching the paper's own analysis
//! assumption "assuming the f_i are random" (§11.3).
//!
//! §11.3, Prange's algorithm, reproduced exactly: throw away all but n
//! of the m constraints, pick an arbitrary element of F_i for each kept
//! constraint, solve the resulting exactly-determined system (Lagrange
//! interpolation here), then measure how many of all m constraints the
//! resulting polynomial satisfies. Repeated over many trials. The
//! paper's own closed-form prediction (derived directly from its plain-
//! English description, cross-checked against its worked example: at
//! r/p=1/2, n/p=1/2, this gives 0.75, matching the paper's own quoted
//! number exactly):
//!   phi_PR = n/m + (1 - n/m) * (r/p),   r = floor(p/2), m = p-1.
//!
//! The paper's own DQI+Berlekamp-Massey asymptotic prediction (eq. 16,
//! m -> p limit), reported alongside for reference -- this is THEIR
//! formula's output at these parameters, not a run of anything:
//!   phi_DQI = 1/2 + sqrt( (n/2p) * (1 - n/2p) )
//! Cross-checked against the paper's own quoted n/p=1/2 example
//! (phi_DQI -> 0.9330): computed here as 1/2 + sqrt(0.25*0.75) =
//! 0.9330, matching to four digits.
//!
//! WHAT THIS DOES NOT CLAIM: no DQI circuit for OPI exists in this
//! codebase, simulated or otherwise. phi_DQI below is the paper's
//! closed-form asymptotic prediction, evaluated at these parameters --
//! a citation, not a measurement. Only phi_PR's measured column is a
//! real run.

#![allow(dead_code)]

use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

struct Xorshift(u64);
impl Xorshift {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    (a + p - (b % p)) % p
}
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn mod_pow(base: u64, exp: u64, p: u64) -> u64 {
    let mut base = base % p;
    let mut exp = exp;
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 != 0 {
            result = mul_mod(result, base, p);
        }
        base = mul_mod(base, base, p);
        exp >>= 1;
    }
    result
}

/// Modular inverse via Fermat's little theorem: a^(p-2) mod p, valid
/// whenever p is prime and a is not a multiple of p.
fn mod_inv(a: u64, p: u64) -> u64 {
    mod_pow(a, p - 2, p)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Distinct prime factors of n, by trial division -- n here is always
/// p-1 for a modest prime p, small enough that trial division is exact
/// and fast.
fn distinct_prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/// A primitive root of GF(p): g such that g^((p-1)/q) != 1 mod p for
/// every prime factor q of p-1 -- the standard, exact test (not a
/// probabilistic guess), checked against every prime factor of p-1.
fn find_primitive_root(p: u64) -> u64 {
    let factors = distinct_prime_factors(p - 1);
    let mut g = 2u64;
    loop {
        if factors.iter().all(|&q| mod_pow(g, (p - 1) / q, p) != 1) {
            return g;
        }
        g += 1;
    }
}

/// Lagrange interpolation over GF(p): the unique polynomial of degree
/// < points.len() passing exactly through every given (x, y) pair,
/// returned in coefficient form (index i = coefficient of x^i).
fn lagrange_interpolate(points: &[(u64, u64)], p: u64) -> Vec<u64> {
    let k = points.len();
    let mut coeffs = alloc::vec![0u64; k];
    for i in 0..k {
        let (xi, yi) = points[i];
        let mut basis = alloc::vec![0u64; k];
        basis[0] = 1;
        let mut degree = 0usize;
        let mut denom = 1u64;
        for j in 0..k {
            if j == i {
                continue;
            }
            let (xj, _) = points[j];
            let mut next = alloc::vec![0u64; k];
            for d in 0..=degree {
                if d + 1 < k {
                    next[d + 1] = add_mod(next[d + 1], basis[d], p);
                }
                next[d] = sub_mod(next[d], mul_mod(basis[d], xj, p), p);
            }
            basis = next;
            degree += 1;
            denom = mul_mod(denom, sub_mod(xi, xj, p), p);
        }
        let scale = mul_mod(yi, mod_inv(denom, p), p);
        for d in 0..k {
            coeffs[d] = add_mod(coeffs[d], mul_mod(basis[d], scale, p), p);
        }
    }
    coeffs
}

fn eval_poly(coeffs: &[u64], x: u64, p: u64) -> u64 {
    let mut acc = 0u64;
    for &c in coeffs.iter().rev() {
        acc = add_mod(mul_mod(acc, x, p), c, p);
    }
    acc
}

// ─────────────────────────────────────────────────────────────────────────
// GAO DECODING — a real, polynomial-time, purely classical Reed-Solomon
// unique decoder (Shuhong Gao, 2002), mathematically equivalent in
// correction power to the Berlekamp-Massey syndrome decoder Algorithm 1
// of the DQI paper actually specifies (both correct up to half the
// code's minimum distance). Built to answer directly, in code, the
// question of whether DQI's own decoding step needs anything beyond
// classical polynomial-time computation: it does not. Applied below to
// the paper's own explicitly named special case of OPI, |f_i^{-1}(+1)|=1
// -- a genuine planted polynomial with single-valued positions, some
// corrupted -- since Gao/Berlekamp-Massey decode a SINGLE received value
// per position, not the general size-r SET case run_opi/Prange handle
// above. Conflating the two would be a new unchecked claim, not this one.

fn poly_trim(v: &mut Vec<u64>) {
    while v.len() > 1 && *v.last().unwrap() == 0 {
        v.pop();
    }
}
fn poly_deg(v: &[u64]) -> isize {
    if v.len() == 1 && v[0] == 0 {
        -1
    } else {
        (v.len() as isize) - 1
    }
}
fn poly_add(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut out = alloc::vec![0u64; n];
    for i in 0..n {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        out[i] = add_mod(x, y, p);
    }
    poly_trim(&mut out);
    out
}
fn poly_sub(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut out = alloc::vec![0u64; n];
    for i in 0..n {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        out[i] = sub_mod(x, y, p);
    }
    poly_trim(&mut out);
    out
}
fn poly_mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if poly_deg(a) < 0 || poly_deg(b) < 0 {
        return alloc::vec![0u64];
    }
    let mut out = alloc::vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            out[i + j] = add_mod(out[i + j], mul_mod(ai, bj, p), p);
        }
    }
    poly_trim(&mut out);
    out
}
/// Polynomial long division over GF(p): returns (quotient, remainder).
fn poly_divmod(a: &[u64], b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let db = poly_deg(b);
    assert!(db >= 0, "division by zero polynomial");
    // Index by the ACTUAL degree, not b.len()-1: if b carries any
    // untrimmed trailing zero coefficient, b[b.len()-1] is 0, mod_inv(0,p)
    // is 0 (0^(p-2) mod p = 0, not a real inverse), coeff is always 0, the
    // leading term of rem never gets eliminated, and this loop never
    // terminates. Confirmed by hitting exactly that hang before this fix.
    let lead_inv = mod_inv(b[db as usize], p);
    let mut rem = a.to_vec();
    poly_trim(&mut rem);
    let da_start = poly_deg(&rem);
    let mut quot = alloc::vec![0u64; (da_start - db + 1).max(1) as usize];
    let mut diag_iters: u64 = 0;
    loop {
        diag_iters += 1;
        if diag_iters > 100_000 {
            sprintln!(
                "opi gao DIAG: poly_divmod exceeded 100000 iterations, a={:?} b={:?} db={} rem_len={}",
                &a[..a.len().min(6)], &b[..b.len().min(6)], db, rem.len()
            );
            break;
        }
        let dr = poly_deg(&rem);
        if dr < db {
            break;
        }
        let coeff = mul_mod(rem[dr as usize], lead_inv, p);
        let shift = (dr - db) as usize;
        quot[shift] = coeff;
        for (j, &bj) in b.iter().enumerate() {
            if bj == 0 {
                continue;
            }
            let idx = shift + j;
            rem[idx] = sub_mod(rem[idx], mul_mod(coeff, bj, p), p);
        }
        poly_trim(&mut rem);
        if poly_deg(&rem) < 0 {
            break;
        }
    }
    poly_trim(&mut quot);
    (quot, rem)
}

/// Gao's Reed-Solomon decoding algorithm: given N (point, value) pairs
/// and a message-degree bound k, recovers the unique degree-<k
/// polynomial agreeing with more than (N+k)/2 of the points, if one
/// exists -- i.e. corrects up to floor((N-k)/2) errors, deterministically,
/// in polynomial time, no repeated trials, no randomness anywhere in
/// this function.
pub fn gao_decode(points: &[u64], values: &[u64], k: usize, p: u64) -> Option<Vec<u64>> {
    let pts: Vec<(u64, u64)> = points.iter().zip(values.iter()).map(|(&x, &y)| (x, y)).collect();
    let n = pts.len();
    // g(x) = product (x - x_i)
    let mut g: Vec<u64> = alloc::vec![1u64];
    for &(x, _) in &pts {
        g = poly_mul(&g, &[sub_mod(0, x, p), 1], p);
    }
    let r_poly = lagrange_interpolate(&pts, p);
    let threshold = ((n + k) / 2) as isize;

    let (mut r_prev, mut r_curr) = (g, r_poly);
    let (mut t_prev, mut t_curr): (Vec<u64>, Vec<u64>) = (alloc::vec![0u64], alloc::vec![1u64]);
    while poly_deg(&r_curr) >= threshold {
        let (q, rem) = poly_divmod(&r_prev, &r_curr, p);
        let t_next = poly_sub(&t_prev, &poly_mul(&q, &t_curr, p), p);
        r_prev = r_curr;
        r_curr = rem;
        t_prev = t_curr;
        t_curr = t_next;
        if poly_deg(&r_curr) < 0 {
            break;
        }
    }
    if poly_deg(&t_curr) < 0 {
        return None;
    }
    let (f, rem) = poly_divmod(&r_curr, &t_curr, p);
    if poly_deg(&rem) >= 0 {
        return None; // division not exact -- decoding failure
    }
    if poly_deg(&f) >= k as isize {
        return None;
    }
    let mut out = f;
    out.resize(k, 0);
    Some(out)
}

/// The paper's own explicitly named special case (Remark 5.2 area):
/// |f_i^{-1}(+1)|=1, "noisy polynomial reconstruction" -- a genuine
/// planted degree-<k polynomial evaluated at N points, exactly t of
/// them corrupted to a wrong single value, t = floor((N-k)/2), the
/// Gao/Berlekamp-Massey unique-decoding radius. One deterministic
/// decode, not a search over trials.
pub struct GaoResult {
    pub p: u64,
    pub n_points: usize,
    pub k: usize,
    pub errors_planted: usize,
    pub max_correctable: usize,
    pub decoded_correctly: bool,
    #[cfg(feature = "hosted")]
    pub micros: i64,
}

pub fn run_gao_demo(p: u64, k: usize, seed: u64) -> Result<GaoResult, String> {
    if !is_prime(p) {
        return Err(format!("p={} is not prime", p));
    }
    let n_points = (p - 1) as usize;
    if k == 0 || k >= n_points {
        return Err(format!("k={} must satisfy 0 < k < p-1={}", k, n_points));
    }
    let t = (n_points - k) / 2; // Gao's unique-decoding radius

    let mut rng = Xorshift(seed ^ 0x9E3779B97F4A7C15);
    let message: Vec<u64> = (0..k).map(|_| rng.next_below(p)).collect();
    let points: Vec<u64> = (0..n_points as u64).collect();
    let mut values: Vec<u64> = points.iter().map(|&x| eval_poly(&message, x, p)).collect();

    let mut corrupted: Vec<usize> = Vec::new();
    while corrupted.len() < t {
        let pos = rng.next_below(n_points as u64) as usize;
        if !corrupted.contains(&pos) {
            corrupted.push(pos);
        }
    }
    for &pos in &corrupted {
        loop {
            let bad = rng.next_below(p);
            if bad != values[pos] {
                values[pos] = bad;
                break;
            }
        }
    }

    #[cfg(feature = "hosted")]
    let t0 = now_micros();
    let decoded = gao_decode(&points, &values, k, p);
    #[cfg(feature = "hosted")]
    let micros = now_micros() - t0;

    let decoded_correctly = decoded.as_deref() == Some(message.as_slice());
    Ok(GaoResult {
        p,
        n_points,
        k,
        errors_planted: t,
        max_correctable: t,
        decoded_correctly,
        #[cfg(feature = "hosted")]
        micros,
    })
}

pub fn gao_report(r: &GaoResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "gao: p={}, N=p-1={}, k={} (message degree < {}), errors planted = {} (Gao's unique-decoding radius floor((N-k)/2))\n",
        r.p, r.n_points, r.k, r.k, r.errors_planted
    ));
    out.push_str("  single deterministic decode -- no trials, no randomness in the decoder itself\n");
    out.push_str(&format!(
        "  decoded polynomial exactly matches the planted message: {}\n",
        r.decoded_correctly
    ));
    #[cfg(feature = "hosted")]
    out.push_str(&format!("  runtime: {} us ({:.3} ms)\n", r.micros, r.micros as f64 / 1000.0));
    out
}

#[cfg(feature = "hosted")]
fn now_micros() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

fn sqrt_f64(x: f64) -> f64 {
    libm::sqrt(x)
}

pub struct OpiResult {
    pub p: u64,
    pub n: usize,
    pub m: usize,
    pub r: usize,
    pub gamma: u64,
    pub trials: usize,
    pub best_satisfied: usize,
    pub best_fraction: f64,
    pub phi_pr_theory: f64,
    pub phi_dqi_asymptotic: f64,
    #[cfg(feature = "hosted")]
    pub micros: i64,
}

/// The real run, matched to Definition 2.2 and §11.3: m=p-1 constraint
/// points γ^0..γ^{p-2}, each with an independent random size-⌊p/2⌋
/// satisfying set (balanced, per eq. 15), Prange's algorithm run for
/// `trials` repetitions, best satisfaction fraction reported against
/// both the paper's own finite-size Prange prediction and its
/// asymptotic DQI+BM prediction.
pub fn run_opi(p: u64, n: usize, trials: usize, seed: u64) -> Result<OpiResult, String> {
    if !is_prime(p) {
        return Err(format!("p={} is not prime -- GF(p) arithmetic needs a prime field", p));
    }
    let m = (p - 1) as usize;
    if n == 0 || n >= m {
        return Err(format!("n={} must satisfy 0 < n < p-1={}", n, m));
    }
    let r = (p / 2) as usize;
    let gamma = find_primitive_root(p);

    let mut rng = Xorshift(seed ^ 0x9E3779B97F4A7C15);

    // The m=p-1 constraint points: gamma^0 .. gamma^{p-2}, every
    // nonzero field element exactly once (gamma is primitive).
    let points: Vec<u64> = {
        let mut v = Vec::with_capacity(m);
        let mut cur = 1u64;
        for _ in 0..m {
            v.push(cur);
            cur = mul_mod(cur, gamma, p);
        }
        v
    };

    // Independent random size-r subset of GF(p) for each constraint --
    // the "balanced" instance, size stated by eq. 15, membership drawn
    // fresh per constraint (the paper's own "assuming the f_i are
    // random" analysis assumption, not a weaker stand-in for it).
    let sat_sets: Vec<Vec<bool>> = (0..m)
        .map(|_| {
            let mut membership = alloc::vec![false; p as usize];
            let mut count = 0usize;
            while count < r {
                let v = rng.next_below(p) as usize;
                if !membership[v] {
                    membership[v] = true;
                    count += 1;
                }
            }
            membership
        })
        .collect();

    #[cfg(feature = "hosted")]
    let t0 = now_micros();

    let mut best_satisfied = 0usize;
    for _ in 0..trials {
        // Prange: keep a random n of the m constraints.
        let mut idx: Vec<usize> = (0..m).collect();
        for i in 0..n {
            let j = i + rng.next_below((m - i) as u64) as usize;
            idx.swap(i, j);
        }
        let kept = &idx[..n];
        // For each kept constraint, pick an element of its F_i.
        let subset: Vec<(u64, u64)> = kept
            .iter()
            .map(|&i| {
                let members: Vec<u64> = (0..p).filter(|&v| sat_sets[i][v as usize]).collect();
                let pick = members[rng.next_below(members.len() as u64) as usize];
                (points[i], pick)
            })
            .collect();
        let coeffs = lagrange_interpolate(&subset, p);
        let satisfied = (0..m)
            .filter(|&i| sat_sets[i][eval_poly(&coeffs, points[i], p) as usize])
            .count();
        if satisfied > best_satisfied {
            best_satisfied = satisfied;
        }
    }

    #[cfg(feature = "hosted")]
    let micros = now_micros() - t0;

    let n_f = n as f64;
    let m_f = m as f64;
    let p_f = p as f64;
    let r_f = r as f64;
    let phi_pr_theory = n_f / m_f + (1.0 - n_f / m_f) * (r_f / p_f);
    let half_over_p = n_f / (2.0 * p_f);
    let phi_dqi_asymptotic = 0.5 + sqrt_f64(half_over_p * (1.0 - half_over_p));

    Ok(OpiResult {
        p,
        n,
        m,
        r,
        gamma,
        trials,
        best_satisfied,
        best_fraction: best_satisfied as f64 / m as f64,
        phi_pr_theory,
        phi_dqi_asymptotic,
        #[cfg(feature = "hosted")]
        micros,
    })
}

pub fn report(res: &OpiResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "opi: p={}, n={}, m=p-1={}, r=floor(p/2)={}, gamma={} (n/p={:.4})\n",
        res.p, res.n, res.m, res.r, res.gamma, res.n as f64 / res.p as f64
    ));
    out.push_str("  balanced (Def. 2.2, eq. 15): each of the m constraints has an independent random size-r satisfying set\n");
    out.push_str(&format!(
        "  Prange's algorithm (§11.3), {} trials: best {} / {} satisfied (fraction {:.4})\n",
        res.trials, res.best_satisfied, res.m, res.best_fraction
    ));
    out.push_str(&format!(
        "  Prange closed-form prediction n/m + (1-n/m)(r/p): {:.4}  (single-trial expectation; \"best\" above is the max over {} trials, so it is expected to run above this, per §11.3's own \"logarithmic number of standard deviations onto the tail\" argument)\n",
        res.phi_pr_theory, res.trials
    ));
    out.push_str(&format!(
        "  paper's DQI+BM asymptotic prediction (eq. 16, citation not a run here): {:.4}\n",
        res.phi_dqi_asymptotic
    ));
    #[cfg(feature = "hosted")]
    out.push_str(&format!("  runtime: {} us ({:.3} ms)\n", res.micros, res.micros as f64 / 1000.0));
    out
}

pub fn repl_opi(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("opi — Optimal Polynomial Intersection (Definition 2.2), Prange's algorithm (§11.3) over GF(p)");
        sprintln!("  opi run <p> <n> <trials> [seed]   balanced instance, Prange's algorithm, report satisfied/m and runtime");
        sprintln!("  n is the OPI degree bound (Q has degree <= n-1); m=p-1 constraints are used, not n");
        sprintln!("  opi gao <p> <k> [seed]            the single-valued special case, Gao's decoder (Algorithm 1's classical twin), one deterministic decode");
        return;
    }
    match args[0] {
        "run" => {
            let p: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(101);
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);
            let trials: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(2000);
            let seed: u64 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
            match run_opi(p, n, trials, seed) {
                Ok(r) => sprintln!("{}", report(&r)),
                Err(e) => sprintln!("opi: {}", e),
            }
        }
        "gao" => {
            let p: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(101);
            let k: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
            match run_gao_demo(p, k, seed) {
                Ok(r) => sprintln!("{}", gao_report(&r)),
                Err(e) => sprintln!("opi gao: {}", e),
            }
        }
        other => sprintln!("opi: unknown subcommand '{}' (try 'opi help')", other),
    }
}
