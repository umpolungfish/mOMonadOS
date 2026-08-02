-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → AREV → EVALF → ENGAGR → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: Confirm that the local phase matches the expected boundary condition.
-- Fingerprint: sig=(7,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 8)]

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
--   [5] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [6] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [7] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [9] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [10] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [11] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [12] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def confirm_that_the_local_phase_matches_f61cc0_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def confirm_that_the_local_phase_matches_f61cc0_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def confirm_that_the_local_phase_matches_f61cc0_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def confirm_that_the_local_phase_matches_f61cc0_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def confirm_that_the_local_phase_matches_f61cc0_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def confirm_that_the_local_phase_matches_f61cc0_protocol : IGProtocol confirm_that_the_local_phase_matches_f61cc0_s0 confirm_that_the_local_phase_matches_f61cc0_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct confirm_that_the_local_phase_matches_f61cc0_s8 confirm_that_the_local_phase_matches_f61cc0_s8 = confirm_that_the_local_phase_matches_f61cc0_s8 (idempotent)
  (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l0 confirm_that_the_local_phase_matches_f61cc0_s0 confirm_that_the_local_phase_matches_f61cc0_s1) (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l1 confirm_that_the_local_phase_matches_f61cc0_s1 confirm_that_the_local_phase_matches_f61cc0_s2) (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l2 confirm_that_the_local_phase_matches_f61cc0_s2 confirm_that_the_local_phase_matches_f61cc0_s3) (.seq (.prod (.arrow confirm_that_the_local_phase_matches_f61cc0_l3 confirm_that_the_local_phase_matches_f61cc0_s3 confirm_that_the_local_phase_matches_f61cc0_s8) (.arrow confirm_that_the_local_phase_matches_f61cc0_l3 confirm_that_the_local_phase_matches_f61cc0_s3 confirm_that_the_local_phase_matches_f61cc0_s8)) (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l8 confirm_that_the_local_phase_matches_f61cc0_s8 confirm_that_the_local_phase_matches_f61cc0_s8) (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l8 confirm_that_the_local_phase_matches_f61cc0_s8 confirm_that_the_local_phase_matches_f61cc0_s9) (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l9 confirm_that_the_local_phase_matches_f61cc0_s9 confirm_that_the_local_phase_matches_f61cc0_s10) (.seq (.arrow confirm_that_the_local_phase_matches_f61cc0_l10 confirm_that_the_local_phase_matches_f61cc0_s10 confirm_that_the_local_phase_matches_f61cc0_s11) (.arrow confirm_that_the_local_phase_matches_f61cc0_l11 confirm_that_the_local_phase_matches_f61cc0_s11 confirm_that_the_local_phase_matches_f61cc0_s12)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def confirm_that_the_local_phase_matches_f61cc0_true_arm : IGProtocol confirm_that_the_local_phase_matches_f61cc0_s0 confirm_that_the_local_phase_matches_f61cc0_s12 :=
  (confirm_that_the_local_phase_matches_f61cc0_protocol).restrictToEVALT

-- false arm
noncomputable def confirm_that_the_local_phase_matches_f61cc0_false_arm : IGProtocol confirm_that_the_local_phase_matches_f61cc0_s0 confirm_that_the_local_phase_matches_f61cc0_s12 :=
  (confirm_that_the_local_phase_matches_f61cc0_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def confirm_that_the_local_phase_matches_f61cc0_tier_ground : OuroboricityTier := TierFunctor.obj confirm_that_the_local_phase_matches_f61cc0_s0
def confirm_that_the_local_phase_matches_f61cc0_tier : OuroboricityTier := TierFunctor.obj confirm_that_the_local_phase_matches_f61cc0_s12
#eval confirm_that_the_local_phase_matches_f61cc0_tier_ground  -- tier of the ground (pre-transformation)
#eval confirm_that_the_local_phase_matches_f61cc0_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem confirm_that_the_local_phase_matches_f61cc0_frobenius :
    igFrobeniusAlg.mul confirm_that_the_local_phase_matches_f61cc0_s0 confirm_that_the_local_phase_matches_f61cc0_s0 = confirm_that_the_local_phase_matches_f61cc0_s0 :=
  igFrobAlg_self_fusion confirm_that_the_local_phase_matches_f61cc0_s0
