// ─── Circuit — substrate round trips through IMASM ─────────────
//
// Two circuits, both routed through the twelve-glyph alphabet:
//
//   x86 → IMASM → RNA → IMASM → x86
//   RNA → IMASM → x86 → IMASM → wasm → IMASM → AA
//
// The claim under test is not that a binary returns byte-identical. It cannot:
// every substrate leg is many-to-one, so it has a fiber and no inverse. What
// closes is the IMASM word. Each leg is a retraction, μ∘δ = id on glyphs, and
// the outer composite δ∘μ is idempotent rather than identity. The second
// circuit ends at amino acids, so it is a path rather than a loop, and its
// claim is that routing RNA to protein THROUGH two machine substrates returns
// what direct translation returns. The detour is invisible.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

use crate::belnap::B4;
use crate::belnap_ring_shor::Glyph;
use crate::rebis::codon::{Codon, translate_codon};
use crate::rebis::AminoAcid;
use crate::vox::{classify_instruction, Instruction};

// ── The RNA leg ────────────────────────────────────────────────
//
// Nothing here is assigned. A codon translates to an amino acid through the
// live table; an amino acid carries a primitive iff every one of its codons
// sits in the split stratum (`AminoAcid::to_primitive`); and a primitive value
// belongs to exactly one axis (`canonical_ig::axis_of`). The axis IS the glyph.
// So the map is: codon → amino acid → primitive → axis → glyph, each step read
// off something that already existed.
//
// This is the same twelve-to-twelve correspondence `GeneticCode.lean` proves as
// `aaToPrimitive` / `primitiveToAA`, with `promoted_card = 12` and
// `twenty_eq_eight_plus_twelve` behind it. The eight ground-layer amino acids
// carry no primitive and no glyph.

/// B4 in discriminant order. Used only to enumerate; the order is the enum's.
const B4_ORDER: [B4; 4] = [B4::N, B4::T, B4::F, B4::B];

/// The twelve-to-twelve correspondence, in the canonical axis order of
/// `canonical_ig::PRIMITIVE_ORDER`. This mirrors `GeneticCode.lean`'s
/// `primitiveToAA`, which is the statement of record; the axis order is
/// `⊢⊣><⋈⊤∈∋⊙⊥⊞⊡` and the amino acids are its promoted layer.
pub const PROMOTED_BY_AXIS: [(char, AminoAcid); 12] = [
    ('⊢', AminoAcid::Met),  // Dimensionality
    ('⊣', AminoAcid::Trp),  // Topology
    ('>', AminoAcid::Cys),  // Relational
    ('<', AminoAcid::Tyr),  // Polarity
    ('⋈', AminoAcid::Phe),  // Fidelity
    ('⊤', AminoAcid::Ile),  // Kinetics
    ('∈', AminoAcid::His),  // Scope
    ('∋', AminoAcid::Asn),  // Composition
    ('⊙', AminoAcid::Gln),  // Criticality
    ('⊥', AminoAcid::Asp),  // Chirality
    ('⊞', AminoAcid::Lys),  // Stoichiometry
    ('⊡', AminoAcid::Glu),  // Winding
];

/// μ_RNA, first half: the glyph an amino acid carries, if any.
pub fn aa_to_glyph(aa: AminoAcid) -> Option<Glyph> {
    PROMOTED_BY_AXIS
        .iter()
        .find(|(_, a)| *a == aa)
        .and_then(|(c, _)| Glyph::from_char(*c))
}

