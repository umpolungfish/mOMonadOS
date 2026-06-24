// cr3echrz — Mathematical theorem operationalization engine
// Ported from Python cr3echrz to Rust for mOMonadOS
// Author: Lando⊗⊙perator
//
// Modules:
//   shared      — opcode registry, grammar mappings, canonical sequences, domains
//   p3theorem   — 7-theorem unified engine (Collatz, Goldbach, Three-Body, Burnside,
//                  Erdős–Straus, Inverse Galois, Baum–Connes)
//   p4rakernel  — 6-module Belnap+Frobenius p4rakernel engine (Burnside, Connes,
//                  Erdős–Straus, Goldbach, Landau, Three-Body)
//
// Integrates with mOMonadOS: Belnap FOUR (belnap.rs), Frobenius verifier
// (frob_verify.rs), IMASM/parasm VM (parasm.rs), crystal FS (crystal.rs),
// and the menu system (menu.rs).

pub mod shared;
pub mod p3theorem;
pub mod p4rakernel;
