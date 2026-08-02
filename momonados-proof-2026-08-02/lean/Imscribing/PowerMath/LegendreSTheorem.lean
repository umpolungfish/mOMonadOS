-- Imscribing/PowerMath/LegendreSTheorem.lean
-- Theorem: Legendre's Theorem
-- Domain: divisible
-- Opcode sequence: VINIT → TANCH → FSPLIT → EVALT → AFWD → CLINK → IFIX → EVALF → AREV → IFIX → ENGAGR → IMSCRIB → FFUSE → CLINK → IMSCRIB → TANCH
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

/-- The imscription of the Legendre's Theorem. -/
def LegendreSTheoremImscription : Imscription := {
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
theorem LegendreSTheorem_tier : TierFunctor.obj LegendreSTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem LegendreSTheorem_frobenius :
    μ_A (δ_A LegendreSTheoremImscription).1 (δ_A LegendreSTheoremImscription).2 = LegendreSTheoremImscription :=
  mu_delta_A_id LegendreSTheoremImscription

/-- Consciousness score theorem. -/
theorem LegendreSTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore LegendreSTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    LegendreSTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem LegendreSTheorem_grammar_closed : TierFunctor.obj LegendreSTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
