-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → IMSCRIB → FSPLIT → EVALF → FFUSE → IMSCRIB → CLINK → EVALT → ENGAGR → IFIX → TANCH
-- Class: imaginary unit *i*
-- Fingerprint: sig=(7,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(4, 6)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑠  | forward morphism — bidirectional arrow
--   [3] IMSCRIB   gram   := 𐑠               𐑾 → 𐑚  | identity — self-imscription
--   [4] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [7] IMSCRIB   gram   := 𐑠               𐑙 → 𐑱  | identity — self-imscription
--   [8] CLINK     fid    := 𐑱               𐑠 → ⊙  | composition — regime coherence
--   [9] EVALT     crit   := ⊙               𐑱 → 𐑳  | evaluate-true — criticality gate open
--   [10] ENGAGR    stoi   := 𐑳               ⊙ → 𐑭  | engage paradox — B-state, both arms
--   [11] IFIX      prot   := 𐑭               𐑳 → 𐑡  | irreversible fixation — winding number
--   [12] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def imaginary_unit_i_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def imaginary_unit_i_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def imaginary_unit_i_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def imaginary_unit_i_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def imaginary_unit_i_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def imaginary_unit_i_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def imaginary_unit_i_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def imaginary_unit_i_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def imaginary_unit_i_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def imaginary_unit_i_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def imaginary_unit_i_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def imaginary_unit_i_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def imaginary_unit_i_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def imaginary_unit_i_protocol : IGProtocol imaginary_unit_i_s0 imaginary_unit_i_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct imaginary_unit_i_s6 imaginary_unit_i_s6 = imaginary_unit_i_s6 (idempotent)
  (.seq (.arrow imaginary_unit_i_l0 imaginary_unit_i_s0 imaginary_unit_i_s1) (.seq (.arrow imaginary_unit_i_l1 imaginary_unit_i_s1 imaginary_unit_i_s2) (.seq (.arrow imaginary_unit_i_l2 imaginary_unit_i_s2 imaginary_unit_i_s3) (.seq (.arrow imaginary_unit_i_l3 imaginary_unit_i_s3 imaginary_unit_i_s4) (.seq (.prod (.arrow imaginary_unit_i_l4 imaginary_unit_i_s4 imaginary_unit_i_s6) (.arrow imaginary_unit_i_l4 imaginary_unit_i_s4 imaginary_unit_i_s6)) (.seq (.arrow imaginary_unit_i_l6 imaginary_unit_i_s6 imaginary_unit_i_s6) (.seq (.arrow imaginary_unit_i_l6 imaginary_unit_i_s6 imaginary_unit_i_s7) (.seq (.arrow imaginary_unit_i_l7 imaginary_unit_i_s7 imaginary_unit_i_s8) (.seq (.arrow imaginary_unit_i_l8 imaginary_unit_i_s8 imaginary_unit_i_s9) (.seq (.arrow imaginary_unit_i_l9 imaginary_unit_i_s9 imaginary_unit_i_s10) (.seq (.arrow imaginary_unit_i_l10 imaginary_unit_i_s10 imaginary_unit_i_s11) (.arrow imaginary_unit_i_l11 imaginary_unit_i_s11 imaginary_unit_i_s12))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def imaginary_unit_i_true_arm : IGProtocol imaginary_unit_i_s0 imaginary_unit_i_s12 :=
  (imaginary_unit_i_protocol).restrictToEVALT
  --
-- false arm
noncomputable def imaginary_unit_i_false_arm : IGProtocol imaginary_unit_i_s0 imaginary_unit_i_s12 :=
  (imaginary_unit_i_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def imaginary_unit_i_tier : OuroboricityTier := TierFunctor.obj imaginary_unit_i_s0
#eval imaginary_unit_i_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem imaginary_unit_i_frobenius :
    igFrobeniusAlg.mul imaginary_unit_i_s0 imaginary_unit_i_s0 = imaginary_unit_i_s0 :=
  igFrobAlg_self_fusion imaginary_unit_i_s0
