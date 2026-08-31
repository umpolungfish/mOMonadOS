import Mathlib.Tactic
import Mathlib.Data.Nat.Factorization.Basic
import Mathlib.NumberTheory.ArithmeticFunction.Defs
import Mathlib.NumberTheory.ArithmeticFunction.Misc
open Nat ArithmeticFunction

noncomputable def v₂ (n : ℕ) : ℕ := (Nat.factorization n) 2

lemma sigma_prime_pow_formula (p k : ℕ) (hp : Nat.Prime p) :
    sigma 1 (p ^ k) = ∑ i ∈ Finset.range (k+1), p ^ i := by
  rw [sigma_apply, Nat.divisors_prime_pow hp, Finset.sum_map]; simp

lemma pow_mod2_one (p : ℕ) (hp_odd : p % 2 = 1) (i : ℕ) : p ^ i % 2 = 1 := by
  induction i with
  | zero => simp
  | succ n ih => simp [pow_succ, Nat.mul_mod, hp_odd, ih]

lemma sigma_mod2_odd_prime (p a : ℕ) (hp : p.Prime) (hp_ne2 : p ≠ 2) :
    sigma 1 (p ^ a) % 2 = (a + 1) % 2 := by
  have hp_odd : p % 2 = 1 := by
    have hp_not_even : ¬ 2 ∣ p := by
      intro h; apply hp_ne2
      have h2prime : (2 : ℕ).Prime := by norm_num
      have := hp.eq_one_or_self_of_dvd 2 h
      rcases this with (h2 | h2)
      · norm_num at h2
      · exact h2.symm
    have hm := Nat.mod_two_eq_zero_or_one p
    rcases hm with (h | h)
    · exfalso; apply hp_not_even; exact ⟨p / 2, by omega⟩
    · exact h
  induction' a with n ih
  · simp [sigma_prime_pow_formula p 0 hp]
  · have h_next : sigma 1 (p ^ (n+1)) = ∑ i ∈ Finset.range (n+2), p ^ i :=
      sigma_prime_pow_formula p (n+1) hp
    have h_prev : sigma 1 (p ^ n) = ∑ i ∈ Finset.range (n+1), p ^ i :=
      sigma_prime_pow_formula p n hp
    rw [h_next, Finset.sum_range_succ, Nat.add_mod, ← h_prev, ih, pow_mod2_one p hp_odd (n+1)]
    simp

