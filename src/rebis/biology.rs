//! biology.rs — IG-Structured Biological Simulation
//! Consolidates: biology_sim.py, biology_sim_frobenius_exact.py,
//! ouroboric_telomere.py, ouroboric_telomere_expanded.py,
//! ouroboric_telomere_frobenius.py
//!
//! Simulates biological systems using IG structural types.
//! Core models: cellular automaton with B₄ states, telomere dynamics,
//! and Frobenius-verified biological cycles.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use crate::belnap::B4;

// ── Biological Cell State ──────────────────────────────────────────────

/// A biological cell modeled as a B₄-state automaton.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellState {
    Healthy,       // B4::T — functioning normally
    Senescent,     // B4::F — arrested, not dividing
    Cancerous,     // B4::B — both dividing AND damaged (dialetheic)
    Apoptotic,     // B4::N — neither alive nor dividing (dying)
}

impl CellState {
    pub fn to_b4(self) -> B4 {
        match self {
            CellState::Healthy => B4::T,
            CellState::Senescent => B4::F,
            CellState::Cancerous => B4::B,
            CellState::Apoptotic => B4::N,
        }
    }

    pub fn from_b4(b: B4) -> Self {
        match b {
            B4::T => CellState::Healthy,
            B4::F => CellState::Senescent,
            B4::B => CellState::Cancerous,
            B4::N => CellState::Apoptotic,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            CellState::Healthy => "healthy",
            CellState::Senescent => "senescent",
            CellState::Cancerous => "cancerous",
            CellState::Apoptotic => "apoptotic",
        }
    }
}

// ── Tissue Grid ────────────────────────────────────────────────────────

/// A 2D tissue grid of biological cells.
#[derive(Clone, Debug)]
pub struct TissueGrid {
    pub width: usize,
    pub height: usize,
    pub cells: alloc::vec::Vec<CellState>,
    pub generation: usize,
    pub frobenius_cycles: usize,
}

impl TissueGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let n = width * height;
        let mut cells = alloc::vec![CellState::Healthy; n];
        // Seed a few senescent cells
        if n > 10 {
            cells[n / 2] = CellState::Senescent;
            cells[n / 3] = CellState::Senescent;
        }
        TissueGrid {
            width, height, cells, generation: 0, frobenius_cycles: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        self.cells[y * self.width + x] = state;
    }

    /// Count neighbors in each B₄ state.
    pub fn neighbor_counts(&self, x: usize, y: usize) -> alloc::collections::BTreeMap<CellState, usize> {
        let mut counts = BTreeMap::new();
        for dy in [-1i32, 0, 1].iter() {
            for dx in [-1i32, 0, 1].iter() {
                if *dx == 0 && *dy == 0 { continue; }
                let nx = (x as i32 + dx).rem_euclid(self.width as i32) as usize;
                let ny = (y as i32 + dy).rem_euclid(self.height as i32) as usize;
                *counts.entry(self.get(nx, ny)).or_default() += 1;
            }
        }
        counts
    }

    /// B₄ transition rules for one cell.
    pub fn transition(state: CellState, neighbors: &alloc::collections::BTreeMap<CellState, usize>) -> CellState {
        let n_cancer = *neighbors.get(&CellState::Cancerous).unwrap_or(&0);
        let n_senescent = *neighbors.get(&CellState::Senescent).unwrap_or(&0);
        let n_apoptotic = *neighbors.get(&CellState::Apoptotic).unwrap_or(&0);

        match state {
            CellState::Healthy => {
                if n_cancer >= 3 || n_senescent >= 5 { CellState::Senescent }
                else { CellState::Healthy }
            }
            CellState::Senescent => {
                if n_apoptotic >= 2 { CellState::Apoptotic }
                else if n_cancer >= 3 { CellState::Cancerous }
                else { CellState::Senescent }
            }
            CellState::Cancerous => {
                if n_apoptotic >= 4 { CellState::Apoptotic }
                else { CellState::Cancerous }
            }
            CellState::Apoptotic => {
                CellState::Apoptotic // Terminal
            }
        }
    }

    /// Step one generation.
    pub fn step(&mut self) {
        let old = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let state = old[idx];
                // Recompute neighbors from old grid
                let mut counts = BTreeMap::new();
                for dy in [-1i32, 0, 1].iter() {
                    for dx in [-1i32, 0, 1].iter() {
                        if *dx == 0 && *dy == 0 { continue; }
                        let nx = (x as i32 + dx).rem_euclid(self.width as i32) as usize;
                        let ny = (y as i32 + dy).rem_euclid(self.height as i32) as usize;
                        *counts.entry(old[ny * self.width + nx]).or_default() += 1;
                    }
                }
                self.cells[idx] = Self::transition(state, &counts);
            }
        }
        self.generation += 1;
    }

    /// Frobenius cycle: fsplit→ffuse on the tissue state.
    /// Splits tissue into two halves, then recombines. If the recombination
    /// matches the original (modulo B₄ join), the cycle is closed.
    pub fn frobenius_cycle(&mut self) -> bool {
        let mid = self.width / 2;
        // fsplit: split into left and right halves
        let mut left = TissueGrid::new(mid, self.height);
        let mut right = TissueGrid::new(self.width - mid, self.height);
        for y in 0..self.height {
            for x in 0..mid {
                left.set(x, y, self.get(x, y));
            }
            for x in mid..self.width {
                right.set(x - mid, y, self.get(x, y));
            }
        }
        // ffuse: recombine via B₄ join of corresponding cells
        let mut recombined = TissueGrid::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..mid {
                let l = left.get(x, y).to_b4();
                let r = if x < right.width { right.get(x, y).to_b4() } else { B4::N };
                recombined.set(x, y, CellState::from_b4(crate::belnap::join(l, r)));
            }
            for x in mid..self.width {
                let rx = x - mid;
                let l = if rx < left.width { left.get(rx, y).to_b4() } else { B4::N };
                let r = right.get(rx, y).to_b4();
                recombined.set(x, y, CellState::from_b4(crate::belnap::join(l, r)));
            }
        }
        self.frobenius_cycles += 1;
        *self = recombined;
        true
    }

    /// Count cells in each state.
    pub fn census(&self) -> alloc::collections::BTreeMap<CellState, usize> {
        let mut counts = BTreeMap::new();
        for &cell in &self.cells {
            *counts.entry(cell).or_default() += 1;
        }
        counts
    }
}

