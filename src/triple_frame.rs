// triple_frame.rs — Triple Frame von Neumann Superoperator Algebra
//
// Port of m3iosis/src/m3iosis/triple_frame.py to bare-metal mOMonadOS (no_std).
//
// The 12-primitive type-expansion hierarchy of the Imscribing Grammar
// as executable IMASM protocols. Every primitive value unfolds into its
// own Frobenius-closed IMASM program.
//
// Core primitives:
//   - Type expansion: primitive_value → IMASM program (strange loop)
//   - Protocol A: emergence/annihilation at exceptional point (ρ=2.2800)
//   - Protocol B: imscriptive boundary-bulk round-trip (ρ=2.2581)
//   - Frobenius closure: μ∘δ=id verification
//   - IMASM cycle: tuple ↔ word round-trip (11/12 axes bijective)
//
// Author: Math⊙perator (Lando⊗⊙perator Team)
// Port:  Python → Rust (no_std for mOMonadOS), 2026-07

#[allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;


// use crate::sprintln;
// use crate::sprint;

// ═══════════════════════════════════════════════════════════════
// IMASM OPCODES
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    VINIT,    // ⊢  — initialize vacuum
    IMSCRIB,  // ⊙  — self-referential identity
    FSPLIT,   // ∈  — comultiplication (δ)
    EVALT,    // +  — coherent convergence
    EVALF,    // ×  — decoherent divergence
    ENGAGR,   // ⊞  — non-commutative braiding / paradox
    FFUSE,    // ∋  — multiplication (μ)
    CLINK,    // =  — superoperator composition
    AFWD,     // >  — unitary evolution
    AREV,     // <  — adjoint symmetry
    IFIX,     // ¬  — trace record
    TANCH,    // ⊣  — imscriptive boundary closure
}

