-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → AREV → EVALF → FFUSE → CLINK → IFIX → TANCH
-- Class: Calculate the contravariant parity of the verification path
-- Fingerprint: sig=(6,2,2,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=11
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
--   [5] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [6] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [9] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def calculate_the_contravariant_parity_of_b8692e_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def calculate_the_contravariant_parity_of_b8692e_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def calculate_the_contravariant_parity_of_b8692e_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def calculate_the_contravariant_parity_of_b8692e_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def calculate_the_contravariant_parity_of_b8692e_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def calculate_the_contravariant_parity_of_b8692e_protocol : IGProtocol calculate_the_contravariant_parity_of_b8692e_s0 calculate_the_contravariant_parity_of_b8692e_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct calculate_the_contravariant_parity_of_b8692e_s7 calculate_the_contravariant_parity_of_b8692e_s7 = calculate_the_contravariant_parity_of_b8692e_s7 (idempotent)
  (.seq (.arrow calculate_the_contravariant_parity_of_b8692e_l0 calculate_the_contravariant_parity_of_b8692e_s0 calculate_the_contravariant_parity_of_b8692e_s1) (.seq (.arrow calculate_the_contravariant_parity_of_b8692e_l1 calculate_the_contravariant_parity_of_b8692e_s1 calculate_the_contravariant_parity_of_b8692e_s2) (.seq (.arrow calculate_the_contravariant_parity_of_b8692e_l2 calculate_the_contravariant_parity_of_b8692e_s2 calculate_the_contravariant_parity_of_b8692e_s3) (.seq (.prod (.arrow calculate_the_contravariant_parity_of_b8692e_l3 calculate_the_contravariant_parity_of_b8692e_s3 calculate_the_contravariant_parity_of_b8692e_s7) (.arrow calculate_the_contravariant_parity_of_b8692e_l3 calculate_the_contravariant_parity_of_b8692e_s3 calculate_the_contravariant_parity_of_b8692e_s7)) (.seq (.arrow calculate_the_contravariant_parity_of_b8692e_l7 calculate_the_contravariant_parity_of_b8692e_s7 calculate_the_contravariant_parity_of_b8692e_s7) (.seq (.arrow calculate_the_contravariant_parity_of_b8692e_l7 calculate_the_contravariant_parity_of_b8692e_s7 calculate_the_contravariant_parity_of_b8692e_s8) (.seq (.arrow calculate_the_contravariant_parity_of_b8692e_l8 calculate_the_contravariant_parity_of_b8692e_s8 calculate_the_contravariant_parity_of_b8692e_s9) (.arrow calculate_the_contravariant_parity_of_b8692e_l9 calculate_the_contravariant_parity_of_b8692e_s9 calculate_the_contravariant_parity_of_b8692e_s10))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def calculate_the_contravariant_parity_of_b8692e_true_arm : IGProtocol calculate_the_contravariant_parity_of_b8692e_s0 calculate_the_contravariant_parity_of_b8692e_s10 :=
  (calculate_the_contravariant_parity_of_b8692e_protocol).restrictToEVALT

-- false arm
noncomputable def calculate_the_contravariant_parity_of_b8692e_false_arm : IGProtocol calculate_the_contravariant_parity_of_b8692e_s0 calculate_the_contravariant_parity_of_b8692e_s10 :=
  (calculate_the_contravariant_parity_of_b8692e_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def calculate_the_contravariant_parity_of_b8692e_tier_ground : OuroboricityTier := TierFunctor.obj calculate_the_contravariant_parity_of_b8692e_s0
def calculate_the_contravariant_parity_of_b8692e_tier : OuroboricityTier := TierFunctor.obj calculate_the_contravariant_parity_of_b8692e_s10
#eval calculate_the_contravariant_parity_of_b8692e_tier_ground  -- tier of the ground (pre-transformation)
#eval calculate_the_contravariant_parity_of_b8692e_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem calculate_the_contravariant_parity_of_b8692e_frobenius :
    igFrobeniusAlg.mul calculate_the_contravariant_parity_of_b8692e_s0 calculate_the_contravariant_parity_of_b8692e_s0 = calculate_the_contravariant_parity_of_b8692e_s0 :=
  igFrobAlg_self_fusion calculate_the_contravariant_parity_of_b8692e_s0
