-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → AFWD → CLINK → FSPLIT → EVALT → IFIX → FFUSE → FSPLIT → EVALF → AREV → ENGAGR → IFIX → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: the nonzero index of an odd ring: n_plus minus n_minus = -1 for the macrocycle [[0,2,1],[2,0,1],[1,1,0]], 0 for every even cycle
-- Fingerprint: sig=(9,4,3,3)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=19
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(5, 8), (9, 14)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑾  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑾 → 𐑱  | forward morphism — bidirectional arrow
--   [4] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [9] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [12] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [13] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [14] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [15] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [16] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [17] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [18] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_protocol : IGProtocol the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s0 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s18 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8 = the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8 (idempotent)
  (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l0 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s0 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s1) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l1 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s1 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s2) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l2 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s2 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s3) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l3 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s3 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s4) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l4 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s4 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s5) (.seq (.prod (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l5 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s5 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8) (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l5 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s5 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8)) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l8 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l8 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s8 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s9) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l9 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s9 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s10) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l10 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s10 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s11) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l11 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s11 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s12) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l12 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s12 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s13) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l13 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s13 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s14) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l14 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s14 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s15) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l15 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s15 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s16) (.seq (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l16 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s16 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s17) (.arrow the_nonzero_index_of_an_odd_ring_n_plus_8d6162_l17 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s17 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s18)))))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_true_arm : IGProtocol the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s0 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s18 :=
  (the_nonzero_index_of_an_odd_ring_n_plus_8d6162_protocol).restrictToEVALT

-- false arm
noncomputable def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_false_arm : IGProtocol the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s0 the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s18 :=
  (the_nonzero_index_of_an_odd_ring_n_plus_8d6162_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_nonzero_index_of_an_odd_ring_n_plus_8d6162_tier : OuroboricityTier := TierFunctor.obj the_nonzero_index_of_an_odd_ring_n_plus_8d6162_s0
#eval the_nonzero_index_of_an_odd_ring_n_plus_8d6162_tier  -- the Grammar's own verdict on its tier
