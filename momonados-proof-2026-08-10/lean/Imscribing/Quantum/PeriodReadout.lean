/-
# The period readout: the modular fixed point (the unmoved mover)

The one-shot topological readout (`fibqc readout <a> <N>`, mOMonadOS)
assembles the Fibonacci-Shor ModExp braid, measures its Jones invariant once
at the root t = 1/5, and reports the period r = ord_N(a). The period is the
modular fixed point: a^r ≡ 1 (mod N) — the step count at which the
exponentiation orbit x ↦ a·x closes on itself.

In the Alkahest reading this is the unmoved mover: the orbit map closes
exactly at r, and r is minimal — μ∘δ=id over the orbit. The readout is the
dissolution: the non-Abelian braid word (⊡ = non-Abelian) is reduced to the
integer period (⊡ = integer winding), the alkahest vessel's ⊡-promotion.

What is formalized here:

1. **The orbit closes.** `a ^ orderOf a = 1` for any element of any monoid
   (`pow_orderOf_eq_one`).
2. **The order is the period, minimally.** For the unit `unitOfCoprime a N h`
   in the units group `(ZMod N)ˣ` — a group, hence cancellative, unlike
   `ZMod N` itself with its zero divisors — the order closes the orbit and
   no smaller positive exponent does. This is the readout's contract:
   `period = Some(r)` with r exact.
3. **The orbit exists.** `Nat.Coprime a N` ⟹ some positive `r` closes it.
4. **All 11 kernel-verified sweep cases**, each `IsOrder a N r` closed by
   kernel computation (`decide`): the fixed point `a^r = 1` in `ZMod N` and
   the minimality over the interval `[1, r-1]`.

The tenths phase lattice of the model is formalized separately in
`Imscribing/Quantum/WindingLattice.lean`.
-/

import Mathlib

namespace Imscribing.Quantum.PeriodReadout

/-! ## The period as a predicate: fixed point plus minimality -/

/-- `r` is the order of `a` modulo `N`: the orbit closes at `r`
    (`a^r = 1` in `ZMod N`) and no smaller positive `k` closes it. -/
def IsOrder (a N r : ℕ) : Prop :=
  (a : ZMod N) ^ r = 1 ∧
    ∀ k ∈ Finset.Icc 1 (r - 1), (a : ZMod N) ^ k ≠ 1

/-! ## The general fixed point: the orbit closes -/

/-- The orbit closes: the period is the modular fixed point. -/
theorem orbit_closes (a N : ℕ) : (a : ZMod N) ^ orderOf (a : ZMod N) = 1 :=
  pow_orderOf_eq_one (a : ZMod N)

/-! ## The order IS the period, minimally, in the units group

`ZMod N` is not a `LeftCancelMonoid` (zero divisors: in `ZMod 15`,
`3 * 5 = 0`), so the divisibility characterization of `orderOf` does not
apply to its elements directly. It does apply in the units group `(ZMod N)ˣ`,
which is a group. The readout's `a` is a unit exactly when `Nat.Coprime a N`
(`ZMod.isUnit_iff_coprime`). -/

/-- The unit's order closes the orbit and is minimal — the exact contract of
    the readout's `period = Some(r)`. -/
theorem unit_order_is_period (a N : ℕ) (h : Nat.Coprime a N) [NeZero N] :
    (ZMod.unitOfCoprime a h) ^ orderOf (ZMod.unitOfCoprime a h) = 1 ∧
      ∀ k : ℕ, 0 < k → k < orderOf (ZMod.unitOfCoprime a h) →
        (ZMod.unitOfCoprime a h) ^ k ≠ 1 := by
  constructor
  · exact pow_orderOf_eq_one (ZMod.unitOfCoprime a h)
  · intro k hk hklt hk1
    have hdvd : orderOf (ZMod.unitOfCoprime a h) ∣ k :=
      (orderOf_dvd_iff_pow_eq_one).mpr hk1
    have hpos : 0 < orderOf (ZMod.unitOfCoprime a h) :=
      orderOf_pos (ZMod.unitOfCoprime a h)
    exact (not_lt_of_ge (Nat.le_of_dvd hk hdvd)) hklt

/-! ## The orbit exists: gcd(a,N)=1 ⟹ some positive r closes it -/

/-- Coprimality makes `a` a unit of `ZMod N`; the unit group is finite, so
    its order is positive and closes the orbit. The unit's value coerces
    back to `a` (`ZMod.coe_unitOfCoprime`). -/
theorem orbit_exists (a N : ℕ) [NeZero N] (h : Nat.Coprime a N) :
    ∃ r : ℕ, 0 < r ∧ (a : ZMod N) ^ r = 1 := by
  let u : (ZMod N)ˣ := ZMod.unitOfCoprime a h
  refine ⟨orderOf u, orderOf_pos u, ?_⟩
  have hpow : u ^ orderOf u = 1 := pow_orderOf_eq_one u
  have hcoerce : ((u ^ orderOf u : (ZMod N)ˣ) : ZMod N) =
      (u : ZMod N) ^ orderOf u := by
    exact map_pow (Units.coeHom (ZMod N)) u (orderOf u)
  have hu : (u : ZMod N) ^ orderOf u = 1 := by
    simpa [hpow] using hcoerce.symm
  exact (ZMod.coe_unitOfCoprime a h) ▸ hu

/-! ## The sweep: all 11 kernel-verified periods, computed in Lean

The kernel's `fibqc readout` reported `period = Some(r)` for each of these
(word → Jones invariant → winding → r, one shot). Here `decide` verifies
the same `IsOrder`: the fixed point and the minimality, by kernel
computation. The two instruments agree on all 11. -/

example : IsOrder 7 15 4 := by unfold IsOrder; decide
example : IsOrder 2 15 4 := by unfold IsOrder; decide
example : IsOrder 4 15 2 := by unfold IsOrder; decide
example : IsOrder 2 9 6 := by unfold IsOrder; decide
example : IsOrder 2 7 3 := by unfold IsOrder; decide
example : IsOrder 3 7 6 := by unfold IsOrder; decide
example : IsOrder 2 5 4 := by unfold IsOrder; decide
example : IsOrder 3 5 4 := by unfold IsOrder; decide
example : IsOrder 4 9 3 := by unfold IsOrder; decide
example : IsOrder 3 10 4 := by unfold IsOrder; decide
example : IsOrder 3 14 6 := by unfold IsOrder; decide

end Imscribing.Quantum.PeriodReadout
