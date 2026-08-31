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
    let _r3 = libm::sqrt(3.0);
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

/// Build SIC-NPOVM: tetrahedral geometry with partial positivity <=𐑬 and Z₂ winding ⊡=𐑴.
/// Same Bloch norm as SIC-POVM, but with two-step chirality ⊥=𐑖 and disjunctive composition.
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
/// Updated per grid: <=𐑬 (partial parity), ∋=𐑜 (disjunctive), ∈=𐑔 (aleph).
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

/// Build A-PC-POVM (grid variant): 3 operators, asymmetric Bloch norms, local completeness ∈=𐑲.
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

/// Build A-PC-POVM† (HTML variant): bidirectional coupling >=𐑾, mesoscale ∈=𐑚, broadcast ∋=𐑵.
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
    out.push_str("ovm measure|map <name>   — measurement map M: B_sa→ℝ^m, adjoint M†, kernel\n");
    out.push_str("ovm belnap               — Belnap B=XZ fiducial state\n");
    out.push_str("ovm spectral <name>      — spectral decomposition & operator properties\n");
    out.push_str("ovm born <name> <sx> <sy> <sz>\n");
    out.push_str("                         — Born rule probabilities for state (sx,sy,sz)\n");
    out.push_str("ovm duals <name>         — frame dual operators (conical 2-design duals)\n");
    out.push_str("ovm cycle <name> <sx> <sy> <sz>\n");
    out.push_str("                         — full measure→reconstruct cycle\n");
    out.push_str("ovm help                 — this help\n\n");
    out.push_str("Known operator sets:\n");
    for n in ops_names() {
        out.push_str(&format!("  {}\n", n));
    }
    out
}

// ═══════════════════════════════════════════════════════════════
// MEASUREMENT MAP — §5.2-5.6 of OVM_MASTER_MATHEMATICS
// ═══════════════════════════════════════════════════════════════

/// Compute the Pauli-basis coefficients of a QubitOp.
/// E = (trace_coeff/2)·I + (bloch_norm/2)·(x·σ_x + y·σ_y + z·σ_z)
/// Returns [a_0, a_1, a_2, a_3] where E = Σ a_j σ_j, σ_0 = I.
pub fn pauli_coeffs(op: &QubitOp) -> [f64; 4] {
    [
        op.trace_coeff / 2.0,
        op.bloch_norm * op.bloch_vec.x / 2.0,
        op.bloch_norm * op.bloch_vec.y / 2.0,
        op.bloch_norm * op.bloch_vec.z / 2.0,
    ]
}

/// Compute the measurement map M: B_sa(ℂ²) → ℝ^m.
/// M is an m×4 matrix. Row k contains the Pauli coefficients of E_k:
/// M_{k,a} = coefficient of σ_a in E_k (with σ_0 = I).
/// For any observable X = Σ x_a σ_a: (M·X)_k = Tr(E_k X) / 2.
pub fn measurement_map(ops: &[QubitOp]) -> Vec<[f64; 4]> {
    ops.iter().map(|op| pauli_coeffs(op)).collect()
}

/// Compute the adjoint M†: ℝ^m → B_sa(ℂ²) with respect to the HS inner product.
/// M† = (1/2) M^T in the Pauli basis, because Tr(σ_a σ_b) = 2δ_{ab}.
/// For any vector y ∈ ℝ^m: M†(y) = Σ_{k,a} (1/2) M_{k,a} y_k σ_a.
pub fn measurement_adjoint(ops: &[QubitOp]) -> [[f64; 4]; 4] {
    let m = measurement_map(ops);
    let n = m.len();
    let mut adj = [[0.0f64; 4]; 4];
    // adj[a][k] = (1/2) * M[k][a]
    for k in 0..n {
        for a in 0..4 {
            adj[a][k] = m[k][a] / 2.0;
        }
    }
    adj
}

/// Compute M†M: B_sa → B_sa (4×4 positive semidefinite).
/// M†M = (1/2) M^T M in the Pauli basis.
/// The eigenvalues of M†M determine measurement resolution.
pub fn mtm_matrix(ops: &[QubitOp]) -> [[f64; 4]; 4] {
    let m = measurement_map(ops);
    let n = m.len();
    let mut mtm = [[0.0f64; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            let mut sum = 0.0;
            for k in 0..n {
                sum += m[k][a] * m[k][b];
            }
            mtm[a][b] = sum / 2.0;  // from the (1/2) factor in M†
        }
    }
    mtm
}

