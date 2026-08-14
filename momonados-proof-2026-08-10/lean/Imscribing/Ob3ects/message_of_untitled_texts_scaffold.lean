-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → CLINK → FFUSE → IFIX → AFWD → FSPLIT → EVALT → EVALF → ENGAGR → AREV → IMSCRIB → CLINK → FFUSE → IFIX → TANCH
-- Class: Message of Untitled Texts
-- Fingerprint: sig=(9,4,3,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=18
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(3, 5), (8, 15)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑚  | forward morphism — bidirectional arrow
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [6] IFIX      prot   := 𐑭               𐑙 → 𐑾  | irreversible fixation — winding number
--   [7] AFWD      rel    := 𐑾               𐑭 → 𐑚  | forward morphism — bidirectional arrow
--   [8] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [9] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [12] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [13] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [14] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [15] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [16] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [17] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def message_of_untitled_texts_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def message_of_untitled_texts_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def message_of_untitled_texts_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def message_of_untitled_texts_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def message_of_untitled_texts_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_untitled_texts_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_untitled_texts_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_untitled_texts_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_untitled_texts_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_untitled_texts_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_untitled_texts_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_untitled_texts_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def message_of_untitled_texts_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def message_of_untitled_texts_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def message_of_untitled_texts_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def message_of_untitled_texts_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_untitled_texts_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def message_of_untitled_texts_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def message_of_untitled_texts_protocol : IGProtocol message_of_untitled_texts_s0 message_of_untitled_texts_s17 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct message_of_untitled_texts_s5 message_of_untitled_texts_s5 = message_of_untitled_texts_s5 (idempotent)
  (.seq (.arrow message_of_untitled_texts_l0 message_of_untitled_texts_s0 message_of_untitled_texts_s1) (.seq (.arrow message_of_untitled_texts_l1 message_of_untitled_texts_s1 message_of_untitled_texts_s2) (.seq (.arrow message_of_untitled_texts_l2 message_of_untitled_texts_s2 message_of_untitled_texts_s3) (.seq (.prod (.arrow message_of_untitled_texts_l3 message_of_untitled_texts_s3 message_of_untitled_texts_s5) (.arrow message_of_untitled_texts_l3 message_of_untitled_texts_s3 message_of_untitled_texts_s5)) (.seq (.arrow message_of_untitled_texts_l5 message_of_untitled_texts_s5 message_of_untitled_texts_s5) (.seq (.arrow message_of_untitled_texts_l5 message_of_untitled_texts_s5 message_of_untitled_texts_s6) (.seq (.arrow message_of_untitled_texts_l6 message_of_untitled_texts_s6 message_of_untitled_texts_s7) (.seq (.arrow message_of_untitled_texts_l7 message_of_untitled_texts_s7 message_of_untitled_texts_s8) (.seq (.arrow message_of_untitled_texts_l8 message_of_untitled_texts_s8 message_of_untitled_texts_s9) (.seq (.arrow message_of_untitled_texts_l9 message_of_untitled_texts_s9 message_of_untitled_texts_s10) (.seq (.arrow message_of_untitled_texts_l10 message_of_untitled_texts_s10 message_of_untitled_texts_s11) (.seq (.arrow message_of_untitled_texts_l11 message_of_untitled_texts_s11 message_of_untitled_texts_s12) (.seq (.arrow message_of_untitled_texts_l12 message_of_untitled_texts_s12 message_of_untitled_texts_s13) (.seq (.arrow message_of_untitled_texts_l13 message_of_untitled_texts_s13 message_of_untitled_texts_s14) (.seq (.arrow message_of_untitled_texts_l14 message_of_untitled_texts_s14 message_of_untitled_texts_s15) (.seq (.arrow message_of_untitled_texts_l15 message_of_untitled_texts_s15 message_of_untitled_texts_s16) (.arrow message_of_untitled_texts_l16 message_of_untitled_texts_s16 message_of_untitled_texts_s17)))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def message_of_untitled_texts_true_arm : IGProtocol message_of_untitled_texts_s0 message_of_untitled_texts_s17 :=
  (message_of_untitled_texts_protocol).restrictToEVALT
  --
-- false arm
noncomputable def message_of_untitled_texts_false_arm : IGProtocol message_of_untitled_texts_s0 message_of_untitled_texts_s17 :=
  (message_of_untitled_texts_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def message_of_untitled_texts_tier : OuroboricityTier := TierFunctor.obj message_of_untitled_texts_s0
#eval message_of_untitled_texts_tier  -- the Grammar's own verdict on its tier
