// rebis/fold3d.rs — B4→Ramachandran→Cartesian backbone reconstruction and a
// real PDB writer.
//
// Port of red-hot_rebis's rhr_p4rky/serpent_rod_v2.py (B4_RAMACHANDRAN table,
// build_frame/place_atom/build_backbone) and rhr_p4rky/pdb_writer.py (ATOM/
// HELIX/SHEET/TER record formatting). fold.rs already gives real Chou-Fasman
// secondary structure and heuristic contacts from sequence alone; this adds
// the one thing that was still missing — actual 3D backbone coordinates,
// derived the same way red-hot_rebis derives them: one B4 value per residue
// steps the Ramachandran table, whose (phi, psi) angles place each atom by
// NeRF internal-to-Cartesian construction.
//
// One deliberate departure from the Python source: it indexes its B4 path
// per-nucleotide but reads it with the per-residue loop index, an off-by-
// factor-of-3 mismatch. Here every residue already has its own real B4 (the
// first position of the codon that produced it, forward or reconstructed),
// so the lookup is direct instead of reusing that mismatch.

use crate::belnap::B4;
use crate::rebis::AminoAcid;
use alloc::string::String;
use alloc::vec::Vec;
use libm::sqrt;

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
const BOND_C_N: f64 = 1.329;
const BOND_C_O: f64 = 1.231;
const ANGLE_N_CA_C: f64 = 111.0 * DEG;
const ANGLE_CA_C_N: f64 = 116.2 * DEG;
const ANGLE_C_N_CA: f64 = 121.7 * DEG;

/// The Ramachandran step for one B4→B4 transition: the exact 16-entry table
/// from serpent_rod_v2.py's B4_RAMACHANDRAN, keyed by (from, to).
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
            if e.ss == ss {
                found = true;
                if e.conf > m { m = e.conf; }
            }
        }
    }
    if found { m } else { 0.5 }
}

/// One Ramachandran step per residue: residue 0's "from" is the fictitious
/// N predecessor (matching serpent_rod_v2.py's i==0 special case), every
/// later residue's "from" is the previous residue's own B4.
pub fn rama_steps(b4_path: &[B4]) -> Vec<RamaEntry> {
    let mut out = Vec::with_capacity(b4_path.len());
    for i in 0..b4_path.len() {
        let from = if i == 0 { B4::N } else { b4_path[i - 1] };
        out.push(ramachandran(from, b4_path[i]));
    }
    out
}

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
        bond_len * libm::sin(bond_angle) * libm::cos(dihedral),
        bond_len * libm::sin(bond_angle) * libm::sin(dihedral),
        -bond_len * libm::cos(bond_angle),
    );
    (
        prev.0 + local.0 * x.0 + local.1 * y.0 + local.2 * z.0,
        prev.1 + local.0 * x.1 + local.1 * y.1 + local.2 * z.1,
        prev.2 + local.0 * x.2 + local.1 * y.2 + local.2 * z.2,
    )
}

pub struct BackboneAtom {
    pub n: Vec3,
    pub ca: Vec3,
    pub c: Vec3,
    pub o: Vec3,
}

/// Build the real 3D backbone (N, CA, C, O per residue) from a Ramachandran
/// step per residue, by the same NeRF internal→Cartesian construction
/// serpent_rod_v2.py's build_backbone uses.
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

/// A contiguous run of one Ramachandran ss label, for HELIX/SHEET records —
/// this describes the same geometry the backbone was built from, distinct
/// from fold.rs's own Chou-Fasman secondary-structure call.
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

fn pdb_res_name(aa: AminoAcid) -> &'static str {
    match aa {
        AminoAcid::Ala => "ALA", AminoAcid::Arg => "ARG", AminoAcid::Asn => "ASN",
        AminoAcid::Asp => "ASP", AminoAcid::Cys => "CYS", AminoAcid::Gln => "GLN",
        AminoAcid::Glu => "GLU", AminoAcid::Gly => "GLY", AminoAcid::His => "HIS",
        AminoAcid::Ile => "ILE", AminoAcid::Leu => "LEU", AminoAcid::Lys => "LYS",
        AminoAcid::Met => "MET", AminoAcid::Phe => "PHE", AminoAcid::Pro => "PRO",
        AminoAcid::Ser => "SER", AminoAcid::Thr => "THR", AminoAcid::Trp => "TRP",
        AminoAcid::Tyr => "TYR", AminoAcid::Val => "VAL", AminoAcid::Stop => "UNK",
    }
}

// Columns match the wwPDB v3.3 ATOM spec exactly (same layout pdb_writer.py's
// ATOM_FMT uses), which is also exactly what rebis::pdb::parse_pdb_ca_atoms
// reads back: name at 12..16, res name at 17..20, chain at 21, res_seq at
// 22..26, x/y/z at 30..38/38..46/46..54. A PDB this writer produces round-
// trips through mOMonadOS's own PDB reader.
fn format_atom(serial: u32, name: &str, res_name: &str, chain_id: char, res_seq: u32,
               xyz: Vec3, temp: f64, element: &str) -> String {
    alloc::format!(
        "ATOM  {:>5} {} {:<3} {}{:>4}    {:>8.3}{:>8.3}{:>8.3}{:>6.2}{:>6.2}          {:<2}  ",
        serial, name, res_name, chain_id, res_seq, xyz.0, xyz.1, xyz.2, 1.0_f64, temp, element
    )
}

