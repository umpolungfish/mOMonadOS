// shared.rs — Universal opcode registry, grammar mappings, canonical sequences, domains
// Ported from cr3echrz/shared/ for mOMonadOS — Phase 10 dynamic registry
// Author: Lando⊗⊙perator
#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ─── 12 Universal IMASM Opcodes ────────────────────────────────────
// JUSTIFIED static: the 12 opcodes ARE the grammar definition (cf. CARDS in catalog.rs).

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Opcode {
    VInit   = 0,
    TAnchor = 1,
    FSplit  = 2,
    FFuse   = 3,
    EvalT   = 4,
    EvalF   = 5,
    EngPar  = 6,
    AFwd    = 7,
    ARev    = 8,
    CLink   = 9,
    ImScrib = 10,
    IFix    = 11,
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
// JUSTIFIED static: this IS the grammar primitive↔opcode correspondence.

pub struct GrammarPrim {
    pub symbol: &'static str,
    pub latin: &'static str,
    pub desc: &'static str,
}

pub fn opcode_grammar(op: Opcode) -> GrammarPrim {
    match op {
        Opcode::VInit   => GrammarPrim { symbol: "\u{1047C}", latin: "Ð", desc: "Dimensionality" },
        Opcode::TAnchor => GrammarPrim { symbol: "\u{10461}", latin: "Þ", desc: "Topology" },
        Opcode::FSplit  => GrammarPrim { symbol: "\u{1045A}", latin: "Γ", desc: "Split (δ)" },
        Opcode::FFuse   => GrammarPrim { symbol: "\u{10459}", latin: "Σ", desc: "Fuse (μ)" },
        Opcode::EvalT   => GrammarPrim { symbol: "⊙",  latin: "φ̂", desc: "Criticality" },
        Opcode::EvalF   => GrammarPrim { symbol: "\u{10456}", latin: "Ħ", desc: "Chirality" },
        Opcode::EngPar  => GrammarPrim { symbol: "\u{10473}", latin: "Σ", desc: "Stoichiometry" },
        Opcode::AFwd    => GrammarPrim { symbol: "\u{1047E}", latin: "Ř", desc: "Coupling" },
        Opcode::ARev    => GrammarPrim { symbol: "\u{1046C}", latin: "Φ", desc: "Parity" },
        Opcode::CLink   => GrammarPrim { symbol: "\u{10471}", latin: "ƒ", desc: "Kinetics" },
        Opcode::ImScrib => GrammarPrim { symbol: "\u{10460}", latin: "ɢ", desc: "Composition" },
        Opcode::IFix    => GrammarPrim { symbol: "\u{1046D}", latin: "Ω", desc: "Winding" },
    }
}

// ─── Canonical Bootstrap Sequences ──────────────────────────────────
// JUSTIFIED static: these ARE the grammar — the 12 canonical IMASM programs.

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

// ─── Dynamic Domain Keyword Registry ────────────────────────────────
// Replaces hardcoded keyword lists in infer_domain().
// Keywords→domain mapping is populated at boot from the static bootstrap,
// extensible at runtime via register_domain_keyword().

pub struct DomainEntry {
    pub domain: &'static str,
    pub keywords: &'static [&'static str],
}

/// Static bootstrap — the known domain keyword sets (reference data, justified).
pub static DOMAIN_BOOTSTRAP: &[DomainEntry] = &[
    DomainEntry { domain: "mathematical", keywords: &[
        "theorem", "conjecture", "connes", "collatz", "goldbach",
        "galois", "burnside", "erdos", "straus", "baum",
        "three_body", "threebody", "pythagorean", "landau",
    ]},
    DomainEntry { domain: "physical", keywords: &[
        "quantum", "field_theory", "cosmology", "black_hole", "gravity",
        "neutrino", "gauge", "higgs",
    ]},
    DomainEntry { domain: "alchemical", keywords: &[
        "alembic", "stone", "lapis", "elixir", "rebis", "hermetic", "alchemical",
    ]},
    DomainEntry { domain: "magical", keywords: &[
        "magic", "servitor", "sigil", "goetic", "pentagram", "apotropaic",
    ]},
    DomainEntry { domain: "computational", keywords: &[
        "kernel", "compiler", "protocol", "virtual_machine", "proof_assistant",
    ]},
    DomainEntry { domain: "divinatory", keywords: &[
        "tarot", "hexagram", "geomancy", "scrying", "rune", "futhark",
    ]},
];

/// Runtime domain keyword map: keyword → domain.
/// Initialized from DOMAIN_BOOTSTRAP at first access; extensible via register_domain_keyword().
static mut DOMAIN_KEYWORD_MAP: Option<Vec<(String, &'static str)>> = None;

fn ensure_domain_map() -> &'static mut Vec<(String, &'static str)> {
    unsafe {
        if DOMAIN_KEYWORD_MAP.is_none() {
            let mut v = Vec::new();
            for entry in DOMAIN_BOOTSTRAP {
                for kw in entry.keywords {
                    v.push((String::from(*kw), entry.domain));
                }
            }
            DOMAIN_KEYWORD_MAP = Some(v);
        }
        DOMAIN_KEYWORD_MAP.as_mut().unwrap()
    }
}

/// Register a new keyword→domain mapping at runtime.
pub fn register_domain_keyword(keyword: &str, domain: &'static str) {
    let map = ensure_domain_map();
    let kw = keyword.to_lowercase();
    // Update if exists, else insert
    if let Some(entry) = map.iter_mut().find(|(k, _)| k.as_str() == kw) {
        entry.1 = domain;
    } else {
        map.push((String::from(kw), domain));
    }
}

/// Classify a theorem/ob3ect name into its domain type (dynamic lookup).
pub fn infer_domain(name: &str) -> &'static str {
    let nl = name.to_lowercase();
    let map = ensure_domain_map();
    for (keyword, domain) in map.iter() {
        if nl.contains(keyword.as_str()) {
            return domain;
        }
    }
    "symbolic"
}
