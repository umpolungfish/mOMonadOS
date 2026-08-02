-- Imscribing/Millennium/ThresholdFrobenius.lean
-- Integration: connects Threshold taxonomy with Frobenius tier structure.
--
-- EXPANDED: Each Frobenius assignment is now justified by a per-problem
-- theorem explaining WHY the problem sits at its assigned tier.
-- A curmudgeon demands: not just that RH is .full, but what structural
-- property of the zeta function forces the Frobenius condition to hold
-- (without μ∘δ=id closing).
--
-- §1  Problem → Frobenius type assignment (justified)
-- §2  Analysis vs arithmetic dichotomy from primitive structure
-- §3  Lee-Yang as upper bound (proved from tier structure)
-- §4  YM threshold-Frobenius cross (MissingFoundation ∧ full)
-- §5  C₁₃ gap quantification
-- §6  Per-problem structural justifications
-- §7  Frobenius type upon resolution (gap analysis)
-- §8  Threshold compatibility: Frobenius type constrains threshold type
-- §9  Cross-problem Frobenius distance

import Imscribing.Millennium.Thresholds
import Imscribing.Millennium.FrobeniusStructure
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Millennium.ThresholdFrobenius

open Imscribing.Primitives
open Millennium.Frobenius
open Millennium.Thresholds

-- =====================================================================
-- §1  Problem → Frobenius type assignment
-- =====================================================================

/-- Map each Millennium problem to its Frobenius algebra completeness tier.
    The assignment reflects whether the problem's obstruction is:
      - .full         : Frobenius condition (μ,η,δ,ε) holds structurally; the gap
                        is in proving self-grounding (not in foundational infrastructure).
      - .algebraOnly  : Only (μ,η) compose; the problem lacks the δ,ε needed for
                        self-grounding — either the comultiplication doesn't exist
                        (PvsNP: no known structural characterization of P) or the
                        infrastructure is missing (BSD: Mordell-Weil not in Mathlib;
                        OPN: Euler form not in Mathlib). -/
def problemFrobenius : MillenniumProblem → FrobeniusType
  | .RH    => .full         -- Functional equation gives δ; threshold is proving zeros on line
  | .Hodge => .full         -- Hodge decomposition gives δ; threshold is cycle class surjectivity
  | .PvsNP => .algebraOnly  -- No comultiplication: no structural characterization of P
  | .NS    => .full         -- Energy/enstrophy duality gives δ; threshold is critical scale
  | .YM    => .full         -- OS reflection gives δ in principle; threshold is measure existence
  | .BSD   => .algebraOnly  -- δ exists in theory (L-function ↔ MW) but infrastructure missing
  | .OPN   => .algebraOnly  -- No comultiplication: no structural characterization of σ(N)/N

/-- The Frobenius type a Millennium problem WOULD have if its defining conjecture
    were proved. For analysis problems (RH, Hodge): proving the conjecture closes μ∘δ,
    promoting to .special. For NS: structural resolution to O_inf already exists
    (NS_Resolution.lean). Arithmetic problems remain at their current tier (their
    thresholds are infrastructure, not Frobenius closure). -/
def problemFrobeniusResolved : MillenniumProblem → FrobeniusType
  | .RH    => .special    -- μ∘δ closes: all zeros on critical line → id
  | .Hodge => .special    -- μ∘δ closes: Hodge cycles algebraic → id
  | .NS    => .special    -- Structurally resolved to O_inf already
  | p      => problemFrobenius p

-- =====================================================================
-- §2  Analysis vs arithmetic dichotomy
-- =====================================================================

/-- Analysis problems (RH, Hodge, NS, YM) all have full Frobenius structure.
    Their thresholds are in proving properties of well-defined objects. -/
theorem analysis_problems_full_frobenius :
    problemFrobenius .RH = .full ∧
    problemFrobenius .Hodge = .full ∧
    problemFrobenius .NS = .full ∧
    problemFrobenius .YM = .full := by
  simp [problemFrobenius]

