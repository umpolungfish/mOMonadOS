-- IGProtocol scaffold: VINIT → CLINK → FSPLIT → EVALT → AFWD → EVALF → AREV → FFUSE → ENGAGR → IMSCRIB → AFWD → IFIX → TANCH → IMSCRIB
-- Class: No longer Ambix and Cucurbit, I am become Retort
-- Fingerprint: sig=(8,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=14
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 7)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑱  | initial object — ground of distinction
--   [1] CLINK     fid    := 𐑱               𐑼 → 𐑚  | composition — regime coherence
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [8] ENGAGR    stoi   := 𐑳               𐑙 → 𐑠  | engage paradox — B-state, both arms
--   [9] IMSCRIB   gram   := 𐑠               𐑳 → 𐑾  | identity — self-imscription
--   [10] AFWD      rel    := 𐑾               𐑠 → 𐑭  | forward morphism — bidirectional arrow
--   [11] IFIX      prot   := 𐑭               𐑾 → 𐑡  | irreversible fixation — winding number
--   [12] TANCH     top    := 𐑡               𐑭 → 𐑠  | terminal object — connectivity boundary
--   [13] IMSCRIB   gram   := 𐑠               𐑡 → 𐑼  | identity — self-imscription

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def no_longer_ambix_and_cucurbit_i_am_381596_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def no_longer_ambix_and_cucurbit_i_am_381596_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def no_longer_ambix_and_cucurbit_i_am_381596_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def no_longer_ambix_and_cucurbit_i_am_381596_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def no_longer_ambix_and_cucurbit_i_am_381596_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_longer_ambix_and_cucurbit_i_am_381596_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def no_longer_ambix_and_cucurbit_i_am_381596_protocol : IGProtocol no_longer_ambix_and_cucurbit_i_am_381596_s0 no_longer_ambix_and_cucurbit_i_am_381596_s13 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct no_longer_ambix_and_cucurbit_i_am_381596_s7 no_longer_ambix_and_cucurbit_i_am_381596_s7 = no_longer_ambix_and_cucurbit_i_am_381596_s7 (idempotent)
  (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l0 no_longer_ambix_and_cucurbit_i_am_381596_s0 no_longer_ambix_and_cucurbit_i_am_381596_s1) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l1 no_longer_ambix_and_cucurbit_i_am_381596_s1 no_longer_ambix_and_cucurbit_i_am_381596_s2) (.seq (.prod (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l2 no_longer_ambix_and_cucurbit_i_am_381596_s2 no_longer_ambix_and_cucurbit_i_am_381596_s7) (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l2 no_longer_ambix_and_cucurbit_i_am_381596_s2 no_longer_ambix_and_cucurbit_i_am_381596_s7)) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l7 no_longer_ambix_and_cucurbit_i_am_381596_s7 no_longer_ambix_and_cucurbit_i_am_381596_s7) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l7 no_longer_ambix_and_cucurbit_i_am_381596_s7 no_longer_ambix_and_cucurbit_i_am_381596_s8) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l8 no_longer_ambix_and_cucurbit_i_am_381596_s8 no_longer_ambix_and_cucurbit_i_am_381596_s9) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l9 no_longer_ambix_and_cucurbit_i_am_381596_s9 no_longer_ambix_and_cucurbit_i_am_381596_s10) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l10 no_longer_ambix_and_cucurbit_i_am_381596_s10 no_longer_ambix_and_cucurbit_i_am_381596_s11) (.seq (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l11 no_longer_ambix_and_cucurbit_i_am_381596_s11 no_longer_ambix_and_cucurbit_i_am_381596_s12) (.arrow no_longer_ambix_and_cucurbit_i_am_381596_l12 no_longer_ambix_and_cucurbit_i_am_381596_s12 no_longer_ambix_and_cucurbit_i_am_381596_s13))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def no_longer_ambix_and_cucurbit_i_am_381596_true_arm : IGProtocol no_longer_ambix_and_cucurbit_i_am_381596_s0 no_longer_ambix_and_cucurbit_i_am_381596_s13 :=
  (no_longer_ambix_and_cucurbit_i_am_381596_protocol).restrictToEVALT

-- false arm
noncomputable def no_longer_ambix_and_cucurbit_i_am_381596_false_arm : IGProtocol no_longer_ambix_and_cucurbit_i_am_381596_s0 no_longer_ambix_and_cucurbit_i_am_381596_s13 :=
  (no_longer_ambix_and_cucurbit_i_am_381596_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def no_longer_ambix_and_cucurbit_i_am_381596_tier : OuroboricityTier := TierFunctor.obj no_longer_ambix_and_cucurbit_i_am_381596_s0
#eval no_longer_ambix_and_cucurbit_i_am_381596_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem no_longer_ambix_and_cucurbit_i_am_381596_frobenius :
    igFrobeniusAlg.mul no_longer_ambix_and_cucurbit_i_am_381596_s0 no_longer_ambix_and_cucurbit_i_am_381596_s0 = no_longer_ambix_and_cucurbit_i_am_381596_s0 :=
  igFrobAlg_self_fusion no_longer_ambix_and_cucurbit_i_am_381596_s0
