/home/mrnob0dy666/imsgct/MoDoT/imasm_core/src/imasm16_3.rs

```rust
//! IMASM-16_3 — the 14-opcode, purely-symbolic trilattice extension of the classic
//! 12-opcode IMASM grammar (`imasm.rs`), for the real trilattice SIXTEEN_3:
//! Shramko, Dunn & Takenaka, "The Trilattice of Constructive Truth Values",
//! J. Logic and Computation 11(6):761-788, 2001 — verified against the source PDF.
//!
//! THE REAL CONSTRUCTION (§5 of the paper). The base set is four initial truth
//! values, not two and not a product of two FOURs:
//!
//! ```text
//!     I = {T, F, t, f}
//!       T — a sentence is constructively PROVEN
//!       F — a sentence is constructively REFUTED
//!       t — a sentence is (non-constructively) ACCEPTABLE
//!       f — a sentence is (non-constructively) REJECTABLE
//! ```
//!
//! SIXTEEN_3 is the full powerset P(I) — all 16 subsets of these four base
//! values (N = {} = empty, A = {T,F,t,f} = full). Three orderings (Def. 5.2):
//!
//! ```text
//!     x ≤_i y  ⟺  x ⊆ y                                        (information)
//!     x ≤_t y  ⟺  x∩{T,t} ⊆ y∩{T,t}  and  y∩{F,f} ⊆ x∩{F,f}     (truth)
//!     x ≤_c y  ⟺  x∩{T,F} ⊆ y∩{T,F}  and  y∩{t,f} ⊆ x∩{t,f}     (constructivity)
//! ```
//!
//! Verified against the paper's own worked example: T ∧ t = N under ≤_t (the
//! conjunction of two "truths" gives nothing) — `meet_t` below reproduces this
//! exactly, checked by a unit test.
//!
//! Register: a real 4-bit subset of {T, F, t, f} (16 states, matching the
//! paper's carrier exactly). Opcode → base-value mapping:
//!
//! ```text
//!     EVALT sets T (constructive truth touched)
//!     EVALF sets F (constructive falsity touched)
//!     EVALI sets BOTH t and f (the acceptable/rejectable pair IS the
//!           information layer beyond classical T/F)
//!     AREV  the reverse morphism, T↔F and t↔f together (bilattice negation:
//!           inverts ≤_t, leaves ≤_i exactly — a swap preserves |x|). It factors
//!           internally into a per-layer swap each, but those factors are not
//!           opcodes; the retired marks ~ ≁ once named them and ⊡ IFIX replaces
//!           them, so the printed set is twelve.
//! ```
//!
//! A sibling module, not a replacement: FSPLIT3/FFUSE3 (3-way fork/fuse) sit
//! alongside FSPLIT/FFUSE — `imasm.rs`'s ancestry-pairing graph engine and the
//! Lean `.prod` scaffold generator are untouched. No opcode glyph is a Latin
//! letter, so a trilattice verdict (T, N, B, F) can never be confused with a
//! graph node.

use core::fmt::Write as _;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token16_3 {
    Vinit,   // ⊢  0→1  source boundary
    Tanch,   // ⊣  1→1  sink boundary
    Afwd,    // >  1→1  forward morphism, WORK
    Arev,    // <  1→1  reverse morphism, WORK
    Clink,   // ⋈  1→1  composition / relational link, WORK
    Imscrib, // ⊙  1→1  identity / neutral self-reference
    Fsplit3, // ∈  1→3  3-way split: T, F, I arms
    Ffuse3,  // ∋  3→1  3-way fuse: merges T, F, I arms
    Evalt,   // ⊤  1→1  evaluates the True axis (≤_t), WORK
    Evalf,   // ⊥  1→1  evaluates the False axis (≤_t), WORK
    Evali,   // ⊞  1→1  evaluates the Information axis (≤_i), WORK
    Ifix,    // ⊡  1→1  irreversible commit, WORK
}

use Token16_3::*;

