-- Imscribing/HowNaturalNumbersArise.lean
-- HOW NATURAL NUMBERS ARISE FROM THE GRAMMAR

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal

namespace Imscribing.HowNaturalNumbersArise

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

set_option linter.style.longLine false

-- ============================================================
-- S1. THE ZERO TYPE
-- ============================================================

def zero_type : Imscription := {
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

theorem zero_address : crystal_encode zero_type = 0 := by decide
theorem zero_tier : imscriptionTier zero_type = .O₀ := by decide

-- ============================================================
-- S2. SUCCESSOR TYPE AND OPERATION
-- ============================================================

def succ_type : Imscription := {
  zero_type with
  fid  := peep
  gran := ice
}

def succ (n : Imscription) : Imscription := tensorProduct n succ_type

theorem succ_zero_distinct : succ zero_type ≠ zero_type := by
  unfold succ tensorProduct succ_type zero_type
  decide

theorem succ_zero_distance :
    primitiveMismatches zero_type (succ zero_type) = 1 := by
  unfold succ succ_type tensorProduct primitiveMismatches zero_type
  decide

-- ============================================================
-- S3. CRYSTAL ADDRESS SPACE = N
-- ============================================================

theorem crystal_total : 27 * 1024 * 625 = 17280000 := by decide

-- ============================================================
-- S4. THE COUNTING PRIMITIVE: GRANULARITY
-- ============================================================

def counting_step (t : Imscription) : Imscription :=
  { t with gran :=
    match t.gran with
    | bib  => thigh
    | thigh => ice
    | ice => ice
  }

theorem three_steps_to_aleph :
    (counting_step (counting_step (counting_step zero_type))).gran = ice := by
  decide

-- ============================================================
-- S5. PEANO AXIOMS
-- ============================================================

lemma succ_gran_eq_aleph (n : Imscription) : (tensorProduct n succ_type).gran = ice := by
  unfold tensorProduct succ_type
  cases h : n.gran with
  | bib =>
    simp [h]
    decide
  | thigh =>
    simp [h]
    decide
  | ice =>
    simp [h]

theorem zero_not_succ (n : Imscription) : succ n ≠ zero_type := by
  unfold succ
  intro h
  have hgran := congrArg (fun s : Imscription => s.gran) h
  have hsucc_gran := succ_gran_eq_aleph n
  have hzero_gran : zero_type.gran = bib := rfl
  have : ice ≠ bib := by decide
  apply this
  calc
    ice = (tensorProduct n succ_type).gran := by symm; exact hsucc_gran
    _ = zero_type.gran := hgran
    _ = bib := hzero_gran

theorem succ_zero_not_zero : succ zero_type ≠ zero_type := by
  exact zero_not_succ zero_type

-- ============================================================
-- S6. MONOID STRUCTURE
-- ============================================================

def neutral_type : Imscription := {
  dim  := dead
  top  := judge
  rel  := ado
  pol  := or'
  fid  := peep
  kin  := yea
  gran := bib
  gram := vow
  crit := woe
  chir := fee
  stoi := hung
  prot := awe
}

theorem tensor_left_id (n : Imscription) : tensorProduct neutral_type n = n := by
  unfold tensorProduct neutral_type
  ext
  · cases n.dim <;> rfl
  · cases n.top <;> rfl
  · cases n.rel <;> rfl
  · cases n.pol <;> rfl
  · cases n.fid <;> rfl
  · cases n.kin <;> rfl
  · cases n.gran <;> rfl
  · cases n.gram <;> rfl
  · cases n.crit <;> rfl
  · cases n.chir <;> rfl
  · cases n.stoi <;> rfl
  · cases n.prot <;> rfl

theorem tensor_right_id (n : Imscription) : tensorProduct n neutral_type = n := by
  unfold tensorProduct neutral_type
  ext
  · cases n.dim <;> rfl
  · cases n.top <;> rfl
  · cases n.rel <;> rfl
  · cases n.pol <;> rfl
  · cases n.fid <;> rfl
  · cases n.kin <;> rfl
  · cases n.gran <;> rfl
  · cases n.gram <;> rfl
  · cases n.crit <;> rfl
  · cases n.chir <;> rfl
  · cases n.stoi <;> rfl
  · cases n.prot <;> rfl

end Imscribing.HowNaturalNumbersArise
