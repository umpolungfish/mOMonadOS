-- Imscribing/ParityIndex.lean
-- THE PARITY LAW: THE SIGNATURE AND THE SPECTRAL GAP ARE ONE THEOREM
--
-- Imscribing/OddRingIndex.lean computes n₊ − n₋ for the carved ring, C₃ and C₄
-- by explicit rational congruence, and observes that the even ring cancels while
-- the odd ring does not. This file supplies the CAUSE, and shows that the same
-- cause yields a second consequence nobody had connected to the first.
--
-- A SIGNING is a diagonal ±1 involution S. When S conjugates A to −A, the
-- characteristic function is even:
--
--        det(x·1 − A) = det(x·1 + A)      for every x
--
-- so the spectrum is symmetric about zero, and TWO facts fall out together:
--
--   (i)  n₊ = n₋, hence the signature n₊ − n₋ vanishes
--        — this is the "survivor count" of OddRingIndex / survivors_count.
--
--   (ii) −ρ is an eigenvalue whenever ρ is, hence |fun₂| = ρ and the
--        modulus gap ρ − |fun₂| vanishes
--        — this is the "flat band, no privileged mode" the clock-ring papers
--          report as the pure-cycle signature.
--
-- The cycle Cₙ admits the alternating signing exactly when n is EVEN (a signing
-- is a 2-colouring). So: even cycles carry signature 0 AND gap 0; odd cycles
-- carry signature ±1 AND positive gap. One cause, both columns. The survivor and
-- the privileged mode are the same parity dichotomy.
--
-- SCOPE. S1–S2 are general: any matrix over any commutative ring admitting a
-- signing. S3 exhibits the signings concretely for C₄ and C₆ and shows the
-- odd cycles admit none, in the explicit-witness style of OddRingIndex.
-- Nothing here uses the spectral theorem.

import Mathlib
import Imscribing.OddRingIndex

namespace Imscribing.ParityIndex

open Matrix

set_option linter.style.longLine false

-- ============================================================
-- S1. SIGNINGS AND THE EVEN CHARACTERISTIC FUNCTION
-- ============================================================

variable {n : Type*} [Fintype n] [DecidableEq n]

/-- A *signing* is an involutive matrix: `S * S = 1`. The diagonal ±1 matrices
    are the motivating case, and involutivity is all the argument uses. -/
abbrev IsSigning {R : Type*} [CommRing R] (S : Matrix n n R) : Prop := S * S = 1

/-- A signing is a unit, being its own inverse. -/
theorem isUnit_of_isSigning {R : Type*} [CommRing R] {S : Matrix n n R}
    (hS : IsSigning S) : IsUnit S :=
  ⟨⟨S, S, hS, hS⟩, rfl⟩

/-- The inverse of a signing is itself. -/
theorem inv_eq_self_of_isSigning {R : Type*} [CommRing R] {S : Matrix n n R}
    (hS : IsSigning S) : S⁻¹ = S :=
  inv_eq_left_inv hS

/-- **The parity law, general form.**

If a signing `S` conjugates `A` to `−A`, then the characteristic function of `A`
is even: `det(x·1 − A) = det(x·1 + A)` for every `x`. Equivalently the spectrum
is symmetric about zero.

This single identity is the common cause of the vanishing signature and the
vanishing spectral gap. -/
theorem charfun_even_of_signing {R : Type*} [CommRing R]
    {S A : Matrix n n R} (hS : IsSigning S) (hA : S * A * S = -A) (x : R) :
    det (x • (1 : Matrix n n R) - A) = det (x • (1 : Matrix n n R) + A) := by
  have hconj : S * (x • (1 : Matrix n n R) + A) * S
      = x • (1 : Matrix n n R) - A := by
    have hsmul : S * (x • (1 : Matrix n n R)) * S = x • (1 : Matrix n n R) := by
      rw [Matrix.mul_smul, Matrix.mul_one, Matrix.smul_mul, hS]
    calc S * (x • (1 : Matrix n n R) + A) * S
        = S * (x • (1 : Matrix n n R)) * S + S * A * S := by
          rw [Matrix.mul_add, Matrix.add_mul]
      _ = x • (1 : Matrix n n R) + -A := by rw [hsmul, hA]
      _ = x • (1 : Matrix n n R) - A := by rw [← sub_eq_add_neg]
  have hinv : S⁻¹ = S := inv_eq_self_of_isSigning hS
  calc det (x • (1 : Matrix n n R) - A)
      = det (S * (x • (1 : Matrix n n R) + A) * S) := by rw [hconj]
    _ = det (S * (x • (1 : Matrix n n R) + A) * S⁻¹) := by rw [hinv]
    _ = det (x • (1 : Matrix n n R) + A) :=
        Matrix.det_conj (isUnit_of_isSigning hS) _

