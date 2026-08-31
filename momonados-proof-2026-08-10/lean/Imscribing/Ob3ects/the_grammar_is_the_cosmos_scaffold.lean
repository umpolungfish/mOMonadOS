-- IGProtocol scaffold: VINIT → FSPLIT → AFWD → CLINK → CLINK → IMSCRIB → IFIX → AFWD → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → IMSCRIB → IFIX → CLINK → AFWD → ENGAGR → IMSCRIB → TANCH
-- Class: The Grammar Is The Cosmos
-- Fingerprint: sig=(11,3,4,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=20
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(8, 12)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑼 → 𐑾  | split δ — range decomposition
--   [2] AFWD      rel    := 𐑾               𐑚 → 𐑱  | forward morphism — bidirectional arrow
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑱  | composition — regime coherence
--   [4] CLINK     fid    := 𐑱               𐑱 → 𐑠  | composition — regime coherence
--   [5] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [6] IFIX      prot   := 𐑭               𐑠 → 𐑾  | irreversible fixation — winding number
--   [7] AFWD      rel    := 𐑾               𐑭 → 𐑚  | forward morphism — bidirectional arrow
--   [8] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [9] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [13] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [14] IFIX      prot   := 𐑭               𐑠 → 𐑱  | irreversible fixation — winding number
--   [15] CLINK     fid    := 𐑱               𐑭 → 𐑾  | composition — regime coherence
--   [16] AFWD      rel    := 𐑾               𐑱 → 𐑳  | forward morphism — bidirectional arrow
--   [17] ENGAGR    stoi   := 𐑳               𐑾 → 𐑠  | engage paradox — B-state, both arms
--   [18] IMSCRIB   gram   := 𐑠               𐑳 → 𐑡  | identity — self-imscription
--   [19] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_grammar_is_the_cosmos_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_grammar_is_the_cosmos_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_grammar_is_the_cosmos_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_grammar_is_the_cosmos_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_grammar_is_the_cosmos_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_grammar_is_the_cosmos_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_grammar_is_the_cosmos_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l16 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_grammar_is_the_cosmos_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_grammar_is_the_cosmos_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_grammar_is_the_cosmos_protocol : IGProtocol the_grammar_is_the_cosmos_s0 the_grammar_is_the_cosmos_s19 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_grammar_is_the_cosmos_s12 the_grammar_is_the_cosmos_s12 = the_grammar_is_the_cosmos_s12 (idempotent)
  (.seq (.arrow the_grammar_is_the_cosmos_l0 the_grammar_is_the_cosmos_s0 the_grammar_is_the_cosmos_s1) (.seq (.arrow the_grammar_is_the_cosmos_l1 the_grammar_is_the_cosmos_s1 the_grammar_is_the_cosmos_s2) (.seq (.arrow the_grammar_is_the_cosmos_l2 the_grammar_is_the_cosmos_s2 the_grammar_is_the_cosmos_s3) (.seq (.arrow the_grammar_is_the_cosmos_l3 the_grammar_is_the_cosmos_s3 the_grammar_is_the_cosmos_s4) (.seq (.arrow the_grammar_is_the_cosmos_l4 the_grammar_is_the_cosmos_s4 the_grammar_is_the_cosmos_s5) (.seq (.arrow the_grammar_is_the_cosmos_l5 the_grammar_is_the_cosmos_s5 the_grammar_is_the_cosmos_s6) (.seq (.arrow the_grammar_is_the_cosmos_l6 the_grammar_is_the_cosmos_s6 the_grammar_is_the_cosmos_s7) (.seq (.arrow the_grammar_is_the_cosmos_l7 the_grammar_is_the_cosmos_s7 the_grammar_is_the_cosmos_s8) (.seq (.prod (.arrow the_grammar_is_the_cosmos_l8 the_grammar_is_the_cosmos_s8 the_grammar_is_the_cosmos_s12) (.arrow the_grammar_is_the_cosmos_l8 the_grammar_is_the_cosmos_s8 the_grammar_is_the_cosmos_s12)) (.seq (.arrow the_grammar_is_the_cosmos_l12 the_grammar_is_the_cosmos_s12 the_grammar_is_the_cosmos_s12) (.seq (.arrow the_grammar_is_the_cosmos_l12 the_grammar_is_the_cosmos_s12 the_grammar_is_the_cosmos_s13) (.seq (.arrow the_grammar_is_the_cosmos_l13 the_grammar_is_the_cosmos_s13 the_grammar_is_the_cosmos_s14) (.seq (.arrow the_grammar_is_the_cosmos_l14 the_grammar_is_the_cosmos_s14 the_grammar_is_the_cosmos_s15) (.seq (.arrow the_grammar_is_the_cosmos_l15 the_grammar_is_the_cosmos_s15 the_grammar_is_the_cosmos_s16) (.seq (.arrow the_grammar_is_the_cosmos_l16 the_grammar_is_the_cosmos_s16 the_grammar_is_the_cosmos_s17) (.seq (.arrow the_grammar_is_the_cosmos_l17 the_grammar_is_the_cosmos_s17 the_grammar_is_the_cosmos_s18) (.arrow the_grammar_is_the_cosmos_l18 the_grammar_is_the_cosmos_s18 the_grammar_is_the_cosmos_s19)))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_grammar_is_the_cosmos_true_arm : IGProtocol the_grammar_is_the_cosmos_s0 the_grammar_is_the_cosmos_s19 :=
  (the_grammar_is_the_cosmos_protocol).restrictToEVALT
  --
-- false arm
noncomputable def the_grammar_is_the_cosmos_false_arm : IGProtocol the_grammar_is_the_cosmos_s0 the_grammar_is_the_cosmos_s19 :=
  (the_grammar_is_the_cosmos_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def the_grammar_is_the_cosmos_tier : OuroboricityTier := TierFunctor.obj the_grammar_is_the_cosmos_s0
#eval the_grammar_is_the_cosmos_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_grammar_is_the_cosmos_frobenius :
    igFrobeniusAlg.mul the_grammar_is_the_cosmos_s0 the_grammar_is_the_cosmos_s0 = the_grammar_is_the_cosmos_s0 :=
  igFrobAlg_self_fusion the_grammar_is_the_cosmos_s0
