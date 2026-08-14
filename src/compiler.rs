// ─── compiler.rs ───────────────────────────────────────────────────────
// Compile between mathematical representations (build.txt §349).
//
// The one clean bridge is δ/μ between braids and IMASM. This compiles a braid
// (source) to IMASM, to its Jones invariant, or to the Lean closure statement;
// and compiles a token-name sequence back to a braid (μ, read_tangle). Token
// NAMES are used for the imasm side, never glyphs — the glyph→token ordering is
// a known trap and names are unambiguous.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::braid_protocol::{braid_to_imasm, parse_token_name, read_tangle, token_name};
use crate::fibonacci_qc::jones_polynomial;

fn parse_ints(items: &[&str]) -> Vec<i32> {
    items.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

pub fn compiler_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    // Split at "--to".
    let to_pos = flat.iter().position(|&s| s == "--to");
    let target = to_pos.and_then(|i| flat.get(i + 1).copied());
    let src_end = to_pos.unwrap_or(flat.len());
    let src = &flat[..src_end];

    if src.len() < 2 || target.is_none() {
        return "compiler <source> --to <target>\n\n\
                sources:  braid <ints...>        e.g.  braid 1 2 1\n\
                          imasm <NAMES...>       e.g.  imasm VINIT FSPLIT FFUSE TANCH\n\
                targets:  imasm | jones | lean | braid\n\n\
                Braids compile to imasm/jones/lean; a token-name word compiles\n\
                to a braid (μ, read_tangle). Names, never glyphs.\n\n\
                Try:  compiler braid 1 2 1 --to imasm\n".to_string();
    }
    let kind = src[0];
    let target = target.unwrap();
    let mut out = String::from("COMPILER\n========\n\n");

    match (kind, target) {
        ("braid", "imasm") => {
            let gens = parse_ints(&src[1..]);
            let prog = braid_to_imasm(&gens, 1, false);
            let names: Vec<&str> = prog.iter().map(|t| token_name(t)).collect();
            out.push_str(&format!("braid → imasm:  {}\n", names.join(" ")));
            out.push_str("certificate:    δ (braid_to_imasm), closure-preserving\n");
        }
        ("braid", "jones") => {
            let gens = parse_ints(&src[1..]);
            let j = jones_polynomial(3, &gens);
            out.push_str(&format!("braid → Jones:  {:.6} + {:.6}i   |·|={:.6}\n", j.re, j.im, j.norm()));
        }
        ("braid", "lean") => {
            let gens = parse_ints(&src[1..]);
            let prog = braid_to_imasm(&gens, 1, false);
            let closes = read_tangle(&prog, gens.len() + 2, 1).map(|t| t.closes).unwrap_or(false);
            out.push_str("braid → Lean:   theorem: μ∘δ = id\n");
            out.push_str(&format!("closure:        {}\n", if closes { "PASS (tangle closes)" } else { "FAIL (tangle open)" }));
        }
        ("imasm", "braid") => {
            let toks: Option<Vec<_>> = src[1..].iter().map(|n| parse_token_name(n)).collect();
            match toks {
                Some(prog) => match read_tangle(&prog, src.len() + 1, 1) {
                    Ok(tr) => {
                        let g: Vec<String> = tr.generators.iter().map(|x| format!("{}", x)).collect();
                        out.push_str(&format!("imasm → braid:  [{}]   ({} crossings)\n", g.join(" "), tr.crossings));
                        out.push_str(&format!("certificate:    μ (read_tangle), closes: {}\n", tr.closes));
                    }
                    Err(e) => out.push_str(&format!("imasm → braid:  FAIL ({})\n", e)),
                },
                None => out.push_str("one of the token names did not parse.\n"),
            }
        }
        _ => out.push_str(&format!("no route: {} --to {}\n", kind, target)),
    }
    out
}
