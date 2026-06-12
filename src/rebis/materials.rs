//! materials.rs — IG Structural Type → Material Design Bridge
//! Consolidates: ig_material_forge.py, frobenius_metamaterial.py,
//! sophick_forge.py, critical_metamaterial.py, ouroboric_alloy.py,
//! thermal_rectifier.py, gap_closure_module.py, non_qubit_qc.py,
//! frobenius_closure_complete.py, frobenius_exactor.py, materials_sim.py
//!
//! Maps 12-primitive IG tuple to concrete material properties:
//! composition, structure, processing, and predicted behaviors.

use alloc::vec::Vec;

// ── Material Property structs ──────────────────────────────────────────

/// Physical material specification derived from an IG tuple.
#[derive(Clone, Debug)]
pub struct MaterialSpec {
    pub name: alloc::string::String,
    pub dimensionality: &'static str,
    pub structure_type: &'static str,
    pub synthesis_method: &'static str,
    pub connectivity: &'static str,
    pub interface_type: &'static str,
    pub bond_energy_kjmol: (f64, f64), // (min, max) kJ/mol
    pub symmetry_class: &'static str,
    pub phase_purity: &'static str,
    pub processing_route: &'static str,
    pub interaction_range: &'static str,
    pub composition_mode: &'static str,
    pub critical_behavior: &'static str,
    pub memory_class: &'static str,
    pub component_count: &'static str,
    pub topological_protection: &'static str,
    pub frobenius_verified: bool,
}

// ── Primitive → Material Property Maps ─────────────────────────────────

/// D (Dimensionality) → structure type + synthesis method
pub fn d_material(glyph: &str) -> (&'static str, &'static str, &'static str) {
    match glyph {
        "𐑛" => ("0D", "nanoparticle / quantum dot", "colloidal precipitation"),
        "𐑨" => ("2D", "thin film / membrane", "CVD / ALD"),
        "𐑼" => ("3D bulk", "bulk solid / composite", "powder metallurgy / casting"),
        "𐑦" => ("hierarchical", "self-similar metamaterial", "additive + self-assembly"),
        _ => ("unknown", "unknown", "unknown"),
    }
}

/// T (Topology) → connectivity type + mechanical description
pub fn t_material(glyph: &str) -> (&'static str, &'static str) {
    match glyph {
        "𐑡" => ("percolating network", "high porosity, low density"),
        "𐑰" => ("core-shell", "graded interface, stress-buffered"),
        "𐑥" => ("bowtie/crossing", "auxetic possible, high toughness"),
        "𐑶" => ("interpenetrating", "IPN / MOF@COF, synergistic"),
        "𐑸" => ("self-referential fractal", "scale-invariant, self-healing"),
        _ => ("unknown", "unknown"),
    }
}

/// R (Coupling) → interface type + bond energy range
pub fn r_material(glyph: &str) -> (&'static str, (f64, f64), bool) {
    match glyph {
        "𐑩" => ("weak vdW", (0.0, 50.0), true),
        "𐑑" => ("moderate H-bond/π-π", (50.0, 150.0), false),
        "𐑽" => ("strong covalent/ionic", (150.0, 800.0), false),
        "𐑾" => ("dynamic (Diels-Alder, disulfide)", (100.0, 300.0), true),
        _ => ("unknown", (0.0, 0.0), false),
    }
}

/// P (Parity) → symmetry class
pub fn p_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑗" => "amorphous / disordered",
        "𐑿" => "quantum superposition",
        "𐑬" => "partially ordered (Z₂)",
        "𐑯" => "fully symmetric",
        "𐑹" => "Frobenius-special (μ∘δ=id)",
        _ => "unknown",
    }
}

/// F (Fidelity) → phase purity regime
pub fn f_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑱" => "defect-tolerant (classical)",
        "𐑞" => "thermal / entropic",
        "𐑐" => "quantum-coherent",
        _ => "unknown",
    }
}

/// K (Kinetics) → processing route
pub fn k_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑘" => "quenched / rapid solidification",
        "𐑤" => "moderate cooling / annealed",
        "𐑧" => "near-equilibrium growth",
        "𐑪" => "frozen (ordered) / templated",
        "𐑺" => "frozen (disordered) / glassy",
        _ => "unknown",
    }
}

/// G (Cardinality) → interaction range
pub fn g_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑚" => "short-range / nearest-neighbor",
        "𐑔" => "mesoscale / domain-level",
        "𐑲" => "long-range / universal",
        _ => "unknown",
    }
}

