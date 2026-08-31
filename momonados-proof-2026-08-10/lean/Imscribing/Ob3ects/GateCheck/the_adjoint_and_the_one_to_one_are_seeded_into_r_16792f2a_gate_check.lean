-- IGProtocol scaffold: VINIT → AFWD → FSPLIT → EVALF → EVALT → IMSCRIB → ENGAGR → IFIX → AREV → CLINK → FFUSE → TANCH
-- Class: The adjoint and the one-to-one are seeded into R4 and R5, and the closure witnesses overwrite them at the first imscription, so those registers carry tuple structure before imscription and closure evidence after it.
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 10)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑚  | forward morphism — bidirectional arrow
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [6] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [7] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [10] FFUSE     stoi   := 𐑙               𐑙 → 𐑡  | fuse μ — assembly mode
--   [11] TANCH     top    := 𐑡               𐑙 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_adjoint_and_the_one_to_one_are_b8f7bb_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_adjoint_and_the_one_to_one_are_b8f7bb_protocol : IGProtocol the_adjoint_and_the_one_to_one_are_b8f7bb_s0 the_adjoint_and_the_one_to_one_are_b8f7bb_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_adjoint_and_the_one_to_one_are_b8f7bb_s10 the_adjoint_and_the_one_to_one_are_b8f7bb_s10 = the_adjoint_and_the_one_to_one_are_b8f7bb_s10 (idempotent)
  (.seq (.arrow the_adjoint_and_the_one_to_one_are_b8f7bb_l0 the_adjoint_and_the_one_to_one_are_b8f7bb_s0 the_adjoint_and_the_one_to_one_are_b8f7bb_s1) (.seq (.arrow the_adjoint_and_the_one_to_one_are_b8f7bb_l1 the_adjoint_and_the_one_to_one_are_b8f7bb_s1 the_adjoint_and_the_one_to_one_are_b8f7bb_s2) (.seq (.prod (.arrow the_adjoint_and_the_one_to_one_are_b8f7bb_l2 the_adjoint_and_the_one_to_one_are_b8f7bb_s2 the_adjoint_and_the_one_to_one_are_b8f7bb_s10) (.arrow the_adjoint_and_the_one_to_one_are_b8f7bb_l2 the_adjoint_and_the_one_to_one_are_b8f7bb_s2 the_adjoint_and_the_one_to_one_are_b8f7bb_s10)) (.seq (.arrow the_adjoint_and_the_one_to_one_are_b8f7bb_l10 the_adjoint_and_the_one_to_one_are_b8f7bb_s10 the_adjoint_and_the_one_to_one_are_b8f7bb_s10) (.arrow the_adjoint_and_the_one_to_one_are_b8f7bb_l10 the_adjoint_and_the_one_to_one_are_b8f7bb_s10 the_adjoint_and_the_one_to_one_are_b8f7bb_s11)))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_adjoint_and_the_one_to_one_are_b8f7bb_true_arm : IGProtocol the_adjoint_and_the_one_to_one_are_b8f7bb_s0 the_adjoint_and_the_one_to_one_are_b8f7bb_s11 :=
  (the_adjoint_and_the_one_to_one_are_b8f7bb_protocol).restrictToEVALT

-- false arm
noncomputable def the_adjoint_and_the_one_to_one_are_b8f7bb_false_arm : IGProtocol the_adjoint_and_the_one_to_one_are_b8f7bb_s0 the_adjoint_and_the_one_to_one_are_b8f7bb_s11 :=
  (the_adjoint_and_the_one_to_one_are_b8f7bb_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_adjoint_and_the_one_to_one_are_b8f7bb_tier_ground : OuroboricityTier := TierFunctor.obj the_adjoint_and_the_one_to_one_are_b8f7bb_s0
def the_adjoint_and_the_one_to_one_are_b8f7bb_tier : OuroboricityTier := TierFunctor.obj the_adjoint_and_the_one_to_one_are_b8f7bb_s11
#eval the_adjoint_and_the_one_to_one_are_b8f7bb_tier_ground  -- tier of the ground (pre-transformation)
#eval the_adjoint_and_the_one_to_one_are_b8f7bb_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_adjoint_and_the_one_to_one_are_b8f7bb_frobenius :
    igFrobeniusAlg.mul the_adjoint_and_the_one_to_one_are_b8f7bb_s0 the_adjoint_and_the_one_to_one_are_b8f7bb_s0 = the_adjoint_and_the_one_to_one_are_b8f7bb_s0 :=
  igFrobAlg_self_fusion the_adjoint_and_the_one_to_one_are_b8f7bb_s0
