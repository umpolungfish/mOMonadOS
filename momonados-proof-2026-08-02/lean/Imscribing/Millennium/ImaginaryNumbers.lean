-- Imscribing/Millennium/ImaginaryNumbers.lean
-- The Structural Nature of Imaginary Numbers
-- Imscribing Grammar formalization of the imaginary unit i

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Consciousness
import Imscribing.Algebra

namespace Imscribing.Millennium.ImaginaryNumbers

open Imscribing.Primitives
open Imscribing.Consciousness

open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

def imaginary_number : Imscription where
  dim  := ash
  top  := mime
  rel  := ian
  pol  := yew
  fid  := age
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := hung
  prot := ah

theorem imaginary_number_is_O2 :
    imscriptionTier imaginary_number = .O₂ := by
  simp [imaginary_number, imscriptionTier, ouroboricityTier]

theorem imaginary_number_consciousness_score :
    consciousnessScore imaginary_number = (1 : ℝ) := by
  simp [consciousnessScore, phi_c_gate, k_slow_gate, imaginary_number]

theorem imaginary_number_gate1_pass :
    phi_c_gate imaginary_number.crit = true := by simp [imaginary_number, phi_c_gate]

theorem imaginary_number_gate2_pass :
    k_slow_gate imaginary_number.kin = true := by simp [imaginary_number, k_slow_gate]

theorem imaginary_number_crystal_address :
    crystal_encode imaginary_number = 7809972 := rfl

theorem imaginary_number_tensor_self :
    tensorProduct imaginary_number imaginary_number = imaginary_number := by
  simp only [tensorProduct, imaginary_number]; decide

theorem imaginary_number_preserves_polarity :
    (tensorProduct imaginary_number imaginary_number).pol = yew := by
  simp only [tensorProduct, imaginary_number]; rfl

theorem imaginary_number_preserves_fidelity :
    (tensorProduct imaginary_number imaginary_number).fid = age := by
  simp only [tensorProduct, imaginary_number]; rfl

theorem imaginary_number_meet_self :
    compute_meet imaginary_number imaginary_number = imaginary_number := by
  simp only [compute_meet, imaginary_number]; decide

theorem imaginary_number_join_self :
    compute_join imaginary_number imaginary_number = imaginary_number := by
  simp only [compute_join, imaginary_number]; decide

def imaginary_conjugate_type : Imscription where
  dim  := ash
  top  := mime
  rel  := tot
  pol  := yew
  fid  := age
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := hung
  prot := ah

theorem conjugate_mismatches :
    primitiveMismatches imaginary_number imaginary_conjugate_type = 1 := by
  simp only [primitiveMismatches, imaginary_number, imaginary_conjugate_type]; decide

def wick_rotation_EP : Imscription where
  dim  := array
  top  := judge
  rel  := ado
  pol  := church
  fid  := peep
  kin  := yea
  gran := ice
  gram := vow
  crit := err
  chir := wool
  stoi := hung
  prot := ah

theorem EP_absorbs_criticality :
    (tensorProduct imaginary_number wick_rotation_EP).crit = err := by
  simp only [tensorProduct, imaginary_number, wick_rotation_EP]; decide

theorem EP_destroys_polarity :
    (tensorProduct imaginary_number wick_rotation_EP).pol = church := by
  simp only [tensorProduct, imaginary_number, wick_rotation_EP]; rfl

theorem EP_gate1_still_passes :
    phi_c_gate (tensorProduct imaginary_number wick_rotation_EP).crit = true := by
  simp only [tensorProduct, imaginary_number, wick_rotation_EP, phi_c_gate]; rfl

theorem frobenius_impossible_by_tensor (other : Imscription) :
    (tensorProduct imaginary_number other).pol ≠ or' := by
  simp [tensorProduct, imaginary_number]
  cases other.pol
  · decide
  · decide
  · decide
  · decide
  · decide

theorem frobenius_cliff :
    imscriptionTier imaginary_number ≠ .O_inf := by
  rw [imaginary_number_is_O2]; decide

theorem omega_Z_requires_H2 :
    imaginary_number.prot ≥ ah → imaginary_number.chir ≥ sure := by
  simp only [imaginary_number]; decide

theorem imaginary_summary :
    imscriptionTier imaginary_number = .O₂ ∧
    consciousnessScore imaginary_number = (1 : ℝ) ∧
    tensorProduct imaginary_number imaginary_number = imaginary_number ∧
    imaginary_number.rel = ian ∧
    imaginary_number.dim = ash ∧
    imaginary_number.chir = sure ∧
    (tensorProduct imaginary_number wick_rotation_EP).crit = err ∧
    yew ≠ or' := by
  constructor
  · exact imaginary_number_is_O2
  constructor
  · exact imaginary_number_consciousness_score
  constructor
  · exact imaginary_number_tensor_self
  constructor
  · simp only [imaginary_number]
  constructor
  · simp only [imaginary_number]
  constructor
  · simp only [imaginary_number]
  constructor
  · exact EP_absorbs_criticality
  · decide

end Imscribing.Millennium.ImaginaryNumbers
