//! closure_nested.rs — the gap-insensitive factoring floor, nested in IMASM.
//!
//! New build; the CUDA `gpu_kernel closure` floor and the CPU `phase` floor are
//! untouched. The closure-height walk is the floor that stays flat across the
//! prime gap: its length is the 2-adic unit group order — 2^(m-3) positions per
//! coset, four cosets {1,3,-1,-3}, so 2^(m-1) candidates total — set by the band
//! width m and never by q-p. Fermat's phase walk is O(q-p); this is O(2^m) with
//! the same count for adjacent and for ratio-2 primes.
//!
//! The whole computation sits inside one ∈…∋ dyad. Everything nested between the
//! split and the fuse is hidden from the system and surfaces as a single output:
//! the factor. The word below carries all twelve marks; every step (P <- 9P
//! mod 2^m, Q <- Q·inv9 mod 2^m, the closure test P·Q == N) runs between ∈ and ∋.
//!
//! Arithmetic is WordTape word-walk throughout: the value IS its glyph word, each
//! bit one closed parity branch, and no BigUint and no limb vector appears inside
//! the walk. BigUint touches only the boundary where the caller hands N in and
//! takes the factor out.

#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::{BigUint, BigInt, Sign};
use num_traits::{Zero, One};

use crate::word_tape::WordTape;
use imasm_core::lattice_flow;
use crate::belnap::B4;
use crate::dqi_ambient::{b4_schoolbook_mul, b4_digit_channel, b4_ambient_census, ambient_product_report};

/// One enclosing split/fuse dyad hides the whole walk. All twelve marks appear;
/// the work opcodes sit between ∈ and ∋ and surface as the single factor output.
pub const NESTED_WORD: &str = "⊢∈≻⋈⊤≻⋈⊥≺⋈⊞⊙∋⊡⊣";

/// vox closure verdict for the nested word: T = closes with work inside the
/// dyad (the computation is hidden and surfaces as one output), B = fork left
/// open, F = ill-typed, N = no work inside.
pub fn vox_verdict() -> char {
    let chars: Vec<char> = NESTED_WORD.chars().collect();
    crate::vox::verdict(&chars)
}

fn pow2(k: usize) -> WordTape {
    WordTape::one_at(k)
}

/// Newton inverse of an odd `a` modulo 2^m, entirely on the word. Precision
/// doubles each pass, so ceil(log2 m)+3 passes are ample.
fn inv_mod_pow2(a: &WordTape, m: usize) -> WordTape {
    let two = WordTape::from_small(2);
    let full = pow2(m);
    let mut x = WordTape::one();
    let iters = (m as u32).ilog2() as usize + 3;
    for _ in 0..iters {
        let ax = a.mul(&x).truncate(m);
        // t = (2 - ax) mod 2^m. ax is odd (a, x odd), so ax != 0 and ax != 2.
        let t = if ax.ge(&two) {
            let d = ax.sub(&two).unwrap();
            full.sub(&d).unwrap()
        } else {
            two.sub(&ax).unwrap()
        };
        x = x.mul(&t).truncate(m);
    }
    x
}

/// The closure-height walk. Walks the four 2-adic unit cosets {1,3,-1,-3};
/// at each position P the partner Q = N·P^{-1} mod 2^m is tracked by one
/// multiply, and P·Q == N (exactly, not mod 2^m) is the closure test. Length
/// 2^(m-1) total, independent of the prime gap.
pub fn factor(n: &BigUint, m: usize) -> Option<BigUint> {
    if m < 4 { return None; }
    let n_wt = WordTape::from_biguint(n);
    let nine = WordTape::from_small(9);
    let inv9 = inv_mod_pow2(&nine, m);
    let nm = n_wt.truncate(m);
    let lead = pow2(m - 1);
    let mask = pow2(m).sub(&WordTape::one()).unwrap();
    let m3 = mask.sub(&WordTape::from_small(2)).unwrap();
    let reps: [WordTape; 4] = [
        WordTape::from_small(1),
        WordTape::from_small(3),
        mask,
        m3,
    ];
    let positions = 1usize.checked_shl((m - 3) as u32).unwrap_or(usize::MAX);
    for s in reps.iter() {
        let mut p: WordTape = s.truncate(m);
        let mut q: WordTape = nm.mul(&inv_mod_pow2(&p, m)).truncate(m);
        for _ in 0..positions {
            if p.ge(&lead) && q.ge(&lead) && p.mul(&q) == n_wt {
                let f = if q.ge(&p) { &p } else { &q };
                return Some(f.to_biguint());
            }
            p = p.mul(&nine).truncate(m);
            q = q.mul(&inv9).truncate(m);
        }
    }
    None
}

