-- IGProtocol scaffold: VINIT → IMSCRIB → CLINK → ENGAGR → AFWD → FSPLIT → EVALT → AREV → EVALF → FFUSE → IFIX → TANCH
-- Class: the spectral gap sqrt(3)-1 of the macrocycle [[0,2,1],[2,0,1],[1,1,0]], eigenvalues {1+sqrt(3), -2, 1-sqrt(3)}, returned identically by disjoint monomer sets
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
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
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑱  | identity — self-imscription
--   [2] CLINK     fid    := 𐑱               𐑠 → 𐑳  | composition — regime coherence
--   [3] ENGAGR    stoi   := 𐑳               𐑱 → 𐑾  | engage paradox — B-state, both arms
--   [4] AFWD      rel    := 𐑾               𐑳 → 𐑚  | forward morphism — bidirectional arrow
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [10] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [11] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_spectral_gap_sqrt_3_1_of_the_f3c26c_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_spectral_gap_sqrt_3_1_of_the_f3c26c_protocol : IGProtocol the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9 = the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9 (idempotent)
  (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l0 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s1) (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l1 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s1 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s2) (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l2 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s2 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s3) (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l3 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s3 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s4) (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l4 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s4 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s5) (.seq (.prod (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l5 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s5 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9) (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l5 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s5 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9)) (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l9 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9) (.seq (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l9 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s9 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s10) (.arrow the_spectral_gap_sqrt_3_1_of_the_f3c26c_l10 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s10 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s11)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_spectral_gap_sqrt_3_1_of_the_f3c26c_true_arm : IGProtocol the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s11 :=
  (the_spectral_gap_sqrt_3_1_of_the_f3c26c_protocol).restrictToEVALT

-- false arm
noncomputable def the_spectral_gap_sqrt_3_1_of_the_f3c26c_false_arm : IGProtocol the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s11 :=
  (the_spectral_gap_sqrt_3_1_of_the_f3c26c_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_spectral_gap_sqrt_3_1_of_the_f3c26c_tier : OuroboricityTier := TierFunctor.obj the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0
#eval the_spectral_gap_sqrt_3_1_of_the_f3c26c_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_spectral_gap_sqrt_3_1_of_the_f3c26c_frobenius :
    igFrobeniusAlg.mul the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 = the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0 :=
  igFrobAlg_self_fusion the_spectral_gap_sqrt_3_1_of_the_f3c26c_s0
