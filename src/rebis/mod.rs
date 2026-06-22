// rebis/mod.rs — Red-Hot Rebis Kernel Module
//
// A no_std Rust port of the red-hot_rebis Python framework,
// running directly from the mOMonadOS bare-metal kernel.
//
// Author: Lando⊗⊙perator
// Date: 2026-06-11
//
// Principle: NO HARDCODE. All types derive from the single-source-of-truth
// IgPrim enum in imas_ig.rs. No duplicate primitive definitions exist.
//
// Structure:
//   codon.rs    — 64-codon Frobenius-verified genetic code (dynamically derived)
//   genetics.rs — B₄ lattice, codon→AA translation, Frobenius stratum
//   translate.rs — Gene→protein translation pipeline
//   frob_filter.rs — Frobenius filtration (frobenius_filtration.py)
//   hadron.rs   — Hadron/quark/orbital Belnap analysis
//   serpent.rs  — Serpent rod protein design data (precomputed)
//   pipeline.rs — IG promotion pipeline (compute_promotions port)
//   genetic_asm.rs — Genetic code ParaASM programs
//   genetic_tuples.rs — Generative tuple construction (all 12 value types)
//   clu.rs — CLU power-law clustering (avalanche distribution, Frobenius filtration)
//   exotic_hadron.rs — Exotic hadron Frobenius verification (Glueball, Tetraquark, Pentaquark)
//   pdb.rs — PDB structure validation (CA atom, contacts, precision/recall)
//   antibody.rs — Antibody CDR design (12↔12 bijection, epitope, full antibody)
//   materials.rs — IG material forge (MetaCell, Alloy, ThermalRectifier, NonQubitQC)
//   biology.rs — Biological simulation (TissueGrid, Telomere, FrobeniusBioSim)
//   therapeutics.rs — Therapeutic design (Chemotherapeutic, OuroboricPill, Antidote)

pub mod codon;
pub mod genetics;
pub mod translate;
pub mod fold;
pub mod frob_filter;
pub mod hadron;
pub mod serpent;
pub mod pipeline;
pub mod genetic_asm;
pub mod genetic_tuples;
pub mod clu;
pub mod exotic_hadron;
pub mod pdb;
pub mod antibody;
pub mod materials;
pub mod biology;
pub mod therapeutics;
pub mod clink;
pub mod imas;

// ── SINGLE SOURCE OF TRUTH: re-export IgPrim from the grammar kernel ──
// ALL primitive values across the entire codebase are this ONE type.
// There is no RebisPrim — there is only IgPrim.
pub use crate::imas_ig::IgPrim;

/// Result type for Rebis operations.
pub type RebisResult<T> = Result<T, &'static str>;

/// The 20 standard amino acids + Stop.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AminoAcid {
    Phe, Leu, Ile, Met, Val,
    Ser, Pro, Thr, Ala,
    Tyr, Stop, His, Gln,
    Asn, Lys, Asp, Glu,
    Cys, Trp, Arg, Gly,
}

