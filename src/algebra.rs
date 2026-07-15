#![allow(dead_code)]
// algebra.rs — IG Lattice Algebra: distance, meet, join, tensor
//
// ALL ordinal tables, weights, and operations now delegate to catalog.rs.
// No hardcoded ordinal arrays or weight constants remain here.
//
// Operations:
//   primitive_mismatches(a, b) -> u8     Hamming distance [0,12]
//   tuple_distance(a, b) -> f32          Weighted quasi-metric
//   meet(a, b) -> LatticeResult          Greatest lower bound
//   join(a, b) -> LatticeResult          Least upper bound
//   tensor(a, b) -> IgTuple              Composite: max on union, min on P/F

use crate::imas_ig::{IgPrim, IgTuple};
use crate::catalog;

// ─── Hamming distance ─────────────────────────────────────────────────────

/// Canonical Hamming distance over all 12 primitives.
/// Returns [0, 12]. Zero iff the two imscriptions are identical as 12-tuples.
pub fn primitive_mismatches(a: &IgTuple, b: &IgTuple) -> u8 {
    let mut d: u8 = 0;
    if a.d != b.d { d += 1; }
    if a.t != b.t { d += 1; }
    if a.r != b.r { d += 1; }
    if a.p != b.p { d += 1; }
    if a.f != b.f { d += 1; }
    if a.k != b.k { d += 1; }
    if a.g != b.g { d += 1; }
    if a.c != b.c { d += 1; }
    if a.phi != b.phi { d += 1; }
    if a.h != b.h { d += 1; }
    if a.s != b.s { d += 1; }
    if a.omega != b.omega { d += 1; }
    d
}

// ─── Weighted distance ────────────────────────────────────────────────────

/// Weighted quasi-metric between two IgTuples.
/// Weights sourced from catalog::distance_weights() — dynamically configurable.
/// Ordinal gaps for F, K, G, Omega, H; binary mismatch for categorical D, T, R, P, C, Phi, S.
pub fn tuple_distance(a: &IgTuple, b: &IgTuple) -> f32 {
    let w = catalog::distance_weights().as_array();
    let mut d: f32 = 0.0;

    // Categorical — binary mismatch
    d += w[0] * (a.d != b.d) as u8 as f32;
    d += w[1] * (a.t != b.t) as u8 as f32;
    d += w[2] * (a.r != b.r) as u8 as f32;
    d += w[3] * (a.p != b.p) as u8 as f32;
    d += w[7] * (a.c != b.c) as u8 as f32;
    d += w[8] * (a.phi != b.phi) as u8 as f32;
    d += w[10] * (a.s != b.s) as u8 as f32;

    // Ordinal gaps — using catalog ordinal tables
    d += w[4] * catalog::ord_gap(a.f, b.f, &catalog::F_ORD) as f32;
    d += w[5] * catalog::ord_gap(a.k, b.k, &catalog::K_ORD) as f32;
    d += w[6] * catalog::ord_gap(a.g, b.g, &catalog::G_ORD) as f32;
    d += w[9] * catalog::ord_gap(a.omega, b.omega, &catalog::OMEGA_ORD) as f32;
    d += w[11] * catalog::ord_gap(a.h, b.h, &catalog::H_ORD) as f32;

    d
}

// ─── Lattice result ───────────────────────────────────────────────────────

/// Result of meet or join. CONFLICT on categorical disagreement.
#[derive(Clone, Debug)]
pub struct LatticeResult {
    pub op: &'static str,          // "meet" or "join"
    pub tuple: IgTuple,
    pub conflicts: [bool; 12],     // per-primitive conflict flags
    pub notes: [u8; 8],            // note codes (max 8)
    pub note_count: u8,
}

impl LatticeResult {
    pub fn is_valid(&self) -> bool {
        !self.conflicts.iter().any(|&c| c)
    }
}

// ─── Meet ──────────────────────────────────────────────────────────────────

/// Greatest lower bound of two tuples.
/// Ordered primitives (F,K,G,Omega,H): min over ordinal.
/// Categorical primitives (D,T,R,P,C,Phi,S): exact match required, else CONFLICT.
/// ⊙ (⊙) is absorbing: any meet involving ⊙ yields ⊙.
pub fn meet(a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(a, b, true)
}