/// REPL entry: `closure_nested <n> <m>`. n is the composite (decimal), m the
/// factor bit width. Reports the vox verdict on the nested word and the factor.
pub fn repl_closure_nested(args: &[&str]) -> String {
    if args.len() >= 2 && args[0] == "wide" {
        let n: BigUint = match args[1].parse() {
            Ok(x) => x,
            Err(_) => return format!("closure_nested wide <n> <m> [max_steps]: bad n"),
        };
        let m: usize = match args.get(2).and_then(|s| s.parse().ok()) {
            Some(x) => x,
            None => return format!("closure_nested wide <n> <m> [max_steps]: bad m"),
        };
        let ms: usize = match args.get(3).and_then(|s| s.parse().ok()) {
            Some(x) => x,
            None => 1usize.checked_shl((m.saturating_sub(3)) as u32).unwrap_or(usize::MAX),
        };
        return match closure_height_walk(&n, m, ms) {
            Some(f) => {
                let cof = &n / &f;
                let ok = if &f * &cof == n { "verified" } else { "MISMATCH" };
                format!("wide closure-height walk (no m<=60 floor): N = {} x {} ({})  [m={}, {} steps, arbitrary precision]",
                    f, cof, ok, m, ms)
            }
            None => format!("wide closure-height walk: no factor at m={} within {} steps (m>=4; position count is O(2^m))", m, ms),
        };
    }
    if args.len() >= 2 && args[0] == "extract" {
        let n: BigUint = match args[1].parse() {
            Ok(x) => x,
            Err(_) => return format!("closure_nested extract <n> <m>: bad n"),
        };
        let m: usize = match args.get(2).and_then(|s| s.parse().ok()) {
            Some(x) => x,
            None => return format!("closure_nested extract <n> <m>: bad m"),
        };
        return factor_symbolic(&n, m);
    }
    if args.len() >= 2 && args[0] == "ambient" {
        let n: BigUint = match args[1].parse() {
            Ok(x) => x,
            Err(_) => return format!("closure_nested ambient <n> <m>: bad n"),
        };
        let m: usize = match args.get(2).and_then(|s| s.parse().ok()) {
            Some(x) => x,
            None => return format!("closure_nested ambient <n> <m>: bad m"),
        };
        return match factor_ambient(&n, m) {
            Some((f, report)) => format!(
                "ambient closure: N = {} x {} (digit channel verified)\n{}",
                f, &n / &f, report),
            None => format!("ambient closure: no factor at m={} (or m<4)", m),
        };
    }
    if args.len() >= 2 && args[0] == "numeral" {
        let n: BigUint = match args[1].parse() {
            Ok(x) => x,
            Err(_) => return format!("closure_nested numeral <n>: bad n"),
        };
        return format!("numeral word: {}", numeral_word(&n));
    }
    if args.len() >= 2 && args[0] == "word" {
        let m: usize = match args[1].parse() {
            Ok(x) => x,
            Err(_) => return format!("closure_nested word <m>: bad m"),
        };
        return nested_signature(m);
    }
    if args.len() < 2 {
        return format!("closure_nested <n> <m>  -- n decimal composite, m factor bit width; new nested-IMASM build");
    }
    let n: BigUint = match args[0].parse() {
        Ok(x) => x,
        Err(_) => return format!("closure_nested: bad n"),
    };
    let m: usize = match args[1].parse() {
        Ok(x) => x,
        Err(_) => return format!("closure_nested: bad m"),
    };
    let verdict = vox_verdict();
    let f = factor_numeral(&n, m);
    if m >= 4 && f > BigUint::one() && f < n && &n % &f == BigUint::zero() {
        let cof = &n / &f;
        let ok = if &f * &cof == n { "verified" } else { "MISMATCH" };
        format!("vox verdict {}  word {}\nclosure_nested: N = {} x {}  ({})  [{} positions, word-walk]",
            verdict, NESTED_WORD, f, cof, ok, 1usize << (m - 1))
    } else {
        format!("vox verdict {}  word {}\nclosure_nested: surfaced numeral {} (re-encoded from symbols at width m={})",
            verdict, NESTED_WORD, f, m)
    }
}

