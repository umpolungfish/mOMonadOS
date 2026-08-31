-- IGProtocol scaffold: VINIT → FSPLIT → EVALT → AFWD → CLINK → ENGAGR → CLINK → EVALF → AREV → ENGAGR → CLINK → AFWD → FFUSE → IFIX → TANCH
-- Class: A protocol program with a dyad that brackets a transform for the reseed ring
-- Fingerprint: sig=(8,2,4,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=15
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 12)]

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
--   [4] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [5] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [6] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [10] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [11] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [13] IFIX      prot   := 𐑭               𐑙 → 𐑡  | irreversible fixation — winding number
--   [14] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_protocol_program_with_a_dyad_that_aed636_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_protocol_program_with_a_dyad_that_aed636_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_protocol_program_with_a_dyad_that_aed636_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_protocol_program_with_a_dyad_that_aed636_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_protocol_program_with_a_dyad_that_aed636_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_protocol_program_with_a_dyad_that_aed636_protocol : IGProtocol a_protocol_program_with_a_dyad_that_aed636_s0 a_protocol_program_with_a_dyad_that_aed636_s14 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_protocol_program_with_a_dyad_that_aed636_s12 a_protocol_program_with_a_dyad_that_aed636_s12 = a_protocol_program_with_a_dyad_that_aed636_s12 (idempotent)
  (.seq (.arrow a_protocol_program_with_a_dyad_that_aed636_l0 a_protocol_program_with_a_dyad_that_aed636_s0 a_protocol_program_with_a_dyad_that_aed636_s1) (.seq (.prod (.arrow a_protocol_program_with_a_dyad_that_aed636_l1 a_protocol_program_with_a_dyad_that_aed636_s1 a_protocol_program_with_a_dyad_that_aed636_s12) (.arrow a_protocol_program_with_a_dyad_that_aed636_l1 a_protocol_program_with_a_dyad_that_aed636_s1 a_protocol_program_with_a_dyad_that_aed636_s12)) (.seq (.arrow a_protocol_program_with_a_dyad_that_aed636_l12 a_protocol_program_with_a_dyad_that_aed636_s12 a_protocol_program_with_a_dyad_that_aed636_s12) (.seq (.arrow a_protocol_program_with_a_dyad_that_aed636_l12 a_protocol_program_with_a_dyad_that_aed636_s12 a_protocol_program_with_a_dyad_that_aed636_s13) (.arrow a_protocol_program_with_a_dyad_that_aed636_l13 a_protocol_program_with_a_dyad_that_aed636_s13 a_protocol_program_with_a_dyad_that_aed636_s14)))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_protocol_program_with_a_dyad_that_aed636_true_arm : IGProtocol a_protocol_program_with_a_dyad_that_aed636_s0 a_protocol_program_with_a_dyad_that_aed636_s14 :=
  (a_protocol_program_with_a_dyad_that_aed636_protocol).restrictToEVALT

-- false arm
noncomputable def a_protocol_program_with_a_dyad_that_aed636_false_arm : IGProtocol a_protocol_program_with_a_dyad_that_aed636_s0 a_protocol_program_with_a_dyad_that_aed636_s14 :=
  (a_protocol_program_with_a_dyad_that_aed636_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_protocol_program_with_a_dyad_that_aed636_tier : OuroboricityTier := TierFunctor.obj a_protocol_program_with_a_dyad_that_aed636_s0
#eval a_protocol_program_with_a_dyad_that_aed636_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_protocol_program_with_a_dyad_that_aed636_frobenius :
    igFrobeniusAlg.mul a_protocol_program_with_a_dyad_that_aed636_s0 a_protocol_program_with_a_dyad_that_aed636_s0 = a_protocol_program_with_a_dyad_that_aed636_s0 :=
  igFrobAlg_self_fusion a_protocol_program_with_a_dyad_that_aed636_s0
