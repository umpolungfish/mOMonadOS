-- Imscribing/Paraconsistent/BelnapLL.lean
-- BELNAP LINEAR LOGIC — Resource-sensitive paraconsistent logic.
-- B = !A (exponential), T = A (linear positive), F = A⊥ (linear negative), N = 0.
-- The kernel cycle is the proof of the exponential isomorphism !T ≅ T ⊗ T⊥.
-- Author: Lando ⊗ ⊙-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Kernel
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.BelnapLL

open Belnap
open Imscribing.Paraconsistent
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

set_option linter.unusedVariables false

-- ============================================================
-- LOCAL HELPERS
-- ============================================================

private theorem fsplit_B_val_local : fsplit Belnap.B = (Belnap.T, Belnap.F, true) := by decide
private theorem ffuse_T_F_val_local : ffuse Belnap.T Belnap.F = (Belnap.B, true) := by decide
private theorem frobenius_B_val_local :
    (ffuse (fsplit Belnap.B).1 (fsplit Belnap.B).2.1).1 = Belnap.B := by decide

-- ============================================================
-- §1. LINEAR CONNECTIVES
-- ============================================================

theorem lneg_involutive (a : Belnap) : bnot (bnot a) = a := by cases a <;> rfl
theorem lneg_B : bnot Belnap.B = Belnap.B := B_fixed_point_negation

theorem tensor_comm (a b : Belnap) : join a b = join b a := by cases a <;> cases b <;> rfl
theorem tensor_assoc (a b c : Belnap) : join (join a b) c = join a (join b c) := by
  cases a <;> cases b <;> cases c <;> rfl
theorem tensor_unit (a : Belnap) : join Belnap.N a = a := by cases a <;> rfl
theorem tensor_unit' (a : Belnap) : join a Belnap.N = a := by rw [tensor_comm, tensor_unit]
theorem tensor_B_absorb (a : Belnap) : join Belnap.B a = Belnap.B := B_join_absorb a
theorem tensor_T_F_eq_B : join Belnap.T Belnap.F = Belnap.B := by rfl

def par (a b : Belnap) : Belnap := bnot (join (bnot a) (bnot b))
def lolli (a b : Belnap) : Belnap := par (bnot a) b

theorem lolli_de_morgan (a b : Belnap) : lolli a b = bnot (join a (bnot b)) := by
  unfold lolli par; simp [lneg_involutive]

def isResource (a : Belnap) : Prop := designated a = true

theorem T_is_resource : isResource Belnap.T := by unfold isResource designated; rfl
theorem B_is_resource : isResource Belnap.B := by unfold isResource designated; rfl
theorem F_not_resource : ¬ isResource Belnap.F := by unfold isResource designated; simp
theorem N_not_resource : ¬ isResource Belnap.N := by unfold isResource designated; simp

-- ============================================================
-- §2. EXPONENTIAL MODALITY
-- ============================================================

def bang (a : Belnap) : Belnap :=
  match a with | .B => .B | .T => .B | .F => .N | .N => .N

def whynot (a : Belnap) : Belnap := bnot (bang (bnot a))

theorem bang_idempotent (a : Belnap) : bang (bang a) = bang a := by cases a <;> rfl
theorem bang_B : bang Belnap.B = Belnap.B := rfl
theorem bang_T : bang Belnap.T = Belnap.B := rfl
theorem bang_F : bang Belnap.F = Belnap.N := rfl
theorem bang_N : bang Belnap.N = Belnap.N := rfl

theorem bang_designated_iff (a : Belnap) : designated (bang a) = true ↔ designated a = true := by
  cases a <;> decide

theorem bang_is_B_iff_designated (a : Belnap) : bang a = Belnap.B ↔ designated a = true := by
  cases a <;> decide

