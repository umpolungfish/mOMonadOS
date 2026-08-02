/-
Imscribing/Classical/ErdosStrausDecomposition.lean

Erdős–Straus Conjecture: Structural Decomposition via the Nesting Technique.
Author: Lando⊗⊙perator

Pattern: ⊢ ⊙ ◇ ◇ + > ● = < × ∋ ¬ ⊣ → TF (dialetheic via weight_flow)

The weight_flow analysis lands in TF: both T and F survive.
- T: the congruence class decomposition scheme covers all n
- F: the actual construction for n≡1 mod 4 is not completed (genuinely open)
-/

import Mathlib
open Finset

namespace Imscribing.Classical.ErdosStrausDecomposition

set_option linter.unusedVariables false
set_option linter.style.openClassical false

-- ── The decomposition structure ─────────────────────────

structure EgyptianDecomposition (n : ℕ) where
  x : ℕ
  y : ℕ
  z : ℕ
  hx_pos : 0 < x
  hy_pos : 0 < y
  hz_pos : 0 < z
  h_eq : (4 : ℚ) / (n : ℚ) = (1 : ℚ) / (x : ℚ) + (1 : ℚ) / (y : ℚ) + (1 : ℚ) / (z : ℚ)

def erdosStrausConjecture : Prop :=
  ∀ n : ℕ, n ≥ 2 → Nonempty (EgyptianDecomposition n)

-- ── Three congruence classes solved (T-face) ──────────────

theorem class_zero_exists (n : ℕ) (hn : n % 4 = 0) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) := by
  have h₁ : ∃ k : ℕ, n = 4 * k := by
    use n / 4
    have h₂ : n % 4 = 0 := hn
    have h₃ : n = 4 * (n / 4) := by
      have h₄ := Nat.div_add_mod n 4
      omega
    exact h₃
  obtain ⟨k, rfl⟩ := h₁
  have h₂ : k ≥ 1 := by
    by_contra h
    have h₃ : k = 0 := by omega
    rw [h₃] at *
    norm_num at hn2 ⊢
    <;> omega
  -- Use the parametric formula: 4/(4k) = 1/(k+1) + 1/(k(k+1)+1) + 1/(k(k+1)*(k(k+1)+1))
  have h₃ : 0 < (k + 1 : ℕ) := by linarith
  have h₄ : 0 < (k * (k + 1) + 1 : ℕ) := by
    have h₅ : 0 < k := by linarith
    nlinarith
  have h₅ : 0 < (k * (k + 1) * (k * (k + 1) + 1) : ℕ) := by
    have h₆ : 0 < k := by linarith
    have h₇ : 0 < (k + 1 : ℕ) := by linarith
    have h₈ : 0 < (k * (k + 1) : ℕ) := by positivity
    have h₉ : 0 < (k * (k + 1) + 1 : ℕ) := by positivity
    positivity
  -- Verify the equation
  have h₆ : (4 : ℚ) / ((4 * k : ℕ) : ℚ) = (1 : ℚ) / ((k + 1 : ℕ) : ℚ) + (1 : ℚ) / ((k * (k + 1) + 1 : ℕ) : ℚ) + (1 : ℚ) / ((k * (k + 1) * (k * (k + 1) + 1) : ℕ) : ℚ) := by
    have h₇ : (k : ℚ) ≥ 1 := by exact_mod_cast h₂
    field_simp [h₃, h₄, h₅]
    <;> ring_nf
    <;> field_simp [h₃, h₄, h₅]
    <;> ring_nf
    <;> norm_cast
    <;> field_simp [h₃, h₄, h₅] at *
    <;> ring_nf at *
    <;> norm_cast at *
    <;> nlinarith
  refine' ⟨⟨k + 1, k * (k + 1) + 1, k * (k + 1) * (k * (k + 1) + 1), by positivity, by positivity, by positivity, _⟩⟩
  exact h₆

theorem class_two_exists (n : ℕ) (hn : n % 4 = 2) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) := by
  have h₁ : ∃ k : ℕ, n = 4 * k + 2 := by
    use n / 4
    have h₂ : n % 4 = 2 := hn
    have h₃ : n = 4 * (n / 4) + 2 := by
      have h₄ := Nat.div_add_mod n 4
      omega
    exact h₃
  obtain ⟨k, rfl⟩ := h₁
  -- Use the parametric formula: 4/(4k+2) = 1/(k+1) + 1/(2*(k+1)*(2k+1)) + 1/(2*(k+1)*(2k+1))
  have h₂ : 0 < (k + 1 : ℕ) := by
    by_contra h
    have h₃ : k = 0 := by omega
    have h₄ : (4 * k + 2 : ℕ) ≥ 2 := by omega
    simp [h₃] at h₄ ⊢
    <;> omega
  have h₃ : 0 < (2 * (k + 1) * (2 * k + 1) : ℕ) := by
    have h₄ : 0 < (k + 1 : ℕ) := by positivity
    have h₅ : 0 < (2 * k + 1 : ℕ) := by
      have h₆ : 0 ≤ k := by omega
      omega
    positivity
  -- Verify the equation
  have h₄ : (4 : ℚ) / ((4 * k + 2 : ℕ) : ℚ) = (1 : ℚ) / ((k + 1 : ℕ) : ℚ) + (1 : ℚ) / ((2 * (k + 1) * (2 * k + 1) : ℕ) : ℚ) + (1 : ℚ) / ((2 * (k + 1) * (2 * k + 1) : ℕ) : ℚ) := by
    have h₅ : (k : ℚ) ≥ 0 := by exact_mod_cast (by omega)
    field_simp [h₂, h₃]
    <;> ring_nf
    <;> field_simp [h₂, h₃]
    <;> ring_nf
    <;> norm_cast
    <;> field_simp [h₂, h₃] at *
    <;> ring_nf at *
    <;> norm_cast at *
    <;> nlinarith
  refine' ⟨⟨k + 1, 2 * (k + 1) * (2 * k + 1), 2 * (k + 1) * (2 * k + 1), by positivity, by positivity, by positivity, _⟩⟩
  exact h₄

