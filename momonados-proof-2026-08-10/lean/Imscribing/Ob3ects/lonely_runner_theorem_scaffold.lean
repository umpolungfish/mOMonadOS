-- IGProtocol scaffold: VINIT → TANCH → AFWD → AFWD → AFWD → CLINK → CLINK → FSPLIT → EVALT → EVALF → AREV → CLINK → ENGAGR → AFWD → CLINK → FSPLIT → EVALT → EVALF → IFIX → IMSCRIB → AREV → CLINK → FSPLIT → EVALT → EVALF → AFWD → CLINK → IFIX → IMSCRIB → CLINK → FSPLIT → EVALT → EVALF → AFWD → CLINK → ENGAGR → AREV → CLINK → IFIX → FFUSE → IMSCRIB → TANCH
-- Class: Lonely Runner Theorem
-- Fingerprint: sig=(24,5,10,3)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=42
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(30, 39)]

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
--   [2] AFWD      rel    := 𐑾               𐑡 → 𐑾  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [4] AFWD      rel    := 𐑾               𐑾 → 𐑱  | forward morphism — bidirectional arrow
--   [5] CLINK     fid    := 𐑱               𐑾 → 𐑱  | composition — regime coherence
--   [6] CLINK     fid    := 𐑱               𐑱 → 𐑚  | composition — regime coherence
--   [7] FSPLIT    gran   := 𐑚               𐑱 → ⊙  | split δ — range decomposition
--   [8] EVALT     crit   := ⊙               𐑚 → 𐑖  | evaluate-true — criticality gate open
--   [9] EVALF     chir   := 𐑖               ⊙ → 𐑗  | evaluate-false — chirality check
--   [10] AREV      pol    := 𐑗               𐑖 → 𐑱  | reverse morphism — parity flip
--   [11] CLINK     fid    := 𐑱               𐑗 → 𐑳  | composition — regime coherence
--   [12] ENGAGR    stoi   := 𐑳               𐑱 → 𐑾  | engage paradox — B-state, both arms
--   [13] AFWD      rel    := 𐑾               𐑳 → 𐑱  | forward morphism — bidirectional arrow
--   [14] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [15] FSPLIT    gran   := 𐑚               𐑱 → ⊙  | split δ — range decomposition
--   [16] EVALT     crit   := ⊙               𐑚 → 𐑖  | evaluate-true — criticality gate open
--   [17] EVALF     chir   := 𐑖               ⊙ → 𐑭  | evaluate-false — chirality check
--   [18] IFIX      prot   := 𐑭               𐑖 → 𐑠  | irreversible fixation — winding number
--   [19] IMSCRIB   gram   := 𐑠               𐑭 → 𐑗  | identity — self-imscription
--   [20] AREV      pol    := 𐑗               𐑠 → 𐑱  | reverse morphism — parity flip
--   [21] CLINK     fid    := 𐑱               𐑗 → 𐑚  | composition — regime coherence
--   [22] FSPLIT    gran   := 𐑚               𐑱 → ⊙  | split δ — range decomposition
--   [23] EVALT     crit   := ⊙               𐑚 → 𐑖  | evaluate-true — criticality gate open
--   [24] EVALF     chir   := 𐑖               ⊙ → 𐑾  | evaluate-false — chirality check
--   [25] AFWD      rel    := 𐑾               𐑖 → 𐑱  | forward morphism — bidirectional arrow
--   [26] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [27] IFIX      prot   := 𐑭               𐑱 → 𐑠  | irreversible fixation — winding number
--   [28] IMSCRIB   gram   := 𐑠               𐑭 → 𐑱  | identity — self-imscription
--   [29] CLINK     fid    := 𐑱               𐑠 → 𐑚  | composition — regime coherence
--   [30] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [31] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [32] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [33] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [34] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [35] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [36] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [37] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [38] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [39] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [40] IMSCRIB   gram   := 𐑠               𐑙 → 𐑡  | identity — self-imscription
--   [41] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def lonely_runner_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def lonely_runner_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def lonely_runner_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def lonely_runner_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def lonely_runner_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def lonely_runner_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def lonely_runner_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def lonely_runner_theorem_s39 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lonely_runner_theorem_s40 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def lonely_runner_theorem_s41 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def lonely_runner_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def lonely_runner_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lonely_runner_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_l25 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lonely_runner_theorem_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def lonely_runner_theorem_l33 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def lonely_runner_theorem_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l37 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def lonely_runner_theorem_l39 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l40 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def lonely_runner_theorem_l41 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def lonely_runner_theorem_protocol : IGProtocol lonely_runner_theorem_s0 lonely_runner_theorem_s41 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct lonely_runner_theorem_s39 lonely_runner_theorem_s39 = lonely_runner_theorem_s39 (idempotent)
  (.seq (.arrow lonely_runner_theorem_l0 lonely_runner_theorem_s0 lonely_runner_theorem_s1) (.seq (.arrow lonely_runner_theorem_l1 lonely_runner_theorem_s1 lonely_runner_theorem_s2) (.seq (.arrow lonely_runner_theorem_l2 lonely_runner_theorem_s2 lonely_runner_theorem_s3) (.seq (.arrow lonely_runner_theorem_l3 lonely_runner_theorem_s3 lonely_runner_theorem_s4) (.seq (.arrow lonely_runner_theorem_l4 lonely_runner_theorem_s4 lonely_runner_theorem_s5) (.seq (.arrow lonely_runner_theorem_l5 lonely_runner_theorem_s5 lonely_runner_theorem_s6) (.seq (.arrow lonely_runner_theorem_l6 lonely_runner_theorem_s6 lonely_runner_theorem_s7) (.seq (.arrow lonely_runner_theorem_l7 lonely_runner_theorem_s7 lonely_runner_theorem_s8) (.seq (.arrow lonely_runner_theorem_l8 lonely_runner_theorem_s8 lonely_runner_theorem_s9) (.seq (.arrow lonely_runner_theorem_l9 lonely_runner_theorem_s9 lonely_runner_theorem_s10) (.seq (.arrow lonely_runner_theorem_l10 lonely_runner_theorem_s10 lonely_runner_theorem_s11) (.seq (.arrow lonely_runner_theorem_l11 lonely_runner_theorem_s11 lonely_runner_theorem_s12) (.seq (.arrow lonely_runner_theorem_l12 lonely_runner_theorem_s12 lonely_runner_theorem_s13) (.seq (.arrow lonely_runner_theorem_l13 lonely_runner_theorem_s13 lonely_runner_theorem_s14) (.seq (.arrow lonely_runner_theorem_l14 lonely_runner_theorem_s14 lonely_runner_theorem_s15) (.seq (.arrow lonely_runner_theorem_l15 lonely_runner_theorem_s15 lonely_runner_theorem_s16) (.seq (.arrow lonely_runner_theorem_l16 lonely_runner_theorem_s16 lonely_runner_theorem_s17) (.seq (.arrow lonely_runner_theorem_l17 lonely_runner_theorem_s17 lonely_runner_theorem_s18) (.seq (.arrow lonely_runner_theorem_l18 lonely_runner_theorem_s18 lonely_runner_theorem_s19) (.seq (.arrow lonely_runner_theorem_l19 lonely_runner_theorem_s19 lonely_runner_theorem_s20) (.seq (.arrow lonely_runner_theorem_l20 lonely_runner_theorem_s20 lonely_runner_theorem_s21) (.seq (.arrow lonely_runner_theorem_l21 lonely_runner_theorem_s21 lonely_runner_theorem_s22) (.seq (.arrow lonely_runner_theorem_l22 lonely_runner_theorem_s22 lonely_runner_theorem_s23) (.seq (.arrow lonely_runner_theorem_l23 lonely_runner_theorem_s23 lonely_runner_theorem_s24) (.seq (.arrow lonely_runner_theorem_l24 lonely_runner_theorem_s24 lonely_runner_theorem_s25) (.seq (.arrow lonely_runner_theorem_l25 lonely_runner_theorem_s25 lonely_runner_theorem_s26) (.seq (.arrow lonely_runner_theorem_l26 lonely_runner_theorem_s26 lonely_runner_theorem_s27) (.seq (.arrow lonely_runner_theorem_l27 lonely_runner_theorem_s27 lonely_runner_theorem_s28) (.seq (.arrow lonely_runner_theorem_l28 lonely_runner_theorem_s28 lonely_runner_theorem_s29) (.seq (.arrow lonely_runner_theorem_l29 lonely_runner_theorem_s29 lonely_runner_theorem_s30) (.seq (.prod (.arrow lonely_runner_theorem_l30 lonely_runner_theorem_s30 lonely_runner_theorem_s39) (.arrow lonely_runner_theorem_l30 lonely_runner_theorem_s30 lonely_runner_theorem_s39)) (.seq (.arrow lonely_runner_theorem_l39 lonely_runner_theorem_s39 lonely_runner_theorem_s39) (.seq (.arrow lonely_runner_theorem_l39 lonely_runner_theorem_s39 lonely_runner_theorem_s40) (.arrow lonely_runner_theorem_l40 lonely_runner_theorem_s40 lonely_runner_theorem_s41))))))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def lonely_runner_theorem_true_arm : IGProtocol lonely_runner_theorem_s0 lonely_runner_theorem_s41 :=
  (lonely_runner_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def lonely_runner_theorem_false_arm : IGProtocol lonely_runner_theorem_s0 lonely_runner_theorem_s41 :=
  (lonely_runner_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def lonely_runner_theorem_tier : OuroboricityTier := TierFunctor.obj lonely_runner_theorem_s0
#eval lonely_runner_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem lonely_runner_theorem_frobenius :
    igFrobeniusAlg.mul lonely_runner_theorem_s0 lonely_runner_theorem_s0 = lonely_runner_theorem_s0 :=
  igFrobAlg_self_fusion lonely_runner_theorem_s0
