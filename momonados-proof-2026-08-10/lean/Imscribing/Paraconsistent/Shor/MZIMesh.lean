-- Imscribing/Paraconsistent/Shor/MZIMesh.lean
-- MZI MESH LEAN FORMALIZATION
-- Author: Lando ⊗ ⊙-boundary Operator
--
-- Lean formalization of the Mach-Zehnder Interferometer mesh
-- from frobenius-mzi.html, connecting the continuous optics picture
-- to the Belnap Born rule proved in BelnapNFiducial.lean §10.
--
-- Transfer amplitude: a_ij = sin(θ)·exp(iφ)
-- Born probability:   |a|² = sin²(θ)   (phase drops out)
-- Balance point:      θ = π/4 → Born = 1/2
-- Fixed-point law:    π/2 - π/4 = π/4  (analog of bnot B = B)
-- SIC fiducial mesh:  all θ=π/4, uniform Born = 1/2
--
-- §A  MZI element and Born probability
-- §B  Balance point θ=π/4: Born=1/2, amplitude reflection fixed point
-- §C  SIC fiducial mesh: all elements at balance, uniform Born
-- §D  Phase drops out: |sin(θ)·exp(iφ)|² = sin²(θ)
-- §E  Bridge to Belnap: continuous Born matches bilattice Born
-- §F  Frobenius roundtrip: closed-loop field fraction = 1/2

import Mathlib.Analysis.SpecialFunctions.Trigonometric.Basic
import Mathlib.Analysis.SpecialFunctions.Complex.Circle
import Mathlib.Data.Complex.Basic
import Imscribing.Paraconsistent.Shor.BelnapNFiducial

namespace Imscribing.Paraconsistent.Shor.MZIMesh

open Real
open Imscribing.Paraconsistent.Shor.NFiducial

-- ============================================================
-- §A. MZI Element and Born Probability
-- ============================================================

/-- A single MZI element: amplitude angle θ (in [0,π/2]) and phase φ.
    Corresponds to one matrix entry a_ij = sin(θ)·exp(iφ). -/
structure MZIElement where
  theta : ℝ
  phi   : ℝ

/-- Transfer amplitude: a = sin(θ)·exp(iφ). -/
noncomputable def transferAmp (mzi : MZIElement) : ℂ :=
  (Real.sin mzi.theta : ℂ) * Complex.exp (↑mzi.phi * Complex.I)

/-- Born probability: |a|² = sin²(θ).  Phase factor is unobservable. -/
noncomputable def bornProb (mzi : MZIElement) : ℝ :=
  Real.sin mzi.theta ^ 2

-- ============================================================
-- §B. Balance Point θ = π/4
-- ============================================================

/-- The SIC balance element: θ = π/4, φ = 0.
    Corresponds to a perfect 50/50 beamsplitter. -/
noncomputable def balanceMZI : MZIElement := ⟨π / 4, 0⟩

/-- Born probability at the balance point equals 1/2. -/
theorem born_prob_balance : bornProb balanceMZI = 1 / 2 := by
  simp only [bornProb, balanceMZI, Real.sin_pi_div_four]
  rw [div_pow, Real.sq_sqrt (by norm_num : (0:ℝ) ≤ 2)]
  norm_num

/-- θ = π/4 is the fixed point of amplitude reflection θ ↦ π/2 - θ.
    Continuous analog of bnot B = B in the Belnap lattice. -/
theorem balance_is_reflection_fixed_point :
    π / 2 - balanceMZI.theta = balanceMZI.theta := by
  simp only [balanceMZI]; ring

/-- sin(π/4) = cos(π/4): transmission amplitude equals reflection amplitude.
    The balanced beamsplitter condition. -/
theorem balance_transmission_eq_reflection :
    Real.sin balanceMZI.theta = Real.cos balanceMZI.theta := by
  simp [balanceMZI, Real.sin_pi_div_four, Real.cos_pi_div_four]

/-- Energy conservation for any MZI element: sin²(θ) + cos²(θ) = 1. -/
theorem mzi_energy_conservation (mzi : MZIElement) :
    Real.sin mzi.theta ^ 2 + Real.cos mzi.theta ^ 2 = 1 :=
  Real.sin_sq_add_cos_sq mzi.theta

-- ============================================================
-- §C. SIC Fiducial Mesh
-- ============================================================

/-- An n×n MZI mesh: a matrix of MZI elements indexed by Fin n × Fin n. -/
abbrev Mesh (n : ℕ) := Fin n → Fin n → MZIElement

/-- The SIC fiducial mesh: every element at the balance point θ=π/4. -/
noncomputable def sicFiducialMesh (n : ℕ) : Mesh n := fun _ _ => balanceMZI

