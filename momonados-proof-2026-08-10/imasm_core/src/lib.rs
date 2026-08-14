#![cfg_attr(not(feature = "std"), no_std)]

//! imasm_core — the ONE kernel of tokens, registers, and machines.
//!
//! Every face lives here and nowhere else: the classic 12-opcode Token, the
//! SIXTEEN_3 trilattice extension (Token16_3, Reg16_3, the register machine,
//! the tri-ancestral verdict), and the glyph faces of all of them. Consumers
//! (ask_native, vita_native, mOMonadOS) LINK this crate and derive their
//! alphabets from it — a face correction here propagates by recompilation,
//! and the drift class of bugs (a hand-copied vocabulary going stale) becomes
//! a compile error instead of a data sweep.

extern crate alloc;

pub mod check;
pub mod classic;
pub mod imasm16_3;

/// The sixteen SIXTEEN_3 register states in canonical vocabulary order
/// (N, A, then T-F-t-f combination order): the order the model alphabets use.
pub fn state_order() -> [imasm16_3::Reg16_3; 16] {
    ["N", "A", "T", "F", "t", "f", "TF", "Tt", "Tf", "Ft", "Ff", "tf",
     "TFt", "TFf", "Ttf", "Ftf"]
        .map(|n| imasm16_3::Reg16_3::from_name(n).expect("canonical state name"))
}
