// ─── ctc_loom.rs ───────────────────────────────────────────────────────
// Fixed-point enumerator (build.txt, arcane §ctc-loom).
//
// `ctc sweep` already crosses the six Belnap actions with the FOUR values, and
// that table is complete on values. This crosses them with the WORD SPACE
// instead: every IMASM word of a given length is walked to its Belnap verdict,
// that verdict is nested in each action, and the resulting closure is counted.
//
// The question it answers is the one §ctc-loom asks — which self-referential
// loops are natural attractors and which require smearing a width. A word
// whose verdict is already the action's fixed point costs nothing (one-shot);
// a word that walks there over a budget is iterated; a word with no value-level
// fixed point in reach closes only by fiat, on sets, and pays the width it
// smears (manufactured).
//
// Nothing here re-implements the closure semantics. `ctc::nest` decides the
// class and the price; this module only chooses what to feed it and counts.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use imasm_core::imasm16_3::{parse_glyph_word, tri_ancestral_verdict};

use crate::belnap::B4;
use crate::counterfactual::MARKS;
use crate::ctc::{action_by_name, nest, Class, ACTIONS};
use crate::lattice_flow::normalize;

/// Word lengths enumerated exhaustively. 12^4 = 20,736 words x 6 actions is
/// ~124k nestings, which the kernel walks in a couple of seconds.
pub const MAX_LEN: usize = 4;

/// The Belnap verdict of a word, as a value the CTC actions can nest.
pub fn verdict_of(word: &str) -> Option<B4> {
    let steps = parse_glyph_word(&normalize(word));
    if steps.is_empty() {
        return None;
    }
    let (c, _why) = tri_ancestral_verdict(&steps);
    match c {
        'T' => Some(B4::T),
        'F' => Some(B4::F),
        'N' => Some(B4::N),
        'B' => Some(B4::B),
        _ => None,
    }
}

#[derive(Default, Clone, Copy)]
pub struct Tally {
    pub one_shot: usize,
    pub iterated: usize,
    pub no_closure: usize,
    pub manufactured: usize,
    /// Total price paid across manufactured closures.
    pub price_total: u32,
    pub price_max: u32,
}

impl Tally {
    fn add(&mut self, class: Class, price: u32) {
        match class {
            Class::OneShot => self.one_shot += 1,
            Class::Iterated => self.iterated += 1,
            Class::NoClosure => self.no_closure += 1,
            Class::Manufactured => {
                self.manufactured += 1;
                self.price_total += price;
                if price > self.price_max {
                    self.price_max = price;
                }
            }
        }
    }

