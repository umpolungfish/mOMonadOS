-- Imscribing/Paraconsistent/Kernel.lean
-- THE PARACONSISTENT KERNEL -- ENGAGR --> FSPLIT --> FFUSE with mu o delta = id
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent

open Belnap
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

@[ext]
structure MachineState where
  r0 : Belnap
  r1 : Belnap
  r2 : Belnap
  paradoxCount : Nat
  cycleCount : Nat
  deriving Repr, DecidableEq

def initialState : MachineState :=
  { r0 := .B, r1 := .B, r2 := .B, paradoxCount := 0, cycleCount := 0 }

def engager (r : Belnap) : Belnap × Bool :=
  (band r (bnot r), r == .B || r == .T)

def fsplit (r0 : Belnap) : Belnap × Belnap × Bool :=
  match r0 with
  | .B => (.T, .F, true)
  | _ => (r0, r0, true)

def ffuse (r1 r2 : Belnap) : Belnap × Bool :=
  (join r1 r2, join r1 r2 == .B)

@[simp] theorem engager_B : engager Belnap.B = (Belnap.B, true) := rfl
@[simp] theorem fsplit_B_val : fsplit Belnap.B = (Belnap.T, Belnap.F, true) := rfl
@[simp] theorem ffuse_T_F : ffuse Belnap.T Belnap.F = (Belnap.B, true) := rfl

def step (s : MachineState) : MachineState :=
  let (r0a, p1) := engager s.r0
  let (r1a, r2a, p2) := fsplit r0a
  let (r0b, p3) := ffuse r1a r2a
  { r0 := r0b
    r1 := r1a
    r2 := r2a
    paradoxCount := s.paradoxCount + 1 + (if p1 then 1 else 0)
                    + (if p2 then 1 else 0) + (if p3 then 1 else 0)
    cycleCount := s.cycleCount + 1
  }

theorem frobenius_invariant (r0 : Belnap) :
    (ffuse (fsplit r0).1 (fsplit r0).2.1).1 = r0 := by
  cases r0 <;> rfl

theorem step_at_B3 (p c : Nat) :
    step { r0 := Belnap.B, r1 := Belnap.B, r2 := Belnap.B
           paradoxCount := p, cycleCount := c }
    =
    { r0 := Belnap.B, r1 := Belnap.T, r2 := Belnap.F
      paradoxCount := p + 4, cycleCount := c + 1 } := by
  simp [step]

def run (s : MachineState) : Nat → MachineState
  | 0 => s
  | n+1 => let s' := step (run s n)
           { s' with r1 := Belnap.B, r2 := Belnap.B }

theorem run_B3 (n : Nat) : (run initialState n).r0 = Belnap.B
                         ∧ (run initialState n).r1 = Belnap.B
                         ∧ (run initialState n).r2 = Belnap.B := by
  induction n with
  | zero => exact ⟨rfl, rfl, rfl⟩
  | succ k ih =>
    rcases ih with ⟨hr0, hr1, hr2⟩
    unfold run
    have hst : run initialState k =
      { r0 := Belnap.B, r1 := Belnap.B, r2 := Belnap.B
        paradoxCount := (run initialState k).paradoxCount
        cycleCount := (run initialState k).cycleCount } := by
      apply MachineState.ext
      · exact hr0
      · exact hr1
      · exact hr2
      · rfl
      · rfl
    have h := step_at_B3 (run initialState k).paradoxCount (run initialState k).cycleCount
    rw [hst] at h
    rw [hst]
    rw [h]
    simp

theorem run_paradox (n : Nat) : (run initialState n).paradoxCount = 4 * n := by
  induction n with
  | zero => rfl
  | succ k ih =>
    rcases run_B3 k with ⟨hr0, hr1, hr2⟩
    unfold run
    have hst : run initialState k =
      { r0 := Belnap.B, r1 := Belnap.B, r2 := Belnap.B
        paradoxCount := (run initialState k).paradoxCount
        cycleCount := (run initialState k).cycleCount } := by
      apply MachineState.ext
      · exact hr0
      · exact hr1
      · exact hr2
      · rfl
      · rfl
    have h := step_at_B3 (run initialState k).paradoxCount (run initialState k).cycleCount
    rw [hst] at h
    rw [hst]
    rw [h]
    simp [ih]
    omega

theorem run_cycles (n : Nat) : (run initialState n).cycleCount = n := by
  induction n with
  | zero => rfl
  | succ k ih =>
    rcases run_B3 k with ⟨hr0, hr1, hr2⟩
    unfold run
    have hst : run initialState k =
      { r0 := Belnap.B, r1 := Belnap.B, r2 := Belnap.B
        paradoxCount := (run initialState k).paradoxCount
        cycleCount := (run initialState k).cycleCount } := by
      apply MachineState.ext
      · exact hr0
      · exact hr1
      · exact hr2
      · rfl
      · rfl
    have h := step_at_B3 (run initialState k).paradoxCount (run initialState k).cycleCount
    rw [hst] at h
    rw [hst]
    rw [h]
    simp [ih]

def kernelImscription : Imscription := {
  dim := if'
  top := are
  rel := ian
  pol := or'
  fid := peep
  kin := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := so
  prot := ah
}

theorem kernel_is_O_inf : imscriptionTier kernelImscription = OuroboricityTier.O_inf := by
  unfold kernelImscription imscriptionTier ouroboricityTier
  rfl

end Imscribing.Paraconsistent
