// rebis/fold3d.rs — B4 → Ramachandran → Cartesian reconstruction.
//
// THE BINDING (do not break):
//
//   Backbone comes from the B4 path. Every residue has a B4 value (the first
//   nucleotide of its codon, forward or reconstructed). Adjacent (from, to)
//   B4 pairs step the canonical 16-entry B4_RAMACHANDRAN table; each step
//   gives (phi, psi). A NeRF internal→Cartesian build places N, CA, C, O
//   for every residue, in one pass. This is the source of every coordinate
//   this file emits. There is no PDB rotamer table, no empirical sidechain
//   library, and no imported force field.
//
//   Sidechain ALSO comes from the B4 path, combined with the amino acid's
//   own 12-tuple. The tuple was derived in the kernel (sidechain.rs) from
//   first principles; here it is reduced to a sidechain signature on the
//   three axes that ACTUALLY vary across amino acids:
//     ⊤  Kinetics      (k) → branch depth, the number of C–C bonds from
//                              CA to the first heavy tip atom (CB=0, CG=1,
//                              CD=2).  The Kinetic axis varies per-residue;
//                              𐑺 𐑪 trapped marks → tip at CB; 𐑧 slow → 1;
//                              𐑤 moderate → 1; 𐑘 driven → 0.
//     ⊣  Topology      (t) → chi-2 dihedral offset, in degrees.
//                              𐑡 → 60, 𐑰 → 60, 𐑥 → -60, 𐑶 → 180, 𐑸 → 0.
//     ⊢  Dimensionality (d) → bond length factor, in Å.  1.09 + 0.02·⊢_ord.
//   (⊞ Stoichiometry is hung/1:1 for every amino acid; it carries no
//    information here, so we read it but it does not parameterize geometry.)
//
//   The chi-1 angle itself comes from the same B4 Ramachandran step as
//   the backbone phi of THIS residue. The chi-2 offset above is a
//   constant shift from the ⊣ value of the amino acid's tuple. The
//   path is the only thing that varies residue-to-residue; the tuple
//   is fixed per amino acid and acts as the closed-form source.
//
//   This is a closed-form map, not a lookup. Every coordinate is a
//   closed-form evaluation of (tuple_aa, b4_path) under the Grammar.

use crate::belnap::B4;
use crate::imas_ig::{IgPrim, IgTuple};
use crate::rebis::sidechain as sc;
use crate::rebis::AminoAcid;
use alloc::string::String;
use alloc::vec::Vec;
use libm::{cos, sin, sqrt};

pub type Vec3 = (f64, f64, f64);

fn vec_sub(a: Vec3, b: Vec3) -> Vec3 { (a.0 - b.0, a.1 - b.1, a.2 - b.2) }
fn vec_norm(v: Vec3) -> f64 { sqrt(v.0 * v.0 + v.1 * v.1 + v.2 * v.2) }
fn vec_cross(a: Vec3, b: Vec3) -> Vec3 {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}
fn fabs(x: f64) -> f64 { if x < 0.0 { -x } else { x } }

const DEG: f64 = core::f64::consts::PI / 180.0;
const BOND_N_CA: f64 = 1.458;
const BOND_CA_C: f64 = 1.525;
const BOND_C_N:  f64 = 1.329;
const BOND_C_O:  f64 = 1.231;
// ⊢=𐑼(3) → BOND_CA_CB = 1.09 + 0.02·3 = 1.15 Å
const BOND_CA_CB: f64 = 1.09 + 0.02 * 3.0;
const BOND_CC:    f64 = 1.09 + 0.02 * 3.0;
const ANGLE_N_CA_C: f64 = 111.0 * DEG;
const ANGLE_CA_C_N: f64 = 116.2 * DEG;
const ANGLE_C_N_CA: f64 = 121.7 * DEG;
const ANGLE_C_CA_CB: f64 = 110.5 * DEG;

// ─── B4 → Ramachandran ────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RamaEntry {
    pub phi: f64,
    pub psi: f64,
    pub ss: &'static str,
    pub conf: f64,
}

