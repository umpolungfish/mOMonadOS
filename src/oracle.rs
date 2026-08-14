// ─── oracle.rs ─────────────────────────────────────────────────────────
// Adversarial theorem checker (build.txt §181).
//
// Not a proof assistant — the opposite. "Assume this is false. What is the
// cheapest structural counterexample?"
//
// The status vocabulary (empirically supported vs theorem) already exists in
// `redteam audit` and `provenance`, so this does not restate it. What this adds
// is the ATTACK: an exhaustive hunt for a counterexample, ordered by cost.
//
// Words are enumerated by increasing length, so the FIRST counterexample found
// is the cheapest one that exists — not merely the first the search happened to
// try. That ordering is what makes "cheapest" a claim rather than a flourish.
//
// When the hunt finds nothing it reports the exact space it exhausted and
// refuses the word "theorem". A claim surviving 22,620 attacks is a claim that
// survived 22,620 attacks.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::belnap::B4;
use crate::counterfactual::{apply, read, Perturbation, MARKS};
use crate::ctc::{action_by_name, nest, Class};
use crate::ctc_loom::verdict_of;

/// Longest word the attack enumerates. 12^1+..+12^4 = 22,620 candidates.
pub const MAX_ATTACK_LEN: usize = 4;

pub struct Attack {
    pub claim: &'static str,
    pub restatement: &'static str,
    /// The cheapest counterexample, if the hunt found one.
    pub counterexample: Option<(String, String)>,
    pub tested: usize,
    pub exhausted_to: usize,
}

fn each_word_of_len(n: usize, mut f: impl FnMut(&str) -> bool) -> (usize, bool) {
    let base = MARKS.len();
    let total = base.pow(n as u32);
    let mut buf: Vec<char> = Vec::with_capacity(n);
    let mut seen = 0usize;
    for code in 0..total {
        buf.clear();
        let mut c = code;
        for _ in 0..n {
            buf.push(MARKS[c % base]);
            c /= base;
        }
        let w: String = buf.iter().collect();
        seen += 1;
        if f(&w) {
            return (seen, true);
        }
    }
    (seen, false)
}

/// Hunt by increasing length. `probe` returns Some(reason) when the word breaks
/// the claim; the search stops at the first break, which is therefore minimal.
fn hunt(
    claim: &'static str,
    restatement: &'static str,
    mut probe: impl FnMut(&str) -> Option<String>,
) -> Attack {
    let mut tested = 0usize;
    let mut found: Option<(String, String)> = None;
    let mut exhausted_to = 0usize;

    for len in 1..=MAX_ATTACK_LEN {
        let mut hit: Option<(String, String)> = None;
        let (seen, _broke) = each_word_of_len(len, |w| match probe(w) {
            Some(reason) => {
                hit = Some((w.to_string(), reason));
                true
            }
            None => false,
        });
        tested += seen;
        exhausted_to = len;
        if hit.is_some() {
            found = hit;
            break;
        }
    }

    Attack {
        claim,
        restatement,
        counterexample: found,
        tested,
        exhausted_to,
    }
}

