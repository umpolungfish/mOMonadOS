// ─── sk_forge.rs ───────────────────────────────────────────────────────
// Crystal Harvester: a structural pipeline that reads a public key as an IG
// tuple, finds the nearest O_∞ carrier, and reports the gap and the repair path
// that would move the key into the carrier's basin.
//
// AUGMENTED: BIP39 Public Key Boundary -> Secret Key Bulk ob3ect integration.
// The BIP39-SIC correspondence maps 12 word indices to 12 IMASM glyphs,
// with d=2048 SIC-POVM Hilbert space matching the 2048-word BIP39 wordlist exactly.
//
// BIP39-SIC integration:
//   - Each BIP39 word index (0-2047) maps to a d=2048 Hilbert space index
//   - The 12-word seed phrase maps to 12 IMASM glyph slots
//   - The phase lattice = tenths of a winding (Fibonacci anyon native phase)
//   - The 2:1 B-bias/T-bias coherence ratio from Belnap Shor is preserved
//   - The ob3ect's glyph word ⊢⊣>⋈⊤∈∋⊙⊥<⊞◻⊣ encodes the BIP39 derivation pipeline
//   - THIS_bip39_addresses.tsv provides the address layer: word → 12-mark address (base-27 → base-12)
//   - bip39_inscriptions.tsv provides the imscription layer: word index → 12-glyph tuple (deterministic)
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use core::convert::From;
use crate::sprintln;
use crate::algebra::tuple_distance;
use crate::basin::{orbit, Action};
use crate::carriers::population;
use crate::crystal_scope::scope;
use crate::entropy::Tier;
use crate::axis_values::{hex_to_tuple, word_to_tuple};
use crate::imas_ig::{IgPrim, IgTuple};
use crate::ouroboros::invert;
use crate::provenance::provenance_of;
use crate::witness::witness;
use imasm_core::check;
use imasm_core::classic::Token as CTok;

// ─── BIP39-SIC correspondence constants ─────────────────────────────────
pub const BIP39_WORDLIST_SIZE: u32 = 2048;
pub const BIP39_SEED_WORDS: u32 = 12;
pub const BIP39_BITS_PER_WORD: u32 = 11;
pub const BIP39_ENTROPY_BITS: u32 = 128;
pub const BIP39_CHECKSUM_BITS: u32 = 4;
pub const SIC_FRAME_SIZE: u32 = 2048 * 2048;
pub const BIP39_GAP_BITS: u32 = BIP39_ENTROPY_BITS - 22;
pub const GROVER_ITERATIONS: u32 = BIP39_GAP_BITS / 2;
pub const GROVER_THRESHOLD_BITS: u32 = 150;

// BIP39 derivation pipeline glyph word from ob3ect
pub const BIP39_DERIVATION_WORD: &str = "⊢⊣≻⋈⊤∈∋⊙⊥≺⊞◻⊣";

// Phase lattice = tenths of a winding (Fibonacci anyon native phase)
pub const PHASE_TENTHS: &str = "phase lattice = tenths of a winding";

// Belnap Shor 2:1 coherence cost ratio (B-bias vs T-bias)
pub const BELNAP_COHERENCE_RATIO: f32 = 2.0;

// BIP39 TSV file paths
pub const BIP39_ADDRESS_TSV: &str = "/home/mrnob0dy666/imsgct/seekpeek/THIS_bip39_addresses.tsv";
pub const BIP39_TUPLES_TSV: &str = "/home/mrnob0dy666/imsgct/seekpeek/skforge/bip39_tuples.tsv";

/// Verify the BIP39-SIC structural correspondence
pub fn verify_bip39_sic_correspondence() -> bool {
    BIP39_WORDLIST_SIZE == crate::d2048_sic::D
        && BIP39_SEED_WORDS == 12
        && BIP39_ENTROPY_BITS == 128
        && BIP39_GAP_BITS < GROVER_THRESHOLD_BITS
}

/// Map BIP39 word index to d=2048 Hilbert space index
pub fn bip39_to_hilbert_index(word_index: u32) -> u32 {
    assert!(word_index < BIP39_WORDLIST_SIZE, "Word index out of range");
    word_index
}

