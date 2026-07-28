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

/// The 12-op alphabet writes split and fuse as ◇ and ●; the trilattice core
/// writes them ∈ and ∋. A word copied out of a bootstrap report is in the
/// first and the machine reads the second, so translate rather than drop.
fn normalize(word: &str) -> String {
    let mut out = String::new();
    for c in word.chars() {
        match c {
            '◇' | '⊗' => out.push('∈'),
            '●' | '⊕' => out.push('∋'),
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

/// Was anything counted, then cleared with nothing banked behind it?
///
/// AREV empties the register and leaves open frames alone, so a result fused
/// back to depth zero is exposed to the next reversal, while the same result
/// held one level up survives it. A program that establishes something, then
/// reverses, then bounds must open the region that will HOLD the result before
/// the region that COMPUTES it, and close them in that order.
pub fn banked_report(word: &str) {
    let steps = parse_glyph_word(&normalize(word));
    if steps.is_empty() {
        sprintln!("  no IMASM glyphs in that word");
        return;
    }
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

    sprintln!("word   : {}", render(&steps));
    if exposed.is_empty() && live_clears == 0 {
        // Passing because nothing was ever at risk is not the same as passing
        // because the frame held.
        sprintln!("  VACUOUS — no clear ever fired against a live register");
        sprintln!("    {} deposit(s), {} step(s) inert after a fixation", deposits, inert);
    } else if exposed.is_empty() {
        sprintln!("  OK — weight survived {} live clear(s) by being banked", live_clears);
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
