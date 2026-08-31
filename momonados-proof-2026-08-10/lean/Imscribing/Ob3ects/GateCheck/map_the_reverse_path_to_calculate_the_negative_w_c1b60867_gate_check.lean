-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → AREV → EVALF → FFUSE → IFIX → CLINK → TANCH
-- Class: Map the reverse path to calculate the negative winding contribution
-- Fingerprint: sig=(5,2,1,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=9
-- Expected tier: O₀
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
--   [4] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [6] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [7] CLINK     fid    := 𐑱               𐑭 → 𐑡  | composition — regime coherence
--   [8] TANCH     top    := 𐑡               𐑱 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def map_the_reverse_path_to_calculate_the_211fcb_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_s6 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := ah }
private def map_the_reverse_path_to_calculate_the_211fcb_s7 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := ah }
private def map_the_reverse_path_to_calculate_the_211fcb_s8 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def map_the_reverse_path_to_calculate_the_211fcb_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def map_the_reverse_path_to_calculate_the_211fcb_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def map_the_reverse_path_to_calculate_the_211fcb_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def map_the_reverse_path_to_calculate_the_211fcb_protocol : IGProtocol map_the_reverse_path_to_calculate_the_211fcb_s0 map_the_reverse_path_to_calculate_the_211fcb_s8 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct map_the_reverse_path_to_calculate_the_211fcb_s5 map_the_reverse_path_to_calculate_the_211fcb_s5 = map_the_reverse_path_to_calculate_the_211fcb_s5 (idempotent)
  (.seq (.arrow map_the_reverse_path_to_calculate_the_211fcb_l0 map_the_reverse_path_to_calculate_the_211fcb_s0 map_the_reverse_path_to_calculate_the_211fcb_s1) (.seq (.arrow map_the_reverse_path_to_calculate_the_211fcb_l1 map_the_reverse_path_to_calculate_the_211fcb_s1 map_the_reverse_path_to_calculate_the_211fcb_s2) (.seq (.prod (.arrow map_the_reverse_path_to_calculate_the_211fcb_l2 map_the_reverse_path_to_calculate_the_211fcb_s2 map_the_reverse_path_to_calculate_the_211fcb_s5) (.arrow map_the_reverse_path_to_calculate_the_211fcb_l2 map_the_reverse_path_to_calculate_the_211fcb_s2 map_the_reverse_path_to_calculate_the_211fcb_s5)) (.seq (.arrow map_the_reverse_path_to_calculate_the_211fcb_l5 map_the_reverse_path_to_calculate_the_211fcb_s5 map_the_reverse_path_to_calculate_the_211fcb_s5) (.seq (.arrow map_the_reverse_path_to_calculate_the_211fcb_l5 map_the_reverse_path_to_calculate_the_211fcb_s5 map_the_reverse_path_to_calculate_the_211fcb_s6) (.seq (.arrow map_the_reverse_path_to_calculate_the_211fcb_l6 map_the_reverse_path_to_calculate_the_211fcb_s6 map_the_reverse_path_to_calculate_the_211fcb_s7) (.arrow map_the_reverse_path_to_calculate_the_211fcb_l7 map_the_reverse_path_to_calculate_the_211fcb_s7 map_the_reverse_path_to_calculate_the_211fcb_s8)))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- false arm
noncomputable def map_the_reverse_path_to_calculate_the_211fcb_false_arm : IGProtocol map_the_reverse_path_to_calculate_the_211fcb_s0 map_the_reverse_path_to_calculate_the_211fcb_s8 :=
  (map_the_reverse_path_to_calculate_the_211fcb_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def map_the_reverse_path_to_calculate_the_211fcb_tier_ground : OuroboricityTier := TierFunctor.obj map_the_reverse_path_to_calculate_the_211fcb_s0
def map_the_reverse_path_to_calculate_the_211fcb_tier : OuroboricityTier := TierFunctor.obj map_the_reverse_path_to_calculate_the_211fcb_s8
#eval map_the_reverse_path_to_calculate_the_211fcb_tier_ground  -- tier of the ground (pre-transformation)
#eval map_the_reverse_path_to_calculate_the_211fcb_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem map_the_reverse_path_to_calculate_the_211fcb_frobenius :
    igFrobeniusAlg.mul map_the_reverse_path_to_calculate_the_211fcb_s0 map_the_reverse_path_to_calculate_the_211fcb_s0 = map_the_reverse_path_to_calculate_the_211fcb_s0 :=
  igFrobAlg_self_fusion map_the_reverse_path_to_calculate_the_211fcb_s0
