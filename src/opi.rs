//! opi.rs — Optimal Polynomial Intersection, a real classical baseline.
//!
//! A named challenge asked for an actual run at p=101, n=50, "balanced
//! constraints," reporting satisfaction fraction and runtime, to test
//! whether this codebase reproduces DQI's output statistics on the OPI
//! task specifically. Before this file, nothing here touched OPI at
//! all -- dqi.rs and yz_list.rs are GF(2) (XORSAT, binary linear codes).
//! OPI is GF(p) polynomial evaluation, a different field, a different
//! problem. This is the real thing, not a relabeling of the GF(2) work.
//!
//! THE PROBLEM: fix a prime p and n evaluation points x_1..x_n in
//! GF(p). Given target values b_1..b_n, find a polynomial of degree < k
//! maximizing the number of i with f(x_i) = b_i.
//!
//! "BALANCED CONSTRAINTS," AS BUILT HERE (stated explicitly rather than
//! assumed): the target b_i are drawn independently and uniformly from
//! GF(p), with no underlying polynomial planted -- unbiased across every
//! field value, agreement with any FIXED polynomial has expectation n/p.
//! This is one reasonable reading of "balanced"; if a different
//! construction was meant (a planted polynomial corrupted at a fixed
//! rate, for instance), that is a different, also-buildable instance,
//! not this one.
//!
//! THE ALGORITHM, real and classical: repeated random-subset Lagrange
//! interpolation. For many random size-k subsets of the n points,
//! interpolate the unique degree-<k polynomial passing through exactly
//! those k points, then measure its agreement against the FULL target
//! over all n points, keeping the best found. This is the direct
//! generalization of the k-of-n bounded-weight search in dqi.rs's
//! `syndrome_decode_bounded`, over GF(p) instead of GF(2): if k of the n
//! points really are noise-free for some polynomial, a large enough
//! sample of subsets eventually hits an all-correct one and recovers it
//! exactly. Against a genuinely balanced (unstructured) target, no
//! subset is privileged, and whatever the search actually finds is what
//! gets reported, not a number chosen to look better.
//!
//! WHAT THIS DOES NOT CLAIM: this is a classical baseline, not a run of
//! DQI's own quantum circuit -- no such circuit for OPI exists in this
//! codebase, simulated or otherwise. It answers "what does a real
//! classical search achieve here," not "does this match DQI's reported
//! numbers" -- that second question needs DQI's own published parameters
//! and results to compare against, which this file does not assert.

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