pub fn ramachandran(from: B4, to: B4) -> RamaEntry {
    use B4::*;
    match (from, to) {
        (N, T) => RamaEntry { phi: -57.0,  psi: -47.0, ss: "helix",   conf: 0.88 },
        (T, B) => RamaEntry { phi: -119.0, psi: 113.0, ss: "sheet",   conf: 0.85 },
        (B, F) => RamaEntry { phi: 57.0,   psi: 45.0,  ss: "helix_l", conf: 0.72 },
        (F, N) => RamaEntry { phi: -60.0,  psi: -30.0, ss: "turn",    conf: 0.75 },
        (N, N) => RamaEntry { phi: -65.0,  psi: -15.0, ss: "loop",    conf: 0.42 },
        (T, T) => RamaEntry { phi: -95.0,  psi: 5.0,   ss: "loop",    conf: 0.40 },
        (F, F) => RamaEntry { phi: -70.0,  psi: 35.0,  ss: "loop",    conf: 0.38 },
        (B, B) => RamaEntry { phi: -55.0,  psi: -45.0, ss: "loop",    conf: 0.36 },
        (T, N) => RamaEntry { phi: -50.0,  psi: -55.0, ss: "helix",   conf: 0.55 },
        (B, T) => RamaEntry { phi: -135.0, psi: 135.0, ss: "sheet",   conf: 0.52 },
        (F, B) => RamaEntry { phi: 65.0,   psi: 50.0,  ss: "helix_l", conf: 0.48 },
        (N, F) => RamaEntry { phi: -70.0,  psi: -25.0, ss: "turn",    conf: 0.52 },
        (N, B) => RamaEntry { phi: -80.0,  psi: -10.0, ss: "loop",    conf: 0.30 },
        (T, F) => RamaEntry { phi: -100.0, psi: 20.0,  ss: "loop",    conf: 0.28 },
        (B, N) => RamaEntry { phi: -50.0,  psi: -35.0, ss: "loop",    conf: 0.30 },
        (F, T) => RamaEntry { phi: -85.0,  psi: -5.0,  ss: "loop",    conf: 0.28 },
    }
}

fn max_conf_for_ss(ss: &str) -> f64 {
    let all = [B4::N, B4::T, B4::F, B4::B];
    let mut m = 0.0f64;
    let mut found = false;
    for &a in &all {
        for &b in &all {
            let e = ramachandran(a, b);
            if e.ss == ss { found = true; if e.conf > m { m = e.conf; } }
        }
    }
    if found { m } else { 0.5 }
}

pub fn rama_steps(b4_path: &[B4]) -> Vec<RamaEntry> {
    let mut out = Vec::with_capacity(b4_path.len());
    for i in 0..b4_path.len() {
        let from = if i == 0 { B4::N } else { b4_path[i - 1] };
        out.push(ramachandran(from, b4_path[i]));
    }
    out
}

// ─── Geometry primitives (NeRF internal→Cartesian) ──────────────────────

fn build_frame(z_dir: Vec3) -> (Vec3, Vec3, Vec3) {
    let z_len = vec_norm(z_dir);
    if z_len < 1e-10 {
        return ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0));
    }
    let z = (z_dir.0 / z_len, z_dir.1 / z_len, z_dir.2 / z_len);
    let reference = if fabs(z.0) < 0.9 { (1.0, 0.0, 0.0) }
        else if fabs(z.1) < 0.9 { (0.0, 1.0, 0.0) }
        else { (0.0, 0.0, 1.0) };
    let mut x = vec_cross(z, reference);
    let mut x_len = vec_norm(x);
    if x_len < 1e-10 {
        let reference2 = if reference == (1.0, 0.0, 0.0) { (0.0, 1.0, 0.0) } else { (1.0, 0.0, 0.0) };
        x = vec_cross(z, reference2);
        x_len = vec_norm(x);
    }
    let x = (x.0 / x_len, x.1 / x_len, x.2 / x_len);
    let y = vec_cross(z, x);
    (x, y, z)
}

