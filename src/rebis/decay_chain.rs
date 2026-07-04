// rebis/decay_chain.rs — Nuclear Decay as IMASM Winding toward Frobenius Fixed Point
// Full port of red-hot_rebis/rhr_p4rky/decay_chain.py
//
// Each radioactive nuclide has ⊙≠⊙ (non-self-referential criticality). Each decay
// event fires δ without a compensating μ, agitating the subatomic Belnap state. The
// chain winds through IMASM type space until it reaches a daughter with ⊙=⊙ — at
// which point μ∘δ=id holds and the winding terminates. The half-life is the dwell
// time per winding step; stability is Frobenius-exactness, not energy exhaustion.

use alloc::vec::Vec;
use alloc::string::ToString;
use crate::sprintln;

use crate::imas_ig::{IgPrim, IgTuple};
use crate::algebra::tuple_distance;

// ─── Decay Step ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DecayStep {
    pub element: &'static str,
    pub isotope: &'static str,
    pub half_life: &'static str,
    pub mode: &'static str,
    pub frobenius_fires: bool,
    pub dist_from_prev: Option<f32>,
    pub stable: bool,
}

// ─── Decay Series ─────────────────────────────────────────────────────────────

pub struct DecaySeries {
    pub key: &'static str,
    pub name: &'static str,
    pub steps: &'static [(&'static str, &'static str, &'static str, &'static str)],
}

pub static DECAY_SERIES: &[DecaySeries] = &[
    DecaySeries {
        key: "U238", name: "Uranium-238 series (4n+2)",
        steps: &[
            ("U",  "U-238",  "4.468 Gy",  "α"),
            ("Th", "Th-234", "24.10 d",   "β"),
            ("Pa", "Pa-234", "1.17 m",    "β"),
            ("U",  "U-234",  "245.5 ky",  "α"),
            ("Th", "Th-230", "75.4 ky",   "α"),
            ("Ra", "Ra-226", "1600 y",    "α"),
            ("Rn", "Rn-222", "3.82 d",    "α"),
            ("Po", "Po-218", "3.05 m",    "α"),
            ("Pb", "Pb-214", "26.8 m",    "β"),
            ("Bi", "Bi-214", "19.7 m",    "β"),
            ("Po", "Po-214", "164 μs",    "α"),
            ("Pb", "Pb-210", "22.3 y",    "β"),
            ("Bi", "Bi-210", "5.01 d",    "β"),
            ("Po", "Po-210", "138.4 d",   "α"),
            ("Pb", "Pb-206", "stable",    "—"),
        ],
    },
    DecaySeries {
        key: "U235", name: "Uranium-235 series (4n+3)",
        steps: &[
            ("U",  "U-235",  "703.8 My",  "α"),
            ("Th", "Th-231", "25.52 h",   "β"),
            ("Pa", "Pa-231", "32.76 ky",  "α"),
            ("Ac", "Ac-227", "21.77 y",   "β"),
            ("Th", "Th-227", "18.68 d",   "α"),
            ("Ra", "Ra-223", "11.43 d",   "α"),
            ("Rn", "Rn-219", "3.96 s",    "α"),
            ("Po", "Po-215", "1.78 ms",   "α"),
            ("Pb", "Pb-211", "36.1 m",    "β"),
            ("Bi", "Bi-211", "2.14 m",    "α"),
            ("Tl", "Tl-207", "4.77 m",    "β"),
            ("Pb", "Pb-207", "stable",    "—"),
        ],
    },
    DecaySeries {
        key: "Th232", name: "Thorium-232 series (4n)",
        steps: &[
            ("Th", "Th-232", "14.05 Gy",  "α"),
            ("Ra", "Ra-228", "5.75 y",    "β"),
            ("Ac", "Ac-228", "6.15 h",    "β"),
            ("Th", "Th-228", "1.912 y",   "α"),
            ("Ra", "Ra-224", "3.66 d",    "α"),
            ("Rn", "Rn-220", "55.6 s",    "α"),
            ("Po", "Po-216", "0.145 s",   "α"),
            ("Pb", "Pb-212", "10.64 h",   "β"),
            ("Bi", "Bi-212", "60.55 m",   "β/α"),
            ("Po", "Po-212", "0.299 μs",  "α"),
            ("Pb", "Pb-208", "stable",    "—"),
        ],
    },
    DecaySeries {
        key: "Ra226", name: "Radium-226 sub-chain (U238 branch from Ra)",
        steps: &[
            ("Ra", "Ra-226", "1600 y",    "α"),
            ("Rn", "Rn-222", "3.82 d",    "α"),
            ("Po", "Po-218", "3.05 m",    "α"),
            ("Pb", "Pb-214", "26.8 m",    "β"),
            ("Bi", "Bi-214", "19.7 m",    "β"),
            ("Po", "Po-214", "164 μs",    "α"),
            ("Pb", "Pb-210", "22.3 y",    "β"),
            ("Pb", "Pb-206", "stable",    "—"),
        ],
    },
    DecaySeries {
        key: "Rn222", name: "Radon-222 chain (environmental)",
        steps: &[
            ("Rn", "Rn-222", "3.82 d",    "α"),
            ("Po", "Po-218", "3.05 m",    "α"),
            ("Pb", "Pb-214", "26.8 m",    "β"),
            ("Bi", "Bi-214", "19.7 m",    "β"),
            ("Po", "Po-214", "164 μs",    "α"),
            ("Pb", "Pb-210", "22.3 y",    "β"),
            ("Bi", "Bi-210", "5.01 d",    "β"),
            ("Po", "Po-210", "138.4 d",   "α"),
            ("Pb", "Pb-206", "stable",    "—"),
        ],
    },
];