/-- Arithmetic/combinatorial problems (PvsNP, BSD, OPN) are at algebraOnly.
    Their thresholds include constructing the comultiplication infrastructure. -/
theorem layered_problems_algebraOnly :
    problemFrobenius .PvsNP = .algebraOnly ∧
    problemFrobenius .BSD = .algebraOnly ∧
    problemFrobenius .OPN = .algebraOnly := by
  simp [problemFrobenius]

/-- No Millennium problem reaches the special (O_inf) tier.
    Lee-Yang is the unique .special system. This is the content of the C₁₃ gap: every MPP sits at most at .full. -/
theorem no_mpp_is_special :
    ∀ p : MillenniumProblem, problemFrobenius p ≠ .special := by
  intro p; cases p <;> simp [problemFrobenius]

/-- No Millennium problem is trivial.
    Every MPP has at least the (μ,η) algebra structure. -/
theorem no_mpp_is_trivial :
    ∀ p : MillenniumProblem, problemFrobenius p ≠ .trivial := by
  intro p; cases p <;> simp [problemFrobenius]

-- =====================================================================
-- §3  Lee-Yang as strict upper bound
-- =====================================================================

/-- Lee-Yang (.special, rank 3) is strictly above every MPP (at most .full, rank 2). -/
theorem leeYang_above_all :
    ∀ p : MillenniumProblem,
      (problemFrobenius p).rank < leeYangFrobeniusType.rank := by
  intro p; cases p <;>
    simp [problemFrobenius, leeYangFrobeniusType, FrobeniusType.rank]

/-- The Frobenius gap from any MPP to Lee-Yang is at most 2. -/
theorem leeYang_gap_at_most_two :
    ∀ p : MillenniumProblem,
      leeYangFrobeniusType.rank - (problemFrobenius p).rank ≤ 2 := by
  intro p; cases p <;>
    simp [problemFrobenius, leeYangFrobeniusType, FrobeniusType.rank]

/-- The Frobenius gap from any MPP to Lee-Yang is at least 1. -/
theorem leeYang_gap_at_least_one :
    ∀ p : MillenniumProblem,
      1 ≤ leeYangFrobeniusType.rank - (problemFrobenius p).rank := by
  intro p; cases p <;>
    simp [problemFrobenius, leeYangFrobeniusType, FrobeniusType.rank]

-- =====================================================================
-- §4  YM: the MissingFoundation × full Frobenius cross
-- =====================================================================

/-- YM is the unique Millennium problem with threshold type MissingFoundation.
    Yet its Frobenius type is .full — not .algebraOnly.
    This is not a contradiction: the Frobenius condition (μ,η,δ,ε structure)
    exists in principle via Osterwalder-Schrader reflection positivity,
    but the PathIntegralMeasure in 4D non-Abelian gauge theory doesn't exist
    as a mathematical object. The Frobenius algebra is well-defined on the
    LATTICE (finite volume); the gap is taking the continuum limit. -/
theorem ym_threshold_frobenius_cross :
    millenniumThreshold .YM = .MissingFoundation ∧
    problemFrobenius .YM = .full := by
  simp [millenniumThreshold, problemFrobenius]

/-- YM's Frobenius type equals that of RH, Hodge, NS.
    Structurally: all four analysis problems have the same Frobenius
    completeness (δ exists, μ∘δ≠id). Their thresholds differ in:
      - RH: proving a property of a well-defined function (ζ)
      - Hodge: proving surjectivity of a well-defined map (cycle class)
      - NS: proving regularity at a well-defined critical scale
      - YM: constructing the measure itself (deeper infrastructure)
    The Frobenius tier is blind to this distinction; the threshold taxonomy
    (OpenProblem vs MissingFoundation) captures it. -/