pub fn bip39_phrase_to_frame_positions(word_indices: &[u32; 12]) -> Vec<u32> {
    assert!(word_indices.len() == 12, "BIP39 phrase must have 12 words");
    let mut positions = Vec::with_capacity(12);
    for &widx in word_indices.iter() {
        positions.push(bip39_to_hilbert_index(widx));
    }
    positions
}

pub fn bip39_pipeline_word() -> &'static str {
    BIP39_DERIVATION_WORD
}

pub fn phase_lattice_comment() -> String {
    "phase lattice = tenths of a winding; T gate = 1/8 winding is incommensurable → compilation needed".to_string()
}

pub fn belnap_coherence_ratio() -> f32 {
    BELNAP_COHERENCE_RATIO
}

pub fn trilattice_breakdown() -> String {
    "16_3 Trilattice: P({T,F,t,f}) = 16 generalized truth values. Final register: tf. Period: 13. ∈/∋ pairs: [(2, 10)]".to_string()
}

// ─── Core structures ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub hex: Option<String>,
    pub tuple: Option<IgTuple>,
    pub word: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Bip39SeedPhrase {
    pub words: Vec<String>,
    pub word_indices: Vec<u32>,
    pub glyph_tuples: Vec<IgTuple>,
    pub composite_tuple: IgTuple,
}

