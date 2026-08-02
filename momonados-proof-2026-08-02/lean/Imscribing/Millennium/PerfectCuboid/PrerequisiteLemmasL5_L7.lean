-- Imscribing/Millennium/PerfectCuboid/PrerequisiteLemmasL5_L7.lean
-- PREREQUISITE LEMMAS L5, L6, L7
-- Author: Lando ⊗ ⊙perator
--
-- These lemmas provide the foundation for the Factorization Lemmas L8–L10.
-- L5: Euclid's Lemma — if a|b*c and gcd(a,b)=1 then a|c
-- L6: Coprime Square Lemma — if gcd(a,b)=1 and a*b=c*c then a,b are squares
-- L7: Pythagorean Parameterization — (m²-n², 2mn, m²+n²) is primitive
--   Pythagorean triple when m,n are coprime with opposite parity

import Mathlib.Data.Nat.Basic
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.Data.Int.Basic
import Mathlib.Data.Int.GCD
import Mathlib.NumberTheory.PythagoreanTriples
import Mathlib.Tactic

open Nat

set_option linter.style.longLine false

namespace Millennium.PerfectCuboid.PrerequisiteLemmas

/- ====================================================================
   L5 — Euclid's Lemma for ℕ
   If a ∣ b*c and Nat.Coprime a b, then a ∣ c.
   Uses Mathlib's Nat.Coprime.dvd_of_dvd_mul_left.
   ==================================================================== -/

/-- L5 — Euclid's Lemma (direct one-liner via Mathlib). -/
theorem Euclid_lemma_nat {a b c : ℕ} (h_div : a ∣ b * c) (h_cop : Nat.Coprime a b) : a ∣ c :=
  h_cop.dvd_of_dvd_mul_left h_div

/- ====================================================================
   L6 — Coprime Square Lemma
   If a,b are coprime positive naturals and a*b = c*c, then a and b are squares.
   Uses Int.sq_of_gcd_eq_one from Mathlib.
   ==================================================================== -/

/-- L6 — If a,b coprime positive and a*b = c*c then a,b are squares. -/
theorem coprime_sq_factor {a b c : ℕ} (ha_pos : 0 < a) (hb_pos : 0 < b)
    (h_cop : Nat.Coprime a b) (h_prod : a * b = c * c) :
    ∃ (u v : ℕ), a = u * u ∧ b = v * v := by
  have h_cop_int : ((a : ℤ).gcd (b : ℤ)) = 1 := by
    rw [Int.gcd_natCast_natCast]
    exact mod_cast h_cop
  have h_prod_int : (a : ℤ) * (b : ℤ) = ((c : ℤ)) ^ 2 := by
    have h := congrArg (fun (x : ℕ) => (x : ℤ)) h_prod
    simpa [sq] using h
  rcases Int.sq_of_gcd_eq_one h_cop_int h_prod_int with ⟨a0, ha_cases⟩
  rcases ha_cases with (ha_eq | ha_neg)
  · have h_cop_int' : ((b : ℤ).gcd (a : ℤ)) = 1 := by
      rw [Int.gcd_comm, h_cop_int]
    have h_prod_int' : (b : ℤ) * (a : ℤ) = ((c : ℤ)) ^ 2 := by
      rw [mul_comm, h_prod_int]
    rcases Int.sq_of_gcd_eq_one h_cop_int' h_prod_int' with ⟨b0, hb_cases⟩
    rcases hb_cases with (hb_eq | hb_neg)
    · set u := a0.natAbs with hu_def
      set v := b0.natAbs with hv_def
      have hu_sq_int : (u : ℤ) * (u : ℤ) = (a : ℤ) := by
        dsimp [u]
        have h : ((a0.natAbs : ℤ) * (a0.natAbs : ℤ)) = a0 * a0 := by
          simpa using (Int.natAbs_mul_self (a := a0))
        rw [h, ha_eq, sq]
      have hv_sq_int : (v : ℤ) * (v : ℤ) = (b : ℤ) := by
        dsimp [v]
        have h : ((b0.natAbs : ℤ) * (b0.natAbs : ℤ)) = b0 * b0 := by
          simpa using (Int.natAbs_mul_self (a := b0))
        rw [h, hb_eq, sq]
      have hu_nat_sq : a = u * u := by exact_mod_cast hu_sq_int.symm
      have hv_nat_sq : b = v * v := by exact_mod_cast hv_sq_int.symm
      exact ⟨u, v, hu_nat_sq, hv_nat_sq⟩
    · have hb_int_pos : 0 < (b : ℤ) := by exact_mod_cast hb_pos
      rw [hb_neg] at hb_int_pos
      nlinarith
  · have ha_int_pos : 0 < (a : ℤ) := by exact_mod_cast ha_pos
    rw [ha_neg] at ha_int_pos
    nlinarith
/- ====================================================================
   L7 — Pythagorean Parameterization Lemma
   If integers m,n are coprime and have opposite parity,
   then (m²-n², 2mn, m²+n²) is a primitive Pythagorean triple.

   Uses Mathlib.NumberTheory.PythagoreanTriples.IsPrimitiveClassified,
   which is defined such that the parametric condition (m,n coprime
   with opposite parity) directly gives primitive classification.
   ==================================================================== -/

/-- L7 — Primitive Pythagorean triple parameterization (ℤ version).
    If m,n ∈ ℤ are coprime with opposite parity, then the
    standard parametric formulas give a primitive Pythagorean triple. -/
theorem primitive_pythagorean_param {m n : ℤ} (h_cop : Int.gcd m n = 1)
    (h_parity : (m % 2 = 0 ∧ n % 2 = 1) ∨ (m % 2 = 1 ∧ n % 2 = 0)) :
    (∃ (h : PythagoreanTriple (m ^ 2 - n ^ 2) (2 * m * n) (m ^ 2 + n ^ 2)), h.IsPrimitiveClassified) := by
  have h_triple : PythagoreanTriple (m ^ 2 - n ^ 2) (2 * m * n) (m ^ 2 + n ^ 2) := by
    unfold PythagoreanTriple
    ring
  refine ⟨h_triple, ?_⟩
  unfold PythagoreanTriple.IsPrimitiveClassified
  refine ⟨m, n, ?_, h_cop, h_parity⟩
  left
  exact ⟨by ring, by ring⟩

end Millennium.PerfectCuboid.PrerequisiteLemmas