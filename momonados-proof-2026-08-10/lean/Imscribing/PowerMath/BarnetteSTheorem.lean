-- Imscribing/PowerMath/BarnetteSTheorem.lean
-- Theorem: Barnette's Theorem
-- Domain: contain
-- Opcode sequence: VINIT → TANCH → AFWD → CLINK → FSPLIT → EVALT → CLINK → IFIX → AREV → EVALF → CLINK → ENGAGR → AREV → FSPLIT → EVALT → CLINK → IMSCRIB → FFUSE → IFIX → TANCH
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

/-- The imscription of the Barnette's Theorem. -/
def BarnetteSTheoremImscription : Imscription := {
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
theorem BarnetteSTheorem_tier : TierFunctor.obj BarnetteSTheoremImscription = .O₀ := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem BarnetteSTheorem_frobenius :
    μ_A (δ_A BarnetteSTheoremImscription).1 (δ_A BarnetteSTheoremImscription).2 = BarnetteSTheoremImscription :=
  mu_delta_A_id BarnetteSTheoremImscription

/-- Consciousness score theorem. -/
theorem BarnetteSTheorem_consciousness :
    Imscribing.Consciousness.consciousnessScore BarnetteSTheoremImscription = (0 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    BarnetteSTheoremImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem BarnetteSTheorem_grammar_closed : TierFunctor.obj BarnetteSTheoremImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