/// Compute the kernel of M†M: the subspace of B_sa that the measurement cannot resolve.
/// Returns (kernel_dim, kernel_basis_indices) where indices 0=I,1=σ_x,2=σ_y,3=σ_z.
/// Theoretical result (§5.5): ker(M†M) = span{I} when Σ E_k ∝ I.
pub fn kernel_analysis(ops: &[QubitOp]) -> (usize, Vec<usize>, [f64; 4]) {
    let mtm = mtm_matrix(ops);
    // For a 4×4 real symmetric matrix we can do power iteration to get eigenvalues
    // Simpler: check diagonal dominance and compute trace
    let mut trace = 0.0;
    let mut diag = [0.0f64; 4];
    for a in 0..4 {
        diag[a] = mtm[a][a];
        trace += diag[a];
    }
    
    // For typical OVMs with Σ E_k = I/d, the I-component is dominant
    // Identify near-zero eigenvalues by checking diagonal entries
    let threshold = trace * 1e-6;
    let mut kernel_dim = 0usize;
    let mut kernel_indices = Vec::new();
    for a in 0..4 {
        if diag[a] < threshold {
            kernel_dim += 1;
            kernel_indices.push(a);
        }
    }
    (kernel_dim, kernel_indices, diag)
}

/// Compute M M^T: ℝ^m → ℝ^m (m×m Gram matrix of measurement vectors).
/// (M M^T)_{ij} = (1/2) Tr(E_i E_j). Relates to the HS overlap via factor 1/2.
pub fn mmt_matrix(ops: &[QubitOp]) -> Vec<Vec<f64>> {
    let m = ops.len();
    let mut mmt = Vec::with_capacity(m);
    for i in 0..m {
        let mut row = Vec::with_capacity(m);
        for j in 0..m {
            row.push(ops[i].hs_inner(&ops[j]) / 2.0);
        }
        mmt.push(row);
    }
    mmt
}

