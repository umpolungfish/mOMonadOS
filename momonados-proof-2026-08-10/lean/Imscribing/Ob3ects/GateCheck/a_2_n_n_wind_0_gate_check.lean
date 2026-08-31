-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → CLINK → FSPLIT → EVALT → IFIX → FFUSE → IMSCRIB → TANCH
-- Class: ∮_γ A = 2πn ∧ n ∈ ℤ ∧ wind(γ) ≠ 0
-- Fingerprint: sig=(6,2,1,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=10
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(4, 7)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑱  | forward morphism — bidirectional arrow
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [4] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [8] IMSCRIB   gram   := 𐑠               𐑙 → 𐑡  | identity — self-imscription
--   [9] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_2_n_n_wind_0_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def a_2_n_n_wind_0_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def a_2_n_n_wind_0_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def a_2_n_n_wind_0_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_2_n_n_wind_0_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_2_n_n_wind_0_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_2_n_n_wind_0_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_2_n_n_wind_0_protocol : IGProtocol a_2_n_n_wind_0_s0 a_2_n_n_wind_0_s9 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_2_n_n_wind_0_s7 a_2_n_n_wind_0_s7 = a_2_n_n_wind_0_s7 (idempotent)
  (.seq (.arrow a_2_n_n_wind_0_l0 a_2_n_n_wind_0_s0 a_2_n_n_wind_0_s1) (.seq (.arrow a_2_n_n_wind_0_l1 a_2_n_n_wind_0_s1 a_2_n_n_wind_0_s2) (.seq (.arrow a_2_n_n_wind_0_l2 a_2_n_n_wind_0_s2 a_2_n_n_wind_0_s3) (.seq (.arrow a_2_n_n_wind_0_l3 a_2_n_n_wind_0_s3 a_2_n_n_wind_0_s4) (.seq (.prod (.arrow a_2_n_n_wind_0_l4 a_2_n_n_wind_0_s4 a_2_n_n_wind_0_s7) (.arrow a_2_n_n_wind_0_l4 a_2_n_n_wind_0_s4 a_2_n_n_wind_0_s7)) (.seq (.arrow a_2_n_n_wind_0_l7 a_2_n_n_wind_0_s7 a_2_n_n_wind_0_s7) (.seq (.arrow a_2_n_n_wind_0_l7 a_2_n_n_wind_0_s7 a_2_n_n_wind_0_s8) (.arrow a_2_n_n_wind_0_l8 a_2_n_n_wind_0_s8 a_2_n_n_wind_0_s9))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_2_n_n_wind_0_true_arm : IGProtocol a_2_n_n_wind_0_s0 a_2_n_n_wind_0_s9 :=
  (a_2_n_n_wind_0_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_2_n_n_wind_0_tier_ground : OuroboricityTier := TierFunctor.obj a_2_n_n_wind_0_s0
def a_2_n_n_wind_0_tier : OuroboricityTier := TierFunctor.obj a_2_n_n_wind_0_s9
#eval a_2_n_n_wind_0_tier_ground  -- tier of the ground (pre-transformation)
#eval a_2_n_n_wind_0_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_2_n_n_wind_0_frobenius :
    igFrobeniusAlg.mul a_2_n_n_wind_0_s0 a_2_n_n_wind_0_s0 = a_2_n_n_wind_0_s0 :=
  igFrobAlg_self_fusion a_2_n_n_wind_0_s0
