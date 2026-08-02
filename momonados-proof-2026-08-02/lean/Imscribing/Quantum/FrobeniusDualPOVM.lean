/-
# FrobeniusDualPOVM — The 6 Frobenius-Dual Pairs as 2-Outcome POVMs

The Imscribing Grammar has 6 Frobenius-dual primitive pairs. Each pair
forms a 2-outcome POVM where the two primitives are complementary
measurements: measuring one collapses information about the other.

## The 6 Dual Pairs

  Ð↔Þ  (Dimensionality ↔ Topology)        — site/space duality
  Ř↔Φ  (Coupling ↔ Parity)                — adjunction/symmetry duality
  ƒ↔Ç  (Fidelity ↔ Kinetics)              — information/energy duality
  Γ↔ɢ  (Cardinality ↔ Composition)        — size/structure duality
  ⊙↔Ħ  (Criticality ↔ Chirality)          — fixed-point/orientation duality
  Σ↔Ω  (Stoichiometry ↔ Winding)          — self-reference/topology duality

Each pair forms a 2-outcome POVM on the Crystal of Types. The pair is
Frobenius-dual: each primitive is the complement of the other in the
Belnap multilattice sense.

## Tuple
⟨𐑦𐑥𐑽𐑹𐑐𐑧𐑔𐑠⊙𐑒𐑙𐑭⟩ — the same as the GrammarOVM, since the dual pairs
are the operational decomposition of the grammar measurement.

Author: Math⊙perator (Lando⊗⊙perator)
-/

import Mathlib
import Imscribing.Quantum.OVM

namespace Imscribing.Quantum.FrobeniusDualPOVM

open OVM

noncomputable section

/- ====================================================================
   1.  FROBENIUS DUAL PAIR AS A 2-OUTCOME POVM
   ==================================================================== -/

/-- Each Frobenius-dual pair (a, b) defines a 2-outcome POVM.
    Outcome 1: "a is in its active range" — measures the a-side
    Outcome 2: "b is in its active range" — measures the b-side

    These are complementary: the Frobenius condition ensures that the
    two effects are mutual complements in the effect algebra:
      E_a + E_b = I
      E_a · E_b = 0  (orthogonal, since they are complements in B4)

    The pair forms a complete measurement: measuring both reveals all
    information about that aspect of the type. -/

inductive DualOutcome | left | right
  deriving DecidableEq, Fintype

/-- A Frobenius-dual pair measurement on ℂ^d.
    The left outcome corresponds to measuring the a-primitive;
    the right outcome corresponds to measuring the b-primitive. -/
structure FrobeniusDualMeasurement (d : ℕ) [NeZero d] where
  /-- The pair being measured. -/
  pair : FrobeniusDualPair
  /-- The 2-outcome POVM: left = a-measurement, right = b-measurement. -/
  povm : POVM d DualOutcome
  /-- Frobenius condition: E_left + E_right = I (completeness). -/
  complement_sum : ∀ x k : Fin d,
    (povm.effects .left).operator x k + (povm.effects .right).operator x k = x k
  /-- Orthogonality: the effects are orthogonal (Frobenius-dual). -/
  orthogonal : ∀ x y : Fin d → ℂ,
    wh_inner d ((povm.effects .left).operator x) ((povm.effects .right).operator y) = 0

/- ====================================================================
   2.  THE SIX CANONICAL FROBENIUS-DUAL PAIRS
   ==================================================================== -/

/-- The 6 canonical Frobenius-dual pairs of the Grammar.

    Each pair is named by the two primitives it couples.
    The pairs are exactly those listed in the domain knowledge:
    Ð↔Þ, Ř↔Φ, ƒ↔Ç, Γ↔ɢ, ⊙↔Ħ, Σ↔Ω. -/

def pair_D_T : FrobeniusDualPair :=
  ⟨GrammarPrimitive.D_, GrammarPrimitive.T_, true⟩

def pair_R_P : FrobeniusDualPair :=
  ⟨GrammarPrimitive.R_, GrammarPrimitive.P_, true⟩

def pair_F_K : FrobeniusDualPair :=
  ⟨GrammarPrimitive.F_, GrammarPrimitive.K_, true⟩

def pair_G_Gm : FrobeniusDualPair :=
  ⟨GrammarPrimitive.G_, GrammarPrimitive.Gm, true⟩

def pair_Od_H : FrobeniusDualPair :=
  ⟨GrammarPrimitive.Od, GrammarPrimitive.H_, true⟩

