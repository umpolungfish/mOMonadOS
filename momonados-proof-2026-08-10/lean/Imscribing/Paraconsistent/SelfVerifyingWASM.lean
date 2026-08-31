-- Imscribing/Paraconsistent/SelfVerifyingWASM.lean
-- SELF-VERIFYING WASM RUNTIME -- Frobenius-wrapped WebAssembly execution.
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

open Imscribing.Primitives

namespace Imscribing.Paraconsistent.SelfVerifyingWASM

open Belnap

inductive WasmType : Type where | i32 | i64
  deriving DecidableEq, Repr

structure WasmValue where
  ty : WasmType
  val : Nat
  deriving DecidableEq, Repr

inductive WasmInstr : Type where
  | i32_const (n : Nat)
  | drop | nop | unreachable
  | verify | checkpoint | assert_invariant
  deriving Repr

structure TaggedValue where
  value : WasmValue
  tag : Belnap
  deriving DecidableEq, Repr

abbrev Stack : Type := List TaggedValue

structure WasmState where
  stack : Stack
  ip : Nat
  frobSnapshot : Stack
  frobInvariantHolds : Belnap
  verifiedSteps : Nat
  totalSteps : Nat

def initialWasmState : WasmState :=
  ⟨[], 0, [], .N, 0, 0⟩

def pushTagged (s : Stack) (tv : TaggedValue) : Stack := tv :: s

def frobTagBin (t1 t2 : Belnap) : Belnap := meet t1 t2

def execOne (s : WasmState) (instr : WasmInstr) : WasmState :=
  match instr with
  | .i32_const n =>
    ⟨pushTagged s.stack ⟨⟨.i32, n⟩, .T⟩, s.ip + 1,
     s.frobSnapshot, s.frobInvariantHolds,
     s.verifiedSteps, s.totalSteps + 1⟩
  | .drop =>
    match s.stack with
    | [] => ⟨[], s.ip + 1, s.frobSnapshot, s.frobInvariantHolds,
             s.verifiedSteps, s.totalSteps + 1⟩
    | _ :: rest => ⟨rest, s.ip + 1, s.frobSnapshot, s.frobInvariantHolds,
                    s.verifiedSteps, s.totalSteps + 1⟩
  | .nop =>
    ⟨s.stack, s.ip + 1, s.frobSnapshot, s.frobInvariantHolds,
     s.verifiedSteps, s.totalSteps + 1⟩
  | .unreachable =>
    ⟨s.stack, s.ip + 1, s.frobSnapshot, .F,
     s.verifiedSteps, s.totalSteps + 1⟩
  | .checkpoint =>
    ⟨s.stack, s.ip + 1, s.stack, s.frobInvariantHolds,
     s.verifiedSteps, s.totalSteps + 1⟩
  | .verify =>
    let allTrusted := s.stack.all (fun tv => tv.tag == .T || tv.tag == .B)
    ⟨s.stack, s.ip + 1, s.frobSnapshot,
     if allTrusted then .B else .F,
     if allTrusted then s.verifiedSteps + 1 else s.verifiedSteps,
     s.totalSteps + 1⟩
  | .assert_invariant =>
    ⟨s.stack, s.ip + 1, s.frobSnapshot,
     frobTagBin s.frobInvariantHolds
       (if s.frobInvariantHolds == .F then .F else .B),
     s.verifiedSteps, s.totalSteps + 1⟩

theorem frob_binop_preserves_B : frobTagBin .B .B = .B := by
  unfold frobTagBin; rfl

theorem frobenius_mu_delta_id_tag (t : Belnap) :
    frobTagBin t .B = t := by
  unfold frobTagBin; cases t <;> rfl

theorem unreachable_sets_F (s : WasmState) :
    (execOne s .unreachable).frobInvariantHolds = .F := by
  unfold execOne; rfl

theorem frobenius_identity_direct (n : Nat) :
    (let s1 := execOne initialWasmState (.i32_const n)
     let s2 := execOne s1 .verify
     s2.frobInvariantHolds = .B) := by
  unfold execOne pushTagged initialWasmState
  simp

def wasmRuntimeTuple : Imscription := {
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
  stoi := .so
  prot := .ah
}

theorem wasm_runtime_is_O_inf :
    imscriptionTier wasmRuntimeTuple = .O_inf := by
  native_decide

end Imscribing.Paraconsistent.SelfVerifyingWASM
