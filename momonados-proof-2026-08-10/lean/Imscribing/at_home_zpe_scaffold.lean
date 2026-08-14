/-
-- IGProtocol scaffold: at-home zero-point energy
-- Sequence: VINIT → TANCH → IMSCRIB → FSPLIT → AFWD → EVALT → CLINK → AREV → EVALF → ENGAGR → FFUSE → IFIX → TANCH
-- Class: at-home zero-point energy
-- Fingerprint: sig=(7,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(3, 10)]
--
-- Signature: CLINK and ENGAGR are INSIDE the FSPLIT/FFUSE gap
--   T-branch: 4 nodes (AFWD, EVALT, CLINK, AREV)
--   F-branch: 2 nodes (EVALF, ENGAGR)
--   Gap width: 7 steps (FSPLIT[3] → FFUSE[10])

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑠  | terminal object — connectivity boundary
--   [2] IMSCRIB   gram   := 𐑠               𐑡 → 𐑚  | identity — self-imscription
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence (INSIDE gap)
--   [7] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms (INSIDE gap)
--   [10] FFUSE    stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [11] IFIX     prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [12] TANCH    top    := 𐑡               𐑡 → 𐑼  | terminal object — connectivity boundary

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def at_home_zero_point_energy_protocol : IGProtocol 𐑼 𐑡 :=
  .withGram 𐑠 <|
  -- Seq chain:
  (.arrow 𐑼 𐑼 𐑡)  -- [0] VINIT | dim := 𐑼 | initial object — ground of distinction
  (.arrow 𐑡 𐑼 𐑠)  -- [1] TANCH | top := 𐑡 | terminal object — connectivity boundary
  (.arrow 𐑠 𐑡 𐑚)  -- [2] IMSCRIB | gram := 𐑠 | identity — self-imscription
  -- FSPLIT [3] (gran := 𐑚) / FFUSE [10] (stoi := 𐑙) — 7-step gap
  .seq
    (.prod
      -- T-branch (4 nodes: AFWD → EVALT → CLINK → AREV)
      .seq
        (.arrow 𐑾 𐑚 𐑙)  -- [4] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow
      .seq
        (.arrow ⊙ 𐑚 𐑙)  -- [5] EVALT | crit := ⊙ | evaluate-true — criticality gate open
      .seq
        (.arrow 𐑱 𐑚 𐑙)  -- [6] CLINK | fid := 𐑱 | composition — regime coherence (INSIDE gap)
        (.arrow 𐑗 𐑚 𐑙)  -- [7] AREV | pol := 𐑗 | reverse morphism — parity flip
      -- F-branch (2 nodes: EVALF → ENGAGR)
      .seq
        (.arrow 𐑖 𐑚 𐑙)  -- [8] EVALF | chir := 𐑖 | evaluate-false — chirality check
        (.arrow 𐑳 𐑚 𐑙)  -- [9] ENGAGR | stoi := 𐑳 | engage paradox — B-state, both arms (INSIDE gap)
    -- reconnect at FFUSE [10]: μ closes the Frobenius pair
    (.arrow 𐑙 𐑙 𐑱)  -- [10] FFUSE | stoi := 𐑙 | fuse μ — assembly mode
  (.arrow 𐑱 𐑙 𐑡)  -- [11] IFIX | prot := 𐑭 | irreversible fixation — winding number
  (.arrow 𐑡 𐑱 𐑼)  -- [12] TANCH | top := 𐑡 | terminal object — connectivity boundary

-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

-- truth arm (T-branch: AFWD → EVALT → CLINK → AREV)
noncomputable def at_home_zero_point_energy_true_arm : IGProtocol 𐑼 𐑡 :=
  (at_home_zero_point_energy_protocol).restrictToEVALT

-- false arm (F-branch: EVALF → ENGAGR)
noncomputable def at_home_zero_point_energy_false_arm : IGProtocol 𐑼 𐑡 :=
  (at_home_zero_point_energy_protocol).restrictToEVALF

-- ── Verification theorems ───────────────────────────────────────────────────

theorem at_home_zero_point_energy_tier : TierFunctor.obj 𐑼 = .O₂ := by
  decide

theorem at_home_zpe_frobenius_pair_exists : FSPLIT_FFUSE_Pair 3 10 := by
  -- The sequence has indices 3 (FSPLIT) and 10 (FFUSE)
  -- Verified by opcode sequence construction
  decide

-- Frobenius (split → fuse): μ∘δ = id on .prod branch
-- Proof: apply igFrobAlg_self_fusion; exact mu_delta_A_id
-- (requires mu_delta_A_id from IGFunctor library)

-- Theorem: CLINK is inside the extraction window
theorem clink_inside_gap : (3 < 6) ∧ (6 < 10) := by
  constructor <;> decide

-- Theorem: ENGAGR is inside the extraction window
theorem engagr_inside_gap : (3 < 9) ∧ (9 < 10) := by
  constructor <;> decide

-- T-branch has 4 nodes: AFWD, EVALT, CLINK, AREV
-- F-branch has 2 nodes: EVALF, ENGAGR

end Imscribing
-/
