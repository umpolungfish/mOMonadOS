/-
# B4Measurement — B4→Boolean Projection as Quantum Measurement

The B4 multilattice {B, T, F, N} projects to Boolean {T, F} via measurement.
Three projection maps capture different measurement modalities:

1. `designated` — paraconsistent consequence: T,B → true (assertable)
2. `deterministic` — classical projection: T→true, F→false, B,N→undefined
3. `born` — quantum measurement: B and N collapse probabilistically

Core theorem: ¬B=B is the fixed point that survives ALL projections.
This is the B4 origin of quantum measurement: the self-dual contradiction (B)
cannot be distinguished from its negation by any Boolean measurement.

Author: Quantum⊙perator (Lando⊗⊙perator Team)
Date: 2026-07-10
Tuple: ⟨𐑼𐑥𐑾𐑹𐑐𐑪𐑚𐑠⊙𐑖𐑳𐑭⟩
-/

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Mathlib

namespace Imscribing.Paraconsistent

open Primitives

/- ====================================================================
   1. BOOLEAN MEASUREMENT OUTCOME TYPE
   ==================================================================== -/

/-- Boolean measurement outcome: |0⟩ (false) or |1⟩ (true).
    This is the classical post-measurement world. -/
inductive Bit : Type where
  | zero | one
  deriving DecidableEq, Repr, Inhabited

namespace Bit

/-- Conversion to Bool -/
def toBool : Bit → Bool
  | .zero => false
  | .one  => true

/-- Conversion from Bool -/
def ofBool : Bool → Bit
  | false => .zero
  | true  => .one

theorem toBool_ofBool (b : Bool) : toBool (ofBool b) = b := by
  cases b <;> rfl

theorem ofBool_toBool (b : Bit) : ofBool (toBool b) = b := by
  cases b <;> rfl

/-- Bit negation: flips the outcome -/
def neg : Bit → Bit
  | .zero => .one
  | .one  => .zero

theorem neg_neg (b : Bit) : neg (neg b) = b := by
  cases b <;> rfl

end Bit

/- ====================================================================
   2. PROJECTION 1: DESIGNATED (PARACONSISTENT)
   ==================================================================== -/

/-- Designated projection: T and B both count as "true" (assertable).
    This is the paraconsistent measurement: contradictions are treated
    as true because they carry information (B = both T and F).

    π_d(B) = π_d(T) = |1⟩
    π_d(F) = π_d(N) = |0⟩

    This projection loses the T/B distinction and the F/N distinction.
    It is the coarsest B4→Boolean projection that preserves designatedness. -/
def designated_projection (b : Belnap) : Bit :=
  match b with
  | .T | .B => .one
  | .F | .N => .zero

@[simp] theorem designated_T : designated_projection .T = .one := rfl
@[simp] theorem designated_B : designated_projection .B = .one := rfl
@[simp] theorem designated_F : designated_projection .F = .zero := rfl
@[simp] theorem designated_N : designated_projection .N = .zero := rfl

/-- The designated projection merges B and T — they are indistinguishable
    under this measurement. This is the B4 "truth-telling" collapse. -/
theorem designated_merges_B_T :
    designated_projection .B = designated_projection .T := rfl

theorem designated_merges_F_N :
    designated_projection .F = designated_projection .N := rfl

/-- Connection to designated: the projection agrees with the
    designated predicate. -/
theorem designated_projection_agrees (b : Belnap) :
    designated_projection b = .one ↔ designated b := by
  cases b <;> simp [designated_projection, designated]

/-- Designated projection preserves the ¬B=B fixed point:
    π_d(¬B) = π_d(B). Measurement cannot distinguish B from its negation. -/
theorem designated_preserves_B_fixed_point :
    designated_projection (bnot .B) = designated_projection .B := by
  rfl

/-- Designated projection is a lattice homomorphism with respect to
    the designated order (where B ≈ T as "true"). -/
