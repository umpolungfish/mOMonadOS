-- Imscribing/Millennium/Z7StructureConstants.lean
-- Six-Coefficient Frobenius Algebra — Structure Constants C_{ijk} ∈ {0,1}
-- Algebra: A ≅ ℝ[ℤ₇], basis {b₀,...,b₆} with relabeled ℤ₇ indices
-- Author: Math⊙perator (Lando⊗⊙perator team)
-- Date: 2026-07-29
--
-- Z₂ involution (group inversion in relabeled basis):
--   0↔0, 1↔6, 2↔4, 3↔5, 4↔2, 5↔3, 6↔1
-- Verified: ∀i, mul(i, inv(i)) = 0 (identity)

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Mathlib

namespace Millennium.Z7StructureConstants

open Imscribing.Primitives


-- ============================================================
-- §1. RELABELING
-- ============================================================

def relabel (i : ℕ) : ℕ :=
  match i with
  | 0 => 0 | 1 => 1 | 2 => 2 | 3 => 3
  | 4 => 5 | 5 => 4 | 6 => 6
  | _ => 0

def invRelabel (i : ℕ) : ℕ :=
  match i with
  | 0 => 0 | 1 => 1 | 2 => 2 | 3 => 3
  | 5 => 4 | 6 => 6 | 4 => 5
  | _ => 0

theorem invRelabel_relabel : ∀ i ∈ List.range 7, invRelabel (relabel i) = i := by
  decide

-- ============================================================
-- §2. Z₂ INVOLUTION (GROUP INVERSION IN RELABELED BASIS)
-- ============================================================

def z2Involution (i : ℕ) : ℕ :=
  match i with
  | 0 => 0 | 1 => 6 | 2 => 4 | 3 => 5
  | 4 => 2 | 5 => 3 | 6 => 1
  | _ => 0

theorem z2_involutive : ∀ i ∈ List.range 7, z2Involution (z2Involution i) = i := by
  decide

-- ============================================================
-- §3. STRUCTURE CONSTANT AND MULTIPLICATION
-- ============================================================

def C (i j k : ℕ) : ℕ :=
  if (relabel i + relabel j) % 7 = relabel k then 1 else 0

def mul (i j : ℕ) : ℕ :=
  invRelabel ((relabel i + relabel j) % 7)

-- ============================================================
-- §4. MULTIPLICATION TABLE (COMPUTATIONALLY VERIFIED)
-- ============================================================

-- Identity laws
theorem b0_left_unit  : ∀ i ∈ List.range 7, mul 0 i = i := by decide
theorem b0_right_unit : ∀ i ∈ List.range 7, mul i 0 = i := by decide

-- Squarings (computed: b₀²=0, b₁²=2, b₂²=5, b₃²=6, b₄²=3, b₅²=1, b₆²=4)
theorem b0_squared : mul 0 0 = 0 := by decide
theorem b1_squared : mul 1 1 = 2 := by decide
theorem b2_squared : mul 2 2 = 5 := by decide
theorem b3_squared : mul 3 3 = 6 := by decide
theorem b4_squared : mul 4 4 = 3 := by decide
theorem b5_squared : mul 5 5 = 1 := by decide
theorem b6_squared : mul 6 6 = 4 := by decide

-- Full multiplication rows (computed from relabeled ℤ₇ group law)
theorem row_0 : List.map (mul 0) (List.range 7) = [0,1,2,3,4,5,6] := by decide
theorem row_1 : List.map (mul 1) (List.range 7) = [1,2,3,5,6,4,0] := by decide
theorem row_2 : List.map (mul 2) (List.range 7) = [2,3,5,4,0,6,1] := by decide
theorem row_3 : List.map (mul 3) (List.range 7) = [3,5,4,6,1,0,2] := by decide
theorem row_4 : List.map (mul 4) (List.range 7) = [4,6,0,1,3,2,5] := by decide
theorem row_5 : List.map (mul 5) (List.range 7) = [5,4,6,0,2,1,3] := by decide
theorem row_6 : List.map (mul 6) (List.range 7) = [6,0,1,2,5,3,4] := by decide

-- ============================================================
-- §5. Z₂ INVOLUTION × MULTIPLICATION CONSISTENCY
-- ============================================================

/-- ∀i < 7: b_i · b_{inv(i)} = b_0.
    Verified: mul(i, z2Involution(i)) = 0 for all i ∈ {0,...,6}. -/
theorem mul_z2_identity : ∀ i ∈ List.range 7, mul i (z2Involution i) = 0 := by
  decide

-- ============================================================
-- §6. FROBENIUS STRUCTURE
-- ============================================================

def deltaIdentity : List (ℕ × ℕ) :=
  (List.range 7).map (fun i => (i, z2Involution i))

theorem delta_pairs : deltaIdentity = [(0,0),(1,6),(2,4),(3,5),(4,2),(5,3),(6,1)] := by
  decide

theorem frobenius_trace_sum_zero :
    (List.map (fun i => mul i (z2Involution i)) (List.range 7)).sum = (0 : ℕ) := by
  decide

-- ============================================================
-- §7. FOURIER COEFFICIENTS
-- ============================================================

noncomputable def a_coeff (n : ℕ) : ℝ :=
  match n with
  | 0 => 1
  | 1 => 1 / ((1 + Real.sqrt 5) / 2) ^ 2
  | 2 => Real.sqrt 3 / 2
  | 3 => 4
  | 4 => Real.arctan (1/4)
  | 5 => 1/13
  | 6 => 2 * Real.pi
  | _ => 0

theorem a0_is_one   : a_coeff 0 = (1 : ℝ) := by simp [a_coeff]
theorem a3_is_four  : a_coeff 3 = (4 : ℝ) := by simp [a_coeff]
theorem a6_is_two_pi : a_coeff 6 = (2 : ℝ) * Real.pi := by simp [a_coeff]

#print axioms mul_z2_identity
#print axioms z2_involutive
#print axioms invRelabel_relabel

end Millennium.Z7StructureConstants
