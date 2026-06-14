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
    D_odot    = 0,  // 𐑦 self-written holographic
    D_wedge   = 1,  // 𐑛 0d point
    D_triangle = 2, // 𐑨 2d surface
    D_infty   = 3,  // 𐑼 infinite-dim

    // T (Topology)
    T_odot   = 4,  // 𐑸 self-ref topology
    T_net    = 5,  // 𐑡 branching network
    T_in     = 6,  // 𐑰 containment
    T_bowtie = 7,  // 𐑥 crossing point
    T_boxtimes = 8, // 𐑶 irreducible product

    // R (Coupling)
    R_lr    = 9,  // 𐑾 bidirectional
    R_dagger = 10, // 𐑽 adjoint
    R_cat   = 11, // 𐑑 functorial
    R_super = 12, // 𐑩 supervenience

    // P (Parity)
    P_pmsym = 13, // 𐑹 Frobenius-special
    P_sym   = 14, // 𐑯 full symmetry
    P_pm    = 15, // 𐑬 partial/Z2
    P_psi   = 16, // 𐑿 quantum superposition
    P_asym  = 17, // 𐑗 none/empty

    // F (Fidelity)
    F_hbar = 18, // 𐑐 quantum
    F_ell  = 19, // 𐑱 classical
    F_eth  = 20, // 𐑞 thermal/noisy

    // K (Kinetics)
    K_trap = 21, // 𐑪 trapped-ordered
    K_slow = 22, // 𐑧 slow/near-equilibrium
    K_mod  = 23, // 𐑤 moderate
    K_fast = 24, // 𐑘 driven/fast
    K_mbl  = 25, // 𐑺 trapped-disorder

    // G (Cardinality)
    G_aleph = 26, // 𐑲 long-range/universal
    G_beth  = 27, // 𐑚 nearest-neighbor/local
    G_gimel = 28, // 𐑔 mesoscale

    // C (Composition)
    C_seq   = 29, // 𐑠 ordered steps
    C_and   = 30, // 𐑝 all-simultaneous
    C_or    = 31, // 𐑜 alternate paths
    C_broad = 32, // 𐑵 one-to-all broadcast

    // Phi (Criticality)
    Phi_c         = 33, // ⊙ critical/power-law
    Phi_c_complex = 34, // 𐑮 complex-plane critical
    Phi_ep        = 35, // 𐑻 exceptional point
    Phi_sub       = 36, // 𐑢 sub-critical
    Phi_super     = 37, // 𐑣 supercritical/runaway

    // H (Chirality)
    H_inf  = 38, // 𐑫 eternal/no finite n
    H2     = 39, // 𐑖 Markov 2
    H1     = 40, // 𐑒 Markov 1
    H0     = 41, // 𐑓 memoryless/Markov 0

    // S (Stoichiometry)
    S_nm   = 42, // 𐑳 multiple distinct
    S_nn   = 43, // 𐑕 many identical
    S_11   = 44, // 𐑙 1:1 one type one instance

    // Omega (Winding)
    Omega_z  = 45, // 𐑭 integer winding
    Omega_z2 = 46, // 𐑴 Z2 parity-protected
    Omega_0  = 47, // 𐑷 trivial/none
    Omega_na = 48, // 𐑟 non-Abelian braiding
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
}
// ─── Fingerprint → IG Tuple Mapping ────────────────────────────

