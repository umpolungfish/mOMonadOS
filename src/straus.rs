//! The Erdős–Straus ladder, as an instrument.
//!
//! `nesting greedy` puts 4/n in the attracted bin with a finite budget: greedy
//! unit-fraction removal always arrives, so Erdős–Straus is a claim about the
//! budget being three. `imasm check` on the two-fork form answers B — a δ fork
//! dangles — and names the remedy: COMMIT one arm rather than search it.
//!
//! Committing it gives a ladder. For `n = 4k+1` and any `r ≡ 3 (mod 4)`, the
//! term `a = (n+r)/4` is an integer and
//!
//!     4/n − 1/a = r/(n·a)
//!
//! so the remainder's numerator is CHOSEN. The second term follows from a
//! divisor: if `n·a = d·e` with `d ≡ −1 (mod r)` and `d = r·t + (r−1)`, then
//!
//!     r/(n·a) = 1/(e(t+1)) + 1/(n·a·(t+1))
//!
//! Both identities are proved in p4ramill (Erdos/StrausGreedyFamily.lean); this
//! module is the reading, not the proof — it reports WHICH rung closes a given
//! `n`, which is the quantity the open question is now about.
//!
//! What the instrument cannot do is settle the conjecture: it finds the rung
//! that closes each `n` it is given, and no finite sweep says every `n` has one.
//! The value it adds is the spectrum — how high the ladder has to reach, and
//! whether that height is bounded on the range looked at.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// The rung that closes `n`, if one is found within `max_rung`.
pub struct Rung {
    pub r: u64,
    pub a: u64,
    pub d: u64,
    pub e: u64,
    pub b: u64,
    pub c: u64,
}

/// Does `d ≡ −1 (mod r)`? That is the divisor condition the second term needs.
fn divisor_closes(d: u64, r: u64) -> bool {
    r >= 2 && d % r == r - 1
}

/// Find the lowest rung `r ≡ 3 (mod 4)` that closes `4/n`, scanning divisors of
/// `n·a` for one congruent to `−1 (mod r)`.
///
/// `r = 3` is the greedy step. Higher rungs are the committed arms.
pub fn lowest_rung(n: u64, max_rung: u64) -> Option<Rung> {
    if n < 2 || n % 4 != 1 {
        return None;
    }
    let mut r = 3u64;
    while r <= max_rung {
        // a = (n + r)/4 is an integer exactly when r ≡ 3 (mod 4), which the step
        // of 4 below maintains.
        let a = (n + r) / 4;
        let m = n.saturating_mul(a);
        if m == 0 {
            r += 4;
            continue;
        }
        // Scan divisors of m for one ≡ −1 (mod r). The smallest such divisor
        // gives the smallest second denominator, so the scan runs upward.
        let mut d = 1u64;
        while d.saturating_mul(d) <= m {
            if m % d == 0 {
                for cand in [d, m / d] {
                    if divisor_closes(cand, r) {
                        let t = (cand - (r - 1)) / r;
                        let e = m / cand;
                        return Some(Rung {
                            r,
                            a,
                            d: cand,
                            e,
                            b: e.saturating_mul(t + 1),
                            c: m.saturating_mul(t + 1),
                        });
                    }
                }
            }
            d += 1;
        }
        r += 4;
    }
    None
}

pub struct Straus;

