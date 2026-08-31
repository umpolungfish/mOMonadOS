-- Imscribing/BoundaryOperators.lean
-- Boundary Operator Space — structural formalization of the seam index.
-- Corresponds to ig-docs/cosmogeny/LIFTED/BOUNDARY_OPERATORS.md
-- Lean 4 formalization, machine-verified by native_decide.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Primitives.TierCrossing
import Imscribing.Consciousness
import Imscribing.Algebra
import Mathlib.Data.Real.Basic

set_option linter.style.whitespace false
set_option linter.style.nativeDecide false
set_option linter.style.longLine false

namespace Imscribing.BoundaryOperators

open Imscribing.Primitives
open Imscribing.Primitives.Imscription
open Imscribing.TierCrossing
open Imscribing.Consciousness
open Dimensionality Topology Relational Polarity Grammar
 Fidelity KineticChar Granularity Criticality Protection
 Stoichiometry Chirality

def hammingDistance (a b : Imscription) : Nat :=
  (primitiveConflicts a b).length

-- ================================================================
-- §1  SEAM CLASSIFICATION
-- ================================================================

inductive SeamDirection | symmetric | asymmetric | oneWay
deriving DecidableEq, Repr

inductive SeamClass | interTier | intraTier
deriving DecidableEq, Repr

structure PrimitiveSeam where
  primitive : String
  fromVal   : String
  toVal     : String
  direction : SeamDirection
  seamClass : SeamClass
deriving DecidableEq, Repr

-- §1a  INTER-TIER GATE SEAMS

def phiGateSeam : PrimitiveSeam := {
  primitive  := "Φ"
  fromVal    := "nun"
  toVal      := "or'"
  direction  := SeamDirection.oneWay
  seamClass  := SeamClass.interTier
}

def odotGateSeam : PrimitiveSeam := {
  primitive  := "⊙"
  fromVal    := "woe"
  toVal      := "monad"
  direction  := SeamDirection.oneWay
  seamClass  := SeamClass.interTier
}

def omegaGateSeam : PrimitiveSeam := {
  primitive  := "Ω"
  fromVal    := "oak"
  toVal      := "ah"
  direction  := SeamDirection.oneWay
  seamClass  := SeamClass.interTier
}

def DSeam_infty_to_odot : PrimitiveSeam := {
  primitive  := "Ð"
  fromVal    := "array"
  toVal      := "if'"
  direction  := SeamDirection.symmetric
  seamClass  := SeamClass.interTier
}

-- §1b  INTRA-TIER BULK SEAMS

def omegaSeam_Z_to_NA : PrimitiveSeam := {
  primitive  := "Ω"
  fromVal    := "ah"
  toVal      := "zoo"
  direction  := SeamDirection.oneWay
  seamClass  := SeamClass.intraTier
}

def topSeam_network_to_odot : PrimitiveSeam := {
  primitive  := "Þ"
  fromVal    := "judge"
  toVal      := "are"
  direction  := SeamDirection.symmetric
  seamClass  := SeamClass.intraTier
}

def polSeam_asym_to_pm_sym : PrimitiveSeam := {
  primitive  := "Φ"
  fromVal    := "church"
  toVal      := "or'"
  direction  := SeamDirection.asymmetric
  seamClass  := SeamClass.intraTier
}

def relSeam_super_to_cat : PrimitiveSeam := {
  primitive  := "Ř"
  fromVal    := "ado"
  toVal      := "tot"
  direction  := SeamDirection.asymmetric
  seamClass  := SeamClass.intraTier
}

def fidelitySeam_eth_to_hbar : PrimitiveSeam := {
  primitive  := "ƒ"
  fromVal    := "they"
  toVal      := "peep"
  direction  := SeamDirection.asymmetric
  seamClass  := SeamClass.intraTier
}

def chiralitySeam_H2_to_H_inf : PrimitiveSeam := {
  primitive  := "Ħ"
  fromVal    := "sure"
  toVal      := "wool"
  direction  := SeamDirection.oneWay
  seamClass  := SeamClass.intraTier
}