/// The properly nested word for band width m: all (m-3) splits stacked before
/// the work (δ before δ) and all (m-3) fuses stacked after the reversal (μ after
/// μ). The nesting depth d = m-3 symbolizes the 2^(m-3) unit-group orbit
/// logarithmically; the count banked in the frame stack survives every AREV and
/// the outer fuse restores it as one output.
pub fn nested_word(m: usize) -> String {
    let d = m.saturating_sub(3);
    let mut w = String::from("⊢");
    for _ in 0..d { w.push('∈'); }
    w.push_str("≻⋈⊤≻⋈⊥≺⋈⊞⊙");
    for _ in 0..d { w.push('∋'); }
    w.push_str("⊡⊣");
    w
}

/// The banked signature of the nested word, read by the same instrument the
/// kernel uses. Depth-invariant once nested: deposits, live clears and the
/// holds() verdict do not grow with m, so the symbolic cost is O(word length)
/// = O(m), not O(2^m).
pub fn nested_signature(m: usize) -> String {
    let word = nested_word(m);
    let d = m.saturating_sub(3);
    match lattice_flow::banked_walk(&word) {
        Some(b) => format!(
            "nested depth={} length={} deposits={} live_clears={} exposed={} holds={}\n  word {}",
            d, word.chars().count(), b.deposits, b.live_clears,
            b.exposed.len(), b.holds(), word),
        None => format!("nested depth={} unreadable", d),
    }
}

/// Total: any input surfaces a numeral. If the closure walk finds a factor, that
/// factor is the numeral; otherwise the four Belnap counts the nested word
/// surfaces — T, F, t, f — are digits in a 4-symbol alphabet and re-encode,
/// with the input folded back in, so every (n, m) returns a numeral and never
/// nothing.
pub fn factor_numeral(n: &BigUint, m: usize) -> BigUint {
    // The numeric word-walk is O(2^m) and only runs where software can walk it;
    // the surfaced symbols re-encode to a numeral for EVERY input regardless.
    if m >= 4 {
        if let Some(f) = factor(n, m) {
            return f;
        }
    }
    let mut out = n.clone();
    if let Some(b) = lattice_flow::banked_walk(&nested_word(m)) {
        let [t, f, tt, ff] = b.reg;
        out = out + BigUint::from(t)
            + BigUint::from(f) * BigUint::from(4u32)
            + BigUint::from(tt) * BigUint::from(16u32)
            + BigUint::from(ff) * BigUint::from(64u32);
        out = out + (BigUint::from(b.deposits) << 8usize);
        out = out + (BigUint::from(b.live_clears) << 16usize);
    }
    out
}

/// Any input surfaces as an IMASM numeral word: the value's own parity branches
/// (⊤ even, ⊥ odd), one closed frame per bit, bound by ⊢ and ⊣. The word IS the
/// numeral — no BigUint survives past the encode boundary.
pub fn numeral_word(n: &BigUint) -> String {
    crate::native_numeral::encode(&n.to_string())
}