/// Least upper bound of two tuples.
/// Ordered primitives (F,K,G,Omega,H): max over ordinal.
/// Categorical primitives: exact match required, else CONFLICT.
/// ⊙ (⊙) is absorbing under join as well.
pub fn join(a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(a, b, false)
}

fn lattice_op(a: &IgTuple, b: &IgTuple, is_meet: bool) -> LatticeResult {
    let op_name = if is_meet { "meet" } else { "join" };

    // ⊙ absorption: ⊙ is absorbing under both meet and join
    let phi = if a.phi == IgPrim::⊙ || b.phi == IgPrim::⊙ {
        IgPrim::⊙
    } else if is_meet {
        catalog::ord_min(a.phi, b.phi, &catalog::PHI_ORD)
    } else {
        catalog::ord_max(a.phi, b.phi, &catalog::PHI_ORD)
    };

    let pick_cat = |v1: IgPrim, v2: IgPrim| -> (IgPrim, bool) {
        if v1 == v2 { (v1, false) } else { (v1, true) }
    };
    let pick_ord = |v1: IgPrim, v2: IgPrim, arr: &[IgPrim]| -> IgPrim {
        if is_meet { catalog::ord_min(v1, v2, arr) } else { catalog::ord_max(v1, v2, arr) }
    };

    let (d, dc) = pick_cat(a.d, b.d);
    let (t, tc) = pick_cat(a.t, b.t);
    let (r, rc) = pick_cat(a.r, b.r);
    let (p, pc) = pick_cat(a.p, b.p);
    let (c, cc) = pick_cat(a.c, b.c);
    let (s, sc) = pick_cat(a.s, b.s);

    let f = pick_ord(a.f, b.f, &catalog::F_ORD);
    let k = pick_ord(a.k, b.k, &catalog::K_ORD);
    let g = pick_ord(a.g, b.g, &catalog::G_ORD);
    let omega = pick_ord(a.omega, b.omega, &catalog::OMEGA_ORD);
    let h = pick_ord(a.h, b.h, &catalog::H_ORD);

    let mut conflicts = [false; 12];
    conflicts[0] = dc; conflicts[1] = tc; conflicts[2] = rc;
    conflicts[3] = pc; conflicts[5] = false;
    conflicts[6] = false;
    conflicts[7] = cc; conflicts[8] = false;
    conflicts[9] = false;
    conflicts[10] = sc; conflicts[11] = false;

    LatticeResult {
        op: op_name,
        tuple: IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega },
        conflicts,
        notes: [0u8; 8],
        note_count: 0,
    }
}

// ─── Tensor product ────────────────────────────────────────────────────────

/// Tensor (composite) product: max on union primitives, min on P and F.
/// Represents coupling two systems together.
/// The 𐑻 absorption rule: tensor(⊙, EP) = EP.
pub fn tensor(a: &IgTuple, b: &IgTuple) -> IgTuple {
    // P and F: min (bottleneck — the weaker link dominates)
    let p = catalog::ord_min(a.p, b.p, &catalog::P_ORD);
    let f = catalog::ord_min(a.f, b.f, &catalog::F_ORD);

    // D, K, Omega, H: max
    let d = catalog::ord_max(a.d, b.d, &catalog::D_ORD);
    let k = catalog::ord_max(a.k, b.k, &catalog::K_ORD);
    let omega = catalog::ord_max(a.omega, b.omega, &catalog::OMEGA_ORD);
    let h = catalog::ord_max(a.h, b.h, &catalog::H_ORD);

    // G: max (union of interaction ranges)
    let g = catalog::ord_max(a.g, b.g, &catalog::G_ORD);

    // Phi: ⊙ absorption rule — tensor(⊙, EP) = EP
    let phi = if a.phi == IgPrim::Phi_ep || b.phi == IgPrim::Phi_ep {
        IgPrim::Phi_ep
    } else if a.phi == IgPrim::⊙ || b.phi == IgPrim::⊙ {
        IgPrim::⊙
    } else {
        catalog::ord_max(a.phi, b.phi, &catalog::PHI_ORD)
    };

    // Categorical: prefer the more structured
    let t = if a.t == b.t { a.t } else { catalog::ord_max(a.t, b.t, &catalog::T_ORD) };
    let r = if a.r == b.r { a.r } else { catalog::ord_max(a.r, b.r, &catalog::R_ORD) };
    let c = if a.c == b.c { a.c } else { catalog::ord_max(a.c, b.c, &catalog::C_ORD) };
    let s = if a.s == b.s { a.s } else { catalog::ord_max(a.s, b.s, &catalog::S_ORD) };

    IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega }
}

