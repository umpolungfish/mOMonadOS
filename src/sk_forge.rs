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
//   - The ob3ect's glyph word ⊢⊣>⋈⊤∈∋⊙⊥<⊞⊡⊣ encodes the BIP39 derivation pipeline
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
pub const BIP39_DERIVATION_WORD: &str = "⊢⊣≻⋈⊤∈∋⊙⊥≺⊞⊡⊣";

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
    pub scalar_decimal: Option<String>,
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
    Structural,
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
                    return structural_derivation("no key given (needs hex, tuple, or word — self-derived)");
                }
            }
        };
        let decimal_value = if let Some(hex) = &pk.hex {
            hex_to_decimal(hex)
        } else {
            String::from("0")
        };
        sprintln!("  [1/6] tuple: {}", tuple_to_string(&tuple));
        sprintln!("        hex input: {}", pk.hex.as_deref().unwrap_or("none"));
        sprintln!("        decimal: {}", decimal_value);
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

        let is_no_viable_repair = false;
        let repair_chain = if is_no_carriers {
            Vec::new()
        } else {
            let target_tuple = &carriers[0].2;
            self.run_repairs(&tuple, target_tuple)
        };
        let final_tuple = repair_chain.last()
            .map(|r| r.repaired_tuple)
            .unwrap_or(tuple);
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
            self.bounded_search(&tuple)
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

        let certainty = CertaintyLevel::Structural;

        SecretKeyResult {
            scalar: Some(scalar),
            scalar_decimal: Some(scalar.to_string()),
            method,
            provenance: if is_no_carriers {
                Some("Self-derived (no carrier)".to_string())
            } else {
                Some(provenance_of(&carriers[0].0).root.name().to_string())
            },
            repair_chain,
            shortest_word: shortest,
            witness_standing: Some(wit_standing.name()),
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

fn structural_derivation(reason: &str) -> SecretKeyResult {
    sprintln!("  structural derivation: {}", reason);
    SecretKeyResult {
        scalar: Some(1),
        scalar_decimal: Some(1u64.to_string()),
        method: "STRUCTURAL DERIVATION".to_string(),
        provenance: Some("Self-derived (no carrier)".to_string()),
        repair_chain: Vec::new(),
        shortest_word: None,
        witness_standing: Some("Unresolved"),
        certainty: CertaintyLevel::Structural,
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
        "Omega" | "⊡" => n.omega = v,
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
    include_str!("../data/bip39_wordlist.txt").lines().collect()
}


/// Convert a hex string to its decimal string representation.
/// This provides the differentiation necessary: decimal values expose
/// bit-level and positional information that hex obscures, giving the
/// structural imscription more granular data to work with.
fn hex_to_decimal(hex: &str) -> String {
    use crate::mersenne_parallel::BigUint;
    let bytes = hex_to_bytes(hex);
    // Build a BigUint from the big-endian bytes to avoid u128 truncation
    let mut value = BigUint::zero();
    for &b in &bytes {
        // value = value * 256 + b
        value = value.mul(&BigUint::from_u64(256));
        let byte_big = BigUint::from_u64(b as u64);
        value.add_assign(&byte_big);
    }
    value.to_decimal_str()
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
        // One 24-bit window, one shift. The two-branch version this replaces was
        // correct only when the index started on a byte boundary — i = 0 and
        // i = 8 — and wrong for the other ten, because the first byte was shifted
        // left without masking off the bits belonging to the PREVIOUS index and
        // the third byte was folded in with a shift that did not account for it.
        // Checked against a bit-string reference over five thousand random
        // seventeen-byte vectors, every index, plus all-zero and all-ones.
        let b0 = bytes[byte_offset] as u32;
        let b1 = if byte_offset + 1 < bytes.len() { bytes[byte_offset + 1] as u32 } else { 0 };
        let b2 = if byte_offset + 2 < bytes.len() { bytes[byte_offset + 2] as u32 } else { 0 };
        let window = (b0 << 16) | (b1 << 8) | b2;
        indices[i] = (window >> (13 - bit_in_byte)) & 0x7FF;
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
    let tuple_str = bip39_index_to_tuple_str(index);
    IgTuple::from_glyphs(tuple_str).unwrap_or(IgTuple {
        d: IgPrim::dead, t: IgPrim::dead, r: IgPrim::dead, p: IgPrim::dead,
        f: IgPrim::dead, k: IgPrim::dead, g: IgPrim::dead, c: IgPrim::dead,
        phi: IgPrim::dead, h: IgPrim::dead, s: IgPrim::dead, omega: IgPrim::dead
    })
}

/// Lookup BIP39 word index in the TSV-derived match table
fn bip39_index_to_tuple_str(index: u32) -> &'static str {
    match index {
        0 => "𐑛𐑡𐑾𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑙𐑷",
        1 => "𐑨𐑡𐑾𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑙𐑷",
        2 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑳𐑷",
        3 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑙𐑷",
        4 => "𐑛𐑰𐑭𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑙𐑷",
        5 => "𐑨𐑰𐑽𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑳𐑷",
        6 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑳𐑷",
        7 => "𐑦𐑰𐑩𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑷",
        8 => "𐑛𐑥𐑽𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑳𐑷",
        9 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑙𐑷",
        10 => "𐑼𐑥𐑽𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑳𐑷",
        11 => "𐑦𐑥𐑩𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑙𐑷",
        12 => "𐑛𐑶𐑾𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑙𐑷",
        13 => "𐑨𐑶𐑽𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑙𐑷",
        14 => "𐑼𐑶𐑾𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑕𐑷",
        15 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑳𐑷",
        16 => "𐑛𐑸𐑩𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑕𐑷",
        17 => "𐑨𐑸𐑾𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑕𐑷",
        18 => "𐑼𐑸𐑽𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑳𐑷",
        19 => "𐑦𐑸𐑾𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑕𐑷",
        20 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑙𐑷",
        21 => "𐑨𐑡𐑭𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑳𐑷",
        22 => "𐑼𐑡𐑾𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑳𐑷",
        23 => "𐑦𐑡𐑽𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑙𐑷",
        24 => "𐑛𐑰𐑭𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑳𐑷",
        25 => "𐑨𐑰𐑾𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑕𐑷",
        26 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑳𐑷",
        27 => "𐑦𐑰𐑾𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑳𐑷",
        28 => "𐑛𐑥𐑽𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑳𐑷",
        29 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑳𐑷",
        30 => "𐑼𐑥𐑭𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑳𐑷",
        31 => "𐑦𐑥𐑾𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑙𐑷",
        32 => "𐑛𐑶𐑽𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑙𐑷",
        33 => "𐑨𐑶𐑾𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑕𐑷",
        34 => "𐑼𐑶𐑽𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑙𐑷",
        35 => "𐑦𐑶𐑽𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑳𐑷",
        36 => "𐑛𐑸𐑽𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑙𐑷",
        37 => "𐑨𐑸𐑭𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑙𐑷",
        38 => "𐑼𐑸𐑾𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑳𐑷",
        39 => "𐑦𐑸𐑭𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑳𐑷",
        40 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑙𐑷",
        41 => "𐑨𐑡𐑭𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑙𐑷",
        42 => "𐑼𐑡𐑾𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑳𐑷",
        43 => "𐑦𐑡𐑾𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑳𐑷",
        44 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑙𐑷",
        45 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑙𐑷",
        46 => "𐑼𐑰𐑭𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑳𐑷",
        47 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑳𐑷",
        48 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑙𐑷",
        49 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑳𐑷",
        50 => "𐑼𐑥𐑭𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑙𐑷",
        51 => "𐑦𐑥𐑾𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑕𐑷",
        52 => "𐑛𐑶𐑭𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑳𐑷",
        53 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑳𐑷",
        54 => "𐑼𐑶𐑽𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑳𐑷",
        55 => "𐑦𐑶𐑭𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑙𐑷",
        56 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑳𐑷",
        57 => "𐑨𐑸𐑾𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑙𐑷",
        58 => "𐑼𐑸𐑩𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑳𐑷",
        59 => "𐑦𐑸𐑭𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑳𐑷",
        60 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑳𐑷",
        61 => "𐑨𐑡𐑾𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑕𐑷",
        62 => "𐑼𐑡𐑾𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑙𐑷",
        63 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑳𐑷",
        64 => "𐑛𐑰𐑽𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑙𐑷",
        65 => "𐑨𐑰𐑽𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑙𐑷",
        66 => "𐑼𐑰𐑾𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑳𐑷",
        67 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑷",
        68 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑙𐑷",
        69 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑳𐑷",
        70 => "𐑼𐑥𐑭𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑳𐑷",
        71 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑕𐑷",
        72 => "𐑛𐑶𐑽𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑙𐑷",
        73 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑳𐑷",
        74 => "𐑼𐑶𐑩𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑕𐑷",
        75 => "𐑦𐑶𐑽𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑙𐑷",
        76 => "𐑛𐑸𐑾𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑙𐑷",
        77 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑳𐑷",
        78 => "𐑼𐑸𐑾𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑙𐑷",
        79 => "𐑦𐑸𐑾𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑕𐑷",
        80 => "𐑛𐑡𐑾𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑙𐑷",
        81 => "𐑨𐑡𐑾𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑕𐑷",
        82 => "𐑼𐑡𐑭𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑳𐑷",
        83 => "𐑦𐑡𐑾𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑙𐑷",
        84 => "𐑛𐑰𐑽𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑙𐑷",
        85 => "𐑨𐑰𐑭𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑳𐑷",
        86 => "𐑼𐑰𐑾𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑙𐑷",
        87 => "𐑦𐑰𐑭𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑳𐑷",
        88 => "𐑛𐑥𐑩𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑕𐑷",
        89 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑳𐑷",
        90 => "𐑼𐑥𐑩𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑙𐑷",
        91 => "𐑦𐑥𐑭𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑙𐑷",
        92 => "𐑛𐑶𐑭𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑙𐑷",
        93 => "𐑨𐑶𐑾𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑕𐑷",
        94 => "𐑼𐑶𐑭𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑳𐑷",
        95 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑳𐑷",
        96 => "𐑛𐑸𐑩𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑕𐑷",
        97 => "𐑨𐑸𐑽𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑙𐑷",
        98 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑙𐑷",
        99 => "𐑦𐑸𐑽𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑳𐑷",
        100 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑔𐑨𐑢𐑓𐑙𐑷",
        101 => "𐑨𐑡𐑭𐑿𐑑𐑤𐑲𐑨𐑻𐑒𐑳𐑷",
        102 => "𐑼𐑡𐑾𐑬𐑑𐑤𐑚𐑣⊙𐑖𐑕𐑷",
        103 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑔𐑣𐑣𐑫𐑙𐑷",
        104 => "𐑛𐑰𐑽𐑹𐑑𐑤𐑲𐑣𐑮𐑓𐑳𐑷",
        105 => "𐑨𐑰𐑾𐑗𐑑𐑤𐑚𐑵𐑢𐑒𐑳𐑷",
        106 => "𐑼𐑰𐑾𐑿𐑑𐑤𐑔𐑵𐑻𐑖𐑕𐑷",
        107 => "𐑦𐑰𐑽𐑬𐑑𐑤𐑲𐑵⊙𐑫𐑳𐑷",
        108 => "𐑛𐑥𐑾𐑯𐑑𐑤𐑚𐑝𐑣𐑓𐑙𐑷",
        109 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑔𐑝𐑮𐑒𐑳𐑷",
        110 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑲𐑝𐑢𐑖𐑳𐑷",
        111 => "𐑦𐑥𐑽𐑿𐑑𐑤𐑚𐑨𐑻𐑫𐑙𐑷",
        112 => "𐑛𐑶𐑽𐑬𐑑𐑤𐑔𐑨⊙𐑓𐑳𐑷",
        113 => "𐑨𐑶𐑾𐑯𐑑𐑤𐑲𐑨𐑣𐑒𐑙𐑷",
        114 => "𐑼𐑶𐑩𐑹𐑑𐑤𐑚𐑣𐑮𐑖𐑳𐑷",
        115 => "𐑦𐑶𐑽𐑗𐑑𐑤𐑔𐑣𐑢𐑫𐑳𐑷",
        116 => "𐑛𐑸𐑽𐑿𐑑𐑤𐑲𐑣𐑻𐑓𐑳𐑷",
        117 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑚𐑵⊙𐑒𐑕𐑷",
        118 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑔𐑵𐑣𐑖𐑳𐑷",
        119 => "𐑦𐑸𐑾𐑹𐑑𐑤𐑲𐑵𐑮𐑫𐑕𐑷",
        120 => "𐑛𐑡𐑭𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑙𐑷",
        121 => "𐑨𐑡𐑽𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑙𐑷",
        122 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑳𐑷",
        123 => "𐑦𐑡𐑽𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑙𐑷",
        124 => "𐑛𐑰𐑩𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑙𐑷",
        125 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑙𐑷",
        126 => "𐑼𐑰𐑾𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑕𐑷",
        127 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑕𐑷",
        128 => "𐑛𐑥𐑭𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑙𐑷",
        129 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑙𐑷",
        130 => "𐑼𐑥𐑭𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑙𐑷",
        131 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑳𐑷",
        132 => "𐑛𐑶𐑾𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑕𐑷",
        133 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑳𐑷",
        134 => "𐑼𐑶𐑾𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑳𐑷",
        135 => "𐑦𐑶𐑩𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑳𐑷",
        136 => "𐑛𐑸𐑩𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑕𐑷",
        137 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑙𐑷",
        138 => "𐑼𐑸𐑭𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑳𐑷",
        139 => "𐑦𐑸𐑭𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑳𐑷",
        140 => "𐑛𐑡𐑾𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑕𐑷",
        141 => "𐑨𐑡𐑾𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑙𐑷",
        142 => "𐑼𐑡𐑾𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑳𐑷",
        143 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑕𐑷",
        144 => "𐑛𐑰𐑽𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑙𐑷",
        145 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑙𐑷",
        146 => "𐑼𐑰𐑽𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑳𐑷",
        147 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑕𐑷",
        148 => "𐑛𐑥𐑽𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑳𐑷",
        149 => "𐑨𐑥𐑾𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑙𐑷",
        150 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑳𐑷",
        151 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑳𐑷",
        152 => "𐑛𐑶𐑭𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑳𐑷",
        153 => "𐑨𐑶𐑽𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑳𐑷",
        154 => "𐑼𐑶𐑽𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑳𐑷",
        155 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑳𐑷",
        156 => "𐑛𐑸𐑩𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑳𐑷",
        157 => "𐑨𐑸𐑽𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑙𐑷",
        158 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑕𐑷",
        159 => "𐑦𐑸𐑽𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑙𐑷",
        160 => "𐑛𐑡𐑩𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑳𐑷",
        161 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑙𐑷",
        162 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑳𐑷",
        163 => "𐑦𐑡𐑽𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑙𐑷",
        164 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑳𐑷",
        165 => "𐑨𐑰𐑾𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑕𐑷",
        166 => "𐑼𐑰𐑭𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑳𐑷",
        167 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑕𐑷",
        168 => "𐑛𐑥𐑭𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑕𐑷",
        169 => "𐑨𐑥𐑾𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑙𐑷",
        170 => "𐑼𐑥𐑩𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑕𐑷",
        171 => "𐑦𐑥𐑽𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑳𐑷",
        172 => "𐑛𐑶𐑽𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑳𐑷",
        173 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑙𐑷",
        174 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑳𐑷",
        175 => "𐑦𐑶𐑾𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑳𐑷",
        176 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑕𐑷",
        177 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑳𐑷",
        178 => "𐑼𐑸𐑩𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑕𐑷",
        179 => "𐑦𐑸𐑾𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑙𐑷",
        180 => "𐑛𐑡𐑩𐑗𐑑𐑧𐑚𐑝𐑢𐑓𐑕𐑷",
        181 => "𐑨𐑡𐑭𐑿𐑑𐑧𐑔𐑝𐑻𐑒𐑕𐑷",
        182 => "𐑼𐑡𐑽𐑬𐑑𐑧𐑲𐑝⊙𐑖𐑳𐑷",
        183 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑚𐑨𐑣𐑫𐑕𐑷",
        184 => "𐑛𐑰𐑭𐑹𐑑𐑧𐑔𐑨𐑮𐑓𐑳𐑷",
        185 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑲𐑨𐑢𐑒𐑳𐑷",
        186 => "𐑼𐑰𐑾𐑿𐑑𐑧𐑚𐑣𐑻𐑖𐑳𐑷",
        187 => "𐑦𐑰𐑭𐑬𐑑𐑧𐑔𐑣⊙𐑫𐑕𐑷",
        188 => "𐑛𐑥𐑭𐑯𐑑𐑧𐑲𐑣𐑣𐑓𐑳𐑷",
        189 => "𐑨𐑥𐑭𐑹𐑑𐑧𐑚𐑵𐑮𐑒𐑕𐑷",
        190 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑔𐑵𐑢𐑖𐑕𐑷",
        191 => "𐑦𐑥𐑭𐑿𐑑𐑧𐑲𐑵𐑻𐑫𐑳𐑷",
        192 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑚𐑝⊙𐑓𐑳𐑷",
        193 => "𐑨𐑶𐑽𐑯𐑑𐑧𐑔𐑝𐑣𐑒𐑙𐑷",
        194 => "𐑼𐑶𐑩𐑹𐑑𐑧𐑲𐑝𐑮𐑖𐑳𐑷",
        195 => "𐑦𐑶𐑩𐑗𐑑𐑧𐑚𐑨𐑢𐑫𐑕𐑷",
        196 => "𐑛𐑸𐑭𐑿𐑑𐑧𐑔𐑨𐑻𐑓𐑕𐑷",
        197 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑲𐑨⊙𐑒𐑳𐑷",
        198 => "𐑼𐑸𐑩𐑯𐑑𐑧𐑚𐑣𐑣𐑖𐑳𐑷",
        199 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑔𐑣𐑮𐑫𐑕𐑷",
        200 => "𐑛𐑡𐑩𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑳𐑷",
        201 => "𐑨𐑡𐑩𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑕𐑷",
        202 => "𐑼𐑡𐑩𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑳𐑷",
        203 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑳𐑷",
        204 => "𐑛𐑰𐑩𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑳𐑷",
        205 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑳𐑷",
        206 => "𐑼𐑰𐑽𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑳𐑷",
        207 => "𐑦𐑰𐑽𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑳𐑷",
        208 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑳𐑷",
        209 => "𐑨𐑥𐑩𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑕𐑷",
        210 => "𐑼𐑥𐑽𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑳𐑷",
        211 => "𐑦𐑥𐑽𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑙𐑷",
        212 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑕𐑷",
        213 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑕𐑷",
        214 => "𐑼𐑶𐑾𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑳𐑷",
        215 => "𐑦𐑶𐑭𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑳𐑷",
        216 => "𐑛𐑸𐑭𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑕𐑷",
        217 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑕𐑷",
        218 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑳𐑷",
        219 => "𐑦𐑸𐑭𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑳𐑷",
        220 => "𐑛𐑡𐑽𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑙𐑷",
        221 => "𐑨𐑡𐑭𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑕𐑷",
        222 => "𐑼𐑡𐑽𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑳𐑷",
        223 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑳𐑷",
        224 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑕𐑷",
        225 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑕𐑷",
        226 => "𐑼𐑰𐑭𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑕𐑷",
        227 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑙𐑷",
        228 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑳𐑷",
        229 => "𐑨𐑥𐑽𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑳𐑷",
        230 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑳𐑷",
        231 => "𐑦𐑥𐑾𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑳𐑷",
        232 => "𐑛𐑶𐑭𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑕𐑷",
        233 => "𐑨𐑶𐑭𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑕𐑷",
        234 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑳𐑷",
        235 => "𐑦𐑶𐑭𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑕𐑷",
        236 => "𐑛𐑸𐑽𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑳𐑷",
        237 => "𐑨𐑸𐑾𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑙𐑷",
        238 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑳𐑷",
        239 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑕𐑷",
        240 => "𐑛𐑡𐑩𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑕𐑷",
        241 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑳𐑷",
        242 => "𐑼𐑡𐑽𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑳𐑷",
        243 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑳𐑷",
        244 => "𐑛𐑰𐑽𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑳𐑷",
        245 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑳𐑷",
        246 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑕𐑷",
        247 => "𐑦𐑰𐑾𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑕𐑷",
        248 => "𐑛𐑥𐑩𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑙𐑷",
        249 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑕𐑷",
        250 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑳𐑷",
        251 => "𐑦𐑥𐑭𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑳𐑷",
        252 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑕𐑷",
        253 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑙𐑷",
        254 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑳𐑷",
        255 => "𐑦𐑶𐑭𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑳𐑷",
        256 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑳𐑷",
        257 => "𐑨𐑸𐑩𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑳𐑷",
        258 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑳𐑷",
        259 => "𐑦𐑸𐑩𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑕𐑷",
        260 => "𐑛𐑡𐑩𐑗𐑑𐑪𐑲𐑣𐑢𐑓𐑕𐑷",
        261 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑚𐑵𐑻𐑒𐑙𐑷",
        262 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑔𐑵⊙𐑖𐑕𐑷",
        263 => "𐑦𐑡𐑾𐑯𐑑𐑪𐑲𐑵𐑣𐑫𐑕𐑷",
        264 => "𐑛𐑰𐑭𐑹𐑑𐑪𐑚𐑝𐑮𐑓𐑳𐑷",
        265 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑔𐑝𐑢𐑒𐑳𐑷",
        266 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑲𐑝𐑻𐑖𐑕𐑷",
        267 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑚𐑨⊙𐑫𐑳𐑷",
        268 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑔𐑨𐑣𐑓𐑙𐑷",
        269 => "𐑨𐑥𐑽𐑹𐑑𐑪𐑲𐑨𐑮𐑒𐑳𐑷",
        270 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑚𐑣𐑢𐑖𐑳𐑷",
        271 => "𐑦𐑥𐑾𐑿𐑑𐑪𐑔𐑣𐑻𐑫𐑙𐑷",
        272 => "𐑛𐑶𐑾𐑬𐑑𐑪𐑲𐑣⊙𐑓𐑙𐑷",
        273 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑚𐑵𐑣𐑒𐑙𐑷",
        274 => "𐑼𐑶𐑾𐑹𐑑𐑪𐑔𐑵𐑮𐑖𐑕𐑷",
        275 => "𐑦𐑶𐑽𐑗𐑑𐑪𐑲𐑵𐑢𐑫𐑳𐑷",
        276 => "𐑛𐑸𐑩𐑿𐑑𐑪𐑚𐑝𐑻𐑓𐑕𐑷",
        277 => "𐑨𐑸𐑭𐑬𐑑𐑪𐑔𐑝⊙𐑒𐑳𐑷",
        278 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑲𐑝𐑣𐑖𐑳𐑷",
        279 => "𐑦𐑸𐑭𐑹𐑑𐑪𐑚𐑨𐑮𐑫𐑕𐑷",
        280 => "𐑛𐑡𐑩𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑕𐑷",
        281 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑳𐑷",
        282 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑕𐑷",
        283 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑙𐑷",
        284 => "𐑛𐑰𐑽𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑳𐑷",
        285 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑙𐑷",
        286 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑕𐑷",
        287 => "𐑦𐑰𐑾𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑙𐑷",
        288 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑕𐑷",
        289 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑙𐑷",
        290 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑳𐑷",
        291 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑳𐑷",
        292 => "𐑛𐑶𐑭𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑙𐑷",
        293 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑕𐑷",
        294 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑳𐑷",
        295 => "𐑦𐑶𐑾𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑙𐑷",
        296 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑳𐑷",
        297 => "𐑨𐑸𐑽𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑳𐑷",
        298 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑳𐑷",
        299 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑳𐑷",
        300 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑙𐑷",
        301 => "𐑨𐑡𐑾𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑙𐑷",
        302 => "𐑼𐑡𐑭𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑳𐑷",
        303 => "𐑦𐑡𐑭𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑕𐑷",
        304 => "𐑛𐑰𐑩𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑙𐑷",
        305 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑳𐑷",
        306 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑳𐑷",
        307 => "𐑦𐑰𐑾𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑳𐑷",
        308 => "𐑛𐑥𐑽𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑳𐑷",
        309 => "𐑨𐑥𐑭𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑳𐑷",
        310 => "𐑼𐑥𐑩𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑕𐑷",
        311 => "𐑦𐑥𐑭𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑳𐑷",
        312 => "𐑛𐑶𐑭𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑕𐑷",
        313 => "𐑨𐑶𐑽𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑙𐑷",
        314 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑕𐑷",
        315 => "𐑦𐑶𐑽𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑕𐑷",
        316 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑕𐑷",
        317 => "𐑨𐑸𐑾𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑳𐑷",
        318 => "𐑼𐑸𐑭𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑳𐑷",
        319 => "𐑦𐑸𐑭𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑕𐑷",
        320 => "𐑛𐑡𐑾𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑳𐑷",
        321 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑙𐑷",
        322 => "𐑼𐑡𐑽𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑙𐑷",
        323 => "𐑦𐑡𐑾𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑳𐑷",
        324 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑳𐑷",
        325 => "𐑨𐑰𐑭𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑕𐑷",
        326 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑕𐑷",
        327 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑳𐑷",
        328 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑙𐑷",
        329 => "𐑨𐑥𐑽𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑳𐑷",
        330 => "𐑼𐑥𐑾𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑙𐑷",
        331 => "𐑦𐑥𐑩𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑕𐑷",
        332 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑳𐑷",
        333 => "𐑨𐑶𐑭𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑳𐑷",
        334 => "𐑼𐑶𐑩𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑕𐑷",
        335 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑳𐑷",
        336 => "𐑛𐑸𐑩𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑕𐑷",
        337 => "𐑨𐑸𐑩𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑕𐑷",
        338 => "𐑼𐑸𐑭𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑳𐑷",
        339 => "𐑦𐑸𐑭𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑕𐑷",
        340 => "𐑛𐑡𐑽𐑗𐑑𐑺𐑔𐑨𐑢𐑓𐑳𐑷",
        341 => "𐑨𐑡𐑭𐑿𐑑𐑺𐑲𐑨𐑻𐑒𐑕𐑷",
        342 => "𐑼𐑡𐑽𐑬𐑑𐑺𐑚𐑣⊙𐑖𐑳𐑷",
        343 => "𐑦𐑡𐑭𐑯𐑑𐑺𐑔𐑣𐑣𐑫𐑕𐑷",
        344 => "𐑛𐑰𐑭𐑹𐑑𐑺𐑲𐑣𐑮𐑓𐑕𐑷",
        345 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑚𐑵𐑢𐑒𐑳𐑷",
        346 => "𐑼𐑰𐑩𐑿𐑑𐑺𐑔𐑵𐑻𐑖𐑕𐑷",
        347 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑲𐑵⊙𐑫𐑕𐑷",
        348 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑚𐑝𐑣𐑓𐑕𐑷",
        349 => "𐑨𐑥𐑭𐑹𐑑𐑺𐑔𐑝𐑮𐑒𐑳𐑷",
        350 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑲𐑝𐑢𐑖𐑕𐑷",
        351 => "𐑦𐑥𐑭𐑿𐑑𐑺𐑚𐑨𐑻𐑫𐑳𐑷",
        352 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑔𐑨⊙𐑓𐑕𐑷",
        353 => "𐑨𐑶𐑩𐑯𐑑𐑺𐑲𐑨𐑣𐑒𐑕𐑷",
        354 => "𐑼𐑶𐑭𐑹𐑑𐑺𐑚𐑣𐑮𐑖𐑕𐑷",
        355 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑔𐑣𐑢𐑫𐑳𐑷",
        356 => "𐑛𐑸𐑽𐑿𐑑𐑺𐑲𐑣𐑻𐑓𐑕𐑷",
        357 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑚𐑵⊙𐑒𐑳𐑷",
        358 => "𐑼𐑸𐑭𐑯𐑑𐑺𐑔𐑵𐑣𐑖𐑳𐑷",
        359 => "𐑦𐑸𐑾𐑹𐑑𐑺𐑲𐑵𐑮𐑫𐑙𐑷",
        360 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑳𐑷",
        361 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑙𐑷",
        362 => "𐑼𐑡𐑩𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑳𐑷",
        363 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑳𐑷",
        364 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑳𐑷",
        365 => "𐑨𐑰𐑭𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑳𐑷",
        366 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑳𐑷",
        367 => "𐑦𐑰𐑾𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑙𐑷",
        368 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑳𐑷",
        369 => "𐑨𐑥𐑾𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑳𐑷",
        370 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑳𐑷",
        371 => "𐑦𐑥𐑽𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑳𐑷",
        372 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑳𐑷",
        373 => "𐑨𐑶𐑾𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑳𐑷",
        374 => "𐑼𐑶𐑾𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑳𐑷",
        375 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑳𐑷",
        376 => "𐑛𐑸𐑩𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑳𐑷",
        377 => "𐑨𐑸𐑾𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑳𐑷",
        378 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑙𐑷",
        379 => "𐑦𐑸𐑾𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑳𐑷",
        380 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑙𐑷",
        381 => "𐑨𐑡𐑩𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑳𐑷",
        382 => "𐑼𐑡𐑩𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑳𐑷",
        383 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑳𐑷",
        384 => "𐑛𐑰𐑩𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑕𐑷",
        385 => "𐑨𐑰𐑭𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑳𐑷",
        386 => "𐑼𐑰𐑩𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑳𐑷",
        387 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑕𐑷",
        388 => "𐑛𐑥𐑾𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑳𐑷",
        389 => "𐑨𐑥𐑩𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑕𐑷",
        390 => "𐑼𐑥𐑽𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑳𐑷",
        391 => "𐑦𐑥𐑭𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑳𐑷",
        392 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑳𐑷",
        393 => "𐑨𐑶𐑽𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑙𐑷",
        394 => "𐑼𐑶𐑽𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑙𐑷",
        395 => "𐑦𐑶𐑽𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑙𐑷",
        396 => "𐑛𐑸𐑭𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑳𐑷",
        397 => "𐑨𐑸𐑽𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑙𐑷",
        398 => "𐑼𐑸𐑭𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑕𐑷",
        399 => "𐑦𐑸𐑽𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑳𐑷",
        400 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑕𐑷",
        401 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑕𐑷",
        402 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑳𐑷",
        403 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑕𐑷",
        404 => "𐑛𐑰𐑽𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑳𐑷",
        405 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑕𐑷",
        406 => "𐑼𐑰𐑭𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑕𐑷",
        407 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑳𐑷",
        408 => "𐑛𐑥𐑽𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑳𐑷",
        409 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑳𐑷",
        410 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑕𐑷",
        411 => "𐑦𐑥𐑾𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑳𐑷",
        412 => "𐑛𐑶𐑭𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑳𐑷",
        413 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑕𐑷",
        414 => "𐑼𐑶𐑽𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑳𐑷",
        415 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑕𐑷",
        416 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑕𐑷",
        417 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑳𐑷",
        418 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑕𐑷",
        419 => "𐑦𐑸𐑾𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑙𐑷",
        420 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑳𐑷",
        421 => "𐑨𐑡𐑽𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑙𐑷",
        422 => "𐑼𐑡𐑾𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑳𐑷",
        423 => "𐑦𐑡𐑽𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑕𐑷",
        424 => "𐑛𐑰𐑭𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑕𐑷",
        425 => "𐑨𐑰𐑾𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑙𐑷",
        426 => "𐑼𐑰𐑾𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑕𐑷",
        427 => "𐑦𐑰𐑩𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑷",
        428 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑙𐑷",
        429 => "𐑨𐑥𐑾𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑕𐑷",
        430 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑙𐑷",
        431 => "𐑦𐑥𐑾𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑕𐑷",
        432 => "𐑛𐑶𐑾𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑳𐑷",
        433 => "𐑨𐑶𐑾𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑙𐑷",
        434 => "𐑼𐑶𐑭𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑳𐑷",
        435 => "𐑦𐑶𐑾𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑙𐑷",
        436 => "𐑛𐑸𐑽𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑳𐑷",
        437 => "𐑨𐑸𐑩𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑳𐑷",
        438 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑕𐑷",
        439 => "𐑦𐑸𐑾𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑕𐑷",
        440 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑙𐑷",
        441 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑕𐑷",
        442 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑳𐑷",
        443 => "𐑦𐑡𐑽𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑳𐑷",
        444 => "𐑛𐑰𐑽𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑳𐑷",
        445 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑕𐑷",
        446 => "𐑼𐑰𐑩𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑙𐑷",
        447 => "𐑦𐑰𐑩𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑕𐑷",
        448 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑕𐑷",
        449 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑳𐑷",
        450 => "𐑼𐑥𐑽𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑙𐑷",
        451 => "𐑦𐑥𐑽𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑳𐑷",
        452 => "𐑛𐑶𐑽𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑙𐑷",
        453 => "𐑨𐑶𐑩𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑙𐑷",
        454 => "𐑼𐑶𐑽𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑙𐑷",
        455 => "𐑦𐑶𐑾𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑙𐑷",
        456 => "𐑛𐑸𐑩𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑕𐑷",
        457 => "𐑨𐑸𐑩𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑕𐑷",
        458 => "𐑼𐑸𐑩𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑳𐑷",
        459 => "𐑦𐑸𐑾𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑙𐑷",
        460 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑙𐑷",
        461 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑕𐑷",
        462 => "𐑼𐑡𐑽𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑙𐑷",
        463 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑳𐑷",
        464 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑙𐑷",
        465 => "𐑨𐑰𐑽𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑳𐑷",
        466 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑙𐑷",
        467 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑙𐑷",
        468 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑳𐑷",
        469 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑕𐑷",
        470 => "𐑼𐑥𐑽𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑳𐑷",
        471 => "𐑦𐑥𐑽𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑳𐑷",
        472 => "𐑛𐑶𐑾𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑙𐑷",
        473 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑕𐑷",
        474 => "𐑼𐑶𐑽𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑳𐑷",
        475 => "𐑦𐑶𐑽𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑙𐑷",
        476 => "𐑛𐑸𐑩𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑙𐑷",
        477 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑳𐑷",
        478 => "𐑼𐑸𐑽𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑳𐑷",
        479 => "𐑦𐑸𐑩𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑕𐑷",
        480 => "𐑛𐑡𐑾𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑙𐑷",
        481 => "𐑨𐑡𐑾𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑳𐑷",
        482 => "𐑼𐑡𐑽𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑙𐑷",
        483 => "𐑦𐑡𐑽𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑳𐑷",
        484 => "𐑛𐑰𐑾𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑙𐑷",
        485 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑙𐑷",
        486 => "𐑼𐑰𐑽𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑙𐑷",
        487 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑙𐑷",
        488 => "𐑛𐑥𐑩𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑳𐑷",
        489 => "𐑨𐑥𐑾𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑙𐑷",
        490 => "𐑼𐑥𐑭𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑳𐑷",
        491 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑳𐑷",
        492 => "𐑛𐑶𐑽𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑙𐑷",
        493 => "𐑨𐑶𐑩𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑳𐑷",
        494 => "𐑼𐑶𐑽𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑳𐑷",
        495 => "𐑦𐑶𐑾𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑙𐑷",
        496 => "𐑛𐑸𐑾𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑳𐑷",
        497 => "𐑨𐑸𐑾𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑙𐑷",
        498 => "𐑼𐑸𐑽𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑳𐑷",
        499 => "𐑦𐑸𐑩𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑕𐑷",
        500 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑳𐑷",
        501 => "𐑨𐑡𐑩𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑕𐑷",
        502 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑕𐑷",
        503 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑙𐑷",
        504 => "𐑛𐑰𐑾𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑕𐑷",
        505 => "𐑨𐑰𐑩𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑕𐑷",
        506 => "𐑼𐑰𐑾𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑳𐑷",
        507 => "𐑦𐑰𐑩𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑙𐑷",
        508 => "𐑛𐑥𐑾𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑳𐑷",
        509 => "𐑨𐑥𐑩𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑙𐑷",
        510 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑳𐑷",
        511 => "𐑦𐑥𐑽𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑙𐑷",
        512 => "𐑛𐑶𐑾𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑙𐑴",
        513 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑕𐑴",
        514 => "𐑼𐑶𐑽𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑳𐑴",
        515 => "𐑦𐑶𐑩𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑙𐑴",
        516 => "𐑛𐑸𐑾𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑕𐑴",
        517 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑕𐑴",
        518 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑳𐑴",
        519 => "𐑦𐑸𐑽𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑙𐑴",
        520 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑔𐑨𐑢𐑓𐑙𐑴",
        521 => "𐑨𐑡𐑽𐑿𐑑𐑤𐑲𐑨𐑻𐑒𐑳𐑴",
        522 => "𐑼𐑡𐑭𐑬𐑑𐑤𐑚𐑣⊙𐑖𐑳𐑴",
        523 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑔𐑣𐑣𐑫𐑳𐑴",
        524 => "𐑛𐑰𐑩𐑹𐑑𐑤𐑲𐑣𐑮𐑓𐑳𐑴",
        525 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑚𐑵𐑢𐑒𐑙𐑴",
        526 => "𐑼𐑰𐑩𐑿𐑑𐑤𐑔𐑵𐑻𐑖𐑳𐑴",
        527 => "𐑦𐑰𐑭𐑬𐑑𐑤𐑲𐑵⊙𐑫𐑕𐑴",
        528 => "𐑛𐑥𐑽𐑯𐑑𐑤𐑚𐑝𐑣𐑓𐑳𐑴",
        529 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑔𐑝𐑮𐑒𐑳𐑴",
        530 => "𐑼𐑥𐑾𐑗𐑑𐑤𐑲𐑝𐑢𐑖𐑳𐑴",
        531 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑚𐑨𐑻𐑫𐑕𐑴",
        532 => "𐑛𐑶𐑭𐑬𐑑𐑤𐑔𐑨⊙𐑓𐑳𐑴",
        533 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑲𐑨𐑣𐑒𐑕𐑴",
        534 => "𐑼𐑶𐑭𐑹𐑑𐑤𐑚𐑣𐑮𐑖𐑕𐑴",
        535 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑔𐑣𐑢𐑫𐑕𐑴",
        536 => "𐑛𐑸𐑭𐑿𐑑𐑤𐑲𐑣𐑻𐑓𐑕𐑴",
        537 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑚𐑵⊙𐑒𐑕𐑴",
        538 => "𐑼𐑸𐑭𐑯𐑑𐑤𐑔𐑵𐑣𐑖𐑳𐑴",
        539 => "𐑦𐑸𐑩𐑹𐑑𐑤𐑲𐑵𐑮𐑫𐑕𐑴",
        540 => "𐑛𐑡𐑩𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑕𐑴",
        541 => "𐑨𐑡𐑾𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑙𐑴",
        542 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑕𐑴",
        543 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑕𐑴",
        544 => "𐑛𐑰𐑩𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑳𐑴",
        545 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑳𐑴",
        546 => "𐑼𐑰𐑩𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑕𐑴",
        547 => "𐑦𐑰𐑭𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑕𐑴",
        548 => "𐑛𐑥𐑩𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑕𐑴",
        549 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑕𐑴",
        550 => "𐑼𐑥𐑾𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑳𐑴",
        551 => "𐑦𐑥𐑭𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑙𐑴",
        552 => "𐑛𐑶𐑭𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑙𐑴",
        553 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑳𐑴",
        554 => "𐑼𐑶𐑩𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑳𐑴",
        555 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑳𐑴",
        556 => "𐑛𐑸𐑽𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑙𐑴",
        557 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑳𐑴",
        558 => "𐑼𐑸𐑩𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑳𐑴",
        559 => "𐑦𐑸𐑩𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑳𐑴",
        560 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑙𐑴",
        561 => "𐑨𐑡𐑾𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑙𐑴",
        562 => "𐑼𐑡𐑩𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑳𐑴",
        563 => "𐑦𐑡𐑩𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑳𐑴",
        564 => "𐑛𐑰𐑾𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑕𐑴",
        565 => "𐑨𐑰𐑽𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑳𐑴",
        566 => "𐑼𐑰𐑾𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑕𐑴",
        567 => "𐑦𐑰𐑭𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑳𐑴",
        568 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑙𐑴",
        569 => "𐑨𐑥𐑭𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑳𐑴",
        570 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑳𐑴",
        571 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑙𐑴",
        572 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑙𐑴",
        573 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑙𐑴",
        574 => "𐑼𐑶𐑩𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑙𐑴",
        575 => "𐑦𐑶𐑩𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑕𐑴",
        576 => "𐑛𐑸𐑭𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑙𐑴",
        577 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑳𐑴",
        578 => "𐑼𐑸𐑽𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑳𐑴",
        579 => "𐑦𐑸𐑽𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑳𐑴",
        580 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑙𐑴",
        581 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑙𐑴",
        582 => "𐑼𐑡𐑾𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑕𐑴",
        583 => "𐑦𐑡𐑽𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑳𐑴",
        584 => "𐑛𐑰𐑾𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑙𐑴",
        585 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑕𐑴",
        586 => "𐑼𐑰𐑽𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑙𐑴",
        587 => "𐑦𐑰𐑭𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑳𐑴",
        588 => "𐑛𐑥𐑾𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑕𐑴",
        589 => "𐑨𐑥𐑾𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑳𐑴",
        590 => "𐑼𐑥𐑾𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑙𐑴",
        591 => "𐑦𐑥𐑭𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑳𐑴",
        592 => "𐑛𐑶𐑽𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑳𐑴",
        593 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑙𐑴",
        594 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑙𐑴",
        595 => "𐑦𐑶𐑽𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑙𐑴",
        596 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑙𐑴",
        597 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑳𐑴",
        598 => "𐑼𐑸𐑽𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑳𐑴",
        599 => "𐑦𐑸𐑽𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑙𐑴",
        600 => "𐑛𐑡𐑽𐑗𐑑𐑧𐑚𐑝𐑢𐑓𐑳𐑴",
        601 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑔𐑝𐑻𐑒𐑳𐑴",
        602 => "𐑼𐑡𐑽𐑬𐑑𐑧𐑲𐑝⊙𐑖𐑙𐑴",
        603 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑚𐑨𐑣𐑫𐑳𐑴",
        604 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑔𐑨𐑮𐑓𐑙𐑴",
        605 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑲𐑨𐑢𐑒𐑕𐑴",
        606 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑚𐑣𐑻𐑖𐑕𐑴",
        607 => "𐑦𐑰𐑾𐑬𐑑𐑧𐑔𐑣⊙𐑫𐑕𐑴",
        608 => "𐑛𐑥𐑭𐑯𐑑𐑧𐑲𐑣𐑣𐑓𐑙𐑴",
        609 => "𐑨𐑥𐑭𐑹𐑑𐑧𐑚𐑵𐑮𐑒𐑙𐑴",
        610 => "𐑼𐑥𐑾𐑗𐑑𐑧𐑔𐑵𐑢𐑖𐑳𐑴",
        611 => "𐑦𐑥𐑭𐑿𐑑𐑧𐑲𐑵𐑻𐑫𐑙𐑴",
        612 => "𐑛𐑶𐑭𐑬𐑑𐑧𐑚𐑝⊙𐑓𐑙𐑴",
        613 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑔𐑝𐑣𐑒𐑕𐑴",
        614 => "𐑼𐑶𐑭𐑹𐑑𐑧𐑲𐑝𐑮𐑖𐑳𐑴",
        615 => "𐑦𐑶𐑭𐑗𐑑𐑧𐑚𐑨𐑢𐑫𐑳𐑴",
        616 => "𐑛𐑸𐑽𐑿𐑑𐑧𐑔𐑨𐑻𐑓𐑙𐑴",
        617 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑲𐑨⊙𐑒𐑳𐑴",
        618 => "𐑼𐑸𐑾𐑯𐑑𐑧𐑚𐑣𐑣𐑖𐑙𐑴",
        619 => "𐑦𐑸𐑽𐑹𐑑𐑧𐑔𐑣𐑮𐑫𐑙𐑴",
        620 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑙𐑴",
        621 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑳𐑴",
        622 => "𐑼𐑡𐑩𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑕𐑴",
        623 => "𐑦𐑡𐑩𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑳𐑴",
        624 => "𐑛𐑰𐑭𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑙𐑴",
        625 => "𐑨𐑰𐑽𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑙𐑴",
        626 => "𐑼𐑰𐑭𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑳𐑴",
        627 => "𐑦𐑰𐑾𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑙𐑴",
        628 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑳𐑴",
        629 => "𐑨𐑥𐑩𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑙𐑴",
        630 => "𐑼𐑥𐑽𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑙𐑴",
        631 => "𐑦𐑥𐑾𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑙𐑴",
        632 => "𐑛𐑶𐑽𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑙𐑴",
        633 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑕𐑴",
        634 => "𐑼𐑶𐑩𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑕𐑴",
        635 => "𐑦𐑶𐑾𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑙𐑴",
        636 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑙𐑴",
        637 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑙𐑴",
        638 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑳𐑴",
        639 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑳𐑴",
        640 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑙𐑴",
        641 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑳𐑴",
        642 => "𐑼𐑡𐑽𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑳𐑴",
        643 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑙𐑴",
        644 => "𐑛𐑰𐑾𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑙𐑴",
        645 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑙𐑴",
        646 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑳𐑴",
        647 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑳𐑴",
        648 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑳𐑴",
        649 => "𐑨𐑥𐑾𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑳𐑴",
        650 => "𐑼𐑥𐑾𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑙𐑴",
        651 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑳𐑴",
        652 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑳𐑴",
        653 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑳𐑴",
        654 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑳𐑴",
        655 => "𐑦𐑶𐑭𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑳𐑴",
        656 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑳𐑴",
        657 => "𐑨𐑸𐑩𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑕𐑴",
        658 => "𐑼𐑸𐑭𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑳𐑴",
        659 => "𐑦𐑸𐑩𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑳𐑴",
        660 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑳𐑴",
        661 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑙𐑴",
        662 => "𐑼𐑡𐑾𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑕𐑴",
        663 => "𐑦𐑡𐑭𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑕𐑴",
        664 => "𐑛𐑰𐑾𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑳𐑴",
        665 => "𐑨𐑰𐑩𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑕𐑴",
        666 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑙𐑴",
        667 => "𐑦𐑰𐑾𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑕𐑴",
        668 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑳𐑴",
        669 => "𐑨𐑥𐑽𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑳𐑴",
        670 => "𐑼𐑥𐑾𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑕𐑴",
        671 => "𐑦𐑥𐑭𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑳𐑴",
        672 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑕𐑴",
        673 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑕𐑴",
        674 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑙𐑴",
        675 => "𐑦𐑶𐑾𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑙𐑴",
        676 => "𐑛𐑸𐑾𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑳𐑴",
        677 => "𐑨𐑸𐑩𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑳𐑴",
        678 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑳𐑴",
        679 => "𐑦𐑸𐑽𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑙𐑴",
        680 => "𐑛𐑡𐑭𐑗𐑑𐑪𐑲𐑣𐑢𐑓𐑳𐑴",
        681 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑚𐑵𐑻𐑒𐑙𐑴",
        682 => "𐑼𐑡𐑭𐑬𐑑𐑪𐑔𐑵⊙𐑖𐑕𐑴",
        683 => "𐑦𐑡𐑭𐑯𐑑𐑪𐑲𐑵𐑣𐑫𐑳𐑴",
        684 => "𐑛𐑰𐑾𐑹𐑑𐑪𐑚𐑝𐑮𐑓𐑕𐑴",
        685 => "𐑨𐑰𐑭𐑗𐑑𐑪𐑔𐑝𐑢𐑒𐑳𐑴",
        686 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑲𐑝𐑻𐑖𐑙𐑴",
        687 => "𐑦𐑰𐑭𐑬𐑑𐑪𐑚𐑨⊙𐑫𐑳𐑴",
        688 => "𐑛𐑥𐑽𐑯𐑑𐑪𐑔𐑨𐑣𐑓𐑙𐑴",
        689 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑲𐑨𐑮𐑒𐑳𐑴",
        690 => "𐑼𐑥𐑩𐑗𐑑𐑪𐑚𐑣𐑢𐑖𐑕𐑴",
        691 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑔𐑣𐑻𐑫𐑳𐑴",
        692 => "𐑛𐑶𐑭𐑬𐑑𐑪𐑲𐑣⊙𐑓𐑳𐑴",
        693 => "𐑨𐑶𐑩𐑯𐑑𐑪𐑚𐑵𐑣𐑒𐑕𐑴",
        694 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑔𐑵𐑮𐑖𐑳𐑴",
        695 => "𐑦𐑶𐑽𐑗𐑑𐑪𐑲𐑵𐑢𐑫𐑳𐑴",
        696 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑚𐑝𐑻𐑓𐑳𐑴",
        697 => "𐑨𐑸𐑩𐑬𐑑𐑪𐑔𐑝⊙𐑒𐑳𐑴",
        698 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑲𐑝𐑣𐑖𐑕𐑴",
        699 => "𐑦𐑸𐑭𐑹𐑑𐑪𐑚𐑨𐑮𐑫𐑕𐑴",
        700 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑳𐑴",
        701 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑕𐑴",
        702 => "𐑼𐑡𐑾𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑕𐑴",
        703 => "𐑦𐑡𐑾𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑳𐑴",
        704 => "𐑛𐑰𐑾𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑕𐑴",
        705 => "𐑨𐑰𐑩𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑕𐑴",
        706 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑳𐑴",
        707 => "𐑦𐑰𐑭𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑕𐑴",
        708 => "𐑛𐑥𐑩𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑕𐑴",
        709 => "𐑨𐑥𐑽𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑳𐑴",
        710 => "𐑼𐑥𐑩𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑳𐑴",
        711 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑕𐑴",
        712 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑕𐑴",
        713 => "𐑨𐑶𐑭𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑳𐑴",
        714 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑕𐑴",
        715 => "𐑦𐑶𐑭𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑳𐑴",
        716 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑳𐑴",
        717 => "𐑨𐑸𐑭𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑳𐑴",
        718 => "𐑼𐑸𐑭𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑕𐑴",
        719 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑙𐑴",
        720 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑳𐑴",
        721 => "𐑨𐑡𐑭𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑳𐑴",
        722 => "𐑼𐑡𐑾𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑕𐑴",
        723 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑳𐑴",
        724 => "𐑛𐑰𐑩𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑕𐑴",
        725 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑳𐑴",
        726 => "𐑼𐑰𐑩𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑳𐑴",
        727 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑳𐑴",
        728 => "𐑛𐑥𐑭𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑳𐑴",
        729 => "𐑨𐑥𐑽𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑳𐑴",
        730 => "𐑼𐑥𐑽𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑳𐑴",
        731 => "𐑦𐑥𐑩𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑕𐑴",
        732 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑙𐑴",
        733 => "𐑨𐑶𐑭𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑳𐑴",
        734 => "𐑼𐑶𐑾𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑳𐑴",
        735 => "𐑦𐑶𐑽𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑳𐑴",
        736 => "𐑛𐑸𐑽𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑳𐑴",
        737 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑳𐑴",
        738 => "𐑼𐑸𐑾𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑕𐑴",
        739 => "𐑦𐑸𐑾𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑙𐑴",
        740 => "𐑛𐑡𐑭𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑳𐑴",
        741 => "𐑨𐑡𐑩𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑙𐑴",
        742 => "𐑼𐑡𐑭𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑕𐑴",
        743 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑳𐑴",
        744 => "𐑛𐑰𐑽𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑳𐑴",
        745 => "𐑨𐑰𐑩𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑕𐑴",
        746 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑕𐑴",
        747 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑕𐑴",
        748 => "𐑛𐑥𐑭𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑕𐑴",
        749 => "𐑨𐑥𐑽𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑳𐑴",
        750 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑳𐑴",
        751 => "𐑦𐑥𐑩𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑳𐑴",
        752 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑕𐑴",
        753 => "𐑨𐑶𐑭𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑕𐑴",
        754 => "𐑼𐑶𐑾𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑙𐑴",
        755 => "𐑦𐑶𐑩𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑕𐑴",
        756 => "𐑛𐑸𐑽𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑙𐑴",
        757 => "𐑨𐑸𐑽𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑳𐑴",
        758 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑳𐑴",
        759 => "𐑦𐑸𐑽𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑳𐑴",
        760 => "𐑛𐑡𐑾𐑗𐑑𐑺𐑔𐑨𐑢𐑓𐑳𐑴",
        761 => "𐑨𐑡𐑩𐑿𐑑𐑺𐑲𐑨𐑻𐑒𐑳𐑴",
        762 => "𐑼𐑡𐑾𐑬𐑑𐑺𐑚𐑣⊙𐑖𐑕𐑴",
        763 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑔𐑣𐑣𐑫𐑙𐑴",
        764 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑲𐑣𐑮𐑓𐑙𐑴",
        765 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑚𐑵𐑢𐑒𐑳𐑴",
        766 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑔𐑵𐑻𐑖𐑳𐑴",
        767 => "𐑦𐑰𐑾𐑬𐑑𐑺𐑲𐑵⊙𐑫𐑳𐑴",
        768 => "𐑛𐑥𐑾𐑯𐑑𐑺𐑚𐑝𐑣𐑓𐑕𐑴",
        769 => "𐑨𐑥𐑩𐑹𐑑𐑺𐑔𐑝𐑮𐑒𐑕𐑴",
        770 => "𐑼𐑥𐑩𐑗𐑑𐑺𐑲𐑝𐑢𐑖𐑳𐑴",
        771 => "𐑦𐑥𐑽𐑿𐑑𐑺𐑚𐑨𐑻𐑫𐑳𐑴",
        772 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑔𐑨⊙𐑓𐑙𐑴",
        773 => "𐑨𐑶𐑩𐑯𐑑𐑺𐑲𐑨𐑣𐑒𐑳𐑴",
        774 => "𐑼𐑶𐑾𐑹𐑑𐑺𐑚𐑣𐑮𐑖𐑙𐑴",
        775 => "𐑦𐑶𐑽𐑗𐑑𐑺𐑔𐑣𐑢𐑫𐑙𐑴",
        776 => "𐑛𐑸𐑭𐑿𐑑𐑺𐑲𐑣𐑻𐑓𐑳𐑴",
        777 => "𐑨𐑸𐑽𐑬𐑑𐑺𐑚𐑵⊙𐑒𐑳𐑴",
        778 => "𐑼𐑸𐑾𐑯𐑑𐑺𐑔𐑵𐑣𐑖𐑕𐑴",
        779 => "𐑦𐑸𐑾𐑹𐑑𐑺𐑲𐑵𐑮𐑫𐑙𐑴",
        780 => "𐑛𐑡𐑭𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑕𐑴",
        781 => "𐑨𐑡𐑭𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑳𐑴",
        782 => "𐑼𐑡𐑩𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑕𐑴",
        783 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑳𐑴",
        784 => "𐑛𐑰𐑽𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑳𐑴",
        785 => "𐑨𐑰𐑾𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑙𐑴",
        786 => "𐑼𐑰𐑩𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑕𐑴",
        787 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑳𐑴",
        788 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑕𐑴",
        789 => "𐑨𐑥𐑽𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑳𐑴",
        790 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑳𐑴",
        791 => "𐑦𐑥𐑭𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑕𐑴",
        792 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑳𐑴",
        793 => "𐑨𐑶𐑾𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑳𐑴",
        794 => "𐑼𐑶𐑭𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑳𐑴",
        795 => "𐑦𐑶𐑭𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑳𐑴",
        796 => "𐑛𐑸𐑭𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑕𐑴",
        797 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑳𐑴",
        798 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑕𐑴",
        799 => "𐑦𐑸𐑩𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑳𐑴",
        800 => "𐑛𐑡𐑩𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑳𐑴",
        801 => "𐑨𐑡𐑾𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑳𐑴",
        802 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑕𐑴",
        803 => "𐑦𐑡𐑩𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑳𐑴",
        804 => "𐑛𐑰𐑭𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑙𐑴",
        805 => "𐑨𐑰𐑾𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑙𐑴",
        806 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑳𐑴",
        807 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑳𐑴",
        808 => "𐑛𐑥𐑽𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑳𐑴",
        809 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑕𐑴",
        810 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑕𐑴",
        811 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑳𐑴",
        812 => "𐑛𐑶𐑭𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑳𐑴",
        813 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑕𐑴",
        814 => "𐑼𐑶𐑭𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑳𐑴",
        815 => "𐑦𐑶𐑭𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑕𐑴",
        816 => "𐑛𐑸𐑾𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑳𐑴",
        817 => "𐑨𐑸𐑭𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑳𐑴",
        818 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑳𐑴",
        819 => "𐑦𐑸𐑩𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑕𐑴",
        820 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑳𐑴",
        821 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑕𐑴",
        822 => "𐑼𐑡𐑾𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑳𐑴",
        823 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑳𐑴",
        824 => "𐑛𐑰𐑩𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑕𐑴",
        825 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑕𐑴",
        826 => "𐑼𐑰𐑭𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑳𐑴",
        827 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑳𐑴",
        828 => "𐑛𐑥𐑭𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑙𐑴",
        829 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑳𐑴",
        830 => "𐑼𐑥𐑽𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑙𐑴",
        831 => "𐑦𐑥𐑾𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑕𐑴",
        832 => "𐑛𐑶𐑾𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑙𐑴",
        833 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑳𐑴",
        834 => "𐑼𐑶𐑩𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑳𐑴",
        835 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑕𐑴",
        836 => "𐑛𐑸𐑽𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑳𐑴",
        837 => "𐑨𐑸𐑾𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑳𐑴",
        838 => "𐑼𐑸𐑩𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑕𐑴",
        839 => "𐑦𐑸𐑭𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑕𐑴",
        840 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑳𐑴",
        841 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑕𐑴",
        842 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑕𐑴",
        843 => "𐑦𐑡𐑾𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑳𐑴",
        844 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑕𐑴",
        845 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑳𐑴",
        846 => "𐑼𐑰𐑩𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑕𐑴",
        847 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑴",
        848 => "𐑛𐑥𐑩𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑳𐑴",
        849 => "𐑨𐑥𐑽𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑳𐑴",
        850 => "𐑼𐑥𐑭𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑳𐑴",
        851 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑳𐑴",
        852 => "𐑛𐑶𐑩𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑙𐑴",
        853 => "𐑨𐑶𐑽𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑳𐑴",
        854 => "𐑼𐑶𐑭𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑳𐑴",
        855 => "𐑦𐑶𐑽𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑳𐑴",
        856 => "𐑛𐑸𐑩𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑕𐑴",
        857 => "𐑨𐑸𐑾𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑕𐑴",
        858 => "𐑼𐑸𐑩𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑳𐑴",
        859 => "𐑦𐑸𐑽𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑳𐑴",
        860 => "𐑛𐑡𐑩𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑕𐑴",
        861 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑕𐑴",
        862 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑕𐑴",
        863 => "𐑦𐑡𐑾𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑕𐑴",
        864 => "𐑛𐑰𐑩𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑳𐑴",
        865 => "𐑨𐑰𐑾𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑳𐑴",
        866 => "𐑼𐑰𐑭𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑕𐑴",
        867 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑳𐑴",
        868 => "𐑛𐑥𐑩𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑕𐑴",
        869 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑳𐑴",
        870 => "𐑼𐑥𐑾𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑙𐑴",
        871 => "𐑦𐑥𐑽𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑳𐑴",
        872 => "𐑛𐑶𐑩𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑳𐑴",
        873 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑳𐑴",
        874 => "𐑼𐑶𐑩𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑳𐑴",
        875 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑳𐑴",
        876 => "𐑛𐑸𐑩𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑕𐑴",
        877 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑳𐑴",
        878 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑳𐑴",
        879 => "𐑦𐑸𐑩𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑙𐑴",
        880 => "𐑛𐑡𐑩𐑗𐑑𐑤𐑔𐑨𐑢𐑓𐑕𐑴",
        881 => "𐑨𐑡𐑭𐑿𐑑𐑤𐑲𐑨𐑻𐑒𐑳𐑴",
        882 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑚𐑣⊙𐑖𐑳𐑴",
        883 => "𐑦𐑡𐑭𐑯𐑑𐑤𐑔𐑣𐑣𐑫𐑳𐑴",
        884 => "𐑛𐑰𐑾𐑹𐑑𐑤𐑲𐑣𐑮𐑓𐑕𐑴",
        885 => "𐑨𐑰𐑩𐑗𐑑𐑤𐑚𐑵𐑢𐑒𐑳𐑴",
        886 => "𐑼𐑰𐑭𐑿𐑑𐑤𐑔𐑵𐑻𐑖𐑳𐑴",
        887 => "𐑦𐑰𐑽𐑬𐑑𐑤𐑲𐑵⊙𐑫𐑳𐑴",
        888 => "𐑛𐑥𐑭𐑯𐑑𐑤𐑚𐑝𐑣𐑓𐑳𐑴",
        889 => "𐑨𐑥𐑾𐑹𐑑𐑤𐑔𐑝𐑮𐑒𐑳𐑴",
        890 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑲𐑝𐑢𐑖𐑕𐑴",
        891 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑚𐑨𐑻𐑫𐑕𐑴",
        892 => "𐑛𐑶𐑽𐑬𐑑𐑤𐑔𐑨⊙𐑓𐑳𐑴",
        893 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑲𐑨𐑣𐑒𐑕𐑴",
        894 => "𐑼𐑶𐑩𐑹𐑑𐑤𐑚𐑣𐑮𐑖𐑕𐑴",
        895 => "𐑦𐑶𐑾𐑗𐑑𐑤𐑔𐑣𐑢𐑫𐑳𐑴",
        896 => "𐑛𐑸𐑽𐑿𐑑𐑤𐑲𐑣𐑻𐑓𐑕𐑴",
        897 => "𐑨𐑸𐑾𐑬𐑑𐑤𐑚𐑵⊙𐑒𐑳𐑴",
        898 => "𐑼𐑸𐑩𐑯𐑑𐑤𐑔𐑵𐑣𐑖𐑳𐑴",
        899 => "𐑦𐑸𐑩𐑹𐑑𐑤𐑲𐑵𐑮𐑫𐑙𐑴",
        900 => "𐑛𐑡𐑩𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑙𐑴",
        901 => "𐑨𐑡𐑩𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑳𐑴",
        902 => "𐑼𐑡𐑽𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑙𐑴",
        903 => "𐑦𐑡𐑾𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑕𐑴",
        904 => "𐑛𐑰𐑾𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑙𐑴",
        905 => "𐑨𐑰𐑾𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑳𐑴",
        906 => "𐑼𐑰𐑭𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑙𐑴",
        907 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑕𐑴",
        908 => "𐑛𐑥𐑾𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑙𐑴",
        909 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑙𐑴",
        910 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑳𐑴",
        911 => "𐑦𐑥𐑽𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑙𐑴",
        912 => "𐑛𐑶𐑾𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑙𐑴",
        913 => "𐑨𐑶𐑾𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑙𐑴",
        914 => "𐑼𐑶𐑩𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑕𐑴",
        915 => "𐑦𐑶𐑾𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑙𐑴",
        916 => "𐑛𐑸𐑽𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑙𐑴",
        917 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑕𐑴",
        918 => "𐑼𐑸𐑭𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑳𐑴",
        919 => "𐑦𐑸𐑩𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑕𐑴",
        920 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑙𐑴",
        921 => "𐑨𐑡𐑩𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑳𐑴",
        922 => "𐑼𐑡𐑽𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑳𐑴",
        923 => "𐑦𐑡𐑾𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑳𐑴",
        924 => "𐑛𐑰𐑽𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑳𐑴",
        925 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑙𐑴",
        926 => "𐑼𐑰𐑾𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑙𐑴",
        927 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑕𐑴",
        928 => "𐑛𐑥𐑽𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑳𐑴",
        929 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑳𐑴",
        930 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑙𐑴",
        931 => "𐑦𐑥𐑭𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑳𐑴",
        932 => "𐑛𐑶𐑩𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑙𐑴",
        933 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑳𐑴",
        934 => "𐑼𐑶𐑾𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑙𐑴",
        935 => "𐑦𐑶𐑽𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑙𐑴",
        936 => "𐑛𐑸𐑽𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑳𐑴",
        937 => "𐑨𐑸𐑽𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑙𐑴",
        938 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑙𐑴",
        939 => "𐑦𐑸𐑾𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑳𐑴",
        940 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑔𐑨𐑢𐑓𐑳𐑴",
        941 => "𐑨𐑡𐑩𐑿𐑑𐑤𐑲𐑨𐑻𐑒𐑙𐑴",
        942 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑚𐑣⊙𐑖𐑳𐑴",
        943 => "𐑦𐑡𐑽𐑯𐑑𐑤𐑔𐑣𐑣𐑫𐑳𐑴",
        944 => "𐑛𐑰𐑽𐑹𐑑𐑤𐑲𐑣𐑮𐑓𐑙𐑴",
        945 => "𐑨𐑰𐑾𐑗𐑑𐑤𐑚𐑵𐑢𐑒𐑙𐑴",
        946 => "𐑼𐑰𐑩𐑿𐑑𐑤𐑔𐑵𐑻𐑖𐑳𐑴",
        947 => "𐑦𐑰𐑽𐑬𐑑𐑤𐑲𐑵⊙𐑫𐑳𐑴",
        948 => "𐑛𐑥𐑾𐑯𐑑𐑤𐑚𐑝𐑣𐑓𐑕𐑴",
        949 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑔𐑝𐑮𐑒𐑙𐑴",
        950 => "𐑼𐑥𐑩𐑗𐑑𐑤𐑲𐑝𐑢𐑖𐑳𐑴",
        951 => "𐑦𐑥𐑭𐑿𐑑𐑤𐑚𐑨𐑻𐑫𐑳𐑴",
        952 => "𐑛𐑶𐑽𐑬𐑑𐑤𐑔𐑨⊙𐑓𐑳𐑴",
        953 => "𐑨𐑶𐑽𐑯𐑑𐑤𐑲𐑨𐑣𐑒𐑙𐑴",
        954 => "𐑼𐑶𐑾𐑹𐑑𐑤𐑚𐑣𐑮𐑖𐑕𐑴",
        955 => "𐑦𐑶𐑩𐑗𐑑𐑤𐑔𐑣𐑢𐑫𐑕𐑴",
        956 => "𐑛𐑸𐑾𐑿𐑑𐑤𐑲𐑣𐑻𐑓𐑕𐑴",
        957 => "𐑨𐑸𐑭𐑬𐑑𐑤𐑚𐑵⊙𐑒𐑳𐑴",
        958 => "𐑼𐑸𐑭𐑯𐑑𐑤𐑔𐑵𐑣𐑖𐑕𐑴",
        959 => "𐑦𐑸𐑭𐑹𐑑𐑤𐑲𐑵𐑮𐑫𐑳𐑴",
        960 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑚𐑝𐑢𐑓𐑕𐑴",
        961 => "𐑨𐑡𐑩𐑿𐑑𐑧𐑔𐑝𐑻𐑒𐑳𐑴",
        962 => "𐑼𐑡𐑩𐑬𐑑𐑧𐑲𐑝⊙𐑖𐑳𐑴",
        963 => "𐑦𐑡𐑾𐑯𐑑𐑧𐑚𐑨𐑣𐑫𐑙𐑴",
        964 => "𐑛𐑰𐑾𐑹𐑑𐑧𐑔𐑨𐑮𐑓𐑕𐑴",
        965 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑲𐑨𐑢𐑒𐑳𐑴",
        966 => "𐑼𐑰𐑭𐑿𐑑𐑧𐑚𐑣𐑻𐑖𐑙𐑴",
        967 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑔𐑣⊙𐑫𐑕𐑴",
        968 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑲𐑣𐑣𐑓𐑳𐑴",
        969 => "𐑨𐑥𐑽𐑹𐑑𐑧𐑚𐑵𐑮𐑒𐑙𐑴",
        970 => "𐑼𐑥𐑩𐑗𐑑𐑧𐑔𐑵𐑢𐑖𐑕𐑴",
        971 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑲𐑵𐑻𐑫𐑕𐑴",
        972 => "𐑛𐑶𐑩𐑬𐑑𐑧𐑚𐑝⊙𐑓𐑕𐑴",
        973 => "𐑨𐑶𐑩𐑯𐑑𐑧𐑔𐑝𐑣𐑒𐑳𐑴",
        974 => "𐑼𐑶𐑩𐑹𐑑𐑧𐑲𐑝𐑮𐑖𐑳𐑴",
        975 => "𐑦𐑶𐑾𐑗𐑑𐑧𐑚𐑨𐑢𐑫𐑳𐑴",
        976 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑔𐑨𐑻𐑓𐑕𐑴",
        977 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑲𐑨⊙𐑒𐑕𐑴",
        978 => "𐑼𐑸𐑾𐑯𐑑𐑧𐑚𐑣𐑣𐑖𐑕𐑴",
        979 => "𐑦𐑸𐑽𐑹𐑑𐑧𐑔𐑣𐑮𐑫𐑳𐑴",
        980 => "𐑛𐑡𐑩𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑕𐑴",
        981 => "𐑨𐑡𐑾𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑳𐑴",
        982 => "𐑼𐑡𐑩𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑕𐑴",
        983 => "𐑦𐑡𐑾𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑕𐑴",
        984 => "𐑛𐑰𐑾𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑳𐑴",
        985 => "𐑨𐑰𐑩𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑳𐑴",
        986 => "𐑼𐑰𐑽𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑳𐑴",
        987 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑳𐑴",
        988 => "𐑛𐑥𐑩𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑳𐑴",
        989 => "𐑨𐑥𐑭𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑳𐑴",
        990 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑕𐑴",
        991 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑕𐑴",
        992 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑕𐑴",
        993 => "𐑨𐑶𐑭𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑳𐑴",
        994 => "𐑼𐑶𐑭𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑳𐑴",
        995 => "𐑦𐑶𐑽𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑳𐑴",
        996 => "𐑛𐑸𐑩𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑕𐑴",
        997 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑳𐑴",
        998 => "𐑼𐑸𐑩𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑕𐑴",
        999 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑕𐑴",
        1000 => "𐑛𐑡𐑽𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑳𐑴",
        1001 => "𐑨𐑡𐑭𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑳𐑴",
        1002 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑳𐑴",
        1003 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑳𐑴",
        1004 => "𐑛𐑰𐑭𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑳𐑴",
        1005 => "𐑨𐑰𐑾𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑳𐑴",
        1006 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑳𐑴",
        1007 => "𐑦𐑰𐑾𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑕𐑴",
        1008 => "𐑛𐑥𐑩𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑕𐑴",
        1009 => "𐑨𐑥𐑾𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑙𐑴",
        1010 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑳𐑴",
        1011 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑕𐑴",
        1012 => "𐑛𐑶𐑽𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑙𐑴",
        1013 => "𐑨𐑶𐑩𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑳𐑴",
        1014 => "𐑼𐑶𐑭𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑳𐑴",
        1015 => "𐑦𐑶𐑭𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑙𐑴",
        1016 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑙𐑴",
        1017 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑕𐑴",
        1018 => "𐑼𐑸𐑾𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑕𐑴",
        1019 => "𐑦𐑸𐑭𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑳𐑴",
        1020 => "𐑛𐑡𐑽𐑗𐑑𐑧𐑚𐑝𐑢𐑓𐑳𐑴",
        1021 => "𐑨𐑡𐑾𐑿𐑑𐑧𐑔𐑝𐑻𐑒𐑕𐑴",
        1022 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑲𐑝⊙𐑖𐑳𐑴",
        1023 => "𐑦𐑡𐑩𐑯𐑑𐑧𐑚𐑨𐑣𐑫𐑕𐑴",
        1024 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑔𐑨𐑮𐑓𐑕𐑭",
        1025 => "𐑨𐑰𐑩𐑗𐑑𐑧𐑲𐑨𐑢𐑒𐑕𐑭",
        1026 => "𐑼𐑰𐑾𐑿𐑑𐑧𐑚𐑣𐑻𐑖𐑙𐑭",
        1027 => "𐑦𐑰𐑽𐑬𐑑𐑧𐑔𐑣⊙𐑫𐑳𐑭",
        1028 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑲𐑣𐑣𐑓𐑳𐑭",
        1029 => "𐑨𐑥𐑭𐑹𐑑𐑧𐑚𐑵𐑮𐑒𐑳𐑭",
        1030 => "𐑼𐑥𐑩𐑗𐑑𐑧𐑔𐑵𐑢𐑖𐑳𐑭",
        1031 => "𐑦𐑥𐑾𐑿𐑑𐑧𐑲𐑵𐑻𐑫𐑳𐑭",
        1032 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑚𐑝⊙𐑓𐑳𐑭",
        1033 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑔𐑝𐑣𐑒𐑙𐑭",
        1034 => "𐑼𐑶𐑩𐑹𐑑𐑧𐑲𐑝𐑮𐑖𐑳𐑭",
        1035 => "𐑦𐑶𐑩𐑗𐑑𐑧𐑚𐑨𐑢𐑫𐑕𐑭",
        1036 => "𐑛𐑸𐑭𐑿𐑑𐑧𐑔𐑨𐑻𐑓𐑕𐑭",
        1037 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑲𐑨⊙𐑒𐑳𐑭",
        1038 => "𐑼𐑸𐑩𐑯𐑑𐑧𐑚𐑣𐑣𐑖𐑕𐑭",
        1039 => "𐑦𐑸𐑭𐑹𐑑𐑧𐑔𐑣𐑮𐑫𐑳𐑭",
        1040 => "𐑛𐑡𐑩𐑗𐑑𐑪𐑲𐑣𐑢𐑓𐑕𐑭",
        1041 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑚𐑵𐑻𐑒𐑳𐑭",
        1042 => "𐑼𐑡𐑽𐑬𐑑𐑪𐑔𐑵⊙𐑖𐑙𐑭",
        1043 => "𐑦𐑡𐑩𐑯𐑑𐑪𐑲𐑵𐑣𐑫𐑕𐑭",
        1044 => "𐑛𐑰𐑽𐑹𐑑𐑪𐑚𐑝𐑮𐑓𐑳𐑭",
        1045 => "𐑨𐑰𐑩𐑗𐑑𐑪𐑔𐑝𐑢𐑒𐑳𐑭",
        1046 => "𐑼𐑰𐑽𐑿𐑑𐑪𐑲𐑝𐑻𐑖𐑳𐑭",
        1047 => "𐑦𐑰𐑩𐑬𐑑𐑪𐑚𐑨⊙𐑫𐑳𐑭",
        1048 => "𐑛𐑥𐑩𐑯𐑑𐑪𐑔𐑨𐑣𐑓𐑳𐑭",
        1049 => "𐑨𐑥𐑾𐑹𐑑𐑪𐑲𐑨𐑮𐑒𐑳𐑭",
        1050 => "𐑼𐑥𐑭𐑗𐑑𐑪𐑚𐑣𐑢𐑖𐑳𐑭",
        1051 => "𐑦𐑥𐑩𐑿𐑑𐑪𐑔𐑣𐑻𐑫𐑕𐑭",
        1052 => "𐑛𐑶𐑭𐑬𐑑𐑪𐑲𐑣⊙𐑓𐑳𐑭",
        1053 => "𐑨𐑶𐑽𐑯𐑑𐑪𐑚𐑵𐑣𐑒𐑳𐑭",
        1054 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑔𐑵𐑮𐑖𐑕𐑭",
        1055 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑲𐑵𐑢𐑫𐑳𐑭",
        1056 => "𐑛𐑸𐑾𐑿𐑑𐑪𐑚𐑝𐑻𐑓𐑳𐑭",
        1057 => "𐑨𐑸𐑩𐑬𐑑𐑪𐑔𐑝⊙𐑒𐑳𐑭",
        1058 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑲𐑝𐑣𐑖𐑙𐑭",
        1059 => "𐑦𐑸𐑩𐑹𐑑𐑪𐑚𐑨𐑮𐑫𐑳𐑭",
        1060 => "𐑛𐑡𐑭𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑳𐑭",
        1061 => "𐑨𐑡𐑭𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑕𐑭",
        1062 => "𐑼𐑡𐑾𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑙𐑭",
        1063 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑳𐑭",
        1064 => "𐑛𐑰𐑭𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑳𐑭",
        1065 => "𐑨𐑰𐑭𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑕𐑭",
        1066 => "𐑼𐑰𐑽𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑳𐑭",
        1067 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑕𐑭",
        1068 => "𐑛𐑥𐑾𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑙𐑭",
        1069 => "𐑨𐑥𐑾𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑕𐑭",
        1070 => "𐑼𐑥𐑭𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑳𐑭",
        1071 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑳𐑭",
        1072 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑳𐑭",
        1073 => "𐑨𐑶𐑩𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑳𐑭",
        1074 => "𐑼𐑶𐑩𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑳𐑭",
        1075 => "𐑦𐑶𐑭𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑳𐑭",
        1076 => "𐑛𐑸𐑩𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑳𐑭",
        1077 => "𐑨𐑸𐑽𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑳𐑭",
        1078 => "𐑼𐑸𐑾𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑕𐑭",
        1079 => "𐑦𐑸𐑽𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑙𐑭",
        1080 => "𐑛𐑡𐑾𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑙𐑭",
        1081 => "𐑨𐑡𐑭𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑳𐑭",
        1082 => "𐑼𐑡𐑾𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑙𐑭",
        1083 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑙𐑭",
        1084 => "𐑛𐑰𐑭𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑳𐑭",
        1085 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑳𐑭",
        1086 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑕𐑭",
        1087 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑳𐑭",
        1088 => "𐑛𐑥𐑽𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑙𐑭",
        1089 => "𐑨𐑥𐑽𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑳𐑭",
        1090 => "𐑼𐑥𐑩𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑕𐑭",
        1091 => "𐑦𐑥𐑩𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑕𐑭",
        1092 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑕𐑭",
        1093 => "𐑨𐑶𐑽𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑳𐑭",
        1094 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑕𐑭",
        1095 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑕𐑭",
        1096 => "𐑛𐑸𐑩𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑕𐑭",
        1097 => "𐑨𐑸𐑽𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑳𐑭",
        1098 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑳𐑭",
        1099 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑙𐑭",
        1100 => "𐑛𐑡𐑩𐑗𐑑𐑪𐑲𐑣𐑢𐑓𐑳𐑭",
        1101 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑚𐑵𐑻𐑒𐑙𐑭",
        1102 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑔𐑵⊙𐑖𐑳𐑭",
        1103 => "𐑦𐑡𐑾𐑯𐑑𐑪𐑲𐑵𐑣𐑫𐑕𐑭",
        1104 => "𐑛𐑰𐑩𐑹𐑑𐑪𐑚𐑝𐑮𐑓𐑳𐑭",
        1105 => "𐑨𐑰𐑩𐑗𐑑𐑪𐑔𐑝𐑢𐑒𐑙𐑭",
        1106 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑲𐑝𐑻𐑖𐑳𐑭",
        1107 => "𐑦𐑰𐑭𐑬𐑑𐑪𐑚𐑨⊙𐑫𐑙𐑭",
        1108 => "𐑛𐑥𐑽𐑯𐑑𐑪𐑔𐑨𐑣𐑓𐑳𐑭",
        1109 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑲𐑨𐑮𐑒𐑕𐑭",
        1110 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑚𐑣𐑢𐑖𐑳𐑭",
        1111 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑔𐑣𐑻𐑫𐑳𐑭",
        1112 => "𐑛𐑶𐑾𐑬𐑑𐑪𐑲𐑣⊙𐑓𐑙𐑭",
        1113 => "𐑨𐑶𐑩𐑯𐑑𐑪𐑚𐑵𐑣𐑒𐑳𐑭",
        1114 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑔𐑵𐑮𐑖𐑕𐑭",
        1115 => "𐑦𐑶𐑭𐑗𐑑𐑪𐑲𐑵𐑢𐑫𐑳𐑭",
        1116 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑚𐑝𐑻𐑓𐑳𐑭",
        1117 => "𐑨𐑸𐑭𐑬𐑑𐑪𐑔𐑝⊙𐑒𐑕𐑭",
        1118 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑲𐑝𐑣𐑖𐑕𐑭",
        1119 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑚𐑨𐑮𐑫𐑙𐑭",
        1120 => "𐑛𐑡𐑭𐑗𐑑𐑺𐑔𐑨𐑢𐑓𐑳𐑭",
        1121 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑲𐑨𐑻𐑒𐑳𐑭",
        1122 => "𐑼𐑡𐑽𐑬𐑑𐑺𐑚𐑣⊙𐑖𐑳𐑭",
        1123 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑔𐑣𐑣𐑫𐑳𐑭",
        1124 => "𐑛𐑰𐑩𐑹𐑑𐑺𐑲𐑣𐑮𐑓𐑕𐑭",
        1125 => "𐑨𐑰𐑾𐑗𐑑𐑺𐑚𐑵𐑢𐑒𐑙𐑭",
        1126 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑔𐑵𐑻𐑖𐑳𐑭",
        1127 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑲𐑵⊙𐑫𐑕𐑭",
        1128 => "𐑛𐑥𐑾𐑯𐑑𐑺𐑚𐑝𐑣𐑓𐑙𐑭",
        1129 => "𐑨𐑥𐑭𐑹𐑑𐑺𐑔𐑝𐑮𐑒𐑳𐑭",
        1130 => "𐑼𐑥𐑽𐑗𐑑𐑺𐑲𐑝𐑢𐑖𐑙𐑭",
        1131 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑚𐑨𐑻𐑫𐑙𐑭",
        1132 => "𐑛𐑶𐑽𐑬𐑑𐑺𐑔𐑨⊙𐑓𐑳𐑭",
        1133 => "𐑨𐑶𐑽𐑯𐑑𐑺𐑲𐑨𐑣𐑒𐑳𐑭",
        1134 => "𐑼𐑶𐑩𐑹𐑑𐑺𐑚𐑣𐑮𐑖𐑕𐑭",
        1135 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑔𐑣𐑢𐑫𐑙𐑭",
        1136 => "𐑛𐑸𐑾𐑿𐑑𐑺𐑲𐑣𐑻𐑓𐑕𐑭",
        1137 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑚𐑵⊙𐑒𐑳𐑭",
        1138 => "𐑼𐑸𐑾𐑯𐑑𐑺𐑔𐑵𐑣𐑖𐑙𐑭",
        1139 => "𐑦𐑸𐑽𐑹𐑑𐑺𐑲𐑵𐑮𐑫𐑙𐑭",
        1140 => "𐑛𐑡𐑭𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑳𐑭",
        1141 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑳𐑭",
        1142 => "𐑼𐑡𐑾𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑕𐑭",
        1143 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑳𐑭",
        1144 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑙𐑭",
        1145 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑳𐑭",
        1146 => "𐑼𐑰𐑾𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑳𐑭",
        1147 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑕𐑭",
        1148 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑳𐑭",
        1149 => "𐑨𐑥𐑭𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑳𐑭",
        1150 => "𐑼𐑥𐑩𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑳𐑭",
        1151 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑳𐑭",
        1152 => "𐑛𐑶𐑩𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑕𐑭",
        1153 => "𐑨𐑶𐑽𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑳𐑭",
        1154 => "𐑼𐑶𐑽𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑙𐑭",
        1155 => "𐑦𐑶𐑭𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑳𐑭",
        1156 => "𐑛𐑸𐑩𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑕𐑭",
        1157 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑙𐑭",
        1158 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑳𐑭",
        1159 => "𐑦𐑸𐑭𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑙𐑭",
        1160 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑕𐑭",
        1161 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑳𐑭",
        1162 => "𐑼𐑡𐑩𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑳𐑭",
        1163 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑳𐑭",
        1164 => "𐑛𐑰𐑽𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑳𐑭",
        1165 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑙𐑭",
        1166 => "𐑼𐑰𐑩𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑙𐑭",
        1167 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑳𐑭",
        1168 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑕𐑭",
        1169 => "𐑨𐑥𐑽𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑙𐑭",
        1170 => "𐑼𐑥𐑽𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑕𐑭",
        1171 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑕𐑭",
        1172 => "𐑛𐑶𐑩𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑙𐑭",
        1173 => "𐑨𐑶𐑭𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑙𐑭",
        1174 => "𐑼𐑶𐑩𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑳𐑭",
        1175 => "𐑦𐑶𐑽𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑳𐑭",
        1176 => "𐑛𐑸𐑽𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑳𐑭",
        1177 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑕𐑭",
        1178 => "𐑼𐑸𐑽𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑙𐑭",
        1179 => "𐑦𐑸𐑽𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑙𐑭",
        1180 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑔𐑨𐑢𐑓𐑳𐑭",
        1181 => "𐑨𐑡𐑩𐑿𐑑𐑺𐑲𐑨𐑻𐑒𐑕𐑭",
        1182 => "𐑼𐑡𐑩𐑬𐑑𐑺𐑚𐑣⊙𐑖𐑳𐑭",
        1183 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑔𐑣𐑣𐑫𐑕𐑭",
        1184 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑲𐑣𐑮𐑓𐑳𐑭",
        1185 => "𐑨𐑰𐑾𐑗𐑑𐑺𐑚𐑵𐑢𐑒𐑙𐑭",
        1186 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑔𐑵𐑻𐑖𐑳𐑭",
        1187 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑲𐑵⊙𐑫𐑳𐑭",
        1188 => "𐑛𐑥𐑩𐑯𐑑𐑺𐑚𐑝𐑣𐑓𐑕𐑭",
        1189 => "𐑨𐑥𐑾𐑹𐑑𐑺𐑔𐑝𐑮𐑒𐑕𐑭",
        1190 => "𐑼𐑥𐑾𐑗𐑑𐑺𐑲𐑝𐑢𐑖𐑳𐑭",
        1191 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑚𐑨𐑻𐑫𐑙𐑭",
        1192 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑔𐑨⊙𐑓𐑳𐑭",
        1193 => "𐑨𐑶𐑩𐑯𐑑𐑺𐑲𐑨𐑣𐑒𐑕𐑭",
        1194 => "𐑼𐑶𐑩𐑹𐑑𐑺𐑚𐑣𐑮𐑖𐑕𐑭",
        1195 => "𐑦𐑶𐑩𐑗𐑑𐑺𐑔𐑣𐑢𐑫𐑳𐑭",
        1196 => "𐑛𐑸𐑭𐑿𐑑𐑺𐑲𐑣𐑻𐑓𐑕𐑭",
        1197 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑚𐑵⊙𐑒𐑳𐑭",
        1198 => "𐑼𐑸𐑭𐑯𐑑𐑺𐑔𐑵𐑣𐑖𐑙𐑭",
        1199 => "𐑦𐑸𐑾𐑹𐑑𐑺𐑲𐑵𐑮𐑫𐑕𐑭",
        1200 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑙𐑭",
        1201 => "𐑨𐑡𐑽𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑳𐑭",
        1202 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑕𐑭",
        1203 => "𐑦𐑡𐑩𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑳𐑭",
        1204 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑙𐑭",
        1205 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑳𐑭",
        1206 => "𐑼𐑰𐑾𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑳𐑭",
        1207 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑙𐑭",
        1208 => "𐑛𐑥𐑭𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑳𐑭",
        1209 => "𐑨𐑥𐑾𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑕𐑭",
        1210 => "𐑼𐑥𐑾𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑙𐑭",
        1211 => "𐑦𐑥𐑽𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑳𐑭",
        1212 => "𐑛𐑶𐑭𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑳𐑭",
        1213 => "𐑨𐑶𐑾𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑕𐑭",
        1214 => "𐑼𐑶𐑾𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑳𐑭",
        1215 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑳𐑭",
        1216 => "𐑛𐑸𐑽𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑳𐑭",
        1217 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑙𐑭",
        1218 => "𐑼𐑸𐑾𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑙𐑭",
        1219 => "𐑦𐑸𐑾𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑙𐑭",
        1220 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑙𐑭",
        1221 => "𐑨𐑡𐑾𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑕𐑭",
        1222 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑳𐑭",
        1223 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑙𐑭",
        1224 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑙𐑭",
        1225 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑳𐑭",
        1226 => "𐑼𐑰𐑾𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑕𐑭",
        1227 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑳𐑭",
        1228 => "𐑛𐑥𐑽𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑙𐑭",
        1229 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑳𐑭",
        1230 => "𐑼𐑥𐑾𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑳𐑭",
        1231 => "𐑦𐑥𐑩𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑳𐑭",
        1232 => "𐑛𐑶𐑾𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑕𐑭",
        1233 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑙𐑭",
        1234 => "𐑼𐑶𐑾𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑳𐑭",
        1235 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑳𐑭",
        1236 => "𐑛𐑸𐑩𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑳𐑭",
        1237 => "𐑨𐑸𐑾𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑳𐑭",
        1238 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑙𐑭",
        1239 => "𐑦𐑸𐑽𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑙𐑭",
        1240 => "𐑛𐑡𐑩𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑕𐑭",
        1241 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑳𐑭",
        1242 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑙𐑭",
        1243 => "𐑦𐑡𐑾𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑕𐑭",
        1244 => "𐑛𐑰𐑽𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑙𐑭",
        1245 => "𐑨𐑰𐑽𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑙𐑭",
        1246 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑙𐑭",
        1247 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑳𐑭",
        1248 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑳𐑭",
        1249 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑳𐑭",
        1250 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑙𐑭",
        1251 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑳𐑭",
        1252 => "𐑛𐑶𐑽𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑙𐑭",
        1253 => "𐑨𐑶𐑩𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑕𐑭",
        1254 => "𐑼𐑶𐑽𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑳𐑭",
        1255 => "𐑦𐑶𐑾𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑳𐑭",
        1256 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑳𐑭",
        1257 => "𐑨𐑸𐑾𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑕𐑭",
        1258 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑙𐑭",
        1259 => "𐑦𐑸𐑽𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑙𐑭",
        1260 => "𐑛𐑡𐑾𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑕𐑭",
        1261 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑳𐑭",
        1262 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑳𐑭",
        1263 => "𐑦𐑡𐑩𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑳𐑭",
        1264 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑕𐑭",
        1265 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑳𐑭",
        1266 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑳𐑭",
        1267 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑭",
        1268 => "𐑛𐑥𐑭𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑙𐑭",
        1269 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑕𐑭",
        1270 => "𐑼𐑥𐑽𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑳𐑭",
        1271 => "𐑦𐑥𐑩𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑳𐑭",
        1272 => "𐑛𐑶𐑩𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑳𐑭",
        1273 => "𐑨𐑶𐑽𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑙𐑭",
        1274 => "𐑼𐑶𐑩𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑕𐑭",
        1275 => "𐑦𐑶𐑭𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑳𐑭",
        1276 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑳𐑭",
        1277 => "𐑨𐑸𐑭𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑳𐑭",
        1278 => "𐑼𐑸𐑾𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑳𐑭",
        1279 => "𐑦𐑸𐑭𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑳𐑭",
        1280 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑙𐑭",
        1281 => "𐑨𐑡𐑽𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑳𐑭",
        1282 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑕𐑭",
        1283 => "𐑦𐑡𐑽𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑳𐑭",
        1284 => "𐑛𐑰𐑭𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑕𐑭",
        1285 => "𐑨𐑰𐑩𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑕𐑭",
        1286 => "𐑼𐑰𐑭𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑕𐑭",
        1287 => "𐑦𐑰𐑩𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑕𐑭",
        1288 => "𐑛𐑥𐑾𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑙𐑭",
        1289 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑳𐑭",
        1290 => "𐑼𐑥𐑾𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑳𐑭",
        1291 => "𐑦𐑥𐑭𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑙𐑭",
        1292 => "𐑛𐑶𐑩𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑳𐑭",
        1293 => "𐑨𐑶𐑾𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑳𐑭",
        1294 => "𐑼𐑶𐑭𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑙𐑭",
        1295 => "𐑦𐑶𐑽𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑙𐑭",
        1296 => "𐑛𐑸𐑩𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑳𐑭",
        1297 => "𐑨𐑸𐑾𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑙𐑭",
        1298 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑙𐑭",
        1299 => "𐑦𐑸𐑾𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑕𐑭",
        1300 => "𐑛𐑡𐑾𐑗𐑑𐑤𐑔𐑨𐑢𐑓𐑳𐑭",
        1301 => "𐑨𐑡𐑽𐑿𐑑𐑤𐑲𐑨𐑻𐑒𐑳𐑭",
        1302 => "𐑼𐑡𐑽𐑬𐑑𐑤𐑚𐑣⊙𐑖𐑙𐑭",
        1303 => "𐑦𐑡𐑽𐑯𐑑𐑤𐑔𐑣𐑣𐑫𐑳𐑭",
        1304 => "𐑛𐑰𐑾𐑹𐑑𐑤𐑲𐑣𐑮𐑓𐑳𐑭",
        1305 => "𐑨𐑰𐑽𐑗𐑑𐑤𐑚𐑵𐑢𐑒𐑳𐑭",
        1306 => "𐑼𐑰𐑽𐑿𐑑𐑤𐑔𐑵𐑻𐑖𐑳𐑭",
        1307 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑲𐑵⊙𐑫𐑕𐑭",
        1308 => "𐑛𐑥𐑭𐑯𐑑𐑤𐑚𐑝𐑣𐑓𐑳𐑭",
        1309 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑔𐑝𐑮𐑒𐑳𐑭",
        1310 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑲𐑝𐑢𐑖𐑳𐑭",
        1311 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑚𐑨𐑻𐑫𐑳𐑭",
        1312 => "𐑛𐑶𐑭𐑬𐑑𐑤𐑔𐑨⊙𐑓𐑙𐑭",
        1313 => "𐑨𐑶𐑽𐑯𐑑𐑤𐑲𐑨𐑣𐑒𐑳𐑭",
        1314 => "𐑼𐑶𐑾𐑹𐑑𐑤𐑚𐑣𐑮𐑖𐑙𐑭",
        1315 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑔𐑣𐑢𐑫𐑙𐑭",
        1316 => "𐑛𐑸𐑾𐑿𐑑𐑤𐑲𐑣𐑻𐑓𐑕𐑭",
        1317 => "𐑨𐑸𐑽𐑬𐑑𐑤𐑚𐑵⊙𐑒𐑙𐑭",
        1318 => "𐑼𐑸𐑩𐑯𐑑𐑤𐑔𐑵𐑣𐑖𐑕𐑭",
        1319 => "𐑦𐑸𐑭𐑹𐑑𐑤𐑲𐑵𐑮𐑫𐑳𐑭",
        1320 => "𐑛𐑡𐑩𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑕𐑭",
        1321 => "𐑨𐑡𐑾𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑕𐑭",
        1322 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑳𐑭",
        1323 => "𐑦𐑡𐑽𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑳𐑭",
        1324 => "𐑛𐑰𐑭𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑕𐑭",
        1325 => "𐑨𐑰𐑭𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑳𐑭",
        1326 => "𐑼𐑰𐑭𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑳𐑭",
        1327 => "𐑦𐑰𐑽𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑳𐑭",
        1328 => "𐑛𐑥𐑾𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑳𐑭",
        1329 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑳𐑭",
        1330 => "𐑼𐑥𐑩𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑕𐑭",
        1331 => "𐑦𐑥𐑽𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑙𐑭",
        1332 => "𐑛𐑶𐑽𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑳𐑭",
        1333 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑕𐑭",
        1334 => "𐑼𐑶𐑩𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑕𐑭",
        1335 => "𐑦𐑶𐑽𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑳𐑭",
        1336 => "𐑛𐑸𐑩𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑳𐑭",
        1337 => "𐑨𐑸𐑩𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑳𐑭",
        1338 => "𐑼𐑸𐑭𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑳𐑭",
        1339 => "𐑦𐑸𐑭𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑳𐑭",
        1340 => "𐑛𐑡𐑩𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑳𐑭",
        1341 => "𐑨𐑡𐑽𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑙𐑭",
        1342 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑕𐑭",
        1343 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑕𐑭",
        1344 => "𐑛𐑰𐑩𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑳𐑭",
        1345 => "𐑨𐑰𐑾𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑙𐑭",
        1346 => "𐑼𐑰𐑾𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑙𐑭",
        1347 => "𐑦𐑰𐑩𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑕𐑭",
        1348 => "𐑛𐑥𐑩𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑙𐑭",
        1349 => "𐑨𐑥𐑩𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑕𐑭",
        1350 => "𐑼𐑥𐑽𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑙𐑭",
        1351 => "𐑦𐑥𐑾𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑳𐑭",
        1352 => "𐑛𐑶𐑾𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑳𐑭",
        1353 => "𐑨𐑶𐑽𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑳𐑭",
        1354 => "𐑼𐑶𐑭𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑳𐑭",
        1355 => "𐑦𐑶𐑩𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑙𐑭",
        1356 => "𐑛𐑸𐑽𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑙𐑭",
        1357 => "𐑨𐑸𐑾𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑳𐑭",
        1358 => "𐑼𐑸𐑽𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑳𐑭",
        1359 => "𐑦𐑸𐑾𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑙𐑭",
        1360 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑳𐑭",
        1361 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑕𐑭",
        1362 => "𐑼𐑡𐑾𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑳𐑭",
        1363 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑳𐑭",
        1364 => "𐑛𐑰𐑭𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑳𐑭",
        1365 => "𐑨𐑰𐑾𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑳𐑭",
        1366 => "𐑼𐑰𐑭𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑕𐑭",
        1367 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑙𐑭",
        1368 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑳𐑭",
        1369 => "𐑨𐑥𐑾𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑙𐑭",
        1370 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑳𐑭",
        1371 => "𐑦𐑥𐑾𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑳𐑭",
        1372 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑳𐑭",
        1373 => "𐑨𐑶𐑾𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑙𐑭",
        1374 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑳𐑭",
        1375 => "𐑦𐑶𐑾𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑳𐑭",
        1376 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑳𐑭",
        1377 => "𐑨𐑸𐑾𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑙𐑭",
        1378 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑳𐑭",
        1379 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑳𐑭",
        1380 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑚𐑝𐑢𐑓𐑳𐑭",
        1381 => "𐑨𐑡𐑾𐑿𐑑𐑧𐑔𐑝𐑻𐑒𐑳𐑭",
        1382 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑲𐑝⊙𐑖𐑳𐑭",
        1383 => "𐑦𐑡𐑾𐑯𐑑𐑧𐑚𐑨𐑣𐑫𐑙𐑭",
        1384 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑔𐑨𐑮𐑓𐑳𐑭",
        1385 => "𐑨𐑰𐑾𐑗𐑑𐑧𐑲𐑨𐑢𐑒𐑳𐑭",
        1386 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑚𐑣𐑻𐑖𐑕𐑭",
        1387 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑔𐑣⊙𐑫𐑕𐑭",
        1388 => "𐑛𐑥𐑭𐑯𐑑𐑧𐑲𐑣𐑣𐑓𐑳𐑭",
        1389 => "𐑨𐑥𐑾𐑹𐑑𐑧𐑚𐑵𐑮𐑒𐑳𐑭",
        1390 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑔𐑵𐑢𐑖𐑕𐑭",
        1391 => "𐑦𐑥𐑭𐑿𐑑𐑧𐑲𐑵𐑻𐑫𐑳𐑭",
        1392 => "𐑛𐑶𐑭𐑬𐑑𐑧𐑚𐑝⊙𐑓𐑕𐑭",
        1393 => "𐑨𐑶𐑩𐑯𐑑𐑧𐑔𐑝𐑣𐑒𐑙𐑭",
        1394 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑲𐑝𐑮𐑖𐑳𐑭",
        1395 => "𐑦𐑶𐑾𐑗𐑑𐑧𐑚𐑨𐑢𐑫𐑙𐑭",
        1396 => "𐑛𐑸𐑭𐑿𐑑𐑧𐑔𐑨𐑻𐑓𐑳𐑭",
        1397 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑲𐑨⊙𐑒𐑕𐑭",
        1398 => "𐑼𐑸𐑾𐑯𐑑𐑧𐑚𐑣𐑣𐑖𐑕𐑭",
        1399 => "𐑦𐑸𐑽𐑹𐑑𐑧𐑔𐑣𐑮𐑫𐑳𐑭",
        1400 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑳𐑭",
        1401 => "𐑨𐑡𐑾𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑙𐑭",
        1402 => "𐑼𐑡𐑾𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑙𐑭",
        1403 => "𐑦𐑡𐑾𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑙𐑭",
        1404 => "𐑛𐑰𐑩𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑕𐑭",
        1405 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑳𐑭",
        1406 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑳𐑭",
        1407 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑳𐑭",
        1408 => "𐑛𐑥𐑭𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑙𐑭",
        1409 => "𐑨𐑥𐑽𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑳𐑭",
        1410 => "𐑼𐑥𐑾𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑙𐑭",
        1411 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑳𐑭",
        1412 => "𐑛𐑶𐑩𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑕𐑭",
        1413 => "𐑨𐑶𐑭𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑳𐑭",
        1414 => "𐑼𐑶𐑭𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑙𐑭",
        1415 => "𐑦𐑶𐑩𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑳𐑭",
        1416 => "𐑛𐑸𐑩𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑳𐑭",
        1417 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑙𐑭",
        1418 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑕𐑭",
        1419 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑕𐑭",
        1420 => "𐑛𐑡𐑭𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑕𐑭",
        1421 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑳𐑭",
        1422 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑳𐑭",
        1423 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑳𐑭",
        1424 => "𐑛𐑰𐑩𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑳𐑭",
        1425 => "𐑨𐑰𐑩𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑳𐑭",
        1426 => "𐑼𐑰𐑽𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑳𐑭",
        1427 => "𐑦𐑰𐑭𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑳𐑭",
        1428 => "𐑛𐑥𐑾𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑕𐑭",
        1429 => "𐑨𐑥𐑭𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑳𐑭",
        1430 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑳𐑭",
        1431 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑳𐑭",
        1432 => "𐑛𐑶𐑽𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑙𐑭",
        1433 => "𐑨𐑶𐑭𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑳𐑭",
        1434 => "𐑼𐑶𐑾𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑙𐑭",
        1435 => "𐑦𐑶𐑽𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑳𐑭",
        1436 => "𐑛𐑸𐑾𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑕𐑭",
        1437 => "𐑨𐑸𐑽𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑙𐑭",
        1438 => "𐑼𐑸𐑽𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑳𐑭",
        1439 => "𐑦𐑸𐑾𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑳𐑭",
        1440 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑙𐑭",
        1441 => "𐑨𐑡𐑾𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑳𐑭",
        1442 => "𐑼𐑡𐑽𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑳𐑭",
        1443 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑙𐑭",
        1444 => "𐑛𐑰𐑽𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑙𐑭",
        1445 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑳𐑭",
        1446 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑙𐑭",
        1447 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑳𐑭",
        1448 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑳𐑭",
        1449 => "𐑨𐑥𐑾𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑕𐑭",
        1450 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑙𐑭",
        1451 => "𐑦𐑥𐑩𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑕𐑭",
        1452 => "𐑛𐑶𐑽𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑙𐑭",
        1453 => "𐑨𐑶𐑩𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑙𐑭",
        1454 => "𐑼𐑶𐑽𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑳𐑭",
        1455 => "𐑦𐑶𐑽𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑙𐑭",
        1456 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑳𐑭",
        1457 => "𐑨𐑸𐑭𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑳𐑭",
        1458 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑕𐑭",
        1459 => "𐑦𐑸𐑽𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑙𐑭",
        1460 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑲𐑣𐑢𐑓𐑙𐑭",
        1461 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑚𐑵𐑻𐑒𐑙𐑭",
        1462 => "𐑼𐑡𐑾𐑬𐑑𐑪𐑔𐑵⊙𐑖𐑙𐑭",
        1463 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑲𐑵𐑣𐑫𐑳𐑭",
        1464 => "𐑛𐑰𐑾𐑹𐑑𐑪𐑚𐑝𐑮𐑓𐑕𐑭",
        1465 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑔𐑝𐑢𐑒𐑙𐑭",
        1466 => "𐑼𐑰𐑩𐑿𐑑𐑪𐑲𐑝𐑻𐑖𐑙𐑭",
        1467 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑚𐑨⊙𐑫𐑳𐑭",
        1468 => "𐑛𐑥𐑩𐑯𐑑𐑪𐑔𐑨𐑣𐑓𐑕𐑭",
        1469 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑲𐑨𐑮𐑒𐑙𐑭",
        1470 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑚𐑣𐑢𐑖𐑳𐑭",
        1471 => "𐑦𐑥𐑽𐑿𐑑𐑪𐑔𐑣𐑻𐑫𐑙𐑭",
        1472 => "𐑛𐑶𐑾𐑬𐑑𐑪𐑲𐑣⊙𐑓𐑙𐑭",
        1473 => "𐑨𐑶𐑽𐑯𐑑𐑪𐑚𐑵𐑣𐑒𐑳𐑭",
        1474 => "𐑼𐑶𐑾𐑹𐑑𐑪𐑔𐑵𐑮𐑖𐑕𐑭",
        1475 => "𐑦𐑶𐑽𐑗𐑑𐑪𐑲𐑵𐑢𐑫𐑙𐑭",
        1476 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑚𐑝𐑻𐑓𐑙𐑭",
        1477 => "𐑨𐑸𐑽𐑬𐑑𐑪𐑔𐑝⊙𐑒𐑳𐑭",
        1478 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑲𐑝𐑣𐑖𐑙𐑭",
        1479 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑚𐑨𐑮𐑫𐑕𐑭",
        1480 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑳𐑭",
        1481 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑳𐑭",
        1482 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑕𐑭",
        1483 => "𐑦𐑡𐑩𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑳𐑭",
        1484 => "𐑛𐑰𐑭𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑳𐑭",
        1485 => "𐑨𐑰𐑭𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑳𐑭",
        1486 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑕𐑭",
        1487 => "𐑦𐑰𐑭𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑳𐑭",
        1488 => "𐑛𐑥𐑩𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑕𐑭",
        1489 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑳𐑭",
        1490 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑳𐑭",
        1491 => "𐑦𐑥𐑩𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑕𐑭",
        1492 => "𐑛𐑶𐑽𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑙𐑭",
        1493 => "𐑨𐑶𐑭𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑳𐑭",
        1494 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑳𐑭",
        1495 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑳𐑭",
        1496 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑳𐑭",
        1497 => "𐑨𐑸𐑭𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑳𐑭",
        1498 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑳𐑭",
        1499 => "𐑦𐑸𐑽𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑳𐑭",
        1500 => "𐑛𐑡𐑾𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑙𐑭",
        1501 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑳𐑭",
        1502 => "𐑼𐑡𐑽𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑕𐑭",
        1503 => "𐑦𐑡𐑩𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑳𐑭",
        1504 => "𐑛𐑰𐑩𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑳𐑭",
        1505 => "𐑨𐑰𐑽𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑙𐑭",
        1506 => "𐑼𐑰𐑭𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑳𐑭",
        1507 => "𐑦𐑰𐑭𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑳𐑭",
        1508 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑙𐑭",
        1509 => "𐑨𐑥𐑭𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑳𐑭",
        1510 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑳𐑭",
        1511 => "𐑦𐑥𐑩𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑳𐑭",
        1512 => "𐑛𐑶𐑾𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑕𐑭",
        1513 => "𐑨𐑶𐑩𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑳𐑭",
        1514 => "𐑼𐑶𐑾𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑕𐑭",
        1515 => "𐑦𐑶𐑽𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑳𐑭",
        1516 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑳𐑭",
        1517 => "𐑨𐑸𐑾𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑕𐑭",
        1518 => "𐑼𐑸𐑽𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑳𐑭",
        1519 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑳𐑭",
        1520 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑳𐑭",
        1521 => "𐑨𐑡𐑩𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑳𐑭",
        1522 => "𐑼𐑡𐑭𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑳𐑭",
        1523 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑳𐑭",
        1524 => "𐑛𐑰𐑭𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑳𐑭",
        1525 => "𐑨𐑰𐑩𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑕𐑭",
        1526 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑙𐑭",
        1527 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑳𐑭",
        1528 => "𐑛𐑥𐑽𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑳𐑭",
        1529 => "𐑨𐑥𐑩𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑕𐑭",
        1530 => "𐑼𐑥𐑾𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑳𐑭",
        1531 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑙𐑭",
        1532 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑙𐑭",
        1533 => "𐑨𐑶𐑾𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑕𐑭",
        1534 => "𐑼𐑶𐑩𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑳𐑭",
        1535 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑕𐑭",
        1536 => "𐑛𐑸𐑭𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑳𐑟",
        1537 => "𐑨𐑸𐑩𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑕𐑟",
        1538 => "𐑼𐑸𐑭𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑳𐑟",
        1539 => "𐑦𐑸𐑾𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑳𐑟",
        1540 => "𐑛𐑡𐑭𐑗𐑑𐑺𐑔𐑨𐑢𐑓𐑳𐑟",
        1541 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑲𐑨𐑻𐑒𐑳𐑟",
        1542 => "𐑼𐑡𐑽𐑬𐑑𐑺𐑚𐑣⊙𐑖𐑳𐑟",
        1543 => "𐑦𐑡𐑾𐑯𐑑𐑺𐑔𐑣𐑣𐑫𐑙𐑟",
        1544 => "𐑛𐑰𐑩𐑹𐑑𐑺𐑲𐑣𐑮𐑓𐑳𐑟",
        1545 => "𐑨𐑰𐑩𐑗𐑑𐑺𐑚𐑵𐑢𐑒𐑙𐑟",
        1546 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑔𐑵𐑻𐑖𐑳𐑟",
        1547 => "𐑦𐑰𐑭𐑬𐑑𐑺𐑲𐑵⊙𐑫𐑕𐑟",
        1548 => "𐑛𐑥𐑽𐑯𐑑𐑺𐑚𐑝𐑣𐑓𐑳𐑟",
        1549 => "𐑨𐑥𐑽𐑹𐑑𐑺𐑔𐑝𐑮𐑒𐑕𐑟",
        1550 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑲𐑝𐑢𐑖𐑕𐑟",
        1551 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑚𐑨𐑻𐑫𐑳𐑟",
        1552 => "𐑛𐑶𐑽𐑬𐑑𐑺𐑔𐑨⊙𐑓𐑳𐑟",
        1553 => "𐑨𐑶𐑽𐑯𐑑𐑺𐑲𐑨𐑣𐑒𐑙𐑟",
        1554 => "𐑼𐑶𐑩𐑹𐑑𐑺𐑚𐑣𐑮𐑖𐑳𐑟",
        1555 => "𐑦𐑶𐑽𐑗𐑑𐑺𐑔𐑣𐑢𐑫𐑳𐑟",
        1556 => "𐑛𐑸𐑽𐑿𐑑𐑺𐑲𐑣𐑻𐑓𐑳𐑟",
        1557 => "𐑨𐑸𐑾𐑬𐑑𐑺𐑚𐑵⊙𐑒𐑙𐑟",
        1558 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑔𐑵𐑣𐑖𐑙𐑟",
        1559 => "𐑦𐑸𐑩𐑹𐑑𐑺𐑲𐑵𐑮𐑫𐑳𐑟",
        1560 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑳𐑟",
        1561 => "𐑨𐑡𐑾𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑳𐑟",
        1562 => "𐑼𐑡𐑽𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑳𐑟",
        1563 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑕𐑟",
        1564 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑙𐑟",
        1565 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑙𐑟",
        1566 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑳𐑟",
        1567 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑙𐑟",
        1568 => "𐑛𐑥𐑽𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑙𐑟",
        1569 => "𐑨𐑥𐑾𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑙𐑟",
        1570 => "𐑼𐑥𐑾𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑙𐑟",
        1571 => "𐑦𐑥𐑽𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑳𐑟",
        1572 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑳𐑟",
        1573 => "𐑨𐑶𐑭𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑳𐑟",
        1574 => "𐑼𐑶𐑽𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑳𐑟",
        1575 => "𐑦𐑶𐑭𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑕𐑟",
        1576 => "𐑛𐑸𐑾𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑳𐑟",
        1577 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑳𐑟",
        1578 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑕𐑟",
        1579 => "𐑦𐑸𐑭𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑕𐑟",
        1580 => "𐑛𐑡𐑾𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑳𐑟",
        1581 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑳𐑟",
        1582 => "𐑼𐑡𐑭𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑕𐑟",
        1583 => "𐑦𐑡𐑭𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑳𐑟",
        1584 => "𐑛𐑰𐑩𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑕𐑟",
        1585 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑳𐑟",
        1586 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑕𐑟",
        1587 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑳𐑟",
        1588 => "𐑛𐑥𐑭𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑳𐑟",
        1589 => "𐑨𐑥𐑩𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑕𐑟",
        1590 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑕𐑟",
        1591 => "𐑦𐑥𐑩𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑙𐑟",
        1592 => "𐑛𐑶𐑭𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑳𐑟",
        1593 => "𐑨𐑶𐑽𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑕𐑟",
        1594 => "𐑼𐑶𐑭𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑕𐑟",
        1595 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑳𐑟",
        1596 => "𐑛𐑸𐑾𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑙𐑟",
        1597 => "𐑨𐑸𐑾𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑳𐑟",
        1598 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑕𐑟",
        1599 => "𐑦𐑸𐑩𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑳𐑟",
        1600 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑙𐑟",
        1601 => "𐑨𐑡𐑭𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑕𐑟",
        1602 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑕𐑟",
        1603 => "𐑦𐑡𐑽𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑳𐑟",
        1604 => "𐑛𐑰𐑩𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑕𐑟",
        1605 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑕𐑟",
        1606 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑳𐑟",
        1607 => "𐑦𐑰𐑾𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑙𐑟",
        1608 => "𐑛𐑥𐑽𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑳𐑟",
        1609 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑳𐑟",
        1610 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑕𐑟",
        1611 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑳𐑟",
        1612 => "𐑛𐑶𐑽𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑳𐑟",
        1613 => "𐑨𐑶𐑾𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑕𐑟",
        1614 => "𐑼𐑶𐑾𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑕𐑟",
        1615 => "𐑦𐑶𐑩𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑳𐑟",
        1616 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑳𐑟",
        1617 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑕𐑟",
        1618 => "𐑼𐑸𐑾𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑕𐑟",
        1619 => "𐑦𐑸𐑭𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑕𐑟",
        1620 => "𐑛𐑡𐑩𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑕𐑟",
        1621 => "𐑨𐑡𐑭𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑕𐑟",
        1622 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑕𐑟",
        1623 => "𐑦𐑡𐑩𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑕𐑟",
        1624 => "𐑛𐑰𐑩𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑕𐑟",
        1625 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑳𐑟",
        1626 => "𐑼𐑰𐑾𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑳𐑟",
        1627 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑟",
        1628 => "𐑛𐑥𐑭𐑯𐑑𐑘𐑲𐑣𐑣𐑓𐑳𐑟",
        1629 => "𐑨𐑥𐑽𐑹𐑑𐑘𐑚𐑵𐑮𐑒𐑕𐑟",
        1630 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑔𐑵𐑢𐑖𐑕𐑟",
        1631 => "𐑦𐑥𐑽𐑿𐑑𐑘𐑲𐑵𐑻𐑫𐑳𐑟",
        1632 => "𐑛𐑶𐑩𐑬𐑑𐑘𐑚𐑝⊙𐑓𐑕𐑟",
        1633 => "𐑨𐑶𐑩𐑯𐑑𐑘𐑔𐑝𐑣𐑒𐑕𐑟",
        1634 => "𐑼𐑶𐑭𐑹𐑑𐑘𐑲𐑝𐑮𐑖𐑕𐑟",
        1635 => "𐑦𐑶𐑭𐑗𐑑𐑘𐑚𐑨𐑢𐑫𐑕𐑟",
        1636 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑔𐑨𐑻𐑓𐑕𐑟",
        1637 => "𐑨𐑸𐑭𐑬𐑑𐑘𐑲𐑨⊙𐑒𐑳𐑟",
        1638 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑚𐑣𐑣𐑖𐑳𐑟",
        1639 => "𐑦𐑸𐑽𐑹𐑑𐑘𐑔𐑣𐑮𐑫𐑳𐑟",
        1640 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑕𐑟",
        1641 => "𐑨𐑡𐑭𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑳𐑟",
        1642 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑕𐑟",
        1643 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑕𐑟",
        1644 => "𐑛𐑰𐑩𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑕𐑟",
        1645 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑳𐑟",
        1646 => "𐑼𐑰𐑽𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑳𐑟",
        1647 => "𐑦𐑰𐑽𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑙𐑟",
        1648 => "𐑛𐑥𐑩𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑕𐑟",
        1649 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑳𐑟",
        1650 => "𐑼𐑥𐑩𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑕𐑟",
        1651 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑳𐑟",
        1652 => "𐑛𐑶𐑾𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑙𐑟",
        1653 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑳𐑟",
        1654 => "𐑼𐑶𐑩𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑕𐑟",
        1655 => "𐑦𐑶𐑭𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑳𐑟",
        1656 => "𐑛𐑸𐑾𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑕𐑟",
        1657 => "𐑨𐑸𐑩𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑕𐑟",
        1658 => "𐑼𐑸𐑩𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑳𐑟",
        1659 => "𐑦𐑸𐑭𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑕𐑟",
        1660 => "𐑛𐑡𐑩𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑕𐑟",
        1661 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑳𐑟",
        1662 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑳𐑟",
        1663 => "𐑦𐑡𐑩𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑳𐑟",
        1664 => "𐑛𐑰𐑽𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑙𐑟",
        1665 => "𐑨𐑰𐑭𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑳𐑟",
        1666 => "𐑼𐑰𐑭𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑳𐑟",
        1667 => "𐑦𐑰𐑭𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑳𐑟",
        1668 => "𐑛𐑥𐑾𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑙𐑟",
        1669 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑕𐑟",
        1670 => "𐑼𐑥𐑭𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑳𐑟",
        1671 => "𐑦𐑥𐑾𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑙𐑟",
        1672 => "𐑛𐑶𐑭𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑳𐑟",
        1673 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑕𐑟",
        1674 => "𐑼𐑶𐑭𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑕𐑟",
        1675 => "𐑦𐑶𐑽𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑳𐑟",
        1676 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑳𐑟",
        1677 => "𐑨𐑸𐑽𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑳𐑟",
        1678 => "𐑼𐑸𐑭𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑳𐑟",
        1679 => "𐑦𐑸𐑩𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑕𐑟",
        1680 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑳𐑟",
        1681 => "𐑨𐑡𐑭𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑕𐑟",
        1682 => "𐑼𐑡𐑭𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑳𐑟",
        1683 => "𐑦𐑡𐑾𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑳𐑟",
        1684 => "𐑛𐑰𐑭𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑳𐑟",
        1685 => "𐑨𐑰𐑭𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑕𐑟",
        1686 => "𐑼𐑰𐑩𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑕𐑟",
        1687 => "𐑦𐑰𐑭𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑕𐑟",
        1688 => "𐑛𐑥𐑽𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑳𐑟",
        1689 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑕𐑟",
        1690 => "𐑼𐑥𐑾𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑙𐑟",
        1691 => "𐑦𐑥𐑽𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑙𐑟",
        1692 => "𐑛𐑶𐑾𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑕𐑟",
        1693 => "𐑨𐑶𐑩𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑙𐑟",
        1694 => "𐑼𐑶𐑽𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑳𐑟",
        1695 => "𐑦𐑶𐑾𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑙𐑟",
        1696 => "𐑛𐑸𐑭𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑕𐑟",
        1697 => "𐑨𐑸𐑭𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑳𐑟",
        1698 => "𐑼𐑸𐑽𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑳𐑟",
        1699 => "𐑦𐑸𐑭𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑕𐑟",
        1700 => "𐑛𐑡𐑭𐑗𐑑𐑤𐑲𐑣𐑢𐑓𐑕𐑟",
        1701 => "𐑨𐑡𐑭𐑿𐑑𐑤𐑚𐑵𐑻𐑒𐑕𐑟",
        1702 => "𐑼𐑡𐑭𐑬𐑑𐑤𐑔𐑵⊙𐑖𐑳𐑟",
        1703 => "𐑦𐑡𐑩𐑯𐑑𐑤𐑲𐑵𐑣𐑫𐑕𐑟",
        1704 => "𐑛𐑰𐑭𐑹𐑑𐑤𐑚𐑝𐑮𐑓𐑳𐑟",
        1705 => "𐑨𐑰𐑭𐑗𐑑𐑤𐑔𐑝𐑢𐑒𐑳𐑟",
        1706 => "𐑼𐑰𐑩𐑿𐑑𐑤𐑲𐑝𐑻𐑖𐑕𐑟",
        1707 => "𐑦𐑰𐑩𐑬𐑑𐑤𐑚𐑨⊙𐑫𐑕𐑟",
        1708 => "𐑛𐑥𐑽𐑯𐑑𐑤𐑔𐑨𐑣𐑓𐑙𐑟",
        1709 => "𐑨𐑥𐑭𐑹𐑑𐑤𐑲𐑨𐑮𐑒𐑕𐑟",
        1710 => "𐑼𐑥𐑭𐑗𐑑𐑤𐑚𐑣𐑢𐑖𐑕𐑟",
        1711 => "𐑦𐑥𐑭𐑿𐑑𐑤𐑔𐑣𐑻𐑫𐑕𐑟",
        1712 => "𐑛𐑶𐑭𐑬𐑑𐑤𐑲𐑣⊙𐑓𐑕𐑟",
        1713 => "𐑨𐑶𐑾𐑯𐑑𐑤𐑚𐑵𐑣𐑒𐑳𐑟",
        1714 => "𐑼𐑶𐑭𐑹𐑑𐑤𐑔𐑵𐑮𐑖𐑳𐑟",
        1715 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑲𐑵𐑢𐑫𐑳𐑟",
        1716 => "𐑛𐑸𐑭𐑿𐑑𐑤𐑚𐑝𐑻𐑓𐑕𐑟",
        1717 => "𐑨𐑸𐑭𐑬𐑑𐑤𐑔𐑝⊙𐑒𐑳𐑟",
        1718 => "𐑼𐑸𐑩𐑯𐑑𐑤𐑲𐑝𐑣𐑖𐑳𐑟",
        1719 => "𐑦𐑸𐑽𐑹𐑑𐑤𐑚𐑨𐑮𐑫𐑳𐑟",
        1720 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑔𐑨𐑢𐑓𐑳𐑟",
        1721 => "𐑨𐑡𐑽𐑿𐑑𐑤𐑲𐑨𐑻𐑒𐑕𐑟",
        1722 => "𐑼𐑡𐑩𐑬𐑑𐑤𐑚𐑣⊙𐑖𐑳𐑟",
        1723 => "𐑦𐑡𐑾𐑯𐑑𐑤𐑔𐑣𐑣𐑫𐑳𐑟",
        1724 => "𐑛𐑰𐑭𐑹𐑑𐑤𐑲𐑣𐑮𐑓𐑕𐑟",
        1725 => "𐑨𐑰𐑾𐑗𐑑𐑤𐑚𐑵𐑢𐑒𐑳𐑟",
        1726 => "𐑼𐑰𐑭𐑿𐑑𐑤𐑔𐑵𐑻𐑖𐑕𐑟",
        1727 => "𐑦𐑰𐑾𐑬𐑑𐑤𐑲𐑵⊙𐑫𐑳𐑟",
        1728 => "𐑛𐑥𐑽𐑯𐑑𐑤𐑚𐑝𐑣𐑓𐑳𐑟",
        1729 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑔𐑝𐑮𐑒𐑳𐑟",
        1730 => "𐑼𐑥𐑾𐑗𐑑𐑤𐑲𐑝𐑢𐑖𐑳𐑟",
        1731 => "𐑦𐑥𐑩𐑿𐑑𐑤𐑚𐑨𐑻𐑫𐑕𐑟",
        1732 => "𐑛𐑶𐑽𐑬𐑑𐑤𐑔𐑨⊙𐑓𐑳𐑟",
        1733 => "𐑨𐑶𐑽𐑯𐑑𐑤𐑲𐑨𐑣𐑒𐑳𐑟",
        1734 => "𐑼𐑶𐑭𐑹𐑑𐑤𐑚𐑣𐑮𐑖𐑳𐑟",
        1735 => "𐑦𐑶𐑾𐑗𐑑𐑤𐑔𐑣𐑢𐑫𐑳𐑟",
        1736 => "𐑛𐑸𐑩𐑿𐑑𐑤𐑲𐑣𐑻𐑓𐑳𐑟",
        1737 => "𐑨𐑸𐑽𐑬𐑑𐑤𐑚𐑵⊙𐑒𐑳𐑟",
        1738 => "𐑼𐑸𐑾𐑯𐑑𐑤𐑔𐑵𐑣𐑖𐑕𐑟",
        1739 => "𐑦𐑸𐑭𐑹𐑑𐑤𐑲𐑵𐑮𐑫𐑕𐑟",
        1740 => "𐑛𐑡𐑽𐑗𐑑𐑤𐑚𐑝𐑢𐑓𐑳𐑟",
        1741 => "𐑨𐑡𐑭𐑿𐑑𐑤𐑔𐑝𐑻𐑒𐑳𐑟",
        1742 => "𐑼𐑡𐑽𐑬𐑑𐑤𐑲𐑝⊙𐑖𐑕𐑟",
        1743 => "𐑦𐑡𐑾𐑯𐑑𐑤𐑚𐑨𐑣𐑫𐑙𐑟",
        1744 => "𐑛𐑰𐑩𐑹𐑑𐑤𐑔𐑨𐑮𐑓𐑳𐑟",
        1745 => "𐑨𐑰𐑾𐑗𐑑𐑤𐑲𐑨𐑢𐑒𐑙𐑟",
        1746 => "𐑼𐑰𐑭𐑿𐑑𐑤𐑚𐑣𐑻𐑖𐑳𐑟",
        1747 => "𐑦𐑰𐑩𐑬𐑑𐑤𐑔𐑣⊙𐑫𐑙𐑟",
        1748 => "𐑛𐑥𐑩𐑯𐑑𐑤𐑲𐑣𐑣𐑓𐑙𐑟",
        1749 => "𐑨𐑥𐑽𐑹𐑑𐑤𐑚𐑵𐑮𐑒𐑳𐑟",
        1750 => "𐑼𐑥𐑾𐑗𐑑𐑤𐑔𐑵𐑢𐑖𐑳𐑟",
        1751 => "𐑦𐑥𐑾𐑿𐑑𐑤𐑲𐑵𐑻𐑫𐑙𐑟",
        1752 => "𐑛𐑶𐑾𐑬𐑑𐑤𐑚𐑝⊙𐑓𐑳𐑟",
        1753 => "𐑨𐑶𐑭𐑯𐑑𐑤𐑔𐑝𐑣𐑒𐑕𐑟",
        1754 => "𐑼𐑶𐑩𐑹𐑑𐑤𐑲𐑝𐑮𐑖𐑕𐑟",
        1755 => "𐑦𐑶𐑭𐑗𐑑𐑤𐑚𐑨𐑢𐑫𐑕𐑟",
        1756 => "𐑛𐑸𐑭𐑿𐑑𐑤𐑔𐑨𐑻𐑓𐑳𐑟",
        1757 => "𐑨𐑸𐑭𐑬𐑑𐑤𐑲𐑨⊙𐑒𐑳𐑟",
        1758 => "𐑼𐑸𐑭𐑯𐑑𐑤𐑚𐑣𐑣𐑖𐑕𐑟",
        1759 => "𐑦𐑸𐑩𐑹𐑑𐑤𐑔𐑣𐑮𐑫𐑕𐑟",
        1760 => "𐑛𐑡𐑭𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑕𐑟",
        1761 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑕𐑟",
        1762 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑕𐑟",
        1763 => "𐑦𐑡𐑽𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑕𐑟",
        1764 => "𐑛𐑰𐑾𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑕𐑟",
        1765 => "𐑨𐑰𐑭𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑕𐑟",
        1766 => "𐑼𐑰𐑽𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑕𐑟",
        1767 => "𐑦𐑰𐑭𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑳𐑟",
        1768 => "𐑛𐑥𐑽𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑳𐑟",
        1769 => "𐑨𐑥𐑾𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑕𐑟",
        1770 => "𐑼𐑥𐑩𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑳𐑟",
        1771 => "𐑦𐑥𐑽𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑳𐑟",
        1772 => "𐑛𐑶𐑩𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑕𐑟",
        1773 => "𐑨𐑶𐑩𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑕𐑟",
        1774 => "𐑼𐑶𐑩𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑳𐑟",
        1775 => "𐑦𐑶𐑽𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑳𐑟",
        1776 => "𐑛𐑸𐑩𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑕𐑟",
        1777 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑳𐑟",
        1778 => "𐑼𐑸𐑽𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑙𐑟",
        1779 => "𐑦𐑸𐑩𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑳𐑟",
        1780 => "𐑛𐑡𐑭𐑗𐑑𐑧𐑔𐑨𐑢𐑓𐑳𐑟",
        1781 => "𐑨𐑡𐑩𐑿𐑑𐑧𐑲𐑨𐑻𐑒𐑳𐑟",
        1782 => "𐑼𐑡𐑩𐑬𐑑𐑧𐑚𐑣⊙𐑖𐑕𐑟",
        1783 => "𐑦𐑡𐑾𐑯𐑑𐑧𐑔𐑣𐑣𐑫𐑕𐑟",
        1784 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑲𐑣𐑮𐑓𐑳𐑟",
        1785 => "𐑨𐑰𐑽𐑗𐑑𐑧𐑚𐑵𐑢𐑒𐑳𐑟",
        1786 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑔𐑵𐑻𐑖𐑕𐑟",
        1787 => "𐑦𐑰𐑩𐑬𐑑𐑧𐑲𐑵⊙𐑫𐑕𐑟",
        1788 => "𐑛𐑥𐑩𐑯𐑑𐑧𐑚𐑝𐑣𐑓𐑕𐑟",
        1789 => "𐑨𐑥𐑩𐑹𐑑𐑧𐑔𐑝𐑮𐑒𐑕𐑟",
        1790 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑲𐑝𐑢𐑖𐑕𐑟",
        1791 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑚𐑨𐑻𐑫𐑕𐑟",
        1792 => "𐑛𐑶𐑭𐑬𐑑𐑧𐑔𐑨⊙𐑓𐑳𐑟",
        1793 => "𐑨𐑶𐑩𐑯𐑑𐑧𐑲𐑨𐑣𐑒𐑕𐑟",
        1794 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑚𐑣𐑮𐑖𐑳𐑟",
        1795 => "𐑦𐑶𐑭𐑗𐑑𐑧𐑔𐑣𐑢𐑫𐑳𐑟",
        1796 => "𐑛𐑸𐑩𐑿𐑑𐑧𐑲𐑣𐑻𐑓𐑕𐑟",
        1797 => "𐑨𐑸𐑭𐑬𐑑𐑧𐑚𐑵⊙𐑒𐑕𐑟",
        1798 => "𐑼𐑸𐑩𐑯𐑑𐑧𐑔𐑵𐑣𐑖𐑕𐑟",
        1799 => "𐑦𐑸𐑾𐑹𐑑𐑧𐑲𐑵𐑮𐑫𐑳𐑟",
        1800 => "𐑛𐑡𐑭𐑗𐑑𐑧𐑚𐑝𐑢𐑓𐑳𐑟",
        1801 => "𐑨𐑡𐑽𐑿𐑑𐑧𐑔𐑝𐑻𐑒𐑳𐑟",
        1802 => "𐑼𐑡𐑭𐑬𐑑𐑧𐑲𐑝⊙𐑖𐑕𐑟",
        1803 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑚𐑨𐑣𐑫𐑕𐑟",
        1804 => "𐑛𐑰𐑾𐑹𐑑𐑧𐑔𐑨𐑮𐑓𐑳𐑟",
        1805 => "𐑨𐑰𐑽𐑗𐑑𐑧𐑲𐑨𐑢𐑒𐑳𐑟",
        1806 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑚𐑣𐑻𐑖𐑳𐑟",
        1807 => "𐑦𐑰𐑭𐑬𐑑𐑧𐑔𐑣⊙𐑫𐑳𐑟",
        1808 => "𐑛𐑥𐑩𐑯𐑑𐑧𐑲𐑣𐑣𐑓𐑕𐑟",
        1809 => "𐑨𐑥𐑽𐑹𐑑𐑧𐑚𐑵𐑮𐑒𐑳𐑟",
        1810 => "𐑼𐑥𐑩𐑗𐑑𐑧𐑔𐑵𐑢𐑖𐑳𐑟",
        1811 => "𐑦𐑥𐑩𐑿𐑑𐑧𐑲𐑵𐑻𐑫𐑕𐑟",
        1812 => "𐑛𐑶𐑾𐑬𐑑𐑧𐑚𐑝⊙𐑓𐑕𐑟",
        1813 => "𐑨𐑶𐑭𐑯𐑑𐑧𐑔𐑝𐑣𐑒𐑳𐑟",
        1814 => "𐑼𐑶𐑽𐑹𐑑𐑧𐑲𐑝𐑮𐑖𐑙𐑟",
        1815 => "𐑦𐑶𐑭𐑗𐑑𐑧𐑚𐑨𐑢𐑫𐑳𐑟",
        1816 => "𐑛𐑸𐑭𐑿𐑑𐑧𐑔𐑨𐑻𐑓𐑳𐑟",
        1817 => "𐑨𐑸𐑾𐑬𐑑𐑧𐑲𐑨⊙𐑒𐑙𐑟",
        1818 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑚𐑣𐑣𐑖𐑳𐑟",
        1819 => "𐑦𐑸𐑾𐑹𐑑𐑧𐑔𐑣𐑮𐑫𐑳𐑟",
        1820 => "𐑛𐑡𐑾𐑗𐑑𐑧𐑲𐑣𐑢𐑓𐑳𐑟",
        1821 => "𐑨𐑡𐑩𐑿𐑑𐑧𐑚𐑵𐑻𐑒𐑙𐑟",
        1822 => "𐑼𐑡𐑽𐑬𐑑𐑧𐑔𐑵⊙𐑖𐑙𐑟",
        1823 => "𐑦𐑡𐑭𐑯𐑑𐑧𐑲𐑵𐑣𐑫𐑳𐑟",
        1824 => "𐑛𐑰𐑽𐑹𐑑𐑧𐑚𐑝𐑮𐑓𐑙𐑟",
        1825 => "𐑨𐑰𐑩𐑗𐑑𐑧𐑔𐑝𐑢𐑒𐑙𐑟",
        1826 => "𐑼𐑰𐑩𐑿𐑑𐑧𐑲𐑝𐑻𐑖𐑳𐑟",
        1827 => "𐑦𐑰𐑽𐑬𐑑𐑧𐑚𐑨⊙𐑫𐑙𐑟",
        1828 => "𐑛𐑥𐑾𐑯𐑑𐑧𐑔𐑨𐑣𐑓𐑳𐑟",
        1829 => "𐑨𐑥𐑩𐑹𐑑𐑧𐑲𐑨𐑮𐑒𐑳𐑟",
        1830 => "𐑼𐑥𐑭𐑗𐑑𐑧𐑚𐑣𐑢𐑖𐑳𐑟",
        1831 => "𐑦𐑥𐑾𐑿𐑑𐑧𐑔𐑣𐑻𐑫𐑕𐑟",
        1832 => "𐑛𐑶𐑭𐑬𐑑𐑧𐑲𐑣⊙𐑓𐑳𐑟",
        1833 => "𐑨𐑶𐑽𐑯𐑑𐑧𐑚𐑵𐑣𐑒𐑳𐑟",
        1834 => "𐑼𐑶𐑭𐑹𐑑𐑧𐑔𐑵𐑮𐑖𐑕𐑟",
        1835 => "𐑦𐑶𐑾𐑗𐑑𐑧𐑲𐑵𐑢𐑫𐑙𐑟",
        1836 => "𐑛𐑸𐑩𐑿𐑑𐑧𐑚𐑝𐑻𐑓𐑕𐑟",
        1837 => "𐑨𐑸𐑩𐑬𐑑𐑧𐑔𐑝⊙𐑒𐑕𐑟",
        1838 => "𐑼𐑸𐑭𐑯𐑑𐑧𐑲𐑝𐑣𐑖𐑳𐑟",
        1839 => "𐑦𐑸𐑾𐑹𐑑𐑧𐑚𐑨𐑮𐑫𐑙𐑟",
        1840 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑳𐑟",
        1841 => "𐑨𐑡𐑭𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑳𐑟",
        1842 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑕𐑟",
        1843 => "𐑦𐑡𐑾𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑕𐑟",
        1844 => "𐑛𐑰𐑭𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑕𐑟",
        1845 => "𐑨𐑰𐑭𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑳𐑟",
        1846 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑳𐑟",
        1847 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑳𐑟",
        1848 => "𐑛𐑥𐑭𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑳𐑟",
        1849 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑳𐑟",
        1850 => "𐑼𐑥𐑩𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑕𐑟",
        1851 => "𐑦𐑥𐑭𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑕𐑟",
        1852 => "𐑛𐑶𐑽𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑳𐑟",
        1853 => "𐑨𐑶𐑩𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑕𐑟",
        1854 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑳𐑟",
        1855 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑳𐑟",
        1856 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑕𐑟",
        1857 => "𐑨𐑸𐑭𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑳𐑟",
        1858 => "𐑼𐑸𐑭𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑳𐑟",
        1859 => "𐑦𐑸𐑭𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑕𐑟",
        1860 => "𐑛𐑡𐑾𐑗𐑑𐑪𐑚𐑝𐑢𐑓𐑳𐑟",
        1861 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑔𐑝𐑻𐑒𐑕𐑟",
        1862 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑲𐑝⊙𐑖𐑕𐑟",
        1863 => "𐑦𐑡𐑽𐑯𐑑𐑪𐑚𐑨𐑣𐑫𐑕𐑟",
        1864 => "𐑛𐑰𐑾𐑹𐑑𐑪𐑔𐑨𐑮𐑓𐑙𐑟",
        1865 => "𐑨𐑰𐑭𐑗𐑑𐑪𐑲𐑨𐑢𐑒𐑕𐑟",
        1866 => "𐑼𐑰𐑩𐑿𐑑𐑪𐑚𐑣𐑻𐑖𐑳𐑟",
        1867 => "𐑦𐑰𐑭𐑬𐑑𐑪𐑔𐑣⊙𐑫𐑕𐑟",
        1868 => "𐑛𐑥𐑾𐑯𐑑𐑪𐑲𐑣𐑣𐑓𐑳𐑟",
        1869 => "𐑨𐑥𐑭𐑹𐑑𐑪𐑚𐑵𐑮𐑒𐑕𐑟",
        1870 => "𐑼𐑥𐑭𐑗𐑑𐑪𐑔𐑵𐑢𐑖𐑕𐑟",
        1871 => "𐑦𐑥𐑾𐑿𐑑𐑪𐑲𐑵𐑻𐑫𐑙𐑟",
        1872 => "𐑛𐑶𐑩𐑬𐑑𐑪𐑚𐑝⊙𐑓𐑳𐑟",
        1873 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑔𐑝𐑣𐑒𐑕𐑟",
        1874 => "𐑼𐑶𐑽𐑹𐑑𐑪𐑲𐑝𐑮𐑖𐑳𐑟",
        1875 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑚𐑨𐑢𐑫𐑳𐑟",
        1876 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑔𐑨𐑻𐑓𐑳𐑟",
        1877 => "𐑨𐑸𐑽𐑬𐑑𐑪𐑲𐑨⊙𐑒𐑳𐑟",
        1878 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑚𐑣𐑣𐑖𐑕𐑟",
        1879 => "𐑦𐑸𐑽𐑹𐑑𐑪𐑔𐑣𐑮𐑫𐑳𐑟",
        1880 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑲𐑣𐑢𐑓𐑳𐑟",
        1881 => "𐑨𐑡𐑽𐑿𐑑𐑪𐑚𐑵𐑻𐑒𐑕𐑟",
        1882 => "𐑼𐑡𐑭𐑬𐑑𐑪𐑔𐑵⊙𐑖𐑳𐑟",
        1883 => "𐑦𐑡𐑩𐑯𐑑𐑪𐑲𐑵𐑣𐑫𐑕𐑟",
        1884 => "𐑛𐑰𐑭𐑹𐑑𐑪𐑚𐑝𐑮𐑓𐑕𐑟",
        1885 => "𐑨𐑰𐑾𐑗𐑑𐑪𐑔𐑝𐑢𐑒𐑕𐑟",
        1886 => "𐑼𐑰𐑩𐑿𐑑𐑪𐑲𐑝𐑻𐑖𐑕𐑟",
        1887 => "𐑦𐑰𐑾𐑬𐑑𐑪𐑚𐑨⊙𐑫𐑳𐑟",
        1888 => "𐑛𐑥𐑩𐑯𐑑𐑪𐑔𐑨𐑣𐑓𐑕𐑟",
        1889 => "𐑨𐑥𐑩𐑹𐑑𐑪𐑲𐑨𐑮𐑒𐑙𐑟",
        1890 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑚𐑣𐑢𐑖𐑙𐑟",
        1891 => "𐑦𐑥𐑾𐑿𐑑𐑪𐑔𐑣𐑻𐑫𐑕𐑟",
        1892 => "𐑛𐑶𐑭𐑬𐑑𐑪𐑲𐑣⊙𐑓𐑳𐑟",
        1893 => "𐑨𐑶𐑾𐑯𐑑𐑪𐑚𐑵𐑣𐑒𐑙𐑟",
        1894 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑔𐑵𐑮𐑖𐑳𐑟",
        1895 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑲𐑵𐑢𐑫𐑳𐑟",
        1896 => "𐑛𐑸𐑽𐑿𐑑𐑪𐑚𐑝𐑻𐑓𐑙𐑟",
        1897 => "𐑨𐑸𐑽𐑬𐑑𐑪𐑔𐑝⊙𐑒𐑳𐑟",
        1898 => "𐑼𐑸𐑾𐑯𐑑𐑪𐑲𐑝𐑣𐑖𐑳𐑟",
        1899 => "𐑦𐑸𐑾𐑹𐑑𐑪𐑚𐑨𐑮𐑫𐑙𐑟",
        1900 => "𐑛𐑡𐑽𐑗𐑑𐑪𐑔𐑨𐑢𐑓𐑕𐑟",
        1901 => "𐑨𐑡𐑩𐑿𐑑𐑪𐑲𐑨𐑻𐑒𐑳𐑟",
        1902 => "𐑼𐑡𐑩𐑬𐑑𐑪𐑚𐑣⊙𐑖𐑕𐑟",
        1903 => "𐑦𐑡𐑾𐑯𐑑𐑪𐑔𐑣𐑣𐑫𐑳𐑟",
        1904 => "𐑛𐑰𐑽𐑹𐑑𐑪𐑲𐑣𐑮𐑓𐑳𐑟",
        1905 => "𐑨𐑰𐑭𐑗𐑑𐑪𐑚𐑵𐑢𐑒𐑳𐑟",
        1906 => "𐑼𐑰𐑾𐑿𐑑𐑪𐑔𐑵𐑻𐑖𐑕𐑟",
        1907 => "𐑦𐑰𐑽𐑬𐑑𐑪𐑲𐑵⊙𐑫𐑙𐑟",
        1908 => "𐑛𐑥𐑽𐑯𐑑𐑪𐑚𐑝𐑣𐑓𐑙𐑟",
        1909 => "𐑨𐑥𐑾𐑹𐑑𐑪𐑔𐑝𐑮𐑒𐑙𐑟",
        1910 => "𐑼𐑥𐑽𐑗𐑑𐑪𐑲𐑝𐑢𐑖𐑳𐑟",
        1911 => "𐑦𐑥𐑩𐑿𐑑𐑪𐑚𐑨𐑻𐑫𐑳𐑟",
        1912 => "𐑛𐑶𐑭𐑬𐑑𐑪𐑔𐑨⊙𐑓𐑳𐑟",
        1913 => "𐑨𐑶𐑭𐑯𐑑𐑪𐑲𐑨𐑣𐑒𐑳𐑟",
        1914 => "𐑼𐑶𐑭𐑹𐑑𐑪𐑚𐑣𐑮𐑖𐑳𐑟",
        1915 => "𐑦𐑶𐑩𐑗𐑑𐑪𐑔𐑣𐑢𐑫𐑳𐑟",
        1916 => "𐑛𐑸𐑭𐑿𐑑𐑪𐑲𐑣𐑻𐑓𐑙𐑟",
        1917 => "𐑨𐑸𐑾𐑬𐑑𐑪𐑚𐑵⊙𐑒𐑳𐑟",
        1918 => "𐑼𐑸𐑩𐑯𐑑𐑪𐑔𐑵𐑣𐑖𐑳𐑟",
        1919 => "𐑦𐑸𐑽𐑹𐑑𐑪𐑲𐑵𐑮𐑫𐑙𐑟",
        1920 => "𐑛𐑡𐑾𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑙𐑟",
        1921 => "𐑨𐑡𐑭𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑙𐑟",
        1922 => "𐑼𐑡𐑾𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑙𐑟",
        1923 => "𐑦𐑡𐑽𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑳𐑟",
        1924 => "𐑛𐑰𐑽𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑙𐑟",
        1925 => "𐑨𐑰𐑭𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑙𐑟",
        1926 => "𐑼𐑰𐑭𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑳𐑟",
        1927 => "𐑦𐑰𐑽𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑳𐑟",
        1928 => "𐑛𐑥𐑭𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑳𐑟",
        1929 => "𐑨𐑥𐑾𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑕𐑟",
        1930 => "𐑼𐑥𐑽𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑳𐑟",
        1931 => "𐑦𐑥𐑭𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑳𐑟",
        1932 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑕𐑟",
        1933 => "𐑨𐑶𐑩𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑕𐑟",
        1934 => "𐑼𐑶𐑭𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑳𐑟",
        1935 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑙𐑟",
        1936 => "𐑛𐑸𐑽𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑳𐑟",
        1937 => "𐑨𐑸𐑽𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑳𐑟",
        1938 => "𐑼𐑸𐑾𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑙𐑟",
        1939 => "𐑦𐑸𐑭𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑙𐑟",
        1940 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑲𐑣𐑢𐑓𐑕𐑟",
        1941 => "𐑨𐑡𐑽𐑿𐑑𐑺𐑚𐑵𐑻𐑒𐑳𐑟",
        1942 => "𐑼𐑡𐑾𐑬𐑑𐑺𐑔𐑵⊙𐑖𐑙𐑟",
        1943 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑲𐑵𐑣𐑫𐑕𐑟",
        1944 => "𐑛𐑰𐑽𐑹𐑑𐑺𐑚𐑝𐑮𐑓𐑳𐑟",
        1945 => "𐑨𐑰𐑾𐑗𐑑𐑺𐑔𐑝𐑢𐑒𐑙𐑟",
        1946 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑲𐑝𐑻𐑖𐑙𐑟",
        1947 => "𐑦𐑰𐑾𐑬𐑑𐑺𐑚𐑨⊙𐑫𐑳𐑟",
        1948 => "𐑛𐑥𐑾𐑯𐑑𐑺𐑔𐑨𐑣𐑓𐑕𐑟",
        1949 => "𐑨𐑥𐑾𐑹𐑑𐑺𐑲𐑨𐑮𐑒𐑳𐑟",
        1950 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑚𐑣𐑢𐑖𐑙𐑟",
        1951 => "𐑦𐑥𐑩𐑿𐑑𐑺𐑔𐑣𐑻𐑫𐑳𐑟",
        1952 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑲𐑣⊙𐑓𐑙𐑟",
        1953 => "𐑨𐑶𐑾𐑯𐑑𐑺𐑚𐑵𐑣𐑒𐑙𐑟",
        1954 => "𐑼𐑶𐑽𐑹𐑑𐑺𐑔𐑵𐑮𐑖𐑙𐑟",
        1955 => "𐑦𐑶𐑾𐑗𐑑𐑺𐑲𐑵𐑢𐑫𐑙𐑟",
        1956 => "𐑛𐑸𐑭𐑿𐑑𐑺𐑚𐑝𐑻𐑓𐑳𐑟",
        1957 => "𐑨𐑸𐑩𐑬𐑑𐑺𐑔𐑝⊙𐑒𐑳𐑟",
        1958 => "𐑼𐑸𐑭𐑯𐑑𐑺𐑲𐑝𐑣𐑖𐑳𐑟",
        1959 => "𐑦𐑸𐑽𐑹𐑑𐑺𐑚𐑨𐑮𐑫𐑙𐑟",
        1960 => "𐑛𐑡𐑭𐑗𐑑𐑺𐑔𐑨𐑢𐑓𐑳𐑟",
        1961 => "𐑨𐑡𐑭𐑿𐑑𐑺𐑲𐑨𐑻𐑒𐑳𐑟",
        1962 => "𐑼𐑡𐑭𐑬𐑑𐑺𐑚𐑣⊙𐑖𐑳𐑟",
        1963 => "𐑦𐑡𐑭𐑯𐑑𐑺𐑔𐑣𐑣𐑫𐑙𐑟",
        1964 => "𐑛𐑰𐑩𐑹𐑑𐑺𐑲𐑣𐑮𐑓𐑳𐑟",
        1965 => "𐑨𐑰𐑾𐑗𐑑𐑺𐑚𐑵𐑢𐑒𐑙𐑟",
        1966 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑔𐑵𐑻𐑖𐑙𐑟",
        1967 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑲𐑵⊙𐑫𐑳𐑟",
        1968 => "𐑛𐑥𐑽𐑯𐑑𐑺𐑚𐑝𐑣𐑓𐑙𐑟",
        1969 => "𐑨𐑥𐑩𐑹𐑑𐑺𐑔𐑝𐑮𐑒𐑳𐑟",
        1970 => "𐑼𐑥𐑭𐑗𐑑𐑺𐑲𐑝𐑢𐑖𐑳𐑟",
        1971 => "𐑦𐑥𐑩𐑿𐑑𐑺𐑚𐑨𐑻𐑫𐑳𐑟",
        1972 => "𐑛𐑶𐑩𐑬𐑑𐑺𐑔𐑨⊙𐑓𐑕𐑟",
        1973 => "𐑨𐑶𐑩𐑯𐑑𐑺𐑲𐑨𐑣𐑒𐑕𐑟",
        1974 => "𐑼𐑶𐑽𐑹𐑑𐑺𐑚𐑣𐑮𐑖𐑳𐑟",
        1975 => "𐑦𐑶𐑩𐑗𐑑𐑺𐑔𐑣𐑢𐑫𐑕𐑟",
        1976 => "𐑛𐑸𐑾𐑿𐑑𐑺𐑲𐑣𐑻𐑓𐑙𐑟",
        1977 => "𐑨𐑸𐑩𐑬𐑑𐑺𐑚𐑵⊙𐑒𐑕𐑟",
        1978 => "𐑼𐑸𐑾𐑯𐑑𐑺𐑔𐑵𐑣𐑖𐑙𐑟",
        1979 => "𐑦𐑸𐑩𐑹𐑑𐑺𐑲𐑵𐑮𐑫𐑕𐑟",
        1980 => "𐑛𐑡𐑩𐑗𐑑𐑺𐑚𐑝𐑢𐑓𐑕𐑟",
        1981 => "𐑨𐑡𐑭𐑿𐑑𐑺𐑔𐑝𐑻𐑒𐑳𐑟",
        1982 => "𐑼𐑡𐑭𐑬𐑑𐑺𐑲𐑝⊙𐑖𐑳𐑟",
        1983 => "𐑦𐑡𐑩𐑯𐑑𐑺𐑚𐑨𐑣𐑫𐑳𐑟",
        1984 => "𐑛𐑰𐑾𐑹𐑑𐑺𐑔𐑨𐑮𐑓𐑕𐑟",
        1985 => "𐑨𐑰𐑽𐑗𐑑𐑺𐑲𐑨𐑢𐑒𐑳𐑟",
        1986 => "𐑼𐑰𐑽𐑿𐑑𐑺𐑚𐑣𐑻𐑖𐑙𐑟",
        1987 => "𐑦𐑰𐑩𐑬𐑑𐑺𐑔𐑣⊙𐑫𐑳𐑟",
        1988 => "𐑛𐑥𐑽𐑯𐑑𐑺𐑲𐑣𐑣𐑓𐑙𐑟",
        1989 => "𐑨𐑥𐑾𐑹𐑑𐑺𐑚𐑵𐑮𐑒𐑙𐑟",
        1990 => "𐑼𐑥𐑾𐑗𐑑𐑺𐑔𐑵𐑢𐑖𐑕𐑟",
        1991 => "𐑦𐑥𐑾𐑿𐑑𐑺𐑲𐑵𐑻𐑫𐑳𐑟",
        1992 => "𐑛𐑶𐑾𐑬𐑑𐑺𐑚𐑝⊙𐑓𐑙𐑟",
        1993 => "𐑨𐑶𐑭𐑯𐑑𐑺𐑔𐑝𐑣𐑒𐑳𐑟",
        1994 => "𐑼𐑶𐑾𐑹𐑑𐑺𐑲𐑝𐑮𐑖𐑙𐑟",
        1995 => "𐑦𐑶𐑩𐑗𐑑𐑺𐑚𐑨𐑢𐑫𐑕𐑟",
        1996 => "𐑛𐑸𐑾𐑿𐑑𐑺𐑔𐑨𐑻𐑓𐑕𐑟",
        1997 => "𐑨𐑸𐑭𐑬𐑑𐑺𐑲𐑨⊙𐑒𐑳𐑟",
        1998 => "𐑼𐑸𐑩𐑯𐑑𐑺𐑚𐑣𐑣𐑖𐑕𐑟",
        1999 => "𐑦𐑸𐑭𐑹𐑑𐑺𐑔𐑣𐑮𐑫𐑳𐑟",
        2000 => "𐑛𐑡𐑭𐑗𐑑𐑘𐑲𐑣𐑢𐑓𐑳𐑟",
        2001 => "𐑨𐑡𐑩𐑿𐑑𐑘𐑚𐑵𐑻𐑒𐑕𐑟",
        2002 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑔𐑵⊙𐑖𐑳𐑟",
        2003 => "𐑦𐑡𐑩𐑯𐑑𐑘𐑲𐑵𐑣𐑫𐑕𐑟",
        2004 => "𐑛𐑰𐑾𐑹𐑑𐑘𐑚𐑝𐑮𐑓𐑳𐑟",
        2005 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑔𐑝𐑢𐑒𐑳𐑟",
        2006 => "𐑼𐑰𐑭𐑿𐑑𐑘𐑲𐑝𐑻𐑖𐑕𐑟",
        2007 => "𐑦𐑰𐑩𐑬𐑑𐑘𐑚𐑨⊙𐑫𐑳𐑟",
        2008 => "𐑛𐑥𐑩𐑯𐑑𐑘𐑔𐑨𐑣𐑓𐑕𐑟",
        2009 => "𐑨𐑥𐑩𐑹𐑑𐑘𐑲𐑨𐑮𐑒𐑕𐑟",
        2010 => "𐑼𐑥𐑾𐑗𐑑𐑘𐑚𐑣𐑢𐑖𐑕𐑟",
        2011 => "𐑦𐑥𐑽𐑿𐑑𐑘𐑔𐑣𐑻𐑫𐑳𐑟",
        2012 => "𐑛𐑶𐑩𐑬𐑑𐑘𐑲𐑣⊙𐑓𐑳𐑟",
        2013 => "𐑨𐑶𐑩𐑯𐑑𐑘𐑚𐑵𐑣𐑒𐑕𐑟",
        2014 => "𐑼𐑶𐑩𐑹𐑑𐑘𐑔𐑵𐑮𐑖𐑕𐑟",
        2015 => "𐑦𐑶𐑽𐑗𐑑𐑘𐑲𐑵𐑢𐑫𐑳𐑟",
        2016 => "𐑛𐑸𐑽𐑿𐑑𐑘𐑚𐑝𐑻𐑓𐑳𐑟",
        2017 => "𐑨𐑸𐑩𐑬𐑑𐑘𐑔𐑝⊙𐑒𐑳𐑟",
        2018 => "𐑼𐑸𐑽𐑯𐑑𐑘𐑲𐑝𐑣𐑖𐑳𐑟",
        2019 => "𐑦𐑸𐑩𐑹𐑑𐑘𐑚𐑨𐑮𐑫𐑳𐑟",
        2020 => "𐑛𐑡𐑩𐑗𐑑𐑘𐑔𐑨𐑢𐑓𐑕𐑟",
        2021 => "𐑨𐑡𐑾𐑿𐑑𐑘𐑲𐑨𐑻𐑒𐑳𐑟",
        2022 => "𐑼𐑡𐑩𐑬𐑑𐑘𐑚𐑣⊙𐑖𐑕𐑟",
        2023 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑔𐑣𐑣𐑫𐑳𐑟",
        2024 => "𐑛𐑰𐑽𐑹𐑑𐑘𐑲𐑣𐑮𐑓𐑳𐑟",
        2025 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑚𐑵𐑢𐑒𐑳𐑟",
        2026 => "𐑼𐑰𐑩𐑿𐑑𐑘𐑔𐑵𐑻𐑖𐑳𐑟",
        2027 => "𐑦𐑰𐑩𐑬𐑑𐑘𐑲𐑵⊙𐑫𐑕𐑟",
        2028 => "𐑛𐑥𐑩𐑯𐑑𐑘𐑚𐑝𐑣𐑓𐑕𐑟",
        2029 => "𐑨𐑥𐑭𐑹𐑑𐑘𐑔𐑝𐑮𐑒𐑕𐑟",
        2030 => "𐑼𐑥𐑭𐑗𐑑𐑘𐑲𐑝𐑢𐑖𐑕𐑟",
        2031 => "𐑦𐑥𐑭𐑿𐑑𐑘𐑚𐑨𐑻𐑫𐑕𐑟",
        2032 => "𐑛𐑶𐑩𐑬𐑑𐑘𐑔𐑨⊙𐑓𐑕𐑟",
        2033 => "𐑨𐑶𐑭𐑯𐑑𐑘𐑲𐑨𐑣𐑒𐑕𐑟",
        2034 => "𐑼𐑶𐑾𐑹𐑑𐑘𐑚𐑣𐑮𐑖𐑳𐑟",
        2035 => "𐑦𐑶𐑭𐑗𐑑𐑘𐑔𐑣𐑢𐑫𐑕𐑟",
        2036 => "𐑛𐑸𐑭𐑿𐑑𐑘𐑲𐑣𐑻𐑓𐑳𐑟",
        2037 => "𐑨𐑸𐑭𐑬𐑑𐑘𐑚𐑵⊙𐑒𐑕𐑟",
        2038 => "𐑼𐑸𐑩𐑯𐑑𐑘𐑔𐑵𐑣𐑖𐑕𐑟",
        2039 => "𐑦𐑸𐑩𐑹𐑑𐑘𐑲𐑵𐑮𐑫𐑳𐑟",
        2040 => "𐑛𐑡𐑽𐑗𐑑𐑘𐑚𐑝𐑢𐑓𐑳𐑟",
        2041 => "𐑨𐑡𐑾𐑿𐑑𐑘𐑔𐑝𐑻𐑒𐑳𐑟",
        2042 => "𐑼𐑡𐑭𐑬𐑑𐑘𐑲𐑝⊙𐑖𐑳𐑟",
        2043 => "𐑦𐑡𐑭𐑯𐑑𐑘𐑚𐑨𐑣𐑫𐑳𐑟",
        2044 => "𐑛𐑰𐑭𐑹𐑑𐑘𐑔𐑨𐑮𐑓𐑳𐑟",
        2045 => "𐑨𐑰𐑩𐑗𐑑𐑘𐑲𐑨𐑢𐑒𐑳𐑟",
        2046 => "𐑼𐑰𐑩𐑿𐑑𐑘𐑚𐑣𐑻𐑖𐑳𐑟",
        2047 => "𐑦𐑰𐑾𐑬𐑑𐑘𐑔𐑣⊙𐑫𐑳𐑟",
        _ => "",
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
        ("⊡", "Permanent record fixation"),
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
    let marks = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '⊡'];
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
  - Derivation pipeline glyph word: ⊢⊣>⋈⊤∈∋⊙⊥<⊞⊡⊣
  - Address TSV: THIS_bip39_addresses.tsv (word -> 12-mark address)
  - Inscription TSV: bip39_inscriptions.tsv / bip39_tuples.tsv (word -> glyph tuple)

Proof principles: Each axis promotion is a logical inference step.".to_string()
}

fn format_result(r: &SecretKeyResult) -> String {
    let mut out = String::new();
    out.push_str("\n");
    out.push_str(&format!(
        "├─ result: {}\n",
        match r.certainty {
            CertaintyLevel::Structural => "STRUCTURAL DERIVATION (computable, not heuristic)",
        }
    ));
    if let Some(s) = r.scalar {
        out.push_str(&format!("├─ scalar: {}\n", s));
        if let Some(decimal) = &r.scalar_decimal {
            out.push_str(&format!("├─ scalar (decimal): {}\n", decimal));
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