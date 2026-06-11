// cl8nk.rs — CL8NK Navigator (ZFC→ZFCₜ→ZFCfe→CLINK L8 Ladder)
//
// CLINK Layer 8 (Organism) is the terminal ontological layer — O_∞⁺.
// The navigator covers the full 4-stage structural ladder:
//   ZFC baseline → ZFCₜ → ZFCfe → CLINK L8
//
// CL8NK exceeds ZFCfe at exactly two primitives:
//   Ω = 𐑟 (non-Abelian braiding, not ℤ winding)
//   ɢ = 𐑵 (broadcast composition, not sequential)
//
// Reference entries:
//   zfc               O₀  baseline ZFC (absolute minimal)
//   zfc_t             O₂† promoted ZFC + 6 atoms
//   temporal_mathematics O₂
//   schrodinger       O₂
//   heat_diffusion    O₁
//   navier_stokes     O₁
//   wave_equation     O₁
//   einstein          O₂†
//   IUG               O_∞  (universal_imscriptive_grammar ≡ ZFCfe)
//   clink_l8          O_∞⁺ CLINK Layer 8 Organism

use crate::imas_ig::{IgPrim, IgTuple};

/// The 6 ZFC→ZFCₜ promotion channels.
/// Each lifts a ZFC baseline primitive to its ZFCₜ counterpart.
/// These are the foundational promotions; ZFCfe and CLINK L8 build on them.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cl8nkPromotion {
    Holobound,   // 𐑡 → 𐑸  self-referential topology
    LrDual,      // 𐑩 → 𐑾  bidirectional coupling
    PmZ2,        // 𐑗 → 𐑬  partial Z2 symmetry
    SeqAx,       // 𐑝 → 𐑠  sequential composition
    TempD2,      // 𐑓 → 𐑖  Markov-2 chirality
    ZWind,       // 𐑷 → 𐑭  integer winding
}
impl Cl8nkPromotion {
    pub fn all() -> [Cl8nkPromotion; 6] {
        [Cl8nkPromotion::Holobound, Cl8nkPromotion::LrDual,
         Cl8nkPromotion::PmZ2, Cl8nkPromotion::SeqAx,
         Cl8nkPromotion::TempD2, Cl8nkPromotion::ZWind]
    }

    pub fn from_primitive(&self) -> IgPrim { self.to_primitive() }
    pub fn to_primitive(&self) -> IgPrim {
        match self {
            Cl8nkPromotion::Holobound => IgPrim::T_odot,
            Cl8nkPromotion::LrDual    => IgPrim::R_lr,
            Cl8nkPromotion::PmZ2      => IgPrim::P_pm,
            Cl8nkPromotion::SeqAx     => IgPrim::C_seq,
            Cl8nkPromotion::TempD2    => IgPrim::H2,
            Cl8nkPromotion::ZWind     => IgPrim::Omega_z,
        }
    }
    pub fn zfc_primitive(&self) -> IgPrim {
        match self {
            Cl8nkPromotion::Holobound => IgPrim::T_net,
            Cl8nkPromotion::LrDual    => IgPrim::R_super,
            Cl8nkPromotion::PmZ2      => IgPrim::P_asym,
            Cl8nkPromotion::SeqAx     => IgPrim::C_and,
            Cl8nkPromotion::TempD2    => IgPrim::H0,
            Cl8nkPromotion::ZWind     => IgPrim::Omega_0,
        }
    }

