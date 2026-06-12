//! antibody.rs — Antibody CDR Design via 12↔12 IG Primitive Bijection
//! Port of rhr_p4rky/antibody_designer.py
//!
//! Given a target epitope's IG primitive activation profile, design
//! complementary CDR sequences using the 12↔12 complementarity pairs:
//!
//!   D↔Ω, T↔H, R↔S, P↔F, K↔G, Gm↔φ̂
//!
//! If the target epitope activates primitive P, the CDR should activate
//! its complement to form a structural contact.

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;
use super::genetic_tuples::{aa_activation, AAActivation};

// ── 12↔12 Primitive Bijection ──────────────────────────────────────────

/// IG primitive name → complementary primitive name.
pub fn complementary_primitive(prim: &str) -> Option<&'static str> {
    match prim {
        "D" | "Ð" => Some("O"),
        "T" | "Þ" => Some("H"),
        "R" | "Ř" => Some("S"),
        "P" | "Φ" => Some("F"),
        "F" | "ƒ" => Some("P"),
        "K" | "Ç" => Some("G"),
        "G" | "Γ" => Some("K"),
        "Gm" | "ɢ" => Some("Phi"),
        "Phi" | "φ̂" => Some("Gm"),
        "H" | "Ħ" => Some("T"),
        "S" | "Σ" => Some("R"),
        "O" | "Ω" => Some("D"),
        _ => None,
    }
}

/// IG primitive name → preferred AA for CDR complement.
pub fn primitive_to_aa(prim: &str) -> Option<char> {
    match prim {
        "D" | "Ð" => Some('M'),  // Met
        "T" | "Þ" => Some('W'),  // Trp
        "R" | "Ř" => Some('C'),  // Cys
        "P" | "Φ" => Some('Y'),  // Tyr
        "F" | "ƒ" => Some('F'),  // Phe
        "K" | "Ç" => Some('I'),  // Ile
        "G" | "Γ" => Some('H'),  // His
        "Gm" | "ɢ" => Some('N'), // Asn
        "Phi" | "φ̂" => Some('Q'), // Gln
        "H" | "Ħ" => Some('D'),  // Asp
        "S" | "Σ" => Some('K'),  // Lys
        "O" | "Ω" => Some('E'),  // Glu
        _ => None,
    }
}

/// AA one-letter code → 3-letter name.
pub fn one_letter_to_three(aa: char) -> &'static str {
    match aa.to_ascii_uppercase() {
        'A' => "Ala", 'R' => "Arg", 'N' => "Asn", 'D' => "Asp",
        'C' => "Cys", 'Q' => "Gln", 'E' => "Glu", 'G' => "Gly",
        'H' => "His", 'I' => "Ile", 'L' => "Leu", 'K' => "Lys",
        'M' => "Met", 'F' => "Phe", 'P' => "Pro", 'S' => "Ser",
        'T' => "Thr", 'W' => "Trp", 'Y' => "Tyr", 'V' => "Val",
        _ => "???",
    }
}

/// AA three-letter name → one-letter code.
pub fn three_to_one_letter(three: &str) -> Option<char> {
    match three.to_ascii_uppercase().as_str() {
        "ALA" => Some('A'), "ARG" => Some('R'), "ASN" => Some('N'),
        "ASP" => Some('D'), "CYS" => Some('C'), "GLN" => Some('Q'),
        "GLU" => Some('E'), "GLY" => Some('G'), "HIS" => Some('H'),
        "ILE" => Some('I'), "LEU" => Some('L'), "LYS" => Some('K'),
        "MET" => Some('M'), "PHE" => Some('F'), "PRO" => Some('P'),
        "SER" => Some('S'), "THR" => Some('T'), "TRP" => Some('W'),
        "TYR" => Some('Y'), "VAL" => Some('V'),
        _ => None,
    }
}

// ── Epitope analysis ───────────────────────────────────────────────────

/// Result of analyzing an epitope's IG primitive activation profile.
#[derive(Clone, Debug)]
pub struct EpitopeAnalysis {
    pub name: alloc::string::String,
    pub sequence: alloc::string::String,
    pub seq_length: usize,
    pub activations: alloc::vec::Vec<ActivationSite>,
    pub activated_primitives: alloc::vec::Vec<alloc::string::String>,
}

