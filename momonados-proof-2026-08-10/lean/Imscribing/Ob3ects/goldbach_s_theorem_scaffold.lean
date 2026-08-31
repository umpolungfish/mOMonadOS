-- IGProtocol scaffold: VINIT → TANCH → AFWD → FSPLIT → AFWD → EVALT → IFIX → CLINK → IMSCRIB → AREV → EVALF → ENGAGR → FFUSE → CLINK → AFWD → FSPLIT → AFWD → EVALT → IFIX → CLINK → IMSCRIB → AREV → EVALF → ENGAGR → FFUSE → CLINK → AFWD → FSPLIT → AFWD → EVALT → IFIX → CLINK → IMSCRIB → AREV → EVALF → ENGAGR → FFUSE → CLINK → TANCH
-- Class: Goldbach's Theorem
-- Fingerprint: sig=(21,6,9,3)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=39
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(3, 12), (15, 24), (27, 36)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑾  | terminal object — connectivity boundary
--   [2] AFWD      rel    := 𐑾               𐑡 → 𐑚  | forward morphism — bidirectional arrow
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [7] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [8] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [9] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [13] CLINK     fid    := 𐑱               𐑙 → 𐑾  | composition — regime coherence
--   [14] AFWD      rel    := 𐑾               𐑱 → 𐑚  | forward morphism — bidirectional arrow
--   [15] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [16] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [17] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [18] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [19] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [20] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [21] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [22] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [23] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [24] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [25] CLINK     fid    := 𐑱               𐑙 → 𐑾  | composition — regime coherence
--   [26] AFWD      rel    := 𐑾               𐑱 → 𐑚  | forward morphism — bidirectional arrow
--   [27] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [28] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [29] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [30] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [31] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [32] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [33] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [34] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [35] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [36] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [37] CLINK     fid    := 𐑱               𐑙 → 𐑡  | composition — regime coherence
--   [38] TANCH     top    := 𐑡               𐑱 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def goldbachs_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def goldbachs_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def goldbachs_theorem_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def goldbachs_theorem_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def goldbachs_theorem_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def goldbachs_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def goldbachs_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def goldbachs_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def goldbachs_theorem_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def goldbachs_theorem_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l26 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l28 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def goldbachs_theorem_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l33 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def goldbachs_theorem_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def goldbachs_theorem_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l37 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def goldbachs_theorem_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def goldbachs_theorem_protocol : IGProtocol goldbachs_theorem_s0 goldbachs_theorem_s38 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct goldbachs_theorem_s12 goldbachs_theorem_s12 = goldbachs_theorem_s12 (idempotent)
  (.seq (.arrow goldbachs_theorem_l0 goldbachs_theorem_s0 goldbachs_theorem_s1) (.seq (.arrow goldbachs_theorem_l1 goldbachs_theorem_s1 goldbachs_theorem_s2) (.seq (.arrow goldbachs_theorem_l2 goldbachs_theorem_s2 goldbachs_theorem_s3) (.seq (.prod (.arrow goldbachs_theorem_l3 goldbachs_theorem_s3 goldbachs_theorem_s12) (.arrow goldbachs_theorem_l3 goldbachs_theorem_s3 goldbachs_theorem_s12)) (.seq (.arrow goldbachs_theorem_l12 goldbachs_theorem_s12 goldbachs_theorem_s12) (.seq (.arrow goldbachs_theorem_l12 goldbachs_theorem_s12 goldbachs_theorem_s13) (.seq (.arrow goldbachs_theorem_l13 goldbachs_theorem_s13 goldbachs_theorem_s14) (.seq (.arrow goldbachs_theorem_l14 goldbachs_theorem_s14 goldbachs_theorem_s15) (.seq (.arrow goldbachs_theorem_l15 goldbachs_theorem_s15 goldbachs_theorem_s16) (.seq (.arrow goldbachs_theorem_l16 goldbachs_theorem_s16 goldbachs_theorem_s17) (.seq (.arrow goldbachs_theorem_l17 goldbachs_theorem_s17 goldbachs_theorem_s18) (.seq (.arrow goldbachs_theorem_l18 goldbachs_theorem_s18 goldbachs_theorem_s19) (.seq (.arrow goldbachs_theorem_l19 goldbachs_theorem_s19 goldbachs_theorem_s20) (.seq (.arrow goldbachs_theorem_l20 goldbachs_theorem_s20 goldbachs_theorem_s21) (.seq (.arrow goldbachs_theorem_l21 goldbachs_theorem_s21 goldbachs_theorem_s22) (.seq (.arrow goldbachs_theorem_l22 goldbachs_theorem_s22 goldbachs_theorem_s23) (.seq (.arrow goldbachs_theorem_l23 goldbachs_theorem_s23 goldbachs_theorem_s24) (.seq (.arrow goldbachs_theorem_l24 goldbachs_theorem_s24 goldbachs_theorem_s25) (.seq (.arrow goldbachs_theorem_l25 goldbachs_theorem_s25 goldbachs_theorem_s26) (.seq (.arrow goldbachs_theorem_l26 goldbachs_theorem_s26 goldbachs_theorem_s27) (.seq (.arrow goldbachs_theorem_l27 goldbachs_theorem_s27 goldbachs_theorem_s28) (.seq (.arrow goldbachs_theorem_l28 goldbachs_theorem_s28 goldbachs_theorem_s29) (.seq (.arrow goldbachs_theorem_l29 goldbachs_theorem_s29 goldbachs_theorem_s30) (.seq (.arrow goldbachs_theorem_l30 goldbachs_theorem_s30 goldbachs_theorem_s31) (.seq (.arrow goldbachs_theorem_l31 goldbachs_theorem_s31 goldbachs_theorem_s32) (.seq (.arrow goldbachs_theorem_l32 goldbachs_theorem_s32 goldbachs_theorem_s33) (.seq (.arrow goldbachs_theorem_l33 goldbachs_theorem_s33 goldbachs_theorem_s34) (.seq (.arrow goldbachs_theorem_l34 goldbachs_theorem_s34 goldbachs_theorem_s35) (.seq (.arrow goldbachs_theorem_l35 goldbachs_theorem_s35 goldbachs_theorem_s36) (.seq (.arrow goldbachs_theorem_l36 goldbachs_theorem_s36 goldbachs_theorem_s37) (.arrow goldbachs_theorem_l37 goldbachs_theorem_s37 goldbachs_theorem_s38)))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def goldbachs_theorem_true_arm : IGProtocol goldbachs_theorem_s0 goldbachs_theorem_s38 :=
  (goldbachs_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def goldbachs_theorem_false_arm : IGProtocol goldbachs_theorem_s0 goldbachs_theorem_s38 :=
  (goldbachs_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def goldbachs_theorem_tier : OuroboricityTier := TierFunctor.obj goldbachs_theorem_s0
#eval goldbachs_theorem_tier  -- the Grammar's own verdict on its tier