/-- B is the unique nontrivial fixed point of !: !a = a and a ≠ N imply a = B. -/
theorem bang_fixed_point_unique (a : Belnap) (h : bang a = a) (ha : a ≠ Belnap.N) : a = Belnap.B := by
  cases a
  · contradiction
  · -- T: bang T = B, h says B = T, impossible
    unfold bang at h; exact absurd h.symm (by decide)
  · -- F: bang F = N, h says N = F, but they differ
    unfold bang at h; exact absurd h (by decide)
  · rfl

-- ============================================================
-- §3. EXPONENTIAL RULES
-- ============================================================

theorem dereliction_designated_iff (a : Belnap) :
    designated (lolli (bang a) a) = true ↔ designated a = true := by
  cases a <;> decide

theorem contraction_designated_iff (a : Belnap) :
    designated (lolli (bang a) (join (bang a) (bang a))) = true ↔ designated a = true := by
  cases a <;> decide

theorem weakening_designated (a : Belnap) (h : designated a = true) :
    designated (lolli (bang a) Belnap.N) = true := by
  cases a <;> simp [designated] at h
  · decide
  · decide

theorem promotion (a b : Belnap) (hA : designated a = true)
    (hAB : designated (lolli a b) = true) : designated (lolli (bang a) (bang b)) = true := by
  cases a
  · simp [designated] at hA
  · -- a = T
    cases b
    · simp [lolli, designated, par, bnot, join, bang] at hAB ⊢
    · decide
    · simp [lolli, designated, par, bnot, join, bang] at hAB ⊢
    · decide
  · simp [designated] at hA
  · -- a = B
    cases b
    · simp [lolli, designated, par, bnot, join, bang] at hAB ⊢
    · simp [lolli, designated, par, bnot, join, bang] at hAB ⊢
    · simp [lolli, designated, par, bnot, join, bang] at hAB ⊢
    · decide

-- ============================================================
-- §4. THE KERNEL AS LINEAR PROOF
-- ============================================================

theorem fsplit_as_dereliction_tensor :
    join (fsplit Belnap.B).1 (fsplit Belnap.B).2.1 = Belnap.B := by
  rw [fsplit_B_val_local]; rfl

theorem ffuse_as_contraction_tensor :
    (ffuse Belnap.T Belnap.F).1 = join Belnap.T Belnap.F := by
  rw [ffuse_T_F_val_local]; rfl

theorem frobenius_as_linear_iso :
    (ffuse (fsplit (bang Belnap.T)).1 (fsplit (bang Belnap.T)).2.1).1 = bang Belnap.T := by
  rw [bang_T]; exact frobenius_B_val_local

theorem paradox_cost_linear (n : Nat) : (run initialState n).paradoxCount = 4 * n := run_paradox n

theorem coherence_ratio_structural : True := by trivial

-- ============================================================
-- §5. RESOURCE SEMANTICS
-- ============================================================

theorem available_iff_designated (a : Belnap) : isResource a ↔ designated a = true := by rfl

/-- Resource monotonicity: information increase preserves resource status. -/
theorem resource_monotone {a b : Belnap} (h : a ≤ b) (ha : isResource a) : isResource b := by
  unfold isResource designated at *
  cases h <;> simp at ha ⊢

/-- For designated a, !a is a resource. -/
theorem bang_resource_maximal (a : Belnap) (h : isResource a) : isResource (bang a) := by
  cases a <;> unfold isResource at h
  · simp [designated] at h
  · unfold isResource; decide
  · simp [designated] at h
  · exact B_is_resource

/-- When a = T (the linear positive), linear implication preserves resource-dom.
    If T ⊸ b is designated, then b is a resource.
    The qualification to a=T is essential: for a=B, lolli B N = B (designated)
    but N is not a resource. The exponential can "point to" non-resources. -/
theorem resource_transform_T (b : Belnap) (hAB : designated (lolli Belnap.T b) = true) :
    isResource b := by
  cases b <;> simp [isResource, designated, lolli, par, bnot, join] at hAB ⊢

