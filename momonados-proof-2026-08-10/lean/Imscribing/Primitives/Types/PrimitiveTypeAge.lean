/-
  Imscribing.Primitives.Types.PrimitiveTypeAge

  Formalization of the primitive value type 'age'
  Family: Fidelity (ordinal 0)
  Surface tokens: classical search, lossy fidelity, bottleneck
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

/-- Surface tokens for 'age' -/
def the_primitive_type_called_age_surfaceTokens : List String := ["classical search", "lossy fidelity", "bottleneck"]

/-- Boundary condition for 'age' -/
def the_primitive_type_called_age_boundaryCondition : String := "The \u2113-fidelity classical search space"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_age_registerVoid : String := "pre-search state with no defined fidelity"
def the_primitive_type_called_age_registerTrue : String := "successful search step within age fidelity bounds"
def the_primitive_type_called_age_registerFalse : String := "search failure or excessive information loss"
def the_primitive_type_called_age_registerBoth : String := "a state of partial information where signal and noise are indistinguishable"

/-- Opcode domain-semantic mapping -/
-- AFWD: search progression
--   The forward movement through the search space toward a target.
-- AREV: backtracking
--   The reversal of search steps when a path is exhausted or invalid.
-- CLINK: step composition
--   The chaining of individual search operations into a continuous path.
-- ENGAGR: stochastic noise
--   The simultaneous presence of signal and noise in a lossy classical system.
-- EVALF: fidelity loss
--   The detection of information loss exceeding the 'age' limit.
-- EVALT: fidelity match
--   The confirmation that the operation maintains the required classical fidelity.
-- FFUSE: fidelity resolution
--   The reconstitution of the search state after the fidelity check.
-- FSPLIT: fidelity comparison
--   The branching of the search based on whether the current fidelity meets the 'age' threshold.
-- IFIX: search log
--   The permanent record of the search path and its fidelity metrics.
-- IMSCRIB: fidelity self-recognition
--   The system identifying its own state as 'age' (classical lossy).
-- TANCH: search termination
--   The final boundary where the search concludes and the result is returned.
-- VINIT: uninitialized search space
--   The state before any search parameters or fidelity constraints are defined.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_age_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_age_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_age_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_age_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_age_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_age_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_age_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_age_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_age_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_age_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_age_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_age_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_age_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_age_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_age_protocol : IGProtocol the_primitive_type_called_age_s0 the_primitive_type_called_age_s15 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_age_l0 the_primitive_type_called_age_s0 the_primitive_type_called_age_s1) (.seq (.arrow the_primitive_type_called_age_l1 the_primitive_type_called_age_s1 the_primitive_type_called_age_s2) (.seq (.arrow the_primitive_type_called_age_l2 the_primitive_type_called_age_s2 the_primitive_type_called_age_s3) (.seq (.prod (.arrow the_primitive_type_called_age_l3 the_primitive_type_called_age_s3 the_primitive_type_called_age_s5) (.arrow the_primitive_type_called_age_l3 the_primitive_type_called_age_s3 the_primitive_type_called_age_s5)) (.seq (.arrow the_primitive_type_called_age_l5 the_primitive_type_called_age_s5 the_primitive_type_called_age_s5) (.seq (.arrow the_primitive_type_called_age_l5 the_primitive_type_called_age_s5 the_primitive_type_called_age_s6) (.seq (.arrow the_primitive_type_called_age_l6 the_primitive_type_called_age_s6 the_primitive_type_called_age_s7) (.seq (.arrow the_primitive_type_called_age_l7 the_primitive_type_called_age_s7 the_primitive_type_called_age_s8) (.seq (.arrow the_primitive_type_called_age_l8 the_primitive_type_called_age_s8 the_primitive_type_called_age_s9) (.seq (.arrow the_primitive_type_called_age_l9 the_primitive_type_called_age_s9 the_primitive_type_called_age_s10) (.seq (.arrow the_primitive_type_called_age_l10 the_primitive_type_called_age_s10 the_primitive_type_called_age_s11) (.seq (.arrow the_primitive_type_called_age_l11 the_primitive_type_called_age_s11 the_primitive_type_called_age_s12) (.seq (.arrow the_primitive_type_called_age_l12 the_primitive_type_called_age_s12 the_primitive_type_called_age_s13) (.seq (.arrow the_primitive_type_called_age_l13 the_primitive_type_called_age_s13 the_primitive_type_called_age_s14) (.arrow the_primitive_type_called_age_l14 the_primitive_type_called_age_s14 the_primitive_type_called_age_s15)))))))))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_age_true_arm : IGProtocol the_primitive_type_called_age_s0 the_primitive_type_called_age_s15 :=
  (the_primitive_type_called_age_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_age_false_arm : IGProtocol the_primitive_type_called_age_s0 the_primitive_type_called_age_s15 :=
  (the_primitive_type_called_age_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_age_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_age_s0

#eval the_primitive_type_called_age_tier  -- the Grammar's own verdict on its tier

/-- 'age' belongs to the Fidelity family at ordinal 0 -/
def the_primitive_type_called_age_family : String := "Fidelity"
def the_primitive_type_called_age_ordinal : Nat := 0
def the_primitive_type_called_age_bootstrapSteps : Nat := 16

end Imscribing.Primitives.Types
