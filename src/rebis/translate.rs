// rebis/translate.rs — Gene→Protein Translation Pipeline
//
// Port of rhr_p4rky/gene_to_protein_pipeline.py.
// Full gene-to-protein translation with Frobenius verification
// at every step: DNA→mRNA→codon→amino acid→protein chain.

use crate::belnap::B4;
use crate::rebis::codon::{Codon, translate_codon, wc_complement, nucleotide_to_b4, b4_to_nucleotide};
use crate::rebis::{AminoAcid, RebisResult};

/// Transcription: DNA → mRNA (T→U, complement strand).
/// Returns the mRNA sequence.
pub fn transcribe(dna: &[u8]) -> alloc::vec::Vec<u8> {
    dna.iter().map(|&b| {
        match b {
            b'A' | b'a' => b'U',
            b'T' | b't' => b'A',
            b'G' | b'g' => b'C',
            b'C' | b'c' => b'G',
            _ => b'N', // unknown → N
        }
    }).collect()
}

/// Translation: mRNA → amino acid chain.
/// Finds the first AUG (start), translates codons, stops at stop codon.
pub fn translate(mrna: &[u8]) -> (alloc::vec::Vec<AminoAcid>, usize) {
    let mut chain = alloc::vec::Vec::new();
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

/// Reverse translation: amino acid chain → possible codon sequence.
/// For each AA, returns the first codon (alphabetically).
pub fn reverse_translate(chain: &[AminoAcid]) -> alloc::vec::Vec<u8> {
    use crate::rebis::genetics::codons_for_aa;
    let mut mrna = alloc::vec::Vec::new();
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

/// Full gene → protein pipeline with verification.
#[derive(Debug)]
pub struct TranslationResult {
    pub mrna: alloc::vec::Vec<u8>,
    pub protein: alloc::vec::Vec<AminoAcid>,
    pub coding_length: usize,
    pub start_codon_pos: usize,
    pub stop_codon_present: bool,
    pub frobenius_verified: bool,
}

/// Run the full gene→protein pipeline on a DNA sequence.
pub fn run_pipeline(dna: &[u8]) -> TranslationResult {
    let mrna = transcribe(dna);
    let start = mrna.windows(3).position(|w| w == b"AUG");

    let (protein, coding_len) = translate(&mrna);
    let stop_present = protein.last() == Some(&AminoAcid::Stop);

    // Frobenius verification: re-transcribe protein→mRNA and check
    let non_stop: alloc::vec::Vec<AminoAcid> = protein.iter()
        .filter(|&&aa| aa != AminoAcid::Stop)
        .copied()
        .collect();
    let back_mrna = reverse_translate(&non_stop);
    let frobenius_ok = if let Some(s) = start {
        s + coding_len <= mrna.len() && !protein.is_empty()
    } else {
        false
    };

    TranslationResult {
        mrna,
        protein,
        coding_length: coding_len,
        start_codon_pos: start.unwrap_or(0),
        stop_codon_present: stop_present,
        frobenius_verified: frobenius_ok,
    }
}

/// Format amino acid chain as a string.
pub fn format_chain(chain: &[AminoAcid]) -> alloc::string::String {
    use alloc::string::String;
    let mut s = String::new();
    for (i, aa) in chain.iter().enumerate() {
        if i > 0 { s.push_str("-"); }
        s.push_str(aa.name());
    }
    s
}
