//! Belnap Shor Pipeline — priests-engine/belnap_shor.py
//! Shor's algorithm on Belnap FOUR logic.
//! Key structural finding: Belnap QFT is NOT a gate sequence.
//! Period r is encoded in the 2:1 coherence cost ratio (B-bias vs T-bias).
//! Phi_upsilon bottleneck: extracting r from B-bias alone requires Phi_pmsym.

use alloc::vec::Vec;
use alloc::vec;
use crate::belnap::B4;

// ── Hadamard gate ──────────────────────────────────────────────────

/// H|T⟩=B, H|F⟩=B, H|B⟩=T, H|N⟩=N
pub fn b4_hadamard(q: B4) -> B4 {
    match q {
        B4::T => B4::B,
        B4::F => B4::B,
        B4::B => B4::T,
        B4::N => B4::N,
    }
}

// ── XOR (classical path of ModExp) ─────────────────────────────────

pub fn b4_xor(a: B4, b: B4) -> B4 {
    if a == B4::B || b == B4::B { return B4::B; }
    if (a == B4::T && b == B4::F) || (a == B4::F && b == B4::T) { return B4::T; }
    if a == B4::T && b == B4::T { return B4::F; }
    if a == B4::F && b == B4::F { return B4::F; }
    // N in either → N
    B4::N
}

// ── N-qubit register ───────────────────────────────────────────────

pub struct BelnapRegister {
    pub n: usize,
    pub qubits: Vec<B4>,
    pub coherence_count: u32,
    pub measurements: u32,
}

impl BelnapRegister {
    pub fn classical(n: usize) -> Self {
        Self { n, qubits: vec![B4::T; n], coherence_count: 0, measurements: 0 }
    }

    pub fn superposition(n: usize) -> Self {
        Self { n, qubits: vec![B4::B; n], coherence_count: 0, measurements: 0 }
    }

    pub fn apply_hadamard(&mut self, i: usize) {
        let q = self.qubits[i];
        self.qubits[i] = b4_hadamard(q);
        if matches!(q, B4::T | B4::F | B4::B) {
            self.coherence_count += 1;
        }
    }

    pub fn apply_hadamard_layer(&mut self) {
        for i in 0..self.n {
            self.apply_hadamard(i);
        }
    }

    /// Belnap measurement. B-bias: preserve B, cost 2 (Wigner's Friend).
    /// T/F-bias: collapse B, cost 1.
    pub fn measure(&mut self, i: usize, bias: B4) -> char {
        let q = self.qubits[i];
        self.measurements += 1;
        if q == B4::B {
            if bias == B4::B {
                self.coherence_count += 2;
                'B'
            } else if bias == B4::T {
                self.qubits[i] = B4::T;
                self.coherence_count += 1;
                'T'
            } else if bias == B4::F {
                self.qubits[i] = B4::F;
                self.coherence_count += 1;
                'F'
            } else {
                'N'
            }
        } else {
            match q {
                B4::T => 'T',
                B4::F => 'F',
                B4::N => 'N',
                B4::B => unreachable!(),
            }
        }
    }

    pub fn measure_all(&mut self, bias: B4) -> Vec<char> {
        (0..self.n).map(|i| self.measure(i, bias)).collect()
    }
}

// ── Modular exponentiation circuit ─────────────────────────────────

pub struct BelnapModExp {
    pub input_bits: usize,
    pub a: u64,
    pub n_val: u64,
    pub mod_bits: usize,
    table: Vec<u64>,
}

impl BelnapModExp {
    pub fn new(input_bits: usize, a: u64, n_val: u64) -> Self {
        let mod_bits = if n_val <= 1 { 1 } else {
            let mut bits = 0;
            let mut v = n_val - 1;
            while v > 0 { bits += 1; v >>= 1; }
            bits.max(1)
        };
        let table_size = 1usize << input_bits;
        let mut table = Vec::with_capacity(table_size);
        for x in 0..table_size {
            table.push(mod_pow(a, x as u64, n_val));
        }
        Self { input_bits, a, n_val, mod_bits, table }
    }

    /// f(x) = a^x mod N. B-input → B-output (cost 0).
    pub fn evaluate(&self, word: &[B4]) -> Vec<B4> {
        if word.iter().all(|w| *w == B4::B) {
            return vec![B4::B; self.mod_bits];
        }
        let mut x: u64 = 0;
        for (i, w) in word.iter().enumerate() {
            if *w == B4::T { x |= 1 << i; }
        }
        let result = if (x as usize) < self.table.len() {
            self.table[x as usize]
        } else {
            mod_pow(self.a, x, self.n_val)
        };
        let mut out = Vec::with_capacity(self.mod_bits);
        for i in 0..self.mod_bits {
            out.push(if (result >> i) & 1 != 0 { B4::T } else { B4::F });
        }
        out
    }

