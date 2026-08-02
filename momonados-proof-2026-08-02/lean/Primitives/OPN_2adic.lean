import Mathlib.Tactic
import Mathlib.NumberTheory.ArithmeticFunction.Defs
import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.Data.Nat.Factorization.Basic
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.RingTheory.Multiplicity
import Mathlib.Algebra.Ring.Parity
import Mathlib.Algebra.Ring.GeomSum
import Mathlib.Data.Nat.Multiplicity

open Nat ArithmeticFunction

namespace Imscribing.Classical.OPN

-- ============================================================
-- DEFINITIONS
-- ============================================================

-- OPNPerfect uses sigma (not proper divisors) for compatibility with the proof
def OPNPerfect (n : ℕ) : Prop := sigma 1 n = 2 * n

noncomputable def v₂ (n : ℕ) : ℕ := (Nat.factorization n) 2

-- ============================================================
-- HELPER LEMMAS
-- ============================================================

private lemma pred_dvd_pow_sub_one (p n : ℕ) (hp : 1 ≤ p) : (p - 1) ∣ (p ^ n - 1) := by
  use ∑ i ∈ Finset.range n, p ^ i
  zify [hp, Nat.one_le_pow n p (by omega)]
  have h1 := geom_sum_mul (p : ℤ) n
  have h2 := mul_comm (∑ i ∈ Finset.range n, (p : ℤ) ^ i) ((p : ℤ) - 1)
  linarith

private lemma v2_eq_one_of_mod4_eq2 {n : ℕ} (hn : n % 4 = 2) : v₂ n = 1 := by
  have hpos : n ≠ 0 := by omega
  have h2prime : Nat.Prime 2 := by decide
  have h2dvd : 2^1 ∣ n := ⟨n / 2, by omega⟩
  have h4notdvd : ¬ (2^2 ∣ n) := by norm_num; intro ⟨k, hk⟩; omega
  simp only [v₂]
  apply Nat.le_antisymm
  · by_contra hlt
    push_neg at hlt
    exact h4notdvd ((h2prime.pow_dvd_iff_le_factorization hpos (k := 2)).mpr (by omega))
  · exact (h2prime.pow_dvd_iff_le_factorization hpos (k := 1)).mp h2dvd

private lemma pow_odd_of_odd {q : ℕ} (hq : q % 2 = 1) (i : ℕ) : q ^ i % 2 = 1 := by
  induction i with
  | zero => simp
  | succ n ih => simp [pow_succ, Nat.mul_mod, hq, ih]

private lemma pow_mod4_of_mod4 {p : ℕ} (hp : p % 4 = 1) (i : ℕ) : p ^ i % 4 = 1 := by
  induction i with
  | zero => simp
  | succ n ih => simp [pow_succ, Nat.mul_mod, hp, ih]

private lemma geom_sum_mod2 (q n : ℕ) (hq : q % 2 = 1) : (∑ i ∈ Finset.range n, q ^ i) % 2 = n % 2 := by
  induction n with
  | zero => simp
  | succ k ih =>
    rw [Finset.sum_range_succ, Nat.add_mod, pow_odd_of_odd hq k, ih]
    omega

private lemma geom_sum_odd_mod2 (q e : ℕ) (hq : q % 2 = 1) : (∑ i ∈ Finset.range (2 * e + 1), q ^ i) % 2 = 1 := by
  have h := geom_sum_mod2 q (2 * e + 1) hq
  omega

private lemma geom_sum_mod4 (p n : ℕ) (hp : p % 4 = 1) : (∑ i ∈ Finset.range n, p ^ i) % 4 = n % 4 := by
  induction n with
  | zero => simp
  | succ k ih =>
    rw [Finset.sum_range_succ, Nat.add_mod, pow_mod4_of_mod4 hp k, ih]
    omega

private lemma geom_sum_mod4_eq2 (p n : ℕ) (hp : p % 4 = 1) (hn : n % 4 = 2) : (∑ i ∈ Finset.range n, p ^ i) % 4 = 2 := by
  have h := geom_sum_mod4 p n hp
  omega

