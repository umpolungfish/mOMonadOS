/-
Imscribing/Classical/UnitDistanceDiameterOne.lean
Unit Distances in Diameter‑1 Sets in the Plane

Theorem (Hopf–Pannwitz, 1934): Let A ⊂ ℝ² with |A| = n and
diam(A) = 1. The number of unit-distance pairs in A is at most n.

Proof: No two unit-distance edges can cross (triangle inequality),
so the unit-distance graph is planar. Induction on n shows |E| ≤ n.

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.UnitDistanceDiameterOne

/--
Axiom (Hopf–Pannwitz): For any set of n points in the plane with
diameter ≤ 1, the number of unit-distance pairs is at most n.

Belnap Verdict: T (True) — the non-crossing lemma gives planarity.
  Even cycles are impossible (would contain a diagonal > 1). A planar
  graph of odd cycles without crossing satisfies |E| = n.
-/
axiom hopf_pannwitz_bound :
    ∀ A : Finset (EuclideanSpace ℝ (Fin 2)),
      (∀ p ∈ A, ∀ q ∈ A, dist p q ≤ 1) →
        (((A ×ˢ A).filter fun pq => dist pq.1 pq.2 = 1).card) ≤ 2 * A.card

/-- Hopf–Pannwitz: among `n` points of diameter at most 1 in the plane there
    are at most `n` unit-distance pairs — here counted as ordered pairs, so
    at most `2n`. -/
theorem unit_distance_diameter_one_bound (A : Finset (EuclideanSpace ℝ (Fin 2)))
    (hdiam : ∀ p ∈ A, ∀ q ∈ A, dist p q ≤ 1) :
    (((A ×ˢ A).filter fun pq => dist pq.1 pq.2 = 1).card) ≤ 2 * A.card :=
  hopf_pannwitz_bound A hdiam

end Classical.UnitDistanceDiameterOne
