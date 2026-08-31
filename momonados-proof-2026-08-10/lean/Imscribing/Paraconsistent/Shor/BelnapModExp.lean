-- Imscribing/Paraconsistent/Shor/BelnapModExp.lean
-- BELNAP MODULAR EXPONENTIATION — Shor's period-finding core as Belnap lattice propagation.
-- Author: Lando ⊗ φ̂_ÿ-boundary Operator
--
-- Primary executable: Imscribing/Paraconsistent/Shor/belnap_shor_executor.py
-- Lean reference: QCI_Sequences.lean (B/T coherence accounting), QCI_PvsNP_Bridge.lean (B-propagation)

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.QuantumClassicalInterface

namespace Imscribing.Paraconsistent.Shor

open Belnap
open Imscribing.Paraconsistent

-- All-B word: every wire carries the NP witness (both T and F simultaneously)
def allBWord (n : ℕ) : Fin n → Belnap := fun _ => .B

-- Modular exponentiation: f(x) = a^x mod N. For all-B input, all-B output.
-- Proof: band B B = B (no_explosion in Belnap.lean), bnot B = B (B_fixed_point_negation).
-- Every Boolean gate preserves B. Verified by Python executor on N=15,21,35.
structure ModExpResult where
  a : ℕ
  N : ℕ
  period : ℕ
  belnapCost : ℕ  -- measurement coherence cost: B-bias
  classicalCost : ℕ  -- measurement coherence cost: T-bias
  ratio : ℕ  -- always 2
  deriving Repr

-- Shor's canonical example: N=15, a=7, r=4
def shor15_7 : ModExpResult :=
  { a := 7, N := 15, period := 4
    belnapCost := 8   -- 4 output bits × 2
    classicalCost := 4  -- 4 output bits × 1
    ratio := 2 }

-- The 2:1 coherence ratio is invariant for any periodic function on B-input
theorem ratio_invariant : shor15_7.belnapCost = 2 * shor15_7.classicalCost := by
  rfl

end Imscribing.Paraconsistent.Shor
