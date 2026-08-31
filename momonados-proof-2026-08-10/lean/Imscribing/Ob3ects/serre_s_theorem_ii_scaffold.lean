-- IGProtocol scaffold: VINIT → TANCH → AFWD → CLINK → IMSCRIB → FSPLIT → EVALT → EVALF → FFUSE → AREV → ENGAGR → IFIX → CLINK → IMSCRIB → AFWD → AREV → FSPLIT → EVALT → EVALF → FFUSE → IFIX → CLINK → IMSCRIB → AFWD → EVALT → EVALF → IFIX → CLINK → IMSCRIB → FSPLIT → EVALT → EVALF → FFUSE → IFIX → ENGAGR → AREV → CLINK → IMSCRIB → AFWD → IFIX → CLINK → IMSCRIB → TANCH
-- Class: Serre's Theorem II
-- Fingerprint: sig=(22,6,10,5)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=43
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 8), (16, 19), (29, 32)]

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
--   [2] AFWD      rel    := 𐑾               𐑡 → 𐑱  | forward morphism — bidirectional arrow
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑠  | composition — regime coherence
--   [4] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [9] AREV      pol    := 𐑗               𐑙 → 𐑳  | reverse morphism — parity flip
--   [10] ENGAGR    stoi   := 𐑳               𐑗 → 𐑭  | engage paradox — B-state, both arms
--   [11] IFIX      prot   := 𐑭               𐑳 → 𐑱  | irreversible fixation — winding number
--   [12] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [13] IMSCRIB   gram   := 𐑠               𐑱 → 𐑾  | identity — self-imscription
--   [14] AFWD      rel    := 𐑾               𐑠 → 𐑗  | forward morphism — bidirectional arrow
--   [15] AREV      pol    := 𐑗               𐑾 → 𐑚  | reverse morphism — parity flip
--   [16] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [17] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [18] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [19] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [20] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [21] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [22] IMSCRIB   gram   := 𐑠               𐑱 → 𐑾  | identity — self-imscription
--   [23] AFWD      rel    := 𐑾               𐑠 → ⊙  | forward morphism — bidirectional arrow
--   [24] EVALT     crit   := ⊙               𐑾 → 𐑖  | evaluate-true — criticality gate open
--   [25] EVALF     chir   := 𐑖               ⊙ → 𐑭  | evaluate-false — chirality check
--   [26] IFIX      prot   := 𐑭               𐑖 → 𐑱  | irreversible fixation — winding number
--   [27] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [28] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [29] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [30] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [31] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [32] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [33] IFIX      prot   := 𐑭               𐑙 → 𐑳  | irreversible fixation — winding number
--   [34] ENGAGR    stoi   := 𐑳               𐑭 → 𐑗  | engage paradox — B-state, both arms
--   [35] AREV      pol    := 𐑗               𐑳 → 𐑱  | reverse morphism — parity flip
--   [36] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [37] IMSCRIB   gram   := 𐑠               𐑱 → 𐑾  | identity — self-imscription
--   [38] AFWD      rel    := 𐑾               𐑠 → 𐑭  | forward morphism — bidirectional arrow
--   [39] IFIX      prot   := 𐑭               𐑾 → 𐑱  | irreversible fixation — winding number
--   [40] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [41] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [42] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def serres_theorem_ii_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def serres_theorem_ii_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def serres_theorem_ii_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s39 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s40 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s41 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def serres_theorem_ii_s42 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def serres_theorem_ii_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def serres_theorem_ii_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def serres_theorem_ii_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l14 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def serres_theorem_ii_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l23 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def serres_theorem_ii_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def serres_theorem_ii_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l33 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def serres_theorem_ii_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def serres_theorem_ii_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l37 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l38 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l39 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def serres_theorem_ii_l40 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l41 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def serres_theorem_ii_l42 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def serres_theorem_ii_protocol : IGProtocol serres_theorem_ii_s0 serres_theorem_ii_s42 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct serres_theorem_ii_s8 serres_theorem_ii_s8 = serres_theorem_ii_s8 (idempotent)
  (.seq (.arrow serres_theorem_ii_l0 serres_theorem_ii_s0 serres_theorem_ii_s1) (.seq (.arrow serres_theorem_ii_l1 serres_theorem_ii_s1 serres_theorem_ii_s2) (.seq (.arrow serres_theorem_ii_l2 serres_theorem_ii_s2 serres_theorem_ii_s3) (.seq (.arrow serres_theorem_ii_l3 serres_theorem_ii_s3 serres_theorem_ii_s4) (.seq (.arrow serres_theorem_ii_l4 serres_theorem_ii_s4 serres_theorem_ii_s5) (.seq (.prod (.arrow serres_theorem_ii_l5 serres_theorem_ii_s5 serres_theorem_ii_s8) (.arrow serres_theorem_ii_l5 serres_theorem_ii_s5 serres_theorem_ii_s8)) (.seq (.arrow serres_theorem_ii_l8 serres_theorem_ii_s8 serres_theorem_ii_s8) (.seq (.arrow serres_theorem_ii_l8 serres_theorem_ii_s8 serres_theorem_ii_s9) (.seq (.arrow serres_theorem_ii_l9 serres_theorem_ii_s9 serres_theorem_ii_s10) (.seq (.arrow serres_theorem_ii_l10 serres_theorem_ii_s10 serres_theorem_ii_s11) (.seq (.arrow serres_theorem_ii_l11 serres_theorem_ii_s11 serres_theorem_ii_s12) (.seq (.arrow serres_theorem_ii_l12 serres_theorem_ii_s12 serres_theorem_ii_s13) (.seq (.arrow serres_theorem_ii_l13 serres_theorem_ii_s13 serres_theorem_ii_s14) (.seq (.arrow serres_theorem_ii_l14 serres_theorem_ii_s14 serres_theorem_ii_s15) (.seq (.arrow serres_theorem_ii_l15 serres_theorem_ii_s15 serres_theorem_ii_s16) (.seq (.arrow serres_theorem_ii_l16 serres_theorem_ii_s16 serres_theorem_ii_s17) (.seq (.arrow serres_theorem_ii_l17 serres_theorem_ii_s17 serres_theorem_ii_s18) (.seq (.arrow serres_theorem_ii_l18 serres_theorem_ii_s18 serres_theorem_ii_s19) (.seq (.arrow serres_theorem_ii_l19 serres_theorem_ii_s19 serres_theorem_ii_s20) (.seq (.arrow serres_theorem_ii_l20 serres_theorem_ii_s20 serres_theorem_ii_s21) (.seq (.arrow serres_theorem_ii_l21 serres_theorem_ii_s21 serres_theorem_ii_s22) (.seq (.arrow serres_theorem_ii_l22 serres_theorem_ii_s22 serres_theorem_ii_s23) (.seq (.arrow serres_theorem_ii_l23 serres_theorem_ii_s23 serres_theorem_ii_s24) (.seq (.arrow serres_theorem_ii_l24 serres_theorem_ii_s24 serres_theorem_ii_s25) (.seq (.arrow serres_theorem_ii_l25 serres_theorem_ii_s25 serres_theorem_ii_s26) (.seq (.arrow serres_theorem_ii_l26 serres_theorem_ii_s26 serres_theorem_ii_s27) (.seq (.arrow serres_theorem_ii_l27 serres_theorem_ii_s27 serres_theorem_ii_s28) (.seq (.arrow serres_theorem_ii_l28 serres_theorem_ii_s28 serres_theorem_ii_s29) (.seq (.arrow serres_theorem_ii_l29 serres_theorem_ii_s29 serres_theorem_ii_s30) (.seq (.arrow serres_theorem_ii_l30 serres_theorem_ii_s30 serres_theorem_ii_s31) (.seq (.arrow serres_theorem_ii_l31 serres_theorem_ii_s31 serres_theorem_ii_s32) (.seq (.arrow serres_theorem_ii_l32 serres_theorem_ii_s32 serres_theorem_ii_s33) (.seq (.arrow serres_theorem_ii_l33 serres_theorem_ii_s33 serres_theorem_ii_s34) (.seq (.arrow serres_theorem_ii_l34 serres_theorem_ii_s34 serres_theorem_ii_s35) (.seq (.arrow serres_theorem_ii_l35 serres_theorem_ii_s35 serres_theorem_ii_s36) (.seq (.arrow serres_theorem_ii_l36 serres_theorem_ii_s36 serres_theorem_ii_s37) (.seq (.arrow serres_theorem_ii_l37 serres_theorem_ii_s37 serres_theorem_ii_s38) (.seq (.arrow serres_theorem_ii_l38 serres_theorem_ii_s38 serres_theorem_ii_s39) (.seq (.arrow serres_theorem_ii_l39 serres_theorem_ii_s39 serres_theorem_ii_s40) (.seq (.arrow serres_theorem_ii_l40 serres_theorem_ii_s40 serres_theorem_ii_s41) (.arrow serres_theorem_ii_l41 serres_theorem_ii_s41 serres_theorem_ii_s42)))))))))))))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def serres_theorem_ii_true_arm : IGProtocol serres_theorem_ii_s0 serres_theorem_ii_s42 :=
  (serres_theorem_ii_protocol).restrictToEVALT
  --
-- false arm
noncomputable def serres_theorem_ii_false_arm : IGProtocol serres_theorem_ii_s0 serres_theorem_ii_s42 :=
  (serres_theorem_ii_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def serres_theorem_ii_tier : OuroboricityTier := TierFunctor.obj serres_theorem_ii_s0
#eval serres_theorem_ii_tier  -- the Grammar's own verdict on its tier
