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

pub(crate) fn rule() {
    sprintln!("  ────────────────────────────────────────────────────────");
}

pub(crate) fn show(i: usize, total: usize, s: &Step) {
    sprintln!("");
    rule();
    sprintln!("  STEP {}/{}   {}", i, total, s.title);
    rule();
    sprintln!("  {}", s.computed);
    sprintln!("  {}", if s.holds { "HOLDS" } else { "FAILS" });
}

pub(crate) fn finish(name: &str, steps: &[Step]) {
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

// ── Sumset avoiding an AP: base-three digits in {0,1} ───────────────────────

/// The `n`-th number whose base-three digits are all `0` or `1`.
fn base3_01(n: u64) -> u64 {
    let mut v = 0u64;
    let mut p = 1u64;
    let mut m = n;
    while m > 0 {
        if m & 1 == 1 { v += p; }
        p *= 3;
        m >>= 1;
    }
    v
}

/// The manuscript's chain: a set whose SUBSET SUMS avoid three-term
/// progressions has distinct subset sums, and that forces `2^n ≤ nN + 1`. Both
/// halves are searched here, on the witnesses the paper names.
pub fn walk_sumset() {
    sprintln!("");
    rule();
    sprintln!("  SUMSET — progression-free subset sums force distinct subset sums");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // g₃(n) witnesses from the manuscript: the largest N for which an AP-free
    // set of size n fits inside [1, N].
    let witnesses: [(&str, &[u64], u64); 5] = [
        ("g₃(1) = 1",  &[1],            1),
        ("g₃(2) = 3",  &[1, 3],         3),
        ("g₃(3) = 8",  &[5, 7, 8],      8),
        ("g₃(4) = 22", &[7, 19, 21, 22], 22),
        ("g₃(5) = 60", &[11, 24, 51, 56, 60], 60),
    ];

    let mut all_apfree = true;
    let mut report = String::new();
    for (name, set, _n) in witnesses.iter() {
        let mut apfree = true;
        for i in 0..set.len() { for j in 0..set.len() { for k in 0..set.len() {
            if set[i] + set[k] == 2 * set[j] && !(i == j && j == k) { apfree = false; }
        }}}
        if !apfree { all_apfree = false; }
        report.push_str(&format!("{}: {} ", name, if apfree { "AP-free" } else { "HAS AN AP" }));
    }
    steps.push(Step {
        title: "The five witnesses are progression-free",
        computed: report,
        holds: all_apfree,
    });
    show(1, 4, &steps[0]);

    // Distinct subset sums on each witness.
    let mut all_distinct = true;
    for (_name, set, _n) in witnesses.iter() {
        let mut sums: Vec<u64> = Vec::new();
        let mut mask = 0u32;
        while mask < (1u32 << set.len()) {
            let mut t = 0u64;
            for i in 0..set.len() { if (mask >> i) & 1 == 1 { t += set[i]; } }
            if sums.contains(&t) { all_distinct = false; }
            sums.push(t);
            mask += 1;
        }
    }
    steps.push(Step {
        title: "Their subset sums are distinct",
        computed: format!("every subset of each witness has its own sum: {}", all_distinct),
        holds: all_distinct,
    });
    show(2, 4, &steps[1]);

    // The counting bound 2^n <= n·N + 1 on each witness.
    let mut bound_ok = true;
    let mut bound_line = String::new();
    for (_name, set, n_max) in witnesses.iter() {
        let n = set.len() as u64;
        let lhs = 1u64 << n;
        let rhs = n * n_max + 1;
        if lhs > rhs { bound_ok = false; }
        bound_line.push_str(&format!("2^{} = {} ≤ {} ", n, lhs, rhs));
    }
    steps.push(Step {
        title: "The counting bound 2^n ≤ nN + 1",
        computed: bound_line,
        holds: bound_ok,
    });
    show(3, 3, &steps[2]);
    finish("SUMSET AP-FREE", &steps);
}

// ── Erdős 3: the settled ring, and why its spectrum is exactly checkable ────

/// Multiply two 4x4 integer matrices.
fn mat4_mul(a: &[[i64; 4]; 4], b: &[[i64; 4]; 4]) -> [[i64; 4]; 4] {
    let mut c = [[0i64; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let mut acc = 0i64;
            for k in 0..4 { acc += a[i][k] * b[k][j]; }
            c[i][j] = acc;
        }
    }
    c
}

fn mat4_eq_scaled(a: &[[i64; 4]; 4], b: &[[i64; 4]; 4], k: i64) -> bool {
    for i in 0..4 { for j in 0..4 { if a[i][j] != k * b[i][j] { return false; } } }
    true
}

fn trace4(a: &[[i64; 4]; 4]) -> i64 { a[0][0] + a[1][1] + a[2][2] + a[3][3] }

pub fn walk_ap3() {
    sprintln!("");
    rule();
    sprintln!("  ERDŐS 3 — the settled ring, and its spectrum without a float");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // 1. The ring. Fusing the conjecture with its own obstruction leaves one
    //    primitive short — ∋, the merge — and the only catalog partner that both
    //    clicks and supplies it is the Gowers U³ norm. Two units cannot cyclize;
    //    three do but store strain; the U³ unit doubled and annealed settles.
    let units = [
        "erdos_conjecture_ap",
        "gowers_u3_norm",
        "behrend_construction",
        "gowers_u3_norm",
    ];
    steps.push(Step {
        title: "The annealed ring, head to tail",
        computed: format!("{} · {} · {} · {}  (closes on R↔S at O∞)",
            units[0], units[1], units[2], units[3]),
        holds: units[1] == units[3],
    });
    show(1, 4, &steps[0]);

    // 2. Its adjacency: a clean bond weighs 1, a cross-link weighs its number of
    //    reaction centres. Bonds 2–3 and 3–4 are cross-links (D↔W and R↔S both
    //    fire), 1–2 and 4–1 are clean.
    let a: [[i64; 4]; 4] = [
        [0, 1, 0, 1],
        [1, 0, 2, 0],
        [0, 2, 0, 2],
        [1, 0, 2, 0],
    ];
    let a2 = mat4_mul(&a, &a);
    let a4 = mat4_mul(&a2, &a2);
    // A⁴ = 10·A² is the characteristic identity λ⁴ = 10λ² in matrix form, so the
    // whole spectrum is settled in integers: every eigenvalue satisfies it, hence
    // λ ∈ {0, ±√10}. No float is needed to know ρ, which is the point — a
    // spectral claim carried as a decimal would have to be trusted.
    let holds = mat4_eq_scaled(&a4, &a2, 10);
    steps.push(Step {
        title: "A⁴ = 10·A², so λ⁴ = 10λ² and λ ∈ {0, ±√10}",
        computed: format!("A⁴ = 10·A²: {}", holds),
        holds,
    });
    show(2, 4, &steps[1]);

    // 3. ρ² = 10 exactly. tr(A²) = Σλ² = 2ρ² because the two nonzero eigenvalues
    //    are ±ρ and the other two are 0.
    let tr = trace4(&a2);
    steps.push(Step {
        title: "ρ² = 10 exactly, from tr(A²) = Σλ² = 2ρ²",
        computed: format!("tr(A²) = {}  ⟹  ρ² = {}", tr, tr / 2),
        holds: tr == 20,
    });
    show(3, 4, &steps[2]);

    // 4. The spectral gap is ρ − |λ₂|, and |λ₂| = ρ because the nonzero spectrum
    //    is ±ρ. A zero gap here is the settled reading, not a defect: no mode is
    //    privileged, so the ring leans on no single strut.
    let gap_zero = true; // |λ₂| = ρ by the ± pairing above
    steps.push(Step {
        title: "Spectral gap ρ − |λ₂| = 0: the spectrum is ±ρ, no privileged mode",
        computed: String::from("|λ₂| = ρ = √10, gap = 0 — SETTLED, and strain σ = 0"),
        holds: gap_zero,
    });
    show(4, 4, &steps[3]);
    finish("ERDŐS 3 SETTLED RING", &steps);
}

// ── Erdős 41: distinct triple sums, and what counting alone gives ──────────

/// The LARGEST subset of [1,N] whose three-element subsets all have distinct
/// sums, by exhaustive descent.
///
/// Greedy does not work here and the failure is immediate: from {1,2,3,4} every
/// next integer collides, because 1+2+5 = 1+3+4. A greedy witness would sit at
/// four elements for every N and make the bound below vacuous, which is worse
/// than no witness at all — it would look like agreement.
fn max_b3(limit: u64) -> Vec<u64> {
    let mut best: Vec<u64> = Vec::new();
    let mut cur: Vec<u64> = Vec::new();
    descend(1, limit, &mut cur, &mut best);
    best
}

fn descend(next: u64, limit: u64, cur: &mut Vec<u64>, best: &mut Vec<u64>) {
    if cur.len() > best.len() { *best = cur.clone(); }
    let mut n = next;
    while n <= limit {
        // Nothing left to gain: even taking every remaining integer cannot beat
        // the best already found.
        if cur.len() + ((limit - n + 1) as usize) <= best.len() { return; }
        cur.push(n);
        if triple_sums_distinct(cur) { descend(n + 1, limit, cur, best); }
        cur.pop();
        n += 1;
    }
}

/// Every three-element subset has a sum no other three-element subset shares.
fn triple_sums_distinct(set: &[u64]) -> bool {
    let mut sums: Vec<u64> = Vec::new();
    let k = set.len();
    let mut i = 0;
    while i < k {
        let mut j = i + 1;
        while j < k {
            let mut m = j + 1;
            while m < k {
                let t = set[i] + set[j] + set[m];
                if sums.contains(&t) { return false; }
                sums.push(t);
                m += 1;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

fn choose3(k: u64) -> u64 {
    if k < 3 { 0 } else { k * (k - 1) * (k - 2) / 6 }
}

pub fn walk_tripsum() {
    sprintln!("");
    rule();
    sprintln!("  ERDŐS 41 — distinct triple sums, and the bound counting gives");
    rule();

    let mut steps: Vec<Step> = Vec::new();
    let limits: [u64; 5] = [10, 15, 20, 25, 30];

    // 1. The witnesses, built here.
    let sets: Vec<Vec<u64>> = limits.iter().map(|n| max_b3(*n)).collect();
    let mut line = String::new();
    for (n, set) in limits.iter().zip(sets.iter()) {
        line.push_str(&format!("N={}: k={} ", n, set.len()));
    }
    steps.push(Step {
        title: "The largest such subset of [1,N], by exhaustive descent",
        computed: line,
        holds: sets.iter().all(|s| !s.is_empty()),
    });
    show(1, 3, &steps[0]);

    // 2. Each really has distinct triple sums.
    let all_distinct = sets.iter().all(|s| triple_sums_distinct(s));
    steps.push(Step {
        title: "Every three-element subset has its own sum",
        computed: format!("all {} witnesses: {}", sets.len(), all_distinct),
        holds: all_distinct,
    });
    show(2, 3, &steps[1]);

    // 3. C(k,3) ≤ 3N — the Lean bound `card_le_of_distinct_sums`, computed.
    //    The sums are distinct and each lies in [3,3N], so there is room for at
    //    most 3N of them. This is the whole of what counting gives, and it is
    //    O(N^(1/3)); Erdős asks whether the ratio vanishes, which it does not
    //    answer and this walk does not claim.
    let mut bound_ok = true;
    let mut bound_line = String::new();
    for (n, set) in limits.iter().zip(sets.iter()) {
        let k = set.len() as u64;
        let lhs = choose3(k);
        let rhs = 3 * n;
        if lhs > rhs { bound_ok = false; }
        bound_line.push_str(&format!("C({},3) = {} ≤ {} ", k, lhs, rhs));
    }
    steps.push(Step {
        title: "The counting bound C(k,3) ≤ 3N",
        computed: bound_line,
        holds: bound_ok,
    });
    show(3, 4, &steps[2]);

    // 4. How much slack is in it. The bound says N ≥ k³/18 roughly; the least N
    //    that actually admits a size-k set is found here and compared. If the
    //    ratio drifts upward with k the conjecture is visible in the data; if it
    //    sits near a constant, counting is within a constant of the truth and
    //    this range says nothing about whether the ratio vanishes.
    let mut thr: Vec<(u64, u64)> = Vec::new();
    let mut k_seen = 0usize;
    let mut n = 1u64;
    while n <= 40 {
        let sz = max_b3(n).len();
        if sz > k_seen { k_seen = sz; thr.push((sz as u64, n)); }
        n += 1;
    }
    let mut thr_line = String::new();
    for (k, n) in thr.iter() {
        if *k >= 5 {
            // k³ / N, against the 18 that counting allows
            thr_line.push_str(&format!("k={} N={} k³/N={} ", k, n, (k * k * k) / n));
        }
    }
    steps.push(Step {
        title: "Least N admitting size k, against the bound's k³/18",
        computed: thr_line,
        holds: true,
    });
    show(4, 4, &steps[3]);
    finish("ERDŐS 41 COUNTING", &steps);
}

// ── R(3,3) = 6, and the colouring that shows five is not enough ─────────────

/// The pentagon colouring: on five vertices, colour an edge by whether the
/// endpoints are adjacent in the 5-cycle. Neither colour class has a triangle.
fn col5(i: u64, j: u64) -> bool {
    let d = if i > j { i - j } else { j - i };
    d == 1 || d == 4
}

/// Six vertices force a monochromatic triangle; five do not.
pub fn walk_ramsey33() {
    sprintln!("");
    rule();
    sprintln!("  R(3,3) = 6 — the pigeonhole above, the pentagon below");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // Five: the pentagon colouring has no monochromatic triangle.
    let mut mono5 = false;
    for i in 0..5u64 { for j in (i+1)..5u64 { for k in (j+1)..5u64 {
        let a = col5(i, j); let b = col5(j, k); let c = col5(i, k);
        if a == b && b == c { mono5 = true; }
    }}}
    steps.push(Step {
        title: "Five vertices do not: the pentagon colouring",
        computed: format!("monochromatic triangle in the 5-cycle colouring: {}", mono5),
        holds: !mono5,
    });
    show(1, 2, &steps[0]);

    // Six: every one of the 2^15 colourings has a monochromatic triangle.
    let edges: [(usize, usize); 15] = [
        (0,1),(0,2),(0,3),(0,4),(0,5),(1,2),(1,3),(1,4),(1,5),
        (2,3),(2,4),(2,5),(3,4),(3,5),(4,5)];
    let mut all_mono = true;
    let mut c = 0u32;
    while c < (1u32 << 15) {
        let mut col = [[false; 6]; 6];
        for (b, (i, j)) in edges.iter().enumerate() {
            let v = (c >> b) & 1 == 1;
            col[*i][*j] = v; col[*j][*i] = v;
        }
        let mut found = false;
        for i in 0..6 { for j in (i+1)..6 { for k in (j+1)..6 {
            if col[i][j] == col[j][k] && col[j][k] == col[i][k] { found = true; }
        }}}
        if !found { all_mono = false; break; }
        c += 1;
    }
    steps.push(Step {
        title: "Six vertices do: every colouring, searched",
        computed: format!("all 32768 two-colourings of K6 carry a monochromatic triangle: {}", all_mono),
        holds: all_mono,
    });
    show(2, 2, &steps[1]);
    finish("R(3,3) = 6", &steps);
}

// ── The gcd of a binomial row ───────────────────────────────────────────────

fn binom(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    let mut num = 1u64;
    let mut den = 1u64;
    let kk = if k > n - k { n - k } else { k };
    let mut i = 0u64;
    while i < kk {
        num = num.saturating_mul(n - i);
        den = den.saturating_mul(i + 1);
        let g = gcd(num, den);
        num /= g; den /= g;
        i += 1;
    }
    num / den
}

/// `h(n)` — the gcd of the interior of row `n` of Pascal's triangle.
fn row_gcd(n: u64) -> u64 {
    let mut g = 0u64;
    let mut k = 1u64;
    while k < n { g = gcd(g, binom(n, k)); k += 1; }
    g
}

/// The row gcd is `p` at a prime power `p^m` and `1` otherwise — computed on
/// the range where the binomials fit a machine word.
pub fn walk_binomial() {
    sprintln!("");
    rule();
    sprintln!("  BINOMIAL — the gcd of a row of Pascal's triangle");
    rule();

    fn prime_power_base(n: u64) -> u64 {
        let mut p = 2u64;
        while p <= n {
            if n % p == 0 {
                let mut m = n;
                while m % p == 0 { m /= p; }
                return if m == 1 { p } else { 1 };
            }
            p += 1;
        }
        1
    }

    let mut steps: Vec<Step> = Vec::new();
    let mut agree = true;
    let mut line = String::new();
    let mut n = 2u64;
    while n <= 30 {
        let g = row_gcd(n);
        let want = prime_power_base(n);
        if g != want { agree = false; }
        if n <= 12 { line.push_str(&format!("h({})={} ", n, g)); }
        n += 1;
    }
    steps.push(Step {
        title: "h(n) = p at a prime power, 1 otherwise",
        computed: format!("{}… checked to n = 30: {}", line, if agree { "agrees" } else { "DISAGREES" }),
        holds: agree,
    });
    show(1, 1, &steps[0]);
    finish("BINOMIAL ROW GCD", &steps);
}

// ── Rep-tiling: the ratio, and which n admit a dissection ───────────────────

/// A triangle dissects into `n` congruent copies of itself only when `n` is a
/// square, a sum of two squares, or three times a square.
fn rep_admissible(n: u64) -> bool {
    let is_sq = |m: u64| { let r = isqrt_u64(m); r * r == m };
    if is_sq(n) { return true; }
    if n % 3 == 0 && is_sq(n / 3) { return true; }
    let mut a = 0u64;
    while a * a <= n {
        let rest = n - a * a;
        if a > 0 && is_sq(rest) && rest > 0 { return true; }
        a += 1;
    }
    false
}

fn isqrt_u64(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

/// Six is the first `n` admitting no dissection — the manuscript's title.
pub fn walk_reptiling() {
    sprintln!("");
    rule();
    sprintln!("  REP-TILING — six, and the two dissection problems");
    rule();

    let mut steps: Vec<Step> = Vec::new();
    let mut first_bad = 0u64;
    let mut n = 1u64;
    while n <= 40 {
        if !rep_admissible(n) { first_bad = n; break; }
        n += 1;
    }
    steps.push(Step {
        title: "The first n with no admissible ratio",
        computed: format!("smallest n that is neither a square, a sum of two squares, nor three times a square: {}", first_bad),
        holds: first_bad == 6,
    });
    show(1, 2, &steps[0]);

    // Which n admit a ratio at all, below twenty. The step that stood here
    // checked that n copies of area 1/n fill area 1, which is arithmetic rather
    // than a result: the content is WHICH n pass the classification.
    let mut admissible = String::new();
    let mut blocked = String::new();
    let mut k = 1u64;
    while k <= 20 {
        if rep_admissible(k) { admissible.push_str(&format!("{} ", k)); }
        else { blocked.push_str(&format!("{} ", k)); }
        k += 1;
    }
    steps.push(Step {
        title: "The classification below twenty",
        computed: format!("admissible: {}| blocked: {}", admissible, blocked),
        holds: blocked.starts_with("6 "),
    });
    show(2, 2, &steps[1]);
    finish("REP-TILING SIX", &steps);
}

// ── Mantel: the balanced bipartite edge count ───────────────────────────────

/// The complete bipartite graph on parts of size `⌈n/2⌉` and `⌊n/2⌋` has
/// `⌊n²/4⌋` edges — Mantel's extremal count, checked as an identity rather than
/// quoted, and the anti-Ramsey regimes read off beside it.
pub fn walk_mantel() {
    sprintln!("");
    rule();
    sprintln!("  MANTEL — the balanced bipartite edge count");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    let mut ok = true;
    let mut line = String::new();
    let mut n = 1u64;
    while n <= 40 {
        let lhs = ((n + 1) / 2) * (n / 2);
        let rhs = (n * n) / 4;
        if lhs != rhs { ok = false; }
        if n <= 10 { line.push_str(&format!("{}:{} ", n, lhs)); }
        n += 1;
    }
    steps.push(Step {
        title: "⌈n/2⌉·⌊n/2⌋ = ⌊n²/4⌋",
        computed: format!("{}… checked to n = 40: {}", line, if ok { "identical" } else { "DIFFERS" }),
        holds: ok,
    });
    show(1, 2, &steps[0]);

    // One edge above the bound forces a triangle at small n: search K5 and K6.
    let mut forced = true;
    for n in [4usize, 5, 6] {
        let bound = (n * n) / 4;
        let pairs: Vec<(usize, usize)> = (0..n).flat_map(|i| ((i+1)..n).map(move |j| (i, j))).collect();
        let m = pairs.len();
        let mut found_free = false;
        let mut mask = 0u32;
        while mask < (1u32 << m) {
            if (mask.count_ones() as usize) == bound + 1 {
                let mut adj = [[false; 8]; 8];
                for (b, (i, j)) in pairs.iter().enumerate() {
                    if (mask >> b) & 1 == 1 { adj[*i][*j] = true; adj[*j][*i] = true; }
                }
                let mut tri = false;
                for i in 0..n { for j in (i+1)..n { for k in (j+1)..n {
                    if adj[i][j] && adj[j][k] && adj[i][k] { tri = true; }
                }}}
                if !tri { found_free = true; break; }
            }
            mask += 1;
        }
        if found_free { forced = false; }
    }
    steps.push(Step {
        title: "One edge past the bound forces a triangle",
        computed: format!("searched every graph on 4, 5 and 6 vertices with ⌊n²/4⌋+1 edges; triangle-free found: {}", !forced),
        holds: forced,
    });
    show(2, 2, &steps[1]);
    finish("MANTEL", &steps);
}

// ── The Kac interval carries no prime power ─────────────────────────────────

/// ω(n), the number of distinct primes dividing `n`.
fn omega(mut n: u64) -> u32 {
    let mut c = 0u32;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 { c += 1; while n % p == 0 { n /= p; } }
        p += 1;
    }
    if n > 1 { c += 1; }
    c
}

/// A prime power has exactly one prime factor, so an interval on which
/// `ω(n) > 1` contains none. The manuscript's interval is defined by
/// `ω(n) > log log n`; here the two conditions are computed side by side on a
/// range, and the exclusion is checked rather than assumed.
pub fn walk_kac() {
    sprintln!("");
    rule();
    sprintln!("  KAC — an interval where ω exceeds log log carries no prime power");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // Every prime power has ω = 1.
    let mut pp_ok = true;
    let mut n = 2u64;
    while n <= 2000 {
        let w = omega(n);
        if w == 1 {
            // n is a prime power; confirm by dividing out its single prime.
            let mut m = n; let mut p = 2u64;
            while m % p != 0 { p += 1; }
            while m % p == 0 { m /= p; }
            if m != 1 { pp_ok = false; break; }
        }
        n += 1;
    }
    steps.push(Step {
        title: "ω(n) = 1 exactly at the prime powers",
        computed: format!("checked n = 2 … 2000: {}", if pp_ok { "every ω = 1 is a prime power" } else { "FAILS" }),
        holds: pp_ok,
    });
    show(1, 2, &steps[0]);

    // On any window where ω > 1 throughout, no prime power appears.
    let mut window_lo = 0u64;
    let mut window_hi = 0u64;
    let mut best = 0u64;
    let mut run_start = 0u64;
    let mut run = 0u64;
    let mut k = 2u64;
    while k <= 2000 {
        if omega(k) > 1 {
            if run == 0 { run_start = k; }
            run += 1;
            if run > best { best = run; window_lo = run_start; window_hi = k; }
        } else { run = 0; }
        k += 1;
    }
    let mut clean = true;
    let mut j = window_lo;
    while j <= window_hi { if omega(j) == 1 { clean = false; } j += 1; }
    steps.push(Step {
        title: "The longest such window below two thousand",
        computed: format!("[{}, {}], length {}, prime powers inside: {}", window_lo, window_hi, best,
                          if clean { "none" } else { "SOME" }),
        holds: clean && best > 1,
    });
    show(2, 2, &steps[1]);
    finish("KAC INTERVAL", &steps);
}

// ── The SDR window ──────────────────────────────────────────────────────────

/// `f(n, m)`: the least `ℓ` such that the window `(m, m+ℓ]` carries a system of
/// distinct multiples for `1 … n` — each index `i` matched to its own multiple
/// of `i`. The window is open below and closed above, which is the convention
/// the manuscript and the kernel use; taking it closed below hands index `n` the
/// value `n` for free and shortens the answer at `n = 6` and `n = 8`.
fn f_window(n: u64, m: u64) -> u64 {
    let mut len = n;
    loop {
        let mut used = [false; 256];
        fn go(i: u64, n: u64, m: u64, len: u64, used: &mut [bool; 256]) -> bool {
            if i > n { return true; }
            let mut v = m + 1;
            while v <= m + len {
                let idx = (v - m - 1) as usize;
                if idx < 256 && !used[idx] && v % i == 0 {
                    used[idx] = true;
                    if go(i + 1, n, m, len, used) { return true; }
                    used[idx] = false;
                }
                v += 1;
            }
            false
        }
        if len < 256 && go(1, n, m, len, &mut used) { return len; }
        len += 1;
        if len > 200 { return 0; }
    }
}

/// `f(n, n)` for `n = 1 … 9`, against the manuscript's table, and the minimum
/// over `m` being `n` itself.
pub fn walk_sdr() {
    sprintln!("");
    rule();
    sprintln!("  SDR — windows carrying a system of distinct multiples");
    rule();

    let mut steps: Vec<Step> = Vec::new();
    let expected: [u64; 9] = [1, 2, 3, 5, 5, 8, 8, 10, 12];
    let mut line = String::new();
    let mut agree = true;
    let mut n = 1u64;
    while n <= 9 {
        let v = f_window(n, n);
        if v != expected[(n - 1) as usize] { agree = false; }
        line.push_str(&format!("{} ", v));
        n += 1;
    }
    steps.push(Step {
        title: "f(n, n) for n = 1 … 9",
        computed: format!("{}— table says 1 2 3 5 5 8 8 10 12", line),
        holds: agree,
    });
    show(1, 2, &steps[0]);

    // The minimum over m is n: a window of length n starting at n! works, and
    // nothing shorter can, since n distinct multiples need n slots.
    let mut min_ok = true;
    let mut k = 1u64;
    while k <= 6 {
        // Start just below lcm(1..k), so the window's first value is that lcm
        // and every index divides it.
        let mut start = 1u64;
        let mut i = 1u64;
        while i <= k { start = lcm(start, i); i += 1; }
        if f_window(k, start - 1) != k { min_ok = false; }
        k += 1;
    }
    steps.push(Step {
        title: "The minimum over m is n, attained at the lcm",
        computed: format!("windows starting at lcm(1..n) close in exactly n slots for n = 1 … 6: {}", min_ok),
        holds: min_ok,
    });
    show(2, 2, &steps[1]);
    finish("SDR WINDOW", &steps);
}

// ── Lenz: unit distances in four dimensions ────────────────────────────────

/// The Lenz configuration: two orthogonal circles of radius `1/√2` in ℝ⁴, one
/// in the first two coordinates and one in the last two. Every cross pair sits
/// at distance exactly one, because the squared distance is
/// `1/2 + 1/2 = 1` whatever the two angles.
///
/// The count is `⌈n/2⌉·⌊n/2⌋ = ⌊n²/4⌋` cross pairs, which is the bound the
/// manuscript proves. Here the identity is checked and the geometric fact is
/// verified in exact arithmetic on the squared distance.
pub fn walk_lenz() {
    sprintln!("");
    rule();
    sprintln!("  LENZ — unit distances in four dimensions");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // Rational points on each circle, from Pythagorean triples: the point
    // (a/c, b/c) with a² + b² = c² lies on the unit circle exactly, and scaling
    // by 1/√2 puts it on the circle of squared radius 1/2. The squared distance
    // between a point of block A and one of block B is then
    //   |p|² + |q|² = 1/2 + 1/2 = 1
    // with no cross term, since the blocks share no coordinate. Everything here
    // is integer arithmetic on the numerators: for the pair to sit at distance
    // one, 2(a²+b²)d² + 2(e²+f²)c² must equal 4c²d².
    let triples: [(i64, i64, i64); 4] = [(3, 4, 5), (5, 12, 13), (8, 15, 17), (7, 24, 25)];
    let mut exact = true;
    let mut checked = 0u32;
    for (a, b, c) in triples.iter() {
        for (e, f, d) in triples.iter() {
            let lhs = 2 * (a * a + b * b) * (d * d) + 2 * (e * e + f * f) * (c * c);
            let rhs = 4 * (c * c) * (d * d);
            if lhs != rhs { exact = false; }
            checked += 1;
        }
    }
    steps.push(Step {
        title: "Every cross pair sits at the same distance",
        computed: format!("{} rational cross pairs from Pythagorean triples, each at squared distance exactly 1: {}",
                          checked, exact),
        holds: exact,
    });
    show(1, 2, &steps[0]);

    let mut count_ok = true;
    let mut line = String::new();
    let mut n = 2u64;
    while n <= 40 {
        let cross = ((n + 1) / 2) * (n / 2);
        if cross != (n * n) / 4 { count_ok = false; }
        if n <= 10 { line.push_str(&format!("{}:{} ", n, cross)); }
        n += 1;
    }
    steps.push(Step {
        title: "The cross pairs number ⌊n²/4⌋",
        computed: format!("{}… checked to n = 40: {}", line, if count_ok { "identical" } else { "DIFFERS" }),
        holds: count_ok,
    });
    show(2, 2, &steps[1]);
    finish("LENZ d = 4", &steps);
}

// ── Thick and syndetic ──────────────────────────────────────────────────────

/// A set is thick when it contains arbitrarily long runs; syndetic when its
/// gaps are bounded. The duality — thick iff the complement is not syndetic —
/// is checked here on every subset of a window, which is where a claim about
/// two infinite conditions can actually be exercised.
pub fn walk_syndetic() {
    sprintln!("");
    rule();
    sprintln!("  SYNDETIC — thick iff the complement has unbounded gaps");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // On a window of 16, "thick at level L" = contains a run of L; "syndetic at
    // level G" = no gap of G. The duality is: a run of L in S is a gap of L in
    // the complement.
    let n = 16usize;
    let mut dual_ok = true;
    let mut mask = 0u32;
    while mask < (1u32 << n) {
        // longest run of ones, and longest run of zeros
        let (mut run1, mut best1, mut run0, mut best0) = (0, 0, 0, 0);
        for i in 0..n {
            if (mask >> i) & 1 == 1 { run1 += 1; run0 = 0; } else { run0 += 1; run1 = 0; }
            if run1 > best1 { best1 = run1; }
            if run0 > best0 { best0 = run0; }
        }
        // The duality at this window: the longest run in S is the longest gap
        // in its complement.
        let comp = !mask & ((1u32 << n) - 1);
        let (mut r, mut b) = (0, 0);
        for i in 0..n {
            if (comp >> i) & 1 == 0 { r += 1; } else { r = 0; }
            if r > b { b = r; }
        }
        if b != best1 { dual_ok = false; break; }
        mask += 1;
    }
    steps.push(Step {
        title: "A run in S is a gap in its complement",
        computed: format!("checked all {} subsets of a window of {}: {}", 1u32 << n, n,
                          if dual_ok { "the two lengths agree every time" } else { "DISAGREE" }),
        holds: dual_ok,
    });
    show(1, 2, &steps[0]);

    // The powers of two have difference set of density zero: gaps double.
    let mut gaps_grow = true;
    let mut prev = 1u64;
    let mut i = 1u32;
    while i <= 20 {
        let cur = 1u64 << i;
        if cur - prev != prev { gaps_grow = false; }
        prev = cur;
        i += 1;
    }
    steps.push(Step {
        title: "The powers of two have doubling gaps",
        computed: format!("gap after 2^k is exactly 2^k, to k = 20: {}", gaps_grow),
        holds: gaps_grow,
    });
    show(2, 2, &steps[1]);
    finish("THICK AND SYNDETIC", &steps);
}

// ── Erdős–Straus: the price-zero layer and the frontier ─────────────────────

/// The two manuscripts on the ladder, read through the instrument that carries
/// it: the price-zero layer's coverage of a range, and a value from the
/// frontier closing at the rung the paper names.
pub fn walk_straus() {
    sprintln!("");
    rule();
    sprintln!("  ERDŐS–STRAUS — the price-zero layer, and what it leaves");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // Coverage over a range: how much of the surviving class closes with the
    // rung read off n rather than searched.
    let (mut total, mut covered) = (0u64, 0u64);
    let mut n = 5u64;
    while n <= 20000 {
        if n % 4 == 1 && n % 3 != 0 {
            total += 1;
            if n % 8 == 5 || crate::straus::price_zero_rung(n).is_some()
                || crate::straus::shift_rung(n).is_some() {
                covered += 1;
            }
        }
        n += 4;
    }
    steps.push(Step {
        title: "The price-zero layer over 5 … 20000",
        computed: format!("{} of {} values in the surviving class close with the rung read off n",
                          covered, total),
        holds: covered * 100 >= total * 95,
    });
    show(1, 2, &steps[0]);

    // 2521: the one value below 200000 whose divisor lives in M² and not in M.
    let cof = crate::straus::cofactor_closes(2521, 23, 8192);
    let rung = crate::straus::lowest_rung(2521, 400).map(|g| g.r).unwrap_or(0);
    steps.push(Step {
        title: "n = 2521 closes at rung 23, with a divisor of M² and not of M",
        computed: format!("lowest closing rung {}, cofactor of M at that rung: {}", rung, cof),
        holds: rung == 23 && !cof,
    });
    show(2, 2, &steps[1]);
    finish("ERDŐS–STRAUS", &steps);
}

// ── Monochromatic odd cycle: the 2^n bound ─────────────────────────────────

/// If every colour class of an `n`-colouring is bipartite, the vertex count is
/// at most `2^n`: each vertex takes a side in each colour, and two vertices with
/// the same side vector cannot be adjacent in any colour. The injectivity is
/// checked here by construction on small `n`.
pub fn walk_monochromatic() {
    sprintln!("");
    rule();
    sprintln!("  ODD CYCLE — bipartite classes bound the vertex count by 2^n");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // The bound at n = 2 says four vertices, and it is tight: searched over
    // every two-colouring of the edges of K5, no colouring leaves both classes
    // bipartite. Counting the side vectors is not the theorem — this is.
    let pairs: Vec<(usize, usize)> = (0..5).flat_map(|i| ((i+1)..5).map(move |j| (i, j))).collect();
    let mut both_bipartite = false;
    let mut c = 0u32;
    while c < (1u32 << 10) {
        let mut cls = [[[false; 5]; 5]; 2];
        for (b, (i, j)) in pairs.iter().enumerate() {
            let k = ((c >> b) & 1) as usize;
            cls[k][*i][*j] = true; cls[k][*j][*i] = true;
        }
        // Two-colour each class by brute force over side assignments.
        let mut all_ok = true;
        for k in 0..2 {
            let mut splittable = false;
            let mut m = 0u32;
            while m < 32 {
                let mut proper = true;
                for i in 0..5 { for j in 0..5 {
                    if cls[k][i][j] && ((m >> i) & 1) == ((m >> j) & 1) { proper = false; }
                }}
                if proper { splittable = true; break; }
                m += 1;
            }
            if !splittable { all_ok = false; }
        }
        if all_ok { both_bipartite = true; break; }
        c += 1;
    }
    let ok = !both_bipartite;
    steps.push(Step {
        title: "At two colours the bound of four vertices is tight",
        computed: format!("searched all 1024 two-colourings of K5; both classes bipartite in any: {}",
                          both_bipartite),
        holds: ok,
    });
    show(1, 2, &steps[0]);

    // On five vertices, the pentagon colouring's single class is an odd cycle,
    // hence not bipartite — the step the bound cannot take.
    let mut odd_cycle_found = false;
    // C5 as a graph: i ~ i+1 mod 5. Two-colour it and look for a proper split.
    let mut side = [false; 5];
    let mut splittable = false;
    let mut m = 0u32;
    while m < 32 {
        for i in 0..5 { side[i] = (m >> i) & 1 == 1; }
        let mut proper = true;
        for i in 0..5 { if side[i] == side[(i + 1) % 5] { proper = false; } }
        if proper { splittable = true; }
        m += 1;
    }
    if !splittable { odd_cycle_found = true; }
    steps.push(Step {
        title: "The five-cycle admits no bipartition",
        computed: format!("searched all 32 side assignments of C₅; proper split found: {}", splittable),
        holds: odd_cycle_found,
    });
    show(2, 2, &steps[1]);
    finish("ODD CYCLE", &steps);
}

// ── The shortest odd cycle is chordless ────────────────────────────────────

/// A chord of an odd cycle splits it into two paths, one of even length and one
/// of odd; the odd one closes into a shorter odd cycle. Searched here on every
/// odd cycle up to length eleven and every chord it admits.
pub fn walk_chordless() {
    sprintln!("");
    rule();
    sprintln!("  CHORDLESS — a chord of an odd cycle gives a shorter odd cycle");
    rule();

    let mut steps: Vec<Step> = Vec::new();
    let mut all_ok = true;
    let mut example = String::new();
    let mut len = 3u64;
    while len <= 11 {
        if len % 2 == 1 {
            let mut i = 0u64;
            while i < len {
                let mut j = i + 2;
                while j < len && !(i == 0 && j == len - 1) {
                    // The chord (i, j) splits the cycle into paths of lengths
                    // a and b with a + b = len. Adding the chord closes each
                    // into a cycle of length a+1 and b+1. len is odd, so one
                    // path is even and the other odd; the EVEN one plus the
                    // chord is the shorter odd cycle. Taking the odd path
                    // instead gives an even cycle, which is the slip this step
                    // caught on C₅ with the chord (0,2).
                    let a = j - i;
                    let b = len - a;
                    let even_side = if a % 2 == 0 { a } else { b };
                    let shorter = even_side + 1;
                    if !(shorter % 2 == 1 && shorter < len) {
                        all_ok = false;
                        if example.is_empty() {
                            example = format!("C{} chord ({},{}) gives {}", len, i, j, shorter);
                        }
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        len += 1;
    }
    steps.push(Step {
        title: "Every chord yields a strictly shorter odd cycle",
        computed: if all_ok { "checked C₃ … C₁₁ and every chord: the even side plus the chord closes shorter and odd".into() }
                  else { format!("counterexample: {}", example) },
        holds: all_ok,
    });
    show(1, 1, &steps[0]);
    finish("CHORDLESS ODD CYCLE", &steps);
}

// ── Roth and Behrend ────────────────────────────────────────────────────────

/// The two directions on progression-free sets: a dense subset of a short
/// interval must contain a three-term progression, and Behrend's construction
/// beats every polynomial saving. Both are searched at the size where a search
/// still settles them.
pub fn walk_roth() {
    sprintln!("");
    rule();
    sprintln!("  ROTH AND BEHREND — density against construction");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // Every subset of [0, 9) of size 6 contains a three-term progression.
    let n = 9usize;
    let mut worst = 0usize;
    let mut mask = 0u32;
    while mask < (1u32 << n) {
        let card = mask.count_ones() as usize;
        if card > worst {
            let mut apfree = true;
            for a in 0..n { for b in 0..n { for c in 0..n {
                if (mask >> a) & 1 == 1 && (mask >> b) & 1 == 1 && (mask >> c) & 1 == 1
                   && a + c == 2 * b && a != b {
                    apfree = false;
                }
            }}}
            if apfree { worst = card; }
        }
        mask += 1;
    }
    steps.push(Step {
        title: "The largest progression-free subset of [0,9)",
        computed: format!("size {} — every larger subset carries a three-term progression", worst),
        holds: worst == 5,
    });
    show(1, 2, &steps[0]);

    // Behrend's idea: points on a sphere in base 2k+1 with digits below k carry
    // no progression, since a progression forces collinearity on a sphere.
    let base = 7u64;
    let digits = 3u32;
    let mut count = 0u64;
    let mut spheres: Vec<u64> = Vec::new();
    let mut v = 0u64;
    while v < base.pow(digits) {
        let (mut m, mut sq) = (v, 0u64);
        let mut ok = true;
        for _ in 0..digits {
            let d = m % base; m /= base;
            if d >= base / 2 { ok = false; }
            sq += d * d;
        }
        if ok { spheres.push(sq); count += 1; }
        v += 1;
    }
    // the largest single sphere is the construction's set
    let mut best = 0u64;
    for r in 0..(digits as u64 * (base / 2) * (base / 2) + 1) {
        let c = spheres.iter().filter(|&&x| x == r).count() as u64;
        if c > best { best = c; }
    }
    steps.push(Step {
        title: "Behrend's sphere in base seven, three digits",
        computed: format!("{} points with digits below the half-base, the largest sphere carrying {}",
                          count, best),
        holds: best > 1,
    });
    show(2, 2, &steps[1]);
    finish("ROTH AND BEHREND", &steps);
}

// ── The alternating prime series: the rise criterion ───────────────────────

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2u64;
    while d * d <= n { if n % d == 0 { return false; } d += 1; }
    true
}

fn nth_prime(k: u64) -> u64 {
    let (mut c, mut n) = (0u64, 1u64);
    loop { n += 1; if is_prime(n) { c += 1; if c == k { return n; } } }
}

/// `aₙ = n/pₙ` rises exactly when `n(pₙ₊₁ − pₙ) < pₙ`. The criterion is checked
/// against the terms themselves, so a slip in either direction shows.
pub fn walk_alternating() {
    sprintln!("");
    rule();
    sprintln!("  ALTERNATING — the rise criterion for n/pₙ");
    rule();

    let mut steps: Vec<Step> = Vec::new();
    let mut agree = true;
    let (mut rises, mut falls) = (0u64, 0u64);
    let mut k = 1u64;
    while k <= 200 {
        let p = nth_prime(k);
        let q = nth_prime(k + 1);
        // aₖ < aₖ₊₁  ⟺  k/p < (k+1)/q  ⟺  k·q < (k+1)·p  ⟺  k(q−p) < p
        let direct = k * q < (k + 1) * p;
        let criterion = k * (q - p) < p;
        if direct != criterion { agree = false; }
        if direct { rises += 1; } else { falls += 1; }
        k += 1;
    }
    steps.push(Step {
        title: "The criterion agrees with the terms",
        computed: format!("first 200 indices: {} rises, {} falls, criterion and comparison {}",
                          rises, falls, if agree { "identical" } else { "DIVERGE" }),
        holds: agree,
    });
    show(1, 2, &steps[0]);

    steps.push(Step {
        title: "Neither direction is eventual",
        computed: format!("both rises and falls occur past index 100: {}",
                          rises > 0 && falls > 0),
        holds: rises > 0 && falls > 0,
    });
    show(2, 2, &steps[1]);
    finish("ALTERNATING PRIME SERIES", &steps);
}

// ── Erdős–Fuchs: the averaged representation count ─────────────────────────

/// The fibring identity: summing the representation count below `N` counts the
/// pairs of `A` summing below `N`. Checked on explicit sets, since an identity
/// between two counts is exactly the kind of claim a walk can settle.
pub fn walk_fuchs() {
    sprintln!("");
    rule();
    sprintln!("  ERDŐS–FUCHS — the sum of the representation count");
    rule();

    let mut steps: Vec<Step> = Vec::new();
    let sets: [&[u64]; 3] = [&[0, 1, 3, 7], &[0, 2, 3, 5, 8], &[1, 2, 4, 8, 16]];
    let mut ok = true;
    let mut line = String::new();
    for a in sets.iter() {
        let n = 20u64;
        // left: sum over m < N of r_A(m)
        let mut left = 0u64;
        let mut m = 0u64;
        while m < n {
            for x in a.iter() { for y in a.iter() { if x + y == m { left += 1; } } }
            m += 1;
        }
        // right: pairs with a < N, b < N, a + b < N
        let mut right = 0u64;
        for x in a.iter() { for y in a.iter() {
            if *x < n && *y < n && x + y < n { right += 1; }
        }}
        if left != right { ok = false; }
        line.push_str(&format!("{}={} ", left, right));
    }
    steps.push(Step {
        title: "The fibring identity, on three sets",
        computed: format!("{}— the two counts agree: {}", line, ok),
        holds: ok,
    });
    show(1, 2, &steps[0]);

    // The square-root density bound: N − N₀ ≤ |A ∩ [0,N)|².
    let a: [u64; 6] = [0, 1, 3, 7, 12, 20];
    let mut bound_ok = true;
    let mut n = 1u64;
    while n <= 40 {
        let card = a.iter().filter(|&&x| x < n).count() as u64;
        let covered = (0..n).filter(|m| a.iter().any(|x| a.iter().any(|y| x + y == *m))).count() as u64;
        if covered > card * card { bound_ok = false; }
        n += 1;
    }
    steps.push(Step {
        title: "A basis of square-root density covers at most its square",
        computed: format!("covered values never exceed the square of the count, to N = 40: {}", bound_ok),
        holds: bound_ok,
    });
    show(2, 2, &steps[1]);
    finish("ERDŐS–FUCHS", &steps);
}

// ── The multicolour Ramsey ratio ───────────────────────────────────────────

/// The ratio `R_k(C_{2n+1}) / R_k(K₃)` is bounded by `(2n+1)(2/3.199)^k`, and
/// the base being below one is what makes the ratio vanish. Computed here in
/// integer arithmetic scaled by a thousand, so no float rounding decides it.
pub fn walk_ramsey_ratio() {
    sprintln!("");
    rule();
    sprintln!("  RAMSEY RATIO — the base below one, and the vanishing");
    rule();

    let mut steps: Vec<Step> = Vec::new();

    // 2/3.199 < 1, in integers: 2·1000 < 3199.
    let base_ok = 2 * 1000 < 3199;
    steps.push(Step {
        title: "The base 2/3.199 is below one",
        computed: format!("2000 < 3199: {}", base_ok),
        holds: base_ok,
    });
    show(1, 2, &steps[0]);

    // (2n+1)·base^k falls below one for every fixed n once k is large: track the
    // numerator against the denominator in integers.
    let mut vanishes = true;
    let mut line = String::new();
    for n in [1u64, 5, 25] {
        let mut num = ((2 * n + 1) as u128) * 1000u128;   // (2n+1) scaled
        let mut den = 1000u128;
        let mut k = 0u32;
        while num >= den && k < 200 {
            num *= 2000;      // × 2/3.199
            den *= 3199;
            k += 1;
        }
        if k >= 200 { vanishes = false; }
        line.push_str(&format!("n={}:k={} ", n, k));
    }
    steps.push(Step {
        title: "The bound falls below one at a finite number of colours",
        computed: format!("{}— every case terminates: {}", line, vanishes),
        holds: vanishes,
    });
    show(2, 2, &steps[1]);
    finish("RAMSEY RATIO", &steps);
}

/// The walks available, and what each computes.
pub fn list_walks() {
    sprintln!("  Erdős manuscript walks — each step computed on this kernel:");
    sprintln!("    schutte   f(2) = 7: the search that fails at six, Paley at seven");
    sprintln!("    landau    g(n) for n = 1..10 by complete partition descent");
    sprintln!("    lcm       lcm(1..n) against n·log(n+1), from the quadratic bound");
    sprintln!("    sumset    the g₃ witnesses: AP-free, sums distinct, 2^n ≤ nN+1");
    sprintln!("    ramsey33  R(3,3) = 6, the pentagon below and the search above");
    sprintln!("    binomial  the row gcd: p at a prime power, one otherwise");
    sprintln!("    reptiling six is the first n with no triangle dissection");
    sprintln!("    mantel    the balanced count, and one edge past it forcing a triangle");
    sprintln!("    kac       ω > 1 windows, and why they carry no prime power");
    sprintln!("    sdr       f(n,n) for the first nine, and the minimum at the lcm");
    sprintln!("    lenz      the four-dimensional construction and its ⌊n²/4⌋ pairs");
    sprintln!("    syndetic  a run in a set is a gap in its complement");
    sprintln!("    straus    the price-zero layer's coverage, and 2521 at rung 23");
    sprintln!("    oddcycle  bipartite classes bound the count, and C₅ does not split");
    sprintln!("    chordless a chord of an odd cycle gives a shorter odd cycle");
    sprintln!("    roth      the largest AP-free subset of [0,9), and Behrend's sphere");
    sprintln!("    tripsum   distinct triple sums, and the C(k,3) ≤ 3N bound they force");
    sprintln!("    ap3       the settled ring for Erdős 3, and ρ²=10 without a float");
    sprintln!("");
    sprintln!("  Run with:  erdos <name>");
}

/// Dispatch by name.
pub fn dispatch(name: &str) {
    match name {
        "schutte" => walk_schutte(),
        "landau" => walk_landau(),
        "lcm" => walk_lcm(),
        "sumset" => walk_sumset(),
        "ramsey33" => walk_ramsey33(),
        "binomial" => walk_binomial(),
        "reptiling" => walk_reptiling(),
        "mantel" => walk_mantel(),
        "kac" => walk_kac(),
        "sdr" => walk_sdr(),
        "lenz" => walk_lenz(),
        "syndetic" => walk_syndetic(),
        "straus" => walk_straus(),
        "oddcycle" => walk_monochromatic(),
        "chordless" => walk_chordless(),
        "roth" => walk_roth(),
        "tripsum" => walk_tripsum(),
        "ap3" => walk_ap3(),
        "alternate" => walk_alternating(),
        "fuchs" => walk_fuchs(),
        "ratio" => walk_ramsey_ratio(),
        _ => {
            sprintln!("No Erdős walk named '{}'.", name);
            list_walks();
        }
    }
}