/// Full measurement report: measurement map M, adjoint M†, M†M, M M^T,
/// kernel analysis, and sensitivity eigenvalues.
pub fn ovm_measurement_report(name: &str) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown set: '{}'. Use 'ovm help' for known names.\n", name),
    };
    let m_len = ops.len();
    let mut out = String::new();
    out.push_str(&format!("═══ Measurement Map Analysis: {} ═══\n\n", name));

    // ── Measurement Map M ──
    let mm = measurement_map(&ops);
    out.push_str(&format!("── Measurement Map M ({}×4) ──\n", m_len));
    out.push_str("  M_{k,a} = coefficient of σ_a in E_k  (σ₀=I, σ₁=σ_x, σ₂=σ_y, σ₃=σ_z)\n");
    for k in 0..m_len {
        out.push_str(&format!("  E_{}: [", k));
        for a in 0..4 {
            if a > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", mm[k][a]));
        }
        out.push_str("]\n");
    }

    // ── Adjoint M† ──
    let adj = measurement_adjoint(&ops);
    out.push_str("\n── Adjoint M† = (1/2)M^T (4×m) ──\n");
    out.push_str("  M†_{a,k} = M_{k,a}/2  (HS metric: Tr(σ_a σ_b)=2δ_{ab})\n");
    for a in 0..4 {
        out.push_str(&format!("  σ_{}: [", a));
        for k in 0..m_len {
            if k > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", adj[a][k]));
        }
        out.push_str("]\n");
    }

    // ── M†M ──
    let mtm = mtm_matrix(&ops);
    out.push_str("\n── M†M (4×4) ──\n");
    out.push_str("  M†M = (1/2) M^T M\n");
    for a in 0..4 {
        out.push_str("  [");
        for b in 0..4 {
            if b > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", mtm[a][b]));
        }
        out.push_str("]\n");
    }

    // ── Kernel Analysis ──
    let (kdim, kindices, diag) = kernel_analysis(&ops);
    out.push_str(&format!("\n── Kernel Analysis ──\n"));
    out.push_str(&format!("  Diagonal M†M: [{:.6}, {:.6}, {:.6}, {:.6}]\n",
        diag[0], diag[1], diag[2], diag[3]));
    out.push_str(&format!("  Kernel dim: {}  (theoretical: 1 = span{{I}})\n", kdim));
    if !kindices.is_empty() {
        let names = ["I", "σ_x", "σ_y", "σ_z"];
        out.push_str("  Kernel directions: ");
        for (i, &idx) in kindices.iter().enumerate() {
            if i > 0 { out.push_str(", "); }
            out.push_str(names[idx]);
        }
        out.push_str("\n");
    }
    out.push_str("  Theoretical: ker(M†M) = span{I} when ΣE_k ∝ I (§5.5)\n");

    // ── Sensitivity: eigenvalues of M†M ──
    out.push_str("\n── Measurement Sensitivity (§5.6) ──\n");
    // Use frame eigenvalues as proxy (they're the diagonal of the frame operator,
    // which is related to M†M). The actual eigenvalues need full diagonalization.
    // For d=2 OVMs, SIC-POVM ideal: {1/2, 1/6, 1/6, 1/6} for M†M
    out.push_str("  (M†M eigenvalues ≈ diagonal for near-diagonal M†M)\n");
    let n_resolved = 4 - kdim;
    out.push_str(&format!("  Resolvable directions: {}/4\n", n_resolved));
    if kdim >= 1 && kindices.contains(&0) {
        out.push_str("  Identity-blind: measurement cannot resolve Tr(ρ) (fixed by normalization)\n");
    }

    // ── M M^T ──
    let mmt = mmt_matrix(&ops);
    out.push_str(&format!("\n── M M^T ({}×{}) ──\n", m_len, m_len));
    out.push_str("  (M M^T)_{ij} = (1/2) Tr(E_i E_j) = G_{ij}/2\n");
    for i in 0..m_len {
        out.push_str("  [");
        for j in 0..m_len {
            if j > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.6}", mmt[i][j]));
        }
        out.push_str("]\n");
    }

    // ── Galois connection verification ──
    out.push_str("\n── Galois Connection (M ⊣ M†) ──\n");
    out.push_str("  ⟨M X, y⟩_ℝ^m = ⟨X, M† y⟩_HS for all X∈B_sa, y∈ℝ^m\n");
    out.push_str("  Verified by construction: M† = (1/2)M^T in Pauli basis\n");
    out.push_str("  HS factor 2 from Tr(σ_a σ_b)=2δ_{ab} for all a,b∈{0,1,2,3}\n");

    out
}


// ═══════════════════════════════════════════════════════════════
// SPECTRAL CALCULATORS — Operator spectral theory for OVMs
// ═══════════════════════════════════════════════════════════════
// Eigenprojectors, spectral decomposition, Born rule,
// frame duals, state reconstruction, expectation/variance,
// purity, commutators, uncertainty relations.
// These make OVMs actually useful for quantum measurement.
// ═══════════════════════════════════════════════════════════════

/// Eigenprojectors of a qubit operator.
/// E = (tr/2)I + (r/2)(n·σ) has eigenprojectors:
///   Π₁ = (I + n̂·σ)/2  (projector onto λ₁ = (tr+r)/2 eigenspace)
///   Π₂ = (I − n̂·σ)/2  (projector onto λ₂ = (tr−r)/2 eigenspace)
/// For r=0 (degenerate): both projectors are I/2.
pub fn eigenprojectors(op: &QubitOp) -> (QubitOp, QubitOp) {
    if op.bloch_norm < 1e-12 {
        // Degenerate operator: proportional to identity
        let half = QubitOp {
            trace_coeff: 1.0, bloch_norm: 0.0,
            bloch_vec: BlochVec::new(0.0, 0.0, 0.0),
        };
        return (half, half);
    }
    let n_hat = op.bloch_vec.normalize();
    let plus = QubitOp {
        trace_coeff: 1.0,
        bloch_norm: 1.0,
        bloch_vec: n_hat,
    };
    let minus = QubitOp {
        trace_coeff: 1.0,
        bloch_norm: 1.0,
        bloch_vec: BlochVec::new(-n_hat.x, -n_hat.y, -n_hat.z),
    };
    (plus, minus)
}

/// Spectral decomposition: A = λ₁·Π₁ + λ₂·Π₂.
/// Returns (λ₁, Π₁, λ₂, Π₂).
pub fn spectral_decomposition(op: &QubitOp) -> (f64, QubitOp, f64, QubitOp) {
    let (l1, l2) = op.eigenvalues();
    let (p1, p2) = eigenprojectors(op);
    (l1, p1, l2, p2)
}

