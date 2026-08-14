/-
  Imscribing.Primitives.Types.PrimitiveTypeVow

  Formalization of the primitive value type 'vow'
  Family: Grammar (ordinal 0)
  Surface tokens: conjunction, simultaneity, constraint
  Author: Lando⊗⊙perator
-/

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.TierCrossing
import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing.Primitives.Types
open Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

/-- Surface tokens for 'vow' -/
def the_primitive_type_called_vow_surfaceTokens : List String := ["conjunction", "simultaneity", "constraint"]

/-- Boundary condition for 'vow' -/
def the_primitive_type_called_vow_boundaryCondition : String := "The Interaction Grammar (\u0393) meet-semilattice"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_vow_registerVoid : String := "uninitialized grammar state"
def the_primitive_type_called_vow_registerTrue : String := "all conditions satisfied (vow success)"
def the_primitive_type_called_vow_registerFalse : String := "at least one condition failed"
def the_primitive_type_called_vow_registerBoth : String := "conflicting simultaneous requirements"

/-- Opcode domain-semantic mapping -/
-- AFWD: requirement addition
--   The forward movement of adding a new condition to the conjunctive set.
-- AREV: constraint relaxation
--   The removal or descent from a specific requirement back toward a broader state.
-- CLINK: logical intersection
--   The sequential chaining of multiple conditions into a single conjunctive requirement.
-- ENGAGR: quantum superposition of constraints
--   A state where a condition is both required and not yet determined.
-- EVALF: violation
--   The state where a condition fails to be met.
-- EVALT: satisfaction
--   The state where a condition is successfully met.
-- FFUSE: conjunctive synthesis
--   The reconstitution of the original state only if all parallel conditions are met.
-- FSPLIT: parallel condition evaluation
--   The branching of a single state into multiple conditions that must all be satisfied.
-- IFIX: vow fixation
--   The permanent recording of the conjunctive interaction in the crystal.
-- IMSCRIB: self-consistency check
--   The identity operation where a requirement is verified against itself.
-- TANCH: vow primitive
--   The terminal definition of the conjunctive interaction mode within the grammar.
-- VINIT: unconstrained state
--   The initial state before any conjunctive requirements are imposed.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_vow_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_vow_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_vow_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_vow_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_vow_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_vow_protocol : IGProtocol the_primitive_type_called_vow_s0 the_primitive_type_called_vow_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_vow_l0 the_primitive_type_called_vow_s0 the_primitive_type_called_vow_s1) (.seq (.arrow the_primitive_type_called_vow_l1 the_primitive_type_called_vow_s1 the_primitive_type_called_vow_s2) (.seq (.prod (.arrow the_primitive_type_called_vow_l2 the_primitive_type_called_vow_s2 the_primitive_type_called_vow_s8) (.arrow the_primitive_type_called_vow_l2 the_primitive_type_called_vow_s2 the_primitive_type_called_vow_s8)) (.seq (.arrow the_primitive_type_called_vow_l8 the_primitive_type_called_vow_s8 the_primitive_type_called_vow_s8) (.seq (.arrow the_primitive_type_called_vow_l8 the_primitive_type_called_vow_s8 the_primitive_type_called_vow_s9) (.seq (.arrow the_primitive_type_called_vow_l9 the_primitive_type_called_vow_s9 the_primitive_type_called_vow_s10) (.arrow the_primitive_type_called_vow_l10 the_primitive_type_called_vow_s10 the_primitive_type_called_vow_s11)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_vow_true_arm : IGProtocol the_primitive_type_called_vow_s0 the_primitive_type_called_vow_s11 :=
  (the_primitive_type_called_vow_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_vow_false_arm : IGProtocol the_primitive_type_called_vow_s0 the_primitive_type_called_vow_s11 :=
  (the_primitive_type_called_vow_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_vow_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_vow_s0
theorem the_primitive_type_called_vow_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_vow_s0 the_primitive_type_called_vow_s0 = the_primitive_type_called_vow_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_vow_s0

#eval the_primitive_type_called_vow_tier  -- the Grammar's own verdict on its tier

/-- 'vow' belongs to the Grammar family at ordinal 0 -/
def the_primitive_type_called_vow_family : String := "Grammar"
def the_primitive_type_called_vow_ordinal : Nat := 0
def the_primitive_type_called_vow_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
