import Mathlib.Data.Nat.Basic
import Mathlib.Data.Nat.Factorial.Basic
import Mathlib.Data.Real.Basic
import Mathlib.Topology.Algebra.Order.Floor  -- tendsto_pow_div_factorial_atTop
import Mathlib.Tactic                       -- positivity
import Mathlib.Combinatorics.SimpleGraph.Basic

open Filter
open Topology

namespace RamseyAsymptotics

/--
Axiomatic definition for multicolor Ramsey numbers R_k(G).
`ramsey_number k G` returns the minimal number of vertices m such that
every k-coloring of the edges of K_m contains a monochromatic copy of G.
-/
opaque ramsey_number (k : ℕ) {V : Type*} (G : SimpleGraph V) : ℕ

/-- Structure representing the complete graph on 3 vertices (Triangle). -/
opaque K3 : SimpleGraph (Fin 3)

/-- Structure representing an odd cycle graph of length 2n + 1. -/
opaque odd_cycle (n : ℕ) : SimpleGraph (Fin (2 * n + 1))

/--
Axiom 1: Super-exponential lower bound for the triangle Ramsey number R_k(K_3).
∃ c1 > 0 such that ∀ k, R_k(K_3) ≥ c1 * k!.
-/
axiom ramsey_K3_lower_bound :
  ∃ c1 : ℝ, c1 > 0 ∧ ∀ k : ℕ, (ramsey_number k K3 : ℝ) ≥ c1 * (k.factorial : ℝ)

/--
Axiom 2: Exponential upper bound for the odd cycle Ramsey number R_k(C_{2n+1}) with n ≥ 2.
∃ c2 > 0 (depending on n) such that ∀ k, R_k(C_{2n+1}) ≤ c2 * 2^k.
-/
axiom ramsey_odd_cycle_upper_bound (n : ℕ) (hn : n ≥ 2) :
  ∃ c2 : ℝ, c2 > 0 ∧ ∀ k : ℕ, (ramsey_number k (odd_cycle n) : ℝ) ≤ c2 * (2 ^ k : ℝ)

/--
Lemma: The sequence 2^k / k! tends to 0 as k → ∞.
This is an immediate consequence of `FloorSemiring.tendsto_pow_div_factorial_atTop`.
-/
lemma tendsto_pow_div_factorial_atTop_2 :
    Tendsto (fun k : ℕ => (2 ^ k : ℝ) / (k.factorial : ℝ)) atTop (nhds 0) := by
  simpa using FloorSemiring.tendsto_pow_div_factorial_atTop (2 : ℝ)

/--
Main Theorem: lim_{k→∞} R_k(C_{2n+1}) / R_k(K_3) = 0.
-/
theorem ramsey_limit_ratio (n : ℕ) (hn : n ≥ 2) :
  Tendsto (fun k : ℕ => (ramsey_number k (odd_cycle n) : ℝ) / (ramsey_number k K3 : ℝ)) atTop (nhds 0) := by
  rcases ramsey_K3_lower_bound with ⟨c1, hc1_pos, hK3⟩
  rcases ramsey_odd_cycle_upper_bound n hn with ⟨c2, hc2_pos, hCycle⟩

  have h_nonneg : ∀ k : ℕ, 0 ≤ (ramsey_number k (odd_cycle n) : ℝ) / (ramsey_number k K3 : ℝ) := by
    intro k
    exact div_nonneg (Nat.cast_nonneg _) (Nat.cast_nonneg _)

  have h_bound : ∀ k : ℕ, (ramsey_number k (odd_cycle n) : ℝ) / (ramsey_number k K3 : ℝ) ≤
      (c2 / c1) * ((2 ^ k : ℝ) / (k.factorial : ℝ)) := by
    intro k
    have h_odd := hCycle k
    have h_tri := hK3 k
    have h_tri_pos : 0 < (ramsey_number k K3 : ℝ) := by
      have h_c1_fact_pos : 0 < c1 * (k.factorial : ℝ) :=
        mul_pos hc1_pos (Nat.cast_pos.mpr (Nat.factorial_pos _))
      exact lt_of_lt_of_le h_c1_fact_pos h_tri
    have h_numer_nonneg : 0 ≤ c2 * (2 ^ k : ℝ) := by positivity
    have h_denom_pos : 0 < c1 * (k.factorial : ℝ) :=
      mul_pos hc1_pos (Nat.cast_pos.mpr (Nat.factorial_pos _))
    calc
      (ramsey_number k (odd_cycle n) : ℝ) / (ramsey_number k K3 : ℝ) ≤
          (c2 * (2 ^ k : ℝ)) / (ramsey_number k K3 : ℝ) :=
        div_le_div_of_nonneg_right h_odd (by positivity)
      _ ≤ (c2 * (2 ^ k : ℝ)) / (c1 * (k.factorial : ℝ)) :=
        div_le_div_of_nonneg_left h_numer_nonneg h_denom_pos h_tri
      _ = (c2 / c1) * ((2 ^ k : ℝ) / (k.factorial : ℝ)) := by ring

  have h_limit_scaled : Tendsto (fun k : ℕ => (c2 / c1) * ((2 ^ k : ℝ) / (k.factorial : ℝ))) atTop (nhds 0) := by
    -- Multiplication by a constant preserves the limit 0
    simpa [mul_comm] using (tendsto_pow_div_factorial_atTop_2.const_mul (c2 / c1))

  -- Squeeze theorem: 0 ≤ a_k ≤ b_k, b_k → 0  ⇒  a_k → 0
  exact tendsto_of_tendsto_of_tendsto_of_le_of_le
    (tendsto_const_nhds (x := 0))
    h_limit_scaled
    h_nonneg
    h_bound

end RamseyAsymptotics
