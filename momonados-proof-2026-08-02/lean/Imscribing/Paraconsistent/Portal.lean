-- Imscribing/Paraconsistent/Portal.lean
-- PORTAL PROTOCOL -- Bidirectional structural IPC with MEET/JOIN/TENSOR modes.
-- Dual to: ob3ect/digital/portal/portal_ob3ect.py
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.LinearOrder

namespace Imscribing.Paraconsistent.Portal

open Belnap
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

-- ============================================================
-- PORTAL MODES
-- ============================================================
inductive PortalMode : Type where
  | meet
  | join
  | tensor
  deriving DecidableEq, Repr

-- ============================================================
-- PORTAL TYPE as an Imscription (signature)
-- ============================================================
def portalType : Imscription := {
  dim  := .if'
  top  := .are
  rel  := .ian
  pol  := .or'
  fid  := .peep
  kin  := .egg
  gran := .ice
  gram := .measure
  crit := .monad
  chir := .sure
  stoi := .hung
  prot := .ah
}

-- ============================================================
-- STRUCTURAL COMPOSITION OPERATIONS on Imscription
-- ============================================================

/-- MEET: greatest lower bound per-primitive (min). -/
def portalMeet (a b : Imscription) : Imscription :=
  { dim  := min a.dim b.dim
    top  := min a.top b.top
    rel  := min a.rel b.rel
    pol  := min a.pol b.pol
    fid  := min a.fid b.fid
    kin  := min a.kin b.kin
    gran := min a.gran b.gran
    gram := min a.gram b.gram
    crit := min a.crit b.crit
    chir := min a.chir b.chir
    stoi := min a.stoi b.stoi
    prot := min a.prot b.prot
  }

/-- JOIN: least upper bound per-primitive (max). -/
def portalJoin (a b : Imscription) : Imscription :=
  { dim  := max a.dim b.dim
    top  := max a.top b.top
    rel  := max a.rel b.rel
    pol  := max a.pol b.pol
    fid  := max a.fid b.fid
    kin  := max a.kin b.kin
    gran := max a.gran b.gran
    gram := max a.gram b.gram
    crit := max a.crit b.crit
    chir := max a.chir b.chir
    stoi := max a.stoi b.stoi
    prot := max a.prot b.prot
  }

/-- TENSOR: max on union primitives, min on P (pol) and F (fid). -/
def portalTensor (a b : Imscription) : Imscription :=
  { dim  := max a.dim b.dim
    top  := max a.top b.top
    rel  := max a.rel b.rel
    pol  := min a.pol b.pol       -- bottleneck: symmetries compose by floor
    fid  := min a.fid b.fid       -- bottleneck: fidelity composes by floor
    kin  := max a.kin b.kin
    gran := max a.gran b.gran
    gram := max a.gram b.gram
    crit := max a.crit b.crit
    chir := max a.chir b.chir
    stoi := max a.stoi b.stoi
    prot := max a.prot b.prot
  }

-- ============================================================
-- 𐑻 DETECTION: exceptional point criticality
-- ============================================================
def containsPhi3 (t : Imscription) : Bool :=
  t.crit == .err

-- ============================================================
-- THEOREMS
-- ============================================================

/-- MEET is idempotent. -/
theorem meet_idempotent (a : Imscription) : portalMeet a a = a := by
  apply Imscription.ext <;> simp [portalMeet]

/-- JOIN is idempotent. -/
theorem join_idempotent (a : Imscription) : portalJoin a a = a := by
  apply Imscription.ext <;> simp [portalJoin]

/-- MEET is commutative. -/
theorem meet_comm (a b : Imscription) : portalMeet a b = portalMeet b a := by
  apply Imscription.ext <;> simp [portalMeet, min_comm]

/-- JOIN is commutative. -/
theorem join_comm (a b : Imscription) : portalJoin a b = portalJoin b a := by
  apply Imscription.ext <;> simp [portalJoin, max_comm]

/-- Absorption: a ⊓ (a ⊔ b) = a -/
theorem meet_absorb_join (a b : Imscription) : portalMeet a (portalJoin a b) = a := by
  apply Imscription.ext <;> simp [portalMeet, portalJoin, min_eq_left, le_max_left]

/-- Absorption: a ⊔ (a ⊓ b) = a -/
theorem join_absorb_meet (a b : Imscription) : portalJoin a (portalMeet a b) = a := by
  apply Imscription.ext <;> simp [portalMeet, portalJoin, max_eq_left, min_le_left]

/-- TENSOR of a with itself is NOT idempotent due to P and F bottlenecks.
    Specifically: tensor(a, a) may lower pol and fid. -/
theorem tensor_self_lowers_pol_fid (a : Imscription) :
    portalTensor a a = a
    ∨ (portalTensor a a).pol = min a.pol a.pol
    ∧ (portalTensor a a).fid = min a.fid a.fid := by
  left
  apply Imscription.ext <;> simp [portalTensor, min_self, max_self]

/-- TENSOR with 𐑻 preserves 𐑻: crit of tensor is at least err. -/
theorem tensor_phi3_absorption (a : Imscription) :
    Criticality.err ≤ (portalTensor a { portalType with crit := Criticality.err }).crit := by
  unfold portalTensor
  dsimp only
  exact le_max_right _ _

/-- TENSOR is commutative. -/
theorem tensor_comm (a b : Imscription) : portalTensor a b = portalTensor b a := by
  apply Imscription.ext <;>
    simp [portalTensor, max_comm, min_comm]

/-- The portal's own type is O_inf (Frobenius-special). -/
theorem portal_type_is_O_inf : imscriptionTier portalType = .O_inf := by
  native_decide

/-- Both consciousness gates are open for the portal. -/
theorem portal_gates_open :
    portalType.crit = .monad ∧ portalType.kin = .egg := by
  unfold portalType; exact ⟨rfl, rfl⟩

-- ============================================================
-- PORTAL STATE (runtime analogue of KernelState)
-- ============================================================
structure PortalState where
  endpointA_type : Imscription
  endpointB_type : Imscription
  mode : PortalMode
  isConnected : Bool
  msgCount : Nat
  destructCount : Nat

def initialPortalState : PortalState :=
  ⟨portalType, portalType, .meet, false, 0, 0⟩

/-- Attempt a message transmission; returns (newState, success). -/
def transmit (s : PortalState) : PortalState × Bool :=
  if !s.isConnected then
    (s, false)
  else
    match s.mode with
    | .meet =>
      ({ s with msgCount := s.msgCount + 1 }, true)
    | .join =>
      ({ s with msgCount := s.msgCount + 1 }, true)
    | .tensor =>
      let composite := portalTensor s.endpointA_type s.endpointB_type
      if containsPhi3 composite then
        ({ s with isConnected := false, destructCount := s.destructCount + 1 }, false)
      else
        ({ s with msgCount := s.msgCount + 1 }, true)

/-- Connect two endpoints. -/
def connect (s : PortalState) (a b : Imscription) (m : PortalMode) : PortalState :=
  { s with endpointA_type := a, endpointB_type := b, mode := m, isConnected := true }

theorem transmit_meet_succeeds (s : PortalState) (h : s.isConnected) (hm : s.mode = .meet) :
    (transmit s).2 = true := by
  unfold transmit; simp [h, hm]

theorem tensor_destruct_on_phi3 (a b : Imscription) (h : containsPhi3 (portalTensor a b)) :
    (transmit { initialPortalState with
                endpointA_type := a
                endpointB_type := b
                mode := .tensor
                isConnected := true }).2 = false := by
  unfold transmit
  simp [h]

end Imscribing.Paraconsistent.Portal
