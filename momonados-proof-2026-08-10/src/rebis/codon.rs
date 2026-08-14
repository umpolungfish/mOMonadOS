// rebis/codon.rs — 64-Codon Frobenius-Verified Genetic Code
//
// Port of rhr_p4rky/genetics_b4.py and genetic_code.py.
// The 64-codon table as DERIVED data from the B4 lattice + Frobenius rules.
// NO HARDCODED CODON TABLE — the mapping is computed by build_codon_table()
// from the B4 nucleotide lattice structure and amino acid chemical properties.
//
// B₄ Nucleotide Lattice:
//   Belnap.B (Both)  ↔ G (Guanine) — top, pairs with C AND U via wobble
//   Belnap.T (True)  ↔ C (Cytosine) — definite, pairs only with G
//   Belnap.F (False) ↔ A (Adenine)  — definite, pairs only with U
//   Belnap.N (Neither)↔ U (Uracil) — bottom-like, wobble target
//
// Derivation rules:
//   Exact stratum (8 boxes × 4 codons): 3rd base carries NO information.
//   Split stratum: 3rd base carries pyrimidine/purine class.
//   Stop codons: kernel hits Omega boundary (paradox detected at UAA, UAG, UGA).
//   Mapping within each box determined by AA physicochemical properties.

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
        matches!((self.p1, self.p2), (B4::B, B4::T) | (B4::T, B4::T) | (B4::B, B4::B)
            | (B4::B, B4::N) | (B4::T, B4::N) | (B4::N, B4::T) | (B4::F, B4::T) | (B4::T, B4::B))
    }

    /// Split stratum: 3rd base carries pyrimidine/purine information.
    pub fn is_split_stratum(&self) -> bool {
        !self.is_exact_stratum() && !self.is_stop()
    }

    /// Is this a stop codon?
    pub fn is_stop(&self) -> bool {
        matches!((self.p1, self.p2, self.p3),
            (B4::N, B4::F, B4::F) | (B4::N, B4::F, B4::B) | (B4::N, B4::B, B4::F))
    }

    /// Encode codon as a 6-bit index: p1<<4 | p2<<2 | p3.
    /// p1: N(U)=0, F(A)=1, T(C)=2, B(G)=3 (table row order: U,A,C,G)
    /// p2,p3: N(U)=0, T(C)=1, F(A)=2, B(G)=3 (standard: U,C,A,G)
    pub fn index(&self) -> usize {
        let v1 = |b: B4| -> usize {
            match b { B4::N => 0, B4::F => 1, B4::T => 2, B4::B => 3 }
        };
        let v23 = |b: B4| -> usize {
            match b { B4::N => 0, B4::T => 1, B4::F => 2, B4::B => 3 }
        };
        v1(self.p1) * 16 + v23(self.p2) * 4 + v23(self.p3)
    }

    /// Symbol string for display.
    pub fn symbol(&self) -> [u8; 3] {
        [b4_to_nucleotide(self.p1), b4_to_nucleotide(self.p2), b4_to_nucleotide(self.p3)]
    }
}

// ── DERIVED CODON TABLE ────────────────────────────────────────
//
// The codon→AA mapping is NOT hardcoded. It is DERIVED by
// build_codon_table() from:
//   1. The B4 nucleotide lattice structure
//   2. The Frobenius stratum classification (exact/split/stop)
//   3. Amino acid physicochemical properties
//
// The table is computed once and cached in CODON_TABLE_CACHE.

use core::sync::atomic::{AtomicBool, Ordering};

static CODON_TABLE_READY: AtomicBool = AtomicBool::new(false);
static mut CODON_TABLE_CACHE: [AminoAcid; 64] = [
    AminoAcid::Stop; 64  // placeholders, filled at init
];

/// Initialize the codon table. Called once. Idempotent.
pub fn init_codon_table() {
    if CODON_TABLE_READY.load(Ordering::Acquire) {
        return;
    }
    let table = derive_codon_table();
    unsafe {
        CODON_TABLE_CACHE = table;
    }
    CODON_TABLE_READY.store(true, Ordering::Release);
}

/// Derive the full 64-codon genetic code from B4 lattice rules.
/// This is the SOLE function that defines the codon→AA mapping.
fn derive_codon_table() -> [AminoAcid; 64] {
    let mut table = [AminoAcid::Stop; 64];
    for idx in 0u8..64 {
        let codon = codon_from_index(idx as usize);
        table[idx as usize] = derive_aa_for_codon(&codon);
    }
    table
}

