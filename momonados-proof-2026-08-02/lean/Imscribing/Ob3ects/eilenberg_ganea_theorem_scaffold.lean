-- IGProtocol scaffold: VINIT → TANCH → AFWD → AREV → CLINK → IMSCRIB → FSPLIT → EVALT → AFWD → FFUSE → EVALF → AREV → ENGAGR → IFIX → CLINK → IMSCRIB → TANCH
-- Class: Eilenberg–Ganea Theorem
-- Fingerprint: sig=(11,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=17
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(6, 9)]

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
--   [2] AFWD      rel    := 𐑾               𐑡 → 𐑗  | forward morphism — bidirectional arrow
--   [3] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [4] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [5] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [10] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [11] AREV      pol    := 𐑗               𐑖 → 𐑳  | reverse morphism — parity flip
--   [12] ENGAGR    stoi   := 𐑳               𐑗 → 𐑭  | engage paradox — B-state, both arms
--   [13] IFIX      prot   := 𐑭               𐑳 → 𐑱  | irreversible fixation — winding number
--   [14] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [15] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [16] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def eilenberg_ganea_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def eilenberg_ganea_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def eilenberg_ganea_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def eilenberg_ganea_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def eilenberg_ganea_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def eilenberg_ganea_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def eilenberg_ganea_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def eilenberg_ganea_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def eilenberg_ganea_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def eilenberg_ganea_theorem_protocol : IGProtocol eilenberg_ganea_theorem_s0 eilenberg_ganea_theorem_s16 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct eilenberg_ganea_theorem_s9 eilenberg_ganea_theorem_s9 = eilenberg_ganea_theorem_s9 (idempotent)
  (.seq (.arrow eilenberg_ganea_theorem_l0 eilenberg_ganea_theorem_s0 eilenberg_ganea_theorem_s1) (.seq (.arrow eilenberg_ganea_theorem_l1 eilenberg_ganea_theorem_s1 eilenberg_ganea_theorem_s2) (.seq (.arrow eilenberg_ganea_theorem_l2 eilenberg_ganea_theorem_s2 eilenberg_ganea_theorem_s3) (.seq (.arrow eilenberg_ganea_theorem_l3 eilenberg_ganea_theorem_s3 eilenberg_ganea_theorem_s4) (.seq (.arrow eilenberg_ganea_theorem_l4 eilenberg_ganea_theorem_s4 eilenberg_ganea_theorem_s5) (.seq (.arrow eilenberg_ganea_theorem_l5 eilenberg_ganea_theorem_s5 eilenberg_ganea_theorem_s6) (.seq (.prod (.arrow eilenberg_ganea_theorem_l6 eilenberg_ganea_theorem_s6 eilenberg_ganea_theorem_s9) (.arrow eilenberg_ganea_theorem_l6 eilenberg_ganea_theorem_s6 eilenberg_ganea_theorem_s9)) (.seq (.arrow eilenberg_ganea_theorem_l9 eilenberg_ganea_theorem_s9 eilenberg_ganea_theorem_s9) (.seq (.arrow eilenberg_ganea_theorem_l9 eilenberg_ganea_theorem_s9 eilenberg_ganea_theorem_s10) (.seq (.arrow eilenberg_ganea_theorem_l10 eilenberg_ganea_theorem_s10 eilenberg_ganea_theorem_s11) (.seq (.arrow eilenberg_ganea_theorem_l11 eilenberg_ganea_theorem_s11 eilenberg_ganea_theorem_s12) (.seq (.arrow eilenberg_ganea_theorem_l12 eilenberg_ganea_theorem_s12 eilenberg_ganea_theorem_s13) (.seq (.arrow eilenberg_ganea_theorem_l13 eilenberg_ganea_theorem_s13 eilenberg_ganea_theorem_s14) (.seq (.arrow eilenberg_ganea_theorem_l14 eilenberg_ganea_theorem_s14 eilenberg_ganea_theorem_s15) (.arrow eilenberg_ganea_theorem_l15 eilenberg_ganea_theorem_s15 eilenberg_ganea_theorem_s16)))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def eilenberg_ganea_theorem_true_arm : IGProtocol eilenberg_ganea_theorem_s0 eilenberg_ganea_theorem_s16 :=
  (eilenberg_ganea_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def eilenberg_ganea_theorem_false_arm : IGProtocol eilenberg_ganea_theorem_s0 eilenberg_ganea_theorem_s16 :=
  (eilenberg_ganea_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def eilenberg_ganea_theorem_tier : OuroboricityTier := TierFunctor.obj eilenberg_ganea_theorem_s0
#eval eilenberg_ganea_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem eilenberg_ganea_theorem_frobenius :
    igFrobeniusAlg.mul eilenberg_ganea_theorem_s0 eilenberg_ganea_theorem_s0 = eilenberg_ganea_theorem_s0 :=
  igFrobAlg_self_fusion eilenberg_ganea_theorem_s0
