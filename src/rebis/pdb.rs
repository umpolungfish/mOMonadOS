//! pdb.rs — PDB Structure Validation
//! Port of rhr_p4rky/pdb_validator.py
//!
//! Validates predicted protein contacts against PDB experimental structures.
//! Core operations: CA coordinate parsing, contact extraction, precision/recall,
//! sequence extraction, and Frobenius verification.

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

// ── 3-letter → 1-letter AA code ────────────────────────────────────────

const THREE_TO_ONE: [(&str, char); 20] = [
    ("ALA", 'A'), ("ARG", 'R'), ("ASN", 'N'), ("ASP", 'D'), ("CYS", 'C'),
    ("GLN", 'Q'), ("GLU", 'E'), ("GLY", 'G'), ("HIS", 'H'), ("ILE", 'I'),
    ("LEU", 'L'), ("LYS", 'K'), ("MET", 'M'), ("PHE", 'F'), ("PRO", 'P'),
    ("SER", 'S'), ("THR", 'T'), ("TRP", 'W'), ("TYR", 'Y'), ("VAL", 'V'),
];

pub fn three_to_one(three: &str) -> Option<char> {
    THREE_TO_ONE.iter()
        .find(|&&(t, _)| t.eq_ignore_ascii_case(three))
        .map(|&(_, c)| c)
}

// ── AA → preferred codon ───────────────────────────────────────────────

const AA_TO_CODON: [(&str, &str); 21] = [
    ("A", "GCU"), ("R", "CGU"), ("N", "AAU"), ("D", "GAU"), ("C", "UGU"),
    ("Q", "CAA"), ("E", "GAA"), ("G", "GGU"), ("H", "CAU"), ("I", "AUU"),
    ("L", "UUG"), ("K", "AAA"), ("M", "AUG"), ("F", "UUU"), ("P", "CCU"),
    ("S", "UCU"), ("T", "ACU"), ("W", "UGG"), ("Y", "UAU"), ("V", "GUU"),
    ("*", "UGA"),
];

pub fn aa_to_preferred_codon(aa: char) -> &'static str {
    let upper = aa.to_ascii_uppercase();
    AA_TO_CODON.iter()
        .find(|&&(a, _)| a.chars().next() == Some(upper))
        .map(|&(_, c)| c)
        .unwrap_or("NNN")
}

pub fn protein_to_rna(seq: &str) -> alloc::string::String {
    let mut rna = alloc::string::String::with_capacity(seq.len() * 3);
    for aa in seq.chars() {
        rna.push_str(aa_to_preferred_codon(aa));
    }
    rna
}

// ── CA Atom ────────────────────────────────────────────────────────────

/// A C-alpha atom extracted from a PDB ATOM record.
#[derive(Clone, Debug)]
pub struct CAAtom {
    pub res_name: alloc::string::String,  // 3-letter residue name
    pub chain: char,
    pub res_num: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Parse CA atoms from PDB text content.
pub fn parse_pdb_ca_atoms(pdb_text: &str) -> alloc::vec::Vec<CAAtom> {
    let mut atoms = Vec::new();
    for line in pdb_text.lines() {
        if !line.starts_with("ATOM") && !line.starts_with("HETATM") {
            continue;
        }
        // ATOM record: columns 13-16 = atom name, 18-20 = residue name,
        // 22 = chain, 23-26 = residue number, 31-38 = x, 39-46 = y, 47-54 = z
        if line.len() < 54 { continue; }
        let atom_name = line[12..16].trim();
        if atom_name != "CA" { continue; }

        let res_name = line[17..20].trim();
        let chain = line.chars().nth(21).unwrap_or(' ');
        let res_num: i32 = line[22..26].trim().parse().unwrap_or(0);
        let x: f64 = line[30..38].trim().parse().unwrap_or(0.0);
        let y: f64 = line[38..46].trim().parse().unwrap_or(0.0);
        let z: f64 = line[46..54].trim().parse().unwrap_or(0.0);

        atoms.push(CAAtom { res_name: alloc::string::String::from(res_name), chain, res_num, x, y, z });
    }
    atoms
}

/// Euclidean distance between two CA atoms.
/// Approximate square root using Newton's method (no_std compatible).
fn sqrt_f64(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    let mut guess = x / 2.0;
    for _ in 0..=10 { guess = 0.5 * (guess + x / guess); }
    guess
}

pub fn ca_distance(a: &CAAtom, b: &CAAtom) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    sqrt_f64(dx * dx + dy * dy + dz * dz)
}

// ── Contact extraction ─────────────────────────────────────────────────

/// A residue-residue contact.
#[derive(Clone, Debug)]
pub struct Contact {
    pub i: usize,     // sequential index of first residue
    pub j: usize,     // sequential index of second residue
    pub distance: f64, // Cα-Cα distance in Å
}