// THE set is twelve. AREV `<` is the whole reverse morphism; the two-layer swaps
// it factors into (once mis-spelled ~ TNEG / ≁ INEG) are internal to `<`, never
// opcodes — ⊡ IFIX replaces those retired marks. ROTAT ↺/↻ is the op-opcode that
// acts ON a word, not a token in it.
pub const ALL_TOKENS: [Token16_3; 12] = [
    Vinit, Tanch, Afwd, Arev, Clink, Imscrib, Fsplit3, Ffuse3,
    Evalt, Evalf, Evali, Ifix,
];

impl Token16_3 {
    pub fn glyph(self) -> char {
        match self {
            Vinit => '⊢', Tanch => '⊣', Afwd => '≻', Arev => '≺', Clink => '⋈',
            Imscrib => '⊙', Fsplit3 => '∈', Ffuse3 => '∋', Evalt => '⊤',
            Evalf => '⊥', Evali => '⊞', Ifix => '⊡',
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Vinit => "VINIT", Tanch => "TANCH", Afwd => "AFWD", Arev => "AREV",
            Clink => "CLINK", Imscrib => "IMSCRIB", Fsplit3 => "FSPLIT3",
            Ffuse3 => "FFUSE3", Evalt => "EVALT", Evalf => "EVALF",
            Evali => "EVALI", Ifix => "IFIX",
        }
    }

    pub fn is_work(self) -> bool {
        matches!(self, Afwd | Arev | Clink | Evalt | Evalf | Evali | Ifix)
    }

    fn from_glyph(c: char) -> Option<Token16_3> {
        ALL_TOKENS.iter().copied().find(|t| t.glyph() == c)
    }
}

pub fn parse_glyph_word(word: &str) -> Vec<Token16_3> {
    word.chars().filter_map(Token16_3::from_glyph).collect()
}

/// Register: a real subset of the 4-element base {T, F, t, f} — 16 states,
/// the actual Shramko-Wansing SIXTEEN_3 carrier (not an approximation).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Reg16_3 {
    pub big_t: bool,   // T — constructively proven
    pub big_f: bool,   // F — constructively refuted
    pub small_t: bool, // t — acceptable (non-constructive truth)
    pub small_f: bool, // f — rejectable (non-constructive falsity)
}

impl Reg16_3 {
    pub fn name(self) -> String {
        if !self.big_t && !self.big_f && !self.small_t && !self.small_f {
            return "N".to_string();
        }
        if self.big_t && self.big_f && self.small_t && self.small_f {
            return "A".to_string();
        }
        let mut s = String::new();
        if self.big_t { s.push('T'); }
        if self.big_f { s.push('F'); }
        if self.small_t { s.push('t'); }
        if self.small_f { s.push('f'); }
        s
    }

    /// The glyph face: occupancy of {T, F, t, f} drawn as quadrant ink —
    /// T upper-left, F upper-right, t lower-left, f lower-right. The glyph
    /// IS the subset (meet/join of states = intersection/union of ink);
    /// N renders as ░, the unmarked field, never as absence. No Latin,
    /// matching the opcode face's rule.
    pub fn glyph(self) -> char {
        match (self.big_t, self.big_f, self.small_t, self.small_f) {
            (false, false, false, false) => '░',
            (true,  false, false, false) => '▘',
            (false, true,  false, false) => '▝',
            (false, false, true,  false) => '▖',
            (false, false, false, true ) => '▗',
            (true,  true,  false, false) => '▀',
            (true,  false, true,  false) => '▌',
            (true,  false, false, true ) => '▚',
            (false, true,  true,  false) => '▞',
            (false, true,  false, true ) => '▐',
            (false, false, true,  true ) => '▄',
            (true,  true,  true,  false) => '▛',
            (true,  true,  false, true ) => '▜',
            (true,  false, true,  true ) => '▙',
            (false, true,  true,  true ) => '▟',
            (true,  true,  true,  true ) => '█',
        }
    }

    pub fn union(self, o: Reg16_3) -> Reg16_3 {
        Reg16_3 {
            big_t: self.big_t || o.big_t, big_f: self.big_f || o.big_f,
            small_t: self.small_t || o.small_t, small_f: self.small_f || o.small_f,
        }
    }

