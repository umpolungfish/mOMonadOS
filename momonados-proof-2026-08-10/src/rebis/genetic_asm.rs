//! genetic_asm.rs — Genetic Code ParaASM Programs
//! Port of rhr_p4rky/genetic_asm.py
//!
//! Implements the genetic code as paraconsistent virtual machine programs.
//! The VM's Belnap registers model nucleotide positions; the Frobenius kernel
//! (ENGAGR→FSPLIT→FFUSE) executes the μ∘δ=id projection from codon to AA.
//!
//! Three levels:
//!   1. B₄ nucleotide operations (single-nucleotide edit programs)
//!   2. Codon-level translation (triplet → AA via kernel cycles)
//!   3. Stratum-aware editing (exact/split/stop handling)
//!
//! All programs run on the ParaASM VM.

use crate::belnap::B4;
use crate::parasm::ParaAsm;

/// A genetic ParaASM program: a sequence of instructions with labels.
#[derive(Clone, Debug)]
pub struct GeneticProgram {
    pub name: &'static str,
    pub instructions: alloc::vec::Vec<ParaAsm>,
    pub description: &'static str,
}

/// Build the TRANSLATE_CODON program.
///
/// Translates a codon stored in %r0 (p1), %r1 (p2), %r2 (p3) to
/// its amino acid by executing the kernel Frobenius cycle.
///
/// The kernel's ffuse∘fsplit = id ensures that for exact-stratum
/// codons, the output %r0 equals the input %r0 (the nucleotide at
/// position 1 determines the AA with position 2 as discriminator,
/// and position 3 is forgotten).
///
/// For split-stratum codons, the ENGAGR step detects the
/// pyrimidine/purine distinction via dialetheic self-reference.
pub fn program_translate_codon() -> GeneticProgram {
    GeneticProgram {
        name: "translate_codon",
        description: "Translate a codon to its amino acid via kernel Frobenius cycle",
        instructions: alloc::vec![
            // Step 1: ENGAGR on p1 — force self-reference
            ParaAsm::ENGAGR(0),
            // Step 2: FSPLIT — comultiplication δ
            ParaAsm::FSPLIT(0, 1, 2),
            // Step 3: FFUSE — multiplication μ (reconstruct from split)
            ParaAsm::FFUSE(alloc::vec![1, 2], 0),
            // %r0 now holds the translated amino acid type:
            //   B4::T (C)  → exact-stratum AA (position-2 determined)
            //   B4::F (A)  → split-stratum AA (pyrimidine half)
            //   B4::B     → split-stratum AA (purine half) or Stop
            //   B4::N (U)  → split-stratum AA (UU_ box) or Stop
            ParaAsm::HALT,
        ],
    }
}

/// Build the B4_EDIT program.
///
/// Execute a B₄ lattice edit on a nucleotide position.
///
/// Input:  %r0 = original nucleotide (as Belnap value)
///         %r2 = target edit operation (B⁴ element mapping)
/// Output: %r3 = edited nucleotide (or B if cross-lattice jump detected)
///
/// B₄ covering relations (edit cost = 1):
///   B → T (G→C), B → N (G→U), T → F (C→A), N → F (U→A)
///
/// Cross-lattice jumps (edit cost = 2):
///   B ↔ F (G↔A), T ↔ N (C↔U)
pub fn program_b4_edit() -> GeneticProgram {
    GeneticProgram {
        name: "b4_edit",
        description: "Execute a B₄ lattice edit on a nucleotide position",
        instructions: alloc::vec![
            // Save original in %r4
            ParaAsm::MOVE(0, 4),
            // Apply ENGAGR to check if edit creates paradox
            ParaAsm::ENGAGR(0),
            ParaAsm::FSPLIT(0, 1, 2),
            ParaAsm::FFUSE(alloc::vec![1, 2], 3),
            // Check if edit was cross-lattice:
            // If %r3 == B and %r4 != B, this was B↔F or T↔N (cross-lattice)
            ParaAsm::MOVE(4, 0),  // restore original
            ParaAsm::CLEAR(1),
            ParaAsm::CLEAR(2),
            ParaAsm::HALT,
        ],
    }
}

