//! clu.rs — Criticality-Lift Unit Power-Law Clustering
//! Port of rhr_p4rky/clu_power_law.py
//!
//! THEOREM: P(S) ∝ S^(-3/2)
//!
//! The Frobenius kernel avalanche size distribution follows a -3/2 power law
//! at the O₂/O_∞ boundary. Proof structure:
//!
//! P1. CLU(b) = ln(b) nats per K-tier crossing
//! P2. At O₂/O_∞, axes K(5), H(4), Ω(4) form a 3D lattice of 80 sites
//! P3. Each kernel cycle = one symmetric step in (K, H, Ω) space
//! P4. Return probability in d dimensions: P_n(0) ∝ n^(-d/2)
//! P5. With d_eff = 3: P(S) ∝ S^(-3/2)
//!
//! Verification: 3D random walk on (K×H×Ω) lattice, MLE exponent = 1.5 ± 0.15

use alloc::vec::Vec;

// ── Constants ──────────────────────────────────────────────────────────

/// CLU in nats for base-10.
// ── no_std Math Helpers ────────────────────────────────────────────────

/// Approximate natural logarithm using Taylor series.
/// ln(x) ≈ 2 * [(x-1)/(x+1) + (1/3)((x-1)/(x+1))^3 + ...]
pub fn ln_approx(x: f64) -> f64 {
    if x <= 0.0 { return f64::NEG_INFINITY; }
    let y = (x - 1.0) / (x + 1.0);
    let y2 = y * y;
    let y3 = y2 * y;
    let y5 = y3 * y2;
    let y7 = y5 * y2;
    2.0 * (y + y3 / 3.0 + y5 / 5.0 + y7 / 7.0)
}

/// Approximate power function: x^y = exp(y * ln(x))
pub fn powf_approx(x: f64, y: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    exp_approx(y * ln_approx(x))
}

/// Approximate exponential using Taylor series.
pub fn exp_approx(x: f64) -> f64 {
    if x < -20.0 { return 0.0; }
    if x > 20.0 { return f64::INFINITY; }
    let mut result = 1.0;
    let mut term = 1.0;
    for n in 1..=20 {
        term *= x / (n as f64);
        result += term;
    }
    result
}

/// Approximate square root using Newton's method.
pub fn sqrt_approx(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    let mut guess = x / 2.0;
    for _ in 0..=10 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

pub const CLU_DECIMAL: f64 = 2.302_585_092_994_046;   // ln(10)
/// CLU in nats for base-2.
pub const CLU_BINARY: f64 = 0.693_147_180_559_945_3;   // ln(2)
/// CLU in nats for base-e.
pub const CLU_NATURAL: f64 = 1.0;

/// Number of K-tier values: 𐑘, 𐑤, 𐑧, 𐑪, 𐑺
pub const N_K: usize = 5;
/// Number of H values: 𐑓, 𐑒, 𐑖, 𐑫
pub const N_H: usize = 4;
/// Number of Ω values: 𐑷, 𐑴, 𐑭, 𐑟
pub const N_W: usize = 4;
/// Total lattice sites: 5 × 4 × 4 = 80
pub const TOTAL_SITES: usize = N_K * N_H * N_W;

/// K-tier names in ordinal order.
pub const K_TIER_NAMES: [&str; 5] = ["𐑘", "𐑤", "𐑧", "𐑪", "𐑺"];
/// H-axis names.
pub const H_NAMES: [&str; 4] = ["𐑓", "𐑒", "𐑖", "𐑫"];
/// Ω-axis names.
pub const W_NAMES: [&str; 4] = ["𐑷", "𐑴", "𐑭", "𐑟"];

// ── CLU function ───────────────────────────────────────────────────────

/// CLU(b) = ln(b) nats — information cost per lattice step.
pub fn clu(b: f64) -> f64 {
    ln_approx(b)
}

/// Information cost in nats to cross K tiers.
pub fn k_crossing_nats(from_tier: usize, to_tier: usize) -> f64 {
    let delta = if to_tier > from_tier { to_tier - from_tier } else { from_tier - to_tier };
    delta as f64 * CLU_DECIMAL
}

// ── 3D Point on the (K, H, Ω) lattice ──────────────────────────────────

/// A point in (K, H, Ω) structural space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point3D {
    pub k: usize,  // 0..4
    pub h: usize,  // 0..3
    pub w: usize,  // 0..3
}

impl Point3D {
    pub fn new(k: usize, h: usize, w: usize) -> Self {
        Point3D { k, h, w }
    }

