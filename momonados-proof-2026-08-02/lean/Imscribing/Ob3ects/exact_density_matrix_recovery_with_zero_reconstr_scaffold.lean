-- IGProtocol scaffold: VINIT → TANCH → FSPLIT → AFWD → EVALT → CLINK → IMSCRIB → IFIX → FFUSE → AFWD → EVALF → ENGAGR → AREV → CLINK → IMSCRIB → FFUSE → FSPLIT → EVALT → AFWD → IFIX → FFUSE → FSPLIT → AFWD → EVALT → CLINK → IMSCRIB → FFUSE → AREV → EVALF → FFUSE
-- Class: Exact density matrix recovery with zero reconstruction entropy (T-arm anchor)
-- Fingerprint: sig=(14,8,6,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=30
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(2, 8), (16, 20), (21, 26)]

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
--   [5] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [6] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [7] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [9] AFWD      rel    := 𐑾               𐑙 → 𐑖  | forward morphism — bidirectional arrow
--   [10] EVALF     chir   := 𐑖               𐑾 → 𐑳  | evaluate-false — chirality check
--   [11] ENGAGR    stoi   := 𐑳               𐑖 → 𐑗  | engage paradox — B-state, both arms
--   [12] AREV      pol    := 𐑗               𐑳 → 𐑱  | reverse morphism — parity flip
--   [13] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [14] IMSCRIB   gram   := 𐑠               𐑱 → 𐑙  | identity — self-imscription
--   [15] FFUSE     stoi   := 𐑙               𐑠 → 𐑚  | fuse μ — assembly mode
--   [16] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [17] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [18] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [19] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [20] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [21] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [22] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [23] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [24] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [25] IMSCRIB   gram   := 𐑠               𐑚 → 𐑙  | identity — self-imscription
--   [26] FFUSE     stoi   := 𐑙               𐑙 → 𐑗  | fuse μ — assembly mode
--   [27] AREV      pol    := 𐑗               𐑙 → 𐑖  | reverse morphism — parity flip
--   [28] EVALF     chir   := 𐑖               𐑗 → 𐑙  | evaluate-false — chirality check
--   [29] FFUSE     stoi   := 𐑙               𐑖 → 𐑼  | fuse μ — assembly mode

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def exact_density_matrix_recovery_with_zero_reconstruction_entropy_t_arm_anchor_protocol : IGProtocol 𐑼 𐑙 :=
  .withGram .measure <|
  -- Seq chain:
  (.arrow 𐑼 𐑼 𐑡)  -- [0] VINIT | dim := 𐑼 | initial object — ground of distinction (Genesis of quantum state before any measurement attempt)
  ▷ (.arrow 𐑡 𐑼 𐑚)  -- [1] TANCH | top := 𐑡 | terminal object — connectivity boundary (Establishing the Crystal constraint boundary for all valid joint assignments)
  -- FSPLIT [2] (gran := 𐑚) (Branching into T-branch (successful) and F-branch (failed) reconstruction paths) / FFUSE [8] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (5 nodes)
      ((.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow ⊙ 𐑚 𐑙) ▷
      (.arrow 𐑱 𐑚 𐑙) ▷
      (.arrow 𐑠 𐑚 𐑙) ▷
      (.arrow 𐑭 𐑚 𐑙))
      -- F-branch (0 nodes)
      (.refl 𐑙)
    -- reconnect at FFUSE [8]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑾)
  ▷ (.arrow 𐑾 𐑙 𐑖)  -- [9] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Forward morphism in F-branch to alternative configuration)
  ▷ (.arrow 𐑖 𐑾 𐑳)  -- [10] EVALF | chir := 𐑖 | evaluate-false — chirality check (Negative evaluation where reconstruction fails with information loss)
  ▷ (.arrow 𐑳 𐑖 𐑗)  -- [11] ENGAGR | stoi := 𐑳 | engage paradox — B-state, both arms (Paraconsistent fixed-point where both T and F states maintained)
  ▷ (.arrow 𐑗 𐑳 𐑱)  -- [12] AREV | pol := 𐑗 | reverse morphism — parity flip (Reverse transformation from quantum to classical Crystal assignment)
  ▷ (.arrow 𐑱 𐑗 𐑠)  -- [13] CLINK | fid := 𐑱 | composition — regime coherence (Composition of reverse-path elements)
  ▷ (.arrow 𐑠 𐑱 𐑙)  -- [14] IMSCRIB | gram := 𐑠 | identity — self-imscription (Self-recognition of classical Crystal point identity)
  ▷ (.arrow 𐑙 𐑠 𐑚)  -- [15] FFUSE | stoi := 𐑙 | fuse μ — assembly mode (Final reconstitution of complete system state)
  -- FSPLIT [16] (gran := 𐑚) (Second branching level into precision vs approximation modes) / FFUSE [20] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (3 nodes)
      ((.arrow ⊙ 𐑚 𐑙) ▷
      (.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow 𐑭 𐑚 𐑙))
      -- F-branch (0 nodes)
      (.refl 𐑙)
    -- reconnect at FFUSE [20]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑙)
  -- FSPLIT [21] (gran := 𐑚) (Third branching into symmetric vs asymmetric measurement modes) / FFUSE [26] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (4 nodes)
      ((.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow ⊙ 𐑚 𐑙) ▷
      (.arrow 𐑱 𐑚 𐑙) ▷
      (.arrow 𐑠 𐑚 𐑙))
      -- F-branch (0 nodes)
      (.refl 𐑙)
    -- reconnect at FFUSE [26]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑗)
  ▷ (.arrow 𐑗 𐑙 𐑖)  -- [27] AREV | pol := 𐑗 | reverse morphism — parity flip (Descent to asymmetric alternative path)
  ▷ (.arrow 𐑖 𐑗 𐑙)  -- [28] EVALF | chir := 𐑖 | evaluate-false — chirality check (Asymmetric mode with suboptimal bounds)
  ▷ (.arrow 𐑙 𐑖 𐑼)  -- [29] FFUSE | stoi := 𐑙 | fuse μ — assembly mode (Final fusion of all branches to complete reconstruction space)
  --
-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

-- truth arm
noncomputable def exact_density_matrix_recovery_with_zero_reconstruction_entropy_t_arm_anchor_true_arm : IGProtocol 𐑼 𐑙 :=
  (exact_density_matrix_recovery_with_zero_reconstruction_entropy_t_arm_anchor_protocol).restrictToEVALT
  --
-- false arm
noncomputable def exact_density_matrix_recovery_with_zero_reconstruction_entropy_t_arm_anchor_false_arm : IGProtocol 𐑼 𐑙 :=
  (exact_density_matrix_recovery_with_zero_reconstruction_entropy_t_arm_anchor_protocol).restrictToEVALF
  --
-- ── Verification theorems ───────────────────────────────────────────────────

theorem exact_density_matrix_recovery_with_zero_reconstruction_entropy_t_arm_anchor_tier : TierFunctor.obj 𐑼 = .O₂ := by decide

end Imscribing
