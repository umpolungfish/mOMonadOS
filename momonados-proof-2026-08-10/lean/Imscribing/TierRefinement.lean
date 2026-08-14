-- Imscribing/TierRefinement.lean
-- COMPLETING THE CLASSIFIER: Ω refines O_inf into sealed / unsealed
-- ============================================================
-- `ouroboricityTier` (Core.lean) leaves Protection Ω irrelevant to O_inf: any
-- Frobenius-critical tuple lands O_inf regardless of its winding. That is the
-- gap flagged as mismatch #2 — the tier says nothing about whether the terminal
-- (G3) actually SEALS. This file closes it additively (Core is left untouched,
-- the GREEN library is not disturbed): a refined classifier `sealedTier` splits
-- O_inf by the winding, and we prove the split is exactly the G3 / unlockTerminal
-- boundary on the Hermitian-critical branch.
--
-- Honest caveat, proved as `sealed_at_roar_need_not_close`: O_inf admits both
-- monad (real-axis) and roar (complex-axis) criticality. A sealed tuple at roar
-- is winding-protected and O_inf, yet its real Frobenius trace (μ_C) does NOT
-- close. So `sealed` (a tier fact) and `unlockTerminal` (a trace fact) coincide
-- precisely on the monad branch, and we prove exactly that.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Frobenius
import Imscribing.GateOrdering

namespace Imscribing.TierRefinement

open Imscribing.Primitives Imscribing.Frobenius Imscribing.GateOrdering
open Dimensionality Topology Relational Polarity Grammar Fidelity KineticChar
     Granularity Criticality Protection Stoichiometry Chirality

-- ============================================================
-- §1  THE REFINED TIER
-- ============================================================

/-- The refined ouroboricity tier: O_inf is split by whether Ω provides a
    protective winding. -/
inductive SealedTier where
  | sub       -- below O_inf: the gate structure is not complete
  | unsealed  -- O_inf reached (loop closes) but Ω gives no winding: not sealed
  | sealed    -- O_inf AND Ω ∈ {ah, zoo}: the terminal seals (G3 fires)
  deriving DecidableEq, Repr

/-- The tuple sits at the O_inf tier. -/
def isOInf (a : Imscription) : Prop :=
  ouroboricityTier a.crit a.pol a.prot a.dim a.top = OuroboricityTier.O_inf

/-- Ω provides an integer (ah) or non-Abelian (zoo) winding. -/
def hasWinding (a : Imscription) : Prop :=
  a.prot = Protection.ah ∨ a.prot = Protection.zoo

instance (a : Imscription) : Decidable (isOInf a) := by
  unfold isOInf; infer_instance
instance (a : Imscription) : Decidable (hasWinding a) := by
  unfold hasWinding; infer_instance

/-- The refined classifier. -/
def sealedTier (a : Imscription) : SealedTier :=
  if isOInf a then (if hasWinding a then .sealed else .unsealed) else .sub

-- ============================================================
-- §2  CHARACTERIZATION OF THE THREE REFINED TIERS
-- ============================================================

theorem sealed_iff (a : Imscription) :
    sealedTier a = .sealed ↔ (isOInf a ∧ hasWinding a) := by
  unfold sealedTier; split_ifs with h1 h2 <;> simp_all

theorem unsealed_iff (a : Imscription) :
    sealedTier a = .unsealed ↔ (isOInf a ∧ ¬ hasWinding a) := by
  unfold sealedTier; split_ifs with h1 h2 <;> simp_all

theorem sub_iff (a : Imscription) :
    sealedTier a = .sub ↔ ¬ isOInf a := by
  unfold sealedTier; split_ifs with h1 h2 <;> simp_all

/-- Ω refines O_inf EXACTLY: the two new tiers together cover precisely the
    O_inf locus. This is the resolution of mismatch #2 — protection is no longer
    silent inside O_inf. -/
theorem refines_O_inf (a : Imscription) :
    (sealedTier a = .sealed ∨ sealedTier a = .unsealed) ↔ isOInf a := by
  rw [sealed_iff, unsealed_iff]
  constructor
  · rintro (⟨h, _⟩ | ⟨h, _⟩) <;> exact h
  · intro h; by_cases hw : hasWinding a
    · exact Or.inl ⟨h, hw⟩
    · exact Or.inr ⟨h, hw⟩