-- ============================================================
-- σ MULTIPLICATIVITY AND THE PRODUCT FORMULA
-- ============================================================

lemma sigma_mul_of_coprime {a b : ℕ} (h : Nat.Coprime a b) : sigma 1 (a * b) = sigma 1 a * sigma 1 b :=
  isMultiplicative_sigma.map_mul_of_coprime h

lemma sigma_factorization_prod (n : ℕ) (hn : n ≠ 0) : sigma 1 n = (∏ p ∈ (Nat.factorization n).support, sigma 1 (p ^ ((Nat.factorization n) p))) := by
  have h_n_prod : (∏ p ∈ (Nat.factorization n).support, p ^ ((Nat.factorization n) p)) = n :=
    Nat.factorization_prod_pow_eq_self hn
  have coprime_prime_pow (p q a b : ℕ) (hp : p.Prime) (hq : q.Prime) (hne : p ≠ q) : Nat.Coprime (p ^ a) (q ^ b) := by
    apply Nat.Coprime.pow a b
    apply hp.coprime_iff_not_dvd.mpr
    intro hdiv; apply hne
    have h_cases : p = 1 ∨ p = q := hq.eq_one_or_self_of_dvd p hdiv
    rcases h_cases with (h | h)
    · exact absurd hp.one_lt (by omega)
    · exact h
  have h_pairwise : ((Nat.factorization n).support : Set ℕ).Pairwise (fun p q =>
      Nat.Coprime (p ^ ((Nat.factorization n) p)) (q ^ ((Nat.factorization n) q))) := by
    intro p hp q hq hne
    have hp_prime : p.Prime := (Nat.mem_primeFactors.mp (by
      simpa [Finset.mem_filter, hp, Finsupp.mem_support_iff] using hp)).1
    have hq_prime : q.Prime := (Nat.mem_primeFactors.mp (by
      simpa [Finset.mem_filter, hq, Finsupp.mem_support_iff] using hq)).1
    exact coprime_prime_pow p q ((Nat.factorization n) p) ((Nat.factorization n) q) hp_prime hq_prime hne
  calc
    sigma 1 n = sigma 1 (∏ p ∈ (Nat.factorization n).support, p ^ ((Nat.factorization n) p)) := by rw [h_n_prod]
    _ = (∏ p ∈ (Nat.factorization n).support, sigma 1 (p ^ ((Nat.factorization n) p))) :=
      (isMultiplicative_sigma (k := 1)).map_prod (fun p => p ^ ((Nat.factorization n) p))
        (Nat.factorization n).support h_pairwise

theorem opn_product_constraint {p k m : ℕ} (hperf : OPNPerfect (p ^ k * m ^ 2)) (hcop : Nat.Coprime (p ^ k) (m ^ 2)) :
    sigma 1 (p ^ k) * sigma 1 (m ^ 2) = 2 * (p ^ k * m ^ 2) := by
  rw [← sigma_mul_of_coprime hcop]
  exact hperf

lemma sigma_prime_pow_ratio (p k : ℕ) (hp : Nat.Prime p) : sigma 1 (p ^ k) * (p - 1) + 1 = p ^ (k + 1) := by
  have h_sum : sigma 1 (p ^ k) = ∑ i ∈ Finset.range (k + 1), p ^ i := by
    rw [sigma_apply, Nat.divisors_prime_pow hp, Finset.sum_map]; simp
  rw [h_sum]
  zify [hp.one_le, Nat.one_le_pow (k + 1) p hp.pos]
  linarith [geom_sum_mul (p : ℤ) (k + 1)]

lemma sigma_prime_pow_lt (p k : ℕ) (hp : Nat.Prime p) : sigma 1 (p ^ k) * (p - 1) < p ^ (k + 1) := by
  have h := sigma_prime_pow_ratio p k hp; omega