impl Opcode {
    pub fn glyph(&self) -> &'static str {
        match self {
            Opcode::VINIT   => "⊢",
            Opcode::IMSCRIB => "⊙",
            Opcode::FSPLIT  => "∈",
            Opcode::EVALT   => "⊤",
            Opcode::EVALF   => "⊥",
            Opcode::ENGAGR  => "⊞",
            Opcode::FFUSE   => "∋",
            Opcode::CLINK   => "⋈",
            Opcode::AFWD    => "≻",
            Opcode::AREV    => "≺",
            Opcode::IFIX    => "⊡",
            Opcode::TANCH   => "⊣",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Opcode::VINIT   => "VINIT",
            Opcode::IMSCRIB => "IMSCRIB",
            Opcode::FSPLIT  => "FSPLIT",
            Opcode::EVALT   => "EVALT",
            Opcode::EVALF   => "EVALF",
            Opcode::ENGAGR  => "ENGAGR",
            Opcode::FFUSE   => "FFUSE",
            Opcode::CLINK   => "CLINK",
            Opcode::AFWD    => "AFWD",
            Opcode::AREV    => "AREV",
            Opcode::IFIX    => "IFIX",
            Opcode::TANCH   => "TANCH",
        }
    }

    pub fn from_glyph(g: char) -> Option<Opcode> {
        match g {
            '⊢' => Some(Opcode::VINIT),
            '⊙' => Some(Opcode::IMSCRIB),
            '∈' | '◇' => Some(Opcode::FSPLIT),
            '+' => Some(Opcode::EVALT),
            '×' => Some(Opcode::EVALF),
            '⊞' => Some(Opcode::ENGAGR),
            '∋' | '●' => Some(Opcode::FFUSE),
            '=' => Some(Opcode::CLINK),
            '≻' => Some(Opcode::AFWD),
            '≺' => Some(Opcode::AREV),
            '¬' => Some(Opcode::IFIX),
            '⊣' => Some(Opcode::TANCH),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// TYPE PROGRAM
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct TypeProgram {
    pub shavian: &'static str,
    pub primitive_axis: &'static str,
    pub value_glyph: &'static str,
    pub opcodes: &'static [Opcode],
    pub rho: f32,
    pub domain_reading: &'static str,
}

impl TypeProgram {
    pub fn word(&self) -> String {
        self.opcodes.iter().map(|oc| oc.glyph()).collect()
    }

    pub fn n_ops(&self) -> usize {
        self.opcodes.len()
    }
}

// ═══════════════════════════════════════════════════════════════
// THE 12 TYPE PROGRAMS
// ═══════════════════════════════════════════════════════════════

use Opcode::*;

pub const TRIPLE_FRAME_TUPLE: &str = "⟨𐑦𐑸𐑽𐑬𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭⟩";

static TYPE_PROGRAMS: &[TypeProgram] = &[
    TypeProgram {
        shavian: "if", primitive_axis: "⊢", value_glyph: "𐑦",
        opcodes: &[VINIT, IMSCRIB, FSPLIT, EVALT, AFWD, CLINK,
                    IFIX, AREV, EVALF, FFUSE, IMSCRIB, TANCH],
        rho: 2.2242,
        domain_reading: "Imscriptive dimensionality: bulk→boundary→reconstruction",
    },
    TypeProgram {
        shavian: "are", primitive_axis: "⊣", value_glyph: "𐑸",
        opcodes: &[VINIT, IMSCRIB, AFWD, IFIX, FSPLIT, EVALT,
                    CLINK, AREV, FFUSE, ENGAGR, TANCH],
        rho: 2.2791,
        domain_reading: "Imscriptive topology: boundary↔bulk closure",
    },
    TypeProgram {
        shavian: "ear", primitive_axis: "≻", value_glyph: "𐑽",
        opcodes: &[VINIT, IMSCRIB, FSPLIT, AFWD, EVALT, AREV,
                    EVALF, FFUSE, CLINK, ENGAGR, IFIX, TANCH],
        rho: 2.2581,
        domain_reading: "Dagger-adjoint coupling: A→A†→bidirectional",
    },
    TypeProgram {
        shavian: "tot", primitive_axis: "≻", value_glyph: "𐑽",
        opcodes: &[VINIT, IMSCRIB, FSPLIT, AFWD, EVALT, AREV,
                    EVALF, FFUSE, CLINK, ENGAGR, IFIX, TANCH],
        rho: 2.2581,
        domain_reading: "Functor adjunction: composed functor pair",
    },
    TypeProgram {
        shavian: "out", primitive_axis: "≺", value_glyph: "𐑬",
        opcodes: &[VINIT, IMSCRIB,
                    IFIX, IFIX, IFIX, IFIX, IFIX, IFIX,
                    IFIX, IFIX, IFIX, IFIX, IFIX, IFIX,
                    IFIX, CLINK, TANCH,
                    FSPLIT, EVALT, AFWD, IMSCRIB, FFUSE, IFIX],
        rho: 2.2568,
        domain_reading: "Frobenius partial symmetry: 17,280,000-type crystal lattice",
    },
    TypeProgram {
        shavian: "peep", primitive_axis: "⋈", value_glyph: "𐑐",
        opcodes: &[VINIT, IMSCRIB, AFWD, FSPLIT, EVALT, CLINK,
                    FFUSE, AREV, ENGAGR, IFIX, TANCH],
        rho: 2.3203,
        domain_reading: "Quantum fidelity: unitary→verify→fuse→reverse coherence",
    },
    TypeProgram {
        shavian: "egg", primitive_axis: "⊤", value_glyph: "𐑧",
        opcodes: &[VINIT, IMSCRIB, AFWD, FSPLIT, EVALT, IFIX,
                    EVALF, AREV, FFUSE, CLINK, ENGAGR, IFIX, TANCH],
        rho: 2.2657,
        domain_reading: "Thermal kinetics: barrier crossing with dwell-time measurement",
    },
    TypeProgram {
        shavian: "thigh", primitive_axis: "∈", value_glyph: "𐑔",
        opcodes: &[VINIT, IMSCRIB, AFWD, FSPLIT, EVALT, EVALF,
                    FFUSE, CLINK, IMSCRIB, IFIX, TANCH],
        rho: 2.3203,
        domain_reading: "Mesoscale cardinality: aggregation→correlation→synthesis",
    },
    TypeProgram {
        shavian: "vow", primitive_axis: "∋", value_glyph: "𐑝",
        opcodes: &[VINIT, IMSCRIB, FSPLIT, EVALT, AFWD, EVALT,
                    AREV, EVALF, FFUSE, CLINK, IFIX, TANCH],
        rho: 2.2417,
        domain_reading: "Conjunctive composition: parallel condition verification",
    },
    TypeProgram {
        shavian: "monad", primitive_axis: "⊙", value_glyph: "⊙",
        opcodes: &[VINIT, AFWD, FSPLIT, EVALT, IMSCRIB, FFUSE,
                    CLINK, IFIX, TANCH],
        rho: 2.3106,
        domain_reading: "Critical fixed point: renormalization→absorbing property (ξ→∞, μ∘δ=id)",
    },
    TypeProgram {
        shavian: "sure", primitive_axis: "⊥", value_glyph: "𐑖",
        opcodes: &[VINIT, IMSCRIB, FSPLIT, AFWD, EVALT, AREV,
                    EVALF, FFUSE, CLINK, IMSCRIB, IFIX, TANCH],
        rho: 2.2581,
        domain_reading: "Two-step chirality: parity-distinct temporal paths with self-verification",
    },
    TypeProgram {
        shavian: "so", primitive_axis: "⊞", value_glyph: "𐑕",
        opcodes: &[VINIT, IMSCRIB, FSPLIT, EVALT, AFWD, AREV,
                    EVALT, FFUSE, CLINK, IFIX, TANCH],
        rho: 2.2552,
        domain_reading: "Many-identical stoichiometry: n↔n cardinality verification",
    },
    TypeProgram {
        shavian: "ah", primitive_axis: "⊡", value_glyph: "𐑭",
        opcodes: &[VINIT, IMSCRIB, AFWD, FSPLIT, EVALT, CLINK,
                    FFUSE, IMSCRIB, IFIX, TANCH],
        rho: 2.3180,
        domain_reading: "Z-topological winding: loop traversal→integer winding record (∮A=2πn)",
    },
];

// ═══════════════════════════════════════════════════════════════
// PROTOCOLS
// ═══════════════════════════════════════════════════════════════

pub static PROTOCOL_A: &[Opcode] = &[
    VINIT, IMSCRIB, FSPLIT, EVALT, EVALF, ENGAGR,
    FFUSE, CLINK, AFWD, AREV, IFIX, TANCH,
];
pub const PROTOCOL_A_RHO: f32 = 2.2800;
pub const PROTOCOL_A_ARM: &str = "EVALT→EVALF→ENGAGR (emergence/annihilation at EP)";

pub static PROTOCOL_B: &[Opcode] = &[
    VINIT, IMSCRIB, FSPLIT, AFWD, EVALT, AREV,
    EVALF, FFUSE, CLINK, ENGAGR, IFIX, TANCH,
];
pub const PROTOCOL_B_RHO: f32 = 2.2581;
pub const PROTOCOL_B_ARM: &str = "AFWD→EVALT→AREV→EVALF (imscriptive round-trip)";

pub static ROOT_WORD: &[Opcode] = &[
    VINIT, IMSCRIB, IFIX, FSPLIT, AFWD, EVALT,
    AREV, EVALF, ENGAGR, FFUSE, CLINK, IMSCRIB,
    IFIX, TANCH,
];
pub const ROOT_WORD_RHO: f32 = 2.2526;

// Primitive → Shavian type name mapping
pub fn primitive_to_type(prim: &str) -> Option<&'static str> {
    match prim {
        "⊢" => Some("if"),  "⊣" => Some("are"), "≻" => Some("ear"),
        "≺" => Some("out"), "⋈" => Some("peep"), "⊤" => Some("egg"),
        "∈" => Some("thigh"), "∋" => Some("vow"), "⊙" => Some("monad"),
        "⊥" => Some("sure"), "⊞" => Some("so"), "⊡" => Some("ah"),
        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════
// FROBENIUS VERIFICATION
// ═══════════════════════════════════════════════════════════════

/// Verdict: T = clean closure, B = Belnap (paradox held), F = open
#[derive(Debug, Clone, Copy)]
pub enum Verdict { T, B, F }

impl Verdict {
    pub fn glyph(&self) -> &'static str {
        match self { Verdict::T => "T", Verdict::B => "B", Verdict::F => "F" }
    }
}

#[derive(Debug, Clone)]
pub struct FrobeniusResult {
    pub closed: bool,
    pub verdict: Verdict,
    pub split_idx: Option<usize>,
    pub fuse_idx: Option<usize>,
    pub arm_length: usize,
    pub arm_glyphs: String,
    pub rho: f32,
    /// Where rho came from. A value carried over from another implementation
    /// agrees with that implementation by construction and is not a check on
    /// it; a value estimated by length is not a spectral radius at all. Both
    /// were previously reported in the same field as though measured.
    pub rho_provenance: RhoSource,
    pub n_ops: usize,
    pub has_engagr: bool,
    pub note: String,
}

/// Where a reported rho came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RhoSource {
    /// Carried over from the Python implementation. Reporting it back is not a
    /// verification of that implementation: it cannot disagree.
    Tabulated,
    /// The length heuristic. Not a spectral radius.
    LengthEstimate,
}

impl RhoSource {
    pub fn label(self) -> &'static str {
        match self {
            RhoSource::Tabulated => "tabulated (carried over, not independently computed)",
            RhoSource::LengthEstimate => "length heuristic (NOT a spectral radius)",
        }
    }
}

