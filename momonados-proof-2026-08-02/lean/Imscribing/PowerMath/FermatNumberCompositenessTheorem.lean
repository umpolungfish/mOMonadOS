-- Imscribing/PowerMath/FermatNumberCompositenessTheorem.lean
-- Theorem: Fermat Number Compositeness Theorem
-- Domain: divisible
-- Opcode sequence: VINIT → TANCH → IMSCRIB → AFWD → CLINK → CLINK → CLINK → FSPLIT → EVALT → IFIX → FFUSE → AREV → EVALF → IFIX → FFUSE → ENGAGR → IMSCRIB → CLINK → TANCH
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

/-- The imscription of the Fermat Number Compositeness Theorem. -/
def FermatNumberCompositenessTheoremImscription : Imscription := {
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
theorem FermatNumberCompositenessTheorem_tier : TierFunctor.obj FermatNumberCompositenessTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem FermatNumberCompositenessTheorem_frobenius :
    μ_A (δ_A FermatNumberCompositenessTheoremImscription).1 (δ_A FermatNumberCompositenessTheoremImscription).2 = FermatNumberCompositenessTheoremImscription :=
  mu_delta_A_id FermatNumberCompositenessTheoremImscription

/-- Consciousness score theorem. -/
theorem FermatNumberCompositenessTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore FermatNumberCompositenessTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    FermatNumberCompositenessTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem FermatNumberCompositenessTheorem_grammar_closed : TierFunctor.obj FermatNumberCompositenessTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