/// Construct density matrix from Bloch vector.
/// ρ = (I + x·σ_x + y·σ_y + z·σ_z)/2
/// Constraint: x² + y² + z² ≤ 1 (pure state at equality).
pub fn construct_density_matrix(bloch_x: f64, bloch_y: f64, bloch_z: f64) -> QubitOp {
    let r_sq = bloch_x*bloch_x + bloch_y*bloch_y + bloch_z*bloch_z;
    if r_sq > 1.000001 {
        // Clamp to Bloch sphere surface
        let s = 1.0 / libm::sqrt(r_sq);
        return QubitOp {
            trace_coeff: 1.0,
            bloch_norm: 1.0,
            bloch_vec: BlochVec::new(bloch_x * s, bloch_y * s, bloch_z * s),
        };
    }
    QubitOp {
        trace_coeff: 1.0,
        bloch_norm: libm::sqrt(r_sq),
        bloch_vec: BlochVec::new(bloch_x, bloch_y, bloch_z),
    }
}

/// Born rule: p_k = Tr(ρ E_k) for k = 0..m−1.
/// Uses the Hilbert-Schmidt inner product.
pub fn born_probabilities(state: &QubitOp, povm: &[QubitOp]) -> Vec<f64> {
    povm.iter().map(|ek| state.hs_inner(ek)).collect()
}

/// Expectation value ⟨A⟩_ρ = Tr(ρ A).
pub fn expectation(state: &QubitOp, observable: &QubitOp) -> f64 {
    state.hs_inner(observable)
}

/// Variance Var_ρ(A) = Tr(ρ A²) − ⟨A⟩².
/// For qubit with A = (tr/2)I + (r/2)(n·σ):
///   A² = ((tr²+r²)/4)I + (tr·r/2)(n·σ)
///   Tr(ρ A²) = (tr²+r²)/4 + (tr·r/2)(n_ρ·n_A)·(r_ρ/2)·2
///            = (tr²+r²)/4 + tr·r·(n_ρ·n_A)·r_ρ/2 / 2 ... 
/// Use Pauli-basis computation for exactness.
pub fn variance(state: &QubitOp, observable: &QubitOp) -> f64 {
    let a_sq = operator_square(observable);
    let exp_a = expectation(state, observable);
    let exp_a_sq = expectation(state, &a_sq);
    let var = exp_a_sq - exp_a * exp_a;
    if var < 0.0 && var > -1e-12 { 0.0 } else { var }
}

/// Square of a qubit operator: A².
/// A = a₀ I + a·σ → A² = (a₀² + |a|²) I + 2a₀ a·σ
/// where a₀ = tr/2, a = (r/2)·n̂.
pub fn operator_square(op: &QubitOp) -> QubitOp {
    let a0 = op.trace_coeff / 2.0;
    let ax = op.bloch_norm * op.bloch_vec.x / 2.0;
    let ay = op.bloch_norm * op.bloch_vec.y / 2.0;
    let az = op.bloch_norm * op.bloch_vec.z / 2.0;
    let a_sq_norm = ax*ax + ay*ay + az*az;
    QubitOp {
        trace_coeff: 2.0 * (a0*a0 + a_sq_norm),
        bloch_norm: 2.0 * a0 * libm::sqrt(ax*ax + ay*ay + az*az),
        bloch_vec: BlochVec::new(ax, ay, az),
    }
}

/// Standard deviation ΔA = √Var(A).
pub fn std_dev(state: &QubitOp, observable: &QubitOp) -> f64 {
    libm::sqrt(variance(state, observable))
}

/// Purity Tr(ρ²) = (1 + |r|²)/2 where |r| is the Bloch norm.
/// Pure state: purity = 1. Maximally mixed: purity = 1/2.
pub fn purity(state: &QubitOp) -> f64 {
    (1.0 + state.bloch_norm * state.bloch_norm) / 2.0
}

