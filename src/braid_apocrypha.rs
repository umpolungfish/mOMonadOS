// ─── braid_apocrypha.rs ────────────────────────────────────────────────
// Targeted braid search (spec: braid-apocrypha).
//
// Instead of compiling a gate to a braid, search backward: given a target
// |Jones| value, enumerate braid words on the given strands by increasing
// length and return the first whose Jones magnitude lands within tolerance.
// Increasing length means the first hit is the shortest braid that realizes
// the target — a claim about the whole space, not about search order.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::fibonacci_qc::jones_polynomial;

fn word_of(code: usize, len: usize, gens: &[i32]) -> Vec<i32> {
    let base = gens.len();
    let mut c = code;
    let mut w = Vec::with_capacity(len);
    for _ in 0..len {
        w.push(gens[c % base]);
        c /= base;
    }
    w
}

pub fn braid_apocrypha_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    let mut target: Option<f64> = None;
    let mut strands = 3usize;
    let mut tol = 1e-3f64;
    let mut max_len = 6usize;
    let mut i = 0;
    while i < flat.len() {
        match flat[i] {
            "--target" => { target = flat.get(i + 1).and_then(|s| s.parse().ok()); i += 1; }
            "--strands" => { strands = flat.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(3); i += 1; }
            "--tol" => { tol = flat.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1e-3); i += 1; }
            "--max-len" => { max_len = flat.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(6); i += 1; }
            _ => {}
        }
        i += 1;
    }
    let target = match target {
        Some(t) => t,
        None => {
            return "braid-apocrypha --target <|Jones|> [--strands N] [--tol e] [--max-len L]\n\n\
                    Search braid words by increasing length for one whose Jones\n\
                    magnitude matches the target. The first hit is the shortest\n\
                    braid realizing it.\n\n\
                    Try:  braid-apocrypha --target 0.618034 --strands 3\n".to_string();
        }
    };
    // Generators for n strands: ±1 .. ±(n-1).
    let mut gens: Vec<i32> = Vec::new();
    for s in 1..strands as i32 { gens.push(s); gens.push(-s); }
    if gens.is_empty() { return "need at least 2 strands\n".to_string(); }

    let mut tested = 0usize;
    for len in 1..=max_len {
        let total = gens.len().pow(len as u32);
        for code in 0..total {
            let w = word_of(code, len, &gens);
            tested += 1;
            let j = jones_polynomial(strands, &w).norm();
            if (j - target).abs() <= tol {
                let mut ws = String::new();
                for (k, g) in w.iter().enumerate() {
                    if k > 0 { ws.push(' '); }
                    ws.push_str(&format!("{}", g));
                }
                let mut out = String::from("BRAID-APOCRYPHA\n===============\n\n");
                out.push_str(&format!("target |Jones|:  {:.6}\n", target));
                out.push_str(&format!("searched:        {} words, length 1..{}\n\n", tested, len));
                out.push_str(&format!("found braid:     [{}]   ({} crossings, {} strands)\n", ws, w.len(), strands));
                out.push_str(&format!("|Jones|:         {:.6}\n", j));
                out.push_str(&format!("residual:        {:.2e}\n", (j - target).abs()));
                out.push_str("\nshortest, because the hunt runs by increasing length and stops\n\
                              at the first match: nothing shorter realizes it.\n");
                return out;
            }
        }
    }
    format!(
        "BRAID-APOCRYPHA\n===============\n\ntarget |Jones|:  {:.6}\nsearched:        {} words, length 1..{}\n\n\
         no braid in the searched space realizes it within {:.0e}. Absence here\n\
         is a statement about length ≤ {}, not about all braids.\n",
        target, tested, max_len, tol, max_len
    )
}