/// Write a full PDB structure file from a real backbone: HEADER/TITLE,
/// REMARK activation/winding/subunit summary, HELIX/SHEET from the
/// Ramachandran-derived secondary structure, ATOM records for every
/// backbone N/CA/C/O, TER, END. Port of pdb_writer.py's write_pdb_from_gen2.
#[allow(clippy::too_many_arguments)]
pub fn write_pdb(
    chain: &[AminoAcid],
    backbone: &[BackboneAtom],
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
        serial += 1; lines.push(format_atom(serial, " N  ", res3, chain_id, res_seq, atom.n, 0.0, " N"));
        serial += 1; lines.push(format_atom(serial, " CA ", res3, chain_id, res_seq, atom.ca, 0.0, " C"));
        serial += 1; lines.push(format_atom(serial, " C  ", res3, chain_id, res_seq, atom.c, 0.0, " C"));
        serial += 1; lines.push(format_atom(serial, " O  ", res3, chain_id, res_seq, atom.o, 0.0, " O"));
    }
    serial += 1;
    let last_res = pdb_res_name(chain.last().copied().unwrap_or(AminoAcid::Ala));
    lines.push(alloc::format!("TER   {:>5}      {:<3} {}{:>4} ", serial, last_res, chain_id, backbone.len()));
    lines.push(String::from("END"));

    let mut out = lines.join("\n");
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rebis::codon::CodeTable;
    use crate::rebis::genetics::preferred_codon_for_aa;

    #[test]
    fn preferred_codon_ala_is_gcc_the_position3_c_exact_variant() {
        let c = preferred_codon_for_aa(AminoAcid::Ala, CodeTable::Standard).unwrap();
        assert_eq!((c.p1, c.p2, c.p3), (B4::B, B4::T, B4::T), "GCC: G,C,C");
    }

    #[test]
    fn preferred_codon_leu_is_cuc_the_fourfold_box_not_the_exact_tier() {
        // Leu's codons all have middle base U, so tier 1 (middle=C) is empty;
        // CUN is a real 4-fold box (CUU/CUC/CUA/CUG all -> Leu), so tier 2
        // picks its position-3 C member.
        let c = preferred_codon_for_aa(AminoAcid::Leu, CodeTable::Standard).unwrap();
        assert_eq!((c.p1, c.p2, c.p3), (B4::T, B4::N, B4::T), "CUC: C,U,C");
    }

    #[test]
    fn preferred_codon_met_is_its_only_codon() {
        let c = preferred_codon_for_aa(AminoAcid::Met, CodeTable::Standard).unwrap();
        assert_eq!((c.p1, c.p2, c.p3), (B4::F, N_U, B4::B), "AUG: A,U,G");
    }

    // B4::N is the nucleotide U (see codon::b4_to_nucleotide) — named here
    // only so the Met test above reads as A,U,G rather than an opaque B4::N.
    #[allow(non_upper_case_globals)]
    const N_U: B4 = B4::N;

    #[test]
    fn build_backbone_produces_one_atom_set_per_residue() {
        let path = [B4::F, B4::T, B4::B, B4::N, B4::F];
        let steps = rama_steps(&path);
        let backbone = build_backbone(&steps);
        assert_eq!(backbone.len(), 5);
        // Every bond length actually built is the real bond length, not a
        // degenerate zero-length placement.
        for i in 1..backbone.len() {
            let d = vec_norm(vec_sub(backbone[i].n, backbone[i - 1].c));
            assert!((d - BOND_C_N).abs() < 1e-6, "N-C(prev) bond should be {} A, got {}", BOND_C_N, d);
        }
    }

    #[test]
    fn written_pdb_round_trips_through_mOMonadOS_own_pdb_reader() {
        use crate::rebis::pdb::parse_pdb_ca_atoms;
        let chain = [AminoAcid::Met, AminoAcid::Ala, AminoAcid::Gly, AminoAcid::Leu, AminoAcid::Ser];
        let path = [B4::F, B4::T, B4::B, B4::N, B4::T];
        let steps = rama_steps(&path);
        let backbone = build_backbone(&steps);
        let elements = group_ss_elements(&steps);
        let pdb = write_pdb(&chain, &backbone, &elements, true, 3, 2, "TEST", 'A');

        let atoms = parse_pdb_ca_atoms(&pdb);
        assert_eq!(atoms.len(), chain.len(), "one CA atom per residue must come back out");
        for (i, atom) in atoms.iter().enumerate() {
            assert_eq!(atom.res_name, pdb_res_name(chain[i]));
            assert_eq!(atom.chain, 'A');
            assert_eq!(atom.res_num, (i + 1) as i32);
            let expect = backbone[i].ca;
            assert!((atom.x - expect.0).abs() < 1e-3);
            assert!((atom.y - expect.1).abs() < 1e-3);
            assert!((atom.z - expect.2).abs() < 1e-3);
        }
    }
}