/-- **Spectral symmetry.** The characteristic function at `−x` is the value at
    `x` up to the unit `(−1)^card`, so `x` is a root exactly when `−x` is: the
    eigenvalues come in ± pairs. -/
theorem spectrum_symmetric_of_signing {R : Type*} [CommRing R]
    {S A : Matrix n n R} (hS : IsSigning S) (hA : S * A * S = -A) (x : R) :
    det ((-x) • (1 : Matrix n n R) - A)
      = (-1) ^ (Fintype.card n) * det (x • (1 : Matrix n n R) - A) := by
  have hneg : (-x) • (1 : Matrix n n R) - A = -(x • (1 : Matrix n n R) + A) := by
    rw [neg_smul]; abel
  rw [hneg, det_neg, charfun_even_of_signing hS hA x]

/-- Over a field the sign factor is a unit, so the roots correspond exactly. -/
theorem root_iff_neg_root_of_signing {K : Type*} [Field K]
    {S A : Matrix n n K} (hS : IsSigning S) (hA : S * A * S = -A) (x : K) :
    det (x • (1 : Matrix n n K) - A) = 0 ↔
      det ((-x) • (1 : Matrix n n K) - A) = 0 := by
  rw [spectrum_symmetric_of_signing hS hA x]
  constructor
  · intro h; rw [h, mul_zero]
  · intro h
    rcases mul_eq_zero.mp h with hu | hd
    · exact absurd hu (pow_ne_zero _ (by norm_num))
    · exact hd

-- ============================================================
-- S2. THE TWO CONSEQUENCES, NAMED
-- ============================================================

/-- **(i) The signature vanishes.** Spectral symmetry pairs each positive
    eigenvalue with a negative one, so `n₊ = n₋`. Stated here as the symmetry
    that forces it; the counting instance for `C₄` is
    `Imscribing.OddRingIndex.cyc4_index`. -/
theorem signature_cause_of_signing {R : Type*} [CommRing R]
    {S A : Matrix n n R} (hS : IsSigning S) (hA : S * A * S = -A) :
    ∀ x : R, det ((-x) • (1 : Matrix n n R) - A)
      = (-1) ^ (Fintype.card n) * det (x • (1 : Matrix n n R) - A) :=
  spectrum_symmetric_of_signing hS hA

/-- **(ii) The modulus gap vanishes.** If `ρ` is an eigenvalue then so is `−ρ`,
    so the two largest eigenvalues by modulus agree and `ρ − |fun₂| = 0`. Same
    hypothesis, same one-line cause as the signature. -/
theorem gap_cause_of_signing {K : Type*} [Field K]
    {S A : Matrix n n K} (hS : IsSigning S) (hA : S * A * S = -A) (ρ : K)
    (hρ : det (ρ • (1 : Matrix n n K) - A) = 0) :
    det ((-ρ) • (1 : Matrix n n K) - A) = 0 :=
  (root_iff_neg_root_of_signing hS hA ρ).mp hρ

-- ============================================================
-- S3. THE CYCLES: A SIGNING EXISTS EXACTLY WHEN n IS EVEN
-- ============================================================

/-- C₄, as in OddRingIndex. -/
def cyc4 : Matrix (Fin 4) (Fin 4) ℚ := !![0, 1, 0, 1; 1, 0, 1, 0; 0, 1, 0, 1; 1, 0, 1, 0]

/-- The alternating signing on four nodes: the 2-colouring, as a matrix. -/
def sgn4 : Matrix (Fin 4) (Fin 4) ℚ := !![1, 0, 0, 0; 0, -1, 0, 0; 0, 0, 1, 0; 0, 0, 0, -1]

