-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → AFWD → EVALT → AREV → EVALF → ENGAGR → FFUSE → IFIX → CLINK → TANCH
-- Class: Reconstitute the net winding invariant from the directional branches
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
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑚  | identity — self-imscription
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [6] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [7] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [9] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [10] CLINK     fid    := 𐑱               𐑭 → 𐑡  | composition — regime coherence
--   [11] TANCH     top    := 𐑡               𐑱 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def reconstitute_the_net_winding_invariant_3b3317_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_net_winding_invariant_3b3317_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def reconstitute_the_net_winding_invariant_3b3317_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def reconstitute_the_net_winding_invariant_3b3317_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def reconstitute_the_net_winding_invariant_3b3317_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def reconstitute_the_net_winding_invariant_3b3317_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def reconstitute_the_net_winding_invariant_3b3317_protocol : IGProtocol reconstitute_the_net_winding_invariant_3b3317_s0 reconstitute_the_net_winding_invariant_3b3317_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct reconstitute_the_net_winding_invariant_3b3317_s8 reconstitute_the_net_winding_invariant_3b3317_s8 = reconstitute_the_net_winding_invariant_3b3317_s8 (idempotent)
  (.seq (.arrow reconstitute_the_net_winding_invariant_3b3317_l0 reconstitute_the_net_winding_invariant_3b3317_s0 reconstitute_the_net_winding_invariant_3b3317_s1) (.seq (.arrow reconstitute_the_net_winding_invariant_3b3317_l1 reconstitute_the_net_winding_invariant_3b3317_s1 reconstitute_the_net_winding_invariant_3b3317_s2) (.seq (.prod (.arrow reconstitute_the_net_winding_invariant_3b3317_l2 reconstitute_the_net_winding_invariant_3b3317_s2 reconstitute_the_net_winding_invariant_3b3317_s8) (.arrow reconstitute_the_net_winding_invariant_3b3317_l2 reconstitute_the_net_winding_invariant_3b3317_s2 reconstitute_the_net_winding_invariant_3b3317_s8)) (.seq (.arrow reconstitute_the_net_winding_invariant_3b3317_l8 reconstitute_the_net_winding_invariant_3b3317_s8 reconstitute_the_net_winding_invariant_3b3317_s8) (.seq (.arrow reconstitute_the_net_winding_invariant_3b3317_l8 reconstitute_the_net_winding_invariant_3b3317_s8 reconstitute_the_net_winding_invariant_3b3317_s9) (.seq (.arrow reconstitute_the_net_winding_invariant_3b3317_l9 reconstitute_the_net_winding_invariant_3b3317_s9 reconstitute_the_net_winding_invariant_3b3317_s10) (.arrow reconstitute_the_net_winding_invariant_3b3317_l10 reconstitute_the_net_winding_invariant_3b3317_s10 reconstitute_the_net_winding_invariant_3b3317_s11)))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def reconstitute_the_net_winding_invariant_3b3317_true_arm : IGProtocol reconstitute_the_net_winding_invariant_3b3317_s0 reconstitute_the_net_winding_invariant_3b3317_s11 :=
  (reconstitute_the_net_winding_invariant_3b3317_protocol).restrictToEVALT

-- false arm
noncomputable def reconstitute_the_net_winding_invariant_3b3317_false_arm : IGProtocol reconstitute_the_net_winding_invariant_3b3317_s0 reconstitute_the_net_winding_invariant_3b3317_s11 :=
  (reconstitute_the_net_winding_invariant_3b3317_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def reconstitute_the_net_winding_invariant_3b3317_tier_ground : OuroboricityTier := TierFunctor.obj reconstitute_the_net_winding_invariant_3b3317_s0
def reconstitute_the_net_winding_invariant_3b3317_tier : OuroboricityTier := TierFunctor.obj reconstitute_the_net_winding_invariant_3b3317_s11
#eval reconstitute_the_net_winding_invariant_3b3317_tier_ground  -- tier of the ground (pre-transformation)
#eval reconstitute_the_net_winding_invariant_3b3317_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem reconstitute_the_net_winding_invariant_3b3317_frobenius :
    igFrobeniusAlg.mul reconstitute_the_net_winding_invariant_3b3317_s0 reconstitute_the_net_winding_invariant_3b3317_s0 = reconstitute_the_net_winding_invariant_3b3317_s0 :=
  igFrobAlg_self_fusion reconstitute_the_net_winding_invariant_3b3317_s0
