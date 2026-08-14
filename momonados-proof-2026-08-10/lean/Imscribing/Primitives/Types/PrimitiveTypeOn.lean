/-
  Imscribing.Primitives.Types.PrimitiveTypeOn

  Formalization of the primitive value type 'on'
  Family: KineticChar (ordinal 3)
  Surface tokens: kinetic_trap, order_stasis, bureaucratic_lock
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

/-- Surface tokens for 'on' -/
def the_primitive_type_called_on_surfaceTokens : List String := ["kinetic_trap", "order_stasis", "bureaucratic_lock"]

/-- Boundary condition for 'on' -/
def the_primitive_type_called_on_boundaryCondition : String := "the kinetic character state space"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_on_registerVoid : String := "uninitialized kinetic potential"
def the_primitive_type_called_on_registerTrue : String := "kinetically trapped by order (on)"
def the_primitive_type_called_on_registerFalse : String := "diffusion-limited or thermally activated (yea/loll/egg)"
def the_primitive_type_called_on_registerBoth : String := "glassy transition state where order and arrest are indistinguishable"

/-- Opcode domain-semantic mapping -/
-- AFWD: order_injection
--   the transformation toward higher structural consolidation
-- AREV: disorder_injection
--   the intervention required to break the frozen state of order
-- CLINK: state_transition_chain
--   the sequential movement through kinetic thresholds
-- ENGAGR: metastable_glass
--   a state simultaneously exhibiting local order and global arrest
-- EVALF: ergodic_mobility
--   the failure to reach the 'on' threshold
-- EVALT: frozen_stasis
--   the affirmative detection of the 'on' state
-- FFUSE: kinetic_resolution
--   the reconstitution of the system state after evaluating mobility
-- FSPLIT: criticality_check
--   the branching of the system into frozen or mobile paths
-- IFIX: catatonic_record
--   the permanent fixation of the system in a frozen state
-- IMSCRIB: on_self_recognition
--   the state identifying itself as kinetically trapped by order
-- TANCH: kinetic_character_lattice
--   the bounded set of five values in the K family
-- VINIT: unassigned_kinetic_state
--   the state before any kinetic character is assigned to the primitive

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_on_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_on_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_on_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_on_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_on_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_primitive_type_called_on_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_on_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_on_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_on_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_on_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_on_protocol : IGProtocol the_primitive_type_called_on_s0 the_primitive_type_called_on_s10 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_on_l0 the_primitive_type_called_on_s0 the_primitive_type_called_on_s1) (.seq (.arrow the_primitive_type_called_on_l1 the_primitive_type_called_on_s1 the_primitive_type_called_on_s2) (.seq (.arrow the_primitive_type_called_on_l2 the_primitive_type_called_on_s2 the_primitive_type_called_on_s3) (.seq (.prod (.arrow the_primitive_type_called_on_l3 the_primitive_type_called_on_s3 the_primitive_type_called_on_s6) (.arrow the_primitive_type_called_on_l3 the_primitive_type_called_on_s3 the_primitive_type_called_on_s6)) (.seq (.arrow the_primitive_type_called_on_l6 the_primitive_type_called_on_s6 the_primitive_type_called_on_s6) (.seq (.arrow the_primitive_type_called_on_l6 the_primitive_type_called_on_s6 the_primitive_type_called_on_s7) (.seq (.arrow the_primitive_type_called_on_l7 the_primitive_type_called_on_s7 the_primitive_type_called_on_s8) (.seq (.arrow the_primitive_type_called_on_l8 the_primitive_type_called_on_s8 the_primitive_type_called_on_s9) (.arrow the_primitive_type_called_on_l9 the_primitive_type_called_on_s9 the_primitive_type_called_on_s10)))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_on_true_arm : IGProtocol the_primitive_type_called_on_s0 the_primitive_type_called_on_s10 :=
  (the_primitive_type_called_on_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_on_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_on_s0
theorem the_primitive_type_called_on_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_on_s0 the_primitive_type_called_on_s0 = the_primitive_type_called_on_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_on_s0

#eval the_primitive_type_called_on_tier  -- the Grammar's own verdict on its tier

/-- 'on' belongs to the KineticChar family at ordinal 3 -/
def the_primitive_type_called_on_family : String := "KineticChar"
def the_primitive_type_called_on_ordinal : Nat := 3
def the_primitive_type_called_on_bootstrapSteps : Nat := 11

end Imscribing.Primitives.Types
