-- IGProtocol scaffold: VINIT → AFWD → IFIX → FSPLIT → EVALT → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → FSPLIT → EVALT → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → FSPLIT → EVALT → AFWD → CLINK → IFIX → ENGAGR → AFWD → AREV → AFWD → AREV → EVALF → FFUSE → FSPLIT → EVALT → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → FSPLIT → EVALT → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → FSPLIT → EVALT → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → FSPLIT → EVALT → IMSCRIB → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → FSPLIT → EVALT → IMSCRIB → AFWD → CLINK → IFIX → EVALF → AREV → CLINK → IFIX → FFUSE → TANCH
-- Class: obtaining ∈⊢⊥<◻><⊞∈⊙⊣<⋈⊞⊣<<>⊢∋⊞⊢⊣<∋∋⊥∈<⊣⊞<⊣◻∋⋈⊣<⊤⊞⊣⊣⊥∈∋⊤⊢⊣◻<◻>⋈⋈>∋⊙⊣⊙⊤∈⊣<⊢ from <⊤⊤⋈⊤⊢⊥⊢∈>⊢⊞⋈⊣⊙>>⊥⊢<◻⊥⊢>⊣>◻∈<⊥⊤⊢<><⊢⊙∈>∋⊣⋈⊥∋⊣⋈⋈<⊣⊥⊞⊣⊤⋈>◻⊢⊣⋈∋⊢⊤⊣>⊥⊙∈⊤⊢∋>⊤⊢⋈<⋈⊞<∋◻⊞⊣>>><⊤<∋⊢⊢◻∋∈∈⊙⊢⊞<⊤⊙⊣⊙⊥⊞>∋∋∈⊥<>∋<∋⊤⋈∋⊞>⋈∋∈⊥⊞⊙⊙⊢
-- Fingerprint: sig=(39,16,17,16)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=88
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 12), (13, 22), (23, 34), (35, 44), (45, 54), (55, 64), (65, 75), (76, 86)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑭  | forward morphism — bidirectional arrow
--   [2] IFIX      prot   := 𐑭               𐑾 → 𐑚  | irreversible fixation — winding number
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [6] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [7] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [10] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [11] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [13] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [14] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [15] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [16] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [17] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [18] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [19] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [20] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [21] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [22] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [23] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [24] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [25] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [26] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [27] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [28] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [29] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [30] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [31] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [32] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [33] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [34] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [35] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [36] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [37] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [38] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [39] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [40] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [41] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [42] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [43] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [44] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [45] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [46] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [47] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [48] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [49] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [50] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [51] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [52] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [53] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [54] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [55] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [56] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [57] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [58] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [59] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [60] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [61] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [62] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [63] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [64] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [65] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [66] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [67] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [68] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [69] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [70] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [71] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [72] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [73] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [74] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [75] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [76] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [77] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [78] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [79] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [80] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [81] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [82] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [83] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [84] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [85] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [86] FFUSE     stoi   := 𐑙               𐑙 → 𐑡  | fuse μ — assembly mode
--   [87] TANCH     top    := 𐑡               𐑙 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def obtaining_from_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s23 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s24 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s25 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s26 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s27 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s28 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def obtaining_from_s29 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def obtaining_from_s30 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def obtaining_from_s31 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def obtaining_from_s32 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def obtaining_from_s33 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := ah }
private def obtaining_from_s34 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s35 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s36 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s37 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s38 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s39 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s40 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s41 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s42 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s43 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s44 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s45 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s46 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s47 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s48 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s49 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s50 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s51 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s52 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s53 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s54 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s55 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s56 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s57 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s58 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s59 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s60 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s61 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s62 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s63 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s64 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s65 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s66 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s67 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s68 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s69 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s70 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s71 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s72 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s73 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s74 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s75 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s76 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s77 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s78 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s79 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s80 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s81 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s82 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s83 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s84 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s85 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s86 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def obtaining_from_s87 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def obtaining_from_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l15 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l23 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l24 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l25 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l26 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l27 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l28 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def obtaining_from_l29 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l30 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l31 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l32 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l33 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l34 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l35 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l36 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l37 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l38 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l39 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l40 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l41 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l42 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l43 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l44 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l45 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l46 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l47 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l48 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l49 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l50 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l51 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l52 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l53 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l54 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l55 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l56 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l57 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l58 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l59 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l60 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l61 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l62 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l63 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l64 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l65 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l66 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l67 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l68 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l69 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l70 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l71 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l72 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l73 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l74 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l75 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l76 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l77 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l78 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l79 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l80 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l81 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l82 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def obtaining_from_l83 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l84 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l85 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def obtaining_from_l86 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def obtaining_from_l87 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def obtaining_from_protocol : IGProtocol obtaining_from_s0 obtaining_from_s87 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct obtaining_from_s12 obtaining_from_s12 = obtaining_from_s12 (idempotent)
  (.seq (.arrow obtaining_from_l0 obtaining_from_s0 obtaining_from_s1) (.seq (.arrow obtaining_from_l1 obtaining_from_s1 obtaining_from_s2) (.seq (.arrow obtaining_from_l2 obtaining_from_s2 obtaining_from_s3) (.seq (.prod (.arrow obtaining_from_l3 obtaining_from_s3 obtaining_from_s12) (.arrow obtaining_from_l3 obtaining_from_s3 obtaining_from_s12)) (.seq (.arrow obtaining_from_l12 obtaining_from_s12 obtaining_from_s12) (.seq (.arrow obtaining_from_l12 obtaining_from_s12 obtaining_from_s13) (.seq (.arrow obtaining_from_l13 obtaining_from_s13 obtaining_from_s14) (.seq (.arrow obtaining_from_l14 obtaining_from_s14 obtaining_from_s15) (.seq (.arrow obtaining_from_l15 obtaining_from_s15 obtaining_from_s16) (.seq (.arrow obtaining_from_l16 obtaining_from_s16 obtaining_from_s17) (.seq (.arrow obtaining_from_l17 obtaining_from_s17 obtaining_from_s18) (.seq (.arrow obtaining_from_l18 obtaining_from_s18 obtaining_from_s19) (.seq (.arrow obtaining_from_l19 obtaining_from_s19 obtaining_from_s20) (.seq (.arrow obtaining_from_l20 obtaining_from_s20 obtaining_from_s21) (.seq (.arrow obtaining_from_l21 obtaining_from_s21 obtaining_from_s22) (.seq (.arrow obtaining_from_l22 obtaining_from_s22 obtaining_from_s23) (.seq (.arrow obtaining_from_l23 obtaining_from_s23 obtaining_from_s24) (.seq (.arrow obtaining_from_l24 obtaining_from_s24 obtaining_from_s25) (.seq (.arrow obtaining_from_l25 obtaining_from_s25 obtaining_from_s26) (.seq (.arrow obtaining_from_l26 obtaining_from_s26 obtaining_from_s27) (.seq (.arrow obtaining_from_l27 obtaining_from_s27 obtaining_from_s28) (.seq (.arrow obtaining_from_l28 obtaining_from_s28 obtaining_from_s29) (.seq (.arrow obtaining_from_l29 obtaining_from_s29 obtaining_from_s30) (.seq (.arrow obtaining_from_l30 obtaining_from_s30 obtaining_from_s31) (.seq (.arrow obtaining_from_l31 obtaining_from_s31 obtaining_from_s32) (.seq (.arrow obtaining_from_l32 obtaining_from_s32 obtaining_from_s33) (.seq (.arrow obtaining_from_l33 obtaining_from_s33 obtaining_from_s34) (.seq (.arrow obtaining_from_l34 obtaining_from_s34 obtaining_from_s35) (.seq (.arrow obtaining_from_l35 obtaining_from_s35 obtaining_from_s36) (.seq (.arrow obtaining_from_l36 obtaining_from_s36 obtaining_from_s37) (.seq (.arrow obtaining_from_l37 obtaining_from_s37 obtaining_from_s38) (.seq (.arrow obtaining_from_l38 obtaining_from_s38 obtaining_from_s39) (.seq (.arrow obtaining_from_l39 obtaining_from_s39 obtaining_from_s40) (.seq (.arrow obtaining_from_l40 obtaining_from_s40 obtaining_from_s41) (.seq (.arrow obtaining_from_l41 obtaining_from_s41 obtaining_from_s42) (.seq (.arrow obtaining_from_l42 obtaining_from_s42 obtaining_from_s43) (.seq (.arrow obtaining_from_l43 obtaining_from_s43 obtaining_from_s44) (.seq (.arrow obtaining_from_l44 obtaining_from_s44 obtaining_from_s45) (.seq (.arrow obtaining_from_l45 obtaining_from_s45 obtaining_from_s46) (.seq (.arrow obtaining_from_l46 obtaining_from_s46 obtaining_from_s47) (.seq (.arrow obtaining_from_l47 obtaining_from_s47 obtaining_from_s48) (.seq (.arrow obtaining_from_l48 obtaining_from_s48 obtaining_from_s49) (.seq (.arrow obtaining_from_l49 obtaining_from_s49 obtaining_from_s50) (.seq (.arrow obtaining_from_l50 obtaining_from_s50 obtaining_from_s51) (.seq (.arrow obtaining_from_l51 obtaining_from_s51 obtaining_from_s52) (.seq (.arrow obtaining_from_l52 obtaining_from_s52 obtaining_from_s53) (.seq (.arrow obtaining_from_l53 obtaining_from_s53 obtaining_from_s54) (.seq (.arrow obtaining_from_l54 obtaining_from_s54 obtaining_from_s55) (.seq (.arrow obtaining_from_l55 obtaining_from_s55 obtaining_from_s56) (.seq (.arrow obtaining_from_l56 obtaining_from_s56 obtaining_from_s57) (.seq (.arrow obtaining_from_l57 obtaining_from_s57 obtaining_from_s58) (.seq (.arrow obtaining_from_l58 obtaining_from_s58 obtaining_from_s59) (.seq (.arrow obtaining_from_l59 obtaining_from_s59 obtaining_from_s60) (.seq (.arrow obtaining_from_l60 obtaining_from_s60 obtaining_from_s61) (.seq (.arrow obtaining_from_l61 obtaining_from_s61 obtaining_from_s62) (.seq (.arrow obtaining_from_l62 obtaining_from_s62 obtaining_from_s63) (.seq (.arrow obtaining_from_l63 obtaining_from_s63 obtaining_from_s64) (.seq (.arrow obtaining_from_l64 obtaining_from_s64 obtaining_from_s65) (.seq (.arrow obtaining_from_l65 obtaining_from_s65 obtaining_from_s66) (.seq (.arrow obtaining_from_l66 obtaining_from_s66 obtaining_from_s67) (.seq (.arrow obtaining_from_l67 obtaining_from_s67 obtaining_from_s68) (.seq (.arrow obtaining_from_l68 obtaining_from_s68 obtaining_from_s69) (.seq (.arrow obtaining_from_l69 obtaining_from_s69 obtaining_from_s70) (.seq (.arrow obtaining_from_l70 obtaining_from_s70 obtaining_from_s71) (.seq (.arrow obtaining_from_l71 obtaining_from_s71 obtaining_from_s72) (.seq (.arrow obtaining_from_l72 obtaining_from_s72 obtaining_from_s73) (.seq (.arrow obtaining_from_l73 obtaining_from_s73 obtaining_from_s74) (.seq (.arrow obtaining_from_l74 obtaining_from_s74 obtaining_from_s75) (.seq (.arrow obtaining_from_l75 obtaining_from_s75 obtaining_from_s76) (.seq (.arrow obtaining_from_l76 obtaining_from_s76 obtaining_from_s77) (.seq (.arrow obtaining_from_l77 obtaining_from_s77 obtaining_from_s78) (.seq (.arrow obtaining_from_l78 obtaining_from_s78 obtaining_from_s79) (.seq (.arrow obtaining_from_l79 obtaining_from_s79 obtaining_from_s80) (.seq (.arrow obtaining_from_l80 obtaining_from_s80 obtaining_from_s81) (.seq (.arrow obtaining_from_l81 obtaining_from_s81 obtaining_from_s82) (.seq (.arrow obtaining_from_l82 obtaining_from_s82 obtaining_from_s83) (.seq (.arrow obtaining_from_l83 obtaining_from_s83 obtaining_from_s84) (.seq (.arrow obtaining_from_l84 obtaining_from_s84 obtaining_from_s85) (.seq (.arrow obtaining_from_l85 obtaining_from_s85 obtaining_from_s86) (.arrow obtaining_from_l86 obtaining_from_s86 obtaining_from_s87))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def obtaining_from_true_arm : IGProtocol obtaining_from_s0 obtaining_from_s87 :=
  (obtaining_from_protocol).restrictToEVALT

-- false arm
noncomputable def obtaining_from_false_arm : IGProtocol obtaining_from_s0 obtaining_from_s87 :=
  (obtaining_from_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def obtaining_from_tier_ground : OuroboricityTier := TierFunctor.obj obtaining_from_s0
def obtaining_from_tier : OuroboricityTier := TierFunctor.obj obtaining_from_s87
#eval obtaining_from_tier_ground  -- tier of the ground (pre-transformation)
#eval obtaining_from_tier  -- the Grammar's own verdict on the closed object
