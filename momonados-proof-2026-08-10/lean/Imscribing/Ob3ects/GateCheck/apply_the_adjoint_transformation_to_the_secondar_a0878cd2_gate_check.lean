-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → AREV → EVALF → FFUSE → CLINK → IFIX → TANCH
-- Class: Apply the adjoint transformation to the secondary branch
-- Fingerprint: sig=(6,2,1,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=10
-- Expected tier: O₀
-- FSPLIT/FFUSE pairs: [(3, 6)]

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
--   [4] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [7] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [8] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [9] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def apply_the_adjoint_transformation_to_the_ea9bd3_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := ah }
private def apply_the_adjoint_transformation_to_the_ea9bd3_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def apply_the_adjoint_transformation_to_the_ea9bd3_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def apply_the_adjoint_transformation_to_the_ea9bd3_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def apply_the_adjoint_transformation_to_the_ea9bd3_protocol : IGProtocol apply_the_adjoint_transformation_to_the_ea9bd3_s0 apply_the_adjoint_transformation_to_the_ea9bd3_s9 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct apply_the_adjoint_transformation_to_the_ea9bd3_s6 apply_the_adjoint_transformation_to_the_ea9bd3_s6 = apply_the_adjoint_transformation_to_the_ea9bd3_s6 (idempotent)
  (.seq (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l0 apply_the_adjoint_transformation_to_the_ea9bd3_s0 apply_the_adjoint_transformation_to_the_ea9bd3_s1) (.seq (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l1 apply_the_adjoint_transformation_to_the_ea9bd3_s1 apply_the_adjoint_transformation_to_the_ea9bd3_s2) (.seq (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l2 apply_the_adjoint_transformation_to_the_ea9bd3_s2 apply_the_adjoint_transformation_to_the_ea9bd3_s3) (.seq (.prod (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l3 apply_the_adjoint_transformation_to_the_ea9bd3_s3 apply_the_adjoint_transformation_to_the_ea9bd3_s6) (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l3 apply_the_adjoint_transformation_to_the_ea9bd3_s3 apply_the_adjoint_transformation_to_the_ea9bd3_s6)) (.seq (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l6 apply_the_adjoint_transformation_to_the_ea9bd3_s6 apply_the_adjoint_transformation_to_the_ea9bd3_s6) (.seq (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l6 apply_the_adjoint_transformation_to_the_ea9bd3_s6 apply_the_adjoint_transformation_to_the_ea9bd3_s7) (.seq (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l7 apply_the_adjoint_transformation_to_the_ea9bd3_s7 apply_the_adjoint_transformation_to_the_ea9bd3_s8) (.arrow apply_the_adjoint_transformation_to_the_ea9bd3_l8 apply_the_adjoint_transformation_to_the_ea9bd3_s8 apply_the_adjoint_transformation_to_the_ea9bd3_s9))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- false arm
noncomputable def apply_the_adjoint_transformation_to_the_ea9bd3_false_arm : IGProtocol apply_the_adjoint_transformation_to_the_ea9bd3_s0 apply_the_adjoint_transformation_to_the_ea9bd3_s9 :=
  (apply_the_adjoint_transformation_to_the_ea9bd3_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def apply_the_adjoint_transformation_to_the_ea9bd3_tier_ground : OuroboricityTier := TierFunctor.obj apply_the_adjoint_transformation_to_the_ea9bd3_s0
def apply_the_adjoint_transformation_to_the_ea9bd3_tier : OuroboricityTier := TierFunctor.obj apply_the_adjoint_transformation_to_the_ea9bd3_s9
#eval apply_the_adjoint_transformation_to_the_ea9bd3_tier_ground  -- tier of the ground (pre-transformation)
#eval apply_the_adjoint_transformation_to_the_ea9bd3_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem apply_the_adjoint_transformation_to_the_ea9bd3_frobenius :
    igFrobeniusAlg.mul apply_the_adjoint_transformation_to_the_ea9bd3_s0 apply_the_adjoint_transformation_to_the_ea9bd3_s0 = apply_the_adjoint_transformation_to_the_ea9bd3_s0 :=
  igFrobAlg_self_fusion apply_the_adjoint_transformation_to_the_ea9bd3_s0
