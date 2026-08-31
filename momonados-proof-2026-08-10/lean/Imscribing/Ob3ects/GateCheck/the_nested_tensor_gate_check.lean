-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → FSPLIT → FSPLIT → EVALT → EVALT → FFUSE → EVALT → FFUSE → EVALF → FFUSE → CLINK → IFIX → TANCH
-- Class: The Nested Tensor
-- Fingerprint: sig=(4,6,4,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=15
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(4, 7), (3, 9), (2, 11)]

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
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑙  | split δ — range decomposition
--   [4] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [8] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [9] FFUSE     stoi   := 𐑙               𐑚 → 𐑙  | fuse μ — assembly mode
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [12] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [13] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [14] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_nested_tensor_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s6 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s7 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s8 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s9 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_s10 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_nested_tensor_s11 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_nested_tensor_s12 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_nested_tensor_s13 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nested_tensor_s14 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_nested_tensor_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_nested_tensor_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nested_tensor_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_nested_tensor_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_nested_tensor_protocol : IGProtocol the_nested_tensor_s0 the_nested_tensor_s14 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_nested_tensor_s7 the_nested_tensor_s7 = the_nested_tensor_s7 (idempotent)
  (.seq (.arrow the_nested_tensor_l0 the_nested_tensor_s0 the_nested_tensor_s1) (.seq (.arrow the_nested_tensor_l1 the_nested_tensor_s1 the_nested_tensor_s2) (.seq (.prod (.arrow the_nested_tensor_l2 the_nested_tensor_s2 the_nested_tensor_s7) (.arrow the_nested_tensor_l2 the_nested_tensor_s2 the_nested_tensor_s7)) (.seq (.arrow the_nested_tensor_l7 the_nested_tensor_s7 the_nested_tensor_s7) (.seq (.arrow the_nested_tensor_l7 the_nested_tensor_s7 the_nested_tensor_s8) (.seq (.arrow the_nested_tensor_l8 the_nested_tensor_s8 the_nested_tensor_s9) (.seq (.arrow the_nested_tensor_l9 the_nested_tensor_s9 the_nested_tensor_s10) (.seq (.arrow the_nested_tensor_l10 the_nested_tensor_s10 the_nested_tensor_s11) (.seq (.arrow the_nested_tensor_l11 the_nested_tensor_s11 the_nested_tensor_s12) (.seq (.arrow the_nested_tensor_l12 the_nested_tensor_s12 the_nested_tensor_s13) (.arrow the_nested_tensor_l13 the_nested_tensor_s13 the_nested_tensor_s14)))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_nested_tensor_true_arm : IGProtocol the_nested_tensor_s0 the_nested_tensor_s14 :=
  (the_nested_tensor_protocol).restrictToEVALT

-- false arm
noncomputable def the_nested_tensor_false_arm : IGProtocol the_nested_tensor_s0 the_nested_tensor_s14 :=
  (the_nested_tensor_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_nested_tensor_tier : OuroboricityTier := TierFunctor.obj the_nested_tensor_s0
#eval the_nested_tensor_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_nested_tensor_frobenius :
    igFrobeniusAlg.mul the_nested_tensor_s0 the_nested_tensor_s0 = the_nested_tensor_s0 :=
  igFrobAlg_self_fusion the_nested_tensor_s0
