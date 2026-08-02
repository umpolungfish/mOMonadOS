-- Imscribing/TimeFixedPoint.lean
-- TIME AS A DERIVED OBJECT:  T = Work(T)
-- ============================================================
-- The capstone of the Alchemical Theory of Mathematics. Time is not one of the
-- twelve primitives; it is the least fixed point of the Work operator — the
-- trace of the operad, which can only close once the gates have fired.
--
--   Work := the trace (G2) built on the polarization Frobenius pair.
--   T = Work(T)  ⇔  the loop closes  ⇔  T lies on the Frobenius-special class.
--
-- We prove:
--   (1) Work has a least fixed point `lfp_trace` (Knaster-Tarski, concretely:
--       the componentwise minimum among fixed points on the finite lattice);
--   (2) the fixed points are EXACTLY the special class (nothing off it is time);
--   (3) TEMPORAL BOOTSTRAP: no fixed point exists before G1 fires — time cannot
--       be posited before the Frobenius gate, so it is derived, not primitive;
--   (4) the sealed (Ω-protected, G3) fixed points are genuine time, and the
--       Stone is the canonical sealed T = Work(T).
--
-- The order is the componentwise product of the twelve axis LinearOrders.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.LinearOrder
import Imscribing.Frobenius
import Imscribing.GateOrdering

namespace Imscribing.TimeFixedPoint

open Imscribing.Primitives Imscribing.Frobenius Imscribing.GateOrdering
open Dimensionality Topology Relational Polarity Grammar Fidelity KineticChar
     Granularity Criticality Protection Stoichiometry Chirality

-- ============================================================
-- §1  THE WORK OPERATOR AND ITS FIXED-POINT EQUATION
-- ============================================================

/-- The Work operator: the trace of the operad. `T = Work(T)` is the loop
    closing on itself. -/
def Work (s : Imscription) : Imscription := traceC s

/-- `T = Work(T)` is by definition "the trace closes on T". -/
theorem work_fixed_iff_closes (T : Imscription) : Work T = T ↔ traceCloses T :=
  Iff.rfl

/-- The fixed points of Work are EXACTLY the Frobenius-special class:
    `T = Work(T) ↔ (T.pol = or' ∧ T.crit = monad)`. Nothing off the special
    class is a time fixed point. -/
