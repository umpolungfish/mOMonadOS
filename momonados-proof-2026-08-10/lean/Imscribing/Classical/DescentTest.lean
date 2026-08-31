import Imscribing.Classical.Solitary10
import Mathlib.Tactic
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.NumberTheory.ArithmeticFunction
open ArithmeticFunction

local notation "σ" => ArithmeticFunction.sigma (1 : ℕ)

lemma sigma_one_apply (n : ℕ) : σ n = ∑ d ∈ Nat.divisors n, d := by
  simp [ArithmeticFunction.sigma_apply]

lemma sigma_ge_self (n : ℕ) (hn : n > 0) : σ n ≥ n := by
  rw [sigma_one_apply]
  have h_self : n ∈ Nat.divisors n := Nat.mem_divisors.mpr ⟨dvd_refl n, hn.ne'⟩
  refine Finset.single_le_sum (fun d hd => Nat.zero_le d) h_self

lemma sigma_two_pow (e : ℕ) : σ (2 ^ e) = 2 ^ (e + 1) - 1 := by
  have hp : Nat.Prime 2 := by native_decide
  rw [ArithmeticFunction.sigma_one_apply_prime_pow hp]
  have hsum : (∑ k ∈ Finset.range (e + 1), (2 : ℕ) ^ k) = 2 ^ (e + 1) - 1 := by
    induction' e with k ih
    · norm_num
    · rw [Finset.sum_range_succ, ih, pow_succ]
      omega
  exact hsum

lemma factor_2_pow_odd (n : ℕ) (hn : n > 0) : ∃ (e : ℕ) (u : ℕ), u % 2 = 1 ∧ n = 2 ^ e * u := by
  induction' n using Nat.strong_induction_on with m ih
  by_cases h2m : 2 ∣ m
  · rcases h2m with ⟨t, hm_eq⟩
    have ht_lt_m : t < m := by
      rw [hm_eq]
      by_cases ht0 : t = 0
      · rw [ht0, mul_zero]; omega
      · omega
    have ht_pos : t > 0 := by
      by_contra! hle
      have ht0 : t = 0 := by omega
      rw [ht0, mul_zero] at hm_eq
      omega
    rcases ih t ht_lt_m ht_pos with ⟨e, u, hu, ht_eq⟩
    refine ⟨e + 1, u, hu, ?_⟩
    calc
      m = 2 * t := hm_eq
      _ = 2 * (2 ^ e * u) := by rw [ht_eq]
      _ = 2 ^ (e + 1) * u := by ring
  · rcases Nat.mod_two_eq_zero_or_one m with (h | h)
    · exfalso; apply h2m; exact Nat.dvd_of_mod_eq_zero h
    · refine ⟨0, m, h, by simp⟩

lemma sigma_mul_coprime {a b : ℕ} (h : Nat.Coprime a b) : σ (a * b) = σ a * σ b :=
  (ArithmeticFunction.isMultiplicative_sigma (k := 1)).map_mul_of_coprime h

lemma factor_from_eq (a q x : ℕ) (hcop : Nat.Coprime a q) (h : a * σ x = q * x) : a ∣ x := by
  have ha_qx : a ∣ q * x := by rw [← h]; exact ⟨σ x, by ring⟩
  exact hcop.dvd_of_dvd_mul_left ha_qx

lemma coprime_332_465 : Nat.Coprime 332 465 := by native_decide
lemma coprime_41872_51305 : Nat.Coprime 41872 51305 := by native_decide
lemma coprime_52_75 : Nat.Coprime 52 75 := by native_decide

