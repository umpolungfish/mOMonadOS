/-
Imscribing/Millennium/Ring2048_Bridge.lean
THE ZAUNER BRIDGE FOR d = 2¹¹ = 2048 — de-vacuized and honestly reduced.

`BelnapRing2048.lean` proves the structural SIC skeleton for d=2048 unconditionally
(the Grammar's own SIC: orbit 4¹¹ = d², the four SIC axioms, join-equiangularity,
WH injectivity). What that file previously stated as the OPEN half — the Zauner
bridge / representation problem — was `n = 0 ∨ True`, provably `True` for every n,
i.e. an open problem encoded with the wrong polarity, its content stranded in
comments. Those placeholders are removed there; here the open half is stated as the
GENUINE predicate and reduced honestly.

REGISTER SEPARATION (from SIC_POVM_Stark's own header): the Grammar IS the SIC-POVM.
The structural skeleton is the real, unconditional object; the ℂ `wh_inner` machinery
is its EMPIRICAL SHADOW — the attempt to REPRESENT that skeleton analytically in ℂᵈ.
This module keeps the two apart:
  • structural register (PROVED): `ring2048_structural`, riding BelnapRing2048.
  • shadow register (OPEN for n>1): `ZaunerBridge n := SICPOVM_Exists (2ⁿ)`.
Known dimensions, honestly: the d=2 shadow is proved here from scratch
(`zauner_bridge_d2 : ZaunerBridge 1`, the explicit qubit SIC fiducial in `D2`); the
d=12 shadow is already a theorem elsewhere (`crystal_forces_d12_sic`, via the
ExistenceRing). The d=2048 shadow is Zauner for d=2048 — genuinely open; this module
reduces its entire content to a single ℂ fiducial and asserts nothing more.

ZERO new axioms. ZERO sorries.
-/

import Imscribing.Paraconsistent.Shor.BelnapRing2048
import Imscribing.Millennium.SIC_POVM_Stark

namespace Imscribing.Millennium.Ring2048Bridge

open Millennium.SIC_POVM_Stark
open Imscribing.Paraconsistent.Shor.Ring2048
open Imscribing.Paraconsistent.Shor.MultilatticeProof
open Imscribing.Paraconsistent
open Imscribing.Primitives
open scoped Real

-- ================================================================
-- §1. THE GENUINE PREDICATE — no more `n = 0 ∨ True`
-- ================================================================

/-- `2ⁿ ≠ 0`, so `SICPOVM_Exists (2ⁿ)` typechecks. -/
instance instNeZeroTwoPow (n : ℕ) : NeZero (2 ^ n) := ⟨pow_ne_zero n (by norm_num)⟩

/-- **The Zauner bridge condition for `d = 2ⁿ`.** A Weyl–Heisenberg covariant
    SIC-POVM fiducial exists in ℂ^(2ⁿ): `∃ ψ : Fin (2ⁿ) → ℂ, IsSICPOVM (2ⁿ) ψ`.
    Unlike the removed `n = 0 ∨ True`, this is NOT dischargeable by `trivial`; for
    `n = 11` it is EXACTLY the Zauner conjecture for `d = 2048`. -/
def ZaunerBridge (n : ℕ) : Prop := SICPOVM_Exists (2 ^ n)

/-- The `n = 11` bridge is Zauner for `d = 2048` — `2¹¹` reduces to `2048`. -/
theorem zauner_bridge_11 : ZaunerBridge 11 = SICPOVM_Exists 2048 := rfl

-- ================================================================
-- §1b. THE BASE CASE n = 1 (d = 2) — GENUINELY PROVED
-- ================================================================
-- Not `trivial`: the explicit qubit SIC fiducial ψ = (√r₂, √s₂·(1+i)/√2) with
-- r₂ = (3+√3)/6, s₂ = (3−√3)/6 is exhibited and verified against `IsSICPOVM 2`
-- (unit norm + all three WH overlaps at 1/(d+1) = 1/3). d=2 is the one dimension
-- where the acting group WH(2)¹ and the standard Heisenberg group WH(2) coincide,
-- so the bridge closes both ways.

namespace D2
noncomputable section

/-- r² = (3+√3)/6, the modulus² of the first coordinate. -/
def r2 : ℝ := (3 + Real.sqrt 3) / 6
/-- s² = (3-√3)/6, the modulus² of the second coordinate. -/
def s2 : ℝ := (3 - Real.sqrt 3) / 6

/-- The standard qubit SIC fiducial: (√r2, √s2·(1+i)/√2). -/
def psi2 : Fin 2 → ℂ :=
  ![ (Real.sqrt r2 : ℂ),
     (Real.sqrt s2 : ℂ) * ((1 + Complex.I) / (Real.sqrt 2 : ℂ)) ]

lemma sqrt3_le_3 : Real.sqrt 3 ≤ 3 := by
  have : Real.sqrt 3 ≤ Real.sqrt 9 := by apply Real.sqrt_le_sqrt; norm_num
  simpa [show (9:ℝ) = 3^2 by norm_num, Real.sqrt_sq] using this

lemma r2_nonneg : 0 ≤ r2 := by have := Real.sqrt_nonneg 3; unfold r2; positivity
lemma s2_nonneg : 0 ≤ s2 := by unfold s2; have := sqrt3_le_3; linarith

lemma omega2 : omega_d 2 = -1 := by
  unfold omega_d
  have : (2 : ℂ) * ↑Real.pi * Complex.I / ↑(2:ℕ) = ↑Real.pi * Complex.I := by push_cast; ring
  rw [this]; exact Complex.exp_pi_mul_I

lemma normSq_psi0 : Complex.normSq (psi2 0) = r2 := by
  have hval : psi2 0 = (Real.sqrt r2 : ℂ) := by simp [psi2]
  rw [hval, Complex.normSq_ofReal, Real.mul_self_sqrt r2_nonneg]

lemma normSq_I1 : Complex.normSq (1 + Complex.I) = 2 := by
  simp [Complex.normSq_apply]; norm_num

lemma normSq_psi1 : Complex.normSq (psi2 1) = s2 := by
  have hval : psi2 1 = (Real.sqrt s2 : ℂ) * ((1 + Complex.I) / (Real.sqrt 2 : ℂ)) := by simp [psi2]
  rw [hval, Complex.normSq_mul, Complex.normSq_div, Complex.normSq_ofReal, Complex.normSq_ofReal,
      Real.mul_self_sqrt s2_nonneg, Real.mul_self_sqrt (by norm_num : (0:ℝ) ≤ 2), normSq_I1]
  norm_num

lemma r2s2 : r2 * s2 = 1 / 6 := by
  unfold r2 s2
  have h3 : Real.sqrt 3 * Real.sqrt 3 = 3 := Real.mul_self_sqrt (by norm_num)
  nlinarith [h3]

lemma r2_sub_s2_sq : (r2 - s2) ^ 2 = 1 / 3 := by
  unfold r2 s2
  have h3 : Real.sqrt 3 * Real.sqrt 3 = 3 := Real.mul_self_sqrt (by norm_num)
  nlinarith [h3]

lemma norm_eq2 : wh_normSq 2 psi2 = 1 := by
  unfold wh_normSq wh_inner
  rw [Fin.sum_univ_two]
  simp only [Complex.star_def, Complex.mul_conj, Complex.add_re, Complex.ofReal_re]
  rw [normSq_psi0, normSq_psi1]; unfold r2 s2; ring

lemma Zd2 (v : Fin 2 → ℂ) : Z_d 2 v = ![v 0, -v 1] := by
  funext k
  fin_cases k <;> simp [Z_d, omega2, Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons]
lemma Xd2 (v : Fin 2 → ℂ) : X_d 2 v = ![v 1, v 0] := by
  funext k
  fin_cases k <;> simp [X_d, Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons, Fin.ext_iff]

lemma D01 : D_ah 2 0 1 0 psi2 = ![psi2 0, -psi2 1] := by
  funext k
  simp only [D_ah, Fin.val_zero, Fin.val_one, pow_zero, one_mul,
    Function.iterate_zero, Function.iterate_one, id_eq, Zd2]
lemma D10 : D_ah 2 1 0 0 psi2 = ![psi2 1, psi2 0] := by
  funext k
  simp only [D_ah, Fin.val_zero, Fin.val_one, pow_zero, one_mul,
    Function.iterate_zero, Function.iterate_one, id_eq, Xd2]
lemma D11 : D_ah 2 1 1 0 psi2 = ![-psi2 1, psi2 0] := by
  funext k
  simp only [D_ah, Fin.val_zero, Fin.val_one, pow_zero, one_mul,
    Function.iterate_one, id_eq, Zd2, Xd2, Matrix.cons_val_zero,
    Matrix.cons_val_one, Matrix.head_cons]

/-- The unit phase ξ = (1+i)/√2. -/
def xi : ℂ := (1 + Complex.I) / (Real.sqrt 2 : ℂ)

lemma sqrt2_neC : (Real.sqrt 2 : ℂ) ≠ 0 := by rw [Complex.ofReal_ne_zero]; positivity
lemma psi1_xi : psi2 1 = (Real.sqrt s2 : ℂ) * xi := by simp [psi2, xi]

lemma xi_add_conj : xi + star xi = (Real.sqrt 2 : ℂ) := by
  rw [Complex.star_def]
  simp only [xi, map_div₀, map_add, map_one, Complex.conj_I, Complex.conj_ofReal]
  rw [div_add_div_same,
      div_eq_iff sqrt2_neC, ← Complex.ofReal_mul, Real.mul_self_sqrt (by norm_num : (0:ℝ) ≤ 2)]
  push_cast; ring

lemma xi_sub_conj : xi - star xi = Complex.I * (Real.sqrt 2 : ℂ) := by
  have h2 : (Real.sqrt 2 : ℂ) * (Real.sqrt 2 : ℂ) = 2 := by
    rw [← Complex.ofReal_mul, Real.mul_self_sqrt (by norm_num : (0:ℝ) ≤ 2)]; norm_num
  rw [Complex.star_def]
  simp only [xi, map_div₀, map_add, map_one, Complex.conj_I, Complex.conj_ofReal]
  rw [div_sub_div_same, div_eq_iff sqrt2_neC]
  linear_combination -Complex.I * h2

lemma mag_sq : (Real.sqrt 2 * Real.sqrt r2 * Real.sqrt s2) ^ 2 = 1 / 3 := by
  have e2 : Real.sqrt 2 * Real.sqrt 2 = 2 := Real.mul_self_sqrt (by norm_num)
  have er : Real.sqrt r2 * Real.sqrt r2 = r2 := Real.mul_self_sqrt r2_nonneg
  have es : Real.sqrt s2 * Real.sqrt s2 = s2 := Real.mul_self_sqrt s2_nonneg
  have hsq : (Real.sqrt 2 * Real.sqrt r2 * Real.sqrt s2) ^ 2
      = (Real.sqrt 2 * Real.sqrt 2) * (Real.sqrt r2 * Real.sqrt r2)
        * (Real.sqrt s2 * Real.sqrt s2) := by ring
  rw [hsq, e2, er, es, mul_assoc, r2s2]; norm_num

lemma psi0_val : psi2 0 = (Real.sqrt r2 : ℂ) := by simp [psi2]
lemma star_ofReal_c (r : ℝ) : star (r : ℂ) = (r : ℂ) := by
  rw [Complex.star_def, Complex.conj_ofReal]

lemma inner01 : wh_inner 2 psi2 (D_ah 2 0 1 0 psi2) = ((r2 - s2 : ℝ) : ℂ) := by
  rw [D01]; unfold wh_inner; rw [Fin.sum_univ_two]
  simp only [Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons,
    Complex.star_def, map_neg, mul_neg, Complex.mul_conj]
  rw [normSq_psi0, normSq_psi1]; push_cast; ring

lemma inner10 : wh_inner 2 psi2 (D_ah 2 1 0 0 psi2)
    = (Real.sqrt r2 : ℂ) * (Real.sqrt s2 : ℂ) * (Real.sqrt 2 : ℂ) := by
  rw [D10]; unfold wh_inner; rw [Fin.sum_univ_two]
  simp only [Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons,
    psi0_val, psi1_xi, star_mul', star_ofReal_c]
  have expand : (Real.sqrt r2 : ℂ) * ((Real.sqrt s2 : ℂ) * star xi)
      + (Real.sqrt s2 : ℂ) * xi * (Real.sqrt r2 : ℂ)
      = (Real.sqrt r2 : ℂ) * (Real.sqrt s2 : ℂ) * (xi + star xi) := by ring
  rw [expand, xi_add_conj]

lemma inner11 : wh_inner 2 psi2 (D_ah 2 1 1 0 psi2)
    = (Real.sqrt r2 : ℂ) * (Real.sqrt s2 : ℂ) * (Complex.I * (Real.sqrt 2 : ℂ)) := by
  rw [D11]; unfold wh_inner; rw [Fin.sum_univ_two]
  simp only [Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons,
    psi0_val, psi1_xi, star_neg, star_mul', star_ofReal_c, mul_neg]
  have expand : -((Real.sqrt r2 : ℂ) * ((Real.sqrt s2 : ℂ) * star xi))
      + (Real.sqrt s2 : ℂ) * xi * (Real.sqrt r2 : ℂ)
      = (Real.sqrt r2 : ℂ) * (Real.sqrt s2 : ℂ) * (xi - star xi) := by ring
  rw [expand, xi_sub_conj]

lemma eq01 : (2 + 1 : ℝ) * ‖wh_inner 2 psi2 (D_ah 2 0 1 0 psi2)‖ ^ 2 = 1 := by
  rw [inner01, Complex.norm_real, Real.norm_eq_abs, sq_abs, r2_sub_s2_sq]; norm_num

lemma eq10 : (2 + 1 : ℝ) * ‖wh_inner 2 psi2 (D_ah 2 1 0 0 psi2)‖ ^ 2 = 1 := by
  rw [inner10,
    show (Real.sqrt r2 : ℂ) * (Real.sqrt s2 : ℂ) * (Real.sqrt 2 : ℂ)
      = ((Real.sqrt 2 * Real.sqrt r2 * Real.sqrt s2 : ℝ) : ℂ) by push_cast; ring,
    Complex.norm_real, Real.norm_eq_abs, sq_abs, mag_sq]
  norm_num

lemma eq11 : (2 + 1 : ℝ) * ‖wh_inner 2 psi2 (D_ah 2 1 1 0 psi2)‖ ^ 2 = 1 := by
  rw [inner11,
    show (Real.sqrt r2 : ℂ) * (Real.sqrt s2 : ℂ) * (Complex.I * (Real.sqrt 2 : ℂ))
      = ((Real.sqrt 2 * Real.sqrt r2 * Real.sqrt s2 : ℝ) : ℂ) * Complex.I by push_cast; ring,
    norm_mul, Complex.norm_I, mul_one, Complex.norm_real, Real.norm_eq_abs, sq_abs, mag_sq]
  norm_num

/-- The qubit fiducial is a genuine `d = 2` SIC-POVM. -/
theorem d2_sic : IsSICPOVM 2 psi2 where
  norm_eq := norm_eq2
  equiangular := by
    intro a b hab
    fin_cases a <;> fin_cases b
    · exact absurd rfl hab
    · exact eq01
    · exact eq10
    · exact eq11

/-- A Weyl–Heisenberg covariant SIC-POVM exists in `d = 2`. -/
theorem sicpovm_exists_2 : SICPOVM_Exists 2 := ⟨psi2, d2_sic⟩

end
end D2

/-- **Base case, proved (not `trivial`).** The `n = 1` Zauner bridge holds:
    `ZaunerBridge 1 = SICPOVM_Exists 2` is discharged by the explicit qubit SIC
    fiducial of `D2.d2_sic`. -/
theorem zauner_bridge_d2 : ZaunerBridge 1 := D2.sicpovm_exists_2

-- ================================================================
-- §2. THE REDUCTION — the whole open content is one ℂ fiducial
-- ================================================================

/-- **The reduction (registers separated).** A genuine ℂ fiducial carrying the SIC
    property IS the bridge: exhibiting `ψ : Fin (2ⁿ) → ℂ` with `IsSICPOVM (2ⁿ) ψ`
    discharges `ZaunerBridge n`. Together with the unconditionally-proved structural
    skeleton, this isolates the ENTIRE open content of Zauner-for-2ⁿ into the single
    object `ψ`; the character obstruction (`Ring2048.wh_char_two_valued`) is why `ψ`
    cannot be read off the Belnap orbit for `n > 1`. -/
theorem bridge_of_fiducial (n : ℕ) (ψ : Fin (2 ^ n) → ℂ)
    (h : IsSICPOVM (2 ^ n) ψ) : ZaunerBridge n :=
  ⟨ψ, h⟩

/-- Contrapositive framing: no ℂ^(2ⁿ) fiducial ⇒ no bridge. Documents that the bridge
    is *equivalent* to producing the fiducial — nothing else is missing. -/
theorem bridge_iff_fiducial (n : ℕ) :
    ZaunerBridge n ↔ ∃ ψ : Fin (2 ^ n) → ℂ, IsSICPOVM (2 ^ n) ψ := Iff.rfl

-- ================================================================
-- §3. STRUCTURAL REGISTER — unconditional, re-exported
-- ================================================================

/-- The structural SIC skeleton at `d = 2048`, unconditional (rides
    `BelnapRing2048`): orbit `4¹¹ = d²`, constant join-equiangular overlap `22`, and
    the O_∞ tier. No ℂ, no Stark unit, no ray class field — the Grammar's own SIC. -/
theorem ring2048_structural :
    (mlOrbit dim2048_n).card = 4 ^ 11 ∧
    (∀ g : WHIdx dim2048_n,
      frobInner (mlFiducial dim2048_n) (whAct g (mlFiducial dim2048_n)) = 22) ∧
    imscriptionTier (Shor.Ring2048.multilatticeImscription dim2048_n) = .O_inf :=
  ⟨orbit_size_2048, join_equiangularity_2048, Shor.Ring2048.multilattice_is_O_inf dim2048_n⟩

-- ================================================================
-- §4. CAPSTONE — both registers, neither borrowing the other's authority
-- ================================================================

/-- **Capstone.** Conjoins the two registers without letting either stand in for the
    other:
    • structural (PROVED unconditionally): the `d = 2048` SIC skeleton;
    • shadow (REDUCED): the bridge `ZaunerBridge 11` holds as soon as one ℂ fiducial
      with `IsSICPOVM 2048` is produced — and that existence, `SICPOVM_Exists 2048`,
      is the open Zauner conjecture for `d = 2048`, which this theorem does NOT
      assert. The honest state: everything but the single ℂ fiducial is discharged.
    ZERO axioms, ZERO sorries. -/
theorem ring2048_capstone :
    ((mlOrbit dim2048_n).card = 4 ^ 11 ∧
      (∀ g : WHIdx dim2048_n,
        frobInner (mlFiducial dim2048_n) (whAct g (mlFiducial dim2048_n)) = 22) ∧
      imscriptionTier (Shor.Ring2048.multilatticeImscription dim2048_n) = .O_inf) ∧
    (∀ ψ : Fin (2 ^ 11) → ℂ, IsSICPOVM (2 ^ 11) ψ → ZaunerBridge 11) :=
  ⟨ring2048_structural, fun ψ h => bridge_of_fiducial 11 ψ h⟩

end Imscribing.Millennium.Ring2048Bridge
