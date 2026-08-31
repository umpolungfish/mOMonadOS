-- IGProtocol scaffold: VINIT → IMSCRIB → CLINK → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → AFWD → FSPLIT → AREV → AREV → ENGAGR → FFUSE → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → AFWD → EVALT → TANCH → IFIX
-- Class: The return leg of the primitives-to-IMASM cycle: reading a concatenated IMASM word back into the twelve primitive types that wrote it. The forward leg writes each type as its own program and concatenates in canonical axis order; the return leg parses the word axis by axis, each axis consuming an admissible type program as a prefix, because type programs are not self-delimiting. The reading recovers the original type at every axis but is not injective: the alphabet emits fewer distinct programs than it has types, and where two colliding types share an axis the word has two pre-images and the reading reports both rather than choosing. A mu that inverts its delta everywhere except on one axis, where the ambiguity belongs to the alphabet and not to the reading.
-- Fingerprint: sig=(18,2,12,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=33
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(18, 22)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑱  | identity — self-imscription
--   [2] CLINK     fid    := 𐑱               𐑠 → 𐑾  | composition — regime coherence
--   [3] AFWD      rel    := 𐑾               𐑱 → ⊙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [5] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [6] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [7] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [8] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [9] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [10] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [11] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [12] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [13] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [14] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [15] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [16] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [17] AFWD      rel    := 𐑾               ⊙ → 𐑚  | forward morphism — bidirectional arrow
--   [18] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [19] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [20] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [21] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [22] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [23] AFWD      rel    := 𐑾               𐑙 → ⊙  | forward morphism — bidirectional arrow
--   [24] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [25] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [26] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [27] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [28] EVALT     crit   := ⊙               𐑾 → 𐑾  | evaluate-true — criticality gate open
--   [29] AFWD      rel    := 𐑾               ⊙ → ⊙  | forward morphism — bidirectional arrow
--   [30] EVALT     crit   := ⊙               𐑾 → 𐑡  | evaluate-true — criticality gate open
--   [31] TANCH     top    := 𐑡               ⊙ → 𐑭  | terminal object — connectivity boundary
--   [32] IFIX      prot   := 𐑭               𐑡 → 𐑼  | irreversible fixation — winding number

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_return_leg_of_the_primitives_to_5c81a6_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_return_leg_of_the_primitives_to_5c81a6_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l9 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l13 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l15 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l17 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l23 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l25 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l27 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l29 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l31 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_return_leg_of_the_primitives_to_5c81a6_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_return_leg_of_the_primitives_to_5c81a6_protocol : IGProtocol the_return_leg_of_the_primitives_to_5c81a6_s0 the_return_leg_of_the_primitives_to_5c81a6_s32 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_return_leg_of_the_primitives_to_5c81a6_s22 the_return_leg_of_the_primitives_to_5c81a6_s22 = the_return_leg_of_the_primitives_to_5c81a6_s22 (idempotent)
  (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l0 the_return_leg_of_the_primitives_to_5c81a6_s0 the_return_leg_of_the_primitives_to_5c81a6_s1) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l1 the_return_leg_of_the_primitives_to_5c81a6_s1 the_return_leg_of_the_primitives_to_5c81a6_s2) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l2 the_return_leg_of_the_primitives_to_5c81a6_s2 the_return_leg_of_the_primitives_to_5c81a6_s3) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l3 the_return_leg_of_the_primitives_to_5c81a6_s3 the_return_leg_of_the_primitives_to_5c81a6_s4) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l4 the_return_leg_of_the_primitives_to_5c81a6_s4 the_return_leg_of_the_primitives_to_5c81a6_s5) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l5 the_return_leg_of_the_primitives_to_5c81a6_s5 the_return_leg_of_the_primitives_to_5c81a6_s6) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l6 the_return_leg_of_the_primitives_to_5c81a6_s6 the_return_leg_of_the_primitives_to_5c81a6_s7) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l7 the_return_leg_of_the_primitives_to_5c81a6_s7 the_return_leg_of_the_primitives_to_5c81a6_s8) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l8 the_return_leg_of_the_primitives_to_5c81a6_s8 the_return_leg_of_the_primitives_to_5c81a6_s9) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l9 the_return_leg_of_the_primitives_to_5c81a6_s9 the_return_leg_of_the_primitives_to_5c81a6_s10) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l10 the_return_leg_of_the_primitives_to_5c81a6_s10 the_return_leg_of_the_primitives_to_5c81a6_s11) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l11 the_return_leg_of_the_primitives_to_5c81a6_s11 the_return_leg_of_the_primitives_to_5c81a6_s12) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l12 the_return_leg_of_the_primitives_to_5c81a6_s12 the_return_leg_of_the_primitives_to_5c81a6_s13) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l13 the_return_leg_of_the_primitives_to_5c81a6_s13 the_return_leg_of_the_primitives_to_5c81a6_s14) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l14 the_return_leg_of_the_primitives_to_5c81a6_s14 the_return_leg_of_the_primitives_to_5c81a6_s15) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l15 the_return_leg_of_the_primitives_to_5c81a6_s15 the_return_leg_of_the_primitives_to_5c81a6_s16) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l16 the_return_leg_of_the_primitives_to_5c81a6_s16 the_return_leg_of_the_primitives_to_5c81a6_s17) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l17 the_return_leg_of_the_primitives_to_5c81a6_s17 the_return_leg_of_the_primitives_to_5c81a6_s18) (.seq (.prod (.arrow the_return_leg_of_the_primitives_to_5c81a6_l18 the_return_leg_of_the_primitives_to_5c81a6_s18 the_return_leg_of_the_primitives_to_5c81a6_s22) (.arrow the_return_leg_of_the_primitives_to_5c81a6_l18 the_return_leg_of_the_primitives_to_5c81a6_s18 the_return_leg_of_the_primitives_to_5c81a6_s22)) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l22 the_return_leg_of_the_primitives_to_5c81a6_s22 the_return_leg_of_the_primitives_to_5c81a6_s22) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l22 the_return_leg_of_the_primitives_to_5c81a6_s22 the_return_leg_of_the_primitives_to_5c81a6_s23) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l23 the_return_leg_of_the_primitives_to_5c81a6_s23 the_return_leg_of_the_primitives_to_5c81a6_s24) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l24 the_return_leg_of_the_primitives_to_5c81a6_s24 the_return_leg_of_the_primitives_to_5c81a6_s25) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l25 the_return_leg_of_the_primitives_to_5c81a6_s25 the_return_leg_of_the_primitives_to_5c81a6_s26) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l26 the_return_leg_of_the_primitives_to_5c81a6_s26 the_return_leg_of_the_primitives_to_5c81a6_s27) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l27 the_return_leg_of_the_primitives_to_5c81a6_s27 the_return_leg_of_the_primitives_to_5c81a6_s28) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l28 the_return_leg_of_the_primitives_to_5c81a6_s28 the_return_leg_of_the_primitives_to_5c81a6_s29) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l29 the_return_leg_of_the_primitives_to_5c81a6_s29 the_return_leg_of_the_primitives_to_5c81a6_s30) (.seq (.arrow the_return_leg_of_the_primitives_to_5c81a6_l30 the_return_leg_of_the_primitives_to_5c81a6_s30 the_return_leg_of_the_primitives_to_5c81a6_s31) (.arrow the_return_leg_of_the_primitives_to_5c81a6_l31 the_return_leg_of_the_primitives_to_5c81a6_s31 the_return_leg_of_the_primitives_to_5c81a6_s32))))))))))))))))))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_return_leg_of_the_primitives_to_5c81a6_true_arm : IGProtocol the_return_leg_of_the_primitives_to_5c81a6_s0 the_return_leg_of_the_primitives_to_5c81a6_s32 :=
  (the_return_leg_of_the_primitives_to_5c81a6_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_return_leg_of_the_primitives_to_5c81a6_tier : OuroboricityTier := TierFunctor.obj the_return_leg_of_the_primitives_to_5c81a6_s0
#eval the_return_leg_of_the_primitives_to_5c81a6_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_return_leg_of_the_primitives_to_5c81a6_frobenius :
    igFrobeniusAlg.mul the_return_leg_of_the_primitives_to_5c81a6_s0 the_return_leg_of_the_primitives_to_5c81a6_s0 = the_return_leg_of_the_primitives_to_5c81a6_s0 :=
  igFrobAlg_self_fusion the_return_leg_of_the_primitives_to_5c81a6_s0
