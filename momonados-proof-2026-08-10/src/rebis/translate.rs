// rebis/translate.rs — Gene→Protein Translation Pipeline
//
// Port of rhr_p4rky/gene_to_protein_pipeline.py.
// Full bidirectional gene↔protein translation with Frobenius verification
// at every step: DNA↔mRNA→codon↔amino acid↔protein chain.
//
// Added reverse direction: Protein → mRNA (all degenerate codons enumerated).

use crate::rebis::codon::{Codon, CodeTable, translate_codon, translate_codon_table, b4_to_nucleotide};
use crate::rebis::AminoAcid;
use alloc::string::String;
use alloc::vec::Vec;

/// Transcription: DNA coding strand → mRNA (T→U only, NO complement).
/// DNA: ATGGCC → mRNA: AUGGCC. The coding strand IS the mRNA with T→U.
pub fn transcribe(dna: &[u8]) -> Vec<u8> {
    dna.iter().map(|&b| {
        match b {
            b'T' | b't' => b'U',
            other => other.to_ascii_uppercase(),
        }
    }).collect()
}

/// Reverse transcription: mRNA → DNA (U→T, no complement).
/// mRNA: AUGGCC → DNA: ATGGCC. Just U→T.
pub fn reverse_transcribe(mrna: &[u8]) -> Vec<u8> {
    mrna.iter().map(|&b| {
        match b {
            b'U' | b'u' => b'T',
            other => other.to_ascii_uppercase(),
        }
    }).collect()
}

/// Translation: mRNA → amino acid chain.
/// Finds the first AUG (start), translates codons, stops at stop codon.
pub fn translate(mrna: &[u8]) -> (Vec<AminoAcid>, usize) {
    let mut chain = Vec::new();
    let mut start_idx = None;

    // Find start codon (AUG)
    for i in 0..mrna.len().saturating_sub(2) {
        if mrna[i] == b'A' && mrna[i+1] == b'U' && mrna[i+2] == b'G' {
            start_idx = Some(i);
            break;
        }
    }

    let start = match start_idx {
        Some(i) => i,
        None => return (chain, 0),
    };

    // Translate codons from start
    let mut pos = start;
    while pos + 2 < mrna.len() {
        let codon = match Codon::from_bytes(mrna[pos], mrna[pos+1], mrna[pos+2]) {
            Ok(c) => c,
            Err(_) => break,
        };
        let aa = translate_codon(&codon);
        if aa == AminoAcid::Stop {
            chain.push(aa); // record stop
            break;
        }
        chain.push(aa);
        pos += 3;
    }

    (chain, pos + 3 - start)
}

// ── Amino Acid Parsing ──────────────────────────────────────────

/// Parse an amino acid from a 3-letter code, 1-letter code, or full name.
/// Case-insensitive. Returns None if unrecognized.
pub fn parse_aa(s: &str) -> Option<AminoAcid> {
    match s.to_uppercase().as_str() {
        // Three-letter codes
        "PHE" | "F" => Some(AminoAcid::Phe),
        "LEU" | "L" => Some(AminoAcid::Leu),
        "ILE" | "I" => Some(AminoAcid::Ile),
        "MET" | "M" => Some(AminoAcid::Met),
        "VAL" | "V" => Some(AminoAcid::Val),
        "SER" | "S" => Some(AminoAcid::Ser),
        "PRO" | "P" => Some(AminoAcid::Pro),
        "THR" | "T" => Some(AminoAcid::Thr),
        "ALA" | "A" => Some(AminoAcid::Ala),
        "TYR" | "Y" => Some(AminoAcid::Tyr),
        "STOP" | "STP" | "*" => Some(AminoAcid::Stop),
        "HIS" | "H" => Some(AminoAcid::His),
        "GLN" | "Q" => Some(AminoAcid::Gln),
        "ASN" | "N" => Some(AminoAcid::Asn),
        "LYS" | "K" => Some(AminoAcid::Lys),
        "ASP" | "D" => Some(AminoAcid::Asp),
        "GLU" | "E" => Some(AminoAcid::Glu),
        "CYS" | "C" => Some(AminoAcid::Cys),
        "TRP" | "W" => Some(AminoAcid::Trp),
        "ARG" | "R" => Some(AminoAcid::Arg),
        "GLY" | "G" => Some(AminoAcid::Gly),
        _ => None,
    }
}

