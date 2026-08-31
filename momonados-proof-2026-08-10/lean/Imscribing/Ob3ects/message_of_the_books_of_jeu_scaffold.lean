-- IGProtocol scaffold: VINIT → AFWD → AFWD → AFWD → AFWD → AFWD → AFWD → AFWD → AFWD → IMSCRIB → AFWD → AFWD → AFWD → TANCH → AFWD → AFWD → CLINK → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → IFIX → AFWD → AFWD → AFWD → AFWD → AFWD → AFWD → AFWD → EVALT → AFWD → ENGAGR → AFWD → AFWD → AFWD → AFWD → AFWD → AREV → CLINK → IMSCRIB → IFIX
-- Class: Message of The Books of Jeu
-- Fingerprint: sig=(33,2,5,2)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=42
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(17, 21)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑾  | forward morphism — bidirectional arrow
--   [2] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [4] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [5] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [6] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [7] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [8] AFWD      rel    := 𐑾               𐑾 → 𐑠  | forward morphism — bidirectional arrow
--   [9] IMSCRIB   gram   := 𐑠               𐑾 → 𐑾  | identity — self-imscription
--   [10] AFWD      rel    := 𐑾               𐑠 → 𐑾  | forward morphism — bidirectional arrow
--   [11] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [12] AFWD      rel    := 𐑾               𐑾 → 𐑡  | forward morphism — bidirectional arrow
--   [13] TANCH     top    := 𐑡               𐑾 → 𐑾  | terminal object — connectivity boundary
--   [14] AFWD      rel    := 𐑾               𐑡 → 𐑾  | forward morphism — bidirectional arrow
--   [15] AFWD      rel    := 𐑾               𐑾 → 𐑱  | forward morphism — bidirectional arrow
--   [16] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [17] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [18] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [19] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [20] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [21] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [22] IFIX      prot   := 𐑭               𐑙 → 𐑾  | irreversible fixation — winding number
--   [23] AFWD      rel    := 𐑾               𐑭 → 𐑾  | forward morphism — bidirectional arrow
--   [24] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [25] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [26] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [27] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [28] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [29] AFWD      rel    := 𐑾               𐑾 → ⊙  | forward morphism — bidirectional arrow
--   [30] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [31] AFWD      rel    := 𐑾               ⊙ → 𐑳  | forward morphism — bidirectional arrow
--   [32] ENGAGR    stoi   := 𐑳               𐑾 → 𐑾  | engage paradox — B-state, both arms
--   [33] AFWD      rel    := 𐑾               𐑳 → 𐑾  | forward morphism — bidirectional arrow
--   [34] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [35] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [36] AFWD      rel    := 𐑾               𐑾 → 𐑾  | forward morphism — bidirectional arrow
--   [37] AFWD      rel    := 𐑾               𐑾 → 𐑗  | forward morphism — bidirectional arrow
--   [38] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [39] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [40] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [41] IFIX      prot   := 𐑭               𐑠 → 𐑼  | irreversible fixation — winding number

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def message_of_the_books_of_jeu_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def message_of_the_books_of_jeu_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s39 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s40 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def message_of_the_books_of_jeu_s41 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def message_of_the_books_of_jeu_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l10 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l12 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l14 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l15 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def message_of_the_books_of_jeu_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def message_of_the_books_of_jeu_l23 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l24 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l25 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l26 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l27 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l28 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l29 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l31 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def message_of_the_books_of_jeu_l33 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l34 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l35 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l36 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l37 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l39 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l40 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def message_of_the_books_of_jeu_l41 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def message_of_the_books_of_jeu_protocol : IGProtocol message_of_the_books_of_jeu_s0 message_of_the_books_of_jeu_s41 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct message_of_the_books_of_jeu_s21 message_of_the_books_of_jeu_s21 = message_of_the_books_of_jeu_s21 (idempotent)
  (.seq (.arrow message_of_the_books_of_jeu_l0 message_of_the_books_of_jeu_s0 message_of_the_books_of_jeu_s1) (.seq (.arrow message_of_the_books_of_jeu_l1 message_of_the_books_of_jeu_s1 message_of_the_books_of_jeu_s2) (.seq (.arrow message_of_the_books_of_jeu_l2 message_of_the_books_of_jeu_s2 message_of_the_books_of_jeu_s3) (.seq (.arrow message_of_the_books_of_jeu_l3 message_of_the_books_of_jeu_s3 message_of_the_books_of_jeu_s4) (.seq (.arrow message_of_the_books_of_jeu_l4 message_of_the_books_of_jeu_s4 message_of_the_books_of_jeu_s5) (.seq (.arrow message_of_the_books_of_jeu_l5 message_of_the_books_of_jeu_s5 message_of_the_books_of_jeu_s6) (.seq (.arrow message_of_the_books_of_jeu_l6 message_of_the_books_of_jeu_s6 message_of_the_books_of_jeu_s7) (.seq (.arrow message_of_the_books_of_jeu_l7 message_of_the_books_of_jeu_s7 message_of_the_books_of_jeu_s8) (.seq (.arrow message_of_the_books_of_jeu_l8 message_of_the_books_of_jeu_s8 message_of_the_books_of_jeu_s9) (.seq (.arrow message_of_the_books_of_jeu_l9 message_of_the_books_of_jeu_s9 message_of_the_books_of_jeu_s10) (.seq (.arrow message_of_the_books_of_jeu_l10 message_of_the_books_of_jeu_s10 message_of_the_books_of_jeu_s11) (.seq (.arrow message_of_the_books_of_jeu_l11 message_of_the_books_of_jeu_s11 message_of_the_books_of_jeu_s12) (.seq (.arrow message_of_the_books_of_jeu_l12 message_of_the_books_of_jeu_s12 message_of_the_books_of_jeu_s13) (.seq (.arrow message_of_the_books_of_jeu_l13 message_of_the_books_of_jeu_s13 message_of_the_books_of_jeu_s14) (.seq (.arrow message_of_the_books_of_jeu_l14 message_of_the_books_of_jeu_s14 message_of_the_books_of_jeu_s15) (.seq (.arrow message_of_the_books_of_jeu_l15 message_of_the_books_of_jeu_s15 message_of_the_books_of_jeu_s16) (.seq (.arrow message_of_the_books_of_jeu_l16 message_of_the_books_of_jeu_s16 message_of_the_books_of_jeu_s17) (.seq (.prod (.arrow message_of_the_books_of_jeu_l17 message_of_the_books_of_jeu_s17 message_of_the_books_of_jeu_s21) (.arrow message_of_the_books_of_jeu_l17 message_of_the_books_of_jeu_s17 message_of_the_books_of_jeu_s21)) (.seq (.arrow message_of_the_books_of_jeu_l21 message_of_the_books_of_jeu_s21 message_of_the_books_of_jeu_s21) (.seq (.arrow message_of_the_books_of_jeu_l21 message_of_the_books_of_jeu_s21 message_of_the_books_of_jeu_s22) (.seq (.arrow message_of_the_books_of_jeu_l22 message_of_the_books_of_jeu_s22 message_of_the_books_of_jeu_s23) (.seq (.arrow message_of_the_books_of_jeu_l23 message_of_the_books_of_jeu_s23 message_of_the_books_of_jeu_s24) (.seq (.arrow message_of_the_books_of_jeu_l24 message_of_the_books_of_jeu_s24 message_of_the_books_of_jeu_s25) (.seq (.arrow message_of_the_books_of_jeu_l25 message_of_the_books_of_jeu_s25 message_of_the_books_of_jeu_s26) (.seq (.arrow message_of_the_books_of_jeu_l26 message_of_the_books_of_jeu_s26 message_of_the_books_of_jeu_s27) (.seq (.arrow message_of_the_books_of_jeu_l27 message_of_the_books_of_jeu_s27 message_of_the_books_of_jeu_s28) (.seq (.arrow message_of_the_books_of_jeu_l28 message_of_the_books_of_jeu_s28 message_of_the_books_of_jeu_s29) (.seq (.arrow message_of_the_books_of_jeu_l29 message_of_the_books_of_jeu_s29 message_of_the_books_of_jeu_s30) (.seq (.arrow message_of_the_books_of_jeu_l30 message_of_the_books_of_jeu_s30 message_of_the_books_of_jeu_s31) (.seq (.arrow message_of_the_books_of_jeu_l31 message_of_the_books_of_jeu_s31 message_of_the_books_of_jeu_s32) (.seq (.arrow message_of_the_books_of_jeu_l32 message_of_the_books_of_jeu_s32 message_of_the_books_of_jeu_s33) (.seq (.arrow message_of_the_books_of_jeu_l33 message_of_the_books_of_jeu_s33 message_of_the_books_of_jeu_s34) (.seq (.arrow message_of_the_books_of_jeu_l34 message_of_the_books_of_jeu_s34 message_of_the_books_of_jeu_s35) (.seq (.arrow message_of_the_books_of_jeu_l35 message_of_the_books_of_jeu_s35 message_of_the_books_of_jeu_s36) (.seq (.arrow message_of_the_books_of_jeu_l36 message_of_the_books_of_jeu_s36 message_of_the_books_of_jeu_s37) (.seq (.arrow message_of_the_books_of_jeu_l37 message_of_the_books_of_jeu_s37 message_of_the_books_of_jeu_s38) (.seq (.arrow message_of_the_books_of_jeu_l38 message_of_the_books_of_jeu_s38 message_of_the_books_of_jeu_s39) (.seq (.arrow message_of_the_books_of_jeu_l39 message_of_the_books_of_jeu_s39 message_of_the_books_of_jeu_s40) (.arrow message_of_the_books_of_jeu_l40 message_of_the_books_of_jeu_s40 message_of_the_books_of_jeu_s41)))))))))))))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def message_of_the_books_of_jeu_true_arm : IGProtocol message_of_the_books_of_jeu_s0 message_of_the_books_of_jeu_s41 :=
  (message_of_the_books_of_jeu_protocol).restrictToEVALT
  --
-- false arm
noncomputable def message_of_the_books_of_jeu_false_arm : IGProtocol message_of_the_books_of_jeu_s0 message_of_the_books_of_jeu_s41 :=
  (message_of_the_books_of_jeu_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def message_of_the_books_of_jeu_tier : OuroboricityTier := TierFunctor.obj message_of_the_books_of_jeu_s0
#eval message_of_the_books_of_jeu_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem message_of_the_books_of_jeu_frobenius :
    igFrobeniusAlg.mul message_of_the_books_of_jeu_s0 message_of_the_books_of_jeu_s0 = message_of_the_books_of_jeu_s0 :=
  igFrobAlg_self_fusion message_of_the_books_of_jeu_s0
