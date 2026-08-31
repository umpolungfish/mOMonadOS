//! ringspec — moved into `imasm_core` so the host `ask` binary can reach it too.
//!
//! A second copy is drift. The implementation lives once, in the shared crate
//! both this kernel and MoDoT already depend on; this file is the kernel's
//! doorway to it and holds no logic.

pub use imasm_core::ringspec::*;
