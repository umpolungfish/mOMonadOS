-- IGProtocol scaffold: VINIT → AFWD → IMSCRIB → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → CLINK → AREV → FSPLIT → EVALT → EVALF → FFUSE → TANCH → IFIX → FSPLIT → EVALT → EVALF → FFUSE → IMSCRIB → CLINK → TANCH → IFIX
-- Class: Apply ⊙ Criticality absorbing element entering Belnap BOTH paradice state
-- Fingerprint: sig=(9,6,7,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=24
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(3, 7), (10, 13), (16, 19)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor
import Imscribing.IGScaffold

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑠  | forward morphism — bidirectional arrow
--   [2] IMSCRIB   gram   := 𐑠               𐑾 → 𐑚  | identity — self-imscription
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑗  | composition — regime coherence
--   [9] AREV      pol    := 𐑗               𐑱 → 𐑚  | reverse morphism — parity flip
--   [10] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [11] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [12] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [13] FFUSE     stoi   := 𐑙               𐑙 → 𐑡  | fuse μ — assembly mode
--   [14] TANCH     top    := 𐑡               𐑙 → 𐑭  | terminal object — connectivity boundary
--   [15] IFIX      prot   := 𐑭               𐑡 → 𐑚  | irreversible fixation — winding number
--   [16] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [17] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [18] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [19] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [20] IMSCRIB   gram   := 𐑠               𐑙 → 𐑱  | identity — self-imscription
--   [21] CLINK     fid    := 𐑱               𐑠 → 𐑡  | composition — regime coherence
--   [22] TANCH     top    := 𐑡               𐑱 → 𐑭  | terminal object — connectivity boundary
--   [23] IFIX      prot   := 𐑭               𐑡 → 𐑼  | irreversible fixation — winding number

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def apply_criticality_absorbing_element_entering_belnap_both_paradice_state_protocol : IGProtocol 𐑼 𐑭 :=
  .withGram .measure <|
  -- Seq chain:
  (.arrow 𐑼 𐑼 𐑾)  -- [0] VINIT | dim := 𐑼 | initial object — ground of distinction (Genesis of primitive space before criticality enters)
  ▷ (.arrow 𐑾 𐑼 𐑠)  -- [1] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Dimensionality flow initiates structural configuration)
  ▷ (.arrow 𐑠 𐑾 𐑚)  -- [2] IMSCRIB | gram := 𐑠 | identity — self-imscription (Criticality ⊙ recognizes itself as self-modeling fixed-point)
  -- FSPLIT [3] (gran := 𐑚) (Criticality bifurcates into stable (EVALT) and explosive (EVALF) branches) / FFUSE [7] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (1 nodes)
      (.arrow ⊙ 𐑚 𐑙)
      -- F-branch (2 nodes)
      ((.arrow 𐑖 𐑚 𐑙) ▷
      (.arrow 𐑳 𐑚 𐑙))
    -- reconnect at FFUSE [7]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑱)
  ▷ (.arrow 𐑱 𐑙 𐑗)  -- [8] CLINK | fid := 𐑱 | composition — regime coherence (Compose elements around paradice state)
  ▷ (.arrow 𐑗 𐑱 𐑚)  -- [9] AREV | pol := 𐑗 | reverse morphism — parity flip (Chirality descent activates parity flip in critical regime)
  -- FSPLIT [10] (gran := 𐑚) (Second split examines criticality's self-absorption property) / FFUSE [13] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (1 nodes)
      (.arrow ⊙ 𐑚 𐑙)
      -- F-branch (1 nodes)
      (.arrow 𐑖 𐑚 𐑙)
    -- reconnect at FFUSE [13]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑡)
  ▷ (.arrow 𐑡 𐑙 𐑭)  -- [14] TANCH | top := 𐑡 | terminal object — connectivity boundary (Crystal boundary seals paraconsistent closure within constraint manifold)
  ▷ (.arrow 𐑭 𐑡 𐑚)  -- [15] IFIX | prot := 𐑭 | irreversible fixation — winding number (Permanently fix the paradice state as invariant)
  -- FSPLIT [16] (gran := 𐑚) (Final split tests frobenius closure property) / FFUSE [19] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (1 nodes)
      (.arrow ⊙ 𐑚 𐑙)
      -- F-branch (1 nodes)
      (.arrow 𐑖 𐑚 𐑙)
    -- reconnect at FFUSE [19]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑠)
  ▷ (.arrow 𐑠 𐑙 𐑱)  -- [20] IMSCRIB | gram := 𐑠 | identity — self-imscription (Criticality confirms self-identity within sealed system)
  ▷ (.arrow 𐑱 𐑠 𐑡)  -- [21] CLINK | fid := 𐑱 | composition — regime coherence (Complete composition sequence closing the structural loop)
  ▷ (.arrow 𐑡 𐑱 𐑭)  -- [22] TANCH | top := 𐑡 | terminal object — connectivity boundary (Final anchor seals entire criticality absorption process)
  ▷ (.arrow 𐑭 𐑡 𐑼)  -- [23] IFIX | prot := 𐑭 | irreversible fixation — winding number (Commit the complete paraconsistent criticality architecture to permanent form)
  --
-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

-- truth arm
noncomputable def apply_criticality_absorbing_element_entering_belnap_both_paradice_state_true_arm : IGProtocol 𐑼 𐑭 :=
  (apply_criticality_absorbing_element_entering_belnap_both_paradice_state_protocol).restrictToEVALT
  --
-- false arm
noncomputable def apply_criticality_absorbing_element_entering_belnap_both_paradice_state_false_arm : IGProtocol 𐑼 𐑭 :=
  (apply_criticality_absorbing_element_entering_belnap_both_paradice_state_protocol).restrictToEVALF
  --
-- ── Verification theorems ───────────────────────────────────────────────────

theorem apply_criticality_absorbing_element_entering_belnap_both_paradice_state_tier : TierFunctor.obj 𐑼 = .O₂ := by decide

end Imscribing