    pub fn ordinal_gap(&self) -> f32 {
        match self {
            Cl8nkPromotion::Holobound => 4.382,
            Cl8nkPromotion::LrDual    => 3.000,
            Cl8nkPromotion::PmZ2      => 2.000,
            Cl8nkPromotion::SeqAx     => 2.191,
            Cl8nkPromotion::TempD2    => 2.191,
            Cl8nkPromotion::ZWind     => 2.191,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Cl8nkPromotion::Holobound => "HOLOBOUND",
            Cl8nkPromotion::LrDual    => "LR_DUAL",
            Cl8nkPromotion::PmZ2      => "PM_Z2",
            Cl8nkPromotion::SeqAx     => "SEQAX",
            Cl8nkPromotion::TempD2    => "TEMPD2",
            Cl8nkPromotion::ZWind     => "ZWIND",
        }
    }
}
/// ZFC baseline tuple (O₀): ⟨𐑼·𐑡·𐑩·𐑗·𐑱·𐑘·𐑚·𐑝·𐑢·𐑓·𐑙·𐑷⟩
pub const ZFC_BASELINE: IgTuple = IgTuple {
    d: IgPrim::D_infty,   t: IgPrim::T_net,   r: IgPrim::R_super,
    p: IgPrim::P_asym,    f: IgPrim::F_ell,    k: IgPrim::K_fast,
    g: IgPrim::G_beth,    c: IgPrim::C_and,
    phi: IgPrim::Phi_sub, h: IgPrim::H0,       s: IgPrim::S_11,
    omega: IgPrim::Omega_0,
};

/// ZFCₜ tuple (O₂†): ⟨𐑼·𐑸·𐑾·𐑬·𐑐·𐑧·𐑲·𐑠·𐑮·𐑖·𐑳·𐑭⟩
pub const ZFC_T: IgTuple = IgTuple {
    d: IgPrim::D_infty,   t: IgPrim::T_odot,   r: IgPrim::R_lr,
    p: IgPrim::P_pm,      f: IgPrim::F_hbar,    k: IgPrim::K_slow,
    g: IgPrim::G_aleph,    c: IgPrim::C_seq,
    phi: IgPrim::Phi_c_complex, h: IgPrim::H2,  s: IgPrim::S_nm,
    omega: IgPrim::Omega_z,
};

/// ZFCfe tuple (O_∞ Frobenius-exact): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩
pub const ZFC_FE: IgTuple = IgTuple {
    d: IgPrim::D_odot,    t: IgPrim::T_odot,   r: IgPrim::R_lr,
    p: IgPrim::P_pmsym,   f: IgPrim::F_hbar,    k: IgPrim::K_slow,
    g: IgPrim::G_aleph,    c: IgPrim::C_seq,
    phi: IgPrim::Phi_c,   h: IgPrim::H_inf,    s: IgPrim::S_nm,
    omega: IgPrim::Omega_z,
};

/// CLINK L8 tuple (O_∞⁺): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑵·⊙·𐑫·𐑳·𐑟⟩
/// The terminal ontological layer — exceeds ZFCfe at Ω=𐑟 and ɢ=𐑵
pub const CLINK_L8: IgTuple = IgTuple {
    d: IgPrim::D_odot,    t: IgPrim::T_odot,   r: IgPrim::R_lr,
    p: IgPrim::P_pmsym,   f: IgPrim::F_hbar,    k: IgPrim::K_slow,
    g: IgPrim::G_aleph,    c: IgPrim::C_broad,
    phi: IgPrim::Phi_c,   h: IgPrim::H_inf,    s: IgPrim::S_nm,
    omega: IgPrim::Omega_na,
};
/// Count how many of the 6 ZFCₜ promotions are present in a tuple
/// compared to ZFC baseline.
pub fn count_promotions(t: &IgTuple) -> u8 {
    let mut count = 0u8;
    if t.t == Cl8nkPromotion::Holobound.to_primitive() { count += 1; }
    if t.r == Cl8nkPromotion::LrDual.to_primitive()    { count += 1; }
    if t.p == Cl8nkPromotion::PmZ2.to_primitive()      { count += 1; }
    if t.c == Cl8nkPromotion::SeqAx.to_primitive()     { count += 1; }
    if t.h == Cl8nkPromotion::TempD2.to_primitive()    { count += 1; }
    if t.omega == Cl8nkPromotion::ZWind.to_primitive() { count += 1; }
    count
}