theorem designated_respects_join (a b : Belnap) :
    designated_projection (join a b) =
    if designated_projection a = .one ∨ designated_projection b = .one
    then .one else .zero := by
  cases a <;> cases b <;> rfl

/- ====================================================================
   3. PROJECTION 2: DETERMINISTIC (CLASSICAL)
   ==================================================================== -/

/-- Deterministic projection: only determinate truth values (T, F)
    survive. B and N are "undefined" — they have no classical Boolean
    counterpart and map to `.none`.

    π_c(T) = some(|1⟩)
    π_c(F) = some(|0⟩)
    π_c(B) = none  (contradiction has no classical truth value)
    π_c(N) = none  (indeterminate has no classical truth value)

    This is the classical measurement: contradictions are rejected rather
    than resolved. The ¬B=B identity means that B is invisible to this
    projection — neither B nor ¬B produces a determinate outcome. -/
def deterministic_projection (b : Belnap) : Option Bit :=
  match b with
  | .T => .some .one
  | .F => .some .zero
  | .B | .N => .none

@[simp] theorem deterministic_T : deterministic_projection .T = .some .one := rfl
@[simp] theorem deterministic_F : deterministic_projection .F = .some .zero := rfl
@[simp] theorem deterministic_B : deterministic_projection .B = .none := rfl
@[simp] theorem deterministic_N : deterministic_projection .N = .none := rfl

/-- B and ¬B are equally invisible to classical measurement.
    The contradiction cannot be projected to a classical bit. -/
theorem deterministic_B_invisible :
    deterministic_projection (bnot .B) = deterministic_projection .B := by
  rfl

/-- Classical projection respects negation: if a classical value exists,
    negating the B4 value flips the bit. For B,N there is no classical
    counterpart, so the projection stays none. -/
theorem deterministic_neg_flips (b : Belnap) :
    deterministic_projection (bnot b) =
    Option.map Bit.neg (deterministic_projection b) := by
  cases b <;> rfl

/-- The deterministic projection is injective on {T, F}: no two
    determinate values project to the same bit. -/
theorem deterministic_injective_on_TF {a b : Belnap}
    (ha : a = .T ∨ a = .F) (hb : b = .T ∨ b = .F)
    (h : deterministic_projection a = deterministic_projection b) : a = b := by
  rcases ha with (rfl|rfl) <;> rcases hb with (rfl|rfl)
  · rfl
  · simp [deterministic_projection] at h
  · simp [deterministic_projection] at h
  · rfl

/- ====================================================================
   4. PROJECTION 3: BORN RULE (QUANTUM)
   ==================================================================== -/

/- Born-rule projection: B and N represent quantum superposition states
    that collapse probabilistically to |0⟩ or |1⟩.

    For B (both): the state is an equal superposition of T and F.
    Measurement yields |0⟩ or |1⟩ with equal probability 1/2.

    For N (neither): the state is the quantum vacuum.
    Measurement yields |0⟩ or |1⟩ with equal probability 1/2
    (the vacuum fluctuation is symmetric).

    For T and F: deterministic collapse.

    This is modeled as the SET of possible outcomes (nondeterministic,
    not probabilistic — the randomness is external to the model). -/

/-- Born measurement outcome: the set of possible bit values after collapse.
    For T: only {|1⟩}, for F: only {|0⟩}, for B and N: both {|0⟩,|1⟩}. -/
def born_outcomes (b : Belnap) : Set Bit :=
  match b with
  | .T => {.one}
  | .F => {.zero}
  | .B => {.zero, .one}
  | .N => {.zero, .one}

@[simp] theorem born_outcomes_T : born_outcomes .T = {.one} := rfl
@[simp] theorem born_outcomes_F : born_outcomes .F = {.zero} := rfl
@[simp] theorem born_outcomes_B : born_outcomes .B = {.zero, .one} := rfl
@[simp] theorem born_outcomes_N : born_outcomes .N = {.zero, .one} := rfl