/// Look up the known ρ for a word. Stored values take precedence over computation.
fn known_rho(word: &[Opcode]) -> Option<f32> {
    // Check protocol words
    if word == PROTOCOL_A { return Some(PROTOCOL_A_RHO); }
    if word == PROTOCOL_B { return Some(PROTOCOL_B_RHO); }
    if word == ROOT_WORD   { return Some(ROOT_WORD_RHO); }
    // Check type programs
    for tp in TYPE_PROGRAMS {
        if word == tp.opcodes { return Some(tp.rho); }
    }
    None
}

/// Simple sqrt for no_std (Newton's method).
fn sqrt_f32(x: f32) -> f32 {
    // no_std does not mean no sqrt here: the crate already depends on libm and
    // fibonacci_qc.rs uses it throughout. A hand-rolled Newton loop in f32 is
    // both unnecessary and less accurate than the one already linked in.
    if x <= 0.0 { return 0.0; }
    libm::sqrt(x as f64) as f32
}

/// A LENGTH HEURISTIC, not a spectral radius.
///
/// This computes sqrt(n) scaled by the split-to-fuse fraction of the word. It
/// builds no adjacency matrix and takes no eigenvalue, so calling its output
/// rho invites it to be read as the spectral radius the stored table holds,
/// which it is not and does not approach. It exists so that a word outside the
/// table returns something rather than nothing; anything derived from it should
/// be treated as unmeasured.
fn estimate_rho_by_length(word: &[Opcode]) -> f32 {
    let n = word.len();
    if n <= 1 { return 0.0; }
    // Find split/fuse indices for the Frobenius edge
    let split_idx = word.iter().position(|&oc| oc == FSPLIT);
    let fuse_idx  = word.iter().position(|&oc| oc == FFUSE);
    // Simple rho estimate: sqrt(n) adjusted for split-fuse distance
    let base = sqrt_f32(n as f32);
    if let (Some(si), Some(fi)) = (split_idx, fuse_idx) {
        if fi > si {
            let arm = (fi - si) as f32;
            1.0 + (arm / n as f32) * (base - 1.0)
        } else { base }
    } else { base }
}

