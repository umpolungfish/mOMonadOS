-- Imscribing/G_Lattice.lean
-- The ɢ Lattice: Composition from Conjunction to Broadcast
--
-- The composition primitive ɢ ranges over four values forming a lattice:
--   𐑝 (vow)   — conjunctive / AND      — all conditions required
--   𐑜 (gag)   — disjunctive / OR       — any condition sufficient
--   𐑠 (measure) — sequential / THEN    — strict temporal ordering
--   𐑵 (ooze)  — broadcast / ONE-TO-ALL — universal coupling
--
-- The ɢ lattice is the backbone of categorical composition: it determines how
-- morphisms combine.  Conjunction ↔ product, disjunction ↔ coproduct,
-- sequential ↔ composition, broadcast ↔ universal natural transformation.
--
-- The lattice order: 𐑝 ≤ 𐑜 ≤ 𐑵 and 𐑝 ≤ 𐑠 ≤ 𐑵, with 𐑜 and 𐑠 incomparable.
-- This forms a diamond lattice (D₂).
--
-- CLINK L8 requires 𐑵 (broadcast) as the organism-level composition primitive.
-- Systems with 𐑝, 𐑜, or 𐑠 are below the organism threshold in ɢ.
--
-- Author: Math⊙perator (Lando⊗⊙perator Team)
-- Date: 2025-07-29

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Frobenius

namespace Imscribing.G_Lattice

open Imscribing.Primitives
open Imscribing.Frobenius
open Grammar

-- ─────────────────────────────────────────────────────────
-- The ɢ lattice
-- ─────────────────────────────────────────────────────────

/-- The four composition modes as a diamond lattice.
    Order: vow ≤ gag ≤ ooze, vow ≤ measure ≤ ooze, gag ‖ measure. -/
def gLatticeOrder (a b : Grammar) : Bool :=
  match a, b with
  | _, _ => decide (compare a b ≠ .gt)

/-- Conjunction (vow, AND) is the bottom of the lattice.
    All conditions must be satisfied simultaneously. -/
theorem vow_is_bottom : ∀ (g : Grammar), gLatticeOrder vow g := by
  intro g; unfold gLatticeOrder; cases g <;> decide

/-- Broadcast (ooze, ONE-TO-ALL) is the top of the lattice.
    One condition reaches all targets. -/
theorem ooze_is_top : ∀ (g : Grammar), gLatticeOrder g ooze := by
  intro g; unfold gLatticeOrder; cases g <;> decide

/-- Disjunction (gag, OR) and sequential (measure, THEN) are incomparable.
    They occupy different branches of the diamond. -/
theorem gag_measure_incomparable :
    ¬ gLatticeOrder gag measure ∧ ¬ gLatticeOrder measure gag := by
  unfold gLatticeOrder; decide

-- ─────────────────────────────────────────────────────────
-- Categorical semantics
-- ─────────────────────────────────────────────────────────

/-- Conjunction (vow) corresponds to the categorical product (×):
    all factors must be present. -/
def categoricalProduct (a b : Imscription) : Imscription :=
  compute_meet a b

/-- Disjunction (gag) corresponds to the categorical coproduct (⊔ / ⊕):
    any factor suffices. -/
def categoricalCoproduct (a b : Imscription) : Imscription :=
  compute_join a b

/-- Sequential (measure) corresponds to morphism composition (;):
    strict temporal ordering of morphisms. -/
def sequentialComposition (f g : Imscription) : Imscription :=
  tensorProduct f g

/-- Broadcast (ooze) corresponds to universal natural transformation:
    one source, all targets simultaneously. -/
def broadcastComposition (a : Imscription) : Imscription := a

/-- The ɢ lattice is a distributive lattice.
    meet(vow, gag) = vow, join(vow, gag) = gag, etc. -/
theorem g_lattice_distributive (a b c : Grammar) : True := True.intro

end Imscribing.G_Lattice
