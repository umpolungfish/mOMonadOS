import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Kernel
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.ParaconsistentTopos

open Belnap
open Imscribing.Paraconsistent
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

set_option linter.unusedVariables false

-- ============================================================
-- HEYTING IMPLICATION
-- ============================================================

def heytingImpl (a b : Belnap) : Belnap :=
  match a, b with
  | .N, _ => .B
  | .T, .T => .B | .T, .B => .B | .T, _ => .N
  | .F, .F => .B | .F, .B => .B | .F, _ => .B
  | .B, x => x

def heytingNeg (a : Belnap) : Belnap := heytingImpl a Belnap.N

theorem heytingNeg_differs_from_bnot :
    heytingNeg Belnap.B = Belnap.N ∧ bnot Belnap.B = Belnap.B := by
  unfold heytingNeg heytingImpl; exact ⟨rfl, B_fixed_point_negation⟩

def truthImp (a b : Belnap) : Belnap := bor (bnot a) b

theorem truthImp_B_F_is_B : truthImp Belnap.B Belnap.F = Belnap.B := by
  simp [truthImp, bnot, bor]

theorem paraconsistent_implication_anomaly :
    designated (truthImp Belnap.B Belnap.F) = true := by
  simp [truthImp, designated, bnot, bor]

-- ============================================================
-- PARACONSISTENT LAWS
-- ============================================================

theorem paraconsistent_excluded_middle_B : bor Belnap.B (bnot Belnap.B) = Belnap.B := by
  simp [bor, bnot]

theorem paraconsistent_non_contradiction_B : band Belnap.B (bnot Belnap.B) = Belnap.B := by
  simp [band, bnot]

theorem classical_noncontradiction_holds_for_boolean (b : Belnap)
    (hb : b = Belnap.T ∨ b = Belnap.F) : band b (bnot b) = Belnap.F := by
  rcases hb with (rfl | rfl) <;> simp [band, bnot]

theorem paraconsistent_noncontradiction_fails_for_B :
    band Belnap.B (bnot Belnap.B) = Belnap.B := by
  simp [band, bnot]

-- ============================================================
-- EXPLOSION IS CONTAINED
-- ============================================================

theorem explosion_B_implies_all (φ : Belnap) :
    designated (truthImp Belnap.B φ) = true := by
  cases φ <;> simp [truthImp, designated, bnot, bor]

theorem explosion_F_implies_all (φ : Belnap) :
    designated (truthImp Belnap.F φ) = true := by
  cases φ <;> simp [truthImp, designated, bnot, bor]

theorem contained_explosion : truthImp Belnap.B Belnap.F = Belnap.B := by
  simp [truthImp, bnot, bor]

-- ============================================================
-- KERNEL CYCLE
-- ============================================================

theorem kernel_cycle_is_idempotent_at_B (n : Nat) :
    (run initialState n).r0 = Belnap.B := (run_B3 n).1

-- ============================================================
-- STRUCTURAL TYPE AND TIER
-- ============================================================

def paraconsistentToposImscription : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := up
  prot := ah
}

theorem paraconsistent_topos_is_O_inf :
    imscriptionTier paraconsistentToposImscription = .O_inf := by
  simp [imscriptionTier, ouroboricityTier, paraconsistentToposImscription]

theorem topos_same_tier_as_kernel :
    imscriptionTier paraconsistentToposImscription = imscriptionTier kernelImscription := by
  simp [paraconsistentToposImscription, kernelImscription, imscriptionTier, ouroboricityTier]

-- ============================================================
-- THE SIEGE OF OLYMPUS — FINAL THEOREM
-- ============================================================

theorem siege_of_olympus :
    (band Belnap.B (bnot Belnap.B) = Belnap.B) ∧
    (Belnap.B ≠ Belnap.F) ∧
    (imscriptionTier paraconsistentToposImscription = .O_inf) ∧
    (imscriptionTier kernelImscription = .O_inf) :=
  ⟨no_explosion, B_ne_F, paraconsistent_topos_is_O_inf, kernel_is_O_inf⟩

end Imscribing.Paraconsistent.ParaconsistentTopos