lemma opn_mod4 (p k m : ℕ) (h_odd : ¬ 2 ∣ p ^ k * m ^ 2) (hp_mod : p % 4 = 1) : (p ^ k * m ^ 2) % 4 = 1 := by
  have hm_odd : m % 2 = 1 := by
    by_contra hm
    push_neg at hm
    have : 2 ∣ m := by omega
    exact h_odd (Dvd.dvd.mul_left (Dvd.dvd.pow this (by norm_num)) (p ^ k))
  have hpk_mod : p ^ k % 4 = 1 := pow_mod4_of_mod4 hp_mod k
  have hm2_mod : m ^ 2 % 4 = 1 := by
    have hm4 : m % 4 = 1 ∨ m % 4 = 3 := by omega
    rcases hm4 with h | h <;> simp [pow_succ, pow_zero, Nat.mul_mod, h]
  calc (p ^ k * m ^ 2) % 4
      = (p ^ k % 4 * (m ^ 2 % 4)) % 4 := by rw [Nat.mul_mod]
    _ = (1 * 1) % 4 := by rw [hpk_mod, hm2_mod]
    _ = 1 := by norm_num

-- ============================================================
-- EULER'S THEOREM (1747)
-- Every odd perfect number has the form n = p^k * m²
-- where p prime, p ≡ k ≡ 1 [MOD 4], gcd(p, m) = 1.
-- ============================================================

lemma sigma_prime_pow_formula (p k : ℕ) (hp : Nat.Prime p) : sigma 1 (p ^ k) = ∑ i ∈ Finset.range (k+1), p ^ i := by
  rw [sigma_apply, Nat.divisors_prime_pow hp, Finset.sum_map]; simp