/// Where `AminoAcid::to_primitive` disagrees with the correspondence above.
///
/// `to_primitive` derives a primitive VALUE from the codon box and then the
/// value's axis is supposed to be the glyph. Several of the values it returns
/// sit on the wrong axis, so the derivation and the canonical correspondence do
/// not agree. Reported rather than papered over.
pub fn primitive_drift() -> Vec<String> {
    let mut out = Vec::new();
    for (axis, aa) in PROMOTED_BY_AXIS {
        let derived = aa
            .to_primitive()
            .and_then(|p| crate::canonical_ig::axis_of(p.glyph()))
            .and_then(|s| s.chars().next());
        match derived {
            Some(d) if d == axis => {}
            Some(d) => out.push(format!(
                "  {}  canonical {}   to_primitive lands on {}",
                aa.code3(),
                axis,
                d
            )),
            None => out.push(format!(
                "  {}  canonical {}   to_primitive lands on no axis",
                aa.code3(),
                axis
            )),
        }
    }
    out
}

/// μ_RNA: the glyph a codon carries. Ground-layer codons and stops carry none.
pub fn codon_to_glyph(c: &Codon) -> Option<Glyph> {
    aa_to_glyph(translate_codon(c))
}

/// Every codon that carries a glyph. The section's representative is the first
/// of these, and `section_choice_is_free` proves the choice does not matter.
pub fn codons_for_glyph(g: Glyph) -> Vec<Codon> {
    all_codons().into_iter().filter(|c| codon_to_glyph(c) == Some(g)).collect()
}

/// δ_RNA: a codon carrying the glyph. Any of them serves — μ routes through the
/// amino acid, so every representative returns the same glyph.
pub fn glyph_to_codon(g: Glyph) -> Option<Codon> {
    codons_for_glyph(g).into_iter().next()
}

/// **The section choice is free.** μ∘δ = id holds for EVERY codon carrying the
/// glyph, not merely for the representative δ happens to pick, so the section is
/// determined up to a choice that changes nothing.
pub fn section_choice_is_free() -> bool {
    Glyph::all().iter().all(|&g| {
        let cs = codons_for_glyph(g);
        !cs.is_empty() && cs.iter().all(|c| codon_to_glyph(c) == Some(g))
    })
}

/// What a codon carries, with no residue left over: a glyph, or a ground-layer
/// amino acid that carries no primitive, or a stop.
pub enum CodonRole {
    Primitive(Glyph, AminoAcid),
    Scaffold(AminoAcid),
    Stop,
}

/// Total classification of codon space. Every codon lands in exactly one arm.
pub fn codon_role(c: &Codon) -> CodonRole {
    let aa = translate_codon(c);
    if aa == AminoAcid::Stop {
        return CodonRole::Stop;
    }
    match aa_to_glyph(aa) {
        Some(g) => CodonRole::Primitive(g, aa),
        None => CodonRole::Scaffold(aa),
    }
}

/// The census of codon space: primitive-bearing, scaffold, stop. These sum to 64.
pub fn codon_census() -> (usize, usize, usize) {
    let mut prim = 0;
    let mut scaf = 0;
    let mut stop = 0;
    for c in all_codons() {
        match codon_role(&c) {
            CodonRole::Primitive(..) => prim += 1,
            CodonRole::Scaffold(_) => scaf += 1,
            CodonRole::Stop => stop += 1,
        }
    }
    (prim, scaf, stop)
}

/// The amino acid a glyph's canonical codon translates to.
pub fn glyph_to_aa(g: Glyph) -> Option<AminoAcid> {
    glyph_to_codon(g).map(|c| translate_codon(&c))
}

/// Render a codon as RNA letters.
pub fn codon_rna(c: &Codon) -> String {
    let n = |b: B4| -> char {
        match b {
            B4::B => 'G',
            B4::T => 'C',
            B4::F => 'A',
            B4::N => 'U',
        }
    };
    let mut s = String::new();
    s.push(n(c.p1));
    s.push(n(c.p2));
    s.push(n(c.p3));
    s
}

// ── The x86 leg ────────────────────────────────────────────────
//
// Ten of the twelve glyphs have an instruction that lifts back to them under
// vox's classifier. The two that do not are ⊢ and ∋: a word opener and a merge
// point. Neither is an instruction. x86 has flat control flow, so both are
// recovered by analysis of the instruction stream rather than read off any
// single instruction, which is exactly what the lifter does.

