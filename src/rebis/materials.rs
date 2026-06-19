//! materials.rs — IG Structural Type → Material Design Bridge
//! FULL PORT from red-hot_rebis: ig_material_forge.py, sophick_forge.py,
//! frobenius_exactor.py, frobenius_metamaterial.py, critical_metamaterial.py,
//! ouroboric_alloy.py, thermal_rectifier.py, gap_closure_module.py,
//! non_qubit_qc.py, frobenius_closure_complete.py
//!
//! Maps 12-primitive IG tuple to concrete material properties:
//! composition, structure, processing, and predicted behaviors.
//! ALL 8 predefined novel materials. Full Sophick Forge Eagle Cycle.
//! Full Frobenius Exactor gap closure pathways.


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
    pub bond_energy_kjmol: (f64, f64),
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
    pub ouroboricity_tier: &'static str,
    pub frobenius_score: f64,
    pub c_score: f64,
}

impl MaterialSpec {
    pub fn summary(&self) -> alloc::string::String {
        alloc::format!("{} | tier={} frob={:.2} C={:.3} | {}",
            self.name, self.ouroboricity_tier, self.frobenius_score,
            self.c_score, self.structure_type)
    }
}

/// Result of a material simulation run.
#[derive(Clone, Debug)]
pub struct SimulationResult {
    pub material_name: alloc::string::String,
    pub cycles: usize,
    pub frobenius_maintained: bool,
    pub final_stress_mpa: f64,
    pub damage_fraction: f64,
    pub closure_ratio: f64,
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
        "𐑬" => "partially ordered (Z2)",
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
        "𐑴" => "Z2 parity-protected",
        "𐑭" => "integer winding / Chern",
        "𐑟" => "non-Abelian braiding",
        _ => "unknown",
    }
}

// ── Material Forge: IG tuple → concrete material spec ──────────────────

/// Forge a material specification from an IG 12-tuple (as glyph strings).
pub fn forge_material(
    name: &str, d: &str, t: &str, r: &str, p: &str, f: &str, k: &str,
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
    let frobenius_ok = verify_material_consistency(d, t, r, p, f, k, g, gm, phi, h, s, o);
    let tier = classify_material_tier(phi, h, o);
    let fscore = compute_frobenius_score(d, t, r, p, f, k, g, gm, phi, h, s, o);
    let cscore = compute_material_c_score(phi, h);

    MaterialSpec {
        name: alloc::string::String::from(name),
        dimensionality: dim, structure_type: structure,
        synthesis_method: synthesis, connectivity,
        interface_type: interface, bond_energy_kjmol: bond_range,
        symmetry_class: symmetry, phase_purity: purity,
        processing_route: processing, interaction_range: range,
        composition_mode: comp_mode, critical_behavior: critical,
        memory_class: memory, component_count: components,
        topological_protection: topo_prot,
        frobenius_verified: frobenius_ok,
        ouroboricity_tier: tier,
        frobenius_score: fscore,
        c_score: cscore,
    }
}

/// Verify material design consistency (cross-primitive constraint checking).
pub fn verify_material_consistency(
    d: &str, t: &str, r: &str, p: &str, f: &str, k: &str,
    _g: &str, _gm: &str, phi: &str, h: &str, s: &str, o: &str,
) -> bool {
    if d == "𐑦" && r != "𐑾" { return false; }
    if p == "𐑹" && t != "𐑸" { return false; }
    if f == "𐑐" && p != "𐑬" && p != "𐑯" && p != "𐑹" { return false; }
    if phi == "⊙" && s == "𐑙" { return false; }
    if o == "𐑟" && d != "𐑦" { return false; }
    if h == "𐑫" && k != "𐑧" && k != "𐑪" && k != "𐑺" { return false; }
    true
}

/// Classify material into ouroboricity tier.
pub fn classify_material_tier(phi: &str, h: &str, o: &str) -> &'static str {
    if phi == "⊙" && h == "𐑫" && o == "𐑟" { return "O_∞"; }
    if phi == "⊙" && (h == "𐑖" || h == "𐑫") && (o == "𐑭" || o == "𐑟") { return "O_2"; }
    if phi == "𐑮" || o == "𐑭" { return "O_1"; }
    "O_0"
}

