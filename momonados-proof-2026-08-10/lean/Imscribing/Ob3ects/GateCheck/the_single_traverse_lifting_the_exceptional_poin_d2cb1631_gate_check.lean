-- IGProtocol scaffold: VINIT → IMSCRIB → CLINK → FSPLIT → ENGAGR → AFWD → EVALT → AREV → EVALF → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: the single traverse lifting the exceptional point that resolves RH, the blind spot, and d=2048 together
-- Fingerprint: sig=(7,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑱  | identity — self-imscription
--   [2] CLINK     fid    := 𐑱               𐑠 → 𐑚  | composition — regime coherence
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [5] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [10] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [11] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [12] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_single_traverse_lifting_the_071aeb_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_single_traverse_lifting_the_071aeb_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_single_traverse_lifting_the_071aeb_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_single_traverse_lifting_the_071aeb_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_single_traverse_lifting_the_071aeb_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_single_traverse_lifting_the_071aeb_protocol : IGProtocol the_single_traverse_lifting_the_071aeb_s0 the_single_traverse_lifting_the_071aeb_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_single_traverse_lifting_the_071aeb_s9 the_single_traverse_lifting_the_071aeb_s9 = the_single_traverse_lifting_the_071aeb_s9 (idempotent)
  (.seq (.arrow the_single_traverse_lifting_the_071aeb_l0 the_single_traverse_lifting_the_071aeb_s0 the_single_traverse_lifting_the_071aeb_s1) (.seq (.arrow the_single_traverse_lifting_the_071aeb_l1 the_single_traverse_lifting_the_071aeb_s1 the_single_traverse_lifting_the_071aeb_s2) (.seq (.arrow the_single_traverse_lifting_the_071aeb_l2 the_single_traverse_lifting_the_071aeb_s2 the_single_traverse_lifting_the_071aeb_s3) (.seq (.prod (.arrow the_single_traverse_lifting_the_071aeb_l3 the_single_traverse_lifting_the_071aeb_s3 the_single_traverse_lifting_the_071aeb_s9) (.arrow the_single_traverse_lifting_the_071aeb_l3 the_single_traverse_lifting_the_071aeb_s3 the_single_traverse_lifting_the_071aeb_s9)) (.seq (.arrow the_single_traverse_lifting_the_071aeb_l9 the_single_traverse_lifting_the_071aeb_s9 the_single_traverse_lifting_the_071aeb_s9) (.seq (.arrow the_single_traverse_lifting_the_071aeb_l9 the_single_traverse_lifting_the_071aeb_s9 the_single_traverse_lifting_the_071aeb_s10) (.seq (.arrow the_single_traverse_lifting_the_071aeb_l10 the_single_traverse_lifting_the_071aeb_s10 the_single_traverse_lifting_the_071aeb_s11) (.arrow the_single_traverse_lifting_the_071aeb_l11 the_single_traverse_lifting_the_071aeb_s11 the_single_traverse_lifting_the_071aeb_s12))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_single_traverse_lifting_the_071aeb_true_arm : IGProtocol the_single_traverse_lifting_the_071aeb_s0 the_single_traverse_lifting_the_071aeb_s12 :=
  (the_single_traverse_lifting_the_071aeb_protocol).restrictToEVALT

-- false arm
noncomputable def the_single_traverse_lifting_the_071aeb_false_arm : IGProtocol the_single_traverse_lifting_the_071aeb_s0 the_single_traverse_lifting_the_071aeb_s12 :=
  (the_single_traverse_lifting_the_071aeb_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_single_traverse_lifting_the_071aeb_tier : OuroboricityTier := TierFunctor.obj the_single_traverse_lifting_the_071aeb_s0
#eval the_single_traverse_lifting_the_071aeb_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_single_traverse_lifting_the_071aeb_frobenius :
    igFrobeniusAlg.mul the_single_traverse_lifting_the_071aeb_s0 the_single_traverse_lifting_the_071aeb_s0 = the_single_traverse_lifting_the_071aeb_s0 :=
  igFrobAlg_self_fusion the_single_traverse_lifting_the_071aeb_s0