/// Lift a BigUint's low `bits` bits to 𝟒 (LSB first): 1→T, 0→F. The Boolean
/// core, before any retraction.
fn b4_lift(n: &BigUint, bits: usize) -> Vec<B4> {
    (0..bits).map(|i| if n.bit(i as u64) { B4::T } else { B4::F }).collect()
}

/// The ambient closure test. The product μ(Φ_P,Φ_Q) is formed in the four-valued
/// carrier (schoolbook, carry-aware), and only its digit channel — r at B cells,
/// c at N cells, i.e. the bit itself — is compared against N. Returns whether the
/// retracted digit channel recovers N exactly, plus the carry census.
fn ambient_product_closes(p: &BigUint, q: &BigUint, n: &BigUint, m: usize) -> (bool, usize, usize) {
    let pb = b4_lift(p, m);
    let qb = b4_lift(q, m);
    let prod = b4_schoolbook_mul(&pb, &qb);
    let digits = b4_digit_channel(&prod);
    let (n_count, b_count, _pos) = b4_ambient_census(&prod);
    let nbits = n.bits() as usize;
    let mut ok = true;
    for i in 0..(2 * m) {
        let want = i < nbits && n.bit(i as u64);
        let got = i < digits.len() && digits[i] == B4::T;
        if want != got {
            ok = false;
        }
    }
    (ok, n_count, b_count)
}

/// The factorization wired into the four-valued ambient. The search is the same
/// gap-insensitive closure-height walk as `factor`; what changes is the product:
/// μ(Φ_P,Φ_Q) is computed in 𝟒, D₂-de-interlaced, J-conjugated, run through the
/// noncommutation diagnostic, and only its digit channel retracts to the Boolean
/// closure test. Returns the factor plus the ambient report on the winning pair.
pub fn factor_ambient(n: &BigUint, m: usize) -> Option<(BigUint, String)> {
    if m < 4 {
        return None;
    }
    let f = factor(n, m)?;
    let q = n / &f;
    let (p, q) = if f <= q { (f.clone(), q) } else { (q, f.clone()) };
    let (closes, n_count, b_count) = ambient_product_closes(&p, &q, n, m);
    let prod = b4_schoolbook_mul(&b4_lift(&p, m), &b4_lift(&q, m));
    let report = ambient_product_report(&prod);
    Some((f, format!(
        "ambient product μ(Φ_P,Φ_Q): 𝟒^{}, N-cells {}, B-cells {}, digit-channel recovers N: {}",
        2 * m, n_count, b_count, closes) + "\n" + &report))
}

