-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → AREV → EVALF → ENGAGR → FFUSE → IFIX → CLINK → IMSCRIB → TANCH
-- Class: Branch the serialized data into a write-stream and a parity-check stream
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
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [9] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [10] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [11] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [12] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def branch_the_serialized_data_into_a_write_8fcf86_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def branch_the_serialized_data_into_a_write_8fcf86_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def branch_the_serialized_data_into_a_write_8fcf86_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def branch_the_serialized_data_into_a_write_8fcf86_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def branch_the_serialized_data_into_a_write_8fcf86_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def branch_the_serialized_data_into_a_write_8fcf86_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def branch_the_serialized_data_into_a_write_8fcf86_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def branch_the_serialized_data_into_a_write_8fcf86_protocol : IGProtocol branch_the_serialized_data_into_a_write_8fcf86_s0 branch_the_serialized_data_into_a_write_8fcf86_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct branch_the_serialized_data_into_a_write_8fcf86_s8 branch_the_serialized_data_into_a_write_8fcf86_s8 = branch_the_serialized_data_into_a_write_8fcf86_s8 (idempotent)
  (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l0 branch_the_serialized_data_into_a_write_8fcf86_s0 branch_the_serialized_data_into_a_write_8fcf86_s1) (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l1 branch_the_serialized_data_into_a_write_8fcf86_s1 branch_the_serialized_data_into_a_write_8fcf86_s2) (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l2 branch_the_serialized_data_into_a_write_8fcf86_s2 branch_the_serialized_data_into_a_write_8fcf86_s3) (.seq (.prod (.arrow branch_the_serialized_data_into_a_write_8fcf86_l3 branch_the_serialized_data_into_a_write_8fcf86_s3 branch_the_serialized_data_into_a_write_8fcf86_s8) (.arrow branch_the_serialized_data_into_a_write_8fcf86_l3 branch_the_serialized_data_into_a_write_8fcf86_s3 branch_the_serialized_data_into_a_write_8fcf86_s8)) (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l8 branch_the_serialized_data_into_a_write_8fcf86_s8 branch_the_serialized_data_into_a_write_8fcf86_s8) (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l8 branch_the_serialized_data_into_a_write_8fcf86_s8 branch_the_serialized_data_into_a_write_8fcf86_s9) (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l9 branch_the_serialized_data_into_a_write_8fcf86_s9 branch_the_serialized_data_into_a_write_8fcf86_s10) (.seq (.arrow branch_the_serialized_data_into_a_write_8fcf86_l10 branch_the_serialized_data_into_a_write_8fcf86_s10 branch_the_serialized_data_into_a_write_8fcf86_s11) (.arrow branch_the_serialized_data_into_a_write_8fcf86_l11 branch_the_serialized_data_into_a_write_8fcf86_s11 branch_the_serialized_data_into_a_write_8fcf86_s12)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def branch_the_serialized_data_into_a_write_8fcf86_true_arm : IGProtocol branch_the_serialized_data_into_a_write_8fcf86_s0 branch_the_serialized_data_into_a_write_8fcf86_s12 :=
  (branch_the_serialized_data_into_a_write_8fcf86_protocol).restrictToEVALT

-- false arm
noncomputable def branch_the_serialized_data_into_a_write_8fcf86_false_arm : IGProtocol branch_the_serialized_data_into_a_write_8fcf86_s0 branch_the_serialized_data_into_a_write_8fcf86_s12 :=
  (branch_the_serialized_data_into_a_write_8fcf86_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def branch_the_serialized_data_into_a_write_8fcf86_tier_ground : OuroboricityTier := TierFunctor.obj branch_the_serialized_data_into_a_write_8fcf86_s0
def branch_the_serialized_data_into_a_write_8fcf86_tier : OuroboricityTier := TierFunctor.obj branch_the_serialized_data_into_a_write_8fcf86_s12
#eval branch_the_serialized_data_into_a_write_8fcf86_tier_ground  -- tier of the ground (pre-transformation)
#eval branch_the_serialized_data_into_a_write_8fcf86_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem branch_the_serialized_data_into_a_write_8fcf86_frobenius :
    igFrobeniusAlg.mul branch_the_serialized_data_into_a_write_8fcf86_s0 branch_the_serialized_data_into_a_write_8fcf86_s0 = branch_the_serialized_data_into_a_write_8fcf86_s0 :=
  igFrobAlg_self_fusion branch_the_serialized_data_into_a_write_8fcf86_s0
