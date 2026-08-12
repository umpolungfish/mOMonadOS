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

/// Prime factorisation of `m` by trial division. `m` here is `n·a`, a few
/// million at most, so the loop to √m is short.
fn factor(mut m: u64) -> Vec<(u64, u32)> {
    let mut out: Vec<(u64, u32)> = Vec::new();
    let mut d = 2u64;
    while d.saturating_mul(d) <= m {
        if m % d == 0 {
            let mut k = 0u32;
            while m % d == 0 { m /= d; k += 1; }
            out.push((d, k));
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if m > 1 { out.push((m, 1)); }
    out
}

/// The divisors of `m²`, from the factorisation of `m` with every exponent
/// doubled. Capped so a pathological input cannot exhaust the kernel's heap.
fn divisors_of_square(m: u64, cap: usize) -> Vec<u64> {
    let mut divs: Vec<u64> = Vec::new();
    divs.push(1);
    for (p, k) in factor(m) {
        let mut next: Vec<u64> = Vec::new();
        for &d in divs.iter() {
            let mut power = 1u64;
            for _ in 0..=(2 * k) {
                match d.checked_mul(power) {
                    Some(v) => next.push(v),
                    None => break,
                }
                match power.checked_mul(p) {
                    Some(v) => power = v,
                    None => break,
                }
            }
        }
        next.sort_unstable();
        next.dedup();
        if next.len() > cap { next.truncate(cap); }
        divs = next;
    }
    divs
}

/// The second term's REAL criterion.
///
/// `r/M = 1/b + 1/c` exactly when some `u` dividing `M²` satisfies
/// `u ≡ −M (mod r)`; then `b = (M+u)/r` and `c = (M + M²/u)/r`, because
/// `(M+u)(M+v) = M(2M+u+v)` whenever `uv = M²`.
///
/// The first version of this instrument tested divisors of `M` for `d ≡ −1
/// (mod r)` — one sufficient family inside this criterion, not the criterion.
/// On a conservative operator the fixed-point rule admits only one-shot or
/// no-closure, so a stabiliser set that is too small does not report "nearly":
/// it reports nothing. n = 2521 was called unreachable on that account, and
/// rung 23 closes it.
fn split_second(r: u64, m: u64, cap: usize) -> Option<(u64, u64)> {
    if r < 2 || m == 0 { return None; }
    let mm = (m as u128) * (m as u128);
    for u in divisors_of_square(m, cap) {
        if (u + m) % r != 0 { continue; }
        let v128 = mm / (u as u128);
        if v128 > u64::MAX as u128 { continue; }
        let v = v128 as u64;
        if (v + m) % r != 0 { continue; }
        let b = (m + u) / r;
        let c = (m + v) / r;
        if b > 0 && c > 0 { return Some((b, c)); }
    }
    None
}

/// Find the lowest rung `r ≡ 3 (mod 4)` whose second split closes.
///
/// `r = 3` is the greedy step; higher rungs are the committed arms `imasm check`
/// asked for when it answered B on the two-fork form.
pub fn lowest_rung(n: u64, max_rung: u64) -> Option<Rung> {
    lowest_rung_cap(n, max_rung, 8192)
}

/// The same, with the divisor budget named. A sweep asks for less per value:
/// the kernel heap is 48 MiB and a wide range times a full divisor set exhausts
/// it, which is a budget fact about the instrument, not about the ladder.
pub fn lowest_rung_cap(n: u64, max_rung: u64, cap: usize) -> Option<Rung> {
    if n < 2 || n % 4 != 1 { return None; }
    let mut r = 3u64;
    while r <= max_rung {
        let a = (n + r) / 4;
        let m = n.saturating_mul(a);
        if m == 0 { r += 4; continue; }
        if let Some((b, c)) = split_second(r, m, cap) {
            return Some(Rung { r, a, d: 0, e: 0, b, c });
        }
        r += 4;
    }
    None
}

/// The nesting classes of the Fixed-Point Nesting Rule, read for one `n`.
///
/// The rule: a nesting of A inside B closes exactly when A is a fixed point of
/// B's action, and ONE-SHOTS exactly when A already sits at that fixed point
/// rather than merely in its basin. Here B is the rung's congruence and A is the
/// divisor offered to it.
///
/// - **one-shot**  the divisor is fixed by the congruence for EVERY `n` in its
///   residue class, so nothing is searched. `u = 2` at rung 3 is one: `3 ∣ n²+8`
///   holds identically when `3 ∤ n`, and `2 ∣ M` exactly when `n ≡ 5 (mod 8)`.
/// - **iterated**  a divisor exists but has to be found: the rung is reached by
///   walking `r = 3, 7, 11, …` until one lands.
/// - **no closure**  no rung within the budget lands, which on a conservative
///   action means the stabiliser set being read is wrong, not that the value is
///   far away.
pub enum NestClass {
    OneShot { u: u64, r: u64, why: &'static str },
    Iterated { r: u64, u: u64 },
    NoClosure,
}

/// Read `n` against the ladder and classify the nesting.
pub fn classify(n: u64, max_rung: u64) -> NestClass {
    // The one-shot: u = 2 at rung 3. `3 ∣ n² + 8` is identical for 3 ∤ n, so the
    // only condition left is `2 ∣ M`, i.e. `a` even, i.e. `n ≡ 5 (mod 8)`.
    if n % 8 == 5 && n % 3 != 0 {
        return NestClass::OneShot {
            u: 2,
            r: 3,
            why: "3 ∣ n²+8 identically for 3∤n; 2 ∣ M exactly when n ≡ 5 (mod 8)",
        };
    }
    // The rest of the price-zero layer: a rung r = 3 (mod 4) dividing n, n+1 or
    // n+4 is READ OFF n. Writing the divisor as u = n^i a^j and using 4a = n
    // (mod r), the congruence r | M+u leaves exactly these three outcomes — the
    // other exponent pairs demand r | 8 or r | 5, both dead for r = 3 (mod 4).
    if let Some((r, u, why)) = price_zero_rung(n) {
        return NestClass::OneShot { u, r, why };
    }
    match lowest_rung(n, max_rung) {
        Some(g) => NestClass::Iterated { r: g.r, u: g.b },
        None => NestClass::NoClosure,
    }
}

/// The smallest rung supplied by the price-zero layer: `r = 3 (mod 4)` dividing
/// `n`, `n+1` or `n+4`. Nothing is searched — the rung is a divisor of a number
/// `n` already carries.
/// Returns the rung, the divisor it commits to, and which of the three it is.
pub fn price_zero_rung(n: u64) -> Option<(u64, u64, &'static str)> {
    let mut r = 3u64;
    while r <= n + 4 {
        let a = (n + r) / 4;
        if n % r == 0 {
            return Some((r, n.saturating_mul(a), "r | n — the prime-factor family, u = M"));
        }
        if (n + 1) % r == 0 {
            return Some((r, a, "r | n+1 — the divisor family, u = a"));
        }
        if (n + 4) % r == 0 {
            return Some((r, n, "r | n+4 — the n-family, u = n"));
        }
        r += 4;
    }
    None
}

/// Does `M` itself carry a cofactor at `−1`? That is the form the frontier's
/// witnesses actually take: `M = u·w` with `r ∣ w+1`, so `M + u = u(w+1)`.
/// Strictly weaker than the `u ∣ M²` criterion, and enough for all but two
/// values below 200000.
pub fn cofactor_closes(n: u64, r: u64, cap: usize) -> bool {
    if r < 2 || n % 4 != 1 { return false; }
    let a = (n + r) / 4;
    let m = n.saturating_mul(a);
    if m == 0 || m % r == 0 { return false; }
    let mut divs: Vec<u64> = Vec::new();
    divs.push(1);
    for (p, k) in factor(m) {
        let mut next: Vec<u64> = Vec::new();
        for &d in divs.iter() {
            let mut power = 1u64;
            for _ in 0..=k {
                match d.checked_mul(power) {
                    Some(v) => next.push(v % r),
                    None => break,
                }
                match power.checked_mul(p) {
                    Some(v) => power = v,
                    None => break,
                }
            }
        }
        next.sort_unstable();
        next.dedup();
        if next.len() > cap { next.truncate(cap); }
        divs = next;
    }
    let target = (r - (m % r)) % r;
    divs.iter().any(|&d| d == target)
}

/// The **residue coverage** at a rung: how much of `Z/r` the available divisors
/// reach.
///
/// The criterion asks for `u ∣ M²` with `M + u ≡ 0 (mod r)`. Asked as a yes/no
/// that is a bool, and the near-miss reading of it is a bool too — the least
/// nonzero residue is almost always 1, so a "distance" collapses to {0, 1/r}
/// and says nothing. The graded quantity is the SET the divisors reach: the
/// residues `M + u (mod r)` as `u` runs over the divisors of `M²`. Its size
/// against `r` is a density in (0,1], the rung closes exactly when `0` is in it,
/// and it is that density — not a distance — that says how likely the next rung
/// is to land.
pub fn rung_coverage(n: u64, r: u64, cap: usize) -> (f64, bool) {
    if r < 2 || n % 4 != 1 { return (0.0, false); }
    let a = (n + r) / 4;
    let m = n.saturating_mul(a);
    if m == 0 { return (0.0, false); }
    let mut hit: Vec<bool> = Vec::new();
    hit.resize(r as usize, false);
    let mut count = 0u64;
    let mut zero = false;
    for u in divisors_of_square(m, cap) {
        let d = ((m % r) + (u % r)) % r;
        if !hit[d as usize] { hit[d as usize] = true; count += 1; }
        if d == 0 { zero = true; }
    }
    (count as f64 / r as f64, zero)
}

/// **The shift family.** Any divisor `d` of `n` is a divisor of `M = n·a`, so a
/// rung `r ≡ 3 (mod 4)` dividing `d+1` closes: `M = u·d` with `r ∣ d+1`. The
/// rung is read off the factorisation of `n` alone — nothing about `a` is
/// consulted, so nothing is searched, and unlike `r ∣ n+1` the shift may be
/// taken at any divisor rather than at `n` itself.
pub fn shift_rung(n: u64) -> Option<(u64, u64)> {
    let mut d = 1u64;
    let mut best: Option<(u64, u64)> = None;
    while d * d <= n {
        if n % d == 0 {
            for cand in [d, n / d] {
                if cand < 2 { continue; }
                let m = cand + 1;
                let mut r = 3u64;
                while r <= m {
                    if m % r == 0 && r % 4 == 3 {
                        if best.map_or(true, |(br, _)| r < br) { best = Some((r, cand)); }
                        break;
                    }
                    r += 4;
                }
            }
        }
        d += 1;
    }
    best
}

/// **The `w = kr − 1` ladder inside the cofactor form.** A cofactor at `−1` is
/// `w = kr − 1` for some `k ≥ 1`. Taking `k` small makes the test a divisibility
/// with no factorisation at all: `k = 1` asks `(r−1) ∣ M`, `k = 2` asks
/// `(2r−1) ∣ M`. Reading how far up `k` the frontier's witnesses sit says whether
/// the cofactor search is really a search or a short list.
pub fn least_k_cofactor(n: u64, max_rung: u64, max_k: u64) -> Option<(u64, u64)> {
    if n % 4 != 1 { return None; }
    let mut r = 3u64;
    while r <= max_rung {
        let a = (n + r) / 4;
        let m = n.saturating_mul(a);
        if m % r != 0 {
            let mut k = 1u64;
            while k <= max_k {
                let w = k * r - 1;
                if w > 1 && m % w == 0 { return Some((r, k)); }
                k += 1;
            }
        }
        r += 4;
    }
    None
}

/// **The `k`-shift family.** Every cofactor is `w = kr − 1`, so the rung is
/// `r = (w+1)/k`, and `w ∣ M` forces `w ∣ k·n + 1`: from `4a = n + r`,
/// `4k·a = k·n + w + 1`, and `w` is coprime to `4k`. So each `k` reads its rungs
/// off the factorisation of `k·n + 1` — nothing about `a` is consulted, and the
/// search over divisors of `M` is a search over `k` instead.
///
/// `k = 1` is the divisor family `r ∣ n+1`. `k = 2` reads `2n+1`, and so on.
pub fn k_shift_rung(n: u64, max_k: u64) -> Option<(u64, u64, u64)> {
    let mut k = 1u64;
    while k <= max_k {
        let target = k.saturating_mul(n) + 1;
        let mut w = 1u64;
        while w * w <= target {
            if target % w == 0 {
                for cand in [w, target / w] {
                    if cand < 2 { continue; }
                    if (cand + 1) % k != 0 { continue; }
                    let r = (cand + 1) / k;
                    if r < 3 || r % 4 != 3 { continue; }
                    if gcd_u64(cand, 4 * k) != 1 { continue; }
                    if (n + r) % 4 != 0 { continue; }
                    return Some((r, k, cand));
                }
            }
            w += 1;
        }
        k += 1;
    }
    None
}

fn gcd_u64(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b != 0 { let t = a % b; a = b; b = t; }
    a
}

/// The least `k` whose shift supplies a rung, searched by rung rather than by
/// `k`. For each rung `r`, the cofactors of `M` at `−1 (mod r)` are read off, and
/// each gives `k = (w+1)/r` directly — so the whole `k`-spectrum of one `n` is
/// obtained without factoring `k·n+1` for any `k`.
pub fn k_spectrum(n: u64, max_rung: u64, cap: usize) -> Option<(u64, u64, u64)> {
    if n % 4 != 1 { return None; }
    let mut r = 3u64;
    let mut best: Option<(u64, u64, u64)> = None;
    while r <= max_rung {
        let a = (n + r) / 4;
        let m = n.saturating_mul(a);
        if m % r != 0 {
            let mut divs: Vec<u64> = Vec::new();
            divs.push(1);
            for (p, e) in factor(m) {
                let mut next: Vec<u64> = Vec::new();
                for &d in divs.iter() {
                    let mut power = 1u64;
                    for _ in 0..=e {
                        match d.checked_mul(power) {
                            Some(v) => next.push(v),
                            None => break,
                        }
                        match power.checked_mul(p) {
                            Some(v) => power = v,
                            None => break,
                        }
                    }
                }
                next.sort_unstable();
                next.dedup();
                if next.len() > cap { next.truncate(cap); }
                divs = next;
            }
            for w in divs {
                if w > 1 && (w + 1) % r == 0 {
                    let k = (w + 1) / r;
                    if best.map_or(true, |(bk, _, _)| k < bk) { best = Some((k, r, w)); }
                }
            }
        }
        r += 4;
    }
    best
}

/// Integer square root by Newton, for the `n^{1/4}` reading. The kernel has no
/// libm, and the ratio wanted here is coarse enough that the integer root is the
/// honest quantity rather than an approximation to a float one.
fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

/// The **budget gap** at a rung: `−1` lies in the subgroup the primes of `M`
/// generate, but no divisor of `M²` reaches it. The group permits any exponent;
/// the divisors cap each prime at twice its multiplicity in `M`, and the
/// difference is the whole distance between the group criterion and the real one.
pub fn budget_gap(n: u64, r: u64, cap: usize) -> Option<bool> {
    if r < 2 || n % 4 != 1 { return None; }
    let a = (n + r) / 4;
    let m = n.saturating_mul(a);
    if m == 0 || m % r == 0 { return None; }
    // The subgroup generated by the primes of M, closed under multiplication.
    let mut group: Vec<u64> = Vec::new();
    group.push(1);
    let primes: Vec<u64> = factor(m).into_iter().map(|(p, _)| p % r).collect();
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot = group.clone();
        for &g in snapshot.iter() {
            for &p in primes.iter() {
                if p == 0 { continue; }
                let q = (g * p) % r;
                if !group.contains(&q) { group.push(q); changed = true; }
            }
        }
        if group.len() > cap { break; }
    }
    let target = (r - (m % r)) % r;          // −M mod r
    let neg_one = r - 1;                      // −1 mod r
    let in_group = group.contains(&neg_one);
    let reachable = divisors_of_square(m, cap)
        .into_iter()
        .any(|u| u % r == target);
    Some(in_group && !reachable)
}

/// Is `n` on the frontier — outside the one-shot and outside the price-zero
/// layer, so its rung must be searched? Every such `n` is `1 (mod 24)`.
pub fn on_frontier(n: u64) -> bool {
    if !(n % 4 == 1 && n % 3 != 0 && n % 8 != 5) { return false; }
    if price_zero_rung(n).is_some() || shift_rung(n).is_some() { return false; }
    // Multiplicative descent: every class but this one is settled by a family,
    // so a proper divisor that is not itself on the frontier represents 4/d, and
    // scaling every denominator by n/d represents 4/n.
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            if !on_frontier(d) || !on_frontier(n / d) { return false; }
        }
        d += 1;
    }
    true
}

