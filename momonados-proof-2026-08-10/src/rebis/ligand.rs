use crate::sprintln;
// rebis/ligand.rs — PDB-Aware Ligand Design from Catalytic Sites
// Port of red-hot_rebis/rhr_p4rky ligand pipeline (types)
//
// The ligand pipeline designs small molecules that fit catalytic active sites.
// Uses IG structural typing to characterize: binding mode (via R coupling),
// electronic complementarity (via P sym/F quantum), and steric fit (via K kinetics).
// The full Python pipeline (ligand_from_active_site.py, ligand_improvements.py,
// ligand_combinatorial.py, ligand_heterocycles.py, ligand_sicpovm.py) is 300K+
// — this Rust module provides the type system and key entry points.

use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use crate::imas_ig::{IgTuple, IgPrim};
use crate::algebra::tuple_distance;

// ─── Binding Mode Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingMode {
    Covalent,     // R = dagger (direct bond)
    Ionic,        // R = lr (bidirectional charge interaction)
    HydrogenBond, // R = cat (directional H-bond network)
    PiStacking,   // R = super (aromatic π-π interaction)
    Hydrophobic,  // R = super (van der Waals, buried)
    MetalCoord,   // R = dagger (metal coordination)
    Unknown,
}

impl BindingMode {
    pub fn name(&self) -> &'static str {
        match self { Self::Covalent => "covalent", Self::Ionic => "ionic",
            Self::HydrogenBond => "H-bond", Self::PiStacking => "π-stacking",
            Self::Hydrophobic => "hydrophobic", Self::MetalCoord => "metal coord",
            Self::Unknown => "unknown" }
    }

    pub fn to_primitive(&self) -> IgPrim {
        match self { Self::Covalent => IgPrim::ear,
            Self::Ionic => IgPrim::ian, Self::HydrogenBond => IgPrim::tot,
            Self::PiStacking => IgPrim::ado, Self::Hydrophobic => IgPrim::ado,
            Self::MetalCoord => IgPrim::ear, Self::Unknown => IgPrim::ado }
    }
}

// ─── Functional Group Types ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FunctionalGroup {
    pub name: &'static str,
    pub smiles_pattern: &'static str,
    pub ig_tuple: IgTuple,
    pub binding_modes: &'static [BindingMode],
    pub rotatable_bonds: u32,
}

