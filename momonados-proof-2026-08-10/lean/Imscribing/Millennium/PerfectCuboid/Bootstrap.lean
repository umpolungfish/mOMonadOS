-- Imscribing/Millennium/PerfectCuboid/Bootstrap.lean
-- BETTER BOOTSTRAP PROBLEM -- Meta-Principle for Infinite Descent
-- Author: Lando (x) Operator

import Mathlib.Order.WellFounded
import Mathlib.Data.Nat.Basic

namespace Millennium.PerfectCuboid.Bootstrap

universe u v

/--
BetterBootstrapProblem captures infinite descent proofs.
-/
class BetterBootstrapProblem (Global : Type u) (Local : Type v) where
  delta : Global -> Local
  mu : Local -> Global
  measure : Local -> Nat
  descent : Global -> Global
  base : Global -> Prop
  id_property : forall g, mu (delta g) = g
  descent_property : forall g, Not (base g) -> measure (delta (descent g)) < measure (delta g)
  descent_preserves_non_base : forall g, Not (base g) -> Not (base (descent g))
  measure_wf : WellFounded (fun (a b : Local) => measure a < measure b)
  base_fixed : forall g, base g -> descent g = g

/--
no_non_base_global: every object in a BBP is base.
-/
theorem no_non_base_global {G L : Type}
    (delta : G -> L) (mu : L -> G) (measure : L -> Nat)
    (descent : G -> G) (base : G -> Prop)
    (id_property : forall g, mu (delta g) = g)
    (descent_property : forall g, Not (base g) -> measure (delta (descent g)) < measure (delta g))
    (descent_preserves_non_base : forall g, Not (base g) -> Not (base (descent g)))
    (measure_wf : WellFounded (fun (a b : L) => measure a < measure b))
    (base_fixed : forall g, base g -> descent g = g)
    (g : G) : base g := by
  by_contra h_non_base
  let iter : Nat -> G := Nat.rec g (fun _ prev => descent prev)
  have iter_succ : forall n, iter (n+1) = descent (iter n) := fun _ => rfl
  have not_base_iter : forall n, Not (base (iter n)) := by
    intro n
    induction n with
    | zero => exact h_non_base
    | succ n ih =>
      rw [iter_succ n]
      exact descent_preserves_non_base (iter n) ih
  have h_chain : forall n,
      measure (delta (iter (n+1))) < measure (delta (iter n)) := by
    intro n
    rw [iter_succ n]
    exact descent_property (iter n) (not_base_iter n)
  let vals : Nat -> Nat := fun n => measure (delta (iter n))
  have h_lt : forall n, vals (n+1) < vals n := h_chain
  have h_bound : forall n, vals n + n <= vals 0 := by
    intro n
    induction n with
    | zero =>
      simp [vals]
    | succ n ih =>
      have hlt_n : vals (n+1) < vals n := h_lt n
      have h_succ_le : vals (n+1) + 1 <= vals n := Nat.succ_le_of_lt hlt_n
      calc
        vals (n+1) + (n+1) = (vals (n+1) + 1) + n := by omega
        _ <= vals n + n := Nat.add_le_add_right h_succ_le n
        _ <= vals 0 := ih
  -- Now the contradiction: at n = vals 0 + 1
  have h_contra := h_bound (vals 0 + 1)
  -- h_contra: vals (vals 0 + 1) + (vals 0 + 1) <= vals 0
  -- Since vals _ >= 0, we get vals 0 + 1 <= vals 0
  have h_nonneg : 0 <= vals (vals 0 + 1) := Nat.zero_le _
  have : vals 0 + 1 <= vals 0 := by omega
  -- But vals 0 + 1 > vals 0
  have h_gt : vals 0 < vals 0 + 1 := by omega
  have : vals 0 < vals 0 := Nat.lt_of_lt_of_le h_gt this
  exact Nat.lt_irrefl _ this

end Millennium.PerfectCuboid.Bootstrap