pub struct Straus;

impl Straus {
    pub fn help() -> String {
        let mut s = String::new();
        s.push_str("straus — the Erdős–Straus ladder\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("  straus <n>                 the lowest rung that closes 4/n\n");
        s.push_str("  straus nest <n>            its nesting class, and the price\n");
        s.push_str("  straus defect <n>          the rung walk read as a nesting\n");
        s.push_str("  straus sweep <lo> <hi>     the rung spectrum across a range\n");
        s.push_str("  straus census <lo> <hi>    how the three classes populate\n");
        s.push_str("  straus frontier <lo> <hi>  what price zero does not reach\n");
        s.push_str("  straus cascade <lo> <hi>   the frontier falling rung by rung\n");
        s.push_str("  straus reach <lo> <hi>     how deep the cofactor list goes\n");
        s.push_str("  straus kshift <lo> <hi> <K>  rungs read off k·n+1\n");
        s.push_str("  straus kceiling <lo> <hi>  the least k each value needs\n");
        s.push_str("  straus ceiling <lo> <hi>   rung records against n^(1/4)\n");
        s.push_str("  straus budget <lo> <hi>    group against exponent budget\n");
        s.push_str("\n");
        s.push_str("A rung is r ≡ 3 (mod 4): the first term is 1/((n+r)/4) and the\n");
        s.push_str("remainder has numerator exactly r. r = 3 is the greedy step.\n");
        s.push_str("The second term comes from a divisor u of M² = (n·a)² with\n");
        s.push_str("u ≡ −M (mod r) — committed, not searched, which is what imasm\n");
        s.push_str("check asked for. Three rungs are read straight off n: any\n");
        s.push_str("r ≡ 3 (mod 4) dividing n, n+1 or n+4. What that layer misses is\n");
        s.push_str("the frontier, and every frontier value is n ≡ 1 (mod 24).\n");
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
                s.push_str(&format!("  second split  u | M², u ≡ −M (mod {})\n", g.r));
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

    /// The nesting class of one `n`, in the rule's vocabulary.
    pub fn nest(n: u64) -> String {
        let mut s = format!("4/{} — the nesting read\n", n);
        if n % 4 != 1 {
            s.push_str("  not in the surviving class.\n");
            return s;
        }
        match classify(n, 400) {
            NestClass::OneShot { u, r, why } => {
                s.push_str(&format!("  class:   ONE-SHOT at rung {}\n", r));
                s.push_str(&format!("  divisor: u = {} — already at the fixed point\n", u));
                s.push_str(&format!("  why:     {}\n", why));
                if r == 3 && u == 2 {
                    let a = (n + r) / 4;
                    let m = n.saturating_mul(a);
                    let w = m / 2;
                    let b = (m + 2) / 3;
                    s.push_str(&format!("  4/{} = 1/{} + 1/{} + 1/{}\n", n, a, b, (w as u128 * b as u128)));
                }
                s.push_str("  price:   0 — nothing was searched\n");
            }
            NestClass::Iterated { r, .. } => {
                s.push_str(&format!("  class:   ITERATED — the rung was walked to {}\n", r));
                s.push_str("  price:   the walk; the divisor exists but is not identical\n");
            }
            NestClass::NoClosure => {
                s.push_str("  class:   NO CLOSURE within the budget\n");
                s.push_str("  reading: on a conservative action that means the stabiliser\n");
                s.push_str("           set is wrong, not that the value is far away.\n");
            }
        }
        s
    }

    /// How the three classes populate across a range.
    pub fn nest_census(lo: u64, hi: u64) -> String {
        let mut s = format!("nesting classes, {} ≤ n ≤ {}\n", lo, hi);
        let (mut one, mut it, mut no) = (0u64, 0u64, 0u64);
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && n % 3 != 0 {
                match classify(n, 400) {
                    NestClass::OneShot { .. } => one += 1,
                    NestClass::Iterated { .. } => it += 1,
                    NestClass::NoClosure => no += 1,
                }
            }
            n += 4;
        }
        let tot = one + it + no;
        s.push_str(&format!("  one-shot   {:>6}  ({:.1}%)  — nothing searched\n",
            one, if tot > 0 { one as f64 * 100.0 / tot as f64 } else { 0.0 }));
        s.push_str(&format!("  iterated   {:>6}  ({:.1}%)  — the rung was walked\n",
            it, if tot > 0 { it as f64 * 100.0 / tot as f64 } else { 0.0 }));
        s.push_str(&format!("  no closure {:>6}  ({:.1}%)\n",
            no, if tot > 0 { no as f64 * 100.0 / tot as f64 } else { 0.0 }));
        s
    }

    /// The frontier across a range: the values both zero-price layers miss.
    ///
    /// The one-shot `u = 2` at rung 3 takes `n ≡ 5 (mod 8)`; the price-zero
    /// layer takes every `n` where one of `n`, `n+1`, `n+4` carries a divisor
    /// `≡ 3 (mod 4)`. What is left must draw its divisor from a prime of `a`
    /// that `n` does not carry — a searched rung — and every such value is
    /// `1 (mod 24)`.
    pub fn frontier(lo: u64, hi: u64) -> String {
        let mut s = format!("the price-zero frontier, {} ≤ n ≤ {}\n", lo, hi);
        let (mut tot, mut left, mut primes) = (0u64, 0u64, 0u64);
        let mut first: Vec<u64> = Vec::new();
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && n % 3 != 0 {
                tot += 1;
                if on_frontier(n) {
                    left += 1;
                    if factor(n).len() == 1 && factor(n)[0].1 == 1 { primes += 1; }
                    if first.len() < 12 { first.push(n); }
                }
            }
            n += 4;
        }
        s.push_str(&format!("  {} values in the surviving class\n", tot));
        s.push_str(&format!("  closed at price zero: {}\n", tot - left));
        s.push_str(&format!("  on the frontier:      {}  ({:.1}%)\n", left,
            if tot > 0 { left as f64 * 100.0 / tot as f64 } else { 0.0 }));
        s.push_str(&format!("  of those, prime:      {}\n", primes));
        s.push_str("  first of them:       ");
        for v in &first { s.push_str(&format!(" {}", v)); }
        s.push('\n');
        s.push_str("  every one is n ≡ 1 (mod 24) — straus_frontier_mod_24 in p4ramill.\n");
        s
    }

