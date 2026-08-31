//! Lattice cycling and weight flow over an IMASM word, in the kernel.
//!
//! A word is a ring and ROTAT is the cyclic shift, so every rotation is the
//! same object. The verdict and the topology hold across the whole orbit; the
//! FINAL REGISTER does not. That makes the phase the only handle on where a
//! word comes to rest, and `cycle_report` prints the map from cut to landing
//! register so the handle can be read rather than guessed.
//!
//! `weight_report` answers the other half. The trilattice machine holds each
//! open fork as a set and closes it with a union, so a finished walk knows
//! WHICH base values were touched and nothing else: not how many times, not by
//! which arm, not whether a value reached the end or was destroyed and restored
//! on the way. This walks the same rules while counting, so the movement is
//! visible. Weight banked in a frame survives a clear that empties the
//! register; weight left in the open does not.
//!
//! The lift of OR to weights is MAX, not sum. Adding would count each deposit
//! twice, once landing in the register and again when its frame closed; under
//! max the fuse RESTORES what a clear destroyed and leaves the rest alone, and
//! at weights zero and one the accounting reduces to the set semantics exactly.
//!
//! Two movements carry no weight at all and are reported because they are
//! otherwise invisible in a final register:
//!
//!   SEED   AFWD and IMSCRIB put T into an empty register directly, so a walk
//!          can land in T having carried nothing
//!   INERT  after IFIX every token but IFIX and IMSCRIB is a no-op, so a word
//!          can be almost entirely inert

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use imasm_core::imasm16_3::{parse_glyph_word, run_word_register, tri_ancestral_verdict, Token16_3};

use crate::sprintln;

/// Bring a word onto the current alphabet before the core reads it.
///
/// The retired spellings are still in every stored word and in anything copied
/// out of an older report, and the tensor forms turn up where a fork and a fuse
/// were written as ⊗ and ⊕. Translating is what lets those load; dropping them
/// would read the word as shorter than it is and change its verdict.
fn normalize(word: &str) -> String {
    let mut out = String::new();
    for c in word.chars() {
        match c {
            '◇' | '⊗' => out.push('∈'),
            '●' | '⊕' => out.push('∋'),
            '=' | '═' => out.push('⋈'),
            '+' => out.push('⊤'),
            '×' => out.push('⊥'),
            '¬' => out.push('⊡'),
            c if c.is_whitespace() => {}
            c => out.push(c),
        }
    }
    out
}

fn render(steps: &[Token16_3]) -> String {
    let mut s = String::new();
    for t in steps { s.push(t.glyph()); }
    s
}

/// Walk the orbit and report where each cut lands.
pub fn cycle_report(word: &str) {
    let steps = parse_glyph_word(&normalize(word));
    let n = steps.len();
    if n == 0 {
        sprintln!("  no IMASM glyphs in that word");
        return;
    }
    sprintln!("word   : {}   period {}", render(&steps), n);
    sprintln!("   {:>3}  {:<6} {:<8} word", "k", "final", "verdict");

    let mut finals: Vec<(String, usize)> = Vec::new();
    for k in 0..n {
        let mut rot: Vec<Token16_3> = Vec::with_capacity(n);
        for i in 0..n { rot.push(steps[(i + k) % n]); }
        let reg = run_word_register(&rot);
        let (v, _) = tri_ancestral_verdict(&rot);
        sprintln!("   {:>3}  {:<6} {:<8} {}", k, reg, v, render(&rot));
        finals.push((reg, k));
    }

    // The map the whole thing exists for: which cut lands you where.
    sprintln!("");
    sprintln!("  landing register by cut:");
    let mut seen: Vec<String> = Vec::new();
    for (reg, _) in finals.iter() {
        if !seen.iter().any(|s| s == reg) { seen.push(reg.clone()); }
    }
    for reg in seen.iter() {
        let mut ks = String::new();
        for (r, k) in finals.iter() {
            if r == reg {
                if !ks.is_empty() { ks.push_str(", "); }
                ks.push_str(&format!("{}", k));
            }
        }
        sprintln!("    {:<6} at k = {}", reg, ks);
    }
    let distinct = seen.len();
    if distinct == 1 {
        sprintln!("  final register is INVARIANT under ROTAT here");
    } else {
        sprintln!("  final register is PHASE-BEARING: {} distinct landings", distinct);
    }
}