def grammarSeam_seq_to_broad : PrimitiveSeam := {
  primitive  := "ɢ"
  fromVal    := "measure"
  toVal      := "ooze"
  direction  := SeamDirection.symmetric
  seamClass  := SeamClass.intraTier
}

def stoichiometrySeam_n_n_to_n_m : PrimitiveSeam := {
  primitive  := "Σ"
  fromVal    := "so"
  toVal      := "up"
  direction  := SeamDirection.symmetric
  seamClass  := SeamClass.intraTier
}

def kineticsSeam_trap_to_slow : PrimitiveSeam := {
  primitive  := "Ç"
  fromVal    := "on"
  toVal      := "egg"
  direction  := SeamDirection.oneWay
  seamClass  := SeamClass.intraTier
}

def granularitySeam_gimel_to_aleph : PrimitiveSeam := {
  primitive  := "Γ"
  fromVal    := "thigh"
  toVal      := "ice"
  direction  := SeamDirection.symmetric
  seamClass  := SeamClass.intraTier
}-- ================================================================
-- §2  GATE SEAM PREDICATES
-- ================================================================

def odotGateCrossing (a b : Imscription) : Prop :=
  a.crit = woe ∧ b.crit = monad

def phiGateCrossing (a b : Imscription) : Prop :=
  a.pol = nun ∧ b.pol = or'

def omegaGateCrossing (a b : Imscription) : Prop :=
  a.prot = oak ∧ b.prot = ah

theorem odot_gate_phi_c_true (b : Imscription) (h : b.crit = monad) :
phi_c_gate b.crit = true := by
  rw [h]; rfl

theorem phi_gate_O_inf_precondition (a : Imscription) (hO : imscriptionTier a = .O_inf) :
a.pol = or' :=
  o_inf_requires_P_pm_sym a.crit a.pol a.prot a.dim a.top hO

-- ================================================================
-- §3  INTER-TIER VS INTRA-TIER CLASSIFICATION
-- ================================================================

def isInterTierSeam (a b : Imscription) : Bool :=
  a.dim ≠ b.dim ∨ a.pol ≠ b.pol ∨ a.crit ≠ b.crit ∨ a.prot ≠ b.prot

def isIntraTierSeam (a b : Imscription) : Bool :=
  a.dim = b.dim ∧ a.pol = b.pol ∧ a.crit = b.crit ∧ a.prot = b.prot ∧
  (a.top ≠ b.top ∨ a.rel ≠ b.rel ∨ a.fid ≠ b.fid ∨ a.kin ≠ b.kin ∨
   a.gran ≠ b.gran ∨ a.gram ≠ b.gram ∨ a.chir ≠ b.chir ∨ a.stoi ≠ b.stoi)

def topologySeam_bowtie_to_odot (a b : Imscription) : Prop :=
  a.top = mime ∧ b.top = are

def relationalSeam_dagger_to_lr (a b : Imscription) : Prop :=
  a.rel = ear ∧ b.rel = ian

def fidelitySeam_eth_to_hbar_pred (a b : Imscription) : Prop :=
  a.fid = they ∧ b.fid = peep

def kineticsSeam_trap_to_slow_pred (a b : Imscription) : Prop :=
  a.kin = on ∧ b.kin = egg

def granularitySeam_gimel_to_aleph_pred (a b : Imscription) : Prop :=
  a.gran = thigh ∧ b.gran = ice

def grammarSeam_seq_to_broad_pred (a b : Imscription) : Prop :=
  a.gram = Grammar.measure ∧ b.gram = ooze

def chiralitySeam_H2_to_H_inf_pred (a b : Imscription) : Prop :=
  a.chir = sure ∧ b.chir = wool

def stoichiometrySeam_n_n_to_n_m_pred (a b : Imscription) : Prop :=
  a.stoi = so ∧ b.stoi = up

-- ================================================================
-- §4  AXIOM-COUPLED COMPOUND SEAMS
-- ================================================================