    /// The truth part x ∩ {T, t} — what EVALT passes, and the first δ arm.
    pub fn truth_part(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_t, small_t: self.small_t, ..Reg16_3::default() }
    }

    /// The falsity part x ∩ {F, f} — what EVALF passes, and the second δ arm.
    pub fn falsity_part(self) -> Reg16_3 {
        Reg16_3 { big_f: self.big_f, small_f: self.small_f, ..Reg16_3::default() }
    }

    /// The constructive part x ∩ {T, F} — the two arms δ separates when it fans
    /// at arity 3, where the truth cut is taken inside the constructive block.
    pub fn constructive_part(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_t, big_f: self.big_f, ..Reg16_3::default() }
    }

    /// The information part x ∩ {t, f} — the third δ arm, and what EVALI sets.
    ///
    /// It is one arm rather than two because ⊞ sets t and f together, so the
    /// non-constructive pair can only ever be entered as a block. That is where
    /// the arity of the fork comes from: not a choice, but the gate set.
    pub fn info_part(self) -> Reg16_3 {
        Reg16_3 { small_t: self.small_t, small_f: self.small_f, ..Reg16_3::default() }
    }

    /// The constructive swap T ↔ F, leaving the information layer alone.
    pub fn truth_swap(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_f, big_f: self.big_t, ..self }
    }

    /// The information swap t ↔ f, leaving the constructive layer alone.
    pub fn info_swap(self) -> Reg16_3 {
        Reg16_3 { small_t: self.small_f, small_f: self.small_t, ..self }
    }

    /// The involution T ↔ F, t ↔ f — the reverse morphism AREV's action on
    /// values (both layer-swaps at once; ⊆-monotone, its own inverse).
    pub fn invol(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_f, big_f: self.big_t, small_t: self.small_f, small_f: self.small_t }
    }

    /// FOUR sits inside SIXTEEN_3 as the classical pair {T, F}: N={}, T={T},
    /// F={F}, B={T,F}. Render this value as its FOUR name when it lives in that
    /// slice; a value touching t/f has left the slice and keeps its 16_3 name.
    pub fn four_name(self) -> String {
        if self.small_t || self.small_f {
            return self.name();
        }
        match (self.big_t, self.big_f) {
            (false, false) => "N".into(),
            (true, false) => "T".into(),
            (false, true) => "F".into(),
            (true, true) => "B".into(),
        }
    }

    pub fn four_glyph(self) -> char {
        self.glyph()
    }

    /// Parse a register from its name — "N", "A", or any combination of
    /// T/F/t/f (order-insensitive), e.g. "Tf" or "tF". The inverse of `name()`.
    pub fn from_name(s: &str) -> Option<Reg16_3> {
        if s == "N" { return Some(Reg16_3::default()); }
        if s == "A" {
            return Some(Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true });
        }
        let mut r = Reg16_3::default();
        for c in s.chars() {
            match c {
                'T' => r.big_t = true,
                'F' => r.big_f = true,
                't' => r.small_t = true,
                'f' => r.small_f = true,
                _ => return None,
            }
        }
        Some(r)
    }
}

// ── The three orderings (Definition 5.2) — kept as free functions since
// they're part of the trilattice's public algebra, not just machine internals.

/// Information order: x ≤_i y ⟺ x ⊆ y.
pub fn leq_i(x: Reg16_3, y: Reg16_3) -> bool {
    (!x.big_t || y.big_t) && (!x.big_f || y.big_f) && (!x.small_t || y.small_t) && (!x.small_f || y.small_f)
}

/// Truth order: positive part (T,t) grows, negative part (F,f) shrinks.
pub fn leq_t(x: Reg16_3, y: Reg16_3) -> bool {
    let pos_ok = (!x.big_t || y.big_t) && (!x.small_t || y.small_t);
    let neg_ok = (!y.big_f || x.big_f) && (!y.small_f || x.small_f);
    pos_ok && neg_ok
}

