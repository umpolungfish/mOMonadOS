/-
  Imscribing.Primitives.Types.PrimitiveTypeEar

  Formalization of the primitive value type 'ear'
  Family: Relational (ordinal 2)
  Surface tokens: adjoint, reciprocity, duality
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

/-- Surface tokens for 'ear' -/
def the_primitive_type_called_ear_surfaceTokens : List String := ["adjoint", "reciprocity", "duality"]

/-- Boundary condition for 'ear' -/
def the_primitive_type_called_ear_boundaryCondition : String := "The Relational Mode (R) type space within the Imscriptive Primitives Core"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_ear_registerVoid : String := "uninitialized relational mode"
def the_primitive_type_called_ear_registerTrue : String := "reciprocal consistency achieved"
def the_primitive_type_called_ear_registerFalse : String := "asymmetric relational breakdown"
def the_primitive_type_called_ear_registerBoth : String := "entangled adjoint pair"

/-- Opcode domain-semantic mapping -/
-- AFWD: morphism A
--   The primary directed transformation from source to target.
-- AREV: adjoint A†
--   The contravariant transformation that reverses the morphism direction.
-- CLINK: functorial composition
--   The chaining of relational mappings to form a coherent structure.
-- ENGAGR: biorthogonal state
--   The simultaneous holding of both A and A† as a single reciprocal unit.
-- EVALF: orthogonality failure
--   The failure of the adjoint to map back to the identity.
-- EVALT: unitary match
--   The successful verification of the reciprocal identity.
-- FFUSE: reciprocal unification
--   The reconstitution of the original relation from its forward and adjoint parts.
-- FSPLIT: dagger bifurcation
--   The separation of a single relation into its forward and adjoint components.
-- IFIX: type assignment
--   The permanent fixation of the 'ear' mode in the Lean kernel.
-- IMSCRIB: self-duality
--   The recognition of the relation as its own dual or identity.
-- TANCH: Relational.ear
--   The terminal definition of the bidirectional dagger mode within the F4 family.
-- VINIT: untyped relation
--   The state before a specific relational mode is assigned to the primitive.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_ear_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_primitive_type_called_ear_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_ear_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_ear_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ear_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_ear_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ear_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_ear_protocol : IGProtocol the_primitive_type_called_ear_s0 the_primitive_type_called_ear_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_ear_l0 the_primitive_type_called_ear_s0 the_primitive_type_called_ear_s1) (.seq (.arrow the_primitive_type_called_ear_l1 the_primitive_type_called_ear_s1 the_primitive_type_called_ear_s2) (.seq (.prod (.arrow the_primitive_type_called_ear_l2 the_primitive_type_called_ear_s2 the_primitive_type_called_ear_s7) (.arrow the_primitive_type_called_ear_l2 the_primitive_type_called_ear_s2 the_primitive_type_called_ear_s7)) (.seq (.arrow the_primitive_type_called_ear_l7 the_primitive_type_called_ear_s7 the_primitive_type_called_ear_s7) (.seq (.arrow the_primitive_type_called_ear_l7 the_primitive_type_called_ear_s7 the_primitive_type_called_ear_s8) (.seq (.arrow the_primitive_type_called_ear_l8 the_primitive_type_called_ear_s8 the_primitive_type_called_ear_s9) (.seq (.arrow the_primitive_type_called_ear_l9 the_primitive_type_called_ear_s9 the_primitive_type_called_ear_s10) (.arrow the_primitive_type_called_ear_l10 the_primitive_type_called_ear_s10 the_primitive_type_called_ear_s11))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_ear_true_arm : IGProtocol the_primitive_type_called_ear_s0 the_primitive_type_called_ear_s11 :=
  (the_primitive_type_called_ear_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_ear_false_arm : IGProtocol the_primitive_type_called_ear_s0 the_primitive_type_called_ear_s11 :=
  (the_primitive_type_called_ear_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_ear_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_ear_s0
theorem the_primitive_type_called_ear_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_ear_s0 the_primitive_type_called_ear_s0 = the_primitive_type_called_ear_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_ear_s0

#eval the_primitive_type_called_ear_tier  -- the Grammar's own verdict on its tier

/-- 'ear' belongs to the Relational family at ordinal 2 -/
def the_primitive_type_called_ear_family : String := "Relational"
def the_primitive_type_called_ear_ordinal : Nat := 2
def the_primitive_type_called_ear_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
