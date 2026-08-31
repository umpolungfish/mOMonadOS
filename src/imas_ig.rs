#![allow(dead_code)]
// imas_ig.rs — IMASM → IG Structural Bridge
//
// Ported from IMSCRIBr/imas_ig_bridge.py (Author: Lando⊗⊙perator)
// Maps kernel Snapshot (StructuralFingerprint) → IG 12-tuple.
// Bridges the kernel's self-imscription to the Imscribing Grammar catalog.
//
// The kernel can now:
//   - Know its own IG type via self-imscribe
//   - Compare against canonical IG types
//   - Compute primitive distances to catalog entries

use crate::kernel::Snapshot;

/// A 12-tuple of IG primitive values as Shavian glyph name constants.
/// Each field corresponds to a primitive family:
///   D: Dimensionality  T: Topology  R: Coupling   P: Parity
///   F: Fidelity        K: Kinetics   G: Cardinality C: Composition
///   Phi: Criticality   H: Chirality  S: Stoich.    Omega: Winding
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

/// IG primitive values as a compact enum.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum IgPrim {
    // D (Dimensionality)
    if_    = 0,  // 𐑦 self-written imscriptive
    dead   = 1,  // 𐑛 0d point
    ash = 2, // 𐑨 2d surface
    array   = 3,  // 𐑼 infinite-dim

    // T (Topology)
    are   = 4,  // 𐑸 self-ref topology
    judge    = 5,  // 𐑡 branching network
    eat     = 6,  // 𐑰 containment
    mime = 7,  // 𐑥 crossing point
    oil = 8, // 𐑶 irreducible product

    // R (Coupling)
    ian    = 9,  // 𐑾 bidirectional
    ear = 10, // 𐑽 adjoint
    tot   = 11, // 𐑑 functorial
    ado = 12, // 𐑩 supervenience

    // P (Parity)
    or_ = 13, // 𐑹 Frobenius-special
    nun   = 14, // 𐑯 full symmetry
    out    = 15, // 𐑬 partial/Z2
    yew   = 16, // 𐑿 quantum superposition
    church  = 17, // 𐑗 none/empty

    // F (Fidelity)
    peep = 18, // 𐑐 quantum
    age  = 19, // 𐑱 classical
    they  = 20, // 𐑞 thermal/noisy

    // K (Kinetics)
    on = 21, // 𐑪 trapped-ordered
    egg = 22, // 𐑧 slow/near-equilibrium
    loll  = 23, // 𐑤 moderate
    yea = 24, // 𐑘 driven/fast
    air  = 25, // 𐑺 trapped-disorder

    // G (Cardinality)
    ice = 26, // 𐑲 long-range/universal
    bib  = 27, // 𐑚 nearest-neighbor/local
    thigh = 28, // 𐑔 mesoscale

    // C (Composition)
    measure   = 29, // 𐑠 ordered steps
    vow   = 30, // 𐑝 all-simultaneous
    gag    = 31, // 𐑜 alternate paths
    ooze = 32, // 𐑵 one-to-all broadcast

    // Phi (Criticality)
    monad  = 33, // ⊙ critical/power-law
    roar = 34, // 𐑮 complex-plane critical
    err        = 35, // 𐑻 exceptional point
    woe       = 36, // 𐑢 sub-critical
    haha     = 37, // 𐑣 supercritical/runaway

    // H (Chirality)
    wool  = 38, // 𐑫 eternal/no finite n
    sure     = 39, // 𐑖 Markov 2
    kick     = 40, // 𐑒 Markov 1
    fee     = 41, // 𐑓 memoryless/Markov 0

    // S (Stoichiometry)
    up   = 42, // 𐑳 multiple distinct
    so   = 43, // 𐑕 many identical
    hung   = 44, // 𐑙 1:1 one type one instance

    // Omega (Winding)
    ah  = 45, // 𐑭 integer winding
    oak = 46, // 𐑴 Z2 parity-protected
    awe  = 47, // 𐑷 trivial/none
    zoo = 48, // 𐑟 non-Abelian braiding
}

