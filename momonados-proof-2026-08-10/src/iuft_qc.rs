// iuft_qc.rs — IUFT Quantum Expansion: 12→3 Euler-angle QC gate encodings
//
// Encodes the 12-primitive IG tuple into a 3-parameter SU(2) gate
// (Euler angles θ, φ, ψ) via the degenerate projection discovered in
// IUFT Quantum Expansion II.
//
// The 12→3 encoding is:
//   θ = f(⊢, ⊡, Σ)    — latitude angle from dimensionality/winding/stoich
//   φ = f(>, <, ⊥)    — azimuthal phase from coupling/parity/chirality
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
///   woe (sub-critical)     →   0°
///   ⊙  (critical)         →  90°  (canonical self-modeling)
///   roar (complex critical) → 180°
///   err (exceptional)  → 270°
///   haha (supercrit) →   0°  (wraps — self-modeling complete)
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

/// θ(⊢, ⊡, Σ): latitude angle from dimensionality, winding, stoichiometry.
///
/// Each primitive is normalized to [0, 1] within its family and contributes
/// equally to the 0–180° range.
fn encode_theta(d: IgPrim, omega: IgPrim, s: IgPrim) -> f64 {
    let nd = normalize_ordinal(d, 4.0);     // ⊢: 1–4
    let nw = normalize_ordinal(omega, 4.0);  // ⊡: 1–4
    let ns = normalize_ordinal(s, 3.0);      // Σ: 1–3
    // Equal-weighted average scaled to [0°, 180°]
    let avg = (nd + nw + ns) / 3.0;
    avg * 180.0
}

/// φ(>, <, ⊥): azimuthal phase from coupling, parity, chirality.
///
/// Each primitive is normalized to [0, 1] within its family and contributes
/// equally to the 0–360° range, producing a full circular encoding.
fn encode_phi(r: IgPrim, p: IgPrim, h: IgPrim) -> f64 {
    let nr = normalize_ordinal(r, 4.0);     // >: 1–4
    let np = normalize_ordinal(p, 5.0);     // <: 1–5
    let nh = normalize_ordinal(h, 4.0);     // ⊥: 1–4
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
    // θ depends on ⊢ (slot 0), ⊡ (slot 11), Σ (slot 10)
    // φ depends on > (slot 2), < (slot 3), ⊥ (slot 9)

    let _ = base; // suppress unused warning for now
    dpsi[8] = 180.0 / 2.0;  // dψ/d⊙ ≈ 180° per ordinal unit
    dtheta[0] = 180.0 / 3.0 / 3.0;  // dθ/d⊢: full range 180°, 3 contributors, 3 ordinal steps
    dtheta[11] = 180.0 / 3.0 / 3.0; // dθ/d⊡
    dtheta[10] = 180.0 / 3.0 / 2.0; // dθ/dΣ: only 2 ordinal steps
    dphi[2] = 360.0 / 3.0 / 3.0;    // dφ/d>
    dphi[3] = 360.0 / 3.0 / 4.0;    // dφ/d<: 4 ordinal steps
    dphi[9] = 360.0 / 3.0 / 3.0;    // dφ/d⊥

    IuftSensitivity { dtheta, dphi, dpsi }
}

// ═══════════════════════════════════════════════════════════════
// WHERE THE HARDCODED GATES WERE
// ═══════════════════════════════════════════════════════════════
//
// Twelve angle triples used to sit here, written out beside the encoder that
// computes them. They had drifted: the graviton's differed from its own catalog
// tuple by 0.397 and the electron's by 0.629, and the electron's comment said
// why — "using encode: θ=180°, φ=105°, ψ=90° — but we refine from IUFT
// expansion". A hand refinement of a computed value is a second source of
// truth, and the second source is the one that goes stale.
//
// They also shadowed the encoder. `gate_for` checked the table first, so
// `iuft encode graviton` printed the hand-written triple under the word
// "encoded" and never touched the tuple.
//
// And they made the consistency check vacuous. For a name with no triple,
// `gate_for` fell through to the catalog, the check encoded the same entry a
// second time, and the two agreed exactly. `iuft verify yhwh` printed 0.000000
// and meant nothing by it.
//
// Nothing is hardcoded now, and no hand-picked reference list replaced it
// either. Names go through `catalog::lookup`, which already carries the
// kernel's own normalisation and aliases, and every gate comes from the tuple
// through `encode`.

