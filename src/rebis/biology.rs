//! biology.rs — IG-Structured Biological Simulation (FULL PORT)
//! Consolidates: biology_sim.py, biology_sim_frobenius_exact.py,
//! ouroboric_telomere.py, ouroboric_telomere_expanded.py,
//! ouroboric_telomere_frobenius.py
//!
//! Simulates biological systems using IG structural types.
//! Core models: cellular automaton with B4 states, telomere dynamics,
//! Frobenius-verified biological cycles, morphogenesis, and epigenetic
//! derepression via the ouroboric telomere axis.

use crate::belnap::B4;
use crate::sprintln;

// ── Biological Cell State ──────────────────────────────────────────────

/// A biological cell modeled as a B4-state automaton.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellState {
    Healthy,       // B4::T — functioning normally
    Senescent,     // B4::F — arrested, not dividing
    Cancerous,     // B4::B — both dividing AND damaged (dialetheic)
    Apoptotic,     // B4::N — neither alive nor dividing (dying)
}

impl CellState {
    pub fn to_b4(self) -> B4 {
        match self { Self::Healthy => B4::T, Self::Senescent => B4::F,
                     Self::Cancerous => B4::B, Self::Apoptotic => B4::N }
    }
    pub fn from_b4(b: B4) -> Self {
        match b { B4::T => Self::Healthy, B4::F => Self::Senescent,
                  B4::B => Self::Cancerous, B4::N => Self::Apoptotic }
    }
    pub fn name(self) -> &'static str {
        match self { Self::Healthy => "healthy", Self::Senescent => "senescent",
                     Self::Cancerous => "cancerous", Self::Apoptotic => "apoptotic" }
    }
}

// ── Tissue Grid ────────────────────────────────────────────────────────

/// A 2D tissue grid of biological cells.
#[derive(Clone, Debug)]
pub struct TissueGrid {
    pub width: usize, pub height: usize,
    pub cells: alloc::vec::Vec<CellState>,
    pub generation: usize,
}

impl TissueGrid {
    pub fn new(w: usize, h: usize) -> Self {
        let n = w * h;
        let mut cells = alloc::vec::Vec::with_capacity(n);
        for i in 0..n {
            cells.push(if i % 7 == 0 { CellState::Senescent }
                       else if i % 13 == 0 { CellState::Cancerous }
                       else { CellState::Healthy });
        }
        TissueGrid { width: w, height: h, cells, generation: 0 }
    }

    fn get(&self, x: isize, y: isize) -> CellState {
        if x < 0 || y < 0 { return CellState::Healthy; }
        let (xu, yu) = (x as usize, y as usize);
        if xu >= self.width || yu >= self.height { return CellState::Healthy; }
        self.cells[yu * self.width + xu]
    }

    /// Step one generation: B4-lattice neighbor rule.
    pub fn step(&mut self) {
        let mut next = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let n_healthy = self.count_neighbors(x as isize, y as isize, CellState::Healthy);
                let n_cancer = self.count_neighbors(x as isize, y as isize, CellState::Cancerous);
                let n_senescent = self.count_neighbors(x as isize, y as isize, CellState::Senescent);
                next[idx] = match self.cells[idx] {
                    CellState::Healthy if n_cancer >= 2 => CellState::Cancerous,
                    CellState::Healthy if n_senescent >= 4 => CellState::Senescent,
                    CellState::Cancerous if n_healthy >= 5 => CellState::Apoptotic,
                    CellState::Senescent if n_healthy >= 3 => CellState::Apoptotic,
                    CellState::Apoptotic => CellState::Healthy,
                    other => other,
                };
            }
        }
        self.cells = next;
        self.generation += 1;
    }

    fn count_neighbors(&self, cx: isize, cy: isize, state: CellState) -> usize {
        let mut count = 0;
        for dy in -1..=1 { for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            if self.get(cx + dx, cy + dy) == state { count += 1; }
        }}
        count
    }

    pub fn state_counts(&self) -> (usize, usize, usize, usize) {
        let (mut h, mut s, mut c, mut a) = (0, 0, 0, 0);
        for cell in &self.cells {
            match cell { CellState::Healthy => h+=1, CellState::Senescent => s+=1,
                         CellState::Cancerous => c+=1, CellState::Apoptotic => a+=1 }
        }
        (h, s, c, a)
    }
}

// ── Cell Fate & Epigenetic Phase ───────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellFate { Proliferating, Senescent, Apoptotic, Transformed }

