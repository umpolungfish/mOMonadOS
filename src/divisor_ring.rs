// divisor_ring.rs — Sigma(n) Constitutive Set Divisor Ring
// Ported from MoDoT/modot/divisor_ring.py to native no_std Rust for mOMonadOS.
//
// For integer n, computes Σ(n) = {d : d|n} (divisors) and applies the
// 'close' verb: forms a divisor ring using the divisibility lattice.
// The close bond identifies 1 and n cyclically, collapsing the chain into a ring.
//
// Results:
//   Prime n (|Σ|=2): degenerate ring (dimer), Ω=1 "prime state"
//   Composite n (|Σ|>2): stable ring with Ω = |Σ(n)|
//   n=1: trivial
//
// Mersenne application: analyze M_p = 2^p - 1 to study the divisor ring
// structure of Mersenne candidates. The spectral invariants and bond density
// reveal structural proximity to the prime state.
//
// Author: Lando⊗⊙perator

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use libm::{sqrt, cos, fabs};

/// π constant for spectral computation (libm doesn't provide it).
const PI: f64 = 3.14159265358979323846;

// ─── Divisors ──────────────────────────────────────────────────────────

/// Compute Σ(n) — the constitutive set of divisors of n.
/// Returns sorted vector of all positive divisors.
pub fn divisors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    if n == 0 {
        return result;
    }
    let limit = sqrt(n as f64) as u64;
    for d in 1..=limit {
        if n % d == 0 {
            result.push(d);
            let complement = n / d;
            if complement != d {
                result.push(complement);
            }
        }
    }
    // Bubble sort — no_std compatible, small vectors
    for i in 0..result.len() {
        for j in 0..result.len() - 1 - i {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
            }
        }
    }
    result
}

// ─── Prime Factorization ───────────────────────────────────────────────

/// Prime factorization of n: returns vector of (prime, exponent) pairs.
pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    if n <= 1 {
        return factors;
    }
    // Factor out 2
    let mut count = 0u32;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    if count > 0 {
        factors.push((2, count));
    }
    // Factor out odd primes
    let mut d: u64 = 3;
    while d * d <= n {
        count = 0;
        while n % d == 0 {
            n /= d;
            count += 1;
        }
        if count > 0 {
            factors.push((d, count));
        }
        d += 2;
    }
    // Remaining prime
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

/// Compute the exponent vector of x with respect to given primes.
fn exponent_vector(x: u64, primes: &[(u64, u32)]) -> Vec<u32> {
    let mut vec = Vec::new();
    for &(p, _) in primes {
        let mut e = 0u32;
        let mut y = x;
        while y % p == 0 {
            e += 1;
            y /= p;
        }
        vec.push(e);
    }
    vec
}

// ─── Divisor Lattice Chain ─────────────────────────────────────────────

