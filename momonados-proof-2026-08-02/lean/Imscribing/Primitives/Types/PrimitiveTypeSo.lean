/-
  Imscribing.Primitives.Types.PrimitiveTypeSo

  Formalization of the primitive value type 'so'
  Family: Granularity (ordinal 2)
  Surface tokens: stoichiometry, matched-ratio, multi-channel
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

/-- Surface tokens for 'so' -/
def the_primitive_type_called_so_surfaceTokens : List String := ["stoichiometry", "matched-ratio", "multi-channel"]

/-- Boundary condition for 'so' -/
def the_primitive_type_called_so_boundaryCondition : String := "Stoichiometry Type Definition"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_so_registerVoid : String := "null-stoichiometry: no defined ratio or participants"
def the_primitive_type_called_so_registerTrue : String := "so-state: verified n:n matched many-to-many relation"
def the_primitive_type_called_so_registerFalse : String := "mismatch-state: failed stoichiometry check"
def the_primitive_type_called_so_registerBoth : String := "super-stoichiometric: multiple valid ratios held in superposition"

/-- Opcode domain-semantic mapping -/
-- AFWD: ratio-mapping
--   The forward transformation from a single input to a set of outputs.
-- AREV: inverse-mapping
--   The reverse transformation from outputs back to inputs to verify count.
-- CLINK: channel-composition
--   The chaining of multiple n-to-n mappings into a single stoichiometric block.
-- ENGAGR: dynamic-scaling
--   A state where the ratio is simultaneously fixed and scalable across different magnitudes.
-- EVALF: unmatched-parity
--   The condition where counts do not match, failing the 'so' definition.
-- EVALT: matched-parity
--   The condition where input count exactly matches output count (n:n).
-- FFUSE: n-to-n-reconstitution
--   Recombining the n-channels back into a single matched stoichiometric relation.
-- FSPLIT: n-to-n-branching
--   Splitting a unified interaction into n distinct input and n distinct output channels.
-- IFIX: type-fixation
--   The permanent recording of the 'so' tag in the Lean environment.
-- IMSCRIB: self-identity-check
--   Verification that the ratio n:n maintains the identity of the participating types.
-- TANCH: S-primitive-closure
--   The final type-checked stoichiometry value within the F3 family.
-- VINIT: untyped-relation
--   The state before any ratio or count is assigned to the interaction.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_so_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_so_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_so_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_so_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_so_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_so_protocol : IGProtocol the_primitive_type_called_so_s0 the_primitive_type_called_so_s10 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_so_l0 the_primitive_type_called_so_s0 the_primitive_type_called_so_s1) (.seq (.arrow the_primitive_type_called_so_l1 the_primitive_type_called_so_s1 the_primitive_type_called_so_s2) (.seq (.prod (.arrow the_primitive_type_called_so_l2 the_primitive_type_called_so_s2 the_primitive_type_called_so_s7) (.arrow the_primitive_type_called_so_l2 the_primitive_type_called_so_s2 the_primitive_type_called_so_s7)) (.seq (.arrow the_primitive_type_called_so_l7 the_primitive_type_called_so_s7 the_primitive_type_called_so_s7) (.seq (.arrow the_primitive_type_called_so_l7 the_primitive_type_called_so_s7 the_primitive_type_called_so_s8) (.seq (.arrow the_primitive_type_called_so_l8 the_primitive_type_called_so_s8 the_primitive_type_called_so_s9) (.arrow the_primitive_type_called_so_l9 the_primitive_type_called_so_s9 the_primitive_type_called_so_s10)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_so_true_arm : IGProtocol the_primitive_type_called_so_s0 the_primitive_type_called_so_s10 :=
  (the_primitive_type_called_so_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_so_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_so_s0
theorem the_primitive_type_called_so_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_so_s0 the_primitive_type_called_so_s0 = the_primitive_type_called_so_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_so_s0

#eval the_primitive_type_called_so_tier  -- the Grammar's own verdict on its tier

/-- 'so' belongs to the Granularity family at ordinal 2 -/
def the_primitive_type_called_so_family : String := "Stoichiometry"
def the_primitive_type_called_so_ordinal : Nat := 1
def the_primitive_type_called_so_bootstrapSteps : Nat := 11

end Imscribing.Primitives.Types
