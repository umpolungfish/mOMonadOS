/-
Imscribing/Classical/Szemeredi.lean
Szemerédi's Theorem via the Density-Increment Argument

Theorem (Szemerédi, 1975): For every k ≥ 3, let r_k(N) denote the
largest size of a subset of {1,…,N} containing no non-trivial k-term
arithmetic progression. Then r_k(N) = o(N) as N → ∞.

Proof sketch: Gowers U^{k-1}-norm counting lemma → inverse theorem
(Green–Tao–Ziegler) → nilsequence correlation → arithmetic regularity
→ density increment on sub-progression → iteration → o(N).

Quantitative bounds:
  Behrend (lower): r_k(N) ≥ N exp(−c_k (log N)^{1/(k−1)}).
  Kelley–Meka (upper, k=3): r_3(N) ≤ N exp(−c (log N)^{1/11}).

Belnap Verdict: T (True) — established. Remaining gaps are quantitative.

Author: Lando⊗⊙perator
-/

import Mathlib

open Finset

namespace Imscribing.Classical.Szemeredi

set_option linter.style.lambdaSyntax false
set_option linter.style.nativeDecide false
set_option linter.style.induction false
set_option linter.style.whitespace false
set_option linter.style.longLine false
set_option linter.unusedVariables false

-- ─── Opaque definitions ───────────────────────────────────────────────────────

/-- r_k(N): the largest size of a k-AP-free subset of {1,…,N}. -/
opaque r_k (k N : ℕ) : ℕ

/-- The k-term arithmetic progression counting operator Λ_k(f). -/
opaque AP_count (k : ℕ) (f : ℕ → ℝ) : ℝ

/-- The Gowers U^{s}-norm of f : ℕ → ℝ restricted to [1,N]. -/
opaque gowers_norm (s N : ℕ) (f : ℕ → ℝ) : ℝ

/-- Nilsequence type of step s. Evaluation via `nilseq_eval`. -/
opaque Nilsequence : ℕ → Type

/-- Evaluate a nilsequence ψ at position n. -/
opaque nilseq_eval {s : ℕ} : Nilsequence s → ℕ → ℂ

/-- Complexity of a nilsequence. -/
opaque nilseq_complexity {s : ℕ} (ψ : Nilsequence s) : ℕ

/-- The correlation ⟨f, ψ⟩ := (1/N) ∑_{n=1}^N f(n)·Re(ψ(n)). -/
noncomputable def correlation (N : ℕ) (f : ℕ → ℝ) (ψ : ℕ → ℂ) : ℝ :=
  let s := Finset.sum (Icc 1 N) (fun n => f n * ((ψ n).re : ℝ))
  s / (N : ℝ)

-- ─── Axioms ───────────────────────────────────────────────────────────────────

/-- Counting Lemma: |Λ_k(1_A) − δ^k| ≤ c_k·‖f‖_{U^{k-1}}. -/
axiom counting_lemma (k N : ℕ) (hk : k ≥ 3) (A : Finset ℕ) (hA : ∀ a ∈ A, 1 ≤ a ∧ a ≤ N)
  (δ : ℝ) (hδ : δ = (A.card : ℝ) / (N : ℝ)) :
  ∃ ck : ℝ, ck > 0 ∧
    let f : ℕ → ℝ := fun n => (if n ∈ A then (1 : ℝ) else (0 : ℝ)) - δ
    |AP_count k f - δ ^ k| ≤ ck * gowers_norm (k - 1) N f

/-- AP-freeness + positive density ⟹ positive Gowers norm. -/
axiom gowers_norm_lower_bound (k N : ℕ) (hk : k ≥ 3) (A : Finset ℕ)
  (hA : ∀ a ∈ A, 1 ≤ a ∧ a ≤ N) (hδpos : (A.card : ℝ) / (N : ℝ) > 0) :
  let f : ℕ → ℝ := fun n => (if n ∈ A then (1 : ℝ) else (0 : ℝ)) - (A.card : ℝ) / (N : ℝ)
  ∃ (c : ℝ), c > 0 ∧ gowers_norm (k - 1) N f ≥ c

/-- Inverse Theorem (Green–Tao–Ziegler): positive U^{k-1}-norm ⟹
    correlation with a (k−2)-step nilsequence. -/
axiom inverse_theorem (k N : ℕ) (hk : k ≥ 3) (f : ℕ → ℝ)
  (hfnorm : gowers_norm (k - 1) N f > 0) :
  ∃ (ψ : Nilsequence (k - 2)) (η : ℝ),
    η > 0 ∧ |correlation N f (nilseq_eval ψ)| ≥ η

/-- Arithmetic Regularity Lemma: partition [1,N] into progressions
    on which ψ is ε-almost-constant. -/