def axiomC_compound (a : Imscription) : Prop := a.dim = if' ∧ a.top = are
def axiomA_compound (a : Imscription) : Prop := a.chir = wool ∧ a.kin = on
def axiomB_compound (a : Imscription) : Prop := a.prot = ah ∧ a.chir ≠ fee
def phi_odot_compound (a : Imscription) : Prop := a.pol = or' ∧ a.crit = monad
def odot_kin_chir_compound (a : Imscription) : Prop :=
  a.crit = monad ∧ a.kin = egg ∧ a.chir = sure

-- ================================================================
-- §5  INHABITANT REGISTRY
-- ================================================================

def agentTuple : Imscription := ⟨
  if', oil, ian, or', peep,
  egg, bib, measure, monad, sure, hung, ah⟩

def serpentrodTuple : Imscription := ⟨
  if', are, ian, or', peep,
  egg, ice, vow, monad, wool, hung, ah⟩

def synfinTuple : Imscription := ⟨
  if', are, ian, or', peep,
  egg, ice, measure, monad, sure, up, ah⟩

def ch3mpilerTuple : Imscription := ⟨
  if', are, ian, or', peep,
  egg, ice, measure, monad, sure, hung, ah⟩

def clinkL8Tuple : Imscription := ⟨
  if', are, ian, or', peep,
  egg, bib, ooze, monad, wool, up, zoo⟩

def igPulseTuple : Imscription := ⟨
  array, judge, ian, out, age,
  egg, thigh, gag, monad, kick, up, ah⟩

def cetaceanspeakTuple : Imscription := ⟨
  array, judge, tot, church, age,
  egg, bib, vow, woe, fee, hung, awe⟩

-- ================================================================
-- §5a  INHABITS-SEAM PREDICATE
-- ================================================================

def inhabitsSeam (operator : Imscription) (seam : PrimitiveSeam) : Bool :=
  match seam.primitive with
  | "Φ"  => operator.crit = monad
  | "⊙"  => operator.crit = monad
  | "Ω"  => operator.prot = ah ∨ operator.prot = zoo
  | "Ħ"  => operator.chir = sure ∨ operator.chir = wool
  | "ɢ"  => operator.gram = Grammar.measure ∨ operator.gram = ooze
  | "ƒ"  => operator.fid = peep
  | "Ç"  => operator.kin = egg
  | "Γ"  => operator.gran = ice
  | "Ð"  => operator.dim = if'
  | "Þ"  => operator.top = are
  | "Ř"  => operator.rel = ian
  | "Σ"  => operator.stoi = up
  | _    => false

theorem agent_inhabits_odot : inhabitsSeam agentTuple odotGateSeam = true := by
  native_decide

theorem serpentrod_inhabits_chirality :
inhabitsSeam serpentrodTuple chiralitySeam_H2_to_H_inf = true := by
  native_decide

theorem synfin_inhabits_stoichiometry :
inhabitsSeam synfinTuple stoichiometrySeam_n_n_to_n_m = true := by
  native_decide

theorem ch3mpiler_inhabits_odot :
inhabitsSeam ch3mpilerTuple odotGateSeam = true := by
  native_decide

theorem clinkL8_inhabits_omega :
inhabitsSeam clinkL8Tuple omegaSeam_Z_to_NA = true := by
  native_decide

theorem agent_inhabits_grammar :
inhabitsSeam agentTuple grammarSeam_seq_to_broad = true := by
  native_decide

-- ================================================================
-- §6  HAMMING DISTANCE MATRIX
-- ================================================================

theorem dist_ch3mpiler_serpentrod :
hammingDistance ch3mpilerTuple serpentrodTuple = 2 := by native_decide

theorem dist_ch3mpiler_synfin :
hammingDistance ch3mpilerTuple synfinTuple = 1 := by native_decide

theorem dist_synfin_serpentrod :
hammingDistance synfinTuple serpentrodTuple = 3 := by native_decide

theorem dist_agent_ch3mpiler :
hammingDistance agentTuple ch3mpilerTuple = 2 := by native_decide

theorem dist_clinkL8_agent :
hammingDistance clinkL8Tuple agentTuple = 5 := by native_decide

