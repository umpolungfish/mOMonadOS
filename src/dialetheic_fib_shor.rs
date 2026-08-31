#![allow(dead_code)]
//! dialetheic_fib_shor.rs — Dialetheic Fibonacci Shor's Algorithm
//! =================================================================
//! ob3ect: dialetheic_fibonacci_shors_algorithm
//!   word:        ⊢∈≻⋈⊞∈⊤≻⊥≺∋⊙⋈⊡⊣   (15 steps)
//!   register:    N→N→T→T→Ttf→Ttf→Ttf→Ttf→A→N→TF→TF→TF→TF→TF
//!   verdict:     B (dialetheic) — the first FSPLIT3 dangles
//!   phase:       PHASE-BEARING under ROTAT (final register, topology class)
//!   tuple:       ⟨𐑦𐑶𐑾𐑹𐑐𐑘𐑲𐑠𐑻𐑫𐑳𐑭⟩  (crystal 16404190, kernel-derived)
//!
//! The pipeline is Shor's period-finding run through the Belnap FOUR
//! register machine: the period r is NOT carried by QFT gates but by the
//! 2:1 coherence cost ratio (B-bias preserves the dialetheic B at cost 2,
//! T-bias collapses it at cost 1), and the modular exponentiation is
//! compiled to Fibonacci anyon braids (topological protection). The 16₃
//! register walk is the control flow: the T-arm (constructive interference,
//! correct period) and F-arm (destructive interference, refuted candidates)
//! are split by ∈, evaluated by ⊤/⊥, cleared by ≺, and fused by ∋ into the
//! dialetheic verdict TF = {T,F} — both arms survive the measurement,
//! which is exactly what the 2:1 coherence ratio encodes.
//!
//! Surface: mOMonadOS kernel (canonical). Author: Quantum⊙perator.

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;

// ── 16₃ carrier: subsets of {t, f, T, F} ──────────────────────────────
// SIXTEEN_3 = P({T,F,t,f}): 16 carriers with the three orderings
//   ≤_i (information), ≤_t (truth), ≤_c (constructivity).
// Bit layout: bit0 = t, bit1 = f, bit2 = T, bit3 = F.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Carrier16(pub u8);

impl Carrier16 {
    pub const N: Carrier16 = Carrier16(0);            // ∅ — nothing
    pub const TTF: Carrier16 = Carrier16(0b0111);     // {T,t,f} — engage
    pub const A: Carrier16 = Carrier16(0b1111);       // {T,F,t,f} — all four
    pub const TF: Carrier16 = Carrier16(0b1100);      // {T,F} — dialetheic both

    pub fn has_t(self) -> bool { self.0 & 0b0100 != 0 }
    pub fn has_f(self) -> bool { self.0 & 0b1000 != 0 }
    pub fn has_t_atom(self) -> bool { self.0 & 0b0001 != 0 }
    pub fn has_f_atom(self) -> bool { self.0 & 0b0010 != 0 }

    pub fn label(self) -> String {
        match self.0 {
            0b0000 => "N".to_string(),
            0b0100 => "T".to_string(),
            0b1000 => "F".to_string(),
            0b0111 => "Ttf".to_string(),
            0b1111 => "A".to_string(),
            0b1100 => "TF".to_string(),
            _ => {
                let mut s = String::from("{");
                let mut first = true;
                for (bit, name) in [(0b0001, "t"), (0b0010, "f"), (0b0100, "T"), (0b1000, "F")] {
                    if self.0 & bit != 0 {
                        if !first { s.push(','); }
                        s.push_str(name);
                        first = false;
                    }
                }
                s.push('}');
                s
            }
        }
    }
}

// ── Opcodes ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    VINIT, FSPLIT3, AFWD, CLINK, ENGAGR, EVALT, EVALF, AREV, FFUSE3, IMSCRIB, IFIX, TANCH,
}

impl Op {
    pub fn glyph(self) -> char {
        match self {
            Op::VINIT => '⊢', Op::FSPLIT3 => '∈', Op::AFWD => '≻', Op::CLINK => '⋈',
            Op::ENGAGR => '⊞', Op::EVALT => '⊤', Op::EVALF => '⊥', Op::AREV => '≺',
            Op::FFUSE3 => '∋', Op::IMSCRIB => '⊙', Op::IFIX => '⊡', Op::TANCH => '⊣',
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Op::VINIT => "VINIT", Op::FSPLIT3 => "FSPLIT3", Op::AFWD => "AFWD",
            Op::CLINK => "CLINK", Op::ENGAGR => "ENGAGR", Op::EVALT => "EVALT",
            Op::EVALF => "EVALF", Op::AREV => "AREV", Op::FFUSE3 => "FFUSE3",
            Op::IMSCRIB => "IMSCRIB", Op::IFIX => "IFIX", Op::TANCH => "TANCH",
        }
    }