/// A single residue's IG primitive activation.
#[derive(Clone, Debug)]
pub struct ActivationSite {
    pub position: usize,
    pub aa: char,
    pub primitive: alloc::string::String,
    pub is_activated: bool,
}

/// Analyze an epitope sequence for IG primitive activations.
/// Scans each AA and maps its activation to the complementary primitive.
pub fn analyze_epitope(epitope_seq: &str, name: &str) -> EpitopeAnalysis {
    let mut activations = Vec::new();
    let mut activated_set: BTreeSet<alloc::string::String> = BTreeSet::new();

    for (i, aa) in epitope_seq.chars().enumerate() {
        let act = aa_activation(aa);
        let mut prim_name = alloc::string::String::from("none");
        let mut is_activated = false;

        // Check which primitive this AA activates
        if act.k_activates.is_some() {
            prim_name = alloc::string::String::from("K");
            is_activated = true;
        } else if act.h_activates.is_some() {
            prim_name = alloc::string::String::from("H");
            is_activated = true;
        } else if act.phi_activates.is_some() {
            prim_name = alloc::string::String::from("Phi");
            is_activated = true;
        } else if act.d_activates.is_some() {
            prim_name = alloc::string::String::from("D");
            is_activated = true;
        } else if act.s_activates.is_some() {
            prim_name = alloc::string::String::from("S");
            is_activated = true;
        }

        if is_activated {
            activated_set.insert(prim_name.clone());
        }

        activations.push(ActivationSite {
            position: i,
            aa,
            primitive: prim_name,
            is_activated,
        });
    }

    EpitopeAnalysis {
        name: alloc::string::String::from(name),
        sequence: alloc::string::String::from(epitope_seq),
        seq_length: epitope_seq.len(),
        activations,
        activated_primitives: activated_set.into_iter().collect(),
    }
}

// ── CDR Design ─────────────────────────────────────────────────────────

/// A designed CDR loop.
#[derive(Clone, Debug)]
pub struct CDRDesign {
    pub cdr_sequence: alloc::string::String,
    pub cdr_rna: alloc::string::String,
    pub composition: alloc::vec::Vec<CDRPosition>,
    pub length: usize,
}

/// One position in the designed CDR.
#[derive(Clone, Debug)]
pub struct CDRPosition {
    pub position: usize,
    pub aa: char,
    pub aa_three: alloc::string::String,
    pub primitive: alloc::string::String,
}

/// Framework AAs for padding CDR to target length.
const FRAMEWORK_AAS: [char; 5] = ['G', 'S', 'T', 'A', 'V']; // Gly, Ser, Thr, Ala, Val

/// Design a CDR sequence complementary to the epitope's activation profile.
pub fn design_cdr(epitope: &EpitopeAnalysis, length: usize) -> CDRDesign {
    let mut cdr_aa: alloc::vec::Vec<char> = Vec::new();
    let mut cdr_prim: alloc::vec::Vec<alloc::string::String> = Vec::new();

    // For each activated primitive, add the complementary AA
    for prim in &epitope.activated_primitives {
        if let Some(comp_prim) = complementary_primitive(prim) {
            if let Some(aa) = primitive_to_aa(comp_prim) {
                cdr_aa.push(aa);
                cdr_prim.push(alloc::string::String::from(comp_prim));
            }
        }
    }

    // Pad to target length with framework residues
    while cdr_aa.len() < length {
        let idx = cdr_aa.len();
        cdr_aa.push(FRAMEWORK_AAS[idx % FRAMEWORK_AAS.len()]);
        cdr_prim.push(alloc::string::String::from("framework"));
    }

    // Truncate if needed
    cdr_aa.truncate(length);
    cdr_prim.truncate(length);

    let cdr_sequence: alloc::string::String = cdr_aa.iter().collect();
    let cdr_rna: alloc::string::String = cdr_aa.iter()
        .map(|&aa| super::pdb::aa_to_preferred_codon(aa))
        .collect::<alloc::vec::Vec<&str>>()
        .concat();

    let composition: alloc::vec::Vec<CDRPosition> = cdr_aa.iter().zip(cdr_prim.iter()).enumerate()
        .map(|(i, (&aa, prim))| CDRPosition {
            position: i,
            aa,
            aa_three: alloc::string::String::from(one_letter_to_three(aa)),
            primitive: prim.clone(),
        })
        .collect();

    CDRDesign { cdr_sequence, cdr_rna, composition, length }
}

