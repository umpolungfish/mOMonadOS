-- IGProtocol scaffold: VINIT → TANCH → FSPLIT → AFWD → EVALT → IFIX → FFUSE → AREV → CLINK → EVALT → IFIX → FFUSE → AREV → CLINK → EVALT → IFIX → FFUSE → AREV → CLINK → EVALT → IFIX → FFUSE → FFUSE → IMSCRIB → ENGAGR → IFIX → TANCH
-- Class: Erdős–Straus Theorem
-- Fingerprint: sig=(11,6,5,5)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=27
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(2, 6)]

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
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [7] AREV      pol    := 𐑗               𐑙 → 𐑱  | reverse morphism — parity flip
--   [8] CLINK     fid    := 𐑱               𐑗 → ⊙  | composition — regime coherence
--   [9] EVALT     crit   := ⊙               𐑱 → 𐑭  | evaluate-true — criticality gate open
--   [10] IFIX      prot   := 𐑭               ⊙ → 𐑙  | irreversible fixation — winding number
--   [11] FFUSE     stoi   := 𐑙               𐑭 → 𐑗  | fuse μ — assembly mode
--   [12] AREV      pol    := 𐑗               𐑙 → 𐑱  | reverse morphism — parity flip
--   [13] CLINK     fid    := 𐑱               𐑗 → ⊙  | composition — regime coherence
--   [14] EVALT     crit   := ⊙               𐑱 → 𐑭  | evaluate-true — criticality gate open
--   [15] IFIX      prot   := 𐑭               ⊙ → 𐑙  | irreversible fixation — winding number
--   [16] FFUSE     stoi   := 𐑙               𐑭 → 𐑗  | fuse μ — assembly mode
--   [17] AREV      pol    := 𐑗               𐑙 → 𐑱  | reverse morphism — parity flip
--   [18] CLINK     fid    := 𐑱               𐑗 → ⊙  | composition — regime coherence
--   [19] EVALT     crit   := ⊙               𐑱 → 𐑭  | evaluate-true — criticality gate open
--   [20] IFIX      prot   := 𐑭               ⊙ → 𐑙  | irreversible fixation — winding number
--   [21] FFUSE     stoi   := 𐑙               𐑭 → 𐑙  | fuse μ — assembly mode
--   [22] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [23] IMSCRIB   gram   := 𐑠               𐑙 → 𐑳  | identity — self-imscription
--   [24] ENGAGR    stoi   := 𐑳               𐑠 → 𐑭  | engage paradox — B-state, both arms
--   [25] IFIX      prot   := 𐑭               𐑳 → 𐑡  | irreversible fixation — winding number
--   [26] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def erd_s_straus_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def erd_s_straus_theorem_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def erd_s_straus_theorem_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def erd_s_straus_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def erd_s_straus_theorem_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def erd_s_straus_theorem_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def erd_s_straus_theorem_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def erd_s_straus_theorem_protocol : IGProtocol erd_s_straus_theorem_s0 erd_s_straus_theorem_s26 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct erd_s_straus_theorem_s6 erd_s_straus_theorem_s6 = erd_s_straus_theorem_s6 (idempotent)
  (.seq (.arrow erd_s_straus_theorem_l0 erd_s_straus_theorem_s0 erd_s_straus_theorem_s1) (.seq (.arrow erd_s_straus_theorem_l1 erd_s_straus_theorem_s1 erd_s_straus_theorem_s2) (.seq (.prod (.arrow erd_s_straus_theorem_l2 erd_s_straus_theorem_s2 erd_s_straus_theorem_s6) (.arrow erd_s_straus_theorem_l2 erd_s_straus_theorem_s2 erd_s_straus_theorem_s6)) (.seq (.arrow erd_s_straus_theorem_l6 erd_s_straus_theorem_s6 erd_s_straus_theorem_s6) (.seq (.arrow erd_s_straus_theorem_l6 erd_s_straus_theorem_s6 erd_s_straus_theorem_s7) (.seq (.arrow erd_s_straus_theorem_l7 erd_s_straus_theorem_s7 erd_s_straus_theorem_s8) (.seq (.arrow erd_s_straus_theorem_l8 erd_s_straus_theorem_s8 erd_s_straus_theorem_s9) (.seq (.arrow erd_s_straus_theorem_l9 erd_s_straus_theorem_s9 erd_s_straus_theorem_s10) (.seq (.arrow erd_s_straus_theorem_l10 erd_s_straus_theorem_s10 erd_s_straus_theorem_s11) (.seq (.arrow erd_s_straus_theorem_l11 erd_s_straus_theorem_s11 erd_s_straus_theorem_s12) (.seq (.arrow erd_s_straus_theorem_l12 erd_s_straus_theorem_s12 erd_s_straus_theorem_s13) (.seq (.arrow erd_s_straus_theorem_l13 erd_s_straus_theorem_s13 erd_s_straus_theorem_s14) (.seq (.arrow erd_s_straus_theorem_l14 erd_s_straus_theorem_s14 erd_s_straus_theorem_s15) (.seq (.arrow erd_s_straus_theorem_l15 erd_s_straus_theorem_s15 erd_s_straus_theorem_s16) (.seq (.arrow erd_s_straus_theorem_l16 erd_s_straus_theorem_s16 erd_s_straus_theorem_s17) (.seq (.arrow erd_s_straus_theorem_l17 erd_s_straus_theorem_s17 erd_s_straus_theorem_s18) (.seq (.arrow erd_s_straus_theorem_l18 erd_s_straus_theorem_s18 erd_s_straus_theorem_s19) (.seq (.arrow erd_s_straus_theorem_l19 erd_s_straus_theorem_s19 erd_s_straus_theorem_s20) (.seq (.arrow erd_s_straus_theorem_l20 erd_s_straus_theorem_s20 erd_s_straus_theorem_s21) (.seq (.arrow erd_s_straus_theorem_l21 erd_s_straus_theorem_s21 erd_s_straus_theorem_s22) (.seq (.arrow erd_s_straus_theorem_l22 erd_s_straus_theorem_s22 erd_s_straus_theorem_s23) (.seq (.arrow erd_s_straus_theorem_l23 erd_s_straus_theorem_s23 erd_s_straus_theorem_s24) (.seq (.arrow erd_s_straus_theorem_l24 erd_s_straus_theorem_s24 erd_s_straus_theorem_s25) (.arrow erd_s_straus_theorem_l25 erd_s_straus_theorem_s25 erd_s_straus_theorem_s26))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def erd_s_straus_theorem_true_arm : IGProtocol erd_s_straus_theorem_s0 erd_s_straus_theorem_s26 :=
  (erd_s_straus_theorem_protocol).restrictToEVALT
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def erd_s_straus_theorem_tier : OuroboricityTier := TierFunctor.obj erd_s_straus_theorem_s0
#eval erd_s_straus_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem erd_s_straus_theorem_frobenius :
    igFrobeniusAlg.mul erd_s_straus_theorem_s0 erd_s_straus_theorem_s0 = erd_s_straus_theorem_s0 :=
  igFrobAlg_self_fusion erd_s_straus_theorem_s0