/// Get the 1-letter code for an amino acid.
pub fn aa_letter(aa: AminoAcid) -> &'static str {
    match aa {
        AminoAcid::Phe => "F", AminoAcid::Leu => "L", AminoAcid::Ile => "I",
        AminoAcid::Met => "M", AminoAcid::Val => "V", AminoAcid::Ser => "S",
        AminoAcid::Pro => "P", AminoAcid::Thr => "T", AminoAcid::Ala => "A",
        AminoAcid::Tyr => "Y", AminoAcid::Stop => "*", AminoAcid::His => "H",
        AminoAcid::Gln => "Q", AminoAcid::Asn => "N", AminoAcid::Lys => "K",
        AminoAcid::Asp => "D", AminoAcid::Glu => "E", AminoAcid::Cys => "C",
        AminoAcid::Trp => "W", AminoAcid::Arg => "R", AminoAcid::Gly => "G",
    }
}

// ── Reverse Translation (Protein → RNA) ─────────────────────────

/// The result of reverse-translating a single amino acid:
/// one or more codons that could encode it.
#[derive(Debug)]
pub struct ReverseHit {
    pub aa: AminoAcid,
    pub codons: Vec<Codon>,
    pub codon_count: usize,     // degeneracy
}

/// Reverse translate an amino acid: return ALL codons that encode it.
pub fn reverse_translate_aa(aa: AminoAcid) -> ReverseHit {
    use crate::rebis::genetics::codons_for_aa;
    let codons = codons_for_aa(aa);
    ReverseHit {
        aa,
        codon_count: codons.len(),
        codons,
    }
}

/// Format a codon as a 3-letter RNA string.
pub fn codon_to_rna(c: &Codon) -> [u8; 3] {
    [b4_to_nucleotide(c.p1), b4_to_nucleotide(c.p2), b4_to_nucleotide(c.p3)]
}

/// Reverse translate a full protein chain to ALL possible mRNA sequences.
/// Returns a vector where each element is the set of possible codons for that position.
pub fn reverse_translate_chain(chain: &[AminoAcid]) -> Vec<ReverseHit> {
    chain.iter().map(|&aa| reverse_translate_aa(aa)).collect()
}

/// Enumerate ALL possible mRNA sequences for a protein chain.
/// WARNING: Product of degeneracies. A chain of 4 AAs averaging 4 codons
/// each produces 4^4=256 sequences. Use with care.
/// Returns the total count and an iterator-like Vec of sequences.
pub fn enumerate_mrna(chain: &[AminoAcid]) -> Vec<Vec<u8>> {
    if chain.is_empty() {
        return Vec::new();
    }

    use crate::rebis::genetics::codons_for_aa;

    // Start with the first AA's codons
    let first = codons_for_aa(chain[0]);
    if first.is_empty() {
        return Vec::new();
    }

    let mut sequences: Vec<Vec<u8>> = first.iter().map(|c| {
        Vec::from(codon_to_rna(c).as_slice())
    }).collect();

    // For each subsequent AA, extend all sequences with its codons
    for &aa in &chain[1..] {
        let codons = codons_for_aa(aa);
        if codons.is_empty() {
            break;
        }
        let mut new_seqs = Vec::with_capacity(sequences.len() * codons.len());
        for seq in &sequences {
            for c in &codons {
                let mut ext = seq.clone();
                ext.extend_from_slice(&codon_to_rna(c));
                new_seqs.push(ext);
            }
        }
        sequences = new_seqs;
    }

    sequences
}

/// Reverse translation: amino acid chain → canonical codon sequence.
/// For each AA, returns the FIRST codon (sorted by index, which groups U→A→C→G).
/// This is the "canonical" representative.
pub fn reverse_translate(chain: &[AminoAcid]) -> Vec<u8> {
    use crate::rebis::genetics::codons_for_aa;
    let mut mrna = Vec::new();
    for &aa in chain {
        let codons = codons_for_aa(aa);
        if codons.is_empty() { break; }
        let c = &codons[0];
        mrna.push(b4_to_nucleotide(c.p1));
        mrna.push(b4_to_nucleotide(c.p2));
        mrna.push(b4_to_nucleotide(c.p3));
    }
    mrna
}

