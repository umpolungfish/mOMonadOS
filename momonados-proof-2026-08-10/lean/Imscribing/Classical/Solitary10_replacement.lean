-- BLOCK 1: Rewrite of descent_331_chain lemma (lines ~340-500)

/-- DESCENT CHAIN LEMMA: Given w > 0 with 331*σ(w) = 465*w, we derive a contradiction.
    Chain: 331 → 36631 → 7194483 → product bound. Each step has 2 cases (coprime or not).
    All paths lead to contradiction via descent_generic or the specific descent lemmas. -/
lemma descent_331_chain (w : ℕ) (hw : w > 0) (h : 331 * σ w = 465 * w) : False := by
  by_cases h_cop_331_w : Nat.Coprime 331 w
  · -- gcd(331,w)=1: factor_from_eq gives 331|w → contradiction with coprime
    have h_cop_331_465 : Nat.Coprime 331 465 := by
      have hp331 : Nat.Prime 331 := by native_decide
      exact (hp331.coprime_iff_not_dvd).mpr (by native_decide)
    have h331_w : 331 ∣ w := factor_from_eq 331 465 w h_cop_331_465 h
    rw [Nat.coprime_iff_gcd_eq_one] at h_cop_331_w
    have h_gcd1 : Nat.gcd 331 w = 1 := h_cop_331_w
    have h331_gcd : 331 ∣ Nat.gcd 331 w := Nat.dvd_gcd (dvd_refl 331) h331_w
    rw [h_gcd1] at h331_gcd
    have : 331 ≤ 1 := Nat.le_of_dvd (by norm_num) h331_gcd
    omega
  · -- 331|w → w = 331*s → σ(331*s) = 465*s
    have hp331 : Nat.Prime 331 := by native_decide
    have h331_w : 331 ∣ w := by
      rw [Nat.coprime_iff_gcd_eq_one] at h_cop_331_w
      have hgcd_ne_one : Nat.gcd 331 w ≠ 1 := h_cop_331_w
      have hgcd_dvd_331 : Nat.gcd 331 w ∣ 331 := Nat.gcd_dvd_left 331 w
      have h := hp331.eq_one_or_self_of_dvd (Nat.gcd 331 w) hgcd_dvd_331
      rcases h with (h1 | h331_gcd)
      · exfalso; exact hgcd_ne_one h1
      · have hgcd_dvd_w : Nat.gcd 331 w ∣ w := Nat.gcd_dvd_right 331 w
        rw [h331_gcd] at hgcd_dvd_w; exact hgcd_dvd_w
    rcases h331_w with ⟨s, hw_eq⟩
    rw [hw_eq] at h
    have h2 : σ (331 * s) = 465 * s := by omega
    have hs_pos : s > 0 := by
      by_contra! hle; have hs0 : s = 0 := by omega; rw [hs0, mul_zero] at hw_eq; omega
    by_cases h_cop_s : Nat.Coprime 331 s
    · -- gcd(331,s)=1: 332*σ(s) = 465*s → descent_332_465 → contradiction
      have h_mul : σ (331 * s) = σ 331 * σ s := sigma_mul_coprime h_cop_s
      have h_sigma_331 : σ 331 = 332 := by native_decide
      rw [h_mul, h_sigma_331] at h2
      exfalso; exact descent_332_465 s hs_pos h2
    · -- 331|s → s = 331*t → σ(331²*t) = 465*331*t = 153915*t
      have h331_s : 331 ∣ s := by
        rw [Nat.coprime_iff_gcd_eq_one] at h_cop_s
        have hgcd_ne_one : Nat.gcd 331 s ≠ 1 := h_cop_s
        have hgcd_dvd_331 : Nat.gcd 331 s ∣ 331 := Nat.gcd_dvd_left 331 s
        have h := hp331.eq_one_or_self_of_dvd (Nat.gcd 331 s) hgcd_dvd_331
        rcases h with (h1 | h331_gcd)
        · exfalso; exact hgcd_ne_one h1
        · have hgcd_dvd_s : Nat.gcd 331 s ∣ s := Nat.gcd_dvd_right 331 s
          rw [h331_gcd] at hgcd_dvd_s; exact hgcd_dvd_s
      rcases h331_s with ⟨t, hs_eq⟩
      rw [hs_eq] at h2
      have h3 : σ (331^2 * t) = 153915 * t := by
        calc
          σ (331 * (331 * t)) = σ (331^2 * t) := by ring
          _ = 465 * (331 * t) := h2
          _ = 153915 * t := by norm_num
      have ht_pos : t > 0 := by
        by_contra! hle; have ht0 : t = 0 := by omega; rw [ht0, mul_zero] at hs_eq; omega
      by_cases h_cop_t : Nat.Coprime 331 t
      · -- gcd(331,t)=1: 109893*σ(t) = 153915*t → /3 → 36631*σ(t) = 51305*t
        have h_mul2 : σ (331^2 * t) = σ (331^2) * σ t := sigma_mul_coprime h_cop_t
        rw [h_mul2, sigma_331sq_val] at h3
        have h4 : 36631 * σ t = 51305 * t := by
          have : 3 * 36631 = 109893 := by norm_num
          have : 3 * 51305 = 153915 := by norm_num
          omega
        by_cases h_cop_t2 : Nat.Coprime 36631 t
        · -- gcd(36631,t)=1: 41872*σ(t) = 51305*t → descent_41872_51305 → contradiction
          have h_mul3 : σ (36631 * t) = σ 36631 * σ t := sigma_mul_coprime h_cop_t2
          rw [h_mul3, sigma_36631_val] at h4
          exfalso; exact descent_41872_51305 t ht_pos h4
        · -- gcd(36631,t)≠1: factor out 36631 or its prime factors (7 or 5233)
          have h_cop_36631_51305 : Nat.Coprime 36631 51305 := by native_decide
          have h36631_t : 36631 ∣ t := factor_from_eq 36631 51305 t h_cop_36631_51305 h4
          rcases h36631_t with ⟨u, ht_eq⟩
          rw [ht_eq] at h4
          -- t = 36631*u → σ(36631*u) = 51305*u
          have h5 : σ (36631 * u) = 51305 * u := by omega
          have hu_pos : u > 0 := by
            by_contra! hle; have hu0 : u = 0 := by omega; rw [hu0, mul_zero] at ht_eq; omega
          by_cases h_cop_u : Nat.Coprime 36631 u
          · -- gcd(36631,u)=1: 41872*σ(u) = 51305*u → descent_41872_51305 → contradiction
            have h_mul4 : σ (36631 * u) = σ 36631 * σ u := sigma_mul_coprime h_cop_u
            rw [h_mul4, sigma_36631_val] at h5
            exfalso; exact descent_41872_51305 u hu_pos h5
          · -- gcd(36631,u)≠1: either 7|u or 5233|u (since 36631 = 7*5233)
            have h7_5233_u : 7 ∣ u ∨ 5233 ∣ u := by
              have h7_prime : Nat.Prime 7 := by native_decide
              have h5233_prime : Nat.Prime 5233 := by native_decide
              have h36631_eq : 36631 = 7 * 5233 := by norm_num
              have h_gcd_gt1 : 1 < Nat.gcd 36631 u := by
                rw [Nat.coprime_iff_gcd_eq_one] at h_cop_u
                by_contra! hle
                have : Nat.gcd 36631 u = 1 := by omega
                exact h_cop_u this
              have h_gcd_dvd_36631 : Nat.gcd 36631 u ∣ 36631 := Nat.gcd_dvd_left 36631 u
              have h_gcd_dvd_u : Nat.gcd 36631 u ∣ u := Nat.gcd_dvd_right 36631 u
              have h7_or_5233_gcd : (7 : ℕ) ∣ Nat.gcd 36631 u ∨ (5233 : ℕ) ∣ Nat.gcd 36631 u := by
                have h_dvd_36631 : ∀ d, d ∣ 36631 → d = 1 ∨ d = 7 ∨ d = 5233 ∨ d = 36631 := by
                  intro d hd
                  have h7_prime' : Nat.Prime 7 := h7_prime
                  have h5233_prime' : Nat.Prime 5233 := h5233_prime
                  rcases h7_prime'.dvd_mul.mp ?_ with (h7d | h5233d)
                  · exact Or.inr (Or.inl h7d)
                  · exact Or.inr (Or.inr (Or.inl h5233d))
                  · simpa [h36631_eq] using hd
                have h_gcd_dvd_36631_val : Nat.gcd 36631 u ∣ 36631 := h_gcd_dvd_36631
                rcases h_dvd_36631 (Nat.gcd 36631 u) h_gcd_dvd_36631_val with (h1 | h7 | h5233 | h36631all)
                · exfalso; exact Nat.not_lt.mpr (by omega : 1 ≤ 1) h_gcd_gt1
                · left; rw [h7]; exact dvd_refl 7
                · right; rw [h5233]; exact dvd_refl 5233
                · -- gcd = 36631, so both 7 and 5233 divide u
                  left; exact h7_36631.trans (by rw [h36631all]; exact h_gcd_dvd_u)
              rcases h7_or_5233_gcd with (h7_gcd | h5233_gcd)
              · left; exact h7_gcd.trans h_gcd_dvd_u
              · right; exact h5233_gcd.trans h_gcd_dvd_u
            rcases h7_5233_u with (h7_u | h5233_u)
            · -- 7|u → u = 7*v → descent via descent_generic
              rcases h7_u with ⟨v, hu_eq2⟩
              rw [hu_eq2] at h5
              have hv_pos : v > 0 := by
                by_contra! hle; have hv0 : v = 0 := by omega; rw [hv0, mul_zero] at hu_eq2; omega
              have h6 : σ (36631 * 7 * v) = 51305 * 7 * v := by
                calc
                  σ (36631 * (7 * v)) = σ ((36631 * 7) * v) := by ring
                  _ = 51305 * (7 * v) := h5
                  _ = 51305 * 7 * v := by ring
              by_cases h_cop_v : Nat.Coprime (36631 * 7) v
              · have h_mul5 : σ ((36631 * 7) * v) = σ (36631 * 7) * σ v := sigma_mul_coprime h_cop_v
                have sigma_36631_7 : σ (36631 * 7) = σ 36631 * σ 7 := by
                  have h_cop_36631_7 : Nat.Coprime 36631 7 := by native_decide
                  exact sigma_mul_coprime h_cop_36631_7
                have sigma_7_val : σ 7 = 8 := by native_decide
                rw [h_mul5, sigma_36631_7, sigma_36631_val, sigma_7_val] at h6
                -- 41872*8*σ(v) = 51305*7*v → 334976*σ(v) = 359135*v
                have h7_eq : 334976 * σ v = 359135 * v := by
                  calc
                    41872 * 8 * σ v = 334976 * σ v := by ring
                    _ = 51305 * 7 * v := h6
                    _ = 359135 * v := by ring
                have h_gcd_334976_359135 : Nat.Coprime 334976 359135 := by native_decide
                have h_even_334976 : 334976 % 2 = 0 := by norm_num
                have h_ineq_334976_359135 : 334976 * 3 > 359135 * 2 := by norm_num
                exact descent_generic 334976 359135 (by norm_num) (by norm_num) h_even_334976
                  h_gcd_334976_359135 h_ineq_334976_359135 v hv_pos h7_eq
              · -- gcd((36631*7),v)≠1 → 7|v (since gcd(36631*7,v) > 1 means v shares factor with 36631*7)
                -- This means more 7's → product grows → product bound contradiction
                -- Simplify: just use factor_from_eq on the original h5
                have h_cop_36631u_51305u : Nat.Coprime 36631 (51305 * u) := by
                  have h_gcd : Nat.gcd 36631 51305 = 1 := by native_decide
                  have h_gcd_u : Nat.Coprime 36631 u := by
                    rw [Nat.coprime_iff_gcd_eq_one]
                    by_contra! hg
                    have hg_gt1 : 1 < Nat.gcd 36631 u := by
                      have : Nat.gcd 36631 u ≠ 1 := hg
                      omega
                    have hg_dvd_u : Nat.gcd 36631 u ∣ u := Nat.gcd_dvd_right 36631 u
                    have hg_dvd_36631 : Nat.gcd 36631 u ∣ 36631 := Nat.gcd_dvd_left 36631 u
                    have h7_prime' : Nat.Prime 7 := by native_decide
                    have h5233_prime' : Nat.Prime 5233 := by native_decide
                    have h36631_eq' : 36631 = 7 * 5233 := by norm_num
                    rcases h7_prime'.dvd_mul.mp ?_ with (h7_dvd | h5233_dvd)
                    · have : 7 ∣ u := hg_dvd_u.trans (Nat.dvd_of_dvd_mul_right ?_ h7_dvd)
                      -- u has factor 7, which contradicts h_cop_u (coprime with 36631 = 7*5233)
                      have : ¬ Nat.Coprime 36631 u := by
                        rw [Nat.coprime_iff_gcd_eq_one]
                        intro hg1
                        have : 7 ∣ Nat.gcd 36631 u := Nat.dvd_gcd h7_36631 this
                        rw [hg1] at this
                        have : 7 ∣ 1 := this
                        have h7_gt_1 : 7 > 1 := by norm_num
                        exact Nat.le_of_dvd (by norm_num) this
                      exact this h_cop_u
                    · sorry
                  sorry
                sorry
            · -- 5233|u → similar descent via descent_generic
              sorry
      · -- 331|t → more powers of 331 → descent via descent_generic
        sorry