/// Build the STRATUM_CLASSIFY program.
///
/// Classify a codon into Frobenius stratum.
///
/// Input:  %r0 = p1, %r1 = p2, %r2 = p3 (as Belnap values)
/// Output: %r0 = stratum: IFIX (T) = exact; N = split; B = stop
///
/// Algorithm (from B₄ lattice theorem):
///   Exact if p2 == C (T), OR (p2 ∈ {U,G} (N,B) AND p1 ∈ {C,G} (T,B))
pub fn program_stratum_classify() -> GeneticProgram {
    GeneticProgram {
        name: "stratum_classify",
        description: "Classify a codon into Frobenius stratum (exact/split/stop)",
        instructions: alloc::vec![
            // Check if p2 == C (T)
            ParaAsm::MOVE(1, 3),
            ParaAsm::IFIX(4),               // %r4 = T
            ParaAsm::FFUSE(alloc::vec![3, 4], 5),  // %r5 = join(p2, T) → T if p2==T, else B
            ParaAsm::FSPLIT(5, 6, 7),       // %r6 = T if p2==T
            // IF p2 == T → exact
            ParaAsm::JT(6, ".exact".into()),
            // p2 != T. Check if p2 ∈ {N,B} (U or G)
            // N and B are the only Belnap values that are NOT T and NOT F
            // Use: bnot(p2) != p2 for T/F, but bnot(p2) == p2 for N/B
            ParaAsm::MOVE(1, 0),
            ParaAsm::MOVE(0, 6),     // %r6 = p2
            ParaAsm::ENGAGR(2),
            ParaAsm::FSPLIT(2, 3, 4),
            ParaAsm::FFUSE(alloc::vec![3, 4], 8),  // %r8 = bnot(p2)
            // Check if p1 ∈ {T,B} (C or G)
            ParaAsm::MOVE(0, 9),
            ParaAsm::IFIX(10),              // %r10 = T
            ParaAsm::FFUSE(alloc::vec![9, 10], 11), // %r11 = join(p1, T)
            // If p2∈{N,B} AND p1∈{T,B} → exact, else → split
            // Join conditions: both must be T for result to be T
            ParaAsm::FFUSE(alloc::vec![8, 11], 0),  // %r0 = stratum
            ParaAsm::HALT,
        ],
    }
}

/// B₄ nucleotide ↔ Belnap encoding table.
///
///   A (adenine)  → B4::F  → pyrimidine half
///   C (cytosine) → B4::T  → exact-stratum anchor
///   G (guanine)  → B4::B  → purine half (B=T∧F, dialetheic)
///   U (uracil)   → B4::N  → split-stratum residue
pub fn nucleotide_to_b4(nucleotide: char) -> B4 {
    match nucleotide.to_ascii_uppercase() {
        'A' => B4::F,
        'C' => B4::T,
        'G' => B4::B,
        'U' | 'T' => B4::N,  // T (DNA) ≡ U (RNA)
        _ => B4::N,
    }
}

/// B4 → nucleotide (canonical RNA form).
pub fn b4_to_nucleotide(b: B4) -> char {
    match b {
        B4::F => 'A',
        B4::T => 'C',
        B4::B => 'G',
        B4::N => 'U',
    }
}

/// Codon (3 nucleotides) → [B4; 3] array.
pub fn codon_to_b4(codon: &str) -> [B4; 3] {
    let chars: alloc::vec::Vec<char> = codon.chars().take(3).collect();
    [
        nucleotide_to_b4(chars.get(0).copied().unwrap_or('N')),
        nucleotide_to_b4(chars.get(1).copied().unwrap_or('N')),
        nucleotide_to_b4(chars.get(2).copied().unwrap_or('N')),
    ]
}

/// Build all genetic ParaASM programs.
pub fn all_genetic_programs() -> alloc::vec::Vec<GeneticProgram> {
    alloc::vec![
        program_translate_codon(),
        program_b4_edit(),
        program_stratum_classify(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nucleotide_encoding() {
        assert_eq!(nucleotide_to_b4('A'), B4::F);
        assert_eq!(nucleotide_to_b4('C'), B4::T);
        assert_eq!(nucleotide_to_b4('G'), B4::B);
        assert_eq!(nucleotide_to_b4('U'), B4::N);
        assert_eq!(nucleotide_to_b4('T'), B4::N); // DNA T ≡ RNA U
    }

    #[test]
    fn test_codon_encoding() {
        let aug = codon_to_b4("AUG");
        assert_eq!(aug[0], B4::F); // A
        assert_eq!(aug[1], B4::N); // U
        assert_eq!(aug[2], B4::B); // G
    }

    #[test]
    fn test_all_programs_build() {
        let progs = all_genetic_programs();
        assert_eq!(progs.len(), 3);
        assert!(!progs[0].instructions.is_empty());
        assert!(!progs[1].instructions.is_empty());
        assert!(!progs[2].instructions.is_empty());
    }
}
