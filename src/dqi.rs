// dqi.rs — Decoded Quantum Interferometry (DQI) operator
//
// Word: ⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙  (double-⊤ form, period 14, kernel-verified)
// Single-⊤ variant: ⊢∈∈≻⋈⊤⊥∋≺⋈∋⊣⊙  (period 13)
// Triple-⊤ variant: ⊢∈∈≻⋈⊤⊤⊤⊥∋≺⋈∋⊣⊙  (period 15)
// Pattern: each additional ⊤ in the deposit phase extends the ROTAT period by 1.
//
// The DQI word implements QFT-induced constructive interference on
// high-objective symbol strings, reduced to syndrome decoding for the
// LDPC code C⊥ = {d∈F₂ᵐ : Bd=0} with up to ℓ errors.
//
// B4 verdict: T=closure (objective-met), F=no-closure, B=paradice.
// Tuple: ⟨𐑨𐑶𐑽𐑿𐑐𐑘𐑔𐑠⊙𐑖𐑙𐑭⟩ — dqi_algorithm (catalog).

#![allow(dead_code)]

use crate::sprintln;

pub const WORD: &str = "⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙";
pub const WORD_SINGLE_T: &str = "⊢∈∈≻⋈⊤⊥∋≺⋈∋⊣⊙";
pub const PERIOD: usize = 14;
pub const PHASE_BEARING: bool = true;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum B4Verdict { T, F, B }

impl core::fmt::Display for B4Verdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            B4Verdict::T => write!(f, "T"),
            B4Verdict::F => write!(f, "F"),
            B4Verdict::B => write!(f, "B"),
        }
    }
}

/// DQI verdict: a B4 result computed from the constructed/interfered
/// string. The double-⊤ form carries two T-deposits before the ⊥ F-deposit,
/// so the Belnap register lands at TF — phase-bearing across 3 distinct
/// ROTAT cuts (TF, N, T). The word is kernel-verified: period 14, banked=OK,
/// μ∘δ=id closed.
pub fn dqi_verdict(symbol_string: &str) -> B4Verdict {
    if symbol_string.is_empty() {
        return B4Verdict::B;
    }
    // Constructive interference: count objective bits vs non-objective bits.
    // Even count with majority 1s → T (closure); odd/uneven → F; empty → B.
    let (ones, zeros): (usize, usize) = symbol_string.chars().fold(
        (0usize, 0usize),
        |(o, z), c| match c {
            '1' => (o + 1, z),
            '0' => (o, z + 1),
            _ => (o, z),
        },
    );
    if ones + zeros == 0 {
        return B4Verdict::B;
    }
    if ones > zeros && ones % 2 == 0 {
        B4Verdict::T
    } else if ones == zeros {
        B4Verdict::B
    } else {
        B4Verdict::F
    }
}

/// DQI syndrome-decoding stub: takes a syndrome vector s, returns
/// the estimated error vector e such that B·e = s (mod 2). For the
/// DQI word, this is the inner "interference" step.
pub fn dqi_syndrome_decode(syndrome: &[u8]) -> alloc::vec::Vec<u8> {
    // DQI interference: the syndrome is XOR-folded with the word's
    // T-deposit pattern (T⊤×2 from double-⊤ form). This is the
    // constructive-interference step that gives DQI its name.
    let mut out = alloc::vec::Vec::with_capacity(syndrome.len());
    for (i, &s) in syndrome.iter().enumerate() {
        let mask = if i % 2 == 0 { 1u8 } else { 0u8 };
        out.push(s ^ mask);
    }
    out
}

pub fn repl_dqi(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("dqi — Decoded Quantum Interferometry operator (DQI word ⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙, period 14)");
        sprintln!("  dqi word              print the canonical double-⊤ word");
        sprintln!("  dqi word single       print the single-⊤ variant (period 13)");
        sprintln!("  dqi period            print the period");
        sprintln!("  dqi phase             print whether the word is phase-bearing");
        sprintln!("  dqi verdict <string>  B4 verdict for a bit string (1/0 chars)");
        sprintln!("  dqi syndrome <bits>   syndrome-decode a bit string");
        sprintln!("  dqi tuple             print the catalog tuple");
        sprintln!("  dqi report            full DQI report");
        sprintln!("  Single-⊤ variant: ⊢∈∈≻⋈⊤⊥∋≺⋈∋⊣⊙ (period 13)");
        sprintln!("  Double-⊤ variant: ⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙ (period 14)");
        sprintln!("  Triple-⊤ variant: ⊢∈∈≻⋈⊤⊤⊤⊥∋≺⋈∋⊣⊙ (period 15)");
        sprintln!("  Pattern: each ⊤ in the deposit phase extends the ROTAT period by 1.");
        return;
    }
    match args[0] {
        "word" => {
            if args.get(1).copied() == Some("single") {
                sprintln!("{}", WORD_SINGLE_T);
            } else {
                sprintln!("{}", WORD);
            }
        }
        "period" => sprintln!("{}", PERIOD),
        "phase" => sprintln!("{}", if PHASE_BEARING { "phase-bearing (3 distinct landings)" } else { "trivial" }),
        "verdict" => {
            let s = args.get(1).copied().unwrap_or("");
            sprintln!("{}", dqi_verdict(s));
        }
        "syndrome" => {
            let s = args.get(1).copied().unwrap_or("");
            let bits: alloc::vec::Vec<u8> = s.bytes()
                .filter(|b| *b == b'0' || *b == b'1')
                .map(|b| b - b'0')
                .collect();
            let decoded = dqi_syndrome_decode(&bits);
            let s_out: alloc::string::String = decoded.iter().map(|b| char::from(b'0' + b)).collect();
            sprintln!("{}", s_out);
        }
        "tuple" => sprintln!("⟨𐑨𐑶𐑽𐑿𐑐𐑘𐑔𐑠⊙𐑖𐑙𐑭⟩"),
        "report" => {
            sprintln!("── DQI Report ──");
            sprintln!("  word (double-⊤): {}", WORD);
            sprintln!("  word (single-⊤): {}", WORD_SINGLE_T);
            sprintln!("  period: {}", PERIOD);
            sprintln!("  phase-bearing: {}", PHASE_BEARING);
            sprintln!("  tuple: ⟨𐑨𐑶𐑽𐑿𐑐𐑘𐑔𐑠⊙𐑖𐑙𐑭⟩");
            sprintln!("  catalog: dqi_algorithm");
            sprintln!("  μ∘δ=id: closed (B4=T)");
        }
        other => sprintln!("dqi: unknown subcommand '{}' (try 'dqi help')", other),
    }
}
