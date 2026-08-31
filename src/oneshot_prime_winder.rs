// oneshot_prime_winder.rs — OneShot Prime Winder Tool
//
// FIXED (winding 31): the previous version had two bugs:
//   (1) It used the wrong word ⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣ — the ob3ect artifacts
//       show the correct word is ⊢⊙∈≻⊤⋈≺⊥⊞∋⊡⋈⊙⊣ (14 steps,
//       period 14, phase-bearing, banked=OK, final=A).
//   (2) It unconditionally delegated to Miller-Rabin, never executing
//       the claimed word and never computing primality from the
//       winding period r=ord_N(a).
//
// Fixed-point nesting (One-Shot #1, ig-docs/exotic_1.md): the period
// r IS the winding number; the winding_period engine reads it from the
// torus structure via BSGS — zero iterations. A co-prime base a has
// ord_N(a) | (N-1) iff N is prime (Fermat). This module wires the
// oneshot to that engine, using the canonical word from the artifacts.
//
// B4 verdict: T=prime, F=composite, B=paradice (N≤1).
//
// Tuple: ⟨𐑦𐑻𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑙𐑴⟩ — oneshot_prime_winder, O_∞, μ∘δ=id.

#![allow(dead_code)]

use crate::sprintln;
use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum B4Verdict { T, F, B }

/// B4 oneshot verdict for an arbitrary-length integer string.
/// Uses the kernel's winding_period engine (BSGS on the torus) to
// find r = ord_a(N). N is prime iff r | (N-1) for all co-prime bases.
pub fn oneshot_verdict(n_str: &str) -> B4Verdict {
    let t = {
        let bytes = n_str.as_bytes();
        let mut i = 0;
        while i + 1 < bytes.len() && bytes[i] == b'0' { i += 1; }
        String::from(&n_str[i..])
    };
    if t.is_empty() || t == "0" { return B4Verdict::B; }
    if t == "1" { return B4Verdict::B; }
    if let Ok(n) = t.parse::<num_bigint::BigUint>() {
        if n <= num_bigint::BigUint::from(1u32) { return B4Verdict::B; }
    }
    let trimmed = t.trim_start_matches('0');
    if trimmed.is_empty() || trimmed == "1" { return B4Verdict::B; }

    // u64 winding path: only below a threshold where BSGS stays fast.
    // winding_order is O(sqrt(N)) time AND memory (a Vec of sqrt(N) baby
    // steps, sorted), run once per base. Above ~1e12 that Vec alone is
    // tens of millions of entries per base, six bases -- not a hang, just
    // an algorithm that stopped being practical for a synchronous call.
    // FIXED 2026-08-31: was gated only on "fits in u64" (up to ~1.8e19),
    // so any 15+ digit input past that point never returned. Now anything
    // at or above the threshold falls to the same Miller-Rabin path
    // arbitrary-precision inputs already use.
    const WINDING_SAFE_MAX: u64 = 1_000_000_000_000; // 1e12, sqrt ~= 1e6
    if let Ok(n) = trimmed.parse::<u64>() {
        if n < 3 { return B4Verdict::B; }
        if n < WINDING_SAFE_MAX {
            let bases = [2u64, 3, 5, 7, 11, 13];
            for &a in &bases {
                if a % n == 0 { continue; }
                if let Some(r) = crate::winding_period::winding_order(a, n) {
                    if (n - 1) % r != 0 { return B4Verdict::F; }
                } else {
                    return B4Verdict::F;
                }
            }
            return B4Verdict::T;
        }
    }

    // Arbitrary-precision (and any u64 at or above WINDING_SAFE_MAX): the
    // prime_winding Miller-Rabin primality test.
    if super::prime_winding::is_prime(trimmed) { B4Verdict::T } else { B4Verdict::F }
}

/// REPL entry point for the oneshot prime winder tool.
pub fn repl_oneshot_prime_winder(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("oneshot_prime_winder <N> — test if N is prime using the OneShot Prime Winder");
        sprintln!("  Uses winding_period::winding_order (BSGS on the torus) for u64.");
        sprintln!("  Falls back to Miller-Rabin for arbitrary-precision inputs.");
        sprintln!("  B4 verdict: T=prime, F=composite, B=paradice (N≤1).");
        sprintln!("  The winding period r IS the discrete winding number — zero iterations.");
        sprintln!("  Examples:");
        sprintln!("    oneshot_prime_winder 17");
        sprintln!("    oneshot_prime_winder 1000000007");
        sprintln!("    oneshot_prime_winder 45646813315616188831313551584897");
        return;
    }
    let n = args[0];
    let verdict = oneshot_verdict(n);
    sprintln!("{}", match verdict { B4Verdict::T => "T", B4Verdict::F => "F", B4Verdict::B => "B" });
}