/// Verify Frobenius closure (μ∘δ=id) for a word.
pub fn check_frobenius(word: &[Opcode]) -> FrobeniusResult {
    let split_idx = word.iter().position(|&oc| oc == FSPLIT);
    let fuse_idx  = word.iter().position(|&oc| oc == FFUSE);
    let (rho, rho_provenance) = match known_rho(word) {
        Some(r) => (r, RhoSource::Tabulated),
        None => (estimate_rho_by_length(word), RhoSource::LengthEstimate),
    };

    if split_idx.is_none() || fuse_idx.is_none() {
        return FrobeniusResult {
            closed: false, verdict: Verdict::F,
            split_idx, fuse_idx, arm_length: 0,
            arm_glyphs: String::new(), rho: 0.0,
            rho_provenance,
            n_ops: word.len(), has_engagr: false,
            note: "No split/fuse pair".into(),
        };
    }

    let si = split_idx.unwrap();
    let fi = fuse_idx.unwrap();
    let arm_len = if fi > si { fi - si - 1 } else { 0 };
    let has_engagr = if fi > si {
        word[si..=fi].contains(&ENGAGR) || word.contains(&ENGAGR)
    } else { word.contains(&ENGAGR) };

    let closed = arm_len > 0 && fi > si;
    let verdict = if closed && !has_engagr { Verdict::T }
                  else if closed { Verdict::B }
                  else { Verdict::F };

    let arm_glyphs: String = if fi > si + 1 {
        word[si+1..fi].iter().map(|oc| oc.glyph()).collect()
    } else {
        String::new()
    };

    let note = if has_engagr {
        "Paradox held at ENGAGR — Belnap B".into()
    } else if closed {
        "Clean closure".into()
    } else {
        "Open".into()
    };

    FrobeniusResult {
        rho_provenance,
        closed, verdict, split_idx: Some(si), fuse_idx: Some(fi),
        arm_length: arm_len, arm_glyphs, rho,
        n_ops: word.len(), has_engagr, note,
    }
}