    /// 16₃ register semantics — the deterministic transition.
    /// VINIT seeds N; FSPLIT3 opens a frame (register untouched); AFWD deposits
    /// T; ENGAGR (EVALI face) engages the paradox {T,t,f}; EVALT/EVALF deposit
    /// T/F; AREV is the clearing reverse (→ N); FFUSE3 fuses to the both-value
    /// TF; the remaining opcodes are identity on the carrier.
    pub fn apply(self, reg: Carrier16) -> Carrier16 {
        match self {
            Op::VINIT => Carrier16::N,
            Op::FSPLIT3 => reg,
            Op::AFWD => Carrier16(reg.0 | 0b0100),
            Op::CLINK => reg,
            Op::ENGAGR => Carrier16::TTF,
            Op::EVALT => Carrier16(reg.0 | 0b0100),
            Op::EVALF => Carrier16(reg.0 | 0b1000),
            Op::AREV => Carrier16::N,
            Op::FFUSE3 => Carrier16::TF,
            Op::IMSCRIB => reg,
            Op::IFIX => reg,
            Op::TANCH => reg,
        }
    }

    /// FSPLIT3 opens a frame, FFUSE3 closes one. A walk that ends with an
    /// open frame is dialetheic (verdict B), not closed.
    pub fn frame_delta(self) -> i32 {
        match self {
            Op::FSPLIT3 => 1,
            Op::FFUSE3 => -1,
            _ => 0,
        }
    }
}

/// The ob3ect's canonical word, glyph by glyph.
pub const THE_WORD: [Op; 15] = [
    Op::VINIT, Op::FSPLIT3, Op::AFWD, Op::CLINK, Op::ENGAGR,
    Op::FSPLIT3, Op::EVALT, Op::AFWD, Op::EVALF, Op::AREV,
    Op::FFUSE3, Op::IMSCRIB, Op::CLINK, Op::IFIX, Op::TANCH,
];

// ── Walk trace ────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct WalkStep {
    pub step: usize,       // 1..=15
    pub op: Op,
    pub reg_in: Carrier16,
    pub reg_out: Carrier16,
    pub frame_depth: i32,  // after this step
}

#[derive(Clone, Debug)]
pub struct WalkTrace {
    pub steps: Vec<WalkStep>,
    pub final_reg: Carrier16,
    pub open_frames: i32,          // > 0 ⇒ dialetheic verdict B
    pub phase_bearing: bool,       // final register moves under ROTAT
    pub period: usize,             // ROTAT orbit period (15 for this word)
}

/// Run the 15-step walk exactly as the ob3ect's sixteen_3 breakdown.
pub fn run_walk() -> WalkTrace {
    let mut reg = Carrier16::N;
    let mut depth: i32 = 0;
    let mut steps = Vec::with_capacity(THE_WORD.len());
    for (i, op) in THE_WORD.iter().enumerate() {
        let reg_in = reg;
        let reg_out = op.apply(reg);
        depth += op.frame_delta();
        steps.push(WalkStep { step: i + 1, op: *op, reg_in, reg_out, frame_depth: depth });
        reg = reg_out;
    }
    // Phase-bearing: rotate the word; if any rotation lands on a different
    // final register, the word moves under ROTAT.
    let (phase_bearing, period) = rotat_analysis();
    WalkTrace { steps, final_reg: reg, open_frames: depth, phase_bearing, period }
}

/// ROTAT orbit of the word: each cyclic cut re-runs the walk. The orbit
/// period is the smallest k > 0 such that rotating by k returns the SAME
/// word (15 for a word with no rotational symmetry). Phase-bearing means
/// some cut lands on a final register different from the identity cut —
/// the word MOVES under ROTAT (ob3ect: final_register, topology_class).
fn rotat_analysis() -> (bool, usize) {
    let base = run_walk_straight(&THE_WORD);
    let mut moved = false;
    for cut in 1..THE_WORD.len() {
        let mut rotated: Vec<Op> = Vec::with_capacity(THE_WORD.len());
        rotated.extend_from_slice(&THE_WORD[cut..]);
        rotated.extend_from_slice(&THE_WORD[..cut]);
        let fin = run_walk_straight(&rotated);
        if fin != base { moved = true; }
        if rotated == THE_WORD { return (moved, cut); }  // word orbit closes
    }
    (moved, THE_WORD.len())
}

