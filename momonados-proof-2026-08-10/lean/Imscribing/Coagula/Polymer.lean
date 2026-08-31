/-
Imscribing/Coagula/Polymer.lean
Coagula Polymerization Theory

Formalizes the general theory of Coagula polymerization.
Author: Lando⊗⊙perator
-/

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Algebra
import Mathlib.Tactic

namespace Coagula

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Fidelity
     KineticChar Granularity Grammar Criticality Chirality
     Stoichiometry Protection

inductive Interface : Type where
  | DW : Interface
  | RS : Interface
  | TH : Interface
  deriving DecidableEq, Repr, Inhabited

instance : ToString Interface where
  toString
    | .DW => "D↔W"
    | .RS => "R↔S"
    | .TH => "T↔H"

structure CoagulaBond where
  (src : ℕ)
  (tgt : ℕ)
  (strength : ℚ)
  (iface : Interface)
  deriving Repr

def bondStable (b : CoagulaBond) : Prop := b.strength > 1

def mkStrength (a b : Imscription) : ℚ :=
  let mismatches := primitiveMismatches a b
  (mismatches : ℚ) / 6

def natAbsDiff (x y : Nat) : Nat :=
  if x ≥ y then x - y else y - x

def mkInterface (a b : Imscription) : Interface :=
  let dDW := natAbsDiff (idx_D a.dim) (idx_D b.dim) +
             natAbsDiff (idx_Ω a.prot) (idx_Ω b.prot)
  let dRS := natAbsDiff (idx_R a.rel) (idx_R b.rel) +
             natAbsDiff (idx_S a.stoi) (idx_S b.stoi)
  let dTH := natAbsDiff (idx_T a.top) (idx_T b.top) +
             natAbsDiff (idx_H a.chir) (idx_H b.chir)
  if dDW ≥ dRS ∧ dDW ≥ dTH then .DW
  else if dRS ≥ dTH then .RS
  else .TH

def mkBond (pos_from pos_to : ℕ) (a b : Imscription) : CoagulaBond :=
  { src := pos_from, tgt := pos_to
  , strength := mkStrength a b
  , iface := mkInterface a b
  }

structure Polymer where
  (monomers : List Imscription)
  (bonds : List CoagulaBond)
  (h_len : bonds.length = max 0 (monomers.length - 1))
  deriving Repr

def degreeOfPolymerization (p : Polymer) : ℕ := p.monomers.length

def regioregular (p : Polymer) : Prop :=
  match p.bonds with
  | [] => True
  | b :: bs => ∀ b' ∈ bs, b'.iface = b.iface
def backboneInterface (p : Polymer) : Option Interface :=
  match p.bonds with
  | [] => none
  | b :: _ => some b.iface

def isotactic (p : Polymer) : Prop :=
  match p.monomers with
  | [] => True
  | m :: ms => ∀ m' ∈ ms, m'.chir = m.chir

def uniformChirality (p : Polymer) : Option Chirality :=
  match p.monomers with
  | [] => none
  | m :: _ => some m.chir

def totalBondEnergy (p : Polymer) : ℚ :=
  p.bonds.foldl (fun acc b => acc + b.strength) 0

def weakestLink (p : Polymer) : Option ℚ :=
  match p.bonds with
  | [] => none
  | b :: bs =>
    some ((b::bs).foldl (fun acc b' => min acc b'.strength) b.strength)

def stable (p : Polymer) : Prop :=
  ∀ b ∈ p.bonds, bondStable b