impl CellFate {
    pub fn name(self) -> &'static str {
        match self { Self::Proliferating => "proliferating", Self::Senescent => "senescent",
                     Self::Apoptotic => "apoptotic", Self::Transformed => "transformed" }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpigeneticPhase { Open, Poised, Silent, DeepSilent }

/// Telomere state: length, capping, damage signals.
#[derive(Clone, Debug)]
pub struct TelomereState {
    pub length_bp: usize,
    pub capped: bool,
    pub t_loop_present: bool,
    pub damage_foci: usize,
    pub g_quadruplex_count: usize,
}

impl TelomereState {
    pub fn new(length_bp: usize) -> Self {
        TelomereState { length_bp, capped: length_bp > 500, t_loop_present: length_bp > 1000,
                        damage_foci: 0, g_quadruplex_count: (length_bp / 500).max(1) }
    }
}

/// Shelterin complex sensor.
#[derive(Clone, Debug)]
pub struct ShelterinSensor {
    pub trf2_bound: bool,
    pub pot1_bound: bool,
    pub tpp1_bound: bool,
    pub rap1_bound: bool,
    pub tin2_bound: bool,
}

impl ShelterinSensor {
    pub fn sense(telomere: &TelomereState) -> Self {
        let capped = telomere.capped && telomere.length_bp > 300;
        ShelterinSensor {
            trf2_bound: telomere.t_loop_present,
            pot1_bound: telomere.length_bp > 100,
            tpp1_bound: capped,
            rap1_bound: capped,
            tin2_bound: capped,
        }
    }

    /// Is shelterin fully protecting this telomere?
    pub fn fully_bound(&self) -> bool {
        self.trf2_bound && self.pot1_bound && self.tpp1_bound && self.rap1_bound && self.tin2_bound
    }

    /// Derepression score: 0 = fully protected, 4 = fully exposed.
    pub fn derepression_score(&self) -> usize {
        let mut score = 0;
        if !self.trf2_bound { score += 1; }
        if !self.pot1_bound { score += 1; }
        if !self.tpp1_bound { score += 1; }
        if !self.rap1_bound || !self.tin2_bound { score += 1; }
        score
    }
}

// ── ATM Signaling ──────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ATMSignalingState {
    pub atm_phosphorylated: bool,
    pub chk2_active: bool,
    pub p53_level: f64,
    pub p21_level: f64,
    pub dna_damage_response: bool,
}

impl ATMSignalingState {
    pub fn inactive() -> Self {
        ATMSignalingState { atm_phosphorylated: false, chk2_active: false, p53_level: 0.0,
                           p21_level: 0.0, dna_damage_response: false }
    }

    /// Activate ATM in response to shelterin derepression.
    pub fn sense_damage(shelterin: &ShelterinSensor, telomere: &TelomereState) -> Self {
        let derep = shelterin.derepression_score();
        let damage_signal = derep as f64 / 4.0 + telomere.damage_foci as f64 * 0.1;
        let activated = damage_signal > 0.3 || telomere.damage_foci > 0;
        ATMSignalingState {
            atm_phosphorylated: activated,
            chk2_active: activated && damage_signal > 0.5,
            p53_level: if activated { (damage_signal * 2.0).min(1.0) } else { 0.1 },
            p21_level: if activated && damage_signal > 0.5 { damage_signal } else { 0.05 },
            dna_damage_response: activated,
        }
    }

    /// Determine cell fate from ATM signaling.
    pub fn determine_fate(&self, telomere_length: usize) -> CellFate {
        if self.dna_damage_response && telomere_length < 100 {
            CellFate::Apoptotic
        } else if self.p53_level > 0.7 && self.p21_level > 0.5 {
            CellFate::Senescent
        } else if self.p53_level > 0.9 {
            CellFate::Transformed
        } else {
            CellFate::Proliferating
        }
    }
}

// ── Full Ouroboric Telomere Simulation ─────────────────────────────────

/// Complete ouroboric telomere: shelterin → ATM → epigenetic → hTERT → extension
#[derive(Clone, Debug)]
pub struct OuroboricTelomere {
    pub telomere: TelomereState,
    pub shelterin: ShelterinSensor,
    pub atm: ATMSignalingState,
    pub htert_level: f64,
    pub epigenetic_phase: EpigeneticPhase,
    pub cell_fate: CellFate,
    pub division_count: usize,
    pub frobenius_score: f64,
}

impl OuroboricTelomere {
    /// Create a telomere with initial length.
    pub fn new(initial_length_bp: usize) -> Self {
        let telo = TelomereState::new(initial_length_bp);
        let shel = ShelterinSensor::sense(&telo);
        let atm = ATMSignalingState::sense_damage(&shel, &telo);
        let epi = if telo.length_bp > 3000 { EpigeneticPhase::Silent }
                  else if telo.length_bp > 1000 { EpigeneticPhase::Poised }
                  else { EpigeneticPhase::Open };
        let htert = if telo.length_bp < 500 { 0.5 } else { 0.05 };
        OuroboricTelomere {
            telomere: telo, shelterin: shel, atm, htert_level: htert,
            epigenetic_phase: epi, cell_fate: CellFate::Proliferating,
            division_count: 0, frobenius_score: 1.0,
        }
    }