// ═══════════════════════════════════════════════════════════════
// LOOKUP FUNCTIONS
// ═══════════════════════════════════════════════════════════════

/// The IUFT QC gate for a name: look the name up in the catalog, take its
/// tuple, encode. There is no other path — a gate this kernel reports is one it
/// computed from the catalog this session.
///
/// `catalog::lookup` does the name handling, so "CLINK L8", "clink_l8" and
/// "CLINK-L8" all arrive at the same entry without a second alias table here.
pub fn gate_for(name: &str) -> Option<IuftQcGate> {
    crate::catalog::lookup(name).map(|e| encode_entry(&e))
}

pub fn gate_for_entry(entry: &CatalogEntry) -> Option<IuftQcGate> {
    gate_for(entry.name)
}

/// Compute the IUFT QC gate distance between two catalog entries.
pub fn gate_distance(name_a: &str, name_b: &str) -> Option<f64> {
    let ga = gate_for(name_a)?;
    let gb = gate_for(name_b)?;
    Some(ga.distance_to(&gb))
}

// ═══════════════════════════════════════════════════════════════
// DISTANCE MATRIX
// ═══════════════════════════════════════════════════════════════

/// Every catalog entry in a domain, with its gate. `None` takes the whole
/// catalog. The selection is the catalog's, not a list kept here.
pub fn gates_in(domain: Option<crate::catalog::Domain>) -> Vec<(&'static str, IuftQcGate)> {
    crate::catalog::catalog_entries(domain)
        .map(|e| (e.name, encode_entry(e)))
        .collect()
}

/// The gates named on a command line, in the order given. A name that does not
/// resolve is dropped, and the caller can see that from the count.
pub fn gates_named(names: &[&str]) -> Vec<(alloc::string::String, IuftQcGate)> {
    use alloc::string::ToString;
    names.iter()
        .filter_map(|&n| gate_for(n).map(|g| (n.to_string(), g)))
        .collect()
}

/// Pairwise distance matrix over the gates handed in.
pub fn distance_matrix(gates: &[(alloc::string::String, IuftQcGate)]) -> Vec<Vec<f64>> {
    let n = gates.len();
    let mut matrix = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            row.push(if i == j { 0.0 } else { gates[i].1.distance_to(&gates[j].1) });
        }
        matrix.push(row);
    }
    matrix
}

