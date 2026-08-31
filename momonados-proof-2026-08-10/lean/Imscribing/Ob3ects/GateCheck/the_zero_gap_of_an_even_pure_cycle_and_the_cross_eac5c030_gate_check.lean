-- IGProtocol scaffold: VINIT → IMSCRIB → CLINK → IFIX → FSPLIT → EVALT → AFWD → EVALF → AREV → ENGAGR → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: the zero gap of an even pure cycle, and the cross-link that lifts rho from 2 to 1+sqrt(3) while lambda_min stays pinned at -2
-- Fingerprint: sig=(7,2,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=14
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(4, 10)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑱  | identity — self-imscription
--   [2] CLINK     fid    := 𐑱               𐑠 → 𐑭  | composition — regime coherence
--   [3] IFIX      prot   := 𐑭               𐑱 → 𐑚  | irreversible fixation — winding number
--   [4] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [10] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [11] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [12] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [13] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_protocol : IGProtocol the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s13 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10 = the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10 (idempotent)
  (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l0 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s1) (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l1 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s1 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s2) (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l2 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s2 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s3) (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l3 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s3 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s4) (.seq (.prod (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l4 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s4 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10) (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l4 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s4 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10)) (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l10 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10) (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l10 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s10 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s11) (.seq (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l11 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s11 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s12) (.arrow the_zero_gap_of_an_even_pure_cycle_and_b0a96b_l12 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s12 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s13)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_true_arm : IGProtocol the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s13 :=
  (the_zero_gap_of_an_even_pure_cycle_and_b0a96b_protocol).restrictToEVALT

-- false arm
noncomputable def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_false_arm : IGProtocol the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s13 :=
  (the_zero_gap_of_an_even_pure_cycle_and_b0a96b_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_zero_gap_of_an_even_pure_cycle_and_b0a96b_tier : OuroboricityTier := TierFunctor.obj the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0
#eval the_zero_gap_of_an_even_pure_cycle_and_b0a96b_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_zero_gap_of_an_even_pure_cycle_and_b0a96b_frobenius :
    igFrobeniusAlg.mul the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 = the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0 :=
  igFrobAlg_self_fusion the_zero_gap_of_an_even_pure_cycle_and_b0a96b_s0
