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
        s.push_str("  collatz junctions <lo> <hi>  where trajectories merge, by arm\n");
        s.push_str("  collatz balance <v> <depth>  the two subtrees feeding one junction\n");
        s.push_str("  collatz balanced <lo> <hi> <depth>  the junctions whose arms match\n");
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


    /// The junction census. A value takes two predecessors exactly when it is
    /// 2 (mod 3) — `2v` on the even arm and `(2v-1)/3` on the odd one — so every
    /// merge in the tree happens there and nowhere else. What is not settled by
    /// that is which junctions carry the traffic, and how the two arms share it.
    /// This walks every seed in the range and counts arrivals at each junction
    /// by the arm they came in on.
    pub fn junctions(lo: u64, hi: u64, top: usize) -> String {
        let mut s = format!("collatz junctions {}..{}\n", lo, hi);
        // sparse counts: (value, even arrivals, odd arrivals)
        let mut keys: Vec<u64> = Vec::new();
        let mut even: Vec<u64> = Vec::new();
        let mut odd: Vec<u64> = Vec::new();
        let mut seeds = 0u64;
        for n in lo..=hi {
            seeds += 1;
            let mut w = n;
            let mut guard = 0;
            while w != 1 && guard < 100_000 {
                let v = step(w);
                if v % 3 == 2 {
                    let from_even = w % 2 == 0 && w == 2 * v;
                    let idx = match keys.binary_search(&v) {
                        Ok(i) => i,
                        Err(i) => { keys.insert(i, v); even.insert(i, 0); odd.insert(i, 0); i }
                    };
                    if from_even { even[idx] += 1; } else { odd[idx] += 1; }
                }
                w = v;
                guard += 1;
            }
        }
        let mut order: Vec<usize> = (0..keys.len()).collect();
        order.sort_by_key(|&i| core::cmp::Reverse(even[i] + odd[i]));
        s.push_str(&format!("  seeds walked:    {}\n", seeds));
        s.push_str(&format!("  junctions used:  {}\n\n", keys.len()));
        s.push_str("       junction     traffic     even arm      odd arm   odd share\n");
        let mut tot_e: u64 = 0;
        let mut tot_o: u64 = 0;
        for &i in order.iter() { tot_e += even[i]; tot_o += odd[i]; }
        for &i in order.iter().take(top) {
            let t = even[i] + odd[i];
            s.push_str(&format!("  {:>13}  {:>10}  {:>11}  {:>11}  {:>10.4}\n",
                keys[i], t, even[i], odd[i], odd[i] as f64 / t as f64));
        }
        // the shape of the split across junctions, not just its total: a
        // junction that carries traffic on one arm only is doing no mixing,
        // whatever the global share says.
        let mut bins = [0u64; 10];
        let mut weight = [0u64; 10];
        for i in 0..keys.len() {
            let t = even[i] + odd[i];
            if t == 0 { continue; }
            let sh = odd[i] as f64 / t as f64;
            let mut b = (sh * 10.0) as usize;
            if b > 9 { b = 9; }
            bins[b] += 1;
            weight[b] += t;
        }
        s.push_str("\n  odd share    junctions      traffic\n");
        for b in 0..10 {
            s.push_str(&format!("  {:.1}-{:.1}  {:>12}  {:>11}\n",
                b as f64 / 10.0, (b + 1) as f64 / 10.0, bins[b], weight[b]));
        }
        s.push_str(&format!("\n  all junctions:   even {}   odd {}   odd share {:.4}\n",
            tot_e, tot_o, tot_o as f64 / (tot_e + tot_o) as f64));
        s.push_str("  the odd share is the fraction of arrivals that used the arm only\n");
        s.push_str("  a 2 (mod 3) value has; the even arm is always available.\n");
        s
    }


    /// How many predecessors sit under a value, out to a given depth. The
    /// backward step is `2m` always, and `(2m-1)/3` when `m = 2 (mod 3)`; the
    /// edge back into the root cycle is cut so the count is of the tree rather
    /// than of the loop.
    pub fn subtree(start: u64, depth: u32) -> u64 {
        let mut level: Vec<u64> = alloc::vec![start];
        let mut total: u64 = 1;
        for _ in 0..depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                if m <= u64::MAX / 2 { next.push(2 * m); }
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 && u % 2 == 1 { next.push(u); }
                }
            }
            total += next.len() as u64;
            level = next;
            if level.is_empty() { break; }
        }
        total
    }

    /// What makes a junction balanced. Traffic measured from a window of seeds
    /// is a fact about the window; the intrinsic quantity is how much tree feeds
    /// each arm. This counts both subtrees to a common depth and reports the
    /// share the odd arm holds.
    pub fn balance(v: u64, depth: u32) -> String {
        let mut s = format!("collatz balance {} at depth {}\n", v, depth);
        if v % 3 != 2 {
            s.push_str(&format!("  {} is {} (mod 3) — one arm only, so it is no junction\n",
                v, v % 3));
            return s;
        }
        let even_arm = 2 * v;
        let odd_arm = (2 * v - 1) / 3;
        let e = Self::subtree(even_arm, depth);
        let o = Self::subtree(odd_arm, depth);
        s.push_str(&format!("  even arm  {:>12}   subtree {:>10}\n", even_arm, e));
        s.push_str(&format!("  odd arm   {:>12}   subtree {:>10}\n", odd_arm, o));
        s.push_str(&format!("  odd share {:.4}\n", o as f64 / (e + o) as f64));
        s
    }

    /// Scan the junctions for the balanced ones. Balance is intrinsic here, read
    /// off the two subtrees rather than off a seed window, so the answer does
    /// not move when the window does.
    pub fn balanced(lo: u64, hi: u64, depth: u32) -> String {
        let mut s = format!("collatz balanced {}..{} at depth {}\n", lo, hi, depth);
        s.push_str("  junctions whose two arms feed within a tenth of each other\n\n");
        s.push_str("       junction     even arm      odd arm   odd share\n");
        let mut seen = 0u64;
        let mut hits = 0u64;
        let mut share_sum = 0.0f64;
        let mut spread = [0u64; 10];
        for v in lo..=hi {
            if v % 3 != 2 { continue; }
            seen += 1;
            let e = Self::subtree(2 * v, depth);
            let o = Self::subtree((2 * v - 1) / 3, depth);
            let sh = o as f64 / (e + o) as f64;
            share_sum += sh;
            let mut b = (sh * 10.0) as usize;
            if b > 9 { b = 9; }
            spread[b] += 1;
            if sh > 0.4 && sh < 0.6 {
                hits += 1;
                if hits <= 30 {
                    s.push_str(&format!("  {:>13}  {:>11}  {:>11}  {:>10.4}\n",
                        v, e, o, sh));
                }
            }
        }
        s.push_str(&format!("\n  junctions scanned: {}\n", seen));
        s.push_str(&format!("  balanced:          {}   ({:.4} of them)\n",
            hits, hits as f64 / seen.max(1) as f64));
        s.push_str(&format!("  mean odd share:    {:.4}\n", share_sum / seen.max(1) as f64));
        s.push_str("  share spread:");
        for b in 0..10 {
            s.push_str(&format!("  {:.1}:{}", b as f64 / 10.0, spread[b]));
        }
        s.push('\n');
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
