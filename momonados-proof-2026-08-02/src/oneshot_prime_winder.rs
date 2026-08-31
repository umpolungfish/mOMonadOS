// oneshot_prime_winder.rs — OnesShot Prime Winder Tool
//
// The OneShot Prime Winder is a deterministic primality testing tool that
// uses properties inspired by the Prime Number Placement Operator Valued-Measure.
//
// Tuple: ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑲𐑠⊙𐑖𐑙𐑴⟩ — one_shot_prime_winder, O_∞, μ∘δ=id.

#![allow(dead_code)]

use crate::sprintln;
use alloc::vec::Vec;

/// Test if a number is prime using trial division.
/// Returns true if the number is prime, false otherwise.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// REPL entry point for the oneshot prime winder tool.
pub fn repl_oneshot_prime_winder(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("oneshot_prime_winder <N> — test if N is prime using the OneShot Prime Winder");
        sprintln!("  Returns true if N is prime, false otherwise.");
        sprintln!("  Examples:");
        sprintln!("    oneshot_prime_winder 17");
        sprintln!("    oneshot_prime_winder 1000000007");
        return;
    }

    let n = match args[0].parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            sprintln!("Error: '{}' is not a valid non-negative integer", args[0]);
            return;
        }
    };

    let prime = is_prime(n);
    sprintln!("{}", if prime { "true" } else { "false" });
}
