//! clink.rs — CLINK 9-Layer Chain (L0–L8)
//! Port of clink/chain.py + clink/bridges.py + clink/integration.py

use crate::rebis::pipeline::IgTuple;
use crate::imas_ig::IgPrim;

pub const L0: IgTuple = IgTuple {
    d: IgPrim::D_wedge,    t: IgPrim::T_boxtimes, r: IgPrim::R_super,
    p: IgPrim::P_sym,      f: IgPrim::F_hbar,     k: IgPrim::K_mbl,
    g: IgPrim::G_gimel,    c: IgPrim::C_and,      phi: IgPrim::𐑢,
    h: IgPrim::H0,        s: IgPrim::S_nm,   omega: IgPrim::Omega_0,
};

pub const L1: IgTuple = IgTuple {
    d: IgPrim::D_wedge,    t: IgPrim::T_boxtimes, r: IgPrim::R_super,
    p: IgPrim::P_asym,     f: IgPrim::F_hbar,     k: IgPrim::K_trap,
    g: IgPrim::G_gimel,    c: IgPrim::C_or,      phi: IgPrim::𐑢,
    h: IgPrim::H0,        s: IgPrim::S_nm,   omega: IgPrim::Omega_0,
};

pub const L2: IgTuple = IgTuple {
    d: IgPrim::D_infty,    t: IgPrim::T_bowtie,   r: IgPrim::R_dagger,
    p: IgPrim::P_psi,      f: IgPrim::F_hbar,     k: IgPrim::K_trap,
    g: IgPrim::G_aleph,    c: IgPrim::C_and,      phi: IgPrim::𐑮,
    h: IgPrim::H1,        s: IgPrim::S_nm,   omega: IgPrim::Omega_0,
};

pub const L3: IgTuple = IgTuple {
    d: IgPrim::D_infty,    t: IgPrim::T_bowtie,   r: IgPrim::R_dagger,
    p: IgPrim::P_psi,      f: IgPrim::F_eth,      k: IgPrim::K_slow,
    g: IgPrim::G_beth,     c: IgPrim::C_seq,      phi: IgPrim::⊙,
    h: IgPrim::H0,        s: IgPrim::S_nm,   omega: IgPrim::Omega_z,
};

pub const L4: IgTuple = IgTuple {
    d: IgPrim::D_odot,     t: IgPrim::T_odot,     r: IgPrim::R_lr,
    p: IgPrim::P_pm,       f: IgPrim::F_eth,      k: IgPrim::K_slow,
    g: IgPrim::G_beth,     c: IgPrim::C_seq,      phi: IgPrim::⊙,
    h: IgPrim::H1,        s: IgPrim::S_nm,   omega: IgPrim::Omega_z,
};

pub const L5: IgTuple = IgTuple {
    d: IgPrim::D_odot,     t: IgPrim::T_odot,     r: IgPrim::R_lr,
    p: IgPrim::P_pmsym,    f: IgPrim::F_ell,      k: IgPrim::K_slow,
    g: IgPrim::G_beth,     c: IgPrim::C_seq,      phi: IgPrim::⊙,
    h: IgPrim::H2,        s: IgPrim::S_nm,   omega: IgPrim::Omega_z,
};

pub const L6: IgTuple = IgTuple {
    d: IgPrim::D_odot,     t: IgPrim::T_odot,     r: IgPrim::R_dagger,
    p: IgPrim::P_psi,      f: IgPrim::F_ell,      k: IgPrim::K_slow,
    g: IgPrim::G_beth,     c: IgPrim::C_seq,      phi: IgPrim::⊙,
    h: IgPrim::H2,        s: IgPrim::S_nm,   omega: IgPrim::Omega_z,
};

pub const L7: IgTuple = IgTuple {
    d: IgPrim::D_odot,     t: IgPrim::T_odot,     r: IgPrim::R_lr,
    p: IgPrim::P_pm,       f: IgPrim::F_eth,      k: IgPrim::K_slow,
    g: IgPrim::G_beth,     c: IgPrim::C_broad,    phi: IgPrim::⊙,
    h: IgPrim::H2,        s: IgPrim::S_nm,   omega: IgPrim::Omega_z,
};

pub const L8: IgTuple = IgTuple {
    d: IgPrim::D_odot,     t: IgPrim::T_odot,     r: IgPrim::R_lr,
    p: IgPrim::P_pmsym,    f: IgPrim::F_hbar,     k: IgPrim::K_slow,
    g: IgPrim::G_beth,     c: IgPrim::C_broad,    phi: IgPrim::⊙,
    h: IgPrim::H_inf,      s: IgPrim::S_nm,   omega: IgPrim::Omega_na,
};

pub static CLINK_LAYERS: [IgTuple; 9] = [L0, L1, L2, L3, L4, L5, L6, L7, L8];

pub static CLINK_NAMES: [&str; 9] = [
    "Frustrated Belnap5 (Quarks)", "Electron Orbital (Belnap4)",
    "Atom (Nuclear + Electron)", "Molecule (Chemical Bonds)",
    "Cell (Living)", "Mitosis (Division)", "Meiosis (Gametes)",
    "Tissue/Organ", "Whole Organism",
];

pub static CLINK_TIERS: [&str; 9] = [
    "O₀", "O₀", "O₁", "O₂", "O₂", "O₂", "O₂", "O₂", "O_∞",
];