/// The catalog entry whose gate is nearest a given gate.
///
/// This used to range over twelve hand-picked names, which made "nearest known"
/// mean "nearest of twelve". It ranges over the catalog now, so the answer is
/// the nearest entry the kernel actually holds.
pub fn nearest_known(gate: &IuftQcGate) -> (&'static str, f64) {
    let mut best_name = "";
    let mut best_dist = f64::INFINITY;
    for e in crate::catalog::catalog_entries(None) {
        let d = gate.distance_to(&encode_entry(e));
        if d < best_dist { best_dist = d; best_name = e.name; }
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
pub fn verify_one(name: &str) {
    match crate::catalog::lookup(name) {
        Some(e) => {
            let g = encode_entry(&e);
            if e.name != name { sprintln!("  '{}' is catalog entry '{}'", name, e.name); }
            sprintln!("  {:>28}: θ={:6.1}°  φ={:6.1}°  ψ={:6.1}°   unitary={}",
                      e.name, g.theta_deg, g.phi_deg, g.psi_deg, verify_unitary(&g));
        }
        None => sprintln!("  no catalog entry for '{}'", name),
    }
}

/// What is left to check once nothing is hardcoded.
///
/// The old check measured a hand-written angle triple against the computed one,
/// and could not answer for a name that had no triple: it encoded the same
/// entry twice and reported perfect agreement. The triples are gone, so that
/// question is gone with them.
///
/// Two real ones remain, and they range over the whole catalog rather than a
/// hand-picked dozen. Every entry must encode to a unitary gate. And the
/// encoding is a 12→3 projection, degenerate by construction, so where it sends
/// different tuples to the same gate is the projection's kernel showing itself
/// — worth counting, not worth hiding.
pub fn verify_catalog() {
    let gates = gates_in(None);
    let mut non_unitary = 0usize;
    for (name, g) in &gates {
        if !verify_unitary(g) {
            sprintln!("  NOT UNITARY: {} (θ={:.1}° φ={:.1}° ψ={:.1}°)",
                      name, g.theta_deg, g.phi_deg, g.psi_deg);
            non_unitary += 1;
        }
    }
    sprintln!("Unitarity: {} of {} catalog gates unitary.",
              gates.len() - non_unitary, gates.len());

    let mut collisions = 0usize;
    let mut shown = 0usize;
    for i in 0..gates.len() {
        for j in (i + 1)..gates.len() {
            if gates[i].1.distance_to(&gates[j].1) < 1e-9 {
                collisions += 1;
                if shown < 12 {
                    sprintln!("  {} = {}", gates[i].0, gates[j].0);
                    shown += 1;
                }
            }
        }
    }
    sprintln!("Degeneracy of the 12→3 projection: {} colliding pairs among {} entries{}",
              collisions, gates.len(),
              if collisions > shown { " (first 12 shown)" } else { "" });
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

/// Print the distance matrix over the named entries.
pub fn print_distance_matrix(names: &[&str]) {
    let gates = gates_named(names);
    if gates.is_empty() {
        sprintln!("iuft matrix <name> <name> ...   e.g. `iuft matrix graviton photon electron`");
        sprintln!("Any catalog entry works; names go through the catalog's own aliases.");
        return;
    }
    let matrix = distance_matrix(&gates);
    sprintln!("IUFT Gate Distance Matrix (projective SU(2) distance):");
    let mut header = alloc::string::String::from(format!("{:>16}", ""));
    for (name, _) in &gates {
        header.push_str(&format!("{:>8}", crate::text::clip(name, 7)));
    }
    sprintln!("{}", header);
    for i in 0..gates.len() {
        let mut row = alloc::string::String::from(
            format!("{:>16}", crate::text::clip(&gates[i].0, 15)));
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
        '𐑦' => Some(IgPrim::if_),
        '𐑛' => Some(IgPrim::dead),
        '𐑨' => Some(IgPrim::ash),
        '𐑼' => Some(IgPrim::array),
        // T Topology
        '𐑸' => Some(IgPrim::are),
        '𐑡' => Some(IgPrim::judge),
        '𐑰' => Some(IgPrim::eat),
        '𐑥' => Some(IgPrim::mime),
        '𐑶' => Some(IgPrim::oil),
        // R Coupling
        '𐑾' => Some(IgPrim::ian),
        '𐑽' => Some(IgPrim::ear),
        '𐑑' => Some(IgPrim::tot),
        '𐑩' => Some(IgPrim::ado),
        // P Parity
        '𐑹' => Some(IgPrim::or_),
        '𐑯' => Some(IgPrim::nun),
        '𐑬' => Some(IgPrim::out),
        '𐑿' => Some(IgPrim::yew),
        '𐑗' => Some(IgPrim::church),
        // F Fidelity
        '𐑐' => Some(IgPrim::peep),
        '𐑱' => Some(IgPrim::age),
        '𐑞' => Some(IgPrim::they),
        // K Kinetics
        '𐑪' => Some(IgPrim::on),
        '𐑧' => Some(IgPrim::egg),
        '𐑤' => Some(IgPrim::loll),
        '𐑘' => Some(IgPrim::yea),
        '𐑺' => Some(IgPrim::air),
        // G Cardinality
        '𐑲' => Some(IgPrim::ice),
        '𐑚' => Some(IgPrim::bib),
        '𐑔' => Some(IgPrim::thigh),
        // C Composition
        '𐑠' => Some(IgPrim::measure),
        '𐑝' => Some(IgPrim::vow),
        '𐑜' => Some(IgPrim::gag),
        '𐑵' => Some(IgPrim::ooze),
        // Phi Criticality
        '⊙' => Some(IgPrim::monad),
        '𐑮' => Some(IgPrim::roar),
        '𐑻' => Some(IgPrim::err),
        '𐑢' => Some(IgPrim::woe),
        '𐑣' => Some(IgPrim::haha),
        // H Chirality
        '𐑫' => Some(IgPrim::wool),
        '𐑖' => Some(IgPrim::sure),
        '𐑒' => Some(IgPrim::kick),
        '𐑓' => Some(IgPrim::fee),
        // S Stoichiometry
        '𐑳' => Some(IgPrim::up),
        '𐑕' => Some(IgPrim::so),
        '𐑙' => Some(IgPrim::hung),
        // Omega Winding
        '𐑭' => Some(IgPrim::ah),
        '𐑴' => Some(IgPrim::oak),
        '𐑷' => Some(IgPrim::awe),
        '𐑟' => Some(IgPrim::zoo),
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