/// Opcode-to-opcode transitions counted ON THE RING.
///
/// A word is a cycle and ROTAT is the cyclic shift, so a word of length n has n
/// transitions, not n-1. The one a linear read drops is the wrap from the last
/// opcode back to the first, and in IMASM that is overwhelmingly TANCH -> VINIT:
/// the anchor returning to the source. Across k programs a linear read loses
/// exactly k such edges, and a table built without them can show a rule as
/// universal that the closing edges break.
pub fn transitions_report(word: &str) {
    let steps = parse_glyph_word(&normalize(word));
    let n = steps.len();
    if n == 0 { sprintln!("  no IMASM glyphs in that word"); return; }
    sprintln!("word   : {}   length {}", render(&steps), n);
    sprintln!("  ring transitions   : {}", n);
    sprintln!("  linear would give  : {}   (drops the closing edge)", n - 1);
    sprintln!("  closing edge       : {} -> {}",
              steps[n - 1].glyph(), steps[0].glyph());
    sprintln!("");
    // count them, most frequent first, without allocating a map
    let mut seen: Vec<((char, char), u32)> = Vec::new();
    for i in 0..n {
        let key = (steps[i].glyph(), steps[(i + 1) % n].glyph());
        if let Some(e) = seen.iter_mut().find(|e| e.0 == key) { e.1 += 1; }
        else { seen.push((key, 1)); }
    }
    seen.sort_by(|a, b| b.1.cmp(&a.1));
    sprintln!("  transitions:");
    for ((a, b), c) in seen.iter() {
        sprintln!("    {} -> {}   {}", a, b, c);
    }
    sprintln!("");
    sprintln!("  Anything read from ABSOLUTE position on a ring measures the cut,");
    sprintln!("  not the word: matrix rows, tetraktys tiers, odd against even.");
    sprintln!("  One rotation moves every value into a different row.");
}

/// Was anything counted, then cleared with nothing banked behind it?
///
/// AREV empties the register and leaves open frames alone, so a result fused
/// back to depth zero is exposed to the next reversal, while the same result
/// held one level up survives it. A program that establishes something, then
/// reverses, then bounds must open the region that will HOLD the result before
/// the region that COMPUTES it, and close them in that order.
/// What the banking walk found. Separated from the printing so a caller can
/// test a word instead of reading about it -- `insert` sweeps hundreds of
/// candidates and needs the verdict, not the paragraph. One walk, two callers.
pub struct Banked {
    pub exposed: Vec<(usize, char, u32)>,
    pub live_clears: u32,
    pub deposits: u32,
    pub inert: u32,
    pub reg: [u32; 4],
}

impl Banked {
    /// Held across every clear that actually fired. Vacuous words are not OK:
    /// nothing was ever at risk, so nothing was held.
    pub fn holds(&self) -> bool { self.exposed.is_empty() && self.live_clears > 0 }
    pub fn vacuous(&self) -> bool { self.exposed.is_empty() && self.live_clears == 0 }
}

pub fn banked_walk(word: &str) -> Option<Banked> {
    let steps = parse_glyph_word(&normalize(word));
    if steps.is_empty() { return None; }
    let mut reg = [0u32; 4];
    let mut frames: Vec<[u32; 4]> = Vec::new();
    let mut fixed = false;
    let mut exposed: Vec<(usize, char, u32)> = Vec::new();
    let mut live_clears = 0u32;
    let mut inert = 0u32;
    let mut deposits = 0u32;

    for (i, t) in steps.iter().enumerate() {
        if fixed && !matches!(t, Token16_3::Ifix | Token16_3::Imscrib) { inert += 1; continue; }
        match t {
            Token16_3::Fsplit3 => frames.push([0; 4]),
            Token16_3::Ffuse3 => {
                if let Some(closed) = frames.pop() {
                    for j in 0..4 {
                        if closed[j] > reg[j] { reg[j] = closed[j]; }
                        if let Some(o) = frames.last_mut() {
                            if closed[j] > o[j] { o[j] = closed[j]; }
                        }
                    }
                }
            }
            Token16_3::Arev | Token16_3::Vinit => {
                let lost: u32 = reg.iter().sum();
                let banked: u32 = frames.iter().map(|f| f.iter().sum::<u32>()).sum();
                if lost > 0 {
                    live_clears += 1;
                    if banked == 0 { exposed.push((i + 1, t.glyph(), lost)); }
                }
                reg = [0; 4];
                if matches!(t, Token16_3::Vinit) { frames.clear(); }
            }
            Token16_3::Ifix => fixed = true,
            _ => {
                let touched: &[usize] = match t {
                    Token16_3::Evalt => &[0],
                    Token16_3::Evalf => &[1],
                    Token16_3::Evali => &[2, 3],
                    _ => &[],
                };
                if !touched.is_empty() { deposits += 1; }
                for &j in touched {
                    reg[j] += 1;
                    if let Some(f) = frames.last_mut() { f[j] += 1; }
                }
            }
        }
    }

    Some(Banked { exposed, live_clears, deposits, inert, reg })
}

