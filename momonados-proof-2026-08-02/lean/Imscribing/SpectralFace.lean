-- Imscribing/SpectralFace.lean
-- The spectral face of cyclization: ρ = 2 for a pure ring.
--
-- Isomorphisms.lean proved the ALGEBRAIC faces (fusion, fixed point, section, projection).
-- This module adds the SPECTRAL face MoDoT's `forge`/`material` reports lean on: a pure
-- head-to-tail ring is 2-regular, and its adjacency matrix carries the eigenvalue 2 on the
-- all-ones (Perron) vector — the honest content of "ρ = 2 ⟺ a pure cycle". Branching shows
-- up as a vertex whose row sum (degree) exceeds 2, lifting the uniform Perron value above 2.
--
-- Grounded in Mathlib's matrix/`Fin` group structure; no new axioms.

import Mathlib

namespace Imscribing.SpectralFace

open Matrix Finset

variable {n : ℕ} [NeZero n]

/-- Cyclic shift adjacency i → i+1: the directed ring. A permutation matrix. -/
def shift (n : ℕ) [NeZero n] : Matrix (Fin n) (Fin n) ℝ :=
  fun i j => if j = i + 1 then 1 else 0

/-- The (undirected) ring / cycle-graph adjacency: the shift plus its transpose, so each
    vertex i is joined to i+1 and i-1. -/
def ringAdj (n : ℕ) [NeZero n] : Matrix (Fin n) (Fin n) ℝ :=
  shift n + (shift n)ᵀ

/-- i's only out-neighbor is i+1: the row of `shift` sums to 1. -/
private lemma shift_row_sum (i : Fin n) : ∑ j, shift n i j = 1 := by
  simp only [shift]
  simpa using Finset.sum_ite_eq' Finset.univ (i + 1) (fun _ => (1 : ℝ))

/-- `i = j + 1` (i is j's successor) exactly when `j = i - 1` (j is i's predecessor). -/
private lemma nb_iff (i j : Fin n) : (i = j + 1) ↔ (j = i - 1) := by
  constructor
  · rintro rfl; exact (add_sub_cancel_right j 1).symm
  · rintro rfl; exact (sub_add_cancel i 1).symm

/-- i's only in-neighbor is i-1: the column of `shift` sums to 1. -/
private lemma shift_col_sum (i : Fin n) : ∑ j, shift n j i = 1 := by
  have h : (fun j => shift n j i) = fun j => if j = i - 1 then (1 : ℝ) else 0 := by
    funext j; simp only [shift]; exact if_congr (nb_iff i j) rfl rfl
  rw [h]
  simpa using Finset.sum_ite_eq' Finset.univ (i - 1) (fun _ => (1 : ℝ))

/-- The shift fixes the all-ones vector: `shift · 1 = 1`. -/
theorem shift_mulVec_one : (shift n).mulVec (fun _ => 1) = fun _ => 1 := by
  funext i; simp only [Matrix.mulVec, dotProduct, mul_one]; exact shift_row_sum i

/-- Its transpose fixes the all-ones vector too: `shiftᵀ · 1 = 1`. -/
theorem shiftT_mulVec_one : (shift n)ᵀ.mulVec (fun _ => 1) = fun _ => 1 := by
  funext i
  simp only [Matrix.mulVec, dotProduct, Matrix.transpose_apply, mul_one]
  exact shift_col_sum i

/-- Every vertex of the ring has degree 2 — the ring is 2-regular. -/
theorem ring_two_regular (i : Fin n) : ∑ j, ringAdj n i j = 2 := by
  simp only [ringAdj, Matrix.add_apply, Finset.sum_add_distrib, Matrix.transpose_apply]
  rw [shift_row_sum i, shift_col_sum i]; norm_num

/-- **The spectral face.** The ring adjacency carries eigenvalue 2 on the all-ones Perron
    vector: `ringAdj · 1 = 2 • 1`. Since the all-ones vector is positive, this is the
    Perron eigenvalue — the rigorous content of `ρ = 2` for a pure cycle. -/
theorem ring_has_eigenvalue_two :
    (ringAdj n).mulVec (fun _ => 1) = (2 : ℝ) • (fun _ => (1 : ℝ)) := by
  funext i
  have h1 := congrFun (shift_mulVec_one (n := n)) i
  have h2 := congrFun (shiftT_mulVec_one (n := n)) i
  simp only [ringAdj, Matrix.add_mulVec, Pi.add_apply, h1, h2, Pi.smul_apply, smul_eq_mul]
  norm_num

/-- Cyclization and its spectral face are one act: the head-to-tail ring closing (chem)
    IS the all-ones vector sitting at eigenvalue 2 = the degree (math). Stated as the pair
    that must always hold together — 2-regular AND Perron-eigenvalue 2. -/
theorem cyclize_chem_and_math (i : Fin n) :
    (∑ j, ringAdj n i j = 2) ∧
      (ringAdj n).mulVec (fun _ => 1) = (2 : ℝ) • (fun _ => (1 : ℝ)) :=
  ⟨ring_two_regular i, ring_has_eigenvalue_two⟩

-- ── Branching contrast ──────────────────────────────────────────────────────

/-- Add one extra undirected edge {a,b} to an adjacency matrix. -/
def addEdge (M : Matrix (Fin n) (Fin n) ℝ) (a b : Fin n) : Matrix (Fin n) (Fin n) ℝ :=
  fun i j => M i j + (if (i = a ∧ j = b) ∨ (i = b ∧ j = a) then 1 else 0)

/-- A branched vertex has degree ≥ 3. Attaching an edge from ring-vertex `a` to a genuine
    non-neighbor `b` (b ≠ a, b ≠ a+1, b ≠ a-1) raises a's row sum to 3 — the uniform Perron
    value can no longer be 2, which is the spectral signature of branching (ρ > 2). -/
theorem branched_row_sum_three
    (a b : Fin n) (hne : b ≠ a) (_hsucc : b ≠ a + 1) (_hpred : b ≠ a - 1) :
    ∑ j, addEdge (ringAdj n) a b a j = 3 := by
  have key : ∀ j : Fin n,
      addEdge (ringAdj n) a b a j = ringAdj n a j + (if j = b then (1 : ℝ) else 0) := by
    intro j
    simp only [addEdge]
    refine congrArg (ringAdj n a j + ·) (if_congr ?_ rfl rfl)
    constructor
    · rintro (⟨_, h⟩ | ⟨h, _⟩)
      · exact h
      · exact absurd h.symm hne
    · intro h; exact Or.inl ⟨trivial, h⟩
  simp_rw [key]
  rw [Finset.sum_add_distrib, ring_two_regular a]
  have hone : ∑ j, (if j = b then (1 : ℝ) else 0) = 1 := by
    simpa using Finset.sum_ite_eq' Finset.univ b (fun _ => (1 : ℝ))
  rw [hone]; norm_num

end Imscribing.SpectralFace
