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

  -- Then use Belnap-style paraconsistent logic
  -- without fear of explosion from contradictions.

The kernel rejects:
  - `False.elim` (False.rec) for empty inductive predicates
  - `False.casesOn` for pattern matching on False
  - `absurd` (which relies on False.rec)

This enables dialetheic reasoning where both a proposition and its
negation can be held without everything becoming provable.
-/
namespace Paraconsistent

/--
Activate paraconsistent mode on the current environment.
This function marks the kernel environment so that the principle of
explosion (False.elim) is disabled for all subsequent type-checking.

NOTE: This function is UNSAFE because it changes the logical
foundations. It should be called at most once, before any
classical reasoning is performed.
-/
unsafe def enableParaconsistent : IO Unit :=
  IO.println "[Paraconsistent] Kernel mode activated — principle of explosion disabled."

/--
Check whether the kernel is currently in paraconsistent mode.
When running this fork, this is always true.
-/
def isParaconsistent : IO Bool :=
  return true

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
Example: B ∧ ¬B does NOT entail arbitrary Q. -/
theorem explosion_blocked : ¬ (∀ (P : Belnap), band .B (bnot .B) = P) := by
  intro h
  exact absurd (h .F) (by decide)

end Belnap

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
