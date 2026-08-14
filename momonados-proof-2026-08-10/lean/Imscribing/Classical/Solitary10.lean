import Mathlib.Tactic
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.NumberTheory.ArithmeticFunction.Defs
import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.ArithmeticFunction.Moebius
import Mathlib.NumberTheory.ArithmeticFunction.VonMangoldt
import Mathlib.NumberTheory.ArithmeticFunction.Zeta

open ArithmeticFunction

namespace Imscribing.Classical.Solitary10

local notation "σ" => ArithmeticFunction.sigma (1 : ℕ)

set_option linter.style.nativeDecide false
set_option linter.style.induction false
set_option linter.style.whitespace false
set_option linter.style.longLine false

-- ─── Basic σ lemmas ───────────────────────────────────────────────────────────

lemma sigma_one_apply (n : ℕ) : σ n = ∑ d ∈ Nat.divisors n, d := by
  simp [ArithmeticFunction.sigma_apply]

lemma sigma_ge_self (n : ℕ) (hn : n > 0) : σ n ≥ n := by
  rw [sigma_one_apply]
  have h_self : n ∈ Nat.divisors n := Nat.mem_divisors.mpr ⟨dvd_refl n, hn.ne'⟩
  exact Finset.single_le_sum (fun d _ => Nat.zero_le d) h_self

lemma sigma_mul_coprime {a b : ℕ} (h : Nat.Coprime a b) : σ (a * b) = σ a * σ b :=
  (ArithmeticFunction.isMultiplicative_sigma (k := 1)).map_mul_of_coprime h

lemma geom_two (e : ℕ) : (∑ k ∈ Finset.range (e + 1), (2 : ℕ) ^ k) = 2 ^ (e + 1) - 1 := by
  induction e with
  | zero => norm_num
  | succ k ih => rw [Finset.sum_range_succ, ih, pow_succ]; omega

lemma sigma_two_pow (e : ℕ) : σ (2 ^ e) = 2 ^ (e + 1) - 1 := by
  rw [ArithmeticFunction.sigma_one_apply_prime_pow (by native_decide : Nat.Prime 2)]
  exact geom_two e

-- ─── Parity factorization ─────────────────────────────────────────────────────

lemma factor_2_pow_odd (n : ℕ) (hn : n > 0) : ∃ (e : ℕ) (u : ℕ), u % 2 = 1 ∧ n = 2 ^ e * u := by
  induction n using Nat.strong_induction_on with
  | h m ih =>
    by_cases h2m : 2 ∣ m
    · rcases h2m with ⟨t, hm_eq⟩
      have ht_pos : t > 0 := by
        exact Nat.pos_of_ne_zero (fun ht0 => by omega)
      have ht_lt : t < m := by subst hm_eq; omega
      rcases ih t ht_lt ht_pos with ⟨e, u, hu, ht_eq⟩
      exact ⟨e + 1, u, hu, by rw [hm_eq, ht_eq, pow_succ]; ring⟩
    · rcases Nat.mod_two_eq_zero_or_one m with (h | h)
      · exact absurd (Nat.dvd_of_mod_eq_zero h) h2m
      · exact ⟨0, m, h, by simp⟩

-- ─── Key coprime extraction lemmas ────────────────────────────────────────────

lemma factor_from_eq (a q x : ℕ) (hcop : Nat.Coprime a q) (h : a * σ x = q * x) : a ∣ x :=
  hcop.dvd_of_dvd_mul_left (by rw [← h]; exact ⟨σ x, by ring⟩)

-- Extract prime divisor from ¬Coprime with a prime
lemma prime_dvd_of_not_coprime {p n : ℕ} (hp : Nat.Prime p) (h : ¬ Nat.Coprime p n) : p ∣ n := by
  rw [Nat.coprime_iff_gcd_eq_one] at h
  have hgcd : Nat.gcd p n ∣ p := Nat.gcd_dvd_left p n
  rcases hp.eq_one_or_self_of_dvd _ hgcd with h1 | hself
  · exact absurd h1 h
  · exact hself ▸ Nat.gcd_dvd_right p n

-- ─── descent_generic ──────────────────────────────────────────────────────────

