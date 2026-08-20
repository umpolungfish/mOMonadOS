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


/// Natural log, since the kernel's float shims carry sqrt and exp but no ln and
/// this is no_std. Range-reduce by powers of two, then the atanh series, which
/// converges fast once the argument sits near one.
pub fn f64_ln(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    const LN2: f64 = 0.693147180559945309417232121458;
    let mut y = x;
    let mut k = 0i32;
    while y >= 2.0 { y /= 2.0; k += 1; }
    while y < 1.0 { y *= 2.0; k -= 1; }
    // ln y = 2 atanh(t), t = (y-1)/(y+1)
    let t = (y - 1.0) / (y + 1.0);
    let t2 = t * t;
    let mut term = t;
    let mut sum = t;
    let mut n = 3.0f64;
    for _ in 0..40 {
        term *= t2;
        sum += term / n;
        n += 2.0;
    }
    2.0 * sum + k as f64 * LN2
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
        s.push_str("  collatz classes <mod> <n> <depth>   is the share fixed by the residue\n");
        s.push_str("  collatz adic <digits> <n> <depth>   the 3-adic map of the share\n");
        s.push_str("  collatz growth <v> <dmax>           the amplitude under a value\n");
        s.push_str("  collatz amplitudes <lo> <hi> <d>    the amplitude over a range, with its recursion\n");
        s.push_str("  collatz birkhoff <lo> <hi> <d>      the cocycle weight averaged along trajectories\n");
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


    /// What fixes the share. The spectrum of odd shares is discrete, with a
    /// third of junctions starved and nothing at all in whole intervals, so the
    /// share is not a continuum being sampled — something finite is setting it.
    /// This groups junctions by residue and reports the spread inside each
    /// class: a share that is constant on a class is a share the residue fixes.
    pub fn classes(modulus: u64, count: u64, depth: u32) -> String {
        let mut s = format!("collatz classes mod {} over {} junctions at depth {}\n",
            modulus, count, depth);
        let m = modulus as usize;
        let mut n_in = alloc::vec![0u64; m];
        let mut sum = alloc::vec![0.0f64; m];
        let mut lo = alloc::vec![2.0f64; m];
        let mut hi = alloc::vec![-1.0f64; m];
        let mut v = 2u64;
        let mut seen = 0u64;
        while seen < count {
            if v % 3 == 2 {
                let e = Self::subtree(2 * v, depth);
                let o = Self::subtree((2 * v - 1) / 3, depth);
                let sh = o as f64 / (e + o) as f64;
                let c = (v % modulus) as usize;
                n_in[c] += 1;
                sum[c] += sh;
                if sh < lo[c] { lo[c] = sh; }
                if sh > hi[c] { hi[c] = sh; }
                seen += 1;
            }
            v += 1;
        }
        s.push_str("      class      n     mean       min       max      spread\n");
        let mut worst = 0.0f64;
        for c in 0..m {
            if n_in[c] == 0 { continue; }
            let sp = hi[c] - lo[c];
            if sp > worst { worst = sp; }
            s.push_str(&format!("  {:>9}  {:>5}  {:>7.4}  {:>8.4}  {:>8.4}  {:>10.4}\n",
                c, n_in[c], sum[c] / n_in[c] as f64, lo[c], hi[c], sp));
        }
        s.push_str(&format!("\n  widest spread inside a class: {:.4}\n", worst));
        s.push_str("  a share the residue fixes shows a spread at zero; a share it only\n");
        s.push_str("  biases keeps a spread while the means separate.\n");
        s
    }


    /// The odd share of one residue class, sampled.
    fn class_share(c: u64, modulus: u64, samples: u64, depth: u32) -> (f64, f64, f64) {
        let mut n = 0u64;
        let mut sum = 0.0f64;
        let mut lo = 2.0f64;
        let mut hi = -1.0f64;
        let mut v = c;
        // A junction smaller than the depth being counted has its arms cut short
        // by the root, not by its own branching, so it reports the window rather
        // than the class. Start above that.
        let floor = 4 * depth as u64;
        while v < floor.max(2) { v += modulus; }
        while n < samples {
            if v % 3 == 2 {
                let e = Self::subtree(2 * v, depth);
                let o = Self::subtree((2 * v - 1) / 3, depth);
                let sh = o as f64 / (e + o) as f64;
                sum += sh;
                if sh < lo { lo = sh; }
                if sh > hi { hi = sh; }
                n += 1;
            } else {
                return (-1.0, 0.0, 0.0);      // the class holds no junctions
            }
            v += modulus;
        }
        (sum / n as f64, lo, hi)
    }

    /// The 3-adic map of the share. Each digit of a junction's base-3 address
    /// pins its odd share further, so the object that sets the whole spectrum is
    /// a function on the 3-adics. This walks the digit tree, refining every
    /// class into its three children and reporting what each digit buys: the
    /// mean it moves to, and the spread it has not yet resolved.
    pub fn adic(digits: u32, samples: u64, depth: u32) -> String {
        let mut s = format!("collatz adic — {} digit(s), {} sample(s) per class, subtree depth {}\n",
            digits, samples, depth);
        s.push_str("  a junction is 2 (mod 3); each further digit splits a class in three\n\n");
        s.push_str("  class (mod)          mean      spread   resolved\n");
        // breadth-first over the digit tree, keeping only classes that hold junctions
        let mut frontier: Vec<(u64, u64)> = alloc::vec![(2, 3)];
        for k in 1..=digits {
            let mut next: Vec<(u64, u64)> = Vec::new();
            for &(c, m) in frontier.iter() {
                let (mean, lo, hi) = Self::class_share(c, m, samples, depth);
                if mean < 0.0 { continue; }
                let spread = hi - lo;
                let bar = if spread < 0.02 { "pinned" }
                          else if spread < 0.08 { "close" }
                          else { "open" };
                let indent = (k as usize - 1) * 2;
                for _ in 0..indent { s.push(' '); }
                s.push_str(&format!("  {:>8} (mod {:<6})  {:>7.4}  {:>10.4}   {}\n",
                    c, m, mean, spread, bar));
                if k < digits {
                    for j in 0..3u64 { next.push((c + j * m, m * 3)); }
                }
            }
            frontier = next;
            if frontier.is_empty() { break; }
        }
        s.push_str("\n  a digit that pins a class has done its work; one that leaves it open\n");
        s.push_str("  hands the rest to the digit below.\n");
        s
    }


    /// The amplitude under a value. Every subtree grows like (4/3)^d, so what
    /// separates one arm from another is the constant in front. This reports the
    /// per-level ratio and the amplitude S(v,d) / (4/3)^d, which is the object
    /// the share is a ratio of.
    pub fn growth(v: u64, dmax: u32) -> String {
        let mut s = format!("collatz growth {} to depth {}\n", v, dmax);
        s.push_str("     depth      subtree    ratio    amplitude\n");
        let mut prev = 0u64;
        for d in 1..=dmax {
            let n = Self::subtree(v, d);
            let ratio = if prev > 0 { n as f64 / prev as f64 } else { 0.0 };
            let mut scale = 1.0f64;
            for _ in 0..d { scale *= 4.0 / 3.0; }
            s.push_str(&format!("  {:>8}  {:>11}  {:>7.4}  {:>11.4}\n",
                d, n, ratio, n as f64 / scale));
            prev = n;
        }
        s
    }


    /// The amplitude itself, over a range. A(v) is the constant in front of
    /// (4/3)^d in the subtree count, so it is what the share is a ratio of, and
    /// it is the unknown in
    ///     A(v) = (3/4) ( A(2v) + [v = 2 mod 3] A((2v-1)/3) ).
    /// Multiples of three carry amplitude zero — their arm is a bare chain — and
    /// the verb marks them rather than printing a vanishing number as if it were
    /// a measurement.
    pub fn amplitudes(lo: u64, hi: u64, depth: u32) -> String {
        let mut s = format!("collatz amplitudes {}..{} at depth {}\n", lo, hi, depth);
        s.push_str("            v   v%3   v%9      subtree     amplitude   check (3/4)(sum of arms)\n");
        let mut scale = 1.0f64;
        for _ in 0..depth { scale *= 4.0 / 3.0; }
        for v in lo..=hi {
            if v % 3 == 0 {
                s.push_str(&format!("  {:>11}  {:>4}  {:>4}   {:>10}     {:>9}   barren chain\n",
                    v, 0, v % 9, Self::subtree(v, depth), "0"));
                continue;
            }
            let a = Self::subtree(v, depth) as f64 / scale;
            // the recursion, read one level down and compared
            let a2 = Self::subtree(2 * v, depth) as f64 / scale;
            let arm = if v % 3 == 2 {
                Self::subtree(2 * (v / 3) + 1, depth) as f64 / scale
            } else { 0.0 };
            let check = 0.75 * (a2 + arm);
            s.push_str(&format!("  {:>11}  {:>4}  {:>4}   {:>10}     {:>9.4}   {:>9.4}  ({:+.2}%)\n",
                v, v % 3, v % 9, Self::subtree(v, depth), a, check,
                100.0 * (check - a) / a));
        }
        s
    }


    /// The Birkhoff average of the cocycle weight along a trajectory.
    ///
    /// Each step multiplies the amplitude by (4/3)·w, so
    ///     w = (3/4) · S(n, d) / S(T n, d)
    /// reads straight off the counts with no share computed separately. The sum
    /// of log w telescopes, so the average is pinned by the two endpoints — and
    /// since the amplitude stays bounded while the trajectory length grows, the
    /// average must approach -log(4/3) and the geometric mean of the weights
    /// must approach 3/4. That is a prediction the walk can refuse.
    ///
    /// What does not telescope is which junctions a trajectory actually visits.
    /// The verb reports the visited weights beside the count, so the measure the
    /// forward walk puts on the tree can be compared with the tree's own.
    pub fn birkhoff(lo: u64, hi: u64, depth: u32) -> String {
        let mut s = format!("collatz birkhoff {}..{} at subtree depth {}\n", lo, hi, depth);
        let mut seeds = 0u64;
        let mut steps_tot = 0u64;
        let mut odd_steps = 0u64;
        let mut log_sum = 0.0f64;
        let mut w_sum = 0.0f64;
        let mut w_min = 2.0f64;
        let mut w_max = -1.0f64;
        for n0 in lo..=hi {
            let mut n = n0;
            let mut guard = 0;
            while n > 4 && guard < 4096 {
                let t = step(n);
                let sn = Self::subtree(n, depth) as f64;
                let st = Self::subtree(t, depth) as f64;
                let w = 0.75 * sn / st;
                if n % 2 == 1 { odd_steps += 1; }
                log_sum += f64_ln(w);
                w_sum += w;
                if w < w_min { w_min = w; }
                if w > w_max { w_max = w; }
                steps_tot += 1;
                n = t;
                guard += 1;
            }
            seeds += 1;
        }
        let mean_log = log_sum / steps_tot as f64;
        let target = -f64_ln(4.0 / 3.0);
        s.push_str(&format!("  seeds:              {}\n", seeds));
        s.push_str(&format!("  steps walked:       {}\n", steps_tot));
        s.push_str(&format!("  odd-step fraction:  {:.4}\n",
            odd_steps as f64 / steps_tot as f64));
        s.push_str(&format!("  mean log w:         {:.6}\n", mean_log));
        s.push_str(&format!("  -log(4/3):          {:.6}\n", target));
        s.push_str(&format!("  gap:                {:+.6}\n", mean_log - target));
        s.push_str(&format!("  geometric mean w:   {:.6}   (3/4 = 0.750000)\n",
            crate::constant_closure::f64_exp(mean_log)));
        s.push_str(&format!("  arithmetic mean w:  {:.6}\n", w_sum / steps_tot as f64));
        s.push_str(&format!("  w range:            {:.4} .. {:.4}\n", w_min, w_max));
        // The sum telescopes, so the gap is one bounded endpoint term divided by
        // the length. Reporting gap x length is what shows it is bounded: a
        // constant here IS the boundedness of the cocycle, and with it the
        // average is log(3/4) exactly rather than approximately.
        let mean_len = steps_tot as f64 / seeds.max(1) as f64;
        s.push_str(&format!("  mean length:        {:.2}\n", mean_len));
        s.push_str(&format!("  gap x length:       {:+.4}   (constant = the cocycle is bounded)\n",
            (mean_log - target) * mean_len));
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
