/-
# The representation function of a Sidon set

The Erdős–Turán conjecture on additive bases: if `A ⊆ ℕ` is an asymptotic basis
of order two, then its representation function

    r_A(n) = #{ (a,b) ∈ A × A : a + b = n }

is unbounded. It is open.

This file proves the bridge between that conjecture and the Sidon condition,
which is the part of the statement `ErdosTuranBound.lean` was already halfway
to. A Sidon set is exactly one whose sums are almost injective, so its
representation function is bounded by two, everywhere and with no hypotheses on
`n`. The consequence is immediate and is the reason Sidon sets are where one
looks for a counterexample: a Sidon set that were an asymptotic basis of order
two would refute the conjecture outright.

The proof shape is the banked count. Fix `n`, split the pairs, hold the pair
`{a,b}` in scope while the swap is applied, and fuse: the Sidon property says
the unordered pair is determined, and the ordered pairs over it are exactly two.
The swap involution `(a,b) ↦ (b,a)` is the same one that carries
`card_unorderedPairs` in `ErdosTuranBound.lean`.
-/

import Mathlib

namespace Imscribing.Classical

open Finset

/-- `B` is Sidon: `a + b = c + d` with all four in `B` forces `{a,b} = {c,d}`. -/
def isSidonFinset (B : Finset ℕ) : Prop :=
  ∀ {a b c d : ℕ}, a ∈ B → b ∈ B → c ∈ B → d ∈ B →
    a + b = c + d → (({a, b} : Finset ℕ) = ({c, d} : Finset ℕ))

/-- The ordered pairs from `B` summing to `n`. This is `r_B(n)` before counting. -/
def reps (B : Finset ℕ) (n : ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun p => p.1 + p.2 = n)

/-- Membership in `reps`, unfolded. -/
lemma mem_reps {B : Finset ℕ} {n : ℕ} {p : ℕ × ℕ} :
    p ∈ reps B n ↔ p.1 ∈ B ∧ p.2 ∈ B ∧ p.1 + p.2 = n := by
  simp [reps, mem_filter, mem_product, and_assoc]

/-- Two numbers are determined, as an ordered pair, by their unordered pair.

    `{a,b} = {c,d}` as finsets leaves exactly two ways to order them, and this
    is where the swap enters: the involution `(a,b) ↦ (b,a)` acts on the fibre
    over an unordered pair, and the fibre has at most two points because that is
    the size of the orbit. -/
lemma pair_cases {a b c d : ℕ} (h : ({a, b} : Finset ℕ) = {c, d}) :
    (c = a ∧ d = b) ∨ (c = b ∧ d = a) := by
  classical
  have hc : c = a ∨ c = b := by
    have hm : c ∈ ({a, b} : Finset ℕ) := by rw [h]; simp
    simpa using hm
  have hd : d = a ∨ d = b := by
    have hm : d ∈ ({a, b} : Finset ℕ) := by rw [h]; simp
    simpa using hm
  have hbmem : b = c ∨ b = d := by
    have hm : b ∈ ({c, d} : Finset ℕ) := by rw [← h]; simp
    simpa using hm
  have hamem : a = c ∨ a = d := by
    have hm : a ∈ ({c, d} : Finset ℕ) := by rw [← h]; simp
    simpa using hm
  rcases hc with hc | hc <;> rcases hd with hd | hd
  · -- c = a and d = a: then {c,d} is a singleton, so b must be a too
    subst hc; subst hd
    rcases hbmem with hb | hb
    · exact Or.inl ⟨rfl, hb.symm⟩
    · exact Or.inl ⟨rfl, hb.symm⟩
  · exact Or.inl ⟨hc, hd⟩
  · exact Or.inr ⟨hc, hd⟩
  · -- c = b and d = b: singleton again, so a = b
    subst hc; subst hd
    rcases hamem with ha | ha
    · exact Or.inl ⟨ha.symm, rfl⟩
    · exact Or.inl ⟨ha.symm, rfl⟩