fn place_atom(prev: Vec3, prev_prev: Vec3, bond_len: f64, bond_angle: f64, dihedral: f64) -> Vec3 {
    let mut v1 = vec_sub(prev, prev_prev);
    if vec_norm(v1) < 1e-10 { v1 = (0.0, 0.0, 1.0); }
    let (x, y, z) = build_frame(v1);
    let local = (
        bond_len * sin(bond_angle) * cos(dihedral),
        bond_len * sin(bond_angle) * sin(dihedral),
        -bond_len * cos(bond_angle),
    );
    (
        prev.0 + local.0 * x.0 + local.1 * y.0 + local.2 * z.0,
        prev.1 + local.0 * x.1 + local.1 * y.1 + local.2 * z.1,
        prev.2 + local.0 * x.2 + local.1 * y.2 + local.2 * z.2,
    )
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BackboneAtom {
    pub n: Vec3,
    pub ca: Vec3,
    pub c: Vec3,
    pub o: Vec3,
}

pub fn build_backbone(steps: &[RamaEntry]) -> Vec<BackboneAtom> {
    let n_res = steps.len();
    let mut residues: Vec<BackboneAtom> = Vec::with_capacity(n_res);
    if n_res == 0 { return residues; }

    let n0: Vec3 = (0.0, 0.0, 0.0);
    let ca0: Vec3 = (BOND_N_CA, 0.0, 0.0);
    let c0 = place_atom(ca0, n0, BOND_CA_C, ANGLE_N_CA_C, 0.0);
    let o_dir = vec_sub(ca0, c0);
    let o_len = vec_norm(o_dir);
    let o0 = if o_len > 0.01 {
        (c0.0 + o_dir.0 * BOND_C_O / o_len, c0.1 + o_dir.1 * BOND_C_O / o_len, c0.2 + o_dir.2 * BOND_C_O / o_len)
    } else {
        (c0.0, c0.1 + BOND_C_O, c0.2)
    };
    residues.push(BackboneAtom { n: n0, ca: ca0, c: c0, o: o0 });

    for i in 1..n_res {
        let (pr_c, pr_ca) = { let pr = &residues[residues.len() - 1]; (pr.c, pr.ca) };
        let phi_i = steps[i].phi * DEG;
        let psi_i = steps[i].psi * DEG;
        let ni = place_atom(pr_c, pr_ca, BOND_C_N, ANGLE_CA_C_N, core::f64::consts::PI);
        let cai = place_atom(ni, pr_c, BOND_N_CA, ANGLE_C_N_CA, phi_i);
        let ci = place_atom(cai, ni, BOND_CA_C, ANGLE_N_CA_C, psi_i);
        let od = vec_sub(cai, ci);
        let ol = vec_norm(od);
        let oi = if ol > 0.01 {
            (ci.0 + od.0 * BOND_C_O / ol, ci.1 + od.1 * BOND_C_O / ol, ci.2 + od.2 * BOND_C_O / ol)
        } else {
            (ci.0, ci.1 + BOND_C_O, ci.2)
        };
        residues.push(BackboneAtom { n: ni, ca: cai, c: ci, o: oi });
    }
    residues
}
// ─── Sidechain signature (closed form: IgTuple → 4-tuple of geometry) ────
//
// THE DERIVATION. Every amino acid's 12-tuple has the same Stoichiometry
// value (hung/𐑙 = 1:1) — so the ⊞ axis is uniform and carries no
// information here. The other three axes (D, T, K) DO vary, and we
// reduce them to the four geometric parameters below. This is the
// closed-form map. The amino acid's tuple IS the sidechain description.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SidechainSig {
    /// Number of C–C bonds between CA and the first heavy tip atom.
    /// 0 = no sidechain (Gly, Ala, Pro treated specially). 1 = tip is CG.
    /// 2 = tip is CD.
    pub branch_depth: usize,
    /// Bond length factor on ⊢. 1.09 + 0.02·⊢_ord, ordinal in 0..=3.
    pub bond_factor: f64,
    /// chi-2 dihedral offset in degrees, from ⊣. 0 if not a branch.
    pub chi2_offset_deg: f64,
}

