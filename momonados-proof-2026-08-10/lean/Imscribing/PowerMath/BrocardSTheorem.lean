-- Imscribing/PowerMath/BrocardSTheorem.lean
-- Theorem: Brocard's Theorem
-- Domain: divisible
-- Opcode sequence: VINIT → TANCH → AFWD → AFWD → AFWD → CLINK → IMSCRIB → FSPLIT → EVALT → AREV → AREV → AREV → EVALF → FFUSE → ENGAGR → IMSCRIB → IFIX → CLINK → IMSCRIB
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

/-- The imscription of the Brocard's Theorem. -/
def BrocardSTheoremImscription : Imscription := {
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
theorem BrocardSTheorem_tier : TierFunctor.obj BrocardSTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem BrocardSTheorem_frobenius :
    μ_A (δ_A BrocardSTheoremImscription).1 (δ_A BrocardSTheoremImscription).2 = BrocardSTheoremImscription :=
  mu_delta_A_id BrocardSTheoremImscription

/-- Consciousness score theorem. -/
theorem BrocardSTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore BrocardSTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    BrocardSTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem BrocardSTheorem_grammar_closed : TierFunctor.obj BrocardSTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