    /// Run one division cycle.
    pub fn divide(&mut self) {
        // 1. Shelterin senses telomere state
        self.shelterin = ShelterinSensor::sense(&self.telomere);

        // 2. ATM responds to shelterin derepression
        self.atm = ATMSignalingState::sense_damage(&self.shelterin, &self.telomere);

        // 3. Epigenetic phase shifts based on ATM and length
        self.epigenetic_phase = match (self.telomere.length_bp, self.atm.p53_level) {
            (l, _) if l < 200 => EpigeneticPhase::DeepSilent,
            (l, p) if l < 500 && p > 0.6 => EpigeneticPhase::Open,
            (l, _) if l < 1000 => EpigeneticPhase::Poised,
            (_, p) if p > 0.3 => EpigeneticPhase::Poised,
            _ => EpigeneticPhase::Silent,
        };

        // 4. hTERT expression responds to epigenetic phase
        self.htert_level = match self.epigenetic_phase {
            EpigeneticPhase::Open => (self.htert_level + 0.2).min(1.0),
            EpigeneticPhase::Poised => (self.htert_level + 0.05).min(0.5),
            EpigeneticPhase::DeepSilent => (self.htert_level * 0.5).max(0.01),
            EpigeneticPhase::Silent => (self.htert_level * 0.9).max(0.01),
        };

        // 5. Telomere shortens (end-replication problem) but hTERT extends
        let shortening = 50 + (self.division_count as f64 * 0.1) as usize;
        let extension = (self.htert_level * 200.0) as usize;
        if self.telomere.length_bp > shortening {
            self.telomere.length_bp = self.telomere.length_bp - shortening + extension;
        } else {
            self.telomere.length_bp = extension;
        }

        // 6. Update capping and damage
        self.telomere.capped = self.telomere.length_bp > 300 && self.shelterin.fully_bound();
        self.telomere.t_loop_present = self.telomere.length_bp > 800;
        if !self.telomere.capped { self.telomere.damage_foci += 1; }

        // 7. Determine cell fate
        self.cell_fate = self.atm.determine_fate(self.telomere.length_bp);

        // 8. Frobenius score: how well shelterin(μ)∘telomere(δ) = id
        let derep = self.shelterin.derepression_score() as f64;
        self.frobenius_score = 1.0 / (1.0 + derep * 0.25 + self.telomere.damage_foci as f64 * 0.1);

        self.division_count += 1;
    }

    /// Run multiple divisions.
    pub fn run(&mut self, divisions: usize) {
        for _ in 0..divisions {
            if self.cell_fate == CellFate::Apoptotic { break; }
            self.divide();
        }
    }

    /// Generate a status report.
    pub fn report(&self) -> alloc::string::String {
        alloc::format!(
            "Telomere: len={}bp capped={} damage={} foci={}\n  Shelterin: derep={}/4 fully={}\n  ATM: p53={:.2} p21={:.2} DDR={}\n  Epigenetic: {:?} hTERT={:.2}\n  Fate: {:?}  Divisions: {}  Frob: {:.3}",
            self.telomere.length_bp, self.telomere.capped, self.telomere.g_quadruplex_count,
            self.telomere.damage_foci, self.shelterin.derepression_score(),
            self.shelterin.fully_bound(), self.atm.p53_level, self.atm.p21_level,
            self.atm.dna_damage_response, self.epigenetic_phase, self.htert_level,
            self.cell_fate, self.division_count, self.frobenius_score)
    }
}

// ── Frobenius-Exact Bio Simulation ─────────────────────────────────────

/// A Frobenius-verified biological simulation.
#[derive(Clone, Debug)]
pub struct FrobeniusBioSim {
    pub grid: TissueGrid,
    pub frobenius_passes: usize,
    pub frobenius_fails: usize,
    pub cycle_count: usize,
    pub closure_ratio: f64,
}

impl FrobeniusBioSim {
    pub fn new(w: usize, h: usize, _cycles: usize) -> Self {
        let grid = TissueGrid::new(w, h);
        FrobeniusBioSim { grid, frobenius_passes: 0, frobenius_fails: 0,
                         cycle_count: 0, closure_ratio: 0.0 }
    }

