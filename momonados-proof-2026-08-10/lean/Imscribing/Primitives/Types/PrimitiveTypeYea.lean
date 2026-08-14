/-
  Imscribing.Primitives.Types.PrimitiveTypeYea

  Formalization of the primitive value type 'yea'
  Family: KineticChar (ordinal 0)
  Surface tokens: diffusion, unconstrained, ergodicity
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

/-- Surface tokens for 'yea' -/
def the_primitive_type_called_yea_surfaceTokens : List String := ["diffusion", "unconstrained", "ergodicity"]

/-- Boundary condition for 'yea' -/
def the_primitive_type_called_yea_boundaryCondition : String := "the kinetic phase space of the Imscribing Primitives"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_yea_registerVoid : String := "static vacuum before the introduction of kinetic energy"
def the_primitive_type_called_yea_registerTrue : String := "active diffusion where the particle successfully explores the volume"
def the_primitive_type_called_yea_registerFalse : String := "localized fluctuation that fails to result in net transport"
def the_primitive_type_called_yea_registerBoth : String := "quantum-like diffusion where all paths are explored simultaneously"

/-- Opcode domain-semantic mapping -/
-- AFWD: stochastic transition
--   the forward movement of a particle or state through the lattice
-- AREV: path reversal
--   the microscopic reversibility inherent in untrapped diffusion
-- CLINK: trajectory concatenation
--   the chaining of individual steps into a continuous path
-- ENGAGR: superposition of trajectories
--   the state of being in multiple locations simultaneously before measurement
-- EVALF: local return
--   the failure to escape the current local neighborhood
-- EVALT: unhindered displacement
--   the successful movement into a new region of phase space
-- FFUSE: path integration
--   the summation of all possible paths to recover the total probability density
-- FSPLIT: bifurcation of potential paths
--   the branching of a single state into multiple possible future positions
-- IFIX: trace record
--   the permanent logging of a specific path taken through the system
-- IMSCRIB: self-diffusion coefficient
--   the identity of the particle as it relates to its own displacement
-- TANCH: ergodic limit
--   the boundary of the phase space where all states are reachable
-- VINIT: unallocated kinetic potential
--   the state of a primitive before its kinetic character is assigned

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_yea_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_yea_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_yea_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_yea_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_yea_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_yea_protocol : IGProtocol the_primitive_type_called_yea_s0 the_primitive_type_called_yea_s12 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_yea_l0 the_primitive_type_called_yea_s0 the_primitive_type_called_yea_s1) (.seq (.arrow the_primitive_type_called_yea_l1 the_primitive_type_called_yea_s1 the_primitive_type_called_yea_s2) (.seq (.prod (.arrow the_primitive_type_called_yea_l2 the_primitive_type_called_yea_s2 the_primitive_type_called_yea_s5) (.arrow the_primitive_type_called_yea_l2 the_primitive_type_called_yea_s2 the_primitive_type_called_yea_s5)) (.seq (.arrow the_primitive_type_called_yea_l5 the_primitive_type_called_yea_s5 the_primitive_type_called_yea_s5) (.seq (.arrow the_primitive_type_called_yea_l5 the_primitive_type_called_yea_s5 the_primitive_type_called_yea_s6) (.seq (.arrow the_primitive_type_called_yea_l6 the_primitive_type_called_yea_s6 the_primitive_type_called_yea_s7) (.seq (.arrow the_primitive_type_called_yea_l7 the_primitive_type_called_yea_s7 the_primitive_type_called_yea_s8) (.seq (.arrow the_primitive_type_called_yea_l8 the_primitive_type_called_yea_s8 the_primitive_type_called_yea_s9) (.seq (.arrow the_primitive_type_called_yea_l9 the_primitive_type_called_yea_s9 the_primitive_type_called_yea_s10) (.seq (.arrow the_primitive_type_called_yea_l10 the_primitive_type_called_yea_s10 the_primitive_type_called_yea_s11) (.arrow the_primitive_type_called_yea_l11 the_primitive_type_called_yea_s11 the_primitive_type_called_yea_s12)))))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_yea_true_arm : IGProtocol the_primitive_type_called_yea_s0 the_primitive_type_called_yea_s12 :=
  (the_primitive_type_called_yea_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_yea_false_arm : IGProtocol the_primitive_type_called_yea_s0 the_primitive_type_called_yea_s12 :=
  (the_primitive_type_called_yea_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_yea_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_yea_s0

#eval the_primitive_type_called_yea_tier  -- the Grammar's own verdict on its tier

/-- 'yea' belongs to the KineticChar family at ordinal 0 -/
def the_primitive_type_called_yea_family : String := "KineticChar"
def the_primitive_type_called_yea_ordinal : Nat := 0
def the_primitive_type_called_yea_bootstrapSteps : Nat := 13

end Imscribing.Primitives.Types
