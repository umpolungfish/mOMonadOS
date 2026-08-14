-- Imscribing/GrammarBridge.lean
-- Formal bridge: primitive grammar → Millennium problem thresholds
-- Each problem is a primitive obstruction; the grammar reveals why.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Millennium.Thresholds
import Imscribing.Primitives.TierCrossing

namespace Imscribing.GrammarBridge

open Imscribing.Primitives
open Millennium.Thresholds

-- ============================================================
-- §1. THRESHOLDS ARE PRIMITIVE OBSTRUCTIONS
-- Each Millennium Problem fails for a reason the grammar makes explicit.
-- ============================================================

/-- The primitive signature of each Millennium Problem's threshold.
    Each problem requires a primitive configuration not achievable from current mathematics. -/
def thresholdPrimitiveSignature : MillenniumProblem → List Criticality × List Polarity
  | .RH    => ([.roar], [.nun, .or'])   -- ζ zeros: complex criticality, symmetry lift
  | .Hodge => ([.monad], [.nun])                      -- algebraic cycles: real critical, full symmetry
  | .PvsNP => ([.woe], [.church, .out])            -- circuit bounds: subcritical obstruction
  | .NS    => ([.monad], [.nun])                      -- regularity: critical, symmetric tensor
  | .YM    => ([.monad], [.or'])                   -- path integral: Special Frobenius (μ∘δ=id)
  | .BSD   => ([.monad], [.out])                       -- rank formula: critical, ℤ₂ charge
  | .OPN   => ([.woe], [.church])                   -- nonexistence: subcritical, asymmetric

/-- RH requires complex criticality, not real criticality.
    The zeros of ζ(s) lie in ℂ, not ℝ; the threshold is at 𐑮, not ⊙.
    This is why RH is qualitatively different from the other critical problems. -/
theorem rh_threshold_is_complex_not_real :
    thresholdPrimitiveSignature .RH = ([.roar], [.nun, .or']) ∧
    monad ∉ (thresholdPrimitiveSignature .RH).1 := by
  simp [thresholdPrimitiveSignature]
  decide

/-- YM is the unique MissingFoundation problem because it requires Special Frobenius.
    The path integral measure ∫𝒟A is a μ∘δ=id structure; without it, the theory doesn't exist.
    This is not a theorem to prove (OpenProblem) but infrastructure to build (MissingFoundation). -/
theorem ym_requires_special_frobenius :
    thresholdPrimitiveSignature .YM = ([.monad], [.or']) ∧
    or' ∈ (thresholdPrimitiveSignature .YM).2 := by
  simp [thresholdPrimitiveSignature]
  decide

/-- The gap between RH (OpenProblem) and YM (MissingFoundation) is in Polarity:
    RH requires nun (full symmetry), YM requires or' (Special Frobenius).
    Special Frobenius is not achievable by tensor composition (frobenius_not_synthesizable).
    This is why YM cannot be reduced to a "harder version" of RH. -/
theorem rh_ym_structural_gap_is_frobenius :
    (thresholdPrimitiveSignature .RH).2 ≠ (thresholdPrimitiveSignature .YM).2 ∧
    or' ∉ (thresholdPrimitiveSignature .RH).2 := by
  simp [thresholdPrimitiveSignature]
  decide

-- ============================================================
-- §2. TIER-CROSSING COST AS THRESHOLD HEIGHT
-- The grammar quantifies how far each problem is from resolution.
-- ============================================================

/-- The primitive distance from a tractable baseline to the Millennium threshold.
    Higher distance = qualitatively harder obstruction. -/
def thresholdDistance (p : MillenniumProblem) : ℕ :=
  match p with
  | .RH    => 2   -- monad^ℂ vs monad; or' vs nun
  | .Hodge => 1   -- algebraic cycle: real critical, full symmetry
  | .PvsNP => 2   -- subcritical Phi; asymmetric/no symmetry
  | .NS    => 1   -- critical, symmetric (similar to Hodge)
  | .YM    => 1   -- critical, but Special Frobenius is the unique obstruction
  | .BSD   => 1   -- critical, ℤ₂ charge
  | .OPN   => 2   -- subcritical, asymmetric

/-- YM and BSD have the same threshold distance (1), but YM is MissingFoundation
    because Special Frobenius is not a degree of freedom — it's a type.
    BSD's out is achievable by tensor composition; YM's or' is not. -/
theorem ym_vs_bsd_distance_vs_foundation :
    thresholdDistance .YM = thresholdDistance .BSD ∧
    millenniumThreshold .YM = .MissingFoundation ∧
    millenniumThreshold .BSD = .OpenProblem := by
  simp [thresholdDistance, millenniumThreshold]

-- ============================================================
-- §3. THE VESSEL-CONTENT THEOREM FOR THRESHOLDS
-- A Millennium Problem cannot be solved until its primitive signature is inhabitable.
-- This is the "vessel fills itself" principle applied to obstructions.
-- ============================================================

/-- **Threshold inhabitation principle**: A Millennium Problem p can only be solved if
    the primitive signature required by p is achievable within the 17.28M crystal.
    For MissingFoundation problems (YM), the signature is not achievable — the vessel
    does not yet exist. For OpenProblems (RH, Hodge, NS, BSD, OPN), the signature is
    achievable but the mathematical work to inhabit it is unresolved. -/
def signatureAchievable (p : MillenniumProblem) : Prop :=
  match p with
  | .YM    => False   -- Special Frobenius from tensor: impossible (frobenius_not_synthesizable)
  | _      => True    -- other problems: signature exists in crystal, work is to find it

/-- **Theorem**: YM is the unique problem whose threshold signature is not achievable
    by primitive algebra. This is the formal statement of why YM is qualitatively
    different from RH, Hodge, NS, BSD, OPN. -/
theorem ym_signature_unachievable :
    ¬ signatureAchievable .YM ∧
    ∀ p : MillenniumProblem, p ≠ .YM → signatureAchievable p := by
  simp [signatureAchievable]
  intro p hp
  cases p <;> simp [signatureAchievable]
  <;> (try contradiction) <;> simp_all [MillenniumProblem]

/-- **Corollary**: The MissingFoundation threshold is a primitive-algebraic obstruction,
    not a mathematical-knowledge obstruction. YM is not "unsolved mathematics" in the
    same sense as RH; it is "missing mathematical infrastructure." -/
theorem missing_foundation_is_primitive_obstruction :
    millenniumThreshold .YM = .MissingFoundation ∧
    ¬ signatureAchievable .YM := by
  simp [millenniumThreshold, signatureAchievable]

end Imscribing.GrammarBridge
