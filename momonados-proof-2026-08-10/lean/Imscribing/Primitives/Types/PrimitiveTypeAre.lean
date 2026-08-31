/-
  Imscribing.Primitives.Types.PrimitiveTypeAre

  Formalization of the primitive value type 'are'
  Family: Topology (ordinal 4)
  Surface tokens: boundary, bulk, encoding
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

/-- Surface tokens for 'are' -/
def the_primitive_type_called_are_surfaceTokens : List String := ["boundary", "bulk", "encoding"]

/-- Boundary condition for 'are' -/
def the_primitive_type_called_are_boundaryCondition : String := "The holographic screen containing the complete bulk projection"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_are_registerVoid : String := "uninitialized manifold without defined dimensionality or topology"
def the_primitive_type_called_are_registerTrue : String := "successful holographic encoding with full bulk-boundary correspondence"
def the_primitive_type_called_are_registerFalse : String := "failed projection where boundary data is insufficient to map the bulk"
def the_primitive_type_called_are_registerBoth : String := "entangled state where bulk and boundary are indistinguishable"

/-- Opcode domain-semantic mapping -/
-- AFWD: projection map
--   The forward morphism from the bulk interior to the boundary surface.
-- AREV: reconstruction map
--   The reverse morphism reconstructing the bulk from boundary data.
-- CLINK: transversal operator
--   The composition of local boundary operations to simulate bulk movement.
-- ENGAGR: bulk-boundary entanglement
--   The simultaneous existence of the system as both interior and surface.
-- EVALF: information loss
--   The state where the boundary projection is incomplete or decoherent.
-- EVALT: lossless correspondence
--   The state where boundary data perfectly recovers the bulk state.
-- FFUSE: holographic synthesis
--   Reconstituting the unified holographic state from the boundary and bulk representations.
-- FSPLIT: duality bifurcation
--   Splitting the system into a boundary-only representation and a bulk-only representation.
-- IFIX: topological record
--   The permanent fixation of the winding number or holographic state.
-- IMSCRIB: self-encoding identity
--   The recognition that the boundary is the bulk in a different representation.
-- TANCH: holographic screen
--   The terminal boundary that contains the entirety of the bulk information.
-- VINIT: unmapped manifold
--   The state of the bulk before any boundary correspondence is established.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_are_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_primitive_type_called_are_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_are_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_are_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_are_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_are_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_are_protocol : IGProtocol the_primitive_type_called_are_s0 the_primitive_type_called_are_s10 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_are_l0 the_primitive_type_called_are_s0 the_primitive_type_called_are_s1) (.seq (.arrow the_primitive_type_called_are_l1 the_primitive_type_called_are_s1 the_primitive_type_called_are_s2) (.seq (.arrow the_primitive_type_called_are_l2 the_primitive_type_called_are_s2 the_primitive_type_called_are_s3) (.seq (.arrow the_primitive_type_called_are_l3 the_primitive_type_called_are_s3 the_primitive_type_called_are_s4) (.seq (.prod (.arrow the_primitive_type_called_are_l4 the_primitive_type_called_are_s4 the_primitive_type_called_are_s8) (.arrow the_primitive_type_called_are_l4 the_primitive_type_called_are_s4 the_primitive_type_called_are_s8)) (.seq (.arrow the_primitive_type_called_are_l8 the_primitive_type_called_are_s8 the_primitive_type_called_are_s8) (.seq (.arrow the_primitive_type_called_are_l8 the_primitive_type_called_are_s8 the_primitive_type_called_are_s9) (.arrow the_primitive_type_called_are_l9 the_primitive_type_called_are_s9 the_primitive_type_called_are_s10))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_are_true_arm : IGProtocol the_primitive_type_called_are_s0 the_primitive_type_called_are_s10 :=
  (the_primitive_type_called_are_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_are_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_are_s0
theorem the_primitive_type_called_are_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_are_s0 the_primitive_type_called_are_s0 = the_primitive_type_called_are_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_are_s0

#eval the_primitive_type_called_are_tier  -- the Grammar's own verdict on its tier

/-- 'are' belongs to the Topology family at ordinal 4 -/
def the_primitive_type_called_are_family : String := "Topology"
def the_primitive_type_called_are_ordinal : Nat := 4
def the_primitive_type_called_are_bootstrapSteps : Nat := 11

end Imscribing.Primitives.Types