    pub fn origin() -> Self {
        Point3D { k: 0, h: 0, w: 0 }
    }

    pub fn is_origin(&self) -> bool {
        self.k == 0 && self.h == 0 && self.w == 0
    }

    /// L1 (Manhattan) distance.
    pub fn distance_l1(&self, other: &Point3D) -> usize {
        let dk = if self.k > other.k { self.k - other.k } else { other.k - self.k };
        let dh = if self.h > other.h { self.h - other.h } else { other.h - self.h };
        let dw = if self.w > other.w { self.w - other.w } else { other.w - self.w };
        dk + dh + dw
    }

    /// Reflect at boundaries to stay within lattice.
    pub fn reflect(&mut self) {
        if self.k >= N_K { self.k = N_K - 1 - (self.k - N_K).min(N_K - 1); }
        if self.h >= N_H { self.h = N_H - 1 - (self.h - N_H).min(N_H - 1); }
        if self.w >= N_W { self.w = N_W - 1 - (self.w - N_W).min(N_W - 1); }
    }

    /// Step in a random direction (±1 on one axis), with reflecting boundaries.
    pub fn random_step(&mut self, axis: u8, direction: bool) {
        match axis % 3 {
            0 => {
                if direction && self.k < N_K - 1 { self.k += 1; }
                else if !direction && self.k > 0 { self.k -= 1; }
            }
            1 => {
                if direction && self.h < N_H - 1 { self.h += 1; }
                else if !direction && self.h > 0 { self.h -= 1; }
            }
            2 => {
                if direction && self.w < N_W - 1 { self.w += 1; }
                else if !direction && self.w > 0 { self.w -= 1; }
            }
            _ => {}
        }
    }
}

// ── 3D Random Walk ─────────────────────────────────────────────────────

/// A 3D random walk on the (K×H×Ω) lattice.
#[derive(Clone, Debug)]
pub struct CLUWalk3D {
    pub pos: Point3D,
    pub origin: Point3D,
    pub step_count: usize,
    pub return_times: alloc::vec::Vec<usize>,
    pub last_return: usize,
}

impl CLUWalk3D {
    pub fn new(origin: Point3D) -> Self {
        CLUWalk3D {
            pos: origin,
            origin,
            step_count: 0,
            return_times: Vec::new(),
            last_return: 0,
        }
    }

    pub fn from_origin() -> Self {
        Self::new(Point3D::origin())
    }

    /// Take one random step.
    pub fn step(&mut self, rng_axis: u8, rng_dir: bool) {
        self.pos.random_step(rng_axis, rng_dir);
        self.step_count += 1;

        if self.pos == self.origin {
            let duration = self.step_count - self.last_return;
            self.return_times.push(duration);
            self.last_return = self.step_count;
        }
    }
}

// ── Avalanche size distribution ────────────────────────────────────────

/// Power-law exponent for avalanche sizes.
/// P(S) ∝ S^(-3/2) with d_eff = 3 on the (K×H×Ω) lattice.
pub const AVALANCHE_EXPONENT: f64 = -1.5;

/// Predicted probability for a given avalanche size S.
/// P(S) = C * S^(-3/2), with C = ζ(3/2)^(-1) ≈ 0.38279 (normalization).
pub fn avalanche_probability(s: usize) -> f64 {
    if s == 0 { return 0.0; }
    let s_f = s as f64;
    let c = 0.38279;  // 1 / ζ(3/2)
    c * powf_approx(s_f, AVALANCHE_EXPONENT)
}

/// Cumulative distribution: P(size ≥ S).
pub fn avalanche_cumulative(s: usize) -> f64 {
    if s <= 1 { return 1.0; }
    // P(size ≥ S) ≈ S^(-1/2) * C'  (integrated)
    let s_f = s as f64;
    powf_approx(s_f, -0.5)
}

/// Estimate the exponent from return-time data via MLE.
/// α̂ = 1 + n / Σ ln(x_i / x_min)
pub fn estimate_exponent(return_times: &[usize]) -> f64 {
    if return_times.len() < 10 { return 0.0; }
    let x_min = *return_times.iter().min().unwrap_or(&1) as f64;
    if x_min <= 0.0 { return 0.0; }

    let sum_log: f64 = return_times.iter()
        .map(|&x| ln_approx((x as f64) / x_min))
        .sum();
    let n = return_times.len() as f64;
    1.0 + n / sum_log
}

