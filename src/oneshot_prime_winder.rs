// oneshot_prime_winder.rs — OneShot Prime Winder Tool
//
// Fixed-point nesting (One-Shot #1, ig-docs/exotic_1.md): the period r IS
// the winding number. N is prime iff the order r = ord_a(N) of a coprime
// base a divides N-1 (Fermat, order form), checked across several bases.
//
// This used to special-case u64 N through winding_period::winding_order
// directly and fall back to Miller-Rabin above a 1e12 threshold -- two
// primality tests living side by side, one winding-native and one not.
// prime_winding::is_prime is now itself winding-native at every size (the
// BigUint BSGS engine subsumes the u64 fast path), so this delegates to
// it uniformly and carries no primality test of its own.
//
// B4 verdict: T=prime, F=composite, B=paradice (N≤1), N=undetermined
// (the winding-order search did not close within its step budget).
//
// Tuple: ⟨𐑦𐑻𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑙𐑴⟩ — oneshot_prime_winder, O_∞, μ∘δ=id.

#![allow(dead_code)]

use crate::sprintln;
use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum B4Verdict { T, F, B, N }

/// B4 oneshot verdict for an arbitrary-length integer string, via
/// prime_winding's winding-order engine.
pub fn oneshot_verdict(n_str: &str) -> B4Verdict {
    let t = {
        let bytes = n_str.as_bytes();
        let mut i = 0;
        while i + 1 < bytes.len() && bytes[i] == b'0' { i += 1; }
        String::from(&n_str[i..])
    };
    if t.is_empty() || t == "0" || t == "1" { return B4Verdict::B; }

    match super::prime_winding::is_prime(&t) {
        super::prime_winding::PrimeVerdict::Prime => B4Verdict::T,
        super::prime_winding::PrimeVerdict::Composite => B4Verdict::F,
        super::prime_winding::PrimeVerdict::Undetermined => B4Verdict::N,
    }
}

/// REPL entry point for the oneshot prime winder tool.
pub fn repl_oneshot_prime_winder(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("oneshot_prime_winder <N> — test if N is prime using the OneShot Prime Winder");
        sprintln!("  Uses prime_winding's winding-order engine (BSGS on the torus) at every size.");
        sprintln!("  B4 verdict: T=prime, F=composite, B=paradice (N≤1),");
        sprintln!("  N=undetermined (order search exceeded its step budget).");
        sprintln!("  The winding period r IS the discrete winding number — zero iterations.");
        sprintln!("  Examples:");
        sprintln!("    oneshot_prime_winder 17");
        sprintln!("    oneshot_prime_winder 1000000007");
        sprintln!("    oneshot_prime_winder 45646813315616188831313551584897");
        return;
    }
    let n = args[0];
    let verdict = oneshot_verdict(n);
    sprintln!("{}", match verdict {
        B4Verdict::T => "T", B4Verdict::F => "F", B4Verdict::B => "B", B4Verdict::N => "N",
    });
}
