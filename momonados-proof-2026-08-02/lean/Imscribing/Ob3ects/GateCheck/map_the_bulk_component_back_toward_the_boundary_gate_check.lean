-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → AREV → EVALT → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: Map the bulk component back toward the boundary
-- Fingerprint: sig=(6,2,1,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=10
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 5)]

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
--   [3] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [6] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [7] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [8] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [9] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def map_the_bulk_component_back_toward_the_a7f973_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s6 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s7 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_s8 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def map_the_bulk_component_back_toward_the_a7f973_s9 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def map_the_bulk_component_back_toward_the_a7f973_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_bulk_component_back_toward_the_a7f973_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def map_the_bulk_component_back_toward_the_a7f973_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def map_the_bulk_component_back_toward_the_a7f973_protocol : IGProtocol map_the_bulk_component_back_toward_the_a7f973_s0 map_the_bulk_component_back_toward_the_a7f973_s9 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct map_the_bulk_component_back_toward_the_a7f973_s5 map_the_bulk_component_back_toward_the_a7f973_s5 = map_the_bulk_component_back_toward_the_a7f973_s5 (idempotent)
  (.seq (.arrow map_the_bulk_component_back_toward_the_a7f973_l0 map_the_bulk_component_back_toward_the_a7f973_s0 map_the_bulk_component_back_toward_the_a7f973_s1) (.seq (.arrow map_the_bulk_component_back_toward_the_a7f973_l1 map_the_bulk_component_back_toward_the_a7f973_s1 map_the_bulk_component_back_toward_the_a7f973_s2) (.seq (.prod (.arrow map_the_bulk_component_back_toward_the_a7f973_l2 map_the_bulk_component_back_toward_the_a7f973_s2 map_the_bulk_component_back_toward_the_a7f973_s5) (.arrow map_the_bulk_component_back_toward_the_a7f973_l2 map_the_bulk_component_back_toward_the_a7f973_s2 map_the_bulk_component_back_toward_the_a7f973_s5)) (.seq (.arrow map_the_bulk_component_back_toward_the_a7f973_l5 map_the_bulk_component_back_toward_the_a7f973_s5 map_the_bulk_component_back_toward_the_a7f973_s5) (.seq (.arrow map_the_bulk_component_back_toward_the_a7f973_l5 map_the_bulk_component_back_toward_the_a7f973_s5 map_the_bulk_component_back_toward_the_a7f973_s6) (.seq (.arrow map_the_bulk_component_back_toward_the_a7f973_l6 map_the_bulk_component_back_toward_the_a7f973_s6 map_the_bulk_component_back_toward_the_a7f973_s7) (.seq (.arrow map_the_bulk_component_back_toward_the_a7f973_l7 map_the_bulk_component_back_toward_the_a7f973_s7 map_the_bulk_component_back_toward_the_a7f973_s8) (.arrow map_the_bulk_component_back_toward_the_a7f973_l8 map_the_bulk_component_back_toward_the_a7f973_s8 map_the_bulk_component_back_toward_the_a7f973_s9))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def map_the_bulk_component_back_toward_the_a7f973_true_arm : IGProtocol map_the_bulk_component_back_toward_the_a7f973_s0 map_the_bulk_component_back_toward_the_a7f973_s9 :=
  (map_the_bulk_component_back_toward_the_a7f973_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def map_the_bulk_component_back_toward_the_a7f973_tier_ground : OuroboricityTier := TierFunctor.obj map_the_bulk_component_back_toward_the_a7f973_s0
def map_the_bulk_component_back_toward_the_a7f973_tier : OuroboricityTier := TierFunctor.obj map_the_bulk_component_back_toward_the_a7f973_s9
#eval map_the_bulk_component_back_toward_the_a7f973_tier_ground  -- tier of the ground (pre-transformation)
#eval map_the_bulk_component_back_toward_the_a7f973_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem map_the_bulk_component_back_toward_the_a7f973_frobenius :
    igFrobeniusAlg.mul map_the_bulk_component_back_toward_the_a7f973_s0 map_the_bulk_component_back_toward_the_a7f973_s0 = map_the_bulk_component_back_toward_the_a7f973_s0 :=
  igFrobAlg_self_fusion map_the_bulk_component_back_toward_the_a7f973_s0
