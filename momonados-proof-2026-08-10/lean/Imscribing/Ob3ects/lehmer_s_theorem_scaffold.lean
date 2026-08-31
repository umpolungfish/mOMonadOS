-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → CLINK → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → CLINK → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → CLINK → ENGAGR → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → FFUSE → IMSCRIB → TANCH
-- Class: Lehmer's Theorem
-- Fingerprint: sig=(15,5,9,4)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=33
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(24, 30)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑚  | identity — self-imscription
--   [2] FSPLIT    gran   := 𐑚               𐑠 → ⊙  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑾  | evaluate-true — criticality gate open
--   [4] AFWD      rel    := 𐑾               ⊙ → 𐑭  | forward morphism — bidirectional arrow
--   [5] IFIX      prot   := 𐑭               𐑾 → 𐑗  | irreversible fixation — winding number
--   [6] AREV      pol    := 𐑗               𐑭 → 𐑖  | reverse morphism — parity flip
--   [7] EVALF     chir   := 𐑖               𐑗 → 𐑱  | evaluate-false — chirality check
--   [8] CLINK     fid    := 𐑱               𐑖 → 𐑚  | composition — regime coherence
--   [9] FSPLIT    gran   := 𐑚               𐑱 → ⊙  | split δ — range decomposition
--   [10] EVALT     crit   := ⊙               𐑚 → 𐑾  | evaluate-true — criticality gate open
--   [11] AFWD      rel    := 𐑾               ⊙ → 𐑭  | forward morphism — bidirectional arrow
--   [12] IFIX      prot   := 𐑭               𐑾 → 𐑗  | irreversible fixation — winding number
--   [13] AREV      pol    := 𐑗               𐑭 → 𐑖  | reverse morphism — parity flip
--   [14] EVALF     chir   := 𐑖               𐑗 → 𐑱  | evaluate-false — chirality check
--   [15] CLINK     fid    := 𐑱               𐑖 → 𐑚  | composition — regime coherence
--   [16] FSPLIT    gran   := 𐑚               𐑱 → ⊙  | split δ — range decomposition
--   [17] EVALT     crit   := ⊙               𐑚 → 𐑾  | evaluate-true — criticality gate open
--   [18] AFWD      rel    := 𐑾               ⊙ → 𐑭  | forward morphism — bidirectional arrow
--   [19] IFIX      prot   := 𐑭               𐑾 → 𐑗  | irreversible fixation — winding number
--   [20] AREV      pol    := 𐑗               𐑭 → 𐑖  | reverse morphism — parity flip
--   [21] EVALF     chir   := 𐑖               𐑗 → 𐑱  | evaluate-false — chirality check
--   [22] CLINK     fid    := 𐑱               𐑖 → 𐑳  | composition — regime coherence
--   [23] ENGAGR    stoi   := 𐑳               𐑱 → 𐑚  | engage paradox — B-state, both arms
--   [24] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [25] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [26] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [27] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [28] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [29] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [30] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [31] IMSCRIB   gram   := 𐑠               𐑙 → 𐑡  | identity — self-imscription
--   [32] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def lehmers_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def lehmers_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def lehmers_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lehmers_theorem_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lehmers_theorem_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def lehmers_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lehmers_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lehmers_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lehmers_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lehmers_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lehmers_theorem_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lehmers_theorem_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def lehmers_theorem_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l26 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lehmers_theorem_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lehmers_theorem_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lehmers_theorem_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def lehmers_theorem_protocol : IGProtocol lehmers_theorem_s0 lehmers_theorem_s32 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct lehmers_theorem_s30 lehmers_theorem_s30 = lehmers_theorem_s30 (idempotent)
  (.seq (.arrow lehmers_theorem_l0 lehmers_theorem_s0 lehmers_theorem_s1) (.seq (.arrow lehmers_theorem_l1 lehmers_theorem_s1 lehmers_theorem_s2) (.seq (.arrow lehmers_theorem_l2 lehmers_theorem_s2 lehmers_theorem_s3) (.seq (.arrow lehmers_theorem_l3 lehmers_theorem_s3 lehmers_theorem_s4) (.seq (.arrow lehmers_theorem_l4 lehmers_theorem_s4 lehmers_theorem_s5) (.seq (.arrow lehmers_theorem_l5 lehmers_theorem_s5 lehmers_theorem_s6) (.seq (.arrow lehmers_theorem_l6 lehmers_theorem_s6 lehmers_theorem_s7) (.seq (.arrow lehmers_theorem_l7 lehmers_theorem_s7 lehmers_theorem_s8) (.seq (.arrow lehmers_theorem_l8 lehmers_theorem_s8 lehmers_theorem_s9) (.seq (.arrow lehmers_theorem_l9 lehmers_theorem_s9 lehmers_theorem_s10) (.seq (.arrow lehmers_theorem_l10 lehmers_theorem_s10 lehmers_theorem_s11) (.seq (.arrow lehmers_theorem_l11 lehmers_theorem_s11 lehmers_theorem_s12) (.seq (.arrow lehmers_theorem_l12 lehmers_theorem_s12 lehmers_theorem_s13) (.seq (.arrow lehmers_theorem_l13 lehmers_theorem_s13 lehmers_theorem_s14) (.seq (.arrow lehmers_theorem_l14 lehmers_theorem_s14 lehmers_theorem_s15) (.seq (.arrow lehmers_theorem_l15 lehmers_theorem_s15 lehmers_theorem_s16) (.seq (.arrow lehmers_theorem_l16 lehmers_theorem_s16 lehmers_theorem_s17) (.seq (.arrow lehmers_theorem_l17 lehmers_theorem_s17 lehmers_theorem_s18) (.seq (.arrow lehmers_theorem_l18 lehmers_theorem_s18 lehmers_theorem_s19) (.seq (.arrow lehmers_theorem_l19 lehmers_theorem_s19 lehmers_theorem_s20) (.seq (.arrow lehmers_theorem_l20 lehmers_theorem_s20 lehmers_theorem_s21) (.seq (.arrow lehmers_theorem_l21 lehmers_theorem_s21 lehmers_theorem_s22) (.seq (.arrow lehmers_theorem_l22 lehmers_theorem_s22 lehmers_theorem_s23) (.seq (.arrow lehmers_theorem_l23 lehmers_theorem_s23 lehmers_theorem_s24) (.seq (.prod (.arrow lehmers_theorem_l24 lehmers_theorem_s24 lehmers_theorem_s30) (.arrow lehmers_theorem_l24 lehmers_theorem_s24 lehmers_theorem_s30)) (.seq (.arrow lehmers_theorem_l30 lehmers_theorem_s30 lehmers_theorem_s30) (.seq (.arrow lehmers_theorem_l30 lehmers_theorem_s30 lehmers_theorem_s31) (.arrow lehmers_theorem_l31 lehmers_theorem_s31 lehmers_theorem_s32))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def lehmers_theorem_true_arm : IGProtocol lehmers_theorem_s0 lehmers_theorem_s32 :=
  (lehmers_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def lehmers_theorem_false_arm : IGProtocol lehmers_theorem_s0 lehmers_theorem_s32 :=
  (lehmers_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def lehmers_theorem_tier : OuroboricityTier := TierFunctor.obj lehmers_theorem_s0
#eval lehmers_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem lehmers_theorem_frobenius :
    igFrobeniusAlg.mul lehmers_theorem_s0 lehmers_theorem_s0 = lehmers_theorem_s0 :=
  igFrobAlg_self_fusion lehmers_theorem_s0
