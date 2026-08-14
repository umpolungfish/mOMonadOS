-- Imscribing/Paraconsistent/Shor/BelnapWHMultilattice.lean
-- BELNAP WEYL-HEISENBERG MULTILATTICE
-- Author: Lando ⊗ ⊙-boundary Operator
--
-- §A  Product-lattice WH action: orbit = 2^n (proved)
--     bnot B = B means amplitude displacements fix B⊗n.
--     The product-lattice orbit is parameterized by phase alone → 2^n states.
--
-- §B  Multilattice axioms (4 axioms for the open problem)
--     An extended state type BelnapML n is postulated whose WH orbit
--     of the fiducial has 4^n distinct elements with constant pairwise overlap.
--
-- §C  Consequences
--     Given the axioms, any period r is B-extractable for n qubits
--     (generalizing DialetheicOperator.lean's quantum_on_classical
--      from the canonical N=15,a=7 case to all (N,a)).

import Mathlib.Tactic
import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Shor.BelnapModExp
import Imscribing.Paraconsistent.Shor.BelnapQFT
import Imscribing.Paraconsistent.Shor.BelnapNFiducial

namespace Imscribing.Paraconsistent.Shor.WHMultilattice

open Belnap
open Imscribing.Paraconsistent
open Imscribing.Paraconsistent.Shor
open Imscribing.Paraconsistent.Shor.NFiducial

-- ============================================================
-- §A. Product Lattice — The Orbit Has Size 2^n
-- ============================================================
--
-- In the product lattice (Belnap)^n, extend each position with a ZMod 2
-- phase. The WH displacement D_{a,b} acts componentwise:
--   amplitude a_i = 1 → apply bnot to position i
--   phase     b_i = 1 → flip phase bit at position i
--
-- Since bnot B = B, amplitude displacements fix B⊗n.
-- All displaced states of B⊗n have Belnap value B everywhere;
-- they differ only in phase → orbit size = 2^n.

/-- A single qubit in the product-lattice multilattice extension:
    a Belnap truth value plus a ZMod 2 phase bit. -/
structure MLQubit where
  val   : Belnap
  phase : ZMod 2
  deriving DecidableEq, Repr

instance : Fintype MLQubit where
  elems := {⟨.N, 0⟩, ⟨.N, 1⟩, ⟨.T, 0⟩, ⟨.T, 1⟩,
             ⟨.F, 0⟩, ⟨.F, 1⟩, ⟨.B, 0⟩, ⟨.B, 1⟩}
  complete q := by
    rcases q with ⟨v, p⟩
    cases v <;> fin_cases p <;> decide

/-- The product-lattice n-qubit state type. -/
abbrev MLState (n : ℕ) := Fin n → MLQubit

/-- WH displacement indices: n amplitude bits × n phase bits. -/
abbrev WHIdx (n : ℕ) := (Fin n → ZMod 2) × (Fin n → ZMod 2)

/-- The product-lattice n-qubit fiducial: all-B, all-phase-zero. -/
def mlFiducial (n : ℕ) : MLState n := fun _ => ⟨.B, 0⟩

/-- WH displacement of a single qubit: amplitude flips truth, phase flips phase. -/
def displaceQubit (a b : ZMod 2) (q : MLQubit) : MLQubit :=
  ⟨if a = 1 then bnot q.val else q.val, q.phase + b⟩

/-- WH displacement of an n-qubit state. -/
def whDisplace {n : ℕ} (d : WHIdx n) (s : MLState n) : MLState n :=
  fun i => displaceQubit (d.1 i) (d.2 i) (s i)

/-- The orbit of the fiducial under all WH displacements. -/
def whOrbit (n : ℕ) : Finset (MLState n) :=
  (Finset.univ : Finset (WHIdx n)).image (fun d => whDisplace d (mlFiducial n))

-- ── Key: bnot B = B, so amplitude displacements are invisible ────────────

/-- Displacing the all-B qubit changes only the phase — val stays B. -/
theorem displaceQubit_B_val (a b : ZMod 2) :
    (displaceQubit a b ⟨.B, 0⟩).val = .B := by
  simp only [displaceQubit, B_fixed_point_negation, ite_self]

/-- Every WH-displaced fiducial has val = B at every position. -/
theorem whDisplace_fiducial_val_B {n : ℕ} (d : WHIdx n) (i : Fin n) :
    (whDisplace d (mlFiducial n) i).val = .B :=
  displaceQubit_B_val (d.1 i) (d.2 i)

