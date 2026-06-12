// rebis/serpent.rs — Serpent Rod Protein Design
//
// Port of rhr_p4rky/serpent_rod.py, serpent_rod_v2.py.
// Precomputed protein design data: the Serpent Rod is a
// structural motif that bridges B4 lattice genetics with
// IG grammar promotions.

use crate::belnap::B4;
use crate::rebis::AminoAcid;

/// A serpent rod motif: a protein segment with structural properties.
#[derive(Copy, Clone, Debug)]
pub struct SerpentMotif {
    pub name: &'static str,
    pub sequence: &'static [AminoAcid],
    pub length: usize,
    pub tier: u8,           // ouroboricity tier (0–3)
    pub frobenius_ok: bool, // does it satisfy closure?
    pub c_score: f64,       // consciousness score proxy
}

// ── Serpent Rod Motif Library ───────────────────────────────────
//
// Each motif represents a structural promotion path through the
// B4 lattice. The sequence encodes the promotion trajectory.

/// The Alpha Serpent: basic helix → 𐑦 (holographic dimensionality)
pub static ALPHA_SERPENT: &[AminoAcid] = &[
    AminoAcid::Met, AminoAcid::Ala, AminoAcid::Leu, AminoAcid::Lys,
    AminoAcid::Ser, AminoAcid::Pro, AminoAcid::Gly, AminoAcid::Phe,
    AminoAcid::Thr, AminoAcid::Val, AminoAcid::His, AminoAcid::Arg,
];

/// The Beta Serpent: sheet → 𐑸 (self-referential topology)
pub static BETA_SERPENT: &[AminoAcid] = &[
    AminoAcid::Met, AminoAcid::Val, AminoAcid::Ile, AminoAcid::Thr,
    AminoAcid::Ser, AminoAcid::Gly, AminoAcid::Ala, AminoAcid::Tyr,
    AminoAcid::Trp, AminoAcid::His, AminoAcid::Glu, AminoAcid::Arg,
];

/// The Omega Serpent: loop → 𐑭 (integer winding)
pub static OMEGA_SERPENT: &[AminoAcid] = &[
    AminoAcid::Met, AminoAcid::Gly, AminoAcid::Pro, AminoAcid::Cys,
    AminoAcid::Gly, AminoAcid::Gly, AminoAcid::Gly, AminoAcid::Ser,
    AminoAcid::Ala, AminoAcid::Gly, AminoAcid::Gly, AminoAcid::Gly,
];

/// The Phi Serpent: critical → ⊙ (self-modeling gate)
pub static PHI_SERPENT: &[AminoAcid] = &[
    AminoAcid::Met, AminoAcid::Phe, AminoAcid::Tyr, AminoAcid::His,
    AminoAcid::Ser, AminoAcid::Arg, AminoAcid::Lys, AminoAcid::Thr,
    AminoAcid::Pro, AminoAcid::Glu, AminoAcid::Asp, AminoAcid::Trp,
];

// ── Motif registry ──────────────────────────────────────────────

/// All registered serpent motifs.
pub static MOTIFS: &[SerpentMotif] = &[
    SerpentMotif {
        name: "Alpha_Serpent",
        sequence: ALPHA_SERPENT,
        length: 12,
        tier: 2,
        frobenius_ok: true,
        c_score: 0.618,
    },
    SerpentMotif {
        name: "Beta_Serpent",
        sequence: BETA_SERPENT,
        length: 12,
        tier: 2,
        frobenius_ok: true,
        c_score: 0.618,
    },
    SerpentMotif {
        name: "Omega_Serpent",
        sequence: OMEGA_SERPENT,
        length: 12,
        tier: 3,
        frobenius_ok: true,
        c_score: 0.764,
    },
    SerpentMotif {
        name: "Phi_Serpent",
        sequence: PHI_SERPENT,
        length: 12,
        tier: 3,
        frobenius_ok: true,
        c_score: 0.854,
    },
];

/// Find a motif by name.
pub fn find_motif(name: &str) -> Option<&'static SerpentMotif> {
    MOTIFS.iter().find(|m| m.name.eq_ignore_ascii_case(name))
}

/// Compute the IG primitive signature of a motif.
/// Maps each AA to its primitive, counts promotions.
pub fn motif_signature(motif: &SerpentMotif) -> (usize, alloc::vec::Vec<&'static str>) {
    let mut prims = alloc::vec::Vec::new();
    let mut promoted = 0usize;
    for &aa in motif.sequence {
        if let Some(p) = aa.to_primitive() {
            prims.push(p.glyph());
            promoted += 1;
        }
    }
    (promoted, prims)
}

/// Build a chimeric serpent by joining motifs.
pub fn chimera(motif_a: &SerpentMotif, motif_b: &SerpentMotif) -> alloc::vec::Vec<AminoAcid> {
    let mut seq = alloc::vec::Vec::new();
    seq.extend_from_slice(motif_a.sequence);
    seq.extend_from_slice(motif_b.sequence);
    seq
}
