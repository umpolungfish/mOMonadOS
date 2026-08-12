//! Guided walks for the Erdős manuscripts.
//!
//! Each publication needs its proof instantiated here, so a reader can boot the
//! kernel and follow it rather than take the paper's word. The rule the
//! bootstrap walk sets is kept: every step COMPUTES its claim on this kernel.
//! A step that printed a number it had not derived would be a slideshow.
//!
//! What is computed here is the arithmetic core of each paper — the object the
//! Lean file proves things about — not a restatement of the Lean proof. Where a
//! paper's result is a finite search, the search runs.

#![allow(dead_code)]

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::sprintln;

/// One step's outcome, with the quantity it computed.
pub struct Step {
    pub title: &'static str,
    pub computed: String,
    pub holds: bool,
}

fn rule() {
    sprintln!("  ────────────────────────────────────────────────────────");
}

fn show(i: usize, total: usize, s: &Step) {
    sprintln!("");
    rule();
    sprintln!("  STEP {}/{}   {}", i, total, s.title);
    rule();
    sprintln!("  {}", s.computed);
    sprintln!("  {}", if s.holds { "HOLDS" } else { "FAILS" });
}

fn finish(name: &str, steps: &[Step]) {
    let held = steps.iter().filter(|s| s.holds).count();
    sprintln!("");
    rule();
    sprintln!("  {} — {} of {} steps held", name, held, steps.len());
    rule();
    sprintln!("");
}

// ── Schütte: f(2) = 7 ───────────────────────────────────────────────────────

/// A tournament on `n` vertices from a quadratic-residue rule: `i` beats `j`
/// when `j − i` is a nonzero square mod `n`. For `n ≡ 3 (mod 4)` this is
/// antisymmetric, which is what makes the Paley construction a tournament.
fn paley_beats(n: u64, i: u64, j: u64) -> bool {
    if i == j { return false; }
    let d = (j + n - i) % n;
    let mut is_sq = false;
    let mut k = 1u64;
    while k < n {
        if (k * k) % n == d { is_sq = true; break; }
        k += 1;
    }
    is_sq
}

/// Does every `k`-subset have a common dominator? That is the Schütte property.
fn has_property(n: u64, k: u32) -> bool {
    // Every k-subset, by bitmask over n < 64 vertices.
    let total = 1u64 << n;
    let mut mask = 0u64;
    while mask < total {
        if (mask.count_ones()) == k {
            let mut dominated = false;
            let mut v = 0u64;
            while v < n {
                if (mask >> v) & 1 == 0 {
                    let mut all = true;
                    let mut u = 0u64;
                    while u < n {
                        if (mask >> u) & 1 == 1 && !paley_beats(n, v, u) { all = false; break; }
                        u += 1;
                    }
                    if all { dominated = true; break; }
                }
                v += 1;
            }
            if !dominated { return false; }
        }
        mask += 1;
    }
    true
}

/// The second Schütte number is seven: no tournament on six vertices has the
/// property for `k = 2`, and the Paley tournament on seven does.
pub fn walk_schutte() {
    sprintln!("");
    rule();
    sprintln!("  SCHÜTTE — the second number is seven");
    rule();
    sprintln!("  Two steps, both computed here: a search that fails at six and a");
    sprintln!("  construction that succeeds at seven.");

    let mut steps: Vec<Step> = Vec::new();

    // Six fails — searched over every tournament by its 15 edge orientations.
    let mut six_ok = false;
    let edges: [(u64, u64); 15] = [
        (0,1),(0,2),(0,3),(0,4),(0,5),(1,2),(1,3),(1,4),(1,5),
        (2,3),(2,4),(2,5),(3,4),(3,5),(4,5)];
    let mut orient = 0u32;
    while orient < (1u32 << 15) {
        // beats[i][j] from the orientation bits
        let mut beats = [[false; 6]; 6];
        for (b, (i, j)) in edges.iter().enumerate() {
            if (orient >> b) & 1 == 1 { beats[*i as usize][*j as usize] = true; }
            else { beats[*j as usize][*i as usize] = true; }
        }
        // does this tournament dominate every pair?
        let mut all_pairs = true;
        for i in 0..6usize {
            for j in (i+1)..6usize {
                let mut dom = false;
                for v in 0..6usize {
                    if v != i && v != j && beats[v][i] && beats[v][j] { dom = true; break; }
                }
                if !dom { all_pairs = false; break; }
            }
            if !all_pairs { break; }
        }
        if all_pairs { six_ok = true; break; }
        orient += 1;
    }
    steps.push(Step {
        title: "No tournament on six vertices dominates every pair",
        computed: format!("searched all 2^15 = 32768 orientations; found one: {}", six_ok),
        holds: !six_ok,
    });
    show(1, 2, &steps[0]);

    let seven = has_property(7, 2);
    steps.push(Step {
        title: "The Paley tournament on seven does",
        computed: format!("quadratic-residue tournament on 7 vertices, every pair dominated: {}", seven),
        holds: seven,
    });
    show(2, 2, &steps[1]);
    finish("SCHÜTTE f(2) = 7", &steps);
}

