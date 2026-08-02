import Mathlib.Tactic
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.NumberTheory.ArithmeticFunction
open ArithmeticFunction

set_option linter.style.nativeDecide false
namespace Imscribing.Classical.Solitary10

local notation "σ" => ArithmeticFunction.sigma (1 : ℕ)

lemma sigma_one_apply (n : ℕ) : σ n = ∑ d ∈ Nat.divisors n, d := by
  simp [ArithmeticFunction.sigma_apply]

lemma sigma_ge_self (n : ℕ) (hn : n > 0) : σ n ≥ n := by
  rw [sigma_one_apply]
  have h_self : n ∈ Nat.divisors n := Nat.mem_divisors.mpr ⟨dvd_refl n, hn.ne'⟩
  refine Finset.single_le_sum (fun d hd => Nat.zero_le d) h_self

lemma sigma_mul_coprime {a b : ℕ} (h : Nat.Coprime a b) : σ (a * b) = σ a * σ b :=
  (ArithmeticFunction.isMultiplicative_sigma (k := 1)).map_mul_of_coprime h

lemma geom_two (e : ℕ) : (∑ k ∈ Finset.range (e + 1), (2 : ℕ) ^ k) = 2 ^ (e + 1) - 1 := by
  induction' e with k ih
  · norm_num
  · rw [Finset.sum_range_succ, ih, pow_succ]
    omega

lemma sigma_two_pow (e : ℕ) : σ (2 ^ e) = 2 ^ (e + 1) - 1 := by
  have hp : Nat.Prime 2 := by native_decide
  rw [ArithmeticFunction.sigma_one_apply_prime_pow hp]
  exact geom_two e
