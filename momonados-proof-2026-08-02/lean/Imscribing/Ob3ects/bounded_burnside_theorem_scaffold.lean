-- IGProtocol scaffold: VINIT → AFWD → AREV → CLINK → TANCH → FSPLIT → EVALT → EVALF → FFUSE → IMSCRIB → ENGAGR → IFIX → IMSCRIB
-- Class: Bounded Burnside Theorem
-- Fingerprint: sig=(7,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 8)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑗  | forward morphism — bidirectional arrow
--   [2] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [3] CLINK     fid    := 𐑱               𐑗 → 𐑡  | composition — regime coherence
--   [4] TANCH     top    := 𐑡               𐑱 → 𐑚  | terminal object — connectivity boundary
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [9] IMSCRIB   gram   := 𐑠               𐑙 → 𐑳  | identity — self-imscription
--   [10] ENGAGR    stoi   := 𐑳               𐑠 → 𐑭  | engage paradox — B-state, both arms
--   [11] IFIX      prot   := 𐑭               𐑳 → 𐑠  | irreversible fixation — winding number
--   [12] IMSCRIB   gram   := 𐑠               𐑭 → 𐑼  | identity — self-imscription

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def bounded_burnside_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def bounded_burnside_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def bounded_burnside_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def bounded_burnside_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def bounded_burnside_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def bounded_burnside_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def bounded_burnside_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def bounded_burnside_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def bounded_burnside_theorem_protocol : IGProtocol bounded_burnside_theorem_s0 bounded_burnside_theorem_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct bounded_burnside_theorem_s8 bounded_burnside_theorem_s8 = bounded_burnside_theorem_s8 (idempotent)
  (.seq (.arrow bounded_burnside_theorem_l0 bounded_burnside_theorem_s0 bounded_burnside_theorem_s1) (.seq (.arrow bounded_burnside_theorem_l1 bounded_burnside_theorem_s1 bounded_burnside_theorem_s2) (.seq (.arrow bounded_burnside_theorem_l2 bounded_burnside_theorem_s2 bounded_burnside_theorem_s3) (.seq (.arrow bounded_burnside_theorem_l3 bounded_burnside_theorem_s3 bounded_burnside_theorem_s4) (.seq (.arrow bounded_burnside_theorem_l4 bounded_burnside_theorem_s4 bounded_burnside_theorem_s5) (.seq (.prod (.arrow bounded_burnside_theorem_l5 bounded_burnside_theorem_s5 bounded_burnside_theorem_s8) (.arrow bounded_burnside_theorem_l5 bounded_burnside_theorem_s5 bounded_burnside_theorem_s8)) (.seq (.arrow bounded_burnside_theorem_l8 bounded_burnside_theorem_s8 bounded_burnside_theorem_s8) (.seq (.arrow bounded_burnside_theorem_l8 bounded_burnside_theorem_s8 bounded_burnside_theorem_s9) (.seq (.arrow bounded_burnside_theorem_l9 bounded_burnside_theorem_s9 bounded_burnside_theorem_s10) (.seq (.arrow bounded_burnside_theorem_l10 bounded_burnside_theorem_s10 bounded_burnside_theorem_s11) (.arrow bounded_burnside_theorem_l11 bounded_burnside_theorem_s11 bounded_burnside_theorem_s12)))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def bounded_burnside_theorem_true_arm : IGProtocol bounded_burnside_theorem_s0 bounded_burnside_theorem_s12 :=
  (bounded_burnside_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def bounded_burnside_theorem_false_arm : IGProtocol bounded_burnside_theorem_s0 bounded_burnside_theorem_s12 :=
  (bounded_burnside_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def bounded_burnside_theorem_tier : OuroboricityTier := TierFunctor.obj bounded_burnside_theorem_s0
#eval bounded_burnside_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem bounded_burnside_theorem_frobenius :
    igFrobeniusAlg.mul bounded_burnside_theorem_s0 bounded_burnside_theorem_s0 = bounded_burnside_theorem_s0 :=
  igFrobAlg_self_fusion bounded_burnside_theorem_s0