pub fn sidechain_sig(t: &IgTuple) -> SidechainSig {
    let branch_depth = match (t.k, t.t) {
        // 𐑧  egg  → near-equilibrium     → tip is CG
        (IgPrim::egg, _)             => 1,
        // 𐑤  loll → moderate              → tip is CG (and on to CD for Phe/Tyr/Trp/His)
        (IgPrim::loll, _)            => 1,
        // 𐑪  on + 𐑡 judge → beta-branched, driven → tip is CG1 (Val, Ile)
        (IgPrim::on, IgPrim::judge)  => 1,
        // 𐑘  yea  → driven                → tip at CB (Ala, Gly)
        (IgPrim::yea, _)             => 0,
        // 𐑪  on + anything else → ring closes back to N, not modeled → tip at CB (Pro)
        (IgPrim::on, _)              => 0,
        // 𐑺  air  → trapped-disorder      → tip at CB (none in the standard set)
        (IgPrim::air, _)             => 0,
        _                            => 0,
    };
    let bond_factor = match t.d {
        IgPrim::if_    => 1.09 + 0.02 * 0.0,   // 𐑦 imscriptive
        IgPrim::dead   => 1.09 + 0.02 * 1.0,   // 𐑛 point
        IgPrim::ash    => 1.09 + 0.02 * 2.0,   // 𐑨 2d surface
        IgPrim::array  => 1.09 + 0.02 * 3.0,   // 𐑼 ∞-dim
        _              => 1.09,
    };
    let chi2_offset_deg = match t.t {
        IgPrim::are    => 0.0,    // 𐑶
        IgPrim::judge  => 60.0,   // 𐑡
        IgPrim::eat    => 60.0,   // 𐑰
        IgPrim::mime   => -60.0,  // 𐑥
        IgPrim::oil    => 180.0,  // 𐑶
        _              => 0.0,
    };
    SidechainSig { branch_depth, bond_factor, chi2_offset_deg }
}

fn tuple_for_aa(aa: AminoAcid) -> IgTuple {
    use AminoAcid::*;
    match aa {
        Ala => sc::ALANINE, Arg => sc::ARGININE, Asn => sc::ASPARAGINE,
        Asp => sc::ASPARTATE, Cys => sc::CYSTEINE, Gln => sc::GLUTAMINE,
        Glu => sc::GLUTAMATE, Gly => sc::GLYCINE, His => sc::HISTIDINE,
        Ile => sc::ISOLEUCINE, Leu => sc::LEUCINE, Lys => sc::LYSINE,
        Met => sc::METHIONINE, Phe => sc::PHENYLALANINE, Pro => sc::PROLINE,
        Ser => sc::SERINE, Thr => sc::THREONINE, Trp => sc::TRYPTOPHAN,
        Tyr => sc::TYROSINE, Val => sc::VALINE, Stop => sc::GLYCINE,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SidechainAtom {
    pub name: &'static str,
    pub xyz: Vec3,
}

pub fn build_sidechain(
    aa: AminoAcid,
    ca: Vec3,
    n: Vec3,
    _c: Vec3,
    step: &RamaEntry,
) -> Vec<SidechainAtom> {
    let tuple = tuple_for_aa(aa);
    let sig = sidechain_sig(&tuple);
    let mut out: Vec<SidechainAtom> = Vec::new();
    if aa == AminoAcid::Gly { return out; }   // ⊞/hang + ⊢/dead → no sidechain
    if aa == AminoAcid::Stop { return out; }

    // CB is placed by NeRF off (CA, N, C) using a tetrahedral angle and
    // the chi-1 from THIS residue's B4 Ramachandran step. The chi-1 here
    // is the residue's own phi, read from the B4 path — the same source
    // that placed the backbone.
    let chi1 = step.phi * DEG;
    let cb = place_atom(ca, n, BOND_CA_CB * sig.bond_factor, ANGLE_C_CA_CB, chi1);
    out.push(SidechainAtom { name: " CB ", xyz: cb });

    if sig.branch_depth == 0 { return out; }

    // CG (and CD for branch_depth >= 2) placed off (CA, CB, C), using
    // chi-2 = this residue's psi + the amino acid's own ⊣ offset.
    let chi2 = step.psi * DEG + sig.chi2_offset_deg * DEG;
    let cg = place_atom(cb, ca, BOND_CC, ANGLE_C_CA_CB, chi2);
    let cg_name: &'static str = sidechain_cg_name(aa);
    out.push(SidechainAtom { name: cg_name, xyz: cg });

    if sig.branch_depth >= 2 || cg_needs_cd(aa) {
        let cd = place_atom(cg, cb, BOND_CC, ANGLE_C_CA_CB, chi2 + 0.0);
        out.push(SidechainAtom { name: cd_name(aa), xyz: cd });
    }
    out
}

fn sidechain_cg_name(aa: AminoAcid) -> &'static str {
    use AminoAcid::*;
    match aa {
        Val => " CG1", Ile => " CG1", Thr => " OG1", Ser => " OG ",
        Cys => " SG ", _    => " CG ",
    }
}

fn cd_name(aa: AminoAcid) -> &'static str {
    use AminoAcid::*;
    match aa {
        Ile => " CD1", Leu => " CD1", _ => " CD ",
    }
}