// ── Full Antibody Construction ─────────────────────────────────────────

/// Canonical CDR lengths (Kabat numbering).
pub const CDR_CANONICAL_LENGTHS: [(&str, usize); 6] = [
    ("VH_CDR1", 7), ("VH_CDR2", 8), ("VH_CDR3", 12),
    ("VL_CDR1", 9), ("VL_CDR2", 7), ("VL_CDR3", 9),
];

/// Framework regions for VH domain.
pub struct AntibodyFramework {
    pub fr1: &'static str,
    pub fr2: &'static str,
    pub fr3: &'static str,
    pub fr4: &'static str,
}

pub const VH_FRAMEWORK: AntibodyFramework = AntibodyFramework {
    fr1: "QVQLVQSGAEVKKPGASVKVSCKASGYTFT",
    fr2: "WVRQAPGQGLEWMG",
    fr3: "RVTMTRDTSTSTVYMELSSLRSEDTAVYYCAR",
    fr4: "WGQGTLVTVSS",
};

pub const VL_FRAMEWORK: AntibodyFramework = AntibodyFramework {
    fr1: "DIQMTQSPSSLSASVGDRVTITC",
    fr2: "WYQQKPGKAPKLLIY",
    fr3: "GVPSRFSGSGSGTDFTFTISSLQPEDIATYYC",
    fr4: "FGQGTKVEIK",
};

/// A designed antibody variable domain.
#[derive(Clone, Debug)]
pub struct AntibodyDomain {
    pub chain_type: alloc::string::String,
    pub full_sequence: alloc::string::String,
    pub fr1: alloc::string::String,
    pub cdr3: CDRDesign,
    pub fr4: alloc::string::String,
}

/// Design a complete antibody variable domain targeting the epitope.
pub fn design_full_antibody(
    epitope: &EpitopeAnalysis,
    chain_type: &str,
    cdr_length: Option<usize>,
) -> AntibodyDomain {
    let length = cdr_length.unwrap_or(12);
    let cdr = design_cdr(epitope, length);

    let fw: &AntibodyFramework = if chain_type == "VL" {
        &VL_FRAMEWORK
    } else {
        &VH_FRAMEWORK
    };

    let full_seq = alloc::format!("{}{}{}", fw.fr3, cdr.cdr_sequence, fw.fr4);

    AntibodyDomain {
        chain_type: alloc::string::String::from(chain_type),
        full_sequence: full_seq,
        fr1: alloc::string::String::from(fw.fr3),
        cdr3: cdr,
        fr4: alloc::string::String::from(fw.fr4),
    }
}

/// Known viral epitopes for testing.
pub struct ViralEpitope {
    pub name: &'static str,
    pub sequence: &'static str,
}

pub const VIRAL_EPITOPES: [ViralEpitope; 4] = [
    ViralEpitope {
        name: "SARS-CoV-2_RBD",
        sequence: "KVGGNYNYLYRLFRKSNLKPFERDISTE",
    },
    ViralEpitope {
        name: "HIV_gp120_C4",
        sequence: "KCNNKTFNGTGPCTNVSTVQCTHGIRPV",
    },
    ViralEpitope {
        name: "Influenza_HA_stem",
        sequence: "WLLWISFAISCFLLCVVLLGFISFAIS",
    },
    ViralEpitope {
        name: "HPV_L1_capsid",
        sequence: "DTPDNKEYPDEYSDTYGDTYDWTD",
    },
];

/// Compute the IG complementarity score between epitope and CDR.
/// Higher score = better structural complementarity.
pub fn complementarity_score(epitope: &EpitopeAnalysis, cdr: &CDRDesign) -> f64 {
    let mut score = 0.0;
    let mut n_pairs = 0;

    for act in &epitope.activations {
        if !act.is_activated { continue; }
        if let Some(comp_prim) = complementary_primitive(&act.primitive) {
            // Check if CDR contains the complementary AA
            for pos in &cdr.composition {
                if pos.primitive == comp_prim {
                    score += 1.0;
                    n_pairs += 1;
                    break;
                }
            }
        }
    }

    if n_pairs > 0 {
        score / n_pairs as f64
    } else {
        0.0
    }
}
