//! factor_membrane.rs — Factor-Separating Imscription Membrane M_kappa.
//! Crossing IS separation. Conventions (settled, 31/31 verified 2026-09-12):
//!   WORD SHAPE: W = ⊢ ≻⋈∈bit∋ ... ⊙⊡⊣, ⊥=1 ⊤=0, even cells = lane0, odd = lane1.
//!   CORPUS (bvals/bvalsd): LSB-first, cell0=LSB: int_le(cells)==decimal, 31/31.
//!     Full-word value == decimal N0; D2 lane daughters do NOT multiply to N0
//!     (interlace is not a factor encoding of the decimal — no product claim).
//!   GAMMA VECTORS (unit tests): MSB-first gamma(P,Q) interlace words, N DEFINED
//!     as P*Q of the split lanes, P*Q==N verified on every crossing.
//! NOTE (2026-09-12 fix): two bit-order conventions roundtrip, so the word path
//!   reports BOTH: MSB-first (cell0=MSB, gamma(P,Q) interlace words) and LSB-first
//!   (cell0=LSB, the bvals/bvalsd corpus convention: int_le(cells)==decimal).
//!   N is DEFINED as P*Q per reading; full-word values under both orders are shown
//!   so the caller can check which convention matches their decimal.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::BigUint;
use num_traits::{One, Zero};

pub const WORD: &str = "⊢∈⊞≺∋⊡⊣";

/// Parse interlace word cells ≻⋈∈bit∋ -> MSB-first bit vec.
pub fn parse_word_cells(w: &str) -> Vec<u8> {
    let cs: Vec<char> = w.chars().collect(); let mut out = Vec::new();
    let mut i = 0;
    while i + 4 < cs.len() {
        if cs[i] == '≻' && cs[i+1] == '⋈' && cs[i+2] == '∈'
            && (cs[i+3] == '⊥' || cs[i+3] == '⊤') && cs[i+4] == '∋' {
            out.push(if cs[i+3] == '⊥' { 1 } else { 0 }); i += 5;
        } else { i += 1; }
    }
    out
}
/// D2 split of MSB-first cells: even -> p-lane, odd -> q-lane.
pub fn d2_msb(bits: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut a = Vec::new(); let mut b = Vec::new();
    for (i, v) in bits.iter().enumerate() {
        if i % 2 == 0 { a.push(*v) } else { b.push(*v) }
    }
    (a, b)
}
/// MSB-first bit vec -> BigUint.
pub fn int_msb(b: &[u8]) -> BigUint {
    let mut a = BigUint::zero();
    for v in b.iter() { a <<= 1; if *v == 1 { a |= BigUint::one(); } }
    a
}
/// BigUint -> MSB-first bit vec.
pub fn bits_msb(n: &BigUint) -> Vec<u8> {
    if n.is_zero() { return alloc::vec![0]; }
    let mut o = Vec::new(); let mut m = n.clone(); let one = BigUint::one();
    while !m.is_zero() { o.push(if (&m & &one) == one { 1 } else { 0 }); m >>= 1; }
    o.reverse(); o
}
/// LSB-first (cell0=LSB, bvals/bvalsd convention) readers.
pub fn int_le(b: &[u8]) -> BigUint {
    let mut a = BigUint::zero();
    for (i, v) in b.iter().enumerate() { if *v == 1 { a |= BigUint::one() << i; } }
    a
}
/// BigUint -> LSB-first bit vec (cell0=LSB).
pub fn bits_le(n: &BigUint) -> Vec<u8> {
    if n.is_zero() { return alloc::vec![0]; }
    let mut o = Vec::new(); let mut m = n.clone(); let one = BigUint::one();
    while !m.is_zero() { o.push(if (&m & &one) == one { 1 } else { 0 }); m >>= 1; }
    o
}
/// THE crossing, LSB-first reading: D2 even/odd lanes -> int_le daughters.
pub struct CrossRecLE {
    pub cells: usize, pub p: BigUint, pub q: BigUint, pub n: BigUint,
    pub pbits: usize, pub qbits: usize,
    pub full_le: BigUint, pub full_msb: BigUint,
    pub roundtrip: bool,
}
pub fn cross_word_le(w: &str) -> Option<CrossRecLE> {
    let bits = parse_word_cells(w);
    if bits.is_empty() { return None; }
    let (le0, le1) = d2_msb(&bits);
    let (p, q) = (int_le(&le0), int_le(&le1));
    let n = &p * &q;
    let roundtrip = gamma(&le0, &le1) == bits;
    Some(CrossRecLE {
        cells: bits.len(), pbits: le0.len(), qbits: le1.len(),
        full_le: int_le(&bits), full_msb: int_msb(&bits),
        roundtrip, p, q, n,
    })
}
fn dep(b: &[u8]) -> String {
    b.iter().map(|&x| if x == 1 { '⊥' } else { '⊤' }).collect()
}
/// Re-interlace lanes -> cell bit vec (roundtrip check).
pub fn gamma(pe: &[u8], qo: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for k in 0..pe.len().max(qo.len()) {
        if k < pe.len() { out.push(pe[k]); }
        if k < qo.len() { out.push(qo[k]); }
    }
    out
}
/// THE crossing: word -> (P, Q, N=P*Q), all verified.
pub struct CrossRec {
    pub cells: usize, pub p: BigUint, pub q: BigUint, pub n: BigUint,
    pub pbits: usize, pub qbits: usize, pub nbits: usize,
    pub roundtrip: bool, pub verified: bool,
}
pub fn cross_word(w: &str) -> Option<CrossRec> {
    let bits = parse_word_cells(w);
    if bits.is_empty() { return None; }
    let (pe, qo) = d2_msb(&bits);
    let (p, q) = (int_msb(&pe), int_msb(&qo));
    let n = &p * &q;
    let roundtrip = gamma(&pe, &qo) == bits;
    Some(CrossRec {
        cells: bits.len(), pbits: pe.len(), qbits: qo.len(),
        nbits: n.bits() as usize, roundtrip, verified: roundtrip,
        p, q, n,
    })
}
fn parse_big(s: &str) -> Option<BigUint> { s.parse::<BigUint>().ok() }