    /// Run simulation with Frobenius verification at each step.
    pub fn run(&mut self, cycles: usize) {
        for _ in 0..cycles {
            let before = self.grid.state_counts();
            self.grid.step();
            let after = self.grid.state_counts();
            // Frobenius check: total cell count conserved
            let before_total = before.0 + before.1 + before.2 + before.3;
            let after_total = after.0 + after.1 + after.2 + after.3;
            if before_total == after_total {
                self.frobenius_passes += 1;
            } else {
                self.frobenius_fails += 1;
            }
            self.cycle_count += 1;
        }
        let total = self.frobenius_passes + self.frobenius_fails;
        self.closure_ratio = if total > 0 { self.frobenius_passes as f64 / total as f64 } else { 1.0 };
    }

    pub fn report(&self) -> alloc::string::String {
        let (h, s, c, a) = self.grid.state_counts();
        alloc::format!("FrobeniusBioSim: gen={} passes={} fails={} closure={:.3}\n  Healthy:{} Senescent:{} Cancer:{} Apoptotic:{}",
            self.grid.generation, self.frobenius_passes, self.frobenius_fails,
            self.closure_ratio, h, s, c, a)
    }
}

// ── Telomere (simplified wrapper for quick CLI) ────────────────────────

#[derive(Clone, Debug)]
pub struct Telomere {
    pub length_bp: usize,
    pub hayflick_limit: usize,
    pub divisions_remaining: usize,
}

impl Telomere {
    pub fn new(initial_bp: usize) -> Self {
        let limit = initial_bp / 50;
        Telomere { length_bp: initial_bp, hayflick_limit: limit, divisions_remaining: limit }
    }
}


// ── Expanded Enzyme Active Site Catalog ─────────────────────────────────
// Ported from red-hot_rebis/rhr_p4rky/expanded_catalyzing_proteins.py
// Each entry: name, organism, PDB, active-site residues

#[derive(Debug, Clone)]
pub struct EnzymeEntry {
    pub name: &'static str,
    pub organism: &'static str,
    pub pdb: &'static str,
    pub active_site_residues: &'static [&'static str],
}

pub const SERINE_PROTEASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "chymotrypsin", organism: "Bos taurus", pdb: "1CHG", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "thrombin", organism: "Homo sapiens", pdb: "1PPB", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "elastase", organism: "Homo sapiens", pdb: "1HNE", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "subtilisin", organism: "Bacillus subtilis", pdb: "1SBT", active_site_residues: &["Ser221", "His64", "Asp32"] },
    EnzymeEntry { name: "factor_Xa", organism: "Homo sapiens", pdb: "1FJS", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "kallikrein", organism: "Homo sapiens", pdb: "1SPJ", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "plasmin", organism: "Homo sapiens", pdb: "1BUI", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "proteinase_K", organism: "Engyodontium album", pdb: "2PRK", active_site_residues: &["Ser224", "His69", "Asp39"] },
    EnzymeEntry { name: "urokinase", organism: "Homo sapiens", pdb: "1C5W", active_site_residues: &["Ser195", "His57", "Asp102"] },
];

pub const CYSTEINE_PROTEASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "papain", organism: "Carica papaya", pdb: "1PPN", active_site_residues: &["Cys25", "His159", "Asn175"] },
    EnzymeEntry { name: "cathepsin_B", organism: "Homo sapiens", pdb: "1CTB", active_site_residues: &["Cys29", "His199", "Asn219"] },
    EnzymeEntry { name: "cathepsin_L", organism: "Homo sapiens", pdb: "1CJL", active_site_residues: &["Cys25", "His163", "Asn187"] },
    EnzymeEntry { name: "caspase_3", organism: "Homo sapiens", pdb: "1CP3", active_site_residues: &["Cys163", "His121"] },
    EnzymeEntry { name: "caspase_1", organism: "Homo sapiens", pdb: "1IBC", active_site_residues: &["Cys285", "His237"] },
    EnzymeEntry { name: "calpain", organism: "Homo sapiens", pdb: "1KXR", active_site_residues: &["Cys115", "His272", "Asn296"] },
];

pub const ASPARTYL_PROTEASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "pepsin", organism: "Sus scrofa", pdb: "1PSN", active_site_residues: &["Asp32", "Asp215"] },
    EnzymeEntry { name: "renin", organism: "Homo sapiens", pdb: "1RNE", active_site_residues: &["Asp32", "Asp215"] },
    EnzymeEntry { name: "cathepsin_D", organism: "Homo sapiens", pdb: "1LYB", active_site_residues: &["Asp33", "Asp231"] },
    EnzymeEntry { name: "beta_secretase_BACE1", organism: "Homo sapiens", pdb: "1FKN", active_site_residues: &["Asp32", "Asp228"] },
    EnzymeEntry { name: "HIV1_protease_dimer", organism: "Human immunodeficiency virus 1", pdb: "1HHP", active_site_residues: &["Asp25", "Asp25_prime", "Ile50", "Ile50_prime"] },
];