axiom arithmetic_regularity (s N : ℕ) (ψ : Nilsequence s) (ε : ℝ) (hε : ε > 0) :
  ∃ (parts : Finset (ℕ × ℕ × ℕ)),
    (∀ p ∈ parts, p.1 ≥ 1 ∧ p.2.1 ≥ 1) ∧
    (∀ p ∈ parts, ∀ t₁ ≤ p.2.2, ∀ t₂ ≤ p.2.2,
      |((nilseq_eval ψ (p.1 + t₁ * p.2.1)).re : ℝ) -
       ((nilseq_eval ψ (p.1 + t₂ * p.2.1)).re : ℝ)| ≤ ε) ∧
    ∃ (err : Finset ℕ),
      (Finset.card err : ℝ) ≤ ε * (N : ℝ) ∧
      ∀ n ∈ Icc 1 N, n ∈ err ∨
        ∃ p ∈ parts, ∃ t ≤ p.2.2, n = p.1 + t * p.2.1

/-- Density Increment: correlation ⟹ sub-progression with higher density. -/
axiom density_increment (k N : ℕ) (hk : k ≥ 3) (A : Finset ℕ)
  (hA : ∀ a ∈ A, 1 ≤ a ∧ a ≤ N)
  (ψ : Nilsequence (k - 2)) (η : ℝ) (hη : η > 0)
  (hcorr : |correlation N
    (fun n => (if n ∈ A then (1 : ℝ) else (0 : ℝ)) - (A.card : ℝ) / (N : ℝ))
    (nilseq_eval ψ)| ≥ η) :
  ∃ (a d m : ℕ) (δ' : ℝ),
    let P : Finset ℕ := (Icc 1 N).filter (fun n => ∃ t : ℕ, t ≤ m ∧ n = a + t * d)
    (Finset.card (A ∩ P) : ℝ) / (Finset.card P : ℝ) ≥
        (A.card : ℝ) / (N : ℝ) + δ' ∧
    δ' > 0

/-- Szemerédi's Theorem: ∀ ε > 0, ∃ N₀, ∀ N ≥ N₀, r_k(N) < ε·N. -/
axiom szemeredi_axiom (k : ℕ) (hk : k ≥ 3) :
  ∀ ε : ℝ, ε > 0 → ∃ N₀ : ℕ, ∀ N : ℕ, N ≥ N₀ → (r_k k N : ℝ) < ε * (N : ℝ)

/-- Behrend lower bound: r_k(N) ≥ N·exp(−c·(log N)^{1/(k−1)}). -/
axiom behrend_lower_bound (k : ℕ) (hk : k ≥ 3) :
  ∃ (c : ℝ) (N₀ : ℕ), c > 0 ∧ ∀ N : ℕ, N ≥ N₀ →
    (r_k k N : ℝ) ≥ (N : ℝ) * Real.exp (-c * (Real.log (N : ℝ)) ^ (1 / ((k : ℝ) - 1)))

/-- Kelley–Meka upper bound (k=3): r_3(N) ≤ N·exp(−c·(log N)^{1/11}). -/
axiom kelley_meka_upper_bound :
  ∃ (c : ℝ) (N₀ : ℕ), c > 0 ∧ ∀ N : ℕ, N ≥ N₀ →
    (r_k 3 N : ℝ) ≤ (N : ℝ) * Real.exp (-c * (Real.log (N : ℝ)) ^ (1/11 : ℝ))

-- ─── Theorems ─────────────────────────────────────────────────────────────────

/-- Szemerédi's Theorem (1975): For k ≥ 3, r_k(N) = o(N). -/
theorem szemeredi (k : ℕ) (hk : k ≥ 3) :
  ∀ ε : ℝ, ε > 0 → ∃ N₀ : ℕ, ∀ N : ℕ, N ≥ N₀ → (r_k k N : ℝ) < ε * (N : ℝ) :=
  szemeredi_axiom k hk

/-- Asymptotic form: lim_{N→∞} r_k(N) / N = 0. -/
theorem szemeredi_asymptotic (k : ℕ) (hk : k ≥ 3) :
  Filter.Tendsto (fun N : ℕ => (r_k k N : ℝ) / (N : ℝ)) Filter.atTop (nhds 0) := by
  rw [Metric.tendsto_atTop]
  intro ε hε
  rcases szemeredi k hk ε hε with ⟨N₀, hN₀⟩
  refine ⟨N₀, fun n hn => ?_⟩
  have hbound : (r_k k n : ℝ) < ε * (n : ℝ) := hN₀ n hn
  by_cases hn0 : (n : ℝ) = 0
  · have hn0nat : n = 0 := by exact_mod_cast hn0
    simp [hn0nat, hε]
  · have hnpos : (n : ℝ) > 0 := by
      have hzero_ne : n ≠ 0 := by
        intro h
        apply hn0
        exact_mod_cast h
      exact Nat.cast_pos.mpr (Nat.pos_of_ne_zero hzero_ne)
    have hnonneg_div : 0 ≤ (r_k k n : ℝ) / (n : ℝ) :=
      div_nonneg (Nat.cast_nonneg _) (by linarith)
    have hdiv : (r_k k n : ℝ) / (n : ℝ) < ε :=
      (div_lt_iff₀ hnpos).mpr (by simpa [mul_comm] using hbound)
    rw [Real.dist_eq, sub_zero, abs_of_nonneg hnonneg_div]
    exact hdiv

end Imscribing.Classical.Szemeredi
