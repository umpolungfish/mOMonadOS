// fde.rs — FDE(n) tower navigation: ascend and descend the truth-value lattice
// at an arbitrary depth.
//
// Transliterates Imscribing/Paraconsistent/FDEAsymptotic.lean directly:
//   FDEN n        : Bot | Mid(Fin n) | Top, n+2 truth values
//   fdeEmbed k n   : FDEN k -> FDEN n     (ascent, k <= n, injective, proven)
//   fdeRestrict n k: FDEN n -> FDEN k     (descent, k <= n, added this session)
//
// Three theorems back this file, all compiled and axiom-checked against the
// real kernel (lake env lean), none of them a sorry:
//   fdeRestrict_fdeEmbed_id   descend after ascend = id, at any depth.
//                             Depends on NO axioms.
//   fdeRestrict_not_injective the descent alone is a real coarsening, not a
//                             second bijection. Depends on propext only.
//   fdeRestrict_trans         restriction factors through any intermediate
//                             tier: the route through a walk does not matter,
//                             only its endpoints. Depends on propext only.
//
// This is that construction, live: `fde walk` runs a value through a named
// sequence of tiers and prints it at every hop, `fde roundtrip` checks the
// identity on the spot instead of citing the proof.

#![allow(dead_code)]

use crate::sprintln;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FdeVal {
    Bot,
    Mid(usize),
    Top,
}

impl core::fmt::Display for FdeVal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FdeVal::Bot => write!(f, "bot"),
            FdeVal::Mid(i) => write!(f, "mid[{}]", i),
            FdeVal::Top => write!(f, "top"),
        }
    }
}

/// fdeEmbed k n hkn : FDEN k -> FDEN n. Fin.castLE only widens the type
/// bound, never the value, so the ascent is the identity on the tag itself.
pub fn fde_embed(_k: usize, _n: usize, x: FdeVal) -> FdeVal {
    x
}

/// fdeRestrict n k hkn : FDEN n -> FDEN k. A mid value already inside range
/// k keeps its index; one only reachable at the deeper tier (index >= k)
/// collapses to top, the "cannot distinguish beyond this depth" pole.
pub fn fde_restrict(_n: usize, k: usize, x: FdeVal) -> FdeVal {
    match x {
        FdeVal::Bot => FdeVal::Bot,
        FdeVal::Mid(i) => if i < k { FdeVal::Mid(i) } else { FdeVal::Top },
        FdeVal::Top => FdeVal::Top,
    }
}

/// A value is well-formed at tier n if a mid index actually fits: i < n.
pub fn fde_wf(n: usize, x: FdeVal) -> bool {
    match x {
        FdeVal::Mid(i) => i < n,
        _ => true,
    }
}

/// Walk a value through a named sequence of tiers, one hop per adjacent
/// pair: embed when the tier grows, restrict when it shrinks, hold when it
/// repeats. Returns the full trace, (tier, value) at every stop.
pub fn fde_walk(tiers: &[usize], start: FdeVal) -> Vec<(usize, FdeVal)> {
    let mut trace = Vec::with_capacity(tiers.len());
    if tiers.is_empty() {
        return trace;
    }
    let mut cur = start;
    trace.push((tiers[0], cur));
    for w in tiers.windows(2) {
        let (from, to) = (w[0], w[1]);
        cur = if to >= from {
            fde_embed(from, to, cur)
        } else {
            fde_restrict(from, to, cur)
        };
        trace.push((to, cur));
    }
    trace
}

/// fdeRestrict_fdeEmbed_id, checked on one concrete value: descend after
/// ascend recovers the input. x must already be well-formed at tier k.
pub fn fde_roundtrip_holds(k: usize, n: usize, x: FdeVal) -> bool {
    let ascended = fde_embed(k, n, x);
    let descended = fde_restrict(n, k, ascended);
    descended == x
}

/// fdeRestrict_trans, checked on one concrete value: restricting straight
/// from n to k equals restricting through an intermediate tier j first.
pub fn fde_trans_holds(n: usize, j: usize, k: usize, x: FdeVal) -> bool {
    let direct = fde_restrict(n, k, x);
    let via_j = fde_restrict(j, k, fde_restrict(n, j, x));
    direct == via_j
}

fn parse_val(s: &str) -> Option<FdeVal> {
    match s {
        "bot" => Some(FdeVal::Bot),
        "top" => Some(FdeVal::Top),
        _ => s.parse::<usize>().ok().map(FdeVal::Mid),
    }
}

fn parse_usize(s: &str) -> Option<usize> {
    s.parse::<usize>().ok()
}

