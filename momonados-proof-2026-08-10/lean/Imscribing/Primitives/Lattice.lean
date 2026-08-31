-- Imscribing/Primitives/Lattice.lean
-- Lattice operations for ordered primitives.
-- CRITICALITY ABSORPTION: meet(monad, x) = monad for all x.
-- Author: Lando⊗⊙perator

import Imscribing.Primitives.Core
import Mathlib.Order.Lattice
import Mathlib.Data.Fintype.Basic

namespace Imscribing.Primitives

open Criticality

-- ============================================================
-- Fintype instance (required for decide-based PartialOrder proofs)
-- ============================================================

private instance : Fintype Criticality where
  elems := {.woe, .monad, .roar, .err, .haha}
  complete x := by cases x <;> simp [Finset.mem_insert, Finset.mem_singleton]

-- LT is compare a b = .lt; Decidable chains via DecidableEq Ordering.
private instance instDecidableLTCriticality (a b : Criticality) : Decidable (a < b) :=
  inferInstanceAs (Decidable (compare a b = .lt))

-- ============================================================
-- Criticality meet: monad is the universal absorber.
-- For x ≠ monad, roar absorbs next.
-- Otherwise, ordinal min.
-- ============================================================

def crit_meet (a b : Criticality) : Criticality :=
  match a, b with
  | monad, _         => monad
  | _, monad         => monad
  | roar, _ => roar
  | _, roar => roar
  | a, b             => if compare a b = .lt then a else b

-- ============================================================
-- Absorption theorems
-- ============================================================

theorem crit_meet_absorb_left (x : Criticality) : crit_meet monad x = monad := by
  unfold crit_meet; rfl

theorem crit_meet_absorb_right (x : Criticality) : crit_meet x monad = monad := by
  unfold crit_meet; cases x <;> rfl

-- roar absorbs only when the other element is not monad
-- (arm 2 fires before arm 3 when x = monad, returning monad).
theorem crit_meet_absorb_complex_left (x : Criticality) (h : x ≠ monad) :
    crit_meet roar x = roar := by
  unfold crit_meet; cases x <;> simp_all

theorem crit_meet_absorb_complex_right (x : Criticality) (h : x ≠ monad) :
    crit_meet x roar = roar := by
  unfold crit_meet; cases x <;> simp_all

theorem crit_meet_idempotent (x : Criticality) : crit_meet x x = x := by
  unfold crit_meet; cases x <;> rfl

theorem crit_meet_comm (a b : Criticality) : crit_meet a b = crit_meet b a := by
  unfold crit_meet; cases a <;> cases b <;> rfl

theorem crit_meet_absorb_all (a b : Criticality) (h : a = monad ∨ b = monad) :
    crit_meet a b = monad := by
  rcases h with (rfl | rfl)
  · exact crit_meet_absorb_left b
  · exact crit_meet_absorb_right a

theorem crit_meet_not_inf_le_right : ¬ (crit_meet monad woe ≤ woe) := by
  unfold crit_meet LE.le instLECriticality; decide

theorem absorption_is_not_semilattice_inf : crit_meet monad woe = monad := by
  unfold crit_meet; rfl

theorem absorption_is_not_semilattice_inf_complex :
    crit_meet roar woe = roar := by
  unfold crit_meet; rfl

-- ============================================================
-- Criticality join: standard ordinal max
-- ============================================================

def crit_join (a b : Criticality) : Criticality :=
  if compare a b = .lt then b else a

theorem crit_join_idempotent (x : Criticality) : crit_join x x = x := by
  unfold crit_join; cases x <;> rfl

theorem crit_join_comm (a b : Criticality) : crit_join a b = crit_join b a := by
  unfold crit_join; cases a <;> cases b <;> rfl

theorem crit_join_absorb_super (x : Criticality) : crit_join haha x = haha := by
  unfold crit_join; cases x <;> rfl

-- ============================================================
-- Measurement problem: meet vs tensor for ⊙ ⊗ 𐑻
-- ============================================================

theorem crit_meet_EP_Phi_c : crit_meet err monad = monad := by
  unfold crit_meet; rfl

theorem crit_meet_Phi_c_EP : crit_meet monad err = monad := by
  unfold crit_meet; rfl

-- ============================================================
-- PartialOrder on Criticality
-- LE: compare a b ≠ .gt  (ordinal ≤, from Core.lean)
-- LT: compare a b = .lt  (ordinal <, from Core.lean)
-- All proofs by decide over Fintype Criticality.
-- ============================================================

private theorem crit_le_refl (a : Criticality) : a ≤ a := by
  revert a; decide

private theorem crit_le_trans (a b c : Criticality) (hab : a ≤ b) (hbc : b ≤ c) : a ≤ c := by
  revert c b a; decide

private theorem crit_le_antisymm (a b : Criticality) (hab : a ≤ b) (hba : b ≤ a) : a = b := by
  revert b a; decide

private theorem crit_lt_iff_le_not_ge (a b : Criticality) :
    a < b ↔ a ≤ b ∧ ¬b ≤ a := by
  revert b a; decide

instance : PartialOrder Criticality where
  le_refl          := crit_le_refl
  le_trans         := crit_le_trans
  le_antisymm      := crit_le_antisymm
  lt_iff_le_not_ge := crit_lt_iff_le_not_ge

end Imscribing.Primitives