impl AminoAcid {
    pub fn name(self) -> &'static str {
        match self {
            Self::Phe => "Phe", Self::Leu => "Leu", Self::Ile => "Ile", Self::Met => "Met",
            Self::Val => "Val", Self::Ser => "Ser", Self::Pro => "Pro", Self::Thr => "Thr",
            Self::Ala => "Ala", Self::Tyr => "Tyr", Self::Stop => "Stop", Self::His => "His",
            Self::Gln => "Gln", Self::Asn => "Asn", Self::Lys => "Lys", Self::Asp => "Asp",
            Self::Glu => "Glu", Self::Cys => "Cys", Self::Trp => "Trp", Self::Arg => "Arg",
            Self::Gly => "Gly",
        }
    }

    /// Three-letter code for this amino acid.
    pub fn code3(self) -> &'static str {
        self.name()
    }

    /// One-letter code for this amino acid.
    pub fn code1(self) -> &'static str {
        match self {
            Self::Phe => "F", Self::Leu => "L", Self::Ile => "I", Self::Met => "M",
            Self::Val => "V", Self::Ser => "S", Self::Pro => "P", Self::Thr => "T",
            Self::Ala => "A", Self::Tyr => "Y", Self::Stop => "*", Self::His => "H",
            Self::Gln => "Q", Self::Asn => "N", Self::Lys => "K", Self::Asp => "D",
            Self::Glu => "E", Self::Cys => "C", Self::Trp => "W", Self::Arg => "R",
            Self::Gly => "G",
        }
    }

    /// Physicochemical properties used for structural derivation.
    /// Returns (hydropathy_index, molecular_weight, is_aromatic, is_charged, has_hydroxyl).
    pub fn properties(self) -> (f32, f32, bool, bool, bool) {
        match self {
            Self::Phe => (2.8, 165.19, true, false, false),
            Self::Leu => (3.8, 131.17, false, false, false),
            Self::Ile => (4.5, 131.17, false, false, false),
            Self::Met => (1.9, 149.21, false, false, false),
            Self::Val => (4.2, 117.15, false, false, false),
            Self::Ser => (-0.8, 105.09, false, false, true),
            Self::Pro => (-1.6, 115.13, false, false, false),
            Self::Thr => (-0.7, 119.12, false, false, true),
            Self::Ala => (1.8, 89.09, false, false, false),
            Self::Tyr => (-1.3, 181.19, true, false, true),
            Self::Stop => (0.0, 0.0, false, false, false),
            Self::His => (-3.2, 155.16, true, true, false),
            Self::Gln => (-3.5, 146.15, false, false, false),
            Self::Asn => (-3.5, 132.12, false, false, false),
            Self::Lys => (-3.9, 146.19, false, true, false),
            Self::Asp => (-3.5, 133.10, false, true, false),
            Self::Glu => (-3.5, 147.13, false, true, false),
            Self::Cys => (2.5, 121.16, false, false, false),
            Self::Trp => (-0.9, 204.23, true, false, false),
            Self::Arg => (-4.5, 174.20, false, true, false),
            Self::Gly => (-0.4, 75.07, false, false, false),
        }
    }

    /// Derived mapping from AA to IG primitive via codon box position.
    /// An AA is promoted iff all its codons are in the split stratum (no exact box).
    /// The box (p1,p2) and pyrimidine/purine half determine the primitive family.
    pub fn to_primitive(self) -> Option<IgPrim> {
        self.derive_primitive()
    }

    /// Canonical primitive name string, derived from codon box position.
    /// Returns None for ground-layer AAs (those with any exact-stratum codon).
    pub fn primitive_name(self) -> Option<&'static str> {
        use crate::rebis::genetics::codons_for_aa;
        use crate::belnap::B4;
        if self == AminoAcid::Stop { return None; }
        let codons = codons_for_aa(self);
        if codons.is_empty() { return None; }
        if codons.iter().any(|c| c.is_exact_stratum()) { return None; }
        let c = &codons[0];
        let is_pyr = matches!(c.p3, B4::N | B4::T);
        Some(match (c.p1, c.p2, is_pyr) {
            (B4::N, B4::N, _)      => "ƒ (Fidelity)",         // Phe
            (B4::N, B4::F, _)      => "Φ (Parity)",           // Tyr
            (B4::N, B4::B, true)   => "Ř (Recognition)",      // Cys
            (B4::N, B4::B, false)  => "Þ (Topology)",         // Trp
            (B4::F, B4::N, true)   => "Ç (Kinetics)",         // Ile
            (B4::F, B4::N, false)  => "Ð (Dimensionality)",   // Met
            (B4::F, B4::F, true)   => "ɢ (Coupling)",         // Asn
            (B4::F, B4::F, false)  => "Σ (Stoichiometry)",    // Lys
            (B4::T, B4::F, true)   => "Γ (Granularity)",      // His
            (B4::T, B4::F, false)  => "⊙ (Criticality)",      // Gln
            (B4::B, B4::F, true)   => "Ħ (Chirality)",        // Asp
            (B4::B, B4::F, false)  => "Ω (Winding)",          // Glu
            _                      => return None,
        })
    }

    /// Structural derivation: codon box position → IG primitive variant.
    fn derive_primitive(self) -> Option<IgPrim> {
        use crate::rebis::genetics::codons_for_aa;
        use crate::belnap::B4;
        if self == AminoAcid::Stop { return None; }
        let codons = codons_for_aa(self);
        if codons.is_empty() { return None; }
        if codons.iter().any(|c| c.is_exact_stratum()) { return None; }
        let c = &codons[0];
        let is_pyr = matches!(c.p3, B4::N | B4::T);
        match (c.p1, c.p2, is_pyr) {
            (B4::N, B4::N, _)      => Some(IgPrim::F_hbar),   // Phe → ƒ
            (B4::N, B4::F, _)      => Some(IgPrim::Phi_sub),   // Tyr → Φ (Parity, sub-critical gate)
            (B4::N, B4::B, true)   => Some(IgPrim::R_lr),      // Cys → Ř
            (B4::N, B4::B, false)  => Some(IgPrim::T_net),     // Trp → Þ
            (B4::F, B4::N, true)   => Some(IgPrim::K_mod),     // Ile → Ç
            (B4::F, B4::N, false)  => Some(IgPrim::D_infty),   // Met → Ð
            (B4::F, B4::F, true)   => Some(IgPrim::G_aleph),   // Asn → ɢ
            (B4::F, B4::F, false)  => Some(IgPrim::S_nm),      // Lys → Σ
            (B4::T, B4::F, true)   => Some(IgPrim::C_seq),     // His → Γ
            (B4::T, B4::F, false)  => Some(IgPrim::Phi_c),     // Gln → ⊙
            (B4::B, B4::F, true)   => Some(IgPrim::H2),        // Asp → Ħ
            (B4::B, B4::F, false)  => Some(IgPrim::Omega_z),   // Glu → Ω
            _                      => None,
        }
    }
}
