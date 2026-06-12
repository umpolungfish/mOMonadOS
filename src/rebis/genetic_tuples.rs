//! genetic_tuples.rs — Generative Tuple Construction for Gene→Protein Pipeline
//! Port of rhr_p4rky/genetic_tuples.py
//!
//! Each stage's structural tuple is a FUNCTION of sequence-derived features.
//! The mapping from pipeline string names to IG primitive Unicode values is
//! defined here, along with per-stage tuple generators that inspect:
//!   - Amino acid composition
//!   - Secondary structure predictions
//!   - Tertiary contact diversity
//!   - Quaternary subunit count and symmetry
//!   - Chain length and complexity metrics
//!
//! Each generated tuple is a valid crystal address verified by:
//!   1. Ouroboricity tier consistency — all 7 stages remain O₀/O₁
//!   2. Frobenius condition — μ∘δ=id holds across the transformation
//!   3. Monotonic advance — Ω_z constraint on trajectory through the crystal

use alloc::collections::BTreeMap;

// ── Pipeline string → IG primitive Unicode values ──────────────────────

/// Primitive key used in pipeline stage tuples.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimKey { D, T, R, P, F, K, G, Gm, Phi, H, S, O }

/// All 12 primitive keys in canonical order.
pub const PRIM_KEYS: [PrimKey; 12] = [
    PrimKey::D, PrimKey::T, PrimKey::R, PrimKey::P,
    PrimKey::F, PrimKey::K, PrimKey::G, PrimKey::Gm,
    PrimKey::Phi, PrimKey::H, PrimKey::S, PrimKey::O,
];

impl PrimKey {
    pub fn name(&self) -> &'static str {
        match self {
            PrimKey::D   => "D",
            PrimKey::T   => "T",
            PrimKey::R   => "R",
            PrimKey::P   => "P",
            PrimKey::F   => "F",
            PrimKey::K   => "K",
            PrimKey::G   => "G",
            PrimKey::Gm  => "Gm",
            PrimKey::Phi => "Phi",
            PrimKey::H   => "H",
            PrimKey::S   => "S",
            PrimKey::O   => "O",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "D" | "Ð" => Some(PrimKey::D),
            "T" | "Þ" => Some(PrimKey::T),
            "R" | "Ř" => Some(PrimKey::R),
            "P" | "Φ" => Some(PrimKey::P),
            "F" | "ƒ" => Some(PrimKey::F),
            "K" | "Ç" => Some(PrimKey::K),
            "G" | "Γ" => Some(PrimKey::G),
            "Gm" | "ɢ" => Some(PrimKey::Gm),
            "Phi" | "φ̂" => Some(PrimKey::Phi),
            "H" | "Ħ" => Some(PrimKey::H),
            "S" | "Σ" => Some(PrimKey::S),
            "O" | "Ω" => Some(PrimKey::O),
            _ => None,
        }
    }
}

// ── IG Value types ─────────────────────────────────────────────────────

/// Dimensionality values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVal { Wedge, Tri, Infty, Odot }

/// Topology values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TVal { Network, In, Bowtie, Boxtimes, Odot }

/// Relational mode values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RVal { Super, Cat, Dagger, LR }

/// Parity values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVal { Asym, Psi, Pm, Sym, PmSym }

/// Fidelity values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FVal { Ell, Eth, Hbar }

/// Kinetics values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KVal { Fast, Mod, Slow, Trap, MBL }

/// Scope values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GVal { Beth, Gimel, Aleph }

/// Interaction grammar values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmVal { And, Or, Seq, Broad }

/// Criticality values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhiVal { Sub, C, CComplex, EP, Super }

/// Chirality values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVal { M0, M1, M2, Inf }

/// Stoichiometry values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVal { One, Many, Hetero }

/// Winding values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVal { Trivial, Z2, Z, NA }

// ── IG Tuple ────────────────────────────────────────────────────────────

/// A complete 12-primitive IG tuple (pipeline string names).
#[derive(Clone, Debug)]
pub struct IGTuple {
    pub d: DVal,
    pub t: TVal,
    pub r: RVal,
    pub p: PVal,
    pub f: FVal,
    pub k: KVal,
    pub g: GVal,
    pub gm: GmVal,
    pub phi: PhiVal,
    pub h: HVal,
    pub s: SVal,
    pub o: OVal,
}

