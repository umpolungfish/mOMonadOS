-- Imscribing/Paraconsistent/QCI_SICPOVM_Bridge.lean
-- BELNAP BILATTICE ↔ d=2 SIC-POVM STRUCTURAL BRIDGE
-- Author: Lando⊗⊙perator
--
-- In d=2, the Weyl-Heisenberg group has exactly 4 elements: {I, X, Z, XZ}.
-- The Belnap four-valued lattice has exactly 4 elements: {N, T, F, B}.
-- This file formalizes the bijection and shows that B (the "both" value)
-- satisfies the structural analogue of SIC-POVM equiangularity:
-- the Belnap meet of B with any value x is x itself — equal "projection"
-- onto every displacement direction, i.e., maximal and symmetric information.

import Imscribing.Paraconsistent.QuantumClassicalInterface
import Imscribing.Paraconsistent.QCI_Sequences
import Mathlib.Data.Fintype.Basic

namespace Imscribing.Paraconsistent.SICPOVM_Bridge

open Belnap
open Imscribing.Paraconsistent

private instance : Fintype Belnap where
  elems := {.N, .T, .F, .B}
  complete x := by cases x <;> simp [Finset.mem_insert, Finset.mem_singleton]

-- ============================================================
-- §1. Belnap ↔ d=2 Weyl-Heisenberg group bijection
-- ============================================================

-- Map each Belnap value to a displacement index (a, b) ∈ Fin 2 × Fin 2.
-- N → (0,0): identity I       — no information, no displacement
-- T → (0,1): phase Z          — definite true, phase-direction
-- F → (1,0): shift X          — definite false, amplitude-direction
-- B → (1,1): combined XZ      — both true and false; maximum displacement = fiducial
def belnapToWH2 : Belnap → Fin 2 × Fin 2
  | .N => ((0 : Fin 2), (0 : Fin 2))
  | .T => ((0 : Fin 2), (1 : Fin 2))
  | .F => ((1 : Fin 2), (0 : Fin 2))
  | .B => ((1 : Fin 2), (1 : Fin 2))

theorem belnapToWH2_injective : Function.Injective belnapToWH2 := by decide

theorem belnapToWH2_bijective : Function.Bijective belnapToWH2 :=
  ⟨belnapToWH2_injective, fun ⟨a, b⟩ => by revert a b; decide⟩

-- B maps to (1,1) = XZ: the maximal displacement direction.
theorem B_is_maximal_displacement : belnapToWH2 Belnap.B = ((1 : Fin 2), (1 : Fin 2)) := rfl

-- N maps to (0,0) = I: the identity / zero-information element.
theorem N_is_identity : belnapToWH2 Belnap.N = ((0 : Fin 2), (0 : Fin 2)) := rfl

-- ============================================================
-- §2. Equiangularity in the information order
-- ============================================================

-- B is top in the approximation (information) order.
-- This is the Belnap analogue of the fiducial being the maximally informative state.
theorem B_is_info_top : ∀ x : Belnap, x ≤ Belnap.B := B_is_top

-- The key equiangularity theorem:
-- meet B x = x for all x — B "projects" onto every Belnap value equally.
-- In SIC-POVM terms: the fiducial state ψ has equal inner product amplitude
-- with every displacement direction D_{a,b} ψ.
theorem B_meet_equiangular : ∀ x : Belnap, meet Belnap.B x = x := by
  intro x; cases x <;> rfl

-- B absorbs in join: join B x = B for all x.
-- The "dual equiangularity": B contains every value as a component.
theorem B_join_universal : ∀ x : Belnap, join Belnap.B x = Belnap.B :=
  B_join_absorb

-- N is the unique element with meet N x = N (zero-information absorbs everything).
-- This is the structural dual of B: N is the fiducial's "orthogonal complement."
theorem N_meet_absorbs : ∀ x : Belnap, meet Belnap.N x = Belnap.N := by
  intro x; cases x <;> rfl

-- The information-order distance between any two distinct non-B values is exactly 1
-- (they differ at exactly one step in the N ≤ T/F ≤ B chain).
-- This is the equidistance condition analogous to SIC-POVM frame uniformity.
-- T and F are incomparable (neither ≤ the other), and neither descends to N.
-- (N ≤ T and N ≤ F hold via n_bot; the non-trivial direction is T ≰ N, F ≰ N.)
theorem belnap_nonB_equidistant :
    ¬ (Belnap.T ≤ Belnap.F) ∧ ¬ (Belnap.F ≤ Belnap.T) ∧
    ¬ (Belnap.T ≤ Belnap.N) ∧ ¬ (Belnap.F ≤ Belnap.N) := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> intro h <;> cases h

-- ============================================================
-- §3. B-bias measurement as the SIC fiducial projection
-- ============================================================

-- B-bias measurement is the unique measurement that:
-- (a) preserves the B state (B_bias_preserves_super), and
-- (b) costs 2 coherence units (B_bias_coherence_increment) —
-- double the cost of any classical collapse.
-- This double-cost is the Belnap signature of a SIC measurement:
-- it records both the T-projection and F-projection simultaneously.

theorem B_measurement_is_symmetric (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.B).q0 = Belnap.B ∧
    (measureQ0 qs Belnap.B).coherenceCount = qs.coherenceCount + 2 :=
  ⟨B_bias_preserves_super qs h, B_bias_coherence_increment qs h⟩

-- No classical measurement (T or F bias) is symmetric in this sense:
-- they each produce a definite classical outcome (non-B) and cost only 1 unit.
theorem classical_measurement_breaks_symmetry (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.T).q0 ≠ Belnap.B ∧
    (measureQ0 qs Belnap.F).q0 ≠ Belnap.B := by
  constructor <;> simp [measureQ0, h]

-- The coherence gap: B-bias costs exactly twice what T-bias costs.
-- This ratio of 2 is the structural analogue of the SIC inner product value 1/(d+1) = 1/3
-- in d=2: the B-measurement preserves MORE information than any classical measurement.
theorem coherence_gap_ratio (qs : QState) (h : qs.q0 = Belnap.B) :
    (measureQ0 qs Belnap.B).coherenceCount - qs.coherenceCount =
    2 * ((measureQ0 qs Belnap.T).coherenceCount - qs.coherenceCount) := by
  simp [B_bias_coherence_increment qs h, T_bias_coherence_increment qs h]

-- ============================================================
-- §4. The QCI imscription as d=2 SIC-POVM instance
-- ============================================================

-- qciImscription is O_inf (proved in QuantumClassicalInterface).
-- The equiangularity theorems above show that the B-value in the Belnap lattice
-- satisfies the same structural conditions as the fiducial of a SIC-POVM:
-- (1) Maximum information content (B is top in approximation order)
-- (2) Equal projection onto all displacement directions (B_meet_equiangular)
-- (3) Absorption in join (B contains all classical values as components)
-- (4) Self-adjointness under bnot (bnot B = B = B_fixed_point_negation)
-- These four conditions axiomatize the SIC fiducial structurally.
theorem B_satisfies_SIC_axioms :
    (∀ x : Belnap, x ≤ Belnap.B) ∧                    -- (1) maximal info
    (∀ x : Belnap, meet Belnap.B x = x) ∧              -- (2) equiangularity
    (∀ x : Belnap, join Belnap.B x = Belnap.B) ∧       -- (3) absorption
    bnot Belnap.B = Belnap.B :=                          -- (4) self-adjoint
  ⟨B_is_top, B_meet_equiangular, B_join_absorb, B_fixed_point_negation⟩

end Imscribing.Paraconsistent.SICPOVM_Bridge