/// δ_x86: a representative instruction for a glyph, where one exists.
pub fn glyph_to_x86(g: Glyph) -> Option<(&'static str, &'static str)> {
    match g.to_char() {
        '⊣' => Some(("ret", "")),
        '>' => Some(("call", "0x401000")),
        '<' => Some(("jmp", "0x401000")),
        '∈' => Some(("jne", "0x401000")),
        '⊙' => Some(("syscall", "")),
        '⊡' => Some(("add", "qword ptr [rax], rbx")),
        '⋈' => Some(("mov", "rax, rbx")),
        '⊤' => Some(("cmp", "rax, rbx")),
        '⊥' => Some(("sete", "al")),
        '⊞' => Some(("xor", "rax, rax")),
        // ⊢ opens the word and ∋ marks a merge. Neither is an instruction.
        _ => None,
    }
}

/// μ_x86: vox's classifier, reached through a synthetic instruction.
pub fn x86_to_glyph(mnemonic: &str, op_str: &str) -> Option<Glyph> {
    let ins = Instruction {
        address: 0,
        mnemonic: mnemonic.to_string(),
        op_str: op_str.to_string(),
    };
    Glyph::from_char(classify_instruction(&ins))
}

// ── The wasm leg ───────────────────────────────────────────────
//
// wasm carries structured control flow, so `block` and `end` are real opcodes.
// All twelve glyphs have a representative here, which is the substantive
// difference between the two machine substrates.

/// δ_wasm: a representative opcode for a glyph.
pub fn glyph_to_wasm(g: Glyph) -> &'static str {
    match g.to_char() {
        '⊢' => "block",
        '⊣' => "return",
        '>' => "call",
        '<' => "br",
        '∈' => "if",
        '∋' => "end",
        '⊙' => "call_indirect",
        '⊡' => "i32.store",
        '⋈' => "local.get",
        '⊤' => "i32.eq",
        '⊥' => "select",
        _ => "i32.add",
    }
}

/// μ_wasm: the glyph an opcode carries.
pub fn wasm_to_glyph(op: &str) -> Option<Glyph> {
    let c = match op {
        "block" | "loop" => '⊢',
        "return" => '⊣',
        "call" => '>',
        "br" | "br_if" | "br_table" => '<',
        "if" => '∈',
        "end" | "else" => '∋',
        "call_indirect" => '⊙',
        o if o.ends_with(".store") => '⊡',
        o if o.starts_with("local.") || o.starts_with("global.") => '⋈',
        o if o.ends_with(".eq") || o.ends_with(".ne") || o.ends_with(".lt_s") => '⊤',
        "select" => '⊥',
        _ => '⊞',
    };
    Glyph::from_char(c)
}

// ── Retractions ────────────────────────────────────────────────

/// A leg's retraction report: which glyphs survive μ∘δ, and which the substrate
/// cannot express at all.
pub struct Retraction {
    pub leg: &'static str,
    pub closed: Vec<Glyph>,
    pub broken: Vec<Glyph>,
    pub unexpressed: Vec<Glyph>,
}

impl Retraction {
    pub fn holds(&self) -> bool {
        self.broken.is_empty()
    }
}

/// μ∘δ = id on the RNA leg.
pub fn retraction_rna() -> Retraction {
    let mut closed = Vec::new();
    let mut broken = Vec::new();
    let mut unexpressed = Vec::new();
    for g in Glyph::all() {
        match glyph_to_codon(g) {
            None => unexpressed.push(g),
            Some(c) => match codon_to_glyph(&c) {
                Some(h) if h == g => closed.push(g),
                _ => broken.push(g),
            },
        }
    }
    Retraction { leg: "RNA", closed, broken, unexpressed }
}

