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
//! DQI itself is implemented, not cited: Lemma 9.2 proves ⟨s⟩ = mr/p +
//! sqrt(r(p-r))/p * w^T A w exactly, for any finite m with 2ℓ+1 < d_perp,
//! where A = A^(m,ell,d) is the real (ell+1)x(ell+1) symmetric tridiagonal
//! matrix in that lemma (diagonal k*d, off-diagonals sqrt(k(m-k+1)), d =
//! (p-2r)/sqrt(r(p-r))) and the optimal w is A's top eigenvector. That
//! eigenvalue problem is solved here directly, by shifted power iteration
//! on the actual matrix at this m and ell -- weight_ladder::lambda_max --
//! giving DQI's real, finite-size expected satisfaction fraction:
//!   phi_DQI_exact = r/p + sqrt(r(p-r))/(mp) * lambda_max(A)
//! The paper's own asymptotic closed form (eq. 16, the m -> p limit of the
//! same lemma) is reported alongside as the control the exact computation
//! should converge toward as p grows:
//!   phi_DQI_asymptotic = 1/2 + sqrt( (n/2p) * (1 - n/2p) )
//!
//! phi_PR's column is the real Prange run, measured. phi_DQI_exact is DQI's
//! own real number for these exact parameters, computed. Prange is not
//! DQI; the two are different algorithms with different performance, and
//! nothing here forces them to agree.

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
///
/// Theta(k^3): each of the k basis polynomials is rebuilt from scratch,
/// one (x - xj) factor at a time. The outer loop runs exactly k times
/// regardless, each iteration costing Theta(k^2) on its own, so checking
/// a deadline once per outer iteration is a cheap, natural checkpoint --
/// it cannot abort mid-iteration, but it catches the run before wasting
/// any of the k^3 total work on iterations that would run past budget.
/// `deadline_micros` is ignored (never aborts) outside hosted builds,
/// which have no wall clock here; returns None if the deadline is hit.
fn lagrange_interpolate(
    points: &[(u64, u64)],
    p: u64,
    #[cfg_attr(not(feature = "hosted"), allow(unused_variables))] deadline_micros: Option<i64>,
) -> Option<Vec<u64>> {
    let k = points.len();
    let mut coeffs = alloc::vec![0u64; k];
    for i in 0..k {
        #[cfg(feature = "hosted")]
        if let Some(deadline) = deadline_micros {
            if now_micros() >= deadline {
                return None;
            }
        }
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
    Some(coeffs)
}

fn eval_poly(coeffs: &[u64], x: u64, p: u64) -> u64 {
    let mut acc = 0u64;
    for &c in coeffs.iter().rev() {
        acc = add_mod(mul_mod(acc, x, p), c, p);
    }
    acc
}

/// Build the OPI instance (constraint points and their satisfying sets) exactly
/// as run_opi does, for the GPU path and its verifier. Returns (m, points,
/// membership) with membership row-major m*p bytes (1 = value satisfies).
pub fn build_instance(p: u64, n: usize, seed: u64) -> Result<(usize, Vec<u64>, Vec<u8>), String> {
    if !is_prime(p) {
        return Err(format!("p={} is not prime -- GF(p) arithmetic needs a prime field", p));
    }
    let m = (p - 1) as usize;
    if n == 0 || n >= m {
        return Err(format!("n={} must satisfy 0 < n < p-1={}", n, m));
    }
    let r = (p / 2) as usize;
    let gamma = find_primitive_root(p);
    let mut points = Vec::with_capacity(m);
    let mut cur = 1u64;
    for _ in 0..m { points.push(cur); cur = mul_mod(cur, gamma, p); }
    let mut rng = Xorshift(seed ^ 0x9E3779B97F4A7C15);
    let mut membership = alloc::vec![0u8; m * p as usize];
    for i in 0..m {
        let mut count = 0usize;
        while count < r {
            let v = rng.next_below(p) as usize;
            if membership[i * p as usize + v] == 0 {
                membership[i * p as usize + v] = 1;
                count += 1;
            }
        }
    }
    Ok((m, points, membership))
}

/// CPU satisfied-constraint count for one fixed subset, the reference the GPU
/// fixed-subset kernel is checked against.
pub fn cpu_count_subset(p: u64, m: usize, _n: usize, points: &[u64], membership: &[u8],
                        kx: &[u64], ky: &[u64]) -> usize {
    let subset: Vec<(u64, u64)> = kx.iter().zip(ky.iter()).map(|(&x, &y)| (x, y)).collect();
    let coeffs = match lagrange_interpolate(&subset, p, None) { Some(c) => c, None => return 0 };
    (0..m).filter(|&i| membership[i * p as usize + eval_poly(&coeffs, points[i], p) as usize] == 1).count()
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

// Lemma 9.2's (ell+1)x(ell+1) tridiagonal eigenvalue problem is not
// specific to OPI -- it's the general shape of any m-exchangeable-trial
// Hamming-weight ladder. Lives in weight_ladder.rs now, one definition
// instead of a private copy here; opi.rs calls crate::weight_ladder::
// lambda_max directly below.

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
    pub ell: usize,
    pub lambda_max: f64,
    pub phi_dqi_exact: f64,
    pub phi_dqi_asymptotic: f64,
    pub trials_completed: usize,
    pub time_capped: bool,
    #[cfg(feature = "hosted")]
    pub micros: i64,
}

/// The real run, matched to Definition 2.2 and §11.3: m=p-1 constraint
/// points γ^0..γ^{p-2}, each with an independent random size-⌊p/2⌋
/// satisfying set (balanced, per eq. 15), Prange's algorithm run for
/// up to `trials` repetitions, best satisfaction fraction reported against
/// both the paper's own finite-size Prange prediction and its
/// asymptotic DQI+BM prediction.
///
/// `time_budget_secs`, when given (hosted builds only -- no_std has no
/// wall clock here), stops the trial loop once elapsed time crosses the
/// budget, even if `trials` has not been reached: `lagrange_interpolate`
/// is Theta(n^3) in the interpolation degree (see the doc comment above),
/// so a single trial at large n can run for tens of minutes, and a
/// fixed trial COUNT gives no way to bound that from the caller.
pub fn run_opi(
    p: u64,
    n: usize,
    trials: usize,
    seed: u64,
    #[cfg_attr(not(feature = "hosted"), allow(unused_variables))] time_budget_secs: Option<f64>,
) -> Result<OpiResult, String> {
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
    #[cfg(feature = "hosted")]
    let budget_micros = time_budget_secs.map(|s| (s * 1_000_000.0) as i64);
    // Absolute deadline handed down into lagrange_interpolate, checked once
    // per its own outer loop iteration -- catches a single oversized trial,
    // not just the gap between trials (see that function's doc comment).
    #[cfg(feature = "hosted")]
    let deadline: Option<i64> = budget_micros.map(|b| t0 + b);
    #[cfg(not(feature = "hosted"))]
    let deadline: Option<i64> = None;

    let mut best_satisfied = 0usize;
    let mut trials_completed = 0usize;
    let mut time_capped = false;

    // GPU: run every Prange trial in parallel, one thread per trial, best by
    // atomic maximum. The CPU loop below is the bare-metal fallback.
    #[cfg(feature = "hosted")]
    {
        let mut membership = alloc::vec![0u8; m * p as usize];
        for i in 0..m {
            for v in 0..p as usize {
                if sat_sets[i][v] { membership[i * p as usize + v] = 1; }
            }
        }
        if let Some(best) = crate::gpu_opi::run_trials(p, m, n, &points, &membership, trials, seed) {
            best_satisfied = best;
            trials_completed = trials;
            let micros = now_micros() - t0;
            let n_f = n as f64; let m_f = m as f64; let p_f = p as f64; let r_f = r as f64;
            let phi_pr_theory = n_f / m_f + (1.0 - n_f / m_f) * (r_f / p_f);
            let half_over_p = n_f / (2.0 * p_f);
            let phi_dqi_asymptotic = 0.5 + sqrt_f64(half_over_p * (1.0 - half_over_p));
            let ell = (n + 1) / 2;
            let d_diag = (p_f - 2.0 * r_f) / sqrt_f64(r_f * (p_f - r_f));
            let lambda_max = crate::weight_ladder::lambda_max(m, ell, d_diag);
            let phi_dqi_exact = r_f / p_f + sqrt_f64(r_f * (p_f - r_f)) / (m_f * p_f) * lambda_max;
            return Ok(OpiResult {
                p, n, m, r, gamma, trials, best_satisfied,
                best_fraction: best_satisfied as f64 / m as f64,
                phi_pr_theory, ell, lambda_max, phi_dqi_exact, phi_dqi_asymptotic,
                trials_completed, time_capped: false, micros,
            });
        }
    }

    for _ in 0..trials {
        #[cfg(feature = "hosted")]
        if let Some(budget) = budget_micros {
            if now_micros() - t0 >= budget {
                time_capped = true;
                break;
            }
        }
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
        let coeffs = match lagrange_interpolate(&subset, p, deadline) {
            Some(c) => c,
            None => {
                time_capped = true;
                break;
            }
        };
        let satisfied = (0..m)
            .filter(|&i| sat_sets[i][eval_poly(&coeffs, points[i], p) as usize])
            .count();
        if satisfied > best_satisfied {
            best_satisfied = satisfied;
        }
        trials_completed += 1;
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

    // The real DQI number: Lemma 9.2's exact finite-m formula, computed by
    // solving the actual (ell+1)x(ell+1) eigenvalue problem at this m, not
    // the m->p asymptotic limit above. ell = floor((n+1)/2), the Berlekamp-
    // Massey correction radius for this Reed-Solomon code (§5).
    let ell = (n + 1) / 2;
    let d_diag = (p_f - 2.0 * r_f) / sqrt_f64(r_f * (p_f - r_f));
    let lambda_max = crate::weight_ladder::lambda_max(m, ell, d_diag);
    let phi_dqi_exact = r_f / p_f + sqrt_f64(r_f * (p_f - r_f)) / (m_f * p_f) * lambda_max;

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
        ell,
        lambda_max,
        phi_dqi_exact,
        phi_dqi_asymptotic,
        trials_completed,
        time_capped,
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
    if res.time_capped {
        if res.trials == usize::MAX {
            out.push_str(&format!(
                "  Prange's algorithm (§11.3), {} trials (time budget hit): best {} / {} satisfied (fraction {:.4})\n",
                res.trials_completed, res.best_satisfied, res.m, res.best_fraction
            ));
        } else {
            out.push_str(&format!(
                "  Prange's algorithm (§11.3), {} of {} requested trials (time budget hit): best {} / {} satisfied (fraction {:.4})\n",
                res.trials_completed, res.trials, res.best_satisfied, res.m, res.best_fraction
            ));
        }
    } else {
        out.push_str(&format!(
            "  Prange's algorithm (§11.3), {} trials: best {} / {} satisfied (fraction {:.4})\n",
            res.trials_completed, res.best_satisfied, res.m, res.best_fraction
        ));
    }
    out.push_str(&format!(
        "  Prange closed-form prediction n/m + (1-n/m)(r/p): {:.4}  (single-trial expectation; \"best\" above is the max over {} trials, so it is expected to run above this, per §11.3's own \"logarithmic number of standard deviations onto the tail\" argument)\n",
        res.phi_pr_theory, res.trials_completed
    ));
    out.push_str(&format!(
        "  DQI exact finite-m computation (Lemma 9.2, {}x{} eigenvalue solve, lambda_max={:.6}): {:.4}\n",
        res.ell + 1, res.ell + 1, res.lambda_max, res.phi_dqi_exact
    ));
    out.push_str(&format!(
        "  DQI asymptotic closed form (eq. 16, m->p limit): {:.4}\n",
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
        sprintln!("  opi run <p> <n> <N>s [seed]       same, but capped by a wall-clock budget of N seconds instead of a trial count");
        sprintln!("  n is the OPI degree bound (Q has degree <= n-1); m=p-1 constraints are used, not n");
        sprintln!("  lagrange_interpolate is Theta(n^3): one trial at large n can run for minutes, so a fixed trial");
        sprintln!("  count alone gives no way to bound wall-clock time -- the <N>s form does.");
        return;
    }
    match args[0] {
        "run" => {
            let p: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(101);
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);
            let trials_arg = args.get(3).copied().unwrap_or("2000");
            let (trials, time_budget_secs): (usize, Option<f64>) = match trials_arg.strip_suffix('s') {
                Some(secs_str) => match secs_str.parse::<f64>() {
                    Ok(secs) if secs > 0.0 => (usize::MAX, Some(secs)),
                    _ => {
                        sprintln!("opi: '{}' is not a valid <N>s time budget", trials_arg);
                        return;
                    }
                },
                None => (trials_arg.parse().unwrap_or(2000), None),
            };
            let seed: u64 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
            match run_opi(p, n, trials, seed, time_budget_secs) {
                Ok(r) => sprintln!("{}", report(&r)),
                Err(e) => sprintln!("opi: {}", e),
            }
        }
        other => sprintln!("opi: unknown subcommand '{}' (try 'opi help')", other),
    }
}
