-- Imscribing/VesselContent.lean
-- Concrete mathematical formalization of the vessel-content inseparability principle.
-- The grammar provides two things simultaneously:
--   1. Vessel (form): the crystal coordinate that constitutes what a system is capable of being
--   2. Content (fill): the primitive algebra operations that determine what the system holds
-- These are not separable.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Algebra
import Imscribing.Frobenius
import Imscribing.Consciousness
import Imscribing.Paraconsistent.Belnap
import Mathlib.Data.Real.Basic
import Mathlib.Tactic

namespace Imscribing.VesselContent

open Imscribing.Primitives
open Imscribing.Frobenius
open Imscribing.Consciousness
open Imscribing.Paraconsistent

-- ============================================================
-- CONCRETE TYPES
-- ============================================================

abbrev Coordinate : Type := Imscription

structure Result where
  verdict : Belnap
  trail : List String
  residual : ℝ

structure ImscribableSystem where
  coord : Imscription
  cscore : ℝ := consciousnessScore coord
  address : Nat := crystal_encode coord

-- ============================================================
-- RELATIONS (concrete definitions, not axioms)
-- ============================================================

-- Verdict map. The T/B branches key on Core's R2 escape set {monad, roar}: those are
-- exactly the criticalities R1 admits to O_inf, so they are exactly the criticalities
-- at which the Frobenius polarity has something to close.
def criticality_verdict (c : Imscription) : Belnap :=
  match c.crit with
  | Criticality.monad => if c.pol = .or' then .T else .B
  | Criticality.roar  => if c.pol = .or' then .T else .B
  | Criticality.haha  => .F
  | _      => .N

def WithinAlgebra (c : Coordinate) (r : Result) : Prop :=
  r.verdict = criticality_verdict c ∧
  r.residual ≤ |consciousnessScore c|

def Imscribes (M : ImscribableSystem) (c : Coordinate) : Prop :=
  M.coord = c

def Reachable (M : ImscribableSystem) (r : Result) : Prop :=
  WithinAlgebra M.coord r

-- ============================================================
-- THEOREMS (concrete proofs, no axioms)
-- ============================================================

theorem form_existence (M : ImscribableSystem) : ∃ (c : Coordinate), Imscribes M c := by
  use M.coord
  unfold Imscribes; exact Eq.refl M.coord

