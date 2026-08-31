-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → EVALF → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: excite the system toward the target modal state
-- Fingerprint: sig=(6,2,2,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=11
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 6)]

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
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [7] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [8] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [9] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def excite_the_system_toward_the_target_5e245c_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def excite_the_system_toward_the_target_5e245c_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def excite_the_system_toward_the_target_5e245c_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def excite_the_system_toward_the_target_5e245c_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def excite_the_system_toward_the_target_5e245c_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def excite_the_system_toward_the_target_5e245c_protocol : IGProtocol excite_the_system_toward_the_target_5e245c_s0 excite_the_system_toward_the_target_5e245c_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct excite_the_system_toward_the_target_5e245c_s6 excite_the_system_toward_the_target_5e245c_s6 = excite_the_system_toward_the_target_5e245c_s6 (idempotent)
  (.seq (.arrow excite_the_system_toward_the_target_5e245c_l0 excite_the_system_toward_the_target_5e245c_s0 excite_the_system_toward_the_target_5e245c_s1) (.seq (.arrow excite_the_system_toward_the_target_5e245c_l1 excite_the_system_toward_the_target_5e245c_s1 excite_the_system_toward_the_target_5e245c_s2) (.seq (.arrow excite_the_system_toward_the_target_5e245c_l2 excite_the_system_toward_the_target_5e245c_s2 excite_the_system_toward_the_target_5e245c_s3) (.seq (.prod (.arrow excite_the_system_toward_the_target_5e245c_l3 excite_the_system_toward_the_target_5e245c_s3 excite_the_system_toward_the_target_5e245c_s6) (.arrow excite_the_system_toward_the_target_5e245c_l3 excite_the_system_toward_the_target_5e245c_s3 excite_the_system_toward_the_target_5e245c_s6)) (.seq (.arrow excite_the_system_toward_the_target_5e245c_l6 excite_the_system_toward_the_target_5e245c_s6 excite_the_system_toward_the_target_5e245c_s6) (.seq (.arrow excite_the_system_toward_the_target_5e245c_l6 excite_the_system_toward_the_target_5e245c_s6 excite_the_system_toward_the_target_5e245c_s7) (.seq (.arrow excite_the_system_toward_the_target_5e245c_l7 excite_the_system_toward_the_target_5e245c_s7 excite_the_system_toward_the_target_5e245c_s8) (.seq (.arrow excite_the_system_toward_the_target_5e245c_l8 excite_the_system_toward_the_target_5e245c_s8 excite_the_system_toward_the_target_5e245c_s9) (.arrow excite_the_system_toward_the_target_5e245c_l9 excite_the_system_toward_the_target_5e245c_s9 excite_the_system_toward_the_target_5e245c_s10)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def excite_the_system_toward_the_target_5e245c_true_arm : IGProtocol excite_the_system_toward_the_target_5e245c_s0 excite_the_system_toward_the_target_5e245c_s10 :=
  (excite_the_system_toward_the_target_5e245c_protocol).restrictToEVALT

-- false arm
noncomputable def excite_the_system_toward_the_target_5e245c_false_arm : IGProtocol excite_the_system_toward_the_target_5e245c_s0 excite_the_system_toward_the_target_5e245c_s10 :=
  (excite_the_system_toward_the_target_5e245c_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def excite_the_system_toward_the_target_5e245c_tier_ground : OuroboricityTier := TierFunctor.obj excite_the_system_toward_the_target_5e245c_s0
def excite_the_system_toward_the_target_5e245c_tier : OuroboricityTier := TierFunctor.obj excite_the_system_toward_the_target_5e245c_s10
#eval excite_the_system_toward_the_target_5e245c_tier_ground  -- tier of the ground (pre-transformation)
#eval excite_the_system_toward_the_target_5e245c_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem excite_the_system_toward_the_target_5e245c_frobenius :
    igFrobeniusAlg.mul excite_the_system_toward_the_target_5e245c_s0 excite_the_system_toward_the_target_5e245c_s0 = excite_the_system_toward_the_target_5e245c_s0 :=
  igFrobAlg_self_fusion excite_the_system_toward_the_target_5e245c_s0
