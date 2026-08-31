/-
  Imscribing.Primitives.Types.PrimitiveTypeLoll

  Formalization of the primitive value type 'loll'
  Family: KineticChar (ordinal 4)
  Surface tokens: viscosity, activation energy, kinetic threshold
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

/-- Surface tokens for 'loll' -/
def the_primitive_type_called_loll_surfaceTokens : List String := ["viscosity", "activation energy", "kinetic threshold"]

/-- Boundary condition for 'loll' -/
def the_primitive_type_called_loll_boundaryCondition : String := "Kinetic Character Lattice (K)"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_loll_registerVoid : String := "uninitialized kinetic state"
def the_primitive_type_called_loll_registerTrue : String := "active diffusion"
def the_primitive_type_called_loll_registerFalse : String := "sub-threshold stasis"
def the_primitive_type_called_loll_registerBoth : String := "critical fluctuation"

/-- Opcode domain-semantic mapping -/
-- AFWD: thermal activation
--   The forward movement toward overcoming the moderate threshold.
-- AREV: viscous relaxation
--   The return to a state of rest below the activation barrier.
-- CLINK: barrier coupling
--   The sequential chaining of small fluctuations into a crossing event.
-- ENGAGR: metastable state
--   The simultaneous presence of both potential and kinetic energy at the barrier peak.
-- EVALF: activation failure
--   The negative state where the system remains below the threshold.
-- EVALT: activation success
--   The affirmative state where the kinetic barrier is surpassed.
-- FFUSE: kinetic equilibrium
--   The reconstitution of the total probability density from both branches.
-- FSPLIT: threshold bifurcation
--   The split between successful barrier crossing and reflection.
-- IFIX: state transition log
--   The permanent record of a successful kinetic event.
-- IMSCRIB: loll identity
--   The self-recognition of the state as moderately constrained but not trapped.
-- TANCH: loll boundary
--   The specific ordinal rank 1 within the F5 kinetic family.
-- VINIT: potential energy surface
--   The unperturbed state before any kinetic character is assigned.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_loll_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_primitive_type_called_loll_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_loll_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_loll_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_loll_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_loll_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_loll_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_loll_protocol : IGProtocol the_primitive_type_called_loll_s0 the_primitive_type_called_loll_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_loll_l0 the_primitive_type_called_loll_s0 the_primitive_type_called_loll_s1) (.seq (.arrow the_primitive_type_called_loll_l1 the_primitive_type_called_loll_s1 the_primitive_type_called_loll_s2) (.seq (.arrow the_primitive_type_called_loll_l2 the_primitive_type_called_loll_s2 the_primitive_type_called_loll_s3) (.seq (.prod (.arrow the_primitive_type_called_loll_l3 the_primitive_type_called_loll_s3 the_primitive_type_called_loll_s8) (.arrow the_primitive_type_called_loll_l3 the_primitive_type_called_loll_s3 the_primitive_type_called_loll_s8)) (.seq (.arrow the_primitive_type_called_loll_l8 the_primitive_type_called_loll_s8 the_primitive_type_called_loll_s8) (.seq (.arrow the_primitive_type_called_loll_l8 the_primitive_type_called_loll_s8 the_primitive_type_called_loll_s9) (.seq (.arrow the_primitive_type_called_loll_l9 the_primitive_type_called_loll_s9 the_primitive_type_called_loll_s10) (.arrow the_primitive_type_called_loll_l10 the_primitive_type_called_loll_s10 the_primitive_type_called_loll_s11))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_loll_true_arm : IGProtocol the_primitive_type_called_loll_s0 the_primitive_type_called_loll_s11 :=
  (the_primitive_type_called_loll_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_loll_false_arm : IGProtocol the_primitive_type_called_loll_s0 the_primitive_type_called_loll_s11 :=
  (the_primitive_type_called_loll_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_loll_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_loll_s0
theorem the_primitive_type_called_loll_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_loll_s0 the_primitive_type_called_loll_s0 = the_primitive_type_called_loll_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_loll_s0

#eval the_primitive_type_called_loll_tier  -- the Grammar's own verdict on its tier

/-- 'loll' belongs to the KineticChar family at ordinal 4 -/
def the_primitive_type_called_loll_family : String := "KineticChar"
def the_primitive_type_called_loll_ordinal : Nat := 1
def the_primitive_type_called_loll_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