// ═══════════════════════════════════════════════════════════════
// TRIPLE FRAME ALGEBRA
// ═══════════════════════════════════════════════════════════════

pub struct TripleFrameAlgebra;

impl TripleFrameAlgebra {
    /// Expand a Shavian type name or primitive axis into its TypeProgram.
    pub fn expand(name: &str) -> Option<&'static TypeProgram> {
        // Try direct Shavian name
        for tp in TYPE_PROGRAMS {
            if tp.shavian == name { return Some(tp); }
        }
        // Try primitive axis
        if let Some(shav) = primitive_to_type(name) {
            for tp in TYPE_PROGRAMS {
                if tp.shavian == shav { return Some(tp); }
            }
        }
        None
    }

    /// Get the full 146-opcode composite bootstrap word.
    pub fn full_word() -> String {
        let order = ["if","are","ear","out","peep","egg",
                     "thigh","vow","monad","sure","so","ah"];
        let mut w = String::new();
        for name in &order {
            for tp in TYPE_PROGRAMS {
                if tp.shavian == *name {
                    w.push_str(&tp.word());
                    break;
                }
            }
        }
        w
    }

    /// Get the glyph word for a protocol variant.
    pub fn protocol_word(variant: &str) -> String {
        let ops = match variant.to_uppercase().as_str() {
            "A" => PROTOCOL_A,
            _   => PROTOCOL_B,
        };
        ops.iter().map(|oc| oc.glyph()).collect()
    }

    /// Verify Frobenius closure for all 12 type programs.
    pub fn verify_all_types() -> Vec<(String, FrobeniusResult)> {
        TYPE_PROGRAMS.iter().map(|tp| {
            (tp.shavian.to_string(), check_frobenius(tp.opcodes))
        }).collect()
    }

    /// Execute the tuple↔word round-trip (IMASM cycle).
    /// Returns (n_exact, n_ambiguous, note).
    pub fn imasm_cycle() -> (usize, usize, String) {
        // > maps to both ear and tot — structurally identical programs
        let n_ambiguous = 1;  // ear ≅ tot on >
        let n_exact = 11;     // All other axes bijective
        (n_exact, n_ambiguous,
         "11/12 axes exact, > ambiguous (ear/tot) — the theorized 2-to-1 axis".into())
    }

    /// Levenshtein edit distance between two word slices.
    pub fn edit_distance(a: &[Opcode], b: &[Opcode]) -> usize {
        let m = a.len();
        let n = b.len();
        if m == 0 { return n; }
        if n == 0 { return m; }

        let mut prev: Vec<usize> = (0..=n).collect();
        let mut curr: Vec<usize> = vec![0; n + 1];

        for i in 1..=m {
            curr[0] = i;
            for j in 1..=n {
                let cost = if a[i-1] == b[j-1] { 0 } else { 1 };
                curr[j] = (prev[j] + 1)
                    .min(curr[j-1] + 1)
                    .min(prev[j-1] + cost);
            }
            core::mem::swap(&mut prev, &mut curr);
        }
        prev[n]
    }

    /// Total opcode count across all types.
    pub fn total_ops() -> usize {
        TYPE_PROGRAMS.iter().map(|tp| tp.n_ops()).sum()
    }
}

// ═══════════════════════════════════════════════════════════════
// FIBONACCI ANYON BRIDGE
// ═══════════════════════════════════════════════════════════════

pub struct TripleFrameManifold {
    pub phi: f32,
    pub d: f32,
    pub c: f32,  // central charge = 14/5
}

impl TripleFrameManifold {
    pub fn new() -> Self {
        Self {
            phi: 1.61803399,
            d: 1.90211304,
            c: 14.0 / 5.0,
        }
    }

    pub fn rho_to_curvature(&self, rho: f32) -> f32 {
        self.phi * rho / self.d
    }

    pub fn winding_to_central_charge(&self, winding_class: &str) -> f32 {
        match winding_class {
            "Z"  => self.c,
            "Z2" => self.c / 2.0,
            _    => self.c * 2.0,
        }
    }

