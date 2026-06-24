// shared.rs — Universal opcode registry, grammar mappings, canonical sequences, domains
// Ported from cr3echrz/shared/ for mOMonadOS
// Author: Lando⊗⊙perator
#![allow(dead_code)]


// ─── 12 Universal IMASM Opcodes ────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Opcode {
    VInit   = 0,  // Initialize the void — ground of distinction
    TAnchor = 1,  // Terminal anchor — boundary condition / theorem statement
    FSplit  = 2,  // Frobenius split δ — decomposition into (T, F) arms
    FFuse   = 3,  // Frobenius fuse μ — recomposition from arms
    EvalT   = 4,  // Evaluate-true — theorem holds / true branch
    EvalF   = 5,  // Evaluate-false — theorem fails / false branch
    EngPar  = 6,  // Engage paradox — dialetheic boundary (both arms)
    AFwd    = 7,  // Forward morphism — theorem-specific forward operation
    ARev    = 8,  // Reverse morphism — theorem-specific reverse operation
    CLink   = 9,  // Chain link — sequential composition
    ImScrib = 10, // Self-imscribe — verify constants / identity
    IFix     = 11, // Irreversible fix — permanent record / Poincaré section
}

impl Opcode {
    pub const ALL: [Opcode; 12] = [
        Opcode::VInit, Opcode::TAnchor, Opcode::FSplit, Opcode::FFuse,
        Opcode::EvalT, Opcode::EvalF, Opcode::EngPar, Opcode::AFwd,
        Opcode::ARev, Opcode::CLink, Opcode::ImScrib, Opcode::IFix,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Opcode::VInit   => "VINIT",
            Opcode::TAnchor => "TANCH",
            Opcode::FSplit  => "FSPLIT",
            Opcode::FFuse   => "FFUSE",
            Opcode::EvalT   => "EVALT",
            Opcode::EvalF   => "EVALF",
            Opcode::EngPar  => "ENGAGR",
            Opcode::AFwd    => "AFWD",
            Opcode::ARev    => "AREV",
            Opcode::CLink   => "CLINK",
            Opcode::ImScrib => "IMSCRIB",
            Opcode::IFix    => "IFIX",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "VINIT" => Some(Opcode::VInit),
            "TANCH" => Some(Opcode::TAnchor),
            "FSPLIT" => Some(Opcode::FSplit),
            "FFUSE" => Some(Opcode::FFuse),
            "EVALT" => Some(Opcode::EvalT),
            "EVALF" => Some(Opcode::EvalF),
            "ENGAGR" => Some(Opcode::EngPar),
            "AFWD" => Some(Opcode::AFwd),
            "AREV" => Some(Opcode::ARev),
            "CLINK" => Some(Opcode::CLink),
            "IMSCRIB" => Some(Opcode::ImScrib),
            "IFIX" => Some(Opcode::IFix),
            _ => None,
        }
    }

    pub fn is_split(self) -> bool { self == Opcode::FSplit }
    pub fn is_fuse(self) -> bool { self == Opcode::FFuse }
    pub fn is_frob_pair(self, other: Opcode) -> bool {
        (self == Opcode::FSplit && other == Opcode::FFuse)
        || (self == Opcode::FFuse && other == Opcode::FSplit)
    }
}

// ─── Grammar primitive mapping ──────────────────────────────────────

pub struct GrammarPrim {
    pub symbol: &'static str,
    pub latin: &'static str,
    pub desc: &'static str,
}

pub fn opcode_grammar(op: Opcode) -> GrammarPrim {
    match op {
        Opcode::VInit   => GrammarPrim { symbol: "𐑼", latin: "Ð", desc: "Dimensionality — ground of distinction" },
        Opcode::TAnchor => GrammarPrim { symbol: "𐑡", latin: "Þ", desc: "Topology — boundary condition / container" },
        Opcode::FSplit  => GrammarPrim { symbol: "𐑚", latin: "Γ", desc: "Split (δ) — Frobenius decomposition" },
        Opcode::FFuse   => GrammarPrim { symbol: "𐑙", latin: "Σ", desc: "Fuse (μ) — Frobenius recomposition" },
        Opcode::EvalT   => GrammarPrim { symbol: "⊙",  latin: "φ̂", desc: "Criticality — evaluate-true gate" },
        Opcode::EvalF   => GrammarPrim { symbol: "𐑖", latin: "Ħ", desc: "Chirality — evaluate-false gate" },
        Opcode::EngPar  => GrammarPrim { symbol: "𐑳", latin: "Σ", desc: "Stoichiometry — engage paradox" },
        Opcode::AFwd    => GrammarPrim { symbol: "𐑾", latin: "Ř", desc: "Coupling — forward morphism" },
        Opcode::ARev    => GrammarPrim { symbol: "𐑬", latin: "Φ", desc: "Parity — reverse morphism" },
        Opcode::CLink   => GrammarPrim { symbol: "𐑱", latin: "ƒ", desc: "Kinetics — chain sequential composition" },
        Opcode::ImScrib => GrammarPrim { symbol: "𐑠", latin: "ɢ", desc: "Composition — self-imscribe / verify" },
        Opcode::IFix    => GrammarPrim { symbol: "𐑭", latin: "Ω", desc: "Winding — irreversible fixation" },
    }
}

// ─── Canonical Bootstrap Sequences ──────────────────────────────────

