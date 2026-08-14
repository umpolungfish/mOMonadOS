import Mathlib

/-!
# SIC_D12_ComputableCyclotomic — piece 1 of the d=12 axiom-drop build

Mathlib's `CyclotomicField` is abstract (splitting-field quotients) and not
computable, so `native_decide` cannot evaluate ring identities in it. This module
builds the computable surrogate: the group algebra ℚ[ℤ/n] (= ℚ[X]/(Xⁿ − 1), which
contains the n-th roots of unity as `X = ζ`).

**Representation choice matters.** A first cut used `Fin n → ℚ` with `Finset.sum`;
`native_decide` on function-equality over that machinery did not compile within
ten minutes. This version represents elements as `List ℚ` (length n) with explicit
`List`/`Nat`/`ℚ` operations — `DecidableEq (List ℚ)` and the ops native-compile
fast, so root-of-unity identities are `native_decide` in well under a second. This
is the foundation the 143 Gram identities (piece 4) will be checked in.
-/

namespace SIC.D12.ComputableCyclotomic

/-- Element of ℚ[ℤ/n]: coefficient list of length `n`, index `i` = coeff of ζⁱ. -/
abbrev Cyc := List ℚ

/-- Coefficient accessor (0 outside range). -/
def coeff (a : Cyc) (i : ℕ) : ℚ := a.getD i 0

def zeroC (n : ℕ) : Cyc := List.replicate n 0
def oneC  (n : ℕ) : Cyc := (List.range n).map (fun i => if i = 0 then 1 else 0)
def zeta  (n : ℕ) : Cyc := (List.range n).map (fun i => if i = 1 then 1 else 0)

def addC (a b : Cyc) : Cyc := List.zipWith (· + ·) a b

/-- Cyclic convolution in ℚ[X]/(Xⁿ−1): `(a*b)_k = Σ_i a_i · b_{(k−i) mod n}`. -/
def mulC (n : ℕ) (a b : Cyc) : Cyc :=
  (List.range n).map (fun k =>
    (List.range n).foldl (fun acc i => acc + coeff a i * coeff b ((k + n - i) % n)) 0)

/-- Complex conjugation on roots of unity: `ζ ↦ ζ⁻¹` (reverse the exponent mod n). -/
def conjC (n : ℕ) (a : Cyc) : Cyc :=
  (List.range n).map (fun i => coeff a ((n - i) % n))

/-- Computable power. -/
def powC (n : ℕ) (a : Cyc) : ℕ → Cyc
  | 0     => oneC n
  | k + 1 => mulC n a (powC n a k)

-- ── Foundation checks: native_decide fires on root-of-unity identities ──

/-- ζ · ζ̄ = 1 in ℚ[ℤ/12]: a root of unity times its conjugate is the identity. -/
theorem zeta_mul_conj_eq_one :
    mulC 12 (zeta 12) (conjC 12 (zeta 12)) = oneC 12 := by native_decide

/-- ζ¹² = 1 in ℚ[ℤ/12]. -/
theorem zeta_pow_twelve : powC 12 (zeta 12) 12 = oneC 12 := by native_decide

/-- ζ⁶ ≠ 1 in ℚ[ℤ/12] (ζ is a *primitive* 12-th root, order 12 not 6). -/
theorem zeta_pow_six_ne_one : powC 12 (zeta 12) 6 ≠ oneC 12 := by native_decide

end SIC.D12.ComputableCyclotomic
