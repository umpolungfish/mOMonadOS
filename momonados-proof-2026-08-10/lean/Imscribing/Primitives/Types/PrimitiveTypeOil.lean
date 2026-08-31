/-
  Imscribing.Primitives.Types.PrimitiveTypeOil

  Formalization of the primitive value type 'oil'
  Family: Topology (ordinal 3)
  Surface tokens: lattice, regularity, periodicity
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

/-- Surface tokens for 'oil' -/
def the_primitive_type_called_oil_surfaceTokens : List String := ["lattice", "regularity", "periodicity"]

/-- Boundary condition for 'oil' -/
def the_primitive_type_called_oil_boundaryCondition : String := "The periodic boundary conditions of the unit cell"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_oil_registerVoid : String := "a featureless continuum without coordinates"
def the_primitive_type_called_oil_registerTrue : String := "a valid lattice site coordinate"
def the_primitive_type_called_oil_registerFalse : String := "an invalid or off-lattice position"
def the_primitive_type_called_oil_registerBoth : String := "a boundary point shared by adjacent cells"

/-- Opcode domain-semantic mapping -/
-- AFWD: translation vector
--   The forward movement from one lattice site to the next adjacent site.
-- AREV: inverse translation
--   The return path from a site to its predecessor, maintaining lattice integrity.
-- CLINK: path concatenation
--   The sequential application of translation vectors to reach distant nodes.
-- ENGAGR: superposition of sites
--   A state where a coordinate exists in a transition between two adjacent cells.
-- EVALF: interstitial void
--   The negative state representing the space between defined lattice sites.
-- EVALT: site occupancy
--   The affirmative state where a lattice node is correctly identified and occupied.
-- FFUSE: coordinate synthesis
--   The recombination of orthogonal components to identify a specific point in the bulk.
-- FSPLIT: bifurcation of dimensions
--   The division of a single coordinate into distinct orthogonal axes of the lattice.
-- IFIX: lattice constant fixation
--   The permanent setting of the distance between nodes, defining the scale of the oil topology.
-- IMSCRIB: unit cell identity
--   The recognition of a single cell as the fundamental repeating unit of the oil topology.
-- TANCH: toroidal closure
--   The final boundary condition that wraps the lattice into a finite, closed manifold.
-- VINIT: unstructured space
--   The pre-topological state before any grid or lattice structure is defined.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_oil_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_oil_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oil_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oil_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_oil_protocol : IGProtocol the_primitive_type_called_oil_s0 the_primitive_type_called_oil_s12 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_oil_l0 the_primitive_type_called_oil_s0 the_primitive_type_called_oil_s1) (.seq (.arrow the_primitive_type_called_oil_l1 the_primitive_type_called_oil_s1 the_primitive_type_called_oil_s2) (.seq (.arrow the_primitive_type_called_oil_l2 the_primitive_type_called_oil_s2 the_primitive_type_called_oil_s3) (.seq (.prod (.arrow the_primitive_type_called_oil_l3 the_primitive_type_called_oil_s3 the_primitive_type_called_oil_s6) (.arrow the_primitive_type_called_oil_l3 the_primitive_type_called_oil_s3 the_primitive_type_called_oil_s6)) (.seq (.arrow the_primitive_type_called_oil_l6 the_primitive_type_called_oil_s6 the_primitive_type_called_oil_s6) (.seq (.arrow the_primitive_type_called_oil_l6 the_primitive_type_called_oil_s6 the_primitive_type_called_oil_s7) (.seq (.arrow the_primitive_type_called_oil_l7 the_primitive_type_called_oil_s7 the_primitive_type_called_oil_s8) (.seq (.arrow the_primitive_type_called_oil_l8 the_primitive_type_called_oil_s8 the_primitive_type_called_oil_s9) (.seq (.arrow the_primitive_type_called_oil_l9 the_primitive_type_called_oil_s9 the_primitive_type_called_oil_s10) (.seq (.arrow the_primitive_type_called_oil_l10 the_primitive_type_called_oil_s10 the_primitive_type_called_oil_s11) (.arrow the_primitive_type_called_oil_l11 the_primitive_type_called_oil_s11 the_primitive_type_called_oil_s12)))))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_oil_true_arm : IGProtocol the_primitive_type_called_oil_s0 the_primitive_type_called_oil_s12 :=
  (the_primitive_type_called_oil_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_oil_false_arm : IGProtocol the_primitive_type_called_oil_s0 the_primitive_type_called_oil_s12 :=
  (the_primitive_type_called_oil_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_oil_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_oil_s0

#eval the_primitive_type_called_oil_tier  -- the Grammar's own verdict on its tier

/-- 'oil' belongs to the Topology family at ordinal 3 -/
def the_primitive_type_called_oil_family : String := "Topology"
def the_primitive_type_called_oil_ordinal : Nat := 3
def the_primitive_type_called_oil_bootstrapSteps : Nat := 13

end Imscribing.Primitives.Types
