import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Millennium.OmegaCorrClosure

namespace Imscribing.Millennium.SevenFortyFourDecomposition

open Imscribing.Primitives
open Imscribing.Millennium.OmegaCorrClosure

-- Constants

def sic_dim : Nat := 12
def n_frobenius_pairs : Nat := 6
def hw_order : Nat := sic_dim * sic_dim
def ordered_outcome_pairs : Nat := sic_dim * (sic_dim - 1)
def n_grammatical_constraints : Nat := n_frobenius_pairs
def constrained_phase_space : Nat := sic_dim * sic_dim - sic_dim - n_grammatical_constraints
def hilbert_gauge : Nat := sic_dim
def n_orbits : Nat := 31
def modular_weight : Nat := 2 * sic_dim

-- Theorems

theorem hw_order_is_144 : hw_order = 144 := by native_decide
theorem ordered_pairs_is_132 : ordered_outcome_pairs = 132 := by native_decide
theorem constrained_space_is_126 : constrained_phase_space = 126 := by native_decide
theorem six_times_126_is_756 : 6 * 126 = 756 := by native_decide
theorem six_times_constrained_is_756 : n_frobenius_pairs * constrained_phase_space = 756 := by native_decide
theorem n_orbits_is_31 : n_orbits = 31 := by native_decide
theorem modular_weight_is_24 : modular_weight = 24 := by native_decide

-- Main theorem: 744 = 6(d^2-d-6) - d = 6*126 - 12

theorem seven_forty_four_decomposition :
  744 = n_frobenius_pairs * (sic_dim * sic_dim - sic_dim - n_grammatical_constraints) - hilbert_gauge := by
  native_decide

theorem seven_forty_four_simplified :
  744 = 6 * 126 - 12 := by
  native_decide

theorem seven_forty_four_expanded :
  744 = 6 * (144 - 12 - 6) - 12 := by
  native_decide

-- Orbit decomposition: 744 = 31 * 24

theorem orbit_decomposition :
  744 = n_orbits * modular_weight := by
  native_decide

theorem orbit_product :
  n_orbits * modular_weight = 744 := by
  native_decide

-- Equivalence of the two decompositions

theorem decomposition_equivalence :
  n_frobenius_pairs * (sic_dim * sic_dim - sic_dim - n_grammatical_constraints) - hilbert_gauge
  = n_orbits * modular_weight := by
  native_decide

-- Factored form: constrained_phase = (d-3)(d+2) = 9*14 = 126

theorem constrained_phase_factors :
  constrained_phase_space = (sic_dim - 3) * (sic_dim + 2) := by
  native_decide

theorem factored_decomposition :
  744 = n_frobenius_pairs * (sic_dim - 3) * (sic_dim + 2) - hilbert_gauge := by
  native_decide

end Imscribing.Millennium.SevenFortyFourDecomposition
