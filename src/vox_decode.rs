//! Re-export of the Vox native decoder from the standalone `vox` crate.
//!
//! The decoder used to be carried here as a copy. c544d09 removed the copy in
//! favour of linking the standalone crate, but left this file holding only the
//! line above, so `repl.rs` asked for `vox_decode::Image` and `vox_decode::walk`
//! against an empty module. The bare target never noticed, because the `vox`
//! command compiles only under the `hosted` feature — which is also the target
//! the ordinal faithfulness guard runs on.
//!
//! One name, one definition: everything here is the standalone crate's, named
//! rather than reimplemented, so a second decoder cannot drift from the first.

// Every consumer of these names is behind the `hosted` feature — the `vox`
// command in repl.rs — so on the bare target the re-export is correct and
// unused at once. Deleting it to quiet the bare build would break the hosted
// one, which is the build the ordinal guard runs on.
#[allow(unused_imports)]
pub use vox_core::vox_decode::*;
