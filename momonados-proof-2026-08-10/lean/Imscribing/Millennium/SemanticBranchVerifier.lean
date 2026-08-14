-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → IFIX → FFUSE → CLINK → FSPLIT → EVALF → AREV → FFUSE → ENGAGR → CLINK → TANCH
-- Class: verification that the correct semantic branch was selected during recombination
-- Fingerprint: sig=(7,4,3,1)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=15
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(3, 6), (8, 11)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑚  | forward morphism — bidirectional arrow
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [7] CLINK     fid    := 𐑱               𐑙 → 𐑚  | composition — regime coherence
--   [8] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [9] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [10] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [11] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [12] ENGAGR    stoi   := 𐑳               𐑙 → 𐑱  | engage paradox — B-state, both arms
--   [13] CLINK     fid    := 𐑱               𐑳 → 𐑡  | composition — regime coherence
--   [14] TANCH     top    := 𐑡               𐑱 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def verification_that_the_correct_semantic_8fa750_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def verification_that_the_correct_semantic_8fa750_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def verification_that_the_correct_semantic_8fa750_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def verification_that_the_correct_semantic_8fa750_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verification_that_the_correct_semantic_8fa750_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def verification_that_the_correct_semantic_8fa750_protocol : IGProtocol verification_that_the_correct_semantic_8fa750_s0 verification_that_the_correct_semantic_8fa750_s14 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct verification_that_the_correct_semantic_8fa750_s6 verification_that_the_correct_semantic_8fa750_s6 = verification_that_the_correct_semantic_8fa750_s6 (idempotent)
  (.seq (.arrow verification_that_the_correct_semantic_8fa750_l0 verification_that_the_correct_semantic_8fa750_s0 verification_that_the_correct_semantic_8fa750_s1) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l1 verification_that_the_correct_semantic_8fa750_s1 verification_that_the_correct_semantic_8fa750_s2) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l2 verification_that_the_correct_semantic_8fa750_s2 verification_that_the_correct_semantic_8fa750_s3) (.seq (.prod (.arrow verification_that_the_correct_semantic_8fa750_l3 verification_that_the_correct_semantic_8fa750_s3 verification_that_the_correct_semantic_8fa750_s6) (.arrow verification_that_the_correct_semantic_8fa750_l3 verification_that_the_correct_semantic_8fa750_s3 verification_that_the_correct_semantic_8fa750_s6)) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l6 verification_that_the_correct_semantic_8fa750_s6 verification_that_the_correct_semantic_8fa750_s6) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l6 verification_that_the_correct_semantic_8fa750_s6 verification_that_the_correct_semantic_8fa750_s7) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l7 verification_that_the_correct_semantic_8fa750_s7 verification_that_the_correct_semantic_8fa750_s8) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l8 verification_that_the_correct_semantic_8fa750_s8 verification_that_the_correct_semantic_8fa750_s9) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l9 verification_that_the_correct_semantic_8fa750_s9 verification_that_the_correct_semantic_8fa750_s10) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l10 verification_that_the_correct_semantic_8fa750_s10 verification_that_the_correct_semantic_8fa750_s11) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l11 verification_that_the_correct_semantic_8fa750_s11 verification_that_the_correct_semantic_8fa750_s12) (.seq (.arrow verification_that_the_correct_semantic_8fa750_l12 verification_that_the_correct_semantic_8fa750_s12 verification_that_the_correct_semantic_8fa750_s13) (.arrow verification_that_the_correct_semantic_8fa750_l13 verification_that_the_correct_semantic_8fa750_s13 verification_that_the_correct_semantic_8fa750_s14)))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def verification_that_the_correct_semantic_8fa750_true_arm : IGProtocol verification_that_the_correct_semantic_8fa750_s0 verification_that_the_correct_semantic_8fa750_s14 :=
  (verification_that_the_correct_semantic_8fa750_protocol).restrictToEVALT

-- false arm
noncomputable def verification_that_the_correct_semantic_8fa750_false_arm : IGProtocol verification_that_the_correct_semantic_8fa750_s0 verification_that_the_correct_semantic_8fa750_s14 :=
  (verification_that_the_correct_semantic_8fa750_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def verification_that_the_correct_semantic_8fa750_tier : OuroboricityTier := TierFunctor.obj verification_that_the_correct_semantic_8fa750_s0
#eval verification_that_the_correct_semantic_8fa750_tier  -- the Grammar's own verdict on its tier