lemma prod_odd (s : Finset ℕ) (f : ℕ → ℕ) (h : ∀ x ∈ s, f x % 2 = 1) :
    (∏ x ∈ s, f x) % 2 = 1 := by
  classical
  refine Finset.induction_on s ?_ ?_
  · simp
  · intro a s' has ih
    have ha : f a % 2 = 1 := h a (Finset.mem_insert_self a s')
    have hs' : ∀ x ∈ s', f x % 2 = 1 := fun x hx => h x (Finset.mem_insert_of_mem hx)
    rw [Finset.prod_insert has, Nat.mul_mod, ha, ih hs']; simp
lemma sigma_factorization_prod (n : ℕ) (hn : n ≠ 0) :
    sigma 1 n = (∏ p ∈ (factorization n).support, sigma 1 (p ^ ((factorization n) p))) := by
  let S := (factorization n).support
  have h_n_prod : (∏ p ∈ S, p ^ ((factorization n) p)) = n := by
    calc
      (∏ p ∈ S, p ^ ((factorization n) p)) = ((factorization n).prod fun p a => p ^ a) := by
        rw [Finsupp.prod_of_support_subset (factorization n) (Finset.Subset.refl _) (fun p a => p ^ a) (by simp)]
        simp
      _ = n := Nat.factorization_prod_pow_eq_self hn
  have coprime_prime_pow (p q a b : ℕ) (hp : p.Prime) (hq : q.Prime) (hne : p ≠ q) :
      Nat.Coprime (p ^ a) (q ^ b) := by
    apply Nat.Coprime.pow a b
    apply hp.coprime_iff_not_dvd.mpr
    intro hdiv; apply hne
    have h_cases : p = 1 ∨ p = q := hq.eq_one_or_self_of_dvd p hdiv
    rcases h_cases with (h | h)
    · have : 1 < p := hp.one_lt; omega
    · exact h
  have h_pairwise : (S : Set ℕ).Pairwise (fun p q => Nat.Coprime
      (p ^ ((factorization n) p)) (q ^ ((factorization n) q))) := by
    intro p hp q hq hne
    have hp_mem : p ∈ n.primeFactors := by
      have : S = n.primeFactors := rfl
      simpa [this] using hp
    have hq_mem : q ∈ n.primeFactors := by
      have : S = n.primeFactors := rfl
      simpa [this] using hq
    have hp_prime : p.Prime := (Nat.mem_primeFactors.mp hp_mem).1
    have hq_prime : q.Prime := (Nat.mem_primeFactors.mp hq_mem).1
    exact coprime_prime_pow p q ((factorization n) p) ((factorization n) q) hp_prime hq_prime hne
  calc
    sigma 1 n = sigma 1 (∏ p ∈ S, p ^ ((factorization n) p)) := by rw [h_n_prod]
    _ = (∏ p ∈ S, sigma 1 (p ^ ((factorization n) p))) :=
      (isMultiplicative_sigma (k := 1)).map_prod (fun p => p ^ ((factorization n) p)) S h_pairwise

theorem euler_opn_form (n : ℕ) (h_odd : ¬ 2 ∣ n) (h_perf : Perfect n) :
    ∃ (p k m : ℕ),
      Nat.Prime p ∧ n = p ^ k * m ^ 2 ∧ p % 4 = 1 ∧ k % 4 = 1 ∧ ¬ p ∣ m := by
  have hn_pos : n ≠ 0 := by
    intro hzero; apply h_odd; rw [hzero]; exact ⟨0, by simp⟩
  have h_sigma_eq : sigma 1 n = 2 * n := h_perf
  have h_sigma_mod4 : sigma 1 n % 4 = 2 := by
    rw [h_sigma_eq]
    have hn_mod2 : n % 2 = 1 := by
      have h := Nat.mod_two_eq_zero_or_one n
      rcases h with (h | h)
      · exfalso; apply h_odd; exact ⟨n / 2, by omega⟩
      · exact h
    have hn_mod4 : n % 4 = 1 ∨ n % 4 = 3 := by
      have h := Nat.mod_add_div n 4; omega
    rcases hn_mod4 with (h | h) <;> omega
  let S := (factorization n).support
  have h_sigma_prod : sigma 1 n = (∏ p ∈ S, sigma 1 (p ^ ((factorization n) p))) :=
    sigma_factorization_prod n hn_pos
  rw [h_sigma_prod] at h_sigma_mod4
  have h_S_odd (p : ℕ) (hp : p ∈ S) : p ≠ 2 := by
    have hp_mem : p ∈ n.primeFactors := by
      have : S = n.primeFactors := rfl; simpa [this] using hp
    have hp_dvd : p ∣ n := (Nat.mem_primeFactors.mp hp_mem).2.1
    intro hp_eq2; apply h_odd; rw [hp_eq2]; exact hp_dvd

  -- Find a prime with odd exponent (at least one must exist)
  have h_exists_odd : ∃ p ∈ S, ((factorization n) p) % 2 = 1 := by
    by_contra h_all_even
    push_neg at h_all_even
    have h_all_odd_sigma : ∀ p ∈ S, sigma 1 (p ^ ((factorization n) p)) % 2 = 1 := by
      intro p hp
      have hp_prime : p.Prime := by
        have hp_mem : p ∈ n.primeFactors := by
          have : S = n.primeFactors := rfl; simpa [this] using hp
        exact (Nat.mem_primeFactors.mp hp_mem).1
      have hp_ne2 : p ≠ 2 := h_S_odd p hp
      have h_exp_even : ((factorization n) p) % 2 = 0 := by
        have h_not_odd : ¬ ((factorization n) p) % 2 = 1 := h_all_even p hp
        omega
      rw [sigma_mod2_odd_prime p ((factorization n) p) hp_prime hp_ne2]
      have : (((factorization n) p) + 1) % 2 = 1 := by
        have h := h_exp_even; omega
      exact this
    have h_prod_odd : (∏ p ∈ S, sigma 1 (p ^ ((factorization n) p))) % 2 = 1 :=
      prod_odd S (fun p => sigma 1 (p ^ ((factorization n) p))) h_all_odd_sigma
    rw [h_prod_odd] at h_sigma_mod4; omega

  rcases h_exists_odd with ⟨p, hpS, hp_odd⟩
  let k := (factorization n) p
  have hk_odd : k % 2 = 1 := hp_odd

  -- No other prime has odd exponent (otherwise product ≡ 0 mod 4)
  have h_no_other_odd : ∀ q ∈ S, q ≠ p → ((factorization n) q) % 2 = 0 := by
    intro q hqS hne
    by_contra hq_odd
    have hq_odd' : ((factorization n) q) % 2 = 1 := by omega
    have hp_prime : p.Prime := by
      have hp_mem : p ∈ n.primeFactors := by
        have : S = n.primeFactors := rfl; simpa [this] using hpS
      exact (Nat.mem_primeFactors.mp hp_mem).1
    have hq_prime : q.Prime := by
      have hq_mem : q ∈ n.primeFactors := by
        have : S = n.primeFactors := rfl; simpa [this] using hqS
      exact (Nat.mem_primeFactors.mp hq_mem).1
    have hp_ne2 : p ≠ 2 := h_S_odd p hpS
    have hq_ne2 : q ≠ 2 := h_S_odd q hqS
    have h_sigma_p_even : 2 ∣ sigma 1 (p ^ k) := by
      rw [Nat.dvd_iff_mod_eq_zero, sigma_mod2_odd_prime p k hp_prime hp_ne2, hk_odd]
      simp
    have h_sigma_q_even : 2 ∣ sigma 1 (q ^ ((factorization n) q)) := by
      rw [Nat.dvd_iff_mod_eq_zero, sigma_mod2_odd_prime q ((factorization n) q) hq_prime hq_ne2, hq_odd']
      simp
    have h_p_in_S : p ∈ S := hpS
    have h_q_in_S : q ∈ S := hqS
    -- The full product has at least two factors of 2, so is divisible by 4
    have h_4_dvd_prod : 4 ∣ ∏ r ∈ S, sigma 1 (r ^ ((factorization n) r)) := by
      have h_dvd_p : sigma 1 (p ^ k) ∣ ∏ r ∈ S, sigma 1 (r ^ ((factorization n) r)) :=
        Finset.dvd_prod_of_mem (fun r => sigma 1 (r ^ ((factorization n) r))) h_p_in_S
      have h_dvd_q : sigma 1 (q ^ ((factorization n) q)) ∣ ∏ r ∈ S, sigma 1 (r ^ ((factorization n) r)) :=
        Finset.dvd_prod_of_mem (fun r => sigma 1 (r ^ ((factorization n) r))) h_q_in_S
      have h_4_dvd_pq : 4 ∣ sigma 1 (p ^ k) * sigma 1 (q ^ ((factorization n) q)) :=
        mul_dvd_mul h_sigma_p_even h_sigma_q_even
      have h_pq_dvd_prod : sigma 1 (p ^ k) * sigma 1 (q ^ ((factorization n) q)) ∣
          ∏ r ∈ S, sigma 1 (r ^ ((factorization n) r)) :=
        mul_dvd_mul h_dvd_p h_dvd_q
      exact h_4_dvd_pq.trans h_pq_dvd_prod
    rw [Nat.dvd_iff_mod_eq_zero] at h_4_dvd_prod
    rw [h_4_dvd_prod] at h_sigma_mod4; omega
  -- Build m² from the product of all prime powers with even exponents
  let other_primes := S.erase p
  have h_other_exponents_even : ∀ q ∈ other_primes, ((factorization n) q) % 2 = 0 :=
    fun q hq => h_no_other_odd q (Finset.mem_of_mem_erase hq) (Finset.ne_of_mem_erase hq)
  have hm_sq_square : (∏ q ∈ other_primes, q ^ ((factorization n) q)) = 
      ((∏ q ∈ other_primes, q ^ (((factorization n) q) / 2)) ^ 2) := by
    calc
      (∏ q ∈ other_primes, q ^ ((factorization n) q))
          = (∏ q ∈ other_primes, (q ^ (((factorization n) q) / 2)) ^ 2) := by
        refine Finset.prod_congr rfl fun q hq => ?_
        have hq_even : ((factorization n) q) % 2 = 0 := h_other_exponents_even q hq
        have h_mul : ((factorization n) q) / 2 * 2 = (factorization n) q := by
          have h_dvd : 2 ∣ (factorization n) q := Nat.dvd_of_mod_eq_zero hq_even
          omega
        simp [pow_mul, h_mul]
      _ = ((∏ q ∈ other_primes, q ^ (((factorization n) q) / 2)) ^ 2) := by
        simp [Finset.prod_pow]
  let m := ∏ q ∈ other_primes, q ^ (((factorization n) q) / 2)
  have hm_sq_eq : (∏ q ∈ other_primes, q ^ ((factorization n) q)) = m ^ 2 := hm_sq_square

  have hp_prime : p.Prime := by
    have hp_mem : p ∈ n.primeFactors := by
      have : S = n.primeFactors := rfl; simpa [this] using hpS
    exact (Nat.mem_primeFactors.mp hp_mem).1

  have h_n_eq : n = p ^ k * m ^ 2 := by
    have h_not_mem : p ∉ other_primes := Finset.not_mem_erase p S
    calc
      n = (∏ r ∈ S, r ^ ((factorization n) r)) := by
        symm; exact calc
          (∏ r ∈ S, r ^ ((factorization n) r)) = ((factorization n).prod fun p a => p ^ a) := by
            rw [Finsupp.prod_of_support_subset (factorization n) (Finset.Subset.refl _) (fun p a => p ^ a) (by simp)]
            simp
          _ = n := Nat.factorization_prod_pow_eq_self hn_pos
      _ = (p ^ k) * (∏ q ∈ other_primes, q ^ ((factorization n) q)) := by
        rw [Finset.prod_insert h_not_mem]
        simp [k]
      _ = (p ^ k) * (m ^ 2) := by rw [hm_sq_eq]
      _ = p ^ k * m ^ 2 := rfl
  -- Show sigma(p^k) ≡ 2 (mod 4) from the product ≡ 2 (mod 4)
  have h_sigma_pk_mod4 : sigma 1 (p ^ k) % 4 = 2 := by
    have h_others_odd : ∀ r ∈ S, r ≠ p → (sigma 1 (r ^ ((factorization n) r))) % 2 = 1 := by
      intro r hr hne
      have hr_prime : r.Prime := by
        have hr_mem : r ∈ n.primeFactors := by
          have : S = n.primeFactors := rfl; simpa [this] using hr
        exact (Nat.mem_primeFactors.mp hr_mem).1
      have hr_ne2 : r ≠ 2 := h_S_odd r hr
      have hr_exp_even : ((factorization n) r) % 2 = 0 := h_no_other_odd r hr hne
      rw [sigma_mod2_odd_prime r ((factorization n) r) hr_prime hr_ne2]
      have : (((factorization n) r) + 1) % 2 = 1 := by omega
      exact this
    have h_not_mem : p ∉ S.erase p := Finset.not_mem_erase p S
    have h_prod_split : (∏ r ∈ S, sigma 1 (r ^ ((factorization n) r))) =
        sigma 1 (p ^ k) * (∏ r ∈ S.erase p, sigma 1 (r ^ ((factorization n) r))) := by
      rw [Finset.prod_insert h_not_mem]; simp
    rw [h_prod_split] at h_sigma_mod4
    have h_rest_odd : (∏ r ∈ S.erase p, sigma 1 (r ^ ((factorization n) r))) % 2 = 1 :=
      prod_odd (S.erase p) (fun r => sigma 1 (r ^ ((factorization n) r))) h_others_odd
    have h_rest_mod4_cases : (∏ r ∈ S.erase p, sigma 1 (r ^ ((factorization n) r))) % 4 = 1 ∨
        (∏ r ∈ S.erase p, sigma 1 (r ^ ((factorization n) r))) % 4 = 3 := by
      have h_mod2 := h_rest_odd
      have h4 := Nat.mod_add_div (∏ r ∈ S.erase p, sigma 1 (r ^ ((factorization n) r))) 4
      omega
    rcases h_rest_mod4_cases with (h | h)
    · rw [Nat.mul_mod, h] at h_sigma_mod4
      have : sigma 1 (p ^ k) % 4 = 2 := by omega; exact this
    · rw [Nat.mul_mod, h] at h_sigma_mod4
      have : sigma 1 (p ^ k) % 4 = 2 := by
        have h_table : (3 * 0) % 4 = 0 ∧ (3 * 1) % 4 = 3 ∧ (3 * 2) % 4 = 2 ∧ (3 * 3) % 4 = 1 := by norm_num
        omega
      exact this

  have h_sigma_pk_eq : sigma 1 (p ^ k) = ∑ i ∈ Finset.range (k+1), p ^ i :=
    sigma_prime_pow_formula p k hp_prime
  rw [h_sigma_pk_eq] at h_sigma_pk_mod4
  have hp_mod4_cases : p % 4 = 1 ∨ p % 4 = 3 := by
    have hp_not_even : ¬ 2 ∣ p := by
      intro h; apply h_S_odd p hpS
      have h2prime : (2 : ℕ).Prime := by norm_num
      have := hp_prime.eq_one_or_self_of_dvd 2 h
      rcases this with (h2 | h2)
      · norm_num at h2
      · exact h2.symm
    have hp_mod2 : p % 2 = 1 := by
      have hm := Nat.mod_two_eq_zero_or_one p
      rcases hm with (h | h)
      · exfalso; apply hp_not_even; exact ⟨p / 2, by omega⟩
      · exact h
    have h := Nat.mod_add_div p 4; omega
  rcases hp_mod4_cases with (hp_mod4 | hp_mod4)
  · -- p ≡ 1 (mod 4) case
    have pow_mod4_one (i : ℕ) : p ^ i % 4 = 1 := by
      induction i with
      | zero => simp
      | succ n ih => simp [pow_succ, Nat.mul_mod, hp_mod4, ih]
    have h_sum_mod4 : (∑ i ∈ Finset.range (k+1), p ^ i) % 4 = (k+1) % 4 := by
      induction' k with n ih
      · simp
      · rw [Finset.sum_range_succ, Nat.add_mod, ih, pow_mod4_one (n+1)]
        simp
    rw [h_sum_mod4] at h_sigma_pk_mod4
    have hk_mod4 : k % 4 = 1 := by
      have : (k+1) % 4 = 2 := h_sigma_pk_mod4
      omega
    -- p ∤ m: since m's prime factors are all different from p
    have h_p_not_dvd_m : ¬ p ∣ m := by
      intro hpm
      have hpm_sq : p ∣ m ^ 2 := by
        have : m ^ 2 = m * m := by ring
        rw [this]; exact mul_dvd_mul hpm hpm
      have hp_dvd_other : p ∣ ∏ q ∈ other_primes, q ^ ((factorization n) q) := by
        rw [hm_sq_eq]; exact hpm_sq
      have hp_not_dvd_other : ¬ p ∣ ∏ q ∈ other_primes, q ^ ((factorization n) q) := by
        have h_p_not_in_other : p ∉ other_primes := Finset.not_mem_erase p S
        have h_prime_factor : ∀ q ∈ other_primes, ¬ p ∣ q := by
          intro q hq
          have hq_prime : q.Prime := by
            have hq_mem : q ∈ n.primeFactors := by
              have hq_S : q ∈ S := Finset.mem_of_mem_erase hq
              have : S = n.primeFactors := rfl
              simpa [this] using hq_S
            exact (Nat.mem_primeFactors.mp hq_mem).1
          have hne : p ≠ q := Finset.ne_of_mem_erase hq
          intro hpq
          apply hne
          have := hq_prime.eq_one_or_self_of_dvd p hpq
          rcases this with (h | h)
          · have : 1 < p := hp_prime.one_lt; omega
          · exact h.symm
        -- If p divides the product, it must divide one of the factors
        have h_p_dvd_factor : ∃ q ∈ other_primes, p ∣ q ^ ((factorization n) q) :=
          Finset.dvd_prod.mp hp_dvd_other
        rcases h_p_dvd_factor with ⟨q, hq, hpq⟩
        have hq_prime : q.Prime := by
          have hq_mem : q ∈ n.primeFactors := by
            have hq_S : q ∈ S := Finset.mem_of_mem_erase hq
            have : S = n.primeFactors := rfl
            simpa [this] using hq_S
          exact (Nat.mem_primeFactors.mp hq_mem).1
        have h_p_dvd_q : p ∣ q := hq_prime.dvd_of_dvd_pow hpq
        exact h_prime_factor q hq h_p_dvd_q
      exact hp_not_dvd_other hp_dvd_other
    exact ⟨p, k, m, hp_prime, h_n_eq, hp_mod4, hk_mod4, h_p_not_dvd_m⟩
  · -- p ≡ 3 (mod 4) case: impossible by mod 4 analysis of the sum
    have h_one_plus_p_mod4 : (1 + p) % 4 = 0 := by
      rw [hp_mod4]; norm_num
    
    have hp_sq_mod4 : p ^ 2 % 4 = 1 := by
      calc
        p ^ 2 % 4 = ((p % 4) ^ 2) % 4 := by simp [Nat.pow_mod]
        _ = (3 ^ 2) % 4 := by rw [hp_mod4]
        _ = 9 % 4 := by norm_num
        _ = 1 := by norm_num
    -- Key lemma: for p ≡ 3 (mod 4) and k odd, the sum ∑_{i=0}^k p^i ≡ 0 (mod 4)
    -- Proof by induction on r: factor (1+p) from each pair of consecutive terms
    have h_sum_even_pairs (r : ℕ) : (∑ i ∈ Finset.range (2*r+2), p ^ i) % 4 = 0 := by
      induction' r with n ih
      · -- Base r=0: sum over {0,1} = 1 + p ≡ 0 (mod 4)
        simp; exact h_one_plus_p_mod4
      · -- Inductive step: sum up to 2(n+1)+1 = sum up to 2n+1 + p^{2n+2} + p^{2n+3}
        rw [show 2*(n+1)+2 = (2*n+2) + 2 by omega]
        rw [Finset.sum_range_succ, Finset.sum_range_succ, add_assoc]
        rw [Nat.add_mod, ih, add_zero]
        have h_last_two : (p ^ (2*n+2) + p ^ (2*n+3)) % 4 = 0 := by
          have h_factor : p ^ (2*n+2) * (1 + p) = p ^ (2*n+2) + p ^ (2*n+3) := by ring
          rw [← h_factor, Nat.mul_mod]
          have h_pow_mod4 : p ^ (2*n+2) % 4 = 1 := by
            rw [show p ^ (2*n+2) = (p ^ 2) ^ (n+1) by ring]
            induction' (n+1) with m ih'
            · simp
            · rw [pow_succ, Nat.mul_mod, hp_sq_mod4, ih']; norm_num
          simp [h_pow_mod4, h_one_plus_p_mod4]
        rw [h_last_two, add_zero]
    
    have hk_form : ∃ r : ℕ, k = 2*r+1 := by
      have := Nat.mod_two_eq_zero_or_one k
      rcases this with (h | h)
      · omega
      · refine ⟨k/2, ?_⟩
        have := Nat.div_add_mod k 2
        omega
    
    rcases hk_form with ⟨r, hkr⟩
    have h_range : k+1 = 2*r+2 := by rw [hkr]; omega
    rw [h_range] at h_sigma_pk_mod4
    rw [h_sum_even_pairs r] at h_sigma_pk_mod4
    omega