/// μ∘δ = id on the x86 leg, over the glyphs x86 can express.
pub fn retraction_x86() -> Retraction {
    let mut closed = Vec::new();
    let mut broken = Vec::new();
    let mut unexpressed = Vec::new();
    for g in Glyph::all() {
        match glyph_to_x86(g) {
            None => unexpressed.push(g),
            Some((mn, ops)) => match x86_to_glyph(mn, ops) {
                Some(h) if h == g => closed.push(g),
                _ => broken.push(g),
            },
        }
    }
    Retraction { leg: "x86", closed, broken, unexpressed }
}

/// μ∘δ = id on the wasm leg.
pub fn retraction_wasm() -> Retraction {
    let mut closed = Vec::new();
    let mut broken = Vec::new();
    for g in Glyph::all() {
        match wasm_to_glyph(glyph_to_wasm(g)) {
            Some(h) if h == g => closed.push(g),
            _ => broken.push(g),
        }
    }
    Retraction { leg: "wasm", closed, broken, unexpressed: Vec::new() }
}

// ── Circuit one: x86 → IMASM → RNA → IMASM → x86 ───────────────

pub struct CircuitOne {
    pub start: Vec<Glyph>,
    pub rna: String,
    pub returned: Vec<Glyph>,
    pub instructions: Vec<String>,
}

impl CircuitOne {
    pub fn closes(&self) -> bool {
        self.start == self.returned
    }
}

/// Run the first circuit over a glyph word. The x86 legs are carried as the
/// instruction list the word emits and the word that list lifts back to.
pub fn circuit_one(word: &[Glyph]) -> CircuitOne {
    let mut rna = String::new();
    let mut returned = Vec::new();
    let mut instructions = Vec::new();

    for &g in word {
        // IMASM → RNA
        let c = match glyph_to_codon(g) {
            Some(c) => c,
            None => continue,
        };
        rna.push_str(&codon_rna(&c));
        // RNA → IMASM
        let back = codon_to_glyph(&c);
        if let Some(h) = back {
            returned.push(h);
            // IMASM → x86
            match glyph_to_x86(h) {
                Some((mn, "")) => instructions.push(mn.to_string()),
                Some((mn, ops)) => instructions.push(format!("{} {}", mn, ops)),
                None => instructions.push(format!("; {} is structural", h.to_char())),
            }
        }
    }

    CircuitOne { start: word.to_vec(), rna, returned, instructions }
}

// ── Circuit two: RNA → IMASM → x86 → IMASM → wasm → IMASM → AA ──

pub struct CircuitTwo {
    pub codons: Vec<Codon>,
    pub direct: Vec<AminoAcid>,
    pub routed: Vec<AminoAcid>,
    pub trace: Vec<String>,
    pub skipped: usize,
    /// Codons that entered the circuit but are not the canonical representative
    /// of their glyph. δ∘μ moves these, so the protein they return is not the
    /// protein they started as. This is the fiber, counted.
    pub offsection: usize,
}

impl CircuitTwo {
    pub fn closes(&self) -> bool {
        self.direct == self.routed
    }
}

/// Parse an RNA string into codons, dropping a ragged tail.
pub fn parse_rna(s: &str) -> Vec<Codon> {
    let b: Vec<u8> = s.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 3 <= b.len() {
        if let Ok(c) = Codon::from_bytes(b[i], b[i + 1], b[i + 2]) {
            out.push(c);
        }
        i += 3;
    }
    out
}