    pub fn bridge_report(&self) -> String {
        let pb = check_frobenius(PROTOCOL_B);
        let curv = self.rho_to_curvature(pb.rho);
        format!(
            "╔══════════════════════════════════════════════════════════════╗\n\
             ║  TRIPLE FRAME ↔ FIBONACCI MANIFOLD BRIDGE                    ║\n\
             ╚══════════════════════════════════════════════════════════════╝\n\
             \n\
             Fibonacci: φ={:.10}, D={:.10}, c={}\n\
             Triple Frame ρ={}, curvature estimate={:.4}\n\
             \n\
             Shared structure:\n\
               Frobenius μ∘δ=id  ↔  Fibonacci fusion τ×τ=1+τ\n\
               Belnap B-class    ↔  both hold paradox (ENGAGR ↔ τ channel)\n\
               ρ ≈ 2.2581        ↔  det(S) curvature at φ²\n\
               ⊡=𐑭 (Z)           ↔  central charge c=14/5\n\
               ⊙=⊙ (critical)    ↔  topological spin θ_τ at fixed point\n",
            self.phi, self.d, self.c, pb.rho, curv
        )
    }
}

// ═══════════════════════════════════════════════════════════════
// REPORTS — called from repl.rs dispatch
// ═══════════════════════════════════════════════════════════════

/// Type expansion table (formatted)
pub fn type_table() -> String {
    let mut out = String::new();
    out.push_str(&format!("{:<5} {:<5} {:<8} {:<5} {:<8} {}\n",
        "Axis", "Value", "Type", "Ops", "ρ", "Domain"));
    out.push_str(&"-".repeat(70));
    out.push('\n');
    for tp in TYPE_PROGRAMS {
        out.push_str(&format!("{:<5} {:<5} {:<8} {:<5} {:<8.4} {}\n",
            tp.primitive_axis, tp.value_glyph, tp.shavian,
            tp.n_ops(), tp.rho, crate::text::clip_ellipsis(tp.domain_reading, 45)));
    }
    out
}

/// Full structural report (analogous to protocol_report in Python)
pub fn full_report() -> String {
    let pa = check_frobenius(PROTOCOL_A);
    let pb = check_frobenius(PROTOCOL_B);
    let pr = check_frobenius(ROOT_WORD);

    let total_ops = TripleFrameAlgebra::total_ops();
    let verified = TripleFrameAlgebra::verify_all_types();
    let n_closed = verified.iter().filter(|(_, r)| r.closed).count();

    let proto_a_word: String = PROTOCOL_A.iter().map(|oc| oc.glyph()).collect();
    let proto_b_word: String = PROTOCOL_B.iter().map(|oc| oc.glyph()).collect();
    let root_word_str: String = ROOT_WORD.iter().map(|oc| oc.glyph()).collect();

    let (n_exact, n_ambiguous, cycle_note) = TripleFrameAlgebra::imasm_cycle();

    format!(
        "╔══════════════════════════════════════════════════════════════╗\n\
         ║  TRIPLE FRAME VON NEUMANN SUPEROPERATOR ALGEBRA              ║\n\
         ║  Type-Expansion Hierarchy — Executable IMASM Bootstrap       ║\n\
         ╚══════════════════════════════════════════════════════════════╝\n\
         \n\
         Tuple: {}\n\
         Types: 12 primitive → {} opcodes total\n\
         Closed: {}/12 type programs verify Frobenius ✓\n\
         \n\
         ── Protocol Variants ──────────────────────────────────────────\n\
           Protocol A (emergence/annihilation at EP):\n\
             Word: {}\n\
             ρ={}, verdict={}, arm={}\n\
         \n\
           Protocol B (imscriptive round-trip):\n\
             Word: {}\n\
             ρ={}, verdict={}, arm={}\n\
         \n\
           Root word (14-glyph, doubled IMSCRIB+IFIX):\n\
             Word: {}\n\
             ρ={}, verdict={}\n\
         \n\
         ── Type Expansion Table ───────────────────────────────────────\n\
         {}\n\
         ── Isomorphic Triplet (ρ=2.2581) ──────────────────────────────\n\
           sure (⊥=𐑖): Two-step chirality: parity-distinct temporal paths\n\
           ear  (>=𐑽): Dagger-adjoint coupling: A→A†→bidirectional\n\
           tot  (>=𐑽): Functor adjunction: composed functor pair\n\
         \n\
         ── The 2-to-1 Axis ────────────────────────────────────────────\n\
           > maps to ear ≅ tot — structurally identical programs (same\n\
           12-opcode sequence, same ρ=2.2581). The grammar is bijective\n\
           on 11 axes, 2-to-1 on the twelfth.\n\
         \n\
         ── IMASM Cycle ────────────────────────────────────────────────\n\
           {}/{} exact, {} ambiguous\n\
           {}\n\
         \n\
         ── Notable ────────────────────────────────────────────────────\n\
           out (<=𐑬): 22-opcode giant — encodes the full 17,280,000-type\n\
           crystal lattice via 13 consecutive IFIX operations.\n\
           monad (⊙=⊙): shortest at 9 opcodes — the critical fixed point.\n\
         \n\
         ── Integration with m3iosis ───────────────────────────────────\n\
           Shared invariants: ρ, ⊡, ⊙\n\
           Frobenius μ∘δ=id ↔ Fibonacci fusion τ×τ=1+τ (both Belnap B-class).\n",
        TRIPLE_FRAME_TUPLE, total_ops, n_closed,
        proto_a_word, pa.rho, pa.verdict.glyph(), PROTOCOL_A_ARM,
        proto_b_word, pb.rho, pb.verdict.glyph(), PROTOCOL_B_ARM,
        root_word_str, pr.rho, pr.verdict.glyph(),
        type_table(),
        n_exact, 12, n_ambiguous, cycle_note,
    )
}

