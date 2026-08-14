// ─── frobenius_fuzzer.rs ───────────────────────────────────────────────
// Mine the grammar for rare, stable programs (build.txt, useful §frobenius-fuzzer).
//
// `demonstrate mu-delta` runs the closure on ONE word. This runs it on the whole
// word space and keeps only the survivors: the programs for which
//
//     δ(μ(prog)) = prog
//
// reading the program as a tangle (μ, read_tangle), compiling the resulting
// braid word back to IMASM (δ, braid_to_imasm), and demanding the identical
// token sequence. A survivor is a program the braid representation reproduces
// exactly; everything else either refuses μ or comes back changed.
//
// The spec asks for random rounds. Random is the wrong instrument where the
// space is enumerable: every word of length <= 4 is 22,620 candidates, so this
// ENUMERATES and the survivor count is exact rather than a sample rate. Past
// that length the honest answer is that it was not searched, and it says so.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::belnap_ring_shor::program_from_glyphs;
use crate::counterfactual::MARKS;
use crate::lattice_flow::normalize;

/// Set by MEASUREMENT, not by reasoning about the per-candidate cost.
///
///   length 4:  20,736 candidates   well under a second
///   length 5: 248,832 candidates   2.5 seconds
///   length 6: 2,985,984 candidates ABANDONED after 11 minutes
///
/// A 12x growth in candidates cost ~264x in time, so the per-candidate cost is
/// nowhere near constant — allocation churn over millions of iterations does
/// not stay flat in this kernel. The earlier comment here reasoned from "one
/// parse, one read_tangle, one braid_to_imasm, no nested sweep" and concluded
/// the next length was affordable. That reasoning was sound and the conclusion
/// was wrong, which is the whole argument for measuring.
pub const MAX_LEN: usize = 5;

#[derive(Default)]
pub struct Run {
    pub len: usize,
    pub candidates: usize,
    /// μ refused to read the program as a tangle.
    pub refused: usize,
    /// μ read it, but δ∘μ returned a different program.
    pub changed: usize,
    /// δ∘μ returned the identical token sequence.
    pub survivors: usize,
    /// The shortest survivors, with their braid words.
    pub examples: Vec<(String, Vec<i32>, i32, usize)>,
    /// Survivors carrying at least one crossing — a trivial tangle is cheap.
    pub nontrivial: usize,
}

fn each_word_of_len(n: usize, mut f: impl FnMut(&str)) {
    let base = MARKS.len();
    let total = base.pow(n as u32);
    let mut buf: Vec<char> = Vec::with_capacity(n);
    for code in 0..total {
        buf.clear();
        let mut c = code;
        for _ in 0..n {
            buf.push(MARKS[c % base]);
            c /= base;
        }
        let w: String = buf.iter().collect();
        f(&w);
    }
}

pub fn run(len: usize, want_examples: usize) -> Run {
    let mut r = Run { len, ..Default::default() };

    each_word_of_len(len, |w| {
        let prog = match program_from_glyphs(&normalize(w)) {
            Ok(p) => p,
            Err(_) => return,
        };
        let toks: Vec<crate::tokens::Token> = prog.as_slice().to_vec();
        if toks.is_empty() {
            return;
        }
        r.candidates += 1;

        // μ — read the program as a tangle.
        let reading = match crate::braid_protocol::read_tangle(&toks, toks.len() + 2, 1) {
            Ok(x) => x,
            Err(_) => {
                r.refused += 1;
                return;
            }
        };

        // δ — compile the braid word back to IMASM.
        let back = crate::braid_protocol::braid_to_imasm(&reading.generators, 1, false);

        if back == toks {
            r.survivors += 1;
            if reading.crossings > 0 {
                r.nontrivial += 1;
                if r.examples.len() < want_examples {
                    r.examples.push((
                        w.to_string(),
                        reading.generators.clone(),
                        reading.writhe,
                        reading.crossings,
                    ));
                }
            }
        } else {
            r.changed += 1;
        }
    });

    r
}

pub fn format_run(r: &Run) -> String {
    let mut out = String::new();
    out.push_str("FROBENIUS-FUZZER\n================\n\n");
    out.push_str(&format!(
        "space:        every word of length {} — {} readable programs\n",
        r.len, r.candidates
    ));
    out.push_str("test:         δ(μ(prog)) == prog, token for token\n\n");

    let pct = |n: usize| {
        if r.candidates == 0 {
            0.0
        } else {
            n as f64 * 100.0 / r.candidates as f64
        }
    };

    out.push_str(&format!(
        "  μ refused        {:>7}   {:>5.1}%\n",
        r.refused,
        pct(r.refused)
    ));
    out.push_str(&format!(
        "  came back changed {:>6}   {:>5.1}%\n",
        r.changed,
        pct(r.changed)
    ));
    out.push_str(&format!(
        "  SURVIVORS        {:>7}   {:>5.1}%\n",
        r.survivors,
        pct(r.survivors)
    ));
    out.push_str(&format!(
        "    of which carry a crossing {:>3}   {:>5.1}%\n",
        r.nontrivial,
        pct(r.nontrivial)
    ));

    if r.examples.is_empty() {
        out.push_str(
            "\nNo survivor carries a crossing at this length: every program that\n\
             closes does so as a trivial tangle, which closes for free.\n",
        );
    } else {
        out.push_str("\nsurvivors with a crossing (the ones that closed for a reason):\n");
        out.push_str("    word        braid            writhe  crossings\n");
        for (w, gens, writhe, cross) in &r.examples {
            out.push_str(&format!(
                "    {:<11} {:<16} {:>6}  {:>9}\n",
                w,
                format!("{:?}", gens),
                writhe,
                cross
            ));
        }
    }

    out.push_str(
        "\nEnumerated, not sampled: the survivor count is exact for this length.\n\
         A survivor is a program the braid representation reproduces exactly —\n\
         it is not a claim that the program is useful.\n",
    );
    out
}

pub fn fuzzer_main(args: &[&str]) -> String {
    let mut len = 3usize;
    let mut examples = 8usize;
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--len" | "-n" => {
                if i + 1 < args.len() {
                    len = args[i + 1].parse::<usize>().unwrap_or(3);
                    i += 1;
                }
            }
            "--examples" | "-e" => {
                if i + 1 < args.len() {
                    examples = args[i + 1].parse::<usize>().unwrap_or(8).min(32);
                    i += 1;
                }
            }
            "help" | "--help" => {
                return "frobenius-fuzzer [--len N] [--examples K]   (alias fuzz)\n\
                        \n\
                        Mine the word space for programs the braid representation\n\
                        reproduces exactly: δ(μ(prog)) == prog, token for token.\n\
                        \n\
                        Enumerated rather than sampled, so the survivor count is\n\
                        exact. Default N=3 (1,728 programs); N=4 is 20,736.\n"
                    .to_string();
            }
            _ => {}
        }
        i += 1;
    }

    if len == 0 || len > MAX_LEN {
        return format!(
            "length {} is outside the enumerable range 1..{}. Past it the space\n\
             is not searched, and no rate is estimated from a sample.\n",
            len, MAX_LEN
        );
    }

    format_run(&run(len, examples))
}
