-- IGProtocol scaffold: VINIT → FSPLIT → AFWD → EVALT → IMSCRIB → CLINK → AREV → EVALF → FFUSE → IFIX → TANCH
-- Class: define cycle_evaluator; input Q4_graph; output set_of_As; process find_all_hamiltonian_cycles for each construct_potentials project_L compute_spectrum
-- Fingerprint: sig=(6,2,2,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=11
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 8)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [5] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [6] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [9] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def define_cycle_evaluator_input_q4_graph_2cd86e_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def define_cycle_evaluator_input_q4_graph_2cd86e_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def define_cycle_evaluator_input_q4_graph_2cd86e_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def define_cycle_evaluator_input_q4_graph_2cd86e_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def define_cycle_evaluator_input_q4_graph_2cd86e_protocol : IGProtocol define_cycle_evaluator_input_q4_graph_2cd86e_s0 define_cycle_evaluator_input_q4_graph_2cd86e_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct define_cycle_evaluator_input_q4_graph_2cd86e_s8 define_cycle_evaluator_input_q4_graph_2cd86e_s8 = define_cycle_evaluator_input_q4_graph_2cd86e_s8 (idempotent)
  (.seq (.arrow define_cycle_evaluator_input_q4_graph_2cd86e_l0 define_cycle_evaluator_input_q4_graph_2cd86e_s0 define_cycle_evaluator_input_q4_graph_2cd86e_s1) (.seq (.prod (.arrow define_cycle_evaluator_input_q4_graph_2cd86e_l1 define_cycle_evaluator_input_q4_graph_2cd86e_s1 define_cycle_evaluator_input_q4_graph_2cd86e_s8) (.arrow define_cycle_evaluator_input_q4_graph_2cd86e_l1 define_cycle_evaluator_input_q4_graph_2cd86e_s1 define_cycle_evaluator_input_q4_graph_2cd86e_s8)) (.seq (.arrow define_cycle_evaluator_input_q4_graph_2cd86e_l8 define_cycle_evaluator_input_q4_graph_2cd86e_s8 define_cycle_evaluator_input_q4_graph_2cd86e_s8) (.seq (.arrow define_cycle_evaluator_input_q4_graph_2cd86e_l8 define_cycle_evaluator_input_q4_graph_2cd86e_s8 define_cycle_evaluator_input_q4_graph_2cd86e_s9) (.arrow define_cycle_evaluator_input_q4_graph_2cd86e_l9 define_cycle_evaluator_input_q4_graph_2cd86e_s9 define_cycle_evaluator_input_q4_graph_2cd86e_s10)))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def define_cycle_evaluator_input_q4_graph_2cd86e_true_arm : IGProtocol define_cycle_evaluator_input_q4_graph_2cd86e_s0 define_cycle_evaluator_input_q4_graph_2cd86e_s10 :=
  (define_cycle_evaluator_input_q4_graph_2cd86e_protocol).restrictToEVALT

-- false arm
noncomputable def define_cycle_evaluator_input_q4_graph_2cd86e_false_arm : IGProtocol define_cycle_evaluator_input_q4_graph_2cd86e_s0 define_cycle_evaluator_input_q4_graph_2cd86e_s10 :=
  (define_cycle_evaluator_input_q4_graph_2cd86e_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def define_cycle_evaluator_input_q4_graph_2cd86e_tier : OuroboricityTier := TierFunctor.obj define_cycle_evaluator_input_q4_graph_2cd86e_s0
#eval define_cycle_evaluator_input_q4_graph_2cd86e_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem define_cycle_evaluator_input_q4_graph_2cd86e_frobenius :
    igFrobeniusAlg.mul define_cycle_evaluator_input_q4_graph_2cd86e_s0 define_cycle_evaluator_input_q4_graph_2cd86e_s0 = define_cycle_evaluator_input_q4_graph_2cd86e_s0 :=
  igFrobAlg_self_fusion define_cycle_evaluator_input_q4_graph_2cd86e_s0
