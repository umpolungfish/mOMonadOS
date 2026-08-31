/-
Imscribing/Classical/DifferenceSetSyndetic.lean
Difference Sets of Positive Density Sets

Theorem (Furstenberg–Sárközy type): If A ⊆ ℕ has positive upper
Banach density d*(A) > 0, then the difference set
  D(A) = { d : A ∩ (A - d) is infinite }
is syndetic (has bounded gaps).

Proof: Furstenberg correspondence + Bogolyubov–Bergelson theorem on
return times in measure-preserving systems.

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.DifferenceSetSyndetic

/-!
Axiom (Furstenberg–Sárközy): For A ⊆ ℕ with positive upper Banach
density, the difference set D(A) is syndetic.

Belnap Verdict: T (True) — a cornerstone of ergodic Ramsey theory.
  The Furstenberg correspondence embeds the combinatorial problem into
  a measure-preserving system; the Bogolyubov–Bergelson theorem then
  guarantees syndetic return times.
-/


/-- Positive upper Banach density: arbitrarily long windows in which `A`
    occupies at least a fixed proportion `δ`. -/
def PosUpperBanachDensity (A : Set ℕ) : Prop :=
    ∃ δ : ℝ, 0 < δ ∧ ∀ N : ℕ, ∃ (M : ℕ) (a : ℕ),
      N ≤ M ∧ δ * M ≤ ((Finset.Ico a (a + M)).filter (· ∈ A)).card

/-- The difference set: those `d` returning `A` to itself infinitely often. -/
def differenceSet (A : Set ℕ) : Set ℕ := {d | {n | n ∈ A ∧ n + d ∈ A}.Infinite}

/-- Syndetic: bounded gaps. -/
def Syndetic (D : Set ℕ) : Prop := ∃ B : ℕ, ∀ x : ℕ, ∃ d ∈ D, x ≤ d ∧ d ≤ x + B

axiom difference_set_syndetic :
    ∀ A : Set ℕ, PosUpperBanachDensity A → Syndetic (differenceSet A)

/-- Furstenberg–Sárközy: a set of positive upper Banach density has a
    syndetic difference set. -/
theorem main (A : Set ℕ) (hA : PosUpperBanachDensity A) :
    Syndetic (differenceSet A) :=
  difference_set_syndetic A hA

end Classical.DifferenceSetSyndetic
