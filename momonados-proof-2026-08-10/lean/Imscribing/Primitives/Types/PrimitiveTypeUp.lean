/-
  Imscribing.Primitives.Types.PrimitiveTypeUp

  Formalization of the primitive value type 'up'
  Family: Stoichiometry (ordinal 2)
  Surface tokens: stoichiometry, unmatched, asymmetry
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

/-- Surface tokens for 'up' -/
def the_primitive_type_called_up_surfaceTokens : List String := ["stoichiometry", "unmatched", "asymmetry"]

/-- Boundary condition for 'up' -/
def the_primitive_type_called_up_boundaryCondition : String := "the stoichiometry type system boundary"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_up_registerVoid : String := "uninitialized cardinality"
def the_primitive_type_called_up_registerTrue : String := "unmatched stoichiometry (up)"
def the_primitive_type_called_up_registerFalse : String := "matched stoichiometry (hung/so)"
def the_primitive_type_called_up_registerBoth : String := "indeterminate stoichiometric ratio"

/-- Opcode domain-semantic mapping -/
-- AFWD: mapping expansion
--   the transformation from a single source to multiple targets
-- AREV: mapping reduction
--   the transformation from multiple sources back to a single target
-- CLINK: relational composition
--   the chaining of multiple n:m mappings into a single stoichiometric path
-- ENGAGR: dynamic ratio
--   a state where the ratio is fluctuating or undefined between matched and unmatched
-- EVALF: matched state
--   the detection of a 1:1 or n:n symmetry which violates the 'up' type
-- EVALT: unmatched state
--   the confirmation that n does not equal m
-- FFUSE: cardinality reconciliation
--   the unification of divergent paths back into a single n:m relation
-- FSPLIT: cardinality divergence
--   the branching of a relation into source-heavy and target-heavy paths
-- IFIX: type fixation
--   the permanent assignment of the 'up' label to the relation
-- IMSCRIB: self-mapping
--   the recognition of the relation as a distinct stoichiometric type
-- TANCH: stoichiometric closure
--   the final type-checked state of the relation
-- VINIT: untyped relation
--   the state before any ratio or mapping cardinality is assigned

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_up_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_up_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_up_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_up_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_up_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_up_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_up_protocol : IGProtocol the_primitive_type_called_up_s0 the_primitive_type_called_up_s12 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_up_l0 the_primitive_type_called_up_s0 the_primitive_type_called_up_s1) (.seq (.arrow the_primitive_type_called_up_l1 the_primitive_type_called_up_s1 the_primitive_type_called_up_s2) (.seq (.prod (.arrow the_primitive_type_called_up_l2 the_primitive_type_called_up_s2 the_primitive_type_called_up_s6) (.arrow the_primitive_type_called_up_l2 the_primitive_type_called_up_s2 the_primitive_type_called_up_s6)) (.seq (.arrow the_primitive_type_called_up_l6 the_primitive_type_called_up_s6 the_primitive_type_called_up_s6) (.seq (.arrow the_primitive_type_called_up_l6 the_primitive_type_called_up_s6 the_primitive_type_called_up_s7) (.seq (.arrow the_primitive_type_called_up_l7 the_primitive_type_called_up_s7 the_primitive_type_called_up_s8) (.seq (.arrow the_primitive_type_called_up_l8 the_primitive_type_called_up_s8 the_primitive_type_called_up_s9) (.seq (.arrow the_primitive_type_called_up_l9 the_primitive_type_called_up_s9 the_primitive_type_called_up_s10) (.seq (.arrow the_primitive_type_called_up_l10 the_primitive_type_called_up_s10 the_primitive_type_called_up_s11) (.arrow the_primitive_type_called_up_l11 the_primitive_type_called_up_s11 the_primitive_type_called_up_s12))))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_up_true_arm : IGProtocol the_primitive_type_called_up_s0 the_primitive_type_called_up_s12 :=
  (the_primitive_type_called_up_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_up_false_arm : IGProtocol the_primitive_type_called_up_s0 the_primitive_type_called_up_s12 :=
  (the_primitive_type_called_up_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_up_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_up_s0

#eval the_primitive_type_called_up_tier  -- the Grammar's own verdict on its tier

/-- 'up' belongs to the Stoichiometry family at ordinal 2 -/
def the_primitive_type_called_up_family : String := "Stoichiometry"
def the_primitive_type_called_up_ordinal : Nat := 2
def the_primitive_type_called_up_bootstrapSteps : Nat := 13

end Imscribing.Primitives.Types