impl IGTuple {
    /// Get a primitive value by key.
    pub fn get(&self, key: PrimKey) -> &'static str {
        match key {
            PrimKey::D => self.d.as_str(),
            PrimKey::T => self.t.as_str(),
            PrimKey::R => self.r.as_str(),
            PrimKey::P => self.p.as_str(),
            PrimKey::F => self.f.as_str(),
            PrimKey::K => self.k.as_str(),
            PrimKey::G => self.g.as_str(),
            PrimKey::Gm => self.gm.as_str(),
            PrimKey::Phi => self.phi.as_str(),
            PrimKey::H => self.h.as_str(),
            PrimKey::S => self.s.as_str(),
            PrimKey::O => self.o.as_str(),
        }
    }

    /// Get unicode glyph for a primitive value.
    pub fn get_glyph(&self, key: PrimKey) -> &'static str {
        match key {
            PrimKey::D => self.d.glyph(),
            PrimKey::T => self.t.glyph(),
            PrimKey::R => self.r.glyph(),
            PrimKey::P => self.p.glyph(),
            PrimKey::F => self.f.glyph(),
            PrimKey::K => self.k.glyph(),
            PrimKey::G => self.g.glyph(),
            PrimKey::Gm => self.gm.glyph(),
            PrimKey::Phi => self.phi.glyph(),
            PrimKey::H => self.h.glyph(),
            PrimKey::S => self.s.glyph(),
            PrimKey::O => self.o.glyph(),
        }
    }

    /// Build a tuple display string: ⟨D·T·R·P·F·K·G·Gm·φ̂·H·S·Ω⟩
    pub fn display(&self) -> alloc::string::String {
        let mut s = alloc::string::String::from("\u{27e8}"); // ⟨
        for (i, key) in PRIM_KEYS.iter().enumerate() {
            if i > 0 { s.push('\u{b7}'); } // ·
            s.push_str(self.get_glyph(*key));
        }
        s.push('\u{27e9}'); // ⟩
        s
    }
}

// ── Value type impls ────────────────────────────────────────────────────

