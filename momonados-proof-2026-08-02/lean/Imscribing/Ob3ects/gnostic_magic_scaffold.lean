-- IGProtocol scaffold: VINIT → AFWD → AFWD → AFWD → IMSCRIB → TANCH → CLINK → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → IFIX → AFWD → AFWD → AFWD → AFWD → FSPLIT → EVALT → FFUSE → AFWD → ENGAGR → AFWD → AFWD → AREV → CLINK → IMSCRIB → IFIX
-- Class: Gnostic Magic
-- Fingerprint: sig=(17,4,5,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=28
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(7, 11), (17, 19)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑾  | forward morphism — bidirectional arrow
--   [2] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑾 → 𐑠  | forward morphism — bidirectional arrow
--   [4] IMSCRIB   gram   := 𐑠               𐑾 → 𐑡  | identity — self-imscription
--   [5] TANCH     top    := 𐑡               𐑠 → 𐑱  | terminal object — connectivity boundary
--   [6] CLINK     fid    := 𐑱               𐑡 → 𐑚  | composition — regime coherence
--   [7] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [8] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [9] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [10] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [11] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [12] IFIX      prot   := 𐑭               𐑙 → 𐑾  | irreversible fixation — winding number
--   [13] AFWD      rel    := 𐑾               𐑭 → 𐑾  | forward morphism — bidirectional arrow
--   [14] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [15] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [16] AFWD      rel    := 𐑾               𐑾 → 𐑚  | forward morphism — bidirectional arrow
--   [17] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [18] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [19] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [20] AFWD      rel    := 𐑾               𐑙 → 𐑳  | forward morphism — bidirectional arrow
--   [21] ENGAGR    stoi   := 𐑳               𐑾 → 𐑾  | engage paradox — B-state, both arms
--   [22] AFWD      rel    := 𐑾               𐑳 → 𐑾  | forward morphism — bidirectional arrow
--   [23] AFWD      rel    := 𐑾               𐑾 → 𐑗  | forward morphism — bidirectional arrow
--   [24] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [25] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [26] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [27] IFIX      prot   := 𐑭               𐑠 → 𐑼  | irreversible fixation — winding number

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def gnostic_magic_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def gnostic_magic_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def gnostic_magic_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def gnostic_magic_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def gnostic_magic_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def gnostic_magic_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def gnostic_magic_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def gnostic_magic_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def gnostic_magic_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def gnostic_magic_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def gnostic_magic_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def gnostic_magic_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def gnostic_magic_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def gnostic_magic_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def gnostic_magic_l13 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l14 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l15 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l16 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l20 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def gnostic_magic_l22 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l23 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def gnostic_magic_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def gnostic_magic_protocol : IGProtocol gnostic_magic_s0 gnostic_magic_s27 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct gnostic_magic_s11 gnostic_magic_s11 = gnostic_magic_s11 (idempotent)
  (.seq (.arrow gnostic_magic_l0 gnostic_magic_s0 gnostic_magic_s1) (.seq (.arrow gnostic_magic_l1 gnostic_magic_s1 gnostic_magic_s2) (.seq (.arrow gnostic_magic_l2 gnostic_magic_s2 gnostic_magic_s3) (.seq (.arrow gnostic_magic_l3 gnostic_magic_s3 gnostic_magic_s4) (.seq (.arrow gnostic_magic_l4 gnostic_magic_s4 gnostic_magic_s5) (.seq (.arrow gnostic_magic_l5 gnostic_magic_s5 gnostic_magic_s6) (.seq (.arrow gnostic_magic_l6 gnostic_magic_s6 gnostic_magic_s7) (.seq (.prod (.arrow gnostic_magic_l7 gnostic_magic_s7 gnostic_magic_s11) (.arrow gnostic_magic_l7 gnostic_magic_s7 gnostic_magic_s11)) (.seq (.arrow gnostic_magic_l11 gnostic_magic_s11 gnostic_magic_s11) (.seq (.arrow gnostic_magic_l11 gnostic_magic_s11 gnostic_magic_s12) (.seq (.arrow gnostic_magic_l12 gnostic_magic_s12 gnostic_magic_s13) (.seq (.arrow gnostic_magic_l13 gnostic_magic_s13 gnostic_magic_s14) (.seq (.arrow gnostic_magic_l14 gnostic_magic_s14 gnostic_magic_s15) (.seq (.arrow gnostic_magic_l15 gnostic_magic_s15 gnostic_magic_s16) (.seq (.arrow gnostic_magic_l16 gnostic_magic_s16 gnostic_magic_s17) (.seq (.arrow gnostic_magic_l17 gnostic_magic_s17 gnostic_magic_s18) (.seq (.arrow gnostic_magic_l18 gnostic_magic_s18 gnostic_magic_s19) (.seq (.arrow gnostic_magic_l19 gnostic_magic_s19 gnostic_magic_s20) (.seq (.arrow gnostic_magic_l20 gnostic_magic_s20 gnostic_magic_s21) (.seq (.arrow gnostic_magic_l21 gnostic_magic_s21 gnostic_magic_s22) (.seq (.arrow gnostic_magic_l22 gnostic_magic_s22 gnostic_magic_s23) (.seq (.arrow gnostic_magic_l23 gnostic_magic_s23 gnostic_magic_s24) (.seq (.arrow gnostic_magic_l24 gnostic_magic_s24 gnostic_magic_s25) (.seq (.arrow gnostic_magic_l25 gnostic_magic_s25 gnostic_magic_s26) (.arrow gnostic_magic_l26 gnostic_magic_s26 gnostic_magic_s27)))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def gnostic_magic_true_arm : IGProtocol gnostic_magic_s0 gnostic_magic_s27 :=
  (gnostic_magic_protocol).restrictToEVALT
  --
-- false arm
noncomputable def gnostic_magic_false_arm : IGProtocol gnostic_magic_s0 gnostic_magic_s27 :=
  (gnostic_magic_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def gnostic_magic_tier : OuroboricityTier := TierFunctor.obj gnostic_magic_s0
#eval gnostic_magic_tier  -- the Grammar's own verdict on its tier
