-- IGProtocol scaffold: VINIT → TANCH → FSPLIT → AFWD → EVALT → AREV → CLINK → IMSCRIB → IFIX → FSPLIT → AFWD → EVALT → AREV → CLINK → IMSCRIB → IFIX → ENGAGR → FSPLIT → AFWD → EVALT → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: Zosimos' Stilling
-- Fingerprint: sig=(13,4,4,3)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=24
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(17, 20)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑚  | terminal object — connectivity boundary
--   [2] FSPLIT    gran   := 𐑚               𐑡 → 𐑾  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → ⊙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑾 → 𐑗  | evaluate-true — criticality gate open
--   [5] AREV      pol    := 𐑗               ⊙ → 𐑱  | reverse morphism — parity flip
--   [6] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [7] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [8] IFIX      prot   := 𐑭               𐑠 → 𐑚  | irreversible fixation — winding number
--   [9] FSPLIT    gran   := 𐑚               𐑭 → 𐑾  | split δ — range decomposition
--   [10] AFWD      rel    := 𐑾               𐑚 → ⊙  | forward morphism — bidirectional arrow
--   [11] EVALT     crit   := ⊙               𐑾 → 𐑗  | evaluate-true — criticality gate open
--   [12] AREV      pol    := 𐑗               ⊙ → 𐑱  | reverse morphism — parity flip
--   [13] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [14] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [15] IFIX      prot   := 𐑭               𐑠 → 𐑳  | irreversible fixation — winding number
--   [16] ENGAGR    stoi   := 𐑳               𐑭 → 𐑚  | engage paradox — B-state, both arms
--   [17] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [18] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [19] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [20] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [21] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [22] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [23] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def zosimos_stilling_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def zosimos_stilling_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def zosimos_stilling_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def zosimos_stilling_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def zosimos_stilling_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def zosimos_stilling_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def zosimos_stilling_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l18 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def zosimos_stilling_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def zosimos_stilling_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def zosimos_stilling_protocol : IGProtocol zosimos_stilling_s0 zosimos_stilling_s23 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct zosimos_stilling_s20 zosimos_stilling_s20 = zosimos_stilling_s20 (idempotent)
  (.seq (.arrow zosimos_stilling_l0 zosimos_stilling_s0 zosimos_stilling_s1) (.seq (.arrow zosimos_stilling_l1 zosimos_stilling_s1 zosimos_stilling_s2) (.seq (.arrow zosimos_stilling_l2 zosimos_stilling_s2 zosimos_stilling_s3) (.seq (.arrow zosimos_stilling_l3 zosimos_stilling_s3 zosimos_stilling_s4) (.seq (.arrow zosimos_stilling_l4 zosimos_stilling_s4 zosimos_stilling_s5) (.seq (.arrow zosimos_stilling_l5 zosimos_stilling_s5 zosimos_stilling_s6) (.seq (.arrow zosimos_stilling_l6 zosimos_stilling_s6 zosimos_stilling_s7) (.seq (.arrow zosimos_stilling_l7 zosimos_stilling_s7 zosimos_stilling_s8) (.seq (.arrow zosimos_stilling_l8 zosimos_stilling_s8 zosimos_stilling_s9) (.seq (.arrow zosimos_stilling_l9 zosimos_stilling_s9 zosimos_stilling_s10) (.seq (.arrow zosimos_stilling_l10 zosimos_stilling_s10 zosimos_stilling_s11) (.seq (.arrow zosimos_stilling_l11 zosimos_stilling_s11 zosimos_stilling_s12) (.seq (.arrow zosimos_stilling_l12 zosimos_stilling_s12 zosimos_stilling_s13) (.seq (.arrow zosimos_stilling_l13 zosimos_stilling_s13 zosimos_stilling_s14) (.seq (.arrow zosimos_stilling_l14 zosimos_stilling_s14 zosimos_stilling_s15) (.seq (.arrow zosimos_stilling_l15 zosimos_stilling_s15 zosimos_stilling_s16) (.seq (.arrow zosimos_stilling_l16 zosimos_stilling_s16 zosimos_stilling_s17) (.seq (.prod (.arrow zosimos_stilling_l17 zosimos_stilling_s17 zosimos_stilling_s20) (.arrow zosimos_stilling_l17 zosimos_stilling_s17 zosimos_stilling_s20)) (.seq (.arrow zosimos_stilling_l20 zosimos_stilling_s20 zosimos_stilling_s20) (.seq (.arrow zosimos_stilling_l20 zosimos_stilling_s20 zosimos_stilling_s21) (.seq (.arrow zosimos_stilling_l21 zosimos_stilling_s21 zosimos_stilling_s22) (.arrow zosimos_stilling_l22 zosimos_stilling_s22 zosimos_stilling_s23))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def zosimos_stilling_true_arm : IGProtocol zosimos_stilling_s0 zosimos_stilling_s23 :=
  (zosimos_stilling_protocol).restrictToEVALT
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def zosimos_stilling_tier : OuroboricityTier := TierFunctor.obj zosimos_stilling_s0
#eval zosimos_stilling_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem zosimos_stilling_frobenius :
    igFrobeniusAlg.mul zosimos_stilling_s0 zosimos_stilling_s0 = zosimos_stilling_s0 :=
  igFrobAlg_self_fusion zosimos_stilling_s0
