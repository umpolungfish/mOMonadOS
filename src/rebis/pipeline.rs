// rebis/pipeline.rs — IG Promotion Pipeline
//
// Port of compute_promotions.py and pipeline/auto_imscriber.py.
// Computes the structural promotions needed to lift a source
// tuple to a target tuple. Maps primitives to their ordinal gaps.

use crate::rebis::RebisPrim;

/// A single primitive promotion: from → to with ordinal gap.
#[derive(Copy, Clone, Debug)]
pub struct Promotion {
    pub family: &'static str,     // e.g., "D", "T", "Phi"
    pub from: RebisPrim,
    pub to: RebisPrim,
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
    pub d: RebisPrim,
    pub t: RebisPrim,
    pub r: RebisPrim,
    pub p: RebisPrim,
    pub f: RebisPrim,
    pub k: RebisPrim,
    pub g: RebisPrim,
    pub c: RebisPrim,
    pub phi: RebisPrim,
    pub h: RebisPrim,
    pub s: RebisPrim,
    pub omega: RebisPrim,
}

impl IgTuple {
    /// The universal_imscriptive_grammar tuple (O_∞).
    pub const IUG: Self = Self {
        d: RebisPrim::D_odot, t: RebisPrim::T_odot, r: RebisPrim::R_lr,
        p: RebisPrim::P_pmsym, f: RebisPrim::F_hbar, k: RebisPrim::K_slow,
        g: RebisPrim::G_aleph, c: RebisPrim::C_seq,
        phi: RebisPrim::Ph_c, h: RebisPrim::H_inf,
        s: RebisPrim::S_hetero, omega: RebisPrim::W_Z,
    };

    /// The genetic_code tuple (O₂).
    pub const GENETIC: Self = Self {
        d: RebisPrim::D_triangle, t: RebisPrim::T_boxtimes, r: RebisPrim::R_lr,
        p: RebisPrim::P_psi, f: RebisPrim::F_hbar, k: RebisPrim::K_slow,
        g: RebisPrim::G_aleph, c: RebisPrim::C_and,
        phi: RebisPrim::Ph_c_complex, h: RebisPrim::H_2,
        s: RebisPrim::S_hetero, omega: RebisPrim::W_Z2,
    };

    /// The standard_model tuple.
    pub const STANDARD_MODEL: Self = Self {
        d: RebisPrim::D_infty, t: RebisPrim::T_net, r: RebisPrim::R_dagger,
        p: RebisPrim::P_sym, f: RebisPrim::F_hbar, k: RebisPrim::K_fast,
        g: RebisPrim::G_gimel, c: RebisPrim::C_or_,
        phi: RebisPrim::Ph_c, h: RebisPrim::H_2,
        s: RebisPrim::S_hetero, omega: RebisPrim::W_Z,
    };

    /// Get a primitive by family name.
    pub fn get(&self, family: &str) -> Option<RebisPrim> {
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
fn ordinal_gap(a: RebisPrim, b: RebisPrim) -> u8 {
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
    let ph = t.phi == RebisPrim::Ph_c;
    let ks = t.k == RebisPrim::K_slow;
    let wz = t.omega == RebisPrim::W_Z;
    let d_odot = t.d == RebisPrim::D_odot;
    let t_odot = t.t == RebisPrim::T_odot;
    let ppmsym = t.p == RebisPrim::P_pmsym;

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