pub fn repl_fde(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("fde — FDE(n) tower navigation (Imscribing.Paraconsistent.FDEAsymptotic)");
        sprintln!("  fde embed <k> <n> <val>          ascend val from tier k into tier n (k<=n)");
        sprintln!("  fde restrict <n> <k> <val>        descend val from tier n to tier k (k<=n)");
        sprintln!("  fde walk <t0> <t1> ... <val>      walk val through a tier sequence, print every hop");
        sprintln!("  fde roundtrip <k> <n> <val>       check descend(ascend(val)) = val live");
        sprintln!("  fde trans <n> <j> <k> <val>       check restrict(n,k) = restrict(j,k) . restrict(n,j)");
        sprintln!("  fde report                        theorem summary + axiom dependencies");
        sprintln!("  val is 'bot', 'top', or a mid index (0-based)");
        sprintln!("  example: fde walk 2 3 4 3 2 1   (electron -> quark -> preon -> back down to mid[1])");
        return;
    }
    match args[0] {
        "embed" => {
            let (Some(k), Some(n), Some(val)) = (
                args.get(1).and_then(|s| parse_usize(s)),
                args.get(2).and_then(|s| parse_usize(s)),
                args.get(3).and_then(|s| parse_val(s)),
            ) else {
                sprintln!("fde embed <k> <n> <val>");
                return;
            };
            if k > n {
                sprintln!("fde: k <= n required (embed is only defined ascending)");
                return;
            }
            if !fde_wf(k, val) {
                sprintln!("fde: {} is not well-formed at tier {}", val, k);
                return;
            }
            sprintln!("{}", fde_embed(k, n, val));
        }
        "restrict" => {
            let (Some(n), Some(k), Some(val)) = (
                args.get(1).and_then(|s| parse_usize(s)),
                args.get(2).and_then(|s| parse_usize(s)),
                args.get(3).and_then(|s| parse_val(s)),
            ) else {
                sprintln!("fde restrict <n> <k> <val>");
                return;
            };
            if k > n {
                sprintln!("fde: k <= n required (restrict is only defined descending)");
                return;
            }
            if !fde_wf(n, val) {
                sprintln!("fde: {} is not well-formed at tier {}", val, n);
                return;
            }
            sprintln!("{}", fde_restrict(n, k, val));
        }
        "walk" => {
            if args.len() < 3 {
                sprintln!("fde walk <t0> <t1> ... <val>  (need at least one tier and a value)");
                return;
            }
            let tail = &args[1..];
            let (tier_strs, val_str) = tail.split_at(tail.len() - 1);
            let tiers: Option<Vec<usize>> = tier_strs.iter().map(|s| parse_usize(s)).collect();
            let (Some(tiers), Some(val)) = (tiers, parse_val(val_str[0])) else {
                sprintln!("fde walk: bad tier or value");
                return;
            };
            if !fde_wf(tiers[0], val) {
                sprintln!("fde: {} is not well-formed at tier {}", val, tiers[0]);
                return;
            }
            let trace = fde_walk(&tiers, val);
            for (i, (t, v)) in trace.iter().enumerate() {
                if i == 0 {
                    sprintln!("  tier {}: {}  (start)", t, v);
                } else {
                    let (from, _) = trace[i - 1];
                    let dir = if *t >= from { "ascend" } else { "descend" };
                    sprintln!("  tier {}: {}  ({})", t, v, dir);
                }
            }
        }
        "roundtrip" => {
            let (Some(k), Some(n), Some(val)) = (
                args.get(1).and_then(|s| parse_usize(s)),
                args.get(2).and_then(|s| parse_usize(s)),
                args.get(3).and_then(|s| parse_val(s)),
            ) else {
                sprintln!("fde roundtrip <k> <n> <val>");
                return;
            };
            if k > n || !fde_wf(k, val) {
                sprintln!("fde: need k<=n and val well-formed at tier k");
                return;
            }
            let ascended = fde_embed(k, n, val);
            let descended = fde_restrict(n, k, ascended);
            let holds = descended == val;
            sprintln!("  {} at tier {} -> embed -> {} at tier {} -> restrict -> {} at tier {}",
                val, k, ascended, n, descended, k);
            sprintln!("  fdeRestrict_fdeEmbed_id holds: {}", holds);
        }
        "trans" => {
            let (Some(n), Some(j), Some(k), Some(val)) = (
                args.get(1).and_then(|s| parse_usize(s)),
                args.get(2).and_then(|s| parse_usize(s)),
                args.get(3).and_then(|s| parse_usize(s)),
                args.get(4).and_then(|s| parse_val(s)),
            ) else {
                sprintln!("fde trans <n> <j> <k> <val>");
                return;
            };
            if !(k <= j && j <= n) || !fde_wf(n, val) {
                sprintln!("fde: need k<=j<=n and val well-formed at tier n");
                return;
            }
            let direct = fde_restrict(n, k, val);
            let via_j = fde_restrict(j, k, fde_restrict(n, j, val));
            sprintln!("  direct  n={} -> k={}: {} -> {}", n, k, val, direct);
            sprintln!("  via j   n={} -> j={} -> k={}: {} -> {}", n, j, k, val, via_j);
            sprintln!("  fdeRestrict_trans holds: {}", direct == via_j);
        }
        "report" => {
            sprintln!("── FDE(n) Tower Descent Report ──");
            sprintln!("  source: Imscribing/Paraconsistent/FDEAsymptotic.lean");
            sprintln!("  FDEN n = Bot | Mid(Fin n) | Top, n+2 truth values");
            sprintln!("  fdeEmbed k n   : FDEN k -> FDEN n, k<=n, injective (proven)");
            sprintln!("  fdeRestrict n k: FDEN n -> FDEN k, k<=n, added this session");
            sprintln!("  fdeRestrict_fdeEmbed_id   : no axioms");
            sprintln!("  fdeRestrict_not_injective : propext only");
            sprintln!("  fdeRestrict_trans         : propext only");
            sprintln!("  physical correspondence: FDE(2) electron, FDE(3) quark, FDE(4) preon, FDE(inf) Planck");
        }
        other => sprintln!("fde: unknown subcommand '{}' (try 'fde help')", other),
    }
}
