//! abc_iutt.rs — computational port of the real arithmetic in the ABC/IUTT
//! Lean development at p4rakernel/p4ramill/Imscribing (ABC.lean,
//! ABC_Exhaustion.lean, ABC_Spectral.lean, ABC_Certificates.lean,
//! ABC_ScaleClosure.lean). Radical, discrepancy, quality, window maximum,
//! and the scale-link monotonicity those files prove as real theorems.
//!
//! What this deliberately does not port: the DialetheicWitness/Verdict
//! wrapping those files also build. A Verdict there is a Type-valued
//! classification read off an already-finished Prop, computed from
//! outside it, never a component of the proof itself -- its own PLift/
//! erased-fields treatment says so directly. There is no analogous "compute
//! a B" operation to build here; every function below returns the same
//! real number (or the same real inequality's truth value) the Lean
//! theorem is actually about.
//!
//! The abc conjecture itself is not decided by anything here. What is
//! real: rad(abc) ≥ 2 for every admissible triple, discrepancy and quality
//! are exact computations, window maxima are exact over a finite range,
//! and scale-link monotonicity (a later window's maximum is never below
//! an earlier one's) is checked directly by enumeration, not assumed from
//! the Lean proof -- a second, independent instrument answering the same
//! question the Lean theorem answers, not a restatement of it.

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::winding_period::gcd;

/// A positive coprime triple a+b=c — `Imscribing.ABC.Triple`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Triple {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl Triple {
    /// None when a or b is zero, or gcd(a,b) != 1 — the same admissibility
    /// `Triple`'s own hypotheses in ABC.lean require.
    pub fn new(a: u64, b: u64) -> Option<Self> {
        if a == 0 || b == 0 {
            return None;
        }
        if gcd(a, b) != 1 {
            return None;
        }
        Some(Triple { a, b, c: a + b })
    }
}

