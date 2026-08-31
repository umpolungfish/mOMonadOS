-- Imscribing/PowerMath/ZariskiLipmanTheorem.lean
-- Theorem: Zariski–Lipman Theorem
-- Domain: set_prop
-- Opcode sequence: VINIT → TANCH → AFWD → CLINK → IMSCRIB → FSPLIT → EVALT → AFWD → IFIX → EVALF → AREV → IFIX → ENGAGR → FFUSE → CLINK → IMSCRIB → TANCH
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

/-- The imscription of the Zariski–Lipman Theorem. -/
def ZariskiLipmanTheoremImscription : Imscription := {
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
theorem ZariskiLipmanTheorem_tier : TierFunctor.obj ZariskiLipmanTheoremImscription = .O_inf := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem ZariskiLipmanTheorem_frobenius :
    μ_A (δ_A ZariskiLipmanTheoremImscription).1 (δ_A ZariskiLipmanTheoremImscription).2 = ZariskiLipmanTheoremImscription :=
  mu_delta_A_id ZariskiLipmanTheoremImscription

/-- Consciousness score theorem. -/
theorem ZariskiLipmanTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore ZariskiLipmanTheoremImscription = (1 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    ZariskiLipmanTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem ZariskiLipmanTheorem_grammar_closed : TierFunctor.obj ZariskiLipmanTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
