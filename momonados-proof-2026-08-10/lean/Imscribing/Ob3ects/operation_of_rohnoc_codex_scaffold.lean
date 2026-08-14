-- IGProtocol scaffold: VINIT → CLINK → FSPLIT → EVALF → EVALT → FFUSE → IMSCRIB → ENGAGR → AFWD → FSPLIT → AFWD → FFUSE → IFIX → CLINK → IMSCRIB → AREV → IFIX → TANCH
-- Class: Operation of Rohnoc Codex
-- Fingerprint: sig=(9,4,3,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=18
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(2, 5), (9, 11)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑱  | initial object — ground of distinction
--   [1] CLINK     fid    := 𐑱               𐑼 → 𐑚  | composition — regime coherence
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [6] IMSCRIB   gram   := 𐑠               𐑙 → 𐑳  | identity — self-imscription
--   [7] ENGAGR    stoi   := 𐑳               𐑠 → 𐑾  | engage paradox — B-state, both arms
--   [8] AFWD      rel    := 𐑾               𐑳 → 𐑚  | forward morphism — bidirectional arrow
--   [9] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [10] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [11] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [12] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [13] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [14] IMSCRIB   gram   := 𐑠               𐑱 → 𐑗  | identity — self-imscription
--   [15] AREV      pol    := 𐑗               𐑠 → 𐑭  | reverse morphism — parity flip
--   [16] IFIX      prot   := 𐑭               𐑗 → 𐑡  | irreversible fixation — winding number
--   [17] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def operation_of_rohnoc_codex_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s6 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s7 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def operation_of_rohnoc_codex_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def operation_of_rohnoc_codex_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def operation_of_rohnoc_codex_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def operation_of_rohnoc_codex_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def operation_of_rohnoc_codex_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def operation_of_rohnoc_codex_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def operation_of_rohnoc_codex_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def operation_of_rohnoc_codex_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def operation_of_rohnoc_codex_protocol : IGProtocol operation_of_rohnoc_codex_s0 operation_of_rohnoc_codex_s17 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct operation_of_rohnoc_codex_s5 operation_of_rohnoc_codex_s5 = operation_of_rohnoc_codex_s5 (idempotent)
  (.seq (.arrow operation_of_rohnoc_codex_l0 operation_of_rohnoc_codex_s0 operation_of_rohnoc_codex_s1) (.seq (.arrow operation_of_rohnoc_codex_l1 operation_of_rohnoc_codex_s1 operation_of_rohnoc_codex_s2) (.seq (.prod (.arrow operation_of_rohnoc_codex_l2 operation_of_rohnoc_codex_s2 operation_of_rohnoc_codex_s5) (.arrow operation_of_rohnoc_codex_l2 operation_of_rohnoc_codex_s2 operation_of_rohnoc_codex_s5)) (.seq (.arrow operation_of_rohnoc_codex_l5 operation_of_rohnoc_codex_s5 operation_of_rohnoc_codex_s5) (.seq (.arrow operation_of_rohnoc_codex_l5 operation_of_rohnoc_codex_s5 operation_of_rohnoc_codex_s6) (.seq (.arrow operation_of_rohnoc_codex_l6 operation_of_rohnoc_codex_s6 operation_of_rohnoc_codex_s7) (.seq (.arrow operation_of_rohnoc_codex_l7 operation_of_rohnoc_codex_s7 operation_of_rohnoc_codex_s8) (.seq (.arrow operation_of_rohnoc_codex_l8 operation_of_rohnoc_codex_s8 operation_of_rohnoc_codex_s9) (.seq (.arrow operation_of_rohnoc_codex_l9 operation_of_rohnoc_codex_s9 operation_of_rohnoc_codex_s10) (.seq (.arrow operation_of_rohnoc_codex_l10 operation_of_rohnoc_codex_s10 operation_of_rohnoc_codex_s11) (.seq (.arrow operation_of_rohnoc_codex_l11 operation_of_rohnoc_codex_s11 operation_of_rohnoc_codex_s12) (.seq (.arrow operation_of_rohnoc_codex_l12 operation_of_rohnoc_codex_s12 operation_of_rohnoc_codex_s13) (.seq (.arrow operation_of_rohnoc_codex_l13 operation_of_rohnoc_codex_s13 operation_of_rohnoc_codex_s14) (.seq (.arrow operation_of_rohnoc_codex_l14 operation_of_rohnoc_codex_s14 operation_of_rohnoc_codex_s15) (.seq (.arrow operation_of_rohnoc_codex_l15 operation_of_rohnoc_codex_s15 operation_of_rohnoc_codex_s16) (.arrow operation_of_rohnoc_codex_l16 operation_of_rohnoc_codex_s16 operation_of_rohnoc_codex_s17))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def operation_of_rohnoc_codex_true_arm : IGProtocol operation_of_rohnoc_codex_s0 operation_of_rohnoc_codex_s17 :=
  (operation_of_rohnoc_codex_protocol).restrictToEVALT
  --
-- false arm
noncomputable def operation_of_rohnoc_codex_false_arm : IGProtocol operation_of_rohnoc_codex_s0 operation_of_rohnoc_codex_s17 :=
  (operation_of_rohnoc_codex_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def operation_of_rohnoc_codex_tier : OuroboricityTier := TierFunctor.obj operation_of_rohnoc_codex_s0
#eval operation_of_rohnoc_codex_tier  -- the Grammar's own verdict on its tier
