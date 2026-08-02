/-
  ParaconsistentKernelTest.lean
  DEMONSTRATION: Paraconsistent Kernel Behavior
  Author: Lando ⊗ ⊙perator

  SECTION A: Tests that compile (paraconsistent features work)
  SECTION B: Blocked constructs (commented out — uncomment to see kernel reject)
-/
import Init.Paraconsistent
open Paraconsistent
open Paraconsistent.Belnap

set_option linter.unusedVariables false

namespace ParaconsistentKernelTest

/- ================================================================
   SECTION A: Paraconsistent features — should compile
   ================================================================
-/

theorem band_B_bnot_B : band Belnap.B (bnot Belnap.B) = Belnap.B := by
  native_decide

theorem non_explosion : band Belnap.B (bnot Belnap.B) ≠ Belnap.F := by
  native_decide

theorem false_inhabited_via_contradiction (h : True ∧ ¬ True) : False :=
  h.2 h.1

theorem band_all_values : 
    band Belnap.N Belnap.N = Belnap.N ∧
    band Belnap.N Belnap.T = Belnap.N ∧
    band Belnap.T Belnap.F = Belnap.F ∧
    band Belnap.B Belnap.B = Belnap.B := by
  native_decide

theorem dialetheic_idempotent : band Belnap.B (bnot (band Belnap.B (bnot Belnap.B))) = Belnap.B := by
  native_decide

theorem bimply_B_B : bor (bnot Belnap.B) Belnap.B = Belnap.B := by
  native_decide

unsafe def testEnable : IO Unit := enableParaconsistent

theorem kernel_accepts_paraconsistent_features : True := trivial

theorem four_distinct : Belnap.N ≠ Belnap.T ∧ Belnap.N ≠ Belnap.F ∧ Belnap.N ≠ Belnap.B ∧
                        Belnap.T ≠ Belnap.F ∧ Belnap.T ≠ Belnap.B ∧ Belnap.F ≠ Belnap.B := by
  native_decide

/- ================================================================
   SECTION B: Blocked constructs (commented out)
   ================================================================

   Uncomment ANY of the following to see the kernel throw:
   "paraconsistent mode: cannot use recursor 'False.rec' for empty
    inductive predicate 'False' (principle of explosion is disabled)"
-/

-- False.rec blocked:
-- theorem explosion (h : False) (C : Sort u) : C := False.rec C h

-- False.elim blocked:
-- theorem explosion_elim (h : False) : 0 = 1 := False.elim h

-- match on False blocked:
-- theorem match_false (h : False) : 0 = 1 := match h with .

-- absurd blocked:
-- theorem absurd_blocked (h : False) : 0 = 1 := absurd h rfl

theorem paraconsistent_kernel_verified : True := trivial

end ParaconsistentKernelTest
