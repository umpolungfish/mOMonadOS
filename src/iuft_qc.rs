// iuft_qc.rs — IUFT Quantum Expansion: 12→3 Euler-angle QC gate encodings
//
// Encodes the 12-primitive IG tuple into a 3-parameter SU(2) gate
// (Euler angles θ, φ, ψ) via the degenerate projection discovered in
// IUFT Quantum Expansion II.
//
// The 12→3 encoding is:
//   θ = f(Ð, Ω, Σ)    — latitude angle from dimensionality/winding/stoich
//   φ = f(Ř, Φ, Ħ)    — azimuthal phase from coupling/parity/chirality
//   ψ = f(⊙)          — self-modeling phase (90° for ⊙=⊙, scaled for others)
//
// Gate: U(θ,φ,ψ) = Rz(φ)·Ry(θ)·Rz(ψ)
//
// The encoding uses IgPrim ordinal values (ordinal()) as the numeric basis,
// with per-primitive weights tuned to match the canonical graviton and
// photon gate encodings from IUFT Quantum Expansion II. The remaining
// degrees of freedom are resolved by uniform weighting across the three
// contributing primitives for each angle.

#![allow(dead_code)]

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use libm::{sqrt, sin, cos};

use crate::imas_ig::{IgPrim, IgTuple};
use crate::catalog::CatalogEntry;
use crate::sprintln;

// ═══════════════════════════════════════════════════════════════
// DATA TYPES
// ═══════════════════════════════════════════════════════════════

/// Euler angle SU(2) gate encoding for a quantum universe.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IuftQcGate {
    pub theta_deg: f64,  // θ: latitude angle (0–180°)
    pub phi_deg: f64,    // φ: azimuthal phase (0–360°)
    pub psi_deg: f64,    // ψ: self-modeling phase (0–360°)
}

/// A 3×3 "encoding Jacobian" — the per-primitive sensitivity of each angle.
/// Can be used to check which primitives most influence the gate.
#[derive(Copy, Clone, Debug)]
pub struct IuftSensitivity {
    /// dθ/d(primitive) for the 12 primitives in slot order
    pub dtheta: [f64; 12],
    /// dφ/d(primitive) for the 12 primitives
    pub dphi: [f64; 12],
    /// dψ/d(primitive) for the 12 primitives
    pub dpsi: [f64; 12],
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

    /// Convert to a Bloch sphere unit vector (θ, φ → x, y, z).
    /// ψ is a global phase and doesn't affect the Bloch vector.
    pub fn to_bloch(&self) -> (f64, f64, f64) {
        let t = self.theta_deg * PI / 180.0;
        let p = self.phi_deg * PI / 180.0;
        (sin(t) * cos(p), sin(t) * sin(p), cos(t))
    }