theorem class_three_exists (n : ℕ) (hn : n % 4 = 3) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) := by
  have h₁ : ∃ k : ℕ, n = 4 * k + 3 := by
    use n / 4
    have h₂ : n % 4 = 3 := hn
    have h₃ : n = 4 * (n / 4) + 3 := by
      have h₄ := Nat.div_add_mod n 4
      omega
    exact h₃
  obtain ⟨k, rfl⟩ := h₁
  -- Use the parametric formula: 4/(4k+3) = 1/(k+1) + 1/((k+1)(4k+3)+1) + 1/(((k+1)(4k+3)+1)*(k+1)(4k+3))
  have h₂ : 0 < (k + 1 : ℕ) := by
    by_contra h
    have h₃ : k = 0 := by omega
    have h₄ : (4 * k + 3 : ℕ) ≥ 2 := by omega
    simp [h₃] at h₄ ⊢
    <;> omega
  have h₃ : 0 < ((k + 1) * (4 * k + 3) + 1 : ℕ) := by
    have h₄ : 0 < (k + 1 : ℕ) := by positivity
    have h₅ : 0 < (4 * k + 3 : ℕ) := by
      have h₆ : 0 ≤ k := by omega
      omega
    positivity
  have h₄ : 0 < (((k + 1) * (4 * k + 3) + 1) * (k + 1) * (4 * k + 3) : ℕ) := by
    have h₅ : 0 < (k + 1 : ℕ) := by positivity
    have h₆ : 0 < (4 * k + 3 : ℕ) := by
      have h₇ : 0 ≤ k := by omega
      omega
    have h₇ : 0 < ((k + 1) * (4 * k + 3) + 1 : ℕ) := by positivity
    positivity
  -- Verify the equation
  have h₅ : (4 : ℚ) / ((4 * k + 3 : ℕ) : ℚ) = (1 : ℚ) / ((k + 1 : ℕ) : ℚ) + (1 : ℚ) / (((k + 1) * (4 * k + 3) + 1 : ℕ) : ℚ) + (1 : ℚ) / ((((k + 1) * (4 * k + 3) + 1) * (k + 1) * (4 * k + 3) : ℕ) : ℚ) := by
    have h₆ : (k : ℚ) ≥ 0 := by exact_mod_cast (by omega)
    field_simp [h₂, h₃, h₄]
    <;> ring_nf
    <;> field_simp [h₂, h₃, h₄]
    <;> ring_nf
    <;> norm_cast
    <;> field_simp [h₂, h₃, h₄] at *
    <;> ring_nf at *
    <;> norm_cast at *
    <;> nlinarith
  refine' ⟨⟨k + 1, (k + 1) * (4 * k + 3) + 1, ((k + 1) * (4 * k + 3) + 1) * (k + 1) * (4 * k + 3), by positivity, by positivity, by positivity, _⟩⟩
  exact h₅

-- ── One congruence class open (F-face) ───────────────────

theorem class_one_exists (n : ℕ) (hn : n % 4 = 1) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) :=
  sorry

-- ── Structural cover (no sorries — the T-face) ────────────

theorem congruence_classes_cover (n : ℕ) : n % 4 = 0 ∨ n % 4 = 1 ∨ n % 4 = 2 ∨ n % 4 = 3 := by
  have hlt : n % 4 < 4 := Nat.mod_lt n (by norm_num)
  omega

theorem structural_cover_is_complete (n : ℕ) (hn : n ≥ 2) : n % 4 = 0 ∨ n % 4 = 1 ∨ n % 4 = 2 ∨ n % 4 = 3 :=
  congruence_classes_cover n

-- ── Incompleteness witness (the F-face) ──────────────────

theorem decomposition_mod4_zero (n : ℕ) (hn : n % 4 = 0) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) :=
  class_zero_exists n hn hn2

theorem decomposition_mod4_two (n : ℕ) (hn : n % 4 = 2) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) :=
  class_two_exists n hn hn2

theorem decomposition_mod4_three (n : ℕ) (hn : n % 4 = 3) (hn2 : n ≥ 2) : Nonempty (EgyptianDecomposition n) :=
  class_three_exists n hn hn2

-- Sorry count: 1 (class_one_exists — genuinely open)
-- Three of four congruence classes resolved: n≡0,2,3 mod 4 ✓
-- n≡1 mod 4 remains the open heart of the conjecture.
-- This file captures the structural boundary: case analysis is complete (T)
-- but the n≡1 construction is not (F), yielding the dialetheic TF state.

end Imscribing.Classical.ErdosStrausDecomposition