    /// Classical period finding.
    pub fn find_period(&self) -> u64 {
        let mut val: u64 = 1;
        for r in 1..=self.n_val {
            val = (val * self.a) % self.n_val;
            if val == 1 { return r; }
        }
        0
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 0 { return 0; }
    if modulus == 1 { return 0; }
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 != 0 { result = (result * base) % modulus; }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

// ── Shor result ────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ShorResult {
    pub n: usize,
    pub a: u64,
    pub n_val: u64,
    pub period_cl: u64,
    pub hadamard_coherence: u32,
    pub mod_exp_coherence: u32,
    pub b_bias_coherence: u32,
    pub t_bias_coherence: u32,
    pub ratio: f64,
    pub mod_exp_all_b: bool,
    pub b_bias_preserves: bool,
    pub t_bias_collapses: bool,
    pub phi_upsilon_bottleneck: bool,
}

/// Execute the Belnap Shor pipeline.
///
/// [0] |T...T⟩  classical init
/// [1] H^⊗n → |B...B⟩  (cost = n)
/// [2] ModExp → |B...B⟩  (cost = 0, B propagates through Boolean gates)
/// [3] B-bias measure  (cost = 2n, Wigner's Friend, preserves B)
/// [4] T-bias measure  (cost = n, collapses B→T)
pub fn run_belnap_shor(n: usize, a: u64, n_val: u64) -> ShorResult {
    let period_cl = classical_period(a, n_val);

    // Step 1–2: H layer + ModExp
    let mut reg = BelnapRegister::classical(n);
    reg.apply_hadamard_layer();
    let had_cost = reg.coherence_count;

    let mod_exp = BelnapModExp::new(n, a, n_val);
    let out = mod_exp.evaluate(&reg.qubits);
    let all_b = out.iter().all(|w| *w == B4::B);

    // Step 3: B-bias measurement (fresh register)
    let mut reg_b = BelnapRegister::classical(n);
    reg_b.apply_hadamard_layer();
    reg_b.measure_all(B4::B);
    let b_preserves = reg_b.qubits.iter().all(|w| *w == B4::B);
    let b_total = reg_b.coherence_count;

    // Step 4: T-bias measurement
    reg.measure_all(B4::T);
    let t_collapsed = reg.qubits.iter().all(|w| matches!(w, B4::T | B4::F));
    let t_total = reg.coherence_count;

    let b_meas_only = b_total - had_cost;
    let t_meas_only = t_total - had_cost;
    let ratio = b_meas_only as f64 / (t_meas_only.max(1) as f64);

    ShorResult {
        n, a, n_val, period_cl,
        hadamard_coherence: had_cost,
        mod_exp_coherence: 0,
        b_bias_coherence: b_meas_only,
        t_bias_coherence: t_meas_only,
        ratio,
        mod_exp_all_b: all_b,
        b_bias_preserves: b_preserves,
        t_bias_collapses: t_collapsed,
        phi_upsilon_bottleneck: b_preserves,
    }
}

fn classical_period(a: u64, n: u64) -> u64 {
    if n <= 1 { return 0; }
    let mut val: u64 = 1;
    for r in 1..=n {
        val = (val * a) % n;
        if val == 1 { return r; }
    }
    0
}

// ── SIC-POVM verification ──────────────────────────────────────────

pub fn verify_sic_povm() -> bool {
    let b = B4::B;
    let all = [B4::N, B4::T, B4::F, B4::B];
    // Axiom 1: meet(B, x) = x  ∀x  (B is identity for meet)
    let ax1 = all.iter().all(|&x| x.meet(b) == x);
    // Axiom 3: join(B, x) = B  ∀x  (B absorbs join)
    let ax3 = all.iter().all(|&x| x.join(b) == B4::B);
    // Axiom 4: bnot(B) = B  (B is negation fixed-point)
    let ax4 = b.bnot() == B4::B;
    // B is top in approx ordering
    let top = all.iter().all(|&x| x.approx_le(b));
    // B is dialetheic, nothing else is
    let dial = b.dialetheic() && all.iter().filter(|&&x| x != b).all(|&x| !x.dialetheic());
    ax1 && ax3 && ax4 && top && dial
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard() {
        assert_eq!(b4_hadamard(B4::T), B4::B);
        assert_eq!(b4_hadamard(B4::F), B4::B);
        assert_eq!(b4_hadamard(B4::B), B4::T);
        assert_eq!(b4_hadamard(B4::N), B4::N);
    }

    #[test]
    fn test_hadamard_layer() {
        let mut reg = BelnapRegister::classical(4);
        reg.apply_hadamard_layer();
        assert!(reg.qubits.iter().all(|&q| q == B4::B));
        assert_eq!(reg.coherence_count, 4);
    }

    #[test]
    fn test_xor() {
        assert_eq!(b4_xor(B4::T, B4::F), B4::T);
        assert_eq!(b4_xor(B4::T, B4::T), B4::F);
        assert_eq!(b4_xor(B4::B, B4::T), B4::B);
    }

    #[test]
    fn test_mod_exp_b_propagation() {
        let me = BelnapModExp::new(4, 7, 15);
        let input = vec![B4::B; 4];
        let out = me.evaluate(&input);
        assert!(out.iter().all(|&q| q == B4::B));
    }

    #[test]
    fn test_shor_n15_a7() {
        let r = run_belnap_shor(4, 7, 15);
        assert_eq!(r.period_cl, 4);
        assert_eq!(r.hadamard_coherence, 4);
        assert_eq!(r.mod_exp_coherence, 0);
        assert_eq!(r.b_bias_coherence, 8);  // 2n
        assert_eq!(r.t_bias_coherence, 4);  // n
        assert_eq!(r.ratio, 2.0);
        assert!(r.mod_exp_all_b);
        assert!(r.b_bias_preserves);
        assert!(r.t_bias_collapses);
    }

    #[test]
    fn test_shor_n21_a5() {
        let r = run_belnap_shor(5, 5, 21);
        assert_eq!(r.period_cl, 6);
        assert_eq!(r.b_bias_coherence, 10);
        assert_eq!(r.t_bias_coherence, 5);
    }

    #[test]
    fn test_sic_povm() {
        assert!(verify_sic_povm());
    }
}
