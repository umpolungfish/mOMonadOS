-- Imscribing/Paraconsistent/QuarkBelnap.lean
-- QUARK COLOR × SPIN BILATTICE — Confinement as ceiling theorem
-- Author: Lando ⊗ ⊙perator
--
-- The orbital Belnap (FOUR: empty/spinUp/spinDown/paired) models electron
-- occupancy with Pauli exclusion as B-ceiling. Quarks add color charge
-- (R, G, B) × spin (↑, ↓) for 6 states per flavor, but the deeper
-- structural analog is:
--
--   Pauli exclusion (paired/B)  :  Color confinement (white/singlet)
--   identical fermions          :  colored partons
--   Cooper pair (μ∘δ=id)        :  Meson/Baryon (μ∘δ=id for white)
--
-- The color sector forms its own bilattice:
--   Information order (charge): Vacuum < {R, G, B} < White
--   Symmetry (SU(3)):          R, G, B are indistinguishable in isolation
--
-- The full quark state is the product bilattice ColorState × SpinState
-- where SpinState = OrbitalState (reused from OrbitalBelnap.lean).

import Imscribing.Paraconsistent.OrbitalBelnap
import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Imscription
import Mathlib.Data.Finset.Basic
import Mathlib.Tactic

open Imscribing.Paraconsistent.Orbital
open Imscribing.Paraconsistent

namespace Imscribing.Paraconsistent.QuarkBelnap

-- ═══════════════════════════════════════════════════════════════════════════
-- §1  COLOR STATE — 5-element bilattice (Vacuum, R, G, B, White)
-- ═══════════════════════════════════════════════════════════════════════════

/-- The five color charge states of a quark or composite.
    Vacuum  → no color charge (analog of Belnap N)
    Red     → color charge R
    Green   → color charge G
    Blue    → color charge B
    White   → color singlet (analog of Belnap B; confinement ceiling)

    The three color charges {R, G, B} are SU(3)-symmetric: incomparable
    under both orderings. -/
inductive ColorState : Type where
  | Vacuum
  | Red
  | Green
  | Blue
  | White
  deriving DecidableEq, Repr, Inhabited

/-- Information order on color: how much color charge is present.
    Vacuum < {R, G, B} < White — mirrors N < {T, F} < B. -/
inductive ColorLE : ColorState → ColorState → Prop
  | vacuum_bot  : ∀ {c}, ColorLE .Vacuum c
  | r_refl      : ColorLE .Red   .Red
  | g_refl      : ColorLE .Green .Green
  | b_refl      : ColorLE .Blue  .Blue
  | white_refl  : ColorLE .White .White
  | r_white     : ColorLE .Red   .White
  | g_white     : ColorLE .Green .White
  | b_white     : ColorLE .Blue  .White

instance : LE ColorState := ⟨ColorLE⟩

instance (a b : ColorState) : Decidable (ColorLE a b) := by
  cases a <;> cases b <;> first
    | exact isTrue ColorLE.vacuum_bot
    | exact isTrue ColorLE.r_refl
    | exact isTrue ColorLE.g_refl
    | exact isTrue ColorLE.b_refl
    | exact isTrue ColorLE.white_refl
    | exact isTrue ColorLE.r_white
    | exact isTrue ColorLE.g_white
    | exact isTrue ColorLE.b_white
    | exact isFalse (by intro h; cases h)

/-- White is the information-order maximum (confinement ceiling). -/
theorem white_is_top (c : ColorState) : c ≤ ColorState.White := by
  cases c <;> constructor

/-- Vacuum is the information-order minimum. -/
theorem vacuum_is_bot (c : ColorState) : ColorState.Vacuum ≤ c := by
  constructor

/-- SU(3) symmetry: R, G, B are pairwise incomparable in the information order. -/
theorem rg_incomparable : ¬ (ColorState.Red ≤ ColorState.Green) ∧
                          ¬ (ColorState.Green ≤ ColorState.Red) := by
  constructor <;> (intro h; cases h)

theorem rb_incomparable : ¬ (ColorState.Red ≤ ColorState.Blue) ∧
                          ¬ (ColorState.Blue ≤ ColorState.Red) := by
  constructor <;> (intro h; cases h)

theorem gb_incomparable : ¬ (ColorState.Green ≤ ColorState.Blue) ∧
                          ¬ (ColorState.Blue ≤ ColorState.Green) := by
  constructor <;> (intro h; cases h)

