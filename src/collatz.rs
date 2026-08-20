// src/collatz.rs — the Collatz object as a nesting, read the way straus reads
// the Erdős–Straus ladder.
//
// The action is one BLOCK: from n, apply the shortcut map n/2 | (3n+1)/2 until
// the value first falls below n. Every block strictly decreases, so the nest is
// monotone in the value and one is the fixed point, held outright. Collatz is
// therefore a claim about the BUDGET on this nesting — how many blocks — and not
// about whether it arrives, exactly as Erdős–Straus is a budget on greedy
// removal. What is open sits inside the action rather than around it: a block
// closes when the first drop exists, and the depth split is the measurement of
// that.
//
// Author: Quantum⊙perator (Lando⊗⊙perator team)

#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The shortcut map itself.
pub fn step(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { (3 * n + 1) / 2 }
}

/// One block: the first value below `n`, and what the block cost in steps.
/// A block that does not close inside its allowance returns `None`, which is
/// the open arm reporting itself rather than a wrong answer.
pub fn block(n: u64, allowance: u32) -> Option<(u64, u32)> {
    if n <= 1 { return Some((1, 0)); }
    let mut v = n;
    let mut steps = 0u32;
    while steps < allowance {
        v = step(v);
        steps += 1;
        if v < n { return Some((v, steps)); }
    }
    None
}

/// The budget: blocks to reach one, and the total shortcut steps they cost.
pub fn budget(n: u64, allowance: u32) -> Option<(u32, u32, u64)> {
    let mut v = n;
    let mut blocks = 0u32;
    let mut total = 0u32;
    let mut peak = n;
    while v > 1 {
        let (next, cost) = block(v, allowance)?;
        // the peak is read inside the block, since that is where the value rises
        let mut w = v;
        for _ in 0..cost { w = step(w); if w > peak { peak = w; } }
        v = next;
        blocks += 1;
        total += cost;
    }
    Some((blocks, total, peak))
}

pub struct Collatz;

impl Collatz {
    pub fn help() -> String {
        let mut s = String::from("collatz — the block nesting, and the budget it spends\n\n");
        s.push_str("The action is one BLOCK: from n, run the shortcut map until the value\n");
        s.push_str("first falls below n. Every block strictly decreases, so the nest is\n");
        s.push_str("monotone in the value and one is held outright. The conjecture is the\n");
        s.push_str("BUDGET on this nesting, not its arrival — the same shape `nesting`\n");
        s.push_str("assigns Erdős–Straus through `greedy`.\n\n");
        s.push_str("  collatz <n>              blocks, steps and peak for one n\n");
        s.push_str("  collatz trace <n>        every block, with the gap ratio it closes\n");
        s.push_str("  collatz merge <a> <b>    where two trajectories first coincide\n");
        s.push_str("  collatz chain <n>        the record chain n -> (4n-1)/3\n");
        s.push_str("  collatz sweep <lo> <hi>  the budget spectrum across a range\n");
        s.push_str("  collatz ceiling <lo> <hi>  the record budgets and where they fall\n");
        s.push_str("  collatz help             this\n\n");
        s.push_str("example:  collatz 27 — seven blocks, and the peak the value reaches\n");
        s
    }

    pub fn one(n: u64) -> String {
        match budget(n, 100_000) {
            None => format!("collatz {}: a block exceeded its allowance — the arm is open here\n", n),
            Some((b, t, peak)) => {
                let mut s = format!("collatz {}\n", n);
                s.push_str(&format!("  blocks to one:   {}\n", b));
                s.push_str(&format!("  shortcut steps:  {}\n", t));
                s.push_str(&format!("  peak value:      {}   ({:.3}x the seed)\n",
                    peak, peak as f64 / n as f64));
                s.push_str(&format!("  budget per bit:  {:.4}\n",
                    b as f64 / (64 - n.leading_zeros()) as f64));
                s
            }
        }
    }

    pub fn trace(n: u64) -> String {
        let mut s = format!("collatz trace {}\n", n);
        s.push_str("  block      from -> to     cost   gap ratio\n");
        let mut v = n;
        let mut i = 0;
        let mut prev_gap = (n - 1) as f64;
        while v > 1 && i < 512 {
            match block(v, 100_000) {
                None => { s.push_str("  a block exceeded its allowance — open here\n"); break; }
                Some((next, cost)) => {
                    let gap = (next - 1) as f64;
                    let q = if prev_gap > 0.0 { gap / prev_gap } else { 0.0 };
                    s.push_str(&format!("  {:>5}  {:>9} -> {:<9} {:>4}   {:.6}\n",
                        i + 1, v, next, cost, q));
                    prev_gap = gap;
                    v = next;
                    i += 1;
                }
            }
        }
        s.push_str(&format!("  arrived at one in {} block(s)\n", i));
        s
    }


