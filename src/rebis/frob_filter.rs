// rebis/frob_filter.rs — Frobenius Filtration
//
// Port of rhr_p4rky/frobenius_filtration.py.
// A structural filter that preserves exactly those elements
// for which the Frobenius condition μ∘δ=id holds.

use crate::belnap::B4;

/// A filter element: a B4 value with a Frobenius check.
#[derive(Copy, Clone, Debug)]
pub struct FilterElement {
    pub value: B4,
    pub passes: bool,
    pub invariant: u8, // Frobenius invariant: 0=identity, 1=parity, 2=fail
}

/// The Frobenius filtration applies fsplit then ffuse and checks
/// whether the result is identical to the input.
pub struct FrobeniusFilter {
    elements: [FilterElement; 64],
    count: usize,
}

impl FrobeniusFilter {
    pub fn new() -> Self {
        Self {
            elements: [FilterElement { value: B4::N, passes: false, invariant: 0 }; 64],
            count: 0,
        }
    }

    /// Add an element to the filter and verify it.
    pub fn push(&mut self, value: B4) {
        if self.count >= 64 { return; }
        // Simulate fsplit: split a B4 value into two components
        let (c1, c2) = fsplit_b4(value);
        // Simulate ffuse: recombine
        let result = ffuse_b4(c1, c2);
        let passes = result == value;
        let invariant = if passes { 0 }
            else if b4_parity(value) == b4_parity(result) { 1 }
            else { 2 };
        self.elements[self.count] = FilterElement { value, passes, invariant };
        self.count += 1;
    }

    /// Count of elements that pass the Frobenius check.
    pub fn pass_count(&self) -> usize {
        self.elements[..self.count].iter().filter(|e| e.passes).count()
    }

    /// Count of elements that fail.
    pub fn fail_count(&self) -> usize {
        self.elements[..self.count].iter().filter(|e| !e.passes).count()
    }

    /// Get all passing elements.
    pub fn passing(&self) -> alloc::vec::Vec<B4> {
        self.elements[..self.count].iter().filter(|e| e.passes).map(|e| e.value).collect()
    }

    /// Frobenius closure ratio: pass_count / total.
    pub fn closure_ratio(&self) -> f64 {
        if self.count == 0 { 0.0 }
        else { self.pass_count() as f64 / self.count as f64 }
    }
}

/// Simplified fsplit on a single B4 value.
/// B4::B splits into (T, F), T into (T, N), F into (N, F), N into (N, N).
pub fn fsplit_b4(v: B4) -> (B4, B4) {
    match v {
        B4::B => (B4::T, B4::F),
        B4::T => (B4::T, B4::N),
        B4::F => (B4::N, B4::F),
        B4::N => (B4::N, B4::N),
    }
}

/// Simplified ffuse on two B4 values.
/// B4 meet: (T,F)→N, (T,N)→T, (N,F)→F, (N,N)→N.
pub fn ffuse_b4(a: B4, b: B4) -> B4 {
    use crate::belnap::meet;
    meet(a, b)
}

/// Parity: B→0, T→1, F→1, N→0 (even/odd under bnot).
pub fn b4_parity(v: B4) -> u8 {
    match v {
        B4::B => 0,
        B4::N => 0,
        B4::T => 1,
        B4::F => 1,
    }
}

// ── Frobenius filtration on codon space ────────────────────────

/// Apply Frobenius filtration to all 64 codons.
/// Returns the set of codons that satisfy ffuse∘fsplit = id.
pub fn filter_codon_space() -> (usize, usize, f64) {
    let mut filter = FrobeniusFilter::new();
    for i in 0..64 {
        let v = codon_index_to_b4(i);
        filter.push(v);
    }
    (filter.pass_count(), filter.fail_count(), filter.closure_ratio())
}

fn codon_index_to_b4(idx: usize) -> B4 {
    match idx % 4 {
        3 => B4::B,
        2 => B4::T,
        1 => B4::F,
        _ => B4::N,
    }
}

/// Power-law clustering analysis (from clu_power_law.py).
/// Computes the Frobenius closure ratio as a function of cluster size.
pub fn cluster_power_law(sizes: &[usize]) -> alloc::vec::Vec<(usize, f64)> {
    let mut results = alloc::vec::Vec::new();
    for &size in sizes {
        let mut filter = FrobeniusFilter::new();
        for i in 0..size.min(64) {
            filter.push(codon_index_to_b4(i));
        }
        results.push((size, filter.closure_ratio()));
    }
    results
}

/// Clustering power-law exponent α from log-log fit:
/// closure_ratio ~ size^(-α)
/// Clustering power-law exponent α from log-log fit:
/// closure_ratio ~ size^(-α)
/// Uses a simplified ratio comparison (avoiding f64::ln which requires std).
pub fn power_law_exponent(sizes: &[usize]) -> f64 {
    let data = cluster_power_law(sizes);
    if data.len() < 2 { return 0.0; }

    // Simple estimate: α ≈ -(r1/r0 - 1) / (s1/s0 - 1)
    let (s0, r0) = data[0];
    let (s1, r1) = data[data.len() - 1];
    if r0 == 0.0 || s0 == 0 || s1 == 0 { return 0.0; }

    let ratio_r = r1 / r0;
    let ratio_s = (s1 as f64) / (s0 as f64);
    if ratio_s <= 1.0 { return 0.0; }

    // α = -ln(r1/r0) / ln(s1/s0) ≈ (1 - r1/r0) / (s1/s0 - 1) for small changes
    (1.0 - ratio_r) / (ratio_s - 1.0)
}
