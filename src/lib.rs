//! Host-testable surface of the kernel.
//!
//! The kernel is a bare-metal binary, which is why `braid_protocol`'s dual pair
//! was written correctly and then never checked: there was nowhere to run it.
//! A dual that has never been closed is a claim, not a verification, and the
//! Frobenius condition is the one thing this module exists to make checkable.
//!
//! Only the portable modules are carried here. Nothing that touches serial,
//! interrupts, the entry point or the halt is included. The kernel declares its
//! own module tree in main.rs and does not read this file, but cargo does build
//! this target alongside the binary, which is why it carries the same no_std
//! condition below.
//!
//! Author: Lando⊗⊙perator

// The bare-metal build compiles this target too, so it carries main.rs's own
// no_std condition. Without it the kernel build tries to link std for the lib
// and fails on the whole prelude.
#![cfg_attr(not(feature = "hosted"), no_std)]
#![allow(uncommon_codepoints)]
#![allow(dead_code)]

extern crate alloc;

pub mod tokens;
pub mod braid_protocol;
pub mod vox;
pub mod period_finding_ecdlp;

// The gate runs on a host; there is no test harness on bare metal.
#[cfg(all(test, feature = "hosted"))]
mod braid_frobenius_tests;