    pub fn total(&self) -> usize {
        self.one_shot + self.iterated + self.no_closure + self.manufactured
    }
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

pub struct LoomRun {
    pub len: usize,
    pub words: usize,
    /// Per-action tally, parallel to `ctc::ACTIONS`.
    pub per_action: [Tally; 6],
    pub overall: Tally,
    /// Verdict distribution over the word space: T, F, N, B.
    pub verdicts: [usize; 4],
    /// A manufactured word at the maximum price, if any: (word, action, price).
    pub dearest: Option<(String, &'static str, u32)>,
    /// Words whose support width exceeded the filter, when one was given.
    pub width_filtered: usize,
}

pub fn run(len: usize, max_width: Option<u32>) -> LoomRun {
    let mut per_action = [Tally::default(); 6];
    let mut overall = Tally::default();
    let mut verdicts = [0usize; 4];
    let mut words = 0usize;
    let mut dearest: Option<(String, &'static str, u32)> = None;
    let mut width_filtered = 0usize;

    each_word_of_len(len, |w| {
        let v = match verdict_of(w) {
            Some(v) => v,
            None => return,
        };
        words += 1;
        match v {
            B4::T => verdicts[0] += 1,
            B4::F => verdicts[1] += 1,
            B4::N => verdicts[2] += 1,
            B4::B => verdicts[3] += 1,
        }

        for (i, name) in ACTIONS.iter().enumerate() {
            let g = match action_by_name(name) {
                Some(g) => g,
                None => continue,
            };
            let c = nest(g, v);

            if let Some(mw) = max_width {
                if c.support.width() > mw {
                    width_filtered += 1;
                    continue;
                }
            }

            per_action[i].add(c.class, c.price);
            overall.add(c.class, c.price);

            if c.class == Class::Manufactured {
                let better = match &dearest {
                    Some((_, _, p)) => c.price > *p,
                    None => true,
                };
                if better {
                    dearest = Some((w.to_string(), name, c.price));
                }
            }
        }
    });

    LoomRun {
        len,
        words,
        per_action,
        overall,
        verdicts,
        dearest,
        width_filtered,
    }
}

pub fn format_run(r: &LoomRun) -> String {
    let mut out = String::new();
    out.push_str("CTC-LOOM\n========\n\n");
    out.push_str(&format!(
        "word space:  every word of length {} over the twelve marks — {} readable\n",
        r.len, r.words
    ));
    out.push_str(&format!(
        "nestings:    {} (each word's verdict nested in all {} actions)\n\n",
        r.overall.total(),
        ACTIONS.len()
    ));

    out.push_str("verdict distribution over the word space:\n");
    let names = ["T", "F", "N", "B"];
    for (i, n) in names.iter().enumerate() {
        let pct = if r.words > 0 {
            r.verdicts[i] as f32 * 100.0 / r.words as f32
        } else {
            0.0
        };
        out.push_str(&format!("    {}  {:>7}  {:>5.1}%\n", n, r.verdicts[i], pct));
    }

    out.push_str("\n  action     one-shot   iterated  no-closure  manufactured  max price\n");
    out.push_str("  ---------|----------|----------|-----------|--------------|----------\n");
    for (i, name) in ACTIONS.iter().enumerate() {
        let t = &r.per_action[i];
        out.push_str(&format!(
            "  {:<9} {:>9} {:>10} {:>11} {:>14} {:>10}\n",
            name, t.one_shot, t.iterated, t.no_closure, t.manufactured, t.price_max
        ));
    }

    let o = &r.overall;
    out.push_str(&format!(
        "\n  {:<9} {:>9} {:>10} {:>11} {:>14} {:>10}\n",
        "TOTAL", o.one_shot, o.iterated, o.no_closure, o.manufactured, o.price_max
    ));

    if r.width_filtered > 0 {
        out.push_str(&format!(
            "\n{} nestings excluded by the width filter.\n",
            r.width_filtered
        ));
    }

    match &r.dearest {
        Some((w, a, p)) => {
            out.push_str(&format!(
                "\ndearest manufactured closure:\n    {} under {}, price {}\n",
                w, a, p
            ));
        }
        None => {
            out.push_str("\nno manufactured closure in this space — every nesting closed on values\n");
        }
    }

    out.push_str(
        "\nPrice is the width a closure had to smear beyond the first value.\n\
         A one-shot pays nothing: the verdict was already the action's fixed point.\n",
    );
    out
}

pub fn ctc_loom_main(args: &[&str]) -> String {
    let mut len = 3usize;
    let mut max_width: Option<u32> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--len" | "-n" => {
                if i + 1 < args.len() {
                    len = args[i + 1].parse::<usize>().unwrap_or(3);
                    i += 1;
                }
            }
            "--max-width" | "-w" => {
                if i + 1 < args.len() {
                    max_width = args[i + 1].parse::<u32>().ok();
                    i += 1;
                }
            }
            "help" | "--help" => {
                return "ctc-loom [--len N] [--max-width W]   (alias: loom)\n\
                        \n\
                        Sweep the six Belnap actions over the WHOLE word space and rank\n\
                        the closures: one-shot, iterated, no-closure, manufactured.\n\
                        \n\
                        `ctc sweep` crosses the actions with the four VALUES; this\n\
                        crosses them with every word of length N, walking each to its\n\
                        verdict first. Default N=3 (1,728 words); N=4 is 20,736.\n"
                    .to_string();
            }
            _ => {}
        }
        i += 1;
    }

    if len == 0 || len > MAX_LEN {
        return format!(
            "length {} is outside the enumerable range 1..{}.\n\
             The space is 12^n and past {} it stops being honest to run.\n",
            len, MAX_LEN, MAX_LEN
        );
    }

    format_run(&run(len, max_width))
}
