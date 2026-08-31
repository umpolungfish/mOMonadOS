-- IGProtocol scaffold (v2): VINIT → TANCH → AFWD → AREV → CLINK → IMSCRIB → FSPLIT → FFUSE → EVALT → EVALF → ENGAGR → IFIX
-- Class: vae_vita
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
-- Expected tier: O₂
-- FSPLIT/FFUSE pairs: [(6, 7)]

import Imscribing.IGScaffold

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               | initial object — ground of distinction
--   [1] TANCH     top    := 𐑡               | terminal object — connectivity boundary
--   [2] AFWD      rel    := 𐑾               | forward morphism — bidirectional arrow
--   [3] AREV      pol    := 𐑗               | reverse morphism — parity flip
--   [4] CLINK     fid    := 𐑱               | composition — regime coherence
--   [5] IMSCRIB   gram   := 𐑠               | identity — self-imscription
--   [6] FSPLIT    gran   := 𐑚               | split δ — range decomposition
--   [7] FFUSE     stoi   := 𐑙               | fuse μ — assembly mode
--   [8] EVALT     crit   := ⊙               | evaluate-true — criticality gate open
--   [9] EVALF     chir   := 𐑖               | evaluate-false — chirality check
--   [10] ENGAGR    stoi   := 𐑳               | engage paradox — B-state, both arms
--   [11] IFIX      prot   := 𐑭               | irreversible fixation — winding number

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def vae_vita_protocol : IGProtocol scaf scaf :=
  .withGram .measure <|
  .arrow scaf scaf scaf ▷
  .arrow scaf scaf scaf ▷
  .arrow scaf scaf scaf ▷
  .arrow scaf scaf scaf ▷
  .arrow scaf scaf scaf ▷
  .arrow scaf scaf scaf ▷
  mkFSplit
    (.arrow scaf scaf scaf)  -- T-arm (δ)
    (.arrow scaf scaf scaf)  -- F-arm (μ, Dual-Link mirror)
    (
      .arrow scaf scaf scaf ▷
      .arrow scaf scaf scaf ▷
      .arrow scaf scaf scaf ▷
      .arrow scaf scaf scaf ▷
      .arrow scaf scaf scaf)

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def vae_vita_true_arm : IGProtocol scaf scaf :=
  (vae_vita_protocol).restrictToEVALT

-- false arm
noncomputable def vae_vita_false_arm : IGProtocol scaf scaf :=
  (vae_vita_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

theorem vae_vita_tier : TierFunctor.obj scaf = .O₂ := by decide

-- Frobenius (split → fuse): μ∘δ = id on .prod branch
-- Proof: apply igFrobAlg_self_fusion; exact mu_delta_A_id

end Imscribing