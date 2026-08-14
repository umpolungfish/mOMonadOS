// ─── basin.rs ──────────────────────────────────────────────────────────
// Fixed-point archaeology (build.txt §37).
//
// `nesting` already distinguishes attraction, non-arrival, finite settling and
// fixed points for maps on reals. This is the same reading for maps on WORDS:
// orbit, attractor, transient depth, cycle length, basin size, and the nearest
// competing attractor.
//
// Two actions, both defined by machinery that already exists here — no
// dynamics are invented for the sake of having some:
//
//   ROTAT   the cyclic shift. Invertible, so every orbit is a pure cycle with
//           transient depth 0 and cycle length dividing the word length. That
//           is a fact about the action, not a shortcut in the code.
//
//   REPAIR  w maps to itself when its banking holds; otherwise to the first
//           single-glyph insertion that makes it hold. Non-invertible, so
//           orbits have genuine transients and the fixed points are exactly
//           the words that hold. This is the basin structure §29 asks about.
//
// Basin sizes are EXACT or absent. The word space of length n is 12^n, which
// is enumerable to n=4 (20,736 words) and not beyond; past that the tool says
// so rather than sampling and calling the result a size.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::counterfactual::{read, MARKS};
use crate::lattice_flow::normalize;

/// Largest word length whose full space (12^n) is enumerated exactly.
///
/// The limit is per-action because one step does not cost the same in both.
/// ROTAT is a shift, so 12^4 = 20,736 words is cheap. One REPAIR step is itself
/// an exhaustive 12*n insertion sweep with a walk per candidate, which puts
/// 12^4 into the millions of walks and hangs the kernel — measured on hardware,
/// not estimated. Enumeration stops where it stops being honest to run.
pub const EXACT_ENUM_MAX_LEN: usize = 4;
pub const EXACT_ENUM_MAX_LEN_REPAIR: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Rotat,
    Repair,
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::Rotat => "ROTAT",
            Action::Repair => "REPAIR",
        }
    }

    pub fn invertible(&self) -> bool {
        matches!(self, Action::Rotat)
    }

    /// How far the full space can be enumerated for this action.
    pub fn enum_limit(&self) -> usize {
        match self {
            Action::Rotat => EXACT_ENUM_MAX_LEN,
            Action::Repair => EXACT_ENUM_MAX_LEN_REPAIR,
        }
    }

    pub fn parse(s: &str) -> Option<Action> {
        match s.to_lowercase().as_str() {
            "rotat" | "rotate" | "r" => Some(Action::Rotat),
            "repair" | "p" => Some(Action::Repair),
            _ => None,
        }
    }

    /// One step of the map.
    pub fn step(&self, word: &str) -> String {
        match self {
            Action::Rotat => {
                let chars: Vec<char> = word.chars().collect();
                if chars.len() < 2 {
                    return word.to_string();
                }
                let mut v: Vec<char> = chars[1..].to_vec();
                v.push(chars[0]);
                v.into_iter().collect()
            }
            Action::Repair => {
                match read(word) {
                    // Already a fixed point: banking holds.
                    Some(r) if r.holds => word.to_string(),
                    Some(_) => {
                        let chars: Vec<char> = word.chars().collect();
                        for pos in 0..=chars.len() {
                            for &g in MARKS.iter() {
                                let mut v = chars.clone();
                                v.insert(pos, g);
                                let cand: String = v.into_iter().collect();
                                if let Some(rc) = read(&cand) {
                                    if rc.holds {
                                        return cand;
                                    }
                                }
                            }
                        }
                        // Nothing makes it hold: the word is its own dead end.
                        word.to_string()
                    }
                    None => word.to_string(),
                }
            }
        }
    }
}

pub struct Orbit {
    pub seed: String,
    pub action: Action,
    pub trail: Vec<String>,
    /// Index in `trail` where the cycle starts = transient depth.
    pub transient_depth: usize,
    pub cycle_length: usize,
    /// Canonical attractor label: the lexicographically least word of the cycle.
    pub attractor: String,
    /// True when the walk stopped because it hit the step cap, not a cycle.
    pub truncated: bool,
}

const MAX_STEPS: usize = 64;

pub fn orbit(seed: &str, action: Action) -> Orbit {
    let start = normalize(seed);
    let mut trail: Vec<String> = Vec::new();
    let mut cur = start.clone();
    let mut truncated = false;

    let (depth, cyc) = loop {
        if let Some(prev) = trail.iter().position(|w| *w == cur) {
            break (prev, trail.len() - prev);
        }
        if trail.len() >= MAX_STEPS {
            truncated = true;
            break (trail.len(), 0);
        }
        trail.push(cur.clone());
        cur = action.step(&cur);
    };

    // The attractor is named by the least word on the cycle, so every point of
    // one cycle reports the same attractor.
    let attractor = if cyc == 0 {
        cur.clone()
    } else {
        let mut best = trail[depth].clone();
        for w in trail.iter().skip(depth) {
            if *w < best {
                best = w.clone();
            }
        }
        best
    };

    Orbit {
        seed: start,
        action,
        trail,
        transient_depth: depth,
        cycle_length: cyc,
        attractor,
        truncated,
    }
}