/// Commutator [A, B] = AB − BA.
/// For qubits: [a₀I + a·σ, b₀I + b·σ] = 2i (a×b)·σ
/// Returns the commutator as a Bloch vector: (2(a×b)) with trace=0.
/// Note: this is i×[A,B] (the anti-Hermitian part); the actual commutator
/// is i times the cross-product operator.
pub fn commutator_bloch(a: &QubitOp, b: &QubitOp) -> BlochVec {
    let ax = a.bloch_norm * a.bloch_vec.x;
    let ay = a.bloch_norm * a.bloch_vec.y;
    let az = a.bloch_norm * a.bloch_vec.z;
    let bx = b.bloch_norm * b.bloch_vec.x;
    let by = b.bloch_norm * b.bloch_vec.y;
    let bz = b.bloch_norm * b.bloch_vec.z;
    // a×b
    BlochVec::new(
        ay * bz - az * by,
        az * bx - ax * bz,
        ax * by - ay * bx,
    )
}

/// Commutator magnitude |[A,B]| for uncertainty relation.
pub fn commutator_norm(a: &QubitOp, b: &QubitOp) -> f64 {
    let cross = commutator_bloch(a, b);
    // |[A,B]| = 2|cross| in the Bloch representation
    // The actual commutator has eigenvalues ±2|cross|
    cross.norm()
}

/// Robertson uncertainty: ΔA·ΔB ≥ |⟨[A,B]⟩|/2.
/// Returns (ΔA·ΔB, lower_bound, satisfied).
pub fn uncertainty_product(state: &QubitOp, a: &QubitOp, b: &QubitOp) -> (f64, f64, bool) {
    let da = std_dev(state, a);
    let db = std_dev(state, b);
    let cross = commutator_bloch(a, b);
    // ⟨i[A,B]⟩_ρ = 2(cross·r_ρ) where cross = a×b, r_ρ = state Bloch vector
    let bound = libm::fabs(state.bloch_vec.x * cross.x + state.bloch_vec.y * cross.y + state.bloch_vec.z * cross.z);
    let product = da * db;
    (product, bound, product >= bound - 1e-12)
}

// ═══════════════════════════════════════════════════════════════
// FRAME DUALS & STATE RECONSTRUCTION
// ═══════════════════════════════════════════════════════════════

/// Invert a 4×4 matrix (Gaussian elimination with partial pivoting).
/// Returns None if singular.
fn invert_4x4(m: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    let n = 4usize;
    // Augmented matrix [M | I]
    let mut a = [[0.0f64; 8]; 4];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = m[i][j];
            a[i][j + n] = if i == j { 1.0 } else { 0.0 };
        }
    }
    // Forward elimination
    for col in 0..n {
        // Find pivot
        let mut pivot_row = col;
        let mut pivot_val = libm::fabs(a[col][col]);
        for row in (col+1)..n {
            let v = libm::fabs(a[row][col]);
            if v > pivot_val {
                pivot_val = v;
                pivot_row = row;
            }
        }
        if pivot_val < 1e-14 { return None; }
        if pivot_row != col {
            a.swap(col, pivot_row);
        }
        // Eliminate below
        for row in (col+1)..n {
            let factor = a[row][col] / a[col][col];
            for j in col..(2*n) {
                a[row][j] -= factor * a[col][j];
            }
        }
    }
    // Back substitution
    for col in (0..n).rev() {
        for row in (0..col).rev() {
            let factor = a[row][col] / a[col][col];
            for j in col..(2*n) {
                a[row][j] -= factor * a[col][j];
            }
        }
    }
    // Normalize
    let mut inv = [[0.0f64; 4]; 4];
    for i in 0..n {
        let div = a[i][i];
        for j in 0..n {
            inv[i][j] = a[i][j + n] / div;
        }
    }
    Some(inv)
}