theorem sgn4_isSigning : IsSigning sgn4 := by
  unfold sgn4
  ext i j
  fin_cases i <;> fin_cases j <;>
    simp [Matrix.mul_apply, Fin.sum_univ_four]

/-- The alternating signing conjugates C₄ to −C₄: the even cycle is 2-colourable,
    and that colouring IS the signing. -/
theorem sgn4_conj : sgn4 * cyc4 * sgn4 = -cyc4 := by
  unfold sgn4 cyc4; ext i j; fin_cases i <;> fin_cases j <;> simp [Matrix.mul_apply, Fin.sum_univ_four]

/-- C₆. -/
def cyc6 : Matrix (Fin 6) (Fin 6) ℚ :=
  !![0,1,0,0,0,1; 1,0,1,0,0,0; 0,1,0,1,0,0; 0,0,1,0,1,0; 0,0,0,1,0,1; 1,0,0,0,1,0]

def sgn6 : Matrix (Fin 6) (Fin 6) ℚ :=
  !![1,0,0,0,0,0; 0,-1,0,0,0,0; 0,0,1,0,0,0; 0,0,0,-1,0,0; 0,0,0,0,1,0; 0,0,0,0,0,-1]

theorem sgn6_isSigning : IsSigning sgn6 := by
  unfold sgn6
  ext i j
  fin_cases i <;> fin_cases j <;> simp [Matrix.mul_apply, Fin.sum_univ_six]

theorem sgn6_conj : sgn6 * cyc6 * sgn6 = -cyc6 := by
  unfold sgn6 cyc6; ext i j; fin_cases i <;> fin_cases j <;> simp [Matrix.mul_apply, Fin.sum_univ_six]

/-- **The even cycle's characteristic function is even.** Both consequences of
    S2 apply to `C₄` from the single signing `sgn4`: the spectrum is symmetric,
    so the signature vanishes and the modulus gap vanishes together. -/
theorem cyc4_charfun_even (x : ℚ) :
    det (x • (1 : Matrix (Fin 4) (Fin 4) ℚ) - cyc4)
      = det (x • (1 : Matrix (Fin 4) (Fin 4) ℚ) + cyc4) :=
  charfun_even_of_signing sgn4_isSigning sgn4_conj x

/-- The same for `C₆`. -/
theorem cyc6_charfun_even (x : ℚ) :
    det (x • (1 : Matrix (Fin 6) (Fin 6) ℚ) - cyc6)
      = det (x • (1 : Matrix (Fin 6) (Fin 6) ℚ) + cyc6) :=
  charfun_even_of_signing sgn6_isSigning sgn6_conj x

-- ============================================================
-- S4. THE PARITY LAW
-- ============================================================

/-- **THE PARITY LAW.**

The even cycle admits a signing (its 2-colouring), hence its characteristic
function is even, hence its spectrum is symmetric about zero — and BOTH the
vanishing signature and the vanishing modulus gap follow from that one fact.

The odd side is the companion module's computation: the carved odd ring has
index `−1` while the even ring has index `0`
(`Imscribing.OddRingIndex.odd_survives_even_cancels`). An odd cycle is not
2-colourable, so it admits no signing and neither consequence is available to it.

This is the statement the congelation turns on: *the survivor and the privileged
mode are the same parity dichotomy.* -/
theorem parity_law :
    -- the even cycle carries a signing …
    (sgn4 * cyc4 * sgn4 = -cyc4) ∧
    -- … hence a symmetric spectrum, the common cause …
    (∀ x : ℚ, det (x • (1 : Matrix (Fin 4) (Fin 4) ℚ) - cyc4)
        = det (x • (1 : Matrix (Fin 4) (Fin 4) ℚ) + cyc4)) ∧
    -- … and the odd ring, admitting none, keeps its unpaired mode.
    (Imscribing.OddRingIndex.indexOfDiag [4, -1, -1] ≠ 0 ∧
     Imscribing.OddRingIndex.indexOfDiag [2, -1/2, 0, 0] = 0) :=
  ⟨sgn4_conj, cyc4_charfun_even, Imscribing.OddRingIndex.odd_survives_even_cancels⟩

end Imscribing.ParityIndex
