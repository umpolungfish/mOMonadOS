/-- From 7*σ(z) = 5*z with z>0, derive a contradiction.
    Factor the 7-power from z and use 7*σ(7^k) > 5*7^k for all k≥1. -/
lemma descent_7_5 (z : ℕ) (hz_pos : z > 0) (h_eq : 7 * σ z = 5 * z) : False := by
  have hp7 : Nat.Prime 7 := by native_decide
  have h_cop_7_5 : Nat.Coprime 7 5 := by native_decide
  have h7z : 7 ∣ z := by
    have h7_dvd_5z : 7 ∣ 5 * z := by
      rw [← h_eq]; exact ⟨σ z, by ring⟩
    exact h_cop_7_5.dvd_of_dvd_mul_left h7_dvd_5z
  rcases factor_p_pow 7 z hp7 hz_pos with ⟨k, t, h7t, hz_factor⟩
  have hk_ge1 : k ≥ 1 := by
    by_contra! hk0
    have hk0' : k = 0 := by omega
    rw [hk0', pow_zero, one_mul] at hz_factor
    have : ¬ 7 ∣ z := by rw [hz_factor]; exact h7t
    exact this h7z
  have ht_pos : t > 0 := by
    by_contra! ht0; rw [ht0] at hz_factor; simp at hz_factor; omega
  have h_cop : Nat.Coprime (7 ^ k) t :=
    ((Nat.Prime.coprime_iff_not_dvd hp7).mpr h7t).pow_left k
  rw [hz_factor] at h_eq
  have h_sigma : σ (7 ^ k * t) = σ (7 ^ k) * σ t := sigma_mul_coprime h_cop
  rw [h_sigma] at h_eq
  have h_sigma_ge_t : σ t ≥ t := sigma_ge_self t ht_pos
  -- Key inequality: 7 * σ(7^k) > 5 * 7^k for all k≥1
  -- σ(7^k) = (7^(k+1)-1)/6, so 7*σ(7^k) = 7*(7^(k+1)-1)/6 = (49*7^k-7)/6
  -- 5*7^k = (30*7^k)/6. Since 49*7^k - 7 > 30*7^k ↔ 19*7^k > 7, which is true for all k.
  have h_key : 7 * σ (7 ^ k) > 5 * 7 ^ k := by
    have h_sigma_formula : 6 * σ (7 ^ k) = 7 ^ (k + 1) - 1 := by
      have := sigma_one_apply_prime_pow hp7 (i := k)
      rw [this]
      omega
    have h_19pow : 19 * 7 ^ k > 7 := by
      have h_pow_ge1 : 7 ^ k ≥ 1 := pow_pos (by norm_num) k
      nlinarith
    have h_calc : 7 * (6 * σ (7 ^ k)) > 5 * 7 ^ k * 6 := by
      rw [h_sigma_formula]
      have : 7 * (7 ^ (k + 1) - 1) = 49 * 7 ^ k - 7 := by
        rw [pow_succ]
        ring
      rw [this]
      have : 5 * 7 ^ k * 6 = 30 * 7 ^ k := by ring
      rw [this]
      nlinarith
    have h_pos : 6 * σ (7 ^ k) > 0 := by
      have : σ (7 ^ k) ≥ 1 := by
        apply sigma_ge_self (7 ^ k)
        exact pow_pos (by norm_num) k
      omega
    omega
  nlinarith
