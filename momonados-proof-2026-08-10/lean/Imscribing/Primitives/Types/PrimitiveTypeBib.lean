/-
  Imscribing.Primitives.Types.PrimitiveTypeBib

  Formalization of the primitive value type 'bib'
  Family: Granularity (ordinal 0)
  Surface tokens: mesoscale, correlation, neighborhood
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

/-- Surface tokens for 'bib' -/
def the_primitive_type_called_bib_surfaceTokens : List String := ["mesoscale", "correlation", "neighborhood"]

/-- Boundary condition for 'bib' -/
def the_primitive_type_called_bib_boundaryCondition : String := "local correlation length"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_bib_registerVoid : String := "unscoped data"
def the_primitive_type_called_bib_registerTrue : String := "valid local correlation"
def the_primitive_type_called_bib_registerFalse : String := "local noise"
def the_primitive_type_called_bib_registerBoth : String := "mesoscale interference"

/-- Opcode domain-semantic mapping -/
-- AFWD: spatial mapping
--   projection of data into a local coordinate system
-- AREV: inverse projection
--   mapping local coordinates back to the global field
-- CLINK: neighborhood chaining
--   sequential linking of adjacent local points
-- ENGAGR: boundary overlap
--   a state where a point belongs to two adjacent mesoscale regions
-- EVALF: noise detection
--   failure to find local structure within the bib radius
-- EVALT: correlation match
--   successful identification of a local pattern
-- FFUSE: signal reconstruction
--   recombining components to recover the original local signal
-- FSPLIT: correlation fork
--   splitting a signal into correlated and uncorrelated components
-- IFIX: granularity record
--   permanent fixation of the bib scale in the type registry
-- IMSCRIB: local identity
--   recognition of a point as its own center within the bib scope
-- TANCH: bib boundary
--   the limit of the mesoscale beyond which correlations become collective
-- VINIT: unbounded field
--   the raw data space before local constraints are applied

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_bib_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_primitive_type_called_bib_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_bib_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_bib_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_bib_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_bib_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_bib_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_bib_protocol : IGProtocol the_primitive_type_called_bib_s0 the_primitive_type_called_bib_s10 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_bib_l0 the_primitive_type_called_bib_s0 the_primitive_type_called_bib_s1) (.seq (.arrow the_primitive_type_called_bib_l1 the_primitive_type_called_bib_s1 the_primitive_type_called_bib_s2) (.seq (.arrow the_primitive_type_called_bib_l2 the_primitive_type_called_bib_s2 the_primitive_type_called_bib_s3) (.seq (.prod (.arrow the_primitive_type_called_bib_l3 the_primitive_type_called_bib_s3 the_primitive_type_called_bib_s6) (.arrow the_primitive_type_called_bib_l3 the_primitive_type_called_bib_s3 the_primitive_type_called_bib_s6)) (.seq (.arrow the_primitive_type_called_bib_l6 the_primitive_type_called_bib_s6 the_primitive_type_called_bib_s6) (.seq (.arrow the_primitive_type_called_bib_l6 the_primitive_type_called_bib_s6 the_primitive_type_called_bib_s7) (.seq (.arrow the_primitive_type_called_bib_l7 the_primitive_type_called_bib_s7 the_primitive_type_called_bib_s8) (.seq (.arrow the_primitive_type_called_bib_l8 the_primitive_type_called_bib_s8 the_primitive_type_called_bib_s9) (.arrow the_primitive_type_called_bib_l9 the_primitive_type_called_bib_s9 the_primitive_type_called_bib_s10)))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_bib_true_arm : IGProtocol the_primitive_type_called_bib_s0 the_primitive_type_called_bib_s10 :=
  (the_primitive_type_called_bib_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_bib_false_arm : IGProtocol the_primitive_type_called_bib_s0 the_primitive_type_called_bib_s10 :=
  (the_primitive_type_called_bib_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_bib_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_bib_s0
theorem the_primitive_type_called_bib_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_bib_s0 the_primitive_type_called_bib_s0 = the_primitive_type_called_bib_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_bib_s0

#eval the_primitive_type_called_bib_tier  -- the Grammar's own verdict on its tier

/-- 'bib' belongs to the Granularity family at ordinal 0 -/
def the_primitive_type_called_bib_family : String := "Granularity"
def the_primitive_type_called_bib_ordinal : Nat := 0
def the_primitive_type_called_bib_bootstrapSteps : Nat := 11

end Imscribing.Primitives.Types
