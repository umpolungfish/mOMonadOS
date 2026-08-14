// ─── loss.rs ───────────────────────────────────────────────────────────
// Quantify exactly what a transformation destroys (build.txt §384).
//
// Information destruction as an API primitive. Every figure below is counted
// over the ENUMERATED word space, never estimated: the words of length n are
// walked to their Belnap verdicts, the action is applied, and the two
// distributions are compared.
//
//   input entropy      H over the verdict distribution before the action
//   output entropy     H after it
//   bits destroyed     H_in - H_out, which is >= 0 for a deterministic map
//   collapsed states   verdicts that shared an image with another verdict
//   merged classes     distinct verdicts in, distinct verdicts out
//   irreversible       the ordered pairs that became indistinguishable
//
// A deterministic map on a finite set cannot create entropy, so a positive
// "bits destroyed" is the honest sign and a negative one would be a bug. The
// tier ladder's non-monotonicity lives one level up and is NOT asserted here:
// this measures the value map only, and says so.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::belnap::B4;
use crate::counterfactual::MARKS;
use crate::ctc::{action_by_name, ACTIONS};
use crate::ctc_loom::verdict_of;

pub const MAX_LEN: usize = 4;

fn h_bits(counts: &[usize; 4], total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    let mut h = 0.0f64;
    for &c in counts.iter() {
        if c > 0 {
            let p = c as f64 / total as f64;
            h -= p * libm::log2(p);
        }
    }
    h
}

fn idx(v: B4) -> usize {
    match v {
        B4::T => 0,
        B4::F => 1,
        B4::N => 2,
        B4::B => 3,
    }
}

const VNAME: [&str; 4] = ["T", "F", "N", "B"];

pub struct Loss {
    pub action: String,
    pub len: usize,
    pub words: usize,
    pub before: [usize; 4],
    pub after: [usize; 4],
    pub h_in: f64,
    pub h_out: f64,
    /// Which verdicts map where: image[i] is the image of value i.
    pub image: [Option<usize>; 4],
    /// Ordered pairs (a,b), a<b, that became indistinguishable.
    pub merged_pairs: Vec<(&'static str, &'static str, &'static str)>,
    pub distinct_in: usize,
    pub distinct_out: usize,
}

pub fn measure(action_name: &str, len: usize) -> Option<Loss> {
    let g = action_by_name(action_name)?;

    let mut before = [0usize; 4];
    let mut after = [0usize; 4];
    let mut words = 0usize;

    let base = MARKS.len();
    let total = base.pow(len as u32);
    let mut buf: Vec<char> = Vec::with_capacity(len);
    for code in 0..total {
        buf.clear();
        let mut c = code;
        for _ in 0..len {
            buf.push(MARKS[c % base]);
            c /= base;
        }
        let w: String = buf.iter().collect();
        if let Some(v) = verdict_of(&w) {
            words += 1;
            before[idx(v)] += 1;
            after[idx(g(v))] += 1;
        }
    }

    // The map on values, and the pairs it fuses.
    let all = [B4::T, B4::F, B4::N, B4::B];
    let mut image: [Option<usize>; 4] = [None; 4];
    for v in all {
        image[idx(v)] = Some(idx(g(v)));
    }
    let mut merged_pairs = Vec::new();
    for i in 0..4 {
        for j in (i + 1)..4 {
            // Only count a fusion when BOTH values actually occur: two verdicts
            // that never appear in this space were never distinguishable here,
            // so calling their collapse a loss would be inventing one.
            if before[i] > 0 && before[j] > 0 && image[i] == image[j] {
                let to = VNAME[image[i].unwrap()];
                merged_pairs.push((VNAME[i], VNAME[j], to));
            }
        }
    }

    let distinct_in = before.iter().filter(|c| **c > 0).count();
    let distinct_out = after.iter().filter(|c| **c > 0).count();

    Some(Loss {
        action: action_name.to_string(),
        len,
        words,
        before,
        after,
        h_in: h_bits(&before, words),
        h_out: h_bits(&after, words),
        image,
        merged_pairs,
        distinct_in,
        distinct_out,
    })
}

pub fn format_loss(l: &Loss) -> String {
    let mut out = String::new();
    out.push_str("LOSS\n====\n\n");
    out.push_str(&format!("operation:   {}\n", l.action));
    out.push_str(&format!(
        "object:      every word of length {} — {} readable\n\n",
        l.len, l.words
    ));

    out.push_str("            before      after\n");
    for i in 0..4 {
        out.push_str(&format!(
            "    {}    {:>8}   {:>8}\n",
            VNAME[i], l.before[i], l.after[i]
        ));
    }

    let destroyed = l.h_in - l.h_out;
    out.push_str(&format!("\ninput entropy:    {:.4} bits\n", l.h_in));
    out.push_str(&format!("output entropy:   {:.4} bits\n", l.h_out));
    out.push_str(&format!(
        "bits destroyed:   {:.4}{}\n",
        destroyed,
        if destroyed <= 0.000_01 {
            "   (nothing destroyed — the action is injective on what occurs)"
        } else {
            ""
        }
    ));

    out.push_str(&format!(
        "\ndistinct states:  {} in -> {} out\n",
        l.distinct_in, l.distinct_out
    ));

    out.push_str("\nthe map on values:\n");
    for i in 0..4 {
        match l.image[i] {
            Some(j) => out.push_str(&format!(
                "    {} -> {}{}\n",
                VNAME[i],
                VNAME[j],
                if l.before[i] == 0 { "   (never occurs here)" } else { "" }
            )),
            None => {}
        }
    }

    out.push_str("\nirreversible transitions:\n");
    if l.merged_pairs.is_empty() {
        out.push_str("    none — no two occurring verdicts share an image\n");
    } else {
        for (a, b, to) in &l.merged_pairs {
            out.push_str(&format!(
                "    {} and {} both become {} — the difference is unrecoverable\n",
                a, b, to
            ));
        }
    }

    out.push_str(
        "\nCounted over the enumerated space, not sampled. A deterministic map on\n\
         a finite set cannot create entropy, so bits destroyed is never negative;\n\
         this measures the VALUE map only and says nothing about the tier ladder.\n",
    );
    out
}

pub fn loss_main(args: &[&str]) -> String {
    if args.is_empty() {
        let mut s = String::from(
            "loss <operation> [--len N]\n\
             \n\
             What a transformation destroys, counted over the whole word space:\n\
             input and output entropy, the bits destroyed, which states collapsed,\n\
             and which differences became unrecoverable.\n\
             \n\
             operations: ",
        );
        for (i, a) in ACTIONS.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(a);
        }
        s.push_str("\n\nTry:  loss collapse | loss cycle | loss meet --len 4\n");
        return s;
    }

    let mut len = 3usize;
    let mut i = 1;
    while i < args.len() {
        if (args[i] == "--len" || args[i] == "-n") && i + 1 < args.len() {
            len = args[i + 1].parse::<usize>().unwrap_or(3);
            i += 1;
        }
        i += 1;
    }
    if len == 0 || len > MAX_LEN {
        return format!("length {} is outside the enumerable range 1..{}.\n", len, MAX_LEN);
    }

    match measure(args[0], len) {
        Some(l) => format_loss(&l),
        None => format!(
            "'{}' is not a Belnap action. Try one of: {}\n",
            args[0],
            ACTIONS.join(", ")
        ),
    }
}
