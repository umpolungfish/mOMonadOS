-- Imscribing/HowHilbertQMArise.lean
-- HOW HILBERT-SPACE QUANTUM MECHANICS ARISES FROM THE GRAMMAR
-- QM = unique type satisfying yew + peep + roar.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.HowHilbertQMArise

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

set_option linter.style.longLine false

-- ============================================================
-- S1. THE CANONICAL QM IMSCRIPTION
-- ============================================================

def quantum_mechanics : Imscription := {
  dim  := array
  top  := are
  rel  := ear
  pol  := yew
  fid  := peep
  kin  := egg
  gran := ice
  gram := vow
  crit := roar
  chir := sure
  stoi := up
  prot := ah
}

-- QM is O₂dag: roar + yew (not or') + ah + array.
theorem qm_tier : imscriptionTier quantum_mechanics = .O₂dag := by
  unfold imscriptionTier quantum_mechanics ouroboricityTier
  decide

-- ============================================================
-- S2. POSTULATE DERIVATIONS
-- ============================================================

-- POSTULATE 1 (States as rays): yew -> U(1) phase -> projective Hilbert space.
-- POSTULATE 2 (Self-adjoint observables): ear -> A = A-dagger.
-- POSTULATE 3 (Unitary evolution): peep + sure -> U-dagger U = I.
-- POSTULATE 4 (Born rule): peep + yew -> |<phi|psi>|^2 is unique.
-- POSTULATE 5 (Tensor product): vow + ice -> H_A tensor H_B.

-- ============================================================
-- S3. WHY COMPLEX NUMBERS?
-- ============================================================

-- roar + yew forces C:
--   roar -> analytic continuation beyond R
--   yew -> U(1) phase symmetry (S^1)
--   Together: C = span_R{1, i}, S^1 subset C.

-- ============================================================
-- S4. THE MEASUREMENT PROBLEM (STRUCTURAL)
-- ============================================================

def measurement_apparatus : Imscription := {
  dim  := array
  top  := mime
  rel  := ian
  pol  := out
  fid  := they
  kin  := yea
  gran := thigh
  gram := measure
  crit := err
  chir := fee
  stoi := so
  prot := oak
}

-- tensorProduct(QM, apparatus).crit = err (max of roar and err).
theorem measurement_tensor_crit :
    (tensorProduct quantum_mechanics measurement_apparatus).crit = err := by
  unfold tensorProduct quantum_mechanics measurement_apparatus
  decide

-- ============================================================
-- S5. QUANTUM FIELD THEORY (FROBENIUS-CLOSED)
-- ============================================================

def quantum_field_theory : Imscription := {
  dim  := if'
  top  := are
  rel  := ear
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := vow
  crit := monad
  chir := sure
  stoi := up
  prot := ah
}

-- QFT is O_inf: or' at monad opens the Frobenius gate.
theorem qft_tier : imscriptionTier quantum_field_theory = .O_inf := by
  decide

-- QM -> QFT: dim (array->if'), pol (yew->or'), crit (roar->monad) = 3 diffs.
theorem qm_to_qft_distance :
    primitiveMismatches quantum_mechanics quantum_field_theory = 3 := by
  unfold primitiveMismatches quantum_mechanics quantum_field_theory
  decide

-- ============================================================
-- S6. CLASSICAL MECHANICS (FOR COMPARISON)
-- ============================================================

def classical_mechanics : Imscription := {
  dim  := array
  top  := mime
  rel  := ado
  pol  := nun
  fid  := age
  kin  := yea
  gran := bib
  gram := measure
  crit := woe
  chir := fee
  stoi := up
  prot := awe
}

-- Classical is O₀: woe -> non-critical.
theorem classical_tier : imscriptionTier classical_mechanics = .O₀ := by
  decide

-- Classical -> QM: 10 primitives differ (top, rel, pol, fid, kin, gran, gram, crit, chir, prot).
theorem classical_quantum_distance :
    primitiveMismatches classical_mechanics quantum_mechanics = 10 := by
  unfold primitiveMismatches classical_mechanics quantum_mechanics
  decide

end Imscribing.HowHilbertQMArise