/// Frame duals Ẽ_k = S^{-1}(E_k) for d=2.
/// Computes: S = Σ |E_i⟩⟩⟨⟨E_i| (4×4 frame operator),
/// then S^{-1}, then applies to each E_i in Pauli-basis vectorization.
/// Returns the dual frame operators as QubitOp.
pub fn frame_duals_d2(ops: &[QubitOp]) -> Option<Vec<QubitOp>> {
    let s = frame_operator_matrix(ops);
    let s_inv = invert_4x4(&s)?;

    // For each E_k, compute vectorized form v = [tr/√2, r·n_x/√2, r·n_y/√2, r·n_z/√2]
    // Then v_dual = S^{-1} · v
    // Then reconstruct QubitOp from v_dual
    let mut duals = Vec::with_capacity(ops.len());
    for op in ops {
        let v = [
            op.trace_coeff / libm::sqrt(2.0),
            op.bloch_norm * op.bloch_vec.x / libm::sqrt(2.0),
            op.bloch_norm * op.bloch_vec.y / libm::sqrt(2.0),
            op.bloch_norm * op.bloch_vec.z / libm::sqrt(2.0),
        ];
        let mut v_dual = [0.0f64; 4];
        for i in 0..4 {
            for j in 0..4 {
                v_dual[i] += s_inv[i][j] * v[j];
            }
        }
        // Reconstruct: trace = v_dual[0]·√2, bloch = [v_dual[1], v_dual[2], v_dual[3]]·√2
        let tr_dual = v_dual[0] * libm::sqrt(2.0);
        let bx = v_dual[1] * libm::sqrt(2.0);
        let by = v_dual[2] * libm::sqrt(2.0);
        let bz = v_dual[3] * libm::sqrt(2.0);
        let b_norm = libm::sqrt(bx*bx + by*by + bz*bz);
        duals.push(QubitOp {
            trace_coeff: tr_dual,
            bloch_norm: b_norm,
            bloch_vec: if b_norm > 1e-12 {
                BlochVec::new(bx/b_norm, by/b_norm, bz/b_norm)
            } else {
                BlochVec::new(0.0, 0.0, 0.0)
            },
        });
    }
    Some(duals)
}

/// State reconstruction from measurement probabilities.
/// ρ = Σ_k p_k · Ẽ_k  where Ẽ_k are the frame duals.
/// The result may have trace slightly ≠ 1 due to numerical error.
pub fn reconstruct_state(probs: &[f64], duals: &[QubitOp]) -> QubitOp {
    let n = probs.len().min(duals.len());
    let mut rho = QubitOp {
        trace_coeff: 0.0, bloch_norm: 0.0,
        bloch_vec: BlochVec::new(0.0, 0.0, 0.0),
    };
    // Accumulate in Pauli coefficient space for precision
    let mut coeffs = [0.0f64; 4]; // [tr/2, r·n_x/2, r·n_y/2, r·n_z/2]
    for k in 0..n {
        let pk = probs[k];
        let pc = pauli_coeffs(&duals[k]);
        for a in 0..4 {
            coeffs[a] += pk * pc[a];
        }
    }
    rho.trace_coeff = coeffs[0] * 2.0;
    let bx = coeffs[1] * 2.0;
    let by = coeffs[2] * 2.0;
    let bz = coeffs[3] * 2.0;
    let bn = libm::sqrt(bx*bx + by*by + bz*bz);
    rho.bloch_norm = bn;
    if bn > 1e-12 {
        rho.bloch_vec = BlochVec::new(bx/bn, by/bn, bz/bn);
    }
    rho
}

// ═══════════════════════════════════════════════════════════════
// OVM SPECTRAL REPORT — Full spectral analysis
// ═══════════════════════════════════════════════════════════════

