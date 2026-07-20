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
    ("ASP", Token::Evalf),
    ("GLU", Token::Evalf),
    // Basic (positive charge) → EVALT
    ("HIS", Token::Evalt),
    ("LYS", Token::Evalt),
    // Guanidinium (planar rigid H-bond donor) → TANCH
    ("ARG", Token::Tanch),
    // Nucleophilic (attacks electrophilic centers) → FSPLIT
    ("SER", Token::Fsplit),
    ("CYS", Token::Fsplit),
    ("THR", Token::Fsplit),
    // Aromatic (multi-mode: H-bond + π-stack) → ENGAGR
    ("TYR", Token::Engagr),
    ("TRP", Token::Engagr),
    // Aromatic hydrophobic (directional π-stacking) → AFWD
    ("PHE", Token::Afwd),
    // Polar amide (bidirectional H-bond) → AREV
    ("ASN", Token::Arev),
    ("GLN", Token::Arev),
    // Flexible (minimal steric constraint) → VINIT
    ("GLY", Token::Vinit),
    // Small hydrophobic (methyl) → CLINK
    ("ALA", Token::Clink),
    // Branched hydrophobic (rigid, sterically constrained) → IFIX
    ("VAL", Token::Ifix),
    ("LEU", Token::Ifix),
    ("ILE", Token::Ifix),
    // Thioether (polarizable sulfur) → AREV
    ("MET", Token::Arev),
    // Cyclic (conformationally constrained) → IMSCRIB
    ("PRO", Token::Imscrib),
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
        tokens.push(Token::Imscrib);
    }
    tokens.truncate(8);
    
    Some(ActiveSiteEncoding { tokens, assignments })
}

/// Token → chemical meaning (diagnostic label).
pub fn token_meaning(tok: Token) -> &'static str {
    match tok {
        Token::Evalf  => "acidic (negative)",
        Token::Evalt  => "basic (positive)",
        Token::Tanch  => "guanidinium (planar/rigid)",
        Token::Fsplit => "nucleophile (attacks)",
        Token::Engagr => "aromatic (multi-mode)",
        Token::Afwd   => "aromatic-hydrophobic (π-stack)",
        Token::Arev   => "polar/H-bond (bidirectional)",
        Token::Vinit  => "flexible (minimal steric)",
        Token::Clink  => "small hydrophobic (methyl)",
        Token::Ifix   => "branched hydrophobic (rigid)",
        Token::Imscrib=> "cyclic (constrained)",
        Token::Ffuse  => "fuse (reserved)",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aa_to_token() {
        assert_eq!(aa_to_token("SER"), Some(Token::Fsplit));
        assert_eq!(aa_to_token("ASP"), Some(Token::Evalf));
        assert_eq!(aa_to_token("LYS"), Some(Token::Evalt));
        assert_eq!(aa_to_token("PRO"), Some(Token::Imscrib));
        assert_eq!(aa_to_token("XXX"), None);
    }
    
    #[test]
    fn test_encode_site_chymotrypsin() {
        let residues = &["Ser195", "His57", "Asp102"];
        let result = encode_site(residues).unwrap();
        assert_eq!(result.assignments.len(), 3);
        assert_eq!(result.tokens.len(), 8);
        assert_eq!(result.tokens[0], Token::Fsplit); // Ser
        assert_eq!(result.tokens[1], Token::Evalt);   // His
        assert_eq!(result.tokens[2], Token::Evalf);   // Asp
        // Remaining 5 tokens should be IMSCRIB (padding)
        for i in 3..8 {
            assert_eq!(result.tokens[i], Token::Imscrib);
        }
    }
    
    #[test]
    fn test_encode_site_unknown_residue() {
        let residues = &["XXX123"];
        assert!(encode_site(residues).is_none());
    }
}
