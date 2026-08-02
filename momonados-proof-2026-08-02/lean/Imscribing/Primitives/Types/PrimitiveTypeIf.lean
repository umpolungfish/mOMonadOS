/-
  Imscribing.Primitives.Types.PrimitiveTypeIf

  Formalization of the primitive value type 'if'
  Family: Dimensionality (ordinal 3)
  Surface tokens: holographic projection, bulk encoding, boundary manifold
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

/-- Surface tokens for 'if' -/
def the_primitive_type_called_if_surfaceTokens : List String := ["holographic projection", "bulk encoding", "boundary manifold"]

/-- Boundary condition for 'if' -/
def the_primitive_type_called_if_boundaryCondition : String := "The holographic principle constraint where boundary information density limits bulk volume complexity."

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_if_registerVoid : String := "A state of zero dimensionality where no information is localized or projected."
def the_primitive_type_called_if_registerTrue : String := "A valid holographic mapping where the boundary successfully encodes the bulk."
def the_primitive_type_called_if_registerFalse : String := "A mapping failure where the boundary-bulk correspondence is broken or lossy."
def the_primitive_type_called_if_registerBoth : String := "A state of maximum criticality where bulk and boundary are indistinguishable."

/-- Opcode domain-semantic mapping -/
-- AFWD: bulk-to-boundary projection
--   The forward mapping of higher-dimensional bulk data onto the lower-dimensional boundary manifold.
-- AREV: boundary-to-bulk reconstruction
--   The inverse mapping that recovers the internal bulk state from boundary data.
-- CLINK: recursive encoding chain
--   The sequential application of projection operators across nested dimensional layers.
-- ENGAGR: critical duality
--   The simultaneous existence of bulk and boundary as a single inseparable entity without resolution into one or the other.
-- EVALF: information loss
--   The state where the boundary fails to capture the full complexity of the bulk volume.
-- EVALT: lossless encoding
--   The state where the boundary projection preserves all bulk information degrees of freedom.
-- FFUSE: holographic integration
--   The reunification of bulk and boundary into a single consistent information manifold.
-- FSPLIT: dimensional bifurcation
--   The separation of a unified information state into a bulk volume and a boundary surface.
-- IFIX: topological record
--   The permanent fixation of the holographic mapping into a non-volatile memory state.
-- IMSCRIB: self-consistent manifold
--   The identity state where the boundary correctly recognizes itself as the encoding of its own bulk.
-- TANCH: holographic screen
--   The terminal boundary that contains and defines the limits of the bulk information.
-- VINIT: unstructured data space
--   The raw potential for dimensionality before any geometric projection or encoding occurs.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_if_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_if_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_if_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_if_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_if_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_if_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_if_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_if_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_if_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_if_protocol : IGProtocol the_primitive_type_called_if_s0 the_primitive_type_called_if_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_if_l0 the_primitive_type_called_if_s0 the_primitive_type_called_if_s1) (.seq (.arrow the_primitive_type_called_if_l1 the_primitive_type_called_if_s1 the_primitive_type_called_if_s2) (.seq (.prod (.arrow the_primitive_type_called_if_l2 the_primitive_type_called_if_s2 the_primitive_type_called_if_s9) (.arrow the_primitive_type_called_if_l2 the_primitive_type_called_if_s2 the_primitive_type_called_if_s9)) (.seq (.arrow the_primitive_type_called_if_l9 the_primitive_type_called_if_s9 the_primitive_type_called_if_s9) (.seq (.arrow the_primitive_type_called_if_l9 the_primitive_type_called_if_s9 the_primitive_type_called_if_s10) (.arrow the_primitive_type_called_if_l10 the_primitive_type_called_if_s10 the_primitive_type_called_if_s11))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_if_true_arm : IGProtocol the_primitive_type_called_if_s0 the_primitive_type_called_if_s11 :=
  (the_primitive_type_called_if_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_if_false_arm : IGProtocol the_primitive_type_called_if_s0 the_primitive_type_called_if_s11 :=
  (the_primitive_type_called_if_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_if_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_if_s0
theorem the_primitive_type_called_if_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_if_s0 the_primitive_type_called_if_s0 = the_primitive_type_called_if_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_if_s0

#eval the_primitive_type_called_if_tier  -- the Grammar's own verdict on its tier

/-- 'if' belongs to the Dimensionality family at ordinal 3 -/
def the_primitive_type_called_if_family : String := "Dimensionality"
def the_primitive_type_called_if_ordinal : Nat := 3
def the_primitive_type_called_if_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
