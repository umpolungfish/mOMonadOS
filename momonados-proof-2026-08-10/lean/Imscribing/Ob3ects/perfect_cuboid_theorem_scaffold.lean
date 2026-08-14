-- IGProtocol scaffold: VINIT → AFWD → AFWD → AFWD → AFWD → AFWD → FSPLIT → EVALT → EVALF → FFUSE → ENGAGR → IMSCRIB → AREV → CLINK → EVALT → IFIX → TANCH
-- Class: Perfect Cuboid Theorem
-- Fingerprint: sig=(10,2,4,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=17
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(6, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑾  | forward morphism — bidirectional arrow
--   [2] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [4] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [5] AFWD      rel    := 𐑾               𐑾 → 𐑚  | forward morphism — bidirectional arrow
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [10] ENGAGR    stoi   := 𐑳               𐑙 → 𐑠  | engage paradox — B-state, both arms
--   [11] IMSCRIB   gram   := 𐑠               𐑳 → 𐑗  | identity — self-imscription
--   [12] AREV      pol    := 𐑗               𐑠 → 𐑱  | reverse morphism — parity flip
--   [13] CLINK     fid    := 𐑱               𐑗 → ⊙  | composition — regime coherence
--   [14] EVALT     crit   := ⊙               𐑱 → 𐑭  | evaluate-true — criticality gate open
--   [15] IFIX      prot   := 𐑭               ⊙ → 𐑡  | irreversible fixation — winding number
--   [16] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def perfect_cuboid_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def perfect_cuboid_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def perfect_cuboid_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def perfect_cuboid_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def perfect_cuboid_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def perfect_cuboid_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def perfect_cuboid_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def perfect_cuboid_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def perfect_cuboid_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def perfect_cuboid_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def perfect_cuboid_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def perfect_cuboid_theorem_protocol : IGProtocol perfect_cuboid_theorem_s0 perfect_cuboid_theorem_s16 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct perfect_cuboid_theorem_s9 perfect_cuboid_theorem_s9 = perfect_cuboid_theorem_s9 (idempotent)
  (.seq (.arrow perfect_cuboid_theorem_l0 perfect_cuboid_theorem_s0 perfect_cuboid_theorem_s1) (.seq (.arrow perfect_cuboid_theorem_l1 perfect_cuboid_theorem_s1 perfect_cuboid_theorem_s2) (.seq (.arrow perfect_cuboid_theorem_l2 perfect_cuboid_theorem_s2 perfect_cuboid_theorem_s3) (.seq (.arrow perfect_cuboid_theorem_l3 perfect_cuboid_theorem_s3 perfect_cuboid_theorem_s4) (.seq (.arrow perfect_cuboid_theorem_l4 perfect_cuboid_theorem_s4 perfect_cuboid_theorem_s5) (.seq (.arrow perfect_cuboid_theorem_l5 perfect_cuboid_theorem_s5 perfect_cuboid_theorem_s6) (.seq (.prod (.arrow perfect_cuboid_theorem_l6 perfect_cuboid_theorem_s6 perfect_cuboid_theorem_s9) (.arrow perfect_cuboid_theorem_l6 perfect_cuboid_theorem_s6 perfect_cuboid_theorem_s9)) (.seq (.arrow perfect_cuboid_theorem_l9 perfect_cuboid_theorem_s9 perfect_cuboid_theorem_s9) (.seq (.arrow perfect_cuboid_theorem_l9 perfect_cuboid_theorem_s9 perfect_cuboid_theorem_s10) (.seq (.arrow perfect_cuboid_theorem_l10 perfect_cuboid_theorem_s10 perfect_cuboid_theorem_s11) (.seq (.arrow perfect_cuboid_theorem_l11 perfect_cuboid_theorem_s11 perfect_cuboid_theorem_s12) (.seq (.arrow perfect_cuboid_theorem_l12 perfect_cuboid_theorem_s12 perfect_cuboid_theorem_s13) (.seq (.arrow perfect_cuboid_theorem_l13 perfect_cuboid_theorem_s13 perfect_cuboid_theorem_s14) (.seq (.arrow perfect_cuboid_theorem_l14 perfect_cuboid_theorem_s14 perfect_cuboid_theorem_s15) (.arrow perfect_cuboid_theorem_l15 perfect_cuboid_theorem_s15 perfect_cuboid_theorem_s16)))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def perfect_cuboid_theorem_true_arm : IGProtocol perfect_cuboid_theorem_s0 perfect_cuboid_theorem_s16 :=
  (perfect_cuboid_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def perfect_cuboid_theorem_false_arm : IGProtocol perfect_cuboid_theorem_s0 perfect_cuboid_theorem_s16 :=
  (perfect_cuboid_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def perfect_cuboid_theorem_tier : OuroboricityTier := TierFunctor.obj perfect_cuboid_theorem_s0
#eval perfect_cuboid_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem perfect_cuboid_theorem_frobenius :
    igFrobeniusAlg.mul perfect_cuboid_theorem_s0 perfect_cuboid_theorem_s0 = perfect_cuboid_theorem_s0 :=
  igFrobAlg_self_fusion perfect_cuboid_theorem_s0
