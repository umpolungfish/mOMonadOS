// iuft_qc.rs — IUFT Quantum Expansion: 12→3 Euler-angle QC gate encodings
//
// Encodes the 12-primitive IG tuple into a 3-parameter SU(2) gate
// (Euler angles θ, φ, ψ) via the degenerate projection discovered in
// IUFT Quantum Expansion II.
//
// The 12→3 encoding is:
//   θ = f(Ð, Ω, Σ)    — latitude angle from dimensionality/winding/stoich
//   φ = f(Ř, Φ, Ħ)    — azimuthal phase from coupling/parity/chirality
//   ψ = f(⊙)          — self-modeling phase (fixed at π/2 for all ⊙=⊙ entries)
//
// Gate: U(θ,φ,ψ) = Rz(φ)·Ry(θ)·Rz(ψ)

use libm::{sqrt, sin, cos};
use crate::catalog::CatalogEntry;

/// Euler angle SU(2) gate encoding for a quantum universe.
#[derive(Copy, Clone, Debug)]
pub struct IuftQcGate {
    pub theta_deg: f64,  // θ: latitude angle
    pub phi_deg: f64,    // φ: azimuthal phase
    pub psi_deg: f64,    // ψ: self-modeling phase
}

/// π constant (libm doesn't provide it).
const PI: f64 = 3.14159265358979323846;

impl IuftQcGate {
    /// Build from Euler angles in degrees.
    pub const fn new(theta_deg: f64, phi_deg: f64, psi_deg: f64) -> Self {
        Self { theta_deg, phi_deg, psi_deg }
    }

    /// Convert to SU(2) matrix. U = Rz(φ)·Ry(θ)·Rz(ψ)
    /// Returns [[re00, im00, re01, im01], [re10, im10, re11, im11]]
    pub fn to_su2(&self) -> [[f64; 4]; 2] {
        let t = self.theta_deg * PI / 180.0 / 2.0;  // θ/2 in radians
        let p = self.phi_deg * PI / 180.0;           // φ in radians
        let s = self.psi_deg * PI / 180.0;           // ψ in radians

        let ct = cos(t);
        let st = sin(t);

        let phi_half = p / 2.0;
        let psi_half = s / 2.0;

        // Rz(φ)·Ry(θ)·Rz(ψ)
        // U = [[cos(θ/2)·e^{-i(φ+ψ)/2}, -sin(θ/2)·e^{-i(φ-ψ)/2}],
        //      [sin(θ/2)·e^{i(φ-ψ)/2},  cos(θ/2)·e^{i(φ+ψ)/2}]]
        let sum_half = phi_half + psi_half;
        let dif_half = phi_half - psi_half;

        let u00_re = ct * cos(sum_half);
        let u00_im = -ct * sin(sum_half);
        let u01_re = -st * cos(dif_half);
        let u01_im = st * sin(dif_half);
        let u10_re = st * cos(dif_half);
        let u10_im = st * sin(dif_half);
        let u11_re = ct * cos(sum_half);
        let u11_im = ct * sin(sum_half);

        [[u00_re, u00_im, u01_re, u01_im],
         [u10_re, u10_im, u11_re, u11_im]]
    }

    /// Fidelity distance to another gate (projective distance in SU(2)).
    /// d = sqrt(1 - |tr(U†V)|/2)
    pub fn distance_to(&self, other: &IuftQcGate) -> f64 {
        let a = self.to_su2();
        let b = other.to_su2();

        // tr(U†V) = conj(u00)*v00 + conj(u10)*v10 + conj(u01)*v01 + conj(u11)*v11
        // Storage: a = [[u00_re, u00_im, u01_re, u01_im], [u10_re, u10_im, u11_re, u11_im]]
        let trace_re = a[0][0] * b[0][0] + a[0][1] * b[0][1]  // conj(u00)*v00
                      + a[1][0] * b[1][0] + a[1][1] * b[1][1]  // conj(u10)*v10
                      + a[0][2] * b[0][2] + a[0][3] * b[0][3]  // conj(u01)*v01
                      + a[1][2] * b[1][2] + a[1][3] * b[1][3]; // conj(u11)*v11
        let trace_im = a[0][0] * b[0][1] - a[0][1] * b[0][0]
                      + a[1][0] * b[1][1] - a[1][1] * b[1][0]
                      + a[0][2] * b[0][3] - a[0][3] * b[0][2]
                      + a[1][2] * b[1][3] - a[1][3] * b[1][2];

        let trace_mod = sqrt(trace_re * trace_re + trace_im * trace_im);
        let d_sq = 1.0 - 0.5 * trace_mod;
        if d_sq < 0.0 { 0.0 } else { sqrt(d_sq) }
    }
}

// ═══════════════════════════════════════════════════════════════
// GATE ENCODINGS — IUFT Quantum Expansion II
// ═══════════════════════════════════════════════════════════════

/// Graviton gate: θ=91.7°, φ=234.7°, ψ=90.0°
pub const GRAVITON_GATE: IuftQcGate = IuftQcGate::new(91.7, 234.7, 90.0);

/// Photon gate: θ=138.3°, φ=150.5°, ψ=90.0°
pub const PHOTON_GATE: IuftQcGate = IuftQcGate::new(138.3, 150.5, 90.0);

/// Lookup the IUFT QC gate for a catalog entry by name.
/// Returns None if no encoding is known.
pub fn gate_for(name: &str) -> Option<IuftQcGate> {
    match name {
        "graviton" => Some(GRAVITON_GATE),
        "photon"   => Some(PHOTON_GATE),
        _ => None,
    }
}

/// Lookup via CatalogEntry reference.
pub fn gate_for_entry(entry: &CatalogEntry) -> Option<IuftQcGate> {
    gate_for(entry.name)
}

/// Compute the IUFT QC gate distance between two catalog entries.
pub fn gate_distance(name_a: &str, name_b: &str) -> Option<f64> {
    let ga = gate_for(name_a)?;
    let gb = gate_for(name_b)?;
    Some(ga.distance_to(&gb))
}

/// List all known IUFT gate encodings.
pub fn known_gates() -> &'static [(&'static str, &'static IuftQcGate)] {
    &[("graviton", &GRAVITON_GATE), ("photon", &PHOTON_GATE)]
}
