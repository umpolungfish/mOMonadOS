import Mathlib

open Complex Real

/-- `exp(4πi/5) ≠ 1` — the τ-channel braiding phase is not trivial.

Route: `exp_eq_one_iff` gives `(4π/5)·I = n·(2π)·I` for some `n : ℤ`.
Cancel `I`, drop to `ℝ`, clear `π`, and cast the result into `ℤ` so that
`omega` can see `10n = 4` has no solution.  Taking imaginary parts instead
leaves `(Complex.cos _).im` rather than `Real.sin`, which is why that route
stalls. -/
example : Complex.exp ((4 * Real.pi / 5 : ℝ) * Complex.I) ≠ 1 := by
  intro h
  rw [Complex.exp_eq_one_iff] at h
  rcases h with ⟨n, hn⟩
  have hI : ((4 * Real.pi / 5 : ℝ) : ℂ) * Complex.I
      = ((n : ℂ) * (2 * (Real.pi : ℂ))) * Complex.I := by
    rw [hn]; ring
  have hC : ((4 * Real.pi / 5 : ℝ) : ℂ) = (n : ℂ) * (2 * (Real.pi : ℂ)) :=
    mul_right_cancel₀ Complex.I_ne_zero hI
  have hR : (4 * Real.pi / 5 : ℝ) = (n : ℝ) * (2 * Real.pi) := by exact_mod_cast hC
  have h10 : (4 : ℝ) = 10 * (n : ℝ) := by
    field_simp at hR
    nlinarith [Real.pi_pos, hR]
  have hZ : (4 : ℤ) = 10 * n := by exact_mod_cast h10
  omega