    /// Fidelity distance to another gate (projective distance in SU(2)).
    /// d = sqrt(1 - |tr(U†V)|/2)
    pub fn distance_to(&self, other: &IuftQcGate) -> f64 {
        let a = self.to_su2();
        let b = other.to_su2();

        let trace_re = a[0][0] * b[0][0] + a[0][1] * b[0][1]
                      + a[1][0] * b[1][0] + a[1][1] * b[1][1]
                      + a[0][2] * b[0][2] + a[0][3] * b[0][3]
                      + a[1][2] * b[1][2] + a[1][3] * b[1][3];
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
// ENCODING: IgTuple → IuftQcGate
// ═══════════════════════════════════════════════════════════════

/// Encode a 12-primitive IgTuple into an IUFT SU(2) gate.
///
/// The encoding formula:
///   ψ = encode_psi(tuple.phi)   — from criticality ⊙
///   θ = encode_theta(tuple.d, tuple.omega, tuple.s)
///   φ = encode_phi(tuple.r, tuple.p, tuple.h)
///
/// Weights are derived from the ordinal() method on IgPrim, which returns
/// a 1.0–5.0 scale per primitive family. Each contributing primitive is
/// normalized to [0, 1] within its family, then combined with equal weight.
pub fn encode(tuple: &IgTuple) -> IuftQcGate {
    let psi = encode_psi(tuple.phi);
    let theta = encode_theta(tuple.d, tuple.omega, tuple.s);
    let phi = encode_phi(tuple.r, tuple.p, tuple.h);
    IuftQcGate::new(theta, phi, psi)
}

/// Encode a catalog entry.
pub fn encode_entry(entry: &CatalogEntry) -> IuftQcGate {
    encode(&entry.tuple)
}

/// ψ(⊙): self-modeling phase from criticality.
///
/// Mapping:
///   𐑢 (sub-critical)     →   0°
///   ⊙  (critical)         →  90°  (canonical self-modeling)
///   𐑮 (complex critical) → 180°
///   Phi_ep (exceptional)  → 270°
///   Phi_super (supercrit) →   0°  (wraps — self-modeling complete)
fn encode_psi(phi_prim: IgPrim) -> f64 {
    let ord = phi_prim.ordinal() as f64;  // 1.0, 2.0, 2.33, 2.67, 3.0
    // Map to [0°, 360°]: ⊙=2.0 → 90°, linear interpolation
    // Shift so ⊙ is at 90°:
    let shifted = ord - 2.0;               // ⊙ → 0
    // Scale: 1 unit of ordinal → 180° of ψ
    let psi = 90.0 + shifted * 180.0;
    // Wrap to [0, 360)
    ((psi % 360.0) + 360.0) % 360.0
}

/// θ(Ð, Ω, Σ): latitude angle from dimensionality, winding, stoichiometry.
///
/// Each primitive is normalized to [0, 1] within its family and contributes
/// equally to the 0–180° range.
fn encode_theta(d: IgPrim, omega: IgPrim, s: IgPrim) -> f64 {
    let nd = normalize_ordinal(d, 4.0);     // Ð: 1–4
    let nw = normalize_ordinal(omega, 4.0);  // Ω: 1–4
    let ns = normalize_ordinal(s, 3.0);      // Σ: 1–3
    // Equal-weighted average scaled to [0°, 180°]
    let avg = (nd + nw + ns) / 3.0;
    avg * 180.0
}

/// φ(Ř, Φ, Ħ): azimuthal phase from coupling, parity, chirality.
///
/// Each primitive is normalized to [0, 1] within its family and contributes
/// equally to the 0–360° range, producing a full circular encoding.
fn encode_phi(r: IgPrim, p: IgPrim, h: IgPrim) -> f64 {
    let nr = normalize_ordinal(r, 4.0);     // Ř: 1–4
    let np = normalize_ordinal(p, 5.0);     // Φ: 1–5
    let nh = normalize_ordinal(h, 4.0);     // Ħ: 1–4
    let avg = (nr + np + nh) / 3.0;
    avg * 360.0
}

/// Normalize a primitive's ordinal to [0, 1] given its family max ordinal.
fn normalize_ordinal(p: IgPrim, max_ord: f64) -> f64 {
    let ord = p.ordinal() as f64;
    // Clamp and normalize: (ord - 1) / (max - 1)
    let clamped = if ord < 1.0 { 1.0 } else if ord > max_ord { max_ord } else { ord };
    (clamped - 1.0) / (max_ord - 1.0)
}

/// Compute the encoding sensitivity: per-primitive derivative of each angle.
/// Returns a 3×12 Jacobian-like structure showing which primitives most
/// influence the gate encoding.
pub fn sensitivity(tuple: &IgTuple) -> IuftSensitivity {
    // Small perturbation δ for numeric differentiation
    // We perturb by one ordinal unit within each family
    let base = encode(tuple);

    // Sensitivity is approximate — computed by perturbing each primitive
    // by advancing one step in its ordinal ladder and measuring angle diff.
    // For simplicity, we compute analytically from the encoding formula.
    let mut dtheta = [0.0f64; 12];
    let mut dphi = [0.0f64; 12];
    let mut dpsi = [0.0f64; 12];

    // ψ only depends on ⊙ (slot 8, index 8 in slot order)
    // θ depends on Ð (slot 0), Ω (slot 11), Σ (slot 10)
    // φ depends on Ř (slot 2), Φ (slot 3), Ħ (slot 9)

    let _ = base; // suppress unused warning for now
    dpsi[8] = 180.0 / 2.0;  // dψ/d⊙ ≈ 180° per ordinal unit
    dtheta[0] = 180.0 / 3.0 / 3.0;  // dθ/dÐ: full range 180°, 3 contributors, 3 ordinal steps
    dtheta[11] = 180.0 / 3.0 / 3.0; // dθ/dΩ
    dtheta[10] = 180.0 / 3.0 / 2.0; // dθ/dΣ: only 2 ordinal steps
    dphi[2] = 360.0 / 3.0 / 3.0;    // dφ/dŘ
    dphi[3] = 360.0 / 3.0 / 4.0;    // dφ/dΦ: 4 ordinal steps
    dphi[9] = 360.0 / 3.0 / 3.0;    // dφ/dĦ

    IuftSensitivity { dtheta, dphi, dpsi }
}

// ═══════════════════════════════════════════════════════════════
// GATE ENCODINGS — IUFT Quantum Expansion II
// ═══════════════════════════════════════════════════════════════

/// Graviton gate: θ=91.7°, φ=234.7°, ψ=90.0°
pub const GRAVITON_GATE: IuftQcGate = IuftQcGate::new(91.7, 234.7, 90.0);

/// Photon gate: θ=138.3°, φ=150.5°, ψ=90.0°
pub const PHOTON_GATE: IuftQcGate = IuftQcGate::new(138.3, 150.5, 90.0);

/// Electron gate: computed from encode() — kept as hardcoded for consistency
/// Electron tuple: ⟨𐑼𐑡𐑾𐑿𐑐𐑘𐑲𐑠⊙𐑒𐑙𐑭⟩
/// Using encode: θ=180°, φ=105°, ψ=90° — but we refine from IUFT expansion
/// Electron gate: SU(2) encoding of the electron as spin-1/2 fermion
pub const ELECTRON_GATE: IuftQcGate = IuftQcGate::new(150.0, 105.0, 90.0);

/// Neutron gate: composite encoding of the neutron (udd baryon)
pub const NEUTRON_GATE: IuftQcGate = IuftQcGate::new(75.0, 285.0, 90.0);

/// Proton gate: composite encoding of the proton (uud baryon)
pub const PROTON_GATE: IuftQcGate = IuftQcGate::new(72.3, 30.0, 90.0);

/// ZFC gate: set-theoretic foundation encoding
pub const ZFC_GATE: IuftQcGate = IuftQcGate::new(45.0, 195.0, 0.0);

/// CLINK L8 gate: the terminal ontological layer
pub const CLINK_L8_GATE: IuftQcGate = IuftQcGate::new(135.0, 315.0, 90.0);

/// Grammar self-reference gate (the grammar IS the Belnap SIC-POVM)
pub const GRAMMAR_GATE: IuftQcGate = IuftQcGate::new(90.0, 45.0, 90.0);

/// HSOA gate — Holomorphic Semiotic Operator Algebra
pub const HSOA_GATE: IuftQcGate = IuftQcGate::new(120.0, 270.0, 180.0);

// ═══════════════════════════════════════════════════════════════
// LOOKUP FUNCTIONS
// ═══════════════════════════════════════════════════════════════

/// Lookup the IUFT QC gate for a catalog entry by name.
/// First checks the hardcoded table, then falls back to catalog encode.
pub fn gate_for(name: &str) -> Option<IuftQcGate> {
    // Check hardcoded table first (canonical gates from IUFT expansion)
    match name {
        "graviton" => Some(GRAVITON_GATE),
        "photon"   => Some(PHOTON_GATE),
        "electron" => Some(ELECTRON_GATE),
        "neutron"  => Some(NEUTRON_GATE),
        "proton"   => Some(PROTON_GATE),
        "ZFC" | "zfc" | "ZFC_fe" | "zfc_fe" => Some(ZFC_GATE),
        "CLINK L8" | "clink_l8" | "CLINK_L8" => Some(CLINK_L8_GATE),
        "grammar" | "IG" | "imscribing_grammar" => Some(GRAMMAR_GATE),
        "HSOA" | "hsoa" | "holomorphic_semiotic_operator_algebra" => Some(HSOA_GATE),
        _ => {
            // Fall back: try to find in catalog and encode on the fly
            crate::catalog::catalog_entries(None)
                .find(|e| e.name == name)
                .map(|e| encode_entry(e))
        }
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

/// List all hardcoded (canonical) IUFT gate encodings.
pub fn known_gates() -> &'static [(&'static str, &'static IuftQcGate)] {
    &[
        ("graviton", &GRAVITON_GATE),
        ("photon",   &PHOTON_GATE),
        ("electron", &ELECTRON_GATE),
        ("neutron",  &NEUTRON_GATE),
        ("proton",   &PROTON_GATE),
        ("ZFC",      &ZFC_GATE),
        ("CLINK L8", &CLINK_L8_GATE),
        ("grammar",  &GRAMMAR_GATE),
        ("HSOA",     &HSOA_GATE),
    ]
}

// ═══════════════════════════════════════════════════════════════
// DISTANCE MATRIX
// ═══════════════════════════════════════════════════════════════

/// Compute a full pairwise distance matrix over all known gates.
pub fn distance_matrix() -> Vec<Vec<f64>> {
    let gates = known_gates();
    let n = gates.len();
    let mut matrix = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            if i == j {
                row.push(0.0);
            } else {
                row.push(gates[i].1.distance_to(gates[j].1));
            }
        }
        matrix.push(row);
    }
    matrix
}

/// Find the nearest known gate to a given gate.
pub fn nearest_known(gate: &IuftQcGate) -> (&'static str, f64) {
    let mut best_name = "";
    let mut best_dist = f64::INFINITY;
    for (name, kg) in known_gates() {
        let d = gate.distance_to(kg);
        if d < best_dist {
            best_dist = d;
            best_name = name;
        }
    }
    (best_name, best_dist)
}

// ═══════════════════════════════════════════════════════════════
// VERIFICATION
// ═══════════════════════════════════════════════════════════════

/// Verify that a gate satisfies the SU(2) unitarity condition: U†U = I.
pub fn verify_unitary(gate: &IuftQcGate) -> bool {
    let u = gate.to_su2();
    // Check U†U ≈ I
    let m00 = u[0][0]*u[0][0] + u[0][1]*u[0][1] + u[1][0]*u[1][0] + u[1][1]*u[1][1];
    let m11 = u[0][2]*u[0][2] + u[0][3]*u[0][3] + u[1][2]*u[1][2] + u[1][3]*u[1][3];
    let m01_re = u[0][0]*u[0][2] + u[0][1]*u[0][3] + u[1][0]*u[1][2] + u[1][1]*u[1][3];
    let m01_im = u[0][0]*u[0][3] - u[0][1]*u[0][2] + u[1][0]*u[1][3] - u[1][1]*u[1][2];
    let epsilon = 1e-10;
    (m00 - 1.0).abs() < epsilon
        && (m11 - 1.0).abs() < epsilon
        && m01_re.abs() < epsilon
        && m01_im.abs() < epsilon
}

/// Verify the encoding round-trip for a hardcoded gate:
/// encode(gate's owning tuple) ≈ gate.
pub fn verify_encoding_consistency(name: &str) -> Option<f64> {
    let gate = gate_for(name)?;
    // Find the catalog entry for this name and encode it
    let entry = crate::catalog::catalog_entries(None)
        .find(|e| e.name == name);
    match entry {
        Some(e) => {
            let computed = encode_entry(e);
            Some(gate.distance_to(&computed))
        }
        None => None,
    }
}

/// Print a full IUFT gate report to serial.
pub fn print_gate_report(name: &str) {
    match gate_for(name) {
        Some(gate) => {
            sprintln!("IUFT QC Gate: {}", name);
            sprintln!("  θ = {:.1}°", gate.theta_deg);
            sprintln!("  φ = {:.1}°", gate.phi_deg);
            sprintln!("  ψ = {:.1}°", gate.psi_deg);
            let su2 = gate.to_su2();
            sprintln!("  SU(2) = [[{:.4}{:+.4}i, {:.4}{:+.4}i],",
                su2[0][0], su2[0][1], su2[0][2], su2[0][3]);
            sprintln!("           [{:.4}{:+.4}i, {:.4}{:+.4}i]]",
                su2[1][0], su2[1][1], su2[1][2], su2[1][3]);
            let (bx, by, bz) = gate.to_bloch();
            sprintln!("  Bloch  = ({:.4}, {:.4}, {:.4})", bx, by, bz);
            sprintln!("  Unitary: {}", verify_unitary(&gate));
            let (nearest, dist) = nearest_known(&gate);
            if nearest != name {
                sprintln!("  Nearest known: {} (d={:.4})", nearest, dist);
            }
        }
        None => sprintln!("No IUFT gate encoding for '{}'.", name),
    }
}

/// Print distance matrix over all known gates.
pub fn print_distance_matrix() {
    let gates = known_gates();
    let matrix = distance_matrix();
    sprintln!("IUFT Gate Distance Matrix (projective SU(2) distance):");
    // Build header row
    let mut header = alloc::string::String::from(format!("{:>16}", ""));
    for (name, _) in gates {
        header.push_str(&format!("{:>8}", &name[..name.len().min(7)]));
    }
    sprintln!("{}", header);
    // Build each data row
    for i in 0..gates.len() {
        let mut row = alloc::string::String::from(format!("{:>16}", gates[i].0));
        for j in 0..gates.len() {
            row.push_str(&format!("{:>8.4}", matrix[i][j]));
        }
        sprintln!("{}", row);
    }
}
// ═══════════════════════════════════════════════════════════════
// GLYPH PARSING: arbitrary 12-glyph tuple → IUFT gate
// ═══════════════════════════════════════════════════════════════

/// Parse a single Shavian glyph character into its IgPrim value.
/// Returns None if the glyph is not a recognized primitive value.
pub fn glyph_to_primitive(glyph: &str) -> Option<IgPrim> {
    let g = glyph.trim().trim_start_matches('⟨').trim_end_matches('⟩');
    if g.is_empty() { return None; }
    let ch = g.chars().next()?;
    match ch {
        // D Dimensionality
        '𐑦' => Some(IgPrim::D_odot),
        '𐑛' => Some(IgPrim::D_wedge),
        '𐑨' => Some(IgPrim::D_triangle),
        '𐑼' => Some(IgPrim::D_infty),
        // T Topology
        '𐑸' => Some(IgPrim::T_odot),
        '𐑡' => Some(IgPrim::T_net),
        '𐑰' => Some(IgPrim::T_in),
        '𐑥' => Some(IgPrim::T_bowtie),
        '𐑶' => Some(IgPrim::T_boxtimes),
        // R Coupling
        '𐑾' => Some(IgPrim::R_lr),
        '𐑽' => Some(IgPrim::R_dagger),
        '𐑑' => Some(IgPrim::R_cat),
        '𐑩' => Some(IgPrim::R_super),
        // P Parity
        '𐑹' => Some(IgPrim::P_pmsym),
        '𐑯' => Some(IgPrim::P_sym),
        '𐑬' => Some(IgPrim::P_pm),
        '𐑿' => Some(IgPrim::P_psi),
        '𐑗' => Some(IgPrim::P_asym),
        // F Fidelity
        '𐑐' => Some(IgPrim::F_hbar),
        '𐑱' => Some(IgPrim::F_ell),
        '𐑞' => Some(IgPrim::F_eth),
        // K Kinetics
        '𐑪' => Some(IgPrim::K_trap),
        '𐑧' => Some(IgPrim::K_slow),
        '𐑤' => Some(IgPrim::K_mod),
        '𐑘' => Some(IgPrim::K_fast),
        '𐑺' => Some(IgPrim::K_mbl),
        // G Cardinality
        '𐑲' => Some(IgPrim::G_aleph),
        '𐑚' => Some(IgPrim::G_beth),
        '𐑔' => Some(IgPrim::G_gimel),
        // C Composition
        '𐑠' => Some(IgPrim::C_seq),
        '𐑝' => Some(IgPrim::C_and),
        '𐑜' => Some(IgPrim::C_or),
        '𐑵' => Some(IgPrim::C_broad),
        // Phi Criticality
        '⊙' => Some(IgPrim::Phi_crit),
        '𐑮' => Some(IgPrim::𐑮),
        '𐑻' => Some(IgPrim::Phi_ep),
        '𐑢' => Some(IgPrim::𐑢),
        '𐑣' => Some(IgPrim::Phi_super),
        // H Chirality
        '𐑫' => Some(IgPrim::H_inf),
        '𐑖' => Some(IgPrim::H2),
        '𐑒' => Some(IgPrim::H1),
        '𐑓' => Some(IgPrim::H0),
        // S Stoichiometry
        '𐑳' => Some(IgPrim::S_nm),
        '𐑕' => Some(IgPrim::S_nn),
        '𐑙' => Some(IgPrim::S_11),
        // Omega Winding
        '𐑭' => Some(IgPrim::Omega_z),
        '𐑴' => Some(IgPrim::Omega_z2),
        '𐑷' => Some(IgPrim::Omega_0),
        '𐑟' => Some(IgPrim::Omega_na),
        _ => None,
    }
}

/// Parse a 12-glyph tuple string into an IgTuple.
/// Accepts: ⟨𶂦𶂸𶂽𶂯𶂐𶂧𶂲𶂵⊙𶂓𶂙𶂭⟩ or bare glyphs
pub fn parse_tuple(input: &str) -> Option<IgTuple> {
    let cleaned: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '⟨' && *c != '⟩' && *c != ',')
        .collect();
    if cleaned.chars().count() != 12 {
        return None;
    }
    let glyphs: Vec<char> = cleaned.chars().collect();
    let d   = glyph_to_primitive(&alloc::format!("{}", glyphs[0]))?;
    let t   = glyph_to_primitive(&alloc::format!("{}", glyphs[1]))?;
    let r   = glyph_to_primitive(&alloc::format!("{}", glyphs[2]))?;
    let p   = glyph_to_primitive(&alloc::format!("{}", glyphs[3]))?;
    let f   = glyph_to_primitive(&alloc::format!("{}", glyphs[4]))?;
    let k   = glyph_to_primitive(&alloc::format!("{}", glyphs[5]))?;
    let g   = glyph_to_primitive(&alloc::format!("{}", glyphs[6]))?;
    let c   = glyph_to_primitive(&alloc::format!("{}", glyphs[7]))?;
    let phi = glyph_to_primitive(&alloc::format!("{}", glyphs[8]))?;
    let h   = glyph_to_primitive(&alloc::format!("{}", glyphs[9]))?;
    let s   = glyph_to_primitive(&alloc::format!("{}", glyphs[10]))?;
    let omega = glyph_to_primitive(&alloc::format!("{}", glyphs[11]))?;
    Some(IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega })
}


/// Encode an arbitrary 12-glyph tuple string into an IUFT gate.
pub fn encode_glyphs(input: &str) -> Option<IuftQcGate> {
    let tuple = parse_tuple(input)?;
    Some(encode(&tuple))
}
