-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → EVALT → AFWD → EVALF → AREV → FFUSE → CLINK → IFIX → ENGAGR → CLINK → IMSCRIB → AFWD → IFIX → TANCH
-- Class: describe a descent operator mapping any candidate perfect cuboid integer triple a,b,c with face/space diagonals to a strictly smaller positive integer triple preserving the four Diophantine conditions edgewise terminating by well-ordering
-- Fingerprint: sig=(9,2,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=16
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 7)]

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
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [9] IFIX      prot   := 𐑭               𐑱 → 𐑳  | irreversible fixation — winding number
--   [10] ENGAGR    stoi   := 𐑳               𐑭 → 𐑱  | engage paradox — B-state, both arms
--   [11] CLINK     fid    := 𐑱               𐑳 → 𐑠  | composition — regime coherence
--   [12] IMSCRIB   gram   := 𐑠               𐑱 → 𐑾  | identity — self-imscription
--   [13] AFWD      rel    := 𐑾               𐑠 → 𐑭  | forward morphism — bidirectional arrow
--   [14] IFIX      prot   := 𐑭               𐑾 → 𐑡  | irreversible fixation — winding number
--   [15] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def describe_a_descent_operator_mapping_any_a02af2_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def describe_a_descent_operator_mapping_any_a02af2_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l13 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def describe_a_descent_operator_mapping_any_a02af2_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def describe_a_descent_operator_mapping_any_a02af2_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def describe_a_descent_operator_mapping_any_a02af2_protocol : IGProtocol describe_a_descent_operator_mapping_any_a02af2_s0 describe_a_descent_operator_mapping_any_a02af2_s15 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct describe_a_descent_operator_mapping_any_a02af2_s7 describe_a_descent_operator_mapping_any_a02af2_s7 = describe_a_descent_operator_mapping_any_a02af2_s7 (idempotent)
  (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l0 describe_a_descent_operator_mapping_any_a02af2_s0 describe_a_descent_operator_mapping_any_a02af2_s1) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l1 describe_a_descent_operator_mapping_any_a02af2_s1 describe_a_descent_operator_mapping_any_a02af2_s2) (.seq (.prod (.arrow describe_a_descent_operator_mapping_any_a02af2_l2 describe_a_descent_operator_mapping_any_a02af2_s2 describe_a_descent_operator_mapping_any_a02af2_s7) (.arrow describe_a_descent_operator_mapping_any_a02af2_l2 describe_a_descent_operator_mapping_any_a02af2_s2 describe_a_descent_operator_mapping_any_a02af2_s7)) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l7 describe_a_descent_operator_mapping_any_a02af2_s7 describe_a_descent_operator_mapping_any_a02af2_s7) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l7 describe_a_descent_operator_mapping_any_a02af2_s7 describe_a_descent_operator_mapping_any_a02af2_s8) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l8 describe_a_descent_operator_mapping_any_a02af2_s8 describe_a_descent_operator_mapping_any_a02af2_s9) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l9 describe_a_descent_operator_mapping_any_a02af2_s9 describe_a_descent_operator_mapping_any_a02af2_s10) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l10 describe_a_descent_operator_mapping_any_a02af2_s10 describe_a_descent_operator_mapping_any_a02af2_s11) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l11 describe_a_descent_operator_mapping_any_a02af2_s11 describe_a_descent_operator_mapping_any_a02af2_s12) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l12 describe_a_descent_operator_mapping_any_a02af2_s12 describe_a_descent_operator_mapping_any_a02af2_s13) (.seq (.arrow describe_a_descent_operator_mapping_any_a02af2_l13 describe_a_descent_operator_mapping_any_a02af2_s13 describe_a_descent_operator_mapping_any_a02af2_s14) (.arrow describe_a_descent_operator_mapping_any_a02af2_l14 describe_a_descent_operator_mapping_any_a02af2_s14 describe_a_descent_operator_mapping_any_a02af2_s15))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def describe_a_descent_operator_mapping_any_a02af2_true_arm : IGProtocol describe_a_descent_operator_mapping_any_a02af2_s0 describe_a_descent_operator_mapping_any_a02af2_s15 :=
  (describe_a_descent_operator_mapping_any_a02af2_protocol).restrictToEVALT

-- false arm
noncomputable def describe_a_descent_operator_mapping_any_a02af2_false_arm : IGProtocol describe_a_descent_operator_mapping_any_a02af2_s0 describe_a_descent_operator_mapping_any_a02af2_s15 :=
  (describe_a_descent_operator_mapping_any_a02af2_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def describe_a_descent_operator_mapping_any_a02af2_tier : OuroboricityTier := TierFunctor.obj describe_a_descent_operator_mapping_any_a02af2_s0
#eval describe_a_descent_operator_mapping_any_a02af2_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem describe_a_descent_operator_mapping_any_a02af2_frobenius :
    igFrobeniusAlg.mul describe_a_descent_operator_mapping_any_a02af2_s0 describe_a_descent_operator_mapping_any_a02af2_s0 = describe_a_descent_operator_mapping_any_a02af2_s0 :=
  igFrobAlg_self_fusion describe_a_descent_operator_mapping_any_a02af2_s0