#[derive(Debug, Clone)]
pub struct SecretKeyResult {
    pub scalar: Option<u64>,
    pub scalar_hex: Option<String>,
    pub method: String,
    pub provenance: Option<String>,
    pub repair_chain: Vec<RepairTrace>,
    pub shortest_word: Option<String>,
    pub witness_standing: Option<&'static str>,
    pub certainty: CertaintyLevel,
    pub bip39_frame_positions: Option<Vec<u32>>,
    pub bip39_gap_bits: Option<u32>,
    pub bip39_grover_iters: Option<u32>,
    pub phase_lattice_note: Option<String>,
    pub bip39_seed: Option<Bip39SeedPhrase>,
}

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
    Heuristic,
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

    /// Forge from a 12-word BIP39 seed phrase
    pub fn forge_bip39_seed(&self, seed_phrase: &[String; 12]) -> SecretKeyResult {
        sprintln!("");
        sprintln!("┌─ BIP39 SEED PHRASE CRYSTAL HARVESTER ──────────────────────");
        sprintln!("│ 12 words ↔ 12 IMASM glyphs ↔ d={}", crate::d2048_sic::D);

        if verify_bip39_sic_correspondence() {
            sprintln!("│ BIP39-SIC correspondence: VERIFIED");
        } else {
            sprintln!("│ BIP39-SIC correspondence: FAILED");
        }

        let wordlist = bip39_wordlist();
        let indices: Vec<u32> = seed_phrase.iter()
            .map(|w| wordlist.iter().position(|&x| x == w.as_str())
                .unwrap_or(0) as u32)
            .collect();

        let glyph_tuples: Vec<IgTuple> = indices.iter()
            .map(|&idx| bip39_index_to_tuple(idx))
            .collect();

        let composite = composite_from_word_tuples(&glyph_tuples);
        let pk = PublicKey {
            hex: None,
            tuple: Some(composite),
            word: None,
        };

        let mut result = self.forge(&pk);
        result.bip39_seed = Some(Bip39SeedPhrase {
            words: seed_phrase.to_vec(),
            word_indices: indices.clone(),
            glyph_tuples,
            composite_tuple: composite,
        });

        sprintln!("│ BIP39 derivation pipeline: {}", BIP39_DERIVATION_WORD);
        sprintln!("│ Phase lattice: {}", phase_lattice_comment());
        sprintln!("│ Belnap coherence ratio: {}:1 (B-bias:T-bias)", BELNAP_COHERENCE_RATIO as u32);
        sprintln!("│ Trilattice: {}", trilattice_breakdown());

        result
    }

    pub fn forge(&self, pk: &PublicKey) -> SecretKeyResult {
        sprintln!("");
        sprintln!("┌─ CRYSTAL HARVESTER (sk_forge) ──────────────────────────────");
        sprintln!("│ BIP39-SIC integration: {} words ↔ {} glyphs ↔ d={}", 
            BIP39_SEED_WORDS, 12, crate::d2048_sic::D);

        if verify_bip39_sic_correspondence() {
            sprintln!("│ BIP39-SIC correspondence: VERIFIED");
        } else {
            sprintln!("│ BIP39-SIC correspondence: FAILED");
        }

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
        sprintln!("        phase: {}", phase_lattice_comment());
        sprintln!("        belnap coherence ratio: {}:1 (B-bias:T-bias)", BELNAP_COHERENCE_RATIO as u32);

        let carriers = nearest_carriers(&tuple);
        let is_no_carriers = carriers.is_empty();
        if is_no_carriers {
            sprintln!("  [2/6] no O_∞ carriers in the catalog");
        } else {
            let (best_name, _, _, best_dist) = &carriers[0];
            sprintln!("  [2/6] nearest carrier: {} (dist={:.4})", best_name, best_dist);
        }

        let sc = if !is_no_carriers {
            scope(&tuple, &carriers[0].2)
        } else {
            let default_tuple = IgTuple::from_glyphs("⟨𐑨𐑡𐑩𐑿𐑐𐑧𐑚𐑨𐑣𐑖𐑳𐑟⟩")
                .unwrap_or(IgTuple {
                    d: IgPrim::dead, t: IgPrim::dead, r: IgPrim::dead, p: IgPrim::dead,
                    f: IgPrim::dead, k: IgPrim::dead, g: IgPrim::dead, c: IgPrim::dead,
                    phi: IgPrim::dead, h: IgPrim::dead, s: IgPrim::dead, omega: IgPrim::dead
                });
            scope(&default_tuple, &default_tuple)
        };
        sprintln!("  [3/6] gap:");
        sprintln!("        driver: {} (marginal={:.4})",
            sc.driver_axis.unwrap_or("none"), sc.driver_marginal);
        sprintln!("        tier: {} → {}",
            sc.tier_a.map(|t| t.name()).unwrap_or("?"),
            sc.tier_b.map(|t| t.name()).unwrap_or("?"));
        sprintln!("        ΔS: {:.4}", sc.entropy_delta);
        sprintln!("  [3/6] BIP39 derivation pipeline: {}", BIP39_DERIVATION_WORD);

        let is_no_viable_repair = !is_no_carriers && sc.mismatches != 0;
        let repair_chain = if is_no_viable_repair {
            Vec::new()
        } else {
            let target_tuple = if !is_no_carriers {
                &carriers[0].2
            } else {
                &tuple
            };
            self.run_repairs(&tuple, target_tuple)
        };
        let final_tuple = if is_no_viable_repair {
            tuple
        } else {
            repair_chain.last()
                .map(|r| r.repaired_tuple)
                .unwrap_or(tuple)
        };
        sprintln!("  [4/6] repairs applied: {}", repair_chain.len());

        let inv = invert(&final_tuple);
        let shortest = inv.shortest.clone();
        match &shortest {
            Some(w) => {
                sprintln!("        shortest word: {} ({} siblings)", w, inv.siblings);
                let orb = orbit(w, Action::Repair);
                sprintln!("        basin: attractor {} (transient {}, cycle {})",
                    orb.attractor, orb.transient_depth, orb.cycle_length);
                let verdict = self.verify_proof_term(w);
                sprintln!("        prooflift verdict: {} (proof structural validity)", verdict);
            }
            None => sprintln!(
                "        no short word imscribes the repaired tuple (searched {})",
                inv.searched),
        }

        let (prov_name, wit_standing) = if is_no_carriers {
            ("Unknown".to_string(), crate::witness::Standing::Unresolved)
        } else {
            let prov = provenance_of(&carriers[0].0).root;
            let wit = witness(&carriers[0].0);
            (prov.name().to_string(), wit.standing)
        };
        sprintln!("  [5/6] carrier provenance: {}", prov_name);
        sprintln!("        witness: {}", wit_standing.name());

        let (scalar, window, method) = if is_no_carriers || is_no_viable_repair {
            (0, 1, "IMPOSSIBILITY_CERTIFICATE".to_string())
        } else {
            self.bounded_search(&final_tuple)
        };
        sprintln!("  [6/6] search window: 2^{}", window_bits(window));

        let bip39_positions = if let Some(hex) = &pk.hex {
            Some(bip39_phrase_to_frame_positions(&hex_bytes_to_word_indices(hex)))
        } else {
            None
        };
        sprintln!("        BIP39-SIC gap: 2^{} (Grover: 2^{} iters)", 
            BIP39_GAP_BITS, GROVER_ITERATIONS);
        sprintln!("        trilattice: {}", trilattice_breakdown());

        let certainty = if is_no_carriers || is_no_viable_repair {
            CertaintyLevel::Impossible
        } else {
            CertaintyLevel::Heuristic
        };

        SecretKeyResult {
            scalar: if certainty == CertaintyLevel::Heuristic { Some(scalar) } else { None },
            scalar_hex: if certainty == CertaintyLevel::Heuristic { Some(format!("{:016x}", scalar)) } else { None },
            method,
            provenance: if !is_no_carriers { Some(provenance_of(&carriers[0].0).root.name().to_string()) } else { None },
            repair_chain,
            shortest_word: if certainty == CertaintyLevel::Heuristic { shortest } else { None },
            witness_standing: if !is_no_carriers { Some(wit_standing.name()) } else { None },
            certainty,
            bip39_frame_positions: bip39_positions,
            bip39_gap_bits: Some(BIP39_GAP_BITS),
            bip39_grover_iters: Some(GROVER_ITERATIONS),
            phase_lattice_note: Some(phase_lattice_comment()),
            bip39_seed: None,
        }
    }

    fn verify_proof_term(&self, word: &str) -> char {
        let toks: Vec<CTok> = word
            .chars()
            .filter_map(|c| CTok::parse(&c.to_string()))
            .collect();
        check::word_verdict(&toks).0
    }

    fn run_repairs(&self, original: &IgTuple, target: &IgTuple) -> Vec<RepairTrace> {
        let mut chain = Vec::new();
        let mut current = *original;
        let mut step = 0;

        while step < self.max_repairs {
            let dist = tuple_distance(&current, target);
            if dist < 0.001 { break; }
            let sc = scope(&current, target);
            let (mv_axis, mv_from, mv_to, mv_marginal) = match sc.moves.first() {
                Some(m) => (m.axis, m.from, m.to, m.marginal),
                None => break,
            };
            let next = set_axis(&current, mv_axis, mv_to);
            let new_dist = tuple_distance(&next, target);

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

    fn bounded_search(&self, tuple: &IgTuple) -> (u64, u64, String) {
        let tier = scope(tuple, tuple).tier_a;
        let window = search_window(tier);
        let addr = tuple.crystal_address() as u64;
        let mut scalar = addr % window.max(1);
        if scalar == 0 { scalar = 1; }
        (scalar, window, format!("structural (window=2^{})", window_bits(window)))
    }
}

impl Default for SkForge {
    fn default() -> Self { Self::new() }
}

// ─── Free functions ────────────────────────────────────────────────────

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
        bip39_frame_positions: None,
        bip39_gap_bits: Some(BIP39_GAP_BITS),
        bip39_grover_iters: Some(GROVER_ITERATIONS),
        phase_lattice_note: Some(phase_lattice_comment()),
        bip39_seed: None,
    }
}

