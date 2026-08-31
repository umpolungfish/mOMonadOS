import Mathlib

/-!
# SIC_D12_SymmetricModuli — first exact d=12 SIC moduli, machine-checked

Part of the d=12 axiom-drop build (target: `crystal_forces_d12_sic`). This file
enters the first **exact** fiducial data recovered from the 1500-digit numeric
witness and verifies its arithmetic by `native_decide`.

The base field of the d=12 SIC is `F₁₂ = ℚ(√13)` (Appleby discriminant
`(d−3)(d+1) = 9·13`). The symmetric-orbit moduli of the fiducial lie in the
degree-4 field `ℚ(√2, √13)`. From the recovered fiducial `ψ` (unit-normalised,
`∑ₖ |ψₖ|² = 1`) two moduli come out with small height:

  |z₀|² = 1/12 − (1/24)√2 + (1/156)√13 − (1/312)√26
  |z₆|² = 1/12 + (1/24)√2 + (1/156)√13 + (1/312)√26

They are a **Galois-conjugate pair** under `√2 ↦ −√2` (equivalently the
displacement `k ↦ k+6`). This file models `ℚ(√2,√13)` as a computable flat
degree-4 ℚ-algebra with basis `{1, √2, √13, √26}` and checks, by `native_decide`,
the conjugacy, the exact sum, and the exact product of these two moduli.

Scope: this is exact, unconditional arithmetic over the recovered data; it does
**not** by itself discharge `crystal_forces_d12_sic` (which needs all twelve
coordinates as field elements plus the 143 Weyl–Heisenberg overlap identities).
It is the first verified stone of that build: the flat-field engine applied to
genuine d=12 SIC numbers. The two remaining "symmetric" moduli z₃, z₉ do *not*
lie in ℚ(√2,√13) (their recovered coefficients have ~800-bit denominators — the
low-precision-`lindep` artifact signature), so only z₀, z₆ are entered here.
-/

namespace SIC.D12.SymmetricModuli

/-- Element `a + b√2 + c√13 + d√26` of `ℚ(√2, √13)`, as a coefficient 4-tuple. -/
structure Q2_13 where
  a : ℚ  -- 1
  b : ℚ  -- √2
  c : ℚ  -- √13
  d : ℚ  -- √26
deriving DecidableEq, Repr

namespace Q2_13

def add (x y : Q2_13) : Q2_13 := ⟨x.a + y.a, x.b + y.b, x.c + y.c, x.d + y.d⟩

/-- Multiplication from √2·√2=2, √13·√13=13, √26·√26=26, √2·√13=√26,
    √2·√26=2√13, √13·√26=13√2. -/
def mul (x y : Q2_13) : Q2_13 :=
  { a := x.a*y.a + 2*x.b*y.b + 13*x.c*y.c + 26*x.d*y.d
    b := x.a*y.b + x.b*y.a + 13*x.c*y.d + 13*x.d*y.c
    c := x.a*y.c + x.c*y.a + 2*x.b*y.d + 2*x.d*y.b
    d := x.a*y.d + x.d*y.a + x.b*y.c + x.c*y.b }

/-- Nontrivial Galois automorphism `√2 ↦ −√2` (fixes √13, sends √26 ↦ −√26). -/
def conj2 (x : Q2_13) : Q2_13 := ⟨x.a, -x.b, x.c, -x.d⟩

instance : Add Q2_13 := ⟨add⟩
instance : Mul Q2_13 := ⟨mul⟩

def ofRat (q : ℚ) : Q2_13 := ⟨q, 0, 0, 0⟩

end Q2_13

open Q2_13

/-- The exact fiducial modulus |z₀|² ∈ ℚ(√2,√13). -/
def mod0 : Q2_13 := ⟨1/12, -1/24, 1/156, -1/312⟩

/-- The exact fiducial modulus |z₆|² ∈ ℚ(√2,√13). -/
def mod6 : Q2_13 := ⟨1/12, 1/24, 1/156, 1/312⟩

/-- **z₆ is the √2-conjugate of z₀** — the `k ↦ k+6` Galois symmetry of the
    symmetric SIC orbit, exactly. -/
theorem mod6_is_conj_mod0 : mod0.conj2 = mod6 := by native_decide

/-- **Exact sum**: |z₀|² + |z₆|² = 1/6 + (1/78)√13 (rational over ℚ(√13)). -/
theorem mod_sum : mod0 + mod6 = ⟨1/6, 0, 1/78, 0⟩ := by native_decide

/-- **Exact product** (the ℚ(√2)/ℚ-norm of the conjugate pair):
    |z₀|²·|z₆|² = 7/1872 + (1/1872)√13, with no √2, √26 part. -/
theorem mod_prod : mod0 * mod6 = ⟨7/1872, 0, 1/1872, 0⟩ := by native_decide

/-- The product lies in the base field ℚ(√13) (√2 and √26 components vanish) —
    consistent with z₀, z₆ being conjugate over ℚ(√13). -/
theorem mod_prod_in_base : (mod0 * mod6).b = 0 ∧ (mod0 * mod6).d = 0 := by
  native_decide

end SIC.D12.SymmetricModuli
