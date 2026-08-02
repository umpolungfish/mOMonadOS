-- IGProtocol scaffold: VINIT → TANCH → IMSCRIB → AFWD → CLINK → CLINK → CLINK → FSPLIT → EVALT → IFIX → FFUSE → AREV → EVALF → IFIX → FFUSE → ENGAGR → IMSCRIB → CLINK → TANCH
-- Class: Fermat Number Compositeness Theorem
-- Fingerprint: sig=(11,3,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=19
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(7, 10)]

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
--   [3] AFWD      rel    := 𐑾               𐑠 → 𐑱  | forward morphism — bidirectional arrow
--   [4] CLINK     fid    := 𐑱               𐑾 → 𐑱  | composition — regime coherence
--   [5] CLINK     fid    := 𐑱               𐑱 → 𐑱  | composition — regime coherence
--   [6] CLINK     fid    := 𐑱               𐑱 → 𐑚  | composition — regime coherence
--   [7] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [8] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [9] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [10] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [11] AREV      pol    := 𐑗               𐑙 → 𐑖  | reverse morphism — parity flip
--   [12] EVALF     chir   := 𐑖               𐑗 → 𐑭  | evaluate-false — chirality check
--   [13] IFIX      prot   := 𐑭               𐑖 → 𐑙  | irreversible fixation — winding number
--   [14] FFUSE     stoi   := 𐑙               𐑭 → 𐑳  | fuse μ — assembly mode
--   [15] ENGAGR    stoi   := 𐑳               𐑙 → 𐑠  | engage paradox — B-state, both arms
--   [16] IMSCRIB   gram   := 𐑠               𐑳 → 𐑱  | identity — self-imscription
--   [17] CLINK     fid    := 𐑱               𐑠 → 𐑡  | composition — regime coherence
--   [18] TANCH     top    := 𐑡               𐑱 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def fermat_number_compositeness_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def fermat_number_compositeness_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def fermat_number_compositeness_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def fermat_number_compositeness_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def fermat_number_compositeness_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def fermat_number_compositeness_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def fermat_number_compositeness_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def fermat_number_compositeness_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def fermat_number_compositeness_theorem_protocol : IGProtocol fermat_number_compositeness_theorem_s0 fermat_number_compositeness_theorem_s18 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct fermat_number_compositeness_theorem_s10 fermat_number_compositeness_theorem_s10 = fermat_number_compositeness_theorem_s10 (idempotent)
  (.seq (.arrow fermat_number_compositeness_theorem_l0 fermat_number_compositeness_theorem_s0 fermat_number_compositeness_theorem_s1) (.seq (.arrow fermat_number_compositeness_theorem_l1 fermat_number_compositeness_theorem_s1 fermat_number_compositeness_theorem_s2) (.seq (.arrow fermat_number_compositeness_theorem_l2 fermat_number_compositeness_theorem_s2 fermat_number_compositeness_theorem_s3) (.seq (.arrow fermat_number_compositeness_theorem_l3 fermat_number_compositeness_theorem_s3 fermat_number_compositeness_theorem_s4) (.seq (.arrow fermat_number_compositeness_theorem_l4 fermat_number_compositeness_theorem_s4 fermat_number_compositeness_theorem_s5) (.seq (.arrow fermat_number_compositeness_theorem_l5 fermat_number_compositeness_theorem_s5 fermat_number_compositeness_theorem_s6) (.seq (.arrow fermat_number_compositeness_theorem_l6 fermat_number_compositeness_theorem_s6 fermat_number_compositeness_theorem_s7) (.seq (.prod (.arrow fermat_number_compositeness_theorem_l7 fermat_number_compositeness_theorem_s7 fermat_number_compositeness_theorem_s10) (.arrow fermat_number_compositeness_theorem_l7 fermat_number_compositeness_theorem_s7 fermat_number_compositeness_theorem_s10)) (.seq (.arrow fermat_number_compositeness_theorem_l10 fermat_number_compositeness_theorem_s10 fermat_number_compositeness_theorem_s10) (.seq (.arrow fermat_number_compositeness_theorem_l10 fermat_number_compositeness_theorem_s10 fermat_number_compositeness_theorem_s11) (.seq (.arrow fermat_number_compositeness_theorem_l11 fermat_number_compositeness_theorem_s11 fermat_number_compositeness_theorem_s12) (.seq (.arrow fermat_number_compositeness_theorem_l12 fermat_number_compositeness_theorem_s12 fermat_number_compositeness_theorem_s13) (.seq (.arrow fermat_number_compositeness_theorem_l13 fermat_number_compositeness_theorem_s13 fermat_number_compositeness_theorem_s14) (.seq (.arrow fermat_number_compositeness_theorem_l14 fermat_number_compositeness_theorem_s14 fermat_number_compositeness_theorem_s15) (.seq (.arrow fermat_number_compositeness_theorem_l15 fermat_number_compositeness_theorem_s15 fermat_number_compositeness_theorem_s16) (.seq (.arrow fermat_number_compositeness_theorem_l16 fermat_number_compositeness_theorem_s16 fermat_number_compositeness_theorem_s17) (.arrow fermat_number_compositeness_theorem_l17 fermat_number_compositeness_theorem_s17 fermat_number_compositeness_theorem_s18)))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def fermat_number_compositeness_theorem_true_arm : IGProtocol fermat_number_compositeness_theorem_s0 fermat_number_compositeness_theorem_s18 :=
  (fermat_number_compositeness_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def fermat_number_compositeness_theorem_false_arm : IGProtocol fermat_number_compositeness_theorem_s0 fermat_number_compositeness_theorem_s18 :=
  (fermat_number_compositeness_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def fermat_number_compositeness_theorem_tier : OuroboricityTier := TierFunctor.obj fermat_number_compositeness_theorem_s0
#eval fermat_number_compositeness_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem fermat_number_compositeness_theorem_frobenius :
    igFrobeniusAlg.mul fermat_number_compositeness_theorem_s0 fermat_number_compositeness_theorem_s0 = fermat_number_compositeness_theorem_s0 :=
  igFrobAlg_self_fusion fermat_number_compositeness_theorem_s0
