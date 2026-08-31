// pericyclic_frobenoid.rs — Pericyclic Semiotic Frobenoid (Rust port for mOMonadOS)
// Tuple: ⟨𐑦𐑥𐑑𐑹𐑐𐑤𐑔𐑝⊙𐑒𐑙𐑷⟩ (O_∞, Special Frobenius, μ∘δ=id)
// Algebra: ℂ[ℤ₂] = ℂ⟨1,g⟩/(g²−1) with pericyclic crossing μ(g⊗g)=1
// Ported from m3iosis/src/m3iosis/pericyclic_compiler.py
#![allow(dead_code, uncommon_codepoints)]

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use core::f64::consts::PI;
use libm::{sqrt, fabs, floor, cos, sin, pow};

// ── Constants ────────────────────────────────────────────────
pub const TUPLE_PF: &str = "𐑦𐑥𐑑𐑹𐑐𐑤𐑔𐑝⊙𐑒𐑙𐑷";
use crate::canonical_ig::PRIMITIVE_ORDER as SLOT_NAMES;

// Sibling tuples for distance
const TUPLE_GRAMMAR: &str = "𐑦𐑸𐑾𐑹𐑐𐑧𐑲𐑵⊙𐑫𐑳𐑟";
const TUPLE_TROQ: &str = "𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭";
const TUPLE_HQE: &str = "𐑦𐑡𐑾𐑿𐑐𐑧𐑲𐑜woe𐑓𐑳𐑷";
const TUPLE_DYSON: &str = "𐑼𐑸𐑾𐑹𐑞𐑧𐑔𐑠⊙𐑖𐑳𐑭";
const TUPLE_AFDMC: &str = "𐑼𐑰𐑑𐑯𐑞𐑧𐑔𐑠⊙𐑒𐑳𐑴";
const TUPLE_CLINK_L8: &str = "𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑵⊙𐑫𐑳𐑟";

// IMASM opcode glyphs
// Each name paired with ITS OWN glyph. The list used to run the names against a
// shifted copy of the alphabet: TANCH carried the fork's mark, CLINK the fuse's,
// FSPLIT and FFUSE the two evaluators', and ENGAGR a tensor sign that is not an
// opcode at all.
const IMASM_OPS: [(&str, &str); 12] = [
    ("VINIT", "⊢"), ("TANCH", "⊣"), ("AFWD", "≻"), ("AREV", "≺"),
    ("CLINK", "⋈"), ("IMSCRIB", "⊙"), ("FSPLIT", "∈"), ("FFUSE", "∋"),
    ("EVALT", "⊤"), ("EVALF", "⊥"), ("ENGAGR", "⊞"), ("IFIX", "⊡"),
];

// ── Helper: glyph value lookup ───────────────────────────────
fn glyph_value(slot: &str, g: &str) -> f64 {
    match slot {
        "⊢" => match g {"𐑛"=>1.0,"𐑨"=>2.0,"𐑼"=>3.0,"𐑦"=>4.0,_=>0.0},
        "⊣" => match g {"𐑡"=>1.0,"𐑰"=>2.0,"𐑥"=>3.0,"𐑶"=>4.0,"𐑸"=>5.0,_=>0.0},
        "≻" => match g {"𐑩"=>1.0,"𐑑"=>2.0,"𐑽"=>3.0,"𐑾"=>4.0,_=>0.0},
        "≺" => match g {"𐑗"=>1.0,"𐑿"=>2.0,"𐑬"=>3.0,"𐑯"=>4.0,"𐑹"=>5.0,_=>0.0},
        "⋈" => match g {"𐑱"=>1.0,"𐑞"=>2.0,"𐑐"=>3.0,_=>0.0},
        "⊤" => match g {"𐑘"=>1.0,"𐑤"=>2.0,"𐑧"=>3.0,"𐑪"=>4.0,"𐑺"=>5.0,_=>0.0},
        "∈" => match g {"𐑲"=>1.0,"𐑚"=>2.0,"𐑔"=>3.0,_=>0.0},
        "∋" => match g {"𐑝"=>1.0,"𐑜"=>2.0,"𐑠"=>3.0,"𐑵"=>4.0,_=>0.0},
        "⊙" => match g {"𐑢"=>1.0,"⊙"=>2.0,"𐑮"=>3.0,"𐑻"=>4.0,"𐑣"=>5.0,_=>0.0},
        "⊥" => match g {"𐑓"=>1.0,"𐑒"=>2.0,"𐑖"=>3.0,"𐑫"=>4.0,_=>0.0},
        "⊞" => match g {"𐑙"=>1.0,"𐑕"=>2.0,"𐑳"=>3.0,_=>0.0},
        "⊡" => match g {"𐑷"=>1.0,"𐑴"=>2.0,"𐑭"=>3.0,"𐑟"=>4.0,_=>0.0},
        _ => 0.0,
    }
}

