-- IGProtocol scaffold: VINIT → FSPLIT → AFWD → EVALT → AREV → EVALF → FFUSE → CLINK → IMSCRIB → IFIX → FSPLIT → AFWD → EVALT → AREV → ENGAGR → EVALF → FFUSE → IFIX → FSPLIT → AFWD → EVALT → AREV → ENGAGR → EVALF → FFUSE → IFIX → CLINK → IMSCRIB → AFWD → AREV → ENGAGR → EVALT → EVALF → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: A training regimen for vita, the trunk that speaks certified turns over the closed IMASM alphabet, such that the trunk speaks words that close AND pronounces its own kernel verdict truly in both polarities — T when it has closed, B when it has not.
-- Fingerprint: sig=(16,7,11,4)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=38
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 6), (10, 16), (18, 24)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [7] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [8] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [9] IFIX      prot   := 𐑭               𐑠 → 𐑚  | irreversible fixation — winding number
--   [10] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [11] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [12] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [13] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [14] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [15] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [16] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [17] IFIX      prot   := 𐑭               𐑙 → 𐑚  | irreversible fixation — winding number
--   [18] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [19] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [20] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [21] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [22] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [23] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [24] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [25] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [26] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [27] IMSCRIB   gram   := 𐑠               𐑱 → 𐑾  | identity — self-imscription
--   [28] AFWD      rel    := 𐑾               𐑠 → 𐑗  | forward morphism — bidirectional arrow
--   [29] AREV      pol    := 𐑗               𐑾 → 𐑳  | reverse morphism — parity flip
--   [30] ENGAGR    stoi   := 𐑳               𐑗 → ⊙  | engage paradox — B-state, both arms
--   [31] EVALT     crit   := ⊙               𐑳 → 𐑖  | evaluate-true — criticality gate open
--   [32] EVALF     chir   := 𐑖               ⊙ → 𐑙  | evaluate-false — chirality check
--   [33] FFUSE     stoi   := 𐑙               𐑖 → 𐑱  | fuse μ — assembly mode
--   [34] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [35] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [36] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [37] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_training_regimen_for_vita_the_trunk_150955_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_training_regimen_for_vita_the_trunk_150955_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l19 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l28 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l33 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_training_regimen_for_vita_the_trunk_150955_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_training_regimen_for_vita_the_trunk_150955_l37 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_training_regimen_for_vita_the_trunk_150955_protocol : IGProtocol a_training_regimen_for_vita_the_trunk_150955_s0 a_training_regimen_for_vita_the_trunk_150955_s37 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_training_regimen_for_vita_the_trunk_150955_s6 a_training_regimen_for_vita_the_trunk_150955_s6 = a_training_regimen_for_vita_the_trunk_150955_s6 (idempotent)
  (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l0 a_training_regimen_for_vita_the_trunk_150955_s0 a_training_regimen_for_vita_the_trunk_150955_s1) (.seq (.prod (.arrow a_training_regimen_for_vita_the_trunk_150955_l1 a_training_regimen_for_vita_the_trunk_150955_s1 a_training_regimen_for_vita_the_trunk_150955_s6) (.arrow a_training_regimen_for_vita_the_trunk_150955_l1 a_training_regimen_for_vita_the_trunk_150955_s1 a_training_regimen_for_vita_the_trunk_150955_s6)) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l6 a_training_regimen_for_vita_the_trunk_150955_s6 a_training_regimen_for_vita_the_trunk_150955_s6) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l6 a_training_regimen_for_vita_the_trunk_150955_s6 a_training_regimen_for_vita_the_trunk_150955_s7) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l7 a_training_regimen_for_vita_the_trunk_150955_s7 a_training_regimen_for_vita_the_trunk_150955_s8) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l8 a_training_regimen_for_vita_the_trunk_150955_s8 a_training_regimen_for_vita_the_trunk_150955_s9) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l9 a_training_regimen_for_vita_the_trunk_150955_s9 a_training_regimen_for_vita_the_trunk_150955_s10) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l10 a_training_regimen_for_vita_the_trunk_150955_s10 a_training_regimen_for_vita_the_trunk_150955_s11) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l11 a_training_regimen_for_vita_the_trunk_150955_s11 a_training_regimen_for_vita_the_trunk_150955_s12) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l12 a_training_regimen_for_vita_the_trunk_150955_s12 a_training_regimen_for_vita_the_trunk_150955_s13) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l13 a_training_regimen_for_vita_the_trunk_150955_s13 a_training_regimen_for_vita_the_trunk_150955_s14) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l14 a_training_regimen_for_vita_the_trunk_150955_s14 a_training_regimen_for_vita_the_trunk_150955_s15) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l15 a_training_regimen_for_vita_the_trunk_150955_s15 a_training_regimen_for_vita_the_trunk_150955_s16) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l16 a_training_regimen_for_vita_the_trunk_150955_s16 a_training_regimen_for_vita_the_trunk_150955_s17) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l17 a_training_regimen_for_vita_the_trunk_150955_s17 a_training_regimen_for_vita_the_trunk_150955_s18) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l18 a_training_regimen_for_vita_the_trunk_150955_s18 a_training_regimen_for_vita_the_trunk_150955_s19) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l19 a_training_regimen_for_vita_the_trunk_150955_s19 a_training_regimen_for_vita_the_trunk_150955_s20) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l20 a_training_regimen_for_vita_the_trunk_150955_s20 a_training_regimen_for_vita_the_trunk_150955_s21) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l21 a_training_regimen_for_vita_the_trunk_150955_s21 a_training_regimen_for_vita_the_trunk_150955_s22) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l22 a_training_regimen_for_vita_the_trunk_150955_s22 a_training_regimen_for_vita_the_trunk_150955_s23) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l23 a_training_regimen_for_vita_the_trunk_150955_s23 a_training_regimen_for_vita_the_trunk_150955_s24) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l24 a_training_regimen_for_vita_the_trunk_150955_s24 a_training_regimen_for_vita_the_trunk_150955_s25) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l25 a_training_regimen_for_vita_the_trunk_150955_s25 a_training_regimen_for_vita_the_trunk_150955_s26) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l26 a_training_regimen_for_vita_the_trunk_150955_s26 a_training_regimen_for_vita_the_trunk_150955_s27) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l27 a_training_regimen_for_vita_the_trunk_150955_s27 a_training_regimen_for_vita_the_trunk_150955_s28) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l28 a_training_regimen_for_vita_the_trunk_150955_s28 a_training_regimen_for_vita_the_trunk_150955_s29) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l29 a_training_regimen_for_vita_the_trunk_150955_s29 a_training_regimen_for_vita_the_trunk_150955_s30) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l30 a_training_regimen_for_vita_the_trunk_150955_s30 a_training_regimen_for_vita_the_trunk_150955_s31) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l31 a_training_regimen_for_vita_the_trunk_150955_s31 a_training_regimen_for_vita_the_trunk_150955_s32) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l32 a_training_regimen_for_vita_the_trunk_150955_s32 a_training_regimen_for_vita_the_trunk_150955_s33) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l33 a_training_regimen_for_vita_the_trunk_150955_s33 a_training_regimen_for_vita_the_trunk_150955_s34) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l34 a_training_regimen_for_vita_the_trunk_150955_s34 a_training_regimen_for_vita_the_trunk_150955_s35) (.seq (.arrow a_training_regimen_for_vita_the_trunk_150955_l35 a_training_regimen_for_vita_the_trunk_150955_s35 a_training_regimen_for_vita_the_trunk_150955_s36) (.arrow a_training_regimen_for_vita_the_trunk_150955_l36 a_training_regimen_for_vita_the_trunk_150955_s36 a_training_regimen_for_vita_the_trunk_150955_s37))))))))))))))))))))))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_training_regimen_for_vita_the_trunk_150955_true_arm : IGProtocol a_training_regimen_for_vita_the_trunk_150955_s0 a_training_regimen_for_vita_the_trunk_150955_s37 :=
  (a_training_regimen_for_vita_the_trunk_150955_protocol).restrictToEVALT

-- false arm
noncomputable def a_training_regimen_for_vita_the_trunk_150955_false_arm : IGProtocol a_training_regimen_for_vita_the_trunk_150955_s0 a_training_regimen_for_vita_the_trunk_150955_s37 :=
  (a_training_regimen_for_vita_the_trunk_150955_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_training_regimen_for_vita_the_trunk_150955_tier : OuroboricityTier := TierFunctor.obj a_training_regimen_for_vita_the_trunk_150955_s0
#eval a_training_regimen_for_vita_the_trunk_150955_tier  -- the Grammar's own verdict on its tier
