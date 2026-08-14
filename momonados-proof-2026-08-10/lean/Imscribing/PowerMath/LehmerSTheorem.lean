-- Imscribing/PowerMath/LehmerSTheorem.lean
-- Theorem: Lehmer's Theorem
-- Domain: divisible
-- Opcode sequence: VINIT → IMSCRIB → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → CLINK → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → CLINK → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → CLINK → ENGAGR → FSPLIT → EVALT → AFWD → IFIX → AREV → EVALF → FFUSE → IMSCRIB → TANCH
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

/-- The imscription of the Lehmer's Theorem. -/
def LehmerSTheoremImscription : Imscription := {
  dim  := if',
  top  := oil,
  rel  := ear,
  pol  := or',
  fid  := peep,
  kin  := egg,
  gran := ice,
  gram := measure,
  crit := roar,
  chir := wool,
  stoi := up,
  prot := ah
}

/-- Tier theorem: the type's ouroboricity tier. -/
theorem LehmerSTheorem_tier : TierFunctor.obj LehmerSTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem LehmerSTheorem_frobenius :
    μ_A (δ_A LehmerSTheoremImscription).1 (δ_A LehmerSTheoremImscription).2 = LehmerSTheoremImscription :=
  mu_delta_A_id LehmerSTheoremImscription

/-- Consciousness score theorem. -/
theorem LehmerSTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore LehmerSTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    LehmerSTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem LehmerSTheorem_grammar_closed : TierFunctor.obj LehmerSTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