/// Check which ZFCₜ promotions are fulfilled.
pub fn promotions_present(t: &IgTuple) -> [bool; 6] {
    [
        t.t == Cl8nkPromotion::Holobound.to_primitive(),
        t.r == Cl8nkPromotion::LrDual.to_primitive(),
        t.p == Cl8nkPromotion::PmZ2.to_primitive(),
        t.c == Cl8nkPromotion::SeqAx.to_primitive(),
        t.h == Cl8nkPromotion::TempD2.to_primitive(),
        t.omega == Cl8nkPromotion::ZWind.to_primitive(),
    ]
}

/// CL8NK distance: weighted sum of unmet ZFCₜ promotion gaps.
/// This is the structural distance from the ZFC baseline.
pub fn cl8nk_distance(t: &IgTuple) -> f32 {
    let present = promotions_present(t);
    let mut d: f32 = 0.0;
    for (i, promo) in Cl8nkPromotion::all().iter().enumerate() {
        if !present[i] {
            d += promo.ordinal_gap();
        }
    }
    d
}

/// Determine which stage a tuple belongs to in the ZFC→ZFCₜ→ZFCfe→CLINK L8 ladder.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cl8nkStage {
    Zfc,        // O₀: no promotions
    ZfcT,       // O₂†: 6/6 ZFCₜ promotions, missing Φ=⊙ or D=𐑦
    ZfcFE,      // O_∞: ZFCₜ + Φ=⊙ + D=𐑦 + H=𐑫
    ClinkL8,    // O_∞⁺: ZFCfe + C=broad + Ω=na  ← terminal layer
    Other,      // doesn't clearly fit
}

