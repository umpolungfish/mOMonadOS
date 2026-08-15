// ─── sk_forge.rs ───────────────────────────────────────────────────────
// Crystal Harvester: a structural pipeline that reads a public key as an IG
// tuple, finds the nearest O_∞ carrier, and reports the gap and the repair path
// that would move the key into the carrier's basin.
//
// Honest scope: this recovers no real secret. The final "bounded search" is a
// deterministic structural derivation over crystal addresses, labelled
// HEURISTIC, and the pipeline reports IMPOSSIBLE when the key sits in no
// carrier's basin. The value here is the gap analysis, not a key.
//
// Author: written by the local model against a guessed API; grounded to the
// real kernel surface (imas_ig / crystal_scope / carriers / provenance).
//
// Principles from prooflift: The repair process mirrors proof construction -
// each axis promotion is a logical inference step, the gap analysis identifies
// what needs to be proven, and validating the repaired tuple's short word
// representation corresponds to checking if a proof term is well-formed.
// A successful repair that lands in an attractor basin is analogous to a
// closed proof (T verdict), while an incomplete repair resembles an open
// proof (B verdict for undischarged claims).
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::sprintln;
use crate::algebra::tuple_distance;
use crate::basin::{orbit, Action};
use crate::carriers::{population, Carrier};
use crate::crystal_scope::scope;
use crate::entropy::Tier;
use crate::axis_values::{hex_to_tuple, word_to_tuple};
use crate::imas_ig::{IgPrim, IgTuple};
use crate::ouroboros::invert;
use crate::provenance::{provenance_of, Provenance};
use crate::witness::witness;
use imasm_core::check;
use imasm_core::classic::Token as CTok;

/// Public key: hex string, IMASM tuple, or opcode word — one of the three.
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub hex: Option<String>,
    pub tuple: Option<IgTuple>,
    pub word: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SecretKeyResult {
    pub scalar: Option<u64>,
    pub scalar_hex: Option<String>,
    pub method: String,
    pub provenance: Option<Provenance>,
    pub repair_chain: Vec<RepairTrace>,
    /// The shortest word imscribing the repaired tuple (ouroboros-inverse).
    pub shortest_word: Option<String>,
    /// The carrier's standing in the kernel (witness): the Lean/executable
    /// certificate behind the bridge, or its absence.
    pub witness_standing: Option<&'static str>,
    pub certainty: CertaintyLevel,
}

/// One repair step: promote a single axis toward the carrier.
#[derive(Debug, Clone)]
pub struct RepairTrace {
    pub step: usize,
    pub original_tuple: IgTuple,
    pub repair_type: String,
    pub repaired_tuple: IgTuple,
    pub distance_change: f32,
    pub tier_change: String,
    pub cost: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CertaintyLevel {
    /// A structural derivation over crystal addresses. Never a real key.
    Heuristic,
    /// The key sits in no carrier's basin; nothing to derive.
    Impossible,
}

pub struct SkForge {
    max_repairs: usize,
    tier_target: Option<Tier>,
}

impl SkForge {
    pub fn new() -> Self {
        Self { max_repairs: 5, tier_target: None }
    }

    pub fn with_max_repairs(mut self, n: usize) -> Self {
        self.max_repairs = n;
        self
    }

    pub fn with_tier_target(mut self, tier: Tier) -> Self {
        self.tier_target = Some(tier);
        self
    }