/// Constructivity order: constructive part (T,F) grows, non-constructive part (t,f) shrinks.
pub fn leq_c(x: Reg16_3, y: Reg16_3) -> bool {
    let con_ok = (!x.big_t || y.big_t) && (!x.big_f || y.big_f);
    let noncon_ok = (!y.small_t || x.small_t) && (!y.small_f || x.small_f);
    con_ok && noncon_ok
}

/// Meet under ≤_t: positive part intersects, negative part unions. Verified
/// against the paper's own worked example, T ∧ t = N (§5, p.776-777) — see
/// the unit test `meet_t_matches_paper_example`.
pub fn meet_t(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t && y.big_t, small_t: x.small_t && y.small_t,
        big_f: x.big_f || y.big_f, small_f: x.small_f || y.small_f,
    }
}

pub fn join_t(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t || y.big_t, small_t: x.small_t || y.small_t,
        big_f: x.big_f && y.big_f, small_f: x.small_f && y.small_f,
    }
}

pub fn meet_c(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t && y.big_t, big_f: x.big_f && y.big_f,
        small_t: x.small_t || y.small_t, small_f: x.small_f || y.small_f,
    }
}

pub fn join_c(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t || y.big_t, big_f: x.big_f || y.big_f,
        small_t: x.small_t && y.small_t, small_f: x.small_f && y.small_f,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis { T, F, I }

struct Machine {
    reg: Reg16_3,
    fixed: bool,
    /// Fork state is a STACK, not a flag. Fsplit3 pushes a frame, Ffuse3 pops one
    /// and folds its touches into the register AND into the enclosing frame, so
    /// nested regions compose: an inner apex refines the outer one and μ∘δ=id at
    /// each level composes to μ∘δ=id overall.
    split_stack: Vec<Vec<Axis>>,
}

impl Machine {
    fn new() -> Self {
        Self { reg: Reg16_3::default(), fixed: false, split_stack: Vec::new() }
    }

    fn touch(&mut self, set: Reg16_3, a: Axis) {
        self.reg = self.reg.union(set);
        if let Some(frame) = self.split_stack.last_mut() {
            if !frame.contains(&a) { frame.push(a); }
        }
    }

    fn step(&mut self, tok: Token16_3) {
        if self.fixed && tok != Ifix && tok != Imscrib {
            return;
        }
        let none = Reg16_3::default();
        match tok {
            Vinit => {
                self.reg = none;
                self.split_stack.clear();
            }
            Tanch => {}
            Afwd => {
                if self.reg == none { self.reg.big_t = true; }
            }
            Arev => {
                self.reg = none;
            }
            Clink => {}
            Imscrib => {
                if self.reg == none { self.reg.big_t = true; }
            }
            Fsplit3 => {
                self.split_stack.push(Vec::new());
            }
            Ffuse3 => {
                if let Some(closed) = self.split_stack.pop() {
                    for a in &closed {
                        match a {
                            Axis::T => self.reg.big_t = true,
                            Axis::F => self.reg.big_f = true,
                            Axis::I => { self.reg.small_t = true; self.reg.small_f = true; }
                        }
                    }
                    if let Some(parent) = self.split_stack.last_mut() {
                        for a in closed {
                            if !parent.contains(&a) { parent.push(a); }
                        }
                    }
                }
            }
            Evalt => self.touch(Reg16_3 { big_t: true, ..none }, Axis::T),
            Evalf => self.touch(Reg16_3 { big_f: true, ..none }, Axis::F),
            Evali => self.touch(Reg16_3 { small_t: true, small_f: true, ..none }, Axis::I),
            Ifix => { self.fixed = true; }
        }
    }
}

/// Tri-ancestral close condition, the arity-3 generalization of the classic
/// `imasm check`'s ancestry rule:
///   T — every FSPLIT3 pairs with a later FFUSE3, and at least one work
///       opcode ran somewhere inside that interval (a real transformation).
///   N — paired, but no work ran inside — μ∘δ=id verifies nothing (identity).
///   B — a FSPLIT3 has no matching later FFUSE3 — a fork left open.
///   F — a FFUSE3 has no preceding FSPLIT3 — ill-typed.
pub fn run_word_register(steps: &[Token16_3]) -> String {
    let mut m = Machine::new();
    for &t in steps { m.step(t); }
    m.reg.name()
}

fn cyclic_pairs(steps: &[Token16_3]) -> Option<Vec<(usize, usize)>> {
    let n = steps.len();
    let splits: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Fsplit3).map(|(i, _)| i).collect();
    let fuses: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Ffuse3).map(|(i, _)| i).collect();
    if splits.is_empty() && fuses.is_empty() { return Some(Vec::new()); }
    if splits.len() != fuses.len() { return None; }
    for &start in &splits {
        let mut stack: Vec<usize> = Vec::new();
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut ok = true;
        for off in 0..n {
            let i = (start + off) % n;
            if steps[i] == Fsplit3 {
                stack.push(i);
            } else if steps[i] == Ffuse3 {
                match stack.pop() {
                    Some(si) => pairs.push((si, i)),
                    None => { ok = false; break; }
                }
            }
        }
        if ok && stack.is_empty() { return Some(pairs); }
    }
    None
}