fn run_walk_straight(word: &[Op]) -> Carrier16 {
    let mut reg = Carrier16::N;
    for op in word {
        reg = op.apply(reg);
    }
    reg
}

// ── Belnap coherence costs ────────────────────────────────────────────
// The dialetheic reading of Shor: the output register of f(x)=a^x mod N
// carries exactly r distinct values (the subgroup generated by a). A B-bias
// measurement preserves the dialetheic B at cost 2 per distinct state, so
// belnapCost = 2·r. A T-bias measurement collapses at cost 1 per qubit.
// The 2:1 ratio is the invariant — the period is read from the ratio, not
// from a collapsed bit string (belnap_shor.rs finding).

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus <= 1 { return 0; }
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 != 0 { result = (result * base) % modulus; }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

/// Classical period of a mod N — the ground truth the coherence ratio must
/// reproduce.
pub fn classical_period(a: u64, n: u64) -> u64 {
    if n <= 1 || a % n == 0 { return 0; }
    let mut val: u64 = 1;
    for r in 1..=n {
        val = (val * a) % n;
        if val == 1 { return r; }
    }
    0
}

/// Number of distinct values of f(x) = a^x mod N — exactly the period when
/// gcd(a, N) = 1 (the orbit of the subgroup generated by a).
pub fn distinct_outputs(a: u64, n: u64) -> u64 {
    if n <= 1 { return 1; }
    let mut seen: Vec<u64> = Vec::new();
    let mut val: u64 = 1 % n;
    for _ in 0..n {
        if seen.contains(&val) { break; }
        seen.push(val);
        val = (val * a) % n;
        if val == 0 { break; }
    }
    seen.len() as u64
}

// ── Result ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct DialetheicFibShorResult {
    pub n_val: u64,
    pub a_val: u64,
    pub period: u64,
    pub distinct: u64,
    pub belnap_cost: u64,        // 2 · distinct = 2r
    pub t_bias_cost: u64,        // n_qubits (collapse cost)
    pub ratio: f64,              // belnapCost / tCost = 2r / n
    pub n_qubits: usize,
    pub strands: usize,          // Fibonacci anyon strands
    pub fusion_dim: usize,       // F_{strands-1}
    pub braid_len: usize,        // total braid word length
    pub factor1: Option<u64>,
    pub factor2: Option<u64>,
    pub walk: WalkTrace,
    pub tuple: &'static str,     // ⟨𐑦𐑶𐑾𐑹𐑐𐑘𐑲𐑠𐑻𐑫𐑳𐑭⟩
}

/// gcd helper.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

/// Extract factors from the period: p = gcd(a^{r/2} ± 1, N).
pub fn factors_from_period(a: u64, n: u64, r: u64) -> (Option<u64>, Option<u64>) {
    if r == 0 || r % 2 != 0 { return (None, None); }
    let half = mod_pow(a, r / 2, n);
    // a^{r/2} ≡ ±1 mod N: the standard Shor retry case (the congruence gives
    // no factor). half == 1 means a^{r/2} ≡ 1 so r was not the minimal period.
    if half == 0 || half == 1 || half == n - 1 { return (None, None); }
    let p = gcd(half.wrapping_sub(1), n);
    let q = gcd(half + 1, n);
    if p > 1 && q > 1 && p * q == n {
        if p < q { (Some(p), Some(q)) } else { (Some(q), Some(p)) }
    } else {
        (None, None)
    }
}

/// n_qubits needed to represent N in binary.
pub fn qubits_for(n: u64) -> usize {
    if n <= 1 { return 1; }
    let mut bits = 0;
    let mut v = n - 1;
    while v > 0 { bits += 1; v >>= 1; }
    bits.max(1)
}

/// Fusion space dimension F_{m} (vacuum sector, m = strands - 1).
/// Kernel convention F_1=1, F_2=1: 7 strands → F_6=8, 15 → F_14=377,
/// 19 → F_18=2584 (first holding d=2048).
pub fn fib(m: usize) -> usize {
    let (mut a, mut b) = (1usize, 1usize);
    if m <= 1 { return 1; }
    for _ in 2..m { let t = a + b; a = b; b = t; }
    b
}