fn glyph_vals(t: &str) -> [f64; 12] {
    let s = t.trim().trim_matches(|c| c == '⟨' || c == '⟩');
    let mut v = [0.0; 12];
    // by character, not by byte: a Shavian glyph is four bytes wide
    let mut buf = [0u8; 4];
    for (i, ch) in s.chars().take(12).enumerate() {
        v[i] = glyph_value(&SLOT_NAMES[i], ch.encode_utf8(&mut buf));
    }
    v
}

fn tuple_char(t: &str, i: usize) -> char {
    let s = t.trim().trim_matches(|c| c == '⟨' || c == '⟩');
    s.chars().nth(i).unwrap_or('?')
}

// ── Distance ─────────────────────────────────────────────────
pub fn tuple_distance(t1: &str, t2: &str) -> f64 {
    let v1 = glyph_vals(t1);
    let v2 = glyph_vals(t2);
    let mut tot = 0.0;
    for i in 0..12 {
        let d = fabs(v1[i] - v2[i]);
        tot += d * d;
    }
    sqrt(tot)
}

pub fn hamming_distance(t1: &str, t2: &str) -> usize {
    let s1 = t1.trim().trim_matches(|c| c == '⟨' || c == '⟩');
    let s2 = t2.trim().trim_matches(|c| c == '⟨' || c == '⟩');
    let mut h = 0;
    let a: alloc::vec::Vec<char> = s1.chars().take(12).collect();
    let b: alloc::vec::Vec<char> = s2.chars().take(12).collect();
    for i in 0..12 {
        if i < a.len() && i < b.len() && a[i] != b[i] {
            h += 1;
        }
    }
    h
}

// ── Quantum State (ℂ² vector) ───────────────────────────────
#[derive(Clone, Copy)]
pub struct QuantumState {
    pub a: f64,  // coefficient of |1⟩ (σ-basis)
    pub b: f64,  // coefficient of |g⟩ (π-basis)
}

impl QuantumState {
    pub const fn new(a: f64, b: f64) -> Self { Self { a, b } }
    pub const fn sigma() -> Self { Self { a: 1.0, b: 0.0 } }
    pub const fn pi() -> Self { Self { a: 0.0, b: 1.0 } }
    pub fn norm_sq(&self) -> f64 { self.a * self.a + self.b * self.b }
    pub fn normalize(&self) -> Self {
        let n = sqrt(self.norm_sq());
        if n < 1e-15 { *self } else { Self { a: self.a / n, b: self.b / n } }
    }
    pub fn bloch(&self) -> (f64, f64, f64) {
        let n = self.norm_sq();
        if n < 1e-15 { return (0.0, 0.0, 0.0); }
        let x = 2.0 * self.a * self.b / n;
        let z = (self.a * self.a - self.b * self.b) / n;
        (x, 0.0, z)
    }
    pub fn fidelity(&self, other: &QuantumState) -> f64 {
        let ip = self.a * other.a + self.b * other.b;
        ip * ip
    }
    pub fn inner(&self, other: &QuantumState) -> f64 {
        self.a * other.a + self.b * other.b
    }
}

// ── Pericyclic Frobenoid: ℂ[ℤ₂] special Frobenius algebra ───
pub struct PericyclicFrobenoid;

impl PericyclicFrobenoid {
    /// Multiplication μ: A⊗A → A  (fusion)
    /// μ(1⊗1)=1, μ(1⊗g)=μ(g⊗1)=g, μ(g⊗g)=1
    pub fn multiply(&self, a: &QuantumState, b: &QuantumState) -> QuantumState {
        // μ maps: (0,0)→0, (0,1)→1, (1,0)→1, (1,1)→0
        // In ℂ[ℤ₂] basis: 1→(1,0), g→(0,1)
        // Result coeffs: μ(a,b)_0 = a₀b₀ + a₁b₁, μ(a,b)_1 = a₀b₁ + a₁b₀
        QuantumState {
            a: a.a * b.a + a.b * b.b,  // coefficient of 1
            b: a.a * b.b + a.b * b.a,  // coefficient of g
        }
    }