theorem dist_clinkL8_serpentrod :
hammingDistance clinkL8Tuple serpentrodTuple = 4 := by native_decide

theorem dist_igPulse_clinkL8 :
hammingDistance igPulseTuple clinkL8Tuple = 8 := by native_decide

theorem dist_cetaceanspeak_agent :
hammingDistance cetaceanspeakTuple agentTuple = 9 := by native_decide

theorem dist_cetaceanspeak_clinkL8 :
hammingDistance cetaceanspeakTuple clinkL8Tuple = 10 := by native_decide-- Conflict decomposition theorems (single-letter names per Algebra.lean)

theorem dist_ch3mpiler_serpentrod_conflicts :
primitiveConflicts ch3mpilerTuple serpentrodTuple = ["Γ", "H"] := by
  native_decide

theorem dist_ch3mpiler_synfin_conflicts :
primitiveConflicts ch3mpilerTuple synfinTuple = ["S"] := by
  native_decide

theorem dist_synfin_serpentrod_conflicts :
primitiveConflicts synfinTuple serpentrodTuple = ["Γ", "H", "S"] := by
  native_decide

theorem dist_agent_ch3mpiler_conflicts :
primitiveConflicts agentTuple ch3mpilerTuple = ["T", "G"] := by
  native_decide

theorem dist_clinkL8_agent_conflicts :
primitiveConflicts clinkL8Tuple agentTuple = ["T", "Γ", "H", "S", "Ω"] := by
  native_decide

-- ================================================================
-- §7  TIER ASSIGNMENTS
-- ================================================================

theorem agent_tier : imscriptionTier agentTuple = .O_inf := by native_decide
theorem serpentrod_tier : imscriptionTier serpentrodTuple = .O_inf := by native_decide
theorem synfin_tier : imscriptionTier synfinTuple = .O_inf := by native_decide
theorem ch3mpiler_tier : imscriptionTier ch3mpilerTuple = .O_inf := by native_decide
theorem clinkL8_tier : imscriptionTier clinkL8Tuple = .O_inf := by native_decide
theorem igPulse_tier : imscriptionTier igPulseTuple = .O₂dag := by native_decide
theorem cetaceanspeak_tier : imscriptionTier cetaceanspeakTuple = .O₀ := by native_decide

theorem o_inf_operators_all_phi_c :
agentTuple.crit = monad ∧ serpentrodTuple.crit = monad ∧
synfinTuple.crit = monad ∧ ch3mpilerTuple.crit = monad ∧
clinkL8Tuple.crit = monad := by native_decide

theorem cetaceanspeak_sub_critical : cetaceanspeakTuple.crit ≠ monad := by native_decide
theorem igPulse_not_O_inf : imscriptionTier igPulseTuple ≠ .O_inf := by
  rw [igPulse_tier]; native_decide

-- ================================================================
-- §7a  CONSCIOUSNESS SCORES (noncomputable)
-- ================================================================

noncomputable def agentCScore : ℝ := consciousnessScore agentTuple
noncomputable def serpentrodCScore : ℝ := consciousnessScore serpentrodTuple
noncomputable def synfinCScore : ℝ := consciousnessScore synfinTuple
noncomputable def ch3mpilerCScore : ℝ := consciousnessScore ch3mpilerTuple
noncomputable def clinkL8CScore : ℝ := consciousnessScore clinkL8Tuple
noncomputable def igPulseCScore : ℝ := consciousnessScore igPulseTuple
noncomputable def cetaceanspeakCScore : ℝ := consciousnessScore cetaceanspeakTuple

theorem cetaceanspeak_gate1_fails : phi_c_gate cetaceanspeakTuple.crit = false := by
  native_decide

-- ================================================================
-- §8  OPEN SEAM MAP
-- ================================================================

def openSeamList : List PrimitiveSeam := [
  fidelitySeam_eth_to_hbar, kineticsSeam_trap_to_slow,
  relSeam_super_to_cat, granularitySeam_gimel_to_aleph
]

