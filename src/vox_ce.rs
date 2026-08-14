// ─── vox_ce.rs ─────────────────────────────────────────────────────────
// Closure auditor for compiled code (spec: vox-ce).
//
// Lift a hex bytecode body into an IMASM word — each byte selects one of the
// twelve opcode marks — and verdict its control-flow closure through the same
// tri-ancestral reader the rest of the kernel uses. An open fork surfaces as a
// non-holding register: a place the control flow splits and never rejoins.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::counterfactual::{read, MARKS};
use crate::ctc_loom::verdict_of;

fn lift(kind: &str, hex: &str) -> Option<String> {
    // Accept optional 0x, ignore non-hex. Two hex chars = one byte = one mark.
    let clean: Vec<u8> = hex
        .trim_start_matches("0x")
        .bytes()
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        })
        .collect();
    if clean.is_empty() { return None; }
    let mut word = String::new();
    // EVM and WASM lift the same way here: this is a structural CFG shadow, not a
    // decoder — the kind label only records provenance.
    let _ = kind;
    for pair in clean.chunks(2) {
        let byte = (pair[0] as usize) * 16 + pair.get(1).map(|&x| x as usize).unwrap_or(0);
        word.push(MARKS[byte % 12]);
    }
    Some(word)
}

pub fn vox_ce_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    if flat.len() < 2 {
        return "vox-ce <evm|wasm> <hex>\n\n\
                Lift compiled bytecode into an IMASM word and verdict its\n\
                control-flow closure. B = an open fork across a commit; a\n\
                non-holding register names where the flow splits and never fuses.\n\n\
                Try:  vox-ce evm 0x600160025b00\n".to_string();
    }
    let (kind, hex) = (flat[0], flat[1]);
    let word = match lift(kind, hex) {
        Some(w) => w,
        None => return "no hex bytes found\n".to_string(),
    };
    let mut out = String::from("VOX-CE\n======\n\n");
    out.push_str(&format!("kind:    {}\n", kind));
    out.push_str(&format!("lifted:  {}\n", word));
    match read(&word) {
        Some(r) => {
            out.push_str(&format!("register:     {}\n", r.register));
            out.push_str(&format!("closed walk:  {}\n", r.holds && !r.vacuous));
            let verdict = verdict_of(&word).map(|v| v.name()).unwrap_or("?");
            out.push_str(&format!("verdict:      {}", verdict));
            match verdict {
                "B" => out.push_str("   (open fork across a commit — reentrancy/leak shape)\n"),
                "T" => out.push_str("   (control flow closes)\n"),
                "N" => out.push_str("   (linear, no fork)\n"),
                _ => out.push_str("\n"),
            }
        }
        None => out.push_str("the lift produced no readable word.\n"),
    }
    out
}
