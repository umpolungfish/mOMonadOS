/-
Imscribing/Classical/UnitDistancePhaseTransition.lean
Unit Distances in ℝᵈ — Phase Transition at d=4

Theorem: f_d(n), the maximum number of unit distance pairs among
n points in ℝᵈ, satisfies:
  f_d(n) = O(n^{4/3}) for d=2
  f_d(n) = O(n^{3/2}) for d=3
  f_d(n) = ½(1 - 1/⌊d/2⌋)·n² + o(n²) for d ≥ 4

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.UnitDistancePhaseTransition

/-!
Axiom (Erdős–Lenz phase transition):
  d=2: Szemerédi–Trotter gives O(n^{4/3}).
  d=3: Zarankiewicz-type bound gives O(n^{3/2}).
  d≥4: Lenz construction (orthogonal circles) + Erdős–Stone gives
    the exact asymptotic constant ½(1 - 1/⌊d/2⌋).

Belnap Verdict: B (Both) — d=2,3 bounds are partially open (exact
  exponents may yet improve); d≥4 is settled.
-/


/-- The number of unit-distance pairs in `A`, counted as ordered pairs. -/
noncomputable def unitPairs {d : ℕ} (A : Finset (EuclideanSpace ℝ (Fin d))) : ℕ :=
    ((A ×ˢ A).filter fun pq => dist pq.1 pq.2 = 1).card

/-- The planar bound, `f₂(n) = O(n^{4/3})` (Szemerédi–Trotter). -/
axiom unit_distance_planar :
    ∃ C : ℝ, 0 < C ∧ ∀ (n : ℕ) (A : Finset (EuclideanSpace ℝ (Fin 2))),
      A.card = n → (unitPairs A : ℝ) ≤ C * (n : ℝ) ^ ((4 : ℝ) / 3)

/-- The spatial bound, `f₃(n) = O(n^{3/2})` (Zarankiewicz type). -/
axiom unit_distance_spatial :
    ∃ C : ℝ, 0 < C ∧ ∀ (n : ℕ) (A : Finset (EuclideanSpace ℝ (Fin 3))),
      A.card = n → (unitPairs A : ℝ) ≤ C * (n : ℝ) ^ ((3 : ℝ) / 2)

/-- The settled regime `d ≥ 4`: the Lenz construction plus Erdős–Stone give
    the exact asymptotic `f_d(n) = ½(1 - 1/⌊d/2⌋)n² + o(n²)`, so the ordered
    count is `(1 - 1/⌊d/2⌋)n² + o(n²)`. -/
axiom unit_distance_phase_transition :
    ∀ d : ℕ, 4 ≤ d → ∀ ε : ℝ, 0 < ε → ∃ N : ℕ, ∀ n : ℕ, N ≤ n →
      (∀ A : Finset (EuclideanSpace ℝ (Fin d)), A.card = n →
        (unitPairs A : ℝ) ≤ (1 - 1 / (d / 2 : ℕ)) * (n : ℝ) ^ 2 + ε * (n : ℝ) ^ 2) ∧
      (∃ A : Finset (EuclideanSpace ℝ (Fin d)), A.card = n ∧
        (1 - 1 / (d / 2 : ℕ)) * (n : ℝ) ^ 2 - ε * (n : ℝ) ^ 2 ≤ (unitPairs A : ℝ))

/-- The phase transition at `d = 4`: below it only upper bounds are known,
    at and above it the constant is exact. -/
theorem main (d : ℕ) (hd : 4 ≤ d) (ε : ℝ) (hε : 0 < ε) :
    ∃ N : ℕ, ∀ n : ℕ, N ≤ n →
      (∀ A : Finset (EuclideanSpace ℝ (Fin d)), A.card = n →
        (unitPairs A : ℝ) ≤ (1 - 1 / (d / 2 : ℕ)) * (n : ℝ) ^ 2 + ε * (n : ℝ) ^ 2) ∧
      (∃ A : Finset (EuclideanSpace ℝ (Fin d)), A.card = n ∧
        (1 - 1 / (d / 2 : ℕ)) * (n : ℝ) ^ 2 - ε * (n : ℝ) ^ 2 ≤ (unitPairs A : ℝ)) :=
  unit_distance_phase_transition d hd ε hε

end Classical.UnitDistancePhaseTransition
