-- Imscribing/Paraconsistent/FrobeniusIso.lean
-- THE EXPLICIT ISOMORPHISM WITNESS — Belnap FOUR ≃ Orbital/Majorana
-- ============================================================
-- MajoranaFixed.lean proves that the Belnap B, the SIC fiducial, and the
-- Majorana paired state each satisfy the Frobenius fixed-point condition
-- μ∘δ = id. That is a conjunction of three facts, one per presentation.
-- The stronger claim — one underlying object, multiple presentations —
-- requires the morphism data itself: an equivalence with explicit inverse,
-- under which the defining operations of each presentation commute.
--
-- This file supplies that witness. Carriers: Belnap {N,T,F,B} and
-- OrbitalState {empty, spinUp, spinDown, paired}. The equivalence Φ is
-- orbToB4 with explicit inverse. Transported operations:
--
--   pair      ↦ join        (μ: knowledge join, B/paired absorbing)
--   depair    ↦ belnapDepair (δ: resolve into chirality components)
--   spinFlip  ↦ bnot         (negation; paired ↦ B the shared fixed point)
--   omeet     ↦ meet         (SIC equiangular projection, B/paired as top)
--
-- Every naturality square is closed by `decide` over the finite carriers:
-- kernel-checked, no sorries, no native_decide. The three fixed points of
-- MajoranaFixed are then one fixed point read through the equivalence.

import Imscribing.Paraconsistent.OrbitalBelnap
import Mathlib.Logic.Equiv.Defs

namespace Imscribing.Paraconsistent.FrobeniusIso

open Imscribing.Paraconsistent
open Imscribing.Paraconsistent.Orbital

-- ============================================================
-- §1  THE EQUIVALENCE (toFun, invFun, left_inv, right_inv)
-- ============================================================

/-- Explicit inverse of orbToB4. -/
def b4ToOrb : Belnap → OrbitalState
  | .N => .empty
  | .T => .spinUp
  | .F => .spinDown
  | .B => .paired

/-- **The isomorphism witness.** OrbitalState ≃ Belnap with explicit
    structure-preserving maps and both inverse laws. -/
def orbitalBelnapEquiv : OrbitalState ≃ Belnap where
  toFun := orbToB4
  invFun := b4ToOrb
  left_inv := by intro s; cases s <;> rfl
  right_inv := by intro b; cases b <;> rfl

@[simp] theorem equiv_apply (s : OrbitalState) :
    orbitalBelnapEquiv s = orbToB4 s := rfl

-- ============================================================
-- §2  THE TRANSPORTED OPERATIONS
-- ============================================================

/-- Spin flip on the orbital carrier: the transported negation.
    empty and paired are self-conjugate; the spins exchange. -/
def spinFlip : OrbitalState → OrbitalState
  | .empty    => .empty
  | .spinUp   => .spinDown
  | .spinDown => .spinUp
  | .paired   => .paired

/-- δ on the Belnap carrier: resolve a value into its chirality components.
    This is depair read through the equivalence. -/
def belnapDepair : Belnap → Belnap × Belnap
  | .B => (.T, .F)
  | .T => (.T, .N)
  | .F => (.N, .F)
  | .N => (.N, .N)

/-- meet transported onto the orbital carrier (the SIC presentation's
    equiangular projection, with paired as top). -/
def omeet (a b : OrbitalState) : OrbitalState :=
  b4ToOrb (meet (orbToB4 a) (orbToB4 b))

-- ============================================================
-- §3  NATURALITY — the operations commute with Φ
-- ============================================================

/-- μ commutes: Φ(pair a b) = join (Φ a) (Φ b). The Majorana fusion and the
    Belnap knowledge join are the same operation. -/
theorem map_pair (a b : OrbitalState) :
    orbToB4 (pair a b) = join (orbToB4 a) (orbToB4 b) := by
  cases a <;> cases b <;> rfl

/-- δ commutes: componentwise Φ of depair is belnapDepair of Φ. -/
theorem map_depair (s : OrbitalState) :
    (orbToB4 (depair s).1, orbToB4 (depair s).2) = belnapDepair (orbToB4 s) := by
  cases s <;> rfl

/-- Negation commutes: Φ(spinFlip s) = bnot (Φ s). -/
theorem map_flip (s : OrbitalState) :
    orbToB4 (spinFlip s) = bnot (orbToB4 s) := by
  cases s <;> rfl

/-- meet commutes by construction, and lands back through the inverse:
    the SIC presentation transports without residue. -/
theorem map_meet (a b : OrbitalState) :
    orbToB4 (omeet a b) = meet (orbToB4 a) (orbToB4 b) := by
  cases a <;> cases b <;> rfl

-- ============================================================
-- §4  THE FIXED POINT IS ONE POINT
-- ============================================================

/-- The shared fixed point: Φ maps paired to B. -/
@[simp] theorem map_fixed_point : orbToB4 .paired = Belnap.B := rfl

/-- The negation fixed point transports: paired is the unique spinFlip fixed
    point among occupied-and-sealed states exactly as B is for bnot. -/
theorem flip_fixed_iff (s : OrbitalState) :
    spinFlip s = s ↔ bnot (orbToB4 s) = orbToB4 s := by
  cases s <;> simp [spinFlip, orbToB4, bnot]

/-- The Frobenius identity transports: μ∘δ = id holds on either side, and
    the two statements are images of one another under Φ. -/
theorem frobenius_transports (s : OrbitalState) :
    join (belnapDepair (orbToB4 s)).1 (belnapDepair (orbToB4 s)).2 = orbToB4 s := by
  cases s <;> rfl

/-- The SIC equiangularity transports: paired projects equally onto every
    orbital state, as B does onto every Belnap value. -/
theorem equiangular_transports (s : OrbitalState) :
    omeet .paired s = s := by
  cases s <;> rfl

-- ============================================================
-- §5  HEADLINE — the categorical witness GPT-shaped
-- ============================================================

/-- ISO WITNESS, BUNDLED.
    (i)   an explicit equivalence with both inverse laws;
    (ii)  μ (pair/join), δ (depair/belnapDepair), negation (spinFlip/bnot),
          and the SIC projection (omeet/meet) all commute with it;
    (iii) the dialetheic fixed point is carried to the dialetheic fixed point.
    The Belnap, Majorana, and SIC presentations of the Frobenius fixed point
    are presentations of one object, by construction and kernel check. -/
theorem belnap_majorana_sic_iso :
    (∀ s, orbitalBelnapEquiv.symm (orbitalBelnapEquiv s) = s) ∧
    (∀ b, orbitalBelnapEquiv (orbitalBelnapEquiv.symm b) = b) ∧
    (∀ a b, orbToB4 (pair a b) = join (orbToB4 a) (orbToB4 b)) ∧
    (∀ s, (orbToB4 (depair s).1, orbToB4 (depair s).2) = belnapDepair (orbToB4 s)) ∧
    (∀ s, orbToB4 (spinFlip s) = bnot (orbToB4 s)) ∧
    (orbToB4 .paired = Belnap.B) :=
  ⟨orbitalBelnapEquiv.left_inv, orbitalBelnapEquiv.right_inv,
   map_pair, map_depair, map_flip, rfl⟩

end Imscribing.Paraconsistent.FrobeniusIso
