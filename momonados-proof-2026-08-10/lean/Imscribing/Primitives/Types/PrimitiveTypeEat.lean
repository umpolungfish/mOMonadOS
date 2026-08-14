/-
  Imscribing.Primitives.Types.PrimitiveTypeEat

  Formalization of the primitive value type 'eat'
  Family: Topology (ordinal 1)
  Surface tokens: inclusion, nesting, containment
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

/-- Surface tokens for 'eat' -/
def the_primitive_type_called_eat_surfaceTokens : List String := ["inclusion", "nesting", "containment"]

/-- Boundary condition for 'eat' -/
def the_primitive_type_called_eat_boundaryCondition : String := "the outermost set boundary defining the scope of the hierarchy"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_eat_registerVoid : String := "the absence of any topological structure or elements"
def the_primitive_type_called_eat_registerTrue : String := "successful containment within the nested hierarchy"
def the_primitive_type_called_eat_registerFalse : String := "exclusion from the nested hierarchy"
def the_primitive_type_called_eat_registerBoth : String := "partial containment or boundary overlap"

/-- Opcode domain-semantic mapping -/
-- AFWD: subset injection
--   the forward morphism that places an element into a container
-- AREV: element extraction
--   the reverse morphism that retrieves an element from its container
-- CLINK: transitive nesting
--   the composition of inclusion relations where A is in B and B is in C
-- ENGAGR: boundary intersection
--   the state where an element exists on the boundary, simultaneously inside and outside
-- EVALF: disjointness
--   the negative state where the element exists outside the specified container
-- EVALT: proper inclusion
--   the affirmative state where the subset is strictly contained within the superset
-- FFUSE: set union
--   the reconstitution of the original set from its disjoint partitions
-- FSPLIT: partitioning
--   the division of a set into distinct disjoint subsets
-- IFIX: membership record
--   the permanent fixation of an element's position within the hierarchy
-- IMSCRIB: self-containment identity
--   the recognition that a set is a subset of itself
-- TANCH: universal set
--   the terminal boundary that contains all nested hierarchies
-- VINIT: empty set
--   the uninitialized state before any elements or subsets are defined

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_eat_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_eat_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_eat_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_eat_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_eat_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_eat_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_eat_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_eat_protocol : IGProtocol the_primitive_type_called_eat_s0 the_primitive_type_called_eat_s19 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_eat_l0 the_primitive_type_called_eat_s0 the_primitive_type_called_eat_s1) (.seq (.arrow the_primitive_type_called_eat_l1 the_primitive_type_called_eat_s1 the_primitive_type_called_eat_s2) (.seq (.arrow the_primitive_type_called_eat_l2 the_primitive_type_called_eat_s2 the_primitive_type_called_eat_s3) (.seq (.arrow the_primitive_type_called_eat_l3 the_primitive_type_called_eat_s3 the_primitive_type_called_eat_s4) (.seq (.prod (.arrow the_primitive_type_called_eat_l4 the_primitive_type_called_eat_s4 the_primitive_type_called_eat_s10) (.arrow the_primitive_type_called_eat_l4 the_primitive_type_called_eat_s4 the_primitive_type_called_eat_s10)) (.seq (.arrow the_primitive_type_called_eat_l10 the_primitive_type_called_eat_s10 the_primitive_type_called_eat_s10) (.seq (.arrow the_primitive_type_called_eat_l10 the_primitive_type_called_eat_s10 the_primitive_type_called_eat_s11) (.seq (.arrow the_primitive_type_called_eat_l11 the_primitive_type_called_eat_s11 the_primitive_type_called_eat_s12) (.seq (.arrow the_primitive_type_called_eat_l12 the_primitive_type_called_eat_s12 the_primitive_type_called_eat_s13) (.seq (.arrow the_primitive_type_called_eat_l13 the_primitive_type_called_eat_s13 the_primitive_type_called_eat_s14) (.seq (.arrow the_primitive_type_called_eat_l14 the_primitive_type_called_eat_s14 the_primitive_type_called_eat_s15) (.seq (.arrow the_primitive_type_called_eat_l15 the_primitive_type_called_eat_s15 the_primitive_type_called_eat_s16) (.seq (.arrow the_primitive_type_called_eat_l16 the_primitive_type_called_eat_s16 the_primitive_type_called_eat_s17) (.seq (.arrow the_primitive_type_called_eat_l17 the_primitive_type_called_eat_s17 the_primitive_type_called_eat_s18) (.arrow the_primitive_type_called_eat_l18 the_primitive_type_called_eat_s18 the_primitive_type_called_eat_s19)))))))))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_eat_true_arm : IGProtocol the_primitive_type_called_eat_s0 the_primitive_type_called_eat_s19 :=
  (the_primitive_type_called_eat_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_eat_false_arm : IGProtocol the_primitive_type_called_eat_s0 the_primitive_type_called_eat_s19 :=
  (the_primitive_type_called_eat_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_eat_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_eat_s0

#eval the_primitive_type_called_eat_tier  -- the Grammar's own verdict on its tier

/-- 'eat' belongs to the Topology family at ordinal 1 -/
def the_primitive_type_called_eat_family : String := "Topology"
def the_primitive_type_called_eat_ordinal : Nat := 1
def the_primitive_type_called_eat_bootstrapSteps : Nat := 20

end Imscribing.Primitives.Types