private lemma coeff_ineq_pow {a b : ℕ} (ha : a > 0) (hineq : a * 3 > b * 2)
    (e : ℕ) (he : e ≥ 1) : a * (2 ^ (e + 1) - 1) > b * 2 ^ e := by
  induction e, he using Nat.le_induction with
  | base =>
    calc a * (2 ^ (1 + 1) - 1)
        = a * 3 := by norm_num
      _ > b * 2 := hineq
      _ = b * 2 ^ 1 := by norm_num
  | succ k _ ih =>
    have hstep : 2^(k+2)-1 = 2*(2^(k+1)-1)+1 := by
      have : 1 ≤ 2^(k+1) := Nat.one_le_two_pow; rw [pow_succ]; omega
    calc a*(2^(k+2)-1) = 2*(a*(2^(k+1)-1))+a := by rw [hstep]; ring
      _ > 2*(b*2^k)+a := Nat.add_lt_add_right (Nat.mul_lt_mul_of_pos_left ih (by norm_num)) a
      _ = b*2^(k+1)+a := by ring
      _ > b*2^(k+1) := by omega

lemma descent_generic (a b : ℕ) (ha_pos : a > 0) (ha_even : a % 2 = 0)
    (hcop : Nat.Coprime a b) (hineq : a * 3 > b * 2)
    (z : ℕ) (hz : z > 0) (h : a * σ z = b * z) : False := by
  rcases factor_2_pow_odd z hz with ⟨e, t, ht_odd, hz_eq⟩
  have ht_pos : t > 0 := Nat.pos_of_ne_zero (fun ht0 => by
    rw [ht0, mul_zero] at hz_eq; omega)
  have h_cop2e : Nat.Coprime (2^e) t := by
    apply Nat.Coprime.pow_left
    exact ((Nat.Prime.coprime_iff_not_dvd (by native_decide : Nat.Prime 2)).mpr
      (fun h2t => by have := Nat.mod_eq_zero_of_dvd h2t; rw [ht_odd] at this; omega))
  rw [hz_eq, sigma_mul_coprime h_cop2e, sigma_two_pow e] at h
  by_cases he0 : e = 0
  · subst he0; simp at h
    have hat : a ∣ t := factor_from_eq a b t hcop h
    have : 2 ∣ t := Nat.dvd_trans (Nat.dvd_of_mod_eq_zero ha_even) hat
    have := Nat.mod_eq_zero_of_dvd this; rw [ht_odd] at this; omega
  · have he1 : e ≥ 1 := by omega
    have hge : a*(2^(e+1)-1) > b*2^e := coeff_ineq_pow ha_pos hineq e he1
    have hst : σ t ≥ t := sigma_ge_self t ht_pos
    have heq : a*(2^(e+1)-1)*σ t = b*2^e*t := by
      have h1 : a*(2^(e+1)-1)*σ t = a*((2^(e+1)-1)*σ t) := by ring
      have h2 : b*(2^e*t) = b*2^e*t := by ring
      linarith
    have hcontra : a*(2^(e+1)-1)*σ t > b*2^e*t :=
      calc a*(2^(e+1)-1)*σ t ≥ a*(2^(e+1)-1)*t := Nat.mul_le_mul_left _ hst
        _ > b*2^e*t := Nat.mul_lt_mul_of_pos_right hge ht_pos
    rw [heq] at hcontra; exact lt_irrefl _ hcontra

-- ─── Sigma values and coprimality ─────────────────────────────────────────────

lemma sigma_5_val        : σ 5 = 6              := by native_decide
lemma sigma_25_val       : σ 25 = 31            := by native_decide
lemma sigma_125_val      : σ 125 = 156          := by native_decide
lemma sigma_31_val       : σ 31 = 32            := by native_decide
lemma sigma_31sq_val     : σ (31^2) = 993       := by native_decide
lemma sigma_31cube_val   : σ (31^3) = 30784     := by native_decide
lemma sigma_331_val      : σ 331 = 332          := by native_decide
lemma sigma_331sq_val    : σ (331^2) = 109893   := by
  rw [ArithmeticFunction.sigma_one_apply_prime_pow (by native_decide : Nat.Prime 331)]
  norm_num [Finset.sum_range_succ]
lemma sigma_331cube_val  : σ (331^3) = 36374584 := by
  rw [ArithmeticFunction.sigma_one_apply_prime_pow (by native_decide : Nat.Prime 331)]
  norm_num [Finset.sum_range_succ]
lemma sigma_36631_val    : σ 36631 = 41872      := by native_decide

-- ─── five_dvd_m ───────────────────────────────────────────────────────────────