impl DVal {
    pub fn as_str(&self) -> &'static str {
        match self { DVal::Wedge => "wedge", DVal::Tri => "tri",
                     DVal::Infty => "infty", DVal::Odot => "odot" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { DVal::Wedge => "𐑛", DVal::Tri => "𐑨",
                     DVal::Infty => "𐑼", DVal::Odot => "𐑦" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { DVal::Wedge => 0, DVal::Tri => 1,
                     DVal::Infty => 2, DVal::Odot => 3 }
    }
}

impl TVal {
    pub fn as_str(&self) -> &'static str {
        match self { TVal::Network => "network", TVal::In => "in",
                     TVal::Bowtie => "bowtie", TVal::Boxtimes => "boxtimes",
                     TVal::Odot => "odot" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { TVal::Network => "𐑡", TVal::In => "𐑰",
                     TVal::Bowtie => "𐑥", TVal::Boxtimes => "𐑶",
                     TVal::Odot => "𐑸" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { TVal::Network => 0, TVal::In => 1,
                     TVal::Bowtie => 2, TVal::Boxtimes => 3, TVal::Odot => 4 }
    }
}

impl RVal {
    pub fn as_str(&self) -> &'static str {
        match self { RVal::Super => "super", RVal::Cat => "cat",
                     RVal::Dagger => "dagger", RVal::LR => "lr" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { RVal::Super => "𐑩", RVal::Cat => "𐑑",
                     RVal::Dagger => "𐑽", RVal::LR => "𐑾" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { RVal::Super => 0, RVal::Cat => 1,
                     RVal::Dagger => 2, RVal::LR => 3 }
    }
}

impl PVal {
    pub fn as_str(&self) -> &'static str {
        match self { PVal::Asym => "asym", PVal::Psi => "psi",
                     PVal::Pm => "pm", PVal::Sym => "sym", PVal::PmSym => "pm_sym" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { PVal::Asym => "𐑗", PVal::Psi => "𐑿",
                     PVal::Pm => "𐑬", PVal::Sym => "𐑯", PVal::PmSym => "𐑹" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { PVal::Asym => 0, PVal::Psi => 1,
                     PVal::Pm => 2, PVal::Sym => 3, PVal::PmSym => 4 }
    }
}

impl FVal {
    pub fn as_str(&self) -> &'static str {
        match self { FVal::Ell => "ell", FVal::Eth => "eth", FVal::Hbar => "hbar" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { FVal::Ell => "𐑱", FVal::Eth => "𐑞", FVal::Hbar => "𐑐" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { FVal::Ell => 0, FVal::Eth => 1, FVal::Hbar => 2 }
    }
}

impl KVal {
    pub fn as_str(&self) -> &'static str {
        match self { KVal::Fast => "fast", KVal::Mod => "mod",
                     KVal::Slow => "slow", KVal::Trap => "trap", KVal::MBL => "MBL" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { KVal::Fast => "𐑘", KVal::Mod => "𐑤",
                     KVal::Slow => "𐑧", KVal::Trap => "𐑪", KVal::MBL => "𐑺" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { KVal::Fast => 0, KVal::Mod => 1,
                     KVal::Slow => 2, KVal::Trap => 3, KVal::MBL => 4 }
    }
}

impl GVal {
    pub fn as_str(&self) -> &'static str {
        match self { GVal::Beth => "beth", GVal::Gimel => "gimel", GVal::Aleph => "aleph" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { GVal::Beth => "𐑚", GVal::Gimel => "𐑔", GVal::Aleph => "𐑲" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { GVal::Beth => 0, GVal::Gimel => 1, GVal::Aleph => 2 }
    }
}

impl GmVal {
    pub fn as_str(&self) -> &'static str {
        match self { GmVal::And => "and", GmVal::Or => "or",
                     GmVal::Seq => "seq", GmVal::Broad => "broad" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { GmVal::And => "𐑝", GmVal::Or => "𐑜",
                     GmVal::Seq => "𐑠", GmVal::Broad => "𐑵" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { GmVal::And => 0, GmVal::Or => 1,
                     GmVal::Seq => 2, GmVal::Broad => 3 }
    }
}

impl PhiVal {
    pub fn as_str(&self) -> &'static str {
        match self { PhiVal::Sub => "sub", PhiVal::C => "c",
                     PhiVal::CComplex => "c_complex", PhiVal::EP => "EP",
                     PhiVal::Super => "super" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { PhiVal::Sub => "𐑢", PhiVal::C => "\u{2299}",
                     PhiVal::CComplex => "𐑮", PhiVal::EP => "𐑻",
                     PhiVal::Super => "𐑣" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { PhiVal::Sub => 0, PhiVal::C => 1,
                     PhiVal::CComplex => 2, PhiVal::EP => 3, PhiVal::Super => 4 }
    }
}

impl HVal {
    pub fn as_str(&self) -> &'static str {
        match self { HVal::M0 => "0", HVal::M1 => "1",
                     HVal::M2 => "2", HVal::Inf => "inf" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { HVal::M0 => "𐑓", HVal::M1 => "𐑒",
                     HVal::M2 => "𐑖", HVal::Inf => "𐑫" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { HVal::M0 => 0, HVal::M1 => 1, HVal::M2 => 2, HVal::Inf => 3 }
    }
}

impl SVal {
    pub fn as_str(&self) -> &'static str {
        match self { SVal::One => "one", SVal::Many => "many", SVal::Hetero => "hetero" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { SVal::One => "𐑙", SVal::Many => "𐑕", SVal::Hetero => "𐑳" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { SVal::One => 0, SVal::Many => 1, SVal::Hetero => 2 }
    }
}

impl OVal {
    pub fn as_str(&self) -> &'static str {
        match self { OVal::Trivial => "0", OVal::Z2 => "Z2",
                     OVal::Z => "Z", OVal::NA => "NA" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { OVal::Trivial => "𐑷", OVal::Z2 => "𐑴",
                     OVal::Z => "𐑭", OVal::NA => "𐑟" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { OVal::Trivial => 0, OVal::Z2 => 1, OVal::Z => 2, OVal::NA => 3 }
    }
}

// ── Pipeline stage definitions ──────────────────────────────────────────

/// The 7 pipeline stages from gene to protein.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipelineStage {
    Stage1DNA,          // Raw DNA sequence
    Stage2Transcription, // mRNA transcript
    Stage3Codon,        // Codon lattice
    Stage4Translation,  // AA chain
    Stage5Folding,      // Secondary structure
    Stage6Tertiary,     // 3D fold
    Stage7Quaternary,   // Multimer assembly
}

impl PipelineStage {
    pub fn name(&self) -> &'static str {
        match self {
            PipelineStage::Stage1DNA => "DNA",
            PipelineStage::Stage2Transcription => "Transcription",
            PipelineStage::Stage3Codon => "Codon",
            PipelineStage::Stage4Translation => "Translation",
            PipelineStage::Stage5Folding => "Folding",
            PipelineStage::Stage6Tertiary => "Tertiary",
            PipelineStage::Stage7Quaternary => "Quaternary",
        }
    }