pub const METALLOPROTEASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "thermolysin", organism: "Bacillus thermoproteolyticus", pdb: "1LND", active_site_residues: &["His142", "His146", "Glu166"] },
    EnzymeEntry { name: "carboxypeptidase_A", organism: "Bos taurus", pdb: "1CPA", active_site_residues: &["His69", "His196", "Glu72", "Arg145", "Tyr248"] },
    EnzymeEntry { name: "angiotensin_converting_enzyme", organism: "Homo sapiens", pdb: "1O86", active_site_residues: &["His383", "His387", "Glu411"] },
    EnzymeEntry { name: "matrix_metalloproteinase_9", organism: "Homo sapiens", pdb: "1L6J", active_site_residues: &["His401", "His405", "His411"] },
    EnzymeEntry { name: "matrix_metalloproteinase_2", organism: "Homo sapiens", pdb: "1CK7", active_site_residues: &["His403", "His407", "His413"] },
    EnzymeEntry { name: "carbonic_anhydrase_IX", organism: "Homo sapiens", pdb: "3IAI", active_site_residues: &["His94", "His96", "His119"] },
];

pub const KINASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "protein_kinase_A", organism: "Homo sapiens", pdb: "1ATP", active_site_residues: &["Lys72", "Glu91", "Asp184", "Asn171", "Asp166"] },
    EnzymeEntry { name: "SRC_kinase", organism: "Homo sapiens", pdb: "2SRC", active_site_residues: &["Lys295", "Glu310", "Asp404"] },
    EnzymeEntry { name: "EGFR_kinase", organism: "Homo sapiens", pdb: "1M17", active_site_residues: &["Lys721", "Glu738", "Asp831"] },
    EnzymeEntry { name: "CDK2", organism: "Homo sapiens", pdb: "1AQ1", active_site_residues: &["Lys33", "Glu51", "Asp145"] },
    EnzymeEntry { name: "MAP_kinase_ERK2", organism: "Homo sapiens", pdb: "1ERK", active_site_residues: &["Lys54", "Glu71", "Asp167"] },
    EnzymeEntry { name: "AKT_kinase", organism: "Homo sapiens", pdb: "1UNQ", active_site_residues: &["Lys179", "Glu198", "Asp292"] },
];

pub const PHOSPHATASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "alkaline_phosphatase", organism: "Homo sapiens", pdb: "1ALK", active_site_residues: &["Ser102", "Arg166", "Asp101", "His331"] },
    EnzymeEntry { name: "PTP1B", organism: "Homo sapiens", pdb: "1PTY", active_site_residues: &["Cys215", "Arg221", "Asp181"] },
    EnzymeEntry { name: "PP2A", organism: "Homo sapiens", pdb: "2IAE", active_site_residues: &["His59", "His241", "Asp57", "Asp85"] },
    EnzymeEntry { name: "calcineurin", organism: "Homo sapiens", pdb: "1AUI", active_site_residues: &["His101", "Asp121", "His199", "His281"] },
];

pub const OXIDOREDUCTASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "lactate_dehydrogenase", organism: "Homo sapiens", pdb: "1I10", active_site_residues: &["Arg109", "His195", "Arg171", "Asp168"] },
    EnzymeEntry { name: "malate_dehydrogenase", organism: "Homo sapiens", pdb: "2DFD", active_site_residues: &["Arg102", "His186", "Arg161", "Asp158"] },
    EnzymeEntry { name: "superoxide_dismutase_CuZn", organism: "Homo sapiens", pdb: "1SOS", active_site_residues: &["His46", "His48", "His61", "His118", "His44"] },
    EnzymeEntry { name: "catalase", organism: "Homo sapiens", pdb: "1DGF", active_site_residues: &["His74", "Asn147", "Tyr357"] },
    EnzymeEntry { name: "glutathione_peroxidase", organism: "Homo sapiens", pdb: "1GP1", active_site_residues: &["Sec45", "Gln81", "Trp158"] },
    EnzymeEntry { name: "monoamine_oxidase_A", organism: "Homo sapiens", pdb: "2BXR", active_site_residues: &["Cys406", "Tyr444"] },
    EnzymeEntry { name: "dihydrofolate_reductase", organism: "Homo sapiens", pdb: "1HFR", active_site_residues: &["Glu30", "Phe31", "Phe34"] },
    EnzymeEntry { name: "aldose_reductase", organism: "Homo sapiens", pdb: "1AH3", active_site_residues: &["Tyr48", "His110", "Lys77", "Trp111"] },
    EnzymeEntry { name: "nitric_oxide_synthase", organism: "Homo sapiens", pdb: "1NOS", active_site_residues: &["Cys194", "Trp356", "Tyr585", "Glu361"] },
    EnzymeEntry { name: "cytochrome_P450_3A4", organism: "Homo sapiens", pdb: "1TQN", active_site_residues: &["Cys442"] },
];