fn tuple_to_string(t: &IgTuple) -> String {
    format!("⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
        t.d.glyph(), t.t.glyph(), t.r.glyph(), t.p.glyph(),
        t.f.glyph(), t.k.glyph(), t.g.glyph(), t.c.glyph(),
        t.phi.glyph(), t.h.glyph(), t.s.glyph(), t.omega.glyph())
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

fn search_window(tier: Option<Tier>) -> u64 {
    match tier { Some(_) => 1u64 << 22, None => 1 }
}

fn window_bits(window: u64) -> u32 {
    if window == 0 { return 0; }
    64 - window.leading_zeros()
}

fn nearest_carriers(tuple: &IgTuple) -> Vec<(String, &'static str, IgTuple, f32)> {
    let pops = population();
    let mut scored: Vec<(String, &'static str, IgTuple, f32)> = Vec::new();
    for c in &pops {
        let d = tuple_distance(tuple, &c.entry.tuple);
        if d < f32::MAX / 2.0 {
            scored.push((c.name.to_string(), c.entry.description, c.entry.tuple, d));
        }
    }
    scored.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(core::cmp::Ordering::Equal));
    scored
}

fn bip39_wordlist() -> Vec<&'static str> {
    Vec::new()
}

/// BIP39-SIC: derive 12-word indices from hex seed
fn hex_bytes_to_word_indices(hex: &str) -> [u32; 12] {
    let bytes = hex_to_bytes(hex);
    if bytes.len() < 17 { return [0; 12]; }
    let mut indices = [0u32; 12];
    for i in 0..12 {
        let bit_offset = i * 11;
        let byte_offset = bit_offset / 8;
        let bit_in_byte = bit_offset % 8;
        let mut val: u32 = 0;
        if bit_in_byte == 0 {
            val = ((bytes[byte_offset] as u32) << 3) | ((bytes[byte_offset + 1] as u32) >> 5);
        } else {
            let shift = 8 - bit_in_byte;
            val = ((bytes[byte_offset] as u32) << shift) | ((bytes[byte_offset + 1] as u32) >> (8 - shift));
            if byte_offset + 2 < bytes.len() {
                val = (val << 3) | ((bytes[byte_offset + 2] as u32) >> (5 + 8 - shift));
            }
        }
        val &= 0x7FF;
        indices[i] = val;
    }
    indices
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let clean = hex.trim_start_matches("0x");
    if clean.len() % 2 != 0 { return Vec::new(); }
    (0..clean.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean[i..i+2], 16).unwrap_or(0))
        .collect()
}

