// algebra.rs — IG Lattice Algebra: distance, meet, join, tensor
//
// Ported from imscribing_grammar/imscrbgrmr/algebra.py
// Implements the full IG lattice over the 12-primitive product space.
//
// Operations:
//   primitive_mismatches(a, b) -> u8     Hamming distance [0,12]
//   tuple_distance(a, b) -> f32          Weighted quasi-metric
//   meet(a, b) -> LatticeResult          Greatest lower bound
//   join(a, b) -> LatticeResult          Least upper bound
//   tensor(a, b) -> IgTuple              Composite: max on union, min on P/F
//
// Ordinal conventions (aligned with algebra.py and Lean Core.lean):
//   F: ell(0) < eth(1) < hbar(2)
//   K: fast(0) < mod(1) < slow(2) < trap(3) < mbl(4)
//   G: aleph(0) < beth(1) < gimel(2)   [aleph=finest, gimel=coarsest]
//   Omega: 0(0) < z2(1) < z(2) < na(3)
//   H: h0(0) < h1(1) < h2(2) < inf(3)

use crate::imas_ig::{IgPrim, IgTuple};

// ─── Ordinal tables ───────────────────────────────────────────────────────

const F_ORD: [IgPrim; 3] = [IgPrim::F_ell, IgPrim::F_eth, IgPrim::F_hbar];
const K_ORD: [IgPrim; 5] = [IgPrim::K_fast, IgPrim::K_mod, IgPrim::K_slow, IgPrim::K_trap, IgPrim::K_mbl];
const G_ORD: [IgPrim; 3] = [IgPrim::G_aleph, IgPrim::G_beth, IgPrim::G_gimel];
const OMEGA_ORD: [IgPrim; 4] = [IgPrim::Omega_0, IgPrim::Omega_z2, IgPrim::Omega_z, IgPrim::Omega_na];
const H_ORD: [IgPrim; 4] = [IgPrim::H0, IgPrim::H1, IgPrim::H2, IgPrim::H_inf];
fn ord_index(arr: &[IgPrim], val: IgPrim) -> Option<usize> {
    arr.iter().position(|&x| x == val)
}

fn ord_min(a: IgPrim, b: IgPrim, arr: &[IgPrim]) -> IgPrim {
    let ia = ord_index(arr, a).unwrap_or(0);
    let ib = ord_index(arr, b).unwrap_or(0);
    arr[if ia < ib { ia } else { ib }]
}

fn ord_max(a: IgPrim, b: IgPrim, arr: &[IgPrim]) -> IgPrim {
    let ia = ord_index(arr, a).unwrap_or(0);
    let ib = ord_index(arr, b).unwrap_or(0);
    arr[if ia > ib { ia } else { ib }]
}

fn ord_gap(a: IgPrim, b: IgPrim, arr: &[IgPrim]) -> i32 {
    let ia = ord_index(arr, a).unwrap_or(0) as i32;
    let ib = ord_index(arr, b).unwrap_or(0) as i32;
    (ib - ia).abs()
}

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

/// Default per-primitive weights for tuple_distance, matching algebra.py.
static DEFAULT_WEIGHTS: [f32; 12] = [
    2.0,  // D: dimensionality mismatch most penalised
    1.5,  // T: topology
    1.0,  // R: coupling
    0.8,  // P: parity
    0.6,  // F: fidelity (per ordinal step)
    0.5,  // K: kinetics (per ordinal step)
    0.4,  // G: granularity (per ordinal step)
    0.6,  // Gamma/C: composition
    0.3,  // Phi: criticality
    0.7,  // Omega: protection (ordinal gap × weight)
    0.5,  // S: stoichiometry
    0.4,  // H: chirality (ordinal gap × weight)
];