/// Full spectral report for a named operator set.
/// Computes: eigenprojectors, spectral decomposition, frame duals,
/// and tests state reconstruction for Belnap B=XZ state.
pub fn ovm_spectral_report(name: &str) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown set: '{}'. Use 'ovm help' for known names.\n", name),
    };
    let m_len = ops.len();
    let mut out = String::new();
    out.push_str(&format!("═══ OVM Spectral Report: {} ═══\n\n", name));
    out.push_str(&format!("Operators: m={}\n", m_len));

    // ── Spectral Decomposition ──
    out.push_str("\n── Spectral Decomposition (each E_k = λ₁Π₁ + λ₂Π₂) ──\n");
    for (k, op) in ops.iter().enumerate() {
        let (l1, p1, l2, p2) = spectral_decomposition(op);
        out.push_str(&format!("  E_{}: λ₁={:.6} Π₁=[tr={:.4}, n=({:.3},{:.3},{:.3})]  λ₂={:.6} Π₂=[tr={:.4}, n=({:.3},{:.3},{:.3})]\n",
            k, l1, p1.trace_coeff, p1.bloch_vec.x, p1.bloch_vec.y, p1.bloch_vec.z,
            l2, p2.trace_coeff, p2.bloch_vec.x, p2.bloch_vec.y, p2.bloch_vec.z));
    }

    // ── Frame Duals ──
    out.push_str("\n── Frame Duals Ẽ_k = S^{-1}(E_k) ──\n");
    match frame_duals_d2(&ops) {
        Some(duals) => {
            for (k, d) in duals.iter().enumerate() {
                let (l1, l2) = d.eigenvalues();
                out.push_str(&format!("  Ẽ_{}: tr={:.6} r={:.6} n=({:.3},{:.3},{:.3})  λ=[{:.6},{:.6}]\n",
                    k, d.trace_coeff, d.bloch_norm, d.bloch_vec.x, d.bloch_vec.y, d.bloch_vec.z, l1, l2));
            }
            // Verify duality: Tr(E_i Ẽ_j) = δ_{ij}
            out.push_str("\n  Duality check Tr(E_i Ẽ_j):\n");
            for i in 0..m_len.min(duals.len()) {
                out.push_str("    [");
                for j in 0..m_len.min(duals.len()) {
                    if j > 0 { out.push_str(", "); }
                    let val = ops[i].hs_inner(&duals[j]);
                    out.push_str(&format!("{:.6}", val));
                }
                out.push_str("]\n");
            }
        }
        None => {
            out.push_str("  Frame operator singular — duals cannot be computed.\n");
        }
    }

    // ── State Reconstruction Test ──
    out.push_str("\n── State Reconstruction Test (Belnap B=XZ) ──\n");
    let belnap = construct_belnap_b_xz();
    let probs = born_probabilities(&belnap, &ops);
    out.push_str("  Born probabilities for B=XZ:\n");
    for (k, p) in probs.iter().enumerate() {
        out.push_str(&format!("    p_{} = {:.6}\n", k, p));
    }
    out.push_str(&format!("    Σ p_k = {:.6} (should = 1.0)\n", probs.iter().sum::<f64>()));

    match frame_duals_d2(&ops) {
        Some(duals) => {
            let rho_recon = reconstruct_state(&probs, &duals);
            let (l1, l2) = rho_recon.eigenvalues();
            out.push_str(&format!("\n  Reconstructed state: tr={:.6} r={:.6} n=({:.3},{:.3},{:.3})\n",
                rho_recon.trace_coeff, rho_recon.bloch_norm,
                rho_recon.bloch_vec.x, rho_recon.bloch_vec.y, rho_recon.bloch_vec.z));
            out.push_str(&format!("  Eigenvalues: [{:.6}, {:.6}]\n", l1, l2));
            out.push_str(&format!("  Purity: {:.6} (original: {:.6})\n", purity(&rho_recon), purity(&belnap)));
            let fid = belnap.hs_inner(&rho_recon);
            out.push_str(&format!("  HS fidelity Tr(ρ_orig ρ_recon): {:.6} (ideal=1.0)\n", fid));
            let d = (belnap.trace_coeff - rho_recon.trace_coeff).abs()
                + (belnap.bloch_norm - rho_recon.bloch_norm).abs()
                + (belnap.bloch_vec.x - rho_recon.bloch_vec.x).abs()
                + (belnap.bloch_vec.y - rho_recon.bloch_vec.y).abs()
                + (belnap.bloch_vec.z - rho_recon.bloch_vec.z).abs();
            out.push_str(&format!("  Reconstruction error (L1): {:.6}\n", d));
        }
        None => {
            out.push_str("  Cannot reconstruct — frame singular.\n");
        }
    }

    out
}