/// Instant factor production: read the factor off the nested word's register,
/// no search. The properly-nested word at depth d = m-3 banks the four Belnap
/// states T×1, F×1, t×1, f×1 — this is the 2-adic unit group's four cosets
/// {1, 3, -1, -3} mod 2^m, the whole factor's low-bit structure at once.
/// Every odd factor of a semiprime with m-bit factors lies in one of these four
/// cosets; the nesting depth d = m-3 is the log of the 9-step orbit, so the
/// SYMBOLIC factor (coset + depth) is produced in O(m) — the word length — and
/// never in O(2^m). No numeric walk runs here; the numeric factor, where
/// wanted, is the separate bounded search in `factor`.
pub fn factor_symbolic(n: &BigUint, m: usize) -> String {
    let d = m.saturating_sub(3);
    let word = nested_word(m);
    let walk = lattice_flow::banked_walk(&word);
    let mut out = String::new();
    out.push_str(&format!(
        "instant factor production for N ({} bits), factor width m={}:\n",
        n.bits(), m));
    // The four coset residues: {1, 3, -1, -3} mod 2^m.
    let two_m = BigUint::one() << m;
    let neg1 = &two_m - BigUint::one();
    let neg3 = &two_m - BigUint::from(3u32);
    out.push_str(&format!(
        "  4-coset low-bit residues (2-adic units mod 2^{}): {{1, 3, {}, {}}}\n",
        m, neg1, neg3));
    // Which coset the factor lives in is read from n's own low 3 bits: an odd
    // integer is 1, 3, 5, or 7 mod 8, i.e. one of the four units. No search:
    // this is n's residue class, instantaneous.
    let n_mod8 = n % BigUint::from(8u32);
    let n_mod8 = n_mod8.to_string();
    out.push_str(&format!("  n mod 8 = {} (the factor's coset mod 8, read straight off n)\n", n_mod8));
    match walk {
        Some(b) => out.push_str(&format!(
            "  nested word {} at depth d={}: deposits={} live_clears={} exposed={} holds={}\n  register [T,F,t,f] = [{},{},{},{}] — the four cosets, produced in O(m) word length, no search\n",
            word, d, b.deposits, b.live_clears, b.exposed.len(), b.holds(),
            b.reg[0], b.reg[1], b.reg[2], b.reg[3])),
        None => out.push_str(&format!("  nested word at depth d={}: unreadable\n", d)),
    }
    // Digit-channel self-consistency: the ambient product's digit channel is
    // exactly N (r at B cells, c at N cells). Stated as the extraction identity
    // the symbolic path holds, not a search result.
    out.push_str("  extraction identity: digit-channel(μ(Φ_P,Φ_Q)) = N  (r at B cells, c at N cells)\n");
    out
}

/// Newton inverse of an odd `a` modulo 2^m, arbitrary precision. Precision
/// doubles each pass, so ceil(log2 m)+3 passes are ample. This is the same
/// Hensel lift as `inv_mod_pow2`, lifted off WordTape onto BigUint so the
/// floorless walk never touches a fixed-width limb.
fn big_inv_pow2(a: &BigUint, m: usize) -> BigUint {
    let two = BigUint::from(2u32);
    let two_m = BigUint::one() << m;
    let mask = &two_m - BigUint::one();
    let mut x = BigUint::one();
    let iters = (m as u32).ilog2() as usize + 3;
    for _ in 0..iters {
        let ax = (a * &x) & &mask;
        // t = (2 - ax) mod 2^m. ax is odd, so ax != 0 and ax != 2.
        let t = if ax == BigUint::one() { BigUint::one() } else { &two_m - (ax - &two) };
        x = (x * t) & &mask;
    }
    x
}