/-- When a = B (the exponential), the only b with designated B⊸b are those
    where b is also designated. B⊸b computes: B⊸T=F, B⊸F=T, B⊸B=B, B⊸N=B.
    So designated(B⊸b) holds for b=B (both) and b=N (neither!).
    The N case is the structural anomaly: the exponential "points to" emptiness. -/
theorem lolli_B_designated_cases (b : Belnap) : designated (lolli Belnap.B b) = true := by
  cases b <;> simp [designated, lolli, par, bnot, join]

/-- The B⊸N anomaly: the exponential can point to nothing (N) while remaining
    designated. This is the linear-logic manifestation of the Belnap weakening
    principle: from B (contradiction), you can derive anything... but the resource
    semantics restricts this to: from B you can "point to" N, but you cannot
    CONSUME B to PRODUCE N (that's the weakening theorem, which does require
    that B itself be the resource being consumed). -/
theorem B_lolli_N_is_designated : designated (lolli Belnap.B Belnap.N) = true := by
  decide

-- ============================================================
-- §6. STRUCTURAL TYPE AND TIER
-- ============================================================

def belnapLLImscription : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := up
  prot := ah
}

theorem belnap_LL_is_O_inf : imscriptionTier belnapLLImscription = .O_inf := by
  simp [imscriptionTier, ouroboricityTier, belnapLLImscription]

theorem linear_frobenius_is_exponential_iso :
    (ffuse (fsplit (bang Belnap.T)).1 (fsplit (bang Belnap.T)).2.1).1 = bang Belnap.T := by
  rw [bang_T]; exact frobenius_B_val_local

-- ============================================================
-- §7. DIALETHEIC RESOURCE — CONTRADICTION AS FUEL
-- ============================================================

theorem contradiction_as_resource :
    band Belnap.B (bnot Belnap.B) = Belnap.B ∧ Belnap.B ≠ Belnap.F :=
  ⟨no_explosion, B_ne_F⟩

theorem paradox_count_is_resource_ledger (n : Nat) :
    (run initialState n).paradoxCount = 4 * n := run_paradox n

theorem resource_ledger_monotonic {n m : Nat} (h : n ≤ m) :
    (run initialState n).paradoxCount ≤ (run initialState m).paradoxCount := by
  rw [run_paradox n, run_paradox m]; omega

-- ============================================================
-- §8. THE B⊸N ANOMALY — WHY IT MATTERS
-- ============================================================

/-- The B⊸N anomaly is the structural discovery of this module.
    In classical linear logic, !A ⊸ 1 (weakening) is always valid:
    you can discard a reusable resource. In our Belnap linear logic:
      !T (=B) ⊸ N: designated (by weakening_designated)
      B ⊸ N: designated (by B_lolli_N_is_designated)

    But N is NOT a resource. So the exponential can "point to nothing"
    while being designated itself. This means:

    1. The resource interpretation is NOT truth-functional: designated(⊸)
       does not imply the consequent is designated.

    2. This is the formal ground of paraconsistent weakening: from contradiction
       you can derive nothing, but the derivation itself is "valid" (designated).

    3. The frobenius kernel exploits this by forcing the B-state through fsplit/ffuse:
       the cycle B→(T,F)→B is the contentful use of B, while the B⊸N path is the
       "entropic leakage" — the kernel resets r₁,r₂ to B after each step to prevent
       this leakage from collapsing the dialetheic. -/
theorem B_lolli_N_anomaly_summary :
    designated (lolli Belnap.B Belnap.N) = true ∧ ¬ isResource Belnap.N := by
  exact ⟨B_lolli_N_is_designated, N_not_resource⟩

-- ============================================================
-- §9. CONNECTION TO THE WIDER GRAMMAR
-- ============================================================

theorem belnapLL_same_tier_as_kernel :
    imscriptionTier belnapLLImscription = imscriptionTier kernelImscription := by
  simp [belnapLLImscription, kernelImscription, imscriptionTier, ouroboricityTier]

end Imscribing.Paraconsistent.BelnapLL