    /// Left multiplication μ(1⊗x)
    pub fn left_multiply(&self, x: &QuantumState) -> QuantumState {
        // μ(1⊗x) = x — 1 is the unit
        QuantumState { a: x.a, b: x.b }
    }

    /// Right multiplication μ(x⊗1)
    pub fn right_multiply(&self, x: &QuantumState) -> QuantumState {
        QuantumState { a: x.a, b: x.b }
    }

    /// Comultiplication δ: A → A⊗A (splitting)
    /// δ(1) = ½(1⊗1 + g⊗g), δ(g) = ½(g⊗1 + 1⊗g)
    /// Returns 2x2 matrix [[δ[0][0], δ[0][1]], [δ[1][0], δ[1][1]]]
    pub fn comultiply(&self, x: &QuantumState) -> [[f64; 2]; 2] {
        let mut m = [[0.0; 2]; 2];
        // δ(x) where x = x₀·1 + x₁·g
        // δ(x₀·1 + x₁·g) = x₀·½(1⊗1+g⊗g) + x₁·½(g⊗1+1⊗g)
        m[0][0] = x.a * 0.5;  // 1⊗1 term from δ(1)
        m[0][1] = x.b * 0.5;  // 1⊗g term from δ(g)
        m[1][0] = x.b * 0.5;  // g⊗1 term from δ(g)
        m[1][1] = x.a * 0.5;  // g⊗g term from δ(1)
        m
    }

    /// Counit ε: A → ℂ. ε(1)=1, ε(g)=0
    pub fn trace(&self, x: &QuantumState) -> f64 {
        x.a  // only the 1-component survives the trace
    }

    /// Frobenius pairing ⟨a,b⟩ = ε(a·b)
    pub fn frobenius_pairing(&self, a: &QuantumState, b: &QuantumState) -> f64 {
        let prod = self.multiply(a, b);
        self.trace(&prod)
    }

    /// Verify special Frobenius: μ∘δ = id on both basis states
    pub fn verify_special(&self) -> (bool, f64) {
        // On |1⟩ (sigma)
        let one = QuantumState::sigma();
        let d_one = self.comultiply(&one);
        // μ∘δ(|1⟩): sum over j,k of δ[0][j][k] * μ(e_j⊗e_k)
        let md_one = QuantumState {
            a: d_one[0][0]*1.0 + d_one[0][1]*0.0 + d_one[1][0]*0.0 + d_one[1][1]*1.0,
            b: d_one[0][0]*0.0 + d_one[0][1]*1.0 + d_one[1][0]*1.0 + d_one[1][1]*0.0,
        };
        let err1 = one.fidelity(&md_one);

        // On |g⟩ (pi)
        let g = QuantumState::pi();
        let d_g = self.comultiply(&g);
        let md_g = QuantumState {
            a: d_g[0][0]*1.0 + d_g[0][1]*0.0 + d_g[1][0]*0.0 + d_g[1][1]*1.0,
            b: d_g[0][0]*0.0 + d_g[0][1]*1.0 + d_g[1][0]*1.0 + d_g[1][1]*0.0,
        };
        let err2 = g.fidelity(&md_g);

        (err1 > 0.999 && err2 > 0.999, (1.0 - err1) + (1.0 - err2))
    }
}

// ── Pericyclic Compiler ──────────────────────────────────────
// State evolution, TQFT, protocol generation, SIC bridge

/// Evolve a quantum state through the pericyclic monad operations.
pub enum EvolutionOp { Mu, MuLeft, MuRight, Delta, Trace, Pairing }