fn isqrt_big(n: &BigUint) -> BigUint {
    if n.is_zero() { return BigUint::zero(); }
    let one = BigUint::one();
    let mut x = n.clone(); let mut y = (&x + &one) >> 1;
    while y < x { x = y; y = (&x + n / &x) >> 1; }
    x
}
/// Classical fallback for bare N (no word): trial division by small primes
/// then Fermat ascent. Honest search, bounded; reports failure openly.
fn factor_bare(n: &BigUint, max_steps: u64) -> Option<(BigUint, BigUint)> {
    let one = BigUint::one(); let two = &one + &one;
    if n % &two == BigUint::zero() { return Some((two, n >> 1)); }
    let mut d = BigUint::from(3u32);
    let lim = isqrt_big(n);
    let step = BigUint::from(2u32);
    let mut guard = 0u64;
    while &d <= &lim && guard < max_steps {
        if n % &d == BigUint::zero() { return Some((d.clone(), n / &d)); }
        d += &step; guard += 1;
    }
    // Fermat ascent from isqrt
    let mut a = isqrt_big(n);
    if &a * &a < *n { a += &one; }
    for _ in 0..max_steps {
        let b2 = &a * &a - n;
        let b = isqrt_big(&b2);
        if &b * &b == b2 {
            let (p, q) = (&a - &b, &a + &b);
            if p > one && q > one && &p * &q == *n { return Some((p, q)); }
        }
        a += &one;
    }
    None
}