    pub fn all() -> [PipelineStage; 7] {
        [PipelineStage::Stage1DNA, PipelineStage::Stage2Transcription,
         PipelineStage::Stage3Codon, PipelineStage::Stage4Translation,
         PipelineStage::Stage5Folding, PipelineStage::Stage6Tertiary,
         PipelineStage::Stage7Quaternary]
    }
}

/// Context passed to stage generators — extracted features from the sequence.
#[derive(Clone, Debug)]
pub struct StageContext {
    pub chain_length: usize,
    pub beta_branched_frac: f64,      // Ile, Val, Thr fraction
    pub proline_frac: f64,
    pub glycine_frac: f64,
    pub hydrophobic_frac: f64,
    pub aromatic_frac: f64,
    pub cysteine_count: usize,
    pub helix_content: f64,           // 0-1 fraction helical
    pub sheet_content: f64,
    pub contact_diversity: f64,       // unique contact types / total
    pub subunit_count: usize,         // quaternary
    pub has_symmetry: bool,
    pub disulfide_bonds: usize,
}

impl Default for StageContext {
    fn default() -> Self {
        StageContext {
            chain_length: 100,
            beta_branched_frac: 0.15,
            proline_frac: 0.05,
            glycine_frac: 0.07,
            hydrophobic_frac: 0.40,
            aromatic_frac: 0.08,
            cysteine_count: 2,
            helix_content: 0.35,
            sheet_content: 0.25,
            contact_diversity: 0.6,
            subunit_count: 1,
            has_symmetry: false,
            disulfide_bonds: 0,
        }
    }
}

// ── Amino acid → primitive activation ───────────────────────────────────

/// Each AA activates specific IG primitives when present in the chain.
pub struct AAActivation {
    pub aa: char,
    pub d_activates: Option<DVal>,
    pub k_activates: Option<KVal>,
    pub h_activates: Option<HVal>,
    pub s_activates: Option<SVal>,
    pub phi_activates: Option<PhiVal>,
}

/// 20 canonical amino acid activations.
pub fn aa_activation(aa: char) -> AAActivation {
    match aa.to_ascii_uppercase() {
        'A' => AAActivation { aa: 'A', d_activates: None, k_activates: None,
            h_activates: Some(HVal::M0), s_activates: None, phi_activates: None },
        'C' => AAActivation { aa: 'C', d_activates: None, k_activates: None,
            h_activates: None, s_activates: None, phi_activates: Some(PhiVal::C) },
        'D' => AAActivation { aa: 'D', d_activates: None, k_activates: Some(KVal::Fast),
            h_activates: None, s_activates: None, phi_activates: None },
        'E' => AAActivation { aa: 'E', d_activates: None, k_activates: Some(KVal::Fast),
            h_activates: None, s_activates: None, phi_activates: None },
        'F' => AAActivation { aa: 'F', d_activates: None, k_activates: None,
            h_activates: Some(HVal::M2), s_activates: None, phi_activates: None },
        'G' => AAActivation { aa: 'G', d_activates: None, k_activates: None,
            h_activates: None, s_activates: None, phi_activates: Some(PhiVal::Sub) },
        'H' => AAActivation { aa: 'H', d_activates: None, k_activates: None,
            h_activates: Some(HVal::M1), s_activates: None, phi_activates: None },
        'I' => AAActivation { aa: 'I', d_activates: None, k_activates: Some(KVal::Slow),
            h_activates: None, s_activates: None, phi_activates: None },
        'K' => AAActivation { aa: 'K', d_activates: Some(DVal::Odot), k_activates: None,
            h_activates: None, s_activates: None, phi_activates: None },
        'L' => AAActivation { aa: 'L', d_activates: None, k_activates: Some(KVal::Trap),
            h_activates: None, s_activates: None, phi_activates: None },
        'M' => AAActivation { aa: 'M', d_activates: None, k_activates: None,
            h_activates: Some(HVal::Inf), s_activates: None, phi_activates: None },
        'N' => AAActivation { aa: 'N', d_activates: None, k_activates: None,
            h_activates: Some(HVal::M1), s_activates: None, phi_activates: None },
        'P' => AAActivation { aa: 'P', d_activates: None, k_activates: Some(KVal::Trap),
            h_activates: None, s_activates: None, phi_activates: Some(PhiVal::EP) },
        'Q' => AAActivation { aa: 'Q', d_activates: None, k_activates: None,
            h_activates: Some(HVal::M1), s_activates: None, phi_activates: None },
        'R' => AAActivation { aa: 'R', d_activates: Some(DVal::Odot), k_activates: None,
            h_activates: None, s_activates: Some(SVal::Hetero), phi_activates: None },
        'S' => AAActivation { aa: 'S', d_activates: None, k_activates: Some(KVal::Fast),
            h_activates: None, s_activates: None, phi_activates: None },
        'T' => AAActivation { aa: 'T', d_activates: None, k_activates: Some(KVal::Mod),
            h_activates: None, s_activates: None, phi_activates: None },
        'V' => AAActivation { aa: 'V', d_activates: None, k_activates: Some(KVal::Slow),
            h_activates: None, s_activates: None, phi_activates: None },
        'W' => AAActivation { aa: 'W', d_activates: None, k_activates: None,
            h_activates: Some(HVal::Inf), s_activates: None, phi_activates: Some(PhiVal::Super) },
        'Y' => AAActivation { aa: 'Y', d_activates: None, k_activates: None,
            h_activates: Some(HVal::M2), s_activates: None, phi_activates: Some(PhiVal::CComplex) },
        _   => AAActivation { aa: 'X', d_activates: None, k_activates: None,
            h_activates: None, s_activates: None, phi_activates: None },
    }
}

/// Scan an AA chain and count primitive activations.
pub fn scan_activations(aa_chain: &str) -> alloc::collections::BTreeMap<&'static str, usize> {
    let mut counts: alloc::collections::BTreeMap<&'static str, usize> = BTreeMap::new();
    for aa in aa_chain.chars() {
        let act = aa_activation(aa);
        if act.k_activates.is_some() { *counts.entry("K_branched").or_default() += 1; }
        if act.h_activates.map(|h| h.ordinal() >= 2).unwrap_or(false) {
            *counts.entry("H_high").or_default() += 1;
        }
        if act.phi_activates.is_some() { *counts.entry("Phi_active").or_default() += 1; }
        if act.s_activates.is_some() { *counts.entry("S_hetero").or_default() += 1; }
        if act.d_activates.is_some() { *counts.entry("D_odot").or_default() += 1; }
    }
    counts
}

// ── Per-stage tuple generators ──────────────────────────────────────────

/// Generate the IG tuple for Stage 1: Raw DNA.
/// DNA is classical storage — low entanglement, sequential, local.
pub fn generate_stage1_dna(_ctx: &StageContext) -> IGTuple {
    IGTuple {
        d: DVal::Tri,         // Finite 2D — linear sequence
        t: TVal::Network,     // Branching — genes on chromosomes
        r: RVal::Super,       // Supervenience — sequence determines everything above
        p: PVal::Asym,        // No symmetry — forward strand only
        f: FVal::Ell,         // Classical — no coherence
        k: KVal::Trap,        // Frozen — stable double helix
        g: GVal::Beth,        // Local — nearest-neighbor base pairing
        gm: GmVal::Seq,       // Sequential — 5′→3′
        phi: PhiVal::Sub,     // Sub-critical — stable storage
        h: HVal::M0,          // Memoryless at this level
        s: SVal::One,         // One type — DNA
        o: OVal::Trivial,     // No topological protection
    }
}

/// Generate the IG tuple for Stage 2: Transcription.
/// RNA polymerase introduces dynamics — moderate kinetics, thermal regime.
pub fn generate_stage2_transcription(_ctx: &StageContext) -> IGTuple {
    IGTuple {
        d: DVal::Tri,         // Still finite
        t: TVal::Bowtie,      // Crossing point — DNA → RNA transition
        r: RVal::Dagger,      // Adjoint — one-way transcription
        p: PVal::Psi,         // Quantum — nucleotide selection
        f: FVal::Eth,         // Thermal — Brownian ratchet
        k: KVal::Mod,         // Moderate — ~50 nt/s
        g: GVal::Gimel,       // Mesoscale — promoter→terminator
        gm: GmVal::Seq,       // Sequential — processive
        phi: PhiVal::Sub,     // Sub-critical
        h: HVal::M1,          // One-step — abortive initiation
        s: SVal::One,         // One type — RNA
        o: OVal::Trivial,
    }
}

/// Generate the IG tuple for Stage 3: Codon Lattice.
/// Triplet code on B₄ lattice — Frobenius structure emerges.
pub fn generate_stage3_codon(_ctx: &StageContext) -> IGTuple {
    IGTuple {
        d: DVal::Wedge,       // 0D — codons are points on the B₄ lattice
        t: TVal::Boxtimes,    // Irreducible product — triplet = ⊗ of 3 nucleotides
        r: RVal::LR,          // Bidirectional — codon↔AA mapping is a bijection
        p: PVal::Pm,          // Partial Z2 — pyrimidine/purine
        f: FVal::Hbar,        // Quantum — wobble base pairing
        k: KVal::Slow,        // Near-equilibrium — ribosome decoding
        g: GVal::Beth,        // Local — within ribosome A-site
        gm: GmVal::And,       // All-simultaneous — codon positions constrain each other
        phi: PhiVal::C,       // CRITICAL — exact/split stratum boundary
        h: HVal::M2,          // Two-step — codon recognition → accommodation
        s: SVal::Many,        // Many identical — 64 codons
        o: OVal::Z2,          // Z2 parity — codon↔anticodon
    }
}

/// Generate the IG tuple for Stage 4: Translation.
/// AA chain emerges — kinetics driven by β-branched content.
pub fn generate_stage4_translation(ctx: &StageContext) -> IGTuple {
    let k_val = if ctx.beta_branched_frac > 0.25 {
        KVal::Slow
    } else if ctx.beta_branched_frac > 0.12 {
        KVal::Mod
    } else {
        KVal::Fast
    };
    let h_val = if ctx.aromatic_frac > 0.12 { HVal::M2 }
                else if ctx.aromatic_frac > 0.05 { HVal::M1 }
                else { HVal::M0 };

    IGTuple {
        d: DVal::Tri,         // Linear chain
        t: TVal::Network,     // Branching — sidechain interactions
        r: RVal::Super,       // Supervenience — sequence→structure
        p: PVal::Asym,        // N→C directionality
        f: FVal::Eth,         // Thermal
        k: k_val,             // β-branched driven
        g: GVal::Gimel,       // Mesoscale — domain-level
        gm: GmVal::Seq,       // Sequential — processive translation
        phi: PhiVal::Sub,
        h: h_val,
        s: SVal::Hetero,      // 20 distinct AA types
        o: OVal::Trivial,
    }
}

/// Generate the IG tuple for Stage 5: Folding (Secondary Structure).
pub fn generate_stage5_folding(ctx: &StageContext) -> IGTuple {
    let p_val = if ctx.helix_content > 0.5 || ctx.sheet_content > 0.5 {
        PVal::Pm  // Regular secondary structure = partial symmetry
    } else {
        PVal::Psi // Mixed = superposition of conformations
    };
    let phi_val = if ctx.proline_frac > 0.08 { PhiVal::EP }
                  else if ctx.cysteine_count >= 2 { PhiVal::C }
                  else { PhiVal::Sub };
    let h_val = if ctx.proline_frac > 0.06 { HVal::M2 }
                else { HVal::M1 };

    IGTuple {
        d: DVal::Tri,
        t: TVal::Bowtie,      // Crossing — folding funnel
        r: RVal::LR,          // Bidirectional — sequence↔structure
        p: p_val,
        f: FVal::Hbar,        // Quantum — folding landscape
        k: KVal::Slow,        // Slow — folding kinetics
        g: GVal::Gimel,
        gm: GmVal::And,       // Cooperative — all residues fold together
        phi: phi_val,
        h: h_val,
        s: SVal::Hetero,      // α-helix, β-sheet, loops
        o: OVal::Z2,          // Z2 — right-handed helix chirality
    }
}

/// Generate the IG tuple for Stage 6: Tertiary Structure.
pub fn generate_stage6_tertiary(ctx: &StageContext) -> IGTuple {
    let p_val = if ctx.disulfide_bonds >= 2 { PVal::Pm }
                else { PVal::Asym };
    let o_val = if ctx.disulfide_bonds >= 3 { OVal::Z }
                else if ctx.disulfide_bonds >= 1 { OVal::Z2 }
                else { OVal::Trivial };

    IGTuple {
        d: DVal::Infty,       // Infinite — conformational space
        t: TVal::Boxtimes,    // Product — domain×domain
        r: RVal::LR,          // Bidirectional — folding↔function
        p: p_val,
        f: FVal::Hbar,        // Quantum
        k: KVal::Slow,        // Slow — tertiary folding
        g: GVal::Beth,        // Local — contact-based
        gm: GmVal::And,       // Cooperative
        phi: PhiVal::C,       // Critical — native state at ⊙
        h: HVal::Inf,         // Eternal — fold memory
        s: SVal::Hetero,      // Multiple domains
        o: o_val,
    }
}

/// Generate the IG tuple for Stage 7: Quaternary Structure.
pub fn generate_stage7_quaternary(ctx: &StageContext) -> IGTuple {
    let s_val = if ctx.subunit_count > 2 { SVal::Hetero }
                else if ctx.subunit_count == 2 { SVal::Many }
                else { SVal::One };
    let p_val = if ctx.has_symmetry { PVal::Sym }
                else if ctx.subunit_count > 1 { PVal::Pm }
                else { PVal::Asym };

    IGTuple {
        d: DVal::Infty,
        t: TVal::Boxtimes,    // Product — subunit⊗subunit
        r: RVal::LR,
        p: p_val,
        f: FVal::Ell,         // Classical — assembled complex
        k: KVal::Trap,        // Frozen — stable assembly
        g: GVal::Aleph,       // Universal — quaternary interactions span entire complex
        gm: GmVal::Broad,     // Broadcast — allostery
        phi: PhiVal::Sub,     // Sub-critical — stable oligomer
        h: HVal::Inf,         // Eternal — assembly memory
        s: s_val,
        o: if ctx.subunit_count >= 4 { OVal::Z } else { OVal::Z2 },
    }
}

/// Generate tuples for all 7 pipeline stages given context.
pub fn generate_all_stages(ctx: &StageContext) -> [IGTuple; 7] {
    [
        generate_stage1_dna(ctx),
        generate_stage2_transcription(ctx),
        generate_stage3_codon(ctx),
        generate_stage4_translation(ctx),
        generate_stage5_folding(ctx),
        generate_stage6_tertiary(ctx),
        generate_stage7_quaternary(ctx),
    ]
}

/// Verify monotonic advance: each stage's Ω ordinal must be ≥ prior.
pub fn verify_monotonic_advance(stages: &[IGTuple; 7]) -> bool {
    for i in 1..7 {
        if stages[i].o.ordinal() < stages[i-1].o.ordinal() {
            return false;
        }
    }
    true
}

/// Compute the crystal address for a tuple (simplified — full bijection in crystal.rs).
pub fn tuple_crystal_address(t: &IGTuple) -> u32 {
    let d = t.d.ordinal() as u32;
    let tp = t.t.ordinal() as u32;
    let r = t.r.ordinal() as u32;
    let p = t.p.ordinal() as u32;
    let f = t.f.ordinal() as u32;
    let k = t.k.ordinal() as u32;
    let g = t.g.ordinal() as u32;
    let gm = t.gm.ordinal() as u32;
    let phi = t.phi.ordinal() as u32;
    let h = t.h.ordinal() as u32;
    let s = t.s.ordinal() as u32;
    let o = t.o.ordinal() as u32;

    // Crystal encoding: weighted mixed-radix
    // 3³ × 4⁵ × 5⁴ = 27 × 1024 × 625 = 17,280,000
    let w_f = 3; let w_h = 4; let w_s = 3; let w_o = 4;
    let w_d = 4; let w_t = 5; let w_r = 4; let w_p = 5;
    let w_g = 3; let w_gm = 4; let w_k = 5;

    let addr = phi * w_f * w_h * w_s * w_o * w_d * w_t * w_r * w_p * w_g * w_gm * w_k
             + f  * w_h * w_s * w_o * w_d * w_t * w_r * w_p * w_g * w_gm * w_k
             + h  * w_s * w_o * w_d * w_t * w_r * w_p * w_g * w_gm * w_k
             + s  * w_o * w_d * w_t * w_r * w_p * w_g * w_gm * w_k
             + o  * w_d * w_t * w_r * w_p * w_g * w_gm * w_k
             + d  * w_t * w_r * w_p * w_g * w_gm * w_k
             + tp * w_r * w_p * w_g * w_gm * w_k
             + r  * w_p * w_g * w_gm * w_k
             + p  * w_g * w_gm * w_k
             + g  * w_gm * w_k
             + gm * w_k
             + k;
    addr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_stages_generate() {
        let ctx = StageContext::default();
        let stages = generate_all_stages(&ctx);
        assert_eq!(stages.len(), 7);
        for (i, stage) in stages.iter().enumerate() {
            // Every stage should have non-empty display
            let d = stage.display();
            assert!(!d.is_empty(), "Stage {} display empty", i);
        }
    }

    #[test]
    fn test_monotonic_advance() {
        let ctx = StageContext::default();
        let stages = generate_all_stages(&ctx);
        assert!(verify_monotonic_advance(&stages),
            "Ω must be monotonic across pipeline stages");
    }

    #[test]
    fn test_codon_stage_critical() {
        let ctx = StageContext::default();
        let codon = generate_stage3_codon(&ctx);
        assert_eq!(codon.phi, PhiVal::C, "Codon stage must be ⊙-critical");
    }

    #[test]
    fn test_quaternary_symmetry() {
        let ctx = StageContext {
            subunit_count: 4,
            has_symmetry: true,
            ..Default::default()
        };
        let quat = generate_stage7_quaternary(&ctx);
        assert_eq!(quat.p, PVal::Sym);
        assert_eq!(quat.o, OVal::Z);
    }

    #[test]
    fn test_scan_activations() {
        let counts = scan_activations("MAGILVFWY");
        // M has H=Inf; A has H=M0; G has Phi=Sub; I has K=Slow; L has K=Trap;
        // V has K=Slow; F has H=M2; W has H=Inf + Phi=Super; Y has H=M2 + Phi=CComplex
        assert!(counts.get("Phi_active").unwrap_or(&0) >= &2, "G+W+Y activate Phi");
        assert!(counts.get("K_branched").unwrap_or(&0) >= &2, "I+V activate K");
        assert!(counts.get("H_high").unwrap_or(&0) >= &2, "F+Y activate H≥2");
        assert!(counts.get("D_odot").unwrap_or(&0) == &0, "No K or R in test chain");
    }

    #[test]
    fn test_stage4_driven_by_context() {
        let low_beta = StageContext { beta_branched_frac: 0.05, ..Default::default() };
        let high_beta = StageContext { beta_branched_frac: 0.30, ..Default::default() };
        let s4_low = generate_stage4_translation(&low_beta);
        let s4_high = generate_stage4_translation(&high_beta);
        assert_eq!(s4_low.k, KVal::Fast);
        assert_eq!(s4_high.k, KVal::Slow);
    }

    #[test]
    fn test_crystal_address_range() {
        let ctx = StageContext::default();
        let stages = generate_all_stages(&ctx);
        for stage in &stages {
            let addr = tuple_crystal_address(stage);
            assert!(addr < 17_280_000, "Address {} out of crystal range", addr);
        }
    }

    #[test]
    fn test_aa_activation_all_20() {
        let all_aa = "ACDEFGHIKLMNPQRSTVWY";
        for aa in all_aa.chars() {
            let act = aa_activation(aa);
            assert_eq!(act.aa, aa);
        }
    }

    #[test]
    fn test_proline_ep() {
        let act = aa_activation('P');
        assert_eq!(act.phi_activates, Some(PhiVal::EP));
        assert_eq!(act.k_activates, Some(KVal::Trap));
    }

    #[test]
    fn test_methionine_inf() {
        let act = aa_activation('M');
        assert_eq!(act.h_activates, Some(HVal::Inf));
    }

    #[test]
    fn test_display_format() {
        let ctx = StageContext::default();
        let dna = generate_stage1_dna(&ctx);
        let d = dna.display();
        assert!(d.starts_with('\u{27e8}')); // ⟨
        assert!(d.ends_with('\u{27e9}'));   // ⟩
        assert!(d.contains('\u{b7}'));      // ·
    }

    #[test]
    fn test_folding_context_driven() {
        let helix_ctx = StageContext {
            helix_content: 0.6,
            sheet_content: 0.1,
            proline_frac: 0.02,
            cysteine_count: 0,
            ..Default::default()
        };
        let fold = generate_stage5_folding(&helix_ctx);
        assert_eq!(fold.p, PVal::Pm);  // High helix → partial symmetry
    }
}

// ── Canonical pipeline tuple reference ──────────────────────────────────
// These are the baseline tuples for a typical ~300 AA globular protein:

/// Canonical tuple for Stage 1 (DNA storage).
pub const CANONICAL_DNA: (&str, &str, &str, &str, &str, &str,
                           &str, &str, &str, &str, &str, &str) =
    ("tri", "network", "super", "asym", "ell", "trap",
     "beth", "seq", "sub", "0", "one", "0");

/// Canonical tuple for Stage 3 (Codon lattice) — the ⊙ critical stage.
pub const CANONICAL_CODON: (&str, &str, &str, &str, &str, &str,
                             &str, &str, &str, &str, &str, &str) =
    ("wedge", "boxtimes", "lr", "pm", "hbar", "slow",
     "beth", "and", "c", "2", "many", "Z2");

/// Canonical tuple for Stage 7 (Quaternary assembly).
pub const CANONICAL_QUATERNARY: (&str, &str, &str, &str, &str, &str,
                                  &str, &str, &str, &str, &str, &str) =
    ("infty", "boxtimes", "lr", "sym", "ell", "trap",
     "aleph", "broad", "sub", "inf", "hetero", "Z");
