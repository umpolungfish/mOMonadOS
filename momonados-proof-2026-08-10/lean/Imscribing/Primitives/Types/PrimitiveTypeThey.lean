/-
  Imscribing.Primitives.Types.PrimitiveTypeThey

  Formalization of the primitive value type 'they'
  Family: Stoichiometry (ordinal 1)
  Surface tokens: HotSwap, threshold, fidelity
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

/-- Surface tokens for 'they' -/
def the_primitive_type_called_they_surfaceTokens : List String := ["HotSwap", "threshold", "fidelity"]

/-- Boundary condition for 'they' -/
def the_primitive_type_called_they_boundaryCondition : String := "Fidelity Family F3"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_they_registerVoid : String := "uninitialized signal state"
def the_primitive_type_called_they_registerTrue : String := "signal exceeds \u03b7 threshold"
def the_primitive_type_called_they_registerFalse : String := "signal below \u03b7 threshold"
def the_primitive_type_called_they_registerBoth : String := "signal at exact criticality \u03b7"

/-- Opcode domain-semantic mapping -/
-- AFWD: signal amplification
--   The forward transformation of the signal toward the detection threshold.
-- AREV: noise floor descent
--   The reverse transformation or reduction of signal to the noise floor.
-- CLINK: sampling chain
--   The sequential composition of discrete signal samples.
-- ENGAGR: metastable state
--   A paradice where the signal is exactly at the threshold boundary.
-- EVALF: packet drop
--   The negative state where the signal fails the η threshold.
-- EVALT: HotSwap trigger
--   The affirmative state where the signal meets the η threshold.
-- FFUSE: signal integrator
--   Reconstitutes the total signal energy from the split paths.
-- FSPLIT: threshold comparator
--   Splits the signal into supra-threshold and sub-threshold paths.
-- IFIX: fidelity log
--   The permanent record of the fidelity classification event.
-- IMSCRIB: self-consistency check
--   The identity operation verifying the signal matches its own signature.
-- TANCH: Fidelity.they
--   The terminal classification of the signal as a threshold-limited type.
-- VINIT: untyped signal
--   The raw unclassified data stream before fidelity assessment.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_they_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_they_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_they_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_they_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_they_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_they_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_they_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_they_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_they_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_they_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_they_protocol : IGProtocol the_primitive_type_called_they_s0 the_primitive_type_called_they_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_they_l0 the_primitive_type_called_they_s0 the_primitive_type_called_they_s1) (.seq (.arrow the_primitive_type_called_they_l1 the_primitive_type_called_they_s1 the_primitive_type_called_they_s2) (.seq (.arrow the_primitive_type_called_they_l2 the_primitive_type_called_they_s2 the_primitive_type_called_they_s3) (.seq (.arrow the_primitive_type_called_they_l3 the_primitive_type_called_they_s3 the_primitive_type_called_they_s4) (.seq (.prod (.arrow the_primitive_type_called_they_l4 the_primitive_type_called_they_s4 the_primitive_type_called_they_s10) (.arrow the_primitive_type_called_they_l4 the_primitive_type_called_they_s4 the_primitive_type_called_they_s10)) (.seq (.arrow the_primitive_type_called_they_l10 the_primitive_type_called_they_s10 the_primitive_type_called_they_s10) (.arrow the_primitive_type_called_they_l10 the_primitive_type_called_they_s10 the_primitive_type_called_they_s11)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_they_true_arm : IGProtocol the_primitive_type_called_they_s0 the_primitive_type_called_they_s11 :=
  (the_primitive_type_called_they_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_they_false_arm : IGProtocol the_primitive_type_called_they_s0 the_primitive_type_called_they_s11 :=
  (the_primitive_type_called_they_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_they_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_they_s0
theorem the_primitive_type_called_they_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_they_s0 the_primitive_type_called_they_s0 = the_primitive_type_called_they_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_they_s0

#eval the_primitive_type_called_they_tier  -- the Grammar's own verdict on its tier

/-- 'they' belongs to the Stoichiometry family at ordinal 1 -/
def the_primitive_type_called_they_family : String := "Fidelity"
def the_primitive_type_called_they_ordinal : Nat := 1
def the_primitive_type_called_they_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
