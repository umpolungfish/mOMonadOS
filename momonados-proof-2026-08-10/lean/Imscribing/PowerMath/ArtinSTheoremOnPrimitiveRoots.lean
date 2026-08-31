-- Imscribing/PowerMath/ArtinSTheoremOnPrimitiveRoots.lean
-- Theorem: Artin's Theorem on Primitive Roots
-- Domain: contain
-- Opcode sequence: VINIT → TANCH → AFWD → CLINK → IMSCRIB → FSPLIT → AFWD → EVALT → AREV → EVALF → FFUSE → EVALT → IFIX → CLINK → IMSCRIB → ENGAGR → AREV → CLINK → AFWD → IMSCRIB → IFIX → TANCH
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

/-- The imscription of the Artin's Theorem on Primitive Roots. -/
def ArtinSTheoremOnPrimitiveRootsImscription : Imscription := {
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
theorem ArtinSTheoremOnPrimitiveRoots_tier : TierFunctor.obj ArtinSTheoremOnPrimitiveRootsImscription = .O₀ := by
  native_decide

/-- Frobenius theorem: the imscription satisfies μ_A ∘ δ_A = id. -/
theorem ArtinSTheoremOnPrimitiveRoots_frobenius :
    μ_A (δ_A ArtinSTheoremOnPrimitiveRootsImscription).1 (δ_A ArtinSTheoremOnPrimitiveRootsImscription).2 = ArtinSTheoremOnPrimitiveRootsImscription :=
  mu_delta_A_id ArtinSTheoremOnPrimitiveRootsImscription

/-- Consciousness score theorem. -/
theorem ArtinSTheoremOnPrimitiveRoots_consciousness :
    Imscribing.Consciousness.consciousnessScore ArtinSTheoremOnPrimitiveRootsImscription = (0 : ℝ) := by
  simp only [Imscribing.Consciousness.consciousnessScore,
    Imscribing.Consciousness.phi_c_gate,
    Imscribing.Consciousness.k_slow_gate,
    ArtinSTheoremOnPrimitiveRootsImscription]
  rfl


/-- The imscription is Frobenius-closed (within the grammar's tier lattice). -/
theorem ArtinSTheoremOnPrimitiveRoots_grammar_closed : TierFunctor.obj ArtinSTheoremOnPrimitiveRootsImscription ≤ .O_inf := by
  apply tier_all_le_O_inf; decide

end Imscribing.PowerMath