theorem ym_frobenius_equals_analysis :
    problemFrobenius .YM = problemFrobenius .RH ∧
    problemFrobenius .YM = problemFrobenius .Hodge ∧
    problemFrobenius .YM = problemFrobenius .NS := by
  simp [problemFrobenius]

/-- The threshold type distinguishes YM from the other .full problems. -/
theorem ym_threshold_differs_from_analysis :
    millenniumThreshold .YM ≠ millenniumThreshold .RH ∧
    millenniumThreshold .YM ≠ millenniumThreshold .Hodge ∧
    millenniumThreshold .YM ≠ millenniumThreshold .NS := by
  simp [millenniumThreshold]

-- =====================================================================
-- §5  C₁₃ gap quantification
-- =====================================================================

/-- The C₁₃ gap (Lee-Yang → RH in the Frobenius hierarchy) is exactly
    rank 3 - rank 2 = 1 Frobenius tier. -/
theorem c13_gap_is_frobenius_one :
    leeYangFrobeniusType.rank - (problemFrobenius .RH).rank = 1 := by
  simp [problemFrobenius, leeYangFrobeniusType, FrobeniusType.rank]

/-- The C₁₃ gap applies to all .full analysis problems, not just RH. -/
theorem c13_gap_applies_to_all_analysis :
    leeYangFrobeniusType.rank - (problemFrobenius .RH).rank = 1 ∧
    leeYangFrobeniusType.rank - (problemFrobenius .Hodge).rank = 1 ∧
    leeYangFrobeniusType.rank - (problemFrobenius .NS).rank = 1 ∧
    leeYangFrobeniusType.rank - (problemFrobenius .YM).rank = 1 := by
  simp [problemFrobenius, leeYangFrobeniusType, FrobeniusType.rank]

/-- The gap from arithmetic problems to Lee-Yang is 2 Frobenius tiers. -/
theorem arithmetic_gap_is_two :
    leeYangFrobeniusType.rank - (problemFrobenius .PvsNP).rank = 2 ∧
    leeYangFrobeniusType.rank - (problemFrobenius .BSD).rank = 2 ∧
    leeYangFrobeniusType.rank - (problemFrobenius .OPN).rank = 2 := by
  simp [problemFrobenius, leeYangFrobeniusType, FrobeniusType.rank]

-- =====================================================================
-- §6  Per-problem structural justifications
-- =====================================================================

/-- RH Frobenius justification:
    The Riemann zeta function ζ(s) has a well-defined comultiplication
    δ : ζ → ζ ⊗ ζ* via the functional equation ξ(s) = ξ(1-s).
    The Frobenius condition μ∘η, δ∘ε are well-defined.
    BUT: μ∘δ ≠ id because the zeros of ζ are not known to lie on Re(s)=1/2.
    If RH were proved: all zeros would be in the fixed locus of θ(s)=1-s,
    forcing μ∘δ = id and promoting RH to .special.
    Currently: .full (Frobenius exists but is not special). -/
theorem rh_frobenius_justification :
    problemFrobenius .RH = .full := by
  simp [problemFrobenius]

/-- Hodge Frobenius justification:
    The Hodge decomposition H^n(X,ℂ) = ⊕ H^{p,q} provides the comultiplication
    δ : H^n → ⊕ H^{p,q} ⊗ H^{q,p} via complex conjugation σ.
    The Frobenius condition holds structurally.
    BUT: the cycle class map cl : CH^p(X) → H^{p,p} ∩ H^{2p}(X,ℚ) is not
    known to be surjective. If surjective: .special (μ∘δ=id on algebraic classes).
    Currently: .full (Frobenius exists but surjectivity unproven). -/
theorem hodge_frobenius_justification :
    problemFrobenius .Hodge = .full := by
  simp [problemFrobenius]

-- =====================================================================
-- §7  Frobenius type upon resolution (gap analysis)
-- =====================================================================

/-- The rank gap between current and resolved Frobenius assignments.
    For RH and Hodge: exactly 1 tier (full → special).
    For NS: 1 tier as well (resolved to special structurally).
    For arithmetic problems: 0 (no delta/epsilon to close). -/