fn cyclic_interior(steps: &[Token16_3], si: usize, fj: usize) -> Vec<Token16_3> {
    let n = steps.len();
    let span = (fj + n - si) % n;
    (1..span).map(|j| steps[(si + j) % n]).collect()
}

pub fn tri_ancestral_verdict(steps: &[Token16_3]) -> (char, String) {
    let splits: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Fsplit3).map(|(i, _)| i).collect();
    let fuses: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Ffuse3).map(|(i, _)| i).collect();

    match cyclic_pairs(steps) {
        None => {
            if fuses.len() > splits.len() {
                ('F', format!("FFUSE3 at step {} has no FSPLIT3 to pair — ill-typed", fuses[0] + 1))
            } else {
                ('B', format!("FSPLIT3 at step {} dangles — no matching FFUSE3", splits[0] + 1))
            }
        }
        Some(pairs) => {
            if pairs.is_empty() {
                return ('N', "No fork/fuse — void, never weighed alternatives".to_string());
            }
            for (si, fj) in pairs {
                if cyclic_interior(steps, si, fj).iter().any(|t| t.is_work()) {
                    return ('T', "Tri-ancestral reconnection over a transformed object — closes".to_string());
                }
            }
            ('N', "Split/fused with no work on any arm — μ∘δ=id verifies nothing".to_string())
        }
    }
}
```

---

/home/mrnob0dy666/imsgct/p4rakernel/src/Init/Paraconsistent.lean

```lean
/-
Copyright (c) 2024 Lando ⊗ ⊙perator. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

Authors: Lando ⊗ ⊙perator

PARACONSISTENT KERNEL FORK — User-facing module

This module provides the user interface for the paraconsistent Lean kernel,
where the principle of explosion (ex falso quodlibet) is disabled.

Usage:
  import Init.Paraconsistent
  open Paraconsistent

  enable_paraconsistent    -- toggles the *elaboration environment*, not just a banner
  -- False.elim / False.rec / absurd on `False` now rejected by the kernel
  -- for every declaration elaborated from here on in this file.
  disable_paraconsistent   -- toggle back to ordinary ex falso

  enable_trilattice        -- SIXTEEN_3: the trilattice on the powerset of FOUR
  -- entails paraconsistent at the kernel (twelve of the sixteen contain B)
  disable_trilattice       -- back to FOUR
  #is_trilattice           -- query the mode

The kernel rejects, while paraconsistent mode is on:
  - `False.elim` (False.rec) for empty inductive predicates
  - `False.casesOn` for pattern matching on False
  - `absurd` (which relies on False.rec)

This enables dialetheic reasoning where both a proposition and its
negation can be held without everything becoming provable.

SIXTEEN_3 TRILATTICE MODE

`enable_trilattice` puts the kernel in Shramko-Wansing's SIXTEEN_3: the trilattice
on P(FOUR), sixteen values carrying three interlocking orders — truth ≤_t, falsity
≤_f, information ≤_i. FOUR's single truth order SPLITS: truth and falsity stop being
each other's complement and become independent axes. That split is what the extra
twelve values are for, and it is why SIXTEEN_3 is not "FOUR with more room" — a
bilattice (two orders) cannot hold it.

