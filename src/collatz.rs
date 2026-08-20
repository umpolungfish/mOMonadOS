#![allow(dead_code)]

// src/collatz.rs — the Collatz object as a nesting, read the way straus reads
// the Erdős–Straus ladder.
//
/// The action is one BLOCK: from n, apply the shortcut map n/2 | (3n+1)/2 until
/// the value first falls below n. Every block strictly decreases, so the nest is
/// monotone in the value and one is the fixed point, held outright. Collatz is
/// therefore a claim about the BUDGET on this nesting — how many blocks — and not
/// about whether it arrives, exactly as Erdős–Straus is a budget on greedy
/// removal. What is open sits inside the action rather than around it: a block
/// closes when the first drop exists, and the depth split is the measurement of
/// that.
///
// Author: Quantum⊙perator (Lando⊗⊙perator team)
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


/// Sine and cosine by Taylor with range reduction, since the kernel's float
/// shims carry sqrt and exp and this is no_std. Only needed to a few ulp: the
/// character sums below are read at three or four digits.
const TAU: f64 = 6.283185307179586476925286766559;

pub fn f64_sin(x: f64) -> f64 {
    let mut t = x;
    while t > TAU / 2.0 { t -= TAU; }
    while t < -TAU / 2.0 { t += TAU; }
    let t2 = t * t;
    let mut term = t;
    let mut sum = t;
    let mut n = 1.0f64;
    for _ in 0..24 {
        n += 2.0;
        term *= -t2 / (n * (n - 1.0));
        sum += term;
    }
    sum
}

