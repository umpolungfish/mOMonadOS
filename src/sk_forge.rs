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
#![allow(dead_code)]
#![allow(uncommon_codepoints)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::sprintln;
use crate::algebra::tuple_distance;
use crate::carriers::{population, Carrier};
use crate::crystal_scope::{scope, Scope};
use crate::entropy::Tier;
use crate::imas_ig::{IgPrim, IgTuple};
use crate::provenance::{provenance_of, Provenance};

// Canonical per-axis value lists, low ordinal → high, sourced from the IgPrim
// ordinal() table in imas_ig.rs. A count or byte is reduced modulo the family
// size and indexes into its own axis, so every derived value is a real member.
const D_VALS: [IgPrim; 4] = [IgPrim::dead, IgPrim::ash, IgPrim::array, IgPrim::if_];
const T_VALS: [IgPrim; 5] = [IgPrim::judge, IgPrim::eat, IgPrim::mime, IgPrim::oil, IgPrim::are];
const R_VALS: [IgPrim; 4] = [IgPrim::ado, IgPrim::tot, IgPrim::ear, IgPrim::ian];
const P_VALS: [IgPrim; 5] = [IgPrim::church, IgPrim::yew, IgPrim::out, IgPrim::nun, IgPrim::or_];
const F_VALS: [IgPrim; 3] = [IgPrim::age, IgPrim::they, IgPrim::peep];
const K_VALS: [IgPrim; 5] = [IgPrim::yea, IgPrim::loll, IgPrim::egg, IgPrim::on, IgPrim::air];
const G_VALS: [IgPrim; 3] = [IgPrim::bib, IgPrim::thigh, IgPrim::ice];
const C_VALS: [IgPrim; 4] = [IgPrim::vow, IgPrim::gag, IgPrim::measure, IgPrim::ooze];
const PHI_VALS: [IgPrim; 5] = [IgPrim::woe, IgPrim::monad, IgPrim::roar, IgPrim::err, IgPrim::haha];
const H_VALS: [IgPrim; 4] = [IgPrim::fee, IgPrim::kick, IgPrim::sure, IgPrim::wool];
const S_VALS: [IgPrim; 3] = [IgPrim::hung, IgPrim::so, IgPrim::up];
const OM_VALS: [IgPrim; 4] = [IgPrim::awe, IgPrim::oak, IgPrim::ah, IgPrim::zoo];

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
        if carriers.is_empty() {
            sprintln!("  [2/6] no O_∞ carriers in the catalog");
            return self.impossibility_certificate(&tuple, &carriers);
        }
        let (best, best_dist) = &carriers[0];
        sprintln!("  [2/6] nearest carrier: {} (dist={:.4})", best.name, best_dist);

        // 3. Gap analysis.
        let sc = scope(&tuple, &best.entry.tuple);
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
        let repair_chain = self.run_repairs(&tuple, &best.entry.tuple);
        if repair_chain.is_empty() && sc.mismatches != 0 {
            sprintln!("  [4/6] no viable repair (gap present but no axis moved)");
            return self.impossibility_certificate(&tuple, &carriers);
        }
        let final_tuple = repair_chain
            .last()
            .map(|r| r.repaired_tuple)
            .unwrap_or(tuple);
        sprintln!("  [4/6] repairs applied: {}", repair_chain.len());

        // 5. Provenance of the carrier the repair aimed at.
        let prov = provenance_of(best.name).root;
        sprintln!("  [5/6] carrier provenance: {}", prov.name());

        // 6. Bounded structural derivation. Deterministic, honest, HEURISTIC.
        let (scalar, window, method) = self.bounded_search(&final_tuple, best);
        sprintln!("  [6/6] search window: 2^{}", window_bits(window));

        SecretKeyResult {
            scalar: Some(scalar),
            scalar_hex: Some(format!("{:016x}", scalar)),
            method,
            provenance: Some(prov),
            repair_chain,
            certainty: CertaintyLevel::Heuristic,
        }
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
            let sc = scope(&current, target);
            let (mv_axis, mv_from, mv_to, mv_marginal) = match sc.moves.first() {
                Some(m) => (m.axis, m.from, m.to, m.marginal),
                None => break,
            };
            let next = set_axis(&current, mv_axis, mv_to);
            let new_dist = tuple_distance(&next, target);

            let tier_before = scope(&current, target)
                .tier_a
                .map(|t| t.name())
                .unwrap_or("?");
            let tier_after = scope(&next, target).tier_a.map(|t| t.name()).unwrap_or("?");

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
        sprintln!("  [certificate] key not in any O_∞ carrier's basin");
        sprintln!("                nearest carrier distance: {:.4}", d);
        let _ = tuple;
        SecretKeyResult {
            scalar: None,
            scalar_hex: None,
            method: "IMPOSSIBILITY_CERTIFICATE".to_string(),
            provenance: None,
            repair_chain: Vec::new(),
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
        "R" | ">" => n.r = v,
        "P" | "<" => n.p = v,
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
fn word_to_tuple(word: &str) -> IgTuple {
    let mut c = [0usize; 12];
    for ch in word.chars() {
        match ch {
            '⊢' => c[0] += 1,
            '⊣' => c[1] += 1,
            '>' => c[2] += 1,
            '<' => c[3] += 1,
            '⋈' => c[4] += 1,
            '⊤' => c[5] += 1,
            '∈' => c[6] += 1,
            '∋' => c[7] += 1,
            '⊙' => c[8] += 1,
            '⊥' => c[9] += 1,
            '⊞' => c[10] += 1,
            '◻' => c[11] += 1,
            _ => {}
        }
    }
    tuple_from_indices(&c)
}

/// FNV-1a over the hex string → 12 bytes → one value per axis.
fn hex_to_tuple(hex: &str) -> IgTuple {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut bytes = [0usize; 12];
    for (i, b) in hex.bytes().enumerate() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
        bytes[i % 12] ^= (h & 0xff) as usize;
    }
    tuple_from_indices(&bytes)
}