impl IgPrim {
    /// Shavian glyph string for this primitive value.
    /// Shavian glyph string for this primitive value.
    /// Delegates to catalog::primitive_glyph() — single source of truth.
    pub fn glyph(self) -> &'static str {
        crate::catalog::primitive_glyph(self)
    }

    /// Short name for this primitive (for status display).
    /// Short name for this primitive (for status display).
    /// Delegates to catalog::primitive_short() — single source of truth.
    pub fn short(self) -> &'static str {
        crate::catalog::primitive_short(self)
    }

    /// True ordinal value, sourced from the Python catalog's
    /// imscrbgrmr.canonical_primitives.ORDINALS table (live, 2026-06-16).
    ///
    /// IMPORTANT: this is NOT the same as `self as u8`. For most families
    /// the enum discriminant happens to be the exact inverse of the ordinal
    /// (lower discriminant = higher ordinal), and existing gate-verify code
    /// elsewhere in this kernel exploits that via `(x as u8) <= (thresh as u8)`.
    /// That trick silently breaks for three families that carry a
    /// non-monotonic extra value: ⊤ (Kinetics: air=4.5 sits between
    /// on=4 and yea=1, not below yea), ⊙/Phi (Criticality:
    /// roar=2.33 and err=2.67 sit between ⊙=2 and
    /// haha=3, not below woe=1), and ⊡ (Winding: zoo=4 sits
    /// above ah=3, not below awe=1). Any new gate logic should
    /// compare `ordinal()` directly rather than raw discriminants.
    pub fn ordinal(self) -> f32 {
        use IgPrim::*;
        match self {
            // ⊢ Dimensionality
            dead => 1.0, ash => 2.0, array => 3.0, if_ => 4.0,
            // ⊣ Topology
            judge => 1.0, eat => 2.0, mime => 3.0, oil => 4.0, are => 5.0,
            // > Recognition
            ado => 1.0, tot => 2.0, ear => 3.0, ian => 4.0,
            // < Parity
            church => 1.0, yew => 2.0, out => 3.0, nun => 4.0, or_ => 5.0,
            // ⋈ Fidelity
            age => 1.0, they => 2.0, peep => 3.0,
            // ⊤ Kinetics — non-monotonic: air is *above* on, not below yea.
            yea => 1.0, loll => 2.0, egg => 3.0, on => 4.0, air => 4.5,
            // ∈ Granularity
            bib => 1.0, thigh => 2.0, ice => 3.0,
            // ∋ Coupling
            vow => 1.0, gag => 2.0, measure => 3.0, ooze => 4.0,
            // ⊙ Criticality — non-monotonic: roar/err sit between
            // ⊙ and haha, not below woe.
            woe => 1.0, monad => 2.0, roar => 2.33, err => 2.67, haha => 3.0,
            // ⊥ Chirality
            fee => 1.0, kick => 2.0, sure => 3.0, wool => 4.0,
            // ⊞ Stoichiometry
            hung => 1.0, so => 2.0, up => 3.0,
            // ⊡ Winding — non-monotonic: zoo sits above ah, not below awe.
            awe => 1.0, oak => 2.0, ah => 3.0, zoo => 4.0,
        }
    }
}
// ─── Fingerprint → IG Tuple Mapping ────────────────────────────

impl IgTuple {
    /// Map a kernel Snapshot to its IG 12-tuple.
    /// This is the structural bridge — same rules as imas_ig_bridge.py.
    /// Parse a 12-glyph tuple, with or without ⟨⟩ brackets and any separators.
    ///
    /// Slot order is the canonical ⊢ ⊣ ≻ ≺ ⋈ ⊤ ∈ ∋ ⊙ ⊥ ⊞ ⊡. Returns the index
    /// of the first glyph that is not a primitive, so a bad tuple names its own
    /// fault rather than failing wholesale.
    pub fn from_glyphs(src: &str) -> Result<IgTuple, (usize, alloc::string::String)> {
        let mut vals: alloc::vec::Vec<IgPrim> = alloc::vec::Vec::new();
        let mut idx = 0usize;
        for c in src.chars() {
            if c.is_whitespace() || c == '⟨' || c == '⟩' || c == ';' || c == ',' || c == '·' {
                continue;
            }
            let mut buf = [0u8; 4];
            let g: &str = c.encode_utf8(&mut buf);
            match crate::catalog::primitive_from_glyph(g) {
                Some(p) => vals.push(p),
                None => return Err((idx, alloc::string::String::from(g))),
            }
            idx += 1;
        }
        if vals.len() != 12 {
            return Err((vals.len(), alloc::format!("expected 12 glyphs, got {}", vals.len())));
        }
        Ok(IgTuple {
            d: vals[0], t: vals[1], r: vals[2], p: vals[3],
            f: vals[4], k: vals[5], g: vals[6], c: vals[7],
            phi: vals[8], h: vals[9], s: vals[10], omega: vals[11],
        })
    }

