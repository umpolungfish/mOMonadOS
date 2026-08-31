-- Imscribing/PowerMath/CasasAlveroTheorem.lean
-- Theorem: Casas-Alvero Theorem
-- Domain: contain
-- Opcode sequence: VINIT → AFWD → AFWD → AFWD → CLINK → CLINK → CLINK → CLINK → FSPLIT → EVALT → AFWD → IFIX → FFUSE → EVALF → AREV → IFIX → FFUSE → ENGAGR → IMSCRIB → TANCH
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

/-- The imscription of the Casas-Alvero Theorem. -/
def CasasAlveroTheoremImscription : Imscription := {
  dim  := array,
  top  := judge,
  rel  := ado,
  pol  := church,
  fid  := age,
  kin  := yea,
  gran := bib,
  gram := vow,
  crit := woe,
  chir := fee,
  stoi := hung,
  prot := awe
}

/-- Tier theorem: the type's ouroboricity tier. -/
theorem CasasAlveroTheorem_tier : TierFunctor.obj CasasAlveroTheoremImscription = .O₀ := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem CasasAlveroTheorem_frobenius :
    μ_A (δ_A CasasAlveroTheoremImscription).1 (δ_A CasasAlveroTheoremImscription).2 = CasasAlveroTheoremImscription :=
  mu_delta_A_id CasasAlveroTheoremImscription

/-- Consciousness score theorem. -/
theorem CasasAlveroTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore CasasAlveroTheoremImscription = (0 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    CasasAlveroTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem CasasAlveroTheorem_grammar_closed : TierFunctor.obj CasasAlveroTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
