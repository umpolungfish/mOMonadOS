// zfc_t.rs — ZFCₜ Navigation and Promotion Channels
//
// ZFCₜ = ZFC + chirality + winding topology (tier O₂†)
// Implements the 6 promotion channels from ZFC baseline to ZFCₜ,
// the per-primitive ZFCₜ formula decomposition, and distance to ZFCₜ.
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
//   IUG               O_∞  (universal_imscriptive_grammar)

use crate::imas_ig::{IgPrim, IgTuple};

/// The 6 ZFCₜ promotion channels.
/// Each lifts a ZFC primitive to its ZFCₜ counterpart.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZfcTPromotion {
    Holobound,   // 𐑡 → 𐑸  self-referential topology
    LrDual,      // 𐑩 → 𐑾  bidirectional coupling
    PmZ2,        // 𐑗 → 𐑬  partial Z2 symmetry
    SeqAx,       // 𐑝 → 𐑠  sequential composition
    TempD2,      // 𐑓 → 𐑖  Markov-2 chirality
    ZWind,       // 𐑷 → 𐑭  integer winding
}

impl ZfcTPromotion {
    pub fn all() -> [ZfcTPromotion; 6] {
        [ZfcTPromotion::Holobound, ZfcTPromotion::LrDual,
         ZfcTPromotion::PmZ2, ZfcTPromotion::SeqAx,
         ZfcTPromotion::TempD2, ZfcTPromotion::ZWind]
    }

    pub fn from_primitive(&self) -> IgPrim { self.to_primitive() }
    pub fn to_primitive(&self) -> IgPrim {
        match self {
            ZfcTPromotion::Holobound => IgPrim::T_odot,
            ZfcTPromotion::LrDual    => IgPrim::R_lr,
            ZfcTPromotion::PmZ2      => IgPrim::P_pm,
            ZfcTPromotion::SeqAx     => IgPrim::C_seq,
            ZfcTPromotion::TempD2    => IgPrim::H2,
            ZfcTPromotion::ZWind     => IgPrim::Omega_z,
        }
    }
    pub fn zfc_primitive(&self) -> IgPrim {
        match self {
            ZfcTPromotion::Holobound => IgPrim::T_net,
            ZfcTPromotion::LrDual    => IgPrim::R_super,
            ZfcTPromotion::PmZ2      => IgPrim::P_asym,
            ZfcTPromotion::SeqAx     => IgPrim::C_and,
            ZfcTPromotion::TempD2    => IgPrim::H0,
            ZfcTPromotion::ZWind     => IgPrim::Omega_0,
        }
    }

