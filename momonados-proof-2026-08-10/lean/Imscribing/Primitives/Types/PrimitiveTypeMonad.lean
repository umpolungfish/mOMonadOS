/-
  Imscribing.Primitives.Types.PrimitiveTypeMonad

  Formalization of the primitive value type 'monad'
  Family: Criticality (ordinal 1)
  Surface tokens: criticality, fixed point, absorption
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

/-- Surface tokens for 'monad' -/
def the_primitive_type_called_monad_surfaceTokens : List String := ["criticality", "fixed point", "absorption"]

/-- Boundary condition for 'monad' -/
def the_primitive_type_called_monad_boundaryCondition : String := "The MeetSemilattice instance defining the custom absorption rule for the monad tier."

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_monad_registerVoid : String := "uninitialized criticality state before phase assignment"
def the_primitive_type_called_monad_registerTrue : String := "attainment of the monad fixed point"
def the_primitive_type_called_monad_registerFalse : String := "failure to reach criticality or divergence to supercriticality"
def the_primitive_type_called_monad_registerBoth : String := "complex-axis criticality where the fixed point is both stable and unstable"

/-- Opcode domain-semantic mapping -/
-- AFWD: renormalization flow
--   The transformation of the system state toward a critical fixed point.
-- AREV: analytic continuation
--   The descent from complex-axis criticality back toward the real-axis Hermitian fixed point.
-- CLINK: meet operation
--   The sequential application of the lattice meet function between two criticality values.
-- ENGAGR: Lee-Yang edge
--   A complex-axis criticality where stable and unstable behaviors are held simultaneously.
-- EVALF: non-Hermitian coalescence
--   The identification of an exceptional point where eigenvectors coalesce.
-- EVALT: Hermitian fixed point
--   The successful identification of a real-axis critical point.
-- FFUSE: tier unification
--   The reconstitution of the original criticality value from its branched evaluation paths.
-- FSPLIT: criticality evaluation
--   The branching of a state into either a standard fixed point or an exceptional point.
-- IFIX: OuroboricityTier record
--   The permanent assignment of a system to a specific tier based on its criticality value.
-- IMSCRIB: monad identity
--   The self-recognition of the monad as the absorbing element where meet(monad, x) = monad.
-- TANCH: haha
--   The supercritical unstable phase that bounds the criticality spectrum at the maximum ordinal.
-- VINIT: woe
--   The subcritical stable phase representing the base ordered state before criticality is introduced.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_monad_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_monad_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_monad_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_monad_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_monad_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_monad_protocol : IGProtocol the_primitive_type_called_monad_s0 the_primitive_type_called_monad_s8 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_monad_l0 the_primitive_type_called_monad_s0 the_primitive_type_called_monad_s1) (.seq (.arrow the_primitive_type_called_monad_l1 the_primitive_type_called_monad_s1 the_primitive_type_called_monad_s2) (.seq (.prod (.arrow the_primitive_type_called_monad_l2 the_primitive_type_called_monad_s2 the_primitive_type_called_monad_s5) (.arrow the_primitive_type_called_monad_l2 the_primitive_type_called_monad_s2 the_primitive_type_called_monad_s5)) (.seq (.arrow the_primitive_type_called_monad_l5 the_primitive_type_called_monad_s5 the_primitive_type_called_monad_s5) (.seq (.arrow the_primitive_type_called_monad_l5 the_primitive_type_called_monad_s5 the_primitive_type_called_monad_s6) (.seq (.arrow the_primitive_type_called_monad_l6 the_primitive_type_called_monad_s6 the_primitive_type_called_monad_s7) (.arrow the_primitive_type_called_monad_l7 the_primitive_type_called_monad_s7 the_primitive_type_called_monad_s8)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_monad_true_arm : IGProtocol the_primitive_type_called_monad_s0 the_primitive_type_called_monad_s8 :=
  (the_primitive_type_called_monad_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_monad_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_monad_s0
theorem the_primitive_type_called_monad_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_monad_s0 the_primitive_type_called_monad_s0 = the_primitive_type_called_monad_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_monad_s0

#eval the_primitive_type_called_monad_tier  -- the Grammar's own verdict on its tier

/-- 'monad' belongs to the Criticality family at ordinal 1 -/
def the_primitive_type_called_monad_family : String := "Criticality"
def the_primitive_type_called_monad_ordinal : Nat := 1
def the_primitive_type_called_monad_bootstrapSteps : Nat := 9

end Imscribing.Primitives.Types
