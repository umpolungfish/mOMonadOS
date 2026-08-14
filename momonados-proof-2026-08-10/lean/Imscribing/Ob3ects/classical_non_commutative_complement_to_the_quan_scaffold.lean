-- IGProtocol scaffold: VINIT → TANCH → FSPLIT → AFWD → EVALT → IMSCRIB → CLINK → FFUSE → AREV → EVALF → ENGAGR → CLINK → FFUSE → IMSCRIB → IFIX → AFWD → CLINK → IMSCRIB → FSPLIT → EVALT → EVALF → FFUSE → AREV → IFIX
-- Class: Classical non-commutative complement to the quantum probabilities (F-arm anchor)
-- Fingerprint: sig=(12,5,5,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=24
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(2, 7), (18, 21)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor
import Imscribing.IGScaffold

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑚  | terminal object — connectivity boundary
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [6] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [8] AREV      pol    := 𐑗               𐑙 → 𐑖  | reverse morphism — parity flip
--   [9] EVALF     chir   := 𐑖               𐑗 → 𐑳  | evaluate-false — chirality check
--   [10] ENGAGR    stoi   := 𐑳               𐑖 → 𐑱  | engage paradox — B-state, both arms
--   [11] CLINK     fid    := 𐑱               𐑳 → 𐑙  | composition — regime coherence
--   [12] FFUSE     stoi   := 𐑙               𐑱 → 𐑠  | fuse μ — assembly mode
--   [13] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [14] IFIX      prot   := 𐑭               𐑠 → 𐑾  | irreversible fixation — winding number
--   [15] AFWD      rel    := 𐑾               𐑭 → 𐑱  | forward morphism — bidirectional arrow
--   [16] CLINK     fid    := 𐑱               𐑾 → 𐑠  | composition — regime coherence
--   [17] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [18] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [19] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [20] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [21] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [22] AREV      pol    := 𐑗               𐑙 → 𐑭  | reverse morphism — parity flip
--   [23] IFIX      prot   := 𐑭               𐑗 → 𐑼  | irreversible fixation — winding number

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def classical_non_commutative_complement_to_the_quantum_probabilities_f_arm_anchor_protocol : IGProtocol 𐑼 𐑭 :=
  .withGram .measure <|
  -- Seq chain:
  (.arrow 𐑼 𐑼 𐑡)  -- [0] VINIT | dim := 𐑼 | initial object — ground of distinction (Genesis of unclassified void state before any primitive assignment)
  ▷ (.arrow 𐑡 𐑼 𐑚)  -- [1] TANCH | top := 𐑡 | terminal object — connectivity boundary (Establishing the Crystal of Types as terminal containment of all possibilities)
  -- FSPLIT [2] (gran := 𐑚) (Splitting the primitive assignments into valid and invalid branches) / FFUSE [7] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (4 nodes)
      ((.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow ⊙ 𐑚 𐑙) ▷
      (.arrow 𐑠 𐑚 𐑙) ▷
      (.arrow 𐑱 𐑚 𐑙))
      -- F-branch (0 nodes)
      (.refl 𐑙)
    -- reconnect at FFUSE [7]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑗)
  ▷ (.arrow 𐑗 𐑙 𐑖)  -- [8] AREV | pol := 𐑗 | reverse morphism — parity flip (Reverse morphism of paraconsistent absorption by Criticality primitive)
  ▷ (.arrow 𐑖 𐑗 𐑳)  -- [9] EVALF | chir := 𐑖 | evaluate-false — chirality check (Evaluating negative rejection of structurally invalid combinations on F-branch)
  ▷ (.arrow 𐑳 𐑖 𐑱)  -- [10] ENGAGR | stoi := 𐑳 | engage paradox — B-state, both arms (Paraconsistent state at Criticality (⊙) absorbing contradictions)
  ▷ (.arrow 𐑱 𐑳 𐑙)  -- [11] CLINK | fid := 𐑱 | composition — regime coherence (Composition of F-branch results)
  ▷ (.arrow 𐑙 𐑱 𐑠)  -- [12] FFUSE | stoi := 𐑙 | fuse μ — assembly mode (Frobenius fusion completing the evaluation cycle)
  ▷ (.arrow 𐑠 𐑙 𐑭)  -- [13] IMSCRIB | gram := 𐑠 | identity — self-imscription (Self-recognition of completed classification system)
  ▷ (.arrow 𐑭 𐑠 𐑾)  -- [14] IFIX | prot := 𐑭 | irreversible fixation — winding number (Permanently fixing valid types in the Crystal lattice)
  ▷ (.arrow 𐑾 𐑭 𐑱)  -- [15] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Forward propagation of classified types to measurement space)
  ▷ (.arrow 𐑱 𐑾 𐑠)  -- [16] CLINK | fid := 𐑱 | composition — regime coherence (Tensor composition of type-to-measurement mappings)
  ▷ (.arrow 𐑠 𐑱 𐑚)  -- [17] IMSCRIB | gram := 𐑠 | identity — self-imscription (Self-verification of SIC-POVM construction from classified types)
  -- FSPLIT [18] (gran := 𐑚) (Branching evaluation for quantum measurement consistency) / FFUSE [21] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (1 nodes)
      (.arrow ⊙ 𐑚 𐑙)
      -- F-branch (1 nodes)
      (.arrow 𐑖 𐑚 𐑙)
    -- reconnect at FFUSE [21]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑗)
  ▷ (.arrow 𐑗 𐑙 𐑭)  -- [22] AREV | pol := 𐑗 | reverse morphism — parity flip (Criticality absorption maintaining paraconsistent closure)
  ▷ (.arrow 𐑭 𐑗 𐑼)  -- [23] IFIX | prot := 𐑭 | irreversible fixation — winding number (Final crystalline record of quantum-classical complementarity)
  --
-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

-- truth arm
noncomputable def classical_non_commutative_complement_to_the_quantum_probabilities_f_arm_anchor_true_arm : IGProtocol 𐑼 𐑭 :=
  (classical_non_commutative_complement_to_the_quantum_probabilities_f_arm_anchor_protocol).restrictToEVALT
  --
-- false arm
noncomputable def classical_non_commutative_complement_to_the_quantum_probabilities_f_arm_anchor_false_arm : IGProtocol 𐑼 𐑭 :=
  (classical_non_commutative_complement_to_the_quantum_probabilities_f_arm_anchor_protocol).restrictToEVALF
  --
-- ── Verification theorems ───────────────────────────────────────────────────

theorem classical_non_commutative_complement_to_the_quantum_probabilities_f_arm_anchor_tier : TierFunctor.obj 𐑼 = .O₂ := by decide

end Imscribing
