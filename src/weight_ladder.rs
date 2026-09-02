//! weight_ladder.rs — the general reduction Theorem 4.3 proved for one
//! problem (DQI's OPI, opi.rs), pulled out to its own name so any other
//! problem with the same shape can call it directly.
//!
//! The shape: m independent, identical binary trials (a coin, a
//! constraint, a bit), each contributing to a running weight k, with an
//! expectation to compute that depends on k through three things only:
//! a diagonal term proportional to k*d, and nearest-weight-sector
//! coupling proportional to sqrt((k+1)(m-k)) going up and sqrt(k(m-k+1))
//! going down. That is Lemma 9.2's exact (ell+1)x(ell+1) real symmetric
//! tridiagonal matrix A^(m,ell,d), and it is not specific to OPI: it is
//! what a Hamming-weight-indexed ladder operator looks like whenever the
//! m trials are exchangeable. semicircle_lambda_max below was opi.rs's
//! own private helper (formerly `fn`, not `pub`) computing exactly this;
//! nothing about its math ever depended on OPI's p, n, or r, so it is
//! made public here and opi.rs now calls into it rather than carrying a
//! private duplicate. A second copy is drift -- one definition, two
//! callers.
//!
//! What this file does NOT claim: that every counting problem has this
//! shape. It needs genuine exchangeability, m trials that are actually
//! independent and identically distributed, not merely m things being
//! counted. A problem whose trials are correlated (a branching process
//! where step k+1's outcome depends on step k's value, for instance)
//! does not reduce to this matrix no matter how it is rephrased -- checked
//! directly against this project's own collatz.rs before writing this
//! file, which is exactly such a case and does NOT qualify.

#![allow(dead_code)]
extern crate alloc;
use alloc::vec::Vec;

fn sqrt_f64(x: f64) -> f64 {
    libm::sqrt(x)
}

fn abs_f64(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// Lemma 9.2's (ell+1)x(ell+1) real symmetric tridiagonal matrix
/// A^(m,ell,d): diagonal entries k*d for k=0..=ell, off-diagonals
/// a_k = sqrt(k(m-k+1)) connecting rows k-1 and k. Returns its largest
/// eigenvalue by shifted power iteration: a Gershgorin shift makes
/// A+cI positive semidefinite, so plain power iteration on the shifted
/// matrix converges to A's true top eigenvalue, not whichever extreme
/// has larger magnitude.
///
/// m: the number of independent binary trials. ell: the truncation
/// depth, matrix dimension ell+1 (weight sectors 0..=ell). d: the
/// per-trial diagonal bias.
pub fn lambda_max(m: usize, ell: usize, d_diag: f64) -> f64 {
    let dim = ell + 1;
    let diag: Vec<f64> = (0..dim).map(|k| k as f64 * d_diag).collect();
    let off: Vec<f64> = (1..dim)
        .map(|k| sqrt_f64(k as f64 * (m as f64 - k as f64 + 1.0)))
        .collect();

    let mut shift = 0.0f64;
    for i in 0..dim {
        let mut row = abs_f64(diag[i]);
        if i > 0 {
            row += abs_f64(off[i - 1]);
        }
        if i < dim - 1 {
            row += abs_f64(off[i]);
        }
        if row > shift {
            shift = row;
        }
    }

    let mut v = alloc::vec![1.0f64; dim];
    let mut norm = sqrt_f64(v.iter().map(|x| x * x).sum());
    for x in v.iter_mut() {
        *x /= norm;
    }

    for _ in 0..2000 {
        let mut w = alloc::vec![0.0f64; dim];
        for i in 0..dim {
            let mut wi = (diag[i] + shift) * v[i];
            if i > 0 {
                wi += off[i - 1] * v[i - 1];
            }
            if i < dim - 1 {
                wi += off[i] * v[i + 1];
            }
            w[i] = wi;
        }
        norm = sqrt_f64(w.iter().map(|x| x * x).sum());
        if norm < 1e-300 {
            break;
        }
        for x in w.iter_mut() {
            *x /= norm;
        }
        v = w;
    }

    let mut av = alloc::vec![0.0f64; dim];
    for i in 0..dim {
        let mut wi = diag[i] * v[i];
        if i > 0 {
            wi += off[i - 1] * v[i - 1];
        }
        if i < dim - 1 {
            wi += off[i] * v[i + 1];
        }
        av[i] = wi;
    }
    let num: f64 = (0..dim).map(|i| v[i] * av[i]).sum();
    let den: f64 = (0..dim).map(|i| v[i] * v[i]).sum();
    num / den
}

/// `weight_ladder <m> <ell> <d>`: report lambda_max directly, for any
/// problem with this shape, no OPI-specific normalization applied. A
/// caller with a specific combinatorial meaning for m, ell, d (as
/// opi.rs's run_opi has) applies its own reduction on top of this raw
/// number; this is the shared, problem-agnostic core alone.
pub fn repl_weight_ladder(args: &[&str]) -> alloc::string::String {
    use alloc::format;
    use alloc::string::String;
    if args.is_empty() || args[0] == "help" {
        let mut s = String::new();
        s.push_str("weight_ladder <m> <ell> <d>\n");
        s.push_str("  m    number of independent, identical binary trials\n");
        s.push_str("  ell  truncation depth (matrix dimension ell+1, weight sectors 0..=ell)\n");
        s.push_str("  d    per-trial diagonal bias\n");
        s.push_str("Returns the largest eigenvalue of Lemma 9.2's exact (ell+1)x(ell+1) real\n");
        s.push_str("symmetric tridiagonal matrix A^(m,ell,d): diagonal k*d, off-diagonal\n");
        s.push_str("sqrt(k(m-k+1)). Requires genuine exchangeability across the m trials --\n");
        s.push_str("see weight_ladder.rs's own doc comment for what disqualifies a problem.\n");
        s.push_str("e.g. weight_ladder 10006 2477 0.0\n");
        return s;
    }
    if args.len() < 3 {
        return format!("weight_ladder: need <m> <ell> <d>, got {} argument(s) -- try 'weight_ladder help'", args.len());
    }
    let m: usize = match args[0].parse() {
        Ok(v) => v,
        Err(_) => return format!("weight_ladder: '{}' is not a valid m (non-negative integer)", args[0]),
    };
    let ell: usize = match args[1].parse() {
        Ok(v) => v,
        Err(_) => return format!("weight_ladder: '{}' is not a valid ell (non-negative integer)", args[1]),
    };
    let d: f64 = match args[2].parse() {
        Ok(v) => v,
        Err(_) => return format!("weight_ladder: '{}' is not a valid d (real number)", args[2]),
    };
    if ell >= m {
        return format!("weight_ladder: ell ({}) must be < m ({}) -- the matrix has ell+1 rows, each row k needs m-k+1 >= 0", ell, m);
    }
    let lm = lambda_max(m, ell, d);
    format!(
        "weight_ladder m={} ell={} d={}: lambda_max = {:.10}\n  (A^(m,ell,d), the (ell+1)x(ell+1) real symmetric tridiagonal matrix of Lemma 9.2)",
        m, ell, d, lm
    )
}
