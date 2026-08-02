-- IGProtocol scaffold: VINIT → TANCH → IMSCRIB → FSPLIT → AFWD → EVALT → AREV → EVALF → ENGAGR → FFUSE → CLINK → IFIX → IMSCRIB → TANCH
-- Class: The Distinguishable and the Gap
-- Fingerprint: sig=(8,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=14
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑠  | terminal object — connectivity boundary
--   [2] IMSCRIB   gram   := 𐑠               𐑡 → 𐑚  | identity — self-imscription
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [10] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [11] IFIX      prot   := 𐑭               𐑱 → 𐑠  | irreversible fixation — winding number
--   [12] IMSCRIB   gram   := 𐑠               𐑭 → 𐑡  | identity — self-imscription
--   [13] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_distinguishable_and_the_gap_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_distinguishable_and_the_gap_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_distinguishable_and_the_gap_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_distinguishable_and_the_gap_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_distinguishable_and_the_gap_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_distinguishable_and_the_gap_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_distinguishable_and_the_gap_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_distinguishable_and_the_gap_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_distinguishable_and_the_gap_protocol : IGProtocol the_distinguishable_and_the_gap_s0 the_distinguishable_and_the_gap_s13 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_distinguishable_and_the_gap_s9 the_distinguishable_and_the_gap_s9 = the_distinguishable_and_the_gap_s9 (idempotent)
  (.seq (.arrow the_distinguishable_and_the_gap_l0 the_distinguishable_and_the_gap_s0 the_distinguishable_and_the_gap_s1) (.seq (.arrow the_distinguishable_and_the_gap_l1 the_distinguishable_and_the_gap_s1 the_distinguishable_and_the_gap_s2) (.seq (.arrow the_distinguishable_and_the_gap_l2 the_distinguishable_and_the_gap_s2 the_distinguishable_and_the_gap_s3) (.seq (.prod (.arrow the_distinguishable_and_the_gap_l3 the_distinguishable_and_the_gap_s3 the_distinguishable_and_the_gap_s9) (.arrow the_distinguishable_and_the_gap_l3 the_distinguishable_and_the_gap_s3 the_distinguishable_and_the_gap_s9)) (.seq (.arrow the_distinguishable_and_the_gap_l9 the_distinguishable_and_the_gap_s9 the_distinguishable_and_the_gap_s9) (.seq (.arrow the_distinguishable_and_the_gap_l9 the_distinguishable_and_the_gap_s9 the_distinguishable_and_the_gap_s10) (.seq (.arrow the_distinguishable_and_the_gap_l10 the_distinguishable_and_the_gap_s10 the_distinguishable_and_the_gap_s11) (.seq (.arrow the_distinguishable_and_the_gap_l11 the_distinguishable_and_the_gap_s11 the_distinguishable_and_the_gap_s12) (.arrow the_distinguishable_and_the_gap_l12 the_distinguishable_and_the_gap_s12 the_distinguishable_and_the_gap_s13)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_distinguishable_and_the_gap_true_arm : IGProtocol the_distinguishable_and_the_gap_s0 the_distinguishable_and_the_gap_s13 :=
  (the_distinguishable_and_the_gap_protocol).restrictToEVALT

-- false arm
noncomputable def the_distinguishable_and_the_gap_false_arm : IGProtocol the_distinguishable_and_the_gap_s0 the_distinguishable_and_the_gap_s13 :=
  (the_distinguishable_and_the_gap_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_distinguishable_and_the_gap_tier : OuroboricityTier := TierFunctor.obj the_distinguishable_and_the_gap_s0
#eval the_distinguishable_and_the_gap_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_distinguishable_and_the_gap_frobenius :
    igFrobeniusAlg.mul the_distinguishable_and_the_gap_s0 the_distinguishable_and_the_gap_s0 = the_distinguishable_and_the_gap_s0 :=
  igFrobAlg_self_fusion the_distinguishable_and_the_gap_s0