// ── Pipeline ────────────────────────────────────────────────────

/// Full gene → protein pipeline with verification.
#[derive(Debug)]
pub struct TranslationResult {
    pub mrna: Vec<u8>,
    pub protein: Vec<AminoAcid>,
    pub coding_length: usize,
    pub start_codon_pos: usize,
    pub stop_codon_present: bool,
    pub frobenius_verified: bool,
    pub primitive_labels: Vec<Option<&'static str>>,
    pub code_table: CodeTable,
}

/// Run the full gene→protein pipeline on a DNA sequence.
pub fn run_pipeline(dna: &[u8]) -> TranslationResult {
    run_pipeline_table(dna, CodeTable::Standard)
}

/// Run the pipeline with a specific genetic code table.
pub fn run_pipeline_table(dna: &[u8], code_table: CodeTable) -> TranslationResult {
    use crate::rebis::genetics::codons_for_aa_table;

    let mrna = transcribe(dna);
    let start = mrna.windows(3).position(|w| w == b"AUG");

    // Find start, translate codons
    let mut protein: Vec<AminoAcid> = Vec::new();
    let mut coding_len = 0usize;
    let mut pos = match start {
        Some(i) => i,
        None => {
            return TranslationResult {
                mrna, protein, coding_length: 0, start_codon_pos: 0,
                stop_codon_present: false, frobenius_verified: false,
                primitive_labels: Vec::new(), code_table,
            };
        }
    };
    let start_pos = pos;
    while pos + 2 < mrna.len() {
        let codon = match Codon::from_bytes(mrna[pos], mrna[pos+1], mrna[pos+2]) {
            Ok(c) => c, Err(_) => break,
        };
        let aa = translate_codon_table(&codon, code_table);
        protein.push(aa);
        if aa == AminoAcid::Stop { coding_len = pos + 3 - start_pos; break; }
        pos += 3;
    }
    if coding_len == 0 && !protein.is_empty() {
        coding_len = protein.len() * 3;
    }
    let stop_present = protein.last() == Some(&AminoAcid::Stop);

    // Frobenius round-trip: Protein→canonical mRNA→Protein using same table
    let non_stop: Vec<AminoAcid> = protein.iter().filter(|&&aa| aa != AminoAcid::Stop).copied().collect();
    let back_mrna: Vec<u8> = {
        let mut v = Vec::new();
        for &aa in &non_stop {
            let codons = codons_for_aa_table(aa, code_table);
            if codons.is_empty() { break; }
            let c = &codons[0];
            v.push(b4_to_nucleotide(c.p1));
            v.push(b4_to_nucleotide(c.p2));
            v.push(b4_to_nucleotide(c.p3));
        }
        v
    };
    let mut roundtrip: Vec<AminoAcid> = Vec::new();
    let mut rpos = 0usize;
    while rpos + 2 < back_mrna.len() {
        match Codon::from_bytes(back_mrna[rpos], back_mrna[rpos+1], back_mrna[rpos+2]) {
            Ok(c) => {
                let aa = translate_codon_table(&c, code_table);
                if aa == AminoAcid::Stop { break; }
                roundtrip.push(aa);
            }
            Err(_) => break,
        }
        rpos += 3;
    }
    let frobenius_ok = !non_stop.is_empty()
        && non_stop.len() == roundtrip.len()
        && non_stop.iter().zip(roundtrip.iter()).all(|(a, b)| a == b);

    // IG primitive labels per AA
    let primitive_labels: Vec<Option<&'static str>> = protein.iter()
        .map(|&aa| aa.primitive_name())
        .collect();

    TranslationResult {
        mrna,
        protein,
        coding_length: coding_len,
        start_codon_pos: start.unwrap_or(0),
        stop_codon_present: stop_present,
        frobenius_verified: frobenius_ok,
        primitive_labels,
        code_table,
    }
}

