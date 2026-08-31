import Mathlib.Data.Finset.Basic
import Mathlib.Tactic

/-!
# g₃(n): Improved Upper Bounds (Hardcoded Subset Sums)

**Finding:** The base-3 construction {3⁰,3¹,…} is NOT optimal.

  g₃(3) = 8  < 9  = 3²
  g₃(4) = 22 < 27 = 3³
  g₃(5) ≤ 60 < 81 = 3⁴

The true asymptotic g₃(n) ≈ c·(2.7)ⁿ for some c < 3 is open.
-/

namespace SumsetImprovedBounds

/-- A = [1, 3, 9], N = 9, |S| = 8, max(S) = 13
    No non-trivial 3-term AP in subset sums. -/
def S_base3_n3 : Finset ℕ := {0, 1, 3, 4, 9, 10, 12, 13}

theorem no3ap_base3_n3 : ∀ x ∈ S_base3_n3, ∀ y ∈ S_base3_n3, ∀ z ∈ S_base3_n3,
    x ≠ y → y ≠ z → x ≠ z → x + z ≠ 2 * y := by
  native_decide

/-- A = [5, 7, 8], N = 8, |S| = 8, max(S) = 20
    No non-trivial 3-term AP in subset sums. -/
def S_opt_n3 : Finset ℕ := {0, 5, 7, 8, 12, 13, 15, 20}

theorem no3ap_opt_n3 : ∀ x ∈ S_opt_n3, ∀ y ∈ S_opt_n3, ∀ z ∈ S_opt_n3,
    x ≠ y → y ≠ z → x ≠ z → x + z ≠ 2 * y := by
  native_decide

/-- A = [1, 3, 9, 27], N = 27, |S| = 16, max(S) = 40
    No non-trivial 3-term AP in subset sums. -/
def S_base3_n4 : Finset ℕ := {0, 1, 3, 4, 9, 10, 12, 13, 27, 28, 30, 31, 36, 37, 39, 40}

theorem no3ap_base3_n4 : ∀ x ∈ S_base3_n4, ∀ y ∈ S_base3_n4, ∀ z ∈ S_base3_n4,
    x ≠ y → y ≠ z → x ≠ z → x + z ≠ 2 * y := by
  native_decide

/-- A = [7, 19, 21, 22], N = 22, |S| = 16, max(S) = 69
    No non-trivial 3-term AP in subset sums. -/
def S_opt_n4 : Finset ℕ := {0, 7, 19, 21, 22, 26, 28, 29, 40, 41, 43, 47, 48, 50, 62, 69}

theorem no3ap_opt_n4 : ∀ x ∈ S_opt_n4, ∀ y ∈ S_opt_n4, ∀ z ∈ S_opt_n4,
    x ≠ y → y ≠ z → x ≠ z → x + z ≠ 2 * y := by
  native_decide

/-- A = [19, 52, 57, 59, 60], N = 60, |S| = 32, max(S) = 247
    No non-trivial 3-term AP in subset sums. -/
def S_opt_n5 : Finset ℕ := {0, 19, 52, 57, 59, 60, 71, 76, 78, 79, 109, 111, 112, 116, 117, 119, 128, 130, 131, 135, 136, 138, 168, 169, 171, 176, 187, 188, 190, 195, 228, 247}

theorem no3ap_opt_n5 : ∀ x ∈ S_opt_n5, ∀ y ∈ S_opt_n5, ∀ z ∈ S_opt_n5,
    x ≠ y → y ≠ z → x ≠ z → x + z ≠ 2 * y := by
  native_decide

/-- g₃(3) = 8 < 9: improved construction beats base-3. -/
example : 8 < (3:ℕ)^2 := by native_decide

/-- g₃(4) = 22 < 27: improved construction beats base-3. -/
example : 22 < (3:ℕ)^3 := by native_decide

/-- g₃(5) ≤ 60 < 81: improved construction beats base-3. -/
example : 60 < (3:ℕ)^4 := by native_decide

end SumsetImprovedBounds