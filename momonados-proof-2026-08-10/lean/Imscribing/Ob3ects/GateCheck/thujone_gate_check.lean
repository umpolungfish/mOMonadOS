-- IGProtocol scaffold: VINIT → FSPLIT → EVALT → AFWD → FFUSE → EVALF → AREV → ENGAGR → IMSCRIB → IFIX → TANCH
-- Class: thujone
-- Fingerprint: sig=(5,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=11
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 4)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [5] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [6] AREV      pol    := 𐑗               𐑖 → 𐑳  | reverse morphism — parity flip
--   [7] ENGAGR    stoi   := 𐑳               𐑗 → 𐑠  | engage paradox — B-state, both arms
--   [8] IMSCRIB   gram   := 𐑠               𐑳 → 𐑭  | identity — self-imscription
--   [9] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [10] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def thujone_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def thujone_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def thujone_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def thujone_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def thujone_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def thujone_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def thujone_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def thujone_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def thujone_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def thujone_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def thujone_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def thujone_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def thujone_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def thujone_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def thujone_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def thujone_protocol : IGProtocol thujone_s0 thujone_s10 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct thujone_s4 thujone_s4 = thujone_s4 (idempotent)
  (.seq (.arrow thujone_l0 thujone_s0 thujone_s1) (.seq (.prod (.arrow thujone_l1 thujone_s1 thujone_s4) (.arrow thujone_l1 thujone_s1 thujone_s4)) (.seq (.arrow thujone_l4 thujone_s4 thujone_s4) (.seq (.arrow thujone_l4 thujone_s4 thujone_s5) (.seq (.arrow thujone_l5 thujone_s5 thujone_s6) (.seq (.arrow thujone_l6 thujone_s6 thujone_s7) (.seq (.arrow thujone_l7 thujone_s7 thujone_s8) (.seq (.arrow thujone_l8 thujone_s8 thujone_s9) (.arrow thujone_l9 thujone_s9 thujone_s10)))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def thujone_true_arm : IGProtocol thujone_s0 thujone_s10 :=
  (thujone_protocol).restrictToEVALT

-- false arm
noncomputable def thujone_false_arm : IGProtocol thujone_s0 thujone_s10 :=
  (thujone_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def thujone_tier_ground : OuroboricityTier := TierFunctor.obj thujone_s0
def thujone_tier : OuroboricityTier := TierFunctor.obj thujone_s10
#eval thujone_tier_ground  -- tier of the ground (pre-transformation)
#eval thujone_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem thujone_frobenius :
    igFrobeniusAlg.mul thujone_s0 thujone_s0 = thujone_s0 :=
  igFrobAlg_self_fusion thujone_s0
