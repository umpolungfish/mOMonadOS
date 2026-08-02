-- IGProtocol scaffold: VINIT → FSPLIT → EVALT → AFWD → CLINK → FFUSE → IMSCRIB → ENGAGR → EVALF → AREV → IFIX → TANCH
-- Class: a three-member odd ring with a doubled bond between nodes 0 and 1 interacting with the productive Frobenius channel of the parity-driven expansion loop such that the loop's mu-delta composition is the identity on the ring's unpaired mode
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 5)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [6] IMSCRIB   gram   := 𐑠               𐑙 → 𐑳  | identity — self-imscription
--   [7] ENGAGR    stoi   := 𐑳               𐑠 → 𐑖  | engage paradox — B-state, both arms
--   [8] EVALF     chir   := 𐑖               𐑳 → 𐑗  | evaluate-false — chirality check
--   [9] AREV      pol    := 𐑗               𐑖 → 𐑭  | reverse morphism — parity flip
--   [10] IFIX      prot   := 𐑭               𐑗 → 𐑡  | irreversible fixation — winding number
--   [11] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_three_member_odd_ring_with_a_doubled_fef3b4_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_three_member_odd_ring_with_a_doubled_fef3b4_protocol : IGProtocol a_three_member_odd_ring_with_a_doubled_fef3b4_s0 a_three_member_odd_ring_with_a_doubled_fef3b4_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_three_member_odd_ring_with_a_doubled_fef3b4_s5 a_three_member_odd_ring_with_a_doubled_fef3b4_s5 = a_three_member_odd_ring_with_a_doubled_fef3b4_s5 (idempotent)
  (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l0 a_three_member_odd_ring_with_a_doubled_fef3b4_s0 a_three_member_odd_ring_with_a_doubled_fef3b4_s1) (.seq (.prod (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l1 a_three_member_odd_ring_with_a_doubled_fef3b4_s1 a_three_member_odd_ring_with_a_doubled_fef3b4_s5) (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l1 a_three_member_odd_ring_with_a_doubled_fef3b4_s1 a_three_member_odd_ring_with_a_doubled_fef3b4_s5)) (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l5 a_three_member_odd_ring_with_a_doubled_fef3b4_s5 a_three_member_odd_ring_with_a_doubled_fef3b4_s5) (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l5 a_three_member_odd_ring_with_a_doubled_fef3b4_s5 a_three_member_odd_ring_with_a_doubled_fef3b4_s6) (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l6 a_three_member_odd_ring_with_a_doubled_fef3b4_s6 a_three_member_odd_ring_with_a_doubled_fef3b4_s7) (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l7 a_three_member_odd_ring_with_a_doubled_fef3b4_s7 a_three_member_odd_ring_with_a_doubled_fef3b4_s8) (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l8 a_three_member_odd_ring_with_a_doubled_fef3b4_s8 a_three_member_odd_ring_with_a_doubled_fef3b4_s9) (.seq (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l9 a_three_member_odd_ring_with_a_doubled_fef3b4_s9 a_three_member_odd_ring_with_a_doubled_fef3b4_s10) (.arrow a_three_member_odd_ring_with_a_doubled_fef3b4_l10 a_three_member_odd_ring_with_a_doubled_fef3b4_s10 a_three_member_odd_ring_with_a_doubled_fef3b4_s11)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_three_member_odd_ring_with_a_doubled_fef3b4_true_arm : IGProtocol a_three_member_odd_ring_with_a_doubled_fef3b4_s0 a_three_member_odd_ring_with_a_doubled_fef3b4_s11 :=
  (a_three_member_odd_ring_with_a_doubled_fef3b4_protocol).restrictToEVALT

-- false arm
noncomputable def a_three_member_odd_ring_with_a_doubled_fef3b4_false_arm : IGProtocol a_three_member_odd_ring_with_a_doubled_fef3b4_s0 a_three_member_odd_ring_with_a_doubled_fef3b4_s11 :=
  (a_three_member_odd_ring_with_a_doubled_fef3b4_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_three_member_odd_ring_with_a_doubled_fef3b4_tier : OuroboricityTier := TierFunctor.obj a_three_member_odd_ring_with_a_doubled_fef3b4_s0
#eval a_three_member_odd_ring_with_a_doubled_fef3b4_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_three_member_odd_ring_with_a_doubled_fef3b4_frobenius :
    igFrobeniusAlg.mul a_three_member_odd_ring_with_a_doubled_fef3b4_s0 a_three_member_odd_ring_with_a_doubled_fef3b4_s0 = a_three_member_odd_ring_with_a_doubled_fef3b4_s0 :=
  igFrobAlg_self_fusion a_three_member_odd_ring_with_a_doubled_fef3b4_s0