impl Straus {
    pub fn help() -> String {
        let mut s = String::new();
        s.push_str("straus — the Erdős–Straus ladder\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("  straus <n>            the lowest rung that closes 4/n\n");
        s.push_str("  straus sweep <lo> <hi>  the rung spectrum across a range\n");
        s.push_str("\n");
        s.push_str("A rung is r ≡ 3 (mod 4): the first term is 1/((n+r)/4) and the\n");
        s.push_str("remainder has numerator exactly r. r = 3 is the greedy step.\n");
        s.push_str("The second term comes from a divisor d ≡ −1 (mod r) of n·a —\n");
        s.push_str("committed, not searched, which is what imasm check asked for.\n");
        s
    }

    /// One `n`, read against the ladder.
    pub fn read(n: u64) -> String {
        let mut s = format!("4/{} against the ladder\n", n);
        if n % 4 != 1 {
            s.push_str("  not in the surviving class — n ≢ 1 (mod 4), and every other\n");
            s.push_str("  class is already settled by a parametric family.\n");
            return s;
        }
        match lowest_rung(n, 400) {
            None => {
                s.push_str("  no rung ≤ 400 closes it — the ladder does not reach here,\n");
                s.push_str("  which is a budget statement, not an impossibility.\n");
            }
            Some(g) => {
                s.push_str(&format!("  rung r = {}{}\n", g.r,
                    if g.r == 3 { "   (the greedy step)" } else { "   (a committed arm)" }));
                s.push_str(&format!("  first term    1/{}\n", g.a));
                s.push_str(&format!("  divisor       d = {} ≡ −1 (mod {}),  e = {}\n", g.d, g.r, g.e));
                s.push_str(&format!("  4/{} = 1/{} + 1/{} + 1/{}\n", n, g.a, g.b, g.c));
                // The identity is checked here, not asserted: cross-multiplied
                // over u128 so nothing is taken on trust from the construction.
                let (a, b, c) = (g.a as u128, g.b as u128, g.c as u128);
                let n128 = n as u128;
                let lhs = 4u128 * a * b * c;
                let rhs = n128 * (b * c + a * c + a * b);
                s.push_str(&format!("  checked       {}\n",
                    if lhs == rhs { "4·abc = n·(bc+ac+ab) — holds" } else { "FAILS" }));
            }
        }
        s
    }

    /// The spectrum across a range: which rung each `n` needed.
    pub fn sweep(lo: u64, hi: u64) -> String {
        let mut s = format!("rung spectrum, n ≡ 1 (mod 4), {} ≤ n ≤ {}\n", lo, hi);
        s.push_str("═══════════════════════════════════════════════════════════\n");
        let mut counts: Vec<(u64, u64)> = Vec::new();
        let mut unreached: Vec<u64> = Vec::new();
        let mut worst = 0u64;
        let mut worst_n = 0u64;
        let mut total = 0u64;
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && n % 3 != 0 {
                total += 1;
                match lowest_rung(n, 400) {
                    None => unreached.push(n),
                    Some(g) => {
                        if g.r > worst { worst = g.r; worst_n = n; }
                        match counts.iter_mut().find(|(r, _)| *r == g.r) {
                            Some((_, c)) => *c += 1,
                            None => counts.push((g.r, 1)),
                        }
                    }
                }
            }
            n += 4;
        }
        counts.sort_by_key(|(r, _)| *r);
        s.push_str(&format!("  {} values in the class (3 ∤ n)\n\n", total));
        s.push_str("  rung |  closed here | share\n");
        s.push_str("  -----|--------------|-------\n");
        for (r, c) in &counts {
            let pct = if total > 0 { (*c as f64) * 100.0 / (total as f64) } else { 0.0 };
            s.push_str(&format!("  {:>4} | {:>12} | {:.1}%\n", r, c, pct));
        }
        s.push_str(&format!("\n  highest rung needed: {} (at n = {})\n", worst, worst_n));
        s.push_str(&format!("  unreached by r ≤ 400: {}\n", unreached.len()));
        if !unreached.is_empty() {
            // Naming them is the point of the sweep. A count says a residue
            // exists; the list says WHICH n, and that is what the next rung —
            // or the next mechanism — has to answer for.
            s.push_str("  the values the ladder did not reach:\n   ");
            for (i, v) in unreached.iter().enumerate() {
                if i > 0 && i % 10 == 0 { s.push_str("\n   "); }
                s.push_str(&format!(" {}", v));
            }
            s.push('\n');
        }
        s.push_str("\n  A bounded spectrum across a range is evidence about the range and\n");
        s.push_str("  nothing more: the open question is whether the height is bounded\n");
        s.push_str("  at all, and no sweep answers that.\n");
        s
    }
}
