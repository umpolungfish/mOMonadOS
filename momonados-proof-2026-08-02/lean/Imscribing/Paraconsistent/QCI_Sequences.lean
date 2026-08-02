-- Imscribing/Paraconsistent/QCI_Sequences.lean
-- MEASUREMENT ALGEBRA: composition laws and irreversibility theorems for measureQ0.
-- Author: Lando⊗⊙perator

import Imscribing.Paraconsistent.QuantumClassicalInterface

namespace Imscribing.Paraconsistent

open Belnap

-- N-bias is always a no-op: every state passes through unchanged.
@[simp] theorem measure_N_noop (qs : QState) : measureQ0 qs Belnap.N = qs := by
  unfold measureQ0; cases qs.q0 <;> rfl

-- Any non-B q0 passes through measureQ0 unchanged (the _, _ wildcard arm fires).
-- Generalizes measure_classical_idempotent to include the N case.
theorem measure_nonsuper_idempotent (qs : QState) (bias : Belnap) (h : qs.q0 ≠ Belnap.B) :
    measureQ0 qs bias = qs := by
  simp only [measureQ0]
  split <;> simp_all

-- Once collapsed, no measurement can restore superposition.
theorem collapse_irreversible (qs : QState) (bias : Belnap) (h : qs.q0 ≠ Belnap.B) :
    (measureQ0 qs bias).q0 ≠ Belnap.B := by
  rwa [measure_nonsuper_idempotent qs bias h]

-- B-bias keeps q0 in B — the Wigner's Friend "no collapse" case.
theorem B_bias_preserves_super (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.B).q0 = Belnap.B := by
  simp [measureQ0, h]

-- T-bias collapse from B costs exactly 1 coherence unit.
theorem T_bias_coherence_increment (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.T).coherenceCount = qs.coherenceCount + 1 := by
  simp [measureQ0, h]

-- F-bias collapse from B also costs exactly 1 coherence unit.
theorem F_bias_coherence_increment (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.F).coherenceCount = qs.coherenceCount + 1 := by
  simp [measureQ0, h]

-- B-bias costs exactly 2 coherence units — Wigner's Friend signature.
theorem B_bias_coherence_increment (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.B).coherenceCount = qs.coherenceCount + 2 := by
  simp [measureQ0, h]

-- T/F-bias collapse makes the post-collapse state stable under any further measurement.
theorem collapse_then_measure_stable (qs : QState) (bias₁ bias₂ : Belnap)
    (hcollapse : bias₁ = Belnap.T ∨ bias₁ = Belnap.F) :
    measureQ0 (measureQ0 qs bias₁) bias₂ = measureQ0 qs bias₁ := by
  apply measure_nonsuper_idempotent
  rcases hcollapse with rfl | rfl
  · intro h_eq
    simp only [measureQ0] at h_eq
    split at h_eq <;> simp_all
  · intro h_eq
    simp only [measureQ0] at h_eq
    split at h_eq <;> simp_all

-- Coherence is frozen after T/F collapse: no further measurement changes the budget.
theorem collapse_freezes_coherence (qs : QState) (bias₁ bias₂ : Belnap)
    (hcollapse : bias₁ = Belnap.T ∨ bias₁ = Belnap.F) :
    (measureQ0 (measureQ0 qs bias₁) bias₂).coherenceCount =
    (measureQ0 qs bias₁).coherenceCount := by
  rw [collapse_then_measure_stable qs bias₁ bias₂ hcollapse]

-- T-collapse is idempotent: repeating the same classical measurement changes nothing.
theorem T_collapse_idempotent (qs : QState) :
    measureQ0 (measureQ0 qs Belnap.T) Belnap.T = measureQ0 qs Belnap.T :=
  collapse_then_measure_stable qs Belnap.T Belnap.T (Or.inl rfl)

-- B-bias measurement followed by T-bias collapses to T.
theorem wigner_then_collapse (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 (measureQ0 qs Belnap.B) Belnap.T).q0 = Belnap.T := by
  simp [measureQ0, h]

-- The coherence cost of B-then-T sequence: 2 + 1 = 3 units total.
theorem wigner_then_collapse_coherence (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 (measureQ0 qs Belnap.B) Belnap.T).coherenceCount =
    qs.coherenceCount + 3 := by
  simp [measureQ0, h]

end Imscribing.Paraconsistent