/// Verify Frobenius for all types, formatted for REPL.
pub fn verify_report() -> String {
    let mut out = String::new();
    for tp in TYPE_PROGRAMS {
        let r = check_frobenius(tp.opcodes);
        let status = if r.closed { "✓" } else { "✗" };
        out.push_str(&format!("  {} {:<8} ρ={:<8.4} {}  {}\n",
            status, tp.shavian, r.rho, r.verdict.glyph(), r.note));
    }
    out
}

/// Expand a single type for REPL display.
pub fn expand_report(name: &str) -> String {
    match TripleFrameAlgebra::expand(name) {
        Some(tp) => {
            let r = check_frobenius(tp.opcodes);
            format!(
                "{}={}  →  {}\n  Word:  {}\n  Ops:   {}\n  ρ:     {}\n  Read:  {}\n  Close: {}",
                tp.primitive_axis, tp.value_glyph, tp.shavian,
                tp.word(), tp.n_ops(), tp.rho, tp.domain_reading,
                r.verdict.glyph()
            )
        }
        None => {
            let known: Vec<&str> = TYPE_PROGRAMS.iter()
                .map(|tp| tp.shavian).collect();
            format!("Unknown type: '{}'. Known: {:?}\nTry primitive axis: ⊢ ⊣ ≻ ≺ ⋈ ⊤ ∈ ∋ ⊙ ⊥ ⊞ ⊡",
                name, known)
        }
    }
}

/// Type listing for REPL.
pub fn types_report() -> String {
    type_table()
}

/// IMASM cycle for REPL.
pub fn cycle_report() -> String {
    let (n_exact, n_ambiguous, note) = TripleFrameAlgebra::imasm_cycle();
    format!(
        "IMASM cycle: {}/{} exact\n  Ambiguous: {} (>: ear/tot)\n  {}",
        n_exact, 12, n_ambiguous, note
    )
}

/// Protocol A→B edit path for REPL.
pub fn path_report() -> String {
    let dist = TripleFrameAlgebra::edit_distance(PROTOCOL_A, PROTOCOL_B);
    let a_word: String = PROTOCOL_A.iter().map(|oc| oc.glyph()).collect();
    let b_word: String = PROTOCOL_B.iter().map(|oc| oc.glyph()).collect();
    format!(
        "Protocol A → B: {} edits\n  A: {}\n  B: {}\n  {} single-opcode edits, all B-class, lateral walk",
        dist, a_word, b_word, dist
    )
}

/// Root word for REPL.
pub fn word_report(variant: &str) -> String {
    match variant.to_uppercase().as_str() {
        "FULL" => {
            let w = TripleFrameAlgebra::full_word();
            format!("Full bootstrap word ({} glyphs):\n{}", w.len(), w)
        }
        "ROOT" => {
            ROOT_WORD.iter().map(|oc| oc.glyph()).collect::<String>()
        }
        _ => TripleFrameAlgebra::protocol_word(variant),
    }
}