/// The 12 canonical IMASM bootstrap sequences (I–XII).
pub static CANONICAL_SEQUENCES: &[(&str, &[Opcode])] = &[
    ("I_Dialetheic_Bootstrap", &[
        Opcode::VInit, Opcode::TAnchor, Opcode::FSplit, Opcode::EvalT, Opcode::AFwd,
        Opcode::FFuse, Opcode::FSplit, Opcode::EvalF, Opcode::ARev, Opcode::FFuse,
        Opcode::EngPar, Opcode::CLink, Opcode::ImScrib, Opcode::IFix, Opcode::IFix, Opcode::TAnchor,
    ]),
    ("II_Void_Genesis", &[
        Opcode::VInit, Opcode::ImScrib, Opcode::AFwd, Opcode::FSplit, Opcode::EvalT,
        Opcode::AFwd, Opcode::EvalF, Opcode::ARev, Opcode::FFuse, Opcode::CLink,
        Opcode::IFix, Opcode::TAnchor,
    ]),
    ("III_Anchor_Protocol", &[
        Opcode::VInit, Opcode::TAnchor, Opcode::ImScrib, Opcode::AFwd, Opcode::ARev,
        Opcode::CLink, Opcode::FSplit, Opcode::FFuse, Opcode::EvalT, Opcode::EvalF,
        Opcode::EngPar, Opcode::IFix,
    ]),
    ("IV_Dual_Bootstrap", &[
        Opcode::VInit, Opcode::ImScrib, Opcode::AFwd, Opcode::FSplit, Opcode::EvalT,
        Opcode::AFwd, Opcode::FFuse, Opcode::FSplit, Opcode::EvalF, Opcode::ARev,
        Opcode::FFuse, Opcode::EngPar, Opcode::CLink, Opcode::ImScrib, Opcode::IFix, Opcode::TAnchor,
    ]),
    ("V_Linear_Chain", &[
        Opcode::VInit, Opcode::TAnchor, Opcode::AFwd, Opcode::CLink, Opcode::ARev,
        Opcode::ImScrib, Opcode::IFix,
    ]),
    ("VI_Empty_Bootstrap", &[
        Opcode::VInit, Opcode::TAnchor,
    ]),
    ("VII_Parakernel", &[
        Opcode::VInit, Opcode::TAnchor, Opcode::ImScrib, Opcode::FSplit, Opcode::EvalT,
        Opcode::FFuse, Opcode::FSplit, Opcode::EvalF, Opcode::FFuse, Opcode::EngPar,
        Opcode::CLink, Opcode::AFwd, Opcode::ARev, Opcode::ImScrib, Opcode::IFix, Opcode::TAnchor,
    ]),
    ("VIII_Frobenius_Kernel", &[
        Opcode::VInit, Opcode::FSplit, Opcode::FFuse, Opcode::TAnchor,
    ]),
    ("IX_Chiral_Pairs", &[
        Opcode::VInit, Opcode::EvalT, Opcode::EvalF, Opcode::EngPar, Opcode::FSplit,
        Opcode::FFuse, Opcode::TAnchor,
    ]),
    ("X_Truth_Machine", &[
        Opcode::VInit, Opcode::TAnchor, Opcode::ImScrib, Opcode::EvalT, Opcode::AFwd,
        Opcode::FSplit, Opcode::FFuse, Opcode::EvalF, Opcode::ARev, Opcode::CLink, Opcode::IFix,
    ]),
    ("XI_Eternal_Return", &[
        Opcode::VInit, Opcode::AFwd, Opcode::CLink, Opcode::ARev, Opcode::EngPar,
        Opcode::ImScrib, Opcode::IFix, Opcode::TAnchor,
    ]),
    ("XII_ROM_Burn", &[
        Opcode::VInit, Opcode::ImScrib, Opcode::IFix, Opcode::TAnchor,
    ]),
];

pub fn canonical_sequence(name: &str) -> Option<&[Opcode]> {
    CANONICAL_SEQUENCES.iter()
        .find(|(n, _)| *n == name)
        .map(|(_, seq)| *seq)
}

pub fn canonical_name_index(index: usize) -> Option<&'static str> {
    if index < CANONICAL_SEQUENCES.len() {
        Some(CANONICAL_SEQUENCES[index].0)
    } else {
        None
    }
}

// ─── Domain Classification ──────────────────────────────────────────

/// Classify a theorem/ob3ect name into its domain type.
pub fn infer_domain(name: &str) -> &'static str {
    let nl = name.to_lowercase();
    // Mathematical keywords
    for kw in ["theorem", "conjecture", "connes", "collatz", "goldbach",
                "galois", "burnside", "erdos", "straus", "baum",
                "three_body", "threebody", "pythagorean", "landau"] {
        if nl.contains(kw) { return "mathematical"; }
    }
    // Physical keywords
    for kw in ["quantum", "field_theory", "cosmology", "black_hole", "gravity",
                "neutrino", "gauge", "higgs"] {
        if nl.contains(kw) { return "physical"; }
    }
    // Alchemical
    for kw in ["alembic", "stone", "lapis", "elixir", "rebis", "hermetic",
                "alchemical"] {
        if nl.contains(kw) { return "alchemical"; }
    }
    // Magical
    for kw in ["magic", "servitor", "sigil", "goetic", "pentagram", "apotropaic"] {
        if nl.contains(kw) { return "magical"; }
    }
    // Computational
    for kw in ["kernel", "compiler", "protocol", "virtual_machine", "proof_assistant"] {
        if nl.contains(kw) { return "computational"; }
    }
    // Divinatory
    for kw in ["tarot", "hexagram", "geomancy", "scrying", "rune", "futhark"] {
        if nl.contains(kw) { return "divinatory"; }
    }
    "symbolic"
}
