/-
  Imscribing.Primitives.Types.PrimitiveTypeOr

  Formalization of the primitive value type 'or'
  Family: Polarity (ordinal 1)
  Surface tokens: Frobenius, singularity, encoding
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

/-- Surface tokens for 'or' -/
def the_primitive_type_called_or_surfaceTokens : List String := ["Frobenius", "singularity", "encoding"]

/-- Boundary condition for 'or' -/
def the_primitive_type_called_or_boundaryCondition : String := "Axiom D Holographic Closure"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_or_registerVoid : String := "unpolarized state before symmetry breaking"
def the_primitive_type_called_or_registerTrue : String := "exact holographic correspondence"
def the_primitive_type_called_or_registerFalse : String := "symmetry mismatch or lossy encoding"
def the_primitive_type_called_or_registerBoth : String := "Frobenius singularity at the critical point"

/-- Opcode domain-semantic mapping -/
-- AFWD: encoding map δ
--   The forward morphism projecting bulk data onto the boundary.
-- AREV: decoding map μ
--   The reverse morphism reconstructing the bulk from the boundary projection.
-- CLINK: composition μ ∘ δ
--   The sequential chaining of the encoding and decoding operations.
-- ENGAGR: criticality ⊙
--   The paradice state where the system is simultaneously bulk and boundary at the phase transition.
-- EVALF: lossy projection
--   The negative state where information is lost during the dimensionality split.
-- EVALT: exact correspondence
--   The affirmative state where the boundary fully encodes the bulk.
-- FFUSE: bulk reconstruction
--   Reconstitutes the original bulk object from the boundary data and constraints.
-- FSPLIT: boundary projection
--   Splits the bulk object into its boundary representation and its internal constraints.
-- IFIX: or' fixation
--   The permanent fixation of the Frobenius property in the type record.
-- IMSCRIB: identity id
--   The self-recognition where the reconstructed bulk matches the original.
-- TANCH: O_inf tier
--   The terminal state of the grammar where the Frobenius condition is satisfied.
-- VINIT: untyped vacuum
--   The state before any polarity or symmetry assignment exists.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_or_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_or_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_primitive_type_called_or_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_or_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_or_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_or_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_or_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_or_protocol : IGProtocol the_primitive_type_called_or_s0 the_primitive_type_called_or_s10 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_or_l0 the_primitive_type_called_or_s0 the_primitive_type_called_or_s1) (.seq (.prod (.arrow the_primitive_type_called_or_l1 the_primitive_type_called_or_s1 the_primitive_type_called_or_s7) (.arrow the_primitive_type_called_or_l1 the_primitive_type_called_or_s1 the_primitive_type_called_or_s7)) (.seq (.arrow the_primitive_type_called_or_l7 the_primitive_type_called_or_s7 the_primitive_type_called_or_s7) (.seq (.arrow the_primitive_type_called_or_l7 the_primitive_type_called_or_s7 the_primitive_type_called_or_s8) (.seq (.arrow the_primitive_type_called_or_l8 the_primitive_type_called_or_s8 the_primitive_type_called_or_s9) (.arrow the_primitive_type_called_or_l9 the_primitive_type_called_or_s9 the_primitive_type_called_or_s10))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_or_true_arm : IGProtocol the_primitive_type_called_or_s0 the_primitive_type_called_or_s10 :=
  (the_primitive_type_called_or_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_or_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_or_s0
theorem the_primitive_type_called_or_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_or_s0 the_primitive_type_called_or_s0 = the_primitive_type_called_or_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_or_s0

#eval the_primitive_type_called_or_tier  -- the Grammar's own verdict on its tier

/-- 'or' belongs to the Polarity family at ordinal 1 -/
def the_primitive_type_called_or_family : String := "Polarity"
def the_primitive_type_called_or_ordinal : Nat := 4
def the_primitive_type_called_or_bootstrapSteps : Nat := 11

end Imscribing.Primitives.Types
