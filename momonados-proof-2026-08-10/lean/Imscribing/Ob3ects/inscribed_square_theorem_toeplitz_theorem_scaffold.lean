-- IGProtocol scaffold: VINIT → TANCH → AFWD → CLINK → IFIX → AFWD → CLINK → IFIX → AFWD → CLINK → IFIX → AFWD → CLINK → IFIX → FSPLIT → EVALT → IMSCRIB → IFIX → FFUSE → EVALF → AREV → AFWD → CLINK → IFIX → FSPLIT → EVALT → IMSCRIB → IFIX → FFUSE → EVALF → AREV → AFWD → CLINK → IFIX → AFWD → CLINK → IFIX → FSPLIT → EVALT → IMSCRIB → IFIX → FFUSE → ENGAGR → IMSCRIB → IFIX → FFUSE → TANCH
-- Class: Inscribed Square Theorem (Toeplitz' Theorem)
-- Fingerprint: sig=(23,7,6,11)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=47
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(14, 18), (24, 28), (37, 41)]

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
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [4] IFIX      prot   := 𐑭               𐑱 → 𐑾  | irreversible fixation — winding number
--   [5] AFWD      rel    := 𐑾               𐑭 → 𐑱  | forward morphism — bidirectional arrow
--   [6] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [7] IFIX      prot   := 𐑭               𐑱 → 𐑾  | irreversible fixation — winding number
--   [8] AFWD      rel    := 𐑾               𐑭 → 𐑱  | forward morphism — bidirectional arrow
--   [9] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [10] IFIX      prot   := 𐑭               𐑱 → 𐑾  | irreversible fixation — winding number
--   [11] AFWD      rel    := 𐑾               𐑭 → 𐑱  | forward morphism — bidirectional arrow
--   [12] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [13] IFIX      prot   := 𐑭               𐑱 → 𐑚  | irreversible fixation — winding number
--   [14] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [15] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [16] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [17] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [18] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [19] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [20] AREV      pol    := 𐑗               𐑖 → 𐑾  | reverse morphism — parity flip
--   [21] AFWD      rel    := 𐑾               𐑗 → 𐑱  | forward morphism — bidirectional arrow
--   [22] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [23] IFIX      prot   := 𐑭               𐑱 → 𐑚  | irreversible fixation — winding number
--   [24] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [25] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [26] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [27] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [28] FFUSE     stoi   := 𐑙               𐑙 → 𐑖  | fuse μ — assembly mode
--   [29] EVALF     chir   := 𐑖               𐑙 → 𐑗  | evaluate-false — chirality check
--   [30] AREV      pol    := 𐑗               𐑖 → 𐑾  | reverse morphism — parity flip
--   [31] AFWD      rel    := 𐑾               𐑗 → 𐑱  | forward morphism — bidirectional arrow
--   [32] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [33] IFIX      prot   := 𐑭               𐑱 → 𐑾  | irreversible fixation — winding number
--   [34] AFWD      rel    := 𐑾               𐑭 → 𐑱  | forward morphism — bidirectional arrow
--   [35] CLINK     fid    := 𐑱               𐑾 → 𐑭  | composition — regime coherence
--   [36] IFIX      prot   := 𐑭               𐑱 → 𐑚  | irreversible fixation — winding number
--   [37] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [38] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [39] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [40] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [41] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [42] ENGAGR    stoi   := 𐑳               𐑙 → 𐑠  | engage paradox — B-state, both arms
--   [43] IMSCRIB   gram   := 𐑠               𐑳 → 𐑭  | identity — self-imscription
--   [44] IFIX      prot   := 𐑭               𐑠 → 𐑙  | irreversible fixation — winding number
--   [45] FFUSE     stoi   := 𐑙               𐑭 → 𐑡  | fuse μ — assembly mode
--   [46] TANCH     top    := 𐑡               𐑙 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def inscribed_square_theorem_toeplitz_9f051b_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s39 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s40 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s41 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s42 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s43 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s44 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s45 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_s46 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def inscribed_square_theorem_toeplitz_9f051b_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l21 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l31 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l33 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l34 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l37 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l39 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l40 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l41 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l42 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l43 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l44 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def inscribed_square_theorem_toeplitz_9f051b_l45 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def inscribed_square_theorem_toeplitz_9f051b_l46 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def inscribed_square_theorem_toeplitz_9f051b_protocol : IGProtocol inscribed_square_theorem_toeplitz_9f051b_s0 inscribed_square_theorem_toeplitz_9f051b_s46 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct inscribed_square_theorem_toeplitz_9f051b_s18 inscribed_square_theorem_toeplitz_9f051b_s18 = inscribed_square_theorem_toeplitz_9f051b_s18 (idempotent)
  (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l0 inscribed_square_theorem_toeplitz_9f051b_s0 inscribed_square_theorem_toeplitz_9f051b_s1) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l1 inscribed_square_theorem_toeplitz_9f051b_s1 inscribed_square_theorem_toeplitz_9f051b_s2) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l2 inscribed_square_theorem_toeplitz_9f051b_s2 inscribed_square_theorem_toeplitz_9f051b_s3) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l3 inscribed_square_theorem_toeplitz_9f051b_s3 inscribed_square_theorem_toeplitz_9f051b_s4) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l4 inscribed_square_theorem_toeplitz_9f051b_s4 inscribed_square_theorem_toeplitz_9f051b_s5) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l5 inscribed_square_theorem_toeplitz_9f051b_s5 inscribed_square_theorem_toeplitz_9f051b_s6) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l6 inscribed_square_theorem_toeplitz_9f051b_s6 inscribed_square_theorem_toeplitz_9f051b_s7) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l7 inscribed_square_theorem_toeplitz_9f051b_s7 inscribed_square_theorem_toeplitz_9f051b_s8) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l8 inscribed_square_theorem_toeplitz_9f051b_s8 inscribed_square_theorem_toeplitz_9f051b_s9) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l9 inscribed_square_theorem_toeplitz_9f051b_s9 inscribed_square_theorem_toeplitz_9f051b_s10) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l10 inscribed_square_theorem_toeplitz_9f051b_s10 inscribed_square_theorem_toeplitz_9f051b_s11) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l11 inscribed_square_theorem_toeplitz_9f051b_s11 inscribed_square_theorem_toeplitz_9f051b_s12) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l12 inscribed_square_theorem_toeplitz_9f051b_s12 inscribed_square_theorem_toeplitz_9f051b_s13) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l13 inscribed_square_theorem_toeplitz_9f051b_s13 inscribed_square_theorem_toeplitz_9f051b_s14) (.seq (.prod (.arrow inscribed_square_theorem_toeplitz_9f051b_l14 inscribed_square_theorem_toeplitz_9f051b_s14 inscribed_square_theorem_toeplitz_9f051b_s18) (.arrow inscribed_square_theorem_toeplitz_9f051b_l14 inscribed_square_theorem_toeplitz_9f051b_s14 inscribed_square_theorem_toeplitz_9f051b_s18)) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l18 inscribed_square_theorem_toeplitz_9f051b_s18 inscribed_square_theorem_toeplitz_9f051b_s18) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l18 inscribed_square_theorem_toeplitz_9f051b_s18 inscribed_square_theorem_toeplitz_9f051b_s19) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l19 inscribed_square_theorem_toeplitz_9f051b_s19 inscribed_square_theorem_toeplitz_9f051b_s20) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l20 inscribed_square_theorem_toeplitz_9f051b_s20 inscribed_square_theorem_toeplitz_9f051b_s21) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l21 inscribed_square_theorem_toeplitz_9f051b_s21 inscribed_square_theorem_toeplitz_9f051b_s22) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l22 inscribed_square_theorem_toeplitz_9f051b_s22 inscribed_square_theorem_toeplitz_9f051b_s23) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l23 inscribed_square_theorem_toeplitz_9f051b_s23 inscribed_square_theorem_toeplitz_9f051b_s24) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l24 inscribed_square_theorem_toeplitz_9f051b_s24 inscribed_square_theorem_toeplitz_9f051b_s25) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l25 inscribed_square_theorem_toeplitz_9f051b_s25 inscribed_square_theorem_toeplitz_9f051b_s26) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l26 inscribed_square_theorem_toeplitz_9f051b_s26 inscribed_square_theorem_toeplitz_9f051b_s27) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l27 inscribed_square_theorem_toeplitz_9f051b_s27 inscribed_square_theorem_toeplitz_9f051b_s28) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l28 inscribed_square_theorem_toeplitz_9f051b_s28 inscribed_square_theorem_toeplitz_9f051b_s29) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l29 inscribed_square_theorem_toeplitz_9f051b_s29 inscribed_square_theorem_toeplitz_9f051b_s30) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l30 inscribed_square_theorem_toeplitz_9f051b_s30 inscribed_square_theorem_toeplitz_9f051b_s31) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l31 inscribed_square_theorem_toeplitz_9f051b_s31 inscribed_square_theorem_toeplitz_9f051b_s32) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l32 inscribed_square_theorem_toeplitz_9f051b_s32 inscribed_square_theorem_toeplitz_9f051b_s33) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l33 inscribed_square_theorem_toeplitz_9f051b_s33 inscribed_square_theorem_toeplitz_9f051b_s34) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l34 inscribed_square_theorem_toeplitz_9f051b_s34 inscribed_square_theorem_toeplitz_9f051b_s35) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l35 inscribed_square_theorem_toeplitz_9f051b_s35 inscribed_square_theorem_toeplitz_9f051b_s36) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l36 inscribed_square_theorem_toeplitz_9f051b_s36 inscribed_square_theorem_toeplitz_9f051b_s37) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l37 inscribed_square_theorem_toeplitz_9f051b_s37 inscribed_square_theorem_toeplitz_9f051b_s38) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l38 inscribed_square_theorem_toeplitz_9f051b_s38 inscribed_square_theorem_toeplitz_9f051b_s39) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l39 inscribed_square_theorem_toeplitz_9f051b_s39 inscribed_square_theorem_toeplitz_9f051b_s40) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l40 inscribed_square_theorem_toeplitz_9f051b_s40 inscribed_square_theorem_toeplitz_9f051b_s41) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l41 inscribed_square_theorem_toeplitz_9f051b_s41 inscribed_square_theorem_toeplitz_9f051b_s42) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l42 inscribed_square_theorem_toeplitz_9f051b_s42 inscribed_square_theorem_toeplitz_9f051b_s43) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l43 inscribed_square_theorem_toeplitz_9f051b_s43 inscribed_square_theorem_toeplitz_9f051b_s44) (.seq (.arrow inscribed_square_theorem_toeplitz_9f051b_l44 inscribed_square_theorem_toeplitz_9f051b_s44 inscribed_square_theorem_toeplitz_9f051b_s45) (.arrow inscribed_square_theorem_toeplitz_9f051b_l45 inscribed_square_theorem_toeplitz_9f051b_s45 inscribed_square_theorem_toeplitz_9f051b_s46))))))))))))))))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def inscribed_square_theorem_toeplitz_9f051b_true_arm : IGProtocol inscribed_square_theorem_toeplitz_9f051b_s0 inscribed_square_theorem_toeplitz_9f051b_s46 :=
  (inscribed_square_theorem_toeplitz_9f051b_protocol).restrictToEVALT
  --
-- false arm
noncomputable def inscribed_square_theorem_toeplitz_9f051b_false_arm : IGProtocol inscribed_square_theorem_toeplitz_9f051b_s0 inscribed_square_theorem_toeplitz_9f051b_s46 :=
  (inscribed_square_theorem_toeplitz_9f051b_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def inscribed_square_theorem_toeplitz_9f051b_tier : OuroboricityTier := TierFunctor.obj inscribed_square_theorem_toeplitz_9f051b_s0
#eval inscribed_square_theorem_toeplitz_9f051b_tier  -- the Grammar's own verdict on its tier