pub fn banked_report(word: &str) {
    let steps = parse_glyph_word(&normalize(word));
    let b = match banked_walk(word) {
        Some(b) => b,
        None => { sprintln!("  no IMASM glyphs in that word"); return; }
    };
    let (exposed, live_clears, deposits, inert, reg) =
        (b.exposed, b.live_clears, b.deposits, b.inert, b.reg);

    sprintln!("word   : {}", render(&steps));
    if exposed.is_empty() && live_clears == 0 {
        // Passing because nothing was ever at risk is not the same as passing
        // because the frame held.
        sprintln!("  VACUOUS — no clear ever fired against a live register");
        sprintln!("    {} deposit(s), {} step(s) inert after a fixation", deposits, inert);
    } else if exposed.is_empty() {
        sprintln!("  OK — weight survived {} live clear(s) by being banked", live_clears);
        // The second, independent fact. Banking asks whether a frame was open
        // at the clear; surplus asks where the splits fell between deposits of
        // the same value. A word can bank correctly and still lose a count.
        let mut surplus = 0u32;
        for j in 0..4 { if reg[j] > 1 { surplus += reg[j] - 1; } }
        if surplus > 0 {
            sprintln!("    up to {} unit(s) of repeat deposit may be flattened by a", surplus);
            sprintln!("    fold between sibling regions: the fold keeps the larger,");
            sprintln!("    not the sum. Deposits in ONE region keep both.");
        }
    } else {
        let total: u32 = exposed.iter().map(|e| e.2).sum();
        sprintln!("  {} unit(s) cleared with nothing banked behind them:", total);
        for (step, g, w) in exposed.iter() {
            sprintln!("    step {} {} cleared {} with nothing behind it", step, g, w);
        }
        sprintln!("  open the region that HOLDS the result before the region that");
        sprintln!("  COMPUTES it, and close them in that order.");
    }
}

/// Every single-glyph insertion that turns an exposed word into one that holds.
///
/// A word that loses weight is not usually rewritten; it is repaired, and the
/// repair is almost always one glyph in the right place. Rather than reason
/// about which, this walks all twelve glyphs at every position and reports the
/// ones that hold. The search is small -- twelve times length-plus-one -- and
/// exact, so there is nothing to infer.
pub fn insert_report(word: &str) {
    let steps = parse_glyph_word(&normalize(word));
    if steps.is_empty() { sprintln!("  no IMASM glyphs in that word"); return; }
    let base = render(&steps);
    let n = steps.len();

    sprintln!("word   : {}   length {}", base, n);
    match banked_walk(&base) {
        Some(b) if b.holds()   => { sprintln!("  already holds — nothing to repair"); return; }
        Some(b) if b.vacuous() => sprintln!("  vacuous: no clear ever fired, so nothing is at risk"),
        Some(b) => {
            let lost: u32 = b.exposed.iter().map(|e| e.2).sum();
            sprintln!("  exposed: {} unit(s) cleared with nothing banked", lost);
        }
        None => return,
    }

    let glyphs = ['⊢', '⊙', '∈', '∋', '⊤', '⊥', '>', '<', '⋈', '⊞', '⊡', '⊣'];
    let chars: Vec<char> = base.chars().collect();

    // Distinct words, not distinct sites. Inserting a glyph beside an identical
    // one yields the same word from two positions, and counting both would
    // report a repair twice and overstate how many ways out there are.
    let mut seen: Vec<String> = Vec::new();
    let mut tried = 0u32;

    sprintln!("  insertions that hold:");
    for pos in 0..=n {
        for g in glyphs.iter() {
            let mut cand = String::new();
            for (k, c) in chars.iter().enumerate() {
                if k == pos { cand.push(*g); }
                cand.push(*c);
            }
            if pos == n { cand.push(*g); }
            if seen.iter().any(|w| w == &cand) { continue; }
            tried += 1;
            if let Some(b) = banked_walk(&cand) {
                if b.holds() {
                    sprintln!("    {} at {:>2}   {}", g, pos, cand);
                    seen.push(cand);
                }
            }
        }
    }
    let found = seen.len();
    if found == 0 {
        sprintln!("    none — no single glyph repairs this word");
    } else {
        sprintln!("  {} distinct word(s) hold, of {} tried", found, tried);
    }
}