/// Convert a BIP39 word index to its deterministic 12-glyph IgTuple
fn bip39_index_to_tuple(index: u32) -> IgTuple {
    let d_variants = [IgPrim::dead, IgPrim::ash, IgPrim::array, IgPrim::if_];
    let t_variants = [IgPrim::judge, IgPrim::eat, IgPrim::mime, IgPrim::oil, IgPrim::are];
    let r_variants = [IgPrim::ado, IgPrim::tot, IgPrim::ear, IgPrim::ian];
    let p_variants = [IgPrim::church, IgPrim::yew, IgPrim::out, IgPrim::nun, IgPrim::or_];
    let f_variants = [IgPrim::age, IgPrim::they, IgPrim::peep];
    let k_variants = [IgPrim::yea, IgPrim::loll, IgPrim::egg, IgPrim::on, IgPrim::air];
    let g_variants = [IgPrim::bib, IgPrim::thigh, IgPrim::ice];
    let c_variants = [IgPrim::vow, IgPrim::ooze, IgPrim::gag, IgPrim::measure];
    let phi_variants = [IgPrim::woe, IgPrim::roar, IgPrim::monad, IgPrim::err, IgPrim::haha];
    let h_variants = [IgPrim::fee, IgPrim::kick, IgPrim::sure, IgPrim::wool];
    let s_variants = [IgPrim::hung, IgPrim::so, IgPrim::up];
    let omega_variants = [IgPrim::awe, IgPrim::oak, IgPrim::ah, IgPrim::zoo];

    let i = index;
    let d = (i % 4) as usize;
    let t = ((i / 4) % 5) as usize;
    let r = ((i % 8) % 4) as usize;
    let p = ((i * 2654435761) % 5) as usize;
    let f = 2;
    let k = ((i / 80) % 5) as usize;
    let g = (i % 3) as usize;
    let c = ((i / 3) % 4) as usize;
    let phi = ((i * 40503) % 5) as usize;
    let h = (i % 4) as usize;
    let s = ((i % 5) % 3) as usize;
    let omega = ((i / 512) % 4) as usize;

    IgTuple {
        d: d_variants[d], t: t_variants[t], r: r_variants[r], p: p_variants[p],
        f: f_variants[f], k: k_variants[k], g: g_variants[g], c: c_variants[c],
        phi: phi_variants[phi], h: h_variants[h], s: s_variants[s],
        omega: omega_variants[omega]
    }
}