pub fn attack(name: &str) -> Option<Attack> {
    match name {
        // Expected to SURVIVE: the ring makes the verdict phase-independent.
        "rotat-verdict" => Some(hunt(
            "ROTAT preserves the verdict",
            "a word and its rotation always agree on the tri-ancestral verdict",
            |w| {
                let a = read(w)?;
                let b = read(&apply(w, Perturbation::Rotate(1)))?;
                if a.verdict != b.verdict {
                    Some(format!("verdict {} -> {} under one shift", a.verdict, b.verdict))
                } else {
                    None
                }
            },
        )),
        // Expected to FALL: the register is exactly what phase moves.
        "rotat-register" => Some(hunt(
            "ROTAT preserves the final register",
            "a word and its rotation always land on the same register",
            |w| {
                let a = read(w)?;
                let b = read(&apply(w, Perturbation::Rotate(1)))?;
                if a.register != b.register {
                    Some(format!("register {} -> {} under one shift", a.register, b.register))
                } else {
                    None
                }
            },
        )),
        // Expected to SURVIVE: ctc-loom found meet one-shot on the whole space.
        "meet-possesses" => Some(hunt(
            "every verdict is already a fixed point of meet",
            "nesting any word's verdict in meet costs nothing",
            |w| {
                let v = verdict_of(w)?;
                let g = action_by_name("meet")?;
                let c = nest(g, v);
                if c.class != Class::OneShot {
                    Some(format!("class {} at price {}", c.class.name(), c.price))
                } else {
                    None
                }
            },
        )),
        // Expected to FALL immediately: cycle was manufactured on every word.
        "cycle-possesses" => Some(hunt(
            "every verdict is already a fixed point of cycle",
            "nesting any word's verdict in cycle costs nothing",
            |w| {
                let v = verdict_of(w)?;
                let g = action_by_name("cycle")?;
                let c = nest(g, v);
                if c.class != Class::OneShot {
                    Some(format!("class {} at price {}", c.class.name(), c.price))
                } else {
                    None
                }
            },
        )),
        // Expected to SURVIVE: swapping the fork and fuse marks is an involution
        // exchanging the two ill-typed branches, so F-words and B-words pair up.
        "fb-swap-symmetry" => Some(hunt(
            "swapping ∈ and ∋ exchanges the F and B verdicts",
            "the fork/fuse swap maps every F word to a B word and back",
            |w| {
                let v = verdict_of(w)?;
                let swapped: String = w
                    .chars()
                    .map(|c| match c {
                        '∈' => '∋',
                        '∋' => '∈',
                        other => other,
                    })
                    .collect();
                let sv = verdict_of(&swapped)?;
                let expected = match v {
                    B4::F => Some(B4::B),
                    B4::B => Some(B4::F),
                    _ => None,
                };
                match expected {
                    Some(e) if sv != e => Some(format!(
                        "{} has verdict {:?} but its swap {} has {:?}",
                        w, v, swapped, sv
                    )),
                    _ => None,
                }
            },
        )),
        // Expected to FALL: a glyph can break banking as easily as fix it.
        "insertion-never-breaks-banking" => Some(hunt(
            "inserting a glyph never turns a holding word into an exposed one",
            "banking is monotone under insertion",
            |w| {
                let a = read(w)?;
                if !a.holds {
                    return None;
                }
                for &g in MARKS.iter() {
                    for pos in 0..=w.chars().count() {
                        let mut v: Vec<char> = w.chars().collect();
                        v.insert(pos, g);
                        let cand: String = v.into_iter().collect();
                        if let Some(r) = read(&cand) {
                            if !r.holds && !r.vacuous {
                                return Some(format!(
                                    "holds, but inserting {} at {} gives {} which is exposed",
                                    g, pos, cand
                                ));
                            }
                        }
                    }
                }
                None
            },
        )),
        _ => None,
    }
}

pub const CLAIMS: [&str; 6] = [
    "rotat-verdict",
    "rotat-register",
    "meet-possesses",
    "cycle-possesses",
    "fb-swap-symmetry",
    "insertion-never-breaks-banking",
];

pub fn format_attack(a: &Attack) -> String {
    let mut out = String::new();
    out.push_str("ORACLE\n======\n\n");
    out.push_str(&format!("CLAIM:        {}\n", a.claim));
    out.push_str(&format!("as tested:    {}\n\n", a.restatement));
    out.push_str(&format!(
        "attacked:     {} words, every word of length 1..{} in increasing order\n\n",
        a.tested, a.exhausted_to
    ));

    match &a.counterexample {
        Some((word, why)) => {
            out.push_str("CHEAPEST COUNTEREXAMPLE\n");
            out.push_str(&format!("    {}   ({} glyphs)\n", word, word.chars().count()));
            out.push_str(&format!("    {}\n\n", why));
            out.push_str(
                "It is the cheapest because the hunt runs by increasing length and\n\
                 stops at the first break: nothing shorter exists.\n\n",
            );
            out.push_str("STATUS:\n  REFUTED\n");
        }
        None => {
            out.push_str("counterexamples: none\n\n");
            out.push_str("STATUS:\n  survived every attack in the searched space\n\n");
            out.push_str("NOT:\n  a theorem. The space is finite and bounded at length ");
            out.push_str(&format!("{};\n", a.exhausted_to));
            out.push_str(
                "  a claim that survived this hunt is a claim that survived this\n\
                 \x20 hunt. Nothing here quantifies over longer words.\n",
            );
        }
    }
    out
}

pub fn oracle_main(args: &[&str]) -> String {
    if args.is_empty() {
        let mut s = String::from(
            "oracle <claim>\n\
             \n\
             Assume the claim is false and hunt for the cheapest structural\n\
             counterexample. Words are tried by increasing length, so the first\n\
             break found is the smallest one that exists.\n\
             \n\
             Surviving the hunt is NOT proof: the space is bounded at length 4\n\
             and the report says so rather than saying 'theorem'.\n\
             \n\
             claims:\n",
        );
        for c in CLAIMS {
            s.push_str(&format!("    {}\n", c));
        }
        s.push_str("\nTry:  oracle rotat-register\n");
        return s;
    }

    match attack(args[0]) {
        Some(a) => format_attack(&a),
        None => format!(
            "No claim named '{}'. Run `oracle` for the list.\n",
            args[0]
        ),
    }
}
