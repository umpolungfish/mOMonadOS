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
    show(1, 3, &steps[0]);

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
    show(2, 3, &steps[1]);

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
    sprintln!("");
    sprintln!("  Run with:  erdos schutte | erdos landau | erdos lcm");
    sprintln!("             erdos sumset  | erdos ramsey33");
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
        _ => {
            sprintln!("No Erdős walk named '{}'.", name);
            list_walks();
        }
    }
}