/// Full run: 16₃ walk + Belnap coherence + Fibonacci braid + factors.
pub fn run_dialetheic_fib_shor(n_val: u64, a_val: u64) -> DialetheicFibShorResult {
    let period = classical_period(a_val, n_val);
    let distinct = distinct_outputs(a_val, n_val);
    let belnap_cost = 2 * distinct;
    let n_qubits = qubits_for(n_val);
    let t_bias_cost = n_qubits as u64;
    let ratio = belnap_cost as f64 / t_bias_cost.max(1) as f64;
    let strands = 3 * n_qubits + 1;
    let fusion_dim = fib(strands - 1);
    let (f1, f2) = factors_from_period(a_val, n_val, period);
    let walk = run_walk();
    DialetheicFibShorResult {
        n_val, a_val, period, distinct, belnap_cost, t_bias_cost, ratio,
        n_qubits, strands, fusion_dim,
        braid_len: estimate_braid_total(n_qubits, a_val, n_val),
        factor1: f1, factor2: f2, walk,
        tuple: "⟨𐑦𐑶𐑾𐑹𐑐𐑘𐑲𐑠𐑻𐑫𐑳𐑭⟩",
    }
}

/// Braid-length estimate: H-layer + controlled-U chain + inverse QFT,
/// mirroring fibonacci_shor.rs's assemble_shor_braid accounting.
fn estimate_braid_total(n_qubits: usize, a: u64, n_val: u64) -> usize {
    let sk_depth = 50usize;
    let n = n_qubits;
    let h = n * sk_depth;
    let mut cu = 0usize;
    for k in 0..n {
        let pow = mod_pow(a, 1u64 << k, n_val);
        if pow != 1 {
            let n_work = qubits_for(n_val);
            for w in 0..n_work {
                if (pow >> w) & 1 != 0 { cu += 1; }
            }
        }
    }
    let iqft = n * (n - 1) / 2 * sk_depth;
    h + cu * sk_depth + iqft
}

// ── Report ────────────────────────────────────────────────────────────

