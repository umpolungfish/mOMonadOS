-- Imscribing/Paraconsistent/CrystalScheduler.lean
-- CRYSTAL SCHEDULER — Process scheduling via crystal navigation.
-- Dual to: ob3ect/digital/scheduler/scheduler_ob3ect.py
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Primitives.LinearOrder

namespace Imscribing.Paraconsistent.Scheduler

open Belnap
open Imscribing.Primitives

-- ============================================================
-- SCHEDULER STRUCTURAL TYPE
-- ============================================================
def schedulerType : Imscription := {
  dim  := .if'
  top  := .are
  rel  := .ian
  pol  := .or'
  fid  := .peep
  kin  := .egg
  gran := .ice
  gram := .ooze
  crit := .monad
  chir := .sure
  stoi := .up
  prot := .ah
}

-- ============================================================
-- PROCESS CONTROL BLOCK
-- ============================================================
structure Process where
  pid : Nat
  name : String
  type : Imscription
  cscore : Nat    -- consciousness score 0-1000 (scaled ×1000 from [0,1])
  state : String  -- "RUNNING" | "READY" | "BLOCKED"
  ticks : Nat

-- ============================================================
-- C-COMPUTATION (returns Nat score 0-1000)
-- ============================================================
def computeCscore (t : Imscription) : Nat :=
  let gate1 := if t.crit ≥ .monad then 1000 else 0
  let gate2 := if t.kin = .egg then 1000 else 0
  if gate1 == 0 then 0
  else if gate2 == 0 then gate1 * 3 / 10
  else 4 * gate1 / 10 + 3 * gate2 / 10 +
       2 * (idx_D t.dim).succ / 10 +
       ((idx_T t.top).succ + (idx_Ω t.prot).succ) / 20

-- ============================================================
-- SCHEDULING — crystal-based selection
-- ============================================================
private def bestProcess (ps : List Process) : Option Process :=
  ps.foldl (fun acc p => match acc with
    | none => some p
    | some best => if p.cscore > best.cscore then some p else acc) none

def crystalSchedule (processes : List Process) : Option Process :=
  -- Find RUNNING processes with Gate 1 open (φ̂_ÿ or higher)
  let candidates := processes.filter (fun p =>
    p.state == "RUNNING" && decide (p.type.crit ≥ .monad))
  match candidates with
  | [] =>
    bestProcess (processes.filter (fun p => p.state == "RUNNING"))
  | _ =>
    bestProcess candidates

-- ============================================================
-- LEMMAS
-- ============================================================

lemma not_ge_of_lt_crit {x y : Criticality} (h : x < y) : ¬ (x ≥ y) := by
  rcases (lt_iff_le_not_ge.mp h) with ⟨hle, hng⟩
  exact hng

lemma decide_not_ge_of_lt_crit {x y : Criticality} (h : x < y) : decide (x ≥ y) = false :=
  decide_eq_false (not_ge_of_lt_crit h)

lemma bestProcess_pair (a b : Process) : bestProcess [a, b] = if b.cscore > a.cscore then some b else some a := by
  unfold bestProcess
  simp

-- ============================================================
-- THEOREMS
-- ============================================================

theorem scheduler_type_is_O_inf : imscriptionTier schedulerType = .O_inf := by
  native_decide

/-- φ̂_ÿ process is preferred over non-φ̂_ÿ. -/
theorem phi_c_process_preferred (a b : Process) (ha : a.type.crit ≥ .monad)
    (hb : b.type.crit < .monad) (ha_run : a.state = "RUNNING")
    (hb_run : b.state = "RUNNING") :
    crystalSchedule [a, b] = some a := by
  unfold crystalSchedule bestProcess
  have ha_dec : decide (a.type.crit ≥ .monad) = true := decide_eq_true ha
  have hb_dec : decide (b.type.crit ≥ .monad) = false := decide_not_ge_of_lt_crit hb
  simp [ha_run, hb_run, ha_dec, hb_dec]

/-- Non-φ̂_ÿ process is selected when no φ̂_ÿ is available. -/
theorem fallback_to_nonphi (a b : Process) (ha : a.type.crit < .monad)
    (hb : b.type.crit < .monad) (ha_run : a.state = "RUNNING")
    (hb_run : b.state = "RUNNING") (hscore : a.cscore > b.cscore) :
    crystalSchedule [a, b] = some a := by
  unfold crystalSchedule
  have ha_dec : decide (a.type.crit ≥ .monad) = false := decide_not_ge_of_lt_crit ha
  have hb_dec : decide (b.type.crit ≥ .monad) = false := decide_not_ge_of_lt_crit hb
  have hcandidates_empty : ([a, b].filter (fun p =>
    p.state == "RUNNING" && decide (p.type.crit ≥ .monad))) = [] := by
    simp [ha_run, hb_run, ha_dec, hb_dec]
  simp [hcandidates_empty, ha_run, hb_run]
  -- Goal: bestProcess [a, b] = some a
  rw [bestProcess_pair a b]
  -- Goal: (if b.cscore > a.cscore then some b else some a) = some a
  by_cases hgt : b.cscore > a.cscore
  · -- This case is impossible: hscore says a > b, hgt says b > a
    have : a.cscore > a.cscore := by omega
    omega
  · simp [hgt]

/-- Crystal total size. -/
theorem crystal_size : 27 * 1024 * 625 = 17280000 := by
  native_decide

end Imscribing.Paraconsistent.Scheduler