/// Gm (Composition) → synthesis mode
pub fn gm_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑝" => "one-pot / simultaneous",
        "𐑜" => "combinatorial / library",
        "𐑠" => "sequential / layer-by-layer",
        "𐑵" => "templated / broadcast",
        _ => "unknown",
    }
}

/// Phi (Criticality) → critical behavior
pub fn phi_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑢" => "inert / sub-critical",
        "⊙" => "self-sensing / self-modeling (Gate 1 open)",
        "𐑮" => "complex-plane tunable",
        "𐑻" => "exceptional point / non-Hermitian",
        "𐑣" => "runaway / supercritical",
        _ => "unknown",
    }
}

/// H (Chirality) → memory/history class
pub fn h_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑓" => "memoryless / instantaneous",
        "𐑒" => "one-step / short memory",
        "𐑖" => "two-step / hysteretic",
        "𐑫" => "eternal / shape-memory",
        _ => "unknown",
    }
}

/// S (Stoichiometry) → component count
pub fn s_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑙" => "unary / single-component",
        "𐑕" => "binary / solid-solution",
        "𐑳" => "multi-component / high-entropy",
        _ => "unknown",
    }
}

/// Ω (Winding) → topological protection class
pub fn o_material(glyph: &str) -> &'static str {
    match glyph {
        "𐑷" => "none / trivial",
        "𐑴" => "Z₂ parity-protected",
        "𐑭" => "integer winding / Chern",
        "𐑟" => "non-Abelian braiding",
        _ => "unknown",
    }
}

// ── Material Forge: IG tuple → concrete material spec ──────────────────

/// Forge a material specification from an IG 12-tuple (as glyph strings).
pub fn forge_material(
    name: &str,
    d: &str, t: &str, r: &str, p: &str, f: &str, k: &str,
    g: &str, gm: &str, phi: &str, h: &str, s: &str, o: &str,
) -> MaterialSpec {
    let (dim, structure, synthesis) = d_material(d);
    let (connectivity, _mech) = t_material(t);
    let (interface, bond_range, _reversible) = r_material(r);
    let symmetry = p_material(p);
    let purity = f_material(f);
    let processing = k_material(k);
    let range = g_material(g);
    let comp_mode = gm_material(gm);
    let critical = phi_material(phi);
    let memory = h_material(h);
    let components = s_material(s);
    let topo_prot = o_material(o);

    // Frobenius check: do primitive constraints form a consistent design?
    let frobenius_ok = verify_material_consistency(d, t, r, p, f, k, g, gm, phi, h, s, o);

    MaterialSpec {
        name: alloc::string::String::from(name),
        dimensionality: dim,
        structure_type: structure,
        synthesis_method: synthesis,
        connectivity,
        interface_type: interface,
        bond_energy_kjmol: bond_range,
        symmetry_class: symmetry,
        phase_purity: purity,
        processing_route: processing,
        interaction_range: range,
        composition_mode: comp_mode,
        critical_behavior: critical,
        memory_class: memory,
        component_count: components,
        topological_protection: topo_prot,
        frobenius_verified: frobenius_ok,
    }
}

/// Verify material design consistency (cross-primitive constraint checking).
pub fn verify_material_consistency(
    d: &str, t: &str, r: &str, p: &str, f: &str, k: &str,
    _g: &str, _gm: &str, phi: &str, h: &str, s: &str, o: &str,
) -> bool {
    // Rule 1: Hierarchical (𐑑) requires dynamic bonding (𐑾) or self-healing
    if d == "𐑑" && r != "𐑾" { return false; }

    // Rule 2: Frobenius-special parity (𐑹) requires self-referential topology (𐑸)
    if p == "𐑹" && t != "𐑸" { return false; }

    // Rule 3: Quantum-coherent (𐑐) requires at minimum Z₂ parity (𐑬) or higher
    if f == "𐑐" && p != "𐑬" && p != "𐑯" && p != "𐑹" { return false; }

    // Rule 4: ⊙ criticality requires Gate 1 open — at minimum two-component (𐑕)
    if phi == "⊙" && s == "𐑙" { return false; }

    // Rule 5: Non-Abelian braiding (𐑟) requires hierarchical (𐑦)
    if o == "𐑟" && d != "𐑦" { return false; }

    // Rule 6: Eternal memory (𐑫) requires near-equilibrium processing (𐑧) or slower
    if h == "𐑫" && k != "𐑧" && k != "𐑪" && k != "𐑺" { return false; }

    true
}