// ─── Display helpers ───────────────────────────────────────────────────────

use core::fmt;

impl IgTuple {
    /// Display as ⟨d·t·r·p·f·k·g·c·phi·h·s·omega⟩ using Shavian glyphs.
    pub fn display_shavian(&self) -> ShavianDisplay<'_> {
        ShavianDisplay { tuple: self }
    }
}

pub struct ShavianDisplay<'a> {
    tuple: &'a IgTuple,
}

impl<'a> fmt::Display for ShavianDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = self.tuple;
        write!(f, "\u{27e8}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{b7}{}\u{27e9}",
            catalog::primitive_glyph(t.d), catalog::primitive_glyph(t.t),
            catalog::primitive_glyph(t.r), catalog::primitive_glyph(t.p),
            catalog::primitive_glyph(t.f), catalog::primitive_glyph(t.k),
            catalog::primitive_glyph(t.g), catalog::primitive_glyph(t.c),
            catalog::primitive_glyph(t.phi), catalog::primitive_glyph(t.h),
            catalog::primitive_glyph(t.s), catalog::primitive_glyph(t.omega))
    }
}

impl fmt::Display for LatticeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.op, self.tuple.display_shavian())?;
        if !self.is_valid() {
            write!(f, " [CONFLICTS:")?;
            let names = ["D","T","R","P","F","K","C","Φ","Ω","S","H"];
            for i in 0..12 {
                if self.conflicts[i] {
                    write!(f, " {}", names[i])?;
                }
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tuple(d: IgPrim, t: IgPrim, r: IgPrim, p: IgPrim,
                  f: IgPrim, k: IgPrim, g: IgPrim, c: IgPrim,
                  phi: IgPrim, h: IgPrim, s: IgPrim, omega: IgPrim) -> IgTuple {
        IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega }
    }

    fn oinf() -> IgTuple { catalog::o_inf_tuple() }
    fn o0() -> IgTuple { catalog::o_0_tuple() }

    #[test]
    fn test_primitive_mismatches_self() {
        let a = oinf();
        assert_eq!(primitive_mismatches(&a, &a), 0);
    }

    #[test]
    fn test_primitive_mismatches_max() {
        let a = oinf();
        let b = o0();
        assert_eq!(primitive_mismatches(&a, &b), 12);
    }

    #[test]
    fn test_weighted_distance() {
        let a = oinf();
        let b = o0();
        let d = tuple_distance(&a, &b);
        assert!(d > 10.0);
    }

    #[test]
    fn test_meet_self() {
        let a = oinf();
        let r = meet(&a, &a);
        assert!(r.is_valid());
        assert_eq!(r.tuple, a);
    }

    #[test]
    fn test_meet_conflict() {
        let a = oinf();
        let b = o0();
        let r = meet(&a, &b);
        assert!(!r.is_valid());
    }

    #[test]
    fn test_tensor() {
        let a = oinf();
        let b = o0();
        let t = tensor(&a, &b);
        assert_eq!(t.p, IgPrim::P_asym);
        assert_eq!(t.f, IgPrim::F_ell);
        assert_eq!(t.d, IgPrim::D_odot);
    }

    #[test]
    fn test_tensor_phi_absorption() {
        let mut ep = oinf();
        ep.phi = IgPrim::Phi_ep;
        let o = oinf();
        let t = tensor(&o, &ep);
        assert_eq!(t.phi, IgPrim::Phi_ep);
    }
}