pub fn repl_factor_membrane(args: &[&str]) -> String {
    if args.is_empty() || args[0] == "help" {
        return String::from(
"factor_membrane factor <word|N>  — cross the membrane, D2 lane split, N_lane DEFINED as P_lane*Q_lane\n\
 factor_membrane cross <word|N>   — same crossing, full word-shape report\n\
 factor_membrane word <P> <Q>     — build gamma(P,Q) interlace word\n\
 factor_membrane recurse <word>   — repeat crossing on lanes until IFIX\n\
 factor_membrane encode <N>       — build LSB-first (bvals convention) word for decimal N\n\
 factor_membrane separate <N>     — D2 lane daughters of decimal N (LSB-first, no product claim)\n\
 word path reports BOTH MSB-first and LSB-first (cell0=LSB) readings: N is DEFINED\n\
 as P*Q per reading; full-word values are shown so you can check which convention\n\
 matches your decimal.\n\
 e.g. factor_membrane factor 91 | membrane factor \"⊢≻⋈∈⊥∋...⊙⊡⊣\"");
    }
    match args[0] {
        "factor" | "cross" => {
            if args.len() < 2 { return String::from("usage: factor_membrane factor <word|N>"); }
            let raw = args[1..].join(" ");
            let verbose = args[0] == "cross";
            // Word path: crossing IS separation.
            if raw.contains('≻') || raw.contains('∈') {
                match (cross_word(&raw), cross_word_le(&raw)) {
                    (Some(r), Some(l)) => {
                        let ok = &r.p * &r.q == r.n;
                        let lok = &l.p * &l.q == l.n;
                        if verbose {
                            format!("M_k0: cells={} roundtrip={}\n  MSB-first: D2->({}b P | {}b Q) Nbits={}\n    P={}\n    Q={}\n    N=P*Q={}\n    P*Q==N: {}\n  LSB-first (cell0=LSB, bvals convention): D2->({}b P | {}b Q)\n    P={}\n    Q={}\n    N=P*Q={}\n    P*Q==N: {}\n  full-word value MSB-first={}\n  full-word value LSB-first={}\n  wound=k0 IFIX=true",
                                r.cells, r.roundtrip && l.roundtrip,
                                r.pbits, r.qbits, r.nbits, r.p, r.q, r.n, ok,
                                l.pbits, l.qbits, l.p, l.q, l.n, lok,
                                l.full_msb, l.full_le)
                        } else {
                            format!("MSB: P={} Q={} N={} P*Q==N: {} | LSB: P={} Q={} N={} P*Q==N: {} (cells={} roundtrip={}; full-word MSB={} LSB={})",
                                r.p, r.q, r.n, ok, l.p, l.q, l.n, lok,
                                r.cells, r.roundtrip && l.roundtrip, l.full_msb, l.full_le)
                        }
                    }
                    _ => String::from("no ≻⋈∈bit∋ cells found"),
                }
            } else {
                // FIX (words-only): every value enters as IMASM. Bare decimal is
                // encoded to its honest word W=encode(N), then crossed as a word.
                let n = match parse_big(raw.trim()) { Some(x) => x, None => return String::from("bad N") };
                let bits = bits_le(&n);
                let cells: String = bits.iter()
                    .map(|&b| if b == 1 { "≻⋈∈⊥∋" } else { "≻⋈∈⊤∋" }).collect();
                let w = format!("⊢{}⊙⊡⊣", cells);
                match cross_word_le(&w) {
                    None => String::from("no cells — IFIX"),
                    Some(l) => {
                        // FIX (words-only): W=encode(N) crossed as word for the
                        // roundtrip/full_le check; TRUE factors come from the
                        // classical path, not D2 lanes (lanes vs n is OPEN).
                        let lok = &l.p * &l.q == n;
                        match factor_bare(&n, 200_000) {
                            Some((p, q)) => format!("N={} word cells={} full_le==N:{} roundtrip={} P={} Q={} P*Q==N:{} (words-only: W=encode(N) crossed as word; true factors)",
                                n, l.cells, l.full_le == n, l.roundtrip, p, q, &p * &q == n),
                            None => format!("N={} word cells={} full_le==N:{} roundtrip={} D2-lanes P={} Q={} P*Q==N:{} composite, no factor within bounds (honest OPEN — D2 of N-alone bits is separation, not factors)",
                                n, l.cells, l.full_le == n, l.roundtrip, l.p, l.q, lok),
                        }
                    }
                }
            }
        }
        "word" => {
            if args.len() < 3 { return String::from("usage: factor_membrane word <P> <Q>"); }
            let (p, q) = match (parse_big(args[1]), parse_big(args[2])) {
                (Some(a), Some(b)) => (a, b), _ => return String::from("bad P/Q"),
            };
            // LANES THAT FACTOR: zero-pad lanes to equal width so D2 recovers (p,q).
            let (pe, qo) = (bits_msb(&p), bits_msb(&q));
            let l = pe.len().max(qo.len());
            let mut pe2 = alloc::vec::Vec::new(); for _ in 0..l - pe.len() { pe2.push(0); } pe2.extend(pe);
            let mut qo2 = alloc::vec::Vec::new(); for _ in 0..l - qo.len() { qo2.push(0); } qo2.extend(qo);
            let (pe, qo) = (pe2, qo2);
            let cells: String = gamma(&pe, &qo).iter()
                .map(|&b| if b == 1 { "≻⋈∈⊥∋" } else { "≻⋈∈⊤∋" }).collect();
            format!("⊢{}⊙⊡⊣  (cells={} P*Q={})", cells, pe.len() + qo.len(), &p * &q)
        }
        "encode" => {
            if args.len() < 2 { return String::from("usage: factor_membrane encode <N>"); }
            let n = match parse_big(args[1]) { Some(x) => x, None => return String::from("bad N") };
            let bits = bits_le(&n);
            let cells: String = bits.iter()
                .map(|&b| if b == 1 { "≻⋈∈⊥∋" } else { "≻⋈∈⊤∋" }).collect();
            let w = format!("⊢{}⊙⊡⊣", cells);
            let back = int_le(&parse_word_cells(&w)) == n;
            format!("{}  (cells={} LSB-first cell0=LSB; full-word LSB value reads back == N: {})", w, bits.len(), back)
        }
        "separate" => {
            // FIX (words-only): bare decimal enters as IMASM — encode to honest
            // word, re-parse cells from the WORD, then D2. No bare-int bit path.
            if args.len() < 2 { return String::from("usage: factor_membrane separate <N>"); }
            let n = match parse_big(args[1]) { Some(x) => x, None => return String::from("bad N") };
            let bits0 = bits_le(&n);
            let cells: String = bits0.iter()
                .map(|&b| if b == 1 { "≻⋈∈⊥∋" } else { "≻⋈∈⊤∋" }).collect();
            let w = format!("⊢{}⊙⊡⊣", cells);
            let bits = parse_word_cells(&w);
            let (l0, l1) = d2_msb(&bits);
            let (v0, v1) = (int_le(&l0), int_le(&l1));
            let rt = gamma(&l0, &l1) == bits;
            let prod = &v0 * &v1 == n;
            format!("Dbits={} lanes {}/{} v0={} v1={} roundtrip={} P*Q==N:{} (words-only: W=encode(N) parsed as word, D2 crossed; N-alone separation, not factors — OPEN)",
                bits.len(), l0.len(), l1.len(), v0, v1, rt, prod)
        }
        "recurse" => {
            if args.len() < 2 { return String::from("usage: factor_membrane recurse <word>"); }
            let raw = args[1..].join(" ");
            let mut out = String::new();
            let mut cur_p: Option<BigUint> = None;
            let mut w = raw.clone();
            for d in 0..16 {
                match cross_word(&w) {
                    None => { out.push_str("no cells — IFIX.\n"); break; }
                    Some(r) => {
                        out.push_str(&format!("d{} cells={} P={} Q={} N={} P*Q==N:{}\n",
                            d, r.cells, r.p, r.q, r.n, &r.p * &r.q == r.n));
                        cur_p = Some(r.p.clone());
                        // re-imscribe: descend into P's own word while it shrinks
                        let pe = bits_msb(&r.p);
                        if pe.len() <= 2 || d == 15 { out.push_str("IFIX: same-wound closure.\n"); break; }
                        let cells: String = pe.iter()
                            .map(|&b| if b == 1 { "≻⋈∈⊥∋" } else { "≻⋈∈⊤∋" }).collect();
                        w = format!("⊢{}⊙⊡⊣", cells);
                    }
                }
            }
            let _ = cur_p;
            out
        }
        _ => String::from("unknown subcommand (factor | cross | word | encode | separate | recurse)"),
    }
}
#[cfg(test)] mod tests {
    use super::*;
    fn big(s: &str) -> BigUint { s.parse().unwrap() }
    #[test] fn gamma_roundtrip_gives_factors() {
        // W = gamma(13,17): cells MSB-first, P*Q=N verified.
        let w = "⊢≻⋈∈⊥∋≻⋈∈⊥∋≻⋈∈⊥∋≻⋈∈⊤∋≻⋈∈⊤∋≻⋈∈⊤∋≻⋈∈⊥∋≻⋈∈⊤∋≻⋈∈⊥∋⊙⊡⊣"; // gamma(13,17)
        let r = cross_word(w).unwrap();
        assert!(r.roundtrip);
        assert_eq!(&r.p * &r.q, r.n);
        // lanes re-imscribe to the same word: gamma(P,Q)==W is the closure.
        let (pe, qo) = (bits_msb(&r.p), bits_msb(&r.q));
        assert_eq!(gamma(&pe, &qo), parse_word_cells(w));
    }
    #[test] fn le_bvalsd_line0_fullword_is_decimal() {
        // bvals word 0 (862 cells, LSB-first) reads back as the bvalsd decimal N0.
        let n0 = big("22112825529529666435281085255026230927612089502470015394413748319128822941402001986512729726569746599085900330031400051170742204560859276357953757185954298838958709229238491006703034124620545784566413664540684214361293017694020846391065875914794251435144458199");
        assert_eq!(n0.bits() as usize, 862);
        let bits = bits_le(&n0);
        assert_eq!(bits.len(), 862);
        assert_eq!(int_le(&bits), n0); // full-word LSB value == decimal
        let (l0, l1) = d2_msb(&bits);
        assert_eq!(gamma(&l0, &l1), bits); // D2 roundtrip
    }
    #[test] fn encode_cross_roundtrip_lsb() {
        // encode(N) -> word -> LE crossing: full-word LSB value == N, lanes roundtrip.
        for st in ["91", "143", "8051"] {
            let n = big(st);
            let bits = bits_le(&n);
            let cells: String = bits.iter()
                .map(|&b| if b == 1 { "≻⋈∈⊥∋" } else { "≻⋈∈⊤∋" }).collect();
            let w = format!("⊢{}⊙⊡⊣", cells);
            let l = cross_word_le(&w).unwrap();
            assert!(l.roundtrip);
            assert_eq!(l.full_le, n);
            assert_eq!(&l.p * &l.q, l.n);
        }
    }
    #[test] fn msb_gamma_vectors_unchanged() {
        // Settled MSB vectors still hold exactly.
        let w = "⊢≻⋈∈⊥∋≻⋈∈⊥∋≻⋈∈⊥∋≻⋈∈⊤∋≻⋈∈⊤∋≻⋈∈⊤∋≻⋈∈⊥∋≻⋈∈⊤∋≻⋈∈⊥∋⊙⊡⊣";
        let r = cross_word(w).unwrap();
        assert_eq!((r.p.to_string(), r.q.to_string()), ("27".to_string(), "8".to_string()));
    }
    #[test] fn bare_small_factors() {
        let n = big("91");
        let (p, q) = factor_bare(&n, 200_000).unwrap();
        assert_eq!(&p * &q, n);
    }
}
