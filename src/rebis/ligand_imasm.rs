//! ligand_imasm.rs — IMASM Active-Site Encoding
//!
//! Encodes enzyme active sites as IMASM arrangements by mapping each
//! catalytic residue to an IMASM token based on chemical properties
//! (charge, polarity, aromaticity, nucleophilicity, size, flexibility).
//!
//! Ported from red-hot_rebis/rhr_p4rky/ligand_imasm.py
//! Chemical-property-based mapping (not role-based), preserving N→C topology.
//!
//! Structural type: ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑙𐑭⟩ — O_∞ self-referential

use crate::tokens::Token;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// IMASM token assignment per amino acid (3-letter code).
/// Chemical-property-based, NOT role-based.
pub const AA_TO_TOKEN: &[(&str, Token)] = &[
    // Acidic (negative charge) → EVALF
    ("ASP", Token::EVALF),
    ("GLU", Token::EVALF),
    // Basic (positive charge) → EVALT
    ("HIS", Token::EVALT),
    ("LYS", Token::EVALT),
    // Guanidinium (planar rigid H-bond donor) → TANCH
    ("ARG", Token::TANCH),
    // Nucleophilic (attacks electrophilic centers) → FSPLIT
    ("SER", Token::FSPLIT),
    ("CYS", Token::FSPLIT),
    ("THR", Token::FSPLIT),
    // Aromatic (multi-mode: H-bond + π-stack) → ENGAGR
    ("TYR", Token::ENGAGR),
    ("TRP", Token::ENGAGR),
    // Aromatic hydrophobic (directional π-stacking) → AFWD
    ("PHE", Token::AFWD),
    // Polar amide (bidirectional H-bond) → AREV
    ("ASN", Token::AREV),
    ("GLN", Token::AREV),
    // Flexible (minimal steric constraint) → VINIT
    ("GLY", Token::VINIT),
    // Small hydrophobic (methyl) → CLINK
    ("ALA", Token::CLINK),
    // Branched hydrophobic (rigid, sterically constrained) → IFIX
    ("VAL", Token::IFIX),
    ("LEU", Token::IFIX),
    ("ILE", Token::IFIX),
    // Thioether (polarizable sulfur) → AREV
    ("MET", Token::AREV),
    // Cyclic (conformationally constrained) → IMSCRIB
    ("PRO", Token::IMSCRIB),
];

/// Chemical class label for each amino acid.
pub const AA_CHEMICAL_CLASS: &[(&str, &str)] = &[
    ("ASP", "acidic"),           ("GLU", "acidic"),
    ("HIS", "basic"),            ("LYS", "basic"),
    ("ARG", "guanidinium"),
    ("SER", "nucleophile"),      ("CYS", "nucleophile"),    ("THR", "nucleophile"),
    ("TYR", "aromatic"),         ("TRP", "aromatic"),
    ("PHE", "aromatic_hydrophobic"),
    ("ASN", "polar_amide"),      ("GLN", "polar_amide"),
    ("GLY", "flexible"),
    ("ALA", "small_hydrophobic"),
    ("VAL", "branched_h"),       ("LEU", "branched_h"),     ("ILE", "branched_h"),
    ("MET", "thioether"),
    ("PRO", "cyclic"),
];

/// Look up IMASM token for a 3-letter amino acid code.
pub fn aa_to_token(code: &str) -> Option<Token> {
    for &(aa, tok) in AA_TO_TOKEN {
        if aa == code { return Some(tok); }
    }
    None
}

/// Look up chemical class for a 3-letter amino acid code.
pub fn aa_chemical_class(code: &str) -> &'static str {
    for &(aa, cls) in AA_CHEMICAL_CLASS {
        if aa == code { return cls; }
    }
    "unknown"
}