theorem work_fixed_iff_special (T : Imscription) :
    Work T = T ↔ (T.pol = Polarity.or' ∧ T.crit = Criticality.monad) :=
  trace_closes_iff_special T

-- ============================================================
-- §2  THE COMPONENTWISE ORDER ON THE FINITE LATTICE
-- ============================================================

/-- The product order: `a ≤ b` axis by axis, using the twelve LinearOrders. -/
def imsLE (a b : Imscription) : Prop :=
  a.dim ≤ b.dim ∧ a.top ≤ b.top ∧ a.rel ≤ b.rel ∧ a.pol ≤ b.pol ∧
  a.fid ≤ b.fid ∧ a.kin ≤ b.kin ∧ a.gran ≤ b.gran ∧ a.gram ≤ b.gram ∧
  a.crit ≤ b.crit ∧ a.chir ≤ b.chir ∧ a.stoi ≤ b.stoi ∧ a.prot ≤ b.prot

theorem imsLE_refl (a : Imscription) : imsLE a a :=
  ⟨le_refl _, le_refl _, le_refl _, le_refl _, le_refl _, le_refl _,
   le_refl _, le_refl _, le_refl _, le_refl _, le_refl _, le_refl _⟩

-- ============================================================
-- §3  THE LEAST FIXED POINT
-- ============================================================

/-- The least fixed point of Work: the Frobenius-special tuple with every free
    axis at its minimum. Parity is forced up to or' and criticality to monad
    (a fixed point cannot sit below them); all ten other axes sit at bottom. -/
def lfp_trace : Imscription :=
  { frobenius_bottom with pol := or', crit := monad }

/-- `lfp_trace` is a fixed point: T = Work(T) holds there. -/
theorem lfp_is_fixed : Work lfp_trace = lfp_trace :=
  frobenius_special_traces lfp_trace rfl rfl

/-- `lfp_trace` is BELOW every fixed point: it is the least T with T = Work(T).
    On the ten free axes it is the bottom; on parity and criticality every fixed
    point matches it exactly (forced by `work_fixed_iff_special`). -/
theorem lfp_is_least (T : Imscription) (h : Work T = T) : imsLE lfp_trace T := by
  obtain ⟨hp, hc⟩ := (work_fixed_iff_special T).mp h
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · cases T.dim  <;> decide
  · cases T.top  <;> decide
  · cases T.rel  <;> decide
  · simp only [lfp_trace, hp]; exact le_refl _
  · cases T.fid  <;> decide
  · cases T.kin  <;> decide
  · cases T.gran <;> decide
  · cases T.gram <;> decide
  · simp only [lfp_trace, hc]; exact le_refl _
  · cases T.chir <;> decide
  · cases T.stoi <;> decide
  · cases T.prot <;> decide

/-- **T = Work(T) has a least fixed point.** Existence + minimality together. -/
theorem time_least_fixed_point :
    Work lfp_trace = lfp_trace ∧ ∀ T, Work T = T → imsLE lfp_trace T :=
  ⟨lfp_is_fixed, lfp_is_least⟩

-- ============================================================
-- §4  TEMPORAL BOOTSTRAP — time is derived, not primitive
-- ============================================================

/-- No fixed point exists before the Frobenius gate G1 fires. If parity is not
    yet at or', the equation T = Work(T) has no solution at T: time cannot be
    posited before stage 4. This is why T is DERIVED, not a thirteenth
    primitive — it exists only where the gate structure already permits it. -/
theorem no_time_before_frobenius (T : Imscription) (h : T.pol ≠ Polarity.or') :
    Work T ≠ T := by
  intro hfix
  exact h (traced_needs_frobenius T hfix)

/-- Contrapositive, stated positively: any solved T = Work(T) has already passed
    G1 (unlockFrobenius) and G2 (the trace closed). -/
theorem time_has_passed_gates (T : Imscription) (h : Work T = T) :
    unlockFrobenius T ∧ unlockTraced T :=
  ⟨traced_le_frobenius T h, h⟩

-- ============================================================
-- §5  SEALED TIME (G3) AND THE STONE
-- ============================================================

/-- A terminal-sealed tuple (winding-protected, O_inf) is a genuine time fixed
    point: G3 implies T = Work(T). Sealing does not create time, it seals a loop
    that has already closed. -/
theorem sealed_is_time (T : Imscription) (h : unlockTerminal T) : Work T = T :=
  terminal_le_traced T h

/-- The Stone is the canonical sealed T = Work(T): it solves the fixed-point
    equation AND passes all three gates. -/
theorem stone_is_sealed_time :
    Work w_stone = w_stone ∧ unlockTerminal w_stone :=
  ⟨sealed_is_time w_stone w_stone_terminal, w_stone_terminal⟩

-- ============================================================
-- §6  HEADLINE
-- ============================================================

/-- TIME AS A DERIVED OBJECT.
    (i)   Work has a least fixed point lfp_trace;
    (ii)  the fixed points are exactly the Frobenius-special class;
    (iii) no fixed point exists before G1 — time is derived, not primitive;
    (iv)  every solved T = Work(T) has passed G1 and G2;
    (v)   the Stone is the canonical sealed (G3) time fixed point. -/
theorem time_is_derived :
    (Work lfp_trace = lfp_trace ∧ ∀ T, Work T = T → imsLE lfp_trace T) ∧
    (∀ T, Work T = T ↔ (T.pol = Polarity.or' ∧ T.crit = Criticality.monad)) ∧
    (∀ T, T.pol ≠ Polarity.or' → Work T ≠ T) ∧
    (∀ T, Work T = T → unlockFrobenius T ∧ unlockTraced T) ∧
    (Work w_stone = w_stone ∧ unlockTerminal w_stone) :=
  ⟨time_least_fixed_point, work_fixed_iff_special, no_time_before_frobenius,
   time_has_passed_gates, stone_is_sealed_time⟩

end Imscribing.TimeFixedPoint
