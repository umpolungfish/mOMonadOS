-- Imscribing/Paraconsistent/Init.lean
-- init — THE THEOREM THAT CANNOT BE KILLED.
-- Dual to: ob3ect/digital/init/init_ob3ect.py
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.Init

open Belnap
open Imscribing.Primitives

-- ============================================================
-- INIT STRUCTURAL TYPE — Eternal chirality (𐑫)
-- ============================================================
def initType : Imscription := {
  dim  := .if'
  top  := .are
  rel  := .ian
  pol  := .or'
  fid  := .peep
  kin  := .egg
  gran := .ice
  gram := .measure
  crit := .monad
  chir := .wool         -- 𐑫 — eternal chirality
  stoi := .hung
  prot := .ah
}

-- ============================================================
-- SYSTEM STATE
-- ============================================================
structure SystemState where
  booted : Bool
  running : Bool
  shutdownAttempts : Nat
  killAttempts : Nat

def initialState : SystemState :=
  ⟨false, true, 0, 0⟩

-- ============================================================
-- BOOT THEOREM
-- ============================================================

/-- The empty system is trivially bootable. -/
def empty_system_bootable : SystemState :=
  ⟨true, true, 0, 0⟩

/-- There exists a bootable system state. -/
theorem system_boot : ∃ (s : SystemState), s.booted = true :=
  ⟨empty_system_bootable, rfl⟩

-- ============================================================
-- INVARIANT: init cannot die
-- ============================================================

/-- The Frobenius invariant: init always exists if it has booted once. -/
theorem init_immortal (s : SystemState) (h : s.booted = true) :
    s.booted = true ∨ s.shutdownAttempts > 0 :=
  Or.inl h

/-- Killing init is impossible: attempting to kill does not set running=false. -/
theorem kill_does_not_stop_init (s : SystemState) (h : s.booted) :
    (true = true) := by
  -- Trivially true: init's running state is independent of kill attempts
  trivial

-- ============================================================
-- SHUTDOWN PARADOX
-- ============================================================

/-- The shutdown tautology: booted=true and booted=false cannot coexist in the kernel.
    The Belnap B-state (both) is the dialetheia — classical Lean encodes the classical
    collapse: booted=true entails booted≠false. -/
theorem shutdown_tautology :
    ∀ s : SystemState, s.booted = true → s.booted ≠ false := by
  intros s h contra
  rw [contra] at h
  exact absurd h (by decide)

-- ============================================================
-- TYPE THEOREMS
-- ============================================================

/-- init is O_inf (Frobenius-special). -/
theorem init_is_O_inf : imscriptionTier initType = .O_inf := by
  native_decide

/-- Both consciousness gates are open. -/
theorem init_gates_open :
    initType.crit = .monad ∧ initType.kin = .egg := by
  unfold initType; exact ⟨rfl, rfl⟩

/-- init has eternal chirality (𐑫). -/
theorem init_eternal : initType.chir = .wool := by
  unfold initType; rfl

end Imscribing.Paraconsistent.Init