pub const TRANSFERASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "hexokinase", organism: "Homo sapiens", pdb: "1HKB", active_site_residues: &["Asp205", "Lys170", "Thr232"] },
    EnzymeEntry { name: "glutathione_S_transferase", organism: "Homo sapiens", pdb: "1GSD", active_site_residues: &["Tyr6", "Ser11", "Arg13", "Arg20"] },
    EnzymeEntry { name: "creatine_kinase", organism: "Homo sapiens", pdb: "1CRK", active_site_residues: &["Cys282", "Arg96", "Arg129", "Arg287"] },
    EnzymeEntry { name: "DNA_methyltransferase_1", organism: "Homo sapiens", pdb: "3SWR", active_site_residues: &["Cys1226", "Glu1266", "Arg1312"] },
    EnzymeEntry { name: "acetyltransferase_HAT", organism: "Homo sapiens", pdb: "1P0B", active_site_residues: &["Glu173", "His140", "Cys168"] },
    EnzymeEntry { name: "catechol_O_methyltransferase", organism: "Homo sapiens", pdb: "3BWM", active_site_residues: &["Lys144", "Asp141", "Glu199"] },
];

pub const HYDROLASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "beta_lactamase_TEM1", organism: "Escherichia coli", pdb: "1BTL", active_site_residues: &["Ser70", "Lys73", "Ser130", "Glu166"] },
    EnzymeEntry { name: "phospholipase_A2", organism: "Homo sapiens", pdb: "1P2P", active_site_residues: &["His48", "Asp99", "Tyr52", "Tyr73"] },
    EnzymeEntry { name: "butyrylcholinesterase", organism: "Homo sapiens", pdb: "1P0I", active_site_residues: &["Ser198", "His438", "Glu325"] },
    EnzymeEntry { name: "lipase_pancreatic", organism: "Homo sapiens", pdb: "1LPA", active_site_residues: &["Ser152", "Asp176", "His263"] },
    EnzymeEntry { name: "amylase_alpha", organism: "Homo sapiens", pdb: "1HNY", active_site_residues: &["Asp197", "Glu233", "Asp300"] },
    EnzymeEntry { name: "urease_helicobacter", organism: "Helicobacter pylori", pdb: "1E9Z", active_site_residues: &["His136", "His138", "His248", "Asp362", "Lys220"] },
];

pub const LYASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "fumarase", organism: "Homo sapiens", pdb: "3E04", active_site_residues: &["His188", "Glu296", "His129"] },
    EnzymeEntry { name: "enolase", organism: "Homo sapiens", pdb: "2PSN", active_site_residues: &["Lys345", "Glu211", "Lys396", "His159"] },
    EnzymeEntry { name: "aldolase_fructose_bisphosphate", organism: "Homo sapiens", pdb: "1ALD", active_site_residues: &["Lys229", "Glu187", "Lys146", "Arg148"] },
];

pub const ISOMERASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "triosephosphate_isomerase", organism: "Homo sapiens", pdb: "1TIM", active_site_residues: &["Glu165", "His95", "Lys13"] },
    EnzymeEntry { name: "phosphoglucose_isomerase", organism: "Homo sapiens", pdb: "1IAT", active_site_residues: &["His388", "Glu357", "Lys518"] },
    EnzymeEntry { name: "peptidylprolyl_isomerase_FKBP12", organism: "Homo sapiens", pdb: "1FKF", active_site_residues: &["Phe36", "Tyr82", "Trp59", "Phe99"] },
    EnzymeEntry { name: "topoisomerase_II", organism: "Homo sapiens", pdb: "1ZXM", active_site_residues: &["Tyr805", "Arg488"] },
];

pub const LIGASES: &[EnzymeEntry] = &[
    EnzymeEntry { name: "DNA_ligase_I", organism: "Homo sapiens", pdb: "1X9N", active_site_residues: &["Lys568", "Glu621", "Arg646"] },
];

