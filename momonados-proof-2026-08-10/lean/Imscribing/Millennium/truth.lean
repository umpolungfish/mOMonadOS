-- Imscribing/Millennium/truth.lean
-- HOW TRUTH VALUES ARISE FROM THE GRAMMAR

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal

namespace Millennium.truth

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

set_option linter.style.longLine false
set_option linter.style.nativeDecide false

-- ============================================================
-- S1. CLASSICAL TRUTH (age + woe -> Boolean)
-- ============================================================

def classical_truth : Imscription := {
  dim  := dead
  top  := judge
  rel  := ado
  pol  := church
  fid  := age
  kin  := yea
  gran := bib
  gram := vow
  crit := woe
  chir := fee
  stoi := hung
  prot := awe
}

theorem classical_is_O0 : imscriptionTier classical_truth = .O₀ := by
  native_decide

-- ============================================================
-- S2. BELNAP FOUR-VALUED TRUTH
-- ============================================================

def belnap_truth : Imscription := {
  dim  := ash
  top  := mime
  rel  := ado
  pol  := church
  fid  := they
  kin  := egg
  gran := thigh
  gram := gag
  crit := woe
  chir := kick
  stoi := so
  prot := awe
}

-- ============================================================
-- S3. QUANTUM TRUTH (peep + roar -> O₂dag)
-- ============================================================

def quantum_truth : Imscription := {
  dim  := array
  top  := oil
  rel  := ian
  pol  := yew
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := roar
  chir := sure
  stoi := up
  prot := ah
}

theorem quantum_is_O2dag : imscriptionTier quantum_truth = .O₂dag := by
  native_decide

-- ============================================================
-- S4. FROBENIUS-EXACT TRUTH (monad + or' -> O_inf)
-- ============================================================

def frobenius_truth : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := wool
  stoi := up
  prot := ah
}

theorem frobenius_is_Oinf : imscriptionTier frobenius_truth = .O_inf := by
  native_decide

-- ============================================================
-- S5. DISTANCES BETWEEN TRUTH TYPES
-- ============================================================

-- Classical truth is maximally distant from Frobenius truth.
theorem classical_frobenius_distinct : classical_truth ≠ frobenius_truth := by
  native_decide

-- Quantum truth and Frobenius truth differ (O₂dag vs O_inf).
theorem quantum_frobenius_distinct : quantum_truth ≠ frobenius_truth := by
  native_decide

-- Classical and quantum truth are distinct regimes.
theorem classical_quantum_distinct : classical_truth ≠ quantum_truth := by
  native_decide

-- ============================================================
-- S6. THE LIAR PARADOX (err + on)
-- ============================================================

def liar_type : Imscription := {
  dim  := ash
  top  := mime
  rel  := ear
  pol  := church
  fid  := age
  kin  := on
  gran := bib
  gram := gag
  crit := err
  chir := fee
  stoi := so
  prot := awe
}

theorem liar_is_O0 : imscriptionTier liar_type = .O₀ := by
  native_decide

end Millennium.truth