    pub fn from_snapshot(snap: &Snapshot) -> Self {
        let d = snap.token_diversity;
        let p = snap.period;
        let fo = snap.frobenius_order as usize;
        let sr = snap.self_ref;
        let dc = snap.dialetheia_complete || snap.b_live_ticks > 0;
        let sx = snap.sig.3; // IFIX count
        // R2 (O_inf_dag) structural conditions. kernel.rs names these for the exact
        // primitive values they carry — atomic_reentry is "dim=dead", a point-like
        // fork of one FSPLIT/FFUSE pair; bifurcation_revisited is "top=mime", that
        // single fork recurring every wrap. Both are computed at kernel.rs:677 and
        // were never read here, so dead and mime had a second definition in
        // token_diversity and period, and the kernel's own replicative-opening
        // program ⊙∈∋⊙ derived to 𐑨 𐑸 instead of the 𐑛 𐑥 it targets.
        let ar = snap.atomic_reentry;
        let br = snap.bifurcation_revisited;

        // D — Dimensionality: a point-like fork is 0d, otherwise token diversity
        let d_val = if ar { IgPrim::dead }
            else if d <= 2 { IgPrim::dead }
            else if d <= 5 { IgPrim::ash }
            else if d <= 9 { IgPrim::array }
            else { IgPrim::if_ };

        // T — Topology: the recurring single fork is the bowtie, and it is more
        // specific than self-reference alone, which it implies
        let t_val = if br { IgPrim::mime }
            else if sr { IgPrim::are }
            else if p == 1 { IgPrim::judge }
            else if p == 2 { IgPrim::mime }
            else if fo > 0 { IgPrim::oil }
            else { IgPrim::eat };

        // R — Coupling from frobenius_order
        let r_val = match fo {
            1 => IgPrim::ian,
            2 => IgPrim::ear,
            3 => IgPrim::tot,
            _ => IgPrim::ado,
        };

        // P — Parity from frobenius_order + dialetheia
        let p_val = match fo {
            1 => IgPrim::or_,
            2 => IgPrim::nun,
            3 => IgPrim::out,
            _ => if dc { IgPrim::yew } else { IgPrim::church },
        };

        // F — Fidelity from dialetheia + period
        let f_val = if dc { IgPrim::peep }
            else if p == 1 { IgPrim::age }
            else { IgPrim::they };

        // K — Kinetics from period + IFIX count
        //   on (𐑪) is trapped by ORDER: the fixation count sits exactly on eight.
        //   air (𐑺) is trapped by DISORDER: fixed past that count, with no ordered
        //   count to sit on. Without this branch `air` was never emitted at all, and
        //   184 catalog entries carry it.
        let k_val = if sx == 8 { IgPrim::on }
            else if sx > 8 { IgPrim::air }
            else if p == 1 { IgPrim::egg }
            else if p <= 4 { IgPrim::loll }
            else { IgPrim::yea };

        // G — Cardinality from IFIX + diversity
        // This axis is bib / thigh / ice — it counts DISTINCT MARKS, not fixations. Branching
        // on sx welded it to k_val's `sx == 8`, so eight fixations forced ℵ and the
        // pairs (⊤𐑪,∈𐑔), (⊤𐑪,∈𐑚) and every (⊤𐑺,·) became unwritable: 998 catalog
        // entries, ten_sefirot and CLINK L9 among them. `--recalibrate` walks ⊤ and ∈
        // through every value with the other held, so the Grammar keeps them conjugate
        // and free; the code had them collapsed.
        let g_val = if d >= 10 { IgPrim::ice }
            else if d >= 4 { IgPrim::thigh }
            else { IgPrim::bib };

        // C — Composition from frobenius_order + period
        //   A three-arity fuse (FFUSE3) joins its arms all at once, which is `vow`
        //   ("all-simultaneous", STITCH_3 f∧g∧h) — the same reading `measure`
        //   ("ordered steps") gets wrong for a functorial fork that a two-arity
        //   sequential fuse gets right. So fo == 3 composes simultaneously.
        let c_val = if fo == 3 { IgPrim::vow }
            else if fo > 0 { IgPrim::measure }
            else if p == 1 { IgPrim::vow }
            else if p == 2 { IgPrim::gag }
            else { IgPrim::ooze };

        // Phi — Criticality from self_ref + dialetheia + period
        let phi_val = if sr && dc { IgPrim::monad }
            else if sr { IgPrim::roar }
            else if dc { IgPrim::err }
            else if p == 1 { IgPrim::woe }
            else { IgPrim::haha };

        // H — Chirality from period
        let h_val = match p {
            1 => IgPrim::fee,
            2 => IgPrim::kick,
            3 => IgPrim::sure,
            _ => IgPrim::wool,
        };

        // S — Stoichiometry from non-zero signature count
        let nz = (if snap.sig.0 > 0 { 1 } else { 0 })
               + (if snap.sig.1 > 0 { 1 } else { 0 })
               + (if snap.sig.2 > 0 { 1 } else { 0 })
               + (if snap.sig.3 > 0 { 1 } else { 0 });
        let s_val = if nz == 1 { IgPrim::hung }
            else if nz == 2 { IgPrim::so }
            else { IgPrim::up };

        // Omega — Winding from frobenius_order + self_ref + period
        let omega_val = match fo {
            1 => IgPrim::ah,
            2 => IgPrim::oak,
            _ => if sr { IgPrim::ah }
                else if p == 2 { IgPrim::oak }
                else { IgPrim::awe },
        };

        IgTuple {
            d: d_val, t: t_val, r: r_val, p: p_val,
            f: f_val, k: k_val, g: g_val, c: c_val,
            phi: phi_val, h: h_val, s: s_val, omega: omega_val,
        }
    }