// ─── Element → approximate structural tuple ──────────────────────────────────
// Simplified: maps element symbol to an IgTuple capturing the structural type.
// Derived from elem2imasm.py derive_tuple() logic.
// Stable isotopes end at ⊙ self-referentiality.

fn element_tuple(sym: &str) -> IgTuple {
    // Map element symbol to a structural tuple based on periodicity
    match sym {
        // Noble gases: fully symmetric, high criticality
        "Rn" => IgTuple { d: IgPrim::D_odot, t: IgPrim::T_odot, r: IgPrim::R_lr, p: IgPrim::P_sym, f: IgPrim::F_ell, k: IgPrim::K_mod, g: IgPrim::G_aleph, c: IgPrim::C_broad, phi: IgPrim::Phi_c, h: IgPrim::H0, s: IgPrim::S_11, omega: IgPrim::Omega_0 },
        // Lead (Pb-206/207/208): stable — Frobenius fixed point
        "Pb" => IgTuple { d: IgPrim::D_odot, t: IgPrim::T_odot, r: IgPrim::R_lr, p: IgPrim::P_pmsym, f: IgPrim::F_ell, k: IgPrim::K_trap, g: IgPrim::G_aleph, c: IgPrim::C_or, phi: IgPrim::Phi_c, h: IgPrim::H0, s: IgPrim::S_11, omega: IgPrim::Omega_0 },
        // Bismuth: near stable, Bi-209 is stable
        "Bi" => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_bowtie, r: IgPrim::R_dagger, p: IgPrim::P_psi, f: IgPrim::F_ell, k: IgPrim::K_mod, g: IgPrim::G_gimel, c: IgPrim::C_and, phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_11, omega: IgPrim::Omega_z2 },
        // Actinides: complex, high criticality
        "U"  => IgTuple { d: IgPrim::D_odot, t: IgPrim::T_bowtie, r: IgPrim::R_dagger, p: IgPrim::P_psi, f: IgPrim::F_hbar, k: IgPrim::K_mod, g: IgPrim::G_gimel, c: IgPrim::C_or, phi: IgPrim::Phi_ep, h: IgPrim::H2, s: IgPrim::S_nn, omega: IgPrim::Omega_z2 },
        "Th" => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_bowtie, r: IgPrim::R_dagger, p: IgPrim::P_psi, f: IgPrim::F_hbar, k: IgPrim::K_mod, g: IgPrim::G_gimel, c: IgPrim::C_or, phi: IgPrim::Phi_ep, h: IgPrim::H2, s: IgPrim::S_nn, omega: IgPrim::Omega_z2 },
        "Pa" => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_net, r: IgPrim::R_dagger, p: IgPrim::P_psi, f: IgPrim::F_hbar, k: IgPrim::K_fast, g: IgPrim::G_gimel, c: IgPrim::C_or, phi: IgPrim::Phi_ep, h: IgPrim::H1, s: IgPrim::S_nn, omega: IgPrim::Omega_z2 },
        "Ac" => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_net, r: IgPrim::R_super, p: IgPrim::P_psi, f: IgPrim::F_hbar, k: IgPrim::K_mod, g: IgPrim::G_gimel, c: IgPrim::C_or, phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_nn, omega: IgPrim::Omega_z2 },
        // Middle elements
        "Ra" => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_bowtie, r: IgPrim::R_dagger, p: IgPrim::P_psi, f: IgPrim::F_hbar, k: IgPrim::K_mod, g: IgPrim::G_beth, c: IgPrim::C_and, phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_11, omega: IgPrim::Omega_z2 },
        "Po" => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_bowtie, r: IgPrim::R_dagger, p: IgPrim::P_psi, f: IgPrim::F_eth, k: IgPrim::K_mod, g: IgPrim::G_beth, c: IgPrim::C_or, phi: IgPrim::Phi_sub, h: IgPrim::H1, s: IgPrim::S_11, omega: IgPrim::Omega_z2 },
        "Tl" => IgTuple { d: IgPrim::D_wedge, t: IgPrim::T_net, r: IgPrim::R_super, p: IgPrim::P_psi, f: IgPrim::F_ell, k: IgPrim::K_mod, g: IgPrim::G_beth, c: IgPrim::C_and, phi: IgPrim::Phi_sub, h: IgPrim::H0, s: IgPrim::S_11, omega: IgPrim::Omega_0 },
        // Default: middle of periodic table
        _ => IgTuple { d: IgPrim::D_triangle, t: IgPrim::T_net, r: IgPrim::R_super, p: IgPrim::P_asym, f: IgPrim::F_ell, k: IgPrim::K_mod, g: IgPrim::G_beth, c: IgPrim::C_and, phi: IgPrim::Phi_sub, h: IgPrim::H0, s: IgPrim::S_11, omega: IgPrim::Omega_0 },
    }
}

