-- IGProtocol scaffold: VINIT → AFWD → AFWD → AFWD → CLINK → CLINK → CLINK → CLINK → FSPLIT → EVALT → AFWD → IFIX → FFUSE → EVALF → AREV → IFIX → FFUSE → ENGAGR → IMSCRIB → TANCH
-- Class: Casas-Alvero Theorem
-- Fingerprint: sig=(12,3,3,2)
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
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑾  | forward morphism — bidirectional arrow
--   [2] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑾 → 𐑱  | forward morphism — bidirectional arrow
--   [4] CLINK     fid    := 𐑱               𐑾 → 𐑱  | composition — regime coherence
--   [5] CLINK     fid    := 𐑱               𐑱 → 𐑱  | composition — regime coherence
--   [6] CLINK     fid    := 𐑱               𐑱 → 𐑱  | composition — regime coherence
--   [7] CLINK     fid    := 𐑱               𐑱 → 𐑚  | composition — regime coherence
--   [8] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [9] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [10] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [11] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [13] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [14] AREV      pol    := 𐑗               𐑖 → 𐑭  | reverse morphism — parity flip
--   [15] IFIX      prot   := 𐑭               𐑗 → 𐑙  | irreversible fixation — winding number
--   [16] FFUSE     stoi   := 𐑙               𐑭 → 𐑳  | fuse μ — assembly mode
--   [17] ENGAGR    stoi   := 𐑳               𐑙 → 𐑠  | engage paradox — B-state, both arms
--   [18] IMSCRIB   gram   := 𐑠               𐑳 → 𐑡  | identity — self-imscription
--   [19] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def casas_alvero_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def casas_alvero_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def casas_alvero_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def casas_alvero_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def casas_alvero_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def casas_alvero_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def casas_alvero_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def casas_alvero_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def casas_alvero_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def casas_alvero_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def casas_alvero_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def casas_alvero_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def casas_alvero_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def casas_alvero_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def casas_alvero_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def casas_alvero_theorem_protocol : IGProtocol casas_alvero_theorem_s0 casas_alvero_theorem_s19 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct casas_alvero_theorem_s12 casas_alvero_theorem_s12 = casas_alvero_theorem_s12 (idempotent)
  (.seq (.arrow casas_alvero_theorem_l0 casas_alvero_theorem_s0 casas_alvero_theorem_s1) (.seq (.arrow casas_alvero_theorem_l1 casas_alvero_theorem_s1 casas_alvero_theorem_s2) (.seq (.arrow casas_alvero_theorem_l2 casas_alvero_theorem_s2 casas_alvero_theorem_s3) (.seq (.arrow casas_alvero_theorem_l3 casas_alvero_theorem_s3 casas_alvero_theorem_s4) (.seq (.arrow casas_alvero_theorem_l4 casas_alvero_theorem_s4 casas_alvero_theorem_s5) (.seq (.arrow casas_alvero_theorem_l5 casas_alvero_theorem_s5 casas_alvero_theorem_s6) (.seq (.arrow casas_alvero_theorem_l6 casas_alvero_theorem_s6 casas_alvero_theorem_s7) (.seq (.arrow casas_alvero_theorem_l7 casas_alvero_theorem_s7 casas_alvero_theorem_s8) (.seq (.prod (.arrow casas_alvero_theorem_l8 casas_alvero_theorem_s8 casas_alvero_theorem_s12) (.arrow casas_alvero_theorem_l8 casas_alvero_theorem_s8 casas_alvero_theorem_s12)) (.seq (.arrow casas_alvero_theorem_l12 casas_alvero_theorem_s12 casas_alvero_theorem_s12) (.seq (.arrow casas_alvero_theorem_l12 casas_alvero_theorem_s12 casas_alvero_theorem_s13) (.seq (.arrow casas_alvero_theorem_l13 casas_alvero_theorem_s13 casas_alvero_theorem_s14) (.seq (.arrow casas_alvero_theorem_l14 casas_alvero_theorem_s14 casas_alvero_theorem_s15) (.seq (.arrow casas_alvero_theorem_l15 casas_alvero_theorem_s15 casas_alvero_theorem_s16) (.seq (.arrow casas_alvero_theorem_l16 casas_alvero_theorem_s16 casas_alvero_theorem_s17) (.seq (.arrow casas_alvero_theorem_l17 casas_alvero_theorem_s17 casas_alvero_theorem_s18) (.arrow casas_alvero_theorem_l18 casas_alvero_theorem_s18 casas_alvero_theorem_s19)))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def casas_alvero_theorem_true_arm : IGProtocol casas_alvero_theorem_s0 casas_alvero_theorem_s19 :=
  (casas_alvero_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def casas_alvero_theorem_false_arm : IGProtocol casas_alvero_theorem_s0 casas_alvero_theorem_s19 :=
  (casas_alvero_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def casas_alvero_theorem_tier : OuroboricityTier := TierFunctor.obj casas_alvero_theorem_s0
#eval casas_alvero_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem casas_alvero_theorem_frobenius :
    igFrobeniusAlg.mul casas_alvero_theorem_s0 casas_alvero_theorem_s0 = casas_alvero_theorem_s0 :=
  igFrobAlg_self_fusion casas_alvero_theorem_s0