impl IgTuple {
    /// Map a kernel Snapshot to its IG 12-tuple.
    /// This is the structural bridge — same rules as imas_ig_bridge.py.
    pub fn from_snapshot(snap: &Snapshot) -> Self {
        let d = snap.token_diversity;
        let p = snap.period;
        let fo = snap.frobenius_order as usize;
        let sr = snap.self_ref;
        let dc = snap.dialetheia_complete || snap.b_live_ticks > 0;
        let sx = snap.sig.3; // IFIX count

        // D — Dimensionality from token diversity
        let d_val = if d <= 2 { IgPrim::D_wedge }
            else if d <= 5 { IgPrim::D_triangle }
            else if d <= 9 { IgPrim::D_infty }
            else { IgPrim::D_odot };

        // T — Topology from self_ref + period + frobenius_order
        let t_val = if sr { IgPrim::T_odot }
            else if p == 1 { IgPrim::T_net }
            else if p == 2 { IgPrim::T_bowtie }
            else if fo > 0 { IgPrim::T_boxtimes }
            else { IgPrim::T_in };

        // R — Coupling from frobenius_order
        let r_val = match fo {
            1 => IgPrim::R_lr,
            2 => IgPrim::R_dagger,
            3 => IgPrim::R_cat,
            _ => IgPrim::R_super,
        };

        // P — Parity from frobenius_order + dialetheia
        let p_val = match fo {
            1 => IgPrim::P_pmsym,
            2 => IgPrim::P_sym,
            3 => IgPrim::P_pm,
            _ => if dc { IgPrim::P_psi } else { IgPrim::P_asym },
        };

        // F — Fidelity from dialetheia + period
        let f_val = if dc { IgPrim::F_hbar }
            else if p == 1 { IgPrim::F_ell }
            else { IgPrim::F_eth };

        // K — Kinetics from period + IFIX count
        let k_val = if sx == 8 { IgPrim::K_trap }
            else if p == 1 { IgPrim::K_slow }
            else if p <= 4 { IgPrim::K_mod }
            else { IgPrim::K_fast };

        // G — Cardinality from IFIX + diversity
        let g_val = if sx >= 3 { IgPrim::G_aleph }
            else if sx >= 1 { IgPrim::G_gimel }
            else if d <= 3 { IgPrim::G_beth }
            else { IgPrim::G_gimel };

        // C — Composition from frobenius_order + period
        let c_val = if fo > 0 { IgPrim::C_seq }
            else if p == 1 { IgPrim::C_and }
            else if p == 2 { IgPrim::C_or }
            else { IgPrim::C_broad };

        // Phi — Criticality from self_ref + dialetheia + period
        let phi_val = if sr && dc { IgPrim::Phi_c }
            else if sr { IgPrim::Phi_c_complex }
            else if dc { IgPrim::Phi_ep }
            else if p == 1 { IgPrim::Phi_sub }
            else { IgPrim::Phi_super };

        // H — Chirality from period
        let h_val = match p {
            1 => IgPrim::H0,
            2 => IgPrim::H1,
            3 => IgPrim::H2,
            _ => IgPrim::H_inf,
        };

        // S — Stoichiometry from non-zero signature count
        let nz = (if snap.sig.0 > 0 { 1 } else { 0 })
               + (if snap.sig.1 > 0 { 1 } else { 0 })
               + (if snap.sig.2 > 0 { 1 } else { 0 })
               + (if snap.sig.3 > 0 { 1 } else { 0 });
        let s_val = if nz == 1 { IgPrim::S_11 }
            else if nz == 2 { IgPrim::S_nn }
            else { IgPrim::S_nm };

        // Omega — Winding from frobenius_order + self_ref + period
        let omega_val = match fo {
            1 => IgPrim::Omega_z,
            2 => IgPrim::Omega_z2,
            _ => if sr { IgPrim::Omega_z }
                else if p == 2 { IgPrim::Omega_z2 }
                else { IgPrim::Omega_0 },
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

/// Display helper for IgTuple — formats as ⟨D · T · R · P · F · K · G · C · Φ · H · S · Ω⟩
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
        d: IgPrim::D_infty, t: IgPrim::T_odot, r: IgPrim::R_lr,
        p: IgPrim::P_pmsym, f: IgPrim::F_hbar, k: IgPrim::K_mod,
        g: IgPrim::G_aleph, c: IgPrim::C_seq, phi: IgPrim::Phi_c,
        h: IgPrim::H2, s: IgPrim::S_nm, omega: IgPrim::Omega_z,
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
        use crate::tokens::canonical_name;
        let current = IgTuple::from_snapshot(snap);
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
}