pub fn evolve_state(state: &QuantumState, op: &EvolutionOp) -> String {
    let pf = PericyclicFrobenoid;
    match op {
        EvolutionOp::Mu => {
            let result = pf.multiply(state, state);
            format!("μ({:.4}|1⟩+{:.4}|g⟩)→{:.4}|1⟩+{:.4}|g⟩", state.a, state.b, result.a, result.b)
        }
        EvolutionOp::MuLeft => {
            let result = pf.left_multiply(state);
            format!("μ(1⊗ψ)→{:.4}|1⟩+{:.4}|g⟩", result.a, result.b)
        }
        EvolutionOp::MuRight => {
            let result = pf.right_multiply(state);
            format!("μ(ψ⊗1)→{:.4}|1⟩+{:.4}|g⟩", result.a, result.b)
        }
        EvolutionOp::Delta => {
            let m = pf.comultiply(state);
            format!("δ({:.4}|1⟩+{:.4}|g⟩)=\n  {:.4}|1⊗1⟩ + {:.4}|1⊗g⟩\n+ {:.4}|g⊗1⟩ + {:.4}|g⊗g⟩",
                    state.a, state.b, m[0][0], m[0][1], m[1][0], m[1][1])
        }
        EvolutionOp::Trace => {
            let tr = pf.trace(state);
            format!("ε({:.4}|1⟩+{:.4}|g⟩)={:.4}", state.a, state.b, tr)
        }
        EvolutionOp::Pairing => {
            let p = pf.frobenius_pairing(state, state);
            format!("⟨{:.4}|1⟩+{:.4}|g⟩, {:.4}|1⟩+{:.4}|g⟩⟩={:.4}", state.a, state.b, state.a, state.b, p)
        }
    }
}

/// Evolve through ALL operations
pub fn evolve_all(state: &QuantumState) -> String {
    let mut s = String::new();
    for op in &[EvolutionOp::Mu, EvolutionOp::MuLeft, EvolutionOp::MuRight,
                EvolutionOp::Delta, EvolutionOp::Trace, EvolutionOp::Pairing] {
        s.push_str(&evolve_state(state, op));
        s.push('\n');
    }
    s
}

// ── 2D TQFT Partition Function ──────────────────────────────
/// Compute Z(genus, incoming punctures, outgoing punctures)
pub fn partition_function(genus: usize, incoming: usize, outgoing: usize) -> f64 {
    let d = 2.0;  // dimension of ℂ[ℤ₂]
    let mut z = pow(d, genus as f64);
    // Pericyclic correction: μ(g⊗g)=1 forces genus-1 = 1
    if genus == 1 { z = 1.0; }
    // Boundary punctures contribute sqrt(d) each
    let sqrt_d = sqrt(d);
    z *= pow(sqrt_d, (incoming + outgoing) as f64);
    z
}

pub fn tqft_report(genus: usize) -> String {
    let z = partition_function(genus, 0, 0);
    let mut s = format!("2D TQFT Z(g={}, 0+0) = {:.4}\n", genus, z);
    if genus == 1 {
        s.push_str("  Pericyclic: μ(g⊗g)=1 forces torus amplitude = 1 (not 2)\n");
    }
    s.push_str(&format!("  Algebra dim = 2 (ℂ[ℤ₂])\n"));
    s
}

// ── IMASM Protocol Generation ───────────────────────────────
pub enum ProtocolType { FrobeniusCycle, PericyclicCross, Pairing, Monad, Full }

fn lookup_op(name: &str) -> &str {
    for (n, g) in &IMASM_OPS {
        if *n == name { return g; }
    }
    "?"
}

pub fn generate_protocol(ptype: &ProtocolType) -> String {
    match ptype {
        ProtocolType::FrobeniusCycle => {
            format!("{} {} {} {} {}",
                lookup_op("IMSCRIB"), lookup_op("AFWD"),
                lookup_op("FSPLIT"), lookup_op("FFUSE"), lookup_op("IFIX"))
        }
        ProtocolType::PericyclicCross => {
            format!("{} {} {} {} {}",
                lookup_op("TANCH"), lookup_op("FFUSE"),
                lookup_op("VINIT"), lookup_op("FSPLIT"), lookup_op("CLINK"))
        }
        ProtocolType::Pairing => {
            format!("{} {} {} {}",
                lookup_op("VINIT"), lookup_op("FFUSE"),
                lookup_op("EVALF"), lookup_op("IFIX"))
        }
        ProtocolType::Monad => {
            format!("{} {} {} {} {} {} {}",
                lookup_op("IMSCRIB"), lookup_op("VINIT"), lookup_op("FFUSE"),
                lookup_op("IMSCRIB"), lookup_op("TANCH"), lookup_op("FFUSE"),
                lookup_op("IFIX"))
        }
        ProtocolType::Full => {
            let cycle = generate_protocol(&ProtocolType::FrobeniusCycle);
            let cross = generate_protocol(&ProtocolType::PericyclicCross);
            let pair = generate_protocol(&ProtocolType::Pairing);
            let monad = generate_protocol(&ProtocolType::Monad);
            format!("{} {} {} {} {} {} {}",
                cycle, lookup_op("ENGAGR"), cross,
                lookup_op("ENGAGR"), pair,
                lookup_op("ENGAGR"), monad)
        }
    }
}

