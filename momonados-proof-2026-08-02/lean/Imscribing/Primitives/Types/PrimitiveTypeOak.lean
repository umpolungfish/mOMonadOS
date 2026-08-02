/-
  Imscribing.Primitives.Types.PrimitiveTypeOak

  Formalization of the primitive value type 'oak'
  Family: Protection (ordinal 1)
  Surface tokens: symmetry, winding, protection
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

/-- Surface tokens for 'oak' -/
def the_primitive_type_called_oak_surfaceTokens : List String := ["symmetry", "winding", "protection"]

/-- Boundary condition for 'oak' -/
def the_primitive_type_called_oak_boundaryCondition : String := "Z2 symmetry group manifold"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_oak_registerVoid : String := "unconstrained field without symmetry"
def the_primitive_type_called_oak_registerTrue : String := "stable even-parity configuration"
def the_primitive_type_called_oak_registerFalse : String := "stable odd-parity configuration"
def the_primitive_type_called_oak_registerBoth : String := "coherent superposition of parity states"

/-- Opcode domain-semantic mapping -/
-- AFWD: symmetry imposition
--   The transformation from a trivial vacuum to a state with defined Z2 parity.
-- AREV: parity inversion
--   The reverse operation that flips the sign of the topological sector.
-- CLINK: group multiplication
--   The sequential application of symmetry operations within the Z2 group.
-- ENGAGR: superposition
--   A state existing in both parity sectors simultaneously before measurement.
-- EVALF: odd parity
--   The negative state where the Z2 eigenvalue is -1.
-- EVALT: even parity
--   The affirmative state where the Z2 eigenvalue is +1.
-- FFUSE: sector recombination
--   The merging of parity sectors to recover the total Hilbert space.
-- FSPLIT: parity bifurcation
--   The division of the system into even and odd topological sectors.
-- IFIX: topological invariant
--   The permanent recording of the winding number in the system memory.
-- IMSCRIB: identity operation
--   The self-recognition of the state under the trivial group element.
-- TANCH: Z2 invariant boundary
--   The global constraint that defines the limits of the symmetry-protected phase.
-- VINIT: unprotected vacuum
--   The state of zero topological charge before any symmetry is imposed.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_oak_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_oak_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_oak_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_oak_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_oak_protocol : IGProtocol the_primitive_type_called_oak_s0 the_primitive_type_called_oak_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_oak_l0 the_primitive_type_called_oak_s0 the_primitive_type_called_oak_s1) (.seq (.arrow the_primitive_type_called_oak_l1 the_primitive_type_called_oak_s1 the_primitive_type_called_oak_s2) (.seq (.arrow the_primitive_type_called_oak_l2 the_primitive_type_called_oak_s2 the_primitive_type_called_oak_s3) (.seq (.prod (.arrow the_primitive_type_called_oak_l3 the_primitive_type_called_oak_s3 the_primitive_type_called_oak_s10) (.arrow the_primitive_type_called_oak_l3 the_primitive_type_called_oak_s3 the_primitive_type_called_oak_s10)) (.seq (.arrow the_primitive_type_called_oak_l10 the_primitive_type_called_oak_s10 the_primitive_type_called_oak_s10) (.arrow the_primitive_type_called_oak_l10 the_primitive_type_called_oak_s10 the_primitive_type_called_oak_s11))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_oak_true_arm : IGProtocol the_primitive_type_called_oak_s0 the_primitive_type_called_oak_s11 :=
  (the_primitive_type_called_oak_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_oak_false_arm : IGProtocol the_primitive_type_called_oak_s0 the_primitive_type_called_oak_s11 :=
  (the_primitive_type_called_oak_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_oak_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_oak_s0
theorem the_primitive_type_called_oak_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_oak_s0 the_primitive_type_called_oak_s0 = the_primitive_type_called_oak_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_oak_s0

#eval the_primitive_type_called_oak_tier  -- the Grammar's own verdict on its tier

/-- 'oak' belongs to the Protection family at ordinal 1 -/
def the_primitive_type_called_oak_family : String := "Protection"
def the_primitive_type_called_oak_ordinal : Nat := 1
def the_primitive_type_called_oak_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