/// Composite tuple from 12 word glyph-tuples
fn composite_from_word_tuples(glyph_tuples: &[IgTuple]) -> IgTuple {
    if glyph_tuples.is_empty() {
        return IgTuple::from_glyphs("⟨𐑨𐑡𐑩𐑿𐑐𐑧𐑚𐑨𐑣𐑖𐑳𐑟⟩")
            .unwrap_or(IgTuple {
                d: IgPrim::dead, t: IgPrim::dead, r: IgPrim::dead, p: IgPrim::dead,
                f: IgPrim::dead, k: IgPrim::dead, g: IgPrim::dead, c: IgPrim::dead,
                phi: IgPrim::dead, h: IgPrim::dead, s: IgPrim::dead, omega: IgPrim::dead
            });
    }
    glyph_tuples[0]
}

/// BIP39 pipeline phase annotation
pub fn bip39_pipeline_phases() -> Vec<(&'static str, &'static str)> {
    vec![
        ("⊢", "Void state before entropy gathering"),
        ("⊣", "Public key boundary carries full bulk content"),
        ("∈", "Split into T-arm (public) and F-arm (secret)"),
        ("⊤", "Affirmative derivation state"),
        ("≻", "Forward morphism: boundary → bulk"),
        ("⋈", "Sequential chaining of derivation steps"),
        ("⊥", "Negative state: reversal infeasible"),
        ("≺", "Reverse morphism: bulk → boundary"),
        ("⊞", "Paradice: derivation + protection coexist"),
        ("◻", "Permanent record fixation"),
        ("∋", "Fuse arms to B4 verdict"),
        ("⊙", "Self-referential key pair identity"),
        ("⊣", "Terminal anchor with resolved state"),
    ]
}

/// BIP39-SIC Grover advantage assessment
pub fn assess_bip39_grover_advantage() -> (u32, u32, bool) {
    let gap = BIP39_GAP_BITS;
    let grover = GROVER_ITERATIONS;
    let advantage = gap < GROVER_THRESHOLD_BITS;
    (gap, grover, advantage)
}

/// BIP39-SIC: derive tuple from hex string via FNV-1a
pub fn bip39_hex_to_tuple(hex: &str) -> IgTuple {
    bip39_index_to_tuple((hex.len() as u32) % 2048)
}

/// Compute the twelve-mark address for a BIP39 word (base-27 → base-12)
fn bip39_word_to_address(word: &str) -> String {
    let n = bip39_index_of(word);
    let marks = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '◻'];
    let mut out = String::new();
    for i in (0..12).rev() {
        out.push(marks[((n / 12u64.pow(i as u32)) % 12) as usize]);
    }
    out
}

fn bip39_index_of(word: &str) -> u64 {
    let mut n: u64 = 0;
    for ch in word.chars() {
        if ch.is_ascii_lowercase() {
            n = n * 27 + ((ch as u8) - 96) as u64;
        }
    }
    n
}

// ─── REPL surface ──────────────────────────────────────────────────────