/// Run the second circuit. Direct translation is the control; the routed chain
/// is the same codons carried through x86 and wasm and back each time.
pub fn circuit_two(rna: &str) -> CircuitTwo {
    let codons = parse_rna(rna);
    let mut direct = Vec::new();
    let mut routed = Vec::new();
    let mut trace = Vec::new();
    let mut skipped = 0usize;
    let mut offsection = 0usize;

    for c in &codons {
        direct.push(translate_codon(c));

        // RNA → IMASM
        let g = match codon_to_glyph(c) {
            Some(g) => g,
            None => {
                // A diagonal codon carries no glyph. It cannot enter the
                // circuit, so the routed chain has nothing to say about it.
                skipped += 1;
                trace.push(format!("{}  diagonal, carries no glyph", codon_rna(c)));
                continue;
            }
        };

        // IMASM → x86 → IMASM
        let after_x86 = match glyph_to_x86(g) {
            Some((mn, ops)) => x86_to_glyph(mn, ops),
            None => Some(g), // structural: the lifter re-derives it, it does not travel
        };
        let g2 = match after_x86 {
            Some(h) => h,
            None => {
                trace.push(format!("{}  lost on the x86 leg", codon_rna(c)));
                continue;
            }
        };

        // IMASM → wasm → IMASM
        let g3 = match wasm_to_glyph(glyph_to_wasm(g2)) {
            Some(h) => h,
            None => {
                trace.push(format!("{}  lost on the wasm leg", codon_rna(c)));
                continue;
            }
        };

        // IMASM → AA
        let aa = match glyph_to_aa(g3) {
            Some(a) => a,
            None => continue,
        };
        routed.push(aa);
        let canonical = glyph_to_codon(g);
        let on_section = canonical.as_ref() == Some(c);
        if !on_section {
            offsection += 1;
        }
        trace.push(format!(
            "{}  {}  {}  {}  {}{}",
            codon_rna(c),
            g.to_char(),
            glyph_to_x86(g).map(|(m, _)| m).unwrap_or("—"),
            glyph_to_wasm(g2),
            aa.code3(),
            if on_section {
                String::new()
            } else {
                match &canonical {
                    Some(k) => format!("   off-section: {} is the canonical codon", codon_rna(k)),
                    None => String::new(),
                }
            }
        ));
    }

    // Direct translation only speaks for codons that entered the circuit.
    let direct: Vec<AminoAcid> = codons
        .iter()
        .filter(|c| codon_to_glyph(c).is_some())
        .map(translate_codon)
        .collect();

    CircuitTwo { codons, direct, routed, trace, skipped, offsection }
}

// ── Strands and frames ─────────────────────────────────────────
//
// The glyph is read off the first two codon positions; the third is wobble and
// carries none. Watson-Crick reverse complement sends (p1,p2,p3) to
// (comp p3, comp p2, comp p1), so the antisense strand reads its FIRST position
// off the sense strand's wobble. Whatever the sense strand discards is what the
// antisense strand puts in a glyph-bearing position.

/// The glyph word a strand carries, with `.` where a codon carries none.
pub fn strand_word(codons: &[Codon]) -> String {
    codons
        .iter()
        .map(|c| match codon_role(c) {
            CodonRole::Primitive(g, _) => g.to_char(),
            CodonRole::Scaffold(_) => '·',
            CodonRole::Stop => '|',
        })
        .collect()
}

/// Reverse complement of a codon list, read 5'→3' on the other strand.
pub fn antisense(codons: &[Codon]) -> Vec<Codon> {
    codons.iter().rev().map(|c| c.reverse_complement()).collect()
}

/// Re-frame a codon list by shifting the underlying base string.
pub fn frame(rna: &str, shift: usize) -> Vec<Codon> {
    let cleaned: String = rna.chars().filter(|c| !c.is_whitespace()).collect();
    if shift >= cleaned.len() {
        return Vec::new();
    }
    parse_rna(&cleaned[shift..])
}

pub struct StrandReport {
    pub sense: String,
    pub antisense: String,
    pub frames: [String; 3],
}

pub fn strand_report(rna: &str) -> StrandReport {
    let c0 = parse_rna(rna);
    StrandReport {
        sense: strand_word(&c0),
        antisense: strand_word(&antisense(&c0)),
        frames: [
            strand_word(&frame(rna, 0)),
            strand_word(&frame(rna, 1)),
            strand_word(&frame(rna, 2)),
        ],
    }
}

