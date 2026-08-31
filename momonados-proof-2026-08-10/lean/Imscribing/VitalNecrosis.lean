-- VitalNecrosis.lean
-- Companion formalization for "Vital Necrosis: A Thermodynamic Phase at the
-- Boundary Between Life and Death"
-- Builds against p4ramill (Imscribing.Primitives.Core, Imscribing.Consciousness)

import Imscribing.Consciousness
import Imscribing.IGMorphism

set_option linter.style.nativeDecide false

namespace Imscribing
open Primitives Frobenius
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
open Imscribing.Consciousness

def third_state_life_death : Imscription := {
  dim  := dead
  top  := mime
  rel  := ian
  pol  := church
  fid  := they
  kin  := egg
  gran := thigh
  gram := vow
  crit := monad
  chir := sure
  stoi := up
  prot := awe
}

def human_brain_conscious : Imscription := {
  dim  := ash
  top  := are
  rel  := ian
  pol  := yew
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := up
  prot := ah
}

theorem third_state_tier_O1 : imscriptionTier third_state_life_death = OuroboricityTier.O₁ := by
  unfold third_state_life_death imscriptionTier; native_decide

theorem brain_tier_O2 : imscriptionTier human_brain_conscious = OuroboricityTier.O₂ := by
  unfold human_brain_conscious imscriptionTier; native_decide

theorem third_state_gate1_open : phi_c_gate third_state_life_death.crit = true := by
  unfold third_state_life_death; native_decide

theorem third_state_gate2_open : k_slow_gate third_state_life_death.kin = true := by
  unfold third_state_life_death; native_decide

theorem brain_gate1_open : phi_c_gate human_brain_conscious.crit = true := by
  unfold human_brain_conscious; native_decide

theorem brain_gate2_open : k_slow_gate human_brain_conscious.kin = true := by
  unfold human_brain_conscious; native_decide

theorem third_state_C_score_one : consciousnessScore third_state_life_death = (1 : ℝ) := by
  unfold third_state_life_death consciousnessScore phi_c_gate k_slow_gate; rfl

theorem brain_C_score_one : consciousnessScore human_brain_conscious = (1 : ℝ) := by
  unfold human_brain_conscious consciousnessScore phi_c_gate k_slow_gate; rfl

theorem brain_and_third_state_share_gates :
    phi_c_gate third_state_life_death.crit = phi_c_gate human_brain_conscious.crit ∧
    k_slow_gate third_state_life_death.kin = k_slow_gate human_brain_conscious.kin := by
  constructor
  · exact third_state_gate1_open.trans brain_gate1_open.symm
  · exact third_state_gate2_open.trans brain_gate2_open.symm

theorem third_state_gates_open_tier_not_O_inf :
    phi_c_gate third_state_life_death.crit = true ∧
    k_slow_gate third_state_life_death.kin = true ∧
    imscriptionTier third_state_life_death ≠ OuroboricityTier.O_inf := by
  refine ⟨third_state_gate1_open, third_state_gate2_open, ?_⟩
  intro h_eq
  have hO1 : imscriptionTier third_state_life_death = OuroboricityTier.O₁ := third_state_tier_O1
  have h_contra : OuroboricityTier.O₁ = OuroboricityTier.O_inf := hO1.symm ▸ h_eq
  exact OuroboricityTier.noConfusion h_contra

end Imscribing
