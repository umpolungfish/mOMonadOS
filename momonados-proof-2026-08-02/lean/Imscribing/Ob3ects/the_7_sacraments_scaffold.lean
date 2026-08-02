-- IGProtocol scaffold: VINIT → AFWD → FSPLIT → EVALT → IFIX → CLINK → AFWD → EVALT → IFIX → CLINK → IMSCRIB → ENGAGR → EVALT → AFWD → CLINK → AREV → EVALF → FSPLIT → AREV → EVALT → FFUSE → AFWD → CLINK → AFWD → FSPLIT → EVALT → IFIX → AFWD → CLINK → FSPLIT → EVALT → AFWD → CLINK → AFWD → AREV → AFWD → EVALT → AFWD → AREV → TANCH → IMSCRIB → CLINK → IFIX
-- Class: The 7 Sacraments
-- Fingerprint: sig=(25,5,9,4)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=43
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(17, 20)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑚  | forward morphism — bidirectional arrow
--   [2] FSPLIT    gran   := 𐑚               𐑾 → ⊙  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑭  | evaluate-true — criticality gate open
--   [4] IFIX      prot   := 𐑭               ⊙ → 𐑱  | irreversible fixation — winding number
--   [5] CLINK     fid    := 𐑱               𐑭 → 𐑾  | composition — regime coherence
--   [6] AFWD      rel    := 𐑾               𐑱 → ⊙  | forward morphism — bidirectional arrow
--   [7] EVALT     crit   := ⊙               𐑾 → 𐑭  | evaluate-true — criticality gate open
--   [8] IFIX      prot   := 𐑭               ⊙ → 𐑱  | irreversible fixation — winding number
--   [9] CLINK     fid    := 𐑱               𐑭 → 𐑠  | composition — regime coherence
--   [10] IMSCRIB   gram   := 𐑠               𐑱 → 𐑳  | identity — self-imscription
--   [11] ENGAGR    stoi   := 𐑳               𐑠 → ⊙  | engage paradox — B-state, both arms
--   [12] EVALT     crit   := ⊙               𐑳 → 𐑾  | evaluate-true — criticality gate open
--   [13] AFWD      rel    := 𐑾               ⊙ → 𐑱  | forward morphism — bidirectional arrow
--   [14] CLINK     fid    := 𐑱               𐑾 → 𐑗  | composition — regime coherence
--   [15] AREV      pol    := 𐑗               𐑱 → 𐑖  | reverse morphism — parity flip
--   [16] EVALF     chir   := 𐑖               𐑗 → 𐑚  | evaluate-false — chirality check
--   [17] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [18] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [19] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [20] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [21] AFWD      rel    := 𐑾               𐑙 → 𐑱  | forward morphism — bidirectional arrow
--   [22] CLINK     fid    := 𐑱               𐑾 → 𐑾  | composition — regime coherence
--   [23] AFWD      rel    := 𐑾               𐑱 → 𐑚  | forward morphism — bidirectional arrow
--   [24] FSPLIT    gran   := 𐑚               𐑾 → ⊙  | split δ — range decomposition
--   [25] EVALT     crit   := ⊙               𐑚 → 𐑭  | evaluate-true — criticality gate open
--   [26] IFIX      prot   := 𐑭               ⊙ → 𐑾  | irreversible fixation — winding number
--   [27] AFWD      rel    := 𐑾               𐑭 → 𐑱  | forward morphism — bidirectional arrow
--   [28] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [29] FSPLIT    gran   := 𐑚               𐑱 → ⊙  | split δ — range decomposition
--   [30] EVALT     crit   := ⊙               𐑚 → 𐑾  | evaluate-true — criticality gate open
--   [31] AFWD      rel    := 𐑾               ⊙ → 𐑱  | forward morphism — bidirectional arrow
--   [32] CLINK     fid    := 𐑱               𐑾 → 𐑾  | composition — regime coherence
--   [33] AFWD      rel    := 𐑾               𐑱 → 𐑗  | forward morphism — bidirectional arrow
--   [34] AREV      pol    := 𐑗               𐑾 → 𐑾  | reverse morphism — parity flip
--   [35] AFWD      rel    := 𐑾               𐑗 → ⊙  | forward morphism — bidirectional arrow
--   [36] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [37] AFWD      rel    := 𐑾               ⊙ → 𐑗  | forward morphism — bidirectional arrow
--   [38] AREV      pol    := 𐑗               𐑾 → 𐑡  | reverse morphism — parity flip
--   [39] TANCH     top    := 𐑡               𐑗 → 𐑠  | terminal object — connectivity boundary
--   [40] IMSCRIB   gram   := 𐑠               𐑡 → 𐑱  | identity — self-imscription
--   [41] CLINK     fid    := 𐑱               𐑠 → 𐑭  | composition — regime coherence
--   [42] IFIX      prot   := 𐑭               𐑱 → 𐑼  | irreversible fixation — winding number

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_7_sacraments_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_7_sacraments_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_7_sacraments_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_7_sacraments_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_7_sacraments_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_7_sacraments_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_7_sacraments_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_7_sacraments_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_7_sacraments_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_7_sacraments_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s39 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s40 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s41 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_7_sacraments_s42 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_7_sacraments_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_7_sacraments_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l13 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_7_sacraments_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l21 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l23 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_7_sacraments_l27 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l31 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l33 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l35 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l37 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l39 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l40 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l41 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_7_sacraments_l42 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_7_sacraments_protocol : IGProtocol the_7_sacraments_s0 the_7_sacraments_s42 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_7_sacraments_s20 the_7_sacraments_s20 = the_7_sacraments_s20 (idempotent)
  (.seq (.arrow the_7_sacraments_l0 the_7_sacraments_s0 the_7_sacraments_s1) (.seq (.arrow the_7_sacraments_l1 the_7_sacraments_s1 the_7_sacraments_s2) (.seq (.arrow the_7_sacraments_l2 the_7_sacraments_s2 the_7_sacraments_s3) (.seq (.arrow the_7_sacraments_l3 the_7_sacraments_s3 the_7_sacraments_s4) (.seq (.arrow the_7_sacraments_l4 the_7_sacraments_s4 the_7_sacraments_s5) (.seq (.arrow the_7_sacraments_l5 the_7_sacraments_s5 the_7_sacraments_s6) (.seq (.arrow the_7_sacraments_l6 the_7_sacraments_s6 the_7_sacraments_s7) (.seq (.arrow the_7_sacraments_l7 the_7_sacraments_s7 the_7_sacraments_s8) (.seq (.arrow the_7_sacraments_l8 the_7_sacraments_s8 the_7_sacraments_s9) (.seq (.arrow the_7_sacraments_l9 the_7_sacraments_s9 the_7_sacraments_s10) (.seq (.arrow the_7_sacraments_l10 the_7_sacraments_s10 the_7_sacraments_s11) (.seq (.arrow the_7_sacraments_l11 the_7_sacraments_s11 the_7_sacraments_s12) (.seq (.arrow the_7_sacraments_l12 the_7_sacraments_s12 the_7_sacraments_s13) (.seq (.arrow the_7_sacraments_l13 the_7_sacraments_s13 the_7_sacraments_s14) (.seq (.arrow the_7_sacraments_l14 the_7_sacraments_s14 the_7_sacraments_s15) (.seq (.arrow the_7_sacraments_l15 the_7_sacraments_s15 the_7_sacraments_s16) (.seq (.arrow the_7_sacraments_l16 the_7_sacraments_s16 the_7_sacraments_s17) (.seq (.prod (.arrow the_7_sacraments_l17 the_7_sacraments_s17 the_7_sacraments_s20) (.arrow the_7_sacraments_l17 the_7_sacraments_s17 the_7_sacraments_s20)) (.seq (.arrow the_7_sacraments_l20 the_7_sacraments_s20 the_7_sacraments_s20) (.seq (.arrow the_7_sacraments_l20 the_7_sacraments_s20 the_7_sacraments_s21) (.seq (.arrow the_7_sacraments_l21 the_7_sacraments_s21 the_7_sacraments_s22) (.seq (.arrow the_7_sacraments_l22 the_7_sacraments_s22 the_7_sacraments_s23) (.seq (.arrow the_7_sacraments_l23 the_7_sacraments_s23 the_7_sacraments_s24) (.seq (.arrow the_7_sacraments_l24 the_7_sacraments_s24 the_7_sacraments_s25) (.seq (.arrow the_7_sacraments_l25 the_7_sacraments_s25 the_7_sacraments_s26) (.seq (.arrow the_7_sacraments_l26 the_7_sacraments_s26 the_7_sacraments_s27) (.seq (.arrow the_7_sacraments_l27 the_7_sacraments_s27 the_7_sacraments_s28) (.seq (.arrow the_7_sacraments_l28 the_7_sacraments_s28 the_7_sacraments_s29) (.seq (.arrow the_7_sacraments_l29 the_7_sacraments_s29 the_7_sacraments_s30) (.seq (.arrow the_7_sacraments_l30 the_7_sacraments_s30 the_7_sacraments_s31) (.seq (.arrow the_7_sacraments_l31 the_7_sacraments_s31 the_7_sacraments_s32) (.seq (.arrow the_7_sacraments_l32 the_7_sacraments_s32 the_7_sacraments_s33) (.seq (.arrow the_7_sacraments_l33 the_7_sacraments_s33 the_7_sacraments_s34) (.seq (.arrow the_7_sacraments_l34 the_7_sacraments_s34 the_7_sacraments_s35) (.seq (.arrow the_7_sacraments_l35 the_7_sacraments_s35 the_7_sacraments_s36) (.seq (.arrow the_7_sacraments_l36 the_7_sacraments_s36 the_7_sacraments_s37) (.seq (.arrow the_7_sacraments_l37 the_7_sacraments_s37 the_7_sacraments_s38) (.seq (.arrow the_7_sacraments_l38 the_7_sacraments_s38 the_7_sacraments_s39) (.seq (.arrow the_7_sacraments_l39 the_7_sacraments_s39 the_7_sacraments_s40) (.seq (.arrow the_7_sacraments_l40 the_7_sacraments_s40 the_7_sacraments_s41) (.arrow the_7_sacraments_l41 the_7_sacraments_s41 the_7_sacraments_s42)))))))))))))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_7_sacraments_true_arm : IGProtocol the_7_sacraments_s0 the_7_sacraments_s42 :=
  (the_7_sacraments_protocol).restrictToEVALT
  --
-- false arm
noncomputable def the_7_sacraments_false_arm : IGProtocol the_7_sacraments_s0 the_7_sacraments_s42 :=
  (the_7_sacraments_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def the_7_sacraments_tier : OuroboricityTier := TierFunctor.obj the_7_sacraments_s0
#eval the_7_sacraments_tier  -- the Grammar's own verdict on its tier