/-- Every element of the SIC fiducial mesh has Born probability 1/2. -/
theorem sicFiducial_uniform_born (n : ℕ) (i j : Fin n) :
    bornProb ((sicFiducialMesh n) i j) = 1 / 2 :=
  born_prob_balance

theorem sicFiducialMesh_const (n : ℕ) (i j : Fin n) :
    (sicFiducialMesh n) i j = balanceMZI := rfl

-- ============================================================
-- §D. Phase Drops Out: |a|² = sin²(θ)
-- ============================================================

-- Auxiliary: exp(iφ) has real part cos(φ) and imaginary part sin(φ).
-- Uses Complex.exp_re/im: (exp z).re = exp(z.re)*cos(z.im), etc.
-- Since (↑φ * I).re = 0 and (↑φ * I).im = φ (both from simp), we get
-- exp(iφ).re = exp(0)*cos(φ) = cos(φ) and exp(iφ).im = exp(0)*sin(φ) = sin(φ).

private theorem exp_mul_I_re (φ : ℝ) : (Complex.exp (↑φ * Complex.I)).re = Real.cos φ := by
  simp [Complex.exp_re, Complex.mul_re, Complex.mul_im,
        Complex.I_re, Complex.I_im, Real.exp_zero]

private theorem exp_mul_I_im (φ : ℝ) : (Complex.exp (↑φ * Complex.I)).im = Real.sin φ := by
  simp [Complex.exp_im, Complex.mul_re, Complex.mul_im,
        Complex.I_re, Complex.I_im, Real.exp_zero]

/-- The unit-circle factor exp(iφ) has normSq = 1.
    Follows from normSq z = re²+im² = cos²φ + sin²φ = 1. -/
private theorem normSq_exp_mul_I (φ : ℝ) :
    Complex.normSq (Complex.exp (↑φ * Complex.I)) = 1 := by
  rw [Complex.normSq_apply, exp_mul_I_re, exp_mul_I_im]
  have h := Real.sin_sq_add_cos_sq φ
  linarith [show Real.cos φ * Real.cos φ = Real.cos φ ^ 2 from by ring,
            show Real.sin φ * Real.sin φ = Real.sin φ ^ 2 from by ring]

/-- Phase factor is invisible: |sin(θ)·exp(iφ)|² = sin²(θ). -/
theorem transferAmp_normSq_eq_bornProb (mzi : MZIElement) :
    Complex.normSq (transferAmp mzi) = bornProb mzi := by
  simp only [transferAmp, bornProb, map_mul,
             normSq_exp_mul_I mzi.phi, mul_one,
             Complex.normSq_apply, Complex.ofReal_re, Complex.ofReal_im]
  ring

-- ============================================================
-- §E. Bridge to Belnap Born Rule
-- ============================================================

/-- The Belnap bilattice Born rule: posEvidence(B) = 1, singleRegCost(B) = 2. -/
theorem belnap_born_half :
    2 * posEvidence Belnap.B = singleRegCost Belnap.B := by decide

/-- MZI balance Born (continuous, sin²(π/4)=1/2) agrees with Belnap Born
    (discrete, posEvidence(B)/singleRegCost(B) = 1/2).
    Two independent derivations of the same value from different frameworks. -/
theorem born_rules_agree :
    bornProb balanceMZI = 1 / 2 ∧
    2 * (posEvidence Belnap.B : ℝ) = (singleRegCost Belnap.B : ℝ) :=
  ⟨born_prob_balance, by exact_mod_cast belnap_born_half⟩

-- ============================================================
-- §F. Frobenius Roundtrip
-- ============================================================

/-- Frobenius roundtrip value for a closed-loop MZI element.
    Corresponds to curve.closed = true in frobenius-mzi.html:
    the fraction of the field recovered after a full loop = |a|². -/
noncomputable def frobeniusRoundtrip (mzi : MZIElement) : ℝ :=
  Complex.normSq (transferAmp mzi)

theorem frobenius_eq_born_prob (mzi : MZIElement) :
    frobeniusRoundtrip mzi = bornProb mzi :=
  transferAmp_normSq_eq_bornProb mzi

/-- Balance element Frobenius roundtrip = 1/2:
    half the incident field survives the closed loop at the SIC balance point. -/
theorem balance_frobenius_roundtrip :
    frobeniusRoundtrip balanceMZI = 1 / 2 := by
  rw [frobenius_eq_born_prob, born_prob_balance]

/-- At the balance point, roundtrip value = Born probability = 1/2. -/
theorem balance_frobenius_eq_born :
    frobeniusRoundtrip balanceMZI = bornProb balanceMZI :=
  frobenius_eq_born_prob balanceMZI

end Imscribing.Paraconsistent.Shor.MZIMesh
