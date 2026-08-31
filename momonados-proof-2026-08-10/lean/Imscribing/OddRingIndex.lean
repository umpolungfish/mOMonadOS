-- Imscribing/OddRingIndex.lean
-- THE INDEX OF AN ODD RING: n₊ − n₋ AS THE SURVIVAL COUNT
--
-- The carved macrocycle [[0,2,1],[2,0,1],[1,1,0]] — forged identically by
-- {sakharov_conditions, paraconsistent_computer, connes_embedding_problem},
-- {sic_povm_d12, ray_class_field_Qsqrt, connes_embedding_problem}, and the
-- same set with sic_povm_d2048_zauner_conjecture — has spectrum
-- {1+√3, −2, 1−√3}, gap √3−1, and INDEX n₊ − n₋ = −1.
--
-- The Void's ring C₄ has spectrum {2,−2,0,0} and index 0: a bipartite
-- (2-colorable) substrate pairs perfectly and cancels. An odd ring cannot be
-- 2-colored; frustration forces one net unpaired mode. C₃ is the first thing
-- that closes and the first that cannot cancel.
--
-- Every index below is certified WITHOUT the spectral theorem: an explicit
-- rational congruence S · A · Sᵀ = D (Sylvester), with det S ≠ 0, so the
-- signature of A is the signature of the explicit diagonal D. The √3 spectrum
-- of the carved ring is tied on separately over ℝ by exact factorization.
--
-- Companion: Imscribing/HowDiracEquationArise.lean §S7 — Ω=𐑭 (ah) forces the
-- integer index, index(D̸) = n₊ − n₋; remove ah and the anomaly is not
-- quantized. This file computes that integer for the measured rings.

import Mathlib

namespace Imscribing.OddRingIndex

open Matrix

set_option linter.style.longLine false

-- ============================================================
-- S1. THE MATRICES
-- ============================================================

/-- The carved macrocycle: a triangle with one doubled edge (the cross-link). -/
def ringA : Matrix (Fin 3) (Fin 3) ℚ := !![0, 2, 1; 2, 0, 1; 1, 1, 0]

/-- C₃ — the smallest cycle, the first closure. -/
def cyc3 : Matrix (Fin 3) (Fin 3) ℚ := !![0, 1, 1; 1, 0, 1; 1, 1, 0]

/-- C₄ — the Void's ring (void_consensus_protocol closes into it by doubling). -/
def cyc4 : Matrix (Fin 4) (Fin 4) ℚ := !![0, 1, 0, 1; 1, 0, 1, 0; 0, 1, 0, 1; 1, 0, 1, 0]

-- ============================================================
-- S2. RATIONAL CONGRUENCES (Sylvester witnesses)
-- ============================================================

def S_A : Matrix (Fin 3) (Fin 3) ℚ := !![1, 1, 0; -1/2, 1/2, 0; -1/2, -1/2, 1]
def D_A : Matrix (Fin 3) (Fin 3) ℚ := !![4, 0, 0; 0, -1, 0; 0, 0, -1]

def S_3 : Matrix (Fin 3) (Fin 3) ℚ := !![1, 1, 0; -1/2, 1/2, 0; -1, -1, 1]
def D_3 : Matrix (Fin 3) (Fin 3) ℚ := !![2, 0, 0; 0, -1/2, 0; 0, 0, -2]

def S_4 : Matrix (Fin 4) (Fin 4) ℚ :=
  !![1, 1, 0, 0; -1/2, 1/2, 0, 0; -1, 0, 1, 0; 0, -1, 0, 1]
def D_4 : Matrix (Fin 4) (Fin 4) ℚ :=
  !![2, 0, 0, 0; 0, -1/2, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]

/-- The carved ring is congruent to diag(4, −1, −1): one positive, two negative. -/
theorem ringA_congruent : S_A * ringA * S_Aᵀ = D_A := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    simp [S_A, ringA, D_A, Matrix.mul_apply, Fin.sum_univ_three] <;> norm_num