pub fn f64_cos(x: f64) -> f64 { f64_sin(x + TAU / 4.0) }

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
        s.push_str("  collatz amax <lo> <hi> <d>          the largest amplitude in a window\n");
        s.push_str("  collatz fourier <depth> <rmax>      the character sums of the tree measure\n");
        s.push_str("  collatz flow <depth> <r>            the two terms of the level map on coefficients\n");
        s.push_str("  collatz collisions <depth> <r>      equidistribution as a collision count\n");
        s.push_str("  collatz excess <depth> <r>          the excess recursion and its prediction\n");
        s.push_str("  collatz perturb <depth>             the involution and its odd-arm perturbation\n");
        s.push_str("  collatz perturb9 <depth>            the conductor-9 identity, exact\n");
        s.push_str("  collatz norm <depth> [rungs]        the weighted excess norm and its contraction\n");
        s.push_str("  collatz disjunct <depth>            is the weighted cross sum carried by one rung\n");
        s.push_str("  collatz attack <depth> <rungs> <minN>  hunt a counterexample to the contraction\n");
        s.push_str("  collatz jratio <depth> <rungs>      the junction excess ratio the CS route turns on\n");
        s.push_str("  collatz lag <depth> <r>             the cross term against the lag average\n");
        s.push_str("  collatz lambda <depth>              the proportionality of the two arms at conductor 3\n");
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


    /// The largest amplitude in a window. Boundedness of the cocycle is exactly
    /// the claim that this does not drift upward as the window widens, so the
    /// verb reports the maximum and where it sits rather than an average that
    /// would hide it. The Fibonacci ceiling proved in the Lean module allows
    /// phi^d; what is measured here is whether anything approaches it.
    pub fn amax(lo: u64, hi: u64, depth: u32) -> String {
        let mut s = format!("collatz amax {}..{} at depth {}\n", lo, hi, depth);
        let mut scale = 1.0f64;
        let mut fib_scale = 1.0f64;
        let phi = 1.6180339887498949;
        for _ in 0..depth { scale *= 4.0 / 3.0; fib_scale *= phi; }
        let mut best = 0.0f64;
        let mut arg = lo;
        let mut worst_ratio = 0.0f64;
        let mut worst_arg = lo;
        let mut sum = 0.0f64;
        let mut n = 0u64;
        for v in lo..=hi {
            if v % 3 == 0 { continue; }
            let c = Self::subtree(v, depth) as f64;
            let a = c / scale;
            sum += a;
            n += 1;
            if a > best { best = a; arg = v; }
            let phi_share = c / fib_scale;
            if phi_share > worst_ratio { worst_ratio = phi_share; worst_arg = v; }
        }
        s.push_str(&format!("  values read:        {}\n", n));
        s.push_str(&format!("  mean amplitude:     {:.4}\n", sum / n.max(1) as f64));
        s.push_str(&format!("  max amplitude:      {:.4}  at v = {}\n", best, arg));
        s.push_str(&format!("  max against phi^d:  {:.3e}  at v = {}\n", worst_ratio, worst_arg));
        s.push_str("  a max that holds as the window widens is the cocycle staying bounded;\n");
        s.push_str("  one that climbs with the window is the thing that would break it.\n");
        s
    }


    /// The character sums of the tree measure, level by level.
    ///
    /// Equidistribution mod 3^r is exactly the vanishing of every nonprincipal
    /// character sum, so this measures them directly rather than through a
    /// maximum deviation, which throws the signs away. Doubling permutes the
    /// characters of a fixed conductor while the odd arm sends a conductor 3^r
    /// character to one of conductor 3^(r+1), so the coefficient at each level is
    /// fed from the level above and never from below: what the numbers show is
    /// whether that flow decays.
    pub fn fourier(depth: u32, rmax: u32) -> String {
        let mut s = format!("collatz fourier to depth {} for conductors up to 3^{}\n", depth, rmax);
        s.push_str("  |mu-hat(chi)| for the first nonprincipal character of each conductor\n\n");
        s.push_str("  level      nodes");
        for r in 1..=rmax { s.push_str(&format!("     3^{}", r)); }
        s.push_str("\n");
        let mut level: Vec<u64> = alloc::vec![1];
        let mut prev: Vec<f64> = alloc::vec![0.0; rmax as usize];
        for d in 1..=depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                next.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            level = next;
            if level.is_empty() { break; }
            let n = level.len() as f64;
            s.push_str(&format!("  {:>5}  {:>9}", d, level.len()));
            let mut cur: Vec<f64> = Vec::new();
            let mut phase1 = 0.0f64;
            for r in 1..=rmax {
                let modulus = 3u64.pow(r);
                let mut re = 0.0f64;
                let mut im = 0.0f64;
                for &m in level.iter() {
                    let ang = TAU * ((m % modulus) as f64) / (modulus as f64);
                    re += f64_cos(ang);
                    im += f64_sin(ang);
                }
                let mag = crate::constant_closure::f64_sqrt(re * re + im * im) / n;
                if r == 1 {
                    // signed imbalance read on the real axis, the cleanest form of
                    // the alternation at conductor three
                    phase1 = re / n;
                }
                cur.push(mag);
                s.push_str(&format!("  {:>7.4}", mag));
            }
            // The size that matters is not the coefficient but the coefficient
            // times root N. A sample drawn uniformly has character sums of order
            // one over root N, so this column reading order one IS the
            // equidistribution, at the strongest rate there is.
            let root_n = crate::constant_closure::f64_sqrt(n);
            s.push_str("   x sqrt(N)");
            for r in 0..rmax as usize {
                s.push_str(&format!(" {:>6.3}", cur[r] * root_n));
            }
            // Doubling has order two on the live classes mod 3, so it is an
            // involution there and the coefficient's phase conjugates each level.
            // An imbalance therefore alternates in sign, which is the source of
            // the negative correlation between a level and its own image.
            s.push_str(&format!("   arg1 {:>+7.3}", phase1));
            let _ = &prev;
            s.push_str("\n");
            prev = cur;
        }
        s.push_str("\n  a coefficient falling geometrically is the equidistribution happening;\n");
        s.push_str("  one that holds is the flow between conductors sustaining itself.\n");
        s
    }


    /// The two terms of the level map on Fourier coefficients, measured.
    ///
    /// One level sends
    ///     mu(j, r)  ->  (3/4) [ mu(2j, r) + e(-j/3^(r+1)) (1/3) sum_s w^(-2s)
    ///                            mu(2j + s 3^r, r+1) ]
    /// with w a cube root of unity. The first term is the doubling permutation,
    /// a contraction by exactly 3/4. The second is the odd arm, and because
    /// sum_s w^(-2s) = 0 it sees only the DIFFERENCE of the three lifts, never
    /// their size — a constant across the lifts cancels exactly.
    ///
    /// So the contraction is 3/4 plus whatever the spread at the conductor above
    /// contributes. This verb reports both terms and their sum against the
    /// coefficient the next level actually has, which checks the identity and
    /// says which term carries the level.
    pub fn flow(depth: u32, r: u32) -> String {
        let mut s = format!("collatz flow at conductor 3^{} to depth {}\n", r, depth);
        s.push_str("  |same| is the doubling term, |feed| the odd arm's difference term\n\n");
        s.push_str("  level      nodes     |same|     |feed|    |sum|    |next|   feed/same\n");
        let modulus = 3u64.pow(r);
        let fine = 3u64.pow(r + 1);
        let mut level: Vec<u64> = alloc::vec![1];
        let mut canc_sum = 0.0f64;
        let mut canc_n = 0u64;
        // coefficient of the level set at (j, modulus)
        let coeff = |lvl: &Vec<u64>, j: u64, m: u64| -> (f64, f64) {
            let n = lvl.len() as f64;
            let mut re = 0.0;
            let mut im = 0.0;
            for &v in lvl.iter() {
                let ang = TAU * ((j * (v % m)) % m) as f64 / m as f64;
                re += f64_cos(ang);
                im += f64_sin(ang);
            }
            (re / n, im / n)
        };
        for d in 1..=depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                next.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            let ratio = level.len() as f64 / next.len() as f64;   // = 3/4 in the limit
            // same-conductor term
            let (sr, si) = coeff(&level, 2 % modulus.max(1), modulus);
            let same = (ratio * sr, ratio * si);
            // feed term: (1/3) sum_s w^(-2s) mu(2 + s 3^r, r+1), rotated by e(-1/3^(r+1))
            let mut fr = 0.0;
            let mut fi = 0.0;
            for st in 0..3u64 {
                let j = (2 + st * modulus) % fine;
                let (cr, ci) = coeff(&level, j, fine);
                let ang = -TAU * (2.0 * st as f64) / 3.0;
                let (wr, wi) = (f64_cos(ang), f64_sin(ang));
                fr += cr * wr - ci * wi;
                fi += cr * wi + ci * wr;
            }
            fr /= 3.0; fi /= 3.0;
            let rot = -TAU / (fine as f64);
            let (rr, ri) = (f64_cos(rot), f64_sin(rot));
            let feed = (ratio * (fr * rr - fi * ri), ratio * (fr * ri + fi * rr));
            let sum = (same.0 + feed.0, same.1 + feed.1);
            let (nr, ni) = coeff(&next, 1, modulus);
            let mag = |p: (f64, f64)| crate::constant_closure::f64_sqrt(p.0 * p.0 + p.1 * p.1);
            let tri = mag(same) + mag(feed);
            if tri > 1e-9 && d > 6 { canc_sum += mag(sum) / tri; canc_n += 1; }
            s.push_str(&format!("  {:>5}  {:>9}  {:>9.5}  {:>9.5}  {:>8.5}  {:>8.5}  {:>10.3}\n",
                d, next.len(), mag(same), mag(feed), mag(sum), mag((nr, ni)),
                if mag(same) > 1e-12 { mag(feed) / mag(same) } else { 0.0 }));
            level = next;
        }
        s.push_str(&format!("\n  mean |sum| / (|same| + |feed|): {:.4}   over {} level(s)\n",
            canc_sum / canc_n.max(1) as f64, canc_n));
        s.push_str("  one would be the triangle bound saturated, so a number well under it\n");
        s.push_str("  is the two terms cancelling in phase — which is where the decay is.\n");
        s.push_str("  |sum| tracking |next| is the identity itself holding.\n");
        s
    }


    /// Equidistribution as a collision count.
    ///
    /// Summing the squared coefficients over a conductor gives
    ///     sum_j |mu(j,r)|^2 = 3^r C(r) / N^2,
    /// where C(r) counts the pairs of level nodes sharing a residue mod 3^r. So
    /// equidistribution is exactly C(r) = N^2 / 3^r up to lower order, and the
    /// excess over that IS the nonprincipal mass. That turns the analytic
    /// question into a counting one, and the count splits by which arms the two
    /// nodes came down:
    ///
    ///   doubling-doubling: 2a = 2b mod 3^r iff a = b mod 3^r        -> C(r)
    ///   odd-odd:           u(a) = u(b) mod 3^r iff a = b mod 3^(r+1) -> finer C
    ///   mixed:             2a = u(b) mod 3^r                         -> the cross term
    ///
    /// The first two are forced by the bijections already proved. The cross term
    /// is the only free quantity, so this verb measures it.
    pub fn collisions(depth: u32, r: u32) -> String {
        let mut s = format!("collatz collisions at conductor 3^{} to depth {}\n", r, depth);
        s.push_str("  excess = 3^r C / N^2 - 1, the nonprincipal mass; cross is the mixed pairs\n\n");
        s.push_str("  level      nodes      excess   x N      dd share   oo share   cross share\n");
        let modulus = 3u64.pow(r);
        let mut level: Vec<u64> = alloc::vec![1];
        for d in 1..=depth {
            let mut evens: Vec<u64> = Vec::new();
            let mut odds: Vec<u64> = Vec::new();
            for &m in level.iter() {
                evens.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { odds.push(u); }
                }
            }
            let mut next: Vec<u64> = Vec::new();
            next.extend_from_slice(&evens);
            next.extend_from_slice(&odds);
            if next.is_empty() { break; }
            let n = next.len() as u64;
            // histogram of the next level, and of each arm separately
            let mut hist = alloc::vec![0u64; modulus as usize];
            let mut he = alloc::vec![0u64; modulus as usize];
            let mut ho = alloc::vec![0u64; modulus as usize];
            for &v in next.iter() { hist[(v % modulus) as usize] += 1; }
            for &v in evens.iter() { he[(v % modulus) as usize] += 1; }
            for &v in odds.iter() { ho[(v % modulus) as usize] += 1; }
            let mut c: u64 = 0;
            let mut cdd: u64 = 0;
            let mut coo: u64 = 0;
            let mut cx: u64 = 0;
            for i in 0..modulus as usize {
                c += hist[i] * hist[i];
                cdd += he[i] * he[i];
                coo += ho[i] * ho[i];
                cx += 2 * he[i] * ho[i];
            }
            let nf = n as f64;
            let excess = (modulus as f64) * (c as f64) / (nf * nf) - 1.0;
            s.push_str(&format!("  {:>5}  {:>9}  {:>10.5}  {:>7.3}  {:>9.4}  {:>9.4}  {:>12.4}\n",
                d, n, excess, excess * nf,
                cdd as f64 / c as f64, coo as f64 / c as f64, cx as f64 / c as f64));
            level = next;
        }
        s.push_str("\n  excess x N holding steady is the square-root law in counting form:\n");
        s.push_str("  the collisions exceed the uniform count by a bounded multiple of N.\n");
        s
    }


    /// The excess recursion, tested.
    ///
    /// Write e_d(r) for the collision excess, 3^r C / N^2 - 1. The three legs of
    /// the next level's count are forced: the doubling leg is exactly the
    /// previous C(r), the odd leg is the junction population's collisions over
    /// the 3^r junction classes mod 3^(r+1), and the cross leg is
    ///     C_cross = 2 sum_c n_r(c) m(phi(c)),   phi(a) = 3(a - 2^-1) + 2,
    /// with phi a bijection from classes mod 3^r onto junction classes mod
    /// 3^(r+1). By Cauchy-Schwarz the cross leg's deviation from its flat value
    /// is at most the PRODUCT of the two deviations, so it is second order and
    /// the recursion is linear to first order:
    ///
    ///     e_{d+1}(r)  <=  (9/16) e_d(r) + (1/16) e_d(r+1) + O(e^(3/2))
    ///
    /// The linear part sums to 10/16 when the two excesses are equal, and to
    /// 12/16 when the finer one is three times the coarser, which is what the
    /// modulus ratio suggests. Either is a contraction, and 3/4 per level in the
    /// excess is 0.866 per level in the coefficients — the square-root rate
    /// already measured. This verb checks the prediction against the walk.
    pub fn excess(depth: u32, r: u32) -> String {
        let mut s = format!("collatz excess at conductor 3^{} to depth {}\n", r, depth);
        s.push_str("  read in E = e * N, the scale a square-root law lives at:\n");
        s.push_str("     E_(d+1)(r) = (3/4) E_d(r) + (1/12) E_d(r+1)\n");
        s.push_str("  which is multiplier ONE when E(r+1) = 3 E(r) — the square-root law is a\n");
        s.push_str("  marginal fixed point of the level map, neither growing nor decaying.\n\n");
        s.push_str("  level      nodes       E(r)     E(r+1)   ratio    predicted     actual   pred/act   cross/N   CS bnd/N\n");
        let modulus = 3u64.pow(r);
        let fine = 3u64.pow(r + 1);
        let excess_of = |lvl: &Vec<u64>, m: u64| -> f64 {
            let n = lvl.len() as f64;
            let mut h = alloc::vec![0u64; m as usize];
            for &v in lvl.iter() { h[(v % m) as usize] += 1; }
            let mut c = 0u64;
            for i in 0..m as usize { c += h[i] * h[i]; }
            (m as f64) * (c as f64) / (n * n) - 1.0
        };
        let mut level: Vec<u64> = alloc::vec![1];
        let mut cross_tot = 0.0f64;
        let mut cross_neg = 0u64;
        let mut cross_n = 0u64;
        let mut cs_ok = 0u64;
        for d in 1..=depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                next.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            // the cross leg measured against its flat value, and against the
            // Cauchy-Schwarz bound on it: if the deviation sits inside the bound
            // and carries a sign, the cross term is not a negligible remainder.
            let mut evens: Vec<u64> = Vec::new();
            let mut odds: Vec<u64> = Vec::new();
            for &m in level.iter() {
                evens.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { odds.push(u); }
                }
            }
            let mut he = alloc::vec![0u64; modulus as usize];
            let mut ho = alloc::vec![0u64; modulus as usize];
            for &v in evens.iter() { he[(v % modulus) as usize] += 1; }
            for &v in odds.iter() { ho[(v % modulus) as usize] += 1; }
            let mut cx = 0.0f64;
            let mut de = 0.0f64;
            let mut dobar = 0.0f64;
            let ne = evens.len() as f64;
            let no = odds.len() as f64;
            for i in 0..modulus as usize {
                cx += 2.0 * he[i] as f64 * ho[i] as f64;
                let a = he[i] as f64 - ne / modulus as f64;
                let b = ho[i] as f64 - no / modulus as f64;
                de += a * a;
                dobar += b * b;
            }
            let flat = 2.0 * ne * no / modulus as f64;
            let cs = 2.0 * crate::constant_closure::f64_sqrt(de * dobar);
            let nprev = level.len() as f64;
            let nnext = next.len() as f64;
            let er = excess_of(&level, modulus) * nprev;
            let ef = excess_of(&level, fine) * nprev;
            let pred = 0.75 * er + (1.0 / 12.0) * ef;
            let act = excess_of(&next, modulus) * nnext;
            if d > 8 {
                let dev = (cx - flat) / nnext;
                cross_tot += dev;
                if dev < 0.0 { cross_neg += 1; }
                if (cx - flat).abs() <= cs { cs_ok += 1; }
                cross_n += 1;
                s.push_str(&format!("  {:>5}  {:>9}  {:>9.5}  {:>9.5}  {:>6.2}  {:>11.5}  {:>9.5}  {:>9.3}  {:>+10.4}  {:>9.4}\n",
                    d, next.len(), er, ef, if er.abs() > 1e-12 { ef / er } else { 0.0 },
                    pred, act, if act.abs() > 1e-12 { pred / act } else { 0.0 },
                    (cx - flat) / nnext, cs / nnext));
            }
            level = next;
        }
        s.push_str(&format!("\n  cross deviation: {} of {} level(s) negative, mean {:+.5} per unit N\n",
            cross_neg, cross_n, cross_tot / cross_n.max(1) as f64));
        s.push_str(&format!("  inside the Cauchy-Schwarz bound in {} of {} — the bound is structural,\n",
            cs_ok, cross_n));
        s.push_str("  the sign is not. A sign that holds is what pins E at its fixed point.\n");
        s
    }


    /// The involution and its perturbation, exactly.
    ///
    /// At conductor three the even children swap the two live classes, so their
    /// contribution to the imbalance n1 - n2 is exactly its negation. The odd
    /// children come only from the junctions and land by their mod-9 class:
    /// b = 2, 5, 8 (mod 9) sends u = 1, 0, 2 (mod 3). So the imbalance obeys
    ///
    ///     I_(d+1) = -I_d + (m2 - m8)
    ///
    /// with m_c the level's mod-9 counts and I the unnormalized imbalance. The
    /// involution is the minus sign; the perturbation is one difference of two
    /// mod-9 classes, and nothing else. This verb checks the identity and reports
    /// the size of the perturbation against the imbalance it perturbs.
    pub fn perturb(depth: u32) -> String {
        let mut s = String::from("collatz perturb — the involution at conductor 3 and its odd-arm term\n");
        s.push_str("  I(d+1) = -I(d) + (m2 - m8),  m over residues mod 9\n\n");
        s.push_str("  level      nodes       I(d)    m2 - m8   predicted     I(d+1)  ok   |pert|/|I|\n");
        let mut level: Vec<u64> = alloc::vec![1];
        let mut ratio_sum = 0.0f64;
        let mut ratio_n = 0u64;
        let mut bad = 0u64;
        for d in 1..=depth {
            let mut n = [0i64; 3];
            let mut m = [0i64; 9];
            for &v in level.iter() {
                n[(v % 3) as usize] += 1;
                m[(v % 9) as usize] += 1;
            }
            let imbalance = n[1] - n[2];
            let pert = m[2] - m[8];
            let pred = -imbalance + pert;
            let mut next: Vec<u64> = Vec::new();
            for &v in level.iter() {
                next.push(2 * v);
                if v % 3 == 2 {
                    let u = (2 * v - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            let mut nn = [0i64; 3];
            for &v in next.iter() { nn[(v % 3) as usize] += 1; }
            let actual = nn[1] - nn[2];
            let ok = pred == actual;
            if !ok { bad += 1; }
            if imbalance != 0 && d > 4 {
                ratio_sum += (pert as f64).abs() / (imbalance as f64).abs();
                ratio_n += 1;
            }
            if d > 6 || !ok {
                s.push_str(&format!("  {:>5}  {:>9}  {:>9}  {:>9}  {:>10}  {:>9}  {:>3}  {:>11.3}\n",
                    d, next.len(), imbalance, pert, pred, actual,
                    if ok { "yes" } else { "NO" },
                    if imbalance != 0 { (pert as f64).abs() / (imbalance as f64).abs() } else { 0.0 }));
            }
            level = next;
        }
        s.push_str(&format!("\n  identity failed on {} level(s)\n", bad));
        s.push_str(&format!("  mean |perturbation| / |imbalance|: {:.4}\n",
            ratio_sum / ratio_n.max(1) as f64));
        s.push_str("  under one, the involution dominates and the sign alternates; over one,\n");
        s.push_str("  the odd arm carries the level and the alternation breaks.\n");
        s
    }


    /// The weighted norm the tower closes in.
    ///
    /// The excess tower is finite: a level of N nodes has empty classes once
    /// 3^r > N, so it has about 0.262 d rungs and not infinitely many. In the
    /// weighted norm
    ///     ||e|| = sup over r of 3^(-r) e(r)
    /// the level map obeys
    ///     3^(-r) e_(d+1)(r) <= (9/16) 3^(-r) e_d(r) + (3/16) 3^(-(r+1)) e_d(r+1)
    ///                       <= (12/16) ||e_d||,
    /// because the finer term carries coefficient 1/16 and the weight pays 3 for
    /// the digit. That is a contraction at 3/4 per level, and (3/4)^d is exactly
    /// 1/N_d — the square-root law, derived rather than fitted.
    ///
    /// This verb measures the norm, its per-level ratio against 3/4, and which
    /// conductor attains it.
    pub fn norm(depth: u32, fixed_rungs: u32) -> String {
        let mut s = format!("collatz norm — the weighted excess norm and its contraction\n  rungs {}\n",
            if fixed_rungs > 0 { alloc::format!("held at {}", fixed_rungs) } else { alloc::string::String::from("as the tower allows") });
        s.push_str("  two folds over the same rungs: the max, which keeps the larger and\n");
        s.push_str("  discards the rest, and the sum, which keeps both. The ob3ect's banked\n");
        s.push_str("  check reads one unit lost to the max fold, so the sum is the honest one.\n\n");
        s.push_str("  In the sum fold the recursion adds exactly:\n");
        s.push_str("     ||e|| <= (9/16)||e|| + (3/16)||e|| + C  =  (3/4)||e|| + C,\n");
        s.push_str("  so the only free quantity is c = C / ||e||, and the contraction holds\n");
        s.push_str("  when c stays under a quarter. That column is the one that decides.\n\n");
        s.push_str("  level      nodes   rungs      sum-fold    ratio    (3/4)prev           c   c < 1/4\n");
        let excess_of = |lvl: &Vec<u64>, m: u64| -> f64 {
            let n = lvl.len() as f64;
            let mut h = alloc::vec![0u64; m as usize];
            for &v in lvl.iter() { h[(v % m) as usize] += 1; }
            let mut c = 0u64;
            for i in 0..m as usize { c += h[i] * h[i]; }
            (m as f64) * (c as f64) / (n * n) - 1.0
        };
        let mut level: Vec<u64> = alloc::vec![1];
        let mut prev = 0.0f64;
        let mut prev_sum = 0.0f64;
        let mut c_sum = 0.0f64;
        let mut c_n = 0u64;
        let mut c_ok = 0u64;
        let mut c_max = -9.0f64;
        let mut prev2_sum = 0.0f64;
        let mut pair_sum = 0.0f64;
        let mut pair_n = 0u64;
        let mut pair_ok = 0u64;
        let mut pair_max = 0.0f64;
        for d in 1..=depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                next.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            level = next;
            let n = level.len() as u64;
            let mut best = 0.0f64;
            let mut total = 0.0f64;
            let mut rungs = 0u32;
            let mut r = 1u32;
            // A rung opening is the NORM's truncation moving, not the dynamics:
            // the sum gains a term the level below never had, so the ratio jumps
            // once and settles. Holding the rung count fixed takes that artefact
            // out and leaves what the level map actually does.
            let rung_cap = if fixed_rungs > 0 { fixed_rungs } else { 20 };
            while 3u64.pow(r) <= n && r <= rung_cap {
                let w = excess_of(&level, 3u64.pow(r)) / (3.0f64).powi(r as i32);
                if w > best { best = w; }
                total += w;
                rungs = r;
                r += 1;
                if r > 20 { break; }
            }
            if d > 8 && prev_sum > 0.0 {
                let three_quarter = 0.75 * prev_sum;
                let c = (total - three_quarter) / prev_sum;
                c_sum += c;
                c_n += 1;
                if c < 0.25 { c_ok += 1; }
                if c > c_max { c_max = c; }
                s.push_str(&format!("  {:>5}  {:>9}  {:>6}  {:>11.7}  {:>7.3}  {:>11.7}  {:>+10.4}  {:>8}\n",
                    d, n, rungs, total, total / prev_sum, three_quarter, c,
                    if c < 0.25 { "yes" } else { "NO" }));
            }
            let _ = best;
            if d > 8 && prev2_sum > 0.0 {
                let pr = total / prev2_sum;
                pair_sum += pr;
                pair_n += 1;
                if pr < 0.5625 { pair_ok += 1; }
                if pr > pair_max { pair_max = pr; }
            }
            prev2_sum = prev_sum;
            prev = best;
            prev_sum = total;
        }
        s.push_str(&format!("\n  c under a quarter in {} of {} level(s); mean c {:+.4}, worst {:+.4}\n",
            c_ok, c_n, c_sum / c_n.max(1) as f64, c_max));
        // The involution has period two, so the two-step map is where an
        // excursion and its partner meet. A pair ratio under (3/4)^2 = 0.5625
        // is the contraction holding across the swap even where one level alone
        // expands.
        s.push_str(&format!("  two-step ratio: under 0.5625 in {} of {}; mean {:.4}, worst {:.4}\n",
            pair_ok, pair_n, pair_sum / pair_n.max(1) as f64, pair_max));
        s.push_str("  0.5625 is (3/4) squared, what two clean levels would give.\n");
        s.push_str("  3/4 + c is the contraction constant, and the fold discards nothing.\n");
        s
    }


    /// Is the weighted cross sum carried by ONE rung?
    ///
    /// The ob3ect grounds the fuse at 𐑜, disjunction: one sufficient residue
    /// satisfies the bound rather than all of them simultaneously. If that reads
    /// true here, the weighted cross sum C = sum_r 3^-r cross(r) is dominated by
    /// a single rung, and bounding C reduces to bounding that one term rather
    /// than every term at once. This measures the share the largest rung takes,
    /// and which rung it is.
    pub fn disjunct(depth: u32) -> String {
        let mut s = String::from("collatz disjunct — the share of the weighted cross sum in its largest rung\n\n");
        s.push_str("  level      nodes   rungs      |C|     top rung    share   at r\n");
        let mut level: Vec<u64> = alloc::vec![1];
        let mut share_sum = 0.0f64;
        let mut share_n = 0u64;
        for d in 1..=depth {
            let mut evens: Vec<u64> = Vec::new();
            let mut odds: Vec<u64> = Vec::new();
            for &m in level.iter() {
                evens.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { odds.push(u); }
                }
            }
            let mut next: Vec<u64> = Vec::new();
            next.extend_from_slice(&evens);
            next.extend_from_slice(&odds);
            if next.is_empty() { break; }
            let n = next.len() as u64;
            let ne = evens.len() as f64;
            let no = odds.len() as f64;
            let mut total = 0.0f64;
            let mut top = 0.0f64;
            let mut arg = 0u32;
            let mut rungs = 0u32;
            let mut r = 1u32;
            while 3u64.pow(r) <= n && r <= 12 {
                let m = 3u64.pow(r);
                let mut he = alloc::vec![0u64; m as usize];
                let mut ho = alloc::vec![0u64; m as usize];
                for &v in evens.iter() { he[(v % m) as usize] += 1; }
                for &v in odds.iter() { ho[(v % m) as usize] += 1; }
                let mut cx = 0.0f64;
                for i in 0..m as usize { cx += 2.0 * he[i] as f64 * ho[i] as f64; }
                let flat = 2.0 * ne * no / m as f64;
                let term = (cx - flat) * (m as f64) / ((n * n) as f64) / (3.0f64).powi(r as i32);
                total += term;
                if term.abs() > top.abs() { top = term; arg = r; }
                rungs = r;
                r += 1;
            }
            if d > 10 && total.abs() > 1e-12 {
                let sh = top.abs() / total.abs();
                share_sum += sh;
                share_n += 1;
                s.push_str(&format!("  {:>5}  {:>9}  {:>6}  {:>+9.6}  {:>+10.6}  {:>7.3}  {:>5}\n",
                    d, n, rungs, total, top, sh, arg));
            }
            level = next;
        }
        s.push_str(&format!("\n  mean share of the largest rung: {:.4} over {} level(s)\n",
            share_sum / share_n.max(1) as f64, share_n));
        s.push_str("  a share near one is the disjunctive reading: one rung carries the sum,\n");
        s.push_str("  so the bound needs that rung and not every rung at once.\n");
        s
    }


    /// Hunt the cheapest counterexample to the contraction, oracle-style.
    ///
    /// The claim under attack: past a stated level size, the weighted cross term
    /// c = C/||e|| stays under a quarter, so 3/4 + c is a contraction. This walks
    /// every level to the given depth with the rung count held, computes c, and
    /// stops at the first level past the threshold that breaks it. Surviving the
    /// hunt is not a proof; what the verb reports is what was exhausted.
    pub fn attack(depth: u32, fixed_rungs: u32, min_nodes: u64) -> String {
        let mut s = format!("collatz attack — claim: c < 1/4 for every level with N >= {}\n", min_nodes);
        s.push_str(&format!("  rungs held at {} and required live at both levels, to depth {}\n\n",
            fixed_rungs, depth));
        let excess_of = |lvl: &Vec<u64>, m: u64| -> f64 {
            let n = lvl.len() as f64;
            let mut h = alloc::vec![0u64; m as usize];
            for &v in lvl.iter() { h[(v % m) as usize] += 1; }
            let mut c = 0u64;
            for i in 0..m as usize { c += h[i] * h[i]; }
            (m as f64) * (c as f64) / (n * n) - 1.0
        };
        let norm_of = |lvl: &Vec<u64>| -> f64 {
            let n = lvl.len() as u64;
            let mut t = 0.0f64;
            let mut r = 1u32;
            while 3u64.pow(r) <= n && r <= fixed_rungs {
                t += excess_of(lvl, 3u64.pow(r)) / (3.0f64).powi(r as i32);
                r += 1;
            }
            t
        };
        let mut level: Vec<u64> = alloc::vec![1];
        let mut prev_norm = 0.0f64;
        let mut prev_n = 0u64;
        let mut tested = 0u64;
        let mut worst = -9.0f64;
        let mut worst_d = 0u32;
        let mut broke: Option<(u32, u64, f64)> = None;
        for d in 1..=depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                next.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            level = next;
            let n = level.len() as u64;
            let cur = norm_of(&level);
            // A rung that opens between the two levels adds a term the level
            // below never had, so the pair is not comparable. Capping does not
            // prevent that — it only delays it — so the test requires the full
            // rung count live at BOTH levels.
            let both_rungs_live = prev_n >= 3u64.pow(fixed_rungs) && n >= 3u64.pow(fixed_rungs);
            if prev_norm > 0.0 && n >= min_nodes && both_rungs_live {
                let c = (cur - 0.75 * prev_norm) / prev_norm;
                tested += 1;
                if c > worst { worst = c; worst_d = d; }
                if c >= 0.25 && broke.is_none() {
                    broke = Some((d, n, c));
                }
            }
            prev_norm = cur;
            prev_n = n;
        }
        match broke {
            Some((d, n, c)) => {
                s.push_str(&format!("  BROKEN at level {} (N = {}): c = {:+.4}\n", d, n, c));
                s.push_str("  the cheapest counterexample past the threshold; the claim is false\n");
                s.push_str("  as stated and the threshold or the constant has to move.\n");
            }
            None => {
                s.push_str(&format!("  survived: {} level(s) tested, none reached c = 1/4\n", tested));
                s.push_str(&format!("  worst c = {:+.4} at level {}\n", worst, worst_d));
                s.push_str("  surviving is not proof. This is what was exhausted, and the claim\n");
                s.push_str("  stands unrefuted over exactly that range.\n");
            }
        }
        s
    }


    /// The one number the Cauchy-Schwarz route turns on.
    ///
    /// Writing each histogram as flat plus deviation, the flat parts cancel
    /// because both deviations sum to zero, so
    ///     cross(r) = 2 sum_c a(c) b(phi(c)) 3^r / N'^2
    /// and Cauchy-Schwarz with the arm proportions 3/4 and 1/4 gives
    ///     |cross(r)| <= (3/8) sqrt( e_even(r) e_odd(r) )
    /// where e_even(r) is the level's own excess exactly, by the doubling
    /// bijection. Summing with the weight,
    ///     c <= (3/8) sqrt(3) sqrt( ||e_J|| / ||e|| )
    /// with e_J the junction subpopulation's excess. So the route closes exactly
    /// when that ratio sits under 0.1482, and this verb measures it.
    pub fn jratio(depth: u32, rungs: u32) -> String {
        let mut s = format!("collatz jratio — the junction excess against the level's, {} rungs\n", rungs);
        s.push_str("  c <= 0.6495 sqrt(ratio); the route closes when ratio < 0.1482\n\n");
        s.push_str("  level      nodes    arm image      ||e||      ||e_J||    ratio    c bound\n");
        let excess_of = |lvl: &Vec<u64>, m: u64| -> f64 {
            let n = lvl.len() as f64;
            if n < 1.0 { return 0.0; }
            let mut h = alloc::vec![0u64; m as usize];
            for &v in lvl.iter() { h[(v % m) as usize] += 1; }
            let mut c = 0u64;
            for i in 0..m as usize { c += h[i] * h[i]; }
            (m as f64) * (c as f64) / (n * n) - 1.0
        };
        let norm_of = |lvl: &Vec<u64>| -> f64 {
            let n = lvl.len() as u64;
            let mut t = 0.0f64;
            let mut r = 1u32;
            while 3u64.pow(r) <= n && r <= rungs {
                t += excess_of(lvl, 3u64.pow(r)) / (3.0f64).powi(r as i32);
                r += 1;
            }
            t
        };
        let mut level: Vec<u64> = alloc::vec![1];
        let mut worst = 0.0f64;
        let mut n_ok = 0u64;
        let mut n_tot = 0u64;
        for d in 1..=depth {
            let mut next: Vec<u64> = Vec::new();
            for &m in level.iter() {
                next.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            level = next;
            let n = level.len() as u64;
            if n < 3u64.pow(rungs) { continue; }
            // b in the derivation is the odd ARM IMAGE, the u = 2t+1 values,
            // which the arm bijection spreads over every residue. The junction
            // parents are all 2 (mod 3) by definition and so are maximally
            // concentrated at the first rung — measuring those measures the
            // definition, not the mixing.
            let junctions: Vec<u64> = level.iter().copied()
                .filter(|v| v % 3 == 2)
                .map(|v| 2 * (v / 3) + 1)
                .collect();
            if junctions.len() < 9 { continue; }
            let e = norm_of(&level);
            let ej = norm_of(&junctions);
            if e <= 0.0 { continue; }
            let ratio = ej / e;
            let bound = 0.6495 * crate::constant_closure::f64_sqrt(ratio.max(0.0));
            n_tot += 1;
            if bound < 0.25 { n_ok += 1; }
            if bound > worst { worst = bound; }
            if d > 20 {
                s.push_str(&format!("  {:>5}  {:>9}  {:>10}  {:>9.6}  {:>11.6}  {:>7.3}  {:>9.3}\n",
                    d, n, junctions.len(), e, ej, ratio, bound));
            }
        }
        s.push_str(&format!("\n  the bound lands under a quarter in {} of {} level(s); worst {:.3}\n",
            n_ok, n_tot, worst));
        s
    }


    /// The cross term as an autocorrelation at a lag.
    ///
    /// Both deviations are the SAME level's, read under two affine maps, so the
    /// cross term is the level's autocorrelation at a nonzero lag. Since the
    /// deviation sums to zero, the autocorrelations over ALL lags sum to zero:
    ///     R(0) = ||d||^2,   sum over nonzero lags of R = -||d||^2,
    /// so a typical nonzero lag sits at -||d||^2 / (3^r - 1). That is negative,
    /// and smaller than the Cauchy-Schwarz bound ||d||^2 by exactly the factor
    /// 3^r - 1. This verb reads the actual lag value against both.
    pub fn lag(depth: u32, r: u32) -> String {
        let mut s = format!("collatz lag — the cross term against the lag average, conductor 3^{}\n", r);
        s.push_str("  CS allows ||d||^2; the lag average is -||d||^2/(3^r - 1)\n\n");
        s.push_str("  level      nodes       actual    lag avg     CS bnd   act/avg   act/CS\n");
        let modulus = 3u64.pow(r);
        let mut level: Vec<u64> = alloc::vec![1];
        let mut ratio_sum = 0.0f64;
        let mut ratio_n = 0u64;
        for d in 1..=depth {
            let mut evens: Vec<u64> = Vec::new();
            let mut odds: Vec<u64> = Vec::new();
            for &m in level.iter() {
                evens.push(2 * m);
                if m % 3 == 2 {
                    let u = (2 * m - 1) / 3;
                    if u != 1 { odds.push(u); }
                }
            }
            let mut next: Vec<u64> = Vec::new();
            next.extend_from_slice(&evens);
            next.extend_from_slice(&odds);
            if next.is_empty() { break; }
            let ne = evens.len() as f64;
            let no = odds.len() as f64;
            if no < 9.0 { level = next; continue; }
            let mut he = alloc::vec![0f64; modulus as usize];
            let mut ho = alloc::vec![0f64; modulus as usize];
            for &v in evens.iter() { he[(v % modulus) as usize] += 1.0; }
            for &v in odds.iter() { ho[(v % modulus) as usize] += 1.0; }
            let mut cross = 0.0f64;
            let mut norm_e = 0.0f64;
            let mut norm_o = 0.0f64;
            for i in 0..modulus as usize {
                let a = he[i] - ne / modulus as f64;
                let b = ho[i] - no / modulus as f64;
                cross += a * b;
                norm_e += a * a;
                norm_o += b * b;
            }
            let cs = crate::constant_closure::f64_sqrt(norm_e * norm_o);
            let lag_avg = -cs / (modulus as f64 - 1.0);
            if d > 14 && cs > 1e-9 {
                let r1 = cross / lag_avg;
                let r2 = cross / cs;
                ratio_sum += r2.abs();
                ratio_n += 1;
                s.push_str(&format!("  {:>5}  {:>9}  {:>+11.3}  {:>+9.3}  {:>9.3}  {:>8.3}  {:>+7.3}\n",
                    d, next.len(), cross, lag_avg, cs, r1, r2));
            }
            level = next;
        }
        s.push_str(&format!("\n  mean |actual| / CS bound: {:.4} over {} level(s)\n",
            ratio_sum / ratio_n.max(1) as f64, ratio_n));
        s.push_str(&format!("  1/(3^r - 1) = {:.4} — what a typical lag would give\n",
            1.0 / (modulus as f64 - 1.0)));
        s
    }


    /// The conductor-nine identity, checked in exact integers.
    ///
    /// Each class c mod 9 of the next level is fed by exactly one class mod 9 of
    /// this one through the doubling arm — the class 5c, since 2*5 = 1 mod 9 —
    /// and by exactly one class mod 27 through the odd arm, namely
    /// 3*((5(c-1)) mod 9) + 2. So
    ///     n'(c) = n(5c mod 9) + m27(oddSource c)
    /// with n the level's mod-9 counts and m27 its mod-27 counts. No sign here:
    /// at conductor nine the doubling permutation is a six-cycle, not an
    /// involution, so the recursion is exact without being alternating.
    pub fn perturb9(depth: u32) -> String {
        let mut s = String::from("collatz perturb9 — the conductor-9 identity, exact integers\n");
        s.push_str("  n'(c) = n(5c mod 9) + m27(3*((5(c-1)) mod 9) + 2)\n\n");
        s.push_str("  level      nodes   classes checked   mismatches\n");
        let mut level: Vec<u64> = alloc::vec![1];
        let mut bad_total = 0u64;
        for d in 1..=depth {
            let mut n9 = [0i64; 9];
            let mut m27 = [0i64; 27];
            for &v in level.iter() {
                n9[(v % 9) as usize] += 1;
                m27[(v % 27) as usize] += 1;
            }
            let mut next: Vec<u64> = Vec::new();
            for &v in level.iter() {
                next.push(2 * v);
                if v % 3 == 2 {
                    let u = (2 * v - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            let mut nn9 = [0i64; 9];
            for &v in next.iter() { nn9[(v % 9) as usize] += 1; }
            let mut bad = 0u64;
            for c in 0..9usize {
                let doubling_src = (5 * c) % 9;
                let t_class = (5 * (c + 8)) % 9;
                let odd_src = (3 * t_class + 2) % 27;
                let pred = n9[doubling_src] + m27[odd_src];
                if pred != nn9[c] { bad += 1; }
            }
            bad_total += bad;
            if d > depth.saturating_sub(10) {
                s.push_str(&format!("  {:>5}  {:>9}  {:>16}  {:>11}\n", d, next.len(), 9, bad));
            }
            level = next;
        }
        s.push_str(&format!("\n  mismatches over the whole walk: {}\n", bad_total));
        s.push_str("  a zero here is the identity holding class by class, level by level.\n");
        s
    }


    /// The proportionality at conductor three.
    ///
    /// The even arm's deviation is the level's own with classes 1 and 2
    /// transposed, a = (n0, n2, n1) - N/3, and the odd arm's is the junctions'
    /// mod-9 split b = (m5, m2, m8) - No/3. Both are zero-sum three-vectors, so
    /// they live in a plane, and the measurement says they are nearly parallel:
    /// |cos| runs to 0.99 and above. Then b is close to lambda times a and the
    /// whole sign question is the sign of lambda.
    ///
    /// This reports lambda = (a.b)/||a||^2 per level, with the cosine beside it
    /// so a near-parallel reading can be told from a chance one.
    pub fn lambda(depth: u32) -> String {
        let mut s = String::from("collatz lambda — b = lambda a at conductor three\n");
        s.push_str("  a = (n0, n2, n1) - N/3 (even arm), b = (m5, m2, m8) - No/3 (odd arm)\n\n");
        s.push_str("  level      nodes      lambda   sign    disjoint   part-whole\n");
        let mut level: Vec<u64> = alloc::vec![1];
        let mut neg = 0u64;
        let mut tot = 0u64;
        let mut lsum = 0.0f64;
        let mut dis_sum = 0.0f64;
        let mut pw_sum = 0.0f64;
        let mut dis_neg = 0u64;
        let mut pw_pos = 0u64;
        for d in 1..=depth {
            let mut n3 = [0f64; 3];
            let mut m9 = [0f64; 9];
            for &v in level.iter() {
                n3[(v % 3) as usize] += 1.0;
                m9[(v % 9) as usize] += 1.0;
            }
            let nn = level.len() as f64;
            let no = n3[2];
            let a = [n3[0] - nn / 3.0, n3[2] - nn / 3.0, n3[1] - nn / 3.0];
            let b = [m9[5] - no / 3.0, m9[2] - no / 3.0, m9[8] - no / 3.0];
            let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
            let na = crate::constant_closure::f64_sqrt(a[0]*a[0] + a[1]*a[1] + a[2]*a[2]);
            let nb = crate::constant_closure::f64_sqrt(b[0]*b[0] + b[1]*b[1] + b[2]*b[2]);
            let mut next: Vec<u64> = Vec::new();
            for &v in level.iter() {
                next.push(2 * v);
                if v % 3 == 2 {
                    let u = (2 * v - 1) / 3;
                    if u != 1 { next.push(u); }
                }
            }
            if next.is_empty() { break; }
            if d > 12 && na > 1e-9 {
                let lam = dot / (na * na);
                let cos = if nb > 1e-9 { dot / (na * nb) } else { 0.0 };
                tot += 1;
                lsum += lam;
                if lam < 0.0 { neg += 1; }
                // the six integers the sign is a function of, so an exceptional
                // level can be read rather than just counted
                let x = n3[0] - nn / 3.0;
                let y = n3[1] - nn / 3.0;
                let z = n3[2] - nn / 3.0;
                let pp = m9[2] - no / 3.0;
                let qq = m9[5] - no / 3.0;
                let rr = m9[8] - no / 3.0;
                // The three terms are not alike. n0 and n1 are DISJOINT from the
                // junction parts m5, m8, so at fixed total those pairs
                // anti-correlate; n2 is the WHOLE of which m2 is a part, so that
                // pair correlates positively. Two against one is the sign.
                let t_disjoint = x * qq + y * rr;
                let t_partwhole = z * pp;
                dis_sum += t_disjoint;
                pw_sum += t_partwhole;
                if t_disjoint < 0.0 { dis_neg += 1; }
                if t_partwhole > 0.0 { pw_pos += 1; }
                s.push_str(&format!("  {:>5}  {:>9}  {:>+10.4}  {:>5}  {:>+10.1}  {:>+11.1}\n",
                    d, level.len(), lam,
                    if lam < 0.0 { "neg" } else { "POS" }, t_disjoint, t_partwhole));
            }
            level = next;
        }
        s.push_str(&format!("\n  lambda negative in {} of {} level(s); mean {:+.4}\n",
            neg, tot, lsum / tot.max(1) as f64));
        s.push_str(&format!("  disjoint terms negative in {} of {}, mean {:+.1}\n", dis_neg, tot, dis_sum / tot.max(1) as f64));
        s.push_str(&format!("  part-whole term positive in {} of {}, mean {:+.1}\n", pw_pos, tot, pw_sum / tot.max(1) as f64));
        s.push_str("  two disjoint pairs pulling one way against one part-whole pair pulling\n");
        s.push_str("  the other is the sign, and it is an expectation, not a rule.\n");
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