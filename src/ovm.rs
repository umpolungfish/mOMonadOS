// ovm.rs — Operator-Valued Measure Computation Tools
// Native bare-metal implementation for mOMonadOS.
// 
// COMPUTATIONAL TOOLS for quantum measurement operators.
// No taxonomy. No catalog. Just math.
//
// Bloch vector representation: E = (tr/2) I + (r/2) Σ n_i σ_i
// where r = bloch_norm, n = bloch_vec (unit), σ_i are Pauli matrices.
// d=2 constructions use Bloch-vector representation with SO(3) rotation.
//
// Author: Math⊙perator (Lando⊗⊙perator team)
// Date: 2026-07-31 (rewritten as computation tools)

#![allow(dead_code)]
#![allow(uncommon_codepoints)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

#[derive(Copy, Clone, Debug)]
pub struct BlochVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BlochVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self { BlochVec { x, y, z } }

    pub fn norm(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> BlochVec {
        let n = self.norm();
        BlochVec { x: self.x / n, y: self.y / n, z: self.z / n }
    }

    pub fn dot(&self, other: &BlochVec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Rotate around z-axis by angle theta (radians).
    pub fn rot_z(&self, theta: f64) -> BlochVec {
        let ct = libm::cos(theta);
        let st = libm::sin(theta);
        BlochVec {
            x: self.x * ct - self.y * st,
            y: self.x * st + self.y * ct,
            z: self.z,
        }
    }

    /// Rotate by SO(3) matrix with three Euler angles.
    pub fn rot_so3(&self, a: f64, b: f64, c: f64) -> BlochVec {
        let v = self.rot_z(c);
        let ct = libm::cos(b);
        let st = libm::sin(b);
        let vy = v.y * ct - v.z * st;
        let vz = v.y * st + v.z * ct;
        let v = BlochVec { x: v.x, y: vy, z: vz };
        v.rot_z(a)
    }

    /// Scale Bloch vector by factor.
    pub fn scale(&self, s: f64) -> BlochVec {
        BlochVec { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}

/// A qubit operator E = (tr/2)·I + n·v·σ, stored as (trace_coeff, bloch_norm, bloch_vec).
#[derive(Copy, Clone, Debug)]
pub struct QubitOp {
    pub trace_coeff: f64,
    pub bloch_norm: f64,
    pub bloch_vec: BlochVec,
}

impl QubitOp {
    /// Eigenvalues of E: [tr/2 + n/2, tr/2 - n/2].
    pub fn eigenvalues(&self) -> (f64, f64) {
        let half_tr = self.trace_coeff / 2.0;
        let half_n = self.bloch_norm / 2.0;
        (half_tr + half_n, half_tr - half_n)
    }

    /// True if both eigenvalues ≥ -1e-9 (positive semidefinite).
    pub fn is_positive(&self) -> bool {
        let (l1, l2) = self.eigenvalues();
        l1 >= -1e-9 && l2 >= -1e-9
    }

    /// True if at least one eigenvalue < 0 (NOVM).
    pub fn is_negative(&self) -> bool {
        let (_, l2) = self.eigenvalues();
        l2 < -1e-9
    }

    /// Hilbert-Schmidt inner product with another operator.
    /// ⟨⟨E|F⟩⟩ = Tr(E F) = (tr_E·tr_F)/4 + (n_E·n_F)·v_E·v_F / 2
    pub fn hs_inner(&self, other: &QubitOp) -> f64 {
        let trace_term = self.trace_coeff * other.trace_coeff / 4.0;
        let bloch_term = self.bloch_norm * other.bloch_norm * self.bloch_vec.dot(&other.bloch_vec) / 2.0;
        trace_term + bloch_term
    }
}
// ═══════════════════════════════════════════════════════════════
// OVM D=2 CONSTRUCTION FUNCTIONS
// ═══════════════════════════════════════════════════════════════

/// Build SIC-POVM operators for d=2: 4 equiangular Bloch vectors forming a regular tetrahedron.
pub fn construct_sic_povm_d2() -> [QubitOp; 4] {
    let r3 = libm::sqrt(3.0);
    let n = 1.0 / r3;
    let tr = 0.5;
    let vertices = [
        BlochVec::new(1.0, 1.0, 1.0),
        BlochVec::new(1.0, -1.0, -1.0),
        BlochVec::new(-1.0, 1.0, -1.0),
        BlochVec::new(-1.0, -1.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build SIC-NOVM: tetrahedron with Bloch norms > 1/2 for negative eigenvalues.
pub fn construct_sic_novm_d2() -> [QubitOp; 4] {
    let r3 = libm::sqrt(3.0);
    let n = 0.693;
    let tr = 0.5;
    let vertices = [
        BlochVec::new(1.0, 1.0, 1.0), BlochVec::new(1.0, -1.0, -1.0),
        BlochVec::new(-1.0, 1.0, -1.0), BlochVec::new(-1.0, -1.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build SIC-NPOVM: tetrahedral geometry with partial positivity Φ=𐑬 and Z₂ winding Ω=𐑴.
/// Same Bloch norm as SIC-POVM, but with two-step chirality Ħ=𐑖 and disjunctive composition.
pub fn construct_sic_npovm_d2() -> [QubitOp; 4] {
    let r3 = libm::sqrt(3.0);
    let n = 1.0 / r3;
    let tr = 0.5;
    // Tetrahedron with z-reflection for NPOVM character — partial positivity
    let vertices = [
        BlochVec::new(1.0, 1.0, 1.0),
        BlochVec::new(1.0, -1.0, -1.0),
        BlochVec::new(-1.0, 1.0, 1.0),    // z-flip relative to SIC
        BlochVec::new(-1.0, -1.0, -1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build antisymmetric IC POVM (ℤ₂ pairing, X = σ_z conjugation).
pub fn construct_aminus_ic_povm_d2() -> [QubitOp; 4] {
    let r3 = libm::sqrt(3.0);
    let n = 1.0 / r3;
    let tr = 0.5;
    let vertices = [
        BlochVec::new(1.0, 1.0, 1.0),       // E₀₀
        BlochVec::new(-1.0, -1.0, 1.0),      // E₁₀ = σ_z(E₀₀)
        BlochVec::new(1.0, -1.0, -1.0),       // E₀₁
        BlochVec::new(-1.0, 1.0, -1.0),       // E₁₁ = σ_z(E₀₁)
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build asymmetric IC POVM: unequal Bloch norms, no Clifford grading.
pub fn construct_ai_cpovm_d2() -> [QubitOp; 4] {
    let norms = [0.3, 0.4, 0.5, 0.6];
    let traces = [0.4, 0.5, 0.5, 0.6];
    let vertices = [
        BlochVec::new(1.0, 0.0, 0.0),
        BlochVec::new(-0.5, 0.8660254037844386, 0.0),
        BlochVec::new(-0.5, -0.8660254037844386, 0.0),
        BlochVec::new(0.0, 0.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        ops[i] = QubitOp { trace_coeff: traces[i], bloch_norm: norms[i], bloch_vec: vertices[i] };
    }
    ops
}

/// Build AI-CNOVM: asymmetric IC with negative eigenvalues.
pub fn construct_ai_cnovm_d2() -> [QubitOp; 4] {
    let norms = [0.65, 0.75, 0.85, 0.95];
    let traces = [0.4, 0.5, 0.5, 0.6];
    let vertices = [
        BlochVec::new(1.0, 0.0, 0.0),
        BlochVec::new(-0.5, 0.8660254037844386, 0.0),
        BlochVec::new(-0.5, -0.8660254037844386, 0.0),
        BlochVec::new(0.0, 0.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        ops[i] = QubitOp { trace_coeff: traces[i], bloch_norm: norms[i], bloch_vec: vertices[i] };
    }
    ops
}

/// Build AI-NOVM: asymmetric PC negative static (information-incomplete, m=3<4).
pub fn construct_ai_novm_d2() -> [QubitOp; 3] {
    let norms = [0.65, 0.75, 0.95];
    let traces = [0.4, 0.6, 1.0];
    let vertices = [
        BlochVec::new(1.0, 0.0, 0.0),
        BlochVec::new(-0.5, 0.8660254037844386, 0.0),
        BlochVec::new(-0.5, -0.8660254037844386, 0.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        ops[i] = QubitOp { trace_coeff: traces[i], bloch_norm: norms[i], bloch_vec: vertices[i] };
    }
    ops
}
/// Build S-PC-POVM: 3 operators (paracomplete), equiangular.
/// Updated per grid: Φ=𐑬 (partial parity), ɢ=𐑜 (disjunctive), Γ=𐑔 (aleph).
/// m=3 < d²=4, frame is anisotropic but carries aleph cardinality.
pub fn construct_s_pc_povm_d2() -> [QubitOp; 3] {
    let n = 1.0 / libm::sqrt(3.0);
    let tr = 2.0 / 3.0;
    let vertices = [
        BlochVec::new(0.0, 0.0, 1.0),
        BlochVec::new(2.0 * libm::sqrt(2.0) / 3.0, 0.0, -1.0/3.0),
        BlochVec::new(-libm::sqrt(2.0)/3.0, libm::sqrt(6.0)/3.0, -1.0/3.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build S-PC-NOVM: 3 operators with negative eigenvalues, equiangular, disjunctive.
pub fn construct_s_pc_novm_d2() -> [QubitOp; 3] {
    let n = 0.693;
    let tr = 2.0 / 3.0;
    let vertices = [
        BlochVec::new(0.0, 0.0, 1.0),
        BlochVec::new(2.0 * libm::sqrt(2.0) / 3.0, 0.0, -1.0/3.0),
        BlochVec::new(-libm::sqrt(2.0)/3.0, libm::sqrt(6.0)/3.0, -1.0/3.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build A⁻-PC-POVM: antisymmetric paracomplete positive — ℤ₂ pairing, m=3, disjunctive composition.
pub fn construct_aminus_pc_povm_d2() -> [QubitOp; 3] {
    let n = 1.0 / libm::sqrt(3.0);
    let tr = 2.0 / 3.0;
    // σ_z-conjugate pairs embedded in 3-operator set
    let vertices = [
        BlochVec::new(0.0, 0.0, 1.0),          // invariant under σ_z
        BlochVec::new(1.0, 1.0, -0.5),          // E₀₀
        BlochVec::new(-1.0, -1.0, -0.5),         // E₁₀ = σ_z(E₀₀)
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build A⁻-PC-NOVM: antisymmetric paracomplete negative with disjunctive composition.
pub fn construct_aminus_pc_novm_d2() -> [QubitOp; 3] {
    let n = 0.693;
    let tr = 2.0 / 3.0;
    let vertices = [
        BlochVec::new(0.0, 0.0, 1.0),
        BlochVec::new(1.0, 1.0, -0.5),
        BlochVec::new(-1.0, -1.0, -0.5),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build A-PC-POVM (grid variant): 3 operators, asymmetric Bloch norms, local completeness Γ=𐑲.
pub fn construct_a_pc_povm_d2() -> [QubitOp; 3] {
    let norms = [0.3, 0.5, 0.7];
    let traces = [0.4, 0.6, 1.0];
    let vertices = [
        BlochVec::new(1.0, 0.0, 0.0),
        BlochVec::new(-0.5, 0.8660254037844386, 0.0),
        BlochVec::new(-0.5, -0.8660254037844386, 0.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        ops[i] = QubitOp { trace_coeff: traces[i], bloch_norm: norms[i], bloch_vec: vertices[i] };
    }
    ops
}

/// Build A-PC-POVM† (HTML variant): bidirectional coupling Ř=𐑾, mesoscale Γ=𐑚, broadcast ɢ=𐑵.
pub fn construct_a_pc_povm_dagger_d2() -> [QubitOp; 3] {
    let norms = [0.35, 0.55, 0.65];
    let traces = [0.5, 0.6, 0.9];
    let vertices = [
        BlochVec::new(1.0, 0.0, 0.0),
        BlochVec::new(-0.5, 0.8660254037844386, 0.0),
        BlochVec::new(0.0, 0.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        ops[i] = QubitOp { trace_coeff: traces[i], bloch_norm: norms[i], bloch_vec: vertices[i] };
    }
    ops
}
// ═══════════════════════════════════════════════════════════════
// ═══════════════════════════════════════════════════════════════
// SUSY OVM CONSTRUCTORS — Mirror parents with SUSY symmetry class
// ═══════════════════════════════════════════════════════════════

/// Build SUSY-IC-POVM: tetrahedral IC POVM with SUSY symmetry.
/// Mirrors SIC-POVM (same geometry, trace=0.5, norm=1/√3).
pub fn construct_susy_ic_povm_d2() -> [QubitOp; 4] {
    let r3 = libm::sqrt(3.0);
    let n = 1.0 / r3;
    let tr = 0.5;
    let vertices = [
        BlochVec::new(1.0, 1.0, 1.0),
        BlochVec::new(1.0, -1.0, -1.0),
        BlochVec::new(-1.0, 1.0, -1.0),
        BlochVec::new(-1.0, -1.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build SUSY-IC-NOVM: tetrahedral IC NOVM with SUSY symmetry.
/// Mirrors SIC-NOVM (same geometry, trace=0.5, norm=0.693).
pub fn construct_susy_ic_novm_d2() -> [QubitOp; 4] {
    let n = 0.693;
    let tr = 0.5;
    let vertices = [
        BlochVec::new(1.0, 1.0, 1.0), BlochVec::new(1.0, -1.0, -1.0),
        BlochVec::new(-1.0, 1.0, -1.0), BlochVec::new(-1.0, -1.0, 1.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 4];
    for i in 0..4 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build SUSY-PC-POVM: 3-operator paracomplete POVM with SUSY symmetry.
/// Mirrors S-PC-POVM (equiangular, m=3, trace=2/3, norm=1/√3).
pub fn construct_susy_pc_povm_d2() -> [QubitOp; 3] {
    let n = 1.0 / libm::sqrt(3.0);
    let tr = 2.0 / 3.0;
    let vertices = [
        BlochVec::new(0.0, 0.0, 1.0),
        BlochVec::new(2.0 * libm::sqrt(2.0) / 3.0, 0.0, -1.0/3.0),
        BlochVec::new(-libm::sqrt(2.0)/3.0, libm::sqrt(6.0)/3.0, -1.0/3.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}

/// Build SUSY-PC-NOVM: 3-operator paracomplete NOVM with SUSY symmetry.
/// Mirrors S-PC-NOVM (equiangular, m=3, trace=2/3, norm=0.693).
pub fn construct_susy_pc_novm_d2() -> [QubitOp; 3] {
    let n = 0.693;
    let tr = 2.0 / 3.0;
    let vertices = [
        BlochVec::new(0.0, 0.0, 1.0),
        BlochVec::new(2.0 * libm::sqrt(2.0) / 3.0, 0.0, -1.0/3.0),
        BlochVec::new(-libm::sqrt(2.0)/3.0, libm::sqrt(6.0)/3.0, -1.0/3.0),
    ];
    let mut ops = [QubitOp { trace_coeff: 0.0, bloch_norm: 0.0, bloch_vec: BlochVec::new(0.0,0.0,0.0) }; 3];
    for i in 0..3 {
        let norm = vertices[i].norm();
        ops[i] = QubitOp { trace_coeff: tr, bloch_norm: n,
            bloch_vec: BlochVec::new(vertices[i].x / norm, vertices[i].y / norm, vertices[i].z / norm) };
    }
    ops
}
// PROPERTY VERIFICATION
// ═══════════════════════════════════════════════════════════════

/// Check equiangularity: |⟨ψ_i|ψ_j⟩|² = const for all i≠j.
pub fn check_equiangularity(ops: &[QubitOp]) -> (bool, f64, f64) {
    if ops.len() < 2 { return (true, 0.0, 0.0); }
    let ref_val = ops[0].hs_inner(&ops[1]);
    let mut min_val = ref_val;
    let mut max_val = ref_val;
    let mut all_eq = true;
    for i in 0..ops.len() {
        for j in (i+1)..ops.len() {
            let val = ops[i].hs_inner(&ops[j]);
            if val < min_val { min_val = val; }
            if val > max_val { max_val = val; }
            if (val - ref_val).abs() > 1e-6 { all_eq = false; }
        }
    }
    (all_eq, min_val, max_val)
}

/// Check positivity: all eigenvalues ≥ 0.
pub fn check_positivity(ops: &[QubitOp]) -> (bool, usize, usize) {
    let mut n_pos = 0usize;
    let mut n_neg = 0usize;
    for op in ops {
        if op.is_positive() { n_pos += 1; } else { n_neg += 1; }
    }
    (n_neg == 0, n_pos, n_neg)
}

/// Check sum-to-identity: Σ E_i = I (for d=2, Σ tr = d = 2).
pub fn check_sum_to_i(ops: &[QubitOp]) -> (bool, f64) {
    let sum_tr: f64 = ops.iter().map(|op| op.trace_coeff).sum();
    (libm::fabs(sum_tr - 2.0) < 1e-6, sum_tr)
}

/// Check IC rank: number of linearly independent operators.
pub fn check_ic_rank(ops: &[QubitOp]) -> usize {
    if ops.len() >= 4 { 4 } else { ops.len() }
}

/// Format an operator spectrum as string.
pub fn format_spectrum(ops: &[QubitOp]) -> String {
    let mut out = String::new();
    for (i, op) in ops.iter().enumerate() {
        let (l1, l2) = op.eigenvalues();
        out.push_str(&format!("  E_{}: [{:.6}, {:.6}]", i, l1, l2));
        if l2 < 0.0 { out.push_str(" ✗ (NEGATIVE)"); }
        if l2 >= -1e-9 && l2 <= 1e-9 { out.push_str(" (boundary)"); }
        out.push('\n');
    }
    out
}

/// Compute frame eigenvalues for completeness analysis.
pub fn compute_frame_evals(ops: &[QubitOp]) -> [f64; 4] {
    let mut f = [[0.0f64; 4]; 4];
    for op in ops {
        let v = [op.trace_coeff / 2.0, op.bloch_norm * op.bloch_vec.x,
                 op.bloch_norm * op.bloch_vec.y, op.bloch_norm * op.bloch_vec.z];
        for i in 0..4 {
            for j in 0..4 { f[i][j] += v[i] * v[j]; }
        }
    }
    let mut evals = [0.0f64; 4];
    for i in 0..4 { evals[i] = f[i][i]; }
    evals
}

// ═══════════════════════════════════════════════════════════════
// TIME EVOLUTION — Oscillating OVM types
// ═══════════════════════════════════════════════════════════════

/// Apply SO(3) time evolution with incommensurate frequencies.
pub fn evolve_ops(ops: &[QubitOp], t: f64) -> Vec<QubitOp> {
    let a = t;
    let b = t * libm::sqrt(2.0);
    let c = t * libm::sqrt(3.0);
    ops.iter().map(|op| QubitOp {
        trace_coeff: op.trace_coeff,
        bloch_norm: op.bloch_norm,
        bloch_vec: op.bloch_vec.rot_so3(a, b, c),
    }).collect()
}

/// Apply σ_z-compatible time evolution (for antisymmetric types).
pub fn evolve_ops_z(ops: &[QubitOp], t: f64) -> Vec<QubitOp> {
    let omega = t * (libm::sqrt(2.0) - 1.0);
    ops.iter().map(|op| QubitOp {
        trace_coeff: op.trace_coeff,
        bloch_norm: op.bloch_norm,
        bloch_vec: op.bloch_vec.rot_z(omega),
    }).collect()
}
// ═══════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════
// TOOLS — Name dispatch + computation reports
// ═══════════════════════════════════════════════════════════════

/// String-based dispatch: resolve a name to a canonical operator set.
/// No enum. No taxonomy. Just the constructors.
pub fn ops_by_name(name: &str) -> Option<Vec<QubitOp>> {
    let n = name.to_lowercase().replace('_', "-");
    match n.as_str() {
        // ── SIC (tetrahedral, IC, 4 ops) ──
        "sic-povm" => Some(construct_sic_povm_d2().to_vec()),
        "sic-novm" => Some(construct_sic_novm_d2().to_vec()),
        "sic-npovm" => Some(construct_sic_npovm_d2().to_vec()),

        // ── Antisymmetric IC ──
        "a-minus-ic-povm" => Some(construct_aminus_ic_povm_d2().to_vec()),
        "a-minus-ic-novm" => {
            let mut ops = construct_aminus_ic_povm_d2().to_vec();
            for op in &mut ops { op.bloch_norm = 0.693; }
            Some(ops)
        }

        // ── Asymmetric IC ──
        "ai-cpovm" => Some(construct_ai_cpovm_d2().to_vec()),
        "ai-cnovm" => Some(construct_ai_cnovm_d2().to_vec()),

        // ── Symmetric PC ──
        "s-pc-povm" => Some(construct_s_pc_povm_d2().to_vec()),
        "s-pc-novm" => Some(construct_s_pc_novm_d2().to_vec()),

        // ── Antisymmetric PC ──
        "a-minus-pc-povm" => Some(construct_aminus_pc_povm_d2().to_vec()),
        "a-minus-pc-novm" => Some(construct_aminus_pc_novm_d2().to_vec()),

        // ── Asymmetric PC ──
        "a-pc-povm" => Some(construct_a_pc_povm_d2().to_vec()),
        "a-pc-povm-dagger" | "a-pc-povm†" => Some(construct_a_pc_povm_dagger_d2().to_vec()),
        "ai-novm" => Some(construct_ai_novm_d2().to_vec()),

        // ── SUSY IC ──
        "susy-ic-povm" => Some(construct_susy_ic_povm_d2().to_vec()),
        "susy-ic-novm" => Some(construct_susy_ic_novm_d2().to_vec()),

        // ── SUSY PC ──
        "susy-pc-povm" => Some(construct_susy_pc_povm_d2().to_vec()),
        "susy-pc-novm" => Some(construct_susy_pc_novm_d2().to_vec()),

        _ => None,
    }
}

/// List all known operator set names.
pub fn ops_names() -> &'static [&'static str] {
    &[
        "sic-povm", "sic-novm", "sic-npovm",
        "a-minus-ic-povm", "a-minus-ic-novm",
        "ai-cpovm", "ai-cnovm",
        "s-pc-povm", "s-pc-novm",
        "a-minus-pc-povm", "a-minus-pc-novm",
        "a-pc-povm", "a-pc-povm-dagger", "ai-novm",
        "susy-ic-povm", "susy-ic-novm",
        "susy-pc-povm", "susy-pc-novm",
    ]
}

/// Compute eigenvalues of a single qubit operator from Bloch parameters.
/// E = (tr/2)I + (r/2)(n_x σ_x + n_y σ_y + n_z σ_z)
/// λ± = tr/2 ± r/2   (for unit Bloch vector, eigenvalues of n·σ are ±1)
pub fn compute_eigenvalues(_bloch_x: f64, _bloch_y: f64, _bloch_z: f64, bloch_norm: f64, trace: f64) -> (f64, f64) {
    let half_trace = trace / 2.0;
    let half_norm = bloch_norm / 2.0;
    (half_trace + half_norm, half_trace - half_norm)
}

/// Compute the Hilbert-Schmidt overlap matrix G_ij = Tr(E_i E_j).
pub fn overlap_matrix(ops: &[QubitOp]) -> Vec<Vec<f64>> {
    let m = ops.len();
    let mut g = Vec::with_capacity(m);
    for i in 0..m {
        let mut row = Vec::with_capacity(m);
        for j in 0..m {
            row.push(ops[i].hs_inner(&ops[j]));
        }
        g.push(row);
    }
    g
}

/// Compute the full frame operator S = Σ_i |E_i⟩⟩⟨⟨E_i|.
/// Returns the 4×4 matrix in row-major order for d=2.
/// Vectorization: |E⟩⟩ = [tr(E)/√2, r·n_x/√2, r·n_y/√2, r·n_z/√2] in the Pauli basis.
pub fn frame_operator_matrix(ops: &[QubitOp]) -> [[f64; 4]; 4] {
    let mut s = [[0.0f64; 4]; 4];
    for op in ops {
        let v = [op.trace_coeff / libm::sqrt(2.0),
                 op.bloch_norm * op.bloch_vec.x / libm::sqrt(2.0),
                 op.bloch_norm * op.bloch_vec.y / libm::sqrt(2.0),
                 op.bloch_norm * op.bloch_vec.z / libm::sqrt(2.0)];
        for i in 0..4 {
            for j in 0..4 {
                s[i][j] += v[i] * v[j];
            }
        }
    }
    s
}

/// Diagonal approximation to frame eigenvalues (the diagonal of S in Pauli basis).
/// For SIC-POVM at d=2: should be [1, 1/3, 1/3, 1/3].
pub fn frame_eigenvalues(ops: &[QubitOp]) -> [f64; 4] {
    compute_frame_evals(ops)
}

/// Construct the Belnap B = XZ fiducial projector for d=2.
/// B = |ψ⟩⟨ψ| where |ψ⟩ is the Hoggar SIC fiducial (Belnap state).
/// Returns [tr, r_x, r_y, r_z] in Bloch representation.
pub fn belnap_b_xz_bloch() -> [f64; 4] {
    // Belnap B = XZ is the Weyl-Heisenberg fiducial for d=2 SIC-POVM.
    // Bloch vector: (1,1,1)/√3, trace=1, norm=1/√3
    let n = 1.0 / libm::sqrt(3.0);
    [1.0, n, n, n]
}

/// Construct the Belnap B = XZ pure state as a QubitOp.
pub fn construct_belnap_b_xz() -> QubitOp {
    let n = 1.0 / libm::sqrt(3.0);
    QubitOp {
        trace_coeff: 1.0,
        bloch_norm: n,
        bloch_vec: BlochVec::new(n, n, n).normalize(),
    }
}

/// Construct a pure-state projector from Bloch vector direction.
/// Pure state: tr=1, r=1 (norm=1), direction = unit vector (x,y,z).
pub fn construct_pure_projector(bloch_x: f64, bloch_y: f64, bloch_z: f64) -> QubitOp {
    let v = BlochVec::new(bloch_x, bloch_y, bloch_z);
    let n = v.norm();
    QubitOp {
        trace_coeff: 1.0,
        bloch_norm: 1.0,
        bloch_vec: BlochVec::new(bloch_x / n, bloch_y / n, bloch_z / n),
    }
}

// ═══════════════════════════════════════════════════════════════
// COMPUTATION REPORTS — what the REPL calls
// ═══════════════════════════════════════════════════════════════

/// Full computation report for a named operator set.
/// Computes: eigenvalues, overlap matrix, equiangularity, positivity,
/// sum-to-I, IC rank, frame eigenvalues.
pub fn ovm_compute(name: &str) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => {
            let mut out = String::new();
            out.push_str(&format!("Unknown operator set: '{}'\n\n", name));
            out.push_str("Known sets:\n");
            for n in ops_names() {
                out.push_str(&format!("  {}\n", n));
            }
            out.push_str("\nUsage: ovm <name>         — full computation report\n");
            out.push_str("       ovm eigen <x> <y> <z> <norm> <trace> — eigenvalue\n");
            out.push_str("       ovm frame <name>    — frame operator\n");
            out.push_str("       ovm overlap <name>  — HS overlap matrix\n");
            out.push_str("       ovm belnap          — Belnap B=XZ fiducial\n");
            out.push_str("       ovm help            — this help\n");
            return out;
        }
    };

    let m = ops.len();
    let mut out = String::new();
    out.push_str(&format!("═══ OVM Compute: {} ═══\n", name));
    out.push_str(&format!("Operators: m={}\n\n", m));

    // ── Eigenvalues ──
    out.push_str("── Eigenvalues ──\n");
    for (i, op) in ops.iter().enumerate() {
        let (l1, l2) = op.eigenvalues();
        let flag = if l2 < -1e-9 { " ✗ NEGATIVE" } else if l2 < 1e-9 { " (boundary)" } else { "" };
        out.push_str(&format!("  E_{}: λ₁={:.6}  λ₂={:.6}{}\n", i, l1, l2, flag));
    }

    // ── Overlap Matrix ──
    out.push_str("\n── HS Overlap Matrix G_ij = Tr(E_i E_j) ──\n");
    let g = overlap_matrix(&ops);
    for i in 0..m {
        out.push_str("  [");
        for j in 0..m {
            if j > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", g[i][j]));
        }
        out.push_str("]\n");
    }

    // ── Equiangularity ──
    let (eq, min_ov, max_ov) = check_equiangularity(&ops);
    out.push_str(&format!("\n── Equiangularity ──\n"));
    out.push_str(&format!("  Equiangular: {}  (off-diagonal range: [{:.6}, {:.6}])\n", eq, min_ov, max_ov));

    // ── Positivity ──
    let (pos, n_pos, n_neg) = check_positivity(&ops);
    out.push_str(&format!("\n── Positivity ──\n"));
    out.push_str(&format!("  All ≥ 0: {}  ({}/{} positive, {}/{} negative)\n", pos, n_pos, m, n_neg, m));

    // ── Completeness ──
    let (sum_ok, sum_tr) = check_sum_to_i(&ops);
    out.push_str(&format!("\n── Completeness (Σ tr = d = 2) ──\n"));
    out.push_str(&format!("  Σ tr = {:.6}  (target: 2.0)  pass: {}\n", sum_tr, sum_ok));

    // ── IC Rank ──
    let rank = check_ic_rank(&ops);
    out.push_str(&format!("\n── IC Rank ──\n"));
    out.push_str(&format!("  Rank: {}  (d²=4 for full IC, <4 = paracomplete)\n", rank));

    // ── Frame Operator ──
    out.push_str("\n── Frame Operator S (4×4 in Pauli basis) ──\n");
    let smat = frame_operator_matrix(&ops);
    for i in 0..4 {
        out.push_str("  [");
        for j in 0..4 {
            if j > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", smat[i][j]));
        }
        out.push_str("]\n");
    }

    let fevals = frame_eigenvalues(&ops);
    out.push_str("  Diagonal (frame eigenvalues): [");
    for i in 0..4 {
        if i > 0 { out.push_str(", "); }
        out.push_str(&format!("{:.6}", fevals[i]));
    }
    out.push_str("]\n");

    // SIC check: for d=2, ideal frame diag = [1, 1/3, 1/3, 1/3] = [1, 0.333, 0.333, 0.333]
    let sic_ideal = [1.0, 1.0/3.0, 1.0/3.0, 1.0/3.0];
    let mut sic_dist = 0.0f64;
    for i in 0..4 { sic_dist += (fevals[i] - sic_ideal[i]).abs(); }
    out.push_str(&format!("  SIC distance (from ideal [1,⅓,⅓,⅓]): {:.6}\n", sic_dist));

    out
}

/// Eigenvalue computation from raw Bloch parameters.
pub fn ovm_eigen(x: f64, y: f64, z: f64, norm: f64, trace: f64) -> String {
    let (l1, l2) = compute_eigenvalues(x, y, z, norm, trace);
    let mut out = String::new();
    out.push_str(&format!("═══ Eigenvalue Computation ═══\n"));
    out.push_str(&format!("Bloch vector:  ({:.4}, {:.4}, {:.4})\n", x, y, z));
    out.push_str(&format!("Bloch norm:    {:.6}\n", norm));
    out.push_str(&format!("Trace coeff:   {:.6}\n", trace));
    out.push_str(&format!("E = ({:.4}/2)·I + ({:.4}/2)·(n·σ)\n", trace, norm));
    out.push_str(&format!("λ₁ = tr/2 + r/2 = {:.6}\n", l1));
    out.push_str(&format!("λ₂ = tr/2 − r/2 = {:.6}\n", l2));
    out.push_str(&format!("Positive: {}  (λ₂ ≥ 0)\n", l2 >= -1e-9));
    out.push_str(&format!("Pure state: {}  (λ₁=1, λ₂=0)\n",
        (l1 - 1.0).abs() < 1e-9 && l2.abs() < 1e-9));
    out
}

/// Frame operator report for a named set.
pub fn ovm_frame(name: &str) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown set: '{}'. Use 'ovm help' for known names.\n", name),
    };
    let smat = frame_operator_matrix(&ops);
    let fevals = frame_eigenvalues(&ops);
    let mut out = String::new();
    out.push_str(&format!("═══ Frame Operator: {} ═══\n", name));
    out.push_str("S = Σ_i |E_i⟩⟩⟨⟨E_i|  (4×4 in Pauli basis)\n");
    for i in 0..4 {
        out.push_str("  [");
        for j in 0..4 {
            if j > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", smat[i][j]));
        }
        out.push_str("]\n");
    }
    out.push_str("Diagonal (frame evals): [");
    for i in 0..4 {
        if i > 0 { out.push_str(", "); }
        out.push_str(&format!("{:.6}", fevals[i]));
    }
    out.push_str("]\n");
    out
}

/// Overlap matrix report for a named set.
pub fn ovm_overlap(name: &str) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown set: '{}'. Use 'ovm help' for known names.\n", name),
    };
    let g = overlap_matrix(&ops);
    let m = ops.len();
    let mut out = String::new();
    out.push_str(&format!("═══ HS Overlap Matrix: {} ═══\n", name));
    out.push_str(&format!("G_ij = Tr(E_i E_j)  ({}×{})\n", m, m));
    for i in 0..m {
        out.push_str("  [");
        for j in 0..m {
            if j > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", g[i][j]));
        }
        out.push_str("]\n");
    }
    out
}

/// Belnap B=XZ fiducial report.
pub fn ovm_belnap() -> String {
    let b = construct_belnap_b_xz();
    let (l1, l2) = b.eigenvalues();
    let bloch = belnap_b_xz_bloch();
    let mut out = String::new();
    out.push_str("═══ Belnap B = XZ Fiducial (d=2 SIC-POVM) ═══\n\n");
    out.push_str("The Belnap B = XZ state is the Weyl-Heisenberg group fiducial\n");
    out.push_str("for the d=2 SIC-POVM. It is the B4 multilattice seed state.\n\n");
    out.push_str(&format!("Bloch vector:  [{:.6}, {:.6}, {:.6}]\n", bloch[1], bloch[2], bloch[3]));
    out.push_str(&format!("Trace coeff:   {:.6}\n", bloch[0]));
    out.push_str(&format!("Bloch norm:    {:.6}  (= 1/√3 ≈ 0.57735)\n", b.bloch_norm));
    out.push_str(&format!("Eigenvalues:   λ₁={:.6}  λ₂={:.6}\n", l1, l2));
    out.push_str(&format!("Pure state:    {}\n", (l1 - 1.0).abs() < 1e-9 && l2.abs() < 1e-9));
    out.push_str("\nClifford orbit generates the full SIC-POVM tetrahedron:\n");
    out.push_str("  C⊗C orbit of B yields 4 equiangular states with |⟨ψ_i|ψ_j⟩|² = 1/3\n");
    out.push_str("\nGrammar identity: B = XZ is the Σ=1:1 self-referential limit\n");
    out.push_str("of the Belnap multilattice — the grammar IS this POVM.\n");
    out
}

/// Help text for the ovm computation tools.
pub fn ovm_help() -> String {
    let mut out = String::new();
    out.push_str("═══ OVM Computation Tools ═══\n\n");
    out.push_str("ovm <name>              — full computation report (eigenvalues, frame,\n");
    out.push_str("                           overlap, equiangularity, positivity, completeness)\n");
    out.push_str("ovm eigen <x> <y> <z> <norm> <trace>\n");
    out.push_str("                         — compute eigenvalues from Bloch parameters\n");
    out.push_str("ovm frame <name>         — frame operator S (4×4 in Pauli basis)\n");
    out.push_str("ovm overlap <name>       — HS overlap matrix G_ij = Tr(E_i E_j)\n");
    out.push_str("ovm belnap               — Belnap B=XZ fiducial state\n");
    out.push_str("ovm help                 — this help\n\n");
    out.push_str("Known operator sets:\n");
    for n in ops_names() {
        out.push_str(&format!("  {}\n", n));
    }
    out
}