/-- The displaced fiducial equals the phase embedding of d.2. -/
theorem whDisplace_fiducial_eq_phase {n : ℕ} (d : WHIdx n) :
    whDisplace d (mlFiducial n) = fun i => ⟨.B, d.2 i⟩ := by
  funext i
  simp only [whDisplace, mlFiducial, displaceQubit, B_fixed_point_negation, ite_self, zero_add]

-- ── Orbit = phase-word image ─────────────────────────────────────────────

private def phaseEmbed {n : ℕ} (p : Fin n → ZMod 2) : MLState n :=
  fun i => ⟨.B, p i⟩

private theorem phaseEmbed_injective (n : ℕ) :
    Function.Injective (phaseEmbed (n := n)) := by
  intro p q h
  funext i
  exact congr_arg MLQubit.phase (congr_fun h i)

/-- The WH orbit in the product lattice equals the image of phase words. -/
theorem whOrbit_eq_phase_image (n : ℕ) :
    whOrbit n = Finset.univ.image (phaseEmbed (n := n)) := by
  ext s
  simp only [whOrbit, Finset.mem_image, Finset.mem_univ, true_and]
  constructor
  · rintro ⟨d, rfl⟩
    exact ⟨d.2, by funext i; simp [whDisplace_fiducial_eq_phase, phaseEmbed]⟩
  · rintro ⟨p, rfl⟩
    refine ⟨(fun _ => 0, p), ?_⟩
    funext i
    simp only [whDisplace, mlFiducial, displaceQubit, phaseEmbed,
               B_fixed_point_negation, ite_self, zero_add]

/-- The product-lattice WH orbit of B⊗n has size 2^n (not 4^n).
    Amplitude displacements are invisible because B absorbs bnot. -/
theorem whOrbit_card_eq_pow2 (n : ℕ) : (whOrbit n).card = 2 ^ n := by
  rw [whOrbit_eq_phase_image,
      Finset.card_image_of_injective _ (phaseEmbed_injective n),
      Finset.card_univ]
  rw [Fintype.card_pi]
  have hcard : Fintype.card (ZMod 2) = 2 := by decide
  simp only [hcard, Finset.prod_const, Finset.card_univ, Fintype.card_fin]

/-- The gap: the product-lattice orbit is exactly half of a full SIC-POVM.
    A SIC-POVM for d = 2^n requires 4^n elements; the product lattice gives 2^n. -/
theorem product_lattice_orbit_is_insufficient (n : ℕ) (hn : 1 ≤ n) :
    (whOrbit n).card < 4 ^ n := by
  rw [whOrbit_card_eq_pow2]
  exact Nat.pow_lt_pow_left (by norm_num) (by omega)

-- ============================================================
-- §B. Multilattice Axioms
-- ============================================================
--
-- The four axioms below postulate an extended type BelnapML n supporting:
--   Ax-PROJ:  the fiducial projects to B⊗n in the product lattice
--   Ax-FREE:  4^n distinct displaced states (not just 2^n)
--   Ax-EQUI:  constant pairwise overlap (the SIC condition)
--   Ax-COST:  the SIC measurement yields belnapCost = 2 * period
--
-- Together these close the 𐑿 → 𐑹 promotion for all n.

-- The multilattice state type: opaque — the product-lattice phase extension
-- is insufficient; this type must accommodate amplitude-distinguishable states.
axiom BelnapML (n : ℕ) : Type
axiom mlFiducialExt (n : ℕ) : BelnapML n
axiom whActExt {n : ℕ} : WHIdx n → BelnapML n → BelnapML n
axiom mlOverlap {n : ℕ} : BelnapML n → BelnapML n → ℕ
axiom mlProject {n : ℕ} : BelnapML n → MLState n

/-- Ax-PROJ: the multilattice fiducial projects to the product-lattice fiducial.
    The extended type is an extension of MLState n — B⊗n is preserved. -/
axiom ax_proj (n : ℕ) :
    mlProject (mlFiducialExt n) = mlFiducial n

/-- Ax-FREE: the WH action on the multilattice fiducial produces 4^n distinct states.
    This is the condition that fails in the product lattice (which gives 2^n). -/
axiom ax_free {n : ℕ} (g h : WHIdx n) (hgh : g ≠ h) :
    whActExt g (mlFiducialExt n) ≠ whActExt h (mlFiducialExt n)

