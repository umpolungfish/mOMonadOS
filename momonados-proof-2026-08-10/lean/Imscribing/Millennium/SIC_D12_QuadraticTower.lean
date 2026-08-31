import Mathlib
import Imscribing.Millennium.SIC_D12_ComputableCyclotomic

/-!
# SIC_D12_QuadraticTower — piece 2a of the d=12 axiom-drop build

The d=12 SIC fiducial coordinates live in the real-quadratic extension of the
cyclotomic ring — `Q(ζ_n, √m)` with `m` the radicand from `Q(√(d(d-2))) = Q(√120)`
(so `√30`, since `√120 = 2√30`). This module extends piece 1's computable ring
`ℚ[ℤ/n]` (`Cyc = List ℚ`) to the computable quadratic tower: elements `a + b·√m`
with `a b : Cyc`, all operations `List`-based and `native_decide`-able.

Complex conjugation fixes `√m` (it is real, `m > 0`) and sends `ζ ↦ ζ⁻¹`; the
distinct field automorphism `√m ↦ −√m` is `galT` (needed later for norms).
-/

namespace SIC.D12.QuadraticTower

open SIC.D12.ComputableCyclotomic

/-- Tower element `a + b·√m` over the cyclotomic ring. -/
structure Tow where
  a : Cyc   -- rational-cyclotomic part
  b : Cyc   -- coefficient of √m
deriving DecidableEq

/-- Scalar multiply a `Cyc` by a rational. -/
def smulC (q : ℚ) (x : Cyc) : Cyc := x.map (fun c => q * c)

def zeroT (n : ℕ) : Tow := ⟨zeroC n, zeroC n⟩
def oneT  (n : ℕ) : Tow := ⟨oneC n, zeroC n⟩
def addT (x y : Tow) : Tow := ⟨addC x.a y.a, addC x.b y.b⟩

/-- `(a+b√m)(c+d√m) = (ac + m·bd) + (ad + bc)√m`. -/
def mulT (n : ℕ) (m : ℚ) (x y : Tow) : Tow :=
  ⟨addC (mulC n x.a y.a) (smulC m (mulC n x.b y.b)),
   addC (mulC n x.a y.b) (mulC n x.b y.a)⟩

/-- Embed a cyclotomic element (no radical part). -/
def ofCyc (c : Cyc) (n : ℕ) : Tow := ⟨c, zeroC n⟩

/-- The radical `√m` itself. -/
def radT (n : ℕ) : Tow := ⟨zeroC n, oneC n⟩

/-- Complex conjugation: `ζ ↦ ζ⁻¹`, `√m` fixed (real). -/
def conjT (n : ℕ) (x : Tow) : Tow := ⟨conjC n x.a, conjC n x.b⟩

/-- Field automorphism `√m ↦ −√m` (the real-quadratic Galois conjugate). -/
def galT (x : Tow) : Tow := ⟨x.a, x.b.map (fun c => - c)⟩

-- ── Foundation checks (native_decide, tower over ℚ[ℤ/12], m = 30) ──

/-- `(√m)² = m` in the tower: `radT² = m·1`. -/
theorem rad_sq : mulT 12 30 (radT 12) (radT 12) = ⟨smulC 30 (oneC 12), zeroC 12⟩ := by
  native_decide

/-- The ring embeds: `ζ · ζ̄ = 1` lifts into the tower. -/
theorem zeta_mul_conj_lifts :
    mulT 12 30 (ofCyc (zeta 12) 12) (conjT 12 (ofCyc (zeta 12) 12)) = oneT 12 := by
  native_decide

/-- `√m · (√m ↦ −√m) = −m` (norm of the radical is `−m`): shows `galT` computes. -/
theorem rad_gal_norm :
    mulT 12 30 (radT 12) (galT (radT 12)) = ⟨smulC (-30) (oneC 12), zeroC 12⟩ := by
  native_decide

/-- `1` is a left identity in the tower (on a nontrivial element). -/
theorem one_mul_zeta :
    mulT 12 30 (oneT 12) (ofCyc (zeta 12) 12) = ofCyc (zeta 12) 12 := by
  native_decide

end SIC.D12.QuadraticTower