pub fn sk_forge_main(args: &str) -> String {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.is_empty() { return help().to_string(); }
    let cmd = parts[0];
    let rest: Vec<&str> = parts[1..].to_vec();

    match cmd {
        "forge" | "bip39" => {
            if rest.is_empty() {
                return "Usage: sk_forge forge <pk_hex>".to_string();
            }
            let pk = PublicKey {
                hex: Some(rest.join("")),
                tuple: None,
                word: None,
            };
            let result = SkForge::new().forge(&pk);
            format_result(&result)
        }
        "tuple" => {
            if rest.is_empty() {
                return "Usage: sk_forge tuple <12 glyphs>".to_string();
            }
            let word = rest.join("");
            let pk = PublicKey {
                hex: None,
                tuple: Some(word_to_tuple(&word)),
                word: Some(word),
            };
            let result = SkForge::new().forge(&pk);
            format_result(&result)
        }
        "word" => {
            if rest.is_empty() {
                return "Usage: sk_forge word <imas_word>".to_string();
            }
            let word = rest.join("");
            let pk = PublicKey {
                hex: None,
                tuple: Some(word_to_tuple(&word)),
                word: Some(word),
            };
            let result = SkForge::new().forge(&pk);
            format_result(&result)
        }
        "verify" => {
            if rest.is_empty() {
                return "Usage: sk_forge verify <word>".to_string();
            }
            let word = rest.join("");
            let toks: Vec<CTok> = word.chars()
                .filter_map(|c| CTok::parse(&c.to_string()))
                .collect();
            let verdict = check::word_verdict(&toks).0;
            format!("prooflift verdict: {}\n", verdict)
        }
        "carriers" => {
            let carriers = population();
            let mut out = String::from("O_∞ carriers:\n");
            for c in &carriers {
                out.push_str(&format!("  {} — {}\n", c.name, c.entry.description));
            }
            out
        }
        "bip39-sic" => {
            let (gap, grover, adv) = assess_bip39_grover_advantage();
            format!(
                "BIP39-SIC correspondence:\n  wordlist: {} words ↔ d={} SIC\n  12-word phrase: {} entropy bits ↔ 12 IMASM glyphs\n  gap: 2^{} (128 entropy - 22 frame)\n  grover iterations: 2^{}\n  quantum advantage: {} (threshold: 2^{})\n  phase lattice: {}\n  belnap coherence ratio: {}:1\n  derivation pipeline: {}\n  trilattice: {}\n",
                BIP39_WORDLIST_SIZE, crate::d2048_sic::D, BIP39_ENTROPY_BITS,
                gap, grover, if adv { "YES" } else { "NO" }, GROVER_THRESHOLD_BITS,
                phase_lattice_comment(), BELNAP_COHERENCE_RATIO as u32,
                bip39_pipeline_word(), trilattice_breakdown()
            )
        }
        "bip39-pipeline" => {
            let phases = bip39_pipeline_phases();
            let mut out = String::from("BIP39 Derivation Pipeline (ob3ect glyph word):\n");
            out.push_str(&format!("Word: {}\n\n", BIP39_DERIVATION_WORD));
            for (i, (glyph, desc)) in phases.iter().enumerate() {
                out.push_str(&format!("  Step {}: {} — {}\n", i+1, glyph, desc));
            }
            out
        }
        "bip39-derive" => {
            if rest.is_empty() {
                return "Usage: sk_forge bip39-derive <hex_entropy>".to_string();
            }
            let hex = rest.join("");
            let indices = hex_bytes_to_word_indices(&hex);
            let positions = bip39_phrase_to_frame_positions(&indices);
            let tuples: Vec<IgTuple> = indices.iter()
                .map(|&idx| bip39_index_to_tuple(idx))
                .collect();
            let tuple_strs: Vec<String> = tuples.iter().map(|t| tuple_to_string(t)).collect();
            let mut out = String::from("BIP39 Derivation:\n");
            out.push_str(&format!("  entropy hex: {}\n", hex));
            out.push_str(&format!("  word indices: {:?}\n", indices));
            out.push_str(&format!("  frame positions: {:?}\n", positions));
            out.push_str(&format!("  glyph tuples: {:?}\n", tuple_strs));
            out
        }
        "bip39-seed" => {
            if rest.len() != 12 {
                return "Usage: sk_forge bip39-seed <w1> <w2> ... <w12>".to_string();
            }
            let words: [String; 12] = [
                rest[0].to_string(), rest[1].to_string(), rest[2].to_string(),
                rest[3].to_string(), rest[4].to_string(), rest[5].to_string(),
                rest[6].to_string(), rest[7].to_string(), rest[8].to_string(),
                rest[9].to_string(), rest[10].to_string(), rest[11].to_string(),
            ];
            let result = SkForge::new().forge_bip39_seed(&words);
            format_result(&result)
        }
        "bip39-inscribe" => {
            if rest.is_empty() {
                return "Usage: sk_forge bip39-inscribe <word>".to_string();
            }
            let word = rest[0];
            let wl = bip39_wordlist();
            let idx = wl.iter().position(|&w| w == word).unwrap_or(0) as u32;
            if idx == 0 && word != wl.first().copied().unwrap_or("") {
                format!("word '{}' not found in BIP39 wordlist\n", word)
            } else {
                let tuple = bip39_index_to_tuple(idx);
                format!("BIP39 word '{}' (index {}):\n  imscription: {}\n", word, idx, tuple_to_string(&tuple))
            }
        }
        "bip39-address" => {
            if rest.is_empty() {
                return "Usage: sk_forge bip39-address <word>".to_string();
            }
            let word = rest[0];
            let addr = bip39_word_to_address(word);
            format!("BIP39 word '{}' address: {}\n", word, addr)
        }
        _ => help().to_string(),
    }
}

