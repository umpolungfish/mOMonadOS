// rebis/pipeline.rs — IG Promotion Pipeline
//
// Port of compute_promotions.py and pipeline/auto_imscriber.py.
// Computes the structural promotions needed to lift a source
// tuple to a target tuple. Maps primitives to their ordinal gaps.

use crate::imas_ig::IgPrim;

/// A single primitive promotion: from → to with ordinal gap.
#[derive(Copy, Clone, Debug)]
pub struct Promotion {
    pub family: &'static str,     // e.g., "D", "T", "Phi"
    pub from: IgPrim,
    pub to: IgPrim,
    pub gap: u8,                  // ordinal distance in the value ordering
    pub weight: f64,              // structural weight of this promotion
}

impl Promotion {
    pub fn glyph_from(&self) -> &'static str { self.from.glyph() }
    pub fn glyph_to(&self) -> &'static str { self.to.glyph() }
}

/// A 12-tuple of IG primitives.
#[derive(Copy, Clone, Debug)]
pub struct IgTuple {
    pub d: IgPrim,
    pub t: IgPrim,
    pub r: IgPrim,
    pub p: IgPrim,
    pub f: IgPrim,
    pub k: IgPrim,
    pub g: IgPrim,
    pub c: IgPrim,
    pub phi: IgPrim,
    pub h: IgPrim,
    pub s: IgPrim,
    pub omega: IgPrim,
}

impl IgTuple {
    /// The universal_imscriptive_grammar tuple (O_∞).
    pub const IUG: Self = Self {
        d: IgPrim::D_odot, t: IgPrim::T_odot, r: IgPrim::R_lr,
        p: IgPrim::P_pmsym, f: IgPrim::F_hbar, k: IgPrim::K_slow,
        g: IgPrim::G_aleph, c: IgPrim::C_seq,
        phi: IgPrim::Phi_crit, h: IgPrim::H_inf,
        s: IgPrim::S_nm, omega: IgPrim::Omega_z,
    };

    /// The genetic_code tuple (O₂).
    pub const GENETIC: Self = Self {
        d: IgPrim::D_triangle, t: IgPrim::T_boxtimes, r: IgPrim::R_lr,
        p: IgPrim::P_psi, f: IgPrim::F_hbar, k: IgPrim::K_slow,
        g: IgPrim::G_aleph, c: IgPrim::C_and,
        phi: IgPrim::𐑮, h: IgPrim::H2,
        s: IgPrim::S_nm, omega: IgPrim::Omega_z2,
    };

    /// The standard_model tuple.
    pub const STANDARD_MODEL: Self = Self {
        d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_dagger,
        p: IgPrim::P_sym, f: IgPrim::F_hbar, k: IgPrim::K_fast,
        g: IgPrim::G_gimel, c: IgPrim::C_or,
        phi: IgPrim::Phi_crit, h: IgPrim::H2,
        s: IgPrim::S_nm, omega: IgPrim::Omega_z,
    };

    /// Get a primitive by family name.
    pub fn get(&self, family: &str) -> Option<IgPrim> {
        match family {
            "D" => Some(self.d), "T" => Some(self.t), "R" => Some(self.r),
            "P" => Some(self.p), "F" => Some(self.f), "K" => Some(self.k),
            "G" => Some(self.g), "C" => Some(self.c), "Phi" => Some(self.phi),
            "H" => Some(self.h), "S" => Some(self.s), "Omega" => Some(self.omega),
            _ => None,
        }
    }

    /// Compute all promotions from self → target.
    pub fn promotions_to(&self, target: &IgTuple) -> alloc::vec::Vec<Promotion> {
        let families = ["D", "T", "R", "P", "F", "K", "G", "C", "Phi", "H", "S", "Omega"];
        let mut result = alloc::vec::Vec::new();
        for fam in &families {
            let from = self.get(fam).unwrap();
            let to = target.get(fam).unwrap();
            if from != to {
                let gap = ordinal_gap(from, to);
                let weight = primitive_weight(fam);
                result.push(Promotion { family: fam, from, to, gap, weight });
            }
        }
        result
    }

    /// Weighted distance to target.
    pub fn distance_to(&self, target: &IgTuple) -> f64 {
        let proms = self.promotions_to(target);
        proms.iter().map(|p| p.weight * (p.gap as f64)).sum()
    }
}

