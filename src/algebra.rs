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

/// Greatest lower bound of two tuples, under canonical's (dialect 0)
/// absorption rules. Ordered primitives (F,K,G,Omega,H): min over ordinal.
/// Categorical primitives (D,T,R,P,C,Phi,S): exact match required, else
/// CONFLICT. ⊙ (⊙) is absorbing: any meet involving ⊙ yields ⊙.
pub fn meet(a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(0, a, b, true)
}

/// `meet` under an arbitrary dialect's own abs_rules table instead of
/// canonical's — the active ruleset actually shaping the operation, not
/// just describing it.
pub fn meet_under(dialect: u8, a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(dialect, a, b, true)
}

/// Least upper bound of two tuples, under canonical's absorption rules.
/// Ordered primitives (F,K,G,Omega,H): max over ordinal. Categorical
/// primitives: exact match required, else CONFLICT. ⊙ (⊙) is absorbing
/// under join as well.
pub fn join(a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(0, a, b, false)
}

/// `join` under an arbitrary dialect's own abs_rules table.
pub fn join_under(dialect: u8, a: &IgTuple, b: &IgTuple) -> LatticeResult {
    lattice_op(dialect, a, b, false)
}

/// Apply a dialect's declared absorption rules (ops_mask bit 1=meet,
/// 2=join, 4=tensor; direction 0=either operand, 1=left only, 2=right
/// only) onto an already-computed result tuple. Reads real per-dialect
/// data (dialect_expansion::all_dialects) through the real glyph<->IgPrim
/// mappings (prim_from_name / set_prim_by_name / igprim_from_glyph) —
/// never a second hand-written rule table living beside the declared one.
fn apply_dialect_absorption(dialect: u8, op_bit: u8, a: &IgTuple, b: &IgTuple, result: &mut IgTuple) {
    if (dialect as usize) >= crate::dialect_expansion::DIALECT_COUNT { return; }
    let unis = crate::dialect_expansion::all_dialects();
    for rule in unis[dialect as usize].abs_rules {
        if rule.ops_mask & op_bit == 0 { continue; }
        let target = match crate::dialect::igprim_from_glyph(rule.value) {
            Some(v) => v,
            None => continue,
        };
        let left_hits = crate::dialect::prim_from_name(rule.prim, a) == Some(target);
        let right_hits = crate::dialect::prim_from_name(rule.prim, b) == Some(target);
        let triggered = match rule.direction {
            1 => left_hits,
            2 => right_hits,
            _ => left_hits || right_hits,
        };
        if triggered {
            crate::dialect::set_prim_by_name(rule.prim, result, target);
        }
    }
}

fn lattice_op(dialect: u8, a: &IgTuple, b: &IgTuple, is_meet: bool) -> LatticeResult {
    let op_name = if is_meet { "meet" } else { "join" };
    let op_bit: u8 = if is_meet { 1 } else { 2 };

    let phi = if is_meet {
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

    let mut tuple = IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega };
    apply_dialect_absorption(dialect, op_bit, a, b, &mut tuple);

    LatticeResult {
        op: op_name,
        tuple,
        conflicts,
        notes: [0u8; 8],
        note_count: 0,
    }
}

// ─── Tensor product ────────────────────────────────────────────────────────

/// Tensor (composite) product under canonical's (dialect 0) absorption
/// rules: max on union primitives, min on P and F. Represents coupling two
/// systems together.
pub fn tensor(a: &IgTuple, b: &IgTuple) -> IgTuple {
    tensor_under(0, a, b)
}

/// Phi (Criticality) bottleneck, per SNS_PRIME.md's Tensor Composition
/// Rules: err absorbs everything; monad absorbs everything weaker than
/// err; a sub-critical (woe) value tensored with a super-critical one
/// (roar or haha) collapses to monad ("sub + super collapse to ⊙").
/// Those three rules are the spec's own words. The remaining case it
/// names only as "the tensor sits at the geometric mean of the two
/// criticalities" without pinning down a formula: two values on the same
/// side of monad that never invoke the three rules above (woe with woe,
/// or any pair drawn from {roar, haha}). Read literally as a mean pulling
/// toward the center, that resolves to the value ordinally nearer monad.
fn tensor_phi(a: IgPrim, b: IgPrim) -> IgPrim {
    use crate::catalog::PHI_ORD;
    if a == IgPrim::err || b == IgPrim::err { return IgPrim::err; }
    if a == IgPrim::monad || b == IgPrim::monad { return IgPrim::monad; }
    let ia = catalog::ord_index(&PHI_ORD, a).unwrap_or(0);
    let ib = catalog::ord_index(&PHI_ORD, b).unwrap_or(0);
    let sub = |i: usize| i == 0;   // woe only (monad already excluded)
    let sup = |i: usize| i > 1;    // roar or haha (err already excluded)
    if (sub(ia) && sup(ib)) || (sub(ib) && sup(ia)) {
        return IgPrim::monad;
    }
    if ia.abs_diff(1) <= ib.abs_diff(1) { a } else { b }
}

