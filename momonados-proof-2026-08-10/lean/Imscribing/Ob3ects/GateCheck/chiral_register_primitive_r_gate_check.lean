-- IGProtocol scaffold: VINIT → FSPLIT → EVALT → AREV → EVALF → FFUSE → IMSCRIB → CLINK → AFWD → IFIX → TANCH → ENGAGR
-- Class: chiral register primitive R
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
--   [3] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [4] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [6] IMSCRIB   gram   := 𐑠               𐑙 → 𐑱  | identity — self-imscription
--   [7] CLINK     fid    := 𐑱               𐑠 → 𐑾  | composition — regime coherence
--   [8] AFWD      rel    := 𐑾               𐑱 → 𐑭  | forward morphism — bidirectional arrow
--   [9] IFIX      prot   := 𐑭               𐑾 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑳  | terminal object — connectivity boundary
--   [11] ENGAGR    stoi   := 𐑳               𐑡 → 𐑼  | engage paradox — B-state, both arms

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def chiral_register_primitive_r_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s6 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s7 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def chiral_register_primitive_r_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def chiral_register_primitive_r_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def chiral_register_primitive_r_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def chiral_register_primitive_r_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def chiral_register_primitive_r_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def chiral_register_primitive_r_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def chiral_register_primitive_r_protocol : IGProtocol chiral_register_primitive_r_s0 chiral_register_primitive_r_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct chiral_register_primitive_r_s5 chiral_register_primitive_r_s5 = chiral_register_primitive_r_s5 (idempotent)
  (.seq (.arrow chiral_register_primitive_r_l0 chiral_register_primitive_r_s0 chiral_register_primitive_r_s1) (.seq (.prod (.arrow chiral_register_primitive_r_l1 chiral_register_primitive_r_s1 chiral_register_primitive_r_s5) (.arrow chiral_register_primitive_r_l1 chiral_register_primitive_r_s1 chiral_register_primitive_r_s5)) (.seq (.arrow chiral_register_primitive_r_l5 chiral_register_primitive_r_s5 chiral_register_primitive_r_s5) (.seq (.arrow chiral_register_primitive_r_l5 chiral_register_primitive_r_s5 chiral_register_primitive_r_s6) (.seq (.arrow chiral_register_primitive_r_l6 chiral_register_primitive_r_s6 chiral_register_primitive_r_s7) (.seq (.arrow chiral_register_primitive_r_l7 chiral_register_primitive_r_s7 chiral_register_primitive_r_s8) (.seq (.arrow chiral_register_primitive_r_l8 chiral_register_primitive_r_s8 chiral_register_primitive_r_s9) (.seq (.arrow chiral_register_primitive_r_l9 chiral_register_primitive_r_s9 chiral_register_primitive_r_s10) (.arrow chiral_register_primitive_r_l10 chiral_register_primitive_r_s10 chiral_register_primitive_r_s11)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def chiral_register_primitive_r_true_arm : IGProtocol chiral_register_primitive_r_s0 chiral_register_primitive_r_s11 :=
  (chiral_register_primitive_r_protocol).restrictToEVALT

-- false arm
noncomputable def chiral_register_primitive_r_false_arm : IGProtocol chiral_register_primitive_r_s0 chiral_register_primitive_r_s11 :=
  (chiral_register_primitive_r_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def chiral_register_primitive_r_tier : OuroboricityTier := TierFunctor.obj chiral_register_primitive_r_s0
#eval chiral_register_primitive_r_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem chiral_register_primitive_r_frobenius :
    igFrobeniusAlg.mul chiral_register_primitive_r_s0 chiral_register_primitive_r_s0 = chiral_register_primitive_r_s0 :=
  igFrobAlg_self_fusion chiral_register_primitive_r_s0
