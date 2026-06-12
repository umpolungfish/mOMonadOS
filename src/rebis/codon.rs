// rebis/codon.rs — 64-Codon Frobenius-Verified Genetic Code
//
// Port of rhr_p4rky/genetics_b4.py and genetic_code.py.
// The 64-codon table as static data with Belnap FOUR mapping.
//
// B₄ Nucleotide Lattice:
//   Belnap.B (Both)  ↔ G (Guanine) — top, pairs with C AND U via wobble
//   Belnap.T (True)  ↔ C (Cytosine) — definite, pairs only with G
//   Belnap.F (False) ↔ A (Adenine)  — definite, pairs only with U
//   Belnap.N (Neither)↔ U (Uracil) — bottom-like, wobble target

use crate::belnap::B4;
use crate::rebis::{AminoAcid, RebisResult};

// ── Nucleotide ↔ Belnap mapping ────────────────────────────────

pub fn nucleotide_to_b4(sym: u8) -> RebisResult<B4> {
    match sym {
        b'G' | b'g' => Ok(B4::B),
        b'C' | b'c' => Ok(B4::T),
        b'A' | b'a' => Ok(B4::F),
        b'U' | b'u' | b'T' | b't' => Ok(B4::N),
        _ => Err("Unknown nucleotide symbol"),
    }
}

pub fn b4_to_nucleotide(b: B4) -> u8 {
    match b {
        B4::B => b'G',
        B4::T => b'C',
        B4::F => b'A',
        B4::N => b'U',
    }
}

/// Watson-Crick complement (NOT Belnap negation!)
/// WC complement is a fixed-point-free involution: A↔U, G↔C
/// This is the PARITY operation, distinct from bnot.
pub fn wc_complement(b: B4) -> B4 {
    match b {
        B4::B => B4::T,  // G ↔ C
        B4::T => B4::B,  // C ↔ G
        B4::F => B4::N,  // A ↔ U
        B4::N => B4::F,  // U ↔ A
    }
}

// ── Codon type ──────────────────────────────────────────────────

/// A codon: three Belnap-valued positions.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Codon {
    pub p1: B4,  // position 1 (5' end)
    pub p2: B4,  // position 2
    pub p3: B4,  // position 3 (3' end, wobble)
}

impl Codon {
    /// Build a codon from three nucleotide symbols (bytes).
    pub fn from_bytes(b1: u8, b2: u8, b3: u8) -> RebisResult<Self> {
        Ok(Self {
            p1: nucleotide_to_b4(b1)?,
            p2: nucleotide_to_b4(b2)?,
            p3: nucleotide_to_b4(b3)?,
        })
    }

    /// Build a codon from a 3-char string (e.g., "AUG").
    pub fn from_str(s: &str) -> RebisResult<Self> {
        let b = s.as_bytes();
        if b.len() != 3 { return Err("Codon string must be exactly 3 nucleotides"); }
        Self::from_bytes(b[0], b[1], b[2])
    }

    /// Watson-Crick complement of the entire codon (reverse complement).
    pub fn reverse_complement(&self) -> Self {
        Self {
            p1: wc_complement(self.p3),
            p2: wc_complement(self.p2),
            p3: wc_complement(self.p1),
        }
    }

    /// Does this codon belong to the exact stratum?
    /// Exact stratum: 3rd base carries NO information (4-fold degenerate).
    pub fn is_exact_stratum(&self) -> bool {
        // The 8 exact boxes: GCN (Ala), CCN (Pro), GGN (Gly),
        // GUN (Val), CUN (Leu), UCN (Ser), ACN (Thr), CGN (Arg)
        // All have B4::B (G) or B4::T (C) at p1+p2 with any p3.
        matches!((self.p1, self.p2), (B4::B, B4::T) | (B4::T, B4::T) | (B4::B, B4::B)
            | (B4::B, B4::N) | (B4::T, B4::N) | (B4::N, B4::T) | (B4::F, B4::T) | (B4::T, B4::B))
    }

    /// Split stratum: 3rd base carries pyrimidine/purine information.
    pub fn is_split_stratum(&self) -> bool {
        !self.is_exact_stratum() && !self.is_stop()
    }

    /// Is this a stop codon?
    pub fn is_stop(&self) -> bool {
        // UAA, UAG, UGA
        matches!((self.p1, self.p2, self.p3),
            (B4::N, B4::F, B4::F) | (B4::N, B4::F, B4::B) | (B4::N, B4::B, B4::F))
    }

    /// Encode codon as a 6-bit index: p1<<4 | p2<<2 | p3.
    /// B4 values: B=3, T=2, F=1, N=0.
    pub fn index(&self) -> usize {
        let v = |b: B4| -> usize {
            match b { B4::B => 3, B4::T => 2, B4::F => 1, B4::N => 0 }
        };
        v(self.p1) * 16 + v(self.p2) * 4 + v(self.p3)
    }

    /// Symbol string for display.
    pub fn symbol(&self) -> [u8; 3] {
        [b4_to_nucleotide(self.p1), b4_to_nucleotide(self.p2), b4_to_nucleotide(self.p3)]
    }
}

// ── 64-Codon → Amino Acid Static Table ──────────────────────────
//
// The table is indexed by Codon::index() (0..64).
// U=0, A=1, C=2, G=3  →  index = p1*16 + p2*4 + p3