/// The attractor a word falls into, or None if the walk did not settle.
fn attractor_of(word: &str, action: Action) -> Option<String> {
    let o = orbit(word, action);
    if o.truncated {
        None
    } else {
        Some(o.attractor)
    }
}

/// Exhaustive enumeration of every word of length n over the twelve marks.
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

pub struct BasinCensus {
    pub length: usize,
    pub population: usize,
    pub in_basin: usize,
    /// (attractor, size) for the competing attractors, largest first.
    pub competitors: Vec<(String, usize)>,
}

/// Exact basin size over the full space of words of the seed's length.
/// Returns None when that space is too large to enumerate honestly.
pub fn basin_census(target: &str, action: Action, len: usize) -> Option<BasinCensus> {
    if len == 0 || len > action.enum_limit() {
        return None;
    }
    let mut in_basin = 0usize;
    let mut population = 0usize;
    let mut tally: Vec<(String, usize)> = Vec::new();

    each_word_of_len(len, |w| {
        population += 1;
        if let Some(a) = attractor_of(w, action) {
            if a == target {
                in_basin += 1;
            }
            match tally.iter_mut().find(|(name, _)| name == &a) {
                Some((_, n)) => *n += 1,
                None => tally.push((a, 1)),
            }
        }
    });

    tally.sort_by(|a, b| b.1.cmp(&a.1));
    let competitors: Vec<(String, usize)> = tally
        .into_iter()
        .filter(|(name, _)| name != target)
        .take(3)
        .collect();

    Some(BasinCensus {
        length: len,
        population,
        in_basin,
        competitors,
    })
}

pub fn format_basin(seed: &str, action: Action) -> String {
    let o = orbit(seed, action);
    let mut out = String::new();

    out.push_str("BASIN\n=====\n\n");
    out.push_str(&format!("SEED     {}\n", o.seed));
    out.push_str(&format!(
        "ACTION   {} ({})\n\n",
        o.action.name(),
        if o.action.invertible() {
            "invertible: every orbit is a pure cycle"
        } else {
            "non-invertible: orbits carry transients"
        }
    ));

    out.push_str("orbit:\n    ");
    let shown = o.trail.len().min(8);
    for (i, w) in o.trail.iter().take(shown).enumerate() {
        if i > 0 {
            out.push_str(" -> ");
        }
        out.push_str(w);
    }
    if o.trail.len() > shown {
        out.push_str(&format!(" ... (+{} more)", o.trail.len() - shown));
    }
    out.push_str("\n\n");

    if o.truncated {
        out.push_str(&format!(
            "attractor:        none found within {} steps — the walk did not settle\n",
            MAX_STEPS
        ));
        out.push_str("transient depth:  unbounded so far\n");
        out.push_str("cycle length:     none detected\n");
    } else {
        out.push_str(&format!("attractor:        {}\n", o.attractor));
        out.push_str(&format!("transient depth:  {}\n", o.transient_depth));
        out.push_str(&format!(
            "cycle length:     {}{}\n",
            o.cycle_length,
            if o.cycle_length == 1 { "  (fixed point)" } else { "" }
        ));
    }

    let len = o.seed.chars().count();
    out.push('\n');
    match basin_census(&o.attractor, o.action, len) {
        Some(c) => {
            out.push_str(&format!(
                "basin size:       {} of {} words of length {} (exact, full enumeration)\n",
                c.in_basin, c.population, c.length
            ));
            if c.competitors.is_empty() {
                out.push_str("competing attractors:\n    none — this attractor takes the whole space\n");
            } else {
                out.push_str("nearest competing attractors:\n");
                for (name, n) in &c.competitors {
                    out.push_str(&format!("    {:<20} basin {}\n", name, n));
                }
            }
        }
        None => {
            out.push_str(&format!(
                "basin size:       not enumerated — the space of length {} is 12^{} words,\n\
                 \x20                 past the exact limit of 12^{} for {}. No estimate is given.\n",
                len, len, o.action.enum_limit(), o.action.name()
            ));
        }
    }

    out
}

pub fn basin_main(args: &[&str]) -> String {
    if args.is_empty() {
        return "basin <seed> [--action ROTAT|REPAIR]\n\
                \n\
                Orbit, attractor, transient depth, cycle length, and exact basin\n\
                size over the full word space (enumerated to length 4).\n\
                \n\
                  ROTAT   cyclic shift; invertible, so orbits are pure cycles\n\
                  REPAIR  w -> first insertion making its banking hold; the\n\
                          fixed points are exactly the words that hold\n\
                \n\
                Try:  basin \u{22a2}\u{2208}\u{22a4}\u{220b} --action REPAIR\n"
            .to_string();
    }

    let seed = args[0];
    let mut action = Action::Rotat;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--action" || args[i] == "-a" {
            if i + 1 < args.len() {
                match Action::parse(args[i + 1]) {
                    Some(a) => action = a,
                    None => return format!("unknown action '{}' — try ROTAT or REPAIR\n", args[i + 1]),
                }
                i += 1;
            }
        } else if let Some(a) = Action::parse(args[i]) {
            action = a;
        }
        i += 1;
    }

    format_basin(seed, action)
}
