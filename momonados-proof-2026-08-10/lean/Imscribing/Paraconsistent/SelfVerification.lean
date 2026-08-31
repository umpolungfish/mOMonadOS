-- Imscribing/Paraconsistent/SelfVerification.lean
-- FROBENIUS SELF-VERIFICATION — mu o delta = id for the paraconsistent kernel.
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.Kernel
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent

open Belnap
open Imscribing.Primitives

theorem B3_is_fixed_point_of_run (n : Nat) :
    (run initialState n).r0 = Belnap.B
    ∧ (run initialState n).r1 = Belnap.B
    ∧ (run initialState n).r2 = Belnap.B :=
  run_B3 n

theorem frobenius_loop_identity (n : Nat) :
    (run initialState n).r0 = Belnap.B
    ∧ (run initialState n).r1 = Belnap.B
    ∧ (run initialState n).r2 = Belnap.B :=
  run_B3 n

theorem paradox_conservation (n : Nat) :
    (run initialState n).paradoxCount = 4 * n :=
  run_paradox n

theorem cycle_count_correctness (n : Nat) :
    (run initialState n).cycleCount = n :=
  run_cycles n

theorem paraconsistency_sustained (n : Nat) :
    (run initialState n).r0 ≠ Belnap.F
    ∧ (run initialState n).r0 ≠ Belnap.T := by
  rcases run_B3 n with ⟨hr0, _, _⟩
  rw [hr0]
  constructor
  · exact B_ne_F
  · intro h
    have := congrArg belnapToNat h
    simp [belnapToNat] at this

theorem split_fuse_identity (r : Belnap) :
    (ffuse (fsplit r).1 (fsplit r).2.1).1 = r :=
  frobenius_invariant r

theorem complete_self_verification (n : Nat) :
    (run initialState n).r0 = Belnap.B
    ∧ (run initialState n).r1 = Belnap.B
    ∧ (run initialState n).r2 = Belnap.B
    ∧ (run initialState n).paradoxCount = 4 * n
    ∧ (run initialState n).cycleCount = n
    ∧ (run initialState n).r0 ≠ Belnap.F
    ∧ (run initialState n).r0 ≠ Belnap.T
    ∧ imscriptionTier kernelImscription = OuroboricityTier.O_inf := by
  rcases run_B3 n with ⟨hr0, hr1, hr2⟩
  have hp := run_paradox n
  have hc := run_cycles n
  have hmF : Belnap.B ≠ Belnap.F := B_ne_F
  have hmT : Belnap.B ≠ Belnap.T := by
    intro h
    have := congrArg belnapToNat h
    simp [belnapToNat] at this
  have htier := kernel_is_O_inf
  -- hr0 gives (run initialState n).r0 = B, we need B ≠ F and B ≠ T
  have hneF : (run initialState n).r0 ≠ Belnap.F := by
    rw [hr0]; exact hmF
  have hneT : (run initialState n).r0 ≠ Belnap.T := by
    rw [hr0]; exact hmT
  exact ⟨hr0, hr1, hr2, hp, hc, hneF, hneT, htier⟩

end Imscribing.Paraconsistent