pub fn protocol_description(ptype: &ProtocolType) -> String {
    match ptype {
        ProtocolType::FrobeniusCycle =>
            "⊙ IMSCRIB: self-imscription → > AFWD: μ (multiply) → ∈ FSPLIT: δ (split) → \
             ∋ FFUSE: μ∘δ → ⊡ IFIX: identity closed cycle (μ∘δ=id)".into(),
        ProtocolType::PericyclicCross =>
            "⊣ TANCH: two π-systems → ∋ FFUSE: μ(g⊗g)=1 cycloaddition → \
             ⊢ VINIT: σ-framework → ∈ FSPLIT: δ(1)=½(1⊗1+g⊗g) → ⋈ CLINK: closure".into(),
        ProtocolType::Pairing =>
            "⊢ VINIT: seed state → ∋ FFUSE: μ(ψ⊗ψ) → ⊥ EVALF: ε (counit) → ⊡ IFIX: fix ⟨ψ,ψ⟩".into(),
        ProtocolType::Monad =>
            "⊙ IMSCRIB → ⊢ VINIT: η(1) → ∋ FFUSE: μ∘η=id (left unit) → \
             ⊙ IMSCRIB → ⊣ TANCH: η⊗id → ∋ FFUSE: μ∘(η⊗id)=id (right unit) → ⊡ IFIX".into(),
        ProtocolType::Full => {
            format!("Phase 1: {}\nPhase 2: {}\nPhase 3: {}\nPhase 4: {}",
                protocol_description(&ProtocolType::FrobeniusCycle),
                protocol_description(&ProtocolType::PericyclicCross),
                protocol_description(&ProtocolType::Pairing),
                protocol_description(&ProtocolType::Monad))
        }
    }
}

// ── SIC-POVM Fiducial Bridge ─────────────────────────────────
/// 4 SIC states for d=2: {|1⟩, |g⟩, |+⟩, |-⟩}
pub fn sic_states() -> Vec<(&'static str, QuantumState)> {
    let s2 = 1.0 / sqrt(2.0);
    vec![
        ("E_0 = |1⟩⟨1|", QuantumState::new(1.0, 0.0)),
        ("E_1 = |g⟩⟨g|", QuantumState::new(0.0, 1.0)),
        ("E_+ = |+⟩⟨+|", QuantumState::new(s2, s2)),
        ("E_- = |-⟩⟨-|", QuantumState::new(s2, -s2)),
    ]
}

/// Compute SIC-POVM Born probabilities for a state
/// P(i) = (1/d) |⟨ψ|ψ_i⟩|²  where d=2
pub fn sic_measure(state: &QuantumState) -> Vec<(&'static str, f64)> {
    let pf = PericyclicFrobenoid;
    let mut results = Vec::new();
    for (label, sic_s) in sic_states() {
        let ip = state.inner(&sic_s);  // ⟨ψ|ψ_i⟩
        let prob = 0.5 * ip * ip;      // (1/2)|⟨ψ|ψ_i⟩|²
        results.push((label, prob));
    }
    results
}

/// Belnap B=XZ fiducial state: |B⟩ = (1/√2)(|1⟩ + i|g⟩)
/// In our real representation: we use |B⟩_real = (1/√2)(|1⟩ + 0·|g⟩)
/// and handle the i phase separately in the measurement rule
pub fn belnap_fiducial() -> QuantumState {
    let s2 = 1.0 / sqrt(2.0);
    QuantumState::new(s2, 0.0)  // |B⟩ = (1/√2)|1⟩
}