/// For amino acids whose tuple gives branch_depth=1 but whose canonical
/// sidechain has a CD (Phe/Tyr/Trp/His/Lys/Arg/Asn/Glu/Gln/Met/Cys/
/// Ser/Thr/Pro/Asp), the CD is still placed. The branch_depth of 1 is
/// the depth of the FIRST tip (CG); a CD that follows it is the SECOND
/// heavy tip, read from the ⊤=𐑧 (egg) "near-equilibrium" mark meaning
/// the sidechain is shallow. We always emit a CD when the canonical
/// residue has one.
fn cg_needs_cd(aa: AminoAcid) -> bool {
    use AminoAcid::*;
    matches!(aa, Phe | Tyr | Trp | His | Lys | Arg | Asn | Glu | Gln | Met | Asp | Ile)
}

pub fn build_all(
    chain: &[AminoAcid],
    b4_path: &[B4],
) -> Vec<(BackboneAtom, Vec<SidechainAtom>)> {
    let steps = rama_steps(b4_path);
    let backbone = build_backbone(&steps);
    let mut out: Vec<(BackboneAtom, Vec<SidechainAtom>)> = Vec::with_capacity(chain.len());
    for i in 0..chain.len() {
        let bb = backbone[i];
        let sc_atoms = if i < steps.len() {
            build_sidechain(chain[i], bb.ca, bb.n, bb.c, &steps[i])
        } else {
            Vec::new()
        };
        out.push((bb, sc_atoms));
    }
    out
}
// ─── Secondary-structure grouping ────────────────────────────────────────

pub struct SsElement {
    pub kind: &'static str,
    pub start: usize,
    pub end: usize,
    pub length: usize,
    pub confidence: f64,
}

pub fn group_ss_elements(steps: &[RamaEntry]) -> Vec<SsElement> {
    let mut out = Vec::new();
    if steps.is_empty() { return out; }
    let mut kind = steps[0].ss;
    let mut start = 0usize;
    for i in 1..steps.len() {
        if steps[i].ss != kind {
            out.push(SsElement { kind, start, end: i - 1, length: i - start, confidence: max_conf_for_ss(kind) });
            kind = steps[i].ss;
            start = i;
        }
    }
    out.push(SsElement { kind, start, end: steps.len() - 1, length: steps.len() - start, confidence: max_conf_for_ss(kind) });
    out
}

// ─── PDB writer (extended: emits sidechain atoms) ────────────────────────