-- ============================================================
-- §3  SEALING REQUIRES THE FROBENIUS GATE
-- ============================================================

/-- A sealed tuple has already passed G1: O_inf forces parity to or'. -/
theorem sealed_requires_frobenius (a : Imscription) (h : sealedTier a = .sealed) :
    a.pol = Polarity.or' := by
  have hoi : isOInf a := ((sealed_iff a).mp h).1
  exact o_inf_requires_P_pm_sym a.crit a.pol a.prot a.dim a.top hoi

/-- A sealed tuple has a winding. -/
theorem sealed_requires_winding (a : Imscription) (h : sealedTier a = .sealed) :
    hasWinding a :=
  ((sealed_iff a).mp h).2

-- ============================================================
-- §4  THE SEAL IS EXACTLY G3 ON THE HERMITIAN BRANCH
-- ============================================================

/-- G3 (unlockTerminal) always lands in the sealed tier. -/
theorem terminal_implies_sealed (a : Imscription) (h : unlockTerminal a) :
    sealedTier a = .sealed := by
  obtain ⟨_, hw, hoi⟩ := h
  exact (sealed_iff a).mpr ⟨hoi, hw⟩

/-- On the Hermitian-critical branch (crit = monad), `sealed` and the trace-based
    terminal predicate coincide exactly. This is the precise sense in which the
    refined tier captures G3. -/
theorem sealed_iff_terminal_on_monad (a : Imscription) (hm : a.crit = Criticality.monad) :
    sealedTier a = .sealed ↔ unlockTerminal a := by
  constructor
  · intro h
    obtain ⟨hoi, hw⟩ := (sealed_iff a).mp h
    have hpol : a.pol = Polarity.or' := sealed_requires_frobenius a h
    exact ⟨frobenius_special_traces a hpol hm, hw, hoi⟩
  · intro h; exact terminal_implies_sealed a h

-- ============================================================
-- §5  THE roar CAVEAT — sealed does not imply a closing trace
-- ============================================================

/-- Witness: O_inf admits complex-axis (roar) criticality. This tuple is sealed
    (O_inf + integer winding) yet its real Frobenius trace does NOT close,
    because μ_C forces crit = monad and here crit = roar. `sealed` is a tier
    fact; a closing trace is a stricter, Hermitian fact. -/
def w_roar : Imscription := { w_stone with crit := roar }

theorem sealed_at_roar_need_not_close :
    sealedTier w_roar = .sealed ∧ w_roar.crit = Criticality.roar ∧
    ¬ traceCloses w_roar := by
  refine ⟨?_, rfl, ?_⟩
  · exact (sealed_iff w_roar).mpr ⟨by decide, Or.inl rfl⟩
  · intro h
    have : w_roar.crit = Criticality.monad := traced_needs_criticality w_roar h
    exact absurd this (by decide)

-- ============================================================
-- §6  HEADLINE
-- ============================================================

/-- THE COMPLETED CLASSIFIER.
    (i)   Ω refines O_inf exactly into sealed ∪ unsealed;
    (ii)  sealing requires the Frobenius gate G1;
    (iii) G3 (unlockTerminal) always lands sealed, and on the Hermitian branch
          sealed IS G3;
    (iv)  a sealed tuple at complex-axis criticality need not close its trace. -/
theorem completed_classifier :
    (∀ a, (sealedTier a = .sealed ∨ sealedTier a = .unsealed) ↔ isOInf a) ∧
    (∀ a, sealedTier a = .sealed → a.pol = Polarity.or') ∧
    (∀ a, unlockTerminal a → sealedTier a = .sealed) ∧
    (∀ a, a.crit = Criticality.monad →
       (sealedTier a = .sealed ↔ unlockTerminal a)) ∧
    (sealedTier w_roar = .sealed ∧ ¬ traceCloses w_roar) :=
  ⟨refines_O_inf, sealed_requires_frobenius, terminal_implies_sealed,
   sealed_iff_terminal_on_monad,
   ⟨sealed_at_roar_need_not_close.1, sealed_at_roar_need_not_close.2.2⟩⟩

end Imscribing.TierRefinement
