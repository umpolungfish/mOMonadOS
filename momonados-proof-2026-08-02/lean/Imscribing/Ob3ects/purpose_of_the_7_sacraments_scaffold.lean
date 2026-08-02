-- IGProtocol scaffold: VINIT → AREV → FSPLIT → EVALT → EVALF → FFUSE → AFWD → CLINK → FSPLIT → ENGAGR → FFUSE → EVALT → AREV → FSPLIT → EVALF → EVALT → FFUSE → AFWD → CLINK → FSPLIT → EVALT → FFUSE → AREV → FSPLIT → ENGAGR → FFUSE → IMSCRIB → CLINK → IFIX → EVALT → TANCH
-- Class: Purpose of the 7 Sacraments
-- Fingerprint: sig=(11,10,9,1)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=31
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(2, 5), (8, 10), (13, 16), (19, 21), (23, 25)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑗  | initial object — ground of distinction
--   [1] AREV      pol    := 𐑗               𐑼 → 𐑚  | reverse morphism — parity flip
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [6] AFWD      rel    := 𐑾               𐑙 → 𐑱  | forward morphism — bidirectional arrow
--   [7] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [8] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [9] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [10] FFUSE     stoi   := 𐑙               𐑙 → ⊙  | fuse μ — assembly mode
--   [11] EVALT     crit   := ⊙               𐑙 → 𐑗  | evaluate-true — criticality gate open
--   [12] AREV      pol    := 𐑗               ⊙ → 𐑚  | reverse morphism — parity flip
--   [13] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [14] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [15] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [16] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [17] AFWD      rel    := 𐑾               𐑙 → 𐑱  | forward morphism — bidirectional arrow
--   [18] CLINK     fid    := 𐑱               𐑾 → 𐑚  | composition — regime coherence
--   [19] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [20] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [21] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [22] AREV      pol    := 𐑗               𐑙 → 𐑚  | reverse morphism — parity flip
--   [23] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [24] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [25] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [26] IMSCRIB   gram   := 𐑠               𐑙 → 𐑱  | identity — self-imscription
--   [27] CLINK     fid    := 𐑱               𐑠 → 𐑭  | composition — regime coherence
--   [28] IFIX      prot   := 𐑭               𐑱 → ⊙  | irreversible fixation — winding number
--   [29] EVALT     crit   := ⊙               𐑭 → 𐑡  | evaluate-true — criticality gate open
--   [30] TANCH     top    := 𐑡               ⊙ → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def purpose_of_the_7_sacraments_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def purpose_of_the_7_sacraments_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def purpose_of_the_7_sacraments_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def purpose_of_the_7_sacraments_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def purpose_of_the_7_sacraments_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def purpose_of_the_7_sacraments_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def purpose_of_the_7_sacraments_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l17 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def purpose_of_the_7_sacraments_l25 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def purpose_of_the_7_sacraments_l29 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def purpose_of_the_7_sacraments_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def purpose_of_the_7_sacraments_protocol : IGProtocol purpose_of_the_7_sacraments_s0 purpose_of_the_7_sacraments_s30 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct purpose_of_the_7_sacraments_s5 purpose_of_the_7_sacraments_s5 = purpose_of_the_7_sacraments_s5 (idempotent)
  (.seq (.arrow purpose_of_the_7_sacraments_l0 purpose_of_the_7_sacraments_s0 purpose_of_the_7_sacraments_s1) (.seq (.arrow purpose_of_the_7_sacraments_l1 purpose_of_the_7_sacraments_s1 purpose_of_the_7_sacraments_s2) (.seq (.prod (.arrow purpose_of_the_7_sacraments_l2 purpose_of_the_7_sacraments_s2 purpose_of_the_7_sacraments_s5) (.arrow purpose_of_the_7_sacraments_l2 purpose_of_the_7_sacraments_s2 purpose_of_the_7_sacraments_s5)) (.seq (.arrow purpose_of_the_7_sacraments_l5 purpose_of_the_7_sacraments_s5 purpose_of_the_7_sacraments_s5) (.seq (.arrow purpose_of_the_7_sacraments_l5 purpose_of_the_7_sacraments_s5 purpose_of_the_7_sacraments_s6) (.seq (.arrow purpose_of_the_7_sacraments_l6 purpose_of_the_7_sacraments_s6 purpose_of_the_7_sacraments_s7) (.seq (.arrow purpose_of_the_7_sacraments_l7 purpose_of_the_7_sacraments_s7 purpose_of_the_7_sacraments_s8) (.seq (.arrow purpose_of_the_7_sacraments_l8 purpose_of_the_7_sacraments_s8 purpose_of_the_7_sacraments_s9) (.seq (.arrow purpose_of_the_7_sacraments_l9 purpose_of_the_7_sacraments_s9 purpose_of_the_7_sacraments_s10) (.seq (.arrow purpose_of_the_7_sacraments_l10 purpose_of_the_7_sacraments_s10 purpose_of_the_7_sacraments_s11) (.seq (.arrow purpose_of_the_7_sacraments_l11 purpose_of_the_7_sacraments_s11 purpose_of_the_7_sacraments_s12) (.seq (.arrow purpose_of_the_7_sacraments_l12 purpose_of_the_7_sacraments_s12 purpose_of_the_7_sacraments_s13) (.seq (.arrow purpose_of_the_7_sacraments_l13 purpose_of_the_7_sacraments_s13 purpose_of_the_7_sacraments_s14) (.seq (.arrow purpose_of_the_7_sacraments_l14 purpose_of_the_7_sacraments_s14 purpose_of_the_7_sacraments_s15) (.seq (.arrow purpose_of_the_7_sacraments_l15 purpose_of_the_7_sacraments_s15 purpose_of_the_7_sacraments_s16) (.seq (.arrow purpose_of_the_7_sacraments_l16 purpose_of_the_7_sacraments_s16 purpose_of_the_7_sacraments_s17) (.seq (.arrow purpose_of_the_7_sacraments_l17 purpose_of_the_7_sacraments_s17 purpose_of_the_7_sacraments_s18) (.seq (.arrow purpose_of_the_7_sacraments_l18 purpose_of_the_7_sacraments_s18 purpose_of_the_7_sacraments_s19) (.seq (.arrow purpose_of_the_7_sacraments_l19 purpose_of_the_7_sacraments_s19 purpose_of_the_7_sacraments_s20) (.seq (.arrow purpose_of_the_7_sacraments_l20 purpose_of_the_7_sacraments_s20 purpose_of_the_7_sacraments_s21) (.seq (.arrow purpose_of_the_7_sacraments_l21 purpose_of_the_7_sacraments_s21 purpose_of_the_7_sacraments_s22) (.seq (.arrow purpose_of_the_7_sacraments_l22 purpose_of_the_7_sacraments_s22 purpose_of_the_7_sacraments_s23) (.seq (.arrow purpose_of_the_7_sacraments_l23 purpose_of_the_7_sacraments_s23 purpose_of_the_7_sacraments_s24) (.seq (.arrow purpose_of_the_7_sacraments_l24 purpose_of_the_7_sacraments_s24 purpose_of_the_7_sacraments_s25) (.seq (.arrow purpose_of_the_7_sacraments_l25 purpose_of_the_7_sacraments_s25 purpose_of_the_7_sacraments_s26) (.seq (.arrow purpose_of_the_7_sacraments_l26 purpose_of_the_7_sacraments_s26 purpose_of_the_7_sacraments_s27) (.seq (.arrow purpose_of_the_7_sacraments_l27 purpose_of_the_7_sacraments_s27 purpose_of_the_7_sacraments_s28) (.seq (.arrow purpose_of_the_7_sacraments_l28 purpose_of_the_7_sacraments_s28 purpose_of_the_7_sacraments_s29) (.arrow purpose_of_the_7_sacraments_l29 purpose_of_the_7_sacraments_s29 purpose_of_the_7_sacraments_s30)))))))))))))))))))))))))))))
  --
-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def purpose_of_the_7_sacraments_true_arm : IGProtocol purpose_of_the_7_sacraments_s0 purpose_of_the_7_sacraments_s30 :=
  (purpose_of_the_7_sacraments_protocol).restrictToEVALT
  --
-- false arm
noncomputable def purpose_of_the_7_sacraments_false_arm : IGProtocol purpose_of_the_7_sacraments_s0 purpose_of_the_7_sacraments_s30 :=
  (purpose_of_the_7_sacraments_protocol).restrictToEVALF
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def purpose_of_the_7_sacraments_tier : OuroboricityTier := TierFunctor.obj purpose_of_the_7_sacraments_s0
#eval purpose_of_the_7_sacraments_tier  -- the Grammar's own verdict on its tier