    /// Format as a display string: ⟨𐑦 · 𐑸 · 𐑾 · ...⟩
    pub fn display(&self) -> IgDisplay {
        IgDisplay { tuple: *self }
    }

    /// Count primitive mismatches between two IG tuples.
    pub fn distance(&self, other: &IgTuple) -> usize {
        let mut count = 0;
        if self.d != other.d { count += 1; }
        if self.t != other.t { count += 1; }
        if self.r != other.r { count += 1; }
        if self.p != other.p { count += 1; }
        if self.f != other.f { count += 1; }
        if self.k != other.k { count += 1; }
        if self.g != other.g { count += 1; }
        if self.c != other.c { count += 1; }
        if self.phi != other.phi { count += 1; }
        if self.h != other.h { count += 1; }
        if self.s != other.s { count += 1; }
        if self.omega != other.omega { count += 1; }
        count
    }
}

/// Display helper for IgTuple — formats as ⟨D · T · R · P · F · K · G · C · < · H · S · ⊡⟩
pub struct IgDisplay { tuple: IgTuple }

impl core::fmt::Display for IgDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "⟨{} · {} · {} · {} · {} · {} · {} · {} · {} · {} · {} · {}⟩",
            self.tuple.d.glyph(), self.tuple.t.glyph(),
            self.tuple.r.glyph(), self.tuple.p.glyph(),
            self.tuple.f.glyph(), self.tuple.k.glyph(),
            self.tuple.g.glyph(), self.tuple.c.glyph(),
            self.tuple.phi.glyph(), self.tuple.h.glyph(),
            self.tuple.s.glyph(), self.tuple.omega.glyph())
    }
}

// ─── Canonical IG Types ────────────────────────────────────────

/// Compute IG tuples for all 12 canonical programs.
pub fn all_canonical_ig() -> [IgTuple; 12] {
    use crate::kernel::self_imscribe;
    use crate::tokens::canonical;
    let mut result = [IgTuple {
        d: IgPrim::array, t: IgPrim::are, r: IgPrim::ian,
        p: IgPrim::or_, f: IgPrim::peep, k: IgPrim::loll,
        g: IgPrim::ice, c: IgPrim::measure, phi: IgPrim::monad,
        h: IgPrim::sure, s: IgPrim::up, omega: IgPrim::ah,
    }; 12];
    for i in 0..12 {
        if let Some(prog) = canonical(i) {
            let snap = self_imscribe(&prog);
            result[i] = IgTuple::from_snapshot(&snap);
        }
    }
    result
}


// ─── Classification — Nearest Canonical Matching ───────────────

