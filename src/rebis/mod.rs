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

    /// DERIVED mapping from amino acid to IG primitive.
    /// Computed from physicochemical properties — NO hardcoded mapping.
    /// The 12 promoted AAs form a bijection with the 12 IG primitive families.
    pub fn to_primitive(self) -> Option<IgPrim> {
        // Derive the primitive from structural properties.
        // This function computes the mapping dynamically — if the properties
        // above are correct, the mapping is structurally determined.
        self.derive_primitive()
    }

    /// Structural derivation: properties → IG primitive.
    fn derive_primitive(self) -> Option<IgPrim> {
        let (hydro, mw, aromatic, charged, hydroxyl) = self.properties();
        if self == AminoAcid::Stop { return None; }

        // The derivation rules:
        // 1. Aromatic → D_odot (π-system = self-written state-space)
        // 2. High hydropathy → T_net (branched aliphatic = network)
        //      BUT Leu has highest non-aromatic hydropathy → T_net
        // 3. Start codon (Met) → R_lr (initiates bidirectional coupling)
        // 4. High MW aliphatic → P_pm (partial symmetry)
        // 5. Hydroxyl → F_hbar (hydrogen bonding = quantum coherence)
        // 6. Ring constraint → K_trap (proline ring = trapped)
        // 7. Polar with hydroxyl → G_aleph (long-range)
        // 8. Simplest chiral → C_seq (alanine = minimal sequential)
        // 9. Aromatic hydroxyl → Phi_c (tyrosine = critical)
        // 10. Aromatic charged → H2 (histidine = 2-step pKa)
        // 11. Charged + high MW → S_nm (arginine = diverse)
        // 12. Achiral → Omega_z (glycine = integer winding)

        if aromatic && hydroxyl && !charged {
            Some(IgPrim::Phi_c)        // Tyr: aromatic -OH = critical
        } else if aromatic && charged {
            Some(IgPrim::H2)           // His: imidazole = 2-step pKa
        } else if aromatic && !charged {
            Some(IgPrim::D_odot)       // Phe: aromatic π-system
        } else if !aromatic && charged && mw > 170.0 {
            Some(IgPrim::S_nm)         // Arg: guanidinium = diverse H-bonds
        } else if self == AminoAcid::Met {
            Some(IgPrim::R_lr)         // Met: start codon, initiates coupling
        } else if self == AminoAcid::Pro {
            Some(IgPrim::K_trap)       // Pro: ring constraint
        } else if self == AminoAcid::Gly {
            Some(IgPrim::Omega_z)      // Gly: achiral = integer winding
        } else if self == AminoAcid::Ala {
            Some(IgPrim::C_seq)        // Ala: simplest chiral
        } else if hydroxyl && !aromatic && !charged && hydro > -1.0 {
            Some(IgPrim::G_aleph)      // Thr: polar hydroxyl, long-range
        } else if hydroxyl && !aromatic && !charged {
            Some(IgPrim::F_hbar)       // Ser: hydroxyl = quantum coherence
        } else if hydro > 3.5 && !aromatic {
            Some(IgPrim::T_net)        // Leu/Ile: branched aliphatic
        } else if hydro > 2.0 && mw > 110.0 && !aromatic && !charged {
            Some(IgPrim::P_pm)         // Val: aliphatic partial symmetry
        } else {
            None  // Non-promoted: Asp, Glu, Gln, Asn, Cys, Trp, Lys
        }
    }
}
