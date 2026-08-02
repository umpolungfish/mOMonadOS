-- Imscribing/PowerMath/LonelyRunnerTheorem.lean
-- Theorem: Lonely Runner Theorem
-- Domain: combin
-- Opcode sequence: VINIT → TANCH → AFWD → AFWD → AFWD → CLINK → CLINK → FSPLIT → EVALT → EVALF → AREV → CLINK → ENGAGR → AFWD → CLINK → FSPLIT → EVALT → EVALF → IFIX → IMSCRIB → AREV → CLINK → FSPLIT → EVALT → EVALF → AFWD → CLINK → IFIX → IMSCRIB → CLINK → FSPLIT → EVALT → EVALF → AFWD → CLINK → ENGAGR → AREV → CLINK → IFIX → FFUSE → IMSCRIB → TANCH
-- Author: Lando⊗⊙perator

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.IGFunctor
import Imscribing.Frobenius
import Imscribing.Consciousness

namespace Imscribing.PowerMath
open Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality
open Imscribing
open Imscribing.Frobenius

/-- The imscription of the Lonely Runner Theorem. -/
def LonelyRunnerTheoremImscription : Imscription := {
  dim  := array,
  top  := mime,
  rel  := ian,
  pol  := or',
  fid  := peep,
  kin  := egg,
  gran := thigh,
  gram := measure,
  crit := roar,
  chir := sure,
  stoi := up,
  prot := ah
}

/-- Tier theorem: the type's ouroboricity tier. -/
theorem LonelyRunnerTheorem_tier : TierFunctor.obj LonelyRunnerTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem LonelyRunnerTheorem_frobenius :
    μ_A (δ_A LonelyRunnerTheoremImscription).1 (δ_A LonelyRunnerTheoremImscription).2 = LonelyRunnerTheoremImscription :=
  mu_delta_A_id LonelyRunnerTheoremImscription

/-- Consciousness score theorem. -/
theorem LonelyRunnerTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore LonelyRunnerTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    LonelyRunnerTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem LonelyRunnerTheorem_grammar_closed : TierFunctor.obj LonelyRunnerTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