/// How many distinct one-glyph insertions make `base` hold, without printing.
fn repair_count(base: &str) -> usize {
    let glyphs = ['⊢', '⊙', '∈', '∋', '⊤', '⊥', '>', '<', '⋈', '⊞', '⊡', '⊣'];
    let chars: Vec<char> = base.chars().collect();
    let n = chars.len();
    let mut seen: Vec<String> = Vec::new();
    for pos in 0..=n {
        for g in glyphs.iter() {
            let mut cand = String::new();
            for (k, c) in chars.iter().enumerate() {
                if k == pos { cand.push(*g); }
                cand.push(*c);
            }
            if pos == n { cand.push(*g); }
            if seen.iter().any(|w| w == &cand) { continue; }
            if let Some(b) = banked_walk(&cand) {
                if b.holds() { seen.push(cand); }
            }
        }
    }
    seen.len()
}

/// Render a program as a glyph word, in the alphabet the walkers parse.
fn program_word(p: &crate::tokens::Program) -> String {
    let mut w = String::new();
    for t in p.as_slice() { w.push_str(t.code()); }
    normalize(&w)
}

/// Every built-in program, put to the same question.
///
/// The words are not typed in; the kernel hands over its own programs and each
/// is rendered from its tokens. A word that holds is left alone, and a word
/// that is exposed is asked how many single glyphs would close it -- which is
/// the interesting number, because a word with no repair at all is exposed for
/// a structural reason rather than a missing symbol.
pub fn insert_sweep_all() {
    use crate::tokens::*;

    let families: [(&str, usize); 5] = [
        ("canonical",  canonical_count()),
        ("continuous", continuous_count()),
        ("novel",      novel_count()),
        ("shunted",    shunted_count()),
        ("compound",   compound_count()),
    ];

    let mut total = 0u32;
    let mut holding = 0u32;
    let mut vacuous = 0u32;
    let mut repairable = 0u32;
    let mut stuck = 0u32;

    for (fam, count) in families.iter() {
        sprintln!("── {} ──", fam);
        for i in 0..*count {
            let prog = match *fam {
                "canonical"  => canonical(i),
                "continuous" => continuous_program(i),
                "novel"      => novel_program(i),
                "shunted"    => shunted_program(i),
                _            => compound_program(i),
            };
            let prog = match prog { Some(p) => p, None => continue };
            let name = match *fam {
                "canonical"  => canonical_name(i),
                "continuous" => continuous_name(i),
                "novel"      => novel_name(i),
                "shunted"    => shunted_name(i),
                _            => compound_name(i),
            };
            let word = program_word(&prog);
            if word.is_empty() { continue; }
            total += 1;

            match banked_walk(&word) {
                Some(b) if b.holds() => {
                    holding += 1;
                    sprintln!("  {:<28} holds        {}", name, word);
                }
                Some(b) if b.vacuous() => {
                    vacuous += 1;
                    sprintln!("  {:<28} vacuous — no clear fired", name);
                }
                Some(b) => {
                    let lost: u32 = b.exposed.iter().map(|e| e.2).sum();
                    let r = repair_count(&word);
                    if r == 0 { stuck += 1; } else { repairable += 1; }
                    sprintln!("  {:<28} exposed {} — {} repair(s)   {}", name, lost, r, word);
                }
                None => { total -= 1; }
            }
        }
    }

    sprintln!("");
    sprintln!("{} programs: {} hold, {} vacuous, {} exposed-and-repairable, {} exposed-with-no-repair",
        total, holding, vacuous, repairable, stuck);
}