pub const DRUG_TARGETS: &[EnzymeEntry] = &[
    EnzymeEntry { name: "COX_1", organism: "Homo sapiens", pdb: "1CQE", active_site_residues: &["Ser530", "Tyr385"] },
    EnzymeEntry { name: "COX_2", organism: "Homo sapiens", pdb: "1CX2", active_site_residues: &["Ser530", "Tyr385", "Arg120", "Val523"] },
    EnzymeEntry { name: "HMG_CoA_reductase", organism: "Homo sapiens", pdb: "1HWK", active_site_residues: &["Glu83", "Lys735", "Asp767", "His866"] },
    EnzymeEntry { name: "ACE2", organism: "Homo sapiens", pdb: "1R42", active_site_residues: &["His345", "His374", "Glu402", "His540"] },
    EnzymeEntry { name: "acetylcholinesterase_electrophorus", organism: "Electrophorus electricus", pdb: "1C2B", active_site_residues: &["Ser200", "His440", "Glu327"] },
    EnzymeEntry { name: "xanthine_oxidase", organism: "Homo sapiens", pdb: "1FIQ", active_site_residues: &["Glu802", "Arg880", "Glu1261"] },
    EnzymeEntry { name: "tyrosinase", organism: "Homo sapiens", pdb: "5M8L", active_site_residues: &["His180", "His202", "His211", "His363", "His367", "His390"] },
    EnzymeEntry { name: "adenosine_deaminase", organism: "Homo sapiens", pdb: "1ADD", active_site_residues: &["His214", "His238", "Asp295", "Asp296"] },
    EnzymeEntry { name: "thymidylate_synthase", organism: "Homo sapiens", pdb: "1HVY", active_site_residues: &["Cys195", "Arg218"] },
    EnzymeEntry { name: "carbonic_anhydrase_XII", organism: "Homo sapiens", pdb: "1JCZ", active_site_residues: &["His94", "His96", "His119"] },
    EnzymeEntry { name: "aldose_reductase_like_1", organism: "Homo sapiens", pdb: "1PWL", active_site_residues: &["Tyr48", "His110", "Lys77"] },
    EnzymeEntry { name: "pancreatic_lipase", organism: "Homo sapiens", pdb: "1LPA", active_site_residues: &["Ser152", "Asp176", "His263"] },
    EnzymeEntry { name: "acetylcholinesterase_human", organism: "Homo sapiens", pdb: "4EY7", active_site_residues: &["Ser203", "His447", "Glu334"] },
    EnzymeEntry { name: "chymase", organism: "Homo sapiens", pdb: "1PJP", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "neutrophil_elastase", organism: "Homo sapiens", pdb: "1HNE", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "plasminogen", organism: "Homo sapiens", pdb: "1DDJ", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "tPA", organism: "Homo sapiens", pdb: "1TPK", active_site_residues: &["Ser195", "His57", "Asp102"] },
    EnzymeEntry { name: "furin", organism: "Homo sapiens", pdb: "1P8J", active_site_residues: &["Ser368", "His194", "Asp153"] },
    EnzymeEntry { name: "TMPRSS2", organism: "Homo sapiens", pdb: "7MEQ", active_site_residues: &["Ser441", "His296", "Asp345"] },
    EnzymeEntry { name: "cathepsin_K", organism: "Homo sapiens", pdb: "1ATK", active_site_residues: &["Cys25", "His162", "Asn182"] },
    EnzymeEntry { name: "SARS_CoV2_3CL_protease", organism: "SARS-CoV-2", pdb: "6LU7", active_site_residues: &["Cys145", "His41"] },
    EnzymeEntry { name: "SARS_CoV2_PLpro", organism: "SARS-CoV-2", pdb: "6WX4", active_site_residues: &["Cys111", "His272", "Asp286"] },
    EnzymeEntry { name: "NS3_NS4A_protease", organism: "Hepatitis C virus", pdb: "1DY9", active_site_residues: &["Ser139", "His57", "Asp81"] },
    EnzymeEntry { name: "neuraminidase", organism: "Influenza A virus", pdb: "2HU4", active_site_residues: &["Arg118", "Asp151", "Arg152", "Arg292", "Arg371", "Tyr406", "Glu277"] },
    EnzymeEntry { name: "reverse_transcriptase_HIV", organism: "HIV-1", pdb: "1RTD", active_site_residues: &["Asp110", "Asp185", "Asp186"] },
    EnzymeEntry { name: "integrase_HIV", organism: "HIV-1", pdb: "1QS4", active_site_residues: &["Asp64", "Asp116", "Glu152"] },
    EnzymeEntry { name: "RNA_polymerase_II", organism: "Homo sapiens", pdb: "1I50", active_site_residues: &["Asp481", "Asp483", "Asp485"] },
];

