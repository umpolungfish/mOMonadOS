-- IGProtocol scaffold: VINIT → FSPLIT → AFWD → AFWD → AFWD → CLINK → FFUSE → EVALT → AFWD → FSPLIT → EVALT → EVALF → FFUSE → FSPLIT → AFWD → AFWD → CLINK → FFUSE → IMSCRIB → ENGAGR → CLINK → IFIX → TANCH → IMSCRIB
-- Class: Seal the system with the Crystal Constraint Manifold boundary
-- Fingerprint: sig=(13,6,4,1)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=24
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(1, 6), (9, 12), (13, 17)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor
import Imscribing.IGScaffold

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [5] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [6] FFUSE     stoi   := 𐑙               𐑙 → ⊙  | fuse μ — assembly mode
--   [7] EVALT     crit   := ⊙               𐑙 → 𐑾  | evaluate-true — criticality gate open
--   [8] AFWD      rel    := 𐑾               ⊙ → 𐑚  | forward morphism — bidirectional arrow
--   [9] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [10] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [11] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [13] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [14] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [15] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [16] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [17] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [18] IMSCRIB   gram   := 𐑠               𐑙 → 𐑳  | identity — self-imscription
--   [19] ENGAGR    stoi   := 𐑳               𐑠 → 𐑱  | engage paradox — B-state, both arms
--   [20] CLINK     fid    := 𐑱               𐑳 → 𐑭  | composition — regime coherence
--   [21] IFIX      prot   := 𐑭               𐑱 → 𐑡  | irreversible fixation — winding number
--   [22] TANCH     top    := 𐑡               𐑭 → 𐑠  | terminal object — connectivity boundary
--   [23] IMSCRIB   gram   := 𐑠               𐑡 → 𐑼  | identity — self-imscription

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def seal_the_system_with_the_crystal_constraint_manifold_boundary_protocol : IGProtocol 𐑼 𐑠 :=
  .withGram .measure <|
  -- Seq chain:
  (.arrow 𐑼 𐑼 𐑚)  -- [0] VINIT | dim := 𐑼 | initial object — ground of distinction (Genesis before any IG primitives or classification capacity)
  -- FSPLIT [1] (gran := 𐑚) (Branch 12 primitives into three families D(3), T(5), P(4) by type) / FFUSE [6] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (4 nodes)
      ((.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow 𐑱 𐑚 𐑙))
      -- F-branch (0 nodes)
      (.refl 𐑙)
    -- reconnect at FFUSE [6]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 ⊙)
  ▷ (.arrow ⊙ 𐑙 𐑾)  -- [7] EVALT | crit := ⊙ | evaluate-true — criticality gate open (Verify SIC frame potential F = 143/169 = 0.846154 achieved)
  ▷ (.arrow 𐑾 ⊙ 𐑚)  -- [8] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Project onto 12-dimensional primitive measurement space ℂ¹²)
  -- FSPLIT [9] (gran := 𐑚) (Binary split into T-branch (SIC-conforming) and F-branch (heteroskedastic)) / FFUSE [12] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (1 nodes)
      (.arrow ⊙ 𐑚 𐑙)
      -- F-branch (1 nodes)
      (.arrow 𐑖 𐑚 𐑙)
    -- reconnect at FFUSE [12]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑙)
  -- FSPLIT [13] (gran := 𐑚) (Second split on composite structure H₇⊗H₁₂ from Shavian alphabet × IG primitives) / FFUSE [17] (stoi := 𐑙)
  ▷ mkFSplit
      -- T-branch (3 nodes)
      ((.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow 𐑾 𐑚 𐑙) ▷
      (.arrow 𐑱 𐑚 𐑙))
      -- F-branch (0 nodes)
      (.refl 𐑙)
    -- reconnect at FFUSE [17]: μ closes the Frobenius pair
      (.arrow 𐑙 𐑙 𐑠)
  ▷ (.arrow 𐑠 𐑙 𐑳)  -- [18] IMSCRIB | gram := 𐑠 | identity — self-imscription (Criticality primitive ⊙ recognizes self-fixed-point where μ∘δ=id)
  ▷ (.arrow 𐑳 𐑠 𐑱)  -- [19] ENGAGR | stoi := 𐑳 | engage paradox — B-state, both arms (Hold both SIC fiducial and paraconsistent measurement simultaneously)
  ▷ (.arrow 𐑱 𐑳 𐑭)  -- [20] CLINK | fid := 𐑱 | composition — regime coherence (Compose all branches back through Frobenius closure conditions)
  ▷ (.arrow 𐑭 𐑱 𐑡)  -- [21] IFIX | prot := 𐑭 | irreversible fixation — winding number (Permanently record the 17,280,000-element Crystal constraint manifold)
  ▷ (.arrow 𐑡 𐑭 𐑠)  -- [22] TANCH | top := 𐑡 | terminal object — connectivity boundary (Seal entire system within Crystal boundary preventing escape from valid types)
  ▷ (.arrow 𐑠 𐑡 𐑼)  -- [23] IMSCRIB | gram := 𐑠 | identity — self-imscription (Close self-referential loop where IG measures itself via fiducial state)
  --
-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

-- truth arm
noncomputable def seal_the_system_with_the_crystal_constraint_manifold_boundary_true_arm : IGProtocol 𐑼 𐑠 :=
  (seal_the_system_with_the_crystal_constraint_manifold_boundary_protocol).restrictToEVALT
  --
-- false arm
noncomputable def seal_the_system_with_the_crystal_constraint_manifold_boundary_false_arm : IGProtocol 𐑼 𐑠 :=
  (seal_the_system_with_the_crystal_constraint_manifold_boundary_protocol).restrictToEVALF
  --
-- ── Verification theorems ───────────────────────────────────────────────────

theorem seal_the_system_with_the_crystal_constraint_manifold_boundary_tier : TierFunctor.obj 𐑼 = .O₂ := by decide

end Imscribing