/// Order divisors into a chain respecting the divisibility lattice.
/// The Hasse diagram of the divisor lattice is a product of chains
/// (one per prime factor). We linearize by lexicographic sort on
/// the prime exponent vector — producing a Hamiltonian path through
/// the lattice that respects divisibility adjacency.
pub fn divisor_lattice_chain(divs: &[u64]) -> Vec<u64> {
    if divs.len() <= 2 {
        return divs.to_vec();
    }
    let n = divs[divs.len() - 1];
    let pf = prime_factors(n);

    // Compute exponent vectors
    let mut indexed: Vec<(usize, Vec<u32>)> = Vec::new();
    for (i, &d) in divs.iter().enumerate() {
        let ev = exponent_vector(d, &pf);
        indexed.push((i, ev));
    }

    // Lexicographic sort by exponent vector
    indexed.sort_by(|a, b| {
        for (ea, eb) in a.1.iter().zip(b.1.iter()) {
            match ea.cmp(eb) {
                core::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        core::cmp::Ordering::Equal
    });

    indexed.iter().map(|&(i, _)| divs[i]).collect()
}

// ─── Ring Spectrum ─────────────────────────────────────────────────────

/// Spectral invariants of the cycle graph C_k.
/// For k=2 (dimer): eigenvalues {1, -1}.
/// For k>=3: eigenvalues 2*cos(2πj/k) for j=0..k-1.
#[derive(Clone, Debug)]
pub struct RingSpectrum {
    pub rho: f64,
    pub gap: f64,
    pub energy: f64,
    pub spectrum: Vec<f64>,
}

pub fn ring_spectrum(k: usize) -> RingSpectrum {
    if k == 2 {
        RingSpectrum {
            rho: 1.0,
            gap: 0.0,
            energy: 2.0,
            spectrum: vec![1.0, -1.0],
        }
    } else {
        let mut spec = Vec::new();
        for j in 0..k {
            let val = 2.0 * cos(2.0 * PI * (j as f64) / (k as f64));
            spec.push(val);
        }
        // Sort descending
        spec.sort_by(|a, b| b.partial_cmp(a).unwrap_or(core::cmp::Ordering::Equal));

        let rho = 2.0;
        let gap = spec[0] - spec[1];
        let energy: f64 = spec.iter().map(|&v| fabs(v)).sum();

        RingSpectrum { rho, gap, energy, spectrum: spec }
    }
}

// ─── Analysis Result ───────────────────────────────────────────────────

/// Full divisor ring analysis result.
#[derive(Clone, Debug)]
pub struct DivisorRingResult {
    pub n: u64,
    pub sigma_n: Vec<u64>,
    pub k: usize,
    pub verdict: DivisorRingVerdict,
    pub omega: Option<usize>,
    pub ring: Option<RingData>,
    pub note: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DivisorRingVerdict {
    Trivial,
    PrimeState,
    StableRing,
}

/// Ring data for non-trivial divisor rings.
#[derive(Clone, Debug)]
pub struct RingData {
    pub n: usize,
    pub units: Vec<u64>,
    pub conductance: String,
    pub direct_divisibility_bonds: usize,
    pub total_bonds: usize,
    pub spectrum: RingSpectrum,
}

// ─── Analyze ───────────────────────────────────────────────────────────

/// Full divisor ring analysis for n.
pub fn analyze(n: u64) -> DivisorRingResult {
    let divs = divisors(n);
    let k = divs.len();

    if k == 1 {
        return DivisorRingResult {
            n,
            sigma_n: divs,
            k,
            verdict: DivisorRingVerdict::Trivial,
            omega: Some(0),
            ring: None,
            note: String::from(
                "Σ(1) = {1} — a single element cannot form a ring."
            ),
        };
    }

    if k == 2 {
        let spec = ring_spectrum(2);
        let ring = RingData {
            n: 2,
            units: divs.clone(),
            conductance: String::from("DIMER"),
            direct_divisibility_bonds: 1,
            total_bonds: 2,
            spectrum: spec,
        };
        return DivisorRingResult {
            n,
            sigma_n: divs,
            k,
            verdict: DivisorRingVerdict::PrimeState,
            omega: Some(1),
            ring: Some(ring),
            note: format!(
                "A 2-node divisor ring is degenerate — the single bond between 1 and {}. \
                 The Grammar identifies this as the PRIME STATE (Ω=1). \
                 Primes have no internal divisor structure; the ring collapses to a dimer.",
                n
            ),
        };
    }

    // k >= 3: composite — non-trivial divisor lattice
    let ordered = divisor_lattice_chain(&divs);

    // Count direct divisibility bonds in the lattice chain
    let mut bonds: usize = 0;
    for i in 0..k {
        let a = ordered[i];
        let b = ordered[(i + 1) % k];
        if (a != 0 && b % a == 0) || (b != 0 && a % b == 0) {
            bonds += 1;
        }
    }

    let spec = ring_spectrum(k);

    let ring = RingData {
        n: k,
        units: ordered.clone(),
        conductance: String::from("CONDUCTIVE"),
        direct_divisibility_bonds: bonds,
        total_bonds: k,
        spectrum: spec,
    };

    DivisorRingResult {
        n,
        sigma_n: divs,
        k,
        verdict: DivisorRingVerdict::StableRing,
        omega: Some(k),
        ring: Some(ring),
        note: format!(
            "Σ({}) forms a STABLE divisor ring of {} units. \
             The divisor lattice is non-trivial (|Σ|>2), so the close verb \
             identifies 1 and {} cyclically, forming a genuine macrocycle. \
             {}/{} adjacent pairs share direct divisibility; \
             the remaining bonds are structurally mediated through the lattice.",
            n, k, n, bonds, k
        ),
    }
}

// ─── Format Report ─────────────────────────────────────────────────────

/// Pretty-print the divisor ring analysis.
pub fn format_report(result: &DivisorRingResult) -> String {
    let mut lines = Vec::new();

    lines.push(format!("Σ({})  —  constitutive set = divisors of {}", result.n, result.n));
    lines.push(format!("  |Σ({})| = {}", result.n, result.k));

    // Show divisors (limit display for large sets)
    if result.k <= 20 {
        let strs: Vec<String> = result.sigma_n.iter().map(|d| format!("{}", d)).collect();
        lines.push(format!("  Σ({}) = {{{}}}", result.n, strs.join(", ")));
    } else {
        let head: Vec<String> = result.sigma_n[..10].iter().map(|d| format!("{}", d)).collect();
        lines.push(format!("  Σ({}) = {{{}, ... ({} total)}}", result.n, head.join(", "), result.k));
    }
    lines.push(String::new());

    match result.verdict {
        DivisorRingVerdict::Trivial => {
            lines.push(format!("  VERDICT: TRIVIAL"));
            lines.push(format!("  {}", result.note));
            lines.push(format!("  Ω = {}", result.omega.unwrap_or(0)));
        }
        DivisorRingVerdict::PrimeState => {
            lines.push(format!("  VERDICT: PRIME STATE  (Ω = 1)"));
            lines.push(format!("  {} is PRIME. Σ({}) = {{1, {}}}.", result.n, result.n, result.n));
            lines.push(format!("  The 'close' verb forms a DEGENERATE RING (dimer):"));
            lines.push(format!("  a single divisibility bond 1 | {}.", result.n));
            lines.push(format!("  This is the prime state — Ω=1, no internal structure."));
            if let Some(ref ring) = result.ring {
                let spec_strs: Vec<String> = ring.spectrum.spectrum.iter()
                    .map(|v| format!("{:.4}", v)).collect();
                lines.push(format!("  Spectrum: [{}]", spec_strs.join(", ")));
                lines.push(format!("  Conductance: {}", ring.conductance));
            }
        }
        DivisorRingVerdict::StableRing => {
            let omega = result.omega.unwrap_or(result.k);
            lines.push(format!("  VERDICT: STABLE RING  (Ω = {})", omega));
            lines.push(format!("  Σ({}) is NON-TRIVIAL: |Σ|={} > 2.", result.n, result.k));
            lines.push(format!("  The divisor lattice forms a genuine macrocycle under the close verb."));
            if let Some(ref ring) = result.ring {
                let unit_strs: Vec<String> = ring.units.iter().map(|d| format!("{}", d)).collect();
                lines.push(format!("  Ring order: [{}]", unit_strs.join(" . ")));
                lines.push(format!("  Direct divisibility bonds: {}/{}",
                    ring.direct_divisibility_bonds, ring.total_bonds));
                lines.push(format!("  Spectral radius ρ = {:.4}", ring.spectrum.rho));
                lines.push(format!("  Spectral gap = {:.4}", ring.spectrum.gap));
                lines.push(format!("  Graph energy Σ|λ| = {:.4}", ring.spectrum.energy));
                lines.push(format!("  Conductance: {}", ring.conductance));
            }
            lines.push(format!("  {}", result.note));
        }
    }

    lines.join("\n")
}

// ─── Mersenne Analysis ─────────────────────────────────────────────────

/// Analyze a Mersenne candidate: M_p = 2^p - 1.
/// Returns (p, M_p, analysis). If M_p overflows u64, returns None.
pub fn analyze_mersenne(p: u32) -> Option<(u32, u64, DivisorRingResult)> {
    if p > 63 {
        // 2^64 - 1 is max u64
        return None;
    }
    let mp = (1u64 << p) - 1;
    let result = analyze(mp);
    Some((p, mp, result))
}

/// Scan a range of exponents p and collect Mersenne analysis.
/// Returns vector of (p, M_p, verdict_name, omega).
pub fn scan_mersenne_range(p_start: u32, p_end: u32) -> Vec<(u32, u64, String, usize)> {
    let mut results = Vec::new();
    for p in p_start..=p_end {
        if let Some((_p, mp, result)) = analyze_mersenne(p) {
            let verdict = match result.verdict {
                DivisorRingVerdict::PrimeState => "PRIME_STATE",
                DivisorRingVerdict::StableRing => "STABLE_RING",
                DivisorRingVerdict::Trivial => "TRIVIAL",
            };
            results.push((p, mp, String::from(verdict), result.omega.unwrap_or(0)));
        }
    }
    results
}

// ─── Mersenne Structural Proximity ─────────────────────────────────────

/// Compute a "Mersenne proximity score" for M_p = 2^p - 1.
///
/// The score measures how close the divisor ring is to the prime state:
/// - Bond density = direct_divisibility_bonds / total_bonds
/// - Proximity = (bond_density) * (1.0 / omega)
///
/// Higher bond density + smaller omega → closer to prime state.
/// Returns None if M_p overflows u64.
pub fn mersenne_proximity(p: u32) -> Option<f64> {
    if p > 63 { return None; }
    let mp = (1u64 << p) - 1;
    let result = analyze(mp);
    match result.verdict {
        DivisorRingVerdict::PrimeState => Some(1.0),  // exact prime — maximal proximity
        DivisorRingVerdict::Trivial => Some(0.0),
        DivisorRingVerdict::StableRing => {
            if let Some(ring) = result.ring {
                let bond_density = ring.direct_divisibility_bonds as f64 / ring.total_bonds as f64;
                Some(bond_density / (result.k as f64))
            } else {
                Some(0.0)
            }
        }
    }
}
