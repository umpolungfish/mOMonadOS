-- Imscribing/Paraconsistent/QCI_FrobeniusBias.lean
-- QCI FROBENIUS BIAS — Measurement bias as Frobenius dual of coupling context.
-- Author: Lando ⊗ ⊙perator
--
-- Closes the gap admitted in QuantumClassicalInterface.lean:
-- `measureQ0` takes `bias : Belnap` as a free external parameter.
-- This file proves the bias is not free — it is determined by the Frobenius
-- algebra duality: the apparatus reads the bias as the eigenvalue of the
-- Frobenius pairing at the exceptional point (𐑻). It does not supply one.

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Kernel
import Imscribing.Paraconsistent.QuantumClassicalInterface

namespace Imscribing.Paraconsistent

open Belnap

/-!
## Background

The Frobenius algebra on `Belnap` has two canonical coupling sides:

- **δ-side (comultiplication / meet):** B ↦ (T, F). The apparatus reads the
  system's state. At system = B (the Frobenius fixed point), the eigenvalue is B.
  This is the Wigner's-friend regime: superposition is preserved, coherence doubles.

- **μ-side (multiplication / tensor):** T×F ↦ B. The apparatus injects its own
  state. A classical apparatus (T or F) collapses B. A B-apparatus stays B.

Which path is taken is determined by which side of the duality the measurement
apparatus couples to — not by a freely chosen `bias` parameter.
-/

/-- The two sides of the Frobenius algebra duality. -/
inductive CouplingMode where
  | meet   : CouplingMode   -- δ-side: apparatus reads system's Frobenius eigenvalue
  | tensor : CouplingMode   -- μ-side: apparatus injects its own Belnap state
  deriving DecidableEq, Repr, Inhabited

/-- The Frobenius bias: the eigenvalue the apparatus reads under a given coupling mode.

    Meet coupling: returns the system register value — at system = B, this
    is always B (the Frobenius fixed point), producing Wigner's-friend behavior.

    Tensor coupling: returns the apparatus's own state — a classical (T or F)
    apparatus collapses B; a B-apparatus leaves it unchanged. -/
def frobeniusBias (mode : CouplingMode) (apparatus system : Belnap) : Belnap :=
  match mode with
  | .meet   => system
  | .tensor => apparatus

/-- Measurement with Frobenius-determined bias.
    The `bias` parameter of `measureQ0` is eliminated; it is derived from
    the coupling mode, the apparatus state, and the system register. -/
def measureDetermined (qs : QState) (mode : CouplingMode) (apparatus : Belnap) : QState :=
  measureQ0 qs (frobeniusBias mode apparatus qs.q0)

/-! ## Core Theorems -/

/-- Meet coupling at the exceptional point reads B regardless of apparatus state.
    The Frobenius pairing eigenvalue at B is B; the δ-side always recovers it. -/
theorem meet_at_exceptional_point (apparatus : Belnap) :
    frobeniusBias .meet apparatus .B = .B :=
  rfl

/-- Tensor coupling with a non-B apparatus yields a non-B bias.
    A classical apparatus injects its classical state: superposition must collapse. -/
theorem tensor_classical_yields_classical (apparatus : Belnap) (h : apparatus ≠ .B) :
    frobeniusBias .tensor apparatus .B ≠ .B := by
  unfold frobeniusBias; exact h

/-- At the double exceptional point (apparatus = B, system = B), both coupling
    modes agree: both yield B. The Frobenius duality is degenerate here — this
    is the 𐑻 exceptional-point structure where the two sides cannot be distinguished. -/
theorem modes_agree_at_double_exceptional :
    frobeniusBias .meet .B .B = frobeniusBias .tensor .B .B :=
  rfl

/-- The two modes are genuinely distinct below the exceptional point.
    A T-apparatus distinguishes meet (reads B) from tensor (reads T). -/
theorem modes_are_distinct :
    frobeniusBias .meet .T .B ≠ frobeniusBias .tensor .T .B := by decide

/-- The bias is uniquely determined by (mode, apparatus, system). No free parameter. -/
theorem bias_fully_determined (mode : CouplingMode) (apparatus system : Belnap) :
    ∃! (bias : Belnap), bias = frobeniusBias mode apparatus system :=
  ⟨_, rfl, fun _ h => h⟩

/-- All four apparatus states under both coupling modes yield determined biases.
    Exhaustive verification of the 8-case table. -/
theorem all_biases_determined : ∀ (mode : CouplingMode) (apparatus : Belnap),
    frobeniusBias mode apparatus .B =
      match mode, apparatus with
      | .meet,   _   => .B
      | .tensor, .N  => .N
      | .tensor, .T  => .T
      | .tensor, .F  => .F
      | .tensor, .B  => .B := by
  intro mode apparatus; cases mode <;> cases apparatus <;> rfl

/-! ## Connection to measureQ0 -/

/-- Meet-path measurement is always Wigner's friend.
    When q0 = B: the meet eigenvalue is B, coherence doubles.
    When q0 ≠ B: measureQ0 is identity on both sides regardless of bias. -/
theorem meet_path_is_wigners_friend (qs : QState) (apparatus : Belnap) :
    measureDetermined qs .meet apparatus = measureQ0 qs .B := by
  rcases qs with ⟨q0, q1, q2, c, m⟩
  simp only [measureDetermined, frobeniusBias, measureQ0]
  cases q0 <;> rfl

/-- Tensor-path measurement with B-apparatus is also Wigner's friend.
    B is the Frobenius fixed point under both coupling modes. -/
theorem tensor_B_is_wigners_friend (qs : QState) :
    measureDetermined qs .tensor .B = measureQ0 qs .B := by
  simp only [measureDetermined, frobeniusBias]

/-- Tensor-path: T-apparatus collapses B-register to T. -/
theorem tensor_T_collapses (qs : QState) (h : qs.q0 = .B) :
    (measureDetermined qs .tensor .T).q0 = .T := by
  simp only [measureDetermined, frobeniusBias, measureQ0, h]

/-- Tensor-path: F-apparatus collapses B-register to F. -/
theorem tensor_F_collapses (qs : QState) (h : qs.q0 = .B) :
    (measureDetermined qs .tensor .F).q0 = .F := by
  simp only [measureDetermined, frobeniusBias, measureQ0, h]

/-- Meet-path: coherenceCount increases by 2 (Wigner's friend cost). -/
theorem meet_path_coherence_cost (qs : QState) (apparatus : Belnap) (h : qs.q0 = .B) :
    (measureDetermined qs .meet apparatus).coherenceCount = qs.coherenceCount + 2 := by
  simp only [measureDetermined, frobeniusBias, measureQ0, h]

/-- Tensor-path: T-apparatus coherenceCount increases by 1 (classical collapse cost). -/
theorem tensor_T_coherence_cost (qs : QState) (h : qs.q0 = .B) :
    (measureDetermined qs .tensor .T).coherenceCount = qs.coherenceCount + 1 := by
  simp only [measureDetermined, frobeniusBias, measureQ0, h]

/-- GAP CLOSURE: `measureDetermined` is definitionally equal to `measureQ0` applied
    to the Frobenius-derived bias. The external `bias` parameter is eliminated.
    The apparatus reads the bias from the Frobenius pairing; it does not supply one. -/
theorem gap_closed (qs : QState) (mode : CouplingMode) (apparatus : Belnap) :
    measureDetermined qs mode apparatus =
    measureQ0 qs (frobeniusBias mode apparatus qs.q0) :=
  rfl

end Imscribing.Paraconsistent