// ── Which slot carries the amino acid ──────────────────────────
//
// The sense glyph reads (p1,p2); the antisense glyph reads (comp p3, comp p2).
// The middle position is the only one both strands read, so the question is
// whether the code itself also loads that position most heavily. Measured here
// by substitution: over all codon pairs differing in exactly one position, how
// often does the amino acid change.

pub struct SlotLoad {
    pub position: usize,
    pub substitutions: usize,
    pub changed: usize,
}

impl SlotLoad {
    /// Percent of single-position substitutions at this slot that change the AA.
    pub fn percent(&self) -> usize {
        if self.substitutions == 0 { 0 } else { self.changed * 100 / self.substitutions }
    }
}

/// Every codon, built from the same B4 enumeration the glyphs use.
pub fn all_codons() -> Vec<Codon> {
    let mut out = Vec::new();
    for p1 in B4_ORDER {
        for p2 in B4_ORDER {
            for p3 in B4_ORDER {
                out.push(Codon { p1, p2, p3 });
            }
        }
    }
    out
}

fn with_slot(c: &Codon, slot: usize, b: B4) -> Codon {
    match slot {
        0 => Codon { p1: b, p2: c.p2, p3: c.p3 },
        1 => Codon { p1: c.p1, p2: b, p3: c.p3 },
        _ => Codon { p1: c.p1, p2: c.p2, p3: b },
    }
}

fn slot_of(c: &Codon, slot: usize) -> B4 {
    match slot {
        0 => c.p1,
        1 => c.p2,
        _ => c.p3,
    }
}

/// How heavily each codon position is loaded, read off the live codon table.
pub fn slot_loads() -> Vec<SlotLoad> {
    let mut out = Vec::new();
    for slot in 0..3 {
        let mut substitutions = 0usize;
        let mut changed = 0usize;
        for c in all_codons() {
            let aa = translate_codon(&c);
            for b in B4_ORDER {
                if b == slot_of(&c, slot) {
                    continue;
                }
                let d = with_slot(&c, slot, b);
                substitutions += 1;
                if translate_codon(&d) != aa {
                    changed += 1;
                }
            }
        }
        out.push(SlotLoad { position: slot + 1, substitutions, changed });
    }
    out
}

// ── Reports ────────────────────────────────────────────────────

pub fn retraction_lines() -> Vec<String> {
    let mut out = Vec::new();
    for r in [retraction_rna(), retraction_x86(), retraction_wasm()] {
        let closed: String = r.closed.iter().map(|g: &Glyph| g.to_char()).collect();
        let broken: String = r.broken.iter().map(|g: &Glyph| g.to_char()).collect();
        let unexp: String = r.unexpressed.iter().map(|g: &Glyph| g.to_char()).collect();
        out.push(format!(
            "{:<5} μ∘δ=id on {}{}{}",
            r.leg,
            closed,
            if broken.is_empty() { String::new() } else { format!("   BROKEN {}", broken) },
            if unexp.is_empty() {
                String::new()
            } else {
                format!("   not expressible {}", unexp)
            }
        ));
    }
    out
}

/// The whole alphabet, one row per glyph, across every substrate.
pub fn table_lines() -> Vec<String> {
    let mut out = vec!["glyph  codon  aa   n  x86                      wasm".to_string()];
    for g in Glyph::all() {
        let c = match glyph_to_codon(g) {
            Some(c) => c,
            None => continue,
        };
        let x = match glyph_to_x86(g) {
            Some((mn, "")) => mn.to_string(),
            Some((mn, ops)) => format!("{} {}", mn, ops),
            None => "—  (structural)".to_string(),
        };
        out.push(format!(
            "  {}    {}    {}  {}  {:<24} {}",
            g.to_char(),
            codon_rna(&c),
            glyph_to_aa(g).map(|a| a.code3()).unwrap_or("—"),
            codons_for_glyph(g).len(),
            x,
            glyph_to_wasm(g)
        ));
    }
    out
}
