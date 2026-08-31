/-
  Imscribing.Primitives.Types.PrimitiveTypeHaHa

  Formalization of the primitive value type 'ha_ha'
  Family: Criticality (ordinal 4)
  Surface tokens: supercriticality, runaway, instability
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

/-- Surface tokens for 'ha_ha' -/
def the_primitive_type_called_ha_ha_surfaceTokens : List String := ["supercriticality", "runaway", "instability"]

/-- Boundary condition for 'ha_ha' -/
def the_primitive_type_called_ha_ha_boundaryCondition : String := "The runaway threshold of the criticality lattice"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_ha_ha_registerVoid : String := "pre-critical equilibrium"
def the_primitive_type_called_ha_ha_registerTrue : String := "supercritical runaway"
def the_primitive_type_called_ha_ha_registerFalse : String := "subcritical collapse"
def the_primitive_type_called_ha_ha_registerBoth : String := "chaotic turbulence"

/-- Opcode domain-semantic mapping -/
-- AFWD: divergent scaling
--   The forward transformation toward an unstable fixed point.
-- AREV: damping feedback
--   The reverse attempt to pull the system back toward the stable woe phase.
-- CLINK: iterative amplification
--   The sequential chaining of gain cycles that drives the instability.
-- ENGAGR: metastable oscillation
--   A state held between growth and decay without reaching a fixed point.
-- EVALF: thermal death
--   The negative branch where the runaway leads to immediate exhaustion.
-- EVALT: unbounded growth
--   The affirmative branch where the runaway succeeds in escaping control.
-- FFUSE: total volatility
--   The reconstitution of the system state as a singular unstable vector.
-- FSPLIT: bifurcation event
--   The point where the system branches into exponential growth and catastrophic decay.
-- IFIX: critical failure log
--   The permanent record of the system exceeding the haha threshold.
-- IMSCRIB: self-reinforcing loop
--   The identity where the output of the runaway becomes its own input.
-- TANCH: system collapse
--   The terminal boundary where the runaway process exhausts all available resources.
-- VINIT: subcritical order
--   The stable phase before the injection of runaway energy.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_ha_ha_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ha_ha_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ha_ha_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_ha_ha_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_ha_ha_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_ha_ha_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_ha_ha_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_ha_ha_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ha_ha_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ha_ha_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_ha_ha_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ha_ha_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_ha_ha_protocol : IGProtocol the_primitive_type_called_ha_ha_s0 the_primitive_type_called_ha_ha_s13 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_ha_ha_l0 the_primitive_type_called_ha_ha_s0 the_primitive_type_called_ha_ha_s1) (.seq (.arrow the_primitive_type_called_ha_ha_l1 the_primitive_type_called_ha_ha_s1 the_primitive_type_called_ha_ha_s2) (.seq (.arrow the_primitive_type_called_ha_ha_l2 the_primitive_type_called_ha_ha_s2 the_primitive_type_called_ha_ha_s3) (.seq (.arrow the_primitive_type_called_ha_ha_l3 the_primitive_type_called_ha_ha_s3 the_primitive_type_called_ha_ha_s4) (.seq (.prod (.arrow the_primitive_type_called_ha_ha_l4 the_primitive_type_called_ha_ha_s4 the_primitive_type_called_ha_ha_s10) (.arrow the_primitive_type_called_ha_ha_l4 the_primitive_type_called_ha_ha_s4 the_primitive_type_called_ha_ha_s10)) (.seq (.arrow the_primitive_type_called_ha_ha_l10 the_primitive_type_called_ha_ha_s10 the_primitive_type_called_ha_ha_s10) (.seq (.arrow the_primitive_type_called_ha_ha_l10 the_primitive_type_called_ha_ha_s10 the_primitive_type_called_ha_ha_s11) (.seq (.arrow the_primitive_type_called_ha_ha_l11 the_primitive_type_called_ha_ha_s11 the_primitive_type_called_ha_ha_s12) (.arrow the_primitive_type_called_ha_ha_l12 the_primitive_type_called_ha_ha_s12 the_primitive_type_called_ha_ha_s13)))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_ha_ha_true_arm : IGProtocol the_primitive_type_called_ha_ha_s0 the_primitive_type_called_ha_ha_s13 :=
  (the_primitive_type_called_ha_ha_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_ha_ha_false_arm : IGProtocol the_primitive_type_called_ha_ha_s0 the_primitive_type_called_ha_ha_s13 :=
  (the_primitive_type_called_ha_ha_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_ha_ha_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_ha_ha_s0
theorem the_primitive_type_called_ha_ha_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_ha_ha_s0 the_primitive_type_called_ha_ha_s0 = the_primitive_type_called_ha_ha_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_ha_ha_s0

#eval the_primitive_type_called_ha_ha_tier  -- the Grammar's own verdict on its tier

/-- 'ha_ha' belongs to the Criticality family at ordinal 4 -/
def the_primitive_type_called_ha_ha_family : String := "Criticality"
def the_primitive_type_called_ha_ha_ordinal : Nat := 4
def the_primitive_type_called_ha_ha_bootstrapSteps : Nat := 14

end Imscribing.Primitives.Types
