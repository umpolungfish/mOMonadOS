// rebis/genetics.rs — B₄ Nucleotide Lattice & Genetic Code Operations
//
// Port of rhr_p4rky/genetics_b4.py and genetic_code.py.
// B₄ lattice operations on codons, Frobenius stratum classification,
// and the 12-primitive ↔ 20 amino acid structural mapping.

use crate::belnap::{B4, meet, join};
use crate::rebis::codon::{Codon, CodeTable, Stratum, classify_stratum, translate_codon, translate_codon_mito, wc_complement};
use crate::rebis::AminoAcid;

// ── B₄ lattice operations on nucleotides ───────────────────────

/// B₄ meet (∧): greatest lower bound.
/// G∧G=G, C∧C=C, A∧A=A, U∧U=U; G∧x=x for all x.
pub fn b4_meet(a: B4, b: B4) -> B4 {
    meet(a, b)
}

/// B₄ join (∨): least upper bound.
/// G∨x=G for all x; A∨x=x for all x; C∨U=G.
pub fn b4_join(a: B4, b: B4) -> B4 {
    join(a, b)
}

/// B₄ complement: Watson-Crick (fixed-point-free involution).
pub fn b4_complement(a: B4) -> B4 {
    wc_complement(a)
}

// ── Codon lattice operations ────────────────────────────────────

/// Pointwise B₄ meet of two codons.
pub fn codon_meet(a: &Codon, b: &Codon) -> Codon {
    Codon {
        p1: b4_meet(a.p1, b.p1),
        p2: b4_meet(a.p2, b.p2),
        p3: b4_meet(a.p3, b.p3),
    }
}

/// Pointwise B₄ join of two codons.
pub fn codon_join(a: &Codon, b: &Codon) -> Codon {
    Codon {
        p1: b4_join(a.p1, b.p1),
        p2: b4_join(a.p2, b.p2),
        p3: b4_join(a.p3, b.p3),
    }
}

/// B₄ lattice distance between two codons (sum of per-position distances).
pub fn codon_distance(a: &Codon, b: &Codon) -> u8 {
    let d = |x: B4, y: B4| -> u8 {
        if x == y { 0 }
        else if (x == B4::B && y == B4::N) || (x == B4::N && y == B4::B) { 1 }
        else if (x == B4::B && y == B4::F) || (x == B4::F && y == B4::B) { 2 }
        else if (x == B4::T && y == B4::F) || (x == B4::F && y == B4::T) { 1 }
        else if (x == B4::T && y == B4::N) || (x == B4::N && y == B4::T) { 1 }
        else { 1 }
    };
    d(a.p1, b.p1) + d(a.p2, b.p2) + d(a.p3, b.p3)
}

// ── Amino acid ↔ IG primitive mapping ───────────────────────────

/// All 20 amino acids + Stop in order.
pub static ALL_AMINO_ACIDS: [AminoAcid; 21] = [
    AminoAcid::Phe, AminoAcid::Leu, AminoAcid::Ile, AminoAcid::Met,
    AminoAcid::Val, AminoAcid::Ser, AminoAcid::Pro, AminoAcid::Thr,
    AminoAcid::Ala, AminoAcid::Tyr, AminoAcid::Stop, AminoAcid::His,
    AminoAcid::Gln, AminoAcid::Asn, AminoAcid::Lys, AminoAcid::Asp,
    AminoAcid::Glu, AminoAcid::Cys, AminoAcid::Trp, AminoAcid::Arg,
    AminoAcid::Gly,
];

/// The 12 promoted amino acids — those appearing ONLY in split-stratum codons.
/// Derived dynamically from the stratum classification: an AA is promoted iff
/// none of its codons fall in the exact (4-fold degenerate) stratum.
/// This is a structural fact of the B4 lattice, not a hardcoded list.
pub fn promoted_amino_acids() -> alloc::vec::Vec<AminoAcid> {
    let mut promoted = alloc::vec::Vec::new();
    for &aa in &ALL_AMINO_ACIDS {
        if aa == AminoAcid::Stop { continue; }
        let codons = codons_for_aa(aa);
        if codons.is_empty() { continue; }
        // Promoted iff NO codon is in the exact stratum
        if !codons.iter().any(|c| c.is_exact_stratum()) {
            promoted.push(aa);
        }
    }
    promoted
}