    pub fn ordinal_gap(&self) -> f32 {
        match self {
            ZfcTPromotion::Holobound => 4.382,
            ZfcTPromotion::LrDual    => 3.000,
            ZfcTPromotion::PmZ2      => 2.000,
            ZfcTPromotion::SeqAx     => 2.191,
            ZfcTPromotion::TempD2    => 2.191,
            ZfcTPromotion::ZWind     => 2.191,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ZfcTPromotion::Holobound => "HOLOBOUND",
            ZfcTPromotion::LrDual    => "LR_DUAL",
            ZfcTPromotion::PmZ2      => "PM_Z2",
            ZfcTPromotion::SeqAx     => "SEQAX",
            ZfcTPromotion::TempD2    => "TEMPD2",
            ZfcTPromotion::ZWind     => "ZWIND",
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

/// ZFC_fe tuple (O_∞ Frobenius-exact): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩
pub const ZFC_FE: IgTuple = IgTuple {
    d: IgPrim::D_odot,    t: IgPrim::T_odot,   r: IgPrim::R_lr,
    p: IgPrim::P_pmsym,   f: IgPrim::F_hbar,    k: IgPrim::K_slow,
    g: IgPrim::G_aleph,    c: IgPrim::C_seq,
    phi: IgPrim::Phi_c,   h: IgPrim::H_inf,    s: IgPrim::S_nm,
    omega: IgPrim::Omega_z,
};

/// CLINK L8 tuple (O_∞⁺): ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑵·⊙·𐑫·𐑳·𐑟⟩
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
    if t.t == ZfcTPromotion::Holobound.to_primitive() { count += 1; }
    if t.r == ZfcTPromotion::LrDual.to_primitive()    { count += 1; }
    if t.p == ZfcTPromotion::PmZ2.to_primitive()      { count += 1; }
    if t.c == ZfcTPromotion::SeqAx.to_primitive()     { count += 1; }
    if t.h == ZfcTPromotion::TempD2.to_primitive()    { count += 1; }
    if t.omega == ZfcTPromotion::ZWind.to_primitive() { count += 1; }
    count
}

/// Check which ZFCₜ promotions are fulfilled.
pub fn promotions_present(t: &IgTuple) -> [bool; 6] {
    [
        t.t == ZfcTPromotion::Holobound.to_primitive(),
        t.r == ZfcTPromotion::LrDual.to_primitive(),
        t.p == ZfcTPromotion::PmZ2.to_primitive(),
        t.c == ZfcTPromotion::SeqAx.to_primitive(),
        t.h == ZfcTPromotion::TempD2.to_primitive(),
        t.omega == ZfcTPromotion::ZWind.to_primitive(),
    ]
}

/// Compute the ZFCₜ distance: weighted sum of unmet promotion gaps.
pub fn zfc_t_distance(t: &IgTuple) -> f32 {
    let present = promotions_present(t);
    let mut d: f32 = 0.0;
    for (i, promo) in ZfcTPromotion::all().iter().enumerate() {
        if !present[i] {
            d += promo.ordinal_gap();
        }
    }
    d
}

/// Determine which stage a tuple belongs to in the ZFC → ZFCₜ → ZFC_fe → CLINK L8 ladder.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZfcTStage {
    Zfc,        // O₀: no promotions
    ZfcT,       // O₂†: 6/6 ZFCₜ promotions, missing Φ=⊙ or D=𐑦
    ZfcFE,      // O_∞: ZFCₜ + Φ=⊙ + D=𐑦 + H=𐑫
    ClinkL8,    // O_∞⁺: ZFC_fe + C=broad + Ω=na
    Other,      // doesn't clearly fit
}

pub fn classify_stage(t: &IgTuple) -> ZfcTStage {
    let promos = count_promotions(t);
    if t.c == IgPrim::C_broad && t.omega == IgPrim::Omega_na
       && t.phi == IgPrim::Phi_c && t.d == IgPrim::D_odot && t.h == IgPrim::H_inf {
        return ZfcTStage::ClinkL8;
    }
    if t.phi == IgPrim::Phi_c && t.d == IgPrim::D_odot && t.h == IgPrim::H_inf && promos >= 6 {
        return ZfcTStage::ZfcFE;
    }
    if promos >= 5 && t.f == IgPrim::F_hbar && t.k == IgPrim::K_slow && t.g == IgPrim::G_aleph {
        return ZfcTStage::ZfcT;
    }
    if promos == 0 {
        return ZfcTStage::Zfc;
    }
    ZfcTStage::Other
}
/// Per-primitive ZFCₜ formula fragment for a given primitive value.
/// Returns a short formula string showing how this primitive expresses in ZFCₜ.
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
/// Reference entry lookup by name.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZfcTEntry {
    Zfc,
    ZfcT,
    TemporalMathematics,
    Schrodinger,
    HeatDiffusion,
    NavierStokes,
    WaveEquation,
    Einstein,
    IUG,
    Unknown,
}

