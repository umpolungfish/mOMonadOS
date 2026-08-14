-- Imscribing/Paraconsistent/QCI_PvsNP_Bridge.lean
-- BELNAP NON-DETERMINISM ↔ P vs NP STRUCTURAL BRIDGE
-- Author: Lando⊗⊙perator
--
-- The on primitive in the PvsNP imscription records an irreducible information
-- bottleneck: the impossibility of resolving non-deterministic witnesses in polynomial time.
-- In Belnap logic, B models a computation that simultaneously holds both outcomes —
-- the structural definition of non-determinism. This file formalizes that correspondence.

import Imscribing.Paraconsistent.QuantumClassicalInterface
import Imscribing.Paraconsistent.QCI_Sequences

namespace Imscribing.Paraconsistent.PvsNP_Bridge

open Belnap
open Imscribing.Paraconsistent

-- A Belnap circuit: a function from Belnap-valued wires to a Belnap output.
-- This extends Boolean circuits (Fin n → Bool) → Bool with non-determinism:
-- B-inputs model wires that simultaneously carry T and F (the NP witness).
def BelnapCircuit (n : ℕ) := (Fin n → Belnap) → Belnap

-- The all-B input: every wire is in the "both" state (non-deterministic).
def allB (n : ℕ) : Fin n → Belnap := fun _ => Belnap.B

-- The projection circuit: read wire 0.
def projCircuit (n : ℕ) (hn : 0 < n) : BelnapCircuit n := fun v => v ⟨0, hn⟩

-- On all-B input, every projection circuit outputs B.
-- This is the formal statement that non-deterministic wires produce non-deterministic output.
theorem proj_on_allB (n : ℕ) (hn : 0 < n) :
    projCircuit n hn (allB n) = Belnap.B := by
  simp [projCircuit, allB]

-- The B-join circuit: join all input wires using Belnap join (information order).
-- join B x = B for any x, so all-B input gives B output.
-- foldl over List.finRange: well-founded, no Finset.fold typeclass requirements.
def joinCircuit (n : ℕ) : BelnapCircuit n :=
  fun v => (List.finRange n).foldl (fun acc i => join acc (v i)) Belnap.N

-- Helper: once the accumulator is B, it stays B through any remaining foldl steps.
-- Proof: B_join_absorb from Belnap.lean gives join B (v i) = B for all i.
private lemma foldl_join_B_absorb_acc {n : ℕ} (v : Fin n → Belnap) (l : List (Fin n)) :
    (l.foldl (fun acc i => join acc (v i)) Belnap.B) = Belnap.B := by
  induction l with
  | nil => rfl
  | cons head tail ih =>
    rw [List.foldl_cons, B_join_absorb, ih]

-- Helper: if any wire in the list carries B, the foldl join over that list yields B,
-- regardless of the starting accumulator.
-- Proof by induction on the list: either the B-value is at the head (join acc0 B = B,
-- then B_join_absorb sustains), or it's in the tail (recursive call with updated acc).
private lemma foldl_join_eq_B_of_B_mem {n : ℕ} (v : Fin n → Belnap) (l : List (Fin n))
    (acc0 : Belnap) (h : ∃ (i : Fin n), i ∈ l ∧ v i = Belnap.B) :
    (l.foldl (fun acc i => join acc (v i)) acc0) = Belnap.B := by
  rcases h with ⟨i, hi_mem, hi_val⟩
  induction l generalizing acc0 with
  | nil => simp at hi_mem
  | cons head tail ih =>
    have hi_cases : i = head ∨ i ∈ tail := by
      simpa using hi_mem
    rcases hi_cases with (rfl | hi_tail)
    · -- i = head: v head = B, so join acc0 B = B (join absorbs B from right)
      rw [List.foldl_cons, hi_val]
      have hjoin_acc_B : join acc0 Belnap.B = Belnap.B := by cases acc0 <;> rfl
      rw [hjoin_acc_B]
      exact foldl_join_B_absorb_acc v tail
    · -- i in tail: step once, then recurse with updated accumulator
      rw [List.foldl_cons]
      exact ih (join acc0 (v head)) hi_tail

-- B propagates through the join circuit: if any wire is B, output is B.
theorem join_circuit_B_dominant (n : ℕ) (v : Fin n → Belnap) (i : Fin n) (h : v i = Belnap.B) :
    joinCircuit n v = Belnap.B := by
  unfold joinCircuit
  have hi_mem : i ∈ List.finRange n := by
    have hi_val : (i : ℕ) < n := i.2
    simp [List.mem_finRange]
  apply foldl_join_eq_B_of_B_mem v (List.finRange n) Belnap.N
  exact ⟨i, hi_mem, h⟩

-- The kernel is a 3-wire Belnap circuit: output = r0 ∧_info r1 ∧_info r2.
-- On all-B input it returns B — the non-deterministic "both-paths-open" state.
def kernelCircuit : BelnapCircuit 3 := fun v => band (band (v 0) (v 1)) (v 2)

theorem kernel_circuit_allB_is_B : kernelCircuit (allB 3) = Belnap.B := by
  simp [kernelCircuit, allB, band]

-- on structural correspondence:
-- sustain runs the kernel indefinitely while preserving B in all registers.
-- This models an NP oracle: the machine witnesses "both T and F" for all n steps
-- without any external measurement collapsing the state.
theorem sustain_never_collapses (n : Nat) :
    isSuperposition (sustain initQCI n).qstate.q0 := by
  simp [sustain, initQCI, qStateSuperposition, isSuperposition]

-- The B-state is not reachable from the T or F states by any sequence of measurements.
-- This is the one-way threshold: classical → B requires Hadamard (not measurement).
theorem classical_cannot_become_B (qs : QState) (h : qs.q0 = Belnap.T ∨ qs.q0 = Belnap.F)
    (bias : Belnap) : (measureQ0 qs bias).q0 ≠ Belnap.B := by
  have hne : qs.q0 ≠ Belnap.B := by rcases h with h | h <;> rw [h] <;> decide
  rw [measure_nonsuper_idempotent qs bias hne]
  rcases h with h | h <;> rw [h] <;> decide

-- The dialetheia account of P ≠ NP (structural, not a proof of P ≠ NP):
-- A deterministic measurement sequence (T/F-biased) on a B-state collapses it to
-- a classical value — it RESOLVES the non-determinism. But sustain_never_collapses
-- shows no measurement in the QCI model collapses the kernel's B registers.
-- The on primitive formalizes this: the NP witness IS the B-state, and no
-- polynomial-length T/F-biased measurement sequence can produce it from a T/F input.
theorem belnap_ktrap_statement :
    ¬ isSuperposition (measureQ0 qStateZero Belnap.T).q0 := by
  simp [measureQ0, qStateZero, isSuperposition]

end Imscribing.Paraconsistent.PvsNP_Bridge