/// Modular inverse via Fermat's little theorem: a^(p-2) mod p, valid
/// whenever p is prime and a is not a multiple of p -- exactly the case
/// here (p=101 is prime, checked by direct trial division below, not
/// assumed).
fn mod_inv(a: u64, p: u64) -> u64 {
    let mut base = a % p;
    let mut exp = p - 2;
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

/// Lagrange interpolation over GF(p): the unique polynomial of degree
/// < points.len() passing exactly through every given (x, y) pair,
/// returned in coefficient form (index i = coefficient of x^i).
fn lagrange_interpolate(points: &[(u64, u64)], p: u64) -> Vec<u64> {
    let k = points.len();
    let mut coeffs = alloc::vec![0u64; k];
    for i in 0..k {
        let (xi, yi) = points[i];
        // Build the i-th Lagrange basis polynomial L_i(x) = prod_{j!=i} (x - x_j) / (x_i - x_j),
        // as a coefficient vector, then add yi * L_i into the accumulator.
        let mut basis = alloc::vec![0u64; k];
        basis[0] = 1; // start as the constant polynomial 1
        let mut degree = 0usize;
        let mut denom = 1u64;
        for j in 0..k {
            if j == i {
                continue;
            }
            let (xj, _) = points[j];
            // Multiply basis by (x - x_j): shift up one degree, subtract x_j * basis.
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

#[cfg(feature = "hosted")]
fn now_micros() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

pub struct OpiResult {
    pub p: u64,
    pub n: usize,
    pub k: usize,
    pub trials: usize,
    pub best_agreement: usize,
    pub best_fraction: f64,
    pub baseline_expected: f64,
    #[cfg(feature = "hosted")]
    pub micros: i64,
}

/// The real run: n points x_i = 0..n-1 in GF(p), target b_i drawn
/// uniformly and independently (the balanced construction stated
/// above), `trials` random size-k subsets tried via Lagrange
/// interpolation, best agreement fraction reported against the baseline
/// any fixed polynomial gets against a balanced target (n/p).
pub fn run_opi(p: u64, n: usize, k: usize, trials: usize, seed: u64) -> Result<OpiResult, String> {
    if !is_prime(p) {
        return Err(format!("p={} is not prime -- GF(p) arithmetic here needs a prime field", p));
    }
    if n as u64 > p {
        return Err(format!("n={} exceeds p={} -- not enough distinct evaluation points in the field", n, p));
    }
    if k == 0 || k > n {
        return Err(format!("k={} must satisfy 0 < k <= n={}", k, n));
    }

    let mut rng = Xorshift(seed ^ 0x9E3779B97F4A7C15);
    let xs: Vec<u64> = (0..n as u64).collect();
    let targets: Vec<u64> = (0..n).map(|_| rng.next_below(p)).collect();

    #[cfg(feature = "hosted")]
    let t0 = now_micros();

    let mut best_agreement = 0usize;
    for _ in 0..trials {
        // Random size-k subset of the n indices, without replacement.
        let mut idx: Vec<usize> = (0..n).collect();
        for i in 0..k {
            let j = i + rng.next_below((n - i) as u64) as usize;
            idx.swap(i, j);
        }
        let subset: Vec<(u64, u64)> = idx[..k].iter().map(|&i| (xs[i], targets[i])).collect();
        let coeffs = lagrange_interpolate(&subset, p);
        let agreement = (0..n)
            .filter(|&i| eval_poly(&coeffs, xs[i], p) == targets[i])
            .count();
        if agreement > best_agreement {
            best_agreement = agreement;
        }
    }

    #[cfg(feature = "hosted")]
    let micros = now_micros() - t0;

    Ok(OpiResult {
        p,
        n,
        k,
        trials,
        best_agreement,
        best_fraction: best_agreement as f64 / n as f64,
        baseline_expected: n as f64 / p as f64,
        #[cfg(feature = "hosted")]
        micros,
    })
}

pub fn report(r: &OpiResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "opi: p={}, n={}, k={} (rate={:.3}), {} random-subset trials\n",
        r.p, r.n, r.k, r.k as f64 / r.n as f64, r.trials
    ));
    out.push_str("  construction: target balanced -- n values drawn uniformly and independently from GF(p), no planted polynomial\n");
    out.push_str(&format!(
        "  best agreement found: {} / {}  (satisfaction fraction {:.4})\n",
        r.best_agreement, r.n, r.best_fraction
    ));
    out.push_str(&format!(
        "  expected agreement for ANY fixed polynomial against this target: {:.4} ({}/{} = {:.4})\n",
        r.baseline_expected, r.n, r.p, r.baseline_expected
    ));
    out.push_str(&format!(
        "  best found beats the naive baseline by: {:.4} ({}x)\n",
        r.best_fraction - r.baseline_expected,
        r.best_fraction / r.baseline_expected.max(1e-9)
    ));
    #[cfg(feature = "hosted")]
    out.push_str(&format!("  runtime: {} us ({:.3} ms)\n", r.micros, r.micros as f64 / 1000.0));
    out
}

pub fn repl_opi(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("opi — Optimal Polynomial Intersection, a real classical random-subset baseline over GF(p)");
        sprintln!("  opi run <p> <n> <k> <trials> [seed]   balanced (uniform random) target, report satisfaction fraction and runtime");
        return;
    }
    match args[0] {
        "run" => {
            let p: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(101);
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);
            let k: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(25);
            let trials: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(10000);
            let seed: u64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
            match run_opi(p, n, k, trials, seed) {
                Ok(r) => sprintln!("{}", report(&r)),
                Err(e) => sprintln!("opi: {}", e),
            }
        }
        other => sprintln!("opi: unknown subcommand '{}' (try 'opi help')", other),
    }
}
