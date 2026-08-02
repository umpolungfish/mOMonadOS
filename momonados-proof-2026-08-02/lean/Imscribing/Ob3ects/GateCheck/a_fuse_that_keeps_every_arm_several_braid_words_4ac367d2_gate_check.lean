-- IGProtocol scaffold: VINIT → TANCH → FSPLIT → EVALT → AFWD → EVALF → AREV → CLINK → ENGAGR → FFUSE → IMSCRIB → IFIX
-- Class: A fuse that keeps every arm: several braid words sit at the same distance from a target unitary, each recursion leaves a residual rotation pointing its own way, and instead of ranking them the losing words become the dictionary that corrects the surviving one — so the composite braid beats every arm it was chosen from.
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑚  | terminal object — connectivity boundary
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [7] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [8] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [10] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [11] IFIX      prot   := 𐑭               𐑠 → 𐑼  | irreversible fixation — winding number

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_fuse_that_keeps_every_arm_several_743d42_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_fuse_that_keeps_every_arm_several_743d42_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_fuse_that_keeps_every_arm_several_743d42_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_fuse_that_keeps_every_arm_several_743d42_protocol : IGProtocol a_fuse_that_keeps_every_arm_several_743d42_s0 a_fuse_that_keeps_every_arm_several_743d42_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_fuse_that_keeps_every_arm_several_743d42_s9 a_fuse_that_keeps_every_arm_several_743d42_s9 = a_fuse_that_keeps_every_arm_several_743d42_s9 (idempotent)
  (.seq (.arrow a_fuse_that_keeps_every_arm_several_743d42_l0 a_fuse_that_keeps_every_arm_several_743d42_s0 a_fuse_that_keeps_every_arm_several_743d42_s1) (.seq (.arrow a_fuse_that_keeps_every_arm_several_743d42_l1 a_fuse_that_keeps_every_arm_several_743d42_s1 a_fuse_that_keeps_every_arm_several_743d42_s2) (.seq (.prod (.arrow a_fuse_that_keeps_every_arm_several_743d42_l2 a_fuse_that_keeps_every_arm_several_743d42_s2 a_fuse_that_keeps_every_arm_several_743d42_s9) (.arrow a_fuse_that_keeps_every_arm_several_743d42_l2 a_fuse_that_keeps_every_arm_several_743d42_s2 a_fuse_that_keeps_every_arm_several_743d42_s9)) (.seq (.arrow a_fuse_that_keeps_every_arm_several_743d42_l9 a_fuse_that_keeps_every_arm_several_743d42_s9 a_fuse_that_keeps_every_arm_several_743d42_s9) (.seq (.arrow a_fuse_that_keeps_every_arm_several_743d42_l9 a_fuse_that_keeps_every_arm_several_743d42_s9 a_fuse_that_keeps_every_arm_several_743d42_s10) (.arrow a_fuse_that_keeps_every_arm_several_743d42_l10 a_fuse_that_keeps_every_arm_several_743d42_s10 a_fuse_that_keeps_every_arm_several_743d42_s11))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_fuse_that_keeps_every_arm_several_743d42_true_arm : IGProtocol a_fuse_that_keeps_every_arm_several_743d42_s0 a_fuse_that_keeps_every_arm_several_743d42_s11 :=
  (a_fuse_that_keeps_every_arm_several_743d42_protocol).restrictToEVALT

-- false arm
noncomputable def a_fuse_that_keeps_every_arm_several_743d42_false_arm : IGProtocol a_fuse_that_keeps_every_arm_several_743d42_s0 a_fuse_that_keeps_every_arm_several_743d42_s11 :=
  (a_fuse_that_keeps_every_arm_several_743d42_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_fuse_that_keeps_every_arm_several_743d42_tier_ground : OuroboricityTier := TierFunctor.obj a_fuse_that_keeps_every_arm_several_743d42_s0
def a_fuse_that_keeps_every_arm_several_743d42_tier : OuroboricityTier := TierFunctor.obj a_fuse_that_keeps_every_arm_several_743d42_s11
#eval a_fuse_that_keeps_every_arm_several_743d42_tier_ground  -- tier of the ground (pre-transformation)
#eval a_fuse_that_keeps_every_arm_several_743d42_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_fuse_that_keeps_every_arm_several_743d42_frobenius :
    igFrobeniusAlg.mul a_fuse_that_keeps_every_arm_several_743d42_s0 a_fuse_that_keeps_every_arm_several_743d42_s0 = a_fuse_that_keeps_every_arm_several_743d42_s0 :=
  igFrobAlg_self_fusion a_fuse_that_keeps_every_arm_several_743d42_s0