/// rad(n): the product of n's distinct prime factors. Mathlib's convention
/// rad(0)=1 is preserved for n=0, though it never occurs for an admissible
/// triple's own a, b, or c (ABC.md's own note on this same convention).
pub fn radical(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut m = n;
    let mut r: u64 = 1;
    let mut d: u64 = 2;
    while d * d <= m {
        if m % d == 0 {
            r *= d;
            while m % d == 0 {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        r *= m;
    }
    r
}

/// n is squarefree iff its own radical equals it exactly.
pub fn is_squarefree(n: u64) -> bool {
    radical(n) == n
}

/// rad(a*b*c) for a coprime triple. Pairwise coprimality makes this the
/// product of the three separate radicals — the same shortcut
/// ABC_Certificates.md's own reader uses, not a new identity.
pub fn triple_radical(t: Triple) -> u64 {
    radical(t.a) * radical(t.b) * radical(t.c)
}

/// discrepancy_ε(t) = log(c) − (1+ε) log(rad(abc)) — the diagonal entry
/// ABC_Spectral.lean's Dε matrix carries at (t,t), and the exact quantity
/// `conjecture_iff_uniform_discrepancy` in ABC.lean rewrites the conjecture
/// as a bound on.
pub fn discrepancy(t: Triple, eps: f64) -> f64 {
    let rad = triple_radical(t) as f64;
    (t.c as f64).ln() - (1.0 + eps) * rad.ln()
}

/// quality(t) = log(c)/log(rad(abc)) — ABC.md's own quality bound.
pub fn quality(t: Triple) -> f64 {
    let rad = triple_radical(t) as f64;
    (t.c as f64).ln() / rad.ln()
}

/// primeExcess(t) = log(c) − log(rad(abc)), IUTT_ABC_CONSTRUCTION.md's own
/// name for discrepancy at ε=0 — the one quantity `primeExcess_sublinear_
/// iff_abc` proves is equivalent to abc itself. Not a new computation.
pub fn prime_excess(t: Triple) -> f64 {
    discrepancy(t, 0.0)
}

/// arithmeticPacket(t) = (log(rad(abc)), 0, log(c)/4) — ABC.md's own
/// explicit packet, an `IUTT.Packet` (a 3-coordinate vector). Coordinate 0
/// carries the log-radical directly; coordinate 2 is scaled so the
/// j²-weighted evaluator below reads back log(c) exactly.
pub fn arithmetic_packet(t: Triple) -> [f64; 3] {
    let rad = triple_radical(t) as f64;
    [rad.ln(), 0.0, (t.c as f64).ln() / 4.0]
}

/// IUTT's j²-weighted evaluator (IUTT.md: "the weighted functional sums j²
/// times each coordinate"): 0²p[0] + 1²p[1] + 2²p[2].
pub fn weighted_packet(p: [f64; 3]) -> f64 {
    0.0 * p[0] + 1.0 * p[1] + 4.0 * p[2]
}

/// packet_radical_calibration: arithmeticPacket(t)[0] = log(rad(abc)) —
/// true by construction, checked here rather than left implicit.
pub fn packet_radical_calibration(t: Triple) -> bool {
    let rad = triple_radical(t) as f64;
    (arithmetic_packet(t)[0] - rad.ln()).abs() < 1e-12
}

/// packet_height_calibration: the weighted evaluation of arithmeticPacket(t)
/// equals log(c) exactly — the height coordinate ABC_IUTTClosure.lean's
/// `packet_height_calibration` names, checked here by real computation.
pub fn packet_height_calibration(t: Triple) -> bool {
    let expected = (t.c as f64).ln();
    (weighted_packet(arithmetic_packet(t)) - expected).abs() < 1e-9
}

/// cofinal_holds(ε, t): some window's maximum dominates t's own discrepancy
/// — ABC_TailRegime.lean's `cofinal` property for one triple, checked by
/// the window that trivially contains it (cutoff = t.c), the same witness
/// `exactTailRegime`'s own `cofinal` field uses.
pub fn cofinal_holds(eps: f64, t: Triple) -> bool {
    match window_maximum(eps, t.c) {
        Some((max, _)) => discrepancy(t, eps) <= max + 1e-12,
        None => false,
    }
}

/// The real content of ABC_IUTTClosure.lean's IUTTTailReading, at one
/// scale: the attained triple, its packet, and both calibration checks,
/// all computed directly rather than read off a Verdict.
pub struct TailReading {
    pub cutoff: u64,
    pub triple: Triple,
    pub max_discrepancy: f64,
    pub packet: [f64; 3],
    pub radical_calibrated: bool,
    pub height_calibrated: bool,
}

pub fn tail_reading(eps: f64, cutoff: u64) -> Option<TailReading> {
    let (max_discrepancy, triple) = window_maximum(eps, cutoff)?;
    let packet = arithmetic_packet(triple);
    Some(TailReading {
        cutoff,
        triple,
        max_discrepancy,
        packet,
        radical_calibrated: packet_radical_calibration(triple),
        height_calibrated: packet_height_calibration(triple),
    })
}

/// Report over several scales: each one's attained triple, its IUTT packet,
/// and both calibration checks, plus cofinal coverage of that same triple —
/// the real, checkable content ABC_IUTTClosure.lean's exactIUTTTailClosure
/// packages as a Verdict, read out directly here instead.
pub fn tail_closure_report(eps: f64, cutoffs: &[u64]) -> String {
    let mut out = format!("IUTT tail closure — ε={eps}\n");
    for &cutoff in cutoffs {
        match tail_reading(eps, cutoff) {
            Some(r) => {
                out.push_str(&format!(
                    "  c ≤ {cutoff}: attained ({}, {}, {})  discrepancy={:.6}\n    packet = [{:.6}, {:.6}, {:.6}]  radical_calibrated={}  height_calibrated={}  cofinal={}\n",
                    r.triple.a, r.triple.b, r.triple.c, r.max_discrepancy,
                    r.packet[0], r.packet[1], r.packet[2],
                    r.radical_calibrated, r.height_calibrated,
                    cofinal_holds(eps, r.triple),
                ));
            }
            None => out.push_str(&format!("  c ≤ {cutoff}: window empty\n")),
        }
    }
    out
}

/// Machine-readable counterpart of `tail_closure_report`. The schema is
/// stable and intentionally mirrors `CertifiedMeasurementStream`: each entry
/// carries its cutoff, attained triple, discrepancy, packet, and calibration
/// flags. Numbers are emitted as JSON decimals so the artifact can be archived
/// or handed to a certificate-generation step.
pub fn stream_json_report(eps: f64, cutoffs: &[u64]) -> String {
    let mut entries = Vec::new();
    for &cutoff in cutoffs {
        if let Some(r) = tail_reading(eps, cutoff) {
            entries.push(format!(
                "{{\"cutoff\":{cutoff},\"triple\":{{\"a\":{},\"b\":{},\"c\":{}}},\"discrepancy\":{:.12},\"packet\":[{:.12},{:.12},{:.12}],\"radical_calibrated\":{},\"height_calibrated\":{},\"cofinal\":{}}}",
                r.triple.a, r.triple.b, r.triple.c, r.max_discrepancy,
                r.packet[0], r.packet[1], r.packet[2],
                r.radical_calibrated, r.height_calibrated,
                cofinal_holds(eps, r.triple),
            ));
        }
    }
    format!("{{\"epsilon\":{eps:.12},\"measurements\":[{}]}}", entries.join(","))
}

/// ABC_IUTTAsymptotic.lean's IUTTAsymptoticReading requires a proof that
/// windowMaximum(ε, ·) converges to some real limit as its own `converges`
/// field -- every constructor in that file takes that proof as an
/// unsupplied hypothesis; none derives it. There is no unconditional
/// number to port from that file. What is real and checkable is what the
/// finite sequence itself does: report windowMaximum at each cutoff and
/// its growth against the one before, plainly labeled as finite evidence
/// only -- it can rule out neither convergence nor divergence, the same
/// scope ABC_Certificates.md's own reader table states for its readings.
pub fn growth_report(eps: f64, cutoffs: &[u64]) -> String {
    let mut out = format!("ABC growth diagnostic — ε={eps} (finite evidence only; proves neither convergence nor divergence)\n");
    let mut prev: Option<f64> = None;
    let max_cutoff = cutoffs.iter().copied().max().unwrap_or(0);
    let scan = incremental_window_scan(eps, max_cutoff);
    for &cutoff in cutoffs {
        match scan.iter().find(|(c, _, _)| *c == cutoff) {
            Some((_, max, t)) => {
                let delta = prev.map(|p| max - p);
                out.push_str(&format!(
                    "  c ≤ {cutoff}: max={max:.6}  at ({}, {}, {}){}\n",
                    t.a, t.b, t.c,
                    delta.map(|d| format!("  Δ from previous = {d:+.6}")).unwrap_or_default(),
                ));
                prev = Some(*max);
            }
            None => out.push_str(&format!("  c ≤ {cutoff}: window empty\n")),
        }
    }
    out
}

/// Window N: every ordered positive coprime triple with c ≤ cutoff, cutoff
/// = N+2 in ABC_Exhaustion.lean's own indexing (window 0 has c ≤ 2). Both
/// orderings of each unordered pair are included, since discrepancy and
/// quality are symmetric in a,b (ABC_Symmetry.lean) but the certified
/// counts (e.g. cutoff=100 → 3043) are counts of ORDERED triples.
pub fn enumerate_window(cutoff: u64) -> Vec<Triple> {
    let mut out = Vec::new();
    if cutoff < 2 {
        return out;
    }
    for c in 2..=cutoff {
        let mut a = 1u64;
        while 2 * a < c {
            let b = c - a;
            if gcd(a, b) == 1 {
                out.push(Triple { a, b, c });
                out.push(Triple { a: b, b: a, c });
            }
            a += 1;
        }
        if c % 2 == 0 {
            let a = c / 2;
            if a >= 1 && gcd(a, a) == 1 {
                out.push(Triple { a, b: a, c });
            }
        }
    }
    out
}

/// Enumerate only the newly admitted ordered triples at one height.
pub fn enumerate_height(c: u64) -> Vec<Triple> {
    let mut out = Vec::new();
    if c < 2 { return out; }
    let mut a = 1u64;
    while 2 * a < c {
        let b = c - a;
        if gcd(a, b) == 1 {
            out.push(Triple { a, b, c });
            out.push(Triple { a: b, b: a, c });
        }
        a += 1;
    }
    if c % 2 == 0 {
        let a = c / 2;
        if gcd(a, a) == 1 { out.push(Triple { a, b: a, c }); }
    }
    out
}

/// Radical table for all values up to `limit`, built by a sieve over primes.
pub fn radical_sieve(limit: u64) -> Vec<u64> {
    let n = (limit as usize).saturating_add(1);
    let mut table = vec![1u64; n];
    for p in 2..=limit {
        if table[p as usize] == 1 {
            let mut m = p;
            while m <= limit {
                table[m as usize] = table[m as usize].saturating_mul(p);
                m = m.saturating_add(p);
            }
        }
    }
    table
}

fn cached_discrepancy(t: Triple, eps: f64, radicals: &[u64], logs: &[f64]) -> f64 {
    let rad = radicals[t.a as usize] * radicals[t.b as usize] * radicals[t.c as usize];
    logs[t.c as usize] - (1.0 + eps) * (rad as f64).ln()
}

/// Single-pass window maxima. Each admissible triple is evaluated once.
pub fn incremental_window_scan(eps: f64, max_c: u64) -> Vec<(u64, f64, Triple)> {
    let radicals = radical_sieve(max_c);
    let logs: Vec<f64> = (0..=max_c).map(|n| if n < 1 { 0.0 } else { (n as f64).ln() }).collect();
    let mut out = Vec::new();
    let mut best: Option<(f64, Triple)> = None;
    for c in 2..=max_c {
        for t in enumerate_height(c) {
            let value = cached_discrepancy(t, eps, &radicals, &logs);
            if best.map_or(true, |(old, _)| value > old) { best = Some((value, t)); }
        }
        if let Some((value, t)) = best { out.push((c, value, t)); }
    }
    out
}

/// windowMaximum ε cutoff: the attained maximum discrepancy over the
/// window, with its attaining triple. ABC_Exhaustion.lean's windowMaximum;
/// via windowMaximum_is_largest_eigenvalue, also ABC_Spectral.lean's
/// largest eigenvalue of Dε on the same finite family.
pub fn window_maximum(eps: f64, cutoff: u64) -> Option<(f64, Triple)> {
    enumerate_window(cutoff)
        .into_iter()
        .map(|t| (discrepancy(t, eps), t))
        .fold(None, |best, cur| match best {
            None => Some(cur),
            Some(b) => {
                if cur.0 > b.0 {
                    Some(cur)
                } else {
                    Some(b)
                }
            }
        })
}

/// ScaleLink ε N M: window N's maximum ≤ window M's, for cutoff_n ≤
/// cutoff_m. Checked by direct enumeration at both scales here, not
/// assumed from ABC_ScaleClosure.lean's monotonicity theorem — a second,
/// independent instrument reaching the same real inequality, since window
/// M's triple set contains window N's whenever cutoff_n ≤ cutoff_m, so its
/// maximum can only be at least as large.
pub fn scale_link_holds(eps: f64, cutoff_n: u64, cutoff_m: u64) -> bool {
    if cutoff_n > cutoff_m {
        return false;
    }
    match (window_maximum(eps, cutoff_n), window_maximum(eps, cutoff_m)) {
        (Some((n, _)), Some((m, _))) => n <= m,
        (None, _) => true,
        _ => false,
    }
}

/// The real report: window maximum at the requested cutoffs and epsilon,
/// each with its attaining triple, plus a direct scale-link check between
/// consecutive cutoffs — the finite, computational form of everything
/// ABC_ScaleClosure.lean holds as a Witness, minus the Witness itself.
pub fn window_report(eps: f64, cutoffs: &[u64]) -> String {
    let mut out = format!("ABC window report — ε={eps}\n");
    let mut prev: Option<(u64, f64)> = None;
    for &cutoff in cutoffs {
        let n = enumerate_window(cutoff).len();
        match window_maximum(eps, cutoff) {
            Some((max, t)) => {
                out.push_str(&format!(
                    "  c ≤ {cutoff}  ({n} ordered triples): max discrepancy = {max:.6}  at ({}, {}, {})\n",
                    t.a, t.b, t.c
                ));
                if let Some((prev_cutoff, prev_max)) = prev {
                    let holds = prev_max <= max;
                    out.push_str(&format!(
                        "    scale link {prev_cutoff} → {cutoff}: {prev_max:.6} ≤ {max:.6} : {holds}\n"
                    ));
                }
                prev = Some((cutoff, max));
            }
            None => out.push_str(&format!("  c ≤ {cutoff}: window empty\n")),
        }
    }
    out
}

/// Report only champion-displacement events as the cutoff grows from 2 to
/// max_c. A line is emitted exactly when a newly admitted triple exceeds the
/// current window maximum; ties retain the earlier champion.
pub fn champion_report(eps: f64, max_c: u64) -> String {
    let mut out = format!("ABC champion displacements — ε={eps}, c≤{max_c}\n");
    let mut champion: Option<(f64, Triple)> = None;
    let mut events = 0u64;
    for (c, value, t) in incremental_window_scan(eps, max_c) {
            if champion.map_or(true, |(best, _)| value > best) {
                let previous = champion.map(|(_, old)| format!("({}, {}, {})", old.a, old.b, old.c))
                    .unwrap_or_else(|| "none".into());
                out.push_str(&format!(
                    "  event {events}: c={c}  {} → ({}, {}, {})  discrepancy={value:.6}\n",
                    previous, t.a, t.b, t.c
                ));
                champion = Some((value, t));
                events += 1;
            }
    }
    out.push_str(&format!("events={events}\n"));
    out
}

/// Machine-readable champion displacement events.
pub fn champion_json_report(eps: f64, max_c: u64) -> String {
    let mut events = Vec::new();
    let mut champion: Option<(f64, Triple)> = None;
    for (c, value, t) in incremental_window_scan(eps, max_c) {
            if champion.map_or(true, |(best, _)| value > best) {
                let previous = champion.map(|(_, old)| format!("{{\"a\":{},\"b\":{},\"c\":{}}}", old.a, old.b, old.c));
                events.push(format!("{{\"cutoff\":{c},\"previous\":{},\"triple\":{{\"a\":{},\"b\":{},\"c\":{}}},\"discrepancy\":{value:.12}}}",
                    previous.unwrap_or_else(|| "null".into()), t.a, t.b, t.c));
                champion = Some((value, t));
            }
    }
    format!("{{\"epsilon\":{eps:.12},\"max_c\":{max_c},\"events\":[{}]}}", events.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radical_matches_known_values() {
        assert_eq!(radical(216), 6); // 216 = 2^3 * 3^3, ABC.md's own example
        assert_eq!(radical(1), 1);
        assert_eq!(radical(2), 2);
        assert_eq!(radical(72), 6); // rad(1*8*9) = rad(72), ABC.md's example
    }

    #[test]
    fn squarefree_matches_known_values() {
        assert!(is_squarefree(6));
        assert!(!is_squarefree(72));
        assert!(!is_squarefree(216));
    }

    #[test]
    fn example_189_rejects_the_stronger_false_bound() {
        // ABC.md: (a,b,c)=(1,8,9) rejects c <= rad(abc), since rad(72)=6 < 9.
        let t = Triple::new(1, 8).unwrap();
        assert_eq!(t.c, 9);
        assert_eq!(triple_radical(t), 6);
        assert!(t.c > triple_radical(t));
    }

    #[test]
    fn enumerate_window_matches_certified_small_window() {
        // ABC_LogBounds.md: 27 ordered positive coprime triples with c <= 9.
        assert_eq!(enumerate_window(9).len(), 27);
    }

    #[test]
    fn enumerate_window_matches_certified_c32_window() {
        // ABC_WindowCertificates.md: 323 ordered positive coprime triples
        // through c=32.
        assert_eq!(enumerate_window(32).len(), 323);
    }

    #[test]
    fn window_maximum_matches_certified_nine_window() {
        // ABC_LogBounds.md: certifiedWindowNine encloses windowMaximum(1/10, 7)
        // (index 7 = c<=9) in [226/1000, 227/1000], lower witness (1,8,9).
        let (max, t) = window_maximum(0.1, 9).unwrap();
        assert!(max >= 0.226 && max <= 0.227, "max={max}");
        assert_eq!((t.a.min(t.b), t.a.max(t.b), t.c), (1, 8, 9));
    }

    #[test]
    fn window_maximum_matches_certified_c32_window() {
        // ABC_WindowCertificates.md: [113/500, 227/1000] for c<=32, ε=1/10,
        // lower witness also (1,8,9).
        let (max, t) = window_maximum(0.1, 32).unwrap();
        assert!(max >= 0.226 && max <= 0.227, "max={max}");
        assert_eq!((t.a.min(t.b), t.a.max(t.b), t.c), (1, 8, 9));
    }

    #[test]
    fn scale_link_nine_to_thirty_two_holds() {
        assert!(scale_link_holds(0.1, 9, 32));
    }

    #[test]
    fn squarefree_c_never_has_positive_prime_excess() {
        // IUTT_ABC_CONSTRUCTION.md: for squarefree c, c divides rad(abc),
        // so the error (prime_excess) is nonpositive -- checked directly
        // over a real window, not assumed from the proved family theorem.
        for t in enumerate_window(500) {
            if is_squarefree(t.c) {
                assert!(
                    prime_excess(t) <= 1e-9,
                    "squarefree c={} gave positive prime_excess {}",
                    t.c,
                    prime_excess(t)
                );
            }
        }
    }

    #[test]
    fn packet_calibrations_hold_for_known_triples() {
        // (1,8,9) and (1,80,81), the two attaining triples already confirmed
        // against real certified/reader values above and in the module's
        // own live checks.
        for (a, b) in [(1u64, 8u64), (1, 80)] {
            let t = Triple::new(a, b).unwrap();
            assert!(packet_radical_calibration(t), "radical calibration failed for ({a},{b})");
            assert!(packet_height_calibration(t), "height calibration failed for ({a},{b})");
        }
    }

    #[test]
    fn weighted_packet_matches_log_c_directly() {
        // Independent of packet_height_calibration's own tolerance check:
        // recompute log(c) by hand and compare.
        let t = Triple::new(1, 8).unwrap();
        let expected = 9f64.ln();
        assert!((weighted_packet(arithmetic_packet(t)) - expected).abs() < 1e-9);
    }

    #[test]
    fn cofinal_holds_for_every_triple_in_a_real_window() {
        // ABC_TailRegime.lean's cofinal property, checked directly for
        // every triple in a real window rather than just the attaining one.
        for t in enumerate_window(200) {
            assert!(cofinal_holds(0.1, t), "cofinal failed for ({}, {}, {})", t.a, t.b, t.c);
        }
    }

    #[test]
    fn tail_reading_matches_window_maximum_at_the_same_scale() {
        let r = tail_reading(0.1, 32).unwrap();
        let (max, t) = window_maximum(0.1, 32).unwrap();
        assert_eq!(r.max_discrepancy, max);
        assert_eq!(r.triple, t);
        assert!(r.radical_calibrated);
        assert!(r.height_calibrated);
    }
}
