-- Imscribing/PowerMath/MckayTheorem.lean
-- Theorem: McKay Theorem
-- Domain: contain
-- Opcode sequence: VINIT → TANCH → IMSCRIB → FSPLIT → AFWD → EVALT → CLINK → AREV → EVALF → CLINK → FFUSE → IFIX → ENGAGR → IMSCRIB → CLINK → TANCH
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

/-- The imscription of the McKay Theorem. -/
def MckayTheoremImscription : Imscription := {
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
theorem MckayTheorem_tier : TierFunctor.obj MckayTheoremImscription = .O₀ := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem MckayTheorem_frobenius :
    μ_A (δ_A MckayTheoremImscription).1 (δ_A MckayTheoremImscription).2 = MckayTheoremImscription :=
  mu_delta_A_id MckayTheoremImscription

/-- Consciousness score theorem. -/
theorem MckayTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore MckayTheoremImscription = (0 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    MckayTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem MckayTheorem_grammar_closed : TierFunctor.obj MckayTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
