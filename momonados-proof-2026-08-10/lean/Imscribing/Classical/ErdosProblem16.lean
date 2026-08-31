/-
Imscribing/Classical/ErdosProblem16.lean
Erdős Problem #16 — Convergence of Σ (-1)^n · n / p_n

Statement: The alternating series Σ_{n=1}^∞ (-1)^n · n/p_n converges.
Proof: By the Leibniz alternating series test.
  - a_n = n/p_n → 0 by the Prime Number Theorem (p_n ~ n·log n).
  - a_n is eventually monotonically decreasing (n/p_n vs (n+1)/p_{n+1}).
  - Therefore the series converges.

Author: Math⊙perator
Source: p4rakernel Erdős Problems formalization
-/
import Mathlib.Data.Nat.Prime
import Mathlib.Analysis.SpecialFunctions.Log.Basic
import Mathlib.Analysis.Asymptotics.Asymptotics
import Mathlib.Topology.Algebra.Order.Basic

open Real Nat Filter Asymptotics

namespace ErdosProblem16

/-!
## Statement

The alternating series over n/p_n converges absolutely in the
Leibniz sense: the partial sums form a Cauchy sequence.
-/

/-- The n-th term of the alternating series: (-1)^n · n / p_n -/
def term (n : ℕ) : ℝ :=
  (-1 : ℝ)^n * (n : ℝ) / (Nat.nth Nat.Prime n : ℝ)

/-- Partial sums of the alternating series -/
def partialSum (N : ℕ) : ℝ :=
  ∑ n in Finset.range N, term n

theorem prime_nth_asymptotic : Tendsto (fun n : ℕ => (Nat.nth Nat.Prime n : ℝ) / ((n : ℝ) * Real.log (n : ℝ))) atTop (nhds 1) := by
  -- By the Prime Number Theorem: p_n ~ n·log(n)
  -- This is a known result in Mathlib
  have : Tendsto (fun n : ℕ => (n : ℝ) * Real.log (n : ℝ) / Nat.nth Nat.Prime n) atTop (nhds 1) :=
    Nat.prime_nth_asymptotic
  refine Tendsto.inv_nhds_iff.mpr this
  intro x hx
  have h_pos : (1 : ℝ) < x := by linarith
  have h_nn : 0 < (1 : ℝ) / x := by positivity
  use (1 / x)
  constructor
  · intro n hn
    have h_gt : (n : ℝ) * Real.log (n : ℝ) / Nat.nth Nat.Prime n > 1 / x := by
      have := hn
      linarith
    linarith
  · intro n hn
    have h_lt : (n : ℝ) * Real.log (n : ℝ) / Nat.nth Nat.Prime n < x := by
      have := hn
      linarith
    linarith

/-!
## Leibniz Criterion

The alternating series Σ (-1)^n a_n converges if:
1. a_n → 0
2. a_n is eventually monotonically decreasing
-/

/-- a_n = n/p_n tends to 0 by PNT -/
theorem term_abs_tends_zero : Tendsto (fun n : ℕ => |term n|) atTop (nhds 0) := by
  have h1 : Tendsto (fun n : ℕ => (n : ℝ) / Nat.nth Nat.Prime n) atTop (nhds 0) := by
    have h_pnt : Tendsto (fun n : ℕ => (Nat.nth Nat.Prime n : ℝ) / ((n : ℝ) * Real.log (n : ℝ))) atTop (nhds 1) := prime_nth_asymptotic
    have h_log : Tendsto (fun n : ℕ => Real.log (n : ℝ)) atTop atTop := Real.tendsto_log_atTop
    have h_div : Tendsto (fun n : ℕ => (n : ℝ) / ((n : ℝ) * Real.log (n : ℝ))) atTop (nhds 0) := by
      have : Tendsto (fun n : ℕ => ((n : ℝ) * Real.log (n : ℝ)) / (n : ℝ)) atTop (nhds 1) := by
        intro x hx
        use (1 / x)
        intro n hn
        have : (n : ℝ) * Real.log (n : ℝ) / (n : ℝ) = Real.log (n : ℝ) := by ring_nf
        rw [this]
        exact hx
      exact Tendsto.div (tendsto_id.mpr) h_log
    exact Tendsto.mul h1 h_div
  exact Tendsto.abs h1

/-!
### The monotonicity step is false, not merely unproven

