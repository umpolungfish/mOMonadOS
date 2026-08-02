/-
  Imscribing.Primitives.Types.PrimitiveTypeGag

  Formalization of the primitive value type 'gag'
  Family: Grammar (ordinal 1)
  Surface tokens: disjunction, branching, alternative
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

/-- Surface tokens for 'gag' -/
def the_primitive_type_called_gag_surfaceTokens : List String := ["disjunction", "branching", "alternative"]

/-- Boundary condition for 'gag' -/
def the_primitive_type_called_gag_boundaryCondition : String := "Interaction Grammar (\u0393) Type System"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_gag_registerVoid : String := "uninitialized grammar state"
def the_primitive_type_called_gag_registerTrue : String := "successful disjunctive resolution"
def the_primitive_type_called_gag_registerFalse : String := "failed disjunctive resolution"
def the_primitive_type_called_gag_registerBoth : String := "unresolved alternative ambiguity"

/-- Opcode domain-semantic mapping -/
-- AFWD: alternative selection
--   The forward movement toward one of the available branches.
-- AREV: backtrack to junction
--   Returning to the split point to evaluate a different alternative.
-- CLINK: rule composition
--   Chaining multiple disjunctive conditions into a single logical path.
-- ENGAGR: non-deterministic superposition
--   A state where both alternatives are held as valid simultaneously.
-- EVALF: secondary branch success
--   The state where the first alternative fails but the second succeeds.
-- EVALT: primary branch success
--   The state where the first alternative satisfies the condition.
-- FFUSE: logical join
--   Reconstituting the original term from the result of either branch.
-- FSPLIT: disjunctive fork
--   Splitting the execution path into two mutually exclusive alternatives.
-- IFIX: type assignment record
--   The permanent append-only record of the resolved branch in the trace.
-- IMSCRIB: self-identity of gag
--   The recognition that the disjunction is a first-class type.
-- TANCH: Grammar.gag type witness
--   The terminal boundary that validates the disjunctive structure.
-- VINIT: untyped expression
--   The raw state of a term before grammar rules apply.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_gag_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_gag_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_gag_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_gag_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_gag_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_gag_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_gag_protocol : IGProtocol the_primitive_type_called_gag_s0 the_primitive_type_called_gag_s12 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_gag_l0 the_primitive_type_called_gag_s0 the_primitive_type_called_gag_s1) (.seq (.arrow the_primitive_type_called_gag_l1 the_primitive_type_called_gag_s1 the_primitive_type_called_gag_s2) (.seq (.arrow the_primitive_type_called_gag_l2 the_primitive_type_called_gag_s2 the_primitive_type_called_gag_s3) (.seq (.prod (.arrow the_primitive_type_called_gag_l3 the_primitive_type_called_gag_s3 the_primitive_type_called_gag_s10) (.arrow the_primitive_type_called_gag_l3 the_primitive_type_called_gag_s3 the_primitive_type_called_gag_s10)) (.seq (.arrow the_primitive_type_called_gag_l10 the_primitive_type_called_gag_s10 the_primitive_type_called_gag_s10) (.seq (.arrow the_primitive_type_called_gag_l10 the_primitive_type_called_gag_s10 the_primitive_type_called_gag_s11) (.arrow the_primitive_type_called_gag_l11 the_primitive_type_called_gag_s11 the_primitive_type_called_gag_s12)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_gag_true_arm : IGProtocol the_primitive_type_called_gag_s0 the_primitive_type_called_gag_s12 :=
  (the_primitive_type_called_gag_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_gag_false_arm : IGProtocol the_primitive_type_called_gag_s0 the_primitive_type_called_gag_s12 :=
  (the_primitive_type_called_gag_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_gag_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_gag_s0
theorem the_primitive_type_called_gag_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_gag_s0 the_primitive_type_called_gag_s0 = the_primitive_type_called_gag_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_gag_s0

#eval the_primitive_type_called_gag_tier  -- the Grammar's own verdict on its tier

/-- 'gag' belongs to the Grammar family at ordinal 1 -/
def the_primitive_type_called_gag_family : String := "Grammar"
def the_primitive_type_called_gag_ordinal : Nat := 1
def the_primitive_type_called_gag_bootstrapSteps : Nat := 13

end Imscribing.Primitives.Types