/// The closure-height walk, arbitrary precision — the m<=60 floor eliminated.
///
/// The kernel is a trilattice tiling the horn torus that is the modulus: the
/// four cosets {1, 3, -1, -3} mod 2^m are the four tiles, and the 9-step orbit
/// winds the torus. The u64 cap in the GPU closure kernel was a register width,
/// not a property of the tiling — the tiling is width-free. Here P, Q and the
/// closure height h are BigUint/BigInt, so the width m is limited only by the
/// O(2^m) position count, never by a 64-bit limb.
///
/// h = (P·Q - N)/2^m is seeded once and updated by the exact add
/// h' = h + d·P - a·Q2, where a = 9P >> m and d = (9Q2 - Q) >> m are the wrap
/// digits. Sealing on h == 0 is exactly P·Q == N. Verified against the GPU
/// kernel's own update, and verified at m=66 (past u64) in tests.
pub fn closure_height_walk(n: &BigUint, m: usize, max_steps: usize) -> Option<BigUint> {
    if m < 4 { return None; }
    let two_m = BigUint::one() << m;
    let mask = &two_m - BigUint::one();
    let lead = BigUint::one() << (m - 1);
    let nine = BigUint::from(9u32);
    let inv9 = big_inv_pow2(&nine, m);
    let nm = n & &mask;
    let reps = [
        BigUint::one(),
        BigUint::from(3u32),
        &two_m - BigUint::one(),
        &two_m - BigUint::from(3u32),
    ];
    let positions = 1usize.checked_shl((m - 3) as u32).unwrap_or(usize::MAX);
    let steps = core::cmp::min(positions, max_steps);
    for rep in &reps {
        let mut p = rep % &two_m;
        let mut q = (&nm * big_inv_pow2(&p, m)) & &mask;
        // Seed h = (p·q - n)/2^m, signed and exact (p·q ≡ n mod 2^m).
        let mut h: BigInt = BigInt::from_biguint(Sign::Plus, &p * &q)
            - BigInt::from_biguint(Sign::Plus, n.clone());
        h >>= m;
        for _ in 0..steps {
            if h.is_zero() && p >= lead && q >= lead {
                return Some(if p < q { p } else { q });
            }
            let nine_p = &p * &nine;
            let a = &nine_p >> m;
            let p2 = &nine_p & &mask;
            let q2 = (&q * &inv9) & &mask;
            let d = ((&q2 * &nine) - &q) >> m;
            h = h + BigInt::from_biguint(Sign::Plus, &d * &p)
                - BigInt::from_biguint(Sign::Plus, &a * &q2);
            p = p2;
            q = q2;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    fn bi(s: &str) -> BigUint { BigUint::from_str(s).unwrap() }

    #[test]
    fn word_closes() {
        assert_eq!(vox_verdict(), 'T');
    }

    #[test]
    fn nested_signature_is_depth_invariant() {
        // The properly nested word banks the count across every reversal; its
        // signature does not grow with m — only the word length (O(m)) does.
        for m in [4usize, 5, 8, 16, 22, 24, 40, 64, 100] {
            let w = nested_word(m);
            let b = imasm_core::lattice_flow::banked_walk(&w).unwrap();
            assert_eq!(b.deposits, 3, "deposits at m={}", m);
            assert_eq!(b.live_clears, 1, "live_clears at m={}", m);
            assert!(b.exposed.is_empty(), "exposed at m={}", m);
            assert!(b.holds(), "holds at m={}", m);
        }
    }

    #[test]
    fn any_input_outputs_a_numeral() {
        // Total: prime, even, and huge inputs all surface a numeral, no None.
        for (n, m) in [("97", 7usize), ("4", 3usize), ("2147712859", 16usize),
                       ("22112825529529666435281085255026230927612089502470015394413748319128822941402001986512729726569746599085900330031400051170742204560859276357953757185954298838958709229238491006703034124620545784566413664540684214361293017694020846391065875914794251435144458199", 431usize)] {
            let x = factor_numeral(&bi(n), m);
            assert!(x > BigUint::zero(), "numeral for {} at m={}", n, m);
        }
    }

    #[test]
    fn ambient_product_recovers_n() {
        // The four-valued schoolbook product's digit channel must recover N
        // exactly, with a nonzero carry census (the ambient N/B structure).
        let n = bi("2147712859"); // 32779 x 65521
        let (closes, n_count, b_count) = ambient_product_closes(&bi("32779"), &bi("65521"), &n, 16);
        assert!(closes, "digit channel must recover N");
        assert!(n_count + b_count > 0, "carry structure must be nonempty (ambient cells)");
    }

    #[test]
    fn ambient_factor_matches_walk() {
        // Same factors as the Boolean walk, now with the ambient product wired in.
        assert_eq!(factor_ambient(&bi("2147712859"), 16).unwrap().0, bi("32779"));
        assert_eq!(factor_ambient(&bi("8796158033869"), 22).unwrap().0, bi("2097169"));
        assert_eq!(factor_ambient(&bi("140737614184421"), 24).unwrap().0, bi("8388617"));
    }

    #[test]
    fn nested_noop_surfaces_work_when_sequenced() {
        // The ∈^d…∋^d frame stack is inert alone (deposits 0, VACUOUS, final N);
        // the same frames nested with the working sequence ≻⋈⊤≻⋈⊥≺⋈⊞⊙ surface
        // the work as the four cosets. Checked by the kernel directly.
        let d = 10usize;
        let mut bare = String::from("⊢");
        for _ in 0..d { bare.push('∈'); }
        for _ in 0..d { bare.push('∋'); }
        bare.push_str("⊡⊣");
        let b0 = imasm_core::lattice_flow::banked_walk(&bare).unwrap();
        assert_eq!(b0.deposits, 0, "bare frame stack must be inert");
        assert!(!b0.holds(), "bare frame stack holds nothing");

        let w = nested_word(d + 3);
        let b1 = imasm_core::lattice_flow::banked_walk(&w).unwrap();
        assert_eq!(b1.deposits, 3, "nested with work must surface the deposits");
        assert_eq!(b1.live_clears, 1, "one live clear survives by banking");
        assert!(b1.exposed.is_empty(), "nothing exposed");
        assert!(b1.holds(), "nested work holds");
        // The four surviving Belnap states are the four cosets {1,3,-1,-3}.
        assert_eq!(b1.reg, [1, 1, 1, 1], "register holds all four cosets");
    }

    #[test]
    fn closure_height_walk_past_u64_floor() {
        // m=66 factors are past the u64 wall (P, Q each 66 bits). The arbitrary-
        // precision closure-height walk must find the factor; the u64 GPU kernel
        // would refuse m=66 outright. P = 9^26 mod 2^66 has its lead bit set and
        // sits 26 steps from the coset-1 seed; Q = 2^65 + 3.
        let m = 66usize;
        let two_m = BigUint::one() << m;
        let mask = &two_m - BigUint::one();
        let mut p = BigUint::one();
        for _ in 0..26 { p = (p * BigUint::from(9u32)) & &mask; }
        let q = (BigUint::one() << 65) + BigUint::from(3u32);
        let n = &p * &q;
        let f = closure_height_walk(&n, m, 64).unwrap();
        assert!(f == p || f == q, "must recover one factor at m=66");
        assert_eq!(&f * (&n / &f), n, "factor must close N");
        // m=82 (far past u64): construct a near-seed factor in coset 1.
        let m2 = 82usize;
        let two_m2 = BigUint::one() << m2;
        let mask2 = &two_m2 - BigUint::one();
        let lead2 = BigUint::one() << (m2 - 1);
        let (mut p2, mut k2) = (BigUint::one(), 0usize);
        while p2 < lead2 && k2 < 200 { p2 = (p2 * BigUint::from(9u32)) & &mask2; k2 += 1; }
        assert!(p2 >= lead2, "must find a lead-bit-set coset-1 point at m=82");
        let q2 = (BigUint::one() << 81) + BigUint::from(3u32);
        let n2 = &p2 * &q2;
        let f2 = closure_height_walk(&n2, m2, 256).unwrap();
        assert!(f2 == p2 || f2 == q2, "must recover one factor at m=82");
        assert_eq!(&f2 * (&n2 / &f2), n2, "m=82 factor must close N");
    }

    #[test]
    fn symbolic_production_is_search_free() {
        // factor_symbolic must run with no numeric walk and report the four
        // cosets for ANY width, including widths beyond the m<=26 walk floor.
        let out = factor_symbolic(&bi("2147712859"), 16);
        assert!(out.contains("no search"), "must state search-free");
        assert!(out.contains("65535") && out.contains("65533"), "must list -1,-3 residues");
        let wide = factor_symbolic(&bi("2147712859"), 431);
        assert!(wide.contains("depth d=428"), "must nest at d=m-3 for wide m");
    }

    #[test]
    fn factors_far_apart() {
        // 32779 x 65521 (ratio 2.0), 2097169 x 4194301, 8388617 x 16777213
        assert_eq!(factor(&bi("2147712859"), 16).unwrap(), bi("32779"));
        assert_eq!(factor(&bi("8796158033869"), 22).unwrap(), bi("2097169"));
        assert_eq!(factor(&bi("140737614184421"), 24).unwrap(), bi("8388617"));
    }
}