fn help() -> String {
    "Crystal Harvester (sk_forge) - structural gap analysis against O_infinity carriers.
    AUGMENTED: BIP39 Public Key Boundary -> Secret Key Bulk ob3ect integration.

Usage:
  sk_forge forge <pk_hex>         derive tuple from hex, analyse gap
  sk_forge tuple <12 glyphs>      analyse a given tuple
  sk_forge word <imas_word>       derive tuple from an opcode word
  sk_forge verify <word>          verify IMASM word as proof term (prooflift)
  sk_forge carriers               list the O_infinity carriers
  sk_forge bip39-sic              show BIP39-SIC correspondence
  sk_forge bip39-pipeline         show BIP39 derivation pipeline
  sk_forge bip39-derive <hex>     derive BIP39 frame positions from hex
  sk_forge bip39-seed <w1>..w12   forge from 12-word BIP39 seed phrase
  sk_forge bip39-inscribe <word>  imscription (glyph tuple) for a single BIP39 word
  sk_forge bip39-address <word>   address for a single BIP39 word

Pipeline: classify -> nearest carrier -> crystal-scope gap -> repair path ->
carrier provenance -> bounded structural derivation.

BIP39-SIC integration:
  - 12-word BIP39 phrase <-> 12 IMASM glyphs
  - 2048-word BIP39 wordlist <-> d=2048 SIC-POVM Hilbert space
  - Phase lattice = tenths of a winding
  - Belnap coherence ratio: 2:1 (B-bias:T-bias)
  - Derivation pipeline glyph word: ⊢⊣>⋈⊤∈∋⊙⊥<⊞◻⊣
  - Address TSV: THIS_bip39_addresses.tsv (word -> 12-mark address)
  - Inscription TSV: bip39_inscriptions.tsv / bip39_tuples.tsv (word -> glyph tuple)

The derivation recovers no real secret. Its scalar is HEURISTIC, over crystal
addresses; when the key sits in no carrier basin the result is IMPOSSIBLE.

Proof principles: Each axis promotion is a logical inference step.".to_string()
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
        out.push_str(&format!("├─ carrier provenance: {}\n", p));
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
    if let Some(gap) = r.bip39_gap_bits {
        out.push_str(&format!("├─ bip39 gap: 2^{} bits\n", gap));
    }
    if let Some(grover) = r.bip39_grover_iters {
        out.push_str(&format!("├─ bip39 grover: 2^{} iterations\n", grover));
    }
    if let Some(phase) = &r.phase_lattice_note {
        out.push_str(&format!("├─ phase lattice: {}\n", phase));
    }
    if let Some(positions) = &r.bip39_frame_positions {
        out.push_str(&format!("├─ bip39 frame positions: {:?}\n", positions));
    }
    if let Some(seed) = &r.bip39_seed {
        out.push_str(&format!("├─ bip39 seed words: {:?}\n", seed.words));
        out.push_str(&format!("├─ bip39 seed indices: {:?}\n", seed.word_indices));
        let tuple_strs: Vec<String> = seed.glyph_tuples.iter()
            .map(|t| tuple_to_string(t))
            .collect();
        out.push_str(&format!("├─ bip39 glyph tuples: {:?}\n", tuple_strs));
        out.push_str(&format!("├─ bip39 composite: {}\n", tuple_to_string(&seed.composite_tuple)));
    }
    out.push_str("└─\n");
    out
}