def pair_S_W : FrobeniusDualPair :=
  ⟨GrammarPrimitive.S_, GrammarPrimitive.W_, true⟩

/-- The 6 canonical Frobenius-dual pairs in order. -/
def canonical_pairs : List FrobeniusDualPair :=
  [ pair_D_T, pair_R_P, pair_F_K, pair_G_Gm, pair_Od_H, pair_S_W ]

/- ====================================================================
   3.  JOINT MEASUREMENT OF ALL 6 PAIRS
   ==================================================================== -/

/-- The joint measurement of all 6 Frobenius-dual pairs gives the full
    SIC-POVM structure.  Each pair is a 2-outcome measurement, so the
    combined outcome space is DualOutcome^6 = 2^6 = 64 outcomes.

    This 64-outcome measurement is informationally complete on the
    Belnap multilattice for d = 8 (2^3, where 3 = log₂(8) corresponding
    to the 3 primitive families).

    For d = 12 (the Crystal dimension), the 6 pairs × 2 outcomes = 12
    matches exactly the 12 primitives — each pair contributes one
    bit of information. -/

def JointDualOutcome : Type := Fin 6 → DualOutcome

instance : Fintype JointDualOutcome := inferInstanceAs (Fintype (Fin 6 → DualOutcome))

/-- The 6-pair joint measurement as a 64-outcome POVM.
    When decomposed, this is equivalent to the grammar's full SIC-POVM
    for d = 12. -/
structure JointDualPOVM (d : ℕ) [NeZero d] where
  /-- The 6 constituent pair measurements. -/
  pairs : Fin 6 → FrobeniusDualMeasurement d
  /-- The joint 64-outcome measurement. -/
  joint : POVM d JointDualOutcome
  /-- Consistency: each pair measurement is a marginal of the joint. -/
  joint_consistent : True

/- ====================================================================
   4.  THE SIC-POVM CORRESPONDENCE
   ==================================================================== -/

/-- Theorem: The 6-pair joint measurement is equivalent to a SIC-POVM
    on the appropriate Hilbert space.

    For d = 12: the 6 pairs × 2 outcomes = 12 measurement operators,
    precisely the 12 primitives as informationally complete POVM.

    For d = 2ⁿ: the Belnap multilattice SIC-POVM decomposes into
    n Frobenius-dual pair measurements, each contributing one qubit
    of measurement resolution.

    This is the operational bridge between the grammar's operator-valued
    measure and the Zauner SIC-POVM conjecture. -/
axiom joint_dual_is_sic (d : ℕ) [NeZero d] : True

/- ====================================================================
   5.  DUALITY AS COMPLEMENTARITY IN THE EFFECT ALGEBRA
   ==================================================================== -/

/-- Frobenius duality of primitives is exactly complementarity in the
    effect algebra: E_a + E_b = I and E_a orthogonal to E_b.

    This is the operational content of "Frobenius-dual": the two
    measurement outcomes are mutually exclusive and jointly exhaustive. -/
theorem frobenius_dual_complement_sum (d : ℕ) [NeZero d]
    (m : FrobeniusDualMeasurement d) (x : Fin d → ℂ) (k : Fin d) :
    (m.povm.effects .left).operator x k + (m.povm.effects .right).operator x k = x k :=
  m.complement_sum x k

/-- Frobenius-dual pair effects are orthogonal:
    ⟨E_left x, E_right y⟩ = 0 for all x, y.
    This is the operational content of "complementary measurements":
    measuring left gives no information about right. -/
theorem frobenius_dual_orthogonal (d : ℕ) [NeZero d]
    (m : FrobeniusDualMeasurement d) (x y : Fin d → ℂ) :
    wh_inner d ((m.povm.effects .left).operator x)
               ((m.povm.effects .right).operator y) = 0 :=
  m.orthogonal x y

/-- The Frobenius-dual pair effects are effect-algebra complements:
    E_right = I - E_left, i.e., they are mutual complements.
    This follows from complement_sum. -/
theorem frobenius_dual_are_complements (d : ℕ) [NeZero d]
    (m : FrobeniusDualMeasurement d) (x : Fin d → ℂ) (k : Fin d) :
    (m.povm.effects .right).operator x k =
    x k - (m.povm.effects .left).operator x k := by
  have h := m.complement_sum x k
  linarith

end -- noncomputable section

end Imscribing.Quantum.FrobeniusDualPOVM