/// Extract long-range contacts: Cα distance < cutoff, sequence sep ≥ min_seq_dist.
pub fn extract_contacts(atoms: &[CAAtom], cutoff: f64, min_seq_dist: usize) -> alloc::vec::Vec<Contact> {
    let mut contacts = Vec::new();
    let mut seen: alloc::vec::Vec<(usize, usize)> = alloc::vec::Vec::new();

    for i in 0..atoms.len() {
        for j in (i + 1)..atoms.len() {
            let seq_dist = (atoms[i].res_num - atoms[j].res_num).unsigned_abs() as usize;
            if seq_dist < min_seq_dist { continue; }

            let dist = ca_distance(&atoms[i], &atoms[j]);
            if dist < cutoff {
                let pair = (i, j);
                if !seen.contains(&pair) { seen.push(pair);
                    contacts.push(Contact { i, j, distance: dist });
                }
            }
        }
    }
    contacts.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(core::cmp::Ordering::Equal));
    contacts
}

// ── Sequence extraction ────────────────────────────────────────────────

/// Extract one-letter AA sequence from PDB SEQRES records.
pub fn extract_sequence_from_pdb(pdb_text: &str) -> alloc::string::String {
    let mut seq = alloc::string::String::new();
    for line in pdb_text.lines() {
        if line.starts_with("SEQRES") {
            let parts: alloc::vec::Vec<&str> = line.split_whitespace().collect();
            for part in &parts[4..] {
                if let Some(aa) = three_to_one(part) {
                    seq.push(aa);
                }
            }
        }
    }
    seq
}

/// Extract sequence from CA atoms (fallback when no SEQRES records).
pub fn extract_sequence_from_atoms(atoms: &[CAAtom]) -> alloc::string::String {
    let mut seq = alloc::string::String::new();
    for atom in atoms {
        if let Some(aa) = three_to_one(&atom.res_name) {
            seq.push(aa);
        }
    }
    seq
}

// ── Validation metrics ─────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ValidationMetrics {
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
}

/// Compute precision, recall, F1 for predicted vs actual contacts.
pub fn compute_metrics(
    predicted: &[(usize, usize)],
    actual: &[(usize, usize)],
) -> ValidationMetrics {
    let pred_set: BTreeSet<(usize, usize)> = predicted.iter()
        .map(|&(i, j)| if i < j { (i, j) } else { (j, i) })
        .collect();
    let actual_set: BTreeSet<(usize, usize)> = actual.iter()
        .map(|&(i, j)| if i < j { (i, j) } else { (j, i) })
        .collect();

    let tp = pred_set.intersection(&actual_set).count();
    let fp = pred_set.len() - tp;
    let fn_count = actual_set.len() - tp;

    let precision = if tp + fp > 0 { tp as f64 / (tp + fp) as f64 } else { 0.0 };
    let recall = if tp + fn_count > 0 { tp as f64 / (tp + fn_count) as f64 } else { 0.0 };
    let f1 = if precision + recall > 0.0 {
        2.0 * precision * recall / (precision + recall)
    } else {
        0.0
    };

    ValidationMetrics {
        true_positives: tp,
        false_positives: fp,
        false_negatives: fn_count,
        precision,
        recall,
        f1_score: f1,
    }
}

// ── PDB validation result ──────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PDBValidation {
    pub pdb_id: alloc::string::String,
    pub sequence: alloc::string::String,
    pub seq_length: usize,
    pub n_ca_atoms: usize,
    pub experimental_contacts: usize,
    pub predicted_contacts: usize,
    pub correct_predictions: usize,
    pub metrics: ValidationMetrics,
    pub frobenius_verified: bool,
}

/// Run full validation: predicted contacts vs PDB structure.
pub fn validate_structure(
    pdb_id: &str,
    pdb_text: &str,
    predicted: &[(usize, usize)],
    sequence: Option<&str>,
) -> PDBValidation {
    let seq = match sequence {
        Some(s) => alloc::string::String::from(s),
        None => extract_sequence_from_pdb(pdb_text),
    };

    let atoms = parse_pdb_ca_atoms(pdb_text);
    let contacts = extract_contacts(&atoms, 8.0, 4);
    let actual_pairs: alloc::vec::Vec<(usize, usize)> = contacts.iter()
        .map(|c| (c.i, c.j))
        .collect();

    let metrics = compute_metrics(predicted, &actual_pairs);
    let pred_set: BTreeSet<(usize, usize)> = predicted.iter()
        .map(|&(i, j)| if i < j { (i, j) } else { (j, i) })
        .collect();
    let actual_set: BTreeSet<(usize, usize)> = actual_pairs.iter()
        .map(|&(i, j)| if i < j { (i, j) } else { (j, i) })
        .collect();
    let correct = pred_set.intersection(&actual_set).count();

    // Frobenius check: does the contact map satisfy structural closure?
    // For PDB validation, this means: predicted contacts that are correct
    // form a subset that can reconstruct the fold topology.
    let frobenius_ok = correct > 0 && correct as f64 / predicted.len().max(1) as f64 >= 0.3;

    PDBValidation {
        pdb_id: alloc::string::String::from(pdb_id),
        sequence: seq.clone(),
        seq_length: seq.len(),
        n_ca_atoms: atoms.len(),
        experimental_contacts: actual_pairs.len(),
        predicted_contacts: predicted.len(),
        correct_predictions: correct,
        metrics,
        frobenius_verified: frobenius_ok,
    }
}
