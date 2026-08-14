-- IGProtocol scaffold: VINIT → AREV → FSPLIT → EVALT → IMSCRIB → AFWD → ENGAGR → EVALF → FFUSE → CLINK → IFIX → TANCH
-- Class: The closure witnesses in R4 through R7 gate the substrate vote, qualifying the result computed from R0 through R3 rather than contributing weight to it.
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 8)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑗  | initial object — ground of distinction
--   [1] AREV      pol    := 𐑗               𐑼 → 𐑚  | reverse morphism — parity flip
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [5] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [6] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [9] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [10] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [11] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_closure_witnesses_in_r4_through_r7_a2da7d_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_closure_witnesses_in_r4_through_r7_a2da7d_protocol : IGProtocol the_closure_witnesses_in_r4_through_r7_a2da7d_s0 the_closure_witnesses_in_r4_through_r7_a2da7d_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_closure_witnesses_in_r4_through_r7_a2da7d_s8 the_closure_witnesses_in_r4_through_r7_a2da7d_s8 = the_closure_witnesses_in_r4_through_r7_a2da7d_s8 (idempotent)
  (.seq (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l0 the_closure_witnesses_in_r4_through_r7_a2da7d_s0 the_closure_witnesses_in_r4_through_r7_a2da7d_s1) (.seq (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l1 the_closure_witnesses_in_r4_through_r7_a2da7d_s1 the_closure_witnesses_in_r4_through_r7_a2da7d_s2) (.seq (.prod (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l2 the_closure_witnesses_in_r4_through_r7_a2da7d_s2 the_closure_witnesses_in_r4_through_r7_a2da7d_s8) (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l2 the_closure_witnesses_in_r4_through_r7_a2da7d_s2 the_closure_witnesses_in_r4_through_r7_a2da7d_s8)) (.seq (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l8 the_closure_witnesses_in_r4_through_r7_a2da7d_s8 the_closure_witnesses_in_r4_through_r7_a2da7d_s8) (.seq (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l8 the_closure_witnesses_in_r4_through_r7_a2da7d_s8 the_closure_witnesses_in_r4_through_r7_a2da7d_s9) (.seq (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l9 the_closure_witnesses_in_r4_through_r7_a2da7d_s9 the_closure_witnesses_in_r4_through_r7_a2da7d_s10) (.arrow the_closure_witnesses_in_r4_through_r7_a2da7d_l10 the_closure_witnesses_in_r4_through_r7_a2da7d_s10 the_closure_witnesses_in_r4_through_r7_a2da7d_s11)))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_closure_witnesses_in_r4_through_r7_a2da7d_true_arm : IGProtocol the_closure_witnesses_in_r4_through_r7_a2da7d_s0 the_closure_witnesses_in_r4_through_r7_a2da7d_s11 :=
  (the_closure_witnesses_in_r4_through_r7_a2da7d_protocol).restrictToEVALT

-- false arm
noncomputable def the_closure_witnesses_in_r4_through_r7_a2da7d_false_arm : IGProtocol the_closure_witnesses_in_r4_through_r7_a2da7d_s0 the_closure_witnesses_in_r4_through_r7_a2da7d_s11 :=
  (the_closure_witnesses_in_r4_through_r7_a2da7d_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_closure_witnesses_in_r4_through_r7_a2da7d_tier_ground : OuroboricityTier := TierFunctor.obj the_closure_witnesses_in_r4_through_r7_a2da7d_s0
def the_closure_witnesses_in_r4_through_r7_a2da7d_tier : OuroboricityTier := TierFunctor.obj the_closure_witnesses_in_r4_through_r7_a2da7d_s11
#eval the_closure_witnesses_in_r4_through_r7_a2da7d_tier_ground  -- tier of the ground (pre-transformation)
#eval the_closure_witnesses_in_r4_through_r7_a2da7d_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_closure_witnesses_in_r4_through_r7_a2da7d_frobenius :
    igFrobeniusAlg.mul the_closure_witnesses_in_r4_through_r7_a2da7d_s0 the_closure_witnesses_in_r4_through_r7_a2da7d_s0 = the_closure_witnesses_in_r4_through_r7_a2da7d_s0 :=
  igFrobAlg_self_fusion the_closure_witnesses_in_r4_through_r7_a2da7d_s0
