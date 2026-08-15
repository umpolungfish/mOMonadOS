// ringspec.rs — the spectrum of a ring, in integers.
//
// A forged ring's material sheet arrives from the host as decimals: ρ = 3.1623,
// gap = 0.0000. Those have to be trusted. For an integer-weighted ring they do
// not have to be: the adjacency is an integer matrix, so its characteristic
// polynomial has integer coefficients, and every question the sheet answers with
// a decimal is answered here by one of those coefficients.
//
// What the polynomial settles without a float:
//
//   * whether the spectrum is symmetric under λ ↦ -λ — true exactly when the odd
//     coefficients all vanish. A symmetric spectrum pairs the top eigenvalue with
//     its negative, so |λ₂| = ρ and the SPECTRAL GAP IS ZERO.
//
//     And that is exactly bipartiteness, which for a cycle is exactly even
//     length. So the gap is decided by the PARITY of the unit count before any
//     weight is read, and comparing the gap of a 3-ring with the gap of a
//     4-ring measures nothing but 3 against 4. The tool says so on every run,
//     because the first thing I did with the number was over-read it.
//   * Σλ² = tr(A²) = 2·Σwᵢ², so when the spectrum is {0,…,0,±ρ} the radius is
//     pinned as ρ² = Σwᵢ² exactly.
//   * whether the ring is a pure cycle (every weight 1) or carries a cross-link.
//
// Faddeev–LeVerrier gives the coefficients, and for an integer matrix every
// division in it is exact, so i64 carries the whole computation.

#![allow(dead_code)]

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

const MAXN: usize = 12;

/// Cyclic adjacency: bond i joins unit i to unit i+1 with weight w[i], and the
/// last bond closes the ring.
fn ring_adjacency(w: &[i64]) -> Vec<Vec<i64>> {
    let n = w.len();
    let mut a = alloc::vec![alloc::vec![0i64; n]; n];
    for i in 0..n {
        let j = (i + 1) % n;
        a[i][j] += w[i];
        a[j][i] += w[i];
    }
    a
}

fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = alloc::vec![alloc::vec![0i64; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 { continue; }
            for j in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn trace(a: &[Vec<i64>]) -> i64 {
    (0..a.len()).map(|i| a[i][i]).sum()
}

/// Characteristic polynomial coefficients by Faddeev–LeVerrier.
///
/// Returns `[c_0, …, c_n]` for `det(λI − A) = Σ c_k λ^k`, so `c_n = 1`. Every
/// division here is exact on an integer matrix, which is why this stays in i64
/// and the answer is a fact rather than a rounding.
fn char_poly(a: &[Vec<i64>]) -> Vec<i64> {
    let n = a.len();
    let mut coeffs = alloc::vec![0i64; n + 1];
    coeffs[n] = 1;
    let mut m: Vec<Vec<i64>> = alloc::vec![alloc::vec![0i64; n]; n];
    let mut c = 1i64;
    for k in 1..=n {
        // M ← A·M + c·I
        let am = mat_mul(a, &m);
        let mut next = am;
        for i in 0..n { next[i][i] += c; }
        let am2 = mat_mul(a, &next);
        c = -trace(&am2) / (k as i64);
        m = next;
        coeffs[n - k] = c;
    }
    coeffs
}

pub fn ringspec_main(args: &[&str]) -> String {
    let mut out = String::new();
    // The REPL splits a line with splitn(4, ' '), so the last token arrives
    // carrying the rest of the line inside it — "1 2 2 1" reaches here as
    // ["1", "2", "2 1"]. Split again on whitespace rather than widen the REPL's
    // cap, which other commands rely on to keep a trailing phrase intact.
    let weights: Vec<i64> = args
        .iter()
        .flat_map(|s| s.split_whitespace())
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    if weights.len() < 3 {
        out.push_str("ringspec <w1> <w2> <w3> [...]  — the spectrum of a ring, in integers\n\n");
        out.push_str("  Bond weights around the cycle: a clean bond is 1, a cross-link is its\n");
        out.push_str("  number of reaction centres. Three is the minimum — two units cannot\n");
        out.push_str("  cyclize.\n\n");
        out.push_str("  e.g.  ringspec 1 2 2 1     (the Erdős 3 settled ring)\n");
        out.push_str("        ringspec 1 1 1       (a bare triangle)\n");
        return out;
    }
    if weights.len() > MAXN {
        return format!("ringspec: at most {} units\n", MAXN);
    }

    let n = weights.len();
    let a = ring_adjacency(&weights);
    let a2 = mat_mul(&a, &a);
    let sum_sq: i64 = weights.iter().map(|w| w * w).sum();
    let poly = char_poly(&a);

    out.push_str("RINGSPEC — the spectrum of a ring, in integers\n");
    out.push_str("=============================================\n\n");
    out.push_str(&format!("  units:   {}\n", n));
    out.push_str("  weights: ");
    for w in &weights { out.push_str(&format!("{} ", w)); }
    out.push('\n');

    let pure = weights.iter().all(|&w| w == 1);
    out.push_str(&format!(
        "  topology: {}\n",
        if pure { "pure cycle — every junction one clean bond" }
        else { "branched — at least one cross-link lifts the principal mode" }
    ));

    // char poly, high power first, zero terms dropped
    out.push_str("\n  char poly det(λI − A) = ");
    let mut first = true;
    for k in (0..=n).rev() {
        let c = poly[k];
        if c == 0 { continue; }
        if !first { out.push_str(" + "); }
        if k == 0 { out.push_str(&format!("{}", c)); }
        else if c == 1 { out.push_str(&format!("λ^{}", k)); }
        else { out.push_str(&format!("{}λ^{}", c, k)); }
        first = false;
    }
    out.push('\n');

    // Σλ² = tr(A²) = 2Σw², an identity worth showing because it is the one that
    // pins ρ when the rest of the spectrum is zero.
    let tr2 = trace(&a2);
    out.push_str(&format!(
        "\n  Σλ² = tr(A²) = {}   (= 2·Σwᵢ² = 2·{})  {}\n",
        tr2, sum_sq, if tr2 == 2 * sum_sq { "✓" } else { "✗" }
    ));

    // Symmetric spectrum ⟺ every odd coefficient vanishes.
    let symmetric = (0..=n).filter(|k| k % 2 == 1).all(|k| poly[k] == 0);
    out.push_str(&format!(
        "  spectrum symmetric under λ ↦ -λ: {}\n",
        if symmetric { "YES — every odd coefficient vanishes" } else { "no" }
    ));

    if symmetric {
        out.push_str("  ⟹ the top eigenvalue is paired with its negative, so |λ₂| = ρ\n");
        out.push_str("  ⟹ SPECTRAL GAP = 0 exactly — no privileged mode\n");
    } else {
        out.push_str("  ⟹ gap > 0: one mode dominates, the ring leans on a single strut\n");
    }

    // Parity is not a rival explanation for the gap, it is the same fact said
    // shorter. A symmetric spectrum IS bipartiteness, and a cycle is bipartite
    // exactly when it is even — but a ring becomes even by carrying a mediating
    // unit on BOTH sides of what it mediates, and that doubling is what the
    // alternation consists of. Saying "it is only parity" would set the count
    // against the structure as competing causes, when the count is just how the
    // structure shows up in the spectrum.
    out.push_str(&format!(
        "\n  {} ring — {}.\n",
        if n % 2 == 0 { "even" } else { "odd" },
        if n % 2 == 0 { "bipartite, so gap 0 whatever the weights" }
        else { "never bipartite, so gap > 0 whatever the weights" }
    ));
    out.push_str("  Bipartite means the units fall into two classes and every bond runs\n");
    out.push_str("  between them. A ring alternates because something mediates on both\n");
    out.push_str("  sides of what it mediates — which is also why the count is even. One\n");
    out.push_str("  fact, two readings. Compare gap within a parity; across parities the\n");
    out.push_str("  gap is fixed before any weight is read, so compare strain instead.\n");

    // Rank: how many zero eigenvalues, read off the trailing zero coefficients.
    let mut zeros = 0usize;
    while zeros <= n && poly[zeros] == 0 { zeros += 1; }
    if zeros > 0 {
        out.push_str(&format!("  zero eigenvalues: {} (λ^{} divides the polynomial)\n", zeros, zeros));
        if zeros == n - 2 && symmetric {
            out.push_str(&format!(
                "  ⟹ the spectrum is {{0 ×{}, ±ρ}} with ρ² = Σλ²/2 = {} EXACTLY\n",
                zeros, tr2 / 2
            ));
        }
    }

    out.push_str("\n  Every line above is an integer identity. A ring reported as ρ=3.1623\n");
    out.push_str("  has to be believed; one reported as ρ²=10 can be checked.\n");
    out
}