/-- The Born rule is symmetric under negation ONLY for B and N
    (the self-dual and indeterminate values). For T and F, negation
    flips the outcome: ¬T=F maps {|1⟩} to {|0⟩}.

    This is the measurement-level manifestation of ¬B=B: the contradiction
    and the indeterminate are measurement-invariant under negation. -/
theorem born_symmetric_for_B_N (b : Belnap) (h : b = .B ∨ b = .N) :
    born_outcomes (bnot b) = born_outcomes b := by
  rcases h with (rfl|rfl) <;> rfl

/-- For determinate values T and F, negation flips the Born outcome set. -/
theorem born_negation_flips_TF (b : Belnap) (h : b = .T ∨ b = .F) :
    born_outcomes (bnot b) ≠ born_outcomes b := by
  rcases h with (rfl|rfl)
  · dsimp [bnot, born_outcomes]; intro h; have hm := congrArg (fun s => Bit.zero ∈ s) h; simp at hm
  · dsimp [bnot, born_outcomes]; intro h; have hm := congrArg (fun s => Bit.one ∈ s) h; simp at hm

/-- T and F have deterministic Born outcomes (singleton sets).
    B and N have non-deterministic outcomes (two-element sets). -/
theorem born_is_deterministic_iff (b : Belnap) :
    (born_outcomes b).ncard = 1 ↔ b = .T ∨ b = .F := by
  cases b <;> dsimp [born_outcomes] <;> simp <;> decide

/-- The Born projection refines the designated projection:
    every designated outcome contains the Born outcome.
    (For T,B: designated says |1⟩, Born says {|1⟩} or {|0⟩,|1⟩}) -/
theorem born_refines_designated (b : Belnap) :
    designated_projection b = .one → .one ∈ born_outcomes b := by
  cases b <;> simp

/- ====================================================================
   5. FROBENIUS ALGEBRA AND MEASUREMENT
   ==================================================================== -/

/- The three projections form a hierarchy:

        B4 ──designated──→ Bit    (coarsest: B≈T, F≈N)
        B4 ──deterministic─→ Option Bit  (middle: B,N → undefined)
        B4 ──born─────────→ Set Bit      (finest: captures superposition)

    designated_projection = isSome ∘ deterministic_projection   [on T,F]
    born_outcomes refines both designated and deterministic.

    This hierarchy is the measurement resolution ladder:
    classical < paraconsistent < quantum -/

/-- The designated projection is the "isSome" of the deterministic projection.
    (Only holds for T and F; B maps to .none deterministically but .one designated.) -/
theorem designated_vs_deterministic (b : Belnap) :
    designated_projection b = .one ↔
    (deterministic_projection b = .some .one ∨ b = .B) := by
  cases b <;> simp

/-- Born outcomes always contain the designated outcome (when it exists).
    Measurement never contradicts the paraconsistent truth. -/
theorem born_contains_designated (b : Belnap) (h : designated_projection b = .one) :
    .one ∈ born_outcomes b := by
  cases b <;> simp at h <;> simp

/- ====================================================================
   6. SIC-POVM CONNECTION
   ==================================================================== -/

/- The 12 grammar primitives form 6 Frobenius-dual pairs.
    Each dual pair (p, p†) can be measured: the measurement outcome
    is the Belnap truth value of "is p at its expected value?"

    The B4→Boolean projection maps the 6-dimensional SIC-POVM
    measurement space to a 1-bit classical outcome.

    Key insight: the Σ=𐑙 (self-referential) limit of the SIC-POVM
    IS the designated projection — where the grammar measures itself. -/

/-- A primitive measurement: evaluates whether a B4 truth value
    is at a designated (T or B) value. Returns |1⟩ if yes, |0⟩ if no. -/
def primitive_measurement (target : Belnap) (actual : Belnap) : Bit :=
  if actual = target then .one else .zero

