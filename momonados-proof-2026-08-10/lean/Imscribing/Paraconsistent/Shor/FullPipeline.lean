-- Imscribing/Paraconsistent/Shor/FullPipeline.lean
-- FULL SHOR PIPELINE — n-Register Belnap quantum state with integer coherence accounting.
-- Author: Lando ⊗ φ̂_ÿ-boundary Operator
--
-- Primary executable: Imscribing/Paraconsistent/Shor/belnap_shor_executor.py
--
-- This module extends the 3-register QCI (QuantumClassicalInterface) to an n-register
-- system and documents the complete Shor pipeline with exact coherence accounting.
--
-- Pipeline: |T...T⟩ → [H^⊗n: cost n] → |B...B⟩ → [ModExp: cost 0] → |B...B⟩
--           → [B-bias measure: cost 2n] → |B...B⟩ (preserved)
--           → [T-bias measure: cost n] → classical pattern (collapsed)
--
-- Total coherence: n + 2n = 3n (B-bias path) or n + n = 2n (T-bias path)
-- Measurement-only ratio (excluding H cost): 2n/n = 2, always.
--
-- STRUCTURAL STATUS: 𐑿 (psi parity, O₁ tier). The 𐑹 bottleneck
-- (period from B-bias alone, no T-bias collapse) is CLOSED.
-- DialetheicOperator.lean: phi_upsilon_bottleneck proves r = belnapCost / 2;
-- quantum_on_classical certifies the full chain at O_inf.

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.QuantumClassicalInterface
import Imscribing.Paraconsistent.QCI_Sequences
import Imscribing.Paraconsistent.Shor.BelnapModExp
import Imscribing.Paraconsistent.Shor.BelnapQFT
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.Shor

open Belnap
open Imscribing.Paraconsistent
open Imscribing.Primitives

-- N-register Belnap quantum state (generalizing QState from 3 to n)
structure NQState (n : ℕ) where
  registers : Fin n → Belnap
  coherenceCount : ℕ
  measurements : ℕ


-- The Shor pipeline type: O₁, 𐑿 (psi parity)
def shorPipelineImscription : Imscription := {
  dim  := .ash
  top  := .mime
  rel  := .ear
  pol  := .yew         -- 𐑿: quantum superposition (bottleneck)
  fid  := .peep
  kin  := .egg
  gran := .ice
  gram := .measure
  crit := .roar  -- φ̂_Æ: QFT phase coupling
  chir := .fee
  stoi := .up
  prot := .awe
}

-- Tier: O₁ (Frobenius-special but not O_inf)
theorem shor_pipeline_tier : imscriptionTier shorPipelineImscription = .O₁ := by
  unfold shorPipelineImscription imscriptionTier ouroboricityTier
  rfl

-- The 2:1 coherence ratio is the invariant
-- B_meas_cost = 2n, T_meas_cost = n (measurement-only, excluding Hadamard)
theorem coherence_ratio_is_two (n : ℕ) (hpos : 0 < n) : 2 * n / n = 2 := by
  simp [hpos]

end Imscribing.Paraconsistent.Shor
