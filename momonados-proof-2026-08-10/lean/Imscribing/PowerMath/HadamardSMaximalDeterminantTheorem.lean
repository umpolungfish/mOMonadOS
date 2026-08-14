-- Imscribing/PowerMath/HadamardSMaximalDeterminantTheorem.lean
-- Theorem: Hadamard's Maximal Determinant Theorem
-- Domain: inequal
-- Opcode sequence: VINIT → TANCH → AFWD → CLINK → FSPLIT → EVALT → AFWD → EVALF → AREV → FFUSE → IMSCRIB → CLINK → EVALT → IFIX → ENGAGR → AREV → IMSCRIB → CLINK → TANCH
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

/-- The imscription of the Hadamard's Maximal Determinant Theorem. -/
def HadamardSMaximalDeterminantTheoremImscription : Imscription := {
  dim  := array,
  top  := mime,
  rel  := ian,
  pol  := or',
  fid  := peep,
  kin  := egg,
  gran := ice,
  gram := measure,
  crit := roar,
  chir := sure,
  stoi := up,
  prot := ah
}

/-- Tier theorem: the type's ouroboricity tier. -/
theorem HadamardSMaximalDeterminantTheorem_tier : TierFunctor.obj HadamardSMaximalDeterminantTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem HadamardSMaximalDeterminantTheorem_frobenius :
    μ_A (δ_A HadamardSMaximalDeterminantTheoremImscription).1 (δ_A HadamardSMaximalDeterminantTheoremImscription).2 = HadamardSMaximalDeterminantTheoremImscription :=
  mu_delta_A_id HadamardSMaximalDeterminantTheoremImscription

/-- Consciousness score theorem. -/
theorem HadamardSMaximalDeterminantTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore HadamardSMaximalDeterminantTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    HadamardSMaximalDeterminantTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem HadamardSMaximalDeterminantTheorem_grammar_closed : TierFunctor.obj HadamardSMaximalDeterminantTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