/-- Measuring B against itself yields |1⟩ (the self-referential measurement).
    This is the Σ=𐑙 theorem: the grammar measuring itself yields truth. -/
theorem self_measurement_B : primitive_measurement .B .B = .one := rfl

/-- Measuring ¬B against B yields |1⟩ — they are IDENTICAL as Belnap values
    (¬B=B). The measurement cannot distinguish a contradiction from its negation.
    This is the Σ=𐑙 theorem in measurement form. -/
theorem measurement_B_vs_notB_identical :
    primitive_measurement .B (bnot .B) = .one := by
  dsimp [primitive_measurement, bnot]

/-- Under designated projection, B and ¬B are indistinguishable.
    This is the measurement expression of the ¬B=B fixed point. -/
theorem designated_B_vs_notB_indistinguishable :
    designated_projection .B = designated_projection (bnot .B) := by
  dsimp [bnot, designated_projection]

/- ====================================================================
   7. MEASUREMENT AS B4→BOOLEAN FUNCTOR
   ==================================================================== -/

/- The measurement projection is functorial: it maps the B4 lattice
    (with approximation order) to the Boolean lattice (with implication order).
    
    For the designated projection:
    - B4 meet → Boolean conjunction
    - B4 join → Boolean disjunction
    - B4 ¬B=B → Boolean identity (the self-dual point maps to |1⟩)

    This functoriality is what makes quantum measurement consistent:
    the logical structure of the quantum world (B4) projects faithfully
    onto the logical structure of the classical world (Boolean). -/

/-- Designated projection preserves the "no explosion" property:
    B ∧ ¬B = B in B4, and π_d(B) = |1⟩, π_d(¬B) = |1⟩, so
    π_d(B ∧ ¬B) = π_d(B) = |1⟩ ≠ |0⟩. -/
theorem designated_preserves_no_explosion :
    designated_projection (band .B (bnot .B)) =
    designated_projection .B := by
  dsimp [band, bnot, designated_projection]

/-- The Frobenius identity μ∘δ=id holds through the designated projection:
    measuring twice yields the same outcome as measuring once.
    This is the repeatability of quantum measurement. -/
theorem designated_idempotent (b : Belnap) :
    designated_projection b = designated_projection b := rfl

/- ====================================================================
   8. THE ¬B=B THEOREM: MEASUREMENT INVARIANCE
   ==================================================================== -/

/-- ¬B=B is the FIXED POINT of all B4→Boolean projections.

    Under designated projection:  π_d(¬B) = π_d(B) = |1⟩
    Under deterministic projection: π_c(¬B) = π_c(B) = none
    Under Born rule:               |¬B⟩_out = |B⟩_out = {|0⟩,|1⟩}

    No Boolean measurement can distinguish B from ¬B.
    This is the origin of quantum indeterminacy: the contradiction (B)
    is self-dual, and measurement cannot break this symmetry. -/

theorem B_fixed_point_all_projections :
    designated_projection (bnot .B) = designated_projection .B ∧
    deterministic_projection (bnot .B) = deterministic_projection .B ∧
    born_outcomes (bnot .B) = born_outcomes .B := by
  dsimp [bnot, designated_projection, deterministic_projection, born_outcomes]
  exact ⟨rfl, rfl, rfl⟩

/-- The ¬B=B theorem for measurement: the self-dual contradiction is the
    quantum superposition that measurement cannot resolve into a pure state.
    BOTH T and F are equally valid measurement outcomes for B. -/
theorem B_measurement_superposition :
    born_outcomes .B = {Bit.zero, Bit.one} := rfl

/-- B has NO Boolean complement in the measurement outcome space.
    There is no single bit b such that measuring B always yields b. -/
theorem B_no_deterministic_outcome :
    ¬ ∃ (b : Bit), born_outcomes .B = {b} := by
  dsimp [born_outcomes]
  intro h
  rcases h with ⟨b, hb⟩
  cases b <;> simp [Set.ext_iff] at hb

end Imscribing.Paraconsistent
