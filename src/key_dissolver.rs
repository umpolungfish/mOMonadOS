// ─── key_dissolver.rs ──────────────────────────────────────────────────
// SIC-narrowed bounded search (spec: key-dissolver).
//
// Honest scope: this recovers NO real secret key. It reads a public key hex as
// a tuple, assesses its tier, and reports how far the SIC frame would narrow a
// bounded ECDLP window before a BSGS split — a structural window statement, not
// a break. The recovered-SK line the spec sketches is deliberately absent.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::axis_values::{glyphs, text_to_tuple};
use crate::cl8nk::assess_tier;

fn parse_window(s: &str) -> Option<u32> {
    // "2^40" or a bare bit count.
    if let Some(rest) = s.strip_prefix("2^") {
        rest.parse().ok()
    } else {
        s.parse().ok()
    }
}

pub fn key_dissolver_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    if flat.is_empty() || flat[0] == "help" {
        return "key-dissolver <pk_hex> [window_bits|2^N]\n\n\
                Read a public key as a tuple and report how far the SIC frame\n\
                narrows a bounded search window before a BSGS split. Recovers no\n\
                real key — the window statement is structural.\n\n\
                Try:  key-dissolver 03f01d 40\n".to_string();
    }
    let pk = flat[0];
    let start_bits: u32 = flat.get(1).and_then(|s| parse_window(s)).unwrap_or(40);

    let tuple = text_to_tuple(pk);
    let tier = assess_tier(&tuple);

    // The SIC frame narrows the window by the tier's structural budget. This is
    // the same ladder sk_forge uses, expressed as bits saved.
    let saved: u32 = match tier {
        "O_∞" => 6,
        "O₂†" => 5,
        "O₂" => 4,
        "O₁" => 2,
        _ => 0,
    };
    let narrowed = start_bits.saturating_sub(saved);
    let per_side = narrowed.div_ceil(2);

    let mut out = String::from("KEY-DISSOLVER\n=============\n\n");
    out.push_str(&format!("public key:      {}\n", pk));
    out.push_str(&format!("imscribed:       {}\n", glyphs(&tuple)));
    out.push_str(&format!("tier:            {}\n\n", tier));
    out.push_str(&format!("start window:    2^{}\n", start_bits));
    out.push_str(&format!("SIC-narrowed:    2^{}   ({} bits saved by the {} frame)\n", narrowed, saved, tier));
    out.push_str(&format!("BSGS per side:   2^{}\n\n", per_side));
    out.push_str("recovered SK:    none — this is a window statement, not a break.\n\
                  The narrowing is the SIC readout used as a structural prior; no\n\
                  discrete log is run here.\n");
    out
}