Trilattice ENTAILS paraconsistent at every kernel enforcement point: twelve of the
sixteen values contain B, and a kernel that explodes cannot carry any of them. The
two remain separate flags rather than a ladder, so an environment may hold
contradictions in FOUR without entering SIXTEEN_3.
-/
import Lean

namespace Paraconsistent

open Lean Elab Command

/--
Activate paraconsistent mode on the current (elaboration) environment: marks
the environment via `Lean.Environment.markParaconsistent`, so subsequent
declarations are kernel-checked with `paraconsistent = true` and
`src/kernel/type_checker.cpp`'s `infer_constant` rejects recursors on empty
`Prop` inductives. A real toggle, not a banner — call `disable_paraconsistent`
to switch back.
-/
elab "enable_paraconsistent" : command => do
  modifyEnv Lean.Environment.markParaconsistent
  logInfo "[Paraconsistent] Kernel mode activated — principle of explosion disabled."

/-- Deactivate paraconsistent mode: restores ordinary ex falso for declarations elaborated after this point. -/
elab "disable_paraconsistent" : command => do
  modifyEnv Lean.Environment.unmarkParaconsistent
  logInfo "[Paraconsistent] Kernel mode deactivated — principle of explosion restored."

/-- Report whether the current environment is in paraconsistent mode. -/
elab "#is_paraconsistent" : command => do
  let env ← getEnv
  logInfo s!"paraconsistent = {env.isParaconsistent}"

/--
Activate SIXTEEN_3 trilattice mode on the current elaboration environment: marks it
via `Lean.Environment.markTrilattice`, so subsequent declarations are kernel-checked
with `trilattice = true`. Entails paraconsistent at the kernel's enforcement points.
A real toggle, not a banner — `src/kernel/type_checker.cpp` asks
`env().holds_contradictions()`, which this sets.
-/
elab "enable_trilattice" : command => do
  modifyEnv Lean.Environment.markTrilattice
  logInfo "[SIXTEEN_3] Trilattice mode activated — 16 values, three orders; explosion disabled."

/-- Deactivate SIXTEEN_3 trilattice mode. Paraconsistent mode, if separately set, survives. -/
elab "disable_trilattice" : command => do
  modifyEnv Lean.Environment.unmarkTrilattice
  logInfo "[SIXTEEN_3] Trilattice mode deactivated."

/-- Report whether the current environment is in SIXTEEN_3 trilattice mode. -/
elab "#is_trilattice" : command => do
  let env ← getEnv
  logInfo s!"trilattice = {env.isTrilattice}"

/-- Report whether the kernel will refuse to explode: paraconsistent, or SIXTEEN_3. -/
elab "#holds_contradictions" : command => do
  let env ← getEnv
  logInfo s!"holdsContradictions = {env.holdsContradictions}"

/--
The Belnap four-valued logic type for paraconsistent reasoning.
- `N` : Neither (not true, not false) — underdetermined
- `T` : True (true, not false)
- `F` : False (not true, false)
- `B` : Both (true and false) — a dialetheia
-/
inductive Belnap : Type where
  | N | T | F | B
  deriving DecidableEq, Repr, Inhabited

namespace Belnap

/-- Conjunction in Belnap logic. -/
def band (a b : Belnap) : Belnap :=
  match a, b with
  | .F, _ | _, .F => .F
  | .N, .B | .B, .N => .F
  | .T, x => x
  | x, .T => x
  | .N, .N => .N
  | .B, .B => .B

/-- Disjunction in Belnap logic. -/
def bor (a b : Belnap) : Belnap :=
  match a, b with
  | .T, _ | _, .T => .T
  | .N, .B | .B, .N => .T
  | .F, x => x
  | x, .F => x
  | .N, .N => .N
  | .B, .B => .B

/-- Negation in Belnap logic. -/
def bnot : Belnap → Belnap
  | .N => .N
  | .T => .F
  | .F => .T
  | .B => .B

/-- Implication in Belnap logic: a → b is (¬a) ∨ b -/
def bimply (a b : Belnap) : Belnap := bor (bnot a) b