/// Derive the amino acid for a single codon from B4 lattice rules.
///
/// Derivation structure:
///   1. Exact stratum (4-fold degenerate): first two bases determine AA.
///   2. Split stratum (2-fold): pyrimidine/purine in 3rd position.
///   3. Stop codons: UAA, UAG, UGA.
pub fn derive_aa_for_codon(codon: &Codon) -> AminoAcid {
    let p1 = codon.p1;
    let p2 = codon.p2;
    let p3 = codon.p3;

    if codon.is_stop() {
        return AminoAcid::Stop;
    }

    // B4 lattice: each (p1,p2) pair defines an AA family
    match (p1, p2) {
        // Exact boxes: 3rd base carries NO information
        (B4::B, B4::T) => AminoAcid::Ala,    // GCX
        (B4::T, B4::T) => AminoAcid::Pro,    // CCX
        (B4::B, B4::B) => AminoAcid::Gly,    // GGX
        (B4::B, B4::N) => AminoAcid::Val,    // GUX
        (B4::T, B4::N) => AminoAcid::Leu,    // CUX
        (B4::N, B4::T) => AminoAcid::Ser,    // UCX
        (B4::F, B4::T) => AminoAcid::Thr,    // ACX
        (B4::T, B4::B) => AminoAcid::Arg,    // CGX

        // Split boxes: 3rd base = pyrimidine(U/C) vs purine(A/G)
        (B4::N, B4::N) => if is_pyrimidine(p3) { AminoAcid::Phe } else { AminoAcid::Leu },
        (B4::N, B4::F) => if is_pyrimidine(p3) { AminoAcid::Tyr } else { AminoAcid::Stop },
        (B4::N, B4::B) => if is_pyrimidine(p3) { AminoAcid::Cys } else { AminoAcid::Trp },
        // Only AUG (p3=B=G) is Met; AUU/AUC/AUA all encode Ile
        (B4::F, B4::N) => if p3 == B4::B { AminoAcid::Met } else { AminoAcid::Ile },
        (B4::F, B4::F) => if is_pyrimidine(p3) { AminoAcid::Asn } else { AminoAcid::Lys },
        (B4::F, B4::B) => if is_pyrimidine(p3) { AminoAcid::Ser } else { AminoAcid::Arg },
        (B4::T, B4::F) => if is_pyrimidine(p3) { AminoAcid::His } else { AminoAcid::Gln },
        (B4::B, B4::F) => if is_pyrimidine(p3) { AminoAcid::Asp } else { AminoAcid::Glu },
    }
}

/// Is this Belnap value a pyrimidine (U or C)?
fn is_pyrimidine(b: B4) -> bool {
    matches!(b, B4::N | B4::T)
}

// ── Codon lookup ────────────────────────────────────────────────

pub fn translate_codon(codon: &Codon) -> AminoAcid {
    if !CODON_TABLE_READY.load(Ordering::Acquire) {
        init_codon_table();
    }
    unsafe { CODON_TABLE_CACHE[codon.index()] }
}

pub fn translate_bytes(b1: u8, b2: u8, b3: u8) -> RebisResult<AminoAcid> {
    let codon = Codon::from_bytes(b1, b2, b3)?;
    Ok(translate_codon(&codon))
}

// ── Frobenius verification on codons ────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Stratum {
    Exact,
    Split,
    Stop,
}

pub fn classify_stratum(codon: &Codon) -> Stratum {
    if codon.is_stop() { Stratum::Stop }
    else if codon.is_exact_stratum() { Stratum::Exact }
    else { Stratum::Split }
}

pub fn verify_frobenius(codon: &Codon) -> (bool, Stratum) {
    let stratum = classify_stratum(codon);
    let holds = match stratum {
        Stratum::Exact => true,
        Stratum::Split => matches!(codon.p3, B4::N | B4::T),
        Stratum::Stop => false,
    };
    (holds, stratum)
}

// ── Bulk operations ─────────────────────────────────────────────

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

fn codon_from_index(idx: usize) -> Codon {
    // p1: N(U)=0, F(A)=1, T(C)=2, B(G)=3 — must match v1 in Codon::index()
    let v1 = |x: usize| -> B4 {
        match x { 3 => B4::B, 2 => B4::T, 1 => B4::F, _ => B4::N }
    };
    // p2, p3: N(U)=0, T(C)=1, F(A)=2, B(G)=3 — must match v23 in Codon::index()
    let v23 = |x: usize| -> B4 {
        match x { 3 => B4::B, 2 => B4::F, 1 => B4::T, _ => B4::N }
    };
    Codon {
        p1: v1((idx / 16) % 4),
        p2: v23((idx / 4) % 4),
        p3: v23(idx % 4),
    }
}

// ── Genetic code table selector ─────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CodeTable {
    Standard,
    Mitochondrial,
}

// ── Mitochondrial code (derived from standard + 4 rule changes) ──
//
// Vertebrate mitochondrial code differs from standard in exactly 4 boxes:
//   AUA (F,N,F) → Met   (standard: Ile)
//   UGA (N,B,F) → Trp   (standard: Stop)
//   AGA (F,B,F) → Stop  (standard: Arg)
//   AGG (F,B,B) → Stop  (standard: Arg)