/// Result of classifying a kernel snapshot against the 12 canonicals.
pub struct Classification {
    /// Index of the nearest canonical (0–11).
    pub nearest_idx: usize,
    /// Name of the nearest canonical.
    pub nearest_name: &'static str,
    /// IG distance (0–12) to the nearest canonical.
    pub distance: usize,
    /// IG tuple of the current snapshot.
    pub current: IgTuple,
    /// IG tuple of the nearest canonical.
    pub canonical: IgTuple,
    /// All 12 distances (for ranking).
    pub all_distances: [usize; 12],
}

impl Classification {
    /// Classify a kernel snapshot against the 12 canonical IG types.
    pub fn classify(snap: &Snapshot) -> Self {
        Self::classify_tuple(&IgTuple::from_snapshot(snap))
    }

    /// Classify a tuple given directly, rather than read off the live kernel.
    /// `classify <t>` is documented to take its argument; this is what it calls.
    pub fn classify_tuple(current: &IgTuple) -> Self {
        use crate::tokens::canonical_name;
        let current = *current;
        let canonicals = all_canonical_ig();

        let mut nearest_idx = 0;
        let mut nearest_dist = 12; // max possible
        let mut all_distances = [0usize; 12];

        for i in 0..12 {
            let d = current.distance(&canonicals[i]);
            all_distances[i] = d;
            if d < nearest_dist {
                nearest_dist = d;
                nearest_idx = i;
            }
        }

        Classification {
            nearest_idx,
            nearest_name: canonical_name(nearest_idx),
            distance: nearest_dist,
            current,
            canonical: canonicals[nearest_idx],
            all_distances,
        }
    }

    /// Display the classification result.
    pub fn display(&self) -> ClassDisplay<'_> {
        ClassDisplay { c: self }
    }
}

pub struct ClassDisplay<'a> { c: &'a Classification }

impl<'a> core::fmt::Display for ClassDisplay<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = self.c;
        writeln!(f, "Classification:")?;
        writeln!(f, "  Nearest: {} (idx {})  distance={}", c.nearest_name, c.nearest_idx, c.distance)?;
        writeln!(f, "  Current:   {}", c.current.display())?;
        writeln!(f, "  Canonical: {}", c.canonical.display())?;
        // Show top 3 matches
        let mut ranked: [(usize, usize); 12] = [(0, 0); 12];
        for i in 0..12 { ranked[i] = (i, c.all_distances[i]); }
        ranked.sort_by_key(|(_, d)| *d);
        writeln!(f, "  Top matches:")?;
        for k in 0..3.min(12) {
            let (idx, dist) = ranked[k];
            use crate::tokens::canonical_name;
            writeln!(f, "    {}: {} (d={})", k+1, canonical_name(idx), dist)?;
        }
        Ok(())
    }
}

// ─── Crystal Address Encoding ──────────────────────────────────
// Maps IgTuple → crystal address using the kernel's encode function.

impl IgTuple {
    /// Convert this IG tuple to 12 primitive indices (0-based within each family).
    /// Maps each IgPrim to its ordinal position within its primitive family.
    /// Convert this IG tuple to 12 primitive indices (0-based within each family).
    /// Uses catalog ordinal tables — no hardcoded match arms.
    /// Each index is the ordinal position of the primitive value within its family.
    pub fn to_crystal_indices(&self) -> [u8; 12] {
        use crate::catalog;
        [
            catalog::ord_index(&catalog::D_ORD, self.d).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::T_ORD, self.t).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::R_ORD, self.r).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::P_ORD, self.p).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::F_ORD, self.f).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::K_ORD, self.k).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::G_ORD, self.g).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::C_ORD, self.c).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::PHI_ORD, self.phi).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::H_ORD, self.h).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::S_ORD, self.s).unwrap_or(0) as u8,
            catalog::ord_index(&catalog::OMEGA_ORD, self.omega).unwrap_or(0) as u8,
        ]
    }

    /// Crystal address for this IG tuple.
    /// Uses the kernel's encode function from crystal.rs.
    pub fn crystal_address(&self) -> u32 {
        crate::crystal::encode(&self.to_crystal_indices())
    }

    /// Compute the theorem phases count from this tuple.
    /// Phases = sum of all 12 ordinal values, rounded to usize.
    /// This is the ONE-AND-ONLY source of the phases count —
    /// never hardcoded, always derived from the tuple.
    pub fn phases(&self) -> usize {
        let sum = self.d.ordinal() + self.t.ordinal() + self.r.ordinal() + self.p.ordinal()
                + self.f.ordinal() + self.k.ordinal() + self.g.ordinal() + self.c.ordinal()
                + self.phi.ordinal() + self.h.ordinal() + self.s.ordinal() + self.omega.ordinal();
        (sum + 0.5) as usize
    }
}

