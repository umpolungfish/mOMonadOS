-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → CLINK → IFIX → TANCH
-- Class: verify the persistence of the coherent signal in the primary arm
-- Fingerprint: sig=(5,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=11
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 7)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑚  | forward morphism — bidirectional arrow
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [9] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def verify_the_persistence_of_the_coherent_3d9754_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def verify_the_persistence_of_the_coherent_3d9754_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def verify_the_persistence_of_the_coherent_3d9754_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def verify_the_persistence_of_the_coherent_3d9754_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def verify_the_persistence_of_the_coherent_3d9754_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def verify_the_persistence_of_the_coherent_3d9754_protocol : IGProtocol verify_the_persistence_of_the_coherent_3d9754_s0 verify_the_persistence_of_the_coherent_3d9754_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct verify_the_persistence_of_the_coherent_3d9754_s7 verify_the_persistence_of_the_coherent_3d9754_s7 = verify_the_persistence_of_the_coherent_3d9754_s7 (idempotent)
  (.seq (.arrow verify_the_persistence_of_the_coherent_3d9754_l0 verify_the_persistence_of_the_coherent_3d9754_s0 verify_the_persistence_of_the_coherent_3d9754_s1) (.seq (.arrow verify_the_persistence_of_the_coherent_3d9754_l1 verify_the_persistence_of_the_coherent_3d9754_s1 verify_the_persistence_of_the_coherent_3d9754_s2) (.seq (.arrow verify_the_persistence_of_the_coherent_3d9754_l2 verify_the_persistence_of_the_coherent_3d9754_s2 verify_the_persistence_of_the_coherent_3d9754_s3) (.seq (.prod (.arrow verify_the_persistence_of_the_coherent_3d9754_l3 verify_the_persistence_of_the_coherent_3d9754_s3 verify_the_persistence_of_the_coherent_3d9754_s7) (.arrow verify_the_persistence_of_the_coherent_3d9754_l3 verify_the_persistence_of_the_coherent_3d9754_s3 verify_the_persistence_of_the_coherent_3d9754_s7)) (.seq (.arrow verify_the_persistence_of_the_coherent_3d9754_l7 verify_the_persistence_of_the_coherent_3d9754_s7 verify_the_persistence_of_the_coherent_3d9754_s7) (.seq (.arrow verify_the_persistence_of_the_coherent_3d9754_l7 verify_the_persistence_of_the_coherent_3d9754_s7 verify_the_persistence_of_the_coherent_3d9754_s8) (.seq (.arrow verify_the_persistence_of_the_coherent_3d9754_l8 verify_the_persistence_of_the_coherent_3d9754_s8 verify_the_persistence_of_the_coherent_3d9754_s9) (.arrow verify_the_persistence_of_the_coherent_3d9754_l9 verify_the_persistence_of_the_coherent_3d9754_s9 verify_the_persistence_of_the_coherent_3d9754_s10))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def verify_the_persistence_of_the_coherent_3d9754_true_arm : IGProtocol verify_the_persistence_of_the_coherent_3d9754_s0 verify_the_persistence_of_the_coherent_3d9754_s10 :=
  (verify_the_persistence_of_the_coherent_3d9754_protocol).restrictToEVALT

-- false arm
noncomputable def verify_the_persistence_of_the_coherent_3d9754_false_arm : IGProtocol verify_the_persistence_of_the_coherent_3d9754_s0 verify_the_persistence_of_the_coherent_3d9754_s10 :=
  (verify_the_persistence_of_the_coherent_3d9754_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def verify_the_persistence_of_the_coherent_3d9754_tier_ground : OuroboricityTier := TierFunctor.obj verify_the_persistence_of_the_coherent_3d9754_s0
def verify_the_persistence_of_the_coherent_3d9754_tier : OuroboricityTier := TierFunctor.obj verify_the_persistence_of_the_coherent_3d9754_s10
#eval verify_the_persistence_of_the_coherent_3d9754_tier_ground  -- tier of the ground (pre-transformation)
#eval verify_the_persistence_of_the_coherent_3d9754_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem verify_the_persistence_of_the_coherent_3d9754_frobenius :
    igFrobeniusAlg.mul verify_the_persistence_of_the_coherent_3d9754_s0 verify_the_persistence_of_the_coherent_3d9754_s0 = verify_the_persistence_of_the_coherent_3d9754_s0 :=
  igFrobAlg_self_fusion verify_the_persistence_of_the_coherent_3d9754_s0