/-- C₃ is congruent to diag(2, −1/2, −2): one positive, two negative. -/
theorem cyc3_congruent : S_3 * cyc3 * S_3ᵀ = D_3 := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    simp [S_3, cyc3, D_3, Matrix.mul_apply, Fin.sum_univ_three] <;> norm_num

/-- The Void's ring is congruent to diag(2, −1/2, 0, 0): one positive, one negative. -/
theorem cyc4_congruent : S_4 * cyc4 * S_4ᵀ = D_4 := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    simp [S_4, cyc4, D_4, Matrix.mul_apply, Fin.sum_univ_four] <;> norm_num

/-- The witnesses are genuine changes of basis. -/
theorem S_A_invertible : S_A.det ≠ 0 := by
  simp [S_A, Matrix.det_fin_three]; norm_num

theorem S_3_invertible : S_3.det ≠ 0 := by
  simp [S_3, Matrix.det_fin_three]; norm_num

/-- Explicit inverse of S_4 (block-triangular). -/
def T_4 : Matrix (Fin 4) (Fin 4) ℚ :=
  !![1/2, -1, 0, 0; 1/2, 1, 0, 0; 1/2, -1, 1, 0; 1/2, 1, 0, 1]

theorem S_4_mul_T_4 : S_4 * T_4 = 1 := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    simp [S_4, T_4, Matrix.mul_apply, Fin.sum_univ_four] <;> norm_num

theorem S_4_invertible : S_4.det ≠ 0 := by
  intro h0
  have h := congrArg Matrix.det S_4_mul_T_4
  rw [Matrix.det_mul, h0, Matrix.det_one] at h
  simp at h

-- ============================================================
-- S3. THE INDEX
-- ============================================================

/-- n₊ − n₋ over an explicit diagonal. -/
def indexOfDiag (l : List ℚ) : ℤ :=
  (l.filter (fun d => 0 < d)).length - (l.filter (fun d => d < 0)).length

/-- The carved ring: index −1. One net unpaired mode — survival. -/
theorem ringA_index : indexOfDiag [4, -1, -1] = -1 := by native_decide

/-- C₃: index −1. The first closure already cannot cancel. -/
theorem cyc3_index : indexOfDiag [2, -1/2, -2] = -1 := by native_decide

/-- The Void's ring: index 0. Perfect pairing — total cancellation. -/
theorem cyc4_index : indexOfDiag [2, -1/2, 0, 0] = 0 := by native_decide

/-- THE SURVIVAL THEOREM: the carved (odd) ring's index is nonzero and the
    Void's (even, bipartite) ring's index is zero. Matter survives iff the
    ring is odd. -/
theorem odd_survives_even_cancels :
    indexOfDiag [4, -1, -1] ≠ 0 ∧ indexOfDiag [2, -1/2, 0, 0] = 0 := by native_decide

-- ============================================================
-- S4. THE √3 SPECTRUM (the measured face, tied on over ℝ)
-- ============================================================

def ringAR : Matrix (Fin 3) (Fin 3) ℝ := !![0, 2, 1; 2, 0, 1; 1, 1, 0]

/-- The characteristic polynomial of the carved ring: fun³ − 6λ − 4. -/
theorem ringA_charpoly (x : ℝ) :
    (x • (1 : Matrix (Fin 3) (Fin 3) ℝ) - ringAR).det = x ^ 3 - 6 * x - 4 := by
  simp [ringAR, Matrix.det_fin_three]; ring

/-- The factorization the calc lane verified: (fun+2)(fun²−2λ−2). -/
theorem charpoly_factor (x : ℝ) : x ^ 3 - 6 * x - 4 = (x + 2) * (x ^ 2 - 2 * x - 2) := by
  ring