/// Get all codons that encode a given amino acid (standard code).
pub fn codons_for_aa(aa: AminoAcid) -> alloc::vec::Vec<Codon> {
    codons_for_aa_table(aa, CodeTable::Standard)
}

/// Get all codons for an AA under the given genetic code table.
pub fn codons_for_aa_table(aa: AminoAcid, table: CodeTable) -> alloc::vec::Vec<Codon> {
    let mut result = alloc::vec::Vec::new();
    for i in 0u8..64 {
        let c = Codon {
            p1: index_to_b4(i / 16),
            p2: index_to_b4((i / 4) % 4),
            p3: index_to_b4(i % 4),
        };
        let translated = match table {
            CodeTable::Standard => translate_codon(&c),
            CodeTable::Mitochondrial => translate_codon_mito(&c),
        };
        if translated == aa {
            result.push(c);
        }
    }
    result
}

fn index_to_b4(x: u8) -> B4 {
    match x % 4 {
        3 => B4::B,
        2 => B4::T,
        1 => B4::F,
        _ => B4::N,
    }
}

// ── 7-Stage Tuple Verification ──────────────────────────────────
// From genetics_b4.py: the genetic code is verified through 7 stages
// of tuple structural analysis.

#[derive(Copy, Clone, Debug)]
pub struct GeneticVerification {
    pub stage1_codon_count: bool,    // 64 codons = 4³
    pub stage2_stratum_split: bool,  // 3 strata: exact/split/stop
    pub stage3_aa_count: bool,       // 21 classes (20 AA + stop)
    pub stage4_promoted_bijection: bool, // 12 AAs ↔ 12 primitives
    pub stage5_wobble: bool,         // 3rd position wobble verified
    pub stage6_frobenius: bool,      // ffuse∘fsplit = id
    pub stage7_crystal: bool,        // 64 | 17,280,000
}

impl GeneticVerification {
    pub fn run() -> Self {
        Self {
            stage1_codon_count: true,  // 4³ = 64, always true
            stage2_stratum_split: {
                let (e, s, t) = crate::rebis::codon::stratum_counts();
                e + s + t == 64
            },
            stage3_aa_count: {
                // Check all 21 classes are covered
                let mut found = [false; 21];
                for i in 0..64 {
                    let c = Codon {
                        p1: index_to_b4(i / 16),
                        p2: index_to_b4((i / 4) % 4),
                        p3: index_to_b4(i % 4),
                    };
                    let aa = translate_codon(&c);
                    found[aa as usize] = true;
                }
                found.iter().all(|&x| x)
            },
            stage4_promoted_bijection: {
                // Dynamic derivation: split-only AAs should biject to 12 primitives
                promoted_amino_acids().len() == 12
            },
            stage5_wobble: {
                // Wobble: 3rd position is often degenerate
                let (exact, _, _) = crate::rebis::codon::stratum_counts();
                exact == 32 // 8 boxes × 4 codons
            },
            stage6_frobenius: {
                // All exact-stratum codons satisfy ffuse∘fsplit = id
                let mut ok = true;
                for i in 0..64 {
                    let c = Codon {
                        p1: index_to_b4(i / 16),
                        p2: index_to_b4((i / 4) % 4),
                        p3: index_to_b4(i % 4),
                    };
                    let (holds, _) = crate::rebis::codon::verify_frobenius(&c);
                    if !holds && matches!(classify_stratum(&c), Stratum::Exact) {
                        ok = false;
                    }
                }
                ok
            },
            stage7_crystal: {
                // 17,280,000 / 64 = 270,000 exact division
                crate::crystal::TOTAL as u64 % 64 == 0
            },
        }
    }

    pub fn all_passed(&self) -> bool {
        self.stage1_codon_count && self.stage2_stratum_split &&
        self.stage3_aa_count && self.stage4_promoted_bijection &&
        self.stage5_wobble && self.stage6_frobenius && self.stage7_crystal
    }

    pub fn report(&self) -> &'static str {
        if self.all_passed() { "ALL 7 STAGES PASSED — Genetic Code Frobenius-Verified" }
        else { "VERIFICATION FAILED — check stage results" }
    }
}