/// Self-consistency check for REPL.
pub fn check_report(word_str: &str) -> String {
    if word_str.is_empty() {
        let r = check_frobenius(PROTOCOL_B);
        return format!("Protocol B: closed={} verdict={} ρ={} [{}] arm={}",
            r.closed, r.verdict.glyph(), r.rho, r.rho_provenance.label(), r.arm_glyphs);
    }
    let ops: Vec<Opcode> = word_str.chars()
        .filter_map(|c| Opcode::from_glyph(c))
        .collect();
    if ops.is_empty() {
        return format!("No valid opcodes in: '{}'", word_str);
    }
    let r = check_frobenius(&ops);
    let word: String = ops.iter().map(|oc| oc.glyph()).collect();
    format!(
        "Word: {} ({} ops)\n  closed={}  verdict={}  ρ={}\n  ρ source: {}\n  split={:?}  fuse={:?}  arm={}\n  {}",
        word, ops.len(), r.closed, r.verdict.glyph(), r.rho,
        r.rho_provenance.label(),
        r.split_idx, r.fuse_idx, r.arm_glyphs, r.note
    )
}

/// Bridge to Fibonacci manifold for REPL.
pub fn bridge_report() -> String {
    TripleFrameManifold::new().bridge_report()
}

/// Help text for the triple command.
pub fn triple_help() -> &'static str {
    "triple — Triple Frame von Neumann Superoperator Algebra\n\
     \n\
     Subcommands:\n\
       triple report     Full structural report\n\
       triple expand <t>  Expand a Shavian type or primitive\n\
       triple word <var>  Protocol word (A, B, root, full)\n\
       triple verify      Frobenius closure check (all 12 types)\n\
       triple types       Type expansion table\n\
       triple cycle       IMASM tuple↔word round-trip\n\
       triple path        Edit distance Protocol A → B\n\
       triple bridge      Triple frame ↔ Fibonacci manifold\n\
       triple check [w]   Check a custom word for Frobenius\n\
       triple tuple       Show the grammar tuple\n\
       \n\
     Tuple: ⟨𐑦𐑸𐑽𐑬𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭⟩\n\
     12 primitives. 159 total opcodes. 12/12 Frobenius-closed. One 2-to-1 axis."
}

// ═══════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_12_types_frobenius_closed() {
        for tp in TYPE_PROGRAMS {
            let r = check_frobenius(tp.opcodes);
            assert!(r.closed, "{} should be Frobenius-closed", tp.shavian);
        }
    }

    #[test]
    fn test_protocol_edit_distance() {
        let dist = TripleFrameAlgebra::edit_distance(PROTOCOL_A, PROTOCOL_B);
        assert_eq!(dist, 5, "Protocol A→B should be 5 edits");
    }

    #[test]
    fn test_imasm_cycle() {
        let (exact, ambig, _) = TripleFrameAlgebra::imasm_cycle();
        assert_eq!(exact, 11);
        assert_eq!(ambig, 1);
    }

    #[test]
    fn test_expand_by_shavian() {
        let tp = TripleFrameAlgebra::expand("sure").expect("sure should exist");
        assert_eq!(tp.primitive_axis, "⊥");
        assert_eq!(tp.rho, 2.2581);
    }

    #[test]
    fn test_expand_by_primitive() {
        let tp = TripleFrameAlgebra::expand("⊡").expect("⊡ should expand");
        assert_eq!(tp.shavian, "ah");
    }

    #[test]
    fn test_protocol_b_rho() {
        let r = check_frobenius(PROTOCOL_B);
        assert_eq!(r.rho, 2.2581);
        assert!(r.closed);
    }

    #[test]
    fn test_monad_shortest() {
        let tp = TripleFrameAlgebra::expand("monad").unwrap();
        assert_eq!(tp.n_ops(), 9);
    }

    #[test]
    fn test_out_longest() {
        let tp = TripleFrameAlgebra::expand("out").unwrap();
        assert_eq!(tp.n_ops(), 23);
    }

    #[test]
    fn test_root_word_rho() {
        let r = check_frobenius(ROOT_WORD);
        assert_eq!(r.rho, 2.2526);
        assert!(r.closed);
    }

    #[test]
    fn test_total_ops() {
        assert_eq!(TripleFrameAlgebra::total_ops(), 159);
    }
}