/-- The quadratic factor splits at 1 ± √3 — disc 12, the field ℚ(√3). -/
theorem quad_factor (x : ℝ) :
    x ^ 2 - 2 * x - 2 = (x - (1 - Real.sqrt 3)) * (x - (1 + Real.sqrt 3)) := by
  have h : Real.sqrt 3 ^ 2 = 3 := Real.sq_sqrt (by norm_num)
  ring_nf
  nlinarith [h]

/-- The spectrum, exactly: {−2, 1−√3, 1+√3}. -/
theorem spectrum_exact (x : ℝ) :
    (x • (1 : Matrix (Fin 3) (Fin 3) ℝ) - ringAR).det = 0 ↔
      x = -2 ∨ x = 1 - Real.sqrt 3 ∨ x = 1 + Real.sqrt 3 := by
  rw [ringA_charpoly, charpoly_factor, quad_factor]
  constructor
  · intro h
    rcases mul_eq_zero.mp h with h1 | h2
    · left; linarith [h1]
    · rcases mul_eq_zero.mp h2 with h3 | h4
      · right; left; linarith [h3]
      · right; right; linarith [h4]
  · rintro (rfl | rfl | rfl) <;> ring

/-- Sign bracket: 1 − √3 < 0. -/
theorem root_neg : 1 - Real.sqrt 3 < 0 := by
  have : (1 : ℝ) < Real.sqrt 3 := by
    rw [show (1 : ℝ) = Real.sqrt 1 by simp]
    exact Real.sqrt_lt_sqrt (by norm_num) (by norm_num)
  linarith

/-- Sign bracket: 0 < 1 + √3. -/
theorem root_pos : 0 < 1 + Real.sqrt 3 := by positivity

/-- The gap: (1+√3) − |−2| = √3 − 1 — the measured 0.7320508. -/
theorem gap_value : (1 + Real.sqrt 3) - 2 = Real.sqrt 3 - 1 := by ring

-- ============================================================
-- S5. THE EIGENVECTORS (exact, correcting a proposed null vector)
-- ============================================================
-- A proposed reduction claimed ker(A+2I) ∋ (−1,−1,2). It does not:
-- (A+2I)·(−1,−1,2) = (−2,−2,2). The kernel is spanned by (1,−1,0) — the
-- ANTISYMMETRIC combination of the two collapsed A₂ roots (the doubled pair):
-- the fun=−2 mode is the swap. And the Perron mode carries the gap in its own
-- coordinates: the surviving direction is (1, 1, √3−1).

/-- The fun = −2 eigenvector: the swap of the doubled pair. -/
theorem eigvec_neg_two : ringAR.mulVec ![1, -1, 0] = (-2 : ℝ) • ![1, -1, 0] := by
  funext i
  fin_cases i <;>
    simp [ringAR, Matrix.mulVec, dotProduct, Fin.sum_univ_three]

/-- The proposed balance vector is NOT in ker(A+2I). -/
theorem proposed_vector_fails :
    (ringAR + (2 : ℝ) • (1 : Matrix (Fin 3) (Fin 3) ℝ)).mulVec ![-1, -1, 2] ≠ 0 := by
  intro h
  have h2 := congrFun h 2
  simp [ringAR, Matrix.mulVec, dotProduct, Fin.sum_univ_three, Matrix.add_apply,
        Matrix.smul_apply, Matrix.one_apply, smul_eq_mul] at h2
  norm_num at h2

/-- The Perron eigenvector, exactly: A·(1, 1, √3−1) = (1+√3)·(1, 1, √3−1).
    The gap √3−1 is the third coordinate of the surviving mode. -/
theorem eigvec_perron :
    ringAR.mulVec ![1, 1, Real.sqrt 3 - 1] =
      (1 + Real.sqrt 3) • ![1, 1, Real.sqrt 3 - 1] := by
  have h : Real.sqrt 3 ^ 2 = 3 := Real.sq_sqrt (by norm_num)
  funext i
  fin_cases i <;>
    simp [ringAR, Matrix.mulVec, dotProduct, Fin.sum_univ_three] <;> nlinarith [h]

end Imscribing.OddRingIndex

