//! Re-export of the Vox auditor from the standalone `vox` crate (aliased
//! `vox_core`). One source of truth; a fix there propagates here by recompile.
pub use vox_core::vox::*;
