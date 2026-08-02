/-
  Imscribing.Primitives.Types.PrimitiveTypeSure

  Formalization of the primitive value type 'sure'
  Family: Chirality (ordinal 2)
  Surface tokens: chirality, asymmetry, persistence
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

/-- Surface tokens for 'sure' -/
def the_primitive_type_called_sure_surfaceTokens : List String := ["chirality", "asymmetry", "persistence"]

/-- Boundary condition for 'sure' -/
def the_primitive_type_called_sure_boundaryCondition : String := "the temporal memory horizon of the primitive type system"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_sure_registerVoid : String := "uninitialized temporal state with no defined handedness"
def the_primitive_type_called_sure_registerTrue : String := "persistent chiral state with strong temporal asymmetry"
def the_primitive_type_called_sure_registerFalse : String := "achiral state where temporal memory is lost"
def the_primitive_type_called_sure_registerBoth : String := "superposed chiral state holding both orientations simultaneously"

/-- Opcode domain-semantic mapping -/
-- AFWD: right-handed propagation
--   the forward transformation of a chiral signal through time
-- AREV: left-handed propagation
--   the reverse or parity-flipped transformation of a chiral signal
-- CLINK: temporal chaining
--   the sequential composition of chiral states across successive clock cycles
-- ENGAGR: racemic mixture
--   the simultaneous presence of both chiralities without resolution into a single dominant hand
-- EVALF: symmetry restoration
--   the failure to maintain chirality resulting in a return to achiral state
-- EVALT: strong asymmetry
--   the affirmative detection of persistent temporal directionality
-- FFUSE: chirality synthesis
--   the reconstitution of the original signal from its parity-split components
-- FSPLIT: parity bifurcation
--   the branching of a neutral signal into distinct right-handed and left-handed paths
-- IFIX: persistent record
--   the permanent fixation of the chiral state into the type system memory
-- IMSCRIB: chirality recognition
--   the self-identification of the state as possessing a specific handedness
-- TANCH: sure type definition
--   the final boundary that contains the persistent chiral state
-- VINIT: achiral vacuum
--   the state of zero temporal memory where no directionality is established

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_sure_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_sure_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_sure_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_sure_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_sure_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_sure_protocol : IGProtocol the_primitive_type_called_sure_s0 the_primitive_type_called_sure_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_sure_l0 the_primitive_type_called_sure_s0 the_primitive_type_called_sure_s1) (.seq (.arrow the_primitive_type_called_sure_l1 the_primitive_type_called_sure_s1 the_primitive_type_called_sure_s2) (.seq (.prod (.arrow the_primitive_type_called_sure_l2 the_primitive_type_called_sure_s2 the_primitive_type_called_sure_s7) (.arrow the_primitive_type_called_sure_l2 the_primitive_type_called_sure_s2 the_primitive_type_called_sure_s7)) (.seq (.arrow the_primitive_type_called_sure_l7 the_primitive_type_called_sure_s7 the_primitive_type_called_sure_s7) (.seq (.arrow the_primitive_type_called_sure_l7 the_primitive_type_called_sure_s7 the_primitive_type_called_sure_s8) (.seq (.arrow the_primitive_type_called_sure_l8 the_primitive_type_called_sure_s8 the_primitive_type_called_sure_s9) (.seq (.arrow the_primitive_type_called_sure_l9 the_primitive_type_called_sure_s9 the_primitive_type_called_sure_s10) (.arrow the_primitive_type_called_sure_l10 the_primitive_type_called_sure_s10 the_primitive_type_called_sure_s11))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_sure_true_arm : IGProtocol the_primitive_type_called_sure_s0 the_primitive_type_called_sure_s11 :=
  (the_primitive_type_called_sure_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_sure_false_arm : IGProtocol the_primitive_type_called_sure_s0 the_primitive_type_called_sure_s11 :=
  (the_primitive_type_called_sure_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_sure_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_sure_s0
theorem the_primitive_type_called_sure_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_sure_s0 the_primitive_type_called_sure_s0 = the_primitive_type_called_sure_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_sure_s0

#eval the_primitive_type_called_sure_tier  -- the Grammar's own verdict on its tier

/-- 'sure' belongs to the Chirality family at ordinal 2 -/
def the_primitive_type_called_sure_family : String := "Chirality"
def the_primitive_type_called_sure_ordinal : Nat := 2
def the_primitive_type_called_sure_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
