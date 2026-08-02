-- Imscribing/Millennium/PerfectCuboid/FactorizationLemma.lean
-- FACTORIZATION LEMMAS L8, L9, L10 — Factor-Pair Alignment, Cross-GCD Descent
-- Author: Lando ⊗ ⊙perator
--
-- L8: Factor-Pair Square Decomposition — if M·N=b² and gcd(M,N)∈{1,2} then
--     M = d·u², N = d·v² with d = gcd(M,N)
-- L9: Cuboid Factor-Pair Decomposition — applies L8 to the three
--     cuboid factorizations (g-d,g+d)(g-e,g+e)(g-f,g+f)
-- L10: Consistency Consequences — derives formulas for a,b,c in terms
--     of square factors and sum formulas

import Imscribing.Millennium.PerfectCuboid
import Imscribing.Millennium.PerfectCuboid.PrerequisiteLemmasL5_L7
import Mathlib.Data.Nat.Basic
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.Data.Int.Basic
import Mathlib.Data.Int.GCD
import Mathlib.NumberTheory.PythagoreanTriples
import Mathlib.Tactic

open Millennium.PerfectCuboid
open Nat

set_option linter.style.longLine false

namespace Millennium.PerfectCuboid.FactorizationLemma

/- ====================================================================
   L8-AUX: Any divisor of 2 is 1 or 2.
   ==================================================================== -/

/-- Any divisor of 2 is 1 or 2. -/
lemma dvd_two_eq_one_or_two {d : Nat} (hd : d ∣ 2) : d = 1 ∨ d = 2 := by
  have h_le : d ≤ 2 := Nat.le_of_dvd (by norm_num) hd
  interval_cases d
  · norm_num at hd
  · left; rfl
  · right; rfl

/- ====================================================================
   L8-AUX: Coprime Square Lemma (Int bridge)
   ==================================================================== -/

/-- If a,b positive coprime and a*b = c*c, then a,b are squares. -/
lemma coprime_square_factor_nat {a b c : Nat} (ha_pos : 0 < a) (hb_pos : 0 < b)
    (h_cop : Nat.Coprime a b) (h_prod : a * b = c * c) :
    ∃ (u v : Nat), a = u * u ∧ b = v * v := by
  have h_cop_int : ((a : Int).gcd (b : Int)) = 1 := by
    rw [Int.gcd_natCast_natCast]; exact mod_cast h_cop
  have h_prod_int : (a : Int) * (b : Int) = ((c : Int)) ^ 2 := by
    have h := congrArg (fun (x : Nat) => (x : Int)) h_prod
    simpa [sq] using h
  rcases Int.sq_of_gcd_eq_one h_cop_int h_prod_int with ⟨a0, ha_cases⟩
  rcases ha_cases with (ha_eq | ha_neg)
  · have h_cop_int' : ((b : Int).gcd (a : Int)) = 1 := by
      rw [Int.gcd_comm, h_cop_int]
    have h_prod_int' : (b : Int) * (a : Int) = ((c : Int)) ^ 2 := by
      rw [mul_comm, h_prod_int]
    rcases Int.sq_of_gcd_eq_one h_cop_int' h_prod_int' with ⟨b0, hb_cases⟩
    rcases hb_cases with (hb_eq | hb_neg)
    · set u := a0.natAbs with hu_def
      set v := b0.natAbs with hv_def
      have hu_sq_int : (u : Int) * (u : Int) = (a : Int) := by
        dsimp [u]
        have h : ((a0.natAbs : Int) * (a0.natAbs : Int)) = a0 * a0 := by
          simpa using (Int.natAbs_mul_self (a := a0))
        rw [h, ha_eq, sq]
      have hv_sq_int : (v : Int) * (v : Int) = (b : Int) := by
        dsimp [v]
        have h : ((b0.natAbs : Int) * (b0.natAbs : Int)) = b0 * b0 := by
          simpa using (Int.natAbs_mul_self (a := b0))
        rw [h, hb_eq, sq]
      have hu_nat_sq : a = u * u := by exact_mod_cast hu_sq_int.symm
      have hv_nat_sq : b = v * v := by exact_mod_cast hv_sq_int.symm
      exact ⟨u, v, hu_nat_sq, hv_nat_sq⟩
    · have hb_int_pos : 0 < (b : Int) := by exact_mod_cast hb_pos
      rw [hb_neg] at hb_int_pos
      nlinarith
  · have ha_int_pos : 0 < (a : Int) := by exact_mod_cast ha_pos
    rw [ha_neg] at ha_int_pos
    nlinarith

/- ====================================================================
   L8: FACTOR-PAIR SQUARE DECOMPOSITION
   If M·N = b² and gcd(M,N) ∣ 2, then M,N are d·(squares).
   ==================================================================== -/