    /// The pipeline, six stages, printing as it goes.
    pub fn forge(&self, pk: &PublicKey) -> SecretKeyResult {
        sprintln!("");
        sprintln!("┌─ CRYSTAL HARVESTER (sk_forge) ──────────────────────────────");

        // 1. Get the tuple.
        let tuple = match &pk.tuple {
            Some(t) => *t,
            None => {
                if let Some(hex) = &pk.hex {
                    hex_to_tuple(hex)
                } else if let Some(word) = &pk.word {
                    word_to_tuple(word)
                } else {
                    return impossible("no key given (need hex, tuple, or word)");
                }
            }
        };
        sprintln!("  [1/6] tuple: {}", tuple_to_string(&tuple));

        // 2. Nearest carriers.
        let carriers = nearest_carriers(&tuple);
        let is_no_carriers = carriers.is_empty();
        if is_no_carriers {
            sprintln!("  [2/6] no O_∞ carriers in the catalog");
        } else {
            let (best, best_dist) = &carriers[0];
            sprintln!("  [2/6] nearest carrier: {} (dist={:.4})", best.name, best_dist);
        }

        // 3. Gap analysis.
        let sc = if !is_no_carriers {
            scope(&tuple, &carriers[0].0.entry.tuple)
        } else {
            // Default scope when no carriers
            let default_tuple = IgTuple::from_glyphs("⟨𐑨𐑡𐑩𐑿𐑐𐑧𐑚𐑜⊙𐑖𐑳𐑭⟩").unwrap_or_else(|_|
                IgTuple {
                    d: IgPrim::dead, t: IgPrim::dead, r: IgPrim::dead, p: IgPrim::dead,
                    f: IgPrim::dead, k: IgPrim::dead, g: IgPrim::dead, c: IgPrim::dead,
                    phi: IgPrim::dead, h: IgPrim::dead, s: IgPrim::dead, omega: IgPrim::dead
                }
            );
            scope(&default_tuple, &default_tuple)
        };
        sprintln!("  [3/6] gap:");
        sprintln!(
            "        driver: {} (marginal={:.4})",
            sc.driver_axis.unwrap_or("none"),
            sc.driver_marginal
        );
        sprintln!(
            "        tier: {} → {}",
            sc.tier_a.map(|t| t.name()).unwrap_or("?"),
            sc.tier_b.map(|t| t.name()).unwrap_or("?")
        );
        sprintln!("        ΔS: {:.4}", sc.entropy_delta);

        // 4. Repair chain toward the carrier's basin.
        let is_no_viable_repair = !is_no_carriers && sc.mismatches != 0;
        let repair_chain = if is_no_viable_repair {
            Vec::new()
        } else {
            let target_tuple = if !is_no_carriers {
                &carriers[0].0.entry.tuple
            } else {
                &tuple
            };
            self.run_repairs(&tuple, target_tuple)
        };
        let final_tuple = if is_no_viable_repair {
            tuple
        } else {
            repair_chain
                .last()
                .map(|r| r.repaired_tuple)
                .unwrap_or(tuple)
        };
        sprintln!("  [4/6] repairs applied: {}", repair_chain.len());

        // 4b. Shortest word imscribing the repaired tuple (ouroboros-inverse),
        // and whether it settles into an attractor (basin). A repair that lands
        // on a tuple no short word imscribes, or one that runs away under
        // REPAIR, is not a usable bridge.
        let inv = invert(&final_tuple);
        let shortest = inv.shortest.clone();
        match &shortest {
            Some(w) => {
                sprintln!("        shortest word: {} ({} siblings)", w, inv.siblings);
                let orb = orbit(w, Action::Repair);
                sprintln!(
                    "        basin: attractor {} (transient {}, cycle {})",
                    orb.attractor, orb.transient_depth, orb.cycle_length
                );
                
                // Prooflift-inspired verification: check if the shortest word
                // forms a valid IMASM proof term
                let verdict = self.verify_proof_term(w);
                sprintln!("        prooflift verdict: {} (proof structural validity)", verdict);
            }
            None => sprintln!(
                "        no short word imscribes the repaired tuple (searched {})",
                inv.searched
            ),
        }

        // 5. Provenance of the carrier + its witness — the Lean/executable
        // certificate behind the bridge, not a heuristic.
        let (prov_name, wit_standing) = if is_no_carriers {
            ("Unknown".to_string(), crate::witness::Standing::Unresolved)
        } else {
            let prov = provenance_of(&carriers[0].0.name).root;
            let wit = witness(&carriers[0].0.name);
            (prov.name().to_string(), wit.standing)
        };
        sprintln!("  [5/6] carrier provenance: {}", prov_name);
        sprintln!("        witness: {}", wit_standing.name());

        // 6. Bounded structural derivation. Deterministic, honest, HEURISTIC.
        let (scalar, window, method) = if is_no_carriers || is_no_viable_repair {
            // For impossibility cases, we still compute a value but mark it as such
            let window = 1;
            let scalar = 0;
            (scalar, window, "IMPOSSIBILITY_CERTIFICATE".to_string())
        } else {
            self.bounded_search(&final_tuple, &carriers[0].0)
        };
        sprintln!("  [6/6] search window: 2^{}", window_bits(window));

        // Determine certainty level
        let certainty = if is_no_carriers || is_no_viable_repair {
            CertaintyLevel::Impossible
        } else {
            CertaintyLevel::Heuristic
        };

        SecretKeyResult {
            scalar: if certainty == CertaintyLevel::Heuristic { Some(scalar) } else { None },
            scalar_hex: if certainty == CertaintyLevel::Heuristic { Some(format!("{:016x}", scalar)) } else { None },
            method,
            provenance: if !is_no_carriers { Some(provenance_of(&carriers[0].0.name).root) } else { None },
            repair_chain,
            shortest_word: if certainty == CertaintyLevel::Heuristic { shortest } else { None },
            witness_standing: if !is_no_carriers { Some(wit_standing.name()) } else { None },
            certainty,
        }
    }

