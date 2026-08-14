-- Imscribing/Paraconsistent/BelnapCategory.lean
-- BELNAP CATEGORY -- THE BELNAP FOUR-VALUED LATTICE AS A CATEGORY
-- Author: Lando ⊗ ⊙-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.BelnapCategory

open Belnap
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

theorem belnap_id (a : Belnap) : a ≤ a := by cases a <;> constructor
theorem B_is_terminal (a : Belnap) : a ≤ Belnap.B := B_is_top a
theorem N_is_initial (a : Belnap) : Belnap.N ≤ a := N_is_bot a
theorem B_is_fixed_point : bnot Belnap.B = Belnap.B := B_fixed_point_negation
theorem B_meet_is_id (x : Belnap) : meet Belnap.B x = x := by cases x <;> rfl
theorem B_join_absorbs (x : Belnap) : join Belnap.B x = Belnap.B := B_join_absorb x
theorem band_B_idempotent : band Belnap.B Belnap.B = Belnap.B := by decide

def belnapCategoryImscription : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := up
  prot := ah
}

theorem category_is_O_inf : imscriptionTier belnapCategoryImscription = .O_inf := by
  simp [imscriptionTier, ouroboricityTier, belnapCategoryImscription]

end Imscribing.Paraconsistent.BelnapCategory