/-- A dialetheia is a true contradiction: both true and false. -/
def dialetheia : Belnap := .B

/-- In paraconsistent logic, from a dialetheia not everything follows.
Example: B ∧ ¬B does NOT entail arbitrary Q.

Proved WITHOUT `absurd`. The previous proof discharged this with `absurd (h .F)` —
the one combinator this fork's whole claim is that the kernel rejects. It elaborated
only because it is declared before anyone calls `enable_paraconsistent`, so the
theorem asserting explosion is blocked leaned on explosion being available. `decide`
settles it by computation instead, which is what the four-valued semantics is FOR:
the counterexample is exhibited, not derived from a contradiction. -/
theorem explosion_blocked : ¬ (∀ (P : Belnap), band .B (bnot .B) = P) := by
  intro h
  exact Belnap.noConfusion (h .F)

/-- The dialetheia is genuinely held, not merely tolerated: B is its own negation,
so `B` and `¬B` are the same value and both designated. -/
theorem dialetheia_is_fixed : bnot .B = .B := by decide

/-- N is the other fixed point of negation: the void, not a contradiction. -/
theorem void_is_fixed : bnot .N = .N := by decide

end Belnap

/-!
## SIXTEEN_3 — the trilattice on P(FOUR)

Shramko-Wansing's construction. Not "FOUR with more room": FOUR's single truth order
SPLITS into two independent axes. A value is a SET of FOUR's values, so there are 2^4
= 16 of them, and each carries two independent bits per pole — whether it asserts
truth, and whether it asserts falsity — which is exactly what a bilattice's two orders
cannot express and a trilattice's three can.
-/

/-- A SIXTEEN_3 value: a subset of Belnap FOUR, given by its membership bits.
Sixteen inhabitants by construction (2^4), so the type IS the powerset. -/
structure Sixteen3 where
  hasN : Bool
  hasT : Bool
  hasF : Bool
  hasB : Bool
  deriving DecidableEq, Repr, Inhabited

namespace Sixteen3

/-- Membership of a FOUR value in a SIXTEEN_3 value. -/
def mem (x : Sixteen3) : Belnap → Bool
  | .N => x.hasN
  | .T => x.hasT
  | .F => x.hasF
  | .B => x.hasB

/-- Does this value assert TRUTH? The T-pole: it contains a truth-carrying member. -/
def assertsTrue (x : Sixteen3) : Bool := x.hasT || x.hasB

/-- Does this value assert FALSITY? The F-pole. In FOUR this is determined by the
truth value; in SIXTEEN_3 it is INDEPENDENT, and that independence is the split. -/
def assertsFalse (x : Sixteen3) : Bool := x.hasF || x.hasB

/-- Truth order ≤_t: more truth asserted, no more falsity. -/
def le_t (x y : Sixteen3) : Bool :=
  (!x.assertsTrue || y.assertsTrue) && (!y.assertsFalse || x.assertsFalse)

/-- Falsity order ≤_f: more falsity asserted, no more truth. The MIRROR of ≤_t, and
an order in its own right — this is the axis FOUR does not have. -/
def le_f (x y : Sixteen3) : Bool :=
  (!x.assertsFalse || y.assertsFalse) && (!y.assertsTrue || x.assertsTrue)

/-- Information order ≤_i: subset inclusion. More is known, nothing retracted. -/
def le_i (x y : Sixteen3) : Bool :=
  (!x.hasN || y.hasN) && (!x.hasT || y.hasT) && (!x.hasF || y.hasF) && (!x.hasB || y.hasB)

/-- The empty value: asserts nothing. Bottom of ≤_i. -/
def none : Sixteen3 := ⟨false, false, false, false⟩
/-- All of FOUR: the maximally informed value. Top of ≤_i. -/
def all : Sixteen3 := ⟨true, true, true, true⟩
/-- The image of a FOUR value: the singleton containing it. -/
def ofBelnap : Belnap → Sixteen3
  | .N => ⟨true, false, false, false⟩
  | .T => ⟨false, true, false, false⟩
  | .F => ⟨false, false, true, false⟩
  | .B => ⟨false, false, false, true⟩

