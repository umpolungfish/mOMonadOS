-- IGProtocol scaffold: VINIT → TANCH → IMSCRIB → AFWD → FSPLIT → EVALT → CLINK → IFIX → AREV → EVALF → CLINK → IMSCRIB → FSPLIT → ENGAGR → FFUSE → IFIX → TANCH
-- Class: No-Three-in-Line Theorem
-- Fingerprint: sig=(9,3,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=17
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(12, 14)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑠  | terminal object — connectivity boundary
--   [2] IMSCRIB   gram   := 𐑠               𐑡 → 𐑾  | identity — self-imscription
--   [3] AFWD      rel    := 𐑾               𐑠 → 𐑚  | forward morphism — bidirectional arrow
--   [4] FSPLIT    gran   := 𐑚               𐑾 → ⊙  | split δ — range decomposition
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑱  | evaluate-true — criticality gate open
--   [6] CLINK     fid    := 𐑱               ⊙ → 𐑭  | composition — regime coherence
--   [7] IFIX      prot   := 𐑭               𐑱 → 𐑗  | irreversible fixation — winding number
--   [8] AREV      pol    := 𐑗               𐑭 → 𐑖  | reverse morphism — parity flip
--   [9] EVALF     chir   := 𐑖               𐑗 → 𐑱  | evaluate-false — chirality check
--   [10] CLINK     fid    := 𐑱               𐑖 → 𐑠  | composition — regime coherence
--   [11] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [12] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [13] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [14] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [15] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [16] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def no_three_in_line_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def no_three_in_line_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def no_three_in_line_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def no_three_in_line_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def no_three_in_line_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def no_three_in_line_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def no_three_in_line_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def no_three_in_line_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def no_three_in_line_theorem_protocol : IGProtocol no_three_in_line_theorem_s0 no_three_in_line_theorem_s16 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct no_three_in_line_theorem_s14 no_three_in_line_theorem_s14 = no_three_in_line_theorem_s14 (idempotent)
  (.seq (.arrow no_three_in_line_theorem_l0 no_three_in_line_theorem_s0 no_three_in_line_theorem_s1) (.seq (.arrow no_three_in_line_theorem_l1 no_three_in_line_theorem_s1 no_three_in_line_theorem_s2) (.seq (.arrow no_three_in_line_theorem_l2 no_three_in_line_theorem_s2 no_three_in_line_theorem_s3) (.seq (.arrow no_three_in_line_theorem_l3 no_three_in_line_theorem_s3 no_three_in_line_theorem_s4) (.seq (.arrow no_three_in_line_theorem_l4 no_three_in_line_theorem_s4 no_three_in_line_theorem_s5) (.seq (.arrow no_three_in_line_theorem_l5 no_three_in_line_theorem_s5 no_three_in_line_theorem_s6) (.seq (.arrow no_three_in_line_theorem_l6 no_three_in_line_theorem_s6 no_three_in_line_theorem_s7) (.seq (.arrow no_three_in_line_theorem_l7 no_three_in_line_theorem_s7 no_three_in_line_theorem_s8) (.seq (.arrow no_three_in_line_theorem_l8 no_three_in_line_theorem_s8 no_three_in_line_theorem_s9) (.seq (.arrow no_three_in_line_theorem_l9 no_three_in_line_theorem_s9 no_three_in_line_theorem_s10) (.seq (.arrow no_three_in_line_theorem_l10 no_three_in_line_theorem_s10 no_three_in_line_theorem_s11) (.seq (.arrow no_three_in_line_theorem_l11 no_three_in_line_theorem_s11 no_three_in_line_theorem_s12) (.seq (.prod (.arrow no_three_in_line_theorem_l12 no_three_in_line_theorem_s12 no_three_in_line_theorem_s14) (.arrow no_three_in_line_theorem_l12 no_three_in_line_theorem_s12 no_three_in_line_theorem_s14)) (.seq (.arrow no_three_in_line_theorem_l14 no_three_in_line_theorem_s14 no_three_in_line_theorem_s14) (.seq (.arrow no_three_in_line_theorem_l14 no_three_in_line_theorem_s14 no_three_in_line_theorem_s15) (.arrow no_three_in_line_theorem_l15 no_three_in_line_theorem_s15 no_three_in_line_theorem_s16))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def no_three_in_line_theorem_true_arm : IGProtocol no_three_in_line_theorem_s0 no_three_in_line_theorem_s16 :=
  (no_three_in_line_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def no_three_in_line_theorem_false_arm : IGProtocol no_three_in_line_theorem_s0 no_three_in_line_theorem_s16 :=
  (no_three_in_line_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def no_three_in_line_theorem_tier : OuroboricityTier := TierFunctor.obj no_three_in_line_theorem_s0
#eval no_three_in_line_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem no_three_in_line_theorem_frobenius :
    igFrobeniusAlg.mul no_three_in_line_theorem_s0 no_three_in_line_theorem_s0 = no_three_in_line_theorem_s0 :=
  igFrobAlg_self_fusion no_three_in_line_theorem_s0