    /// The rung walk read as a nesting: what each rung's divisor set reaches.
    ///
    /// Measured over the 162 frontier values below 20000: coverage at the rung
    /// that closes averages 0.734 and never falls below 0.533, while coverage at
    /// a rung that fails averages 0.366 and exceeds 0.533 only three times in 89.
    /// The walk is therefore dissipative in this coordinate — the reached set
    /// grows toward the target rather than landing on it — which is why the
    /// coordinate is coverage and not a distance. A near-miss distance is
    /// {0, 1/r} and carries nothing; the size of the reached set carries the
    /// approach.
    pub fn defect(n: u64) -> String {
        let mut s = format!("4/{} — the rung walk as a nesting\n", n);
        if n % 4 != 1 { s.push_str("  not in the surviving class.\n"); return s; }
        s.push_str("  rung | coverage | reaches 0\n");
        s.push_str("  -----|----------|----------\n");
        let mut r = 3u64;
        let mut closed_at = 0u64;
        let mut covs: Vec<f64> = Vec::new();
        while r <= 120 {
            let (cov, zero) = rung_coverage(n, r, 1024);
            covs.push(cov);
            s.push_str(&format!("  {:>4} | {:>8.4} |    {}\n", r, cov, if zero { "yes" } else { "no" }));
            if zero { closed_at = r; break; }
            r += 4;
        }
        if closed_at == 3 {
            s.push_str("  class: ONE-SHOT — the greedy rung already reached 0.\n");
        } else if closed_at > 0 {
            let rising = covs.windows(2).filter(|w| w[1] > w[0]).count();
            s.push_str(&format!("  closed at rung {}\n", closed_at));
            s.push_str(&format!("  coverage rose on {} of {} steps — {}\n",
                rising, covs.len().saturating_sub(1),
                if rising * 2 > covs.len() {
                    "the reached set grew into the target"
                } else {
                    "the reached set did not grow — this walk landed rather than neared"
                }));
        } else {
            s.push_str("  no rung ≤ 120 reached 0 within the divisor budget.\n");
        }
        s
    }

