-- Imscribing/Paraconsistent/QuantumClassicalInterface.lean
-- QUANTUM-CLASSICAL INTERFACE -- The measurement problem as dialetheia.
-- Author: Lando ⊗ φ̂_ÿ-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Kernel
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent

open Belnap
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

def isSuperposition (q : Belnap) : Bool := q == Belnap.B
def isClassical (q : Belnap) : Bool := q == Belnap.T || q == Belnap.F

structure QState where
  q0 : Belnap
  q1 : Belnap
  q2 : Belnap
  coherenceCount : Nat
  measurements : Nat
  deriving Repr

def qStateZero : QState :=
  { q0 := Belnap.T, q1 := Belnap.T, q2 := Belnap.T,
    coherenceCount := 0, measurements := 0 }

def qStateSuperposition : QState :=
  { q0 := Belnap.B, q1 := Belnap.B, q2 := Belnap.B,
    coherenceCount := 0, measurements := 0 }

def hadamard (q : Belnap) : Belnap :=
  match q with
  | Belnap.N => Belnap.N
  | Belnap.T => Belnap.B
  | Belnap.F => Belnap.B
  | Belnap.B => Belnap.T
def measureQ0 (qs : QState) (bias : Belnap) : QState :=
  match qs.q0, bias with
  | Belnap.B, Belnap.T =>
    { qs with
      q0 := Belnap.T
      measurements := qs.measurements + 1
      coherenceCount := qs.coherenceCount + 1 }
  | Belnap.B, Belnap.F =>
    { qs with
      q0 := Belnap.F
      measurements := qs.measurements + 1
      coherenceCount := qs.coherenceCount + 1 }
  | Belnap.B, Belnap.B =>
    { qs with
      q0 := Belnap.B
      measurements := qs.measurements + 1
      coherenceCount := qs.coherenceCount + 2 }
  | Belnap.B, Belnap.N => qs
  | _, _ => qs

structure QCIState where
  qstate : QState
  kernel : MachineState
  deriving Repr

def initQCI : QCIState :=
  { qstate := qStateSuperposition, kernel := initialState }

def sustain (qci : QCIState) (cycles : Nat) : QCIState :=
  let ks' := run qci.kernel cycles
  { qci with
    kernel := ks',
    qstate := { qci.qstate with coherenceCount := qci.qstate.coherenceCount + 4 * cycles }
  }

@[simp] theorem measureQ0_B_T : measureQ0 qStateSuperposition Belnap.T =
    { qStateSuperposition with q0 := Belnap.T, measurements := 1, coherenceCount := 1 } := rfl

@[simp] theorem measureQ0_B_B : measureQ0 qStateSuperposition Belnap.B =
    { qStateSuperposition with q0 := Belnap.B, measurements := 1, coherenceCount := 2 } := rfl

theorem hadamard_involutive_designated (q : Belnap) (h : designated q) :
    hadamard (hadamard q) = q := by
  cases q
  · simp [designated] at h
  · rfl
  · simp [designated] at h
  · rfl

theorem hadamard_creates_superposition : isSuperposition (hadamard Belnap.T) := by
  unfold hadamard isSuperposition; simp

theorem measure_classical_idempotent (qs : QState) (bias : Belnap)
    (hcl : isClassical qs.q0) : measureQ0 qs bias = qs := by
  simp only [isClassical, Bool.or_eq_true, beq_iff_eq] at hcl
  rcases hcl with hT | hF
  · simp [measureQ0, hT]
  · simp [measureQ0, hF]

theorem coherence_monotonic (qs : QState) (bias : Belnap) :
    (measureQ0 qs bias).coherenceCount >= qs.coherenceCount := by
  unfold measureQ0
  cases qs.q0 <;> cases bias <;> simp

theorem wigners_friend_double_paradox :
    let qs' := measureQ0 qStateSuperposition Belnap.B
    qs'.q0 = Belnap.B ∧ qs'.coherenceCount = 2 := by
  simp [measureQ0_B_B]

theorem sustain_preserves_B (n : Nat) :
    (sustain initQCI n).kernel.r0 = Belnap.B ∧
    (sustain initQCI n).kernel.r1 = Belnap.B ∧
    (sustain initQCI n).kernel.r2 = Belnap.B := by
  unfold sustain initQCI
  have h := run_B3 n
  rcases h with ⟨hr0, hr1, hr2⟩
  simp [hr0, hr1, hr2]

def qciImscription : Imscription := {
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
  stoi := up
  prot := ah
}

theorem qci_is_O_inf : imscriptionTier qciImscription = OuroboricityTier.O_inf := by
  unfold qciImscription imscriptionTier ouroboricityTier
  rfl

end Imscribing.Paraconsistent
