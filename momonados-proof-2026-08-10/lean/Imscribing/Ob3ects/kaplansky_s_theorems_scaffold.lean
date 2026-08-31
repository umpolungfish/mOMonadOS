-- IGProtocol scaffold: VINIT → TANCH → FSPLIT → EVALT → AFWD → CLINK → IMSCRIB → EVALF → AREV → CLINK → FSPLIT → EVALT → CLINK → IFIX → IFIX → IFIX → ENGAGR → FFUSE → IMSCRIB → TANCH
-- Class: Kaplansky's Theorems
-- Fingerprint: sig=(10,3,4,3)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=20
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(10, 17)]

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
--   [2] FSPLIT    gran   := 𐑚               𐑡 → ⊙  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑾  | evaluate-true — criticality gate open
--   [4] AFWD      rel    := 𐑾               ⊙ → 𐑱  | forward morphism — bidirectional arrow
--   [5] CLINK     fid    := 𐑱               𐑾 → 𐑠  | composition — regime coherence
--   [6] IMSCRIB   gram   := 𐑠               𐑱 → 𐑖  | identity — self-imscription
--   [7] EVALF     chir   := 𐑖               𐑠 → 𐑗  | evaluate-false — chirality check
--   [8] AREV      pol    := 𐑗               𐑖 → 𐑱  | reverse morphism — parity flip
--   [9] CLINK     fid    := 𐑱               𐑗 → 𐑚  | composition — regime coherence
--   [10] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [11] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [12] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [13] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [14] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [15] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [16] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [17] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [18] IMSCRIB   gram   := 𐑠               𐑙 → 𐑡  | identity — self-imscription
--   [19] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def kaplanskys_theorems_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def kaplanskys_theorems_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def kaplanskys_theorems_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def kaplanskys_theorems_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def kaplanskys_theorems_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def kaplanskys_theorems_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def kaplanskys_theorems_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def kaplanskys_theorems_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def kaplanskys_theorems_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def kaplanskys_theorems_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def kaplanskys_theorems_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def kaplanskys_theorems_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def kaplanskys_theorems_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def kaplanskys_theorems_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def kaplanskys_theorems_protocol : IGProtocol kaplanskys_theorems_s0 kaplanskys_theorems_s19 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct kaplanskys_theorems_s17 kaplanskys_theorems_s17 = kaplanskys_theorems_s17 (idempotent)
  (.seq (.arrow kaplanskys_theorems_l0 kaplanskys_theorems_s0 kaplanskys_theorems_s1) (.seq (.arrow kaplanskys_theorems_l1 kaplanskys_theorems_s1 kaplanskys_theorems_s2) (.seq (.arrow kaplanskys_theorems_l2 kaplanskys_theorems_s2 kaplanskys_theorems_s3) (.seq (.arrow kaplanskys_theorems_l3 kaplanskys_theorems_s3 kaplanskys_theorems_s4) (.seq (.arrow kaplanskys_theorems_l4 kaplanskys_theorems_s4 kaplanskys_theorems_s5) (.seq (.arrow kaplanskys_theorems_l5 kaplanskys_theorems_s5 kaplanskys_theorems_s6) (.seq (.arrow kaplanskys_theorems_l6 kaplanskys_theorems_s6 kaplanskys_theorems_s7) (.seq (.arrow kaplanskys_theorems_l7 kaplanskys_theorems_s7 kaplanskys_theorems_s8) (.seq (.arrow kaplanskys_theorems_l8 kaplanskys_theorems_s8 kaplanskys_theorems_s9) (.seq (.arrow kaplanskys_theorems_l9 kaplanskys_theorems_s9 kaplanskys_theorems_s10) (.seq (.prod (.arrow kaplanskys_theorems_l10 kaplanskys_theorems_s10 kaplanskys_theorems_s17) (.arrow kaplanskys_theorems_l10 kaplanskys_theorems_s10 kaplanskys_theorems_s17)) (.seq (.arrow kaplanskys_theorems_l17 kaplanskys_theorems_s17 kaplanskys_theorems_s17) (.seq (.arrow kaplanskys_theorems_l17 kaplanskys_theorems_s17 kaplanskys_theorems_s18) (.arrow kaplanskys_theorems_l18 kaplanskys_theorems_s18 kaplanskys_theorems_s19))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def kaplanskys_theorems_true_arm : IGProtocol kaplanskys_theorems_s0 kaplanskys_theorems_s19 :=
  (kaplanskys_theorems_protocol).restrictToEVALT
  --
-- false arm
noncomputable def kaplanskys_theorems_false_arm : IGProtocol kaplanskys_theorems_s0 kaplanskys_theorems_s19 :=
  (kaplanskys_theorems_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def kaplanskys_theorems_tier : OuroboricityTier := TierFunctor.obj kaplanskys_theorems_s0
#eval kaplanskys_theorems_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem kaplanskys_theorems_frobenius :
    igFrobeniusAlg.mul kaplanskys_theorems_s0 kaplanskys_theorems_s0 = kaplanskys_theorems_s0 :=
  igFrobAlg_self_fusion kaplanskys_theorems_s0
