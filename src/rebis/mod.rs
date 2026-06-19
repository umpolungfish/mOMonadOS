// rebis/mod.rs — Red-Hot Rebis Kernel Module
//
// A no_std Rust port of the red-hot_rebis Python framework,
// running directly from the mOMonadOS bare-metal kernel.
//
// Author: Lando⊗⊙perator
// Date: 2026-06-11
//
// Structure:
//   codon.rs    — 64-codon Frobenius-verified genetic code (static data)
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


/// The 12 IG primitives as Shavian glyphs — used for promotion tables
/// and structural mapping between the genetic code and the grammar.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum RebisPrim {
    D_odot     = 0,  // 𐑦
    D_wedge    = 1,  // 𐑛
    D_triangle = 2,  // 𐑨
    D_infty    = 3,  // 𐑼
    T_odot     = 4,  // 𐑸
    T_net      = 5,  // 𐑡
    T_in       = 6,  // 𐑰
    T_bowtie   = 7,  // 𐑥
    T_boxtimes = 8,  // 𐑶
    R_lr       = 9,  // 𐑾
    R_dagger   = 10, // 𐑽
    R_cat      = 11, // 𐑑
    R_super    = 12, // 𐑩
    P_pmsym    = 13, // 𐑹
    P_sym      = 14, // 𐑯
    P_pm       = 15, // 𐑬
    P_psi      = 16, // 𐑿
    P_asym     = 17, // 𐑗
    F_hbar     = 18, // 𐑐
    F_ell      = 19, // 𐑱
    F_eth      = 20, // 𐑞
    K_trap     = 21, // 𐑪
    K_slow     = 22, // 𐑧
    K_mod      = 23, // 𐑤
    K_fast     = 24, // 𐑘
    K_mbl      = 25, // 𐑺
    G_aleph    = 26, // 𐑲
    G_beth     = 27, // 𐑚
    G_gimel    = 28, // 𐑔
    C_seq      = 29, // 𐑠
    C_or_      = 30, // 𐑜
    C_and      = 31, // 𐑝
    C_broad    = 32, // 𐑵
    Ph_c       = 33, // ⊙
    Ph_sub     = 34, // 𐑢
    Ph_c_complex = 35, // 𐑮
    Ph_EP      = 36, // 𐑻
    Ph_super   = 37, // 𐑣
    H_inf      = 38, // 𐑫
    H_2        = 39, // 𐑖
    H_1        = 40, // 𐑒
    H_0        = 41, // 𐑓
    S_hetero   = 42, // 𐑳
    S_many     = 43, // 𐑕
    S_one_one  = 44, // 𐑙
    W_Z        = 45, // 𐑭
    W_Z2       = 46, // 𐑴
    W_triv     = 47, // 𐑷
    W_NA       = 48, // 𐑟
}

impl RebisPrim {
    pub fn glyph(self) -> &'static str {
        match self {
            Self::D_odot => "𐑦", Self::D_wedge => "𐑛", Self::D_triangle => "𐑨", Self::D_infty => "𐑼",
            Self::T_odot => "𐑸", Self::T_net => "𐑡", Self::T_in => "𐑰", Self::T_bowtie => "𐑥", Self::T_boxtimes => "𐑶",
            Self::R_lr => "𐑾", Self::R_dagger => "𐑽", Self::R_cat => "𐑑", Self::R_super => "𐑩",
            Self::P_pmsym => "𐑹", Self::P_sym => "𐑯", Self::P_pm => "𐑬", Self::P_psi => "𐑿", Self::P_asym => "𐑗",
            Self::F_hbar => "𐑐", Self::F_ell => "𐑱", Self::F_eth => "𐑞",
            Self::K_trap => "𐑪", Self::K_slow => "𐑧", Self::K_mod => "𐑤", Self::K_fast => "𐑘", Self::K_mbl => "𐑺",
            Self::G_aleph => "𐑲", Self::G_beth => "𐑚", Self::G_gimel => "𐑔",
            Self::C_seq => "𐑠", Self::C_or_ => "𐑜", Self::C_and => "𐑝", Self::C_broad => "𐑵",
            Self::Ph_c => "⊙", Self::Ph_sub => "𐑢", Self::Ph_c_complex => "𐑮", Self::Ph_EP => "𐑻", Self::Ph_super => "𐑣",
            Self::H_inf => "𐑫", Self::H_2 => "𐑖", Self::H_1 => "𐑒", Self::H_0 => "𐑓",
            Self::S_hetero => "𐑳", Self::S_many => "𐑕", Self::S_one_one => "𐑙",
            Self::W_Z => "𐑭", Self::W_Z2 => "𐑴", Self::W_triv => "𐑷", Self::W_NA => "𐑟",
        }
    }
}

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

    /// Map amino acid to its corresponding IG primitive.
    /// 12 promoted AAs form a bijection with the 12 IG primitive families.
    pub fn to_primitive(self) -> Option<RebisPrim> {
        match self {
            Self::Phe => Some(RebisPrim::D_odot),     // aromatic: self-written
            Self::Leu => Some(RebisPrim::T_net),       // branched: network topology
            Self::Met => Some(RebisPrim::R_lr),        // start: initiates coupling
            Self::Val => Some(RebisPrim::P_pm),        // aliphatic: partial symmetry
            Self::Ser => Some(RebisPrim::F_hbar),      // hydroxyl: quantum coherence
            Self::Pro => Some(RebisPrim::K_trap),      // ring constraint: trapped
            Self::Thr => Some(RebisPrim::G_aleph),     // polar: long-range
            Self::Ala => Some(RebisPrim::C_seq),       // simplest chiral: sequential
            Self::Tyr => Some(RebisPrim::Ph_c),        // aromatic -OH: critical
            Self::His => Some(RebisPrim::H_2),         // imidazole: 2-step pKa
            Self::Arg => Some(RebisPrim::S_hetero),    // guanidinium: diverse H-bonds
            Self::Gly => Some(RebisPrim::W_Z),         // achiral: integer winding
            _ => None,
        }
    }
}
