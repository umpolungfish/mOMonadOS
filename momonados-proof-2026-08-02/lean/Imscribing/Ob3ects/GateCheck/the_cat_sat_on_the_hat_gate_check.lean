-- IGProtocol scaffold: VINIT → IMSCRIB → CLINK → IMSCRIB → IFIX → FSPLIT → AFWD → EVALT → CLINK → IMSCRIB → FSPLIT → AFWD → CLINK → IMSCRIB → EVALT → FFUSE → AREV → EVALT → FFUSE → IFIX → TANCH
-- Class: the cat sat on the hat
-- Fingerprint: sig=(12,4,3,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=21
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(10, 15), (5, 18)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑱  | identity — self-imscription
--   [2] CLINK     fid    := 𐑱               𐑠 → 𐑠  | composition — regime coherence
--   [3] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [4] IFIX      prot   := 𐑭               𐑠 → 𐑚  | irreversible fixation — winding number
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [9] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [10] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [11] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [12] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [13] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [14] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [15] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [16] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [17] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [18] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [19] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [20] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_cat_sat_on_the_hat_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_cat_sat_on_the_hat_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_hat_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_hat_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_cat_sat_on_the_hat_protocol : IGProtocol the_cat_sat_on_the_hat_s0 the_cat_sat_on_the_hat_s20 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_cat_sat_on_the_hat_s15 the_cat_sat_on_the_hat_s15 = the_cat_sat_on_the_hat_s15 (idempotent)
  (.seq (.arrow the_cat_sat_on_the_hat_l0 the_cat_sat_on_the_hat_s0 the_cat_sat_on_the_hat_s1) (.seq (.arrow the_cat_sat_on_the_hat_l1 the_cat_sat_on_the_hat_s1 the_cat_sat_on_the_hat_s2) (.seq (.arrow the_cat_sat_on_the_hat_l2 the_cat_sat_on_the_hat_s2 the_cat_sat_on_the_hat_s3) (.seq (.arrow the_cat_sat_on_the_hat_l3 the_cat_sat_on_the_hat_s3 the_cat_sat_on_the_hat_s4) (.seq (.arrow the_cat_sat_on_the_hat_l4 the_cat_sat_on_the_hat_s4 the_cat_sat_on_the_hat_s5) (.seq (.prod (.arrow the_cat_sat_on_the_hat_l5 the_cat_sat_on_the_hat_s5 the_cat_sat_on_the_hat_s15) (.arrow the_cat_sat_on_the_hat_l5 the_cat_sat_on_the_hat_s5 the_cat_sat_on_the_hat_s15)) (.seq (.arrow the_cat_sat_on_the_hat_l15 the_cat_sat_on_the_hat_s15 the_cat_sat_on_the_hat_s15) (.seq (.arrow the_cat_sat_on_the_hat_l15 the_cat_sat_on_the_hat_s15 the_cat_sat_on_the_hat_s16) (.seq (.arrow the_cat_sat_on_the_hat_l16 the_cat_sat_on_the_hat_s16 the_cat_sat_on_the_hat_s17) (.seq (.arrow the_cat_sat_on_the_hat_l17 the_cat_sat_on_the_hat_s17 the_cat_sat_on_the_hat_s18) (.seq (.arrow the_cat_sat_on_the_hat_l18 the_cat_sat_on_the_hat_s18 the_cat_sat_on_the_hat_s19) (.arrow the_cat_sat_on_the_hat_l19 the_cat_sat_on_the_hat_s19 the_cat_sat_on_the_hat_s20))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_cat_sat_on_the_hat_true_arm : IGProtocol the_cat_sat_on_the_hat_s0 the_cat_sat_on_the_hat_s20 :=
  (the_cat_sat_on_the_hat_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_cat_sat_on_the_hat_tier : OuroboricityTier := TierFunctor.obj the_cat_sat_on_the_hat_s0
#eval the_cat_sat_on_the_hat_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_cat_sat_on_the_hat_frobenius :
    igFrobeniusAlg.mul the_cat_sat_on_the_hat_s0 the_cat_sat_on_the_hat_s0 = the_cat_sat_on_the_hat_s0 :=
  igFrobAlg_self_fusion the_cat_sat_on_the_hat_s0
