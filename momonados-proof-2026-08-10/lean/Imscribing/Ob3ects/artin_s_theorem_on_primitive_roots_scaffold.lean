-- IGProtocol scaffold: VINIT → TANCH → AFWD → CLINK → IMSCRIB → FSPLIT → AFWD → EVALT → AREV → EVALF → FFUSE → EVALT → IFIX → CLINK → IMSCRIB → ENGAGR → AREV → CLINK → AFWD → IMSCRIB → IFIX → TANCH
-- Class: Artin's Theorem on Primitive Roots
-- Fingerprint: sig=(14,2,4,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=22
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 10)]

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
--   [6] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [10] FFUSE     stoi   := 𐑙               𐑙 → ⊙  | fuse μ — assembly mode
--   [11] EVALT     crit   := ⊙               𐑙 → 𐑭  | evaluate-true — criticality gate open
--   [12] IFIX      prot   := 𐑭               ⊙ → 𐑱  | irreversible fixation — winding number
--   [13] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [14] IMSCRIB   gram   := 𐑠               𐑱 → 𐑳  | identity — self-imscription
--   [15] ENGAGR    stoi   := 𐑳               𐑠 → 𐑗  | engage paradox — B-state, both arms
--   [16] AREV      pol    := 𐑗               𐑳 → 𐑱  | reverse morphism — parity flip
--   [17] CLINK     fid    := 𐑱               𐑗 → 𐑾  | composition — regime coherence
--   [18] AFWD      rel    := 𐑾               𐑱 → 𐑠  | forward morphism — bidirectional arrow
--   [19] IMSCRIB   gram   := 𐑠               𐑾 → 𐑭  | identity — self-imscription
--   [20] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [21] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def artins_theorem_on_primitive_roots_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def artins_theorem_on_primitive_roots_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def artins_theorem_on_primitive_roots_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def artins_theorem_on_primitive_roots_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def artins_theorem_on_primitive_roots_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def artins_theorem_on_primitive_roots_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def artins_theorem_on_primitive_roots_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def artins_theorem_on_primitive_roots_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def artins_theorem_on_primitive_roots_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def artins_theorem_on_primitive_roots_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def artins_theorem_on_primitive_roots_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def artins_theorem_on_primitive_roots_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def artins_theorem_on_primitive_roots_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l18 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def artins_theorem_on_primitive_roots_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def artins_theorem_on_primitive_roots_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def artins_theorem_on_primitive_roots_protocol : IGProtocol artins_theorem_on_primitive_roots_s0 artins_theorem_on_primitive_roots_s21 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct artins_theorem_on_primitive_roots_s10 artins_theorem_on_primitive_roots_s10 = artins_theorem_on_primitive_roots_s10 (idempotent)
  (.seq (.arrow artins_theorem_on_primitive_roots_l0 artins_theorem_on_primitive_roots_s0 artins_theorem_on_primitive_roots_s1) (.seq (.arrow artins_theorem_on_primitive_roots_l1 artins_theorem_on_primitive_roots_s1 artins_theorem_on_primitive_roots_s2) (.seq (.arrow artins_theorem_on_primitive_roots_l2 artins_theorem_on_primitive_roots_s2 artins_theorem_on_primitive_roots_s3) (.seq (.arrow artins_theorem_on_primitive_roots_l3 artins_theorem_on_primitive_roots_s3 artins_theorem_on_primitive_roots_s4) (.seq (.arrow artins_theorem_on_primitive_roots_l4 artins_theorem_on_primitive_roots_s4 artins_theorem_on_primitive_roots_s5) (.seq (.prod (.arrow artins_theorem_on_primitive_roots_l5 artins_theorem_on_primitive_roots_s5 artins_theorem_on_primitive_roots_s10) (.arrow artins_theorem_on_primitive_roots_l5 artins_theorem_on_primitive_roots_s5 artins_theorem_on_primitive_roots_s10)) (.seq (.arrow artins_theorem_on_primitive_roots_l10 artins_theorem_on_primitive_roots_s10 artins_theorem_on_primitive_roots_s10) (.seq (.arrow artins_theorem_on_primitive_roots_l10 artins_theorem_on_primitive_roots_s10 artins_theorem_on_primitive_roots_s11) (.seq (.arrow artins_theorem_on_primitive_roots_l11 artins_theorem_on_primitive_roots_s11 artins_theorem_on_primitive_roots_s12) (.seq (.arrow artins_theorem_on_primitive_roots_l12 artins_theorem_on_primitive_roots_s12 artins_theorem_on_primitive_roots_s13) (.seq (.arrow artins_theorem_on_primitive_roots_l13 artins_theorem_on_primitive_roots_s13 artins_theorem_on_primitive_roots_s14) (.seq (.arrow artins_theorem_on_primitive_roots_l14 artins_theorem_on_primitive_roots_s14 artins_theorem_on_primitive_roots_s15) (.seq (.arrow artins_theorem_on_primitive_roots_l15 artins_theorem_on_primitive_roots_s15 artins_theorem_on_primitive_roots_s16) (.seq (.arrow artins_theorem_on_primitive_roots_l16 artins_theorem_on_primitive_roots_s16 artins_theorem_on_primitive_roots_s17) (.seq (.arrow artins_theorem_on_primitive_roots_l17 artins_theorem_on_primitive_roots_s17 artins_theorem_on_primitive_roots_s18) (.seq (.arrow artins_theorem_on_primitive_roots_l18 artins_theorem_on_primitive_roots_s18 artins_theorem_on_primitive_roots_s19) (.seq (.arrow artins_theorem_on_primitive_roots_l19 artins_theorem_on_primitive_roots_s19 artins_theorem_on_primitive_roots_s20) (.arrow artins_theorem_on_primitive_roots_l20 artins_theorem_on_primitive_roots_s20 artins_theorem_on_primitive_roots_s21))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def artins_theorem_on_primitive_roots_true_arm : IGProtocol artins_theorem_on_primitive_roots_s0 artins_theorem_on_primitive_roots_s21 :=
  (artins_theorem_on_primitive_roots_protocol).restrictToEVALT
  --
-- false arm
noncomputable def artins_theorem_on_primitive_roots_false_arm : IGProtocol artins_theorem_on_primitive_roots_s0 artins_theorem_on_primitive_roots_s21 :=
  (artins_theorem_on_primitive_roots_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def artins_theorem_on_primitive_roots_tier : OuroboricityTier := TierFunctor.obj artins_theorem_on_primitive_roots_s0
#eval artins_theorem_on_primitive_roots_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem artins_theorem_on_primitive_roots_frobenius :
    igFrobeniusAlg.mul artins_theorem_on_primitive_roots_s0 artins_theorem_on_primitive_roots_s0 = artins_theorem_on_primitive_roots_s0 :=
  igFrobAlg_self_fusion artins_theorem_on_primitive_roots_s0
