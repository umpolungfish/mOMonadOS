/-
  Imscribing.Primitives.Types.PrimitiveTypeAdo

  Formalization of the primitive value type 'ado'
  Family: Relational (ordinal 0)
  Surface tokens: supervisor, subordinate, instruction
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

/-- Surface tokens for 'ado' -/
def the_primitive_type_called_ado_surfaceTokens : List String := ["supervisor", "subordinate", "instruction"]

/-- Boundary condition for 'ado' -/
def the_primitive_type_called_ado_boundaryCondition : String := "hierarchical relational mode"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_ado_registerVoid : String := "unstructured entity without rank"
def the_primitive_type_called_ado_registerTrue : String := "successful command execution and reporting"
def the_primitive_type_called_ado_registerFalse : String := "command failure or refusal"
def the_primitive_type_called_ado_registerBoth : String := "active delegation where authority is shared and contested"

/-- Opcode domain-semantic mapping -/
-- AFWD: command issuance
--   the directed flow of authority from supervisor to subordinate
-- AREV: status report
--   the return of information from subordinate to supervisor
-- CLINK: delegation chain
--   the sequential composition of multiple supervisory levels
-- ENGAGR: delegated autonomy
--   a state where the subordinate is simultaneously acting and deciding independently
-- EVALF: insubordination
--   the negative state where the command is rejected or fails
-- EVALT: compliance
--   the affirmative state where the subordinate executes the command
-- FFUSE: task completion
--   the reunification of execution results and monitoring data into a single report
-- FSPLIT: instruction branching
--   a single command splitting into a task execution path and a monitoring path
-- IFIX: audit log
--   the permanent record of the supervisory transaction
-- IMSCRIB: role recognition
--   the internal identification of an entity as a supervisor or subordinate
-- TANCH: organizational closure
--   the boundary of the hierarchy that contains all supervisory links
-- VINIT: unassigned role
--   the state before a supervisory relationship is established

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_ado_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ado_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_ado_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ado_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ado_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_ado_protocol : IGProtocol the_primitive_type_called_ado_s0 the_primitive_type_called_ado_s9 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_ado_l0 the_primitive_type_called_ado_s0 the_primitive_type_called_ado_s1) (.seq (.arrow the_primitive_type_called_ado_l1 the_primitive_type_called_ado_s1 the_primitive_type_called_ado_s2) (.seq (.arrow the_primitive_type_called_ado_l2 the_primitive_type_called_ado_s2 the_primitive_type_called_ado_s3) (.seq (.prod (.arrow the_primitive_type_called_ado_l3 the_primitive_type_called_ado_s3 the_primitive_type_called_ado_s6) (.arrow the_primitive_type_called_ado_l3 the_primitive_type_called_ado_s3 the_primitive_type_called_ado_s6)) (.seq (.arrow the_primitive_type_called_ado_l6 the_primitive_type_called_ado_s6 the_primitive_type_called_ado_s6) (.seq (.arrow the_primitive_type_called_ado_l6 the_primitive_type_called_ado_s6 the_primitive_type_called_ado_s7) (.seq (.arrow the_primitive_type_called_ado_l7 the_primitive_type_called_ado_s7 the_primitive_type_called_ado_s8) (.arrow the_primitive_type_called_ado_l8 the_primitive_type_called_ado_s8 the_primitive_type_called_ado_s9))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_ado_true_arm : IGProtocol the_primitive_type_called_ado_s0 the_primitive_type_called_ado_s9 :=
  (the_primitive_type_called_ado_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_ado_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_ado_s0
theorem the_primitive_type_called_ado_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_ado_s0 the_primitive_type_called_ado_s0 = the_primitive_type_called_ado_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_ado_s0

#eval the_primitive_type_called_ado_tier  -- the Grammar's own verdict on its tier

/-- 'ado' belongs to the Relational family at ordinal 0 -/
def the_primitive_type_called_ado_family : String := "Relational"
def the_primitive_type_called_ado_ordinal : Nat := 0
def the_primitive_type_called_ado_bootstrapSteps : Nat := 10

end Imscribing.Primitives.Types
