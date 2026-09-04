//! Re-export of the Vox native decoder from the standalone `vox` crate.
//!
//! `repl.rs` uses `vox_decode::Image` and `vox_decode::walk`; both come from the
//! standalone crate, linked under the `hosted` feature (the target the ordinal
//! faithfulness guard runs on).
//!
//! One name, one definition: everything here is the standalone crate's, named
//! rather than reimplemented, so a second decoder cannot drift from the first.

// Every consumer of these names is behind the `hosted` feature — the `vox`
// command in repl.rs — so on the bare target the re-export is correct and
// unused at once. Deleting it to quiet the bare build would break the hosted
// one, which is the build the ordinal guard runs on.
#[allow(unused_imports)]
pub use vox_core::vox_decode::*;