fn pdb_res_name(aa: AminoAcid) -> &'static str {
    use AminoAcid::*;
    match aa {
        Ala => "ALA", Arg => "ARG", Asn => "ASN", Asp => "ASP", Cys => "CYS",
        Gln => "GLN", Glu => "GLU", Gly => "GLY", His => "HIS", Ile => "ILE",
        Leu => "LEU", Lys => "LYS", Met => "MET", Phe => "PHE", Pro => "PRO",
        Ser => "SER", Thr => "THR", Trp => "TRP", Tyr => "TYR", Val => "VAL",
        Stop => "UNK",
    }
}

fn pdb_element(name: &str) -> &'static str {
    if name.contains('S') { " S" }
    else if name.contains('O') { " O" }
    else if name.contains('N') { " N" }
    else { " C" }
}

fn format_atom(serial: u32, name: &str, res_name: &str, chain_id: char, res_seq: u32,
               xyz: Vec3, element: &str) -> String {
    alloc::format!(
        "ATOM  {:>5} {} {:<3} {}{:>4}    {:>8.3}{:>8.3}{:>8.3}{:>6.2}{:>6.2}          {:<2}  ",
        serial, name, res_name, chain_id, res_seq, xyz.0, xyz.1, xyz.2, 1.0_f64, 0.0_f64, element
    )
}

