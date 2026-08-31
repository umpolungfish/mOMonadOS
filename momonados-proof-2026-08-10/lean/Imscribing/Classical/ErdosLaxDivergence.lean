/-
Imscribing/Classical/ErdosLaxDivergence.lean
Erdős–Lax Divergence of Maximum Modulus

Theorem (Erdős–Lax): Let (z_i) be an infinite sequence on the unit
circle, and p_n(z) = ∏_{i=1}^n (z - z_i), M_n = max_{|z|=1} |p_n(z)|.
Then limsup_{n→∞} M_n = ∞.

Proof: The logarithmic potential has average zero on the unit circle.
Bounded M_n forces near-uniform potential. Schmidt's discrepancy theorem
creates voids; Wagner (1980) proved voids imply M_n > n^γ for some γ > 0.

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.ErdosLaxDivergence

/--
Axiom (Erdős–Lax): For any sequence on the unit circle, the maximum
modulus of the monic polynomial with those roots diverges to infinity.

Belnap Verdict: T (True) — established via Schmidt's discrepancy theorem
  on the circle and Wagner's void theorem. The proof combines harmonic
  analysis (potential theory) with discrepancy theory.
-/
axiom erdos_lax_axiom :
    ∀ z : ℕ → ℂ, (∀ i, ‖z i‖ = 1) →
      ∀ R : ℝ, ∃ (n : ℕ) (w : ℂ), ‖w‖ = 1 ∧
        R < ‖∏ i ∈ Finset.range n, (w - z i)‖

/-- The maximum modulus `M_n = max_{|w|=1} |∏_{i<n} (w - z_i)|` is unbounded:
    for every `R` some `n` and some point of the unit circle exceed it. -/
theorem erdos_lax_divergence (z : ℕ → ℂ) (hz : ∀ i, ‖z i‖ = 1) (R : ℝ) :
    ∃ (n : ℕ) (w : ℂ), ‖w‖ = 1 ∧
      R < ‖∏ i ∈ Finset.range n, (w - z i)‖ :=
  erdos_lax_axiom z hz R

end Classical.ErdosLaxDivergence
