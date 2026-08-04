use alloc::vec::Vec;
// rebis/sidechain.rs — Sidechain × Environment Compositional Algebra
// Auto-generated from red-hot_rebis/rhr_p4rky/sidechain_algebra.py
// 20 sidechains × 4 environments = 80 compositional pairs

use crate::imas_ig::IgPrim;
use crate::imas_ig::IgTuple;
use crate::algebra::{meet, join, tensor, tuple_distance};

// ═══ AMINO ACID SIDECHAIN TUPLES (20) ═══

pub const ALANINE: IgTuple = IgTuple {
            d: IgPrim::dead,
            t: IgPrim::judge,
            r: IgPrim::ado,
            p: IgPrim::church,
            f: IgPrim::age,
            k: IgPrim::yea,
            g: IgPrim::bib,
            c: IgPrim::vow,
            h: IgPrim::fee,
            s: IgPrim::hung,
            omega: IgPrim::awe,
            phi: IgPrim::woe,
};

pub const ARGININE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::mime,
            r: IgPrim::ian,
            p: IgPrim::or_,
            f: IgPrim::peep,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const ASPARAGINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::mime,
            r: IgPrim::ear,
            p: IgPrim::yew,
            f: IgPrim::they,
            k: IgPrim::egg,
            g: IgPrim::bib,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const ASPARTATE: IgTuple = IgTuple {
            d: IgPrim::dead,
            t: IgPrim::judge,
            r: IgPrim::ear,
            p: IgPrim::yew,
            f: IgPrim::they,
            k: IgPrim::loll,
            g: IgPrim::bib,
            c: IgPrim::vow,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const CYSTEINE: IgTuple = IgTuple {
            d: IgPrim::dead,
            t: IgPrim::judge,
            r: IgPrim::ian,
            p: IgPrim::yew,
            f: IgPrim::peep,
            k: IgPrim::loll,
            g: IgPrim::bib,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const GLUTAMATE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::ear,
            p: IgPrim::yew,
            f: IgPrim::they,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const GLUTAMINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::ear,
            p: IgPrim::yew,
            f: IgPrim::they,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const GLYCINE: IgTuple = IgTuple {
            d: IgPrim::dead,
            t: IgPrim::judge,
            r: IgPrim::ado,
            p: IgPrim::church,
            f: IgPrim::age,
            k: IgPrim::yea,
            g: IgPrim::thigh,
            c: IgPrim::vow,
            h: IgPrim::fee,
            s: IgPrim::hung,
            omega: IgPrim::awe,
            phi: IgPrim::woe,
};

pub const HISTIDINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::mime,
            r: IgPrim::ian,
            p: IgPrim::out,
            f: IgPrim::they,
            k: IgPrim::egg,
            g: IgPrim::bib,
            c: IgPrim::measure,
            h: IgPrim::sure,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::err,
};

pub const ISOLEUCINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::ado,
            p: IgPrim::out,
            f: IgPrim::age,
            k: IgPrim::on,
            g: IgPrim::bib,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const LEUCINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::ado,
            p: IgPrim::church,
            f: IgPrim::age,
            k: IgPrim::loll,
            g: IgPrim::thigh,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::awe,
            phi: IgPrim::woe,
};

pub const LYSINE: IgTuple = IgTuple {
            d: IgPrim::if_,
            t: IgPrim::judge,
            r: IgPrim::tot,
            p: IgPrim::yew,
            f: IgPrim::they,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const METHIONINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::tot,
            p: IgPrim::church,
            f: IgPrim::they,
            k: IgPrim::loll,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::awe,
            phi: IgPrim::woe,
};

pub const PHENYLALANINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::eat,
            r: IgPrim::ear,
            p: IgPrim::out,
            f: IgPrim::they,
            k: IgPrim::loll,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const PROLINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::eat,
            r: IgPrim::ado,
            p: IgPrim::out,
            f: IgPrim::age,
            k: IgPrim::on,
            g: IgPrim::bib,
            c: IgPrim::vow,
            h: IgPrim::sure,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const SERINE: IgTuple = IgTuple {
            d: IgPrim::dead,
            t: IgPrim::judge,
            r: IgPrim::ian,
            p: IgPrim::yew,
            f: IgPrim::peep,
            k: IgPrim::loll,
            g: IgPrim::bib,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const THREONINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::ian,
            p: IgPrim::out,
            f: IgPrim::peep,
            k: IgPrim::loll,
            g: IgPrim::bib,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const TRYPTOPHAN: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::mime,
            r: IgPrim::ian,
            p: IgPrim::out,
            f: IgPrim::peep,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::sure,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const TYROSINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::mime,
            r: IgPrim::ian,
            p: IgPrim::out,
            f: IgPrim::they,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::sure,
            s: IgPrim::hung,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const VALINE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::judge,
            r: IgPrim::ado,
            p: IgPrim::church,
            f: IgPrim::age,
            k: IgPrim::on,
            g: IgPrim::bib,
            c: IgPrim::vow,
            h: IgPrim::kick,
            s: IgPrim::hung,
            omega: IgPrim::awe,
            phi: IgPrim::woe,
};

// ═══ PROTEIN ENVIRONMENT TUPLES (4) ═══

pub const CHARGED_INTERFACE: IgTuple = IgTuple {
            d: IgPrim::ash,
            t: IgPrim::mime,
            r: IgPrim::ian,
            p: IgPrim::or_,
            f: IgPrim::peep,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::sure,
            s: IgPrim::so,
            omega: IgPrim::ah,
            phi: IgPrim::roar,
};

pub const HYDROPHOBIC_CORE: IgTuple = IgTuple {
            d: IgPrim::dead,
            t: IgPrim::eat,
            r: IgPrim::ado,
            p: IgPrim::church,
            f: IgPrim::age,
            k: IgPrim::on,
            g: IgPrim::bib,
            c: IgPrim::vow,
            h: IgPrim::fee,
            s: IgPrim::so,
            omega: IgPrim::awe,
            phi: IgPrim::woe,
};

pub const INTERFACIAL: IgTuple = IgTuple {
            d: IgPrim::if_,
            t: IgPrim::oil,
            r: IgPrim::ear,
            p: IgPrim::out,
            f: IgPrim::they,
            k: IgPrim::on,
            g: IgPrim::ice,
            c: IgPrim::gag,
            h: IgPrim::kick,
            s: IgPrim::up,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
};

pub const POLAR_SURFACE: IgTuple = IgTuple {
            d: IgPrim::if_,
            t: IgPrim::mime,
            r: IgPrim::ian,
            p: IgPrim::yew,
            f: IgPrim::peep,
            k: IgPrim::egg,
            g: IgPrim::ice,
            c: IgPrim::measure,
            h: IgPrim::sure,
            s: IgPrim::up,
            omega: IgPrim::oak,
            phi: IgPrim::woe,
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
