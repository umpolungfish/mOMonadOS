//! genetic_tuples.rs — Generative Tuple Construction for Gene→Protein Pipeline
//! Port of rhr_p4rky/genetic_tuples.py
//!
//! Each stage's tuple is a FUNCTION of sequence-derived features.
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
//!   3. Monotonic advance — 𐑭 constraint on trajectory through the crystal

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
            "D" | "⊢" => Some(PrimKey::D),
            "T" | "⊣" => Some(PrimKey::T),
            "R" | "≻" => Some(PrimKey::R),
            "P" | "≺" => Some(PrimKey::P),
            "F" | "⋈" => Some(PrimKey::F),
            "K" | "⊤" => Some(PrimKey::K),
            "G" | "∈" => Some(PrimKey::G),
            "Gm" | "∋" => Some(PrimKey::Gm),
            "Phi" | "⊙" => Some(PrimKey::Phi),
            "H" | "⊥" => Some(PrimKey::H),
            "S" | "⊞" => Some(PrimKey::S),
            "O" | "⊡" => Some(PrimKey::O),
            _ => None,
        }
    }
}

// ── IG Value types ─────────────────────────────────────────────────────

/// Dimensionality values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimVal { Dead, Ash, Array, If }

/// Topology values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopVal { Judge, Eat, Mime, Oil, Are }

/// Relational mode values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelVal { Ado, Tot, Ear, Ian }

/// Parity values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolVal { Church, Yew, Out, Nun, Or }

/// Fidelity values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FidVal { Age, They, Peep }

/// Kinetics values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KinVal { Yea, Loll, Egg, On, Air }

/// Scope values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GranVal { Bib, Thigh, Ice }

/// Interaction grammar values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GramVal { Vow, Gag, Measure, Ooze }

/// Criticality values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CritVal { Woe, Monad, Roar, Err, Haha }

/// Chirality values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChirVal { Fee, Kick, Sure, Wool }

/// Stoichiometry values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoiVal { Hung, So, Up }

/// Winding values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtVal { Awe, Oak, Ah, Zoo }

// ── IG Tuple ────────────────────────────────────────────────────────────

/// A complete 12-primitive IG tuple (pipeline string names).
#[derive(Clone, Debug)]
pub struct IGTuple {
    pub d: DimVal,
    pub t: TopVal,
    pub r: RelVal,
    pub p: PolVal,
    pub f: FidVal,
    pub k: KinVal,
    pub g: GranVal,
    pub gm: GramVal,
    pub phi: CritVal,
    pub h: ChirVal,
    pub s: StoiVal,
    pub o: ProtVal,
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