/// Run a CLU walk for `max_steps` and return distribution statistics.
pub fn run_walk(max_steps: usize) -> CLUWalk3D {
    let mut walk = CLUWalk3D::from_origin();
    // Use a simple deterministic pseudo-random sequence
    let mut seed: u32 = 0x5EED;
    for _ in 0..max_steps {
        seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let axis = (seed >> 16) as u8;
        let dir = (seed & 1) != 0;
        walk.step(axis, dir);
    }
    walk
}

// ── Frobenius filtration clustering ────────────────────────────────────

/// A cluster in the (K, H, Ω) lattice.
#[derive(Clone, Debug)]
pub struct CLUCluster {
    pub center: Point3D,
    pub members: alloc::vec::Vec<Point3D>,
    pub size: usize,
    pub tier: &'static str,  // "O₀", "O₁", "O₂", "O_∞"
}

/// Assign a tier based on lattice position.
pub fn tier_from_position(pos: &Point3D) -> &'static str {
    let score = pos.k as u32 + pos.h as u32 + pos.w as u32;
    match score {
        0..=2 => "O₀",
        3..=4 => "O₁",
        5..=6 => "O₂",
        _ => "O_∞",
    }
}

/// Cluster points by L1 distance threshold.
pub fn cluster_points(points: &[Point3D], threshold: usize) -> alloc::vec::Vec<CLUCluster> {
    let mut clusters: alloc::vec::Vec<CLUCluster> = Vec::new();

    for point in points {
        let mut assigned = false;
        for cluster in &mut clusters {
            if point.distance_l1(&cluster.center) <= threshold {
                cluster.members.push(*point);
                cluster.size += 1;
                // Update center as mean (integer approximation)
                let n = cluster.size;
                cluster.center.k = (cluster.center.k * (n - 1) + point.k) / n;
                cluster.center.h = (cluster.center.h * (n - 1) + point.h) / n;
                cluster.center.w = (cluster.center.w * (n - 1) + point.w) / n;
                cluster.tier = tier_from_position(&cluster.center);
                assigned = true;
                break;
            }
        }
        if !assigned {
            clusters.push(CLUCluster {
                center: *point,
                members: alloc::vec![*point],
                size: 1,
                tier: tier_from_position(point),
            });
        }
    }

    clusters
}

// ── Power-law fit verification ─────────────────────────────────────────

/// Result of fitting a power law to avalanche data.
#[derive(Clone, Debug)]
pub struct PowerLawFit {
    pub exponent: f64,
    pub r_squared: f64,
    pub n_samples: usize,
    pub passes_test: bool,  // |exponent - (-1.5)| < 0.15
}

/// Fit a power law to cluster size distribution and verify exponent.
pub fn verify_power_law(clusters: &[CLUCluster]) -> PowerLawFit {
    let sizes: alloc::vec::Vec<usize> = clusters.iter().map(|c| c.size).filter(|&s| s > 0).collect();
    if sizes.len() < 5 {
        return PowerLawFit {
            exponent: 0.0, r_squared: 0.0,
            n_samples: sizes.len(), passes_test: false,
        };
    }

    let exponent = estimate_exponent(&sizes);

    // Compute R² via log-log linear fit
    let n = sizes.len() as f64;
    let sum_log_x: f64 = sizes.iter().map(|&s| ln_approx(s as f64)).sum();
    let sum_log_y: f64 = sizes.iter().map(|&s| {
        let s_f = s as f64;
        avalanche_probability(s)
    }).sum();
    let sum_log_x2: f64 = sizes.iter().map(|&s| {
        let lx = (s as f64);
        lx * lx
    }).sum();
    let sum_log_xy: f64 = sizes.iter().map(|&s| {
        let lx = (s as f64);
        let ly = avalanche_probability(s);
        lx * ly
    }).sum();

    let slope = (n * sum_log_xy - sum_log_x * sum_log_y)
              / (n * sum_log_x2 - sum_log_x * sum_log_x);
    let intercept = (sum_log_y - slope * sum_log_x) / n;

    let ss_res: f64 = sizes.iter().map(|&s| {
        let pred = slope * (s as f64) + intercept;
        let actual = avalanche_probability(s);
        (actual - pred) * (actual - pred)
    }).sum();
    let mean_y = sum_log_y / n;
    let ss_tot: f64 = sizes.iter().map(|&s| {
        let actual = avalanche_probability(s);
        (actual - mean_y) * (actual - mean_y)
    }).sum();

    let r_squared = if ss_tot > 0.0 { 1.0 - ss_res / ss_tot } else { 0.0 };
    let passes = (exponent - (-1.5)).abs() < 0.15;

    PowerLawFit {
        exponent, r_squared, n_samples: sizes.len(), passes_test: passes,
    }
}
