-- IGProtocol scaffold: VINIT → IMSCRIB → IFIX → FSPLIT → AFWD → EVALT → AREV → EVALF → ENGAGR → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: Reconstitute the three frame projections into the unified superoperator.
-- Fingerprint: sig=(7,2,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=14
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑭  | identity — self-imscription
--   [2] IFIX      prot   := 𐑭               𐑠 → 𐑚  | irreversible fixation — winding number
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [10] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [11] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [12] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [13] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def reconstitute_the_three_frame_3636bd_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def reconstitute_the_three_frame_3636bd_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def reconstitute_the_three_frame_3636bd_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def reconstitute_the_three_frame_3636bd_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_three_frame_3636bd_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_three_frame_3636bd_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def reconstitute_the_three_frame_3636bd_protocol : IGProtocol reconstitute_the_three_frame_3636bd_s0 reconstitute_the_three_frame_3636bd_s13 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct reconstitute_the_three_frame_3636bd_s9 reconstitute_the_three_frame_3636bd_s9 = reconstitute_the_three_frame_3636bd_s9 (idempotent)
  (.seq (.arrow reconstitute_the_three_frame_3636bd_l0 reconstitute_the_three_frame_3636bd_s0 reconstitute_the_three_frame_3636bd_s1) (.seq (.arrow reconstitute_the_three_frame_3636bd_l1 reconstitute_the_three_frame_3636bd_s1 reconstitute_the_three_frame_3636bd_s2) (.seq (.arrow reconstitute_the_three_frame_3636bd_l2 reconstitute_the_three_frame_3636bd_s2 reconstitute_the_three_frame_3636bd_s3) (.seq (.prod (.arrow reconstitute_the_three_frame_3636bd_l3 reconstitute_the_three_frame_3636bd_s3 reconstitute_the_three_frame_3636bd_s9) (.arrow reconstitute_the_three_frame_3636bd_l3 reconstitute_the_three_frame_3636bd_s3 reconstitute_the_three_frame_3636bd_s9)) (.seq (.arrow reconstitute_the_three_frame_3636bd_l9 reconstitute_the_three_frame_3636bd_s9 reconstitute_the_three_frame_3636bd_s9) (.seq (.arrow reconstitute_the_three_frame_3636bd_l9 reconstitute_the_three_frame_3636bd_s9 reconstitute_the_three_frame_3636bd_s10) (.seq (.arrow reconstitute_the_three_frame_3636bd_l10 reconstitute_the_three_frame_3636bd_s10 reconstitute_the_three_frame_3636bd_s11) (.seq (.arrow reconstitute_the_three_frame_3636bd_l11 reconstitute_the_three_frame_3636bd_s11 reconstitute_the_three_frame_3636bd_s12) (.arrow reconstitute_the_three_frame_3636bd_l12 reconstitute_the_three_frame_3636bd_s12 reconstitute_the_three_frame_3636bd_s13)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def reconstitute_the_three_frame_3636bd_true_arm : IGProtocol reconstitute_the_three_frame_3636bd_s0 reconstitute_the_three_frame_3636bd_s13 :=
  (reconstitute_the_three_frame_3636bd_protocol).restrictToEVALT

-- false arm
noncomputable def reconstitute_the_three_frame_3636bd_false_arm : IGProtocol reconstitute_the_three_frame_3636bd_s0 reconstitute_the_three_frame_3636bd_s13 :=
  (reconstitute_the_three_frame_3636bd_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def reconstitute_the_three_frame_3636bd_tier_ground : OuroboricityTier := TierFunctor.obj reconstitute_the_three_frame_3636bd_s0
def reconstitute_the_three_frame_3636bd_tier : OuroboricityTier := TierFunctor.obj reconstitute_the_three_frame_3636bd_s13
#eval reconstitute_the_three_frame_3636bd_tier_ground  -- tier of the ground (pre-transformation)
#eval reconstitute_the_three_frame_3636bd_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem reconstitute_the_three_frame_3636bd_frobenius :
    igFrobeniusAlg.mul reconstitute_the_three_frame_3636bd_s0 reconstitute_the_three_frame_3636bd_s0 = reconstitute_the_three_frame_3636bd_s0 :=
  igFrobAlg_self_fusion reconstitute_the_three_frame_3636bd_s0