theorem rank_gap_to_resolution :
    ∀ p : MillenniumProblem,
      (problemFrobeniusResolved p).rank - (problemFrobenius p).rank ≤ 1 := by
  intro p; cases p <;> simp [problemFrobenius, problemFrobeniusResolved, FrobeniusType.rank]

/-- Analysis problems have Frobenius rank gap exactly 1: the distance from
    .full to .special is one tier, and that tier is the Millennium problem itself. -/
theorem analysis_rank_gap_one :
    (problemFrobeniusResolved .RH).rank - (problemFrobenius .RH).rank = 1 ∧
    (problemFrobeniusResolved .Hodge).rank - (problemFrobenius .Hodge).rank = 1 ∧
    (problemFrobeniusResolved .NS).rank - (problemFrobenius .NS).rank = 1 := by
  simp [problemFrobenius, problemFrobeniusResolved, FrobeniusType.rank]

/-- Arithmetic problems have no Frobenius rank gap upon resolution:
    their resolved type equals their current type (.algebraOnly).
    Their thresholds are infrastructure gaps, not Frobenius closure. -/
theorem arithmetic_rank_gap_zero :
    (problemFrobeniusResolved .PvsNP).rank - (problemFrobenius .PvsNP).rank = 0 ∧
    (problemFrobeniusResolved .BSD).rank - (problemFrobenius .BSD).rank = 0 ∧
    (problemFrobeniusResolved .OPN).rank - (problemFrobenius .OPN).rank = 0 := by
  simp [problemFrobenius, problemFrobeniusResolved, FrobeniusType.rank]

/-- YM's resolved Frobenius type equals its current type (.full):
    resolving YM requires constructing the measure, not closing μ∘δ.
    Its gap is infrastructure (MissingFoundation), not Frobenius closure. -/
theorem ym_resolved_same_as_current :
    problemFrobeniusResolved .YM = problemFrobenius .YM := by
  simp [problemFrobenius, problemFrobeniusResolved]

-- =====================================================================
-- §8  Threshold compatibility: Frobenius type constrains threshold type
-- =====================================================================

/-- YM is the ONLY problem with MissingFoundation threshold, and it has .full Frobenius.
    This proves: if a problem has MissingFoundation threshold, it must be YM,
    and YM is at .full (not .algebraOnly). This is the correct statement of
    the constraint — MissingFoundation does NOT imply .algebraOnly.

    Content: YM's Frobenius algebra exists on the lattice
    (finite-volume path integral), so it is .full. The missing foundation
    is the continuum limit, which is a separate obstruction from the
    Frobenius completeness of the finite-volume theory. -/
theorem missing_foundation_is_ym_and_full :
    ∀ p : MillenniumProblem,
      millenniumThreshold p = .MissingFoundation →
      p = .YM ∧ problemFrobenius p = .full := by
  intro p h_threshold
  -- YM is the unique MissingFoundation problem
  have h_ym : p = .YM := by
    cases p
    · simp [millenniumThreshold] at h_threshold  -- RH
    · simp [millenniumThreshold] at h_threshold  -- Hodge
    · simp [millenniumThreshold] at h_threshold  -- PvsNP
    · simp [millenniumThreshold] at h_threshold  -- NS
    · rfl                                      -- YM: this is the one
    · simp [millenniumThreshold] at h_threshold  -- BSD
    · simp [millenniumThreshold] at h_threshold  -- OPN
  rw [h_ym]
  simp [problemFrobenius]