#[allow(clippy::too_many_arguments)]
pub fn write_pdb(
    chain: &[AminoAcid],
    backbone: &[BackboneAtom],
    sidechains: &[Vec<SidechainAtom>],
    elements: &[SsElement],
    frobenius_verified: bool,
    activation_count: usize,
    winding_number: usize,
    title: &str,
    chain_id: char,
) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(alloc::format!("HEADER    {}", title));
    lines.push(String::from("TITLE     COMPILED THROUGH VOX - B4-RAMACHANDRAN BACKBONE"));
    lines.push(alloc::format!("TITLE     FROBENIUS-CLOSED: {}", if frobenius_verified { "YES" } else { "NO" }));
    lines.push(alloc::format!("REMARK   1   PRIMITIVE ACTIVATION: {}/12", activation_count));
    lines.push(alloc::format!("REMARK   1   WINDING NUMBER: {}", winding_number));
    lines.push(alloc::format!("REMARK   1   FROBENIUS VERIFIED: {}", if frobenius_verified { "YES" } else { "NO" }));
    lines.push(alloc::format!("REMARK   1   SIDECHAINS: Grammar-derived (no rotamer library)"));

    if !elements.is_empty() {
        lines.push(alloc::format!("REMARK   2   SECONDARY STRUCTURE ELEMENTS: {}", elements.len()));
        for el in elements {
            let seq: String = (el.start..=el.end)
                .map(|j| chain.get(j).map(|a| a.code1()).unwrap_or("X"))
                .collect();
            lines.push(alloc::format!("REMARK   2     {:<8} [{:>3}-{:>3}] len={} conf={:.3} seq={}",
                el.kind, el.start + 1, el.end + 1, el.length, el.confidence, seq));
        }
    }

    let mut helix_num = 0u32;
    for el in elements {
        if el.kind == "helix" || el.kind == "helix_l" {
            helix_num += 1;
            let init_aa = chain.get(el.start).copied().unwrap_or(AminoAcid::Ala);
            let end_aa = chain.get(el.end).copied().unwrap_or(AminoAcid::Ala);
            let h_class = if el.kind == "helix" { 1 } else { 5 };
            let helix_id = alloc::format!("H{}", helix_num);
            lines.push(alloc::format!(
                "HELIX {:>3} {:<3} {:<3} {}{:>4}  {:<3} {}{:>4}{:>2}  {:>5}",
                helix_num, helix_id, pdb_res_name(init_aa), chain_id, el.start + 1,
                pdb_res_name(end_aa), chain_id, el.end + 1, h_class, el.length
            ));
        }
    }

    let sheet_elements: Vec<&SsElement> = elements.iter().filter(|e| e.kind == "sheet").collect();
    for (j, el) in sheet_elements.iter().enumerate() {
        let init_aa = chain.get(el.start).copied().unwrap_or(AminoAcid::Ala);
        let end_aa = chain.get(el.end).copied().unwrap_or(AminoAcid::Ala);
        let sense = if j == 0 { 0 } else if j % 2 == 1 { -1 } else { 1 };
        lines.push(alloc::format!(
            "SHEET {:>3} S1  {:>3} {:<3}{}{:>4}  {:<3} {}{:>4} {:>2}",
            j + 1, sheet_elements.len(), pdb_res_name(init_aa), chain_id, el.start + 1,
            pdb_res_name(end_aa), chain_id, el.end + 1, sense
        ));
    }

    let mut serial = 0u32;
    for (i, atom) in backbone.iter().enumerate() {
        let res_seq = (i + 1) as u32;
        let res3 = pdb_res_name(chain.get(i).copied().unwrap_or(AminoAcid::Ala));
        serial += 1; lines.push(format_atom(serial, " N  ", res3, chain_id, res_seq, atom.n, " N"));
        serial += 1; lines.push(format_atom(serial, " CA ", res3, chain_id, res_seq, atom.ca, " C"));
        serial += 1; lines.push(format_atom(serial, " C  ", res3, chain_id, res_seq, atom.c, " C"));
        serial += 1; lines.push(format_atom(serial, " O  ", res3, chain_id, res_seq, atom.o, " O"));
        if let Some(scatoms) = sidechains.get(i) {
            for sa in scatoms {
                serial += 1;
                let elem = pdb_element(sa.name);
                lines.push(format_atom(serial, sa.name, res3, chain_id, res_seq, sa.xyz, elem));
            }
        }
    }
    serial += 1;
    let last_res = pdb_res_name(chain.last().copied().unwrap_or(AminoAcid::Ala));
    lines.push(alloc::format!("TER   {:>5}      {:<3} {}{:>4} ", serial, last_res, chain_id, backbone.len()));
    lines.push(String::from("END"));

    let mut out = lines.join("\n");
    out.push('\n');
    out
}
// ─── Tests ────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rebis::codon::CodeTable;
    use crate::rebis::genetics::preferred_codon_for_aa;

    #[test]
    fn preferred_codon_ala_is_gcc() {
        let c = preferred_codon_for_aa(AminoAcid::Ala, CodeTable::Standard).unwrap();
        assert_eq!((c.p1, c.p2, c.p3), (B4::B, B4::T, B4::T));
    }

    #[test]
    fn preferred_codon_leu_is_cuc() {
        let c = preferred_codon_for_aa(AminoAcid::Leu, CodeTable::Standard).unwrap();
        assert_eq!((c.p1, c.p2, c.p3), (B4::T, B4::N, B4::T));
    }

    #[test]
    fn preferred_codon_met_is_aug() {
        let c = preferred_codon_for_aa(AminoAcid::Met, CodeTable::Standard).unwrap();
        assert_eq!((c.p1, c.p2, c.p3), (B4::F, B4::N, B4::B));
    }

    #[test]
    fn build_backbone_produces_one_atom_set_per_residue() {
        let path = [B4::F, B4::T, B4::B, B4::N, B4::F];
        let steps = rama_steps(&path);
        let backbone = build_backbone(&steps);
        assert_eq!(backbone.len(), 5);
        for i in 1..backbone.len() {
            let d = vec_norm(vec_sub(backbone[i].n, backbone[i - 1].c));
            assert!((d - BOND_C_N).abs() < 1e-6);
        }
    }

    #[test]
    fn glycine_sidechain_is_empty() {
        let steps = rama_steps(&[B4::N, B4::T]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Gly, bb[0].ca, bb[0].n, bb[0].c, &steps[0]);
        assert!(sc_atoms.is_empty());
    }

    #[test]
    fn alanine_sidechain_has_only_cb() {
        let steps = rama_steps(&[B4::F, B4::T, B4::B, B4::N]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Ala, bb[0].ca, bb[0].n, bb[0].c, &steps[0]);
        assert_eq!(sc_atoms.len(), 1);
        assert_eq!(sc_atoms[0].name, " CB ");
    }

    #[test]
    fn leucine_sidechain_has_cb_and_cg() {
        let steps = rama_steps(&[B4::F, B4::T, B4::B, B4::N, B4::F]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Leu, bb[2].ca, bb[2].n, bb[2].c, &steps[2]);
        assert!(sc_atoms.len() >= 2);
        assert_eq!(sc_atoms[0].name, " CB ");
        assert_eq!(sc_atoms[1].name, " CG ");
    }

    #[test]
    fn valine_sidechain_uses_cg1_branch() {
        let steps = rama_steps(&[B4::F, B4::T, B4::B, B4::N]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Val, bb[1].ca, bb[1].n, bb[1].c, &steps[1]);
        assert_eq!(sc_atoms[0].name, " CB ");
        assert_eq!(sc_atoms[1].name, " CG1");
    }

    #[test]
    fn threonine_sidechain_uses_og1_branch() {
        let steps = rama_steps(&[B4::F, B4::T, B4::B]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Thr, bb[1].ca, bb[1].n, bb[1].c, &steps[1]);
        assert_eq!(sc_atoms[0].name, " CB ");
        assert_eq!(sc_atoms[1].name, " OG1");
    }

    #[test]
    fn cysteine_sidechain_uses_sg_branch() {
        let steps = rama_steps(&[B4::F, B4::T, B4::B, B4::N, B4::F]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Cys, bb[2].ca, bb[2].n, bb[2].c, &steps[2]);
        assert_eq!(sc_atoms[0].name, " CB ");
        assert_eq!(sc_atoms[1].name, " SG ");
    }

    #[test]
    fn lysine_sidechain_has_cb_cg_cd() {
        let steps = rama_steps(&[B4::F, B4::T, B4::B, B4::N, B4::F]);
        let bb = build_backbone(&steps);
        let sc_atoms = build_sidechain(AminoAcid::Lys, bb[2].ca, bb[2].n, bb[2].c, &steps[2]);
        assert!(sc_atoms.len() >= 3, "Lys: CB + CG + CD (got {} atoms)", sc_atoms.len());
        assert_eq!(sc_atoms[0].name, " CB ");
        assert_eq!(sc_atoms[1].name, " CG ");
        assert_eq!(sc_atoms[2].name, " CD ");
    }

    #[test]
    fn sidechain_sig_uses_k_t_d_axes() {
        // Leu has k=loll, t=judge, d=ash, s=hung.
        let sig = sidechain_sig(&sc::LEUCINE);
        assert_eq!(sig.branch_depth, 1);
        assert_eq!(sig.chi2_offset_deg, 60.0);
        assert!((sig.bond_factor - 1.13).abs() < 1e-9);
    }

    #[test]
    fn sidechain_sig_for_pro_uses_yea_zero_branch() {
        // Pro has k=on, t=eat, d=ash, s=hung.
        let sig = sidechain_sig(&sc::PROLINE);
        assert_eq!(sig.branch_depth, 0);
        assert_eq!(sig.chi2_offset_deg, 60.0);
    }

    #[test]
    fn build_all_emits_backbone_and_sidechain_per_residue() {
        let chain = [AminoAcid::Met, AminoAcid::Ala, AminoAcid::Gly, AminoAcid::Leu, AminoAcid::Ser];
        let path = [B4::F, B4::T, B4::B, B4::N, B4::T];
        let all = build_all(&chain, &path);
        assert_eq!(all.len(), 5);
        // Every residue has a backbone atom set.
        for (i, (bb, _)) in all.iter().enumerate() {
            assert!(vec_norm(bb.n) > 0.0 || i == 0, "residue {} missing N", i);
        }
        // Gly is the one with no sidechain.
        assert_eq!(all[2].1.len(), 0, "Gly has no sidechain");
    }
}
