-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → AFWD → EVALT → CLINK → IFIX → AREV → EVALF → IFIX → FFUSE → ENGAGR → IFIX → CLINK → AFWD → TANCH
-- Class: descent operator that maps any integer perfect cuboid candidate a,b,c,d,e,f,g to a strictly smaller perfect cuboid candidate or proves no such candidate exists via infinite descent on the space diagonal g
-- Fingerprint: sig=(8,2,3,3)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=16
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 10)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑚  | identity — self-imscription
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [6] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [7] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [10] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [11] ENGAGR    stoi   := 𐑳               𐑙 → 𐑭  | engage paradox — B-state, both arms
--   [12] IFIX      prot   := 𐑭               𐑳 → 𐑱  | irreversible fixation — winding number
--   [13] CLINK     fid    := 𐑱               𐑭 → 𐑾  | composition — regime coherence
--   [14] AFWD      rel    := 𐑾               𐑱 → 𐑡  | forward morphism — bidirectional arrow
--   [15] TANCH     top    := 𐑡               𐑾 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def descent_operator_that_maps_any_integer_8d170b_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def descent_operator_that_maps_any_integer_8d170b_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def descent_operator_that_maps_any_integer_8d170b_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l14 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def descent_operator_that_maps_any_integer_8d170b_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def descent_operator_that_maps_any_integer_8d170b_protocol : IGProtocol descent_operator_that_maps_any_integer_8d170b_s0 descent_operator_that_maps_any_integer_8d170b_s15 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct descent_operator_that_maps_any_integer_8d170b_s10 descent_operator_that_maps_any_integer_8d170b_s10 = descent_operator_that_maps_any_integer_8d170b_s10 (idempotent)
  (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l0 descent_operator_that_maps_any_integer_8d170b_s0 descent_operator_that_maps_any_integer_8d170b_s1) (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l1 descent_operator_that_maps_any_integer_8d170b_s1 descent_operator_that_maps_any_integer_8d170b_s2) (.seq (.prod (.arrow descent_operator_that_maps_any_integer_8d170b_l2 descent_operator_that_maps_any_integer_8d170b_s2 descent_operator_that_maps_any_integer_8d170b_s10) (.arrow descent_operator_that_maps_any_integer_8d170b_l2 descent_operator_that_maps_any_integer_8d170b_s2 descent_operator_that_maps_any_integer_8d170b_s10)) (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l10 descent_operator_that_maps_any_integer_8d170b_s10 descent_operator_that_maps_any_integer_8d170b_s10) (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l10 descent_operator_that_maps_any_integer_8d170b_s10 descent_operator_that_maps_any_integer_8d170b_s11) (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l11 descent_operator_that_maps_any_integer_8d170b_s11 descent_operator_that_maps_any_integer_8d170b_s12) (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l12 descent_operator_that_maps_any_integer_8d170b_s12 descent_operator_that_maps_any_integer_8d170b_s13) (.seq (.arrow descent_operator_that_maps_any_integer_8d170b_l13 descent_operator_that_maps_any_integer_8d170b_s13 descent_operator_that_maps_any_integer_8d170b_s14) (.arrow descent_operator_that_maps_any_integer_8d170b_l14 descent_operator_that_maps_any_integer_8d170b_s14 descent_operator_that_maps_any_integer_8d170b_s15)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def descent_operator_that_maps_any_integer_8d170b_true_arm : IGProtocol descent_operator_that_maps_any_integer_8d170b_s0 descent_operator_that_maps_any_integer_8d170b_s15 :=
  (descent_operator_that_maps_any_integer_8d170b_protocol).restrictToEVALT

-- false arm
noncomputable def descent_operator_that_maps_any_integer_8d170b_false_arm : IGProtocol descent_operator_that_maps_any_integer_8d170b_s0 descent_operator_that_maps_any_integer_8d170b_s15 :=
  (descent_operator_that_maps_any_integer_8d170b_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def descent_operator_that_maps_any_integer_8d170b_tier : OuroboricityTier := TierFunctor.obj descent_operator_that_maps_any_integer_8d170b_s0
#eval descent_operator_that_maps_any_integer_8d170b_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem descent_operator_that_maps_any_integer_8d170b_frobenius :
    igFrobeniusAlg.mul descent_operator_that_maps_any_integer_8d170b_s0 descent_operator_that_maps_any_integer_8d170b_s0 = descent_operator_that_maps_any_integer_8d170b_s0 :=
  igFrobAlg_self_fusion descent_operator_that_maps_any_integer_8d170b_s0
