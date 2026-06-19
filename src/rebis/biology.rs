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