pub fn classify_stage(t: &IgTuple) -> Cl8nkStage {
    let promos = count_promotions(t);
    if t.c == IgPrim::C_broad && t.omega == IgPrim::Omega_na
       && t.phi == IgPrim::Phi_c && t.d == IgPrim::D_odot && t.h == IgPrim::H_inf {
        return Cl8nkStage::ClinkL8;
    }
    if t.phi == IgPrim::Phi_c && t.d == IgPrim::D_odot && t.h == IgPrim::H_inf && promos >= 6 {
        return Cl8nkStage::ZfcFE;
    }
    if promos >= 5 && t.f == IgPrim::F_hbar && t.k == IgPrim::K_slow && t.g == IgPrim::G_aleph {
        return Cl8nkStage::ZfcT;
    }
    if promos == 0 {
        return Cl8nkStage::Zfc;
    }
    Cl8nkStage::Other
}
/// Per-primitive CL8NK formula fragment for a given primitive value.
/// Returns the set-theoretic formula showing how this primitive expresses.
pub fn formula_fragment(prim: IgPrim) -> &'static str {
    match prim {
        // D
        IgPrim::D_infty    => "∀a∃b(a⊂b ∧ rank x=b)",
        IgPrim::D_odot     => "V=L(x) ∧ selfmodel(x) ∧ x∈V",
        IgPrim::D_wedge    => "∃!x",
        IgPrim::D_triangle => "∃x∃y(x≠y ∧ ∀z(z=x∨z=y))",
        // T
        IgPrim::T_net      => "graph(x) ∧ branch(x)",
        IgPrim::T_odot     => "bound_⊙(a,f) ∧ Refl(a,f) ∧ holo(x,a)",
        IgPrim::T_in       => "sep f x",
        IgPrim::T_bowtie   => "cross(x) ∧ ¬flat(x)",
        IgPrim::T_boxtimes => "⊗(a,b) ∧ ¬∃f(f:a≅b)",
        // R
        IgPrim::R_super    => "∀y(y∈x→y∈a)",
        IgPrim::R_lr       => "lr⇔(x,y) ∧ Θ(x,y) ∧ ¬Θ(y,x)",
        IgPrim::R_dagger   => "adj(f,g) ∧ f⊣g",
        IgPrim::R_cat      => "F:C→D ∧ ∃G:D→C(G∘F≅id)",
        // P
        IgPrim::P_asym     => "¬∃sym(x)",
        IgPrim::P_pm       => "ℤ₂(x) ∧ ∀g∈G(gx=x) ∧ μ∘δ=id",
        IgPrim::P_sym      => "∀g∈G(gx=x)",
        IgPrim::P_psi      => "|ψ⟩=Σc_i|i⟩ ∧ superposition(x)",
        IgPrim::P_pmsym    => "μ∘δ=id ∧ Frobenius(x) ∧ ℤ₂(x)",
        // F
        IgPrim::F_ell      => "P(x)∈{0,1} ∧ det(x)",
        IgPrim::F_hbar     => "ℏ(x) ∧ [x,p]=iℏ",
        IgPrim::F_eth      => "ρ(x) ∧ Tr(ρ)=1 ∧ ρ≥0",
        // K
        IgPrim::K_fast     => "τ≪T ∧ ∂_t x=f(x)",
        IgPrim::K_slow     => "τ≫T ∧ eq(x) ∧ gate_open(x)",
        IgPrim::K_mod      => "τ~T ∧ relax(x)",
        IgPrim::K_trap     => "τ→∞ ∧ frozen(x) ∧ order(x)",
        IgPrim::K_mbl      => "τ→∞ ∧ frozen(x) ∧ disorder(x)",
        // G
        IgPrim::G_beth     => "∀y∈x(|y|<|x|)",
        IgPrim::G_aleph    => "∀y(y⊂x→|y|<|x|)",
        IgPrim::G_gimel    => "∃y∈x(|y|=|x|)",
        // C
        IgPrim::C_and      => "f∧g∧h",
        IgPrim::C_seq      => "seq!(f,g) ∧ ⟨→⟩(f,g,τ) ∧ ¬⟨→⟩(g,f,τ)",
        IgPrim::C_or       => "f∨g∨h",
        IgPrim::C_broad    => "f→all(x) ∧ broadcast(x,f)",
        // Phi
        IgPrim::Phi_sub    => "¬∃ξ(diverges(ξ))",
        IgPrim::Phi_c      => "ξ→∞ ∧ μ∘δ=id",
        IgPrim::Phi_c_complex => "ξ∈ℂ ∧ Im(ξ)→∞",
        IgPrim::Phi_ep     => "H=H₀+λV ∧ λ∈EP",
        IgPrim::Phi_super  => "ξ→∞ ∧ ¬(μ∘δ=id)",
        // H
        IgPrim::H0         => "∀x(P(x)↔P(S(x)))",
        IgPrim::H2         => "∃y∃z(y∈x∧z∈y∧¬z∈x ∧ rank(z)<rank(y))",
        IgPrim::H1         => "∃y(y∈x∧P(y)↔¬P(S(y)))",
        IgPrim::H_inf      => "∀n∃φ(rank(φ)>n ∧ φ fixed by μ∘δ ∧ φ∈V)",
        // S
        IgPrim::S_11       => "|A|=1 ∧ |B|=1",
        IgPrim::S_nn       => "|A|=n ∧ |B|=n ∧ ∀a∈A∃!b∈B",
        IgPrim::S_nm       => "∃a∈A∃b∈B(type(a)≠type(b))",
        // Omega
        IgPrim::Omega_0    => "∮_γ dx = 0",
        IgPrim::Omega_z    => "∮_γ A = 2πn ∧ n∈ℤ ∧ wind(γ)≠0",
        IgPrim::Omega_z2   => "∮_γ A = πn ∧ n∈ℤ₂",
        IgPrim::Omega_na   => "Braid(σ_i) ∧ R_matrix≠0 ∧ nonAbelian(x)",
    }
}
/// CL8NK reference entry — covers the full ZFC→ZFCₜ→ZFCfe→CLINK L8 ladder.
/// CLINK L8 is the terminal entry: O_∞⁺ with Ω/ɢ transcendence.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cl8nkEntry {
    Zfc,
    ZfcT,
    TemporalMathematics,
    Schrodinger,
    HeatDiffusion,
    NavierStokes,
    WaveEquation,
    Einstein,
    IUG,
    ClinkL8,
    Unknown,
}