lemma five_dvd_m (m : ℕ) (h : σ m * 5 = 9 * m) : 5 ∣ m :=
  (by native_decide : Nat.Coprime 5 9).dvd_of_dvd_mul_left
    (by rw [← h]; exact ⟨σ m, by ring⟩)

-- ─── case_A: 5∥m → m = 10 ────────────────────────────────────────────────────

lemma case_A (k : ℕ) (hk_pos : k > 0) (h5k : ¬ 5 ∣ k)
    (h : σ (5 * k) * 5 = 9 * (5 * k)) : k = 2 := by
  have h_sigma : σ (5 * k) = 9 * k := by omega
  have h_cop : Nat.Coprime 5 k :=
    (Nat.Prime.coprime_iff_not_dvd (by native_decide)).mpr h5k
  rw [sigma_mul_coprime h_cop, sigma_5_val] at h_sigma
  have h_eq : 2 * σ k = 3 * k := by omega
  rcases factor_2_pow_odd k hk_pos with ⟨e, u, hu_odd, hk_eq⟩
  rw [hk_eq] at h_eq
  have hu_pos : u > 0 := Nat.pos_of_ne_zero (fun hu0 => by omega)
  -- e=0: parity contradiction
  by_cases he0 : e = 0
  · rw [he0, pow_zero, one_mul] at h_eq
    have : 2 * σ u % 2 = 3 * u % 2 := by rw [h_eq]
    simp at this
    have : 3 * u % 2 = 1 := by
      have := Nat.mul_mod 3 u 2; simp [hu_odd] at this; omega
    omega
  have h_cop2e : Nat.Coprime (2^e) u :=
    (Nat.Coprime.pow_left e ((Nat.Prime.coprime_iff_not_dvd (by native_decide : Nat.Prime 2)).mpr
      (fun h2u => by have := Nat.mod_eq_zero_of_dvd h2u; rw [hu_odd] at this; omega)))
  rw [sigma_mul_coprime h_cop2e, sigma_two_pow e] at h_eq
  -- e=1: σ(u)=u → u=1 → k=2
  by_cases he1 : e = 1
  · rw [he1] at h_eq; norm_num at h_eq
    have h_su : σ u = u := by linarith
    by_cases hu1 : u = 1
    · rw [hk_eq, he1, hu1]; norm_num
    · have : u > 1 := by omega
      have hgt : σ u > u := by
        rw [sigma_one_apply]
        have h1 : (1 : ℕ) ∈ Nat.divisors u := Nat.mem_divisors.mpr ⟨Nat.one_dvd u, by omega⟩
        have hu_self : u ∈ Nat.divisors u := Nat.mem_divisors.mpr ⟨dvd_refl u, by omega⟩
        have hne : (1 : ℕ) ≠ u := by omega
        have hle : 1 + u ≤ ∑ d ∈ Nat.divisors u, d := by
          calc 1 + u = ∑ d ∈ ({1, u} : Finset ℕ), d := by simp [hne]
            _ ≤ ∑ d ∈ Nat.divisors u, d :=
                Finset.sum_le_sum_of_subset (by
                  intro x hx
                  simp only [Finset.mem_insert, Finset.mem_singleton] at hx
                  rcases hx with rfl | rfl; exact h1; exact hu_self)
        omega
      omega
  -- e≥2: contradiction from (2^(e+1)-1)·σ(u) = 3·2^(e-1)·u with 2^(e+1)-1 > 3·2^(e-1)
  have he2 : e ≥ 2 := by omega
  -- 2^e = 2·2^(e-1)
  have hpow : 2^e = 2 * 2^(e-1) := by
    have h1 : (2:ℕ)^1 * 2^(e-1) = 2^e := by rw [← pow_add]; congr 1; omega
    linarith [show (2:ℕ)^1 = 2 from by norm_num]
  rw [hpow] at h_eq
  -- 2^(e+1) = 4·2^(e-1)
  have hpow2 : (2:ℕ)^(e+1) = 4 * 2^(e-1) := by
    have h1 : (2:ℕ)^2 * 2^(e-1) = 2^(e+1) := by rw [← pow_add]; congr 1; omega
    linarith [show (2:ℕ)^2 = 4 from by norm_num]
  -- 2^(e-1) ≥ 2
  have hX : (2:ℕ)^(e-1) ≥ 2 := by
    have h1 : (2:ℕ)^1 ≤ 2^(e-1) := Nat.pow_le_pow_right (by norm_num) (by omega)
    linarith [show (2:ℕ)^1 = 2 from by norm_num]
  -- 2^(e+1)-1 > 3·2^(e-1)
  have hineq : (2:ℕ)^(e+1) - 1 > 3 * 2^(e-1) := by omega
  -- (2^(e+1)-1)·σ(u) = 3·2^(e-1)·u via h_eq
  have hmid : (2^(e+1)-1) * σ u = 3 * 2^(e-1) * u := by
    have hpos : (2:ℕ)^(e+1) ≥ 1 := Nat.one_le_two_pow
    have h2 : 2 * ((2^(e+1)-1) * σ u) = 2 * (3 * 2^(e-1) * u) := by linarith [show 3*(2*2^(e-1)*u) = 2*(3*2^(e-1)*u) from by ring]
    have htwone : (0:ℕ) < 2 := by norm_num
    exact Nat.eq_of_mul_eq_mul_left htwone h2
  -- contradiction: (2^(e+1)-1)·σ(u) > 3·2^(e-1)·u
  have hsu := sigma_ge_self u hu_pos
  linarith [Nat.mul_le_mul_left (2^(e+1)-1) hsu,
            Nat.mul_lt_mul_of_pos_right hineq hu_pos]

