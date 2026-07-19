// mersenne_parallel.rs — Parallel Mersenne Prime Search for mOMonadOS
// Uses kernel FSPLIT model: fork candidate space, test in parallel, join results.
// Big-integer Lucas-Lehmer with M_p = 2^p - 1 optimized modulo reduction.
//
// Author: Lando⊗⊙perator

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use alloc::format;

/// Big integer using Vec<u64> limbs (little-endian). No_std compatible.
#[derive(Clone, Debug)]
pub struct BigUint {
    limbs: Vec<u64>,
}

impl BigUint {
    pub fn zero() -> Self { BigUint { limbs: Vec::new() } }
    
    pub fn from_u64(n: u64) -> Self {
        if n == 0 { return BigUint::zero(); }
        BigUint { limbs: vec![n] }
    }

    pub fn is_zero(&self) -> bool { self.limbs.is_empty() }

    /// Number of bits
    pub fn bit_len(&self) -> usize {
        if self.limbs.is_empty() { return 0; }
        let top = self.limbs[self.limbs.len() - 1];
        (self.limbs.len() - 1) * 64 + (64 - top.leading_zeros() as usize)
    }

    /// Multiply two BigUints. Simple schoolbook; sufficient for moderate p.
    pub fn mul(&self, other: &BigUint) -> BigUint {
        if self.is_zero() || other.is_zero() { return BigUint::zero(); }
        let n = self.limbs.len();
        let m = other.limbs.len();
        let mut result = vec![0u64; n + m];
        for i in 0..n {
            let mut carry: u64 = 0;
            for j in 0..m {
                let prod = (self.limbs[i] as u128) * (other.limbs[j] as u128) 
                         + (result[i+j] as u128) + (carry as u128);
                result[i+j] = prod as u64;
                carry = (prod >> 64) as u64;
            }
            result[i + m] = carry;
        }
        while result.len() > 1 && result.last() == Some(&0) { result.pop(); }
        BigUint { limbs: result }
    }