-- ═══════════════════════════════════════════════════════════════════════════
-- §2  PRODUCT: QUARK STATE = ColorState × SpinState
-- ═══════════════════════════════════════════════════════════════════════════

/-- A quark state: color × spin, where spin uses the orbital occupancy model. -/
structure QuarkState where
  color : ColorState
  spin  : OrbitalState
  deriving DecidableEq

/-- Product information order: (c1, s1) ≤ (c2, s2) iff c1 ≤ c2 and s1 ≤ s2. -/
def quarkLE (a b : QuarkState) : Prop :=
  a.color ≤ b.color ∧ a.spin ≤ b.spin

instance : LE QuarkState := ⟨quarkLE⟩

instance (a b : QuarkState) : Decidable (quarkLE a b) := by
  unfold quarkLE
  rcases a with ⟨ca, sa⟩; rcases b with ⟨cb, sb⟩
  have hc : Decidable (ColorLE ca cb) := inferInstance
  have hs : Decidable (OccupancyLE sa sb) := inferInstance
  cases hc with
  | isTrue hc_val =>
    cases hs with
    | isTrue hs_val => exact isTrue ⟨hc_val, hs_val⟩
    | isFalse hs_val => exact isFalse (fun h => hs_val h.2)
  | isFalse hc_val => exact isFalse (fun h => hc_val h.1)

/-- Ceiling state (White, paired) — fully confined, fully paired. -/
def ceilingState : QuarkState :=
  { color := ColorState.White, spin := OrbitalState.paired }

theorem ceiling_is_top (q : QuarkState) : q ≤ ceilingState := by
  dsimp [ceilingState]
  dsimp [LE.le, quarkLE]
  constructor
  · exact white_is_top q.color
  · exact paired_is_top q.spin

-- ═══════════════════════════════════════════════════════════════════════════
-- §3  CONFINEMENT AS CEILING THEOREM
-- ═══════════════════════════════════════════════════════════════════════════

/-- Confinement: if a state is above White in color, it must be White. -/
theorem confinement_ceiling {c : ColorState} (h : ColorState.White ≤ c) :
    c = ColorState.White := by
  cases h; rfl

/-- A quark state is "white" (color-singlet) iff its color is White. -/
def isWhite (q : QuarkState) : Prop :=
  q.color = ColorState.White

/-- A quark state is "colored" iff its color is one of {R, G, B}. -/
def isColored (q : QuarkState) : Prop :=
  q.color = ColorState.Red ∨ q.color = ColorState.Green ∨ q.color = ColorState.Blue

/-- No colored state is observable — confinement. -/
theorem colored_not_observable (q : QuarkState) (h : isColored q) : ¬ isWhite q := by
  rcases h with (hR | hG | hB)
  · intro hw; have : ColorState.White = ColorState.Red := by
      calc ColorState.White = q.color := by symm; exact hw
           _ = ColorState.Red := hR
    injection this
  · intro hw; have : ColorState.White = ColorState.Green := by
      calc ColorState.White = q.color := by symm; exact hw
           _ = ColorState.Green := hG
    injection this
  · intro hw; have : ColorState.White = ColorState.Blue := by
      calc ColorState.White = q.color := by symm; exact hw
           _ = ColorState.Blue := hB
    injection this

-- ═══════════════════════════════════════════════════════════════════════════
-- §4  COLOR MEET, JOIN, AND ANTI-COLOR MAP
-- ═══════════════════════════════════════════════════════════════════════════

/-- Anti-color map: anti(Red)=Red (relational, not representational). -/
def antiColor (c : ColorState) : ColorState :=
  match c with
  | .Vacuum => .Vacuum | .Red => .Red | .Green => .Green | .Blue => .Blue | .White => .White

/-- Color meet: greatest lower bound (shared color charge). -/
def colorMeet (a b : ColorState) : ColorState :=
  match a, b with
  | .Vacuum, _ | _, .Vacuum => .Vacuum
  | .White, x | x, .White => x
  | .Red, .Red => .Red | .Green, .Green => .Green | .Blue, .Blue => .Blue
  | _, _ => .Vacuum