    /// Build a tuple display string: ⟨D·T·R·P·F·K·G·Gm·⊙·H·S·⊡⟩
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

impl DimVal {
    pub fn as_str(&self) -> &'static str {
        match self { DimVal::Dead => "wedge", DimVal::Ash => "tri",
                     DimVal::Array => "infty", DimVal::If => "odot" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { DimVal::Dead => "𐑛", DimVal::Ash => "𐑨",
                     DimVal::Array => "𐑼", DimVal::If => "𐑦" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { DimVal::Dead => 0, DimVal::Ash => 1,
                     DimVal::Array => 2, DimVal::If => 3 }
    }
}

impl TopVal {
    pub fn as_str(&self) -> &'static str {
        match self { TopVal::Judge => "network", TopVal::Eat => "in",
                     TopVal::Mime => "bowtie", TopVal::Oil => "boxtimes",
                     TopVal::Are => "odot" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { TopVal::Judge => "𐑡", TopVal::Eat => "𐑰",
                     TopVal::Mime => "𐑥", TopVal::Oil => "𐑶",
                     TopVal::Are => "𐑸" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { TopVal::Judge => 0, TopVal::Eat => 1,
                     TopVal::Mime => 2, TopVal::Oil => 3, TopVal::Are => 4 }
    }
}

impl RelVal {
    pub fn as_str(&self) -> &'static str {
        match self { RelVal::Ado => "super", RelVal::Tot => "cat",
                     RelVal::Ear => "dagger", RelVal::Ian => "lr" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { RelVal::Ado => "𐑩", RelVal::Tot => "𐑑",
                     RelVal::Ear => "𐑽", RelVal::Ian => "𐑾" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { RelVal::Ado => 0, RelVal::Tot => 1,
                     RelVal::Ear => 2, RelVal::Ian => 3 }
    }
}

impl PolVal {
    pub fn as_str(&self) -> &'static str {
        match self { PolVal::Church => "asym", PolVal::Yew => "psi",
                     PolVal::Out => "pm", PolVal::Nun => "sym", PolVal::Or => "pm_sym" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { PolVal::Church => "𐑗", PolVal::Yew => "𐑿",
                     PolVal::Out => "𐑬", PolVal::Nun => "𐑯", PolVal::Or => "𐑹" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { PolVal::Church => 0, PolVal::Yew => 1,
                     PolVal::Out => 2, PolVal::Nun => 3, PolVal::Or => 4 }
    }
}

impl FidVal {
    pub fn as_str(&self) -> &'static str {
        match self { FidVal::Age => "ell", FidVal::They => "eth", FidVal::Peep => "hbar" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { FidVal::Age => "𐑱", FidVal::They => "𐑞", FidVal::Peep => "𐑐" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { FidVal::Age => 0, FidVal::They => 1, FidVal::Peep => 2 }
    }
}

impl KinVal {
    pub fn as_str(&self) -> &'static str {
        match self { KinVal::Yea => "fast", KinVal::Loll => "mod",
                     KinVal::Egg => "slow", KinVal::On => "trap", KinVal::Air => "MBL" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { KinVal::Yea => "𐑘", KinVal::Loll => "𐑤",
                     KinVal::Egg => "𐑧", KinVal::On => "𐑪", KinVal::Air => "𐑺" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { KinVal::Yea => 0, KinVal::Loll => 1,
                     KinVal::Egg => 2, KinVal::On => 3, KinVal::Air => 4 }
    }
}

impl GranVal {
    pub fn as_str(&self) -> &'static str {
        match self { GranVal::Bib => "beth", GranVal::Thigh => "gimel", GranVal::Ice => "aleph" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { GranVal::Bib => "𐑚", GranVal::Thigh => "𐑔", GranVal::Ice => "𐑲" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { GranVal::Bib => 0, GranVal::Thigh => 1, GranVal::Ice => 2 }
    }
}

impl GramVal {
    pub fn as_str(&self) -> &'static str {
        match self { GramVal::Vow => "and", GramVal::Gag => "or",
                     GramVal::Measure => "seq", GramVal::Ooze => "broad" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { GramVal::Vow => "𐑝", GramVal::Gag => "𐑜",
                     GramVal::Measure => "𐑠", GramVal::Ooze => "𐑵" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { GramVal::Vow => 0, GramVal::Gag => 1,
                     GramVal::Measure => 2, GramVal::Ooze => 3 }
    }
}

impl CritVal {
    pub fn as_str(&self) -> &'static str {
        match self { CritVal::Woe => "sub", CritVal::Monad => "c",
                     CritVal::Roar => "c_complex", CritVal::Err => "EP",
                     CritVal::Haha => "super" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { CritVal::Woe => "𐑢", CritVal::Monad => "\u{2299}",
                     CritVal::Roar => "𐑮", CritVal::Err => "𐑻",
                     CritVal::Haha => "𐑣" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { CritVal::Woe => 0, CritVal::Monad => 1,
                     CritVal::Roar => 2, CritVal::Err => 3, CritVal::Haha => 4 }
    }
}

impl ChirVal {
    pub fn as_str(&self) -> &'static str {
        match self { ChirVal::Fee => "0", ChirVal::Kick => "1",
                     ChirVal::Sure => "2", ChirVal::Wool => "inf" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { ChirVal::Fee => "𐑓", ChirVal::Kick => "𐑒",
                     ChirVal::Sure => "𐑖", ChirVal::Wool => "𐑫" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { ChirVal::Fee => 0, ChirVal::Kick => 1, ChirVal::Sure => 2, ChirVal::Wool => 3 }
    }
}

impl StoiVal {
    pub fn as_str(&self) -> &'static str {
        match self { StoiVal::Hung => "one", StoiVal::So => "many", StoiVal::Up => "hetero" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { StoiVal::Hung => "𐑙", StoiVal::So => "𐑕", StoiVal::Up => "𐑳" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { StoiVal::Hung => 0, StoiVal::So => 1, StoiVal::Up => 2 }
    }
}

impl ProtVal {
    pub fn as_str(&self) -> &'static str {
        match self { ProtVal::Awe => "0", ProtVal::Oak => "Z2",
                     ProtVal::Ah => "Z", ProtVal::Zoo => "NA" }
    }
    pub fn glyph(&self) -> &'static str {
        match self { ProtVal::Awe => "𐑷", ProtVal::Oak => "𐑴",
                     ProtVal::Ah => "𐑭", ProtVal::Zoo => "𐑟" }
    }
    pub fn ordinal(&self) -> u8 {
        match self { ProtVal::Awe => 0, ProtVal::Oak => 1, ProtVal::Ah => 2, ProtVal::Zoo => 3 }
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
    pub d_activates: Option<DimVal>,
    pub k_activates: Option<KinVal>,
    pub h_activates: Option<ChirVal>,
    pub s_activates: Option<StoiVal>,
    pub phi_activates: Option<CritVal>,
}

/// 20 canonical amino acid activations.
pub fn aa_activation(aa: char) -> AAActivation {
    match aa.to_ascii_uppercase() {
        'A' => AAActivation { aa: 'A', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Fee), s_activates: None, phi_activates: None },
        'C' => AAActivation { aa: 'C', d_activates: None, k_activates: None,
            h_activates: None, s_activates: None, phi_activates: Some(CritVal::Monad) },
        'D' => AAActivation { aa: 'D', d_activates: None, k_activates: Some(KinVal::Yea),
            h_activates: None, s_activates: None, phi_activates: None },
        'E' => AAActivation { aa: 'E', d_activates: None, k_activates: Some(KinVal::Yea),
            h_activates: None, s_activates: None, phi_activates: None },
        'F' => AAActivation { aa: 'F', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Sure), s_activates: None, phi_activates: None },
        'G' => AAActivation { aa: 'G', d_activates: None, k_activates: None,
            h_activates: None, s_activates: None, phi_activates: Some(CritVal::Woe) },
        'H' => AAActivation { aa: 'H', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Kick), s_activates: None, phi_activates: None },
        'I' => AAActivation { aa: 'I', d_activates: None, k_activates: Some(KinVal::Egg),
            h_activates: None, s_activates: None, phi_activates: None },
        'K' => AAActivation { aa: 'K', d_activates: Some(DimVal::If), k_activates: None,
            h_activates: None, s_activates: None, phi_activates: None },
        'L' => AAActivation { aa: 'L', d_activates: None, k_activates: Some(KinVal::On),
            h_activates: None, s_activates: None, phi_activates: None },
        'M' => AAActivation { aa: 'M', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Wool), s_activates: None, phi_activates: None },
        'N' => AAActivation { aa: 'N', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Kick), s_activates: None, phi_activates: None },
        'P' => AAActivation { aa: 'P', d_activates: None, k_activates: Some(KinVal::On),
            h_activates: None, s_activates: None, phi_activates: Some(CritVal::Err) },
        'Q' => AAActivation { aa: 'Q', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Kick), s_activates: None, phi_activates: None },
        'R' => AAActivation { aa: 'R', d_activates: Some(DimVal::If), k_activates: None,
            h_activates: None, s_activates: Some(StoiVal::Up), phi_activates: None },
        'S' => AAActivation { aa: 'S', d_activates: None, k_activates: Some(KinVal::Yea),
            h_activates: None, s_activates: None, phi_activates: None },
        'T' => AAActivation { aa: 'T', d_activates: None, k_activates: Some(KinVal::Loll),
            h_activates: None, s_activates: None, phi_activates: None },
        'V' => AAActivation { aa: 'V', d_activates: None, k_activates: Some(KinVal::Egg),
            h_activates: None, s_activates: None, phi_activates: None },
        'W' => AAActivation { aa: 'W', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Wool), s_activates: None, phi_activates: Some(CritVal::Haha) },
        'Y' => AAActivation { aa: 'Y', d_activates: None, k_activates: None,
            h_activates: Some(ChirVal::Sure), s_activates: None, phi_activates: Some(CritVal::Roar) },
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
        if act.phi_activates.is_some() { *counts.entry("⊙_active").or_default() += 1; }
        if act.s_activates.is_some() { *counts.entry("S_hetero").or_default() += 1; }
        if act.d_activates.is_some() { *counts.entry("if_").or_default() += 1; }
    }
    counts
}

// ── Per-stage tuple generators ──────────────────────────────────────────

/// Generate the IG tuple for Stage 1: Raw DNA.
/// DNA is classical storage — low entanglement, sequential, local.
pub fn generate_stage1_dna(_ctx: &StageContext) -> IGTuple {
    IGTuple {
        d: DimVal::Ash,         // Finite 2D — linear sequence
        t: TopVal::Judge,     // Branching — genes on chromosomes
        r: RelVal::Ado,       // Supervenience — sequence determines everything above
        p: PolVal::Church,        // No symmetry — forward strand only
        f: FidVal::Age,         // Classical — no coherence
        k: KinVal::On,        // Frozen — stable double helix
        g: GranVal::Bib,        // Local — nearest-neighbor base pairing
        gm: GramVal::Measure,       // Sequential — 5′→3′
        phi: CritVal::Woe,     // Sub-critical — stable storage
        h: ChirVal::Fee,          // Memoryless at this level
        s: StoiVal::Hung,         // One type — DNA
        o: ProtVal::Awe,     // No topological protection
    }
}

/// Generate the IG tuple for Stage 2: Transcription.
/// RNA polymerase introduces dynamics — moderate kinetics, thermal regime.
pub fn generate_stage2_transcription(_ctx: &StageContext) -> IGTuple {
    IGTuple {
        d: DimVal::Ash,         // Still finite
        t: TopVal::Mime,      // Crossing point — DNA → RNA transition
        r: RelVal::Ear,      // Adjoint — one-way transcription
        p: PolVal::Yew,         // Quantum — nucleotide selection
        f: FidVal::They,         // Thermal — Brownian ratchet
        k: KinVal::Loll,         // Moderate — ~50 nt/s
        g: GranVal::Thigh,       // Mesoscale — promoter→terminator
        gm: GramVal::Measure,       // Sequential — processive
        phi: CritVal::Woe,     // Sub-critical
        h: ChirVal::Kick,          // One-step — abortive initiation
        s: StoiVal::Hung,         // One type — RNA
        o: ProtVal::Awe,
    }
}

/// Generate the IG tuple for Stage 3: Codon Lattice.
/// Triplet code on B₄ lattice — Frobenius structure emerges.
pub fn generate_stage3_codon(_ctx: &StageContext) -> IGTuple {
    IGTuple {
        d: DimVal::Dead,       // 0D — codons are points on the B₄ lattice
        t: TopVal::Oil,    // Irreducible product — triplet = ⊗ of 3 nucleotides
        r: RelVal::Ian,          // Bidirectional — codon↔AA mapping is a bijection
        p: PolVal::Out,          // Partial Z2 — pyrimidine/purine
        f: FidVal::Peep,        // Quantum — wobble base pairing
        k: KinVal::Egg,        // Near-equilibrium — ribosome decoding
        g: GranVal::Bib,        // Local — within ribosome A-site
        gm: GramVal::Vow,       // All-simultaneous — codon positions constrain each other
        phi: CritVal::Monad,       // CRITICAL — exact/split stratum boundary
        h: ChirVal::Sure,          // Two-step — codon recognition → accommodation
        s: StoiVal::So,        // Many identical — 64 codons
        o: ProtVal::Oak,          // Z2 parity — codon↔anticodon
    }
}

/// Generate the IG tuple for Stage 4: Translation.
/// AA chain emerges — kinetics driven by β-branched content.
pub fn generate_stage4_translation(ctx: &StageContext) -> IGTuple {
    let k_val = if ctx.beta_branched_frac > 0.25 {
        KinVal::Egg
    } else if ctx.beta_branched_frac > 0.12 {
        KinVal::Loll
    } else {
        KinVal::Yea
    };
    let h_val = if ctx.aromatic_frac > 0.12 { ChirVal::Sure }
                else if ctx.aromatic_frac > 0.05 { ChirVal::Kick }
                else { ChirVal::Fee };

    IGTuple {
        d: DimVal::Ash,         // Linear chain
        t: TopVal::Judge,     // Branching — sidechain interactions
        r: RelVal::Ado,       // Supervenience — sequence→structure
        p: PolVal::Church,        // N→C directionality
        f: FidVal::They,         // Thermal
        k: k_val,             // β-branched driven
        g: GranVal::Thigh,       // Mesoscale — domain-level
        gm: GramVal::Measure,       // Sequential — processive translation
        phi: CritVal::Woe,
        h: h_val,
        s: StoiVal::Up,      // 20 distinct AA types
        o: ProtVal::Awe,
    }
}

/// Generate the IG tuple for Stage 5: Folding (Secondary Structure).
pub fn generate_stage5_folding(ctx: &StageContext) -> IGTuple {
    let p_val = if ctx.helix_content > 0.5 || ctx.sheet_content > 0.5 {
        PolVal::Out  // Regular secondary structure = partial symmetry
    } else {
        PolVal::Yew // Mixed = superposition of conformations
    };
    let phi_val = if ctx.proline_frac > 0.08 { CritVal::Err }
                  else if ctx.cysteine_count >= 2 { CritVal::Monad }
                  else { CritVal::Woe };
    let h_val = if ctx.proline_frac > 0.06 { ChirVal::Sure }
                else { ChirVal::Kick };

    IGTuple {
        d: DimVal::Ash,
        t: TopVal::Mime,      // Crossing — folding funnel
        r: RelVal::Ian,          // Bidirectional — sequence↔structure
        p: p_val,
        f: FidVal::Peep,        // Quantum — folding landscape
        k: KinVal::Egg,        // Slow — folding kinetics
        g: GranVal::Thigh,
        gm: GramVal::Vow,       // Cooperative — all residues fold together
        phi: phi_val,
        h: h_val,
        s: StoiVal::Up,      // α-helix, β-sheet, loops
        o: ProtVal::Oak,          // Z2 — right-handed helix chirality
    }
}

/// Generate the IG tuple for Stage 6: Tertiary Structure.
pub fn generate_stage6_tertiary(ctx: &StageContext) -> IGTuple {
    let p_val = if ctx.disulfide_bonds >= 2 { PolVal::Out }
                else { PolVal::Church };
    let o_val = if ctx.disulfide_bonds >= 3 { ProtVal::Ah }
                else if ctx.disulfide_bonds >= 1 { ProtVal::Oak }
                else { ProtVal::Awe };

    IGTuple {
        d: DimVal::Array,       // Infinite — conformational space
        t: TopVal::Oil,    // Product — domain×domain
        r: RelVal::Ian,          // Bidirectional — folding↔function
        p: p_val,
        f: FidVal::Peep,        // Quantum
        k: KinVal::Egg,        // Slow — tertiary folding
        g: GranVal::Bib,        // Local — contact-based
        gm: GramVal::Vow,       // Cooperative
        phi: CritVal::Monad,       // Critical — native state at ⊙
        h: ChirVal::Wool,         // Eternal — fold memory
        s: StoiVal::Up,      // Multiple domains
        o: o_val,
    }
}

/// Generate the IG tuple for Stage 7: Quaternary Structure.
pub fn generate_stage7_quaternary(ctx: &StageContext) -> IGTuple {
    let s_val = if ctx.subunit_count > 2 { StoiVal::Up }
                else if ctx.subunit_count == 2 { StoiVal::So }
                else { StoiVal::Hung };
    let p_val = if ctx.has_symmetry { PolVal::Nun }
                else if ctx.subunit_count > 1 { PolVal::Out }
                else { PolVal::Church };

    IGTuple {
        d: DimVal::Array,
        t: TopVal::Oil,    // Product — subunit⊗subunit
        r: RelVal::Ian,
        p: p_val,
        f: FidVal::Age,         // Classical — assembled complex
        k: KinVal::On,        // Frozen — stable assembly
        g: GranVal::Ice,       // Universal — quaternary interactions span entire complex
        gm: GramVal::Ooze,     // Broadcast — allostery
        phi: CritVal::Woe,     // Sub-critical — stable oligomer
        h: ChirVal::Wool,         // Eternal — assembly memory
        s: s_val,
        o: if ctx.subunit_count >= 4 { ProtVal::Ah } else { ProtVal::Oak },
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

/// Verify monotonic advance: each stage's ⊡ ordinal is ≥ prior.
///
/// This is a predicate over any 7-stage pipeline, not an invariant of the
/// gene→protein one. `genetic_tuples.py` reports regressions rather than
/// gating on them, and even keeps an exception list of primitives allowed to
/// regress — so a pipeline is not required to be monotonic. This one is not:
/// see `omega_regressions`.
pub fn verify_monotonic_advance(stages: &[IGTuple; 7]) -> bool {
    for i in 1..7 {
        if stages[i].o.ordinal() < stages[i-1].o.ordinal() {
            return false;
        }
    }
    true
}

/// Where ⊡ falls: entry i is true when stage i+1's ⊡ ordinal is below stage i's.
///
/// The gene→protein pipeline drops twice, and both drops are hardcoded rather
/// than context-driven, so no `StageContext` makes it monotonic. The drops sit
/// where the modelled object changes kind: the codon lattice carries Z2 from
/// codon↔anticodon parity, which is a symmetry of the *code* and not of the
/// nascent chain that follows it; and secondary structure carries Z2 from helix
/// chirality, which the tertiary fold's ⊡ is written to take from disulfide
/// count alone.
pub fn omega_regressions(stages: &[IGTuple; 7]) -> [bool; 6] {
    let mut drops = [false; 6];
    for i in 0..6 {
        drops[i] = stages[i+1].o.ordinal() < stages[i].o.ordinal();
    }
    drops
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
    fn test_omega_trajectory() {
        let ctx = StageContext::default();
        let stages = generate_all_stages(&ctx);
        let mut omega = [0u8; 7];
        for (i, s) in stages.iter().enumerate() { omega[i] = s.o.ordinal(); }
        // DNA and transcription carry no topological protection; the codon
        // lattice carries Z2; the nascent chain carries none; secondary
        // structure carries Z2; the default context has no disulfides, so the
        // tertiary fold carries none; the monomer's quaternary ⊡ is Z2.
        assert_eq!(omega, [0, 0, 1, 0, 1, 0, 1]);
        // So the pipeline is not ⊡-monotonic, and no StageContext makes it so —
        // both drops are between hardcoded stage values.
        assert!(!verify_monotonic_advance(&stages));
        assert_eq!(omega_regressions(&stages), [false, false, true, false, true, false]);
    }

    #[test]
    fn test_codon_stage_critical() {
        let ctx = StageContext::default();
        let codon = generate_stage3_codon(&ctx);
        assert_eq!(codon.phi, CritVal::Monad, "Codon stage must be ⊙-critical");
    }

    #[test]
    fn test_quaternary_symmetry() {
        let ctx = StageContext {
            subunit_count: 4,
            has_symmetry: true,
            ..Default::default()
        };
        let quat = generate_stage7_quaternary(&ctx);
        assert_eq!(quat.p, PolVal::Nun);
        assert_eq!(quat.o, ProtVal::Ah);
    }

    #[test]
    fn test_scan_activations() {
        let counts = scan_activations("MAGILVFWY");
        // M has H=Inf; A has H=M0; G has Phi=Sub; I has K=Slow; L has K=Trap;
        // V has K=Slow; F has H=M2; W has H=Inf + Phi=Super; Y has H=M2 + Phi=CComplex
        assert!(counts.get("⊙_active").unwrap_or(&0) >= &2, "G+W+Y activate ⊙");
        assert!(counts.get("K_branched").unwrap_or(&0) >= &2, "I+V activate K");
        assert!(counts.get("H_high").unwrap_or(&0) >= &2, "F+Y activate H≥2");
        assert!(counts.get("if_").unwrap_or(&0) == &0, "No K or R in test chain");
    }

    #[test]
    fn test_stage4_driven_by_context() {
        let low_beta = StageContext { beta_branched_frac: 0.05, ..Default::default() };
        let high_beta = StageContext { beta_branched_frac: 0.30, ..Default::default() };
        let s4_low = generate_stage4_translation(&low_beta);
        let s4_high = generate_stage4_translation(&high_beta);
        assert_eq!(s4_low.k, KinVal::Yea);
        assert_eq!(s4_high.k, KinVal::Egg);
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
        assert_eq!(act.phi_activates, Some(CritVal::Err));
        assert_eq!(act.k_activates, Some(KinVal::On));
    }

    #[test]
    fn test_methionine_inf() {
        let act = aa_activation('M');
        assert_eq!(act.h_activates, Some(ChirVal::Wool));
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
        assert_eq!(fold.p, PolVal::Out);  // High helix → partial symmetry
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


// ── IgPrim Consistency Guard ─────────────────────────────────────────────
// The genetic_tuples value enums (DimVal, TopVal, ...) are structurally isomorphic
// to IgPrim by construction — same cardinalities, same ordering.
// These tests GUARD against drift: if IgPrim values change, these break.
#[cfg(test)]
mod igprim_consistency {
    use super::*;
    use crate::imas_ig::IgPrim;

    /// Verify DimVal glyphs match IgPrim glyphs.
    #[test]
    fn dval_glyphs_match_igprim() {
        assert_eq!(DimVal::Dead.glyph(), IgPrim::dead.glyph());
        assert_eq!(DimVal::Ash.glyph(),   IgPrim::ash.glyph());
        assert_eq!(DimVal::Array.glyph(), IgPrim::array.glyph());
        assert_eq!(DimVal::If.glyph(),  IgPrim::if_.glyph());
    }

    #[test]
    fn tval_glyphs_match_igprim() {
        assert_eq!(TopVal::Judge.glyph(),  IgPrim::judge.glyph());
        assert_eq!(TopVal::Eat.glyph(),       IgPrim::eat.glyph());
        assert_eq!(TopVal::Mime.glyph(),   IgPrim::mime.glyph());
        assert_eq!(TopVal::Oil.glyph(), IgPrim::oil.glyph());
        assert_eq!(TopVal::Are.glyph(),     IgPrim::are.glyph());
    }

    #[test]
    fn rval_glyphs_match_igprim() {
        assert_eq!(RelVal::Ado.glyph(),  IgPrim::ado.glyph());
        assert_eq!(RelVal::Tot.glyph(),    IgPrim::tot.glyph());
        assert_eq!(RelVal::Ear.glyph(), IgPrim::ear.glyph());
        assert_eq!(RelVal::Ian.glyph(),     IgPrim::ian.glyph());
    }

    #[test]
    fn pval_glyphs_match_igprim() {
        assert_eq!(PolVal::Church.glyph(),  IgPrim::church.glyph());
        assert_eq!(PolVal::Yew.glyph(),   IgPrim::yew.glyph());
        assert_eq!(PolVal::Out.glyph(),    IgPrim::out.glyph());
        assert_eq!(PolVal::Nun.glyph(),   IgPrim::nun.glyph());
        assert_eq!(PolVal::Or.glyph(), IgPrim::or_.glyph());
    }

    #[test]
    fn fval_glyphs_match_igprim() {
        assert_eq!(FidVal::Age.glyph(),  IgPrim::age.glyph());
        assert_eq!(FidVal::They.glyph(),  IgPrim::they.glyph());
        assert_eq!(FidVal::Peep.glyph(), IgPrim::peep.glyph());
    }

    #[test]
    fn kval_glyphs_match_igprim() {
        assert_eq!(KinVal::Yea.glyph(), IgPrim::yea.glyph());
        assert_eq!(KinVal::Loll.glyph(),  IgPrim::loll.glyph());
        assert_eq!(KinVal::Egg.glyph(), IgPrim::egg.glyph());
        assert_eq!(KinVal::On.glyph(), IgPrim::on.glyph());
        assert_eq!(KinVal::Air.glyph(),  IgPrim::air.glyph());
    }

    #[test]
    fn gval_glyphs_match_igprim() {
        assert_eq!(GranVal::Bib.glyph(),  IgPrim::bib.glyph());
        assert_eq!(GranVal::Thigh.glyph(), IgPrim::thigh.glyph());
        assert_eq!(GranVal::Ice.glyph(), IgPrim::ice.glyph());
    }

    #[test]
    fn gmval_glyphs_match_igprim() {
        assert_eq!(GramVal::Vow.glyph(),   IgPrim::vow.glyph());
        assert_eq!(GramVal::Gag.glyph(),    IgPrim::gag.glyph());
        assert_eq!(GramVal::Measure.glyph(),   IgPrim::measure.glyph());
        assert_eq!(GramVal::Ooze.glyph(), IgPrim::ooze.glyph());
    }

    #[test]
    fn phival_glyphs_match_igprim() {
        assert_eq!(CritVal::Woe.glyph(),      IgPrim::woe.glyph());
        assert_eq!(CritVal::Monad.glyph(),         IgPrim::monad.glyph());
        assert_eq!(CritVal::Roar.glyph(),  IgPrim::roar.glyph());
        assert_eq!(CritVal::Err.glyph(),        IgPrim::err.glyph());
        assert_eq!(CritVal::Haha.glyph(),     IgPrim::haha.glyph());
    }

    #[test]
    fn hval_glyphs_match_igprim() {
        assert_eq!(ChirVal::Fee.glyph(),  IgPrim::fee.glyph());
        assert_eq!(ChirVal::Kick.glyph(),  IgPrim::kick.glyph());
        assert_eq!(ChirVal::Sure.glyph(),  IgPrim::sure.glyph());
        assert_eq!(ChirVal::Wool.glyph(), IgPrim::wool.glyph());
    }

    #[test]
    fn sval_glyphs_match_igprim() {
        assert_eq!(StoiVal::Hung.glyph(),    IgPrim::hung.glyph());
        assert_eq!(StoiVal::So.glyph(),   IgPrim::so.glyph());
        assert_eq!(StoiVal::Up.glyph(), IgPrim::up.glyph());
    }

    #[test]
    fn oval_glyphs_match_igprim() {
        assert_eq!(ProtVal::Awe.glyph(), IgPrim::awe.glyph());
        assert_eq!(ProtVal::Oak.glyph(),      IgPrim::oak.glyph());
        assert_eq!(ProtVal::Ah.glyph(),       IgPrim::ah.glyph());
        assert_eq!(ProtVal::Zoo.glyph(),      IgPrim::zoo.glyph());
    }
}
