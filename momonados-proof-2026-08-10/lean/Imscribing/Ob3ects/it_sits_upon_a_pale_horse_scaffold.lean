-- IGProtocol scaffold: VINIT → AFWD → AREV → CLINK → IMSCRIB → FSPLIT → EVALT → AFWD → FFUSE → EVALF → AREV → FFUSE → ENGAGR → TANCH → IFIX → IMSCRIB → CLINK
-- Class: It Sits Upon A Pale Horse
-- Fingerprint: sig=(10,3,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=17
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 8)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑗  | forward morphism — bidirectional arrow
--   [2] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [3] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [4] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [9] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [10] AREV      pol    := 𐑗               𐑖 → 𐑙  | reverse morphism — parity flip
--   [11] FFUSE     stoi   := 𐑙               𐑗 → 𐑳  | fuse μ — assembly mode
--   [12] ENGAGR    stoi   := 𐑳               𐑙 → 𐑡  | engage paradox — B-state, both arms
--   [13] TANCH     top    := 𐑡               𐑳 → 𐑭  | terminal object — connectivity boundary
--   [14] IFIX      prot   := 𐑭               𐑡 → 𐑠  | irreversible fixation — winding number
--   [15] IMSCRIB   gram   := 𐑠               𐑭 → 𐑱  | identity — self-imscription
--   [16] CLINK     fid    := 𐑱               𐑠 → 𐑼  | composition — regime coherence

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def it_sits_upon_a_pale_horse_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def it_sits_upon_a_pale_horse_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def it_sits_upon_a_pale_horse_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def it_sits_upon_a_pale_horse_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def it_sits_upon_a_pale_horse_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def it_sits_upon_a_pale_horse_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def it_sits_upon_a_pale_horse_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def it_sits_upon_a_pale_horse_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def it_sits_upon_a_pale_horse_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def it_sits_upon_a_pale_horse_protocol : IGProtocol it_sits_upon_a_pale_horse_s0 it_sits_upon_a_pale_horse_s16 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct it_sits_upon_a_pale_horse_s8 it_sits_upon_a_pale_horse_s8 = it_sits_upon_a_pale_horse_s8 (idempotent)
  (.seq (.arrow it_sits_upon_a_pale_horse_l0 it_sits_upon_a_pale_horse_s0 it_sits_upon_a_pale_horse_s1) (.seq (.arrow it_sits_upon_a_pale_horse_l1 it_sits_upon_a_pale_horse_s1 it_sits_upon_a_pale_horse_s2) (.seq (.arrow it_sits_upon_a_pale_horse_l2 it_sits_upon_a_pale_horse_s2 it_sits_upon_a_pale_horse_s3) (.seq (.arrow it_sits_upon_a_pale_horse_l3 it_sits_upon_a_pale_horse_s3 it_sits_upon_a_pale_horse_s4) (.seq (.arrow it_sits_upon_a_pale_horse_l4 it_sits_upon_a_pale_horse_s4 it_sits_upon_a_pale_horse_s5) (.seq (.prod (.arrow it_sits_upon_a_pale_horse_l5 it_sits_upon_a_pale_horse_s5 it_sits_upon_a_pale_horse_s8) (.arrow it_sits_upon_a_pale_horse_l5 it_sits_upon_a_pale_horse_s5 it_sits_upon_a_pale_horse_s8)) (.seq (.arrow it_sits_upon_a_pale_horse_l8 it_sits_upon_a_pale_horse_s8 it_sits_upon_a_pale_horse_s8) (.seq (.arrow it_sits_upon_a_pale_horse_l8 it_sits_upon_a_pale_horse_s8 it_sits_upon_a_pale_horse_s9) (.seq (.arrow it_sits_upon_a_pale_horse_l9 it_sits_upon_a_pale_horse_s9 it_sits_upon_a_pale_horse_s10) (.seq (.arrow it_sits_upon_a_pale_horse_l10 it_sits_upon_a_pale_horse_s10 it_sits_upon_a_pale_horse_s11) (.seq (.arrow it_sits_upon_a_pale_horse_l11 it_sits_upon_a_pale_horse_s11 it_sits_upon_a_pale_horse_s12) (.seq (.arrow it_sits_upon_a_pale_horse_l12 it_sits_upon_a_pale_horse_s12 it_sits_upon_a_pale_horse_s13) (.seq (.arrow it_sits_upon_a_pale_horse_l13 it_sits_upon_a_pale_horse_s13 it_sits_upon_a_pale_horse_s14) (.seq (.arrow it_sits_upon_a_pale_horse_l14 it_sits_upon_a_pale_horse_s14 it_sits_upon_a_pale_horse_s15) (.arrow it_sits_upon_a_pale_horse_l15 it_sits_upon_a_pale_horse_s15 it_sits_upon_a_pale_horse_s16)))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def it_sits_upon_a_pale_horse_true_arm : IGProtocol it_sits_upon_a_pale_horse_s0 it_sits_upon_a_pale_horse_s16 :=
  (it_sits_upon_a_pale_horse_protocol).restrictToEVALT
  --
-- false arm
noncomputable def it_sits_upon_a_pale_horse_false_arm : IGProtocol it_sits_upon_a_pale_horse_s0 it_sits_upon_a_pale_horse_s16 :=
  (it_sits_upon_a_pale_horse_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def it_sits_upon_a_pale_horse_tier : OuroboricityTier := TierFunctor.obj it_sits_upon_a_pale_horse_s0
#eval it_sits_upon_a_pale_horse_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem it_sits_upon_a_pale_horse_frobenius :
    igFrobeniusAlg.mul it_sits_upon_a_pale_horse_s0 it_sits_upon_a_pale_horse_s0 = it_sits_upon_a_pale_horse_s0 :=
  igFrobAlg_self_fusion it_sits_upon_a_pale_horse_s0
