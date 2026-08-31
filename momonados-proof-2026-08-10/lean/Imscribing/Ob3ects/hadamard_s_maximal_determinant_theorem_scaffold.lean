-- IGProtocol scaffold: VINIT → TANCH → AFWD → CLINK → FSPLIT → EVALT → AFWD → EVALF → AREV → FFUSE → IMSCRIB → CLINK → EVALT → IFIX → ENGAGR → AREV → IMSCRIB → CLINK → TANCH
-- Class: Hadamard's Maximal Determinant Theorem
-- Fingerprint: sig=(12,2,4,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=19
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(4, 9)]

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
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [4] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [10] IMSCRIB   gram   := 𐑠               𐑙 → 𐑱  | identity — self-imscription
--   [11] CLINK     fid    := 𐑱               𐑠 → ⊙  | composition — regime coherence
--   [12] EVALT     crit   := ⊙               𐑱 → 𐑭  | evaluate-true — criticality gate open
--   [13] IFIX      prot   := 𐑭               ⊙ → 𐑳  | irreversible fixation — winding number
--   [14] ENGAGR    stoi   := 𐑳               𐑭 → 𐑗  | engage paradox — B-state, both arms
--   [15] AREV      pol    := 𐑗               𐑳 → 𐑠  | reverse morphism — parity flip
--   [16] IMSCRIB   gram   := 𐑠               𐑗 → 𐑱  | identity — self-imscription
--   [17] CLINK     fid    := 𐑱               𐑠 → 𐑡  | composition — regime coherence
--   [18] TANCH     top    := 𐑡               𐑱 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def hadamards_maximal_determinant_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def hadamards_maximal_determinant_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def hadamards_maximal_determinant_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def hadamards_maximal_determinant_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def hadamards_maximal_determinant_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def hadamards_maximal_determinant_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def hadamards_maximal_determinant_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def hadamards_maximal_determinant_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def hadamards_maximal_determinant_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def hadamards_maximal_determinant_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def hadamards_maximal_determinant_theorem_protocol : IGProtocol hadamards_maximal_determinant_theorem_s0 hadamards_maximal_determinant_theorem_s18 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct hadamards_maximal_determinant_theorem_s9 hadamards_maximal_determinant_theorem_s9 = hadamards_maximal_determinant_theorem_s9 (idempotent)
  (.seq (.arrow hadamards_maximal_determinant_theorem_l0 hadamards_maximal_determinant_theorem_s0 hadamards_maximal_determinant_theorem_s1) (.seq (.arrow hadamards_maximal_determinant_theorem_l1 hadamards_maximal_determinant_theorem_s1 hadamards_maximal_determinant_theorem_s2) (.seq (.arrow hadamards_maximal_determinant_theorem_l2 hadamards_maximal_determinant_theorem_s2 hadamards_maximal_determinant_theorem_s3) (.seq (.arrow hadamards_maximal_determinant_theorem_l3 hadamards_maximal_determinant_theorem_s3 hadamards_maximal_determinant_theorem_s4) (.seq (.prod (.arrow hadamards_maximal_determinant_theorem_l4 hadamards_maximal_determinant_theorem_s4 hadamards_maximal_determinant_theorem_s9) (.arrow hadamards_maximal_determinant_theorem_l4 hadamards_maximal_determinant_theorem_s4 hadamards_maximal_determinant_theorem_s9)) (.seq (.arrow hadamards_maximal_determinant_theorem_l9 hadamards_maximal_determinant_theorem_s9 hadamards_maximal_determinant_theorem_s9) (.seq (.arrow hadamards_maximal_determinant_theorem_l9 hadamards_maximal_determinant_theorem_s9 hadamards_maximal_determinant_theorem_s10) (.seq (.arrow hadamards_maximal_determinant_theorem_l10 hadamards_maximal_determinant_theorem_s10 hadamards_maximal_determinant_theorem_s11) (.seq (.arrow hadamards_maximal_determinant_theorem_l11 hadamards_maximal_determinant_theorem_s11 hadamards_maximal_determinant_theorem_s12) (.seq (.arrow hadamards_maximal_determinant_theorem_l12 hadamards_maximal_determinant_theorem_s12 hadamards_maximal_determinant_theorem_s13) (.seq (.arrow hadamards_maximal_determinant_theorem_l13 hadamards_maximal_determinant_theorem_s13 hadamards_maximal_determinant_theorem_s14) (.seq (.arrow hadamards_maximal_determinant_theorem_l14 hadamards_maximal_determinant_theorem_s14 hadamards_maximal_determinant_theorem_s15) (.seq (.arrow hadamards_maximal_determinant_theorem_l15 hadamards_maximal_determinant_theorem_s15 hadamards_maximal_determinant_theorem_s16) (.seq (.arrow hadamards_maximal_determinant_theorem_l16 hadamards_maximal_determinant_theorem_s16 hadamards_maximal_determinant_theorem_s17) (.arrow hadamards_maximal_determinant_theorem_l17 hadamards_maximal_determinant_theorem_s17 hadamards_maximal_determinant_theorem_s18)))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def hadamards_maximal_determinant_theorem_true_arm : IGProtocol hadamards_maximal_determinant_theorem_s0 hadamards_maximal_determinant_theorem_s18 :=
  (hadamards_maximal_determinant_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def hadamards_maximal_determinant_theorem_false_arm : IGProtocol hadamards_maximal_determinant_theorem_s0 hadamards_maximal_determinant_theorem_s18 :=
  (hadamards_maximal_determinant_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def hadamards_maximal_determinant_theorem_tier : OuroboricityTier := TierFunctor.obj hadamards_maximal_determinant_theorem_s0
#eval hadamards_maximal_determinant_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem hadamards_maximal_determinant_theorem_frobenius :
    igFrobeniusAlg.mul hadamards_maximal_determinant_theorem_s0 hadamards_maximal_determinant_theorem_s0 = hadamards_maximal_determinant_theorem_s0 :=
  igFrobAlg_self_fusion hadamards_maximal_determinant_theorem_s0
