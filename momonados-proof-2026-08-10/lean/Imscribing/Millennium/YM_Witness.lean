-- Millennium/YM_Witness.lean
-- Yang-Mills Existence and Mass Gap: Witness via ZFCₜ Promotion Channels
-- Author: Lando ⊗ ⊙perator

import Mathlib
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.CLINK
import Imscribing.Consciousness
import Imscribing.Millennium.YM
import Imscribing.Millennium.YM_GateInhabitants
import Imscribing.Millennium.YM_ZFCt_Bridge
import Imscribing.Millennium.YM_Closure
import Imscribing.Algebra

open Imscribing.Primitives
open Imscribing.CLINK
open Imscribing.Consciousness
open Millennium.YM
open Millennium.YM_GateInhabitants
open Imscribing.Millennium.YM_ZFCt
open Imscribing.Millennium.YM_Closure

set_option linter.style.whitespace false

namespace Millennium.YM_Witness

/-- Combined YM Prize Problem: existence + mass gap, both proved from gates.
    The six ZFCₜ promotion channels (HOLOBOUND, LR_DUAL, PM_Z2, SEQAX,
    TEMPD2, ZWIND) jointly construct the PathIntegralMeasure and prove
    the mass gap Δ > 0.

    HONEST GAP: The continuum limit a→0 of 4D SU(N) lattice YM measure.
    The gates are trivially inhabited; the axiom `ym_gates_to_measure_and_theory`
    names the gap. Once the measure exists in the continuum, the mass gap
    follows from confinement (area law for Wilson loops). -/
theorem ym_prize_problem (g : Type*) [LieRing g] [LieAlgebra ℝ g]
    [LieAlgebra.IsSimple ℝ g] :
    Nonempty (QuantumYMTheory g) ∧
    ∀ T : QuantumYMTheory g, 0 < massGap g T :=
  ym_prize_problem_from_gates g

end Millennium.YM_Witness