    /// Verify if a word forms a valid IMASM proof term (prooflift principle)
    fn verify_proof_term(&self, word: &str) -> char {
        let toks: Vec<CTok> = word
            .chars()
            .filter_map(|c| CTok::parse(&c.to_string()))
            .collect();
        check::word_verdict(&toks).0
    }

    /// Walk the scope's driver moves, one axis at a time, until close or spent.
    fn run_repairs(&self, original: &IgTuple, target: &IgTuple) -> Vec<RepairTrace> {
        let mut chain = Vec::new();
        let mut current = *original;
        let mut step = 0;

        while step < self.max_repairs {
            let dist = tuple_distance(&current, target);
            if dist < 0.001 {
                break;
            }
            let sc = scope(&current, target); // Compute scope once
            let (mv_axis, mv_from, mv_to, mv_marginal) = match sc.moves.first() {
                Some(m) => (m.axis, m.from, m.to, m.marginal),
                None => break,
            };
            let next = set_axis(&current, mv_axis, mv_to);
            let new_dist = tuple_distance(&next, target);

            // Reuse the scope we already computed for tier_before, and compute
            // scope for the next tuple to get tier_after
            let tier_before = sc.tier_a.map(|t| t.name()).unwrap_or("?");
            let sc_next = scope(&next, target);
            let tier_after = sc_next.tier_a.map(|t| t.name()).unwrap_or("?");

            chain.push(RepairTrace {
                step: step + 1,
                original_tuple: current,
                repair_type: format!("promote {} ({}→{})", mv_axis, mv_from.glyph(), mv_to.glyph()),
                repaired_tuple: next,
                distance_change: dist - new_dist,
                tier_change: format!("{} → {}", tier_before, tier_after),
                cost: mv_marginal as f64,
            });

            current = next;
            step += 1;
        }
        chain
    }

    /// A tier-narrowed window and a deterministic scalar within it. Not a key.
    fn bounded_search(&self, tuple: &IgTuple, carrier: &Carrier) -> (u64, u64, String) {
        let tier = scope(tuple, &carrier.entry.tuple).tier_a;
        let window = search_window(tier);
        let addr = tuple.crystal_address() as u64;
        let caddr = carrier.entry.tuple.crystal_address() as u64;
        let mut scalar = (addr ^ caddr) % window.max(1);
        if scalar == 0 {
            scalar = 1;
        }
        (scalar, window, format!("structural (window=2^{})", window_bits(window)))
    }

    fn impossibility_certificate(
        &self,
        tuple: &IgTuple,
        carriers: &[(Carrier, f32)],
    ) -> SecretKeyResult {
        let d = carriers.first().map(|(_, d)| *d).unwrap_or(f32::INFINITY);
        sprintln!("  [certificate] key not in any bounded-shortcut basin");
        sprintln!("                nearest carrier distance: {:.4}", d);
        // The certificate is a chain, not a bare distance: the nearest carrier,
        // its provenance, and its witness standing. The key is full-strength
        // relative to every carrier that admits a shortcut — reported as the
        // structure it is, not asserted.
        let prov = carriers.first().map(|(c, _)| provenance_of(c.name).root);
        let wstanding = carriers.first().map(|(c, _)| witness(c.name).standing.name());
        if let Some(p) = prov {
            sprintln!("                nearest carrier provenance: {}", p.name());
        }
        if let Some(w) = wstanding {
            sprintln!("                nearest carrier witness: {}", w);
        }
        let _ = tuple;
        SecretKeyResult {
            scalar: None,
            scalar_hex: None,
            method: "IMPOSSIBILITY_CERTIFICATE".to_string(),
            provenance: prov,
            repair_chain: Vec::new(),
            shortest_word: None,
            witness_standing: wstanding,
            certainty: CertaintyLevel::Impossible,
        }
    }
}

fn impossible(reason: &str) -> SecretKeyResult {
    sprintln!("  error: {}", reason);
    SecretKeyResult {
        scalar: None,
        scalar_hex: None,
        method: "ERROR".to_string(),
        provenance: None,
        repair_chain: Vec::new(),
        shortest_word: None,
        witness_standing: None,
        certainty: CertaintyLevel::Impossible,
    }
}

// ─── tuple helpers ─────────────────────────────────────────────────────

fn tuple_to_string(t: &IgTuple) -> String {
    format!(
        "⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
        t.d.glyph(), t.t.glyph(), t.r.glyph(), t.p.glyph(), t.f.glyph(), t.k.glyph(),
        t.g.glyph(), t.c.glyph(), t.phi.glyph(), t.h.glyph(), t.s.glyph(), t.omega.glyph()
    )
}

fn set_axis(t: &IgTuple, axis: &str, v: IgPrim) -> IgTuple {
    let mut n = *t;
    match axis {
        "D" | "⊢" => n.d = v,
        "T" | "⊣" => n.t = v,
        "R" | "≻" => n.r = v,
        "P" | "≺" => n.p = v,
        "F" | "⋈" => n.f = v,
        "K" | "⊤" => n.k = v,
        "G" | "∈" => n.g = v,
        "C" | "∋" => n.c = v,
        "Phi" | "⊙" => n.phi = v,
        "H" | "⊥" => n.h = v,
        "S" | "⊞" => n.s = v,
        "Omega" | "◻" => n.omega = v,
        _ => {}
    }
    n
}

/// Count the twelve opcode marks in a word; reduce each count into its axis.


/// FNV-1a over the hex string → 12 bytes → one value per axis.


fn nearest_carriers(tuple: &IgTuple) -> Vec<(Carrier, f32)> {
    let mut ds: Vec<(Carrier, f32)> = population()
        .into_iter()
        .map(|carrier| {
            let d = tuple_distance(tuple, &carrier.entry.tuple);
            (carrier, d)
        })
        .collect();
    ds.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal));
    ds.truncate(5);
    ds
}