#[cfg(test)]
mod discriminant_gate_tests {
    use super::IgPrim;
    use IgPrim::*;

    /// Every value of the family that `thresh` belongs to, in declaration order.
    fn family_of(thresh: IgPrim) -> &'static [IgPrim] {
        const D: &[IgPrim] = &[if_, dead, ash, array];
        const T: &[IgPrim] = &[are, judge, eat, mime, oil];
        const R: &[IgPrim] = &[ian, ear, tot, ado];
        const P: &[IgPrim] = &[or_, nun, out, yew, church];
        const F: &[IgPrim] = &[peep, age, they];
        const K: &[IgPrim] = &[on, egg, loll, yea, air];
        const G: &[IgPrim] = &[ice, bib, thigh];
        const C: &[IgPrim] = &[measure, vow, gag, ooze];
        const PHI: &[IgPrim] = &[monad, roar, err, woe, haha];
        const H: &[IgPrim] = &[wool, sure, kick, fee];
        const S: &[IgPrim] = &[up, so, hung];
        const OM: &[IgPrim] = &[ah, oak, awe, zoo];
        for fam in [D, T, R, P, F, K, G, C, PHI, H, S, OM] {
            if fam.contains(&thresh) { return fam; }
        }
        unreachable!()
    }

    /// The two ways this kernel writes "primitive is at least `thresh`":
    /// the discriminant trick `(x as u8) <= (thresh as u8)` used by the
    /// hand-crafted dialects 0–7 in `repl.rs`, and `x.ordinal() >=
    /// thresh.ordinal()` used by dialects 8–11 and by `eval_gate_spec` for
    /// every data-driven dialect. Returns the values on which they disagree.
    fn disagreements(thresh: IgPrim) -> alloc::vec::Vec<IgPrim> {
        family_of(thresh).iter().copied().filter(|&x| {
            let by_discriminant = (x as u8) <= (thresh as u8);
            let by_ordinal = x.ordinal() >= thresh.ordinal();
            by_discriminant != by_ordinal
        }).collect()
    }

    /// Parity and Fidelity are declared in strictly descending ordinal order,
    /// so the discriminant trick is sound there — which is why it survived.
    #[test]
    fn discriminant_trick_holds_on_monotonic_families() {
        assert!(disagreements(or_).is_empty(), "< or_: {:?}", disagreements(or_));
        assert!(disagreements(out).is_empty(), "< out: {:?}", disagreements(out));
        assert!(disagreements(peep).is_empty(), "⋈ peep: {:?}", disagreements(peep));
    }

    /// Criticality and Winding carry a value out of declaration order, so the
    /// trick is wrong there — on exactly the thresholds the hand-crafted
    /// dialects gate G2 and G3 with.
    #[test]
    fn discriminant_trick_fails_on_phi_and_omega() {
        // ⊙ ≥ ⊙ (monad): dialects 0,3,4,6,7 reject roar/err/haha, which pass.
        assert_eq!(disagreements(monad), alloc::vec![roar, err, haha]);
        // ⊙ ≥ 𐑢 (woe): dialect 1 rejects haha, which passes.
        assert_eq!(disagreements(woe), alloc::vec![haha]);
        // ⊙ ≥ 𐑮 (roar): dialect 5 wrong BOTH ways — admits monad (2.0 < 2.33)
        // and rejects err and haha, which clear the threshold. woe fails under
        // both readings, so it is not a disagreement.
        assert_eq!(disagreements(roar), alloc::vec![monad, err, haha]);
        // ⊡ ≥ 𐑭 (ah): dialects 0–4,6,7 reject zoo, which passes.
        assert_eq!(disagreements(ah), alloc::vec![zoo]);
        // ⊡ ≥ 𐑟 (zoo): dialect 5's G3 admits every value — a vacuous gate.
        assert_eq!(disagreements(zoo), alloc::vec![ah, oak, awe]);
    }
}