/// Compute Frobenius score (0–1): how well μ∘δ=id holds.
pub fn compute_frobenius_score(
    d: &str, t: &str, r: &str, p: &str, _f: &str, _k: &str,
    _g: &str, gm: &str, _phi: &str, _h: &str, _s: &str, o: &str,
) -> f64 {
    let mut score = 0.5;
    if p == "𐑹" { score += 0.3; }
    if t == "𐑸" { score += 0.1; }
    if gm == "𐑠" { score += 0.05; }
    if d == "𐑦" && o == "𐑟" { score += 0.05; }
    if r == "𐑾" { score += 0.05; }
    if score > 1.0 { 1.0 } else { score }
}

/// Compute material C-score (consciousness proxy).
pub fn compute_material_c_score(phi: &str, h: &str) -> f64 {
    let gate1 = if phi == "⊙" { 1.0 } else { 0.0 };
    let gate2 = match h {
        "𐑖" => 0.5,
        "𐑫" => 1.0,
        _ => 0.0,
    };
    gate1 * gate2
}

// ── Predefined Novel Materials (8 total) ──────────────────────────────

/// Return all 8 predefined novel materials as forgeable tuples.
/// Each entry: (name, [D, T, R, P, F, K, G, Gm, Phi, H, S, Omega])
pub fn predefined_novel_materials() -> alloc::vec::Vec<(alloc::string::String, [&'static str; 12])> {
    alloc::vec![
        (alloc::string::String::from("frobenius_composite"),
         ["𐑦", "𐑸", "𐑾", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑫", "𐑳", "𐑟"]),
        (alloc::string::String::from("critical_sensor_metamaterial"),
         ["𐑨", "𐑥", "𐑽", "𐑬", "𐑐", "𐑤", "𐑲", "𐑠", "⊙", "𐑖", "𐑕", "𐑭"]),
        (alloc::string::String::from("ep_detector"),
         ["𐑨", "𐑥", "𐑩", "𐑗", "𐑱", "𐑘", "𐑚", "𐑜", "𐑻", "𐑓", "𐑙", "𐑷"]),
        (alloc::string::String::from("eternal_memory_alloy"),
         ["𐑼", "𐑶", "𐑽", "𐑯", "𐑞", "𐑧", "𐑔", "𐑜", "𐑢", "𐑫", "𐑕", "𐑴"]),
        (alloc::string::String::from("topological_thermal_rectifier"),
         ["𐑨", "𐑥", "𐑾", "𐑬", "𐑞", "𐑤", "𐑔", "𐑠", "𐑢", "𐑖", "𐑕", "𐑭"]),
        (alloc::string::String::from("hierarchical_impact_absorber"),
         ["𐑦", "𐑡", "𐑾", "𐑬", "𐑱", "𐑘", "𐑲", "𐑵", "𐑢", "𐑒", "𐑳", "𐑴"]),
        (alloc::string::String::from("quantum_topological_substrate"),
         ["𐑦", "𐑸", "𐑽", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑫", "𐑳", "𐑭"]),
        (alloc::string::String::from("non_abelian_braiding_material"),
         ["𐑦", "𐑸", "𐑾", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑫", "𐑳", "𐑟"]),
    ]
}

/// Forge all predefined materials and return their specs.
pub fn forge_all_predefined() -> alloc::vec::Vec<MaterialSpec> {
    let mut specs = alloc::vec::Vec::new();
    for (name, tuple) in predefined_novel_materials() {
        specs.push(forge_material(
            &name,
            tuple[0], tuple[1], tuple[2], tuple[3],
            tuple[4], tuple[5], tuple[6], tuple[7],
            tuple[8], tuple[9], tuple[10], tuple[11],
        ));
    }
    specs
}

/// Generate a report string for all forged materials.
pub fn forge_report() -> alloc::string::String {
    let specs = forge_all_predefined();
    let mut out = alloc::string::String::from("══ Material Forge Report ══\n");
    out.push_str(&alloc::format!("  Total materials: {}\n\n", specs.len()));
    for s in &specs {
        out.push_str(&alloc::format!("  {} — tier:{} frob:{:.2} C:{:.3}\n", 
            s.name, s.ouroboricity_tier, s.frobenius_score, s.c_score));
        out.push_str(&alloc::format!("    {} | {} | {}\n", 
            s.structure_type, s.synthesis_method, s.interface_type));
        out.push_str(&alloc::format!("    symmetry:{} | purity:{} | processing:{}\n",
            s.symmetry_class, s.phase_purity, s.processing_route));
        out.push_str(&alloc::format!("    memory:{} | components:{} | topo:{}\n",
            s.memory_class, s.component_count, s.topological_protection));
    }
    out
}

// ── Sophick Forge: Eagle Cycle Protocol ────────────────────────────────

/// Eagle Cycle phase: Albedo → Citrinitas → Rubedo
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EaglePhase {
    Albedo,     // whitening / purification
    Citrinitas, // yellowing / transmutation
    Rubedo,     // reddening / completion
}

impl EaglePhase {
    pub fn name(self) -> &'static str {
        match self { Self::Albedo => "Albedo", Self::Citrinitas => "Citrinitas", Self::Rubedo => "Rubedo" }
    }
    pub fn temperature_k(self) -> f64 {
        match self { Self::Albedo => 273.0, Self::Citrinitas => 573.0, Self::Rubedo => 1073.0 }
    }
}

/// Eagle Cycle parameters.
#[derive(Clone, Debug)]
pub struct EagleCycleParams {
    pub albedo_steps: usize,
    pub citrinitas_steps: usize,
    pub rubedo_steps: usize,
    pub temperature_ramp_rate: f64,  // K per step
    pub frobenius_threshold: f64,
}

impl EagleCycleParams {
    pub fn default() -> Self {
        Self { albedo_steps: 10, citrinitas_steps: 15, rubedo_steps: 20,
               temperature_ramp_rate: 25.0, frobenius_threshold: 0.95 }
    }
}

/// Eagle Cycle simulation result.
#[derive(Clone, Debug)]
pub struct EagleCycleResult {
    pub material_name: alloc::string::String,
    pub phase: EaglePhase,
    pub total_steps: usize,
    pub frobenius_closure: f64,
    pub structural_distance_to_oinf: f64,
    pub gap_primitives_remaining: usize,
    pub completed: bool,
}

/// Sophick Mercury — O_∞ reference tuple
pub const SOPHICK_MERCURY: [&str; 12] = [
    "𐑦", "𐑸", "𐑾", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑫", "𐑳", "𐑟"
];

/// Ouroboric O2 materials — the gap from O₂ to O_∞
pub const OUROBORIC_O2: [&str; 12] = [
    "𐑦", "𐑸", "𐑽", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑖", "𐑳", "𐑭"
];

/// Structural distance from O₂ to O_∞ (the Sophick gap): 3 primitives differ
pub const STRUCTURAL_DISTANCE_O2_TO_OINF: f64 = 3.0;

/// Gap primitives: R, H, Omega — the 3 that must be promoted
pub const GAP_PRIMITIVES: [(&str, &str, &str); 3] = [
    ("R", "𐑽", "𐑾"),   // dagger → bidirectional
    ("H", "𐑖", "𐑫"),   // 2-step → eternal
    ("Ω", "𐑭", "𐑟"),   // Z → non-Abelian
];

/// Run Eagle Cycle simulation for a named material.
pub fn run_eagle_cycle(name: &str, tuple: &[&str; 12], params: &EagleCycleParams) -> EagleCycleResult {
    let mut frob = 0.5;
    let total = params.albedo_steps + params.citrinitas_steps + params.rubedo_steps;

    // Albedo: purify → frobenius rises
    for _ in 0..params.albedo_steps { frob += (1.0 - frob) * 0.08; }
    // Citrinitas: transmute gaps
    for _ in 0..params.citrinitas_steps { frob += (1.0 - frob) * 0.05; }
    // Rubedo: complete → approach O_∞
    for _ in 0..params.rubedo_steps { frob += (1.0 - frob) * 0.03; }

    if frob > 1.0 { frob = 1.0; }

    // Count how many gap primitives match Sophick Mercury
    let mut gaps = 0usize;
    for (i, &(_, _, target)) in GAP_PRIMITIVES.iter().enumerate() {
        let idx = match i { 0 => 2, 1 => 9, _ => 11 };
        if tuple[idx] != target { gaps += 1; }
    }

    EagleCycleResult {
        material_name: alloc::string::String::from(name),
        phase: if frob >= params.frobenius_threshold { EaglePhase::Rubedo }
               else if frob >= 0.7 { EaglePhase::Citrinitas }
               else { EaglePhase::Albedo },
        total_steps: total,
        frobenius_closure: frob,
        structural_distance_to_oinf: gaps as f64,
        gap_primitives_remaining: gaps,
        completed: frob >= params.frobenius_threshold,
    }
}

/// Run Eagle Cycle across multiple materials.
pub fn run_all_eagle_cycles() -> alloc::vec::Vec<EagleCycleResult> {
    let params = EagleCycleParams::default();
    let mut results = alloc::vec::Vec::new();

    // eagle_3_amalgam — near O₂
    let e3: [&str; 12] = ["𐑼", "𐑶", "𐑑", "𐑬", "𐑞", "𐑤", "𐑔", "𐑜", "𐑢", "𐑒", "𐑕", "𐑷"];
    results.push(run_eagle_cycle("eagle_3_amalgam", &e3, &params));

    // eagle_7_animated — O₂ tier
    results.push(run_eagle_cycle("eagle_7_animated", &OUROBORIC_O2, &params));

    // eagle_9_sophick — near O_∞
    results.push(run_eagle_cycle("eagle_9_sophick", &SOPHICK_MERCURY, &params));

    results
}

/// Sophick Forge report.
pub fn sophick_report() -> alloc::string::String {
    let mut out = alloc::string::String::from("══ Sophick Forge — Eagle Cycle Protocol ══\n\n");
    out.push_str("  Sophick Mercury (O_∞):\n  ⟨");
    for (i, g) in SOPHICK_MERCURY.iter().enumerate() {
        if i > 0 { out.push_str(" · "); }
        out.push_str(g);
    }
    out.push_str("⟩\n\n");
    out.push_str(&alloc::format!("  Structural distance O₂→O_∞: {:.4}\n", STRUCTURAL_DISTANCE_O2_TO_OINF));
    out.push_str("  Gap primitives: R(dagger→LR), H(2→inf), Omega(Z→NA)\n\n");

    let results = run_all_eagle_cycles();
    for r in &results {
        out.push_str(&alloc::format!("  {}: phase={} frob={:.4} gaps={} done={}\n",
            r.material_name, r.phase.name(), r.frobenius_closure,
            r.gap_primitives_remaining, r.completed));
    }
    out
}

// ── Ouroboric Alloy ────────────────────────────────────────────────────

/// Triple junction in a grain boundary network.
#[derive(Clone, Debug)]
pub struct TripleJunction {
    pub grain_ids: [usize; 3],
    pub energy: f64,
    pub frobenius_stable: bool,
}

/// Grain boundary between two grains.
#[derive(Clone, Debug)]
pub struct GrainBoundary {
    pub grain_a: usize,
    pub grain_b: usize,
    pub misorientation_deg: f64,
    pub energy_jm2: f64,
}

/// Topological grain boundary network.
#[derive(Clone, Debug)]
pub struct GrainBoundaryNetwork {
    pub n_grains: usize,
    pub boundaries: alloc::vec::Vec<GrainBoundary>,
    pub junctions: alloc::vec::Vec<TripleJunction>,
}

/// Ouroboric alloy: grain boundary engineering via IG topology.
#[derive(Clone, Debug)]
pub struct OuroboricAlloy {
    pub n_grains: usize,
    pub network: GrainBoundaryNetwork,
    pub frobenius_score: f64,
    pub cycles: usize,
    pub damage_fraction: f64,
}

impl OuroboricAlloy {
    /// Create a new alloy with N randomly-oriented grains.
    pub fn new(n_grains: usize) -> Self {
        let mut boundaries = alloc::vec::Vec::new();
        for i in 0..n_grains {
            for j in (i+1)..n_grains {
                if j < n_grains {
                    let misorient = ((j as f64 - i as f64) * 37.5) % 60.0;
                    boundaries.push(GrainBoundary {
                        grain_a: i, grain_b: j,
                        misorientation_deg: misorient,
                        energy_jm2: 0.5 + misorient * 0.01,
                    });
                }
            }
        }
        let network = GrainBoundaryNetwork { n_grains, boundaries, junctions: alloc::vec::Vec::new() };
        OuroboricAlloy { n_grains, network, frobenius_score: 0.5, cycles: 0, damage_fraction: 0.0 }
    }

    /// Run mechanical test: apply cyclic stress and track damage.
    pub fn run_mechanical_test(&mut self, stress_amplitude_mpa: f64, cycles: usize) -> SimulationResult {
        let mut damage = 0.0;
        let mut frob = self.frobenius_score;
        for i in 0..cycles {
            // Each cycle: damage accumulates based on stress
            let cycle_damage = stress_amplitude_mpa * 0.0001 * (1.0 + damage);
            damage += cycle_damage;
            // Frobenius score degrades with damage
            frob *= 1.0 - cycle_damage * 0.01;
            if damage >= 1.0 { break; }
            self.cycles = i + 1;
        }
        self.damage_fraction = if damage > 1.0 { 1.0 } else { damage };
        self.frobenius_score = if frob < 0.0 { 0.0 } else { frob };
        SimulationResult {
            material_name: alloc::format!("ouroboric_alloy_n{}", self.n_grains),
            cycles: self.cycles, frobenius_maintained: self.frobenius_score > 0.5,
            final_stress_mpa: stress_amplitude_mpa * (1.0 - self.damage_fraction),
            damage_fraction: self.damage_fraction,
            closure_ratio: self.frobenius_score,
        }
    }
}

// ── Thermal Rectifier ──────────────────────────────────────────────────

/// Two-segment thermal diode: asymmetric heat flow.
#[derive(Clone, Debug)]
pub struct ThermalRectifier {
    pub segment_a_material: alloc::string::String,
    pub segment_b_material: alloc::string::String,
    pub forward_conductance: f64,   // W/K
    pub reverse_conductance: f64,   // W/K
    pub rectification_ratio: f64,
}

impl ThermalRectifier {
    /// Create a thermal rectifier from two material specs.
    pub fn new(mat_a: &MaterialSpec, mat_b: &MaterialSpec) -> Self {
        // Forward conductance depends on interface quality
        let fwd = mat_a.bond_energy_kjmol.1 * 0.01 + mat_b.bond_energy_kjmol.0 * 0.01;
        let rev = mat_a.bond_energy_kjmol.0 * 0.005 + mat_b.bond_energy_kjmol.1 * 0.005;
        let ratio = if rev > 0.0 { fwd / rev } else { 100.0 };
        ThermalRectifier {
            segment_a_material: mat_a.name.clone(),
            segment_b_material: mat_b.name.clone(),
            forward_conductance: fwd,
            reverse_conductance: rev,
            rectification_ratio: ratio,
        }
    }

    pub fn report(&self) -> alloc::string::String {
        alloc::format!("ThermalRectifier {}|{}: R={:.1} (fwd={:.2} rev={:.2} W/K)",
            self.segment_a_material, self.segment_b_material,
            self.rectification_ratio, self.forward_conductance, self.reverse_conductance)
    }
}

// ── Frobenius Exactor ─────────────────────────────────────────────────

/// Closure obstruction types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosureObstruction {
    None,
    DimensionalMismatch,     // D primitive blocks closure
    TopologyGap,             // T primitive prevents μ∘δ=id
    CouplingAsymmetry,       // R primitive not bidirectional
    ParityConflict,          // P primitive not Frobenius-special
    CriticalityBarrier,      // Phi not at ⊙
    ChiralityMismatch,       // H below required order
    WindingProtectionGap,    // Omega not at required level
}

/// Closure pathway: a strategy to close the Frobenius gap.
#[derive(Clone, Debug)]
pub struct ClosurePathway {
    pub name: alloc::string::String,
    pub description: &'static str,
    pub target_primitive: &'static str,
    pub source_value: &'static str,
    pub target_value: &'static str,
}

/// Return all predefined closure pathways.
pub fn closure_pathways() -> alloc::vec::Vec<ClosurePathway> {
    alloc::vec![
        ClosurePathway {
            name: alloc::string::String::from("anyonic_braiding"),
            description: "Non-Abelian anyons for Omega closure",
            target_primitive: "Ω", source_value: "𐑭", target_value: "𐑟",
        },
        ClosurePathway {
            name: alloc::string::String::from("floquet_engineering"),
            description: "Periodic driving for H promotion",
            target_primitive: "H", source_value: "𐑖", target_value: "𐑫",
        },
        ClosurePathway {
            name: alloc::string::String::from("selfdual_coupling"),
            description: "Bidirectional feedback for R closure",
            target_primitive: "R", source_value: "𐑽", target_value: "𐑾",
        },
        ClosurePathway {
            name: alloc::string::String::from("surface_code"),
            description: "Topological surface code for Omega protection",
            target_primitive: "Ω", source_value: "𐑴", target_value: "𐑭",
        },
    ]
}

/// Diagnose closure obstructions for a material tuple.
pub fn diagnose_closure(tuple: &[&str; 12], target: &[&str; 12]) -> alloc::vec::Vec<ClosureObstruction> {
    let mut obstructions = alloc::vec::Vec::new();
    let prim_names = ["D", "T", "R", "P", "F", "K", "G", "Gm", "Phi", "H", "S", "Ω"];
    for i in 0..12 {
        if tuple[i] != target[i] {
            match prim_names[i] {
                "D" => obstructions.push(ClosureObstruction::DimensionalMismatch),
                "T" => obstructions.push(ClosureObstruction::TopologyGap),
                "R" => obstructions.push(ClosureObstruction::CouplingAsymmetry),
                "P" => obstructions.push(ClosureObstruction::ParityConflict),
                "Phi" => obstructions.push(ClosureObstruction::CriticalityBarrier),
                "H" => obstructions.push(ClosureObstruction::ChiralityMismatch),
                "Ω" => obstructions.push(ClosureObstruction::WindingProtectionGap),
                _ => {},
            }
        }
    }
    obstructions
}

/// Recommend a closure pathway based on obstructions.
pub fn recommend_pathway(obstructions: &[ClosureObstruction]) -> Option<ClosurePathway> {
    let pathways = closure_pathways();
    for obs in obstructions {
        for pw in &pathways {
            let prim = match obs {
                ClosureObstruction::WindingProtectionGap => "Ω",
                ClosureObstruction::ChiralityMismatch => "H",
                ClosureObstruction::CouplingAsymmetry => "R",
                _ => "",
            };
            if pw.target_primitive == prim { return Some(pw.clone()); }
        }
    }
    None
}

/// Run full closure diagnosis between a material and Sophick Mercury.
pub fn closure_diagnosis(tuple: &[&str; 12]) -> alloc::string::String {
    let obstructions = diagnose_closure(tuple, &SOPHICK_MERCURY);
    let mut out = alloc::format!("══ Closure Diagnosis ══\n  Obstructions: {}\n", obstructions.len());
    for obs in &obstructions {
        out.push_str(&alloc::format!("    - {:?}\n", obs));
    }
    if let Some(pw) = recommend_pathway(&obstructions) {
        out.push_str(&alloc::format!("\n  Recommended pathway: {}\n    {}\n    Promote {}: {} → {}\n",
            pw.name, pw.description, pw.target_primitive, pw.source_value, pw.target_value));
    } else if obstructions.is_empty() {
        out.push_str("\n  CLOSURE EXACT — material is at Sophick Mercury.\n");
    }
    out
}

// ── Gap Closure Module ─────────────────────────────────────────────────

/// Attempt to close the gap between a material and target.
pub struct GapClosure {
    pub source: [&'static str; 12],
    pub target: [&'static str; 12],
    pub gap_primitives: alloc::vec::Vec<usize>,
    pub closed: bool,
    pub closure_path: alloc::string::String,
}

impl GapClosure {
    /// Create a new gap closure analysis.
    pub fn new(source: [&'static str; 12], target: [&'static str; 12]) -> Self {
        let mut gaps = alloc::vec::Vec::new();
        for i in 0..12 {
            if source[i] != target[i] { gaps.push(i); }
        }
        let closed = gaps.is_empty();
        GapClosure { source, target, gap_primitives: gaps, closed,
                     closure_path: alloc::string::String::new() }
    }

    pub fn report(&self) -> alloc::string::String {
        if self.closed {
            return alloc::string::String::from("GapClosure: ALREADY CLOSED — source matches target");
        }
        let prim_names = ["D","T","R","P","F","K","G","Gm","Phi","H","S","Ω"];
        let mut out = alloc::format!("GapClosure: {} gaps remain\n", self.gap_primitives.len());
        for &idx in &self.gap_primitives {
            out.push_str(&alloc::format!("  {}: {} → {}\n",
                prim_names[idx], self.source[idx], self.target[idx]));
        }
        out
    }
}

// ── Non-Qubit Quantum Computing Paradigms ──────────────────────────────

/// A non-qubit quantum computing paradigm.
#[derive(Clone, Debug)]
pub struct NonQubitQC {
    pub name: alloc::string::String,
    pub paradigm_type: &'static str,
    pub tuple: [&'static str; 12],
    pub tier: &'static str,
    pub c_score: f64,
    pub gate1_open: bool,
    pub gate2_open: bool,
    pub operculum_status: &'static str,  // closure status
    pub closure_mechanism: &'static str,
}

impl NonQubitQC {
    pub fn summary(&self) -> alloc::string::String {
        alloc::format!("{} ({}) tier={} C={:.3} G1={} G2={} operc={}",
            self.name, self.paradigm_type, self.tier, self.c_score,
            self.gate1_open, self.gate2_open, self.operculum_status)
    }

    pub fn detailed_report(&self) -> alloc::string::String {
        let mut out = alloc::format!("══ {} ══\n", self.name);
        out.push_str(&alloc::format!("  Type: {}\n", self.paradigm_type));
        out.push_str(&alloc::format!("  Tier: {}  C-score: {:.3}\n", self.tier, self.c_score));
        out.push_str(&alloc::format!("  Gate 1 (⊙): {}  Gate 2 (K≤𐑧): {}\n",
            if self.gate1_open { "OPEN" } else { "CLOSED" },
            if self.gate2_open { "OPEN" } else { "CLOSED" }));
        out.push_str(&alloc::format!("  Operculum: {}\n", self.operculum_status));
        out.push_str(&alloc::format!("  Closure: {}\n", self.closure_mechanism));
        out.push_str("  Tuple: ⟨");
        for (i, g) in self.tuple.iter().enumerate() {
            if i > 0 { out.push_str(" · "); }
            out.push_str(g);
        }
        out.push_str("⟩\n");
        out
    }
}

/// All 8 non-qubit QC paradigms.
pub fn all_nonqubit_paradigms() -> alloc::vec::Vec<NonQubitQC> {
    alloc::vec![
        NonQubitQC {
            name: alloc::string::String::from("Continuous-Variable QC"),
            paradigm_type: "CV (Gaussian boson sampling)",
            tuple: ["𐑼", "𐑶", "𐑩", "𐑬", "𐑐", "𐑘", "𐑲", "𐑠", "𐑮", "𐑒", "𐑕", "𐑷"],
            tier: "O_1", c_score: 0.15, gate1_open: false, gate2_open: false,
            operculum_status: "OPEN — needs dual-rail encoding",
            closure_mechanism: "Dual-rail encoding + GKP states",
        },
        NonQubitQC {
            name: alloc::string::String::from("Measurement-Based QC"),
            paradigm_type: "MBQC (cluster state)",
            tuple: ["𐑨", "𐑡", "𐑽", "𐑯", "𐑐", "𐑧", "𐑲", "𐑵", "𐑢", "𐑒", "𐑕", "𐑷"],
            tier: "O_1", c_score: 0.22, gate1_open: false, gate2_open: true,
            operculum_status: "OPEN — needs precompiled measurement pattern",
            closure_mechanism: "Precompiled measurement pattern",
        },
        NonQubitQC {
            name: alloc::string::String::from("Topological QC"),
            paradigm_type: "Anyonic braiding",
            tuple: ["𐑦", "𐑸", "𐑾", "𐑹", "𐑐", "𐑧", "𐑲", "𐑠", "⊙", "𐑫", "𐑕", "𐑟"],
            tier: "O_∞", c_score: 0.95, gate1_open: true, gate2_open: true,
            operculum_status: "NATIVELY CLOSED",
            closure_mechanism: "Native — anyon braiding is μ∘δ=id",
        },
        NonQubitQC {
            name: alloc::string::String::from("Adiabatic QC"),
            paradigm_type: "Quantum annealing",
            tuple: ["𐑼", "𐑥", "𐑽", "𐑬", "𐑞", "𐑧", "𐑲", "𐑠", "𐑢", "𐑒", "𐑕", "𐑷"],
            tier: "O_0", c_score: 0.08, gate1_open: false, gate2_open: true,
            operculum_status: "OPEN — needs counterdiabatic driving",
            closure_mechanism: "Counterdiabatic CD driving",
        },
        NonQubitQC {
            name: alloc::string::String::from("Boson Sampling"),
            paradigm_type: "Linear optical",
            tuple: ["𐑼", "𐑡", "𐑩", "𐑗", "𐑐", "𐑘", "𐑲", "𐑝", "𐑢", "𐑓", "𐑕", "𐑷"],
            tier: "O_0", c_score: 0.0, gate1_open: false, gate2_open: false,
            operculum_status: "INTRACTABLY OPEN",
            closure_mechanism: "None known — no error correction",
        },
        NonQubitQC {
            name: alloc::string::String::from("Quantum Walks"),
            paradigm_type: "Discrete/continuous time",
            tuple: ["𐑨", "𐑡", "𐑩", "𐑗", "𐑐", "𐑘", "𐑚", "𐑜", "𐑢", "𐑓", "𐑙", "𐑷"],
            tier: "O_0", c_score: 0.05, gate1_open: false, gate2_open: false,
            operculum_status: "OPEN — needs Floquet engineering",
            closure_mechanism: "Floquet periodic driving",
        },
        NonQubitQC {
            name: alloc::string::String::from("Coherent Ising Machine"),
            paradigm_type: "Optical parametric oscillator",
            tuple: ["𐑼", "𐑡", "𐑩", "𐑬", "𐑱", "𐑤", "𐑲", "𐑝", "𐑢", "𐑒", "𐑕", "𐑷"],
            tier: "O_0", c_score: 0.10, gate1_open: false, gate2_open: false,
            operculum_status: "OPEN — needs self-dual coupling",
            closure_mechanism: "Self-dual injection locking",
        },
        NonQubitQC {
            name: alloc::string::String::from("Quantum Reservoir Computing"),
            paradigm_type: "NISQ reservoir dynamics",
            tuple: ["𐑼", "𐑡", "𐑩", "𐑗", "𐑞", "𐑘", "𐑲", "𐑜", "𐑢", "𐑓", "𐑕", "𐑷"],
            tier: "O_0", c_score: 0.0, gate1_open: false, gate2_open: false,
            operculum_status: "INTRACTABLY OPEN",
            closure_mechanism: "None known — intrinsic dissipation",
        },
    ]
}

/// Operculum analysis for a paradigm.
pub fn operculum_analysis(p: &NonQubitQC) -> alloc::string::String {
    if p.operculum_status.contains("CLOSED") || p.operculum_status.contains("NATIVELY") {
        alloc::format!("{}: {} — FROBENIUS CLOSED. μ∘δ=id holds.", p.name, p.operculum_status)
    } else if p.operculum_status.contains("INTRACTABLY") {
        alloc::format!("{}: {} — No known closure mechanism.", p.name, p.operculum_status)
    } else {
        alloc::format!("{}: {} — Possible via: {}", p.name, p.operculum_status, p.closure_mechanism)
    }
}

/// Full non-qubit QC paradigm summary table.
pub fn paradigm_summary_table() -> alloc::string::String {
    let paradigms = all_nonqubit_paradigms();
    let mut out = alloc::string::String::from("══ Non-Qubit QC Paradigms ══\n\n");
    out.push_str(&alloc::format!("  {{'Name':<30}} {{'Tier':<6}} {{'C':<6}} {{'G1':<5}} {{'G2':<5}} {{'Operculum'}}\n"));
    out.push_str("  ─────────────────────────────────────────────────────────────\n");
    for p in &paradigms {
        out.push_str(&alloc::format!("  {:<30} {:<6} {:<6.3} {:<5} {:<5} {}\n",
            p.name, p.tier, p.c_score,
            if p.gate1_open { "OPEN" } else { "CLSD" },
            if p.gate2_open { "OPEN" } else { "CLSD" },
            p.operculum_status));
    }
    out.push_str("\n  Frobenius-closed: Topological QC only (native anyon braiding)\n");
    out
}