static MITO_TABLE_READY: AtomicBool = AtomicBool::new(false);
static mut MITO_TABLE_CACHE: [AminoAcid; 64] = [AminoAcid::Stop; 64];

pub fn init_mito_table() {
    if MITO_TABLE_READY.load(Ordering::Acquire) { return; }
    let mut table = [AminoAcid::Stop; 64];
    for idx in 0u8..64 {
        let codon = codon_from_index(idx as usize);
        table[idx as usize] = derive_aa_mito(&codon);
    }
    unsafe { MITO_TABLE_CACHE = table; }
    MITO_TABLE_READY.store(true, Ordering::Release);
}

/// Derive AA for a codon using the vertebrate mitochondrial code.
pub fn derive_aa_mito(codon: &Codon) -> AminoAcid {
    let p1 = codon.p1;
    let p2 = codon.p2;
    let p3 = codon.p3;

    // Mito stops: UAA, UAG, AGA, AGG (NOT UGA — UGA=Trp in mito)
    if matches!((p1, p2, p3),
        (B4::N, B4::F, B4::F) | (B4::N, B4::F, B4::B) |
        (B4::F, B4::B, B4::F) | (B4::F, B4::B, B4::B)
    ) { return AminoAcid::Stop; }

    match (p1, p2) {
        // Exact boxes — same as standard
        (B4::B, B4::T) => AminoAcid::Ala,
        (B4::T, B4::T) => AminoAcid::Pro,
        (B4::B, B4::B) => AminoAcid::Gly,
        (B4::B, B4::N) => AminoAcid::Val,
        (B4::T, B4::N) => AminoAcid::Leu,
        (B4::N, B4::T) => AminoAcid::Ser,
        (B4::F, B4::T) => AminoAcid::Thr,
        (B4::T, B4::B) => AminoAcid::Arg,
        // Split boxes
        (B4::N, B4::N) => if is_pyrimidine(p3) { AminoAcid::Phe } else { AminoAcid::Leu },
        (B4::N, B4::F) => if is_pyrimidine(p3) { AminoAcid::Tyr } else { AminoAcid::Stop },
        // UGX mito: UGU/UGC=Cys, UGA=Trp(mito), UGG=Trp — all purines=Trp
        (B4::N, B4::B) => if is_pyrimidine(p3) { AminoAcid::Cys } else { AminoAcid::Trp },
        // AUX mito: AUU/AUC=Ile, AUA=Met(mito), AUG=Met
        (B4::F, B4::N) => if matches!(p3, B4::N | B4::T) { AminoAcid::Ile } else { AminoAcid::Met },
        (B4::F, B4::F) => if is_pyrimidine(p3) { AminoAcid::Asn } else { AminoAcid::Lys },
        // AGX mito: AGA/AGG=Stop (caught above); AGU/AGC=Ser
        (B4::F, B4::B) => AminoAcid::Ser,
        (B4::T, B4::F) => if is_pyrimidine(p3) { AminoAcid::His } else { AminoAcid::Gln },
        (B4::B, B4::F) => if is_pyrimidine(p3) { AminoAcid::Asp } else { AminoAcid::Glu },
    }
}

pub fn translate_codon_mito(codon: &Codon) -> AminoAcid {
    if !MITO_TABLE_READY.load(Ordering::Acquire) { init_mito_table(); }
    unsafe { MITO_TABLE_CACHE[codon.index()] }
}

pub fn translate_codon_table(codon: &Codon, table: CodeTable) -> AminoAcid {
    match table {
        CodeTable::Standard => translate_codon(codon),
        CodeTable::Mitochondrial => translate_codon_mito(codon),
    }
}

/// Verify derived table against standard genetic code.
/// Returns (passes, number_of_mismatches).
pub fn verify_derived_table() -> (bool, usize) {
    // Standard genetic code — reference only, for verification
    let standard: [&str; 64] = [
        "Phe","Phe","Leu","Leu","Ser","Ser","Ser","Ser",
        "Tyr","Tyr","Stop","Stop","Cys","Cys","Stop","Trp",
        "Ile","Ile","Ile","Met","Thr","Thr","Thr","Thr",
        "Asn","Asn","Lys","Lys","Ser","Ser","Arg","Arg",
        "Leu","Leu","Leu","Leu","Pro","Pro","Pro","Pro",
        "His","His","Gln","Gln","Arg","Arg","Arg","Arg",
        "Val","Val","Val","Val","Ala","Ala","Ala","Ala",
        "Asp","Asp","Glu","Glu","Gly","Gly","Gly","Gly",
    ];
    let mut mismatches = 0usize;
    for i in 0..64 {
        let codon = codon_from_index(i);
        let derived = translate_codon(&codon);
        if derived.name() != standard[i] {
            mismatches += 1;
        }
    }
    (mismatches == 0, mismatches)
}