lemma descent_generic (a b : ℕ) (ha_pos : a > 0) (hb_pos : b > 0) (ha_even : a % 2 = 0)
    (hcop : Nat.Coprime a b) (hineq : a * 3 > b * 2) (z : ℕ) (hz : z > 0) (h : a * σ z = b * z) : False := by
  rcases factor_2_pow_odd z hz with ⟨e, t, ht_odd, hz_eq⟩
  rw [hz_eq] at h
  have ht_pos : t > 0 := by
    by_contra! hle
    have ht0 : t = 0 := by omega
    rw [ht0, mul_zero] at hz_eq
    omega
  have h_cop : Nat.Coprime (2 ^ e) t := by
    have hp2 : Nat.Prime 2 := by native_decide
    have h2t : ¬ 2 ∣ t := by
      intro h2t
      have h2t_mod : t % 2 = 0 := Nat.mod_eq_zero_of_dvd h2t
      rw [ht_odd] at h2t_mod
      omega
    have h_cop_2_t : Nat.Coprime 2 t :=
      (Nat.Prime.coprime_iff_not_dvd hp2).mpr h2t
    exact h_cop_2_t.pow_left e
  have h_sigma_mul : σ (2 ^ e * t) = σ (2 ^ e) * σ t := sigma_mul_coprime h_cop
  rw [h_sigma_mul, sigma_two_pow e] at h
  by_cases he0 : e = 0
  · subst he0
    simp at hz_eq h
    have ha_t : a ∣ t := factor_from_eq a b t hcop (by omega)
    have h2_t : 2 ∣ t := by
      have h2_a : 2 ∣ a := by
        have hmod : a % 2 = 0 := ha_even
        exact Nat.dvd_of_mod_eq_zero hmod
      exact Nat.dvd_trans h2_a ha_t
    have h2t_mod : t % 2 = 0 := Nat.mod_eq_zero_of_dvd h2_t
    rw [ht_odd] at h2t_mod
    omega
  · have he_pos : e ≥ 1 := by omega
    have h_coeff : a * (2 ^ (e + 1) - 1) > b * 2 ^ e := by
      have h_pow_ineq : ∀ (k : ℕ), 1 ≤ k → a * (2 ^ (k + 1) - 1) > b * 2 ^ k := by
        intro k hk
        have hk1 : k = 1 ∨ 2 ≤ k := by omega
        rcases hk1 with (rfl | hk2)
        · simpa using hineq
        · have h_pow_bound : 2 ^ (k + 1) - 1 ≥ 3 * 2 ^ (k - 1) := by
            have h_pow_eq : 2 ^ (k + 1) = 4 * 2 ^ (k - 1) := by
              calc
                2 ^ (k + 1) = 2 ^ ((k - 1) + 2) := by rw [Nat.sub_add_cancel (by omega : 1 ≤ k)]
                _ = 2 ^ (k - 1) * 2 ^ 2 := by rw [pow_add]
                _ = 2 ^ (k - 1) * 4 := by norm_num
                _ = 4 * 2 ^ (k - 1) := by ring
            rw [h_pow_eq]
            omega
          have h_lhs : a * (2 ^ (k + 1) - 1) ≥ a * (3 * 2 ^ (k - 1)) :=
            Nat.mul_le_mul_left a h_pow_bound
          have h_rhs : b * 2 ^ k = (b * 2) * 2 ^ (k - 1) := by
            calc
              b * 2 ^ k = b * 2 ^ ((k - 1) + 1) := by rw [Nat.sub_add_cancel (by omega : 1 ≤ k)]
              _ = b * (2 ^ (k - 1) * 2) := by rw [pow_succ]
              _ = (b * 2) * 2 ^ (k - 1) := by ring
          rw [h_rhs]
          have h_ineq : a * (3 * 2 ^ (k - 1)) > (b * 2) * 2 ^ (k - 1) := by
            have h_3a_gt_2b : a * 3 > b * 2 := hineq
            nlinarith
          exact lt_of_lt_of_le h_ineq h_lhs
      exact h_pow_ineq e he_pos
    have h_sigma_t_ge_t : σ t ≥ t := sigma_ge_self t ht_pos
    have h_contra : (a * (2 ^ (e + 1) - 1)) * σ t > (b * 2 ^ e) * t := by
      calc
        (a * (2 ^ (e + 1) - 1)) * σ t ≥ (a * (2 ^ (e + 1) - 1)) * t :=
          Nat.mul_le_mul_left (a * (2 ^ (e + 1) - 1)) h_sigma_t_ge_t
        _ > (b * 2 ^ e) * t := Nat.mul_lt_mul_of_pos_right h_coeff ht_pos
    have h_eq : (a * (2 ^ (e + 1) - 1)) * σ t = (b * 2 ^ e) * t := by
      calc
        (a * (2 ^ (e + 1) - 1)) * σ t = a * ((2 ^ (e + 1) - 1) * σ t) := by ring
        _ = b * (2 ^ e * t) := h
        _ = (b * 2 ^ e) * t := by ring
    rw [h_eq] at h_contra
    exact lt_irrefl _ h_contra

lemma descent_332_465 (z : ℕ) (hz : z > 0) (h : 332 * σ z = 465 * z) : False :=
  descent_generic 332 465 (by norm_num) (by norm_num) (by norm_num) coprime_332_465 (by norm_num) z hz h

lemma descent_41872_51305 (z : ℕ) (hz : z > 0) (h : 41872 * σ z = 51305 * z) : False :=
  descent_generic 41872 51305 (by norm_num) (by norm_num) (by norm_num) coprime_41872_51305 (by norm_num) z hz h

lemma descent_52_75 (z : ℕ) (hz : z > 0) (h : 52 * σ z = 75 * z) : False :=
  descent_generic 52 75 (by norm_num) (by norm_num) (by norm_num) coprime_52_75 (by norm_num) z hz h
