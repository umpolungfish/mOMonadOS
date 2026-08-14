// ─── phase.rs ──────────────────────────────────────────────────────────
// Phase as a first-class computational object (build.txt §319).
//
// `cycle` already PRINTS the map from cut to landing register for one word.
// What did not exist is the same walk as a VALUE, and therefore any question
// that compares two words phase by phase. This module supplies the value and
// the comparison; it does not restate what `cycle` says about a single word.
//
// The substrate is `counterfactual::read` — one reader, already used by the
// counterfactual and basin tools — applied to each rotation of the word. So a
// spectrum here and a `cycle` report there are two callers of one walk, not two
// implementations of one idea.
//
// The physics being read: a word is a ring and ROTAT is the cyclic shift, so
// the verdict and topology hold across the whole orbit while the FINAL REGISTER
// does not. The register is therefore the phase observable, and interference
// between two words is a question about where their registers coincide.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::counterfactual::{apply, read, Perturbation};

pub struct Sample {
    pub k: usize,
    pub word: String,
    pub verdict: char,
    pub register: String,
}

/// The word walked around its whole orbit: one sample per cut.
pub fn spectrum(word: &str) -> Vec<Sample> {
    let mut out = Vec::new();
    let n = match read(word) {
        Some(r) => r.length,
        None => return out,
    };
    for k in 0..n {
        let rotated = apply(word, Perturbation::Rotate(k as isize));
        if let Some(r) = read(&rotated) {
            out.push(Sample {
                k,
                word: r.word.clone(),
                verdict: r.verdict,
                register: r.register,
            });
        }
    }
    out
}

/// Distinct registers over the orbit, in first-seen order.
pub fn register_support(s: &[Sample]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for x in s {
        if !out.iter().any(|r| *r == x.register) {
            out.push(x.register.clone());
        }
    }
    out
}

/// The phase period: the smallest shift after which the register sequence
/// repeats. Divides the word length, and is 1 when the register is phase-blind.
pub fn phase_period(s: &[Sample]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    for p in 1..=n {
        if n % p != 0 {
            continue;
        }
        if (0..n).all(|i| s[i].register == s[(i + p) % n].register) {
            return p;
        }
    }
    n
}

pub struct Interference {
    pub a: String,
    pub b: String,
    /// Shifts at which the two words land on the same register.
    pub coincidences: Vec<(usize, String)>,
    pub samples: usize,
    pub verdict_a: char,
    pub verdict_b: char,
}

/// Two words compared phase by phase. Only defined where both have a sample,
/// so words of different length are compared over the shorter orbit and the
/// count says how many shifts were actually examined.
pub fn interference(a: &str, b: &str) -> Interference {
    let sa = spectrum(a);
    let sb = spectrum(b);
    let n = sa.len().min(sb.len());
    let mut coincidences = Vec::new();
    for k in 0..n {
        if sa[k].register == sb[k].register {
            coincidences.push((k, sa[k].register.clone()));
        }
    }
    Interference {
        a: a.to_string(),
        b: b.to_string(),
        coincidences,
        samples: n,
        verdict_a: sa.first().map(|s| s.verdict).unwrap_or('?'),
        verdict_b: sb.first().map(|s| s.verdict).unwrap_or('?'),
    }
}

pub fn format_spectrum(word: &str) -> String {
    let s = spectrum(word);
    if s.is_empty() {
        return format!("'{}' parses to no tokens.\n", word);
    }
    let mut out = String::new();
    out.push_str("PHASE SPECTRUM\n==============\n\n");
    out.push_str(&format!("word:          {}\n", s[0].word));
    out.push_str(&format!("orbit length:  {}\n", s.len()));

    let support = register_support(&s);
    let period = phase_period(&s);
    out.push_str(&format!("phase period:  {}{}\n", period,
        if period == 1 { "   (register is phase-blind on this word)" } else { "" }));
    out.push_str(&format!("register support: {} distinct\n\n", support.len()));

    out.push_str("   k  word                     verdict  register\n");
    for x in &s {
        out.push_str(&format!(
            "  {:>2}  {:<24} {:^7}  {}\n",
            x.k, x.word, x.verdict, x.register
        ));
    }

    let verdicts_held = s.iter().all(|x| x.verdict == s[0].verdict);
    out.push_str(&format!(
        "\nverdict across the orbit: {}\n",
        if verdicts_held {
            format!("{} throughout — invariant, as the ring demands", s[0].verdict)
        } else {
            "NOT invariant — which would contradict ROTAT closure".to_string()
        }
    ));
    out
}

pub fn format_interference(i: &Interference) -> String {
    let mut out = String::new();
    out.push_str("PHASE INTERFERENCE\n==================\n\n");
    out.push_str(&format!("A:  {}   verdict {}\n", i.a, i.verdict_a));
    out.push_str(&format!("B:  {}   verdict {}\n\n", i.b, i.verdict_b));

    if i.samples == 0 {
        out.push_str("No shared orbit — one of the words parses to no tokens.\n");
        return out;
    }

    out.push_str(&format!(
        "compared over {} shift(s) — the shorter orbit\n\n",
        i.samples
    ));

    if i.coincidences.is_empty() {
        out.push_str("coincidences: none — the two never land on the same register\n");
    } else {
        out.push_str(&format!(
            "coincidences: {} of {} shifts\n",
            i.coincidences.len(),
            i.samples
        ));
        for (k, reg) in &i.coincidences {
            out.push_str(&format!("    k={:<3} both land on {}\n", k, reg));
        }
        if i.coincidences.len() == i.samples {
            out.push_str(
                "\n    every shift coincides: these two words are phase-identical\n\
                 \x20   in the register, whatever else differs between them\n",
            );
        }
    }
    out
}

pub fn phase_main(args: &[&str]) -> String {
    if args.is_empty() {
        return "phase <word>                     the orbit as a spectrum\n\
                phase interference <w1> <w2>     where two words share a register\n\
                \n\
                A word is a ring and ROTAT is the cyclic shift: the verdict holds\n\
                across the orbit, the final register does not. The register is the\n\
                phase observable, so interference is a question about where two\n\
                words coincide in it.\n\
                \n\
                (`cycle <word>` prints the same walk for one word; this adds the\n\
                phase period, the register support, and the two-word comparison.)\n"
            .to_string();
    }

    match args[0] {
        "interference" | "i" => {
            if args.len() < 3 {
                return "phase interference <word1> <word2>\n".to_string();
            }
            format_interference(&interference(args[1], args[2]))
        }
        "spectrum" | "s" => {
            if args.len() < 2 {
                return "phase spectrum <word>\n".to_string();
            }
            format_spectrum(args[1])
        }
        w => format_spectrum(w),
    }
}