/// Result of encoding an active site as an IMASM arrangement.
#[derive(Debug, Clone)]
pub struct ActiveSiteEncoding {
    /// The 8-token IMASM arrangement (padded with IMSCRIB if <8 residues).
    pub tokens: Vec<Token>,
    /// Per-residue assignments for diagnostics.
    pub assignments: Vec<ResidueAssignment>,
}

#[derive(Debug, Clone)]
pub struct ResidueAssignment {
    pub residue_label: String,  // e.g. "Ser195"
    pub aa_code: String,        // e.g. "SER"
    pub token: Token,
    pub chemical_class: &'static str,
}

/// Encode an active site as an IMASM arrangement.
///
/// Pipeline:
///   1. Parse each residue string (e.g. "Ser195" → "SER")
///   2. Map to IMASM token via AA_TO_TOKEN (chemical-property-based)
///   3. Order N→C (preserving input order = spatial topology)
///   4. Pad or truncate to 8 tokens
///
/// Returns None if one or more residues could not be mapped.
pub fn encode_site(residues: &[&str]) -> Option<ActiveSiteEncoding> {
    let mut assignments = Vec::new();
    
    for r in residues {
        // Parse 3-letter code from residue string
        let code = if r.len() >= 3 {
            r[..3].to_uppercase()
        } else {
            return None;
        };
        
        let token = aa_to_token(&code)?;
        let chem_class = aa_chemical_class(&code);
        
        assignments.push(ResidueAssignment {
            residue_label: r.to_string(),
            aa_code: code,
            token,
            chemical_class: chem_class,
        });
    }
    
    // Build 8-token arrangement (pad with IMSCRIB, truncate to 8)
    let mut tokens: Vec<Token> = assignments.iter().map(|a| a.token).collect();
    while tokens.len() < 8 {
        tokens.push(Token::IMSCRIB);
    }
    tokens.truncate(8);
    
    Some(ActiveSiteEncoding { tokens, assignments })
}

/// Token → chemical meaning (diagnostic label).
pub fn token_meaning(tok: Token) -> &'static str {
    match tok {
        Token::EVALF  => "acidic (negative)",
        Token::EVALT  => "basic (positive)",
        Token::TANCH  => "guanidinium (planar/rigid)",
        Token::FSPLIT => "nucleophile (attacks)",
        Token::ENGAGR => "aromatic (multi-mode)",
        Token::AFWD   => "aromatic-hydrophobic (π-stack)",
        Token::AREV   => "polar/H-bond (bidirectional)",
        Token::VINIT  => "flexible (minimal steric)",
        Token::CLINK  => "small hydrophobic (methyl)",
        Token::IFIX   => "branched hydrophobic (rigid)",
        Token::IMSCRIB=> "cyclic (constrained)",
        Token::FFUSE  => "fuse (reserved)",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aa_to_token() {
        assert_eq!(aa_to_token("SER"), Some(Token::FSPLIT));
        assert_eq!(aa_to_token("ASP"), Some(Token::EVALF));
        assert_eq!(aa_to_token("LYS"), Some(Token::EVALT));
        assert_eq!(aa_to_token("PRO"), Some(Token::IMSCRIB));
        assert_eq!(aa_to_token("XXX"), None);
    }
    
    #[test]
    fn test_encode_site_chymotrypsin() {
        let residues = &["Ser195", "His57", "Asp102"];
        let result = encode_site(residues).unwrap();
        assert_eq!(result.assignments.len(), 3);
        assert_eq!(result.tokens.len(), 8);
        assert_eq!(result.tokens[0], Token::FSPLIT); // Ser
        assert_eq!(result.tokens[1], Token::EVALT);   // His
        assert_eq!(result.tokens[2], Token::EVALF);   // Asp
        // Remaining 5 tokens should be IMSCRIB (padding)
        for i in 3..8 {
            assert_eq!(result.tokens[i], Token::IMSCRIB);
        }
    }
    
    #[test]
    fn test_encode_site_unknown_residue() {
        let residues = &["XXX123"];
        assert!(encode_site(residues).is_none());
    }
}
