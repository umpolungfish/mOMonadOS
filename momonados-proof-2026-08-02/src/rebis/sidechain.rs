use alloc::vec::Vec;
// rebis/sidechain.rs — Sidechain × Environment Compositional Algebra
// Auto-generated from red-hot_rebis/rhr_p4rky/sidechain_algebra.py
// 20 sidechains × 4 environments = 80 compositional pairs

use crate::imas_ig::IgPrim;
use crate::imas_ig::IgTuple;
use crate::algebra::{meet, join, tensor, tuple_distance};

// ═══ AMINO ACID SIDECHAIN TUPLES (20) ═══

pub const ALANINE: IgTuple = IgTuple {
            d: IgPrim::D_wedge,
            t: IgPrim::T_net,
            r: IgPrim::R_super,
            p: IgPrim::P_asym,
            f: IgPrim::F_ell,
            k: IgPrim::K_fast,
            g: IgPrim::G_beth,
            c: IgPrim::C_and,
            h: IgPrim::H0,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_0,
            phi: IgPrim::𐑢,
};

pub const ARGININE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_lr,
            p: IgPrim::P_pmsym,
            f: IgPrim::F_hbar,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const ASPARAGINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_dagger,
            p: IgPrim::P_psi,
            f: IgPrim::F_eth,
            k: IgPrim::K_slow,
            g: IgPrim::G_beth,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const ASPARTATE: IgTuple = IgTuple {
            d: IgPrim::D_wedge,
            t: IgPrim::T_net,
            r: IgPrim::R_dagger,
            p: IgPrim::P_psi,
            f: IgPrim::F_eth,
            k: IgPrim::K_mod,
            g: IgPrim::G_beth,
            c: IgPrim::C_and,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const CYSTEINE: IgTuple = IgTuple {
            d: IgPrim::D_wedge,
            t: IgPrim::T_net,
            r: IgPrim::R_lr,
            p: IgPrim::P_psi,
            f: IgPrim::F_hbar,
            k: IgPrim::K_mod,
            g: IgPrim::G_beth,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const GLUTAMATE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_dagger,
            p: IgPrim::P_psi,
            f: IgPrim::F_eth,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const GLUTAMINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_dagger,
            p: IgPrim::P_psi,
            f: IgPrim::F_eth,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const GLYCINE: IgTuple = IgTuple {
            d: IgPrim::D_wedge,
            t: IgPrim::T_net,
            r: IgPrim::R_super,
            p: IgPrim::P_asym,
            f: IgPrim::F_ell,
            k: IgPrim::K_fast,
            g: IgPrim::G_gimel,
            c: IgPrim::C_and,
            h: IgPrim::H0,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_0,
            phi: IgPrim::𐑢,
};

pub const HISTIDINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_lr,
            p: IgPrim::P_pm,
            f: IgPrim::F_eth,
            k: IgPrim::K_slow,
            g: IgPrim::G_beth,
            c: IgPrim::C_seq,
            h: IgPrim::H2,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::Phi_ep,
};

pub const ISOLEUCINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_super,
            p: IgPrim::P_pm,
            f: IgPrim::F_ell,
            k: IgPrim::K_trap,
            g: IgPrim::G_beth,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const LEUCINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_super,
            p: IgPrim::P_asym,
            f: IgPrim::F_ell,
            k: IgPrim::K_mod,
            g: IgPrim::G_gimel,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_0,
            phi: IgPrim::𐑢,
};

pub const LYSINE: IgTuple = IgTuple {
            d: IgPrim::D_odot,
            t: IgPrim::T_net,
            r: IgPrim::R_cat,
            p: IgPrim::P_psi,
            f: IgPrim::F_eth,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const METHIONINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_cat,
            p: IgPrim::P_asym,
            f: IgPrim::F_eth,
            k: IgPrim::K_mod,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_0,
            phi: IgPrim::𐑢,
};