    /// Where two trajectories first land on the same value. Two numbers with
    /// nothing to do with each other merge only in the tail every trajectory
    /// shares; two sitting on one branch of the predecessor tree merge at the
    /// branch point, high up and reached fast. The verb answers which.
    pub fn merge(a: u64, b: u64) -> String {
        let mut s = format!("collatz merge {} {}\n", a, b);
        let mut path_a: Vec<u64> = Vec::new();
        let mut v = a;
        let mut guard = 0;
        while v != 1 && guard < 100_000 { path_a.push(v); v = step(v); guard += 1; }
        path_a.push(1);
        let mut v = b;
        let mut steps_b = 0u32;
        guard = 0;
        while guard < 100_000 {
            if let Some(i) = path_a.iter().position(|&x| x == v) {
                s.push_str(&format!("  meet value:      {}\n", v));
                s.push_str(&format!("  steps from {}:   {}\n", a, i));
                s.push_str(&format!("  steps from {}:   {}\n", b, steps_b));
                s.push_str(&format!("  shared tail:     {} of {} steps on the first path\n",
                    path_a.len() - i, path_a.len()));
                s.push_str(&format!("  the meet sits:   {}\n",
                    if v > a.max(b) { "ABOVE both seeds — they join on the way up" }
                    else if v > a.min(b) { "above the smaller seed only" }
                    else { "below both — they join only in the common tail" }));
                return s;
            }
            v = step(v);
            steps_b += 1;
        }
        s.push_str("  no meeting inside the allowance\n");
        s
    }


    /// The chain the budget records climb. Two backward steps — one doubling and
    /// one odd lift, the two arms of the split composed once each — send n to
    /// (4n-1)/3, defined exactly when n = 1 (mod 3). Consecutive records follow
    /// that rule wherever it stays defined, which is why their ratio sits at
    /// four thirds; where it fails the record switches chains instead.
    pub fn chain(n: u64) -> String {
        let mut s = format!("collatz chain {}\n", n);
        s.push_str("  n -> (4n-1)/3, one doubling and one odd lift\n\n");
        let mut v = n;
        let mut i = 0;
        while v % 3 == 1 && i < 64 {
            let next = (4 * v - 1) / 3;
            let b = budget(v, 100_000).map(|t| t.0).unwrap_or(0);
            let bn = budget(next, 100_000).map(|t| t.0).unwrap_or(0);
            s.push_str(&format!("  {:>12} (budget {:>3})  ->  {:>12} (budget {:>3})   ratio {:.6}\n",
                v, b, next, bn, next as f64 / v as f64));
            v = next;
            i += 1;
        }
        s.push_str(&format!("  chain ends at {} — it is {} (mod 3), so the odd lift has no arm here\n",
            v, v % 3));
        s
    }

    pub fn sweep(lo: u64, hi: u64) -> String {
        let mut s = format!("collatz sweep {}..{}\n", lo, hi);
        let (mut maxb, mut argb) = (0u32, lo);
        let (mut maxc, mut argc) = (0u32, lo);
        let (mut maxp, mut argp) = (0.0f64, lo);
        let mut open = 0u64;
        let mut total_b: u64 = 0;
        let mut count: u64 = 0;
        for n in lo..=hi {
            match budget(n, 100_000) {
                None => { open += 1; }
                Some((b, _t, peak)) => {
                    count += 1;
                    total_b += b as u64;
                    if b > maxb { maxb = b; argb = n; }
                    let (bl, cost) = block(n, 100_000).unwrap();
                    let _ = bl;
                    if cost > maxc { maxc = cost; argc = n; }
                    let ratio = peak as f64 / n as f64;
                    if ratio > maxp { maxp = ratio; argp = n; }
                }
            }
        }
        s.push_str(&format!("  values read:       {}\n", count));
        s.push_str(&format!("  blocks open:       {}\n", open));
        s.push_str(&format!("  mean budget:       {:.4}\n",
            if count > 0 { total_b as f64 / count as f64 } else { 0.0 }));
        s.push_str(&format!("  max budget:        {} at n = {}\n", maxb, argb));
        s.push_str(&format!("  costliest block:   {} steps at n = {}\n", maxc, argc));
        s.push_str(&format!("  highest peak:      {:.3}x at n = {}\n", maxp, argp));
        s
    }

    pub fn ceiling(lo: u64, hi: u64) -> String {
        let mut s = format!("collatz ceiling {}..{}\n", lo, hi);
        s.push_str("  records in the budget, each the first of its height\n\n");
        s.push_str("          n     blocks    steps    peak/n\n");
        let mut best = 0u32;
        let mut rows: Vec<(u64, u32, u32, f64)> = Vec::new();
        for n in lo..=hi {
            if let Some((b, t, peak)) = budget(n, 100_000) {
                if b > best {
                    best = b;
                    rows.push((n, b, t, peak as f64 / n as f64));
                }
            }
        }
        for (n, b, t, p) in &rows {
            s.push_str(&format!("  {:>9}  {:>7}  {:>7}  {:>8.3}\n", n, b, t, p));
        }
        if rows.len() >= 2 {
            let first = rows[0];
            let last = rows[rows.len() - 1];
            let span = (last.0 as f64) / (first.0.max(1) as f64);
            s.push_str(&format!("\n  {} record(s); the budget grew {} -> {} while n grew {:.0}x\n",
                rows.len(), first.1, last.1, span));
        }
        s
    }
}
