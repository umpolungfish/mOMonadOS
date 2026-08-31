-- IGProtocol scaffold: VINIT → ENGAGR → AFWD → AREV → CLINK → FSPLIT → EVALT → FFUSE → CLINK → AFWD → IMSCRIB → AFWD → EVALT → IFIX → IMSCRIB → TANCH
-- Class: Operation of Voynich Manuscript
-- Fingerprint: sig=(10,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=16
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 7)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑳  | initial object — ground of distinction
--   [1] ENGAGR    stoi   := 𐑳               𐑼 → 𐑾  | engage paradox — B-state, both arms
--   [2] AFWD      rel    := 𐑾               𐑳 → 𐑗  | forward morphism — bidirectional arrow
--   [3] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [4] CLINK     fid    := 𐑱               𐑗 → 𐑚  | composition — regime coherence
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑾  | composition — regime coherence
--   [9] AFWD      rel    := 𐑾               𐑱 → 𐑠  | forward morphism — bidirectional arrow
--   [10] IMSCRIB   gram   := 𐑠               𐑾 → 𐑾  | identity — self-imscription
--   [11] AFWD      rel    := 𐑾               𐑠 → ⊙  | forward morphism — bidirectional arrow
--   [12] EVALT     crit   := ⊙               𐑾 → 𐑭  | evaluate-true — criticality gate open
--   [13] IFIX      prot   := 𐑭               ⊙ → 𐑠  | irreversible fixation — winding number
--   [14] IMSCRIB   gram   := 𐑠               𐑭 → 𐑡  | identity — self-imscription
--   [15] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def operation_of_voynich_manuscript_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def operation_of_voynich_manuscript_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def operation_of_voynich_manuscript_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def operation_of_voynich_manuscript_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_voynich_manuscript_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l9 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def operation_of_voynich_manuscript_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_voynich_manuscript_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def operation_of_voynich_manuscript_protocol : IGProtocol operation_of_voynich_manuscript_s0 operation_of_voynich_manuscript_s15 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct operation_of_voynich_manuscript_s7 operation_of_voynich_manuscript_s7 = operation_of_voynich_manuscript_s7 (idempotent)
  (.seq (.arrow operation_of_voynich_manuscript_l0 operation_of_voynich_manuscript_s0 operation_of_voynich_manuscript_s1) (.seq (.arrow operation_of_voynich_manuscript_l1 operation_of_voynich_manuscript_s1 operation_of_voynich_manuscript_s2) (.seq (.arrow operation_of_voynich_manuscript_l2 operation_of_voynich_manuscript_s2 operation_of_voynich_manuscript_s3) (.seq (.arrow operation_of_voynich_manuscript_l3 operation_of_voynich_manuscript_s3 operation_of_voynich_manuscript_s4) (.seq (.arrow operation_of_voynich_manuscript_l4 operation_of_voynich_manuscript_s4 operation_of_voynich_manuscript_s5) (.seq (.prod (.arrow operation_of_voynich_manuscript_l5 operation_of_voynich_manuscript_s5 operation_of_voynich_manuscript_s7) (.arrow operation_of_voynich_manuscript_l5 operation_of_voynich_manuscript_s5 operation_of_voynich_manuscript_s7)) (.seq (.arrow operation_of_voynich_manuscript_l7 operation_of_voynich_manuscript_s7 operation_of_voynich_manuscript_s7) (.seq (.arrow operation_of_voynich_manuscript_l7 operation_of_voynich_manuscript_s7 operation_of_voynich_manuscript_s8) (.seq (.arrow operation_of_voynich_manuscript_l8 operation_of_voynich_manuscript_s8 operation_of_voynich_manuscript_s9) (.seq (.arrow operation_of_voynich_manuscript_l9 operation_of_voynich_manuscript_s9 operation_of_voynich_manuscript_s10) (.seq (.arrow operation_of_voynich_manuscript_l10 operation_of_voynich_manuscript_s10 operation_of_voynich_manuscript_s11) (.seq (.arrow operation_of_voynich_manuscript_l11 operation_of_voynich_manuscript_s11 operation_of_voynich_manuscript_s12) (.seq (.arrow operation_of_voynich_manuscript_l12 operation_of_voynich_manuscript_s12 operation_of_voynich_manuscript_s13) (.seq (.arrow operation_of_voynich_manuscript_l13 operation_of_voynich_manuscript_s13 operation_of_voynich_manuscript_s14) (.arrow operation_of_voynich_manuscript_l14 operation_of_voynich_manuscript_s14 operation_of_voynich_manuscript_s15)))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def operation_of_voynich_manuscript_true_arm : IGProtocol operation_of_voynich_manuscript_s0 operation_of_voynich_manuscript_s15 :=
  (operation_of_voynich_manuscript_protocol).restrictToEVALT
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def operation_of_voynich_manuscript_tier : OuroboricityTier := TierFunctor.obj operation_of_voynich_manuscript_s0
#eval operation_of_voynich_manuscript_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem operation_of_voynich_manuscript_frobenius :
    igFrobeniusAlg.mul operation_of_voynich_manuscript_s0 operation_of_voynich_manuscript_s0 = operation_of_voynich_manuscript_s0 :=
  igFrobAlg_self_fusion operation_of_voynich_manuscript_s0