pub const ALL_FUNCTIONAL_GROUPS: &[FunctionalGroup] = &[
    // ── Hydrogen bond donors/acceptors ──
    FunctionalGroup { name: "carboxyl", smiles_pattern: "C(=O)O", rotatable_bonds: 1,
        ig_tuple: IgTuple { d: IgPrim::dead, t: IgPrim::judge, r: IgPrim::ear,
            p: IgPrim::yew, f: IgPrim::peep, k: IgPrim::loll, g: IgPrim::bib,
            c: IgPrim::measure, phi: IgPrim::woe, h: IgPrim::kick, s: IgPrim::hung,
            omega: IgPrim::oak },
        binding_modes: &[BindingMode::Ionic, BindingMode::HydrogenBond],
    },
    FunctionalGroup { name: "amine", smiles_pattern: "C[NH2]", rotatable_bonds: 0,
        ig_tuple: IgTuple { d: IgPrim::dead, t: IgPrim::judge, r: IgPrim::ear,
            p: IgPrim::yew, f: IgPrim::they, k: IgPrim::loll, g: IgPrim::bib,
            c: IgPrim::vow, phi: IgPrim::woe, h: IgPrim::kick, s: IgPrim::hung,
            omega: IgPrim::oak },
        binding_modes: &[BindingMode::Ionic, BindingMode::HydrogenBond],
    },
    // ── Aromatic ──
    FunctionalGroup { name: "phenyl", smiles_pattern: "c1ccccc1", rotatable_bonds: 0,
        ig_tuple: IgTuple { d: IgPrim::dead, t: IgPrim::eat, r: IgPrim::ado,
            p: IgPrim::church, f: IgPrim::age, k: IgPrim::loll, g: IgPrim::bib,
            c: IgPrim::vow, phi: IgPrim::woe, h: IgPrim::fee, s: IgPrim::hung,
            omega: IgPrim::awe },
        binding_modes: &[BindingMode::PiStacking, BindingMode::Hydrophobic],
    },
    FunctionalGroup { name: "pyridine", smiles_pattern: "c1ccncc1", rotatable_bonds: 0,
        ig_tuple: IgTuple { d: IgPrim::ash, t: IgPrim::eat, r: IgPrim::ear,
            p: IgPrim::yew, f: IgPrim::they, k: IgPrim::loll, g: IgPrim::bib,
            c: IgPrim::gag, phi: IgPrim::woe, h: IgPrim::kick, s: IgPrim::hung,
            omega: IgPrim::oak },
        binding_modes: &[BindingMode::PiStacking, BindingMode::HydrogenBond,
            BindingMode::MetalCoord],
    },
    // ── Metal-coordinating ──
    FunctionalGroup { name: "hydroxamate", smiles_pattern: "C(=O)NO", rotatable_bonds: 1,
        ig_tuple: IgTuple { d: IgPrim::ash, t: IgPrim::mime, r: IgPrim::ian,
            p: IgPrim::or_, f: IgPrim::peep, k: IgPrim::egg, g: IgPrim::thigh,
            c: IgPrim::measure, phi: IgPrim::monad, h: IgPrim::kick, s: IgPrim::hung,
            omega: IgPrim::oak },
        binding_modes: &[BindingMode::MetalCoord, BindingMode::HydrogenBond],
    },
    // ── Sulfur-containing ──
    FunctionalGroup { name: "thiol", smiles_pattern:  "C[SH]", rotatable_bonds: 1,
        ig_tuple: IgTuple { d: IgPrim::dead, t: IgPrim::judge, r: IgPrim::ear,
            p: IgPrim::church, f: IgPrim::age, k: IgPrim::loll, g: IgPrim::bib,
            c: IgPrim::vow, phi: IgPrim::woe, h: IgPrim::fee, s: IgPrim::hung,
            omega: IgPrim::awe },
        binding_modes: &[BindingMode::Covalent, BindingMode::MetalCoord],
    },
    // ── Amide (peptide bond) ──
    FunctionalGroup { name: "amide", smiles_pattern:  "C(=O)N", rotatable_bonds: 0,
        ig_tuple: IgTuple { d: IgPrim::dead, t: IgPrim::judge, r: IgPrim::tot,
            p: IgPrim::church, f: IgPrim::they, k: IgPrim::egg, g: IgPrim::bib,
            c: IgPrim::measure, phi: IgPrim::woe, h: IgPrim::kick, s: IgPrim::hung,
            omega: IgPrim::oak },
        binding_modes: &[BindingMode::HydrogenBond],
    },
];// ─── Active Site Pocket ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ActiveSitePocket {
    pub name: String,
    pub catalytic_residues: Vec<String>,
    pub binding_residues: Vec<String>,
    pub expected_modes: Vec<BindingMode>,
    pub depth_angstrom: f32,
    pub hydrophobicity: f32,  // 0.0 (polar) to 1.0 (nonpolar)
}