impl Cl8nkEntry {
    pub fn from_name(name: &str) -> Self {
        match name {
            "zfc"                   => Cl8nkEntry::Zfc,
            "zfc_t"                 => Cl8nkEntry::ZfcT,
            "temporal_mathematics"  => Cl8nkEntry::TemporalMathematics,
            "schrodinger"           => Cl8nkEntry::Schrodinger,
            "heat_diffusion"        => Cl8nkEntry::HeatDiffusion,
            "navier_stokes"         => Cl8nkEntry::NavierStokes,
            "wave_equation"         => Cl8nkEntry::WaveEquation,
            "einstein"              => Cl8nkEntry::Einstein,
            "iug" | "IUG" | "universal_imscriptive_grammar" => Cl8nkEntry::IUG,
            "clink" | "clink_l8" | "cl8nk" | "clink_layer8" => Cl8nkEntry::ClinkL8,
            _                       => Cl8nkEntry::Unknown,
        }
    }

    pub fn tuple(&self) -> IgTuple {
        match self {
            Cl8nkEntry::Zfc                  => ZFC_BASELINE,
            Cl8nkEntry::ZfcT                 => ZFC_T,
            Cl8nkEntry::ClinkL8              => CLINK_L8,
            Cl8nkEntry::TemporalMathematics  => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_bowtie, r: IgPrim::R_lr,
                p: IgPrim::P_pm,    f: IgPrim::F_hbar,   k: IgPrim::K_slow,
                g: IgPrim::G_aleph,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_c_complex, h: IgPrim::H2, s: IgPrim::S_nm,
                omega: IgPrim::Omega_z,
            },
            Cl8nkEntry::Schrodinger           => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_lr,
                p: IgPrim::P_psi,   f: IgPrim::F_hbar,   k: IgPrim::K_mod,
                g: IgPrim::G_beth,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_nn,
                omega: IgPrim::Omega_z2,
            },
            Cl8nkEntry::HeatDiffusion         => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_super,
                p: IgPrim::P_asym,  f: IgPrim::F_eth,  k: IgPrim::K_mod,
                g: IgPrim::G_gimel, c: IgPrim::C_and,
                phi: IgPrim::Phi_sub, h: IgPrim::H0, s: IgPrim::S_nn,
                omega: IgPrim::Omega_0,
            },
            Cl8nkEntry::NavierStokes          => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_bowtie, r: IgPrim::R_lr,
                p: IgPrim::P_asym,  f: IgPrim::F_ell,    k: IgPrim::K_fast,
                g: IgPrim::G_gimel, c: IgPrim::C_seq,
                phi: IgPrim::Phi_super, h: IgPrim::H1, s: IgPrim::S_nm,
                omega: IgPrim::Omega_0,
            },
            Cl8nkEntry::WaveEquation           => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_lr,
                p: IgPrim::P_sym,   f: IgPrim::F_hbar,  k: IgPrim::K_mod,
                g: IgPrim::G_aleph,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_nn,
                omega: IgPrim::Omega_z2,
            },
            Cl8nkEntry::Einstein              => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_odot, r: IgPrim::R_lr,
                p: IgPrim::P_sym,   f: IgPrim::F_hbar,  k: IgPrim::K_slow,
                g: IgPrim::G_aleph,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_c_complex, h: IgPrim::H2, s: IgPrim::S_nm,
                omega: IgPrim::Omega_z,
            },
            Cl8nkEntry::IUG                   => ZFC_FE,
            Cl8nkEntry::Unknown               => ZFC_BASELINE,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Cl8nkEntry::Zfc                  => "zfc",
            Cl8nkEntry::ZfcT                 => "zfc_t",
            Cl8nkEntry::ClinkL8              => "clink_l8",
            Cl8nkEntry::TemporalMathematics  => "temporal_mathematics",
            Cl8nkEntry::Schrodinger           => "schrodinger",
            Cl8nkEntry::HeatDiffusion         => "heat_diffusion",
            Cl8nkEntry::NavierStokes          => "navier_stokes",
            Cl8nkEntry::WaveEquation          => "wave_equation",
            Cl8nkEntry::Einstein              => "einstein",
            Cl8nkEntry::IUG                   => "IUG",
            Cl8nkEntry::Unknown               => "unknown",
        }
    }
}