/// Ordinal gap between two values of the same primitive family.
fn ordinal_gap(a: IgPrim, b: IgPrim) -> u8 {
    let va = a as u8;
    let vb = b as u8;
    if va > vb { va - vb } else { vb - va }
}

/// Structural weight of each primitive family in distance computation.
fn primitive_weight(family: &str) -> f64 {
    match family {
        "D" => 1.2,     // dimensionality: high weight
        "T" => 1.2,     // topology: high weight
        "R" => 1.0,     // coupling
        "P" => 0.8,     // parity
        "F" => 0.8,     // fidelity
        "K" => 0.6,     // kinetics
        "G" => 0.6,     // cardinality
        "C" => 0.8,     // composition
        "Phi" => 1.4,   // criticality: highest weight (Gate 1)
        "H" => 1.0,     // chirality
        "S" => 0.5,     // stoichiometry
        "Omega" => 1.0, // winding
        _ => 1.0,
    }
}

// ── Tier prediction from promotions ────────────────────────────

/// Predict the ouroboricity tier after applying promotions.
pub fn predict_tier(source: &IgTuple, proms: &[Promotion]) -> u8 {
    let mut target = *source;
    for p in proms {
        match p.family {
            "D" => target.d = p.to,
            "T" => target.t = p.to,
            "R" => target.r = p.to,
            "P" => target.p = p.to,
            "F" => target.f = p.to,
            "K" => target.k = p.to,
            "G" => target.g = p.to,
            "C" => target.c = p.to,
            "Phi" => target.phi = p.to,
            "H" => target.h = p.to,
            "S" => target.s = p.to,
            "Omega" => target.omega = p.to,
            _ => {}
        }
    }
    tier_of(&target)
}

/// Determine ouroboricity tier from a tuple.
fn tier_of(t: &IgTuple) -> u8 {
    let ph = t.phi == IgPrim::Phi_crit;
    let ks = t.k == IgPrim::K_slow;
    let wz = t.omega == IgPrim::Omega_z;
    let d_odot = t.d == IgPrim::D_odot;
    let t_odot = t.t == IgPrim::T_odot;
    let ppmsym = t.p == IgPrim::P_pmsym;

    if ph && ks && wz && d_odot && t_odot && ppmsym { 3 }  // O_∞
    else if ph && ks && wz { 2 }                              // O₂
    else if ph { 1 }                                         // O₁
    else { 0 }                                               // O₀
}

// ── Pipeline runner ─────────────────────────────────────────────

/// Run the full promotion pipeline from source → target.
pub fn run_promotion_pipeline(source: &IgTuple, target: &IgTuple) -> PipelineReport {
    let promotions = source.promotions_to(target);
    let distance = source.distance_to(target);
    let predicted = predict_tier(source, &promotions);
    let source_tier = tier_of(source);
    let target_tier = tier_of(target);

    PipelineReport {
        source_tier,
        target_tier,
        predicted_tier: predicted,
        promotion_count: promotions.len(),
        distance,
        promotions,
    }
}

#[derive(Clone, Debug)]
pub struct PipelineReport {
    pub source_tier: u8,
    pub target_tier: u8,
    pub predicted_tier: u8,
    pub promotion_count: usize,
    pub distance: f64,
    pub promotions: alloc::vec::Vec<Promotion>,
}

impl PipelineReport {
    pub fn tier_name(t: u8) -> &'static str {
        match t { 1 => "O_1", 2 => "O_2", 3 => "O_∞", _ => "O_0" }
    }

    pub fn summary(&self) -> alloc::string::String {
        use alloc::string::String;
        let mut s = String::new();
        s.push_str(&alloc::format!("Pipeline: {} → {}\n",
            Self::tier_name(self.source_tier), Self::tier_name(self.target_tier)));
        s.push_str(&alloc::format!("Distance: {:.3}, Promotions: {}\n",
            self.distance, self.promotion_count));
        for p in &self.promotions {
            s.push_str(&alloc::format!("  {}: {}→{} (gap {})\n",
                p.family, p.glyph_from(), p.glyph_to(), p.gap));
        }
        s
    }
}