impl ActiveSitePocket {
    pub fn new(name: &str) -> Self {
        ActiveSitePocket {
            name: name.to_string(),
            catalytic_residues: Vec::new(),
            binding_residues: Vec::new(),
            expected_modes: Vec::new(),
            depth_angstrom: 10.0,
            hydrophobicity: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LigandResult {
    pub smiles: String,
    pub score: f32,
    pub binding_mode: BindingMode,
    pub functional_groups: Vec<&'static str>,
}

// ─── Lookup ───────────────────────────────────────────────────────────────────

pub fn lookup_functional_group(name: &str) -> Option<&'static FunctionalGroup> {
    ALL_FUNCTIONAL_GROUPS.iter().find(|g| g.name == name)
}

pub fn all_functional_group_names() -> Vec<&'static str> {
    ALL_FUNCTIONAL_GROUPS.iter().map(|g| g.name).collect()
}

/// Score compatibility between a binding mode and a functional group
pub fn binding_compatibility(mode: BindingMode, group: &FunctionalGroup) -> f32 {
    if group.binding_modes.contains(&mode) { 1.0 }
    else {
        // Partial score from tuple distance
        let mode_tuple = IgTuple {
            d: IgPrim::dead, t: IgPrim::judge, r: mode.to_primitive(),
            p: IgPrim::church, f: IgPrim::age, k: IgPrim::loll,
            g: IgPrim::bib, c: IgPrim::vow, phi: IgPrim::woe,
            h: IgPrim::fee, s: IgPrim::hung, omega: IgPrim::awe,
        };
        let dist = tuple_distance(&mode_tuple, &group.ig_tuple);
        (1.0 - (dist / 12.0).min(1.0)).max(0.0)
    }
}

/// Score a pocket for a given binding mode using IG tuple complementarity
pub fn score_pocket(_pocket: &ActiveSitePocket, mode: BindingMode) -> f32 {
    let mut score = 0.0;
    for fg in ALL_FUNCTIONAL_GROUPS {
        let compat = binding_compatibility(mode, fg);
        if compat > 0.5 {
            score += compat;
        }
    }
    score
}

/// Suggest functional groups for a given binding mode and pocket
pub fn suggest_groups(pocket: &ActiveSitePocket, mode: BindingMode, max: usize) -> Vec<(&'static str, f32)> {
    let mut suggestions: Vec<(&'static str, f32)> = ALL_FUNCTIONAL_GROUPS.iter()
        .map(|g| (g.name, binding_compatibility(mode, g) * score_pocket(pocket, mode)))
        .filter(|(_, s)| *s > 0.3)
        .collect();
    suggestions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
    suggestions.truncate(max);
    suggestions
}

/// Simplified run: returns ligand suggestions (no actual PDB parsing in no_std)
pub fn ligands_from_pdb(_pdb_path: &str, active_residues: &[&str]) -> Vec<LigandResult> {
    let mut pocket = ActiveSitePocket::new("active_site");
    pocket.catalytic_residues = active_residues.iter().map(|s| s.to_string()).collect();

    let mut results = Vec::new();
    for mode in &[BindingMode::MetalCoord, BindingMode::Ionic, BindingMode::HydrogenBond,
                  BindingMode::Covalent, BindingMode::PiStacking, BindingMode::Hydrophobic] {
        let suggestions = suggest_groups(&pocket, *mode, 3);
        for (fg_name, score) in suggestions {
            let smiles = lookup_functional_group(fg_name)
                .map(|g| g.smiles_pattern.to_string())
                .unwrap_or_default();
            results.push(LigandResult {
                smiles,
                score: score * 100.0,
                binding_mode: *mode,
                functional_groups: vec![fg_name],
            });
        }
    }
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(core::cmp::Ordering::Equal));
    results.truncate(10);
    results
}

/// Print ligand suggestions
pub fn print_ligand_suggestions(site_name: &str, residues: &[&str]) {
    
    sprintln!("═══════════════════════════════════════════════════════");
    sprintln!("  Ligand Design for '{}'", site_name);
    sprintln!("  Active residues: {}", residues.join(", "));
    sprintln!("═══════════════════════════════════════════════════════");

    let results = ligands_from_pdb("", residues);
    if results.is_empty() {
        sprintln!("  No ligands suggested.");
        return;
    }
    sprintln!("  Rank  Score  Mode          FG     SMILES");
    for (i, r) in results.iter().enumerate() {
        sprintln!("  {:>3}.  {:>5.1}  {:<12}  {}  {}",
            i + 1, r.score, r.binding_mode.name(), r.functional_groups.join(","), r.smiles);
    }
}

// ─── Pipeline (structural shell) ──────────────────────────────────────────────
// The full Python ligand pipeline (55K-69K lines each) covers:
//   - PDB parsing & active-site identification
//   - Fragment-based combinatorial library generation
//   - Pharmacophore matching (SIC-POVM dual-link)
//   - IMASM structural optimization
//   - Heterocycle substitution scoring
//   - ADMET property prediction
// This Rust module provides the IG type foundation.

pub fn run_ligand_pipeline(pdb_path: &str) -> Vec<LigandResult> {
    // Entry point — delegates to simplified scoring
    ligands_from_pdb(pdb_path, &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_carboxyl() {
        let fg = lookup_functional_group("carboxyl").unwrap();
        assert_eq!(fg.smiles_pattern, "C(=O)O");
    }

    #[test]
    fn test_suggest_groups_nonempty() {
        let pocket = ActiveSitePocket::new("test");
        let suggestions = suggest_groups(&pocket, BindingMode::HydrogenBond, 5);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_binding_compatibility() {
        let carboxyl = lookup_functional_group("carboxyl").unwrap();
        let compat = binding_compatibility(BindingMode::Ionic, carboxyl);
        assert!((compat - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_all_functional_groups() {
        let names = all_functional_group_names();
        assert!(names.len() >= 6);
    }
}