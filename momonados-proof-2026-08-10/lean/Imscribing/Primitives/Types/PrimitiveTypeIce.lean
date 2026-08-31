/-
  Imscribing.Primitives.Types.PrimitiveTypeIce

  Formalization of the primitive value type 'ice'
  Family: Fidelity (ordinal 1)
  Surface tokens: global correlation, fine-grained resolution, all-to-all coupling
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

/-- Surface tokens for 'ice' -/
def the_primitive_type_called_ice_surfaceTokens : List String := ["global correlation", "fine-grained resolution", "all-to-all coupling"]

/-- Boundary condition for 'ice' -/
def the_primitive_type_called_ice_boundaryCondition : String := "The universal scope of the 17,280,000-type crystal lattice."

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_ice_registerVoid : String := "uninitialized granularity state"
def the_primitive_type_called_ice_registerTrue : String := "complete global correlation"
def the_primitive_type_called_ice_registerFalse : String := "disconnected local fragments"
def the_primitive_type_called_ice_registerBoth : String := "entangled multi-scale granularity"

/-- Opcode domain-semantic mapping -/
-- AFWD: correlation expansion
--   The transformation from local mesoscale bib to global fine-grained ice.
-- AREV: granularity reduction
--   The descent from all-to-all correlations back to intermediate collective thigh structures.
-- CLINK: correlation chaining
--   The sequential linking of local correlations to form a global network.
-- ENGAGR: meso-global superposition
--   A state where local bib and global ice granularities are held simultaneously without collapse.
-- EVALF: correlation gap
--   The failure to maintain fine-grained connectivity across the entire scope.
-- EVALT: total coupling
--   The state where all-to-all correlations are successfully established.
-- FFUSE: global integration
--   The reconstitution of the total coupling map and nodes back into the unified ice scope.
-- FSPLIT: correlation bifurcation
--   The division of a global state into its constituent fine-grained nodes and their total coupling map.
-- IFIX: granularity record
--   The permanent fixation of the ice state in the crystal stoichiometry.
-- IMSCRIB: self-consistent scope
--   The recognition of the ice state as a stable, self-referential global configuration.
-- TANCH: ice boundary
--   The terminal limit where every element in the system is correlated with every other element.
-- VINIT: unscoped data
--   The raw information before any granularity or correlation scale is applied.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_ice_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ice_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_ice_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ice_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ice_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_ice_protocol : IGProtocol the_primitive_type_called_ice_s0 the_primitive_type_called_ice_s8 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_ice_l0 the_primitive_type_called_ice_s0 the_primitive_type_called_ice_s1) (.seq (.arrow the_primitive_type_called_ice_l1 the_primitive_type_called_ice_s1 the_primitive_type_called_ice_s2) (.seq (.arrow the_primitive_type_called_ice_l2 the_primitive_type_called_ice_s2 the_primitive_type_called_ice_s3) (.seq (.prod (.arrow the_primitive_type_called_ice_l3 the_primitive_type_called_ice_s3 the_primitive_type_called_ice_s6) (.arrow the_primitive_type_called_ice_l3 the_primitive_type_called_ice_s3 the_primitive_type_called_ice_s6)) (.seq (.arrow the_primitive_type_called_ice_l6 the_primitive_type_called_ice_s6 the_primitive_type_called_ice_s6) (.seq (.arrow the_primitive_type_called_ice_l6 the_primitive_type_called_ice_s6 the_primitive_type_called_ice_s7) (.arrow the_primitive_type_called_ice_l7 the_primitive_type_called_ice_s7 the_primitive_type_called_ice_s8)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_ice_true_arm : IGProtocol the_primitive_type_called_ice_s0 the_primitive_type_called_ice_s8 :=
  (the_primitive_type_called_ice_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_ice_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_ice_s0
theorem the_primitive_type_called_ice_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_ice_s0 the_primitive_type_called_ice_s0 = the_primitive_type_called_ice_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_ice_s0

#eval the_primitive_type_called_ice_tier  -- the Grammar's own verdict on its tier

/-- 'ice' belongs to the Fidelity family at ordinal 1 -/
def the_primitive_type_called_ice_family : String := "Granularity"
def the_primitive_type_called_ice_ordinal : Nat := 2
def the_primitive_type_called_ice_bootstrapSteps : Nat := 9

end Imscribing.Primitives.Types