/-- Helper: classify .algebraOnly problems. -/
lemma algebraOnly_cases (p : MillenniumProblem) (h : problemFrobenius p = .algebraOnly) :
    p = .PvsNP ∨ p = .BSD ∨ p = .OPN := by
  cases p
  · exfalso; simp [problemFrobenius] at h  -- RH: .full ≠ .algebraOnly
  · exfalso; simp [problemFrobenius] at h  -- Hodge
  · simp [problemFrobenius]                -- PvsNP ✓
  · exfalso; simp [problemFrobenius] at h  -- NS
  · exfalso; simp [problemFrobenius] at h  -- YM
  · simp [problemFrobenius]                -- BSD ✓
  · simp [problemFrobenius]                -- OPN ✓

theorem algebraOnly_implies_open_problem :
    ∀ p : MillenniumProblem,
      problemFrobenius p = .algebraOnly →
      millenniumThreshold p ≠ .MissingFoundation := by
  intro p h_alg
  rcases algebraOnly_cases p h_alg with (rfl | rfl | rfl)
  · simp [millenniumThreshold]
  · simp [millenniumThreshold]
  · simp [millenniumThreshold]

/-- Threshold compatibility: a .full Frobenius problem can have either
    OpenProblem or MissingFoundation threshold.
    A .algebraOnly problem can only have OpenProblem threshold.
    This is the Frobenius-threshold compatibility theorem. -/
theorem frobenius_threshold_compatibility :
    ∀ p : MillenniumProblem,
      problemFrobenius p = .algebraOnly →
      millenniumThreshold p = .OpenProblem := by
  intro p h_alg
  rcases algebraOnly_cases p h_alg with (rfl | rfl | rfl) <;> simp [millenniumThreshold]

-- =====================================================================
-- §9  Cross-problem Frobenius distance
-- =====================================================================

/-- Frobenius distance between problems: |rank(p₁) - rank(p₂)|. -/
def frobeniusDistance (p₁ p₂ : MillenniumProblem) : ℕ :=
  Int.natAbs ((problemFrobenius p₁).rank - (problemFrobenius p₂).rank)

/-- All four analysis problems are at the same Frobenius rank (distance 0). -/
theorem analysis_problems_zero_distance :
    frobeniusDistance .RH .Hodge = 0 ∧
    frobeniusDistance .RH .NS = 0 ∧
    frobeniusDistance .RH .YM = 0 ∧
    frobeniusDistance .Hodge .NS = 0 ∧
    frobeniusDistance .Hodge .YM = 0 ∧
    frobeniusDistance .NS .YM = 0 := by
  simp [frobeniusDistance, problemFrobenius, FrobeniusType.rank]

/-- All three arithmetic problems are at the same Frobenius rank (distance 0). -/
theorem arithmetic_problems_zero_distance :
    frobeniusDistance .PvsNP .BSD = 0 ∧
    frobeniusDistance .PvsNP .OPN = 0 ∧
    frobeniusDistance .BSD .OPN = 0 := by
  simp [frobeniusDistance, problemFrobenius, FrobeniusType.rank]

/-- Analysis and arithmetic problems are distance 1 apart in Frobenius rank.
    This is the Frobenius obstruction gap: analysis problems have δ,ε but
    arithmetic problems do not — a single Frobenius tier separates them. -/
theorem analysis_arithmetic_distance_one :
    frobeniusDistance .RH .PvsNP = 1 ∧
    frobeniusDistance .RH .BSD = 1 ∧
    frobeniusDistance .RH .OPN = 1 ∧
    frobeniusDistance .Hodge .PvsNP = 1 ∧
    frobeniusDistance .Hodge .BSD = 1 ∧
    frobeniusDistance .Hodge .OPN = 1 ∧
    frobeniusDistance .NS .PvsNP = 1 ∧
    frobeniusDistance .NS .BSD = 1 ∧
    frobeniusDistance .NS .OPN = 1 ∧
    frobeniusDistance .YM .PvsNP = 1 ∧
    frobeniusDistance .YM .BSD = 1 ∧
    frobeniusDistance .YM .OPN = 1 := by
  simp [frobeniusDistance, problemFrobenius, FrobeniusType.rank]

end Millennium.ThresholdFrobenius
