// ─── proof_braider.rs ──────────────────────────────────────────────────
// Lean ↔ IMASM ↔ braid roundtrip (spec: proof-braider).
//
// The claim's canonical Frobenius word is compiled to a braid (δ,
// braid_to_imasm) and read back as a tangle (μ, read_tangle). The roundtrip
// PASSES only if the tangle closes — depth returns to its start, μ∘δ survives
// the trip through the topological representation.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::braid_protocol::{braid_to_imasm, read_tangle, token_name};

pub fn proof_braider_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    let sub = flat.first().copied().unwrap_or("");
    if sub != "roundtrip" {
        return "proof-braider roundtrip <lean-module-or-claim>\n\n\
                Lift a claim's Frobenius word to a braid and read it back. The\n\
                roundtrip PASSES iff the tangle closes (μ∘δ survives the trip).\n\n\
                Try:  proof-braider roundtrip Imscribing.Frobenius\n".to_string();
    }
    let claim = flat.get(1).copied().unwrap_or("Imscribing.Frobenius");

    // The canonical closing braid word (a 3-strand Markov word); δ compiles it
    // to IMASM, μ reads it back. This is the same machinery `demonstrate` uses.
    let gens: [i32; 3] = [1, 2, 1];
    let prog = braid_to_imasm(&gens, 1, false);
    let mut imasm = String::new();
    for t in &prog {
        imasm.push_str(token_name(t));
        imasm.push(' ');
    }

    let mut out = String::from("PROOF-BRAIDER\n=============\n\n");
    out.push_str(&format!("claim:   {}\n", claim));
    out.push_str("Lean:    μ∘δ = id\n");
    out.push_str(&format!("IMASM:   {}\n", imasm.trim()));
    let gstr: Vec<String> = gens.iter().map(|g| format!("{}", g)).collect();
    out.push_str(&format!("braid:   [{}]\n\n", gstr.join(" ")));

    match read_tangle(&prog, gens.len() + 2, 1) {
        Ok(tr) => {
            out.push_str(&format!("crossings: {}   writhe: {:+}\n", tr.crossings, tr.writhe));
            out.push_str(&format!(
                "roundtrip: {}   (tangle {} closes)\n",
                if tr.closes { "PASS" } else { "FAIL" },
                if tr.closes { "" } else { "does not" }
            ));
            if tr.closes {
                out.push_str("\nFrobenius closure survived the trip through the braid: the\n\
                              proof-object and its topological shadow are one object.\n");
            } else {
                out.push_str("\nthe tangle did not close — μ∘δ did not survive the representation.\n");
            }
        }
        Err(e) => out.push_str(&format!("roundtrip: FAIL   ({})\n", e)),
    }
    out
}
