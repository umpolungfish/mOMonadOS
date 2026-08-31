/-
Imscribing/Millennium/SIC_StarkUnit.lean

The generalized SIC Stark unit formula, formalized.

For a SIC-POVM dimension d, the base field is ℚ(√((d−3)(d+1))) and the
dominant factor of the Stark unit — the fundamental unit of that field — is

    ε_d = ((d−1) + √((d−3)(d+1))) / 2,     N(ε_d) = 1.

The proof is three lines and none of them mentions d:

  1. (d−3)(d+1) = (d−1)² − 4, so the discriminant is N² − 4 with N = d−1;
  2. a field ℚ(√(N²−4)) contains (N + √(N²−4))/2;
  3. that element has norm (N² − (N²−4))/4 = 1, identically.

Step 3 is an identity in the coefficients, not a fact about any particular
dimension, which is why `norm_one` below takes no hypothesis at all. The three
moduli files carried it three times — at d = 16, 20 and 2048, as `15/2`, `19/2`
and `2047/2` — before it was written down once.

Reference: generalized_stark_unit_formula.md (ig-docs).
-/

import Mathlib

namespace SIC.StarkUnit

/-! ## §1 The discriminant -/

/-- The SIC discriminant in dimension `d`, in Appleby's form. -/
def disc (d : ℚ) : ℚ := (d - 3) * (d + 1)

/-- **The shift.** The discriminant is `N² − 4` for `N = d − 1`. This is the
    whole reason the fundamental unit has a closed form: `N² − 4` is exactly the
    shape whose quadratic order contains `(N + √(N²−4))/2`. -/
theorem disc_eq (d : ℚ) : disc d = (d - 1) ^ 2 - 4 := by
  unfold disc; ring

/-! ## §2 The unit and its norm

An element `a + b√m` of `ℚ(√m)` has norm `a² − m·b²`. The Stark unit is
`a = (d−1)/2`, `b = 1/2`. -/

/-- The norm form on `ℚ(√m)`, in coordinates. -/
def normForm (m a b : ℚ) : ℚ := a * a - m * (b * b)

/-- **The Stark unit has norm one, in every dimension.** No hypothesis: this is
    an identity in `d`, so it holds at d = 12 and d = 2048 for the same reason,
    and there is nothing left to verify dimension by dimension. -/
theorem norm_one (d : ℚ) : normForm (disc d) ((d - 1) / 2) (1 / 2) = 1 := by
  unfold normForm disc; ring

/-- The same statement in the form the moduli files use: given a discriminant
    `m` presented as `N² − 4`, the element `(N/2, 1/2)` has norm one. -/
theorem norm_one_of_shift (N m : ℚ) (h : m = N ^ 2 - 4) :
    normForm m (N / 2) (1 / 2) = 1 := by
  subst h; unfold normForm; ring

/-- Unfolded, for use against a concrete pair of coordinates. -/
theorem norm_one_coords (N m : ℚ) (h : m = N ^ 2 - 4) :
    (N / 2) * (N / 2) - m * ((1 / 2) * (1 / 2)) = 1 := by
  subst h; ring

/-- The conjugate `((d−1) − √disc)/2` has the same norm, so the unit and its
    conjugate are inverse to each other — which is what norm one means. -/
theorem norm_one_conj (d : ℚ) : normForm (disc d) ((d - 1) / 2) (-(1 / 2)) = 1 := by
  unfold normForm disc; ring

/-! ## §3 The dimensions whose base field is ℚ(√5)

When the squarefree part of `(d−3)(d+1)` is 5 the unit is a power of the golden
ratio, and `d − 1` is a Lucas number. These are the dimensions the Fibonacci
route reaches directly. The nine listed in the reference are checked here as
integer identities — `disc d = 5 k²` for an explicit `k` — over ℕ, so the
compiler is not in the trusted base. -/

/-- Dimension, and the `k` with `(d−3)(d+1) = 5k²`. -/
def sqrt5Dims : List (ℕ × ℕ) :=
  [ (4, 1), (8, 3), (19, 8), (48, 21), (124, 55),
    (323, 144), (844, 377), (2208, 987), (5779, 2584) ]

/-- Every listed dimension has discriminant `5k²`, so its base field is ℚ(√5). -/
theorem sqrt5_dims_correct :
    ∀ p ∈ sqrt5Dims, (p.1 - 3) * (p.1 + 1) = 5 * p.2 ^ 2 := by
  decide

/-- And each `k` is a Fibonacci number of even index — the `F_{2n}` of the
    reference, so that `d − 1 = L_{2n}`. -/
theorem sqrt5_ks_are_fib :
    sqrt5Dims.map (fun p => p.2) =
      (List.range 9).map (fun n => Nat.fib (2 * (n + 1))) := by
  decide

/-! ## §4 The dimensions already formalized elsewhere

`SIC_D16_Moduli`, `SIC_D20_Moduli` and `SIC_D2048_Moduli` each fix a
discriminant and a fundamental unit. Those are instances of §2, and this section
records the arithmetic that identifies them as such. -/

theorem d16 : disc 16 = 221 ∧ (16 : ℚ) - 1 = 15 := by
  constructor <;> [unfold disc; skip] <;> norm_num

theorem d20 : disc 20 = 357 ∧ (20 : ℚ) - 1 = 19 := by
  constructor <;> [unfold disc; skip] <;> norm_num

theorem d2048 : disc 2048 = 4190205 ∧ (2048 : ℚ) - 1 = 2047 := by
  constructor <;> [unfold disc; skip] <;> norm_num

/-- d = 12, for which the discriminant is 117 = 9·13 and the base field is
    ℚ(√13). The unit is `(11 + 3√13)/2`, since `117 = 11² − 4`. -/
theorem d12 : disc 12 = 117 ∧ (117 : ℚ) = 11 ^ 2 - 4 := by
  constructor <;> [unfold disc; skip] <;> norm_num

end SIC.StarkUnit