lemma prod_odd (s : Finset ℕ) (f : ℕ → ℕ) (h : ∀ x ∈ s, f x % 2 = 1) : (∏ x ∈ s, f x) % 2 = 1 := by
  induction' s using Finset.induction_on with a s' has ih
  · simp
  · have ha : f a % 2 = 1 := h a (by simpa using Finset.mem_insert_self a s')
    have hs' : ∀ x ∈ s', f x % 2 = 1 := fun x hx => h x (by simpa using Finset.mem_insert_of_mem hx)
    simpa [Finset.prod_insert has, Nat.mul_mod, ha, ih hs']

private lemma sum_range_pow_p_mod4_eq_zero (p : ℕ) (hp_mod4 : p % 4 = 3) (r : ℕ) :
    (∑ i ∈ Finset.range (2*r+2), p ^ i) % 4 = 0 := by
  have h_one_plus_p_mod4 : (1 + p) % 4 = 0 := by
    calc
      (1 + p) % 4 = (1 + p % 4) % 4 := by simp
      _ = (1 + 3) % 4 := by rw [hp_mod4]
      _ = 0 := by norm_num
  have hp_sq_mod4 : p ^ 2 % 4 = 1 := by
    calc
      p ^ 2 % 4 = ((p % 4) ^ 2) % 4 := by simp [Nat.pow_mod]
      _ = (3 ^ 2) % 4 := by rw [hp_mod4]
      _ = 9 % 4 := by norm_num
      _ = 1 := by norm_num
  induction r with
  | zero =>
    simpa [add_comm] using h_one_plus_p_mod4
  | succ n ih =>
    have h_last_two : (p ^ (2*n+2) + p ^ (2*n+3)) % 4 = 0 := by
      have h_factor : p ^ (2*n+2) * (1 + p) = p ^ (2*n+2) + p ^ (2*n+3) := by ring
      rw [← h_factor, Nat.mul_mod]
      have h_pow_mod4 : p ^ (2*n+2) % 4 = 1 := by
        have h_eq : p ^ (2*n+2) = (p ^ 2) ^ (n+1) := by ring
        rw [h_eq]
        exact pow_mod4_of_mod4 hp_sq_mod4 (n+1)
      simp [h_pow_mod4, h_one_plus_p_mod4]
    have h_sum_eq : (∑ i ∈ Finset.range (2*(n+1)+2), p ^ i) =
        (∑ i ∈ Finset.range (2*n+2), p ^ i) + p ^ (2*n+2) + p ^ (2*n+3) := by
      simp [Finset.sum_range_succ, add_assoc, show 2*(n+1)+2 = 2*n+4 by omega]
    calc
      (∑ i ∈ Finset.range (2*(n+1)+2), p ^ i) % 4
          = ((∑ i ∈ Finset.range (2*n+2), p ^ i) + p ^ (2*n+2) + p ^ (2*n+3)) % 4 := by rw [h_sum_eq]
      _ = (((∑ i ∈ Finset.range (2*n+2), p ^ i) % 4) + ((p ^ (2*n+2) + p ^ (2*n+3)) % 4)) % 4 := by
        rw [add_assoc, Nat.add_mod]
      _ = (0 + 0) % 4 := by rw [ih, h_last_two]
      _ = 0 := by norm_num

theorem euler_opn_form (n : ℕ) (h_odd : ¬ 2 ∣ n) (h_perf : OPNPerfect n) :
    ∃ (p k m : ℕ), Nat.Prime p ∧ n = p ^ k * m ^ 2 ∧ p % 4 = 1 ∧ k % 4 = 1 ∧ ¬ p ∣ m := by
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
  let S := (Nat.factorization n).support
  have h_sigma_prod : sigma 1 n = (∏ p ∈ S, sigma 1 (p ^ ((Nat.factorization n) p))) :=
    sigma_factorization_prod n hn_pos
  rw [h_sigma_prod] at h_sigma_mod4
  have h_S_odd (p : ℕ) (hp : p ∈ S) : p ≠ 2 := by
    have hp_mem : p ∈ n.primeFactors := by
      have : S = n.primeFactors := rfl; simpa [this] using hp
    have hp_dvd : p ∣ n := (Nat.mem_primeFactors.mp hp_mem).2.1
    intro hp_eq2; apply h_odd; rw [← hp_eq2]; exact hp_dvd

  -- Find a prime with odd exponent (at least one must exist)
  have h_exists_odd : ∃ p ∈ S, ((Nat.factorization n) p) % 2 = 1 := by
    by_contra h_all_even
    push_neg at h_all_even
    have h_all_odd_sigma : ∀ p ∈ S, sigma 1 (p ^ ((Nat.factorization n) p)) % 2 = 1 := by
      intro p hp
      have hp_prime : p.Prime := by
        have hp_mem : p ∈ n.primeFactors := by
          have : S = n.primeFactors := rfl; simpa [this] using hp
        exact (Nat.mem_primeFactors.mp hp_mem).1
      have hp_ne2 : p ≠ 2 := h_S_odd p hp
      have h_exp_even : ((Nat.factorization n) p) % 2 = 0 := by
        have h_not_odd : ¬ ((Nat.factorization n) p) % 2 = 1 := h_all_even p hp
        omega
      have h_sigma_mod2 : sigma 1 (p ^ ((Nat.factorization n) p)) % 2 = 1 := by
        rw [sigma_prime_pow_formula p ((Nat.factorization n) p) hp_prime]
        have h_sum_mod2 : (∑ i ∈ Finset.range (((Nat.factorization n) p) + 1), p ^ i) % 2 = (((Nat.factorization n) p) + 1) % 2 :=
          geom_sum_mod2 p (((Nat.factorization n) p) + 1) (by
            have hp_not_even : ¬ 2 ∣ p := by
              intro h; apply hp_ne2; exact ((hp_prime.eq_one_or_self_of_dvd 2 h).resolve_left (by norm_num)).symm
            have hm := Nat.mod_two_eq_zero_or_one p
            rcases hm with (h | h)
            · exfalso; apply hp_not_even; exact ⟨p / 2, by omega⟩
            · exact h)
        rw [h_sum_mod2]
        have h_len_odd : (((Nat.factorization n) p) + 1) % 2 = 1 := by omega
        rw [h_len_odd]
      exact h_sigma_mod2
    have h_prod_odd : (∏ p ∈ S, sigma 1 (p ^ ((Nat.factorization n) p))) % 2 = 1 :=
      prod_odd S (fun p => sigma 1 (p ^ ((Nat.factorization n) p))) h_all_odd_sigma
    have h_prod_mod2_eq0 : (∏ p ∈ S, sigma 1 (p ^ ((Nat.factorization n) p))) % 2 = 0 := by
      have h_implies : ∀ (x : ℕ), x % 4 = 2 → x % 2 = 0 := by omega
      exact h_implies _ h_sigma_mod4
    rw [h_prod_odd] at h_prod_mod2_eq0
    omega

  rcases h_exists_odd with ⟨p, hpS, hp_odd⟩
  let k := (Nat.factorization n) p
  have hk_odd : k % 2 = 1 := hp_odd

  -- No other prime has odd exponent (otherwise product divisible by 4, contradiction)
  have h_no_other_odd : ∀ q ∈ S, q ≠ p → ((Nat.factorization n) q) % 2 = 0 := by
    intro q hqS hne
    by_contra hq_odd
    have hq_odd' : ((Nat.factorization n) q) % 2 = 1 := by omega
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
    have hp_odd_val : p % 2 = 1 := by
      have hp_not_even : ¬ 2 ∣ p := by
        intro h; apply hp_ne2; exact ((hp_prime.eq_one_or_self_of_dvd 2 h).resolve_left (by norm_num)).symm
      have hm := Nat.mod_two_eq_zero_or_one p
      rcases hm with (h | h)
      · exfalso; apply hp_not_even; exact ⟨p / 2, by omega⟩
      · exact h
    have hq_odd_val : q % 2 = 1 := by
      have hq_not_even : ¬ 2 ∣ q := by
        intro h; apply hq_ne2; exact ((hq_prime.eq_one_or_self_of_dvd 2 h).resolve_left (by norm_num)).symm
      have hm := Nat.mod_two_eq_zero_or_one q
      rcases hm with (h | h)
      · exfalso; apply hq_not_even; exact ⟨q / 2, by omega⟩
      · exact h
    have h_sigma_p_even : 2 ∣ sigma 1 (p ^ k) := by
      rw [sigma_prime_pow_formula p k hp_prime, Nat.dvd_iff_mod_eq_zero]
      have hk_odd' : (k+1) % 2 = 0 := by
        have : k % 2 = 1 := hk_odd; omega
      calc
        (∑ i ∈ Finset.range (k+1), p ^ i) % 2 = (k+1) % 2 := geom_sum_mod2 p (k+1) hp_odd_val
        _ = 0 := hk_odd'
    have h_sigma_q_even : 2 ∣ sigma 1 (q ^ ((Nat.factorization n) q)) := by
      rw [sigma_prime_pow_formula q ((Nat.factorization n) q) hq_prime, Nat.dvd_iff_mod_eq_zero]
      have hq_exp_odd' : (((Nat.factorization n) q) + 1) % 2 = 0 := by
        have : ((Nat.factorization n) q) % 2 = 1 := hq_odd'; omega
      calc
        (∑ i ∈ Finset.range (((Nat.factorization n) q) + 1), q ^ i) % 2 = (((Nat.factorization n) q) + 1) % 2 :=
          geom_sum_mod2 q (((Nat.factorization n) q) + 1) hq_odd_val
        _ = 0 := hq_exp_odd'
    have h_p_in_S : p ∈ S := hpS
    have h_q_in_S : q ∈ S := hqS
    have h_4_dvd_prod : 4 ∣ ∏ r ∈ S, sigma 1 (r ^ ((Nat.factorization n) r)) := by
      have h_sigma_prod_eq : sigma 1 (p ^ k) * sigma 1 (q ^ ((Nat.factorization n) q)) ∣
          ∏ r ∈ S, sigma 1 (r ^ ((Nat.factorization n) r)) := by
        have h_prod_erase_p : (∏ r ∈ S, sigma 1 (r ^ ((Nat.factorization n) r))) =
            sigma 1 (p ^ k) * (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) := by
          simpa [k] using (Finset.mul_prod_erase S (fun r => sigma 1 (r ^ ((Nat.factorization n) r))) h_p_in_S).symm
        have hq_erase_p : q ∈ S.erase p := by
          rw [Finset.mem_erase]; exact ⟨hne, h_q_in_S⟩
        have h_prod_erase_q : (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) =
            sigma 1 (q ^ ((Nat.factorization n) q)) *
            (∏ r ∈ (S.erase p).erase q, sigma 1 (r ^ ((Nat.factorization n) r))) := by
          rw [← Finset.mul_prod_erase (S.erase p) (fun r => sigma 1 (r ^ ((Nat.factorization n) r))) hq_erase_p]
        rw [h_prod_erase_p, h_prod_erase_q, ← mul_assoc]
        refine ⟨(∏ r ∈ (S.erase p).erase q, sigma 1 (r ^ ((Nat.factorization n) r))), rfl⟩
      have h_4_dvd_pq : 4 ∣ sigma 1 (p ^ k) * sigma 1 (q ^ ((Nat.factorization n) q)) :=
        mul_dvd_mul h_sigma_p_even h_sigma_q_even
      exact h_4_dvd_pq.trans h_sigma_prod_eq
    rw [Nat.dvd_iff_mod_eq_zero] at h_4_dvd_prod
    rw [h_4_dvd_prod] at h_sigma_mod4; omega

  -- Build m² from the product of all prime powers with even exponents
  let other_primes := S.erase p
  have h_other_exponents_even : ∀ q ∈ other_primes, ((Nat.factorization n) q) % 2 = 0 :=
    fun q hq => h_no_other_odd q (Finset.mem_of_mem_erase hq) (Finset.ne_of_mem_erase hq)

  have hm_sq_square : (∏ q ∈ other_primes, q ^ ((Nat.factorization n) q)) = 
      ((∏ q ∈ other_primes, q ^ (((Nat.factorization n) q) / 2)) ^ 2) := by
    calc
      (∏ q ∈ other_primes, q ^ ((Nat.factorization n) q))
          = (∏ q ∈ other_primes, (q ^ (((Nat.factorization n) q) / 2)) ^ 2) := by
        refine Finset.prod_congr rfl fun q hq => ?_
        have hq_even : ((Nat.factorization n) q) % 2 = 0 := h_other_exponents_even q hq
        have h_mul : ((Nat.factorization n) q) / 2 * 2 = (Nat.factorization n) q := by
          have h_dvd : 2 ∣ (Nat.factorization n) q := Nat.dvd_of_mod_eq_zero hq_even
          omega
        calc
          q ^ ((Nat.factorization n) q) = q ^ ((((Nat.factorization n) q) / 2) * 2) := by rw [h_mul]
          _ = (q ^ (((Nat.factorization n) q) / 2)) ^ 2 := by rw [pow_mul]
      _ = ((∏ q ∈ other_primes, q ^ (((Nat.factorization n) q) / 2)) ^ 2) := by
        simp [Finset.prod_pow]
  let m := ∏ q ∈ other_primes, q ^ (((Nat.factorization n) q) / 2)
  have hm_sq_eq : (∏ q ∈ other_primes, q ^ ((Nat.factorization n) q)) = m ^ 2 := hm_sq_square

  have hp_prime : p.Prime := by
    have hp_mem : p ∈ n.primeFactors := by
      have : S = n.primeFactors := rfl; simpa [this] using hpS
    exact (Nat.mem_primeFactors.mp hp_mem).1

  have h_n_eq : n = p ^ k * m ^ 2 := by
    have h_not_mem : p ∉ other_primes := by
      intro h; have hm := Finset.mem_erase.mp h; exact hm.1 rfl
    calc
      n = (∏ r ∈ S, r ^ ((Nat.factorization n) r)) := by
        symm; exact calc
          (∏ r ∈ S, r ^ ((Nat.factorization n) r)) = ((Nat.factorization n).prod fun p a => p ^ a) := by
            rw [Finsupp.prod_of_support_subset (Nat.factorization n) (Finset.Subset.refl _) (fun p a => p ^ a) (by simp)]
          _ = n := Nat.factorization_prod_pow_eq_self hn_pos
      _ = (p ^ k) * (∏ q ∈ other_primes, q ^ ((Nat.factorization n) q)) := by
        simpa [k] using calc
          (∏ r ∈ S, r ^ ((Nat.factorization n) r)) = (p ^ ((Nat.factorization n) p)) * (∏ r ∈ S.erase p, r ^ ((Nat.factorization n) r)) := by
            simpa using (Finset.mul_prod_erase S (fun r => r ^ ((Nat.factorization n) r)) hpS).symm
          _ = p ^ k * (∏ q ∈ other_primes, q ^ ((Nat.factorization n) q)) := by simp [other_primes, k]
      _ = (p ^ k) * (m ^ 2) := by rw [hm_sq_eq]
      _ = p ^ k * m ^ 2 := rfl

  -- Show sigma(p^k) ≡ 2 (mod 4) from the product ≡ 2 (mod 4) and all other factors being odd
  have h_sigma_pk_mod4 : sigma 1 (p ^ k) % 4 = 2 := by
    have h_others_odd : ∀ r ∈ S, r ≠ p → (sigma 1 (r ^ ((Nat.factorization n) r))) % 2 = 1 := by
      intro r hr hne
      have hr_prime : r.Prime := by
        have hr_mem : r ∈ n.primeFactors := by
          have : S = n.primeFactors := rfl; simpa [this] using hr
        exact (Nat.mem_primeFactors.mp hr_mem).1
      have hr_ne2 : r ≠ 2 := h_S_odd r hr
      have hr_exp_even : ((Nat.factorization n) r) % 2 = 0 := h_no_other_odd r hr hne
      rw [sigma_prime_pow_formula r ((Nat.factorization n) r) hr_prime]
      have h_len_odd : (((Nat.factorization n) r) + 1) % 2 = 1 := by
        omega
      calc
        (∑ i ∈ Finset.range (((Nat.factorization n) r) + 1), r ^ i) % 2 = (((Nat.factorization n) r) + 1) % 2 :=
          geom_sum_mod2 r (((Nat.factorization n) r) + 1) (by
            have hr_not_even : ¬ 2 ∣ r := by
              intro h; apply hr_ne2; exact ((hr_prime.eq_one_or_self_of_dvd 2 h).resolve_left (by norm_num)).symm
            have hm := Nat.mod_two_eq_zero_or_one r
            rcases hm with (h | h)
            · exfalso; apply hr_not_even; exact ⟨r / 2, by omega⟩
            · exact h)
        _ = 1 := h_len_odd
    have h_not_mem : p ∉ S.erase p := by
      intro h; have hm := Finset.mem_erase.mp h; exact hm.1 rfl
    have h_prod_split : (∏ r ∈ S, sigma 1 (r ^ ((Nat.factorization n) r))) =
        sigma 1 (p ^ k) * (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) := by
      simpa [k] using (Finset.mul_prod_erase S (fun r => sigma 1 (r ^ ((Nat.factorization n) r))) hpS).symm
    rw [h_prod_split] at h_sigma_mod4
    have h_rest_odd : (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) % 2 = 1 :=
      prod_odd (S.erase p) (fun r => sigma 1 (r ^ ((Nat.factorization n) r))) (fun x hx => h_others_odd x (Finset.mem_of_mem_erase hx) (by
        have hx' := Finset.mem_erase.mp hx
        exact hx'.1))
    have h_rest_mod4_cases : (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) % 4 = 1 ∨
        (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) % 4 = 3 := by
      have h4 := Nat.mod_add_div (∏ r ∈ S.erase p, sigma 1 (r ^ ((Nat.factorization n) r))) 4
      omega
    rcases h_rest_mod4_cases with (h | h)
    · rw [Nat.mul_mod, h] at h_sigma_mod4
      omega
    · rw [Nat.mul_mod, h] at h_sigma_mod4
      omega

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
      omega
    -- p ∤ m: since m's prime factors are all different from p
    have h_p_not_dvd_m : ¬ p ∣ m := by
      intro hpm
      have hpm_sq : p ∣ m ^ 2 :=
        hpm.trans (by simpa [sq] using dvd_mul_right m m)
      have hp_dvd_other : p ∣ ∏ q ∈ other_primes, q ^ ((Nat.factorization n) q) := by
        rw [hm_sq_eq]; exact hpm_sq
      have hp_not_dvd_other : ¬ p ∣ ∏ q ∈ other_primes, q ^ ((Nat.factorization n) q) := by
        have h_prime_factor : ∀ q ∈ other_primes, ¬ p ∣ q := by
          intro q hq
          have hq_prime : q.Prime := by
            have hq_S : q ∈ S := Finset.mem_of_mem_erase hq
            have hq_mem : q ∈ n.primeFactors := by
              have : S = n.primeFactors := rfl
              simpa [this] using hq_S
            exact (Nat.mem_primeFactors.mp hq_mem).1
          have hne : p ≠ q := by
            have hq_erase := Finset.mem_erase.mp hq
            have hq_ne_p : q ≠ p := hq_erase.1
            exact fun hpq => hq_ne_p (hpq.symm)
          intro hpq
          apply hne
          have := hq_prime.eq_one_or_self_of_dvd p hpq
          rcases this with (h | h)
          · have : 1 < p := hp_prime.one_lt; omega
          · exact h
        have h_find_factor : (p ∣ ∏ q ∈ other_primes, q ^ ((Nat.factorization n) q)) → 
            ∃ q ∈ other_primes, p ∣ q ^ ((Nat.factorization n) q) := by
          refine Finset.induction_on other_primes ?_ ?_
          · intro h
            have h' : p ∣ 1 := by simpa [Finset.prod_empty] using h
            exfalso; exact hp_prime.not_dvd_one h'
          · intro a s has ih h_dvd'
            rw [Finset.prod_insert has] at h_dvd'
            rcases hp_prime.dvd_mul.mp h_dvd' with (h | h)
            · exact ⟨a, Finset.mem_insert_self a s, h⟩
            · rcases ih h with ⟨q, hq, hq_dvd⟩
              exact ⟨q, Finset.mem_insert_of_mem hq, hq_dvd⟩
        intro h_dvd
        rcases h_find_factor h_dvd with ⟨q, hq, hpq⟩
        have hq_prime : q.Prime := by
          have hq_S : q ∈ S := Finset.mem_of_mem_erase hq
          have hq_mem : q ∈ n.primeFactors := by
            have : S = n.primeFactors := rfl
            simpa [this] using hq_S
          exact (Nat.mem_primeFactors.mp hq_mem).1
        have h_p_dvd_q : p ∣ q := hp_prime.dvd_of_dvd_pow hpq
        exact h_prime_factor q hq h_p_dvd_q
      exact hp_not_dvd_other hp_dvd_other
    exact ⟨p, k, m, hp_prime, h_n_eq, hp_mod4, hk_mod4, h_p_not_dvd_m⟩
  · -- p ≡ 3 (mod 4) case: impossible — sum ≡ 0 (mod 4) when k is odd
    have h_sum_even_pairs : ∀ r : ℕ, (∑ i ∈ Finset.range (2*r+2), p ^ i) % 4 = 0 :=
      sum_range_pow_p_mod4_eq_zero p hp_mod4
    have hk_odd_val : k % 2 = 1 := hk_odd
    have hk_eq_form : k = 2*(k/2) + 1 := by
      have h_mod := Nat.div_add_mod k 2
      rw [hk_odd_val, add_comm] at h_mod
      omega
    have hk_form : ∃ r : ℕ, k = 2*r+1 := ⟨k/2, hk_eq_form⟩
    rcases hk_form with ⟨r, hkr⟩
    have h_range : k+1 = 2*r+2 := by rw [hkr]
    have h_contra : (∑ i ∈ Finset.range (2*r+2), p ^ i) % 4 = 2 := by
      simpa [h_range] using h_sigma_pk_mod4
    have h_sum_mod4 : (∑ i ∈ Finset.range (2*r+2), p ^ i) % 4 = 0 := h_sum_even_pairs r
    rw [h_sum_mod4] at h_contra
    exfalso; exact (by omega : (0 : ℕ) ≠ 2) h_contra

end Imscribing.Classical.OPN
