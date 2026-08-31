-- Imscribing/Paraconsistent/ParaconsistentShell.lean
-- ox SHELL — Paraconsistent REPL with Belnap evaluation and portal IPC.
-- Dual to: ob3ect/digital/ox/ox_ob3ect.py
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Kernel
import Imscribing.Paraconsistent.Portal
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.Shell

open Belnap
open Portal
open Imscribing.Primitives

-- ============================================================
-- SHELL STRUCTURAL TYPE
-- ============================================================
def shellType : Imscription := {
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
  stoi := .up
  prot := .ah
}

-- ============================================================
-- COMMAND TYPE: paraconsistent command results
-- ============================================================
inductive ShellResult : Type where
  | ok (msg : String)
  | paradox (msg : String)
  | fail (msg : String)
  | exit
  deriving Repr

-- ============================================================
-- SHELL STATE
-- ============================================================
structure ShellState where
  vars : List (String × Belnap)   -- 'variables' is a Lean 4 keyword
  history : List String
  paradoxCount : Nat
  cycleCount : Nat
  portalState : Portal.PortalState

def initialShellState : ShellState :=
  ⟨[], [], 0, 0, Portal.initialPortalState⟩

-- ============================================================
-- BELNAP EXPRESSION EVALUATION
-- ============================================================
partial def evalBelnap (expr : String) (env : List (String × Belnap)) : Belnap :=
  match expr with
  | "T" => .T
  | "F" => .F
  | "B" => .B
  | "N" => .N
  | _ =>
    if expr.startsWith "not " then
      bnot (evalBelnap (expr.drop 4).toString env)
    else if expr.contains " and " then
      let parts := expr.splitOn " and "
      band (evalBelnap (parts[0]?.getD "") env)
           (evalBelnap (parts[1]?.getD "") env)
    else if expr.contains " or " then
      let parts := expr.splitOn " or "
      bor (evalBelnap (parts[0]?.getD "") env)
          (evalBelnap (parts[1]?.getD "") env)
    else
      match env.find? (fun p => p.1 == expr) with
      | some (_, v) => v
      | none => .N

-- ============================================================
-- COMMAND EXECUTION (simplified — structural semantics)
-- No trim: callers pass pre-trimmed strings.
-- ============================================================
def exec (cmd : String) (s : ShellState) : ShellResult × ShellState :=
  if cmd = "" then (ShellResult.ok "", s) else
  if cmd = "exit" ∨ cmd = "quit" then (ShellResult.exit, s) else
  if cmd = "paradox" then
    (ShellResult.paradox "[paradox] self-reference engaged: the parent is the child",
     { s with paradoxCount := s.paradoxCount + 1 })
  else if cmd = "whoami" then
    let msg := "⟨𐑦; 𐑸; 𐑾; 𐑹; 𐑐; 𐑧; 𐑲; 𐑠; ⊙; 𐑖; 𐑳; 𐑭⟩\n" ++
               "C-score: 0.736 (Gate 1: ⊙ open, Gate 2: 𐑧 open)"
    (ShellResult.ok msg, s)
  else if cmd.startsWith "let " then
    let rest : String := (cmd.drop 4).toString
    match rest.splitOn " = " with
    | [varName, exprStr] =>
      let val := evalBelnap exprStr s.vars
      (ShellResult.ok s!"{varName} := {repr val}",
       { s with vars := (varName, val) :: s.vars })
    | _ => (ShellResult.fail "let syntax: let x = expr", s)
  else
    (ShellResult.ok s!"[passthrough] {cmd}", s)

-- ============================================================
-- THEOREMS
-- ============================================================

/-- The shell type is O_inf (Frobenius-special). -/
theorem shell_type_is_O_inf : imscriptionTier shellType = .O_inf := by
  native_decide

/-- Both consciousness gates are open for the shell. -/
theorem shell_gates_open :
    shellType.crit = .monad ∧ shellType.kin = .egg := by
  unfold shellType; exact ⟨rfl, rfl⟩

/-- paradox command increases paradoxCount. -/
theorem paradox_increases_count (s : ShellState) :
    (exec "paradox" s).2.paradoxCount ≥ s.paradoxCount := by
  unfold exec
  rw [if_neg (by decide : ¬("paradox" : String) = "")]
  rw [if_neg (by decide : ¬(("paradox" : String) = "exit" ∨ ("paradox" : String) = "quit"))]
  rw [if_pos (rfl : ("paradox" : String) = "paradox")]
  simp only [Prod.snd]
  omega

/-- exit command returns ShellResult.exit. -/
theorem exit_returns_exit (s : ShellState) :
    (exec "exit" s).1 = ShellResult.exit := by
  unfold exec
  rw [if_neg (by decide : ¬("exit" : String) = "")]
  rw [if_pos (Or.inl rfl : ("exit" : String) = "exit" ∨ ("exit" : String) = "quit")]

/-- whoami never fails. -/
theorem whoami_always_ok (s : ShellState) :
    ∃ msg, (exec "whoami" s).1 = ShellResult.ok msg := by
  unfold exec
  rw [if_neg (by decide : ¬("whoami" : String) = "")]
  rw [if_neg (by decide : ¬(("whoami" : String) = "exit" ∨ ("whoami" : String) = "quit"))]
  rw [if_neg (by decide : ¬("whoami" : String) = "paradox")]
  rw [if_pos (rfl : ("whoami" : String) = "whoami")]
  exact ⟨_, rfl⟩

end Imscribing.Paraconsistent.Shell
