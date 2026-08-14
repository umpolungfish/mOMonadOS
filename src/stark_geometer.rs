// ─── stark_geometer.rs ─────────────────────────────────────────────────
// SIC-POVM Stark arithmetic (spec: stark-geometer).
//
// Given d, form the Appleby discriminant m_d = (d-3)(d+1), the real quadratic
// field F = Q(sqrt m_d), its fundamental-ish unit ε = ((d-1)+sqrt m_d)/2, and
// the ramified primes (the primes dividing m_d). This is the scaffold the
// d=2048 campaign climbs.
//
// Honest scope: the EXACT fiducial as a radical is known only for a few small d
// (4, 8, 12). For larger d the S-unit exponent search over the ray-class tower
// is the open frontier — this tool reports the arithmetic it stands on and
// says where the exact extraction stops, rather than printing a fake fiducial.
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut fs = Vec::new();
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 { n /= p; e += 1; }
            fs.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 { fs.push((n, 1)); }
    fs
}

pub fn stark_geometer_main(args: &[&str]) -> String {
    let flat: Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    let d: u64 = match flat.first().and_then(|s| s.parse().ok()) {
        Some(d) if d >= 4 => d,
        _ => {
            return "stark-geometer <d>   (d >= 4)\n\n\
                    Reports the SIC Stark arithmetic for dimension d: the Appleby\n\
                    discriminant m_d=(d-3)(d+1), the unit ε=((d-1)+√m_d)/2, and\n\
                    the ramified primes. Exact radical fiducials are known for\n\
                    d in {4,8,12}; larger d name the open S-unit frontier.\n\n\
                    Try:  stark-geometer 12   or   stark-geometer 2048\n".to_string();
        }
    };
    let md = (d - 3) * (d + 1);
    let r = isqrt(md);
    let is_square = r * r == md;
    let fs = factor(md);

    let mut out = String::from("STARK-GEOMETER\n==============\n\n");
    out.push_str(&format!("dimension d:        {}\n", d));
    out.push_str(&format!("m_d = (d-3)(d+1):   {}\n", md));
    if is_square {
        out.push_str("F = Q(√m_d):        DEGENERATE — m_d is a perfect square, no real field\n");
        return out;
    }
    out.push_str(&format!("F = Q(√m_d):        real quadratic, √m_d ≈ {}\n", r));
    out.push_str(&format!("unit ε:             ({} + √{})/2\n", d - 1, md));
    let mut prstr = String::new();
    for (i, (p, e)) in fs.iter().enumerate() {
        if i > 0 { prstr.push_str(", "); }
        prstr.push_str(&format!("{}", p));
        if *e > 1 { prstr.push_str(&format!("^{}", e)); }
    }
    out.push_str(&format!("ramified primes:    {}\n", prstr));
    out.push_str(&format!("  (m_d factors, and 1/(d+1) = 1/{} is the SIC normalization)\n", d + 1));

    out.push_str("\nexact fiducial:     ");
    match d {
        4 => out.push_str("known (Q(√5), radical)\n"),
        8 => out.push_str("known (radical over F)\n"),
        12 => out.push_str("known — K16 ring, embedding is a THEOREM (crystal_forces_d12_sic)\n"),
        _ => out.push_str(
            "OPEN — the S-unit exponent search over the ray-class tower of F is\n\
             \x20                   the frontier. This tool gives the arithmetic it climbs, not a\n\
             \x20                   forged number. (For d=2048 the norm sieve separates the\n\
             \x20                   degenerate exponent vectors; see d2048_sieve.)\n",
        ),
    }
    out
}