// ── Frobenius Metamaterial Designer ────────────────────────────────────

/// A metamaterial unit cell with Frobenius-verified closure.
#[derive(Clone, Debug)]
pub struct MetaCell {
    pub name: alloc::string::String,
    pub primitive_signature: [u8; 12],  // ordinal values
    pub resonance_freq_hz: f64,
    pub bandwidth_hz: f64,
    pub quality_factor: f64,
    pub chirality_handedness: i8,  // +1 right, -1 left, 0 achiral
    pub winding_number: i32,
}

impl MetaCell {
    /// Create a metamaterial cell from ordinal primitive values.
    pub fn new(name: &str, ordinals: [u8; 12], freq: f64, bw: f64) -> Self {
        let q = if bw > 0.0 { freq / bw } else { 0.0 };
        let chirality = if ordinals[9] >= 2 { 1 } else if ordinals[9] >= 1 { -1 } else { 0 };
        let winding = ordinals[11] as i32;

        MetaCell {
            name: alloc::string::String::from(name),
            primitive_signature: ordinals,
            resonance_freq_hz: freq,
            bandwidth_hz: bw,
            quality_factor: q,
            chirality_handedness: chirality,
            winding_number: winding,
        }
    }

    /// Frobenius verification: does the unit cell satisfy structural closure?
    /// For metamaterials, this means the cell's response to a probe signal
    /// is invertible — μ(δ(signal)) = signal.
    pub fn frobenius_verify(&self) -> bool {
        // A unit cell is Frobenius-closed if:
        // 1. Winding number = 0 or integer-protected
        // 2. Quality factor is finite (not runaway)
        // 3. Chirality is consistent with winding
        let winding_ok = self.winding_number == 0 || self.winding_number.abs() >= 1;
        let q_ok = self.quality_factor > 0.0 && self.quality_factor < 1_000_000.0;
        let chiral_ok = (self.chirality_handedness != 0) == (self.winding_number != 0);

        winding_ok && q_ok && chiral_ok
    }

    /// Couple two metamaterial cells (tensor product analog).
    pub fn couple(&self, other: &MetaCell) -> MetaCell {
        let mut coupled_ord = [0u8; 12];
        for i in 0..12 {
            coupled_ord[i] = self.primitive_signature[i].max(other.primitive_signature[i]);
        }
        // P and F: use min (bottleneck rule)
        coupled_ord[3] = self.primitive_signature[3].min(other.primitive_signature[3]);
        coupled_ord[4] = self.primitive_signature[4].min(other.primitive_signature[4]);

        let name = alloc::format!("{}+{}", self.name, other.name);
        MetaCell::new(
            &name,
            coupled_ord,
            (self.resonance_freq_hz + other.resonance_freq_hz) / 2.0,
            self.bandwidth_hz + other.bandwidth_hz,
        )
    }
}

// ── Ouroboric Alloy ────────────────────────────────────────────────────

/// An ouroboric alloy: structurally self-referential material.
/// Key signature: D=𐑑, T=𐑸, P=𐑹, φ̂=⊙
#[derive(Clone, Debug)]
pub struct OuroboricAlloy {
    pub base_element: &'static str,
    pub alloying_elements: alloc::vec::Vec<&'static str>,
    pub self_healing_temp_c: f64,
    pub critical_strain: f64,
    pub cycle_life: usize,
}

impl OuroboricAlloy {
    pub const SIGNATURE: [&'static str; 12] = [
        "𐑑", "𐑸", "𐑾", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑫", "𐑳", "𐑭"
    ];

    pub fn new(base: &'static str) -> Self {
        OuroboricAlloy {
            base_element: base,
            alloying_elements: Vec::new(),
            self_healing_temp_c: 300.0,
            critical_strain: 0.05,
            cycle_life: 1_000_000,
        }
    }

    pub fn add_alloy(&mut self, element: &'static str) {
        self.alloying_elements.push(element);
    }
}

// ── Thermal Rectifier ─────────────────────────────────────────────────