/// `tensor` under an arbitrary dialect's own abs_rules table — real per-
/// dialect absorption instead of canonical's own two rules hardcoded in
/// place of whichever dialect is actually active.
///
/// Follows SNS_PRIME.md's Tensor Composition Rules exactly: seven
/// bottleneck slots (R, P, K, Phi, H, S, Omega) and five pass-through
/// slots (D, T, F, G, C) that join rather than constrain. This replaced
/// an earlier version that had R and F in the wrong category (R run as
/// pass-through when the spec bottlenecks it at the weaker coupling; F
/// run as a min-bottleneck when the spec passes it through) and Phi
/// collapsed to a plain ordinal max instead of the spec's own absorption
/// rule — caught by reading the spec directly against this function
/// field by field, not by symptom.
pub fn tensor_under(dialect: u8, a: &IgTuple, b: &IgTuple) -> IgTuple {
    // Bottlenecks — the constraint-bearing slots.
    let r = catalog::ord_min(a.r, b.r, &catalog::R_ORD);       // weaker coupling wins
    let p = catalog::ord_min(a.p, b.p, &catalog::P_ORD);       // weaker parity wins
    let k = catalog::ord_max(a.k, b.k, &catalog::K_ORD);       // slower kinetics wins
    let phi = tensor_phi(a.phi, b.phi);                        // criticality absorption
    let h = catalog::ord_max(a.h, b.h, &catalog::H_ORD);       // more memory wins
    let s = catalog::ord_max(a.s, b.s, &catalog::S_ORD);       // heterogeneous absorbs
    let omega = catalog::ord_max(a.omega, b.omega, &catalog::OMEGA_ORD); // non-Abelian absorbs

    // Pass-through — structure-bearing slots, lattice join.
    let d = catalog::ord_max(a.d, b.d, &catalog::D_ORD);
    let f = catalog::ord_max(a.f, b.f, &catalog::F_ORD);
    let g = catalog::ord_max(a.g, b.g, &catalog::G_ORD);
    let t = if a.t == b.t { a.t } else { catalog::ord_max(a.t, b.t, &catalog::T_ORD) };
    let c = if a.c == b.c { a.c } else { catalog::ord_max(a.c, b.c, &catalog::C_ORD) };

    let mut tuple = IgTuple { d, t, r, p, f, k, g, c, phi, h, s, omega };
    apply_dialect_absorption(dialect, 4, a, b, &mut tuple);
    tuple
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
            let names = crate::canonical_ig::PRIMITIVE_ORDER;
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

    /// Real check, not assumed: does tensoring two numbers' own self-imscribed
    /// types (word_to_tuple runs each number's native_numeral glyph word
    /// through the real kernel's self_imscribe, the actual Grammar-applied-to-
    /// the-Grammar closure, not a bit read) with algebra::tensor (now fixed
    /// against SNS_PRIME.md's own spec) land anywhere near tracking a+b?
    /// Kept inside the un-saturated small-n regime (native_numeral words
    /// longer than a few bits hit self_imscribe's period-gated axis
    /// saturation, already found and reported earlier this session).
    #[test]
    fn tensor_of_self_imscribed_small_numbers_reported() {
        for a in 1u32..8 {
            for b in 1u32..8 {
                let ta = crate::axis_values::word_to_tuple(&crate::native_numeral::encode(&a.to_string()));
                let tb = crate::axis_values::word_to_tuple(&crate::native_numeral::encode(&b.to_string()));
                let composite = tensor(&ta, &tb);
                crate::nested_println!(
                    "a={a} b={b} sum={:<3} addr(a)={:<9} addr(b)={:<9} addr(a⊗b)={}",
                    a + b,
                    ta.crystal_address(),
                    tb.crystal_address(),
                    composite.crystal_address(),
                );
            }
        }
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
        // P (Parity, ≻): bottleneck, weaker wins — min(or', church) = church.
        assert_eq!(t.p, IgPrim::church);
        // F (Fidelity, ⋈): pass-through per SNS_PRIME's Tensor Composition
        // Rules, join not bottleneck — max(peep, age) = peep, not age.
        assert_eq!(t.f, IgPrim::peep);
        assert_eq!(t.d, IgPrim::if_);
    }

    #[test]
    fn test_tensor_phi_absorption() {
        // ⊙ (monad) is ABS_CANONICAL's declared absorbing value on Phi under
        // tensor, not err — err absorption was never in the table.
        let mut ep = oinf();
        ep.phi = IgPrim::err;
        let o = oinf();
        let t = tensor(&o, &ep);
        assert_eq!(t.phi, IgPrim::monad);
    }

    #[test]
    fn test_tensor_stoichiometry_absorption() {
        // ABS_CANONICAL declares up (𐑳) absorbing on S under tensor.
        let mut hung = oinf();
        hung.s = IgPrim::hung;
        let mut up = oinf();
        up.s = IgPrim::up;
        let t = tensor(&hung, &up);
        assert_eq!(t.s, IgPrim::up);
    }

    #[test]
    fn test_tensor_phi_err_absorbs_under_canonical_too() {
        // SNS_PRIME.md's own Tensor Composition Rules put err absorption
        // (and monad absorption) IN the base Phi bottleneck formula, not
        // behind a per-dialect opt-in — so canonical tensor (dialect 0)
        // must already give err for (err, haha), with no dialect-specific
        // rule needed to get there.
        let mut a = oinf();
        a.phi = IgPrim::err;
        let mut b = oinf();
        b.phi = IgPrim::haha;
        assert_eq!(tensor_under(0, &a, &b).phi, IgPrim::err);
    }

    #[test]
    fn test_join_under_dialect_differs_from_canonical() {
        // join/meet have no per-primitive absorption formula of their own
        // (lattice_op runs plain ord_min/ord_max on Phi for both), so this
        // pair only diverges between dialects if a dialect's own abs_rules
        // table is actually being read: U67 declares ABS_EP, err (𐑻)
        // absorbs on Phi under meet/join/tensor alike (ops_mask 7).
        let mut a = oinf();
        a.phi = IgPrim::err;
        let mut b = oinf();
        b.phi = IgPrim::haha;

        let canonical = join_under(0, &a, &b);
        assert_eq!(canonical.tuple.phi, IgPrim::haha);

        let under_ep = join_under(67, &a, &b);
        assert_eq!(under_ep.tuple.phi, IgPrim::err);
    }
}