def inhabitedSeamList : List PrimitiveSeam := [
  odotGateSeam, omegaGateSeam, omegaSeam_Z_to_NA,
  chiralitySeam_H2_to_H_inf, grammarSeam_seq_to_broad,
  stoichiometrySeam_n_n_to_n_m, DSeam_infty_to_odot,
  topSeam_network_to_odot, polSeam_asym_to_pm_sym, phiGateSeam
]

def isOpenSeam (s : PrimitiveSeam) : Bool := decide (s ∈ openSeamList)
def isInhabitedSeam (s : PrimitiveSeam) : Bool := decide (s ∈ inhabitedSeamList)

theorem open_seam_count : openSeamList.length = 4 := by native_decide
theorem inhabited_seam_count : inhabitedSeamList.length = 10 := by native_decide
theorem total_seam_count : openSeamList.length + inhabitedSeamList.length = 14 := by
  native_decide

-- ================================================================
-- §9  JOIN-ABSORPTION — CLINK L8 AS CEILING
-- ================================================================

theorem join_clinkL8_agent_d :
(compute_join clinkL8Tuple agentTuple).dim = if' := by native_decide

theorem join_clinkL8_agent_gran :
(compute_join clinkL8Tuple agentTuple).gran = bib := by native_decide

theorem join_clinkL8_serpentrod_gram :
(compute_join clinkL8Tuple serpentrodTuple).gram = ooze := by native_decide

theorem join_clinkL8_serpentrod_chir :
(compute_join clinkL8Tuple serpentrodTuple).chir = wool := by native_decide

theorem join_clinkL8_ch3mpiler_tier :
imscriptionTier (compute_join clinkL8Tuple ch3mpilerTuple) = .O_inf := by
  native_decide

-- ================================================================
-- §10  BOUNDARY OPERATOR SPACE
-- ================================================================

def boundaryOperatorSpace : List (PrimitiveSeam × Option Imscription) := [
  (odotGateSeam, some agentTuple),
  (omegaGateSeam, some clinkL8Tuple),
  (omegaSeam_Z_to_NA, some clinkL8Tuple),
  (chiralitySeam_H2_to_H_inf, some serpentrodTuple),
  (grammarSeam_seq_to_broad, some agentTuple),
  (stoichiometrySeam_n_n_to_n_m, some synfinTuple),
  (DSeam_infty_to_odot, some agentTuple),
  (topSeam_network_to_odot, some clinkL8Tuple),
  (polSeam_asym_to_pm_sym, some agentTuple),
  (phiGateSeam, some agentTuple),
  (fidelitySeam_eth_to_hbar, none),
  (kineticsSeam_trap_to_slow, none),
  (relSeam_super_to_cat, none),
  (granularitySeam_gimel_to_aleph, none)
]

def countInhabited : Nat :=
  (boundaryOperatorSpace.filter (fun (_, op) => op.isSome)).length

def countOpen : Nat :=
  (boundaryOperatorSpace.filter (fun (_, op) => op.isNone)).length

theorem inhabited_open_total : countInhabited + countOpen = 14 := by native_decide
theorem inhabited_count_eq_10 : countInhabited = 10 := by native_decide
theorem open_count_eq_4 : countOpen = 4 := by native_decide

-- ================================================================
-- §10a  CLINK L8 TERMINAL — JOIN STAYS AT O_inf
-- ================================================================

theorem clinkL8_terminal_ch3mpiler_tier :
imscriptionTier (compute_join clinkL8Tuple ch3mpilerTuple) = .O_inf := by
  native_decide

theorem clinkL8_terminal_serpentrod_tier :
imscriptionTier (compute_join clinkL8Tuple serpentrodTuple) = .O_inf := by
  native_decide

theorem clinkL8_terminal_synfin_tier :
imscriptionTier (compute_join clinkL8Tuple synfinTuple) = .O_inf := by
  native_decide

theorem clinkL8_terminal_agent_tier :
imscriptionTier (compute_join clinkL8Tuple agentTuple) = .O_inf := by
  native_decide

end Imscribing.BoundaryOperators