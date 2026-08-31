-- IGProtocol scaffold: VINIT → AFWD → FSPLIT → EVALF → EVALT → FFUSE → IMSCRIB → AFWD → EVALT → IFIX → TANCH
-- Class: The tuple occupies R0 through R3 alone as four pairs, and R4 through R7 belong to the closure witnesses, so the adjoint and the one-to-one do not enter the register file.
-- Fingerprint: sig=(5,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=11
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 5)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑚  | forward morphism — bidirectional arrow
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [6] IMSCRIB   gram   := 𐑠               𐑙 → 𐑾  | identity — self-imscription
--   [7] AFWD      rel    := 𐑾               𐑠 → ⊙  | forward morphism — bidirectional arrow
--   [8] EVALT     crit   := ⊙               𐑾 → 𐑭  | evaluate-true — criticality gate open
--   [9] IFIX      prot   := 𐑭               ⊙ → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_tuple_occupies_r0_through_r3_alone_06decd_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_tuple_occupies_r0_through_r3_alone_06decd_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_tuple_occupies_r0_through_r3_alone_06decd_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_tuple_occupies_r0_through_r3_alone_06decd_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_tuple_occupies_r0_through_r3_alone_06decd_protocol : IGProtocol the_tuple_occupies_r0_through_r3_alone_06decd_s0 the_tuple_occupies_r0_through_r3_alone_06decd_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_tuple_occupies_r0_through_r3_alone_06decd_s5 the_tuple_occupies_r0_through_r3_alone_06decd_s5 = the_tuple_occupies_r0_through_r3_alone_06decd_s5 (idempotent)
  (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l0 the_tuple_occupies_r0_through_r3_alone_06decd_s0 the_tuple_occupies_r0_through_r3_alone_06decd_s1) (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l1 the_tuple_occupies_r0_through_r3_alone_06decd_s1 the_tuple_occupies_r0_through_r3_alone_06decd_s2) (.seq (.prod (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l2 the_tuple_occupies_r0_through_r3_alone_06decd_s2 the_tuple_occupies_r0_through_r3_alone_06decd_s5) (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l2 the_tuple_occupies_r0_through_r3_alone_06decd_s2 the_tuple_occupies_r0_through_r3_alone_06decd_s5)) (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l5 the_tuple_occupies_r0_through_r3_alone_06decd_s5 the_tuple_occupies_r0_through_r3_alone_06decd_s5) (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l5 the_tuple_occupies_r0_through_r3_alone_06decd_s5 the_tuple_occupies_r0_through_r3_alone_06decd_s6) (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l6 the_tuple_occupies_r0_through_r3_alone_06decd_s6 the_tuple_occupies_r0_through_r3_alone_06decd_s7) (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l7 the_tuple_occupies_r0_through_r3_alone_06decd_s7 the_tuple_occupies_r0_through_r3_alone_06decd_s8) (.seq (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l8 the_tuple_occupies_r0_through_r3_alone_06decd_s8 the_tuple_occupies_r0_through_r3_alone_06decd_s9) (.arrow the_tuple_occupies_r0_through_r3_alone_06decd_l9 the_tuple_occupies_r0_through_r3_alone_06decd_s9 the_tuple_occupies_r0_through_r3_alone_06decd_s10)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_tuple_occupies_r0_through_r3_alone_06decd_true_arm : IGProtocol the_tuple_occupies_r0_through_r3_alone_06decd_s0 the_tuple_occupies_r0_through_r3_alone_06decd_s10 :=
  (the_tuple_occupies_r0_through_r3_alone_06decd_protocol).restrictToEVALT

-- false arm
noncomputable def the_tuple_occupies_r0_through_r3_alone_06decd_false_arm : IGProtocol the_tuple_occupies_r0_through_r3_alone_06decd_s0 the_tuple_occupies_r0_through_r3_alone_06decd_s10 :=
  (the_tuple_occupies_r0_through_r3_alone_06decd_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_tuple_occupies_r0_through_r3_alone_06decd_tier_ground : OuroboricityTier := TierFunctor.obj the_tuple_occupies_r0_through_r3_alone_06decd_s0
def the_tuple_occupies_r0_through_r3_alone_06decd_tier : OuroboricityTier := TierFunctor.obj the_tuple_occupies_r0_through_r3_alone_06decd_s10
#eval the_tuple_occupies_r0_through_r3_alone_06decd_tier_ground  -- tier of the ground (pre-transformation)
#eval the_tuple_occupies_r0_through_r3_alone_06decd_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_tuple_occupies_r0_through_r3_alone_06decd_frobenius :
    igFrobeniusAlg.mul the_tuple_occupies_r0_through_r3_alone_06decd_s0 the_tuple_occupies_r0_through_r3_alone_06decd_s0 = the_tuple_occupies_r0_through_r3_alone_06decd_s0 :=
  igFrobAlg_self_fusion the_tuple_occupies_r0_through_r3_alone_06decd_s0