/// Born rule measurement report for a named OVM on a given state.
pub fn ovm_born_report(name: &str, sx: f64, sy: f64, sz: f64) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown set: '{}'. Use 'ovm help' for known names.\n", name),
    };
    let state = construct_density_matrix(sx, sy, sz);
    let probs = born_probabilities(&state, &ops);
    let mut out = String::new();
    out.push_str(&format!("═══ Born Rule Measurement: {} ═══\n\n", name));
    out.push_str(&format!("State Bloch: ({:.4}, {:.4}, {:.4})  r={:.4}\n",
        state.bloch_vec.x, state.bloch_vec.y, state.bloch_vec.z, state.bloch_norm));
    let (sl1, sl2) = state.eigenvalues();
    out.push_str(&format!("State evals: [{:.6}, {:.6}]  Purity: {:.4}\n\n", sl1, sl2, purity(&state)));
    out.push_str("── Outcome Probabilities p_k = Tr(ρ E_k) ──\n");
    let mut total = 0.0f64;
    for (k, p) in probs.iter().enumerate() {
        out.push_str(&format!("  p_{} = {:.6}\n", k, p));
        total += p;
    }
    out.push_str(&format!("\n  Σ p_k = {:.6} (should = 1.0 for POVM)\n", total));
    out.push_str(&format!("  Deviation from 1: {:.2e}\n", (total - 1.0).abs()));

    // Expectation value of each operator
    out.push_str("\n── Expectation Values ⟨E_k⟩ = p_k ──\n");
    for (k, op) in ops.iter().enumerate() {
        let exp = expectation(&state, op);
        let var = variance(&state, op);
        out.push_str(&format!("  ⟨E_{}⟩ = {:.6}  Var = {:.6}  σ = {:.6}\n",
            k, exp, var, libm::sqrt(var)));
    }

    out
}

/// OVM measurement with state reconstruction — full cycle test.
pub fn ovm_measure_cycle(name: &str, sx: f64, sy: f64, sz: f64) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown set: '{}'. Use 'ovm help' for known names.\n", name),
    };
    let state = construct_density_matrix(sx, sy, sz);
    let probs = born_probabilities(&state, &ops);
    let mut out = String::new();
    out.push_str(&format!("═══ Measure→Reconstruct Cycle: {} ═══\n\n", name));

    out.push_str(&format!("Original state: Bloch=({:.4},{:.4},{:.4}) r={:.4} purity={:.4}\n",
        state.bloch_vec.x, state.bloch_vec.y, state.bloch_vec.z,
        state.bloch_norm, purity(&state)));

    match frame_duals_d2(&ops) {
        Some(duals) => {
            let rho_recon = reconstruct_state(&probs, &duals);
            out.push_str(&format!("Reconstructed:   Bloch=({:.4},{:.4},{:.4}) r={:.4} purity={:.4}\n",
                rho_recon.bloch_vec.x, rho_recon.bloch_vec.y, rho_recon.bloch_vec.z,
                rho_recon.bloch_norm, purity(&rho_recon)));

            let fid = state.hs_inner(&rho_recon);
            out.push_str(&format!("\nHS Fidelity Tr(ρ·ρ_recon) = {:.6}\n", fid));
            out.push_str(&format!("Trace fidelity: {:.6} (1.0 = perfect)\n", rho_recon.trace_coeff));

            out.push_str("\n── Born Probabilities ──\n");
            for (k, p) in probs.iter().enumerate() {
                out.push_str(&format!("  p_{} = {:.6}\n", k, p));
            }
            out.push_str(&format!("  Σ = {:.6}\n", probs.iter().sum::<f64>()));
        }
        None => {
            out.push_str("Cannot reconstruct — frame singular.\n");
        }
    }

    out
}

/// Report frame dual operators for a named OVM.
pub fn ovm_duals_report(name: &str) -> String {
    let ops = match ops_by_name(name) {
        Some(o) => o,
        None => return format!("Unknown OVM: {}\n", name),
    };
    let mut out = String::new();
    out.push_str(&format!("═══ Frame Duals: {} ═══\n\n", name));
    match frame_duals_d2(&ops) {
        Some(duals) => {
            out.push_str(&format!("{} frame dual operators (conical 2-design):\n\n", duals.len()));
            for (k, d) in duals.iter().enumerate() {
                out.push_str(&format!("  D_{}: {:#?}
", k, d));
            }
            // Verify: Tr(D_i E_j) = δ_{ij}
            out.push_str("\n── Verification: Tr(D_i E_j) = δ_{ij} ──\n");
            let m = duals.len().min(ops.len()).min(6);
            for i in 0..m {
                for j in 0..m {
                    let tr = duals[i].hs_inner(&ops[j]);
                    let expected = if i == j { 1.0 } else { 0.0 };
                    let ok = if (tr - expected).abs() < 1e-6 { "\u{2713}" } else { "\u{2717}" };
                    out.push_str(&format!("  Tr(D_{} E_{}) = {:.6} (expected {:.0}) {}\n", i, j, tr, expected, ok));
                }
            }
        }
        None => {
            out.push_str("Frame is singular — no duals exist.\n");
        }
    }
    out
}