/// Run the reverse pipeline: protein chain → mRNA → DNA.
#[derive(Debug)]
pub struct ReverseTranslationResult {
    pub canonical_mrna: Vec<u8>,       // first codon per AA
    pub dna: Vec<u8>,                   // reverse-transcribed
    pub degeneracies: Vec<usize>,       // how many codons per AA position
    pub total_combinations: u64,        // product of degeneracies (capped)
    pub chain: Vec<AminoAcid>,
}

/// Run protein → mRNA → DNA pipeline.
pub fn run_reverse_pipeline(chain: &[AminoAcid]) -> ReverseTranslationResult {
    run_reverse_pipeline_table(chain, CodeTable::Standard)
}

pub fn run_reverse_pipeline_table(chain: &[AminoAcid], table: CodeTable) -> ReverseTranslationResult {
    use crate::rebis::genetics::codons_for_aa_table;

    let canonical_mrna: Vec<u8> = {
        let mut v = Vec::new();
        for &aa in chain {
            let codons = codons_for_aa_table(aa, table);
            if codons.is_empty() { break; }
            let c = &codons[0];
            v.push(b4_to_nucleotide(c.p1));
            v.push(b4_to_nucleotide(c.p2));
            v.push(b4_to_nucleotide(c.p3));
        }
        v
    };
    let dna = reverse_transcribe(&canonical_mrna);

    let degeneracies: Vec<usize> = chain.iter()
        .map(|&aa| codons_for_aa_table(aa, table).len())
        .collect();

    // Compute total combinations (cap at u64::MAX)
    let mut total: u64 = 1;
    for &d in &degeneracies {
        if d == 0 { total = 0; break; }
        total = total.saturating_mul(d as u64);
    }

    ReverseTranslationResult {
        canonical_mrna,
        dna,
        degeneracies,
        total_combinations: total,
        chain: chain.to_vec(),
    }
}

// keep old run_reverse_pipeline stub above

// ── Formatting ──────────────────────────────────────────────────

/// Format amino acid chain as a 3-letter dash-separated string.
pub fn format_chain(chain: &[AminoAcid]) -> String {
    let mut s = String::new();
    for (i, aa) in chain.iter().enumerate() {
        if i > 0 { s.push('-'); }
        s.push_str(aa.name());
    }
    s
}

/// Format amino acid chain as a 1-letter string (no separator).
pub fn format_chain_1letter(chain: &[AminoAcid]) -> String {
    let mut s = String::new();
    for aa in chain {
        s.push_str(aa_letter(*aa));
    }
    s
}

/// Parse a protein string: 3-letter codes (dash or space separated)
/// or 1-letter codes (compact string).
pub fn parse_chain(input: &str) -> Option<Vec<AminoAcid>> {
    let input = input.trim();
    if input.is_empty() {
        return Some(Vec::new());
    }

    // Try 3-letter codes (separated by dash, space, or comma)
    if input.len() >= 3 && (input.contains('-') || input.contains(' ') || input.contains(',')) {
        let parts: Vec<&str> = input.split(&['-', ' ', ','][..])
            .filter(|s| !s.is_empty())
            .collect();
        let mut chain = Vec::with_capacity(parts.len());
        for part in parts {
            chain.push(parse_aa(part)?);
        }
        return Some(chain);
    }

    // Try 1-letter codes (compact string, no separators)
    // Check if all chars are valid 1-letter AA codes
    let mut chain = Vec::with_capacity(input.len());
    for ch in input.chars() {
        let s: String = core::iter::once(ch).collect();
        chain.push(parse_aa(&s)?);
    }
    Some(chain)
}

/// Compute the round-trip fidelity: Protein → mRNA(canonical) → Protein.
/// Returns (original, round_tripped, match_count, total).
pub fn roundtrip_verify(chain: &[AminoAcid]) -> (Vec<AminoAcid>, Vec<AminoAcid>, usize, usize) {
    let mrna = reverse_translate(chain);
    let (round, _) = translate(&mrna);
    let total = chain.len().min(round.len());
    let mut matches = 0usize;
    for i in 0..total {
        if chain[i] == round[i] {
            matches += 1;
        }
    }
    (chain.to_vec(), round, matches, chain.len())
}