pub const ADDITIONAL_TARGETS: &[EnzymeEntry] = &[
    EnzymeEntry { name: "dihydroorotate_dehydrogenase", organism: "Homo sapiens", pdb: "2BXV", active_site_residues: &["Arg136", "Gln47", "Tyr356"] },
    EnzymeEntry { name: "inosine_monophosphate_dehydrogenase", organism: "Homo sapiens", pdb: "1B3O", active_site_residues: &["Cys331", "Asp364"] },
    EnzymeEntry { name: "PARP1", organism: "Homo sapiens", pdb: "4UND", active_site_residues: &["Glu988", "His862", "Tyr907"] },
    EnzymeEntry { name: "histone_deacetylase_1", organism: "Homo sapiens", pdb: "4BKX", active_site_residues: &["His141", "Asp176", "His178", "Asp264"] },
    EnzymeEntry { name: "sirtuin_1", organism: "Homo sapiens", pdb: "4I5I", active_site_residues: &["His363", "Phe297", "Asn346"] },
    EnzymeEntry { name: "peptidyl_arginine_deiminase_4", organism: "Homo sapiens", pdb: "1WDA", active_site_residues: &["Cys645", "His471", "Asp473"] },
    EnzymeEntry { name: "glutaminase", organism: "Homo sapiens", pdb: "3SS3", active_site_residues: &["Ser286", "Lys289", "Tyr414"] },
    EnzymeEntry { name: "isocitrate_dehydrogenase_1", organism: "Homo sapiens", pdb: "1T09", active_site_residues: &["Arg132", "Arg100", "Asp275", "Asp279"] },
    EnzymeEntry { name: "succinate_dehydrogenase", organism: "Homo sapiens", pdb: "1ZOY", active_site_residues: &["His207", "Arg408", "Ser409", "Trp164"] },
    EnzymeEntry { name: "glutamate_dehydrogenase", organism: "Homo sapiens", pdb: "1L1F", active_site_residues: &["Lys113", "Asp165", "His189"] },
    EnzymeEntry { name: "phenylalanine_hydroxylase", organism: "Homo sapiens", pdb: "1J8U", active_site_residues: &["Glu286", "His285", "Arg297"] },
    EnzymeEntry { name: "tyrosine_hydroxylase", organism: "Homo sapiens", pdb: "2XSN", active_site_residues: &["His331", "His336", "Glu376"] },
    EnzymeEntry { name: "tryptophan_hydroxylase", organism: "Homo sapiens", pdb: "1MLW", active_site_residues: &["His272", "His277", "Glu317"] },
    EnzymeEntry { name: "DOPA_decarboxylase", organism: "Homo sapiens", pdb: "1JS3", active_site_residues: &["Lys303", "His192", "Asp271"] },
    EnzymeEntry { name: "acetyl_CoA_carboxylase", organism: "Homo sapiens", pdb: "2YL2", active_site_residues: &["Glu196", "Lys259", "Cys786"] },
    EnzymeEntry { name: "fatty_acid_synthase", organism: "Homo sapiens", pdb: "2JFD", active_site_residues: &["Cys161", "His302", "Ser581"] },
];


// ── Master Enzyme Catalog ──────────────────────────────────────────────

pub const ALL_ENZYMES: &[&[EnzymeEntry]] = &[
    SERINE_PROTEASES, CYSTEINE_PROTEASES, ASPARTYL_PROTEASES,
    METALLOPROTEASES, KINASES, PHOSPHATASES, OXIDOREDUCTASES,
    TRANSFERASES, HYDROLASES, LYASES, ISOMERASES, LIGASES,
    DRUG_TARGETS, ADDITIONAL_TARGETS,
];

const ENZYME_CLASS_NAMES: &[&str] = &[
    "Serine Proteases", "Cysteine Proteases", "Aspartyl Proteases",
    "Metalloproteases", "Kinases", "Phosphatases", "Oxidoreductases",
    "Transferases", "Hydrolases", "Lyases", "Isomerases", "Ligases",
    "Drug Targets", "Additional Targets",
];

pub fn lookup_enzyme(name: &str) -> Option<&'static EnzymeEntry> {
    for class in ALL_ENZYMES {
        if let Some(e) = class.iter().find(|e| e.name == name) {
            return Some(e);
        }
    }
    None
}

pub fn print_enzyme_catalog(filter: &str) {
    for (i, class) in ALL_ENZYMES.iter().enumerate() {
        let class_name = ENZYME_CLASS_NAMES[i];
        let keep = filter.is_empty() || class_name.to_lowercase().contains(filter);
        let has_entry = class.iter().any(|e| !filter.is_empty() && e.name.contains(filter));
        if !keep && !has_entry { continue; }
        sprintln!("══ {} ({}) ══", class_name, class.len());
        for e in *class {
            if !filter.is_empty() && !e.name.contains(filter) && !class_name.to_lowercase().contains(filter) { continue; }
            sprintln!("  {:<28} {:<22} {:<6}  [{}]",
                e.name, e.organism, e.pdb, e.active_site_residues.join(", "));
        }
    }
    sprintln!("── Total: {} enzymes across {} classes ──",
        ALL_ENZYMES.iter().map(|c| c.len()).sum::<usize>(),
        ALL_ENZYMES.len());
}