impl ZfcTEntry {
    pub fn from_name(name: &str) -> Self {
        match name {
            "zfc"                   => ZfcTEntry::Zfc,
            "zfc_t"                 => ZfcTEntry::ZfcT,
            "temporal_mathematics"  => ZfcTEntry::TemporalMathematics,
            "schrodinger"           => ZfcTEntry::Schrodinger,
            "heat_diffusion"        => ZfcTEntry::HeatDiffusion,
            "navier_stokes"         => ZfcTEntry::NavierStokes,
            "wave_equation"         => ZfcTEntry::WaveEquation,
            "einstein"              => ZfcTEntry::Einstein,
            "iug" | "IUG" | "universal_imscriptive_grammar" => ZfcTEntry::IUG,
            _                       => ZfcTEntry::Unknown,
        }
    }

    pub fn tuple(&self) -> IgTuple {
        match self {
            ZfcTEntry::Zfc                  => ZFC_BASELINE,
            ZfcTEntry::ZfcT                 => ZFC_T,
            ZfcTEntry::TemporalMathematics  => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_bowtie, r: IgPrim::R_lr,
                p: IgPrim::P_pm,    f: IgPrim::F_hbar,   k: IgPrim::K_slow,
                g: IgPrim::G_aleph,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_c_complex, h: IgPrim::H2, s: IgPrim::S_nm,
                omega: IgPrim::Omega_z,
            },
            ZfcTEntry::Schrodinger           => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_lr,
                p: IgPrim::P_psi,   f: IgPrim::F_hbar,   k: IgPrim::K_mod,
                g: IgPrim::G_beth,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_nn,
                omega: IgPrim::Omega_z2,
            },
            ZfcTEntry::HeatDiffusion         => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_super,
                p: IgPrim::P_asym,  f: IgPrim::F_eth,  k: IgPrim::K_mod,
                g: IgPrim::G_gimel, c: IgPrim::C_and,
                phi: IgPrim::Phi_sub, h: IgPrim::H0, s: IgPrim::S_nn,
                omega: IgPrim::Omega_0,
            },
            ZfcTEntry::NavierStokes          => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_bowtie, r: IgPrim::R_lr,
                p: IgPrim::P_asym,  f: IgPrim::F_ell,    k: IgPrim::K_fast,
                g: IgPrim::G_gimel, c: IgPrim::C_seq,
                phi: IgPrim::Phi_super, h: IgPrim::H1, s: IgPrim::S_nm,
                omega: IgPrim::Omega_0,
            },
            ZfcTEntry::WaveEquation           => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_net, r: IgPrim::R_lr,
                p: IgPrim::P_sym,   f: IgPrim::F_hbar,  k: IgPrim::K_mod,
                g: IgPrim::G_aleph,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_nn,
                omega: IgPrim::Omega_z2,
            },
            ZfcTEntry::Einstein              => IgTuple {
                d: IgPrim::D_infty, t: IgPrim::T_odot, r: IgPrim::R_lr,
                p: IgPrim::P_sym,   f: IgPrim::F_hbar,  k: IgPrim::K_slow,
                g: IgPrim::G_aleph,  c: IgPrim::C_seq,
                phi: IgPrim::Phi_c_complex, h: IgPrim::H2, s: IgPrim::S_nm,
                omega: IgPrim::Omega_z,
            },
            ZfcTEntry::IUG                   => ZFC_FE,
            ZfcTEntry::Unknown               => ZFC_BASELINE,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ZfcTEntry::Zfc                  => "zfc",
            ZfcTEntry::ZfcT                 => "zfc_t",
            ZfcTEntry::TemporalMathematics  => "temporal_mathematics",
            ZfcTEntry::Schrodinger           => "schrodinger",
            ZfcTEntry::HeatDiffusion         => "heat_diffusion",
            ZfcTEntry::NavierStokes          => "navier_stokes",
            ZfcTEntry::WaveEquation          => "wave_equation",
            ZfcTEntry::Einstein              => "einstein",
            ZfcTEntry::IUG                   => "IUG",
            ZfcTEntry::Unknown               => "unknown",
        }
    }
}