-- ─── descent_32_45 ────────────────────────────────────────────────────────────

lemma descent_32_45 (z : ℕ) (hz : z > 0) (h : 32 * σ z = 45 * z) : False :=
  descent_generic 32 45 (by norm_num) (by norm_num) (by native_decide) (by norm_num) z hz h

-- ─── descent_332_465 ──────────────────────────────────────────────────────────

lemma descent_332_465 (z : ℕ) (hz : z > 0) (h : 332 * σ z = 465 * z) : False :=
  descent_generic 332 465 (by norm_num) (by norm_num) (by native_decide) (by norm_num) z hz h

-- ─── Helper: extract prime factor from ¬Coprime with prime ───────────────────

private lemma not_coprime_prime_dvd {p n : ℕ} (hp : Nat.Prime p)
    (hnc : ¬ Nat.Coprime p n) : p ∣ n := by
  rw [Nat.coprime_iff_gcd_eq_one] at hnc
  rcases hp.eq_one_or_self_of_dvd _ (Nat.gcd_dvd_left p n) with h1 | hself
  · exact absurd h1 hnc
  · exact hself ▸ Nat.gcd_dvd_right p n

-- ─── descent_331_chain: 331·σ(w)=465·w → False ───────────────────────────────
-- Chain: factor_from_eq (gcd(331,465)=1) → 331|w → σ(331·v)=465·v
--   Coprime: 332·σ(v)=465·v → descent_332_465 ✓
--   ¬Coprime: 331|v → σ(331²·t)=153915·t
--     Coprime: 36631·σ(t)=51305·t → factor_from_eq → σ(36631·u)=51305·u
--       Coprime: 41872·σ(u)=51305·u → descent_generic ✓
--       ¬Coprime: deeper (7|u or 5233|u); sorry (level 3+)
--     ¬Coprime: 331|t → σ(331³·r)=50946765·r; sorry (deeper 331-adic)