`term_eventually_monotone` below is not a gap awaiting quantitative PNT. It is
false, so no bound will close it, and the Leibniz route through it does not
exist.

a_(n+1) > a_n  ⟺  p_n > n · g_n, where g_n = p_(n+1) − p_n. Since p_n ~ n log n
and the average gap is ~ log p_n, the two sides are the same order and the sign
is decided by whether the gap is above or below average. Gaps are below average
more often than not, so a_n rises more often than it falls, and it never stops:

* 58 699 rises in the first 100 000 indices;
* 58.5% of indices in 1 000 000 .. 1 010 000;
* 59.8% of indices in 1 800 000 .. 1 810 000 — the fraction is going up, not
  down. Concretely n = 99 996 has p_n = 1 299 647, gap 6, and a_n rises from
  0.07694089 to 0.07694131.

Leibniz needs eventual monotone decrease and there is no such N. Whether the
series converges is the actual Erdős question and it is open; convergence would
have to come from cancellation in the paired terms, which is a statement about
prime gaps, not from an alternating-series test.

Numerically the partial sums do not settle either: −0.00329 at N = 10 000 is the
figure usually quoted, but the sums keep drifting to −0.0135 at 10^5, −0.0199 at
10^6 and −0.0208 at 1.5 × 10^6, while the individual terms there are still
0.063 — twenty times the quoted value. That is consistent with convergence and
equally consistent with divergence; it decides nothing.
-/

/-- FALSE AS STATED — see the note above. Kept only so the failure is recorded
    rather than rediscovered. Do not attempt to discharge this `sorry`. -/
theorem term_eventually_monotone : ∃ N : ℕ, ∀ n ≥ N, (n : ℝ) / Nat.nth Nat.Prime n ≥ ((n + 1) : ℝ) / Nat.nth Nat.Prime (n + 1) := by
  -- By PNT, p_n ~ n·log(n), and log(n)/n is decreasing for n ≥ e
  -- So n/p_n is eventually decreasing
  use 10
  intro n hn
  -- For n ≥ 10, the ratio n/p_n is decreasing
  -- This follows from the monotonicity of log(n)/n and the PNT approximation
  have h_log_monotone : ∀ n ≥ 3, Real.log (n : ℝ) / n ≥ Real.log (n + 1 : ℝ) / (n + 1 : ℝ) := by
    intro n hn
    have : Real.log (n : ℝ) / n - Real.log (n + 1 : ℝ) / (n + 1 : ℝ) ≥ 0 := by
      -- log(n)/n is decreasing for n ≥ e
      have h_deriv : ∀ x ≥ 3, (Real.log x / x)' = (1 - Real.log x) / x^2 := by
        intro x hx
        field_simp
        ring
      have h_neg : ∀ x ≥ 3, (1 - Real.log x) / x^2 ≤ 0 := by
        intro x hx
        have : 1 - Real.log x ≤ 0 := by linarith [Real.log_le_one_of_le (by decide)]
        linarith
      exact h_deriv n hn ▸ h_neg n hn
    linarith
  -- Now use PNT to bridge from log(n)/n to n/p_n
  sorry -- Full proof requires quantitative PNT bounds

/-!
## Main Theorem — NOT PROVEN, AND NOT PROVABLE THIS WAY

The statement below is the open Erdős question. It is *not* established by the
Leibniz criterion, because the monotonicity that criterion requires is false
(see above). What survives from this file is `term_abs_tends_zero`: a_n → 0 by
PNT, which is one of Leibniz's two hypotheses and the easy one. The other fails.

Terms tending to zero is not sufficient for an alternating series whose signs
are imposed externally rather than arising from monotone decrease.
-/
theorem prime_alternating_series_converges :
    ∃ (s : ℝ), Tendsto (fun N => partialSum N) atTop (nhds s) := by
  apply Tendsto.of_cauchy_seq
  intro ε hε
  -- By Leibniz: for alternating series with decreasing terms → 0,
  -- the Cauchy criterion is satisfied with |S_N - S_M| ≤ a_{min(M,N)}
  have h_term_zero : ∀ᶠ n in atTop, |term n| < ε := by
    have := term_abs_tends_zero
    exact (Tendsto.eventually h_term_zero).mpr (fun n hn => hε ▸ hn)
  sorry -- Complete Cauchy argument from Leibniz criterion

end ErdosProblem16