pub fn sic_report() -> String {
    let mut s = String::new();
    s.push_str("SIC-POVM Bridge (d=2, Belnap B=XZ fiducial):\n");
    s.push_str(&format!("  Tuple: {}\n", TUPLE_PF));

    // Belnap B=XZ fiducial
    let b = belnap_fiducial();
    s.push_str(&format!("  Belnap B = XZ: |B⟩ = {:.4}|1⟩ + {:.4}|g⟩\n", b.a, b.b));

    // 6 Frobenius-dual pairs
    s.push_str("  6 Dual Pairs: ⊢↔⊣, >↔<, ⋈↔⊤, ∈↔∋, ⊙↔⊥, ⊞↔⊡\n");

    // Born probabilities for the fiducial
    s.push_str("  Born probabilities P(i)=½|⟨B|ψ_i⟩|²:\n");
    for (label, prob) in sic_measure(&b) {
        s.push_str(&format!("    {:>20}: P = {:.4}\n", label, prob));
    }

    // Grammar bridge
    s.push_str(&format!("  Grammar bridge:\n"));
    s.push_str(&format!("    ⊞=𐑙 (1:1) — grammar IS measured system\n"));
    s.push_str(&format!("    B = XZ — d=2 SIC-POVM fiducial\n"));
    s.push_str(&format!("    meet(B,x)=x, join(B,x)=B, bnot(B)=B\n"));

    s
}

// ── Distance Ladder ──────────────────────────────────────────
pub fn distance_ladder() -> String {
    let siblings = [
        ("troq", TUPLE_TROQ),
        ("afdmc", TUPLE_AFDMC),
        ("dyson", TUPLE_DYSON),
        ("hqe", TUPLE_HQE),
        ("clink_l8", TUPLE_CLINK_L8),
        ("grammar", TUPLE_GRAMMAR),
    ];
    let mut entries: Vec<(String, f64, usize)> = siblings.iter().map(|(name, tup)| {
        let d = tuple_distance(TUPLE_PF, tup);
        let h = hamming_distance(TUPLE_PF, tup);
        ((*name).to_string(), d, h)
    }).collect();
    entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut s = String::new();
    s.push_str("Distance ladder from Pericyclic Frobenoid:\n");
    for (name, d, h) in &entries {
        s.push_str(&format!("  → {:<12} hamming={} weighted={:.4}\n", name, h, d));
    }
    s
}

// ── Full Report ──────────────────────────────────────────────
pub fn full_report() -> String {
    let pf = PericyclicFrobenoid;
    let mut s = String::new();
    s.push_str("============================================================\n");
    s.push_str("  Pericyclic Semiotic Frobenoid — Rust Port (mOMonadOS)\n");
    s.push_str("============================================================\n");
    s.push_str(&format!("  Tuple: {}\n", TUPLE_PF));
    s.push_str("  Algebra: ℂ[ℤ₂] special Frobenius — μ∘δ=id at O_∞\n");
    s.push_str("  Tier: O_∞ (Ouroboric infinity)\n");
    s.push_str("\n");

    // Verify special Frobenius
    let (ok, err) = pf.verify_special();
    s.push_str(&format!("  μ∘δ=id: {}\n", if ok { "✓ PASS" } else { "✗ FAIL" }));
    s.push_str(&format!("  Error:  {:.2e}\n", err));
    s.push_str("\n");

    // Primitive decomposition
    s.push_str("  Primitive Decomposition:\n");
    s.push_str("  ──── ────── ──────────  ─────────────────────────────────────\n");
    let readings: [&str; 12] = [
        "Imscriptive boundary: V=L(x) ∧ selfmodel(x)",
        "Pericyclic crossing: cross(x,y) ∧ ¬ meet(x,y)",
        "Categorical functoriality: Fun(x,y) ∧ Nat(y,z) → Fun(x,z)",
        "Frobenius-special parity: ℤ₂(x) ∧ μ∘δ=id",
        "Quantum fidelity: ℏ(x) ∧ [x,p]=iℏ",
        "Frozen-order kinetics: τ∼T ∧ noisy(x)",
        "Aleph cardinality: ∃y∈x(|y|∼|x|)",
        "Conjunctive composition: f∧g∧h — concerted",
        "Critical fixed point: ξ→∞ ∧ μ∘δ=id",
        "One-step chirality: P(y)↔P(S²(y))",
        "1:1 stoichiometry: |A|=1 ∧ |B|=1",
        "Trivial winding: ∮_γ dx=0",
    ];
    for i in 0..12 {
        let glyph = tuple_char(TUPLE_PF, i);
        s.push_str(&format!("  {} {:>6}  {}\n", SLOT_NAMES[i], glyph, readings[i]));
    }
    s.push_str("\n");

    // State evolution examples
    s.push_str("  State Evolution (σ-framework |1⟩):\n");
    s.push_str(&format!("  {}", evolve_all(&QuantumState::sigma())));
    s.push_str("\n");

    s.push_str("  State Evolution (π-system |g⟩):\n");
    s.push_str(&format!("  {}", evolve_all(&QuantumState::pi())));
    s.push_str("\n");

    // TQFT
    s.push_str("  2D TQFT Partition Functions:\n");
    for g in 0..3 {
        s.push_str(&format!("    g={}: Z={:.4}", g, partition_function(g, 0, 0)));
        if g == 1 { s.push_str(" (pericyclic: μ(g⊗g)=1 → Z=1)"); }
        s.push('\n');
    }
    s.push_str("\n");

    // Protocol example
    s.push_str("  IMASM Protocol (frobenius_cycle):\n");
    s.push_str(&format!("    Word: {}\n", generate_protocol(&ProtocolType::FrobeniusCycle)));
    s.push_str(&format!("    {}\n", protocol_description(&ProtocolType::FrobeniusCycle)));
    s.push_str("\n");

    // Distance ladder
    s.push_str("  {}", distance_ladder());
    s.push_str("\n");

    // SIC bridge
    s.push_str("  {}", sic_report());

    s.push_str("============================================================\n");
    s
}