fn tuple_from_indices(idx: &[usize; 12]) -> IgTuple {
    IgTuple {
        d: D_VALS[idx[0] % D_VALS.len()],
        t: T_VALS[idx[1] % T_VALS.len()],
        r: R_VALS[idx[2] % R_VALS.len()],
        p: P_VALS[idx[3] % P_VALS.len()],
        f: F_VALS[idx[4] % F_VALS.len()],
        k: K_VALS[idx[5] % K_VALS.len()],
        g: G_VALS[idx[6] % G_VALS.len()],
        c: C_VALS[idx[7] % C_VALS.len()],
        phi: PHI_VALS[idx[8] % PHI_VALS.len()],
        h: H_VALS[idx[9] % H_VALS.len()],
        s: S_VALS[idx[10] % S_VALS.len()],
        omega: OM_VALS[idx[11] % OM_VALS.len()],
    }
}

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
    let base: u64 = 1 << 63;
    let factor: u64 = match tier {
        Some(Tier::OInf) => 1 << 20,
        Some(Tier::O2Dagger) => 1 << 15,
        Some(Tier::O2) => 1 << 10,
        Some(Tier::O1) => 1 << 5,
        _ => 1,
    };
    (base / factor).max(1)
}

fn window_bits(w: u64) -> u32 {
    63u32.saturating_sub(w.leading_zeros().min(63))
}

// ─── REPL surface ──────────────────────────────────────────────────────

pub fn sk_forge_main(args: &str) -> String {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let sub = parts.first().copied().unwrap_or("");

    match sub {
        "" | "help" => help().to_string(),

        "forge" => {
            if parts.len() < 2 {
                return "usage: sk_forge forge <pk_hex> [--max-repairs N]\n".to_string();
            }
            let mut max_repairs = 5usize;
            let mut i = 2;
            while i < parts.len() {
                if parts[i] == "--max-repairs" {
                    if let Some(v) = parts.get(i + 1) {
                        max_repairs = v.parse().unwrap_or(5);
                        i += 1;
                    }
                }
                i += 1;
            }
            let pk = PublicKey { hex: Some(parts[1].to_string()), tuple: None, word: None };
            let r = SkForge::new().with_max_repairs(max_repairs).forge(&pk);
            format_result(&r)
        }

        "tuple" => {
            if parts.len() < 2 {
                return "usage: sk_forge tuple <12-glyph tuple>\n".to_string();
            }
            match IgTuple::from_glyphs(parts[1]) {
                Ok(t) => {
                    let pk = PublicKey { hex: None, tuple: Some(t), word: None };
                    format_result(&SkForge::new().forge(&pk))
                }
                Err((pos, msg)) => format!("bad tuple at glyph {}: {}\n", pos, msg),
            }
        }

        "word" => {
            if parts.len() < 2 {
                return "usage: sk_forge word <imas_word>\n".to_string();
            }
            let pk = PublicKey { hex: None, tuple: None, word: Some(parts[1].to_string()) };
            format_result(&SkForge::new().forge(&pk))
        }

        "carriers" => {
            let cs = population();
            let mut out = format!("{} O_∞ carriers:\n", cs.len());
            for (i, c) in cs.iter().take(20).enumerate() {
                out.push_str(&format!("  {}. {} ({})\n", i + 1, c.name, c.domain));
            }
            if cs.len() > 20 {
                out.push_str(&format!("  ... and {} more\n", cs.len() - 20));
            }
            out
        }

        _ => help().to_string(),
    }
}

fn help() -> &'static str {
    "Crystal Harvester (sk_forge) — structural gap analysis against O_∞ carriers.

Usage:
  sk_forge forge <pk_hex> [--max-repairs N]   derive tuple from hex, analyse gap
  sk_forge tuple <12 glyphs>                  analyse a given tuple
  sk_forge word <imas_word>                   derive tuple from an opcode word
  sk_forge carriers                           list the O_∞ carriers

Pipeline: classify → nearest carrier → crystal-scope gap → repair path →
carrier provenance → bounded structural derivation.

The derivation recovers no real secret. Its scalar is HEURISTIC, over crystal
addresses; when the key sits in no carrier's basin the result is IMPOSSIBLE.
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
    if let Some(p) = &r.provenance {
        out.push_str(&format!("├─ carrier provenance: {}\n", p.name()));
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
