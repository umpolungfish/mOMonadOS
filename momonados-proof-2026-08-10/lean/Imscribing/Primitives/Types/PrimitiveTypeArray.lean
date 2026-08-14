/-
  Imscribing.Primitives.Types.PrimitiveTypeArray

  Formalization of the primitive value type 'array'
  Family: Dimensionality (ordinal 2)
  Surface tokens: unbounded_tensor, temporal_lattice, infinite_dimension
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

/-- Surface tokens for 'array' -/
def the_primitive_type_called_array_surfaceTokens : List String := ["unbounded_tensor", "temporal_lattice", "infinite_dimension"]

/-- Boundary condition for 'array' -/
def the_primitive_type_called_array_boundaryCondition : String := "the infinite-dimensional manifold of the array primitive"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_array_registerVoid : String := "uninitialized coordinate space"
def the_primitive_type_called_array_registerTrue : String := "resolved and addressable index"
def the_primitive_type_called_array_registerFalse : String := "unreachable or divergent dimension"
def the_primitive_type_called_array_registerBoth : String := "entangled indices in a non-local state"

/-- Opcode domain-semantic mapping -/
-- AFWD: axis extension
--   the forward generation of a new dimension or index
-- AREV: index contraction
--   the reduction or projection of a dimension back toward the origin
-- CLINK: tensor product
--   the sequential composition of dimensional spaces into a higher-order structure
-- ENGAGR: superposition
--   the simultaneous occupation of multiple dimensional indices without resolution
-- EVALF: divergence
--   the failure to resolve a coordinate within the unbounded generation
-- EVALT: convergence
--   the successful mapping of a value within the infinite-dimensional space
-- FFUSE: coordinate integration
--   the reconstitution of the original coordinate from its orthogonal components
-- FSPLIT: dimensional bifurcation
--   the branching of a single coordinate into independent orthogonal paths
-- IFIX: index fixation
--   the permanent assignment of a value to a specific coordinate in the array
-- IMSCRIB: self-similar cell
--   the identity of a single unit within the infinite array
-- TANCH: unbounded manifold limit
--   the global container for an infinite-dimensional generation
-- VINIT: pre-dimensional vacuum
--   the state before any spatial or temporal axes are defined

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_array_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_array_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_array_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_array_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_array_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_array_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_array_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_array_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_array_protocol : IGProtocol the_primitive_type_called_array_s0 the_primitive_type_called_array_s13 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_array_l0 the_primitive_type_called_array_s0 the_primitive_type_called_array_s1) (.seq (.arrow the_primitive_type_called_array_l1 the_primitive_type_called_array_s1 the_primitive_type_called_array_s2) (.seq (.arrow the_primitive_type_called_array_l2 the_primitive_type_called_array_s2 the_primitive_type_called_array_s3) (.seq (.arrow the_primitive_type_called_array_l3 the_primitive_type_called_array_s3 the_primitive_type_called_array_s4) (.seq (.prod (.arrow the_primitive_type_called_array_l4 the_primitive_type_called_array_s4 the_primitive_type_called_array_s10) (.arrow the_primitive_type_called_array_l4 the_primitive_type_called_array_s4 the_primitive_type_called_array_s10)) (.seq (.arrow the_primitive_type_called_array_l10 the_primitive_type_called_array_s10 the_primitive_type_called_array_s10) (.seq (.arrow the_primitive_type_called_array_l10 the_primitive_type_called_array_s10 the_primitive_type_called_array_s11) (.seq (.arrow the_primitive_type_called_array_l11 the_primitive_type_called_array_s11 the_primitive_type_called_array_s12) (.arrow the_primitive_type_called_array_l12 the_primitive_type_called_array_s12 the_primitive_type_called_array_s13)))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_array_true_arm : IGProtocol the_primitive_type_called_array_s0 the_primitive_type_called_array_s13 :=
  (the_primitive_type_called_array_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_array_false_arm : IGProtocol the_primitive_type_called_array_s0 the_primitive_type_called_array_s13 :=
  (the_primitive_type_called_array_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_array_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_array_s0
theorem the_primitive_type_called_array_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_array_s0 the_primitive_type_called_array_s0 = the_primitive_type_called_array_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_array_s0

#eval the_primitive_type_called_array_tier  -- the Grammar's own verdict on its tier

/-- 'array' belongs to the Dimensionality family at ordinal 2 -/
def the_primitive_type_called_array_family : String := "Dimensionality"
def the_primitive_type_called_array_ordinal : Nat := 2
def the_primitive_type_called_array_bootstrapSteps : Nat := 14

end Imscribing.Primitives.Types
