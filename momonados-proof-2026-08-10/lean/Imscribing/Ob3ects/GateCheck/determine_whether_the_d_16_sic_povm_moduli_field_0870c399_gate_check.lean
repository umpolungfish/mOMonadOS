-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → EVALT → AFWD → CLINK → IFIX → IFIX → FFUSE → EVALF → AREV → ENGAGR → TANCH
-- Class: Determine whether the d=16 SIC-POVM moduli field is the full ray class field of Q(sqrt 221) at the fifth power of the prime above two, or its quotient by the class group of order two.
-- Fingerprint: sig=(6,2,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 8)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑚  | identity — self-imscription
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [6] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [7] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [9] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [10] AREV      pol    := 𐑗               𐑖 → 𐑳  | reverse morphism — parity flip
--   [11] ENGAGR    stoi   := 𐑳               𐑗 → 𐑡  | engage paradox — B-state, both arms
--   [12] TANCH     top    := 𐑡               𐑳 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def determine_whether_the_d_16_sic_povm_baa0eb_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def determine_whether_the_d_16_sic_povm_baa0eb_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def determine_whether_the_d_16_sic_povm_baa0eb_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def determine_whether_the_d_16_sic_povm_baa0eb_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def determine_whether_the_d_16_sic_povm_baa0eb_protocol : IGProtocol determine_whether_the_d_16_sic_povm_baa0eb_s0 determine_whether_the_d_16_sic_povm_baa0eb_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct determine_whether_the_d_16_sic_povm_baa0eb_s8 determine_whether_the_d_16_sic_povm_baa0eb_s8 = determine_whether_the_d_16_sic_povm_baa0eb_s8 (idempotent)
  (.seq (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l0 determine_whether_the_d_16_sic_povm_baa0eb_s0 determine_whether_the_d_16_sic_povm_baa0eb_s1) (.seq (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l1 determine_whether_the_d_16_sic_povm_baa0eb_s1 determine_whether_the_d_16_sic_povm_baa0eb_s2) (.seq (.prod (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l2 determine_whether_the_d_16_sic_povm_baa0eb_s2 determine_whether_the_d_16_sic_povm_baa0eb_s8) (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l2 determine_whether_the_d_16_sic_povm_baa0eb_s2 determine_whether_the_d_16_sic_povm_baa0eb_s8)) (.seq (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l8 determine_whether_the_d_16_sic_povm_baa0eb_s8 determine_whether_the_d_16_sic_povm_baa0eb_s8) (.seq (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l8 determine_whether_the_d_16_sic_povm_baa0eb_s8 determine_whether_the_d_16_sic_povm_baa0eb_s9) (.seq (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l9 determine_whether_the_d_16_sic_povm_baa0eb_s9 determine_whether_the_d_16_sic_povm_baa0eb_s10) (.seq (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l10 determine_whether_the_d_16_sic_povm_baa0eb_s10 determine_whether_the_d_16_sic_povm_baa0eb_s11) (.arrow determine_whether_the_d_16_sic_povm_baa0eb_l11 determine_whether_the_d_16_sic_povm_baa0eb_s11 determine_whether_the_d_16_sic_povm_baa0eb_s12))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def determine_whether_the_d_16_sic_povm_baa0eb_true_arm : IGProtocol determine_whether_the_d_16_sic_povm_baa0eb_s0 determine_whether_the_d_16_sic_povm_baa0eb_s12 :=
  (determine_whether_the_d_16_sic_povm_baa0eb_protocol).restrictToEVALT

-- false arm
noncomputable def determine_whether_the_d_16_sic_povm_baa0eb_false_arm : IGProtocol determine_whether_the_d_16_sic_povm_baa0eb_s0 determine_whether_the_d_16_sic_povm_baa0eb_s12 :=
  (determine_whether_the_d_16_sic_povm_baa0eb_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def determine_whether_the_d_16_sic_povm_baa0eb_tier_ground : OuroboricityTier := TierFunctor.obj determine_whether_the_d_16_sic_povm_baa0eb_s0
def determine_whether_the_d_16_sic_povm_baa0eb_tier : OuroboricityTier := TierFunctor.obj determine_whether_the_d_16_sic_povm_baa0eb_s12
#eval determine_whether_the_d_16_sic_povm_baa0eb_tier_ground  -- tier of the ground (pre-transformation)
#eval determine_whether_the_d_16_sic_povm_baa0eb_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem determine_whether_the_d_16_sic_povm_baa0eb_frobenius :
    igFrobeniusAlg.mul determine_whether_the_d_16_sic_povm_baa0eb_s0 determine_whether_the_d_16_sic_povm_baa0eb_s0 = determine_whether_the_d_16_sic_povm_baa0eb_s0 :=
  igFrobAlg_self_fusion determine_whether_the_d_16_sic_povm_baa0eb_s0