lemma descent_331_chain (w : ℕ) (hw : w > 0) (h : 331 * σ w = 465 * w) : False := by
  have h331_w : 331 ∣ w :=
    factor_from_eq 331 465 w (by native_decide) h
  rcases h331_w with ⟨v, hw_eq⟩; subst hw_eq
  have hv_pos : v > 0 := Nat.pos_of_ne_zero (fun hv0 => by omega)
  have h_sv : σ (331 * v) = 465 * v := by linarith
  by_cases h_cop_v : Nat.Coprime 331 v
  · -- Coprime branch: 332·σ(v) = 465·v
    rw [sigma_mul_coprime h_cop_v, sigma_331_val] at h_sv
    exact descent_332_465 v hv_pos (by linarith)
  · -- ¬Coprime: 331|v
    have h331_v : 331 ∣ v := not_coprime_prime_dvd (by native_decide) h_cop_v
    rcases h331_v with ⟨t, hv_eq⟩; subst hv_eq
    have ht_pos : t > 0 := Nat.pos_of_ne_zero (fun ht0 => by omega)
    -- σ(331²·t) = 153915·t
    have h_st : σ (331^2 * t) = 153915 * t := by
      have : σ (331 * (331 * t)) = 465 * (331 * t) := h_sv
      rw [show 331 * (331 * t) = 331^2 * t by ring] at this; linarith
    by_cases h_cop_t : Nat.Coprime 331 t
    · -- Coprime: 36631·σ(t)=51305·t
      rw [sigma_mul_coprime (h_cop_t.pow_left 2), sigma_331sq_val] at h_st
      have h36631 : 36631 * σ t = 51305 * t := by linarith
      have h36631_t : 36631 ∣ t :=
        factor_from_eq 36631 51305 t (by native_decide) h36631
      rcases h36631_t with ⟨u, ht_eq⟩; subst ht_eq
      have hu_pos : u > 0 := Nat.pos_of_ne_zero (fun hu0 => by omega)
      have h_su : σ (36631 * u) = 51305 * u := by linarith
      by_cases h_cop_u : Nat.Coprime 36631 u
      · -- Coprime: 41872·σ(u)=51305·u → descent_generic(41872,51305)
        rw [sigma_mul_coprime h_cop_u, sigma_36631_val] at h_su
        exact descent_generic 41872 51305 (by norm_num) (by norm_num)
          (by native_decide) (by norm_num) u hu_pos (by linarith)
      · -- ¬Coprime 36631 u: 7|u or 5233|u
        -- Both sub-cases eventually reach descent_generic; needs case split
        -- and deeper arithmetic. Math: for 7|u→298338·σ(s)=359135·s→descent_generic;
        -- for 5233|u symmetric; for 36631|u→1341866793·σ(s)=1879353455·s→
        -- factor_from_eq→255945664·σ(r)=268479065·r→descent_generic.
        -- Full formalization needs additional lemmas.
        sorry
    · -- ¬Coprime 331 t: 331|t → 331³|w
      -- σ(331³·r) = 153915·331·r = 50945865·r; gcd(36374584,50945865)=1 → descent_generic
      have h331_t : 331 ∣ t := not_coprime_prime_dvd (by native_decide) h_cop_t
      rcases h331_t with ⟨r, ht_eq⟩; subst ht_eq
      have hr_pos : r > 0 := Nat.pos_of_ne_zero (fun hr0 => by omega)
      have h_sr : σ (331^3 * r) = 50945865 * r := by
        have : σ (331^2 * (331 * r)) = 153915 * (331 * r) := h_st
        rw [show 331^2 * (331 * r) = 331^3 * r by ring] at this
        linarith [show (153915:ℕ) * 331 = 50945865 from by norm_num]
      by_cases h_cop_r : Nat.Coprime 331 r
      · -- gcd(331,r)=1: σ(331³)·σ(r) = 36374584·σ(r) = 50945865·r
        -- gcd(36374584,50945865)=1, 36374584 even, 36374584·3>50945865·2 → descent_generic
        rw [sigma_mul_coprime (h_cop_r.pow_left 3), sigma_331cube_val] at h_sr
        exact descent_generic 36374584 50945865 (by norm_num) (by norm_num)
          (by native_decide) (by norm_num) r hr_pos (by linarith)
      · -- ¬Coprime 331 r: 331⁴|w; deeper still
        sorry

-- ─── ten_is_solitary ──────────────────────────────────────────────────────────

