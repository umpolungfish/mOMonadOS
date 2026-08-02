-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → AFWD → EVALT → AREV → EVALF → FFUSE → CLINK → IFIX → TANCH
-- Class: Initialize the unmapped branch state within the triple frame architecture
-- Fingerprint: sig=(6,2,2,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=11
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 7)]

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
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [6] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [9] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def initialize_the_unmapped_branch_state_d4f02a_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def initialize_the_unmapped_branch_state_d4f02a_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def initialize_the_unmapped_branch_state_d4f02a_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def initialize_the_unmapped_branch_state_d4f02a_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def initialize_the_unmapped_branch_state_d4f02a_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def initialize_the_unmapped_branch_state_d4f02a_protocol : IGProtocol initialize_the_unmapped_branch_state_d4f02a_s0 initialize_the_unmapped_branch_state_d4f02a_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct initialize_the_unmapped_branch_state_d4f02a_s7 initialize_the_unmapped_branch_state_d4f02a_s7 = initialize_the_unmapped_branch_state_d4f02a_s7 (idempotent)
  (.seq (.arrow initialize_the_unmapped_branch_state_d4f02a_l0 initialize_the_unmapped_branch_state_d4f02a_s0 initialize_the_unmapped_branch_state_d4f02a_s1) (.seq (.arrow initialize_the_unmapped_branch_state_d4f02a_l1 initialize_the_unmapped_branch_state_d4f02a_s1 initialize_the_unmapped_branch_state_d4f02a_s2) (.seq (.prod (.arrow initialize_the_unmapped_branch_state_d4f02a_l2 initialize_the_unmapped_branch_state_d4f02a_s2 initialize_the_unmapped_branch_state_d4f02a_s7) (.arrow initialize_the_unmapped_branch_state_d4f02a_l2 initialize_the_unmapped_branch_state_d4f02a_s2 initialize_the_unmapped_branch_state_d4f02a_s7)) (.seq (.arrow initialize_the_unmapped_branch_state_d4f02a_l7 initialize_the_unmapped_branch_state_d4f02a_s7 initialize_the_unmapped_branch_state_d4f02a_s7) (.seq (.arrow initialize_the_unmapped_branch_state_d4f02a_l7 initialize_the_unmapped_branch_state_d4f02a_s7 initialize_the_unmapped_branch_state_d4f02a_s8) (.seq (.arrow initialize_the_unmapped_branch_state_d4f02a_l8 initialize_the_unmapped_branch_state_d4f02a_s8 initialize_the_unmapped_branch_state_d4f02a_s9) (.arrow initialize_the_unmapped_branch_state_d4f02a_l9 initialize_the_unmapped_branch_state_d4f02a_s9 initialize_the_unmapped_branch_state_d4f02a_s10)))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def initialize_the_unmapped_branch_state_d4f02a_true_arm : IGProtocol initialize_the_unmapped_branch_state_d4f02a_s0 initialize_the_unmapped_branch_state_d4f02a_s10 :=
  (initialize_the_unmapped_branch_state_d4f02a_protocol).restrictToEVALT

-- false arm
noncomputable def initialize_the_unmapped_branch_state_d4f02a_false_arm : IGProtocol initialize_the_unmapped_branch_state_d4f02a_s0 initialize_the_unmapped_branch_state_d4f02a_s10 :=
  (initialize_the_unmapped_branch_state_d4f02a_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def initialize_the_unmapped_branch_state_d4f02a_tier_ground : OuroboricityTier := TierFunctor.obj initialize_the_unmapped_branch_state_d4f02a_s0
def initialize_the_unmapped_branch_state_d4f02a_tier : OuroboricityTier := TierFunctor.obj initialize_the_unmapped_branch_state_d4f02a_s10
#eval initialize_the_unmapped_branch_state_d4f02a_tier_ground  -- tier of the ground (pre-transformation)
#eval initialize_the_unmapped_branch_state_d4f02a_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem initialize_the_unmapped_branch_state_d4f02a_frobenius :
    igFrobeniusAlg.mul initialize_the_unmapped_branch_state_d4f02a_s0 initialize_the_unmapped_branch_state_d4f02a_s0 = initialize_the_unmapped_branch_state_d4f02a_s0 :=
  igFrobAlg_self_fusion initialize_the_unmapped_branch_state_d4f02a_s0
