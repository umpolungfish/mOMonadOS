-- IGProtocol scaffold: VINIT → TANCH → AFWD → AREV → CLINK → IMSCRIB → FSPLIT → EVALT → AFWD → CLINK → EVALF → AREV → CLINK → FFUSE → ENGAGR → IFIX → CLINK → AFWD → AREV → CLINK → IMSCRIB → IFIX → TANCH → CLINK → IMSCRIB → IFIX → AFWD → AREV → CLINK → IMSCRIB → TANCH
-- Class: serotinous cone mechanism
-- Fingerprint: sig=(23,2,3,3)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=31
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(6, 13)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑡  | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               𐑼 → 𐑾  | terminal object — connectivity boundary
--   [2] AFWD      rel    := 𐑾               𐑡 → 𐑗  | forward morphism — bidirectional arrow
--   [3] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [4] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [5] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [9] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [12] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [13] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [14] ENGAGR    stoi   := 𐑳               𐑙 → 𐑭  | engage paradox — B-state, both arms
--   [15] IFIX      prot   := 𐑭               𐑳 → 𐑱  | irreversible fixation — winding number
--   [16] CLINK     fid    := 𐑱               𐑭 → 𐑾  | composition — regime coherence
--   [17] AFWD      rel    := 𐑾               𐑱 → 𐑗  | forward morphism — bidirectional arrow
--   [18] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [19] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [20] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [21] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [22] TANCH     top    := 𐑡               𐑭 → 𐑱  | terminal object — connectivity boundary
--   [23] CLINK     fid    := 𐑱               𐑡 → 𐑠  | composition — regime coherence
--   [24] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [25] IFIX      prot   := 𐑭               𐑠 → 𐑾  | irreversible fixation — winding number
--   [26] AFWD      rel    := 𐑾               𐑭 → 𐑗  | forward morphism — bidirectional arrow
--   [27] AREV      pol    := 𐑗               𐑾 → 𐑱  | reverse morphism — parity flip
--   [28] CLINK     fid    := 𐑱               𐑗 → 𐑠  | composition — regime coherence
--   [29] IMSCRIB   gram   := 𐑠               𐑱 → 𐑡  | identity — self-imscription
--   [30] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def serotinous_cone_mechanism_protocol : IGProtocol 𐑼 𐑡 :=
  .withGram 𐑠 <|
  -- Seq chain:
  (.arrow 𐑼 𐑼 𐑡)  -- [0] VINIT | dim := 𐑼 | initial object — ground of distinction (Genesis of unformed seed mass in void)
  (.arrow 𐑡 𐑼 𐑾)  -- [1] TANCH | top := 𐑡 | terminal object — connectivity boundary (Establishment of mature serotinous cone boundary)
  (.arrow 𐑾 𐑡 𐑗)  -- [2] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Approach of thermal energy toward cone surface)
  (.arrow 𐑗 𐑾 𐑱)  -- [3] AREV | pol := 𐑗 | reverse morphism — parity flip (Ambient cooling maintaining dormant contraction)
  (.arrow 𐑱 𐑗 𐑠)  -- [4] CLINK | fid := 𐑱 | composition — regime coherence (Sequential heat transfer to resin-sealed scales)
  (.arrow 𐑠 𐑱 𐑚)  -- [5] IMSCRIB | gram := 𐑠 | identity — self-imscription (Self-recognition of inherent serotinous trait)
  -- FSPLIT [6] (gran := 𐑚) (Threshold evaluation of fire intensity) / FFUSE [13] (stoi := 𐑙)
  .seq
    (.prod
      -- T-branch (3 nodes)
      .seq
        (.arrow ⊙ 𐑚 𐑙)  -- [7] EVALT | crit := ⊙ | evaluate-true — criticality gate open (Heat exceeds resin melting point on T-arm)
      .seq
        (.arrow 𐑾 𐑚 𐑙)  -- [8] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Resin phase transition to fluid state)
        (.arrow 𐑱 𐑚 𐑙)  -- [9] CLINK | fid := 𐑱 | composition — regime coherence (Thermal expansion propagates to scale joints)
      -- F-branch (3 nodes)
      .seq
        (.arrow 𐑖 𐑚 𐑙)  -- [10] EVALF | chir := 𐑖 | evaluate-false — chirality check (Heat remains below critical threshold on F-arm)
      .seq
        (.arrow 𐑗 𐑚 𐑙)  -- [11] AREV | pol := 𐑗 | reverse morphism — parity flip (Resin maintains solid viscosity on F-arm)
        (.arrow 𐑱 𐑚 𐑙)  -- [12] CLINK | fid := 𐑱 | composition — regime coherence (Structural tension remains dormant on F-arm))
    -- reconnect at FFUSE [13]: μ closes the Frobenius pair
    (.arrow 𐑙 𐑙 𐑳)  -- [13] FFUSE | stoi := 𐑙 (Reintegration of thermal outcome into cone state)
  (.arrow 𐑳 𐑙 𐑭)  -- [14] ENGAGR | stoi := 𐑳 | engage paradox — B-state, both arms (Simultaneous tension and release potential held)
  (.arrow 𐑭 𐑳 𐑱)  -- [15] IFIX | prot := 𐑭 | irreversible fixation — winding number (Permanent seed dispersal record fixed)
  (.arrow 𐑱 𐑭 𐑾)  -- [16] CLINK | fid := 𐑱 | composition — regime coherence (Ecological succession chain initiated)
  (.arrow 𐑾 𐑱 𐑗)  -- [17] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Seed germination forward morphism)
  (.arrow 𐑗 𐑾 𐑱)  -- [18] AREV | pol := 𐑗 | reverse morphism — parity flip (Soil nutrient descent into root zone)
  (.arrow 𐑱 𐑗 𐑠)  -- [19] CLINK | fid := 𐑱 | composition — regime coherence (Root-soil composition and nutrient uptake)
  (.arrow 𐑠 𐑱 𐑭)  -- [20] IMSCRIB | gram := 𐑠 | identity — self-imscription (Self-referential life cycle closure)
  (.arrow 𐑭 𐑠 𐑡)  -- [21] IFIX | prot := 𐑭 | irreversible fixation — winding number (Final ecological baseline fixation)
  (.arrow 𐑡 𐑭 𐑱)  -- [22] TANCH | top := 𐑡 | terminal object — connectivity boundary (System containment verification complete)
  (.arrow 𐑱 𐑡 𐑠)  -- [23] CLINK | fid := 𐑱 | composition — regime coherence (Long-term forest regeneration composition)
  (.arrow 𐑠 𐑱 𐑭)  -- [24] IMSCRIB | gram := 𐑠 | identity — self-imscription (Generational identity preservation)
  (.arrow 𐑭 𐑠 𐑾)  -- [25] IFIX | prot := 𐑭 | irreversible fixation — winding number (Historical fire-ecology record append)
  (.arrow 𐑾 𐑭 𐑗)  -- [26] AFWD | rel := 𐑾 | forward morphism — bidirectional arrow (Canopy light penetration forward morphism)
  (.arrow 𐑗 𐑾 𐑱)  -- [27] AREV | pol := 𐑗 | reverse morphism — parity flip (Leaf litter decomposition descent)
  (.arrow 𐑱 𐑗 𐑠)  -- [28] CLINK | fid := 𐑱 | composition — regime coherence (Soil organic matter composition)
  (.arrow 𐑠 𐑱 𐑡)  -- [29] IMSCRIB | gram := 𐑠 | identity — self-imscription (Ecosystem self-recognition)
  (.arrow 𐑡 𐑠 𐑼)  -- [30] TANCH | top := 𐑡 | terminal object — connectivity boundary (Closed boundary of mature forest state)

-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

-- truth arm
noncomputable def serotinous_cone_mechanism_true_arm : IGProtocol 𐑼 𐑡 :=
  (serotinous_cone_mechanism_protocol).restrictToEVALT

-- false arm
noncomputable def serotinous_cone_mechanism_false_arm : IGProtocol 𐑼 𐑡 :=
  (serotinous_cone_mechanism_protocol).restrictToEVALF

-- ── Verification theorems ───────────────────────────────────────────────────

theorem serotinous_cone_mechanism_tier : TierFunctor.obj 𐑼 = .O₂ := by decide

-- Frobenius (split → fuse): μ∘δ = id on .prod branch
-- Proof: apply igFrobAlg_self_fusion; exact mu_delta_A_id
-- (requires mu_delta_A_id from IGFunctor library)

end Imscribing