/// Weighted quasi-metric between two IgTuples.
/// Ordinal gaps for F, K, G, Omega, H; binary mismatch for categorical D, T, R, P, C, Phi, S.
pub fn tuple_distance(a: &IgTuple, b: &IgTuple) -> f32 {
    let w = &DEFAULT_WEIGHTS;
    let mut d: f32 = 0.0;

    // Categorical — binary mismatch
    d += w[0] * (a.d != b.d) as u8 as f32;
    d += w[1] * (a.t != b.t) as u8 as f32;
    d += w[2] * (a.r != b.r) as u8 as f32;
    d += w[3] * (a.p != b.p) as u8 as f32;
    d += w[7] * (a.c != b.c) as u8 as f32;
    d += w[8] * (a.phi != b.phi) as u8 as f32;
    d += w[10] * (a.s != b.s) as u8 as f32;

    // Ordinal gaps
    d += w[4] * ord_gap(a.f, b.f, &F_ORD) as f32;
    d += w[5] * ord_gap(a.k, b.k, &K_ORD) as f32;
    d += w[6] * ord_gap(a.g, b.g, &G_ORD) as f32;
    d += w[9] * ord_gap(a.omega, b.omega, &OMEGA_ORD) as f32;
    d += w[11] * ord_gap(a.h, b.h, &H_ORD) as f32;

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
/// Phi_c (⊙ / Phi_c) is absorbing: any meet involving ⊙ yields ⊙.
pub fn meet(a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(a, b, true)
}

/// Least upper bound of two tuples.
/// Ordered primitives (F,K,G,Omega,H): max over ordinal.
/// Categorical primitives: exact match required, else CONFLICT.
/// Phi_c (⊙ / Phi_c) is absorbing under join as well.
pub fn join(a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(a, b, false)
}

fn lattice_op(a: &IgTuple, b: &IgTuple, is_meet: bool) -> LatticeResult {
    let op_name = if is_meet { "meet" } else { "join" };
    
    // Phi_c absorption: ⊙ is absorbing under both meet and join
    let phi = if a.phi == IgPrim::Phi_c {
        b.phi  // a is critical → b absorbed? No: ⊙ absorbs others
    } else if b.phi == IgPrim::Phi_c {
        a.phi
    } else if a.phi == b.phi {
        a.phi
    } else {
        // Non-critical Phi: use min/max in Phi ordinal
        // Phi ordinal: sub < c < c_complex < ep < super
        // For meet: min, for join: max
        IgPrim::Phi_sub // placeholder — will resolve below
    };

    // Actually handle Phi properly:
    let phi_ord: [IgPrim; 5] = [IgPrim::Phi_sub, IgPrim::Phi_c, IgPrim::Phi_c_complex, IgPrim::Phi_ep, IgPrim::Phi_super];
    let phi = if a.phi == IgPrim::Phi_c || b.phi == IgPrim::Phi_c {
        IgPrim::Phi_c  // absorbing
    } else if is_meet {
        ord_min(a.phi, b.phi, &phi_ord)
    } else {
        ord_max(a.phi, b.phi, &phi_ord)
    };
    let pick_cat = |v1: IgPrim, v2: IgPrim| -> (IgPrim, bool) {
        if v1 == v2 { (v1, false) } else { (v1, true) }
    };
    let pick_ord = |v1: IgPrim, v2: IgPrim, arr: &[IgPrim]| -> IgPrim {
        if is_meet { ord_min(v1, v2, arr) } else { ord_max(v1, v2, arr) }
    };

    let (d, dc) = pick_cat(a.d, b.d);
    let (t, tc) = pick_cat(a.t, b.t);
    let (r, rc) = pick_cat(a.r, b.r);
    let (p, pc) = pick_cat(a.p, b.p);
    let (c, cc) = pick_cat(a.c, b.c);
    let (s, sc) = pick_cat(a.s, b.s);

    let f = pick_ord(a.f, b.f, &F_ORD);
    let k = pick_ord(a.k, b.k, &K_ORD);
    let g = pick_ord(a.g, b.g, &G_ORD);
    let omega = pick_ord(a.omega, b.omega, &OMEGA_ORD);
    let h = pick_ord(a.h, b.h, &H_ORD);

    let mut conflicts = [false; 12];
    conflicts[0] = dc; conflicts[1] = tc; conflicts[2] = rc;
    conflicts[3] = pc; conflicts[5] = false; // F always resolves
    conflicts[6] = false; // K always resolves
    conflicts[7] = cc; conflicts[8] = false; // Phi resolved above
    conflicts[9] = false; // Omega always resolves
    conflicts[10] = sc; conflicts[11] = false; // H always resolves

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
///   D: max ordinal    T: categorical match or CONFLICT-like
///   R: max ordinal    P: min ordinal (bottleneck)
///   F: min ordinal    K: max ordinal
///   G: max ordinal    C: categorical match
///   Phi: ⊙ absorption (tensor with EP → EP)
///   H: max ordinal    S: max ordinal    Omega: max ordinal
///
/// The ⊙_3 absorption rule: tensor(⊙, EP) = EP.
/// Coupling a self-modeling system to a measurement apparatus selects the tensor.
pub fn tensor(a: &IgTuple, b: &IgTuple) -> IgTuple {
    // P and F: min (bottleneck — the weaker link dominates)
    let p = ord_min(a.p, b.p, &[IgPrim::P_asym, IgPrim::P_psi, IgPrim::P_pm, IgPrim::P_sym, IgPrim::P_pmsym]);
    let f = ord_min(a.f, b.f, &F_ORD);

    // D, K, Omega, H: max
    let d = ord_max(a.d, b.d, &[IgPrim::D_wedge, IgPrim::D_triangle, IgPrim::D_infty, IgPrim::D_odot]);
    let k = ord_max(a.k, b.k, &K_ORD);
    let omega = ord_max(a.omega, b.omega, &OMEGA_ORD);
    let h = ord_max(a.h, b.h, &H_ORD);

    // G: max (union of interaction ranges)
    let g = ord_max(a.g, b.g, &G_ORD);

    // Phi: ⊙ absorption rule — tensor(⊙, EP) = EP
    let phi = if a.phi == IgPrim::Phi_ep || b.phi == IgPrim::Phi_ep {
        IgPrim::Phi_ep
    } else if a.phi == IgPrim::Phi_c || b.phi == IgPrim::Phi_c {
        IgPrim::Phi_c
    } else {
        ord_max(a.phi, b.phi, &[IgPrim::Phi_sub, IgPrim::Phi_c, IgPrim::Phi_c_complex, IgPrim::Phi_ep, IgPrim::Phi_super])
    };

    // Categorical: prefer the more structured
    let t = if a.t == b.t { a.t } else { ord_max(a.t, b.t, &[IgPrim::T_net, IgPrim::T_in, IgPrim::T_bowtie, IgPrim::T_boxtimes, IgPrim::T_odot]) };
    let r = if a.r == b.r { a.r } else { ord_max(a.r, b.r, &[IgPrim::R_super, IgPrim::R_cat, IgPrim::R_dagger, IgPrim::R_lr]) };
    let c = if a.c == b.c { a.c } else { ord_max(a.c, b.c, &[IgPrim::C_and, IgPrim::C_or, IgPrim::C_seq, IgPrim::C_broad]) };
    let s = if a.s == b.s { a.s } else { ord_max(a.s, b.s, &[IgPrim::S_11, IgPrim::S_nn, IgPrim::S_nm]) };

    IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega }
}
// ─── Display helpers ───────────────────────────────────────────────────────

use core::fmt;

impl IgTuple {
    /// Display as ⟨d·t·r·p·f·k·g·c·phi·h·s·omega⟩ using Shavian glyphs.
    pub fn display_shavian(&self) -> ShavianDisplay {
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
            t.d.glyph(), t.t.glyph(), t.r.glyph(), t.p.glyph(),
            t.f.glyph(), t.k.glyph(), t.g.glyph(), t.c.glyph(),
            t.phi.glyph(), t.h.glyph(), t.s.glyph(), t.omega.glyph())
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
    // O_∞ tuple: ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩
    fn oinf() -> IgTuple {
        make_tuple(IgPrim::D_odot, IgPrim::T_odot, IgPrim::R_lr,
                   IgPrim::P_pmsym, IgPrim::F_hbar, IgPrim::K_slow,
                   IgPrim::G_aleph, IgPrim::C_seq,
                   IgPrim::Phi_c, IgPrim::H_inf, IgPrim::S_nm, IgPrim::Omega_z)
    }

    // O₀ tuple: ⟨𐑛·𐑡·𐑩·𐑗·𐑱·𐑘·𐑚·𐑝·𐑢·𐑓·𐑙·𐑷⟩
    fn o0() -> IgTuple {
        make_tuple(IgPrim::D_wedge, IgPrim::T_net, IgPrim::R_super,
                   IgPrim::P_asym, IgPrim::F_ell, IgPrim::K_fast,
                   IgPrim::G_beth, IgPrim::C_and,
                   IgPrim::Phi_sub, IgPrim::H0, IgPrim::S_11, IgPrim::Omega_0)
    }

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
        // 12 categorical mismatches + ordinal gaps = substantial distance
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
        // Categorical primitives differ → conflicts
        assert!(!r.is_valid());
    }

    #[test]
    fn test_tensor() {
        let a = oinf();
        let b = o0();
        let t = tensor(&a, &b);
        // Tensor should resolve: P bottleneck = P_asym (weaker)
        assert_eq!(t.p, IgPrim::P_asym);
        // F bottleneck = F_ell (weaker)
        assert_eq!(t.f, IgPrim::F_ell);
        // D max = D_odot
        assert_eq!(t.d, IgPrim::D_odot);
    }

    #[test]
    fn test_tensor_phi_absorption() {
        // EP system: Phi_ep
        let mut ep = oinf();
        ep.phi = IgPrim::Phi_ep;
        let o = oinf();
        let t = tensor(&o, &ep);
        assert_eq!(t.phi, IgPrim::Phi_ep); // ⊙ absorbed by EP
    }
}