pub const PHENYLALANINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_in,
            r: IgPrim::R_dagger,
            p: IgPrim::P_pm,
            f: IgPrim::F_eth,
            k: IgPrim::K_mod,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const PROLINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_in,
            r: IgPrim::R_super,
            p: IgPrim::P_pm,
            f: IgPrim::F_ell,
            k: IgPrim::K_trap,
            g: IgPrim::G_beth,
            c: IgPrim::C_and,
            h: IgPrim::H2,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const SERINE: IgTuple = IgTuple {
            d: IgPrim::D_wedge,
            t: IgPrim::T_net,
            r: IgPrim::R_lr,
            p: IgPrim::P_psi,
            f: IgPrim::F_hbar,
            k: IgPrim::K_mod,
            g: IgPrim::G_beth,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const THREONINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_lr,
            p: IgPrim::P_pm,
            f: IgPrim::F_hbar,
            k: IgPrim::K_mod,
            g: IgPrim::G_beth,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const TRYPTOPHAN: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_lr,
            p: IgPrim::P_pm,
            f: IgPrim::F_hbar,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H2,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const TYROSINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_lr,
            p: IgPrim::P_pm,
            f: IgPrim::F_eth,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H2,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const VALINE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_net,
            r: IgPrim::R_super,
            p: IgPrim::P_asym,
            f: IgPrim::F_ell,
            k: IgPrim::K_trap,
            g: IgPrim::G_beth,
            c: IgPrim::C_and,
            h: IgPrim::H1,
            s: IgPrim::S_11,
            omega: IgPrim::Omega_0,
            phi: IgPrim::𐑢,
};

// ═══ PROTEIN ENVIRONMENT TUPLES (4) ═══

pub const CHARGED_INTERFACE: IgTuple = IgTuple {
            d: IgPrim::D_triangle,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_lr,
            p: IgPrim::P_pmsym,
            f: IgPrim::F_hbar,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H2,
            s: IgPrim::S_nn,
            omega: IgPrim::Omega_z,
            phi: IgPrim::𐑮,
};

pub const HYDROPHOBIC_CORE: IgTuple = IgTuple {
            d: IgPrim::D_wedge,
            t: IgPrim::T_in,
            r: IgPrim::R_super,
            p: IgPrim::P_asym,
            f: IgPrim::F_ell,
            k: IgPrim::K_trap,
            g: IgPrim::G_beth,
            c: IgPrim::C_and,
            h: IgPrim::H0,
            s: IgPrim::S_nn,
            omega: IgPrim::Omega_0,
            phi: IgPrim::𐑢,
};

pub const INTERFACIAL: IgTuple = IgTuple {
            d: IgPrim::D_odot,
            t: IgPrim::T_boxtimes,
            r: IgPrim::R_dagger,
            p: IgPrim::P_pm,
            f: IgPrim::F_eth,
            k: IgPrim::K_trap,
            g: IgPrim::G_aleph,
            c: IgPrim::C_or,
            h: IgPrim::H1,
            s: IgPrim::S_nm,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub const POLAR_SURFACE: IgTuple = IgTuple {
            d: IgPrim::D_odot,
            t: IgPrim::T_bowtie,
            r: IgPrim::R_lr,
            p: IgPrim::P_psi,
            f: IgPrim::F_hbar,
            k: IgPrim::K_slow,
            g: IgPrim::G_aleph,
            c: IgPrim::C_seq,
            h: IgPrim::H2,
            s: IgPrim::S_nm,
            omega: IgPrim::Omega_z2,
            phi: IgPrim::𐑢,
};

pub fn all_sidechains() -> &'static [(&'static str, &'static IgTuple)] {
    &[
        ("alanine", &ALANINE),
        ("arginine", &ARGININE),
        ("asparagine", &ASPARAGINE),
        ("aspartate", &ASPARTATE),
        ("cysteine", &CYSTEINE),
        ("glutamate", &GLUTAMATE),
        ("glutamine", &GLUTAMINE),
        ("glycine", &GLYCINE),
        ("histidine", &HISTIDINE),
        ("isoleucine", &ISOLEUCINE),
        ("leucine", &LEUCINE),
        ("lysine", &LYSINE),
        ("methionine", &METHIONINE),
        ("phenylalanine", &PHENYLALANINE),
        ("proline", &PROLINE),
        ("serine", &SERINE),
        ("threonine", &THREONINE),
        ("tryptophan", &TRYPTOPHAN),
        ("tyrosine", &TYROSINE),
        ("valine", &VALINE),
    ]
}

pub fn all_environments() -> &'static [(&'static str, &'static IgTuple)] {
    &[
        ("charged_interface", &CHARGED_INTERFACE),
        ("hydrophobic_core", &HYDROPHOBIC_CORE),
        ("interfacial", &INTERFACIAL),
        ("polar_surface", &POLAR_SURFACE),
    ]
}

