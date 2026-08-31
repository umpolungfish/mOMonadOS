//! materials.rs — IG Structural Type → Material Design Bridge
//! Expanded: full MaterialForge, SophickForge, FrobeniusExactor, NonQubitQC paradigms
use alloc::vec::Vec;

/// Predefined novel material names
pub const NOVEL_MATERIALS: &[&str] = &[
    "frobenius_composite",
    "critical_sensor_metamaterial", 
    "ep_detector",
    "eternal_memory_alloy",
    "topological_thermal_rectifier",
    "hierarchical_impact_absorber",
    "quantum_topological_substrate",
    "non_abelian_braiding_material",
];

pub fn material_count() -> usize { NOVEL_MATERIALS.len() }