fn search_window(tier: Option<Tier>) -> u64 {
    // The residual search space is the tier's own type count — a real crystal
    // quantity, not a hand-picked factor. A higher tier holds fewer types, so
    // the window is smaller; with no tier it is the whole crystal.
    tier.map(|t| t.n_types() as u64)
        .unwrap_or(crate::crystal::TOTAL as u64)
        .max(1)
}

fn window_bits(w: u64) -> u32 {
    63u32.saturating_sub(w.leading_zeros().min(63))
}

// ─── REPL surface ──────────────────────────────────────────────────────

pub fn sk_forge_main(args: &str) -> String {
    ::alloc::string::String::new()
}

fn help() -> &'static str {
    "Crystal Harvester (sk_forge) — structural gap analysis against O_∞ carriers.

Usage:
  sk_forge forge <pk_hex> [--max-repairs N]   derive tuple from hex, analyse gap
  sk_forge tuple <12 glyphs>                  analyse a given tuple
  sk_forge word <imas_word>                   derive tuple from an opcode word
  sk_forge verify <word>                      verify IMASM word as proof term (prooflift)
  sk_forge carriers                           list the O_∞ carriers

Pipeline: classify → nearest carrier → crystal-scope gap → repair path →
carrier provenance → bounded structural derivation.

The derivation recovers no real secret. Its scalar is HEURISTIC, over crystal
addresses; when the key sits in no carrier's basin the result is IMPOSSIBLE.

Proof principles: Each axis promotion is a logical inference step. Verifying
the repaired tuple's short word representation checks structural validity
like prooflift checks proof terms.
"
}

fn format_result(r: &SecretKeyResult) -> String {
    let mut out = String::new();
    out.push_str("\n");
    out.push_str(&format!(
        "├─ result: {}\n",
        match r.certainty {
            CertaintyLevel::Heuristic => "HEURISTIC (structural, not a key)",
            CertaintyLevel::Impossible => "IMPOSSIBLE",
        }
    ));
    if let Some(s) = r.scalar {
        out.push_str(&format!("├─ scalar: {}\n", s));
        if let Some(hex) = &r.scalar_hex {
            out.push_str(&format!("├─ scalar (hex): {}\n", hex));
        }
    }
    out.push_str(&format!("├─ method: {}\n", r.method));
    if let Some(w) = &r.shortest_word {
        out.push_str(&format!("├─ shortest word: {}\n", w));
    }
    if let Some(p) = &r.provenance {
        out.push_str(&format!("├─ carrier provenance: {}\n", p.name()));
    }
    if let Some(w) = r.witness_standing {
        out.push_str(&format!("├─ carrier witness: {}\n", w));
    }
    if !r.repair_chain.is_empty() {
        out.push_str(&format!("├─ repair chain ({} steps):\n", r.repair_chain.len()));
        for t in &r.repair_chain {
            out.push_str(&format!(
                "│    step {}: {}  Δdist={:.4}  [{}]\n",
                t.step, t.repair_type, t.distance_change, t.tier_change
            ));
        }
    }
    out.push_str("└─\n");
    out
}