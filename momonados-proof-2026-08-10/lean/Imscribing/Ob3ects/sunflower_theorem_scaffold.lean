-- IGProtocol scaffold: VINIT → TANCH → AFWD → CLINK → AFWD → CLINK → AFWD → CLINK → AFWD → CLINK → AFWD → CLINK → FSPLIT → EVALT → AFWD → CLINK → EVALF → AREV → CLINK → ENGAGR → IMSCRIB → FFUSE → EVALT → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → IFIX → CLINK → IMSCRIB → TANCH → IMSCRIB
-- Class: Sunflower Theorem
-- Fingerprint: sig=(21,2,4,12)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=39
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(12, 21)]

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
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑾  | composition — regime coherence
--   [4] AFWD      rel    := 𐑾               𐑱 → 𐑱  | forward morphism — bidirectional arrow
--   [5] CLINK     fid    := 𐑱               𐑾 → 𐑾  | composition — regime coherence
--   [6] AFWD      rel    := 𐑾               𐑱 → 𐑱  | forward morphism — bidirectional arrow
--   [7] CLINK     fid    := 𐑱               𐑾 → 𐑾  | composition — regime coherence
--   [8] AFWD      rel    := 𐑾               𐑱 → 𐑱  | forward morphism — bidirectional arrow
--   [9] CLINK     fid    := 𐑱               𐑾 → 𐑾  | composition — regime coherence
--   [10] AFWD      rel    := 𐑾               𐑱 → 𐑱  | forward morphism — bidirectional arrow
--   [11] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [12] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [13] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [14] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [15] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [16] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [17] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [18] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [19] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [20] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [21] FFUSE     stoi   := 𐑙               𐑙 → ⊙  | fuse μ — assembly mode
--   [22] EVALT     crit   := ⊙               𐑙 → 𐑭  | evaluate-true — criticality gate open
--   [23] IFIX      prot   := 𐑭               ⊙ → 𐑭  | irreversible fixation — winding number
--   [24] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [25] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [26] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [27] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [28] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [29] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [30] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [31] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [32] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [33] IFIX      prot   := 𐑭               𐑭 → 𐑭  | irreversible fixation — winding number
--   [34] IFIX      prot   := 𐑭               𐑭 → 𐑱  | irreversible fixation — winding number
--   [35] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [36] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [37] TANCH     top    := 𐑡               𐑠 → 𐑠  | terminal object — connectivity boundary
--   [38] IMSCRIB   gram   := 𐑠               𐑡 → 𐑼  | identity — self-imscription

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def sunflower_theorem_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sunflower_theorem_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sunflower_theorem_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sunflower_theorem_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def sunflower_theorem_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def sunflower_theorem_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sunflower_theorem_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sunflower_theorem_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sunflower_theorem_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def sunflower_theorem_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l14 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def sunflower_theorem_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def sunflower_theorem_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l33 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sunflower_theorem_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l37 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sunflower_theorem_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def sunflower_theorem_protocol : IGProtocol sunflower_theorem_s0 sunflower_theorem_s38 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct sunflower_theorem_s21 sunflower_theorem_s21 = sunflower_theorem_s21 (idempotent)
  (.seq (.arrow sunflower_theorem_l0 sunflower_theorem_s0 sunflower_theorem_s1) (.seq (.arrow sunflower_theorem_l1 sunflower_theorem_s1 sunflower_theorem_s2) (.seq (.arrow sunflower_theorem_l2 sunflower_theorem_s2 sunflower_theorem_s3) (.seq (.arrow sunflower_theorem_l3 sunflower_theorem_s3 sunflower_theorem_s4) (.seq (.arrow sunflower_theorem_l4 sunflower_theorem_s4 sunflower_theorem_s5) (.seq (.arrow sunflower_theorem_l5 sunflower_theorem_s5 sunflower_theorem_s6) (.seq (.arrow sunflower_theorem_l6 sunflower_theorem_s6 sunflower_theorem_s7) (.seq (.arrow sunflower_theorem_l7 sunflower_theorem_s7 sunflower_theorem_s8) (.seq (.arrow sunflower_theorem_l8 sunflower_theorem_s8 sunflower_theorem_s9) (.seq (.arrow sunflower_theorem_l9 sunflower_theorem_s9 sunflower_theorem_s10) (.seq (.arrow sunflower_theorem_l10 sunflower_theorem_s10 sunflower_theorem_s11) (.seq (.arrow sunflower_theorem_l11 sunflower_theorem_s11 sunflower_theorem_s12) (.seq (.prod (.arrow sunflower_theorem_l12 sunflower_theorem_s12 sunflower_theorem_s21) (.arrow sunflower_theorem_l12 sunflower_theorem_s12 sunflower_theorem_s21)) (.seq (.arrow sunflower_theorem_l21 sunflower_theorem_s21 sunflower_theorem_s21) (.seq (.arrow sunflower_theorem_l21 sunflower_theorem_s21 sunflower_theorem_s22) (.seq (.arrow sunflower_theorem_l22 sunflower_theorem_s22 sunflower_theorem_s23) (.seq (.arrow sunflower_theorem_l23 sunflower_theorem_s23 sunflower_theorem_s24) (.seq (.arrow sunflower_theorem_l24 sunflower_theorem_s24 sunflower_theorem_s25) (.seq (.arrow sunflower_theorem_l25 sunflower_theorem_s25 sunflower_theorem_s26) (.seq (.arrow sunflower_theorem_l26 sunflower_theorem_s26 sunflower_theorem_s27) (.seq (.arrow sunflower_theorem_l27 sunflower_theorem_s27 sunflower_theorem_s28) (.seq (.arrow sunflower_theorem_l28 sunflower_theorem_s28 sunflower_theorem_s29) (.seq (.arrow sunflower_theorem_l29 sunflower_theorem_s29 sunflower_theorem_s30) (.seq (.arrow sunflower_theorem_l30 sunflower_theorem_s30 sunflower_theorem_s31) (.seq (.arrow sunflower_theorem_l31 sunflower_theorem_s31 sunflower_theorem_s32) (.seq (.arrow sunflower_theorem_l32 sunflower_theorem_s32 sunflower_theorem_s33) (.seq (.arrow sunflower_theorem_l33 sunflower_theorem_s33 sunflower_theorem_s34) (.seq (.arrow sunflower_theorem_l34 sunflower_theorem_s34 sunflower_theorem_s35) (.seq (.arrow sunflower_theorem_l35 sunflower_theorem_s35 sunflower_theorem_s36) (.seq (.arrow sunflower_theorem_l36 sunflower_theorem_s36 sunflower_theorem_s37) (.arrow sunflower_theorem_l37 sunflower_theorem_s37 sunflower_theorem_s38)))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def sunflower_theorem_true_arm : IGProtocol sunflower_theorem_s0 sunflower_theorem_s38 :=
  (sunflower_theorem_protocol).restrictToEVALT
  --
-- false arm
noncomputable def sunflower_theorem_false_arm : IGProtocol sunflower_theorem_s0 sunflower_theorem_s38 :=
  (sunflower_theorem_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def sunflower_theorem_tier : OuroboricityTier := TierFunctor.obj sunflower_theorem_s0
#eval sunflower_theorem_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem sunflower_theorem_frobenius :
    igFrobeniusAlg.mul sunflower_theorem_s0 sunflower_theorem_s0 = sunflower_theorem_s0 :=
  igFrobAlg_self_fusion sunflower_theorem_s0