/-- L8: If M·N=b² and gcd(M,N)∣2 then M,N are d·(squares). -/
lemma factor_pair_square_decomp {M N b : Nat} (hMpos : 0 < M) (hNpos : 0 < N)
    (h_prod : M * N = b * b) (h_gcd_dvd2 : Nat.gcd M N ∣ 2) :
    ∃ (u v : Nat), M = (Nat.gcd M N) * (u * u) ∧ N = (Nat.gcd M N) * (v * v) := by
  rcases dvd_two_eq_one_or_two h_gcd_dvd2 with (hd1 | hd2)
  · -- Case gcd = 1
    have h_cop : Nat.Coprime M N := by
      rw [Nat.coprime_iff_gcd_eq_one, hd1]
    rcases coprime_square_factor_nat hMpos hNpos h_cop h_prod with ⟨u, v, hu, hv⟩
    refine ⟨u, v, ?_, ?_⟩
    · rw [hd1, hu]; ring
    · rw [hd1, hv]; ring
  · -- Case gcd = 2
    have h2M : 2 ∣ M := by rw [← hd2]; exact Nat.gcd_dvd_left _ _
    have h2N : 2 ∣ N := by rw [← hd2]; exact Nat.gcd_dvd_right _ _
    rcases h2M with ⟨M', hM'⟩
    rcases h2N with ⟨N', hN'⟩
    have hM'pos : 0 < M' := by
      have : 0 < 2 * M' := by rw [← hM']; exact hMpos
      omega
    have hN'pos : 0 < N' := by
      have : 0 < 2 * N' := by rw [← hN']; exact hNpos
      omega
    have h2b : 2 ∣ b := by
      have hprime2 : Nat.Prime 2 := by norm_num
      have h4_div : 4 ∣ b * b := by
        rw [← h_prod, hM', hN']
        refine ⟨M' * N', ?_⟩
        ring
      have h2_div_b_sq : 2 ∣ b * b :=
        Nat.dvd_trans (by norm_num : 2 ∣ 4) h4_div
      refine hprime2.dvd_of_dvd_pow (n := 2) ?_
      simpa [sq] using h2_div_b_sq
    rcases h2b with ⟨b', hb'⟩
    have h_prod' : M' * N' = b' * b' := by
      rw [hM', hN', hb'] at h_prod
      have hpos4 : 0 < 4 := by norm_num
      apply (Nat.eq_of_mul_eq_mul_left hpos4)
      calc
        4 * (M' * N') = (2*M') * (2*N') := by ring
        _ = (2*b') * (2*b') := h_prod
        _ = 4 * (b' * b') := by ring
    have h_cop' : Nat.Coprime M' N' := by
      rw [Nat.coprime_iff_gcd_eq_one]
      by_contra h_not
      have h_gt1 : 1 < Nat.gcd M' N' := by
        have h_ne_zero : Nat.gcd M' N' ≠ 0 := by
          intro hzero
          have hM0 : M' = 0 := Nat.eq_zero_of_gcd_eq_zero_left hzero
          have hN0 : N' = 0 := Nat.eq_zero_of_gcd_eq_zero_right hzero
          subst hM0; subst hN0; omega
        omega
      have h_ne_one : Nat.gcd M' N' ≠ 1 := by omega
      obtain ⟨p, hp_prime, hp_dvd_gcd⟩ := Nat.exists_prime_and_dvd h_ne_one
      have hp_two_le : 2 ≤ p := Nat.Prime.two_le hp_prime
      have hp_M' : p ∣ M' := Nat.dvd_trans hp_dvd_gcd (Nat.gcd_dvd_left _ _)
      have hp_N' : p ∣ N' := Nat.dvd_trans hp_dvd_gcd (Nat.gcd_dvd_right _ _)
      have hp_2M : 2*p ∣ M := by
        rw [hM']; exact mul_dvd_mul (dvd_refl 2) hp_M'
      have hp_2N : 2*p ∣ N := by
        rw [hN']; exact mul_dvd_mul (dvd_refl 2) hp_N'
      have hp_gcd : 2*p ∣ Nat.gcd M N := Nat.dvd_gcd hp_2M hp_2N
      rw [hd2] at hp_gcd
      have h2p_eq_2 : 2*p = 2 := by
        apply Nat.dvd_antisymm hp_gcd
        exact ⟨p, by ring⟩
      have hp_eq_1 : p = 1 := by omega
      exact (Nat.Prime.ne_one hp_prime) hp_eq_1
    rcases coprime_square_factor_nat hM'pos hN'pos h_cop' h_prod' with ⟨u, v, hu, hv⟩
    have hM_target : M = (Nat.gcd M N) * (u * u) := by
      rw [hd2, hM', hu]
    have hN_target : N = (Nat.gcd M N) * (v * v) := by
      rw [hd2, hN', hv]
    exact ⟨u, v, hM_target, hN_target⟩