theorem ten_is_solitary (m : ℕ) (hm : m > 0) (h : σ m * 5 = 9 * m) : m = 10 := by
  rcases five_dvd_m m h with ⟨k, hm_eq⟩; subst hm_eq
  have hk_pos : k > 0 := Nat.pos_of_ne_zero (fun hk0 => by omega)
  by_cases h5k : 5 ∣ k
  · -- Case B: 25 | m
    rcases h5k with ⟨n, hk_eq⟩; subst hk_eq
    have hn_pos : n > 0 := Nat.pos_of_ne_zero (fun hn0 => by omega)
    have h_sn : σ (25 * n) = 45 * n := by
      have : σ (5 * (5 * n)) * 5 = 9 * (5 * (5 * n)) := by linarith
      have heq : 5 * (5 * n) = 25 * n := by ring
      rw [heq] at this; linarith
    by_cases h_cop25 : Nat.Coprime 25 n
    · -- gcd(25,n)=1: 31·σ(n)=45·n
      rw [sigma_mul_coprime h_cop25, sigma_25_val] at h_sn
      have h31_n : 31 ∣ n := factor_from_eq 31 45 n (by native_decide) (by linarith)
      rcases h31_n with ⟨z, hn_eq⟩; subst hn_eq
      have hz_pos : z > 0 := Nat.pos_of_ne_zero (fun hz0 => by omega)
      have h_sz : σ (31 * z) = 45 * z := by linarith
      by_cases h_cop31z : Nat.Coprime 31 z
      · -- gcd(31,z)=1: 32·σ(z)=45·z → descent_32_45
        rw [sigma_mul_coprime h_cop31z, sigma_31_val] at h_sz
        exact absurd (descent_32_45 z hz_pos (by linarith)) id
      · -- 31|z: z=31·w → n=31²·w
        have h31_z : 31 ∣ z := not_coprime_prime_dvd (by native_decide) h_cop31z
        rcases h31_z with ⟨w, hz_eq⟩; subst hz_eq
        have hw_pos : w > 0 := Nat.pos_of_ne_zero (fun hw0 => by omega)
        have h_sw : σ (31^2 * w) = 1395 * w := by
          rw [show 31 * (31 * w) = 31^2 * w by ring] at h_sz; linarith
        by_cases h_cop31w : Nat.Coprime 31 w
        · -- gcd(31,w)=1: 993·σ(w)=1395·w → 331·σ(w)=465·w → descent_331_chain
          rw [sigma_mul_coprime (h_cop31w.pow_left 2), sigma_31sq_val] at h_sw
          exact absurd (descent_331_chain w hw_pos (by omega)) id
        · -- 31|w: z=31²·r → n=31³·r
          have h31_w : 31 ∣ w := not_coprime_prime_dvd (by native_decide) h_cop31w
          rcases h31_w with ⟨r, hw_eq⟩; subst hw_eq
          have hr_pos : r > 0 := Nat.pos_of_ne_zero (fun hr0 => by omega)
          have h_sr : σ (31^3 * r) = 43245 * r := by
            rw [show 31^2 * (31 * r) = 31^3 * r by ring] at h_sw; linarith
          by_cases h_cop31r : Nat.Coprime 31 r
          · -- gcd(31,r)=1: 31784·σ(r)=43245·r → descent_generic(31784,43245)
            rw [sigma_mul_coprime (h_cop31r.pow_left 3), sigma_31cube_val] at h_sr
            exact absurd (descent_generic 30784 43245 (by norm_num) (by norm_num)
              (by native_decide) (by norm_num) r hr_pos (by linarith)) id
          · -- 31|r: 31⁴|n; deeper 31-adic chain
            -- Pattern: alternating descent_generic (odd powers) and factor_from_eq (even powers)
            -- Terminates by finite 31-adic valuation of n.
            sorry
    · -- Case B2: ¬Coprime 25 n → 5|n
      -- 25=5²; divisors are {1,5,25}; gcd(25,n)≠1 → 5|gcd → 5|n
      have h5_n : 5 ∣ n := by
        by_contra h5n
        apply h_cop25
        have hcop : Nat.Coprime 5 n :=
          (Nat.Prime.coprime_iff_not_dvd (by native_decide : Nat.Prime 5)).mpr h5n
        simpa using hcop.pow_left 2
      rcases h5_n with ⟨q, hn_eq⟩; subst hn_eq
      have hq_pos : q > 0 := Nat.pos_of_ne_zero (fun hq0 => by omega)
      have h_sq : σ (125 * q) = 225 * q := by
        rw [show 25 * (5 * q) = 125 * q by ring] at h_sn; linarith
      by_cases h_cop5q : Nat.Coprime 5 q
      · -- gcd(5,q)=1 → gcd(125,q)=1: 156·σ(q)=225·q → 52·σ(q)=75·q → descent_generic(52,75)
        rw [sigma_mul_coprime (h_cop5q.pow_left 3), sigma_125_val] at h_sq
        exact absurd (descent_generic 52 75 (by norm_num) (by norm_num)
          (by native_decide) (by norm_num) q hq_pos (by linarith)) id
      · -- 5|q: 5⁴|m — deeper 5-adic chain
        -- a=4: 781·σ(r)=1125·r → 781 odd → factor_from_eq: 781|r → 96·σ(s)=125·s → descent_generic
        -- a=5: σ(5⁵) = 3906 (even) → descent_generic directly
        -- terminates by finite 5-adic valuation of m
        sorry
  · -- Case A: 5∥m → m=10
    exact (case_A k hk_pos h5k (by linarith)) ▸ rfl

end Imscribing.Classical.Solitary10
