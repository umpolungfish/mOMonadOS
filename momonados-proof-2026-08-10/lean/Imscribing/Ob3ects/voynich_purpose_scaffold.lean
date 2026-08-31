-- IGProtocol scaffold: VINIT → ENGAGR → AFWD → IMSCRIB → AREV → CLINK → FSPLIT → EVALT → FFUSE → IFIX → AFWD → IMSCRIB → CLINK → AFWD → FSPLIT → EVALT → FFUSE → IFIX → TANCH
-- Class: Voynich Purpose
-- Fingerprint: sig=(10,4,3,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=False | period=19
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(6, 8), (14, 16)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑳  | initial object — ground of distinction
--   [1] ENGAGR    stoi   := 𐑳               𐑼 → 𐑾  | engage paradox — B-state, both arms
--   [2] AFWD      rel    := 𐑾               𐑳 → 𐑠  | forward morphism — bidirectional arrow
--   [3] IMSCRIB   gram   := 𐑠               𐑾 → 𐑗  | identity — self-imscription
--   [4] AREV      pol    := 𐑗               𐑠 → 𐑱  | reverse morphism — parity flip
--   [5] CLINK     fid    := 𐑱               𐑗 → 𐑚  | composition — regime coherence
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [9] IFIX      prot   := 𐑭               𐑙 → 𐑾  | irreversible fixation — winding number
--   [10] AFWD      rel    := 𐑾               𐑭 → 𐑠  | forward morphism — bidirectional arrow
--   [11] IMSCRIB   gram   := 𐑠               𐑾 → 𐑱  | identity — self-imscription
--   [12] CLINK     fid    := 𐑱               𐑠 → 𐑾  | composition — regime coherence
--   [13] AFWD      rel    := 𐑾               𐑱 → 𐑚  | forward morphism — bidirectional arrow
--   [14] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [15] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [16] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [17] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [18] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def voynich_purpose_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def voynich_purpose_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def voynich_purpose_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l13 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def voynich_purpose_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def voynich_purpose_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def voynich_purpose_protocol : IGProtocol voynich_purpose_s0 voynich_purpose_s18 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct voynich_purpose_s8 voynich_purpose_s8 = voynich_purpose_s8 (idempotent)
  (.seq (.arrow voynich_purpose_l0 voynich_purpose_s0 voynich_purpose_s1) (.seq (.arrow voynich_purpose_l1 voynich_purpose_s1 voynich_purpose_s2) (.seq (.arrow voynich_purpose_l2 voynich_purpose_s2 voynich_purpose_s3) (.seq (.arrow voynich_purpose_l3 voynich_purpose_s3 voynich_purpose_s4) (.seq (.arrow voynich_purpose_l4 voynich_purpose_s4 voynich_purpose_s5) (.seq (.arrow voynich_purpose_l5 voynich_purpose_s5 voynich_purpose_s6) (.seq (.prod (.arrow voynich_purpose_l6 voynich_purpose_s6 voynich_purpose_s8) (.arrow voynich_purpose_l6 voynich_purpose_s6 voynich_purpose_s8)) (.seq (.arrow voynich_purpose_l8 voynich_purpose_s8 voynich_purpose_s8) (.seq (.arrow voynich_purpose_l8 voynich_purpose_s8 voynich_purpose_s9) (.seq (.arrow voynich_purpose_l9 voynich_purpose_s9 voynich_purpose_s10) (.seq (.arrow voynich_purpose_l10 voynich_purpose_s10 voynich_purpose_s11) (.seq (.arrow voynich_purpose_l11 voynich_purpose_s11 voynich_purpose_s12) (.seq (.arrow voynich_purpose_l12 voynich_purpose_s12 voynich_purpose_s13) (.seq (.arrow voynich_purpose_l13 voynich_purpose_s13 voynich_purpose_s14) (.seq (.arrow voynich_purpose_l14 voynich_purpose_s14 voynich_purpose_s15) (.seq (.arrow voynich_purpose_l15 voynich_purpose_s15 voynich_purpose_s16) (.seq (.arrow voynich_purpose_l16 voynich_purpose_s16 voynich_purpose_s17) (.arrow voynich_purpose_l17 voynich_purpose_s17 voynich_purpose_s18))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def voynich_purpose_true_arm : IGProtocol voynich_purpose_s0 voynich_purpose_s18 :=
  (voynich_purpose_protocol).restrictToEVALT
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def voynich_purpose_tier : OuroboricityTier := TierFunctor.obj voynich_purpose_s0
#eval voynich_purpose_tier  -- the Grammar's own verdict on its tier
