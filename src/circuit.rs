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
// Four bases give sixteen ordered pairs at the first two codon positions.
// Four of those are diagonal (p1 = p2) and twelve are not. Twelve is the size
// of the alphabet, so the off-diagonal pairs ARE the glyphs, and the assignment
// is enumeration rather than choice. The third position is the wobble: it
// carries no glyph, which is the same statement the codon table already makes
// about the exact stratum.

/// B4 in discriminant order. Used only to enumerate; the order is the enum's.
const B4_ORDER: [B4; 4] = [B4::N, B4::T, B4::F, B4::B];

/// The twelve off-diagonal base pairs, in enumeration order.
pub fn offdiagonal_pairs() -> Vec<(B4, B4)> {
    let mut out = Vec::new();
    for p1 in B4_ORDER {
        for p2 in B4_ORDER {
            if p1 != p2 {
                out.push((p1, p2));
            }
        }
    }
    out
}

/// δ_RNA: the canonical codon for a glyph. Wobble is set to the unmarked base.
pub fn glyph_to_codon(g: Glyph) -> Codon {
    let pairs = offdiagonal_pairs();
    let idx = Glyph::all().iter().position(|x| *x == g).unwrap_or(0);
    let (p1, p2) = pairs[idx];
    Codon { p1, p2, p3: B4::N }
}

/// μ_RNA: the glyph a codon carries. Diagonal codons carry none — they are the
/// part of codon space the alphabet does not reach.
pub fn codon_to_glyph(c: &Codon) -> Option<Glyph> {
    if c.p1 == c.p2 {
        return None;
    }
    let pairs = offdiagonal_pairs();
    let idx = pairs.iter().position(|(a, b)| *a == c.p1 && *b == c.p2)?;
    Glyph::all().get(idx).copied()
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
        '◻' => Some(("add", "qword ptr [rax], rbx")),
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
        '◻' => "i32.store",
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
        o if o.ends_with(".store") => '◻',
        o if o.starts_with("local.") || o.starts_with("global.") => '⋈',
        o if o.ends_with(".eq") || o.ends_with(".ne") || o.ends_with(".lt_s") => '⊤',
        "select" => '⊥',
        _ => '⊞',
    };
    Glyph::from_char(c)
}

// ── The amino acid leg ─────────────────────────────────────────

/// The amino acid a glyph's canonical codon translates to.
pub fn glyph_to_aa(g: Glyph) -> AminoAcid {
    translate_codon(&glyph_to_codon(g))
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
    for g in Glyph::all() {
        match codon_to_glyph(&glyph_to_codon(g)) {
            Some(h) if h == g => closed.push(g),
            _ => broken.push(g),
        }
    }
    Retraction { leg: "RNA", closed, broken, unexpressed: Vec::new() }
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
        let c = glyph_to_codon(g);
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
        let aa = glyph_to_aa(g3);
        routed.push(aa);
        let canonical = glyph_to_codon(g);
        let on_section = canonical == *c;
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
                format!("   off-section: {} is the canonical codon", codon_rna(&canonical))
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
    let mut out = vec!["glyph  codon  aa   x86                      wasm".to_string()];
    for g in Glyph::all() {
        let c = glyph_to_codon(g);
        let x = match glyph_to_x86(g) {
            Some((mn, "")) => mn.to_string(),
            Some((mn, ops)) => format!("{} {}", mn, ops),
            None => "—  (structural)".to_string(),
        };
        out.push(format!(
            "  {}    {}    {}  {:<24} {}",
            g.to_char(),
            codon_rna(&c),
            glyph_to_aa(g).code3(),
            x,
            glyph_to_wasm(g)
        ));
    }
    out
}
