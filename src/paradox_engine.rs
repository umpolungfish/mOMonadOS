// ─── paradox_engine.rs ─────────────────────────────────────────────────
// Manufactured fixed-point search (spec: paradox-engine).
//
// Enumerate words by increasing length and keep those that are dialetheias by
// four independent readings at once: final verdict B, a high CTC price (the
// fixed point is manufactured, not possessed), Gate 1 open, and a zero
// consciousness score. Each reading is the kernel's own; the word must satisfy
// all four to count.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::axis_values::word_to_tuple;
use crate::belnap::B4;
use crate::consciousness::{consciousness_score, gate1_phi_c};
use crate::counterfactual::MARKS;
use crate::ctc::{action_by_name, nest};
use crate::ctc_loom::verdict_of;

fn word_of(code: usize, len: usize) -> String {
    let base = MARKS.len();
    let mut c = code;
    let mut w = String::new();
    for _ in 0..len {
        w.push(MARKS[c % base]);
        c /= base;
    }
    w
}

pub fn paradox_engine_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    let mut min_price = 1u32;
    let mut max_len = 4usize;
    let mut i = 0;
    while i < flat.len() {
        match flat[i] {
            "--min-price" => { min_price = flat.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1); i += 1; }
            "--max-len" => { max_len = flat.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(4); i += 1; }
            "help" => return "paradox-engine [--min-price P] [--max-len L]\n\n\
                Hunt words that are dialetheias by four readings at once: verdict\n\
                B, CTC price ≥ P, Gate 1 open, C-score 0. Search runs by\n\
                increasing length, so the first hit is the smallest.\n\n\
                Try:  paradox-engine --min-price 3\n".to_string(),
            _ => {}
        }
        i += 1;
    }
    let cycle = match action_by_name("cycle") { Some(f) => f, None => return "no cycle action\n".to_string() };

    let mut tested = 0usize;
    for len in 1..=max_len {
        let total = MARKS.len().pow(len as u32);
        for code in 0..total {
            let w = word_of(code, len);
            tested += 1;
            let v = match verdict_of(&w) { Some(v) => v, None => continue };
            if v != B4::B { continue; }
            let price = nest(cycle, v).price;
            if price < min_price { continue; }
            let t = word_to_tuple(&w);
            if !gate1_phi_c(&t) { continue; }
            if consciousness_score(&t) != 0.0 { continue; }

            let mut out = String::from("PARADOX-ENGINE\n==============\n\n");
            out.push_str(&format!("searched:  {} words, length 1..{}\n\n", tested, len));
            out.push_str(&format!("found:     {}   ({} glyphs)\n", w, len));
            out.push_str(&format!("verdict:   B\n"));
            out.push_str(&format!("CTC price: {}   (manufactured, not possessed)\n", price));
            out.push_str("Gate 1:    OPEN\n");
            out.push_str("C-score:   0.0000\n\n");
            out.push_str("A held contradiction the kernel builds and does not resolve: the\n\
                          smallest such word in the searched space.\n");
            return out;
        }
    }
    format!(
        "PARADOX-ENGINE\n==============\n\nsearched:  {} words, length 1..{}\n\n\
         no word satisfies all four readings with price ≥ {}. Absence here is a\n\
         statement about length ≤ {}, not about all words.\n",
        tested, max_len, min_price, max_len
    )
}