pub fn lookup_sidechain(name: &str) -> Option<&'static IgTuple> {
    all_sidechains().iter().find(|(n, _)| *n == name).map(|(_, t)| *t)
}

pub fn lookup_environment(name: &str) -> Option<&'static IgTuple> {
    all_environments().iter().find(|(n, _)| *n == name).map(|(_, t)| *t)
}

#[derive(Debug, Clone)]
pub struct CompositionalAnalysis {
    pub sidechain: &'static str,
    pub environment: &'static str,
    pub sc_tuple: IgTuple,
    pub env_tuple: IgTuple,
    pub tensor_tuple: IgTuple,
    pub meet_tuple: IgTuple,
    pub join_tuple: IgTuple,
    pub distance_pre: f32,
    pub distance_tensor_sc: f32,
    pub distance_tensor_env: f32,
    pub asymmetry: f32,
    pub domination: &'static str,
    pub n_bottlenecks: u8,
    pub frustration: f32,
}

pub fn analyze(sidechain: &'static str, environment: &'static str) -> Option<CompositionalAnalysis> {
    let sc = lookup_sidechain(sidechain)?;
    let env = lookup_environment(environment)?;

    let t_tensor = tensor(sc, env);
    let t_meet = meet(sc, env);
    let t_join = join(sc, env);

    let d_pre = tuple_distance(sc, env);
    let d_tsc = tuple_distance(&t_tensor, sc);
    let d_tenv = tuple_distance(&t_tensor, env);

    let asym = if d_tenv > 0.0 { d_tsc / d_tenv } else { f32::MAX };
    let domination = if asym > 1.2 { "environment dominates" }
        else if asym < 0.8 { "sidechain dominates" }
        else { "balanced composite" };

    let n_bot = bottleneck_count(sc, env);
    let frustration = if d_pre > 0.0 { d_tsc.min(d_tenv) } else { 0.0 };

    Some(CompositionalAnalysis {
        sidechain,
        environment,
        sc_tuple: *sc,
        env_tuple: *env,
        tensor_tuple: t_tensor,
        meet_tuple: t_meet.tuple,
        join_tuple: t_join.tuple,
        distance_pre: r2(d_pre),
        distance_tensor_sc: r2(d_tsc),
        distance_tensor_env: r2(d_tenv),
        asymmetry: r2(asym),
        domination,
        n_bottlenecks: n_bot,
        frustration: r2(frustration),
    })
}

fn bottleneck_count(a: &IgTuple, b: &IgTuple) -> u8 {
    let mut n: u8 = 0;
    if a.p != b.p { n += 1; }
    if a.f != b.f { n += 1; }
    if a.k != b.k { n += 1; }
    n
}

fn r2(v: f32) -> f32 {
    ((v * 100.0) as i32) as f32 / 100.0
}

pub fn batch_analyze() -> Vec<CompositionalAnalysis> {
    let mut results = Vec::new();
    for (sc_name, _) in all_sidechains() {
        for (env_name, _) in all_environments() {
            if let Some(a) = analyze(sc_name, env_name) {
                results.push(a);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        assert_eq!(all_sidechains().len(), 20);
        assert_eq!(all_environments().len(), 4);
    }

    #[test]
    fn test_batch_80() {
        assert_eq!(batch_analyze().len(), 80);
    }

    #[test]
    fn test_arginine_charged() {
        let a = analyze("arginine", "charged_interface");
        assert!(a.is_some());
        assert_eq!(a.unwrap().sidechain, "arginine");
    }

    #[test]
    fn test_lookup_unknown() {
        assert!(lookup_sidechain("unknown").is_none());
        assert!(lookup_environment("unknown").is_none());
    }
}pub fn frustration_matrix() -> Vec<(&'static str, &'static str, f32)> {
    let mut mat = Vec::new();
    for (sc_name, sc) in all_sidechains() {
        let mut best_env = "";
        let mut best_frustration = f32::MAX;
        for (env_name, env) in all_environments() {
            let d = tuple_distance(sc, env);
            if d < best_frustration {
                best_frustration = d;
                best_env = env_name;
            }
        }
        mat.push((*sc_name, best_env, r2(best_frustration)));
    }
    mat
}
