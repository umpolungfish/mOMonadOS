-- Imscribing/Millennium/IGThresholdAnalysis.lean
-- Cross-pollination: IGMorphism protocols × Millennium thresholds.

import Imscribing.IGMorphism
import Imscribing.Millennium.Thresholds
import Imscribing.Millennium.ThresholdFrobenius

namespace Millennium.IGThresholdAnalysis

open Imscribing
open Imscribing.Primitives
open Millennium.Thresholds
open Millennium.ThresholdFrobenius
open Millennium.Frobenius

theorem ym_not_eq_open_problem :
    millenniumThreshold .YM ≠ millenniumThreshold .RH := by
  simp [millenniumThreshold]

theorem odotOperator_is_O_inf :
    imscriptionTier odotOperator = .O_inf := by
  rfl

def thresholdLiftDepth : MillenniumProblem → ℕ
  | .RH    => 1
  | .Hodge => 1
  | .PvsNP => 1
  | .NS    => 1
  | .YM    => 2
  | .BSD   => 2
  | .OPN   => 2

theorem thresholdLiftDepth_eq_sorryDepth :
    ∀ p : MillenniumProblem, thresholdLiftDepth p = sorryDepth p := by
  intro p; cases p <;> simp [thresholdLiftDepth, sorryDepth]

def analysisThreshold : MillenniumProblem → Prop
  | .RH | .Hodge | .NS | .YM => true
  | _ => false

def arithmeticThreshold : MillenniumProblem → Prop
  | .PvsNP | .BSD | .OPN => true
  | _ => false

theorem analysis_is_full_frobenius :
    ∀ p : MillenniumProblem,
      analysisThreshold p → problemFrobenius p = .full := by
  intro p hp; cases p <;> simp [analysisThreshold, problemFrobenius] <;> trivial

theorem arithmetic_is_algebraOnly :
    ∀ p : MillenniumProblem,
      arithmeticThreshold p → problemFrobenius p = .algebraOnly := by
  intro p hp; cases p <;> simp [arithmeticThreshold, problemFrobenius] <;> trivial

theorem odot_vs_leeYang :
    leeYangFrobeniusType = .special := by
  simp [leeYangFrobeniusType]

end Millennium.IGThresholdAnalysis