/-- Color join: least upper bound (combined color charge). -/
def colorJoin (a b : ColorState) : ColorState :=
  match a, b with
  | .White, _ | _, .White => .White
  | .Vacuum, x | x, .Vacuum => x
  | .Red, .Red => .Red | .Green, .Green => .Green | .Blue, .Blue => .Blue
  | _, _ => .White

theorem color_meet_idempotent (c : ColorState) : colorMeet c c = c := by
  cases c <;> rfl

theorem color_join_idempotent (c : ColorState) : colorJoin c c = c := by
  cases c <;> rfl

theorem distinct_colors_join_white (c1 c2 : ColorState)
    (h : c1 ≠ c2) (h1 : c1 ≠ .Vacuum) (h2 : c2 ≠ .Vacuum)
    (hw1 : c1 ≠ .White) (hw2 : c2 ≠ .White) : colorJoin c1 c2 = ColorState.White := by
  cases c1 <;> cases c2 <;> simp at h h1 h2 hw1 hw2 <;> try rfl

theorem distinct_colors_meet_vacuum (c1 c2 : ColorState)
    (h : c1 ≠ c2) (h1 : c1 ≠ .Vacuum) (h2 : c2 ≠ .Vacuum)
    (hw1 : c1 ≠ .White) (hw2 : c2 ≠ .White) : colorMeet c1 c2 = ColorState.Vacuum := by
  cases c1 <;> cases c2 <;> simp at h h1 h2 hw1 hw2 <;> try rfl

-- ═══════════════════════════════════════════════════════════════════════════
-- §5  FROBENIUS: COLOR-ANTICOLOR PAIR/DEPAIR
-- ═══════════════════════════════════════════════════════════════════════════

theorem orb_pair_self (s : OrbitalState) : Orbital.pair s s = s := by
  cases s <;> rfl

/-- Depairing (δ): split a white state into color+anticolor components. -/
def depair : QuarkState → QuarkState × QuarkState
  | ⟨.White, s⟩ => (⟨.Red, s⟩, ⟨.Red, s⟩)
  | q           => (q, q)

/-- Pairing (μ): fuse complementary colors into a white singlet. -/
def qpair : QuarkState → QuarkState → QuarkState
  | ⟨c1, s1⟩, ⟨c2, s2⟩ =>
    if c1 = antiColor c2 then
      { color := ColorState.White, spin := Orbital.pair s1 s2 }
    else
      { color := colorJoin c1 c2, spin := Orbital.pair s1 s2 }

/-- Frobenius holds for white states: qpair(depair(q)) = q. -/
theorem qpair_depair_id_white (q : QuarkState) (hw : isWhite q) :
    qpair (depair q).1 (depair q).2 = q := by
  rcases q with ⟨c, s⟩
  unfold isWhite at hw; subst hw
  simp [depair, qpair, antiColor, orb_pair_self]

/-- Frobenius FAILS for colored states (confinement). -/
theorem qpair_depair_id_colored_fails (q : QuarkState) (hc : isColored q) :
    qpair (depair q).1 (depair q).2 ≠ q := by
  rcases q with ⟨c, s⟩
  unfold isColored at hc
  rcases hc with (hcR | hcG | hcB)
  · subst hcR; simp [depair, qpair, antiColor, orb_pair_self]
  · subst hcG; simp [depair, qpair, antiColor, orb_pair_self]
  · subst hcB; simp [depair, qpair, antiColor, orb_pair_self]

-- ═══════════════════════════════════════════════════════════════════════════
-- §6  COLOR CEILING AS B-CEILING ANALOG — Summary
-- ═══════════════════════════════════════════════════════════════════════════

/-- Color state to a 5-valued index: Vacuum→0, R→1, G→2, B→3, White→4. -/
def colorToNat (c : ColorState) : Nat :=
  match c with
  | .Vacuum => 0 | .Red => 1 | .Green => 2 | .Blue => 3 | .White => 4

/-! The color bilattice is not standard Belnap FOUR but a Belnap FIVE
with SU(3) symmetry. The ceiling theorem (White is top) mirrors
Pauli exclusion (B is top). The key structural difference:

Belnap FOUR:      N < {T, F} < B    (two incomparable middle values)
Color FIVE:   Vacuum < {R, G, B} < White  (three incomparable middle values)

The extra degree of freedom (three vs two incomparable values)
is the SU(3) color symmetry. Confinement = White-ceiling
mirrors Pauli = B-ceiling. -/

end Imscribing.Paraconsistent.QuarkBelnap