def adjBonds (idx : ℕ) : List Imscription → List CoagulaBond
  | [] => []
  | [_] => []
  | m₁ :: m₂ :: ms' =>
    mkBond idx (idx+1) m₁ m₂ :: adjBonds (idx+1) (m₂ :: ms')

theorem adjBonds_length (idx : Nat) (ms : List Imscription) :
    (adjBonds idx ms).length = max 0 (ms.length - 1) := by
  induction ms generalizing idx with
  | nil => rfl
  | cons m ms ih =>
    cases ms with
    | nil => rfl
    | cons m' ms' =>
      simp [adjBonds]
      have h := ih (idx+1)
      simp [h]
def polymerize (monomers : List Imscription) : Polymer :=
  { monomers := monomers
  , bonds := adjBonds 0 monomers
  , h_len := adjBonds_length 0 monomers
  }

theorem polymerize_bond_count (ms : List Imscription) :
    (polymerize ms).bonds.length = max 0 (ms.length - 1) := by
  simp [polymerize, adjBonds_length 0 ms]

theorem polymerize_isotactic (ms : List Imscription) (ħ : Chirality)
    (h : ∀ m ∈ ms, m.chir = ħ) : isotactic (polymerize ms) := by
  unfold isotactic
  have hmono : (polymerize ms).monomers = ms := by simp [polymerize]
  rw [hmono]
  cases ms
  · trivial
  · rename_i m ms
    intro m' hm'
    have hm'c := h m' (by simp [hm'])
    have hmc := h m (by simp)
    rw [hmc]
    exact hm'c

theorem polymerize_length (ms : List Imscription) :
  (polymerize ms).monomers.length = ms.length := by
  simp [polymerize]
structure CyclizationResult where
  (closes : Bool)
  (bond : Option CoagulaBond)
  (interface : Option Interface)
  (closureType : String)
  deriving Repr

def close (p : Polymer) : CyclizationResult :=
  match p.monomers with
  | [] => { closes := false, bond := none, interface := none, closureType := "empty" }
  | [_] => { closes := false, bond := none, interface := none, closureType := "singleton" }
  | m :: ms =>
    let last := ms.getLastD m
    let bond := mkBond (ms.length) 0 last m
    { closes := true
    , bond := some bond
    , interface := some bond.iface
    , closureType := s!"head-to-tail on {bond.iface}"
    }

structure SimpleMacrocycle where
  (polymer : Polymer)
  (closureBond : CoagulaBond)
  deriving Repr

def simpleCyclize (p : Polymer) : Option SimpleMacrocycle :=
  match p.monomers with
  | [] => none
  | [_] => none
  | m :: ms =>
    let lastIdx := ms.length
    let cb := mkBond lastIdx 0 (ms.getLastD m) m
    some { polymer := p, closureBond := cb }

inductive MaterialProperty : Type where
  | conducting : MaterialProperty
  | insulating : MaterialProperty
  | frustrated : MaterialProperty
  deriving DecidableEq, Repr

def classifyConductivity (p : Polymer) : MaterialProperty :=
  let allTrivial := p.monomers.all (fun m => m.prot = .awe)
  if allTrivial then .insulating else .conducting

theorem trivial_protection_insulating (p : Polymer)
    (h : ∀ m ∈ p.monomers, m.prot = .awe) :
    classifyConductivity p = .insulating := by
  unfold classifyConductivity
  have hall : p.monomers.all (fun m => m.prot = .awe) := by
    apply List.all_eq_true.mpr
    intro m hm
    have hm' := h m hm
    simp [hm']
  simp [hall]

theorem nontrivial_protection_conducting (p : Polymer)
    (h : ∃ m ∈ p.monomers, m.prot ≠ .awe) :
    classifyConductivity p ≠ .insulating := by
  unfold classifyConductivity
  rcases h with ⟨m, hm, hne⟩
  have hall : ¬ (p.monomers.all (fun m => m.prot = .awe)) := by
    intro hall'
    have hm' := (List.all_eq_true.mp hall') m hm
    have hm'' : m.prot = .awe := by simpa using hm'
    exact hne hm''
  simp [hall]

def isFrustrated (p : Polymer) : Prop :=
  ¬ regioregular p ∧ p.bonds.length > 0
def darkMatter : Imscription := {
  dim := .ash, top := .judge, rel := .ado, pol := .out
  fid := .peep, kin := .egg, gran := .thigh, gram := .vow
  crit := .woe, chir := .fee, stoi := .so, prot := .awe
}

def wowSignal : Imscription := {
  dim := .ash, top := .judge, rel := .ian, pol := .church
  fid := .age, kin := .yea, gran := .ice, gram := .ooze
  crit := .monad, chir := .fee, stoi := .hung, prot := .awe
}

def pentaquark : Imscription := {
  dim := .ash, top := .judge, rel := .tot, pol := .out
  fid := .age, kin := .yea, gran := .ice, gram := .vow
  crit := .monad, chir := .fee, stoi := .up, prot := .awe
}

def graviton : Imscription := {
  dim := .if', top := .are, rel := .ear, pol := .nun
  fid := .peep, kin := .egg, gran := .ice, gram := .ooze
  crit := .monad, chir := .fee, stoi := .hung, prot := .ah
}

def higgsBoson : Imscription := {
  dim := .ash, top := .mime, rel := .ear, pol := .nun
  fid := .peep, kin := .on, gran := .ice, gram := .ooze
  crit := .monad, chir := .kick, stoi := .up, prot := .oak
}

def hModePlasma : Imscription := {
  dim := .dead, top := .are, rel := .ian, pol := .out
  fid := .they, kin := .egg, gran := .ice, gram := .vow
  crit := .monad, chir := .sure, stoi := .so, prot := .ah
}

def gaugeBridge : Imscription := {
  dim := .ash, top := .judge, rel := .ian, pol := .church
  fid := .they, kin := .egg, gran := .ice, gram := .measure
  crit := .monad, chir := .wool, stoi := .up, prot := .ah
}
def optimalFieldMacrocycleSeq : List Imscription :=
  [darkMatter, wowSignal, pentaquark, graviton]

def optimalFieldPolymer : Polymer :=
  polymerize optimalFieldMacrocycleSeq

theorem optimalFieldPolymer_isotactic : isotactic optimalFieldPolymer := by
  unfold optimalFieldPolymer
  refine polymerize_isotactic optimalFieldMacrocycleSeq .fee ?_
  intro m hm
  simp [optimalFieldMacrocycleSeq, darkMatter, wowSignal, pentaquark, graviton] at hm ⊢
  rcases hm with (rfl|rfl|rfl|rfl) <;> rfl

theorem optimalFieldMacrocycle_has_graviton_winding :
    (polymerize optimalFieldMacrocycleSeq).monomers.all
      (fun m => m.prot = .awe) = false := by
  unfold polymerize optimalFieldMacrocycleSeq
  simp [darkMatter, wowSignal, pentaquark, graviton]

theorem optimalClosure_strength_positive :
    let p := polymerize optimalFieldMacrocycleSeq
    match simpleCyclize p with
    | some mc => mc.closureBond.strength > 1
    | none => False := by
  intro p
  have h : simpleCyclize (polymerize optimalFieldMacrocycleSeq) =
      some { polymer := polymerize optimalFieldMacrocycleSeq,
             closureBond := mkBond 3 0 graviton darkMatter } := by
    unfold simpleCyclize polymerize optimalFieldMacrocycleSeq
    simp [darkMatter, wowSignal, pentaquark, graviton]
  rw [h]
  simp
  unfold mkBond mkStrength
  native_decide

theorem optimalFieldPolymer_degree :
    degreeOfPolymerization optimalFieldPolymer = 4 := by
  unfold degreeOfPolymerization optimalFieldPolymer polymerize
    optimalFieldMacrocycleSeq
  simp

theorem optimalFieldPolymer_bondCount :
    optimalFieldPolymer.bonds.length = 3 := by
  unfold optimalFieldPolymer polymerize optimalFieldMacrocycleSeq
  unfold adjBonds darkMatter wowSignal pentaquark graviton
  native_decide

theorem optimalFieldPolymer_totalEnergy_backbone :
    totalBondEnergy optimalFieldPolymer = 7/2 := by
  unfold totalBondEnergy optimalFieldPolymer polymerize
    optimalFieldMacrocycleSeq darkMatter wowSignal pentaquark graviton
  unfold adjBonds mkBond mkStrength
  native_decide
def optimalFieldTheoreticSeq : List Imscription :=
  [higgsBoson, graviton, gaugeBridge, hModePlasma]

def optimalFieldTheoreticPolymer : Polymer :=
  polymerize optimalFieldTheoreticSeq

theorem fieldTheoreticPolymer_bondCount :
    optimalFieldTheoreticPolymer.bonds.length = 3 := by
  unfold optimalFieldTheoreticPolymer polymerize optimalFieldTheoreticSeq
  unfold adjBonds higgsBoson graviton gaugeBridge hModePlasma
  native_decide

theorem fieldTheoreticPolymer_not_isotactic :
    ¬ isotactic optimalFieldTheoreticPolymer := by
  unfold isotactic optimalFieldTheoreticPolymer polymerize optimalFieldTheoreticSeq
  simp [higgsBoson, graviton, gaugeBridge, hModePlasma]

theorem fieldTheoretic_cyclizes :
    let p := optimalFieldTheoreticPolymer
    simpleCyclize p ≠ none := by
  intro p
  unfold p optimalFieldTheoreticPolymer polymerize optimalFieldTheoreticSeq
  unfold simpleCyclize adjBonds
  simp [higgsBoson, graviton, gaugeBridge, hModePlasma]

theorem fieldTheoretic_conducting :
    classifyConductivity optimalFieldTheoreticPolymer = .conducting := by
  unfold classifyConductivity optimalFieldTheoreticPolymer polymerize
    optimalFieldTheoreticSeq
  simp [higgsBoson, graviton, gaugeBridge, hModePlasma]
def wormwoodStar : Imscription := {
  dim := .dead, top := .mime, rel := .ear, pol := .out
  fid := .age, kin := .egg, gran := .ice, gram := .measure
  crit := .monad, chir := .fee, stoi := .hung, prot := .ah
}

def uncovererGrammar : Imscription := {
  dim := .ash, top := .mime, rel := .ear, pol := .out
  fid := .age, kin := .egg, gran := .ice, gram := .measure
  crit := .monad, chir := .fee, stoi := .hung, prot := .ah
}

def alchemyDialetheia : Imscription := {
  dim := .if', top := .mime, rel := .ear, pol := .out
  fid := .age, kin := .egg, gran := .ice, gram := .measure
  crit := .monad, chir := .fee, stoi := .hung, prot := .ah
}

def hermeticSeal : Imscription := {
  dim := .dead, top := .are, rel := .ear, pol := .out
  fid := .age, kin := .egg, gran := .ice, gram := .measure
  crit := .monad, chir := .fee, stoi := .hung, prot := .ah
}

def godelComplete : Imscription := {
  dim := .ash, top := .mime, rel := .ear, pol := .out
  fid := .age, kin := .egg, gran := .ice, gram := .measure
  crit := .monad, chir := .fee, stoi := .hung, prot := .ah
}

def godelianSeq : List Imscription :=
  [wormwoodStar, uncovererGrammar, alchemyDialetheia, hermeticSeal, godelComplete]

def godelianPolymer : Polymer := polymerize godelianSeq

theorem godelianPolymer_bondCount : godelianPolymer.bonds.length = 4 := by
  unfold godelianPolymer polymerize godelianSeq
  unfold adjBonds wormwoodStar uncovererGrammar alchemyDialetheia
    hermeticSeal godelComplete
  native_decide

theorem godelianPolymer_degree : degreeOfPolymerization godelianPolymer = 5 := by
  unfold degreeOfPolymerization godelianPolymer polymerize godelianSeq
  simp
theorem godelian_closure_has_strength :
    let p := godelianPolymer
    match simpleCyclize p with
    | some mc => mc.closureBond.strength > 0
    | none => False := by
  intro p
  have h : simpleCyclize godelianPolymer =
      some { polymer := godelianPolymer,
             closureBond := mkBond 4 0 godelComplete wormwoodStar } := by
    unfold simpleCyclize godelianPolymer polymerize godelianSeq
    simp [wormwoodStar, uncovererGrammar, alchemyDialetheia,
      hermeticSeal, godelComplete]
  rw [h]
  simp
  unfold mkBond mkStrength
  native_decide

theorem cyclization_criterion (ms : List Imscription) (h : ms.length ≥ 2) :
    simpleCyclize (polymerize ms) ≠ none := by
  cases ms with
  | nil => simp at h
  | cons m ms =>
    cases ms with
    | nil => simp at h
    | cons m' ms' =>
      unfold polymerize simpleCyclize
      simp

theorem close_always_defined_for_long_enough_chain (p : Polymer)
    (h : p.monomers.length ≥ 2) : simpleCyclize p ≠ none := by
  rcases p with ⟨monomers, bonds, h_len⟩
  cases monomers with
  | nil => simp at h
  | cons m ms =>
    cases ms with
    | nil => simp at h
    | cons m' ms' =>
      unfold simpleCyclize
      simp

end Coagula