fn frobenius_fires(tup: &IgTuple) -> bool {
    // ⊙ fires (self-referential criticality) when phi == Phi_c
    tup.phi == IgPrim::Phi_c
}// ─── Core analysis ────────────────────────────────────────────────────────────

pub fn analyze_chain(series_key: &str) -> Option<(&'static DecaySeries, Vec<DecayStep>)> {
    let series = DECAY_SERIES.iter().find(|s| s.key == series_key)?;
    let mut steps = Vec::new();
    let mut prev_tup: Option<IgTuple> = None;

    for &(sym, isotope, half_life, mode) in series.steps {
        let tup = element_tuple(sym);
        let fires = frobenius_fires(&tup);
        let dist = prev_tup.map(|p| tuple_distance(&p, &tup));
        let stable = half_life == "stable";

        steps.push(DecayStep {
            element: sym,
            isotope,
            half_life,
            mode,
            frobenius_fires: fires,
            dist_from_prev: dist,
            stable,
        });
        prev_tup = Some(tup);
    }
    Some((series, steps))
}

pub fn known_series() -> Vec<&'static str> {
    DECAY_SERIES.iter().map(|s| s.key).collect()
}

pub fn print_chain(series_key: &str) {
    let Some((series, steps)) = analyze_chain(series_key) else {
        sprintln!("Unknown decay series '{}'. Known: U238, U235, Th232, Ra226, Rn222", series_key);
        return;
    };

    sprintln!("═══════════════════════════════════════════════════════");
    sprintln!("  {}", series.name);
    sprintln!("═══════════════════════════════════════════════════════");
    sprintln!("  Step  Isotope    T½           Mode   Frob     Δ     IMASM winding");

    let mut winding = 0u32;
    for (i, s) in steps.iter().enumerate() {
        let frob_sym = if s.frobenius_fires { "✓ FIRES" } else { "·" };
        let dist_str = match s.dist_from_prev {
            Some(d) => alloc::format!("{:.1}", d),
            None => "  —".to_string(),
        };
        let stable_tag = if s.stable { "  ← FIXED POINT" } else { "" };
        if !s.stable { winding += 1; }

        sprintln!("  {:>3}   {:<10} {:<12} {:<6} {:<7} {:>5}  step {}{}",
            i, s.isotope, s.half_life, s.mode, frob_sym, dist_str, i, stable_tag);
    }

    sprintln!();
    sprintln!("  Windings to closure: {}", winding);

    for (i, s) in steps.iter().enumerate() {
        if s.frobenius_fires {
            sprintln!("  Frobenius first fires at step {}: {} ({})", i, s.isotope, s.half_life);
            break;
        }
    }
}

pub fn print_all_series() {
    for s in DECAY_SERIES {
        print_chain(s.key);
        sprintln!();
    }
}

pub fn compare_series() {
    sprintln!("═══════════════════════════════════════════════════════════");
    sprintln!("  Decay series comparison");
    sprintln!("═══════════════════════════════════════════════════════════");
    sprintln!("  Series      Windings   Frobenius fires at   First stable daughter");
    sprintln!("  ────────── ─────────  ────────────────────  ────────────────────");

    for s in DECAY_SERIES {
        let (_, steps) = analyze_chain(s.key).unwrap();
        let windings = steps.iter().filter(|s| !s.stable).count();
        let first_frob = steps.iter().position(|s| s.frobenius_fires);
        let stable = steps.iter().find(|s| s.stable);

        let frob_label = match first_frob {
            Some(idx) => alloc::format!("{} (step {})", steps[idx].isotope, idx),
            None => "never".to_string(),
        };
        let stable_label = stable.map_or("?".to_string(), |s| s.isotope.to_string());

        sprintln!("  {:<10} {:>9}   {:>20}   {}", s.key, windings, frob_label, stable_label);
    }
}

pub fn series_distance(series_key: &str) -> f32 {
    let Some((_, steps)) = analyze_chain(series_key) else { return 0.0 };
    let total: f32 = steps.iter().filter_map(|s| s.dist_from_prev).sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_series() {
        assert_eq!(known_series().len(), 5);
    }

    #[test]
    fn test_u238_has_steps() {
        let (_, steps) = analyze_chain("U238").unwrap();
        assert_eq!(steps.len(), 15);
    }

    #[test]
    fn test_u238_ends_at_stable() {
        let (_, steps) = analyze_chain("U238").unwrap();
        assert!(steps.last().unwrap().stable);
    }

    #[test]
    fn test_frobenius_fires_at_lead() {
        // Pb has phi = Phi_c, so it should fire
        let pb_tup = element_tuple("Pb");
        assert!(frobenius_fires(&pb_tup));
    }

    #[test]
    fn test_uranium_does_not_fire() {
        let u_tup = element_tuple("U");
        assert!(!frobenius_fires(&u_tup));
    }
}