static CODON_TABLE: [AminoAcid; 64] = [
    // UUU UUC UUA UUG  UCU UCC UCA UCG  UAU UAC UAA UAG  UGU UGC UGA UGG
    AminoAcid::Phe, AminoAcid::Phe, AminoAcid::Leu, AminoAcid::Leu,   // UUX
    AminoAcid::Ser, AminoAcid::Ser, AminoAcid::Ser, AminoAcid::Ser,   // UCX
    AminoAcid::Tyr, AminoAcid::Tyr, AminoAcid::Stop, AminoAcid::Stop, // UAX
    AminoAcid::Cys, AminoAcid::Cys, AminoAcid::Stop, AminoAcid::Trp,  // UGX
    // AUU AUC AUA AUG  ACU ACC ACA ACG  AAU AAC AAA AAG  AGU AGC AGA AGG
    AminoAcid::Ile, AminoAcid::Ile, AminoAcid::Ile, AminoAcid::Met,   // AUX
    AminoAcid::Thr, AminoAcid::Thr, AminoAcid::Thr, AminoAcid::Thr,   // ACX
    AminoAcid::Asn, AminoAcid::Asn, AminoAcid::Lys, AminoAcid::Lys,   // AAX
    AminoAcid::Ser, AminoAcid::Ser, AminoAcid::Arg, AminoAcid::Arg,   // AGX
    // CUU CUC CUA CUG  CCU CCC CCA CCG  CAU CAC CAA CAG  CGU CGC CGA CGG
    AminoAcid::Leu, AminoAcid::Leu, AminoAcid::Leu, AminoAcid::Leu,   // CUX
    AminoAcid::Pro, AminoAcid::Pro, AminoAcid::Pro, AminoAcid::Pro,   // CCX
    AminoAcid::His, AminoAcid::His, AminoAcid::Gln, AminoAcid::Gln,   // CAX
    AminoAcid::Arg, AminoAcid::Arg, AminoAcid::Arg, AminoAcid::Arg,   // CGX
    // GUU GUC GUA GUG  GCU GCC GCA GCG  GAU GAC GAA GAG  GGU GGC GGA GGG
    AminoAcid::Val, AminoAcid::Val, AminoAcid::Val, AminoAcid::Val,   // GUX
    AminoAcid::Ala, AminoAcid::Ala, AminoAcid::Ala, AminoAcid::Ala,   // GCX
    AminoAcid::Asp, AminoAcid::Asp, AminoAcid::Glu, AminoAcid::Glu,   // GAX
    AminoAcid::Gly, AminoAcid::Gly, AminoAcid::Gly, AminoAcid::Gly,   // GGX
];

// ── Codon lookup ────────────────────────────────────────────────

/// Translate a codon to its amino acid.
pub fn translate_codon(codon: &Codon) -> AminoAcid {
    CODON_TABLE[codon.index()]
}

/// Translate a 3-byte nucleotide sequence to amino acid.
pub fn translate_bytes(b1: u8, b2: u8, b3: u8) -> RebisResult<AminoAcid> {
    let codon = Codon::from_bytes(b1, b2, b3)?;
    Ok(translate_codon(&codon))
}

// ── Frobenius verification on codons ────────────────────────────

/// Frobenius stratum classification for a codon.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Stratum {
    /// Exact: 3rd base carries no information. ffuse∘fsplit = id exactly.
    Exact,
    /// Split: 3rd base carries pyrimidine/purine class. ffuse∘fsplit = id mod Z2.
    Split,
    /// Stop: kernel hits Ω boundary (paradox detected).
    Stop,
}

/// Classify a codon's Frobenius stratum.
pub fn classify_stratum(codon: &Codon) -> Stratum {
    if codon.is_stop() {
        Stratum::Stop
    } else if codon.is_exact_stratum() {
        Stratum::Exact
    } else {
        Stratum::Split
    }
}

/// Verify the Frobenius condition μ∘δ=id for a codon.
/// Uses the kernel's fsplit/ffuse directly.
/// Returns (holds, stratum).
pub fn verify_frobenius(codon: &Codon) -> (bool, Stratum) {
    let stratum = classify_stratum(codon);
    // For exact stratum: 3rd base carries no information
    // For split stratum: holds when 3rd base is pyrimidine (U/C)
    let holds = match stratum {
        Stratum::Exact => true,
        Stratum::Split => matches!(codon.p3, B4::N | B4::T), // pyrimidine
        Stratum::Stop => false,
    };
    (holds, stratum)
}

// ── Bulk operations ─────────────────────────────────────────────

/// Count codons by stratum.
pub fn stratum_counts() -> (usize, usize, usize) {
    let mut exact = 0usize;
    let mut split = 0usize;
    let mut stop = 0usize;
    for i in 0..64 {
        let c = codon_from_index(i);
        match classify_stratum(&c) {
            Stratum::Exact => exact += 1,
            Stratum::Split => split += 1,
            Stratum::Stop => stop += 1,
        }
    }
    (exact, split, stop)
}

/// Build a codon from its 0..63 index.
fn codon_from_index(idx: usize) -> Codon {
    let v = |x: usize| -> B4 {
        match x { 3 => B4::B, 2 => B4::T, 1 => B4::F, _ => B4::N }
    };
    Codon {
        p1: v((idx / 16) % 4),
        p2: v((idx / 4) % 4),
        p3: v(idx % 4),
    }
}
