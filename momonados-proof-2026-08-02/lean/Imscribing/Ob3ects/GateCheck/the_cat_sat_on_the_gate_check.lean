-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → IMSCRIB → CLINK → AFWD → IMSCRIB → CLINK → AFWD → IMSCRIB → CLINK → AFWD → IMSCRIB → FSPLIT → EVALT → AFWD → FFUSE → IFIX → TANCH
-- Class: the cat sat on the
-- Fingerprint: sig=(15,2,1,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=19
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(13, 16)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑠  | forward morphism — bidirectional arrow
--   [3] IMSCRIB   gram   := 𐑠               𐑾 → 𐑱  | identity — self-imscription
--   [4] CLINK     fid    := 𐑱               𐑠 → 𐑾  | composition — regime coherence
--   [5] AFWD      rel    := 𐑾               𐑱 → 𐑠  | forward morphism — bidirectional arrow
--   [6] IMSCRIB   gram   := 𐑠               𐑾 → 𐑱  | identity — self-imscription
--   [7] CLINK     fid    := 𐑱               𐑠 → 𐑾  | composition — regime coherence
--   [8] AFWD      rel    := 𐑾               𐑱 → 𐑠  | forward morphism — bidirectional arrow
--   [9] IMSCRIB   gram   := 𐑠               𐑾 → 𐑱  | identity — self-imscription
--   [10] CLINK     fid    := 𐑱               𐑠 → 𐑾  | composition — regime coherence
--   [11] AFWD      rel    := 𐑾               𐑱 → 𐑠  | forward morphism — bidirectional arrow
--   [12] IMSCRIB   gram   := 𐑠               𐑾 → 𐑚  | identity — self-imscription
--   [13] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [14] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [15] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [16] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [17] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [18] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_cat_sat_on_the_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_cat_sat_on_the_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l15 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_cat_sat_on_the_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_cat_sat_on_the_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_cat_sat_on_the_protocol : IGProtocol the_cat_sat_on_the_s0 the_cat_sat_on_the_s18 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_cat_sat_on_the_s16 the_cat_sat_on_the_s16 = the_cat_sat_on_the_s16 (idempotent)
  (.seq (.arrow the_cat_sat_on_the_l0 the_cat_sat_on_the_s0 the_cat_sat_on_the_s1) (.seq (.arrow the_cat_sat_on_the_l1 the_cat_sat_on_the_s1 the_cat_sat_on_the_s2) (.seq (.arrow the_cat_sat_on_the_l2 the_cat_sat_on_the_s2 the_cat_sat_on_the_s3) (.seq (.arrow the_cat_sat_on_the_l3 the_cat_sat_on_the_s3 the_cat_sat_on_the_s4) (.seq (.arrow the_cat_sat_on_the_l4 the_cat_sat_on_the_s4 the_cat_sat_on_the_s5) (.seq (.arrow the_cat_sat_on_the_l5 the_cat_sat_on_the_s5 the_cat_sat_on_the_s6) (.seq (.arrow the_cat_sat_on_the_l6 the_cat_sat_on_the_s6 the_cat_sat_on_the_s7) (.seq (.arrow the_cat_sat_on_the_l7 the_cat_sat_on_the_s7 the_cat_sat_on_the_s8) (.seq (.arrow the_cat_sat_on_the_l8 the_cat_sat_on_the_s8 the_cat_sat_on_the_s9) (.seq (.arrow the_cat_sat_on_the_l9 the_cat_sat_on_the_s9 the_cat_sat_on_the_s10) (.seq (.arrow the_cat_sat_on_the_l10 the_cat_sat_on_the_s10 the_cat_sat_on_the_s11) (.seq (.arrow the_cat_sat_on_the_l11 the_cat_sat_on_the_s11 the_cat_sat_on_the_s12) (.seq (.arrow the_cat_sat_on_the_l12 the_cat_sat_on_the_s12 the_cat_sat_on_the_s13) (.seq (.prod (.arrow the_cat_sat_on_the_l13 the_cat_sat_on_the_s13 the_cat_sat_on_the_s16) (.arrow the_cat_sat_on_the_l13 the_cat_sat_on_the_s13 the_cat_sat_on_the_s16)) (.seq (.arrow the_cat_sat_on_the_l16 the_cat_sat_on_the_s16 the_cat_sat_on_the_s16) (.seq (.arrow the_cat_sat_on_the_l16 the_cat_sat_on_the_s16 the_cat_sat_on_the_s17) (.arrow the_cat_sat_on_the_l17 the_cat_sat_on_the_s17 the_cat_sat_on_the_s18)))))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_cat_sat_on_the_true_arm : IGProtocol the_cat_sat_on_the_s0 the_cat_sat_on_the_s18 :=
  (the_cat_sat_on_the_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_cat_sat_on_the_tier : OuroboricityTier := TierFunctor.obj the_cat_sat_on_the_s0
#eval the_cat_sat_on_the_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_cat_sat_on_the_frobenius :
    igFrobeniusAlg.mul the_cat_sat_on_the_s0 the_cat_sat_on_the_s0 = the_cat_sat_on_the_s0 :=
  igFrobAlg_self_fusion the_cat_sat_on_the_s0
