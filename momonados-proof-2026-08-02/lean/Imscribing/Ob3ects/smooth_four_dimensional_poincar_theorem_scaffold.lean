-- IGProtocol scaffold: VINIT → TANCH → AFWD → CLINK → IMSCRIB → FSPLIT → EVALT → AFWD → CLINK → IMSCRIB → EVALF → AREV → CLINK → IMSCRIB → FFUSE → ENGAGR → IFIX → CLINK → IMSCRIB → IFIX → AREV → CLINK → IMSCRIB → TANCH
-- Class: Smooth Four-Dimensional Poincaré Theorem
-- Fingerprint: sig=(17,2,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=24
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 14)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑾  | terminal object — connectivity boundary
--   [2] AFWD      rel    := 𐑾               𐑡 → 𐑱  | forward morphism — bidirectional arrow
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑠  | composition — regime coherence
--   [4] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [8] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [9] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [12] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [13] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [14] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [15] ENGAGR    stoi   := 𐑳               𐑙 → 𐑭  | engage paradox — B-state, both arms
--   [16] IFIX      prot   := 𐑭               𐑳 → 𐑱  | irreversible fixation — winding number
--   [17] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [18] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [19] IFIX      prot   := 𐑭               𐑠 → 𐑗  | irreversible fixation — winding number
--   [20] AREV      pol    := 𐑗               𐑭 → 𐑱  | reverse morphism — parity flip
--   [21] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [22] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [23] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def smooth_four_dimensional_poincar_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def smooth_four_dimensional_poincar_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def smooth_four_dimensional_poincar_theorem_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def smooth_four_dimensional_poincar_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def smooth_four_dimensional_poincar_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def smooth_four_dimensional_poincar_theorem_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def smooth_four_dimensional_poincar_theorem_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def smooth_four_dimensional_poincar_theorem_protocol : IGProtocol smooth_four_dimensional_poincar_theorem_s0 smooth_four_dimensional_poincar_theorem_s23 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct smooth_four_dimensional_poincar_theorem_s14 smooth_four_dimensional_poincar_theorem_s14 = smooth_four_dimensional_poincar_theorem_s14 (idempotent)
  (.seq (.arrow smooth_four_dimensional_poincar_theorem_l0 smooth_four_dimensional_poincar_theorem_s0 smooth_four_dimensional_poincar_theorem_s1) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l1 smooth_four_dimensional_poincar_theorem_s1 smooth_four_dimensional_poincar_theorem_s2) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l2 smooth_four_dimensional_poincar_theorem_s2 smooth_four_dimensional_poincar_theorem_s3) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l3 smooth_four_dimensional_poincar_theorem_s3 smooth_four_dimensional_poincar_theorem_s4) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l4 smooth_four_dimensional_poincar_theorem_s4 smooth_four_dimensional_poincar_theorem_s5) (.seq (.prod (.arrow smooth_four_dimensional_poincar_theorem_l5 smooth_four_dimensional_poincar_theorem_s5 smooth_four_dimensional_poincar_theorem_s14) (.arrow smooth_four_dimensional_poincar_theorem_l5 smooth_four_dimensional_poincar_theorem_s5 smooth_four_dimensional_poincar_theorem_s14)) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l14 smooth_four_dimensional_poincar_theorem_s14 smooth_four_dimensional_poincar_theorem_s14) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l14 smooth_four_dimensional_poincar_theorem_s14 smooth_four_dimensional_poincar_theorem_s15) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l15 smooth_four_dimensional_poincar_theorem_s15 smooth_four_dimensional_poincar_theorem_s16) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l16 smooth_four_dimensional_poincar_theorem_s16 smooth_four_dimensional_poincar_theorem_s17) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l17 smooth_four_dimensional_poincar_theorem_s17 smooth_four_dimensional_poincar_theorem_s18) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l18 smooth_four_dimensional_poincar_theorem_s18 smooth_four_dimensional_poincar_theorem_s19) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l19 smooth_four_dimensional_poincar_theorem_s19 smooth_four_dimensional_poincar_theorem_s20) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l20 smooth_four_dimensional_poincar_theorem_s20 smooth_four_dimensional_poincar_theorem_s21) (.seq (.arrow smooth_four_dimensional_poincar_theorem_l21 smooth_four_dimensional_poincar_theorem_s21 smooth_four_dimensional_poincar_theorem_s22) (.arrow smooth_four_dimensional_poincar_theorem_l22 smooth_four_dimensional_poincar_theorem_s22 smooth_four_dimensional_poincar_theorem_s23))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def smooth_four_dimensional_poincar_theorem_true_arm : IGProtocol smooth_four_dimensional_poincar_theorem_s0 smooth_four_dimensional_poincar_theorem_s23 :=
  (smooth_four_dimensional_poincar_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def smooth_four_dimensional_poincar_theorem_false_arm : IGProtocol smooth_four_dimensional_poincar_theorem_s0 smooth_four_dimensional_poincar_theorem_s23 :=
  (smooth_four_dimensional_poincar_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def smooth_four_dimensional_poincar_theorem_tier : OuroboricityTier := TierFunctor.obj smooth_four_dimensional_poincar_theorem_s0
#eval smooth_four_dimensional_poincar_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem smooth_four_dimensional_poincar_theorem_frobenius :
    igFrobeniusAlg.mul smooth_four_dimensional_poincar_theorem_s0 smooth_four_dimensional_poincar_theorem_s0 = smooth_four_dimensional_poincar_theorem_s0 :=
  igFrobAlg_self_fusion smooth_four_dimensional_poincar_theorem_s0