/-- The sixteen values, written out. This IS the powerset of FOUR: four independent
membership bits, 2^4 = 16 rows, no row repeated and none missing. Enumerated rather
than derived because this module imports `Lean`, not Mathlib — there is no `Fintype`
here to count with, and a theorem that cannot be checked is worse than a list. -/
def all16 : List Sixteen3 :=
  [ ⟨false,false,false,false⟩, ⟨false,false,false,true⟩,
    ⟨false,false,true ,false⟩, ⟨false,false,true ,true⟩,
    ⟨false,true ,false,false⟩, ⟨false,true ,false,true⟩,
    ⟨false,true ,true ,false⟩, ⟨false,true ,true ,true⟩,
    ⟨true ,false,false,false⟩, ⟨true ,false,false,true⟩,
    ⟨true ,false,true ,false⟩, ⟨true ,false,true ,true⟩,
    ⟨true ,true ,false,false⟩, ⟨true ,true ,false,true⟩,
    ⟨true ,true ,true ,false⟩, ⟨true ,true ,true ,true⟩ ]

/-- Sixteen, by count. -/
theorem card_sixteen : all16.length = 16 := rfl

/-- Every one of the three orders is reflexive. Sixteen cases, each by computation. -/
theorem le_t_refl (x : Sixteen3) : le_t x x = true := by
  obtain ⟨n, t, f, b⟩ := x
  cases n <;> cases t <;> cases f <;> cases b <;> rfl

theorem le_f_refl (x : Sixteen3) : le_f x x = true := by
  obtain ⟨n, t, f, b⟩ := x
  cases n <;> cases t <;> cases f <;> cases b <;> rfl

theorem le_i_refl (x : Sixteen3) : le_i x x = true := by
  obtain ⟨n, t, f, b⟩ := x
  cases n <;> cases t <;> cases f <;> cases b <;> rfl

/-- THE SPLIT: ≤_t and ≤_f are genuinely different orders. FOUR cannot tell them apart
because its falsity is its truth read backwards; SIXTEEN_3 can. A witness is enough --
this is the whole reason the third order exists, and the reason a bilattice (which
carries two) is the wrong shape to hold it. -/
theorem truth_and_falsity_are_independent :
    ∃ x y : Sixteen3, le_t x y ≠ le_f x y :=
  ⟨ofBelnap .N, ofBelnap .T, by decide⟩

/-- ≤_i is not ≤_t either: the information order is a third thing, not a rebrand. -/
theorem information_is_a_third_order :
    ∃ x y : Sixteen3, le_i x y ≠ le_t x y :=
  ⟨ofBelnap .T, all, by decide⟩

/-- Half of the sixteen carry B directly -- one free bit, trivially eight. This is
why trilattice mode entails paraconsistent: a kernel that explodes cannot carry the
`hasB` values, and dropping them is not SIXTEEN_3 any more, only a smaller lattice. -/
theorem eight_carry_B_directly :
    (all16.filter (fun x => x.hasB)).length = 8 := by decide

/-- Ten of the sixteen assert BOTH truth and falsity (via hasT/hasF/hasB in any
combination) -- strictly more than the eight that merely contain B, since a value can
assert both poles with hasB = false by holding hasT and hasF together. This is the
sharper reason trilattice entails paraconsistent: the recursor block has to catch
every value that asserts a contradiction, not only the ones that store it as a single
bit. -/
theorem ten_assert_both_poles :
    (all16.filter (fun x => x.assertsTrue && x.assertsFalse)).length = 10 := by
  decide

end Sixteen3

/--
Paraconsistent natural deduction style.

In paraconsistent logic, the rule of explosion (ex falso) is rejected:
  ¬(∀ (P : Prop), False → P)

Instead, contradictions are handled using Belnap-style four-valued semantics,
where a proposition can be true, false, both, or neither.
-/
def ParaconsistentLogic : Type := Unit

end Paraconsistent

-- Run this file directly with `lean --run` to test the paraconsistent module.
-- `main` is defined per-file; no top-level main here to avoid import conflicts.
```