    /// Subtract other from self (self >= other). Modifies in place.
    pub fn sub_assign(&mut self, other: &BigUint) {
        let mut borrow: u64 = 0;
        let n = other.limbs.len();
        for i in 0..self.limbs.len() {
            let sub = if i < n { other.limbs[i] } else { 0 };
            let (val, borrow1) = self.limbs[i].overflowing_sub(sub);
            let (val2, borrow2) = val.overflowing_sub(borrow);
            self.limbs[i] = val2;
            borrow = (borrow1 as u64) + (borrow2 as u64);
        }
        while self.limbs.len() > 1 && self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    /// self = self + other
    pub fn add_assign(&mut self, other: &BigUint) {
        let n = other.limbs.len();
        if self.limbs.len() < n { self.limbs.resize(n, 0); }
        let mut carry: u64 = 0;
        for i in 0..self.limbs.len() {
            let a = self.limbs[i] as u128;
            let b = if i < n { other.limbs[i] as u128 } else { 0 };
            let sum = a + b + carry as u128;
            self.limbs[i] = sum as u64;
            carry = (sum >> 64) as u64;
        }
        if carry > 0 { self.limbs.push(carry); }
    }

    /// self >>= shift bits (in place)
    pub fn shr_assign(&mut self, shift: usize) {
        let limb_shift = shift / 64;
        let bit_shift = shift % 64;
        if limb_shift >= self.limbs.len() {
            self.limbs.clear();
            return;
        }
        if bit_shift == 0 {
            self.limbs.drain(0..limb_shift);
        } else {
            for i in 0..self.limbs.len() - limb_shift {
                let low = self.limbs[i + limb_shift] >> bit_shift;
                let high = if i + limb_shift + 1 < self.limbs.len() {
                    self.limbs[i + limb_shift + 1] << (64 - bit_shift)
                } else { 0 };
                self.limbs[i] = low | high;
            }
            self.limbs.truncate(self.limbs.len() - limb_shift);
        }
        while self.limbs.len() > 1 && self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    /// self &= mask where mask = 2^p - 1 (p bits set)
    pub fn and_low_p_bits(&mut self, p: usize) {
        let full_limbs = p / 64;
        let rem_bits = p % 64;
        if self.limbs.len() > full_limbs {
            self.limbs.truncate(full_limbs + if rem_bits > 0 { 1 } else { 0 });
        }
        if rem_bits > 0 && self.limbs.len() > full_limbs {
            let mask = (1u64 << rem_bits) - 1;
            self.limbs[full_limbs] &= mask;
        }
        while self.limbs.len() > 1 && self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    /// Reduce self modulo M_p = 2^p - 1 using: x mod (2^p-1) = (x & (2^p-1)) + (x >> p)
    pub fn mod_mersenne(&mut self, p: usize) {
        while self.bit_len() > p {
            let mut high = self.clone();
            let mut low = self.clone();
            low.and_low_p_bits(p);
            high.shr_assign(p);
            *self = low;
            self.add_assign(&high);
        }
        // One final check: if self == M_p, reduce to 0
        if self.bit_len() == p {
            let mut all_ones = true;
            let full = p / 64;
            let rem = p % 64;
            for i in 0..full {
                if i < self.limbs.len() && self.limbs[i] != u64::MAX { all_ones = false; }
            }
            if rem > 0 && full < self.limbs.len() {
                if self.limbs[full] != (1u64 << rem) - 1 { all_ones = false; }
            }
            if all_ones { *self = BigUint::zero(); }
        }
    }

    /// Convert to hex string (for display)
    pub fn to_hex(&self) -> String {
        if self.limbs.is_empty() { return String::from("0"); }
        let mut s = String::new();
        for (i, limb) in self.limbs.iter().enumerate().rev() {
            if i == self.limbs.len() - 1 {
                s.push_str(&format!("{:x}", limb));
            } else {
                s.push_str(&format!("{:016x}", limb));
            }
        }
        s
    }
}

// ─── Lucas-Lehmer Test ─────────────────────────────────────────────────

/// Lucas-Lehmer primality test for Mersenne number M_p = 2^p - 1.
/// Returns true if M_p is prime, false if composite.
/// Uses the optimized modulo reduction for M_p.
pub fn lucas_lehmer(p: usize) -> bool {
    if p == 2 { return true; }  // M_2 = 3 is prime
    if p < 2 { return false; }
    
    // Small prime check for exponent
    if p > 2 && p % 2 == 0 { return false; }
    
    let mut s = BigUint::from_u64(4);  // s_0 = 4
    
    for _ in 0..(p - 2) {
        // s = s^2 - 2 mod M_p
        let s_sq = s.mul(&s);
        s = s_sq;
        // Subtract 2
        if s.limbs.is_empty() {
            // s was 0, s^2 = 0, but s^2 - 2 underflows. Use M_p - 2.
            // Actually if s = 0, s^2 = 0, s^2 - 2 = -2 ≡ M_p - 2
            s = BigUint::from_u64(2);
            s.limbs[0] = u64::MAX; // temporary hack: handle properly
        } else {
            let two = BigUint::from_u64(2);
            if s.limbs[0] >= 2 {
                s.limbs[0] -= 2;
            } else {
                // Need to borrow: s - 2 ≡ M_p - (2 - s) ≡ M_p - 2 + s (when s < 2)
                // Simpler: just sub_assign, then if negative add M_p back
                s.sub_assign(&two);
            }
        }
        s.mod_mersenne(p);
    }
    
    s.is_zero()
}

// ─── Parallel Search Coordinator ───────────────────────────────────────

/// Result of testing one candidate exponent.
#[derive(Clone, Debug)]
pub enum CandidateResult {
    Prime { p: usize, mp_decimal: String },
    Composite { p: usize },
    Skipped { p: usize, reason: String },
}

/// Split a range [start, end] into n sub-ranges for parallel dispatch.
pub fn split_range(start: usize, end: usize, n: usize) -> Vec<(usize, usize)> {
    let total = end.saturating_sub(start) + 1;
    if total == 0 || n == 0 { return Vec::new(); }
    let chunk = total / n;
    let rem = total % n;
    let mut ranges = Vec::new();
    let mut cur = start;
    for i in 0..n {
        let extra = if i < rem { 1 } else { 0 };
        let next = cur + chunk + extra;
        if next > cur {
            ranges.push((cur, next - 1));
        }
        cur = next;
    }
    ranges
}

/// Search a range of exponents using Lucas-Lehmer, returning all primes found.
/// This is the worker function — each parallel "slice" calls this.
pub fn search_range(p_start: usize, p_end: usize) -> Vec<CandidateResult> {
    let mut results = Vec::new();
    for p in p_start..=p_end {
        // Only test prime exponents (small optimization)
        if p > 2 && p % 2 == 0 { continue; }
        if p > 3 && p % 3 == 0 { continue; }
        if p > 5 && p % 5 == 0 { continue; }
        
        // Quick check: if p is composite, M_p is composite
        if !is_prime_exponent(p) { continue; }
        
        let is_prime = lucas_lehmer(p);
        if is_prime {
            // Compute M_p for display
            let mp = mersenne_number_decimal(p);
            results.push(CandidateResult::Prime { p, mp_decimal: mp });
        } else {
            results.push(CandidateResult::Composite { p });
        }
    }
    results
}

// ─── Helpers ───────────────────────────────────────────────────────────

/// Quick primality test for the exponent (not Mersenne).
/// For small exponents, trial division is fine.
pub fn is_prime_exponent(n: usize) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

/// Compute M_p = 2^p - 1 as a decimal string. Only for small p (display).
pub fn mersenne_number_decimal(p: usize) -> String {
    if p > 256 {
        return format!("2^{} - 1 ({} bits, too large for decimal)", p, p);
    }
    // For p <= 256, compute exactly using u128 or BigUint
    let mut n = BigUint::from_u64(1);
    for _ in 0..p {
        let two = BigUint::from_u64(2);
        n = n.mul(&two);
    }
    let one = BigUint::from_u64(1);
    n.sub_assign(&one);
    n.to_hex()
}

/// Run a parallel search using the kernel's split model.
/// Splits the range into num_splits parallel slices, searches each,
/// and joins the results. Returns all Mersenne primes found.
pub fn parallel_search(p_start: usize, p_end: usize, num_splits: usize) 
    -> (Vec<(usize, String)>, usize, f64) 
{
    let ranges = split_range(p_start, p_end, num_splits);
    let mut all_primes: Vec<(usize, String)> = Vec::new();
    let mut total_tested: usize = 0;
    
    // In the kernel model, FSPLIT forks into num_splits parallel branches.
    // Each branch calls search_range on its slice. FFUSE joins results.
    // Here we simulate this sequentially for the bare-metal implementation;
    // the structural model is the same — the fork/join IS the parallelism.
    
    for (start, end) in &ranges {
        let results = search_range(*start, *end);
        for r in results {
            match r {
                CandidateResult::Prime { p, mp_decimal } => {
                    all_primes.push((p, mp_decimal));
                }
                CandidateResult::Composite { .. } => {
                    total_tested += 1;
                }
                _ => {}
            }
        }
    }
    
    let elapsed = 0.0; // Will be set by caller
    (all_primes, total_tested, elapsed)
}

// ─── REPL Integration ──────────────────────────────────────────────────

/// Run a Mersenne search and return a formatted report.
/// This is the main entry point called from the REPL.
pub fn search_report(p_start: usize, p_end: usize) -> String {
    let mut s = String::new();
    s.push_str(&format!("═══ MERSE NNE PARALLEL SEARCH p={}..{} ═══\n\n", p_start, p_end));
    s.push_str("Model: Kernel FSPLIT/FFUSE fork-join parallelism\n");
    
    // For range size, determine split count
    let range_size = p_end.saturating_sub(p_start) + 1;
    let num_splits = if range_size > 64 { 8 } 
                     else if range_size > 16 { 4 } 
                     else { 1 };
    
    s.push_str(&format!("Splitting into {} parallel branches\n", num_splits));
    
    let ranges = split_range(p_start, p_end, num_splits);
    for (i, (start, end)) in ranges.iter().enumerate() {
        s.push_str(&format!("  Branch {}: p=[{}, {}]\n", i, *start, *end));
    }
    s.push_str("\n");
    
    // Run the search
    let (primes, tested, _elapsed) = parallel_search(p_start, p_end, num_splits);
    
    s.push_str(&format!("Tested {} prime exponents\n", tested));
    s.push_str(&format!("Found {} Mersenne primes:\n\n", primes.len()));
    
    if primes.is_empty() {
        s.push_str("  NONE — no new Mersenne primes in this range.\n");
    } else {
        for (p, mp) in &primes {
            s.push_str(&format!("  M_{} = {}  ← PRIME\n", p, mp));
        }
    }
    
    // Also run divisor ring structural analysis on any primes found
    for (p, _) in &primes {
        if let Some((_pp, _mp, result)) = crate::divisor_ring::analyze_mersenne(*p as u32) {
            s.push_str(&format!("\n  Structural: {}\n", 
                match result.verdict {
                    crate::divisor_ring::DivisorRingVerdict::PrimeState => "PRIME STATE (Ω=1)",
                    _ => "COMPOSITE",
                }));
        }
    }
    
    s
}