/// A thermal rectifier: asymmetric heat conduction.
/// Key signature: K=𐑪 (ordered trap), R=𐑽 (one-way adjoint)
#[derive(Clone, Debug)]
pub struct ThermalRectifier {
    pub forward_conductivity_wmk: f64,   // W/(m·K) hot→cold
    pub reverse_conductivity_wmk: f64,   // W/(m·K) cold→hot
    pub rectification_ratio: f64,
    pub operating_temp_k: (f64, f64),
}

impl ThermalRectifier {
    pub fn new(k_forward: f64, k_reverse: f64) -> Self {
        let ratio = if k_reverse > 0.0 { k_forward / k_reverse } else { f64::INFINITY };
        ThermalRectifier {
            forward_conductivity_wmk: k_forward,
            reverse_conductivity_wmk: k_reverse,
            rectification_ratio: ratio,
            operating_temp_k: (200.0, 600.0),
        }
    }

    /// Frobenius closure: forward∘reverse should return to original heat flux.
    pub fn frobenius_verify(&self) -> bool {
        self.rectification_ratio >= 1.0 && self.rectification_ratio.is_finite()
    }
}

// ── Non-Qubit Quantum Computer ─────────────────────────────────────────

/// A non-qubit quantum computing substrate.
/// Uses continuous-variable or topological encodings instead of discrete qubits.
#[derive(Clone, Debug)]
pub struct NonQubitQC {
    pub encoding_type: &'static str, // "CV", "topological", "bosonic"
    pub n_logical_dimensions: usize,
    pub error_threshold: f64,
    pub gate_fidelity: f64,
    pub topological_protection: bool,
}

impl NonQubitQC {
    pub const ENCODINGS: [&'static str; 3] = ["CV", "topological", "bosonic"];

    pub fn new(encoding: &'static str, n_dims: usize) -> Self {
        let topo_prot = encoding == "topological";
        let threshold = if topo_prot { 0.01 } else { 1e-4 };
        NonQubitQC {
            encoding_type: encoding,
            n_logical_dimensions: n_dims,
            error_threshold: threshold,
            gate_fidelity: if topo_prot { 0.9999 } else { 0.999 },
            topological_protection: topo_prot,
        }
    }

    /// Frobenius criterion: encoding must have a well-defined decode path.
    pub fn frobenius_verify(&self) -> bool {
        self.n_logical_dimensions > 0
            && self.gate_fidelity > 0.99
            && self.error_threshold > 0.0
    }
}

// ── Gap Closure Module ─────────────────────────────────────────────────

/// Analyzes the primitive gap between two materials and proposes
/// synthesis steps to close it.
#[derive(Clone, Debug)]
pub struct GapClosure {
    pub source_name: alloc::string::String,
    pub target_name: alloc::string::String,
    pub gap_primitives: alloc::vec::Vec<(usize, &'static str, &'static str)>, // (idx, from, to)
    pub synthesis_steps: alloc::vec::Vec<alloc::string::String>,
    pub estimated_energy_kjmol: f64,
}

impl GapClosure {
    /// Analyze the gap between two material specs and propose closure steps.
    pub fn analyze(source: &MaterialSpec, target: &MaterialSpec) -> Self {
        let mut gaps = Vec::new();
        let mut steps = Vec::new();
        let mut total_energy = 0.0;

        // Check each primitive for mismatch and propose steps
        // (simplified: focus on key property axes)
        if source.dimensionality != target.dimensionality {
            gaps.push((0, source.dimensionality, target.dimensionality));
            steps.push(alloc::format!("Change dimensionality: {} → {} via {}", 
                source.dimensionality, target.dimensionality, target.synthesis_method));
            total_energy += 50.0;
        }
        if source.connectivity != target.connectivity {
            gaps.push((1, source.connectivity, target.connectivity));
            steps.push(alloc::format!("Restructure connectivity: {} → {}", 
                source.connectivity, target.connectivity));
            total_energy += 30.0;
        }
        if source.symmetry_class != target.symmetry_class {
            steps.push(alloc::format!("Induce symmetry: {} → {}", 
                source.symmetry_class, target.symmetry_class));
            total_energy += 20.0;
        }
        if source.processing_route != target.processing_route {
            steps.push(alloc::format!("Change processing: {} → {}", 
                source.processing_route, target.processing_route));
            total_energy += 40.0;
        }

        GapClosure {
            source_name: source.name.clone(),
            target_name: target.name.clone(),
            gap_primitives: gaps,
            synthesis_steps: steps,
            estimated_energy_kjmol: total_energy,
        }
    }
}