/// Count what the union throws away.
pub fn weight_report(word: &str) {
    let steps = parse_glyph_word(&normalize(word));
    if steps.is_empty() {
        sprintln!("  no IMASM glyphs in that word");
        return;
    }
    sprintln!("word   : {}", render(&steps));

    // Base values are indexed T, F, t, f throughout.
    const NAMES: [&str; 4] = ["T", "F", "t", "f"];
    let mut reg = [0u32; 4];
    let mut frames: Vec<[u32; 4]> = Vec::new();
    let (mut deposits, mut cleared, mut restored, mut seeded, mut inert) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut fixed = false;
    let mut nonempty = false;

    sprintln!("  movement:");
    for (i, t) in steps.iter().enumerate() {
        let step = i + 1;
        let g = t.glyph();

        // The machine returns early once IFIX has fired: everything but IFIX
        // and IMSCRIB is inert. Counting a movement without the same guard
        // reports clears and fuses that never happened.
        if fixed && !matches!(t, Token16_3::Ifix | Token16_3::Imscrib) {
            inert += 1;
            continue;
        }

        match t {
            Token16_3::Fsplit3 => {
                frames.push([0; 4]);
                sprintln!("   {:>3} {}  open frame at depth {}", step, g, frames.len());
            }
            Token16_3::Ffuse3 => {
                if let Some(closed) = frames.pop() {
                    let mut got = 0u32;
                    for j in 0..4 {
                        if closed[j] > reg[j] { got += closed[j] - reg[j]; reg[j] = closed[j]; }
                        if let Some(outer) = frames.last_mut() {
                            if closed[j] > outer[j] { outer[j] = closed[j]; }
                        }
                    }
                    restored += got;
                    nonempty = reg.iter().any(|&w| w > 0);
                    sprintln!("   {:>3} {}  fuse restores {}", step, g, got);
                }
            }
            Token16_3::Arev | Token16_3::Vinit => {
                let lost: u32 = reg.iter().sum();
                let banked: u32 = frames.iter().map(|f| f.iter().sum::<u32>()).sum();
                cleared += lost;
                reg = [0; 4];
                if matches!(t, Token16_3::Vinit) { frames.clear(); }
                nonempty = false;
                sprintln!("   {:>3} {}  CLEAR loses {}   ({} banked in frames)",
                          step, g, lost, banked);
            }
            Token16_3::Afwd | Token16_3::Imscrib => {
                if !nonempty {
                    seeded += 1;
                    nonempty = true;
                    sprintln!("   {:>3} {}  SEED T into an empty register, no weight", step, g);
                }
            }
            Token16_3::Ifix => { fixed = true; }
            _ => {
                // The evaluators are the only depositors: EVALT touches T,
                // EVALF touches F, EVALI touches t and f together, which is
                // why the constructive pair is never seen split.
                let touched: &[usize] = match t {
                    Token16_3::Evalt => &[0],
                    Token16_3::Evalf => &[1],
                    Token16_3::Evali => &[2, 3],
                    _ => &[],
                };
                if !touched.is_empty() {
                    let mut names = String::new();
                    for &j in touched {
                        reg[j] += 1;
                        if let Some(f) = frames.last_mut() { f[j] += 1; }
                        if !names.is_empty() { names.push('+'); }
                        names.push_str(NAMES[j]);
                    }
                    deposits += 1;
                    nonempty = true;
                    sprintln!("   {:>3} {}  deposit {}   into depth {}",
                              step, g, names, frames.len());
                }
            }
        }
    }

    let mut surv = String::new();
    for j in 0..4 {
        if reg[j] > 0 {
            if !surv.is_empty() { surv.push_str(", "); }
            surv.push_str(&format!("{}×{}", NAMES[j], reg[j]));
        }
    }
    let stranded: u32 = frames.iter().map(|f| f.iter().sum::<u32>()).sum();
    sprintln!("");
    sprintln!("  final    : {}", run_word_register(&steps));
    sprintln!("  surviving: {}", if surv.is_empty() { "none" } else { &surv });
    sprintln!("  deposits {}  cleared {}  restored {}  seeded {}  inert {}",
              deposits, cleared, restored, seeded, inert);
    if stranded > 0 {
        sprintln!("  stranded in frames never fused: {}", stranded);
    }
}
