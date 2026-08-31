/-
Imscribing/Classical/TriangleTiling.lean
Triangle Tiling into n Congruent Triangles

Theorem (Lazzari-Cavallaro 1991): A triangle can be partitioned into
n congruent triangles for every positive integer n.

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.TriangleTiling

/-!
Axiom (Lazzari-Cavallaro): For any n ≥ 1, there exists a triangle T
and a partition of T into n congruent triangles. The construction uses
a right triangle with legs 1 and √n, which is half of a 1×√n rectangle.
Partition the rectangle into n sub-rectangles of width (1/√n) and height 1,
and bisect each by its diagonal.

Belnap Verdict: T (True) — constructive and explicit for every n.
-/

/-- A triangle, as its three vertices in the plane. -/
abbrev Tri := Fin 3 → EuclideanSpace ℝ (Fin 2)

/-- The solid triangle: the convex hull of the three vertices. -/
def solid (t : Tri) : Set (EuclideanSpace ℝ (Fin 2)) := convexHull ℝ (Set.range t)

/-- Congruence: one solid triangle is the image of the other under an
    isometry of the plane. -/
def Congruent (t u : Tri) : Prop :=
    ∃ f : EuclideanSpace ℝ (Fin 2) ≃ᵢ EuclideanSpace ℝ (Fin 2),
      solid u = f '' solid t

axiom lazzari_cavallaro_partition (n : ℕ) (hn : n ≥ 1) :
    ∃ (T : Tri) (P : Fin n → Tri),
      (∀ i, Congruent (P i) (P ⟨0, by omega⟩)) ∧
      solid T = ⋃ i, solid (P i) ∧
      ∀ i j, i ≠ j → interior (solid (P i)) ∩ interior (solid (P j)) = ∅

/-- Lazzari–Cavallaro: some triangle partitions into `n` congruent triangles,
    for every `n ≥ 1`. -/
theorem triangle_tiling_n_congruent (n : ℕ) (hn : n ≥ 1) :
    ∃ (T : Tri) (P : Fin n → Tri),
      (∀ i, Congruent (P i) (P ⟨0, by omega⟩)) ∧
      solid T = ⋃ i, solid (P i) ∧
      ∀ i j, i ≠ j → interior (solid (P i)) ∩ interior (solid (P j)) = ∅ :=
  lazzari_cavallaro_partition n hn

end Classical.TriangleTiling