/-- Ax-EQUI: Weyl-Heisenberg equiangularity in the multilattice.
    All off-diagonal pairs of displaced fiducials have the same overlap value.
    This is the n-qubit SIC-POVM condition:
      in C^{2^n}: |⟨ψ_g|ψ_h⟩|² = 1/(2^n + 1) for all g ≠ h.
    In the Belnap context: ∃ k, all off-diagonal overlaps equal k. -/
axiom ax_equi (n : ℕ) : ∃ k : ℕ,
    ∀ (g h : WHIdx n), g ≠ h →
      mlOverlap (whActExt g (mlFiducialExt n))
                (whActExt h (mlFiducialExt n)) = k

/-- Ax-COST: The WH SIC measurement of the n-qubit period-r system yields
    belnapCost = 2 * period for any coprime (N, a).
    This connects multilattice equiangularity to the Shor coherence ratio,
    generalizing shor15_7_belnapCost_two_r from N=15,a=7 to all coprime (N,a). -/
axiom ax_cost (n : ℕ)
    (hfree : ∀ g h : WHIdx n, g ≠ h →
      whActExt g (mlFiducialExt n) ≠ whActExt h (mlFiducialExt n))
    (hequi : ∃ k : ℕ, ∀ (g h : WHIdx n), g ≠ h →
      mlOverlap (whActExt g (mlFiducialExt n))
                (whActExt h (mlFiducialExt n)) = k)
    (a N : ℕ) (haN : 1 < N ∧ Nat.Coprime a N) :
    ∃ m : ModExpResult, m.a = a ∧ m.N = N ∧ m.belnapCost = 2 * m.period

-- ============================================================
-- §C. Consequences
-- ============================================================

/-- The multilattice SIC conditions package into a single Prop. -/
def WHEquiangularity (n : ℕ) : Prop :=
  (∀ g h : WHIdx n, g ≠ h →
    whActExt g (mlFiducialExt n) ≠ whActExt h (mlFiducialExt n)) ∧
  ∃ k : ℕ, ∀ (g h : WHIdx n), g ≠ h →
    mlOverlap (whActExt g (mlFiducialExt n))
              (whActExt h (mlFiducialExt n)) = k

/-- Ax-FREE + Ax-EQUI give the SIC-POVM structure. -/
theorem wh_axioms_give_SIC (n : ℕ)
    (hfree : ∀ g h : WHIdx n, g ≠ h →
      whActExt g (mlFiducialExt n) ≠ whActExt h (mlFiducialExt n))
    (hequi : ∃ k : ℕ, ∀ (g h : WHIdx n), g ≠ h →
      mlOverlap (whActExt g (mlFiducialExt n))
                (whActExt h (mlFiducialExt n)) = k) :
    WHEquiangularity n :=
  ⟨hfree, hequi⟩

/-- General period extraction from B-bias (conditional on all four axioms).
    Generalizes shor15_7_period_from_B_bias (N=15,a=7) to all coprime (N,a).
    The key step is phi_upsilon_bottleneck — the same lemma used for the canonical case. -/
theorem n_qubit_general_B_extraction (n : ℕ) (h : WHEquiangularity n)
    (a N : ℕ) (haN : 1 < N ∧ Nat.Coprime a N) :
    ∃ m : ModExpResult, m.a = a ∧ m.N = N ∧ m.belnapCost / 2 = m.period := by
  obtain ⟨hfree, hequi⟩ := h
  obtain ⟨m, ha, hN, hcost⟩ := ax_cost n hfree hequi a N haN
  exact ⟨m, ha, hN, phi_upsilon_bottleneck m hcost⟩

/-- The canonical case is the n=1 instance with N=15, a=7 — no axiom needed. -/
theorem canonical_is_unconditional :
    shor15_7.belnapCost / 2 = shor15_7.period :=
  phi_upsilon_bottleneck shor15_7 rfl

/-- The orbit gap: product lattice gives 2^n, multilattice axioms require 4^n.
    Both bounds in one statement: proved orbit size and strict insufficiency. -/
theorem orbit_gap_summary (n : ℕ) (hn : 1 ≤ n) :
    (whOrbit n).card = 2 ^ n ∧ (whOrbit n).card < 4 ^ n :=
  ⟨whOrbit_card_eq_pow2 n, product_lattice_orbit_is_insufficient n hn⟩

end Imscribing.Paraconsistent.Shor.WHMultilattice