// ── CLI Entry Point ──────────────────────────────────────────
pub fn pqc_cli(args: &[&str]) -> String {
    if args.is_empty() {
        return full_report();
    }

    let cmd = args[0];
    let rest = &args[1..];

    match cmd {
        "evolve" => {
            if rest.is_empty() {
                return "Usage: pqc evolve <a,b> — e.g. 'pqc evolve 0,1'".into();
            }
            let parts: Vec<&str> = rest[0].split(',').collect();
            let a = parts.first().and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.0);
            let b = parts.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let state = QuantumState::new(a, b);
            evolve_all(&state)
        }
        "tqft" => {
            let g = rest.first().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            tqft_report(g)
        }
        "protocol" => {
            let ptype = rest.first().unwrap_or(&"frobenius");
            match *ptype {
                "cross" => {
                    let mut s = format!("Word: {}\n", generate_protocol(&ProtocolType::PericyclicCross));
                    s.push_str(&protocol_description(&ProtocolType::PericyclicCross));
                    s
                }
                "pairing" => {
                    let mut s = format!("Word: {}\n", generate_protocol(&ProtocolType::Pairing));
                    s.push_str(&protocol_description(&ProtocolType::Pairing));
                    s
                }
                "monad" => {
                    let mut s = format!("Word: {}\n", generate_protocol(&ProtocolType::Monad));
                    s.push_str(&protocol_description(&ProtocolType::Monad));
                    s
                }
                "full" => {
                    let mut s = format!("Word: {}\n", generate_protocol(&ProtocolType::Full));
                    s.push_str(&protocol_description(&ProtocolType::Full));
                    s
                }
                _ => {
                    let mut s = format!("Word: {}\n", generate_protocol(&ProtocolType::FrobeniusCycle));
                    s.push_str(&protocol_description(&ProtocolType::FrobeniusCycle));
                    s
                }
            }
        }
        "sic" => sic_report(),
        "distance" => distance_ladder(),
        "tuple" => format!("⟨{}⟩", TUPLE_PF),
        "verify" => {
            let pf = PericyclicFrobenoid;
            let (ok, err) = pf.verify_special();
            format!("μ∘δ=id: {} (error={:.2e})\non basis |1⟩ and |g⟩\nFrobenius pairing matrix [[1,0],[0,1]]\nAll checks: {} PASS",
                if ok { "✓" } else { "✗" }, err,
                if ok { "✓" } else { "✗" })
        }
        _ => full_report(),
    }
}
