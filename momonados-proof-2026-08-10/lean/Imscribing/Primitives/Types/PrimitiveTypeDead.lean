/-
  Imscribing.Primitives.Types.PrimitiveTypeDead

  Formalization of the primitive value type 'dead'
  Family: Dimensionality (ordinal 0)
  Surface tokens: manifold, projection, sheet
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

/-- Surface tokens for 'dead' -/
def the_primitive_type_called_dead_surfaceTokens : List String := ["manifold", "projection", "sheet"]

/-- Boundary condition for 'dead' -/
def the_primitive_type_called_dead_boundaryCondition : String := "The 2D Euclidean plane constraint"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_dead_registerVoid : String := "unallocated memory before the 2D manifold is initialized"
def the_primitive_type_called_dead_registerTrue : String := "successful mapping within the local 2D sheet"
def the_primitive_type_called_dead_registerFalse : String := "mapping failure or boundary violation"
def the_primitive_type_called_dead_registerBoth : String := "ambiguous coordinate state held in the local register"

/-- Opcode domain-semantic mapping -/
-- AFWD: linear translation
--   Movement across the flat surface toward a coordinate target.
-- AREV: coordinate inversion
--   The reversal of a vector within the local plane.
-- CLINK: affine transformation
--   The sequential application of linear maps on the sheet.
-- ENGAGR: superposition of coordinates
--   A state where a point occupies two planar locations simultaneously before collapse.
-- EVALF: out-of-bounds error
--   The failure state where a coordinate exceeds the planar boundary.
-- EVALT: on-plane verification
--   Confirmation that the operation remains within the 2D local manifold.
-- FFUSE: path convergence
--   The meeting of two planar trajectories back into a single coordinate point.
-- FSPLIT: bifurcation of the path
--   A single trajectory on the sheet splitting into two distinct planar directions.
-- IFIX: coordinate fixation
--   The permanent marking of a point on the sheet, creating an immutable record.
-- IMSCRIB: local point identity
--   The recognition of a specific coordinate as itself without depth.
-- TANCH: planar boundary
--   The absolute limit of the 2D sheet which contains all local operations.
-- VINIT: unmapped coordinate space
--   The raw geometric potential before any dimensionality is assigned.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_dead_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_dead_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_dead_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_dead_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_dead_protocol : IGProtocol the_primitive_type_called_dead_s0 the_primitive_type_called_dead_s13 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_dead_l0 the_primitive_type_called_dead_s0 the_primitive_type_called_dead_s1) (.seq (.arrow the_primitive_type_called_dead_l1 the_primitive_type_called_dead_s1 the_primitive_type_called_dead_s2) (.seq (.arrow the_primitive_type_called_dead_l2 the_primitive_type_called_dead_s2 the_primitive_type_called_dead_s3) (.seq (.arrow the_primitive_type_called_dead_l3 the_primitive_type_called_dead_s3 the_primitive_type_called_dead_s4) (.seq (.prod (.arrow the_primitive_type_called_dead_l4 the_primitive_type_called_dead_s4 the_primitive_type_called_dead_s9) (.arrow the_primitive_type_called_dead_l4 the_primitive_type_called_dead_s4 the_primitive_type_called_dead_s9)) (.seq (.arrow the_primitive_type_called_dead_l9 the_primitive_type_called_dead_s9 the_primitive_type_called_dead_s9) (.seq (.arrow the_primitive_type_called_dead_l9 the_primitive_type_called_dead_s9 the_primitive_type_called_dead_s10) (.seq (.arrow the_primitive_type_called_dead_l10 the_primitive_type_called_dead_s10 the_primitive_type_called_dead_s11) (.seq (.arrow the_primitive_type_called_dead_l11 the_primitive_type_called_dead_s11 the_primitive_type_called_dead_s12) (.arrow the_primitive_type_called_dead_l12 the_primitive_type_called_dead_s12 the_primitive_type_called_dead_s13))))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_dead_true_arm : IGProtocol the_primitive_type_called_dead_s0 the_primitive_type_called_dead_s13 :=
  (the_primitive_type_called_dead_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_dead_false_arm : IGProtocol the_primitive_type_called_dead_s0 the_primitive_type_called_dead_s13 :=
  (the_primitive_type_called_dead_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_dead_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_dead_s0
theorem the_primitive_type_called_dead_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_dead_s0 the_primitive_type_called_dead_s0 = the_primitive_type_called_dead_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_dead_s0

#eval the_primitive_type_called_dead_tier  -- the Grammar's own verdict on its tier

/-- 'dead' belongs to the Dimensionality family at ordinal 0 -/
def the_primitive_type_called_dead_family : String := "Dimensionality"
def the_primitive_type_called_dead_ordinal : Nat := 0
def the_primitive_type_called_dead_bootstrapSteps : Nat := 14

end Imscribing.Primitives.Types
