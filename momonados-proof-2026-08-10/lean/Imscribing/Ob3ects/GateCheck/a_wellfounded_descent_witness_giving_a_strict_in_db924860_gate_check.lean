-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → AREV → CLINK → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → IFIX → CLINK → IMSCRIB → TANCH
-- Class: a wellfounded descent witness giving a strict integer size order on perfect cuboid candidate triples and a ranked ancestor exposing rank over the descent tree paired as one cocrystallizable complement
-- Fingerprint: sig=(8,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=14
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(5, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑗  | forward morphism — bidirectional arrow
--   [3] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [4] CLINK     fid    := 𐑱               𐑗 → 𐑚  | composition — regime coherence
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [10] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [11] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [12] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [13] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_wellfounded_descent_witness_giving_a_4b2571_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_wellfounded_descent_witness_giving_a_4b2571_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_wellfounded_descent_witness_giving_a_4b2571_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_wellfounded_descent_witness_giving_a_4b2571_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_wellfounded_descent_witness_giving_a_4b2571_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_wellfounded_descent_witness_giving_a_4b2571_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_wellfounded_descent_witness_giving_a_4b2571_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_wellfounded_descent_witness_giving_a_4b2571_protocol : IGProtocol a_wellfounded_descent_witness_giving_a_4b2571_s0 a_wellfounded_descent_witness_giving_a_4b2571_s13 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_wellfounded_descent_witness_giving_a_4b2571_s9 a_wellfounded_descent_witness_giving_a_4b2571_s9 = a_wellfounded_descent_witness_giving_a_4b2571_s9 (idempotent)
  (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l0 a_wellfounded_descent_witness_giving_a_4b2571_s0 a_wellfounded_descent_witness_giving_a_4b2571_s1) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l1 a_wellfounded_descent_witness_giving_a_4b2571_s1 a_wellfounded_descent_witness_giving_a_4b2571_s2) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l2 a_wellfounded_descent_witness_giving_a_4b2571_s2 a_wellfounded_descent_witness_giving_a_4b2571_s3) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l3 a_wellfounded_descent_witness_giving_a_4b2571_s3 a_wellfounded_descent_witness_giving_a_4b2571_s4) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l4 a_wellfounded_descent_witness_giving_a_4b2571_s4 a_wellfounded_descent_witness_giving_a_4b2571_s5) (.seq (.prod (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l5 a_wellfounded_descent_witness_giving_a_4b2571_s5 a_wellfounded_descent_witness_giving_a_4b2571_s9) (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l5 a_wellfounded_descent_witness_giving_a_4b2571_s5 a_wellfounded_descent_witness_giving_a_4b2571_s9)) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l9 a_wellfounded_descent_witness_giving_a_4b2571_s9 a_wellfounded_descent_witness_giving_a_4b2571_s9) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l9 a_wellfounded_descent_witness_giving_a_4b2571_s9 a_wellfounded_descent_witness_giving_a_4b2571_s10) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l10 a_wellfounded_descent_witness_giving_a_4b2571_s10 a_wellfounded_descent_witness_giving_a_4b2571_s11) (.seq (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l11 a_wellfounded_descent_witness_giving_a_4b2571_s11 a_wellfounded_descent_witness_giving_a_4b2571_s12) (.arrow a_wellfounded_descent_witness_giving_a_4b2571_l12 a_wellfounded_descent_witness_giving_a_4b2571_s12 a_wellfounded_descent_witness_giving_a_4b2571_s13)))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_wellfounded_descent_witness_giving_a_4b2571_true_arm : IGProtocol a_wellfounded_descent_witness_giving_a_4b2571_s0 a_wellfounded_descent_witness_giving_a_4b2571_s13 :=
  (a_wellfounded_descent_witness_giving_a_4b2571_protocol).restrictToEVALT

-- false arm
noncomputable def a_wellfounded_descent_witness_giving_a_4b2571_false_arm : IGProtocol a_wellfounded_descent_witness_giving_a_4b2571_s0 a_wellfounded_descent_witness_giving_a_4b2571_s13 :=
  (a_wellfounded_descent_witness_giving_a_4b2571_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_wellfounded_descent_witness_giving_a_4b2571_tier : OuroboricityTier := TierFunctor.obj a_wellfounded_descent_witness_giving_a_4b2571_s0
#eval a_wellfounded_descent_witness_giving_a_4b2571_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_wellfounded_descent_witness_giving_a_4b2571_frobenius :
    igFrobeniusAlg.mul a_wellfounded_descent_witness_giving_a_4b2571_s0 a_wellfounded_descent_witness_giving_a_4b2571_s0 = a_wellfounded_descent_witness_giving_a_4b2571_s0 :=
  igFrobAlg_self_fusion a_wellfounded_descent_witness_giving_a_4b2571_s0
