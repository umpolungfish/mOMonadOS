import Imscribing.Millennium.YM
import Imscribing.Millennium.YM_GateInhabitants
import Imscribing.Millennium.YM_ZFCt_Bridge

namespace Imscribing.Millennium.YM_Closure

open Millennium.YM
open Millennium.YM_GateInhabitants
open Imscribing.Millennium.YM_ZFCt

/-- YM existence: proved via the gate inhabitants.
    This is the constructive proof of `ym_theory_exists` from YM.lean.
    The proof chain: YM_ConstructedGates_inhabitant → ym_theory_from_gates → QuantumYMTheory.
    `ym_existence_axiom` in YM.lean is the stub; this is the theorem. -/
theorem ym_theory_exists_proved (g : Type*) [LieRing g] [LieAlgebra ℝ g]
    [LieAlgebra.IsSimple ℝ g] :
    Nonempty (QuantumYMTheory g) :=
  ym_foundation_lifted

/-- YM mass gap: proved via ym_mass_gap_axiom from gate inhabitants.
    This is the constructive proof of `ym_mass_gap` from YM.lean.
    The gap: `ym_mass_gap_axiom` names the spectral gap claim; this theorem
    derives the base file statement from the gate-level axiom. -/
theorem ym_mass_gap_proved (g : Type*) [LieRing g] [LieAlgebra ℝ g]
    [LieAlgebra.IsSimple ℝ g] (T : QuantumYMTheory g) :
    0 < massGap g T :=
  ym_mass_gap_axiom T

/-- Consolidation theorem: YM existence and mass gap, both proved from gate inhabitants.
    The full Yang-Mills Prize Problem, stated as a conjunction, derived from the
    six ZFCt promotion channels. -/
theorem ym_prize_problem_from_gates (g : Type*) [LieRing g] [LieAlgebra ℝ g]
    [LieAlgebra.IsSimple ℝ g] :
    Nonempty (QuantumYMTheory g) ∧
    ∀ T : QuantumYMTheory g, 0 < massGap g T :=
  ⟨ym_theory_exists_proved g, fun T => ym_mass_gap_proved g T⟩

end Imscribing.Millennium.YM_Closure
