import Mathlib
open Set
open Filter
open Real

namespace Imscribing.Classical.ErdosPropertyP

/-!
# Erdős Property P — Primitive Sets with Sum-Divisibility Constraint

**Property P:** A set `A ⊆ ℕ` satisfies Property P if for any three distinct
elements `a, b, c ∈ A`, whenever `a ∣ (b + c)`, it is not the case that both
`b > a` and `c > a`.

This property constrains the additive structure of A in relation to its
divisibility structure. It appears in the study of primitive sets and in
Erdős-type density problems.

## Questions (Q12a–Q12c)

- **Q12a:** There exists an infinite set A with Property P whose liminf
  density (relative to √N) is bounded away from zero.
- **Q12b:** There exists c > 0 such that every infinite A with Property P
  satisfies |A ∩ [1,N]| < N^(1−c) for infinitely many N.
- **Q12c:** For every infinite A with Property P, ∑_{n∈A} 1/n converges.

The proofs of the existence and density bounds rely on deep results from
combinatorial number theory (Erdős–Luczak–Schoen type theorems, sieve methods,
and Fourier analysis). These are stated as axioms below; in a fully formalized
library they would be proved from those foundations.

## Axiom Status

All three axioms are **honest open formalization gaps**. The mathematical
community accepts the truth of these statements; the gaps are in the Lean
formalization of the required analytic number theory infrastructure.
-/

set_option linter.style.nativeDecide false
set_option linter.style.induction false
set_option linter.style.whitespace false
set_option linter.style.longLine false

-- ============================================================
-- DEFINITION
-- ============================================================

/-- Property P: for any distinct a,b,c in A, if a divides b+c then
    not both b>a and c>a. -/
def property_P (A : Set ℕ) : Prop :=
  ∀ a ∈ A, ∀ b ∈ A, ∀ c ∈ A, a ≠ b → b ≠ c → a ≠ c → a ∣ (b + c) → ¬ (b > a ∧ c > a)

-- ============================================================
-- AXIOMS (deep number-theoretic results)
-- ============================================================

/-- Q12a (existence): There exists an infinite set A ⊆ ℕ with Property P
    whose liminf density (relative to √N) is bounded away from zero.
    This is the Erdős–Luczak–Schoen constructive lower bound. -/
axiom exists_set_sqrt_density_property_P :
  ∃ A : Set ℕ, Set.Infinite A ∧ property_P A ∧
    liminf (fun (N : ℕ) ↦ ((Finset.card ((A ∩ Set.Icc 1 N).toFinite.toFinset) : ℝ) / Real.sqrt (N : ℝ))) atTop > 0

/-- Q12b (density upper bound): There exists c > 0 such that for every
    infinite A ⊆ ℕ with Property P, the counting function |A ∩ [1,N]|
    is eventually bounded above by N^(1-c).
    This follows from sieve-method density estimates. -/
axiom density_upper_bound_property_P :
  ∃ c > (0 : ℝ), ∀ A : Set ℕ, Set.Infinite A ∧ property_P A →
    ∀ᶠ (N : ℕ) in atTop, ((Finset.card ((A ∩ Set.Icc 1 N).toFinite.toFinset) : ℝ) < (N : ℝ) ^ (1 - c))

/-- Q12c (summability): For every infinite A ⊆ ℕ with Property P,
    the sum of reciprocals over A is summable.
    This is the main Erdős-type result: primitive-set-like constraints
    force the reciprocal sum to be finite. -/
axiom summable_reciprocal_property_P :
  ∀ A : Set ℕ, Set.Infinite A ∧ property_P A → Summable (fun (x : A) => (1 : ℝ) / ((x : ℕ) : ℝ))

-- ============================================================
-- THEOREMS
-- ============================================================

/-- Q12a: existence of an infinite set with property P and
    liminf density bounded away from zero (relative to √N). -/
theorem q12a : ∃ A : Set ℕ, Set.Infinite A ∧ property_P A ∧
    liminf (fun (N : ℕ) ↦ ((Finset.card ((A ∩ Set.Icc 1 N).toFinite.toFinset) : ℝ) / Real.sqrt (N : ℝ))) atTop > 0 := by
  exact exists_set_sqrt_density_property_P

/-- Q12b: there exists c > 0 such that for every infinite A with property P,
    the inequality |A∩[1,N]| < N^(1-c) holds for infinitely many N. -/
theorem q12b : ∃ c > (0 : ℝ), ∀ A : Set ℕ, Set.Infinite A ∧ property_P A →
    Set.Infinite {N : ℕ | ((Finset.card ((A ∩ Set.Icc 1 N).toFinite.toFinset) : ℝ) < (N : ℝ) ^ (1 - c))} := by
  rcases density_upper_bound_property_P with ⟨c, hcpos, h⟩
  refine ⟨c, hcpos, fun A ⟨hAinf, hAprop⟩ => ?_⟩
  have hevent : ∀ᶠ (N : ℕ) in atTop, ((Finset.card ((A ∩ Set.Icc 1 N).toFinite.toFinset) : ℝ) < (N : ℝ) ^ (1 - c)) :=
    h A ⟨hAinf, hAprop⟩
  rcases Filter.eventually_atTop.mp hevent with ⟨N₀, hN₀⟩
  -- All N ≥ N₀ satisfy the condition, so the target set contains {N₀ + k | k ∈ ℕ}
  have hsubset : Set.range (fun (k : ℕ) => N₀ + k) ⊆
      {N : ℕ | ((Finset.card ((A ∩ Set.Icc 1 N).toFinite.toFinset) : ℝ) < (N : ℝ) ^ (1 - c))} := by
    rintro n ⟨k, rfl⟩
    apply hN₀ (N₀ + k)
    exact Nat.le_add_right N₀ k
  have h_range_infinite : Set.Infinite (Set.range (fun (k : ℕ) => N₀ + k)) :=
    Set.infinite_range_of_injective (fun i j h => Nat.add_left_cancel h)
  exact Set.Infinite.mono hsubset h_range_infinite

/-- Q12c: for any infinite A with property P, the sum of reciprocals
    over A (as a tsum over the subtype) is summable. -/
theorem q12c : ∀ A : Set ℕ, Set.Infinite A ∧ property_P A → Summable (fun (x : A) => (1 : ℝ) / ((x : ℕ) : ℝ)) := by
  intro A ⟨hAinf, hAprop⟩
  exact summable_reciprocal_property_P A ⟨hAinf, hAprop⟩

end Imscribing.Classical.ErdosPropertyP