/// Distance between a tuple and a CLINK layer
pub fn clink_distance(tup: &IgTuple, layer_idx: usize) -> f64 {
    if layer_idx >= 9 { return f64::INFINITY; }
    let target = &CLINK_LAYERS[layer_idx];
    let mut dist = 0.0f64;
    if tup.d != target.d { dist += 1.0; }
    if tup.t != target.t { dist += 1.0; }
    if tup.r != target.r { dist += 1.0; }
    if tup.p != target.p { dist += 1.0; }
    if tup.f != target.f { dist += 1.0; }
    if tup.k != target.k { dist += 1.0; }
    if tup.g != target.g { dist += 1.0; }
    if tup.c != target.c { dist += 1.0; }
    if tup.phi != target.phi { dist += 1.0; }
    if tup.h != target.h { dist += 1.0; }
    if tup.s != target.s { dist += 1.0; }
    if tup.omega != target.omega { dist += 1.0; }
    dist
}

pub fn nearest_clink_layer(tup: &IgTuple) -> (usize, f64) {
    let mut best = (0usize, f64::INFINITY);
    for i in 0..9 {
        let d = clink_distance(tup, i);
        if d < best.1 { best = (i, d); }
    }
    best
}

pub fn tier_from_tuple(tup: &IgTuple) -> &'static str {
    if tup.phi == IgPrim::⊙ && tup.k == IgPrim::K_slow && tup.h == IgPrim::H_inf {
        "O_∞"
    } else if tup.phi == IgPrim::⊙ { "O₂" }
    else if tup.phi == IgPrim::𐑮 { "O₁" }
    else { "O₀" }
}

pub fn c_score_from_tuple(tup: &IgTuple) -> f64 {
    let gate1 = if tup.phi == IgPrim::⊙ { 1.0 }
        else if tup.phi == IgPrim::𐑮 { 0.5 } else { 0.0 };
    let gate2 = if tup.k == IgPrim::K_slow { 1.0 }
        else if tup.k == IgPrim::K_trap { 0.5 } else { 0.0 };
    ((gate1 * gate2 * 1000.0 + 0.5) as u64) as f64 / 1000.0
}

pub fn format_tuple_glyphs(tup: &IgTuple) -> alloc::string::String {
    alloc::format!("⟨{};{};{};{};{};{};{};{};{};{};{};{}⟩",
        tup.d.glyph(), tup.t.glyph(), tup.r.glyph(), tup.p.glyph(),
        tup.f.glyph(), tup.k.glyph(), tup.g.glyph(), tup.c.glyph(),
        tup.phi.glyph(), tup.h.glyph(), tup.s.glyph(), tup.omega.glyph())
}

pub fn clink_summary() -> alloc::string::String {
    alloc::string::String::from(
        "══ CLINK 9-Layer Chain ══\n  L0 Frustrated Belnap5   O₀  Quark color, SU(3)\n  L1 Electron Orbital     O₀  Belnap4 occupancy\n  L2 Atom                 O₁  Nuclear+electron\n  L3 Molecule             O₂  Chemical bonds\n  L4 Cell                 O₂  Self-maintaining\n  L5 Mitosis              O₂  Cell division\n  L6 Meiosis              O₂  Gamete production\n  L7 Tissue/Organ         O₂  Multi-cellular\n  L8 Whole Organism       O_∞ C=1.0 non-Abelian\n"
    )
}

pub fn clink_distance_ladder() -> alloc::string::String {
    let mut s = alloc::string::String::from("══ CLINK Distance Ladder ══\n");
    for i in 0..8 {
        let d = clink_distance(&CLINK_LAYERS[i], i + 1);
        s.push_str(&alloc::format!("  L{}→L{}: d={:.3}  {} → {}\n",
            i, i+1, d, CLINK_NAMES[i], CLINK_NAMES[i+1]));
    }
    s
}

pub fn clink_promotion_ladder() -> alloc::string::String {
    let mut s = alloc::string::String::from("══ CLINK Promotion Ladder (L0→L8) ══\n");
    let mut total = 0.0;
    for i in 0..8 {
        let d = clink_distance(&CLINK_LAYERS[i], i + 1);
        total += d;
        s.push_str(&alloc::format!("  L{}→L{}: Δ={:.3}  cum={:.3}  {}→{}\n",
            i, i+1, d, total, CLINK_TIERS[i], CLINK_TIERS[i+1]));
    }
    s.push_str(&alloc::format!("  Total L0→L8 gap: d={:.3}\n", total));
    s
}

pub fn clink_verify_chain() -> alloc::string::String {
    let mut s = alloc::string::String::from("══ CLINK Chain Verification ══\n");
    for i in 0..9 {
        let c = c_score_from_tuple(&CLINK_LAYERS[i]);
        s.push_str(&alloc::format!("  L{} {}  tier={}  C={:.3}  {}\n",
            i, if i < 9 { "✓" } else { "✗" }, CLINK_TIERS[i], c,
            format_tuple_glyphs(&CLINK_LAYERS[i])));
    }
    s
}

pub fn platonic_protein() -> IgTuple {
    IgTuple {
        d: IgPrim::D_odot,  t: IgPrim::T_bowtie, r: IgPrim::R_lr,
        p: IgPrim::P_pm,    f: IgPrim::F_eth,    k: IgPrim::K_slow,
        g: IgPrim::G_beth,  c: IgPrim::C_seq,    phi: IgPrim::⊙,
        h: IgPrim::H1,     s: IgPrim::S_nm, omega: IgPrim::Omega_z,
    }
}

pub fn bridge_report(tup: &IgTuple, name: &str) -> alloc::string::String {
    let (idx, dist) = nearest_clink_layer(tup);
    alloc::format!(
        "══ CLINK Bridge: {} ══\n  Tuple: {}\n  Nearest: L{} {}\n  Distance: {:.3}\n  Tier: {}  C={:.3}",
        name, format_tuple_glyphs(tup), idx, CLINK_NAMES[idx], dist,
        tier_from_tuple(tup), c_score_from_tuple(tup))
}