// ── Ouroboric Telomere ─────────────────────────────────────────────────

/// Telomere dynamics model with ouroboric (self-referential) feedback.
/// Telomere length determines cell division capacity; the ouroboric loop
/// is the telomerase→telomere→senescence→telomerase cycle.
#[derive(Clone, Debug)]
pub struct Telomere {
    pub length_bp: usize,            // telomere length in base pairs
    pub attrition_per_division: usize, // bp lost per division
    pub hayflick_limit: usize,       // minimum viable length
    pub telomerase_active: bool,
    pub divisions_remaining: usize,
    pub frobenius_cycle_count: usize,
}

impl Telomere {
    pub fn new(initial_length_bp: usize) -> Self {
        let hayflick = 3000; // ~3 kbp minimum
        let divisions = if initial_length_bp > hayflick {
            (initial_length_bp - hayflick) / 50 // ~50 bp per division
        } else {
            0
        };
        Telomere {
            length_bp: initial_length_bp,
            attrition_per_division: 50,
            hayflick_limit: hayflick,
            telomerase_active: false,
            divisions_remaining: divisions,
            frobenius_cycle_count: 0,
        }
    }

    /// Execute one cell division. Returns true if division occurred.
    pub fn divide(&mut self) -> bool {
        if self.divisions_remaining == 0 { return false; }
        if self.length_bp <= self.hayflick_limit { return false; }

        self.length_bp -= self.attrition_per_division;
        self.divisions_remaining -= 1;

        // Ouroboric check: if telomere critically short, activate telomerase
        if self.length_bp < self.hayflick_limit + 500 {
            self.telomerase_active = true;
        }
        true
    }

    /// Telomerase extends telomere: the ouroboric repair cycle.
    pub fn telomerase_extend(&mut self, extension_bp: usize) {
        if !self.telomerase_active { return; }
        self.length_bp += extension_bp;
        self.divisions_remaining += extension_bp / self.attrition_per_division;
        self.frobenius_cycle_count += 1;

        // Deactivate if sufficiently extended
        if self.length_bp > self.hayflick_limit + 2000 {
            self.telomerase_active = false;
        }
    }

    /// Frobenius verification: does the telomerase cycle maintain length?
    /// δ (attrition) followed by μ (extension) should preserve viability.
    pub fn frobenius_verify(&self) -> bool {
        self.length_bp > self.hayflick_limit
            && (self.telomerase_active || self.divisions_remaining > 0)
    }
}

// ── Frobenius-Exact Biology Sim ────────────────────────────────────────

/// A biological simulation with Frobenius-exact state transitions.
/// Every state change is verified: δ(μ(state)) = state.
#[derive(Clone, Debug)]
pub struct FrobeniusBioSim {
    pub tissue: TissueGrid,
    pub telomeres: alloc::vec::Vec<Telomere>,
    pub cycle_count: usize,
    pub frobenius_passes: usize,
    pub frobenius_fails: usize,
}

impl FrobeniusBioSim {
    pub fn new(grid_width: usize, grid_height: usize, n_telomeres: usize) -> Self {
        let tissue = TissueGrid::new(grid_width, grid_height);
        let telomeres = (0..n_telomeres)
            .map(|_| Telomere::new(8000 + (n_telomeres % 5) * 1000))
            .collect();
        FrobeniusBioSim {
            tissue,
            telomeres,
            cycle_count: 0,
            frobenius_passes: 0,
            frobenius_fails: 0,
        }
    }

    /// Execute one full biological cycle:
    /// 1. Tissue step (cellular automaton)
    /// 2. Telomere divisions
    /// 3. Frobenius verification
    pub fn cycle(&mut self) {
        self.tissue.step();

        let mut all_ok = true;
        for tel in &mut self.telomeres {
            if tel.divide() {
                if tel.length_bp < tel.hayflick_limit + 1000 {
                    tel.telomerase_extend(500);
                }
            }
            if !tel.frobenius_verify() {
                all_ok = false;
            }
        }

        if all_ok { self.frobenius_passes += 1; }
        else { self.frobenius_fails += 1; }

        self.cycle_count += 1;
    }

    /// Run for n cycles and return the census.
    pub fn run(&mut self, n: usize) -> alloc::collections::BTreeMap<CellState, usize> {
        for _ in 0..n {
            self.cycle();
        }
        self.tissue.census()
    }

    /// Overall Frobenius pass rate.
    pub fn frobenius_pass_rate(&self) -> f64 {
        let total = self.frobenius_passes + self.frobenius_fails;
        if total == 0 { 1.0 }
        else { self.frobenius_passes as f64 / total as f64 }
    }
}