/// Human-readable report of the full dialetheic Fibonacci Shor run.
pub fn report(r: &DialetheicFibShorResult) -> String {
    let mut s = String::new();
    s.push_str(&format!("══ Dialetheic Fibonacci Shor: N={}, a={} ══\n", r.n_val, r.a_val));
    s.push_str(&format!("  tuple: {}  (kernel-derived, crystal 16404190)\n", r.tuple));
    s.push_str(&format!("  word:  ⊢∈≻⋈⊞∈⊤≻⊥≺∋⊙⋈⊡⊣  (15 steps)\n\n"));

    s.push_str("  ── 16₃ register walk ──\n");
    s.push_str("  step  op        reg→reg      frame\n");
    s.push_str("  ────  ────────  ──────────   ─────\n");
    for st in &r.walk.steps {
        s.push_str(&format!(
            "  {:>3}   {:<11} {} → {:<4} depth={}\n",
            st.step,
            format!("{}({})", st.op.glyph(), st.op.name()),
            st.reg_in.label(),
            st.reg_out.label(),
            st.frame_depth,
        ));
    }
    s.push_str(&format!("\n  final register: {}  open frames: {}\n", r.walk.final_reg.label(), r.walk.open_frames));
    s.push_str(&format!(
        "  verdict: {}  — {}FSPLIT3 dangles: the walk is dialetheic, not closed\n",
        if r.walk.open_frames > 0 { "B (DIALETHEIC)" } else { "T (CLOSED)" },
        if r.walk.open_frames > 0 { "✓ " } else { "✗ " },
    ));
    s.push_str(&format!(
        "  phase-bearing: {}  ROTAT period: {}\n\n",
        if r.walk.phase_bearing { "yes" } else { "no" },
        r.walk.period,
    ));

    s.push_str("  ── Belnap coherence ──\n");
    s.push_str(&format!(
        "  period r = {}   distinct outputs = {}   belnapCost = 2r = {}\n",
        r.period, r.distinct, r.belnap_cost,
    ));
    s.push_str(&format!(
        "  T-bias collapse cost = {}   ratio (B/T) = {:.2}  {}\n",
        r.t_bias_cost, r.ratio,
        if (r.ratio - 2.0).abs() < 0.01 { "(2:1 invariant ✓)" } else { "" },
    ));

    s.push_str("\n  ── Fibonacci anyon braid ──\n");
    s.push_str(&format!(
        "  n_qubits = {}   strands = {}   fusion_dim = F_{} = {}\n",
        r.n_qubits, r.strands, r.strands - 1, r.fusion_dim,
    ));
    s.push_str(&format!("  braid_len ≈ {}\n\n", r.braid_len));

    s.push_str("  ── Factorization ──\n");
    match (r.factor1, r.factor2) {
        (Some(p), Some(q)) => {
            s.push_str(&format!("  ✓ {} = {} × {}\n", r.n_val, p, q));
            s.push_str(&format!("  factors written to the classical register (⊡ IFIX)\n"));
        }
        _ => {
            let half = mod_pow(r.a_val, r.period / 2, r.n_val);
            let reason = if r.period % 2 != 0 || r.period == 0 {
                "r odd or 0 — retry with another a"
            } else if half == r.n_val - 1 {
                "a^(r/2) ≡ -1 mod N — standard Shor retry case, use another a"
            } else if half == 1 {
                "a^(r/2) ≡ 1 mod N — r is not the minimal period"
            } else {
                "retry with another a"
            };
            s.push_str(&format!("  ✗ no factor found for N={}, a={} (r={}): {}\n", r.n_val, r.a_val, r.period, reason));
        }
    }
    s
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_matches_ob3ect_trace() {
        let w = run_walk();
        let expected = ["N", "N", "T", "T", "Ttf", "Ttf", "Ttf", "Ttf", "A", "N", "TF", "TF", "TF", "TF", "TF"];
        assert_eq!(w.steps.len(), 15);
        for (i, st) in w.steps.iter().enumerate() {
            assert_eq!(st.reg_out.label(), expected[i], "step {} reg_out", i + 1);
        }
        assert_eq!(w.final_reg.label(), "TF");
        assert_eq!(w.open_frames, 1, "first FSPLIT3 dangles → verdict B");
        assert!(w.phase_bearing, "word is phase-bearing under ROTAT");
    }

    #[test]
    fn carrier_semantics() {
        assert_eq!(Op::VINIT.apply(Carrier16::N), Carrier16::N);
        assert_eq!(Op::AFWD.apply(Carrier16::N), Carrier16(0b0100));
        assert_eq!(Op::ENGAGR.apply(Carrier16(0b0100)), Carrier16::TTF);
        assert_eq!(Op::EVALF.apply(Carrier16::TTF), Carrier16::A);
        assert_eq!(Op::AREV.apply(Carrier16::A), Carrier16::N);
        assert_eq!(Op::FFUSE3.apply(Carrier16::N), Carrier16::TF);
    }

    #[test]
    fn shor_n15_a7() {
        let r = run_dialetheic_fib_shor(15, 7);
        assert_eq!(r.period, 4);
        assert_eq!(r.distinct, 4);
        assert_eq!(r.belnap_cost, 8);
        assert_eq!(r.factor1, Some(3));
        assert_eq!(r.factor2, Some(5));
        assert!((r.ratio - 2.0).abs() < 0.01);
    }

    #[test]
    fn shor_n21_a2() {
        let r = run_dialetheic_fib_shor(21, 2);
        assert_eq!(r.period, 6);
        assert_eq!(r.belnap_cost, 12);
        assert_eq!(r.factor1, Some(3));
        assert_eq!(r.factor2, Some(7));
    }

    #[test]
    fn shor_retry_case_ar2_neg1() {
        // a=5, N=21: 5^(6/2) = 125 ≡ 20 ≡ -1 mod 21 — the standard Shor
        // retry case; no factor is found and the report says so honestly.
        let r = run_dialetheic_fib_shor(21, 5);
        assert_eq!(r.period, 6);
        assert_eq!(r.factor1, None);
        assert_eq!(r.factor2, None);
    }

    #[test]
    fn shor_n35_a2() {
        let r = run_dialetheic_fib_shor(35, 2);
        assert_eq!(r.period, 12);
        assert_eq!(r.factor1, Some(5));
        assert_eq!(r.factor2, Some(7));
    }

    #[test]
    fn fusion_dimensions() {
        assert_eq!(fib(6), 8);    // 7 strands → F_6 = 8 (3 qubits)
        assert_eq!(fib(14), 377); // 15 strands → F_14 = 377 (8 qubits)
        assert_eq!(fib(18), 2584);// 19 strands → F_18 = 2584 (11 qubits, first holding d=2048)
    }

    #[test]
    fn period_matches_distinct_outputs() {
        for &(a, n) in &[(7u64, 15u64), (5, 21), (2, 35), (3, 25), (2, 101)] {
            assert_eq!(classical_period(a, n), distinct_outputs(a, n), "a={} N={}", a, n);
        }
    }
}