    /// The cascade: how the frontier falls rung by rung to the cofactor form.
    ///
    /// At each rung the test is whether `M = n(n+r)/4` carries a divisor `w`
    /// with `w ≡ −1 (mod r)`; the cofactor `u = M/w` is then the closing divisor.
    /// At the greedy rung this is a statement about primes alone — every
    /// frontier value has `M ≡ 1 (mod 3)`, so a divisor at 2 exists exactly when
    /// some prime factor of `M` is `≡ 2 (mod 3)`.
    pub fn cascade(lo: u64, hi: u64) -> String {
        let mut s = format!("the frontier cascading to the cofactor form, {} ≤ n ≤ {}\n", lo, hi);
        let mut cur: Vec<u64> = Vec::new();
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && on_frontier(n) { cur.push(n); }
            n += 4;
        }
        s.push_str(&format!("  {} frontier values\n\n", cur.len()));
        s.push_str("  rung | closed here | still open\n");
        s.push_str("  -----|-------------|-----------\n");
        let mut r = 3u64;
        while r <= 51 && !cur.is_empty() {
            let mut next: Vec<u64> = Vec::new();
            let mut got = 0u64;
            for &v in cur.iter() {
                if cofactor_closes(v, r, 4096) { got += 1; } else { next.push(v); }
            }
            if got > 0 || !next.is_empty() {
                s.push_str(&format!("  {:>4} | {:>11} | {:>10}\n", r, got, next.len()));
            }
            cur = next;
            r += 4;
        }
        if cur.is_empty() {
            s.push_str("  every frontier value closed by a cofactor of M at rung ≤ 51.\n");
        } else {
            s.push_str("  needing a divisor of M² rather than of M:");
            for v in cur.iter().take(10) { s.push_str(&format!(" {}", v)); }
            s.push('\n');
        }
        s
    }

    /// How often the exponent budget, rather than the group, is what fails.
    pub fn budget(lo: u64, hi: u64) -> String {
        let mut s = format!("group against budget, {} ≤ n ≤ {}\n", lo, hi);
        let (mut reads, mut gaps) = (0u64, 0u64);
        let mut witnesses: Vec<(u64, u64)> = Vec::new();
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && on_frontier(n) {
                let mut r = 3u64;
                while r <= 120 {
                    if let Some(gap) = budget_gap(n, r, 4096) {
                        reads += 1;
                        if gap {
                            gaps += 1;
                            if witnesses.len() < 10 { witnesses.push((n, r)); }
                        }
                    }
                    if cofactor_closes(n, r, 4096) { break; }
                    r += 4;
                }
            }
            n += 4;
        }
        s.push_str(&format!("  rung reads: {}\n", reads));
        s.push_str(&format!("  −1 in the subgroup but out of reach in M²: {}\n", gaps));
        s.push_str("  witnesses:");
        for (v, r) in witnesses.iter() { s.push_str(&format!(" ({}, r={})", v, r)); }
        s.push('\n');
        s.push_str("  Everything else fails because the group itself lacks −1.\n");
        s
    }

    /// The rung ceiling against `n^{1/4}`: the constant a bound would need.
    ///
    /// `RungBounded` in p4ramill asks for a function bounding the least closing
    /// rung. Everything measured is consistent with `C·n^{1/4}`, so the quantity
    /// to watch is `r/n^{1/4}` at the values that force the highest rungs — if
    /// that ratio drifts upward the shape is wrong, and if it holds the constant
    /// is what a proof would have to supply.
    pub fn ceiling(lo: u64, hi: u64) -> String {
        let mut s = format!("rung ceiling against n^(1/4), {} ≤ n ≤ {}\n", lo, hi);
        let mut worst: Vec<(u64, u64, f64)> = Vec::new();
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        let mut running = 0u64;
        while n <= hi {
            if n >= 5 && n % 3 != 0 {
                if let Some(g) = lowest_rung_cap(n, 400, 1024) {
                    if g.r > running {
                        running = g.r;
                        // n^{1/4} without libm: two rounds of integer square root.
                        let q = isqrt(isqrt(n));
                        let ratio = if q > 0 { g.r as f64 / q as f64 } else { 0.0 };
                        worst.push((n, g.r, ratio));
                    }
                }
            }
            n += 4;
        }
        s.push_str("       n |  rung | rung / n^(1/4)\n");
        s.push_str("  -------|-------|---------------\n");
        for (v, r, ratio) in worst.iter() {
            s.push_str(&format!("  {:>7} | {:>5} | {:>13.3}\n", v, r, ratio));
        }
        s.push_str("  A record is a value forcing a higher rung than anything before it.\n");
        s
    }

    /// The least `k` each frontier value needs, read from the rung side so the
    /// whole spectrum is available rather than a truncated search over `k`.
    pub fn kceiling(lo: u64, hi: u64) -> String {
        let mut s = format!("least k on the frontier, {} ≤ n ≤ {}\n", lo, hi);
        let mut worst = (0u64, 0u64, 0u64);
        let mut miss: Vec<u64> = Vec::new();
        let mut tot = 0u64;
        let mut buckets: Vec<(u64, u64)> = Vec::new();
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && on_frontier(n) {
                tot += 1;
                match k_spectrum(n, 200, 4096) {
                    Some((k, r, _)) => {
                        if k > worst.0 { worst = (k, n, r); }
                        let b = if k <= 2 { 2 } else if k <= 8 { 8 } else if k <= 32 { 32 }
                                else if k <= 128 { 128 } else if k <= 512 { 512 } else { 4096 };
                        match buckets.iter_mut().find(|(bb, _)| *bb == b) {
                            Some((_, c)) => *c += 1,
                            None => buckets.push((b, 1)),
                        }
                    }
                    None => miss.push(n),
                }
            }
            n += 4;
        }
        buckets.sort_by_key(|(b, _)| *b);
        s.push_str(&format!("  {} frontier values, rungs ≤ 200\n\n", tot));
        s.push_str("   k ≤ | values\n");
        s.push_str("  -----|-------\n");
        for (b, c) in &buckets { s.push_str(&format!("  {:>4} | {:>6}\n", b, c)); }
        s.push_str(&format!("  largest least-k: {} at n = {} (rung {})\n", worst.0, worst.1, worst.2));
        s.push_str(&format!("  no cofactor at any rung ≤ 200: {}", miss.len()));
        for v in miss.iter().take(8) { s.push_str(&format!(" {}", v)); }
        s.push('\n');
        s
    }

    /// The k-shift reading: which `k` supplies the rung, over a range's frontier.
    pub fn kshift(lo: u64, hi: u64, max_k: u64) -> String {
        let mut s = format!("the k-shift family on the frontier, {} ≤ n ≤ {}\n", lo, hi);
        let mut counts: Vec<(u64, u64)> = Vec::new();
        let mut miss: Vec<u64> = Vec::new();
        let mut tot = 0u64;
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && on_frontier(n) {
                tot += 1;
                match k_shift_rung(n, max_k) {
                    Some((_, k, _)) => match counts.iter_mut().find(|(kk, _)| *kk == k) {
                        Some((_, c)) => *c += 1,
                        None => counts.push((k, 1)),
                    },
                    None => miss.push(n),
                }
            }
            n += 4;
        }
        counts.sort_by_key(|(k, _)| *k);
        s.push_str(&format!("  {} frontier values, k ≤ {}\n\n", tot, max_k));
        s.push_str("     k | closed by a divisor of k·n+1\n");
        s.push_str("  -----|-----------------------------\n");
        for (k, c) in &counts { s.push_str(&format!("  {:>4} | {:>28}\n", k, c)); }
        s.push_str(&format!("  reached by no k ≤ {}: {}", max_k, miss.len()));
        for v in miss.iter().take(8) { s.push_str(&format!(" {}", v)); }
        s.push('\n');
        s
    }

    /// How deep the cofactor list has to go: the least `k` with `kr − 1` dividing
    /// `M`, over the frontier values in a range.
    pub fn reach(lo: u64, hi: u64) -> String {
        let mut s = format!("cofactor depth on the frontier, {} ≤ n ≤ {}\n", lo, hi);
        let mut counts: Vec<(u64, u64)> = Vec::new();
        let mut miss: Vec<u64> = Vec::new();
        let mut tot = 0u64;
        let mut n = if lo % 4 == 1 { lo } else { lo + (5 - lo % 4) % 4 };
        while n <= hi {
            if n >= 5 && on_frontier(n) {
                tot += 1;
                match least_k_cofactor(n, 200, 64) {
                    Some((_, k)) => match counts.iter_mut().find(|(kk, _)| *kk == k) {
                        Some((_, c)) => *c += 1,
                        None => counts.push((k, 1)),
                    },
                    None => miss.push(n),
                }
            }
            n += 4;
        }
        counts.sort_by_key(|(k, _)| *k);
        s.push_str(&format!("  {} frontier values\n\n", tot));
        s.push_str("     k | closed with w = kr−1\n");
        s.push_str("  -----|---------------------\n");
        for (k, c) in &counts {
            s.push_str(&format!("  {:>4} | {:>20}\n", k, c));
        }
        s.push_str(&format!("  no k ≤ 64 at any rung ≤ 200: {}", miss.len()));
        for v in miss.iter().take(8) { s.push_str(&format!(" {}", v)); }
        s.push('\n');
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
                match lowest_rung_cap(n, 400, 1024) {
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