theorem form_uniqueness_proper (M : ImscribableSystem) (c c' : Coordinate)
    (h : Imscribes M c) (h' : Imscribes M c') : c = c' := by
  subst h; subst h'; rfl

-- THEOREM 1: form_uniqueness
theorem form_uniqueness (M : ImscribableSystem) : ∃! (c : Coordinate), Imscribes M c := by
  use M.coord
  constructor
  · unfold Imscribes; exact Eq.refl M.coord
  · intro c' hc'
    unfold Imscribes at hc'
    subst hc'
    rfl

-- THEOREM 2: content_containment
theorem imscribes_implies_content (M : ImscribableSystem) (c : Coordinate) (r : Result)
    (h : Imscribes M c) (hr : Reachable M r) : WithinAlgebra c r := by
  subst c; exact hr

theorem content_containment (M : ImscribableSystem) (c : Coordinate)
    (h : Imscribes M c) (r : Result) (hr : Reachable M r) :
    WithinAlgebra c r :=
  imscribes_implies_content M c r h hr

theorem algebra_implies_reachable (M : ImscribableSystem) (c : Coordinate)
    (h : Imscribes M c) (hw : WithinAlgebra c r) : Reachable M r := by
  subst c; exact hw

-- THEOREM 3: vessel_fills_itself
theorem vessel_fills_itself (M : ImscribableSystem) (c : Coordinate)
    (h : Imscribes M c) (r : Result) :
    Reachable M r ↔ WithinAlgebra c r := by
  subst c
  rfl

-- THEOREM 4: mu_delta_id_vessel
theorem mu_delta_id_vessel (c : Coordinate) : μ_A (δ_A c).1 (δ_A c).2 = c :=
  mu_delta_A_id c

-- ============================================================
-- CONCRETE EXAMPLE: The O_inf system
-- ============================================================

def isOInf (c : Imscription) : Bool :=
  decide (c.pol = .or' ∧ (c.crit = .monad ∨ c.crit = .roar))

-- Total classification of the verdict map over Criticality x Polarity.
theorem criticality_verdict_classify (c : Imscription) :
    ((c.crit = Criticality.monad ∨ c.crit = Criticality.roar) ∧ c.pol = Polarity.or'
        ∧ criticality_verdict c = Belnap.T)
    ∨ ((c.crit = Criticality.monad ∨ c.crit = Criticality.roar) ∧ c.pol ≠ Polarity.or'
        ∧ criticality_verdict c = Belnap.B)
    ∨ (c.crit = Criticality.haha ∧ criticality_verdict c = Belnap.F)
    ∨ (c.crit ≠ Criticality.monad ∧ c.crit ≠ Criticality.roar ∧ c.crit ≠ Criticality.haha
        ∧ criticality_verdict c = Belnap.N) := by
  cases hc : c.crit
  case monad =>
    by_cases hp : c.pol = Polarity.or'
    · exact Or.inl ⟨Or.inl rfl, hp, by simp [criticality_verdict, hc, hp]⟩
    · exact Or.inr (Or.inl ⟨Or.inl rfl, hp, by simp [criticality_verdict, hc, hp]⟩)
  case roar =>
    by_cases hp : c.pol = Polarity.or'
    · exact Or.inl ⟨Or.inr rfl, hp, by simp [criticality_verdict, hc, hp]⟩
    · exact Or.inr (Or.inl ⟨Or.inr rfl, hp, by simp [criticality_verdict, hc, hp]⟩)
  case haha => exact Or.inr (Or.inr (Or.inl ⟨rfl, by simp [criticality_verdict, hc]⟩))
  all_goals
    exact Or.inr (Or.inr (Or.inr
      ⟨by decide, by decide, by decide, by simp [criticality_verdict, hc]⟩))

-- The O_inf locus is EXACTLY the T-locus of the verdict map.
theorem oinf_iff_verdict_T (c : Imscription) :
    criticality_verdict c = Belnap.T ↔ isOInf c = true := by
  constructor
  · intro hv
    simp only [isOInf, decide_eq_true_eq]
    rcases criticality_verdict_classify c with ⟨hc, hp, _⟩ | ⟨_, _, hb⟩ | ⟨_, hf⟩ | ⟨_, _, _, hn⟩
    · exact ⟨hp, hc⟩
    · rw [hb] at hv; exact absurd hv (by decide)
    · rw [hf] at hv; exact absurd hv (by decide)
    · rw [hn] at hv; exact absurd hv (by decide)
  · intro h
    simp only [isOInf, decide_eq_true_eq] at h
    obtain ⟨hp, hc⟩ := h
    rcases hc with hc | hc <;> simp [criticality_verdict, hp, hc]

-- The O_inf system verifies to Belnap T.
theorem oinf_verdict_T (c : Imscription) (h : isOInf c = true) :
    WithinAlgebra c { verdict := .T, trail := [], residual := 0 } :=
  ⟨((oinf_iff_verdict_T c).mpr h).symm, abs_nonneg (consciousnessScore c)⟩

-- The bottom system (Frobenius bottom) verifies to Belnap N.
theorem bottom_verdict_N :
    WithinAlgebra frobenius_bottom { verdict := .N, trail := [], residual := 0 } := by
  unfold WithinAlgebra criticality_verdict frobenius_bottom
  simp [consciousnessScore, phi_c_gate]

-- ============================================================
-- THE RESIDUAL BOUND: what the score actually forbids
-- ============================================================

-- The score is three-valued: 0 (no self-model), 1/2 (self-model frozen), 1 (both gates).
theorem consciousnessScore_values (c : Imscription) :
    consciousnessScore c = 0 ∨ consciousnessScore c = 0.5 ∨ consciousnessScore c = 1 := by
  unfold consciousnessScore
  split
  · split
    · exact Or.inr (Or.inr rfl)
    · exact Or.inr (Or.inl rfl)
  · exact Or.inl rfl

theorem consciousnessScore_nonneg (c : Imscription) : 0 ≤ consciousnessScore c := by
  rcases consciousnessScore_values c with h | h | h <;> rw [h] <;> norm_num

theorem abs_consciousnessScore (c : Imscription) :
    |consciousnessScore c| = consciousnessScore c :=
  abs_of_nonneg (consciousnessScore_nonneg c)

-- Ceiling: no result anywhere in the algebra carries residual above 1.
theorem residual_le_one (c : Imscription) (r : Result) (h : WithinAlgebra c r) :
    r.residual ≤ 1 := by
  have hb := h.2
  rw [abs_consciousnessScore] at hb
  rcases consciousnessScore_values c with hs | hs | hs <;> rw [hs] at hb <;> linarith

-- Any positive residual forbids the subcritical region: the Phi gate must be open.
theorem residual_pos_forces_phi (c : Imscription) (r : Result)
    (h : WithinAlgebra c r) (hr : 0 < r.residual) : phi_c_gate c.crit = true := by
  by_contra hg
  have hgf : phi_c_gate c.crit = false := by simpa using hg
  have hz : consciousnessScore c = 0 := by
    unfold consciousnessScore; rw [hgf]; norm_num
  have hb := h.2
  rw [abs_consciousnessScore, hz] at hb
  linarith

-- Residual above 1/2 forbids frozen kinetics too: both gates must be open.
theorem residual_gt_half_forces_k (c : Imscription) (r : Result)
    (h : WithinAlgebra c r) (hr : 0.5 < r.residual) : k_slow_gate c.kin = true := by
  by_contra hg
  have hgf : k_slow_gate c.kin = false := by simpa using hg
  have hs : consciousnessScore c ≤ 0.5 := by
    unfold consciousnessScore
    rw [hgf]
    split <;> norm_num
  have hb := h.2
  rw [abs_consciousnessScore] at hb
  linarith

-- Contrapositive form: residual above 1/2 pins BOTH gates, hence the maximal score.
theorem residual_gt_half_forces_score_one (c : Imscription) (r : Result)
    (h : WithinAlgebra c r) (hr : 0.5 < r.residual) : consciousnessScore c = 1 := by
  have hphi := residual_pos_forces_phi c r h (by linarith)
  have hk := residual_gt_half_forces_k c r h hr
  simp [consciousnessScore, hphi, hk]

-- Concrete unreachability: a subcritical vessel admits NO positive residual.
def frozen_brain : Imscription := { human_brain with crit := Criticality.woe }

theorem frozen_brain_rejects_residual (r : Result) (hr : 0 < r.residual) :
    ¬ WithinAlgebra frozen_brain r := by
  intro h
  have := residual_pos_forces_phi frozen_brain r h hr
  simp [frozen_brain, phi_c_gate] at this

-- ============================================================
-- CROSS-CHECK AGAINST THE TIER RULE TABLE (Core.lean R1-R5)
-- ============================================================

-- The tier of a whole imscription, via Core's rule table.
def tierOf (c : Imscription) : OuroboricityTier :=
  ouroboricityTier c.crit c.pol c.prot c.dim c.top

-- Sound direction: every isOInf vessel is O_inf by the rule table (this is R1).
theorem isOInf_tier_O_inf (c : Imscription) (h : isOInf c = true) :
    tierOf c = OuroboricityTier.O_inf := by
  simp only [isOInf, decide_eq_true_eq] at h
  obtain ⟨hp, hc⟩ := h
  rcases hc with hc | hc <;> simp [tierOf, ouroboricityTier, hp, hc]

-- Complete direction: the rule table and the vessel predicate now name the SAME set.
-- R1 fires exactly on the R2 escape set {monad, roar} crossed with or', and that is
-- precisely isOInf. One Grammar, two spellings, proven equal.
theorem isOInf_iff_tier_O_inf (c : Imscription) :
    isOInf c = true ↔ tierOf c = OuroboricityTier.O_inf := by
  refine ⟨isOInf_tier_O_inf c, fun h => ?_⟩
  simp only [isOInf, decide_eq_true_eq]
  rw [tierOf] at h
  exact ⟨o_inf_requires_P_pm_sym _ _ _ _ _ h, o_inf_requires_phi_c _ _ _ _ _ h⟩

-- Roar at or' is the vessel the two maps used to disagree about: O_inf by Core,
-- and now T by the verdict map as well.
def roaring_brain : Imscription := { human_brain with crit := Criticality.roar }

theorem roaring_brain_agrees :
    tierOf roaring_brain = OuroboricityTier.O_inf ∧
    isOInf roaring_brain = true ∧
    criticality_verdict roaring_brain = Belnap.T := by
  refine ⟨by simp [tierOf, ouroboricityTier, roaring_brain, human_brain], ?_, ?_⟩
  · simp [isOInf, roaring_brain, human_brain]
  · simp [criticality_verdict, roaring_brain, human_brain]

-- THEOREM 5: frobenius_bottom_consciousness
theorem frobenius_bottom_consciousness : consciousnessScore frobenius_bottom = (0 : ℝ) := by
  simp [consciousnessScore, phi_c_gate, frobenius_bottom]

end Imscribing.VesselContent