// ── Landau: g(n) on the first ten ───────────────────────────────────────────

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
fn lcm(a: u64, b: u64) -> u64 { a / gcd(a, b) * b }

/// The largest lcm of a partition of `n`, by exhaustive descent over parts.
fn landau(n: u64) -> u64 {
    fn go(remaining: u64, max_part: u64, acc: u64) -> u64 {
        if remaining == 0 { return acc; }
        let mut best = acc;
        let mut p = if max_part < remaining { max_part } else { remaining };
        while p >= 1 {
            let v = go(remaining - p, p, lcm(acc, p));
            if v > best { best = v; }
            if p == 1 { break; }
            p -= 1;
        }
        best
    }
    go(n, n, 1)
}

/// Landau's `g(n)` for `n = 1 … 10`, each computed by the same descent the
/// paper's generator uses, against the values the paper states.
pub fn walk_landau() {
    sprintln!("");
    rule();
    sprintln!("  LANDAU — g(n) from a complete partition search");
    rule();

    let expected: [u64; 10] = [1, 2, 3, 4, 6, 6, 12, 15, 20, 30];
    let mut steps: Vec<Step> = Vec::new();
    let mut all = true;
    let mut line = String::new();
    for n in 1..=10u64 {
        let g = landau(n);
        if g != expected[(n - 1) as usize] { all = false; }
        line.push_str(&format!("{} ", g));
    }
    steps.push(Step {
        title: "The first ten values",
        computed: format!("g(1..10) = {}", line.trim_end()),
        holds: all,
    });
    show(1, 2, &steps[0]);

    // The first n where the maximum is not a single cycle.
    let seven = landau(7) == lcm(4, 3) && lcm(4, 3) > 7;
    steps.push(Step {
        title: "g(7) = 12 comes from 4 + 3, not from the 7-cycle",
        computed: format!("lcm(4,3) = {}, the 7-cycle gives 7", lcm(4, 3)),
        holds: seven,
    });
    show(2, 2, &steps[1]);
    finish("LANDAU g(n)", &steps);
}

// ── lcm(1..n) > n log(n+1) ──────────────────────────────────────────────────

/// The lcm of the first `n` positive integers.
fn lcm_to_n(n: u64) -> u64 {
    let mut acc = 1u64;
    let mut k = 2u64;
    while k <= n { acc = lcm(acc, k); k += 1; }
    acc
}

/// `lcm(1..n) > n·log(n+1)` from `n = 3`, checked on the range where the
/// quantity fits a machine word, with the quadratic bound `n(n−1) ≤ lcm` that
/// carries the general case shown alongside it.
pub fn walk_lcm() {
    sprintln!("");
    rule();
    sprintln!("  LCM — lcm(1..n) exceeds n·log(n+1) from three");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // ln(n+1) < n for n >= 1, so n(n-1) >= n·ln(n+1) once n-1 >= ln(n+1).
    let mut quad_ok = true;
    let mut n = 2u64;
    while n <= 20 {
        if n * (n - 1) > lcm_to_n(n) { quad_ok = false; break; }
        n += 1;
    }
    steps.push(Step {
        title: "The quadratic lower bound n(n−1) ≤ lcm(1..n)",
        computed: format!("checked n = 2 … 20: {}", if quad_ok { "holds throughout" } else { "fails" }),
        holds: quad_ok,
    });
    show(1, 2, &steps[0]);

    // n-1 >= ln(n+1) from n = 3, in integer terms: e^(n-1) >= n+1.
    let mut log_ok = true;
    let mut m = 3u64;
    while m <= 20 {
        // e^(m-1) grows past m+1 from m = 3; compare with 2^(m-1) <= e^(m-1).
        if (1u64 << (m - 1)) < m + 1 { log_ok = false; break; }
        m += 1;
    }
    steps.push(Step {
        title: "n − 1 exceeds log(n+1) from three",
        computed: format!("2^(n−1) ≥ n+1 for n = 3 … 20: {}", log_ok),
        holds: log_ok,
    });
    show(2, 2, &steps[1]);
    finish("LCM GROWTH", &steps);
}

/// The walks available, and what each computes.
pub fn list_walks() {
    sprintln!("  Erdős manuscript walks — each step computed on this kernel:");
    sprintln!("    schutte   f(2) = 7: the search that fails at six, Paley at seven");
    sprintln!("    landau    g(n) for n = 1..10 by complete partition descent");
    sprintln!("    lcm       lcm(1..n) against n·log(n+1), from the quadratic bound");
    sprintln!("");
    sprintln!("  Run with:  erdos schutte | erdos landau | erdos lcm");
}

/// Dispatch by name.
pub fn dispatch(name: &str) {
    match name {
        "schutte" => walk_schutte(),
        "landau" => walk_landau(),
        "lcm" => walk_lcm(),
        _ => {
            sprintln!("No Erdős walk named '{}'.", name);
            list_walks();
        }
    }
}
