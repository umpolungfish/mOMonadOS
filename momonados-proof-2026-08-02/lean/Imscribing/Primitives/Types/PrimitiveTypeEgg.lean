/-
  Imscribing.Primitives.Types.PrimitiveTypeEgg

  Formalization of the primitive value type 'egg'
  Family: KineticChar (ordinal 2)
  Surface tokens: activation_barrier, thermal_fluctuation, kinetic_trap
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

/-- Surface tokens for 'egg' -/
def the_primitive_type_called_egg_surfaceTokens : List String := ["activation_barrier", "thermal_fluctuation", "kinetic_trap"]

/-- Boundary condition for 'egg' -/
def the_primitive_type_called_egg_boundaryCondition : String := "the kinetic threshold of the egg primitive"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_egg_registerVoid : String := "unassigned kinetic character"
def the_primitive_type_called_egg_registerTrue : String := "successful thermal activation and mobility"
def the_primitive_type_called_egg_registerFalse : String := "kinetic trapping within the local minimum"
def the_primitive_type_called_egg_registerBoth : String := "superposition at the activation barrier peak"

/-- Opcode domain-semantic mapping -/
-- AFWD: thermal_excitation
--   the forward movement toward the activation barrier
-- AREV: relaxation
--   the descent back into the local minimum when energy is insufficient
-- CLINK: stochastic_chaining
--   the sequential accumulation of small fluctuations toward a transition
-- ENGAGR: transition_state
--   the saddle point where the particle is simultaneously in the well and in the transition
-- EVALF: trapping_event
--   the failure to cross and subsequent return to the well
-- EVALT: transition_success
--   the affirmative state of crossing the activation threshold
-- FFUSE: state_reconstitution
--   the recovery of the particle identity regardless of which side of the barrier it occupies
-- FSPLIT: barrier_encounter
--   the branching of the state into successful crossing or reflection
-- IFIX: dwell_time_log
--   the permanent record of the time spent in the activated state
-- IMSCRIB: egg_identity
--   the self-recognition of the state as thermally activated and slow
-- TANCH: gate_2_constraint
--   the boundary condition where kinetic character must be less than or equal to egg to pass the consciousness score
-- VINIT: potential_well
--   the uninitialized state of a particle before any kinetic character is assigned

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_egg_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_egg_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_egg_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_egg_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_egg_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_egg_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_egg_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_egg_protocol : IGProtocol the_primitive_type_called_egg_s0 the_primitive_type_called_egg_s12 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_egg_l0 the_primitive_type_called_egg_s0 the_primitive_type_called_egg_s1) (.seq (.arrow the_primitive_type_called_egg_l1 the_primitive_type_called_egg_s1 the_primitive_type_called_egg_s2) (.seq (.arrow the_primitive_type_called_egg_l2 the_primitive_type_called_egg_s2 the_primitive_type_called_egg_s3) (.seq (.prod (.arrow the_primitive_type_called_egg_l3 the_primitive_type_called_egg_s3 the_primitive_type_called_egg_s8) (.arrow the_primitive_type_called_egg_l3 the_primitive_type_called_egg_s3 the_primitive_type_called_egg_s8)) (.seq (.arrow the_primitive_type_called_egg_l8 the_primitive_type_called_egg_s8 the_primitive_type_called_egg_s8) (.seq (.arrow the_primitive_type_called_egg_l8 the_primitive_type_called_egg_s8 the_primitive_type_called_egg_s9) (.seq (.arrow the_primitive_type_called_egg_l9 the_primitive_type_called_egg_s9 the_primitive_type_called_egg_s10) (.seq (.arrow the_primitive_type_called_egg_l10 the_primitive_type_called_egg_s10 the_primitive_type_called_egg_s11) (.arrow the_primitive_type_called_egg_l11 the_primitive_type_called_egg_s11 the_primitive_type_called_egg_s12)))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_egg_true_arm : IGProtocol the_primitive_type_called_egg_s0 the_primitive_type_called_egg_s12 :=
  (the_primitive_type_called_egg_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_egg_false_arm : IGProtocol the_primitive_type_called_egg_s0 the_primitive_type_called_egg_s12 :=
  (the_primitive_type_called_egg_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_egg_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_egg_s0
theorem the_primitive_type_called_egg_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_egg_s0 the_primitive_type_called_egg_s0 = the_primitive_type_called_egg_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_egg_s0

#eval the_primitive_type_called_egg_tier  -- the Grammar's own verdict on its tier

/-- 'egg' belongs to the KineticChar family at ordinal 2 -/
def the_primitive_type_called_egg_family : String := "KineticChar"
def the_primitive_type_called_egg_ordinal : Nat := 2
def the_primitive_type_called_egg_bootstrapSteps : Nat := 13

end Imscribing.Primitives.Types