/-- **The representation function of a Sidon set is bounded by two.**

    For every `n`, with no hypothesis on `n` and no asymptotics: a Sidon set
    admits at most two ordered representations of any number, and the two are a
    single unordered pair with its swap.

    This is the Sidon condition read as a statement about `r`, which is the
    quantity the Erdős–Turán conjecture on additive bases is about. -/
theorem sidon_reps_le_two (B : Finset ℕ) (hB : isSidonFinset B) (n : ℕ) :
    (reps B n).card ≤ 2 := by
  classical
  -- If there is nothing to count we are done; otherwise fix one representation
  -- and hold it while the swap runs. That fixed pair is the banked count.
  rcases Finset.eq_empty_or_nonempty (reps B n) with h | ⟨⟨a, b⟩, hab⟩
  · simp [h]
  obtain ⟨ha, hb, hsum⟩ := mem_reps.mp hab
  -- every representation is that pair or its swap
  have hsub : reps B n ⊆ ({(a, b), (b, a)} : Finset (ℕ × ℕ)) := by
    intro ⟨c, d⟩ hcd
    obtain ⟨hc, hd, hcd'⟩ := mem_reps.mp hcd
    have := hB ha hb hc hd (by omega)
    rcases pair_cases this with ⟨rfl, rfl⟩ | ⟨rfl, rfl⟩ <;> simp
  calc (reps B n).card
      ≤ ({(a, b), (b, a)} : Finset (ℕ × ℕ)).card := Finset.card_le_card hsub
    _ ≤ 2 := Finset.card_insert_le _ _ |>.trans (by simp)

/-- The bound is attained, so two is the right constant and not merely an upper
    bound one could improve. -/
theorem sidon_reps_eq_two_example :
    ∃ (B : Finset ℕ) (n : ℕ), isSidonFinset B ∧ (reps B n).card = 2 := by
  refine ⟨{0, 1}, 1, ?_, by decide⟩
  intro a b c d ha hb hc hd h
  fin_cases ha <;> fin_cases hb <;> fin_cases hc <;> fin_cases hd <;>
    simp_all <;> decide

/-! ## The consequence for Erdős–Turán -/

/-- `A` is an asymptotic basis of order two: every sufficiently large `n` is a
    sum of two elements. -/
def isAsymptoticBasisTwo (A : Set ℕ) : Prop :=
  ∃ N₀, ∀ n ≥ N₀, ∃ a ∈ A, ∃ b ∈ A, a + b = n

/-- The Erdős–Turán conjecture on additive bases, stated over finite windows:
    an asymptotic basis of order two has unbounded representation function. -/
def ErdosTuranAdditiveBases : Prop :=
  ∀ (A : Set ℕ), isAsymptoticBasisTwo A →
    ∀ C : ℕ, ∃ (n : ℕ) (B : Finset ℕ), (↑B ⊆ A) ∧ C < (reps B n).card

/-- **A Sidon set cannot witness the conjecture.**

    If the conjecture holds, no set that is Sidon on every finite window is an
    asymptotic basis of order two: the conjecture demands representation counts
    above every bound, and `sidon_reps_le_two` caps them at two.

    This is why Sidon sets are where a counterexample would have to live, and
    equally why one has never been produced there: the same property that makes
    the sums nearly injective is the property that bounds `r`. -/
theorem sidon_not_basis_of_erdos_turan
    (hET : ErdosTuranAdditiveBases) (A : Set ℕ)
    (hSidon : ∀ B : Finset ℕ, ↑B ⊆ A → isSidonFinset B) :
    ¬ isAsymptoticBasisTwo A := by
  intro hbasis
  obtain ⟨n, B, hBA, hcard⟩ := hET A hbasis 2
  exact absurd (sidon_reps_le_two B (hSidon B hBA) n) (by omega)

#print axioms sidon_reps_le_two
#print axioms sidon_not_basis_of_erdos_turan

end Imscribing.Classical
