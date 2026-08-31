import Mathlib

/-!
# SIC_D12_RayTower — the d=12 SIC coordinate field as a computable tower

The d=12 SIC coordinate field is the ray class field of `K = Q(√13)` of conductor
`(36) = (3d)`, degree **288** over `ℚ` (ray class group `[6,6,2,2]`, `h_m = 144`).
`bnrclassfield` (PARI) decomposes it — FSPLIT — into six tiny cyclic pieces over
`K`, all with trivially small defining polynomials:

* `p₁ : x² + 1`                              → `i`
* `p₂ : x² − (√13+5)/2`                       → `√((5+√13)/2)`
* `p₃ : x² − (√13−1)/2`                       → `√((√13−1)/2)`
* `p₄ : x² + (√13+1)/2`                       → `√(−(√13+1)/2)`
* `p₅ : x³ − 3x − 1`                          → the `ζ₉` real cyclic cubic
* `p₆ : x³ + (33√13−123)x + (215−59√13)`      → the SIC-specific cyclic cubic

This module builds the **quadratic part** — `K`, then the four quadratic layers
`p₁…p₄` — giving the **degree-32 moduli subfield**, and `native_decide`-validates
each generator against its defining polynomial. The two cubic layers (`p₅ p₆`,
`32 → 288`) extend this in the next piece.

Everything is a concrete computable `structure` with `List`/field arithmetic, so
every identity closes by `native_decide`.
-/

namespace SIC.D12.RayTower

/-! ## Base field `K = Q(√13)`, elements `a + b√13`. -/

structure Q13 where
  a : ℚ
  b : ℚ
deriving DecidableEq

instance : Zero Q13 := ⟨⟨0, 0⟩⟩
instance : One  Q13 := ⟨⟨1, 0⟩⟩
instance : Add  Q13 := ⟨fun x y => ⟨x.a + y.a, x.b + y.b⟩⟩
instance : Neg  Q13 := ⟨fun x => ⟨-x.a, -x.b⟩⟩
/-- `(a+b√13)(c+d√13) = (ac + 13bd) + (ad+bc)√13`. -/
instance : Mul  Q13 := ⟨fun x y => ⟨x.a * y.a + 13 * (x.b * y.b), x.a * y.b + x.b * y.a⟩⟩

/-- `√13` itself. -/
def s13 : Q13 := ⟨0, 1⟩

/-- The defining relation of the base: `(√13)² = 13`. -/
theorem s13_sq : s13 * s13 = (⟨13, 0⟩ : Q13) := by native_decide

/-! ## Generic computable quadratic extension `B[t] / (t² − r)`. -/

structure Ext2 (B : Type) where
  a : B   -- constant part
  b : B   -- coefficient of the new generator `t`
deriving DecidableEq

instance [Zero B] : Zero (Ext2 B) := ⟨⟨0, 0⟩⟩
instance [Zero B] [One B] : One (Ext2 B) := ⟨⟨1, 0⟩⟩
instance [Add B] : Add (Ext2 B) := ⟨fun x y => ⟨x.a + y.a, x.b + y.b⟩⟩
instance [Neg B] : Neg (Ext2 B) := ⟨fun x => ⟨-x.a, -x.b⟩⟩

/-- Embed a base element (no `t` part). -/
def emb2 [Zero B] (x : B) : Ext2 B := ⟨x, 0⟩

/-- The new generator `t = 0 + 1·t`. -/
def gen2 [Zero B] [One B] : Ext2 B := ⟨0, 1⟩

/-- Multiplication with reduction `t² = r`, threading the base multiply `mB`:
    `(a+bt)(c+dt) = (ac + r·bd) + (ad + bc)t`. -/
def mul2 [Add B] (mB : B → B → B) (r : B) (x y : Ext2 B) : Ext2 B :=
  ⟨mB x.a y.a + mB r (mB x.b y.b), mB x.a y.b + mB x.b y.a⟩

/-! ## The four quadratic layers, stacked over `K`. -/

-- layer types
abbrev L1 := Ext2 Q13                 -- adjoin i
abbrev L2 := Ext2 L1                  -- adjoin √((5+√13)/2)
abbrev L3 := Ext2 L2                  -- adjoin √((√13−1)/2)
abbrev L4 := Ext2 L3                  -- adjoin √(−(√13+1)/2)   [degree 32 over ℚ]

-- base multiply and per-layer radicands (radicand of layer n lives in layer n−1)
def mulQ : Q13 → Q13 → Q13 := (· * ·)

def r1 : Q13 := ⟨-1, 0⟩                       -- i² = −1
def r2 : L1  := emb2 ⟨5/2, 1/2⟩               -- (5+√13)/2
def r3 : L2  := emb2 (emb2 ⟨-1/2, 1/2⟩)       -- (√13−1)/2
def r4 : L3  := emb2 (emb2 (emb2 ⟨-1/2, -1/2⟩))  -- −(√13+1)/2

-- per-layer multiplication (each threads the one below)
def mul1 : L1 → L1 → L1 := mul2 mulQ r1
def mul2' : L2 → L2 → L2 := mul2 mul1 r2
def mul3 : L3 → L3 → L3 := mul2 mul2' r3
def mul4 : L4 → L4 → L4 := mul2 mul3 r4

-- the four generators
def gi  : L1 := gen2      -- i
def g2  : L2 := gen2      -- √((5+√13)/2)
def g3  : L3 := gen2      -- √((√13−1)/2)
def g4  : L4 := gen2      -- √(−(√13+1)/2)

/-! ## Each generator satisfies its defining polynomial (`native_decide`). -/

/-- `p₁`: `i² = −1`. -/
theorem gi_sq : mul1 gi gi = emb2 r1 := by native_decide

/-- `i⁴ = 1` — a genuine two-step relation, not just the defining identity. -/
theorem gi_fourth : mul1 (mul1 gi gi) (mul1 gi gi) = (1 : L1) := by native_decide

/-- `p₂`: `(√((5+√13)/2))² = (5+√13)/2`. -/
theorem g2_sq : mul2' g2 g2 = emb2 r2 := by native_decide

/-- `p₂` tied back to the base: `2·g₂² − 5 = √13`  (so `g₂` really adjoins √13). -/
theorem g2_rel :
    mul2' g2 g2 + mul2' g2 g2 + (-(emb2 (emb2 (⟨5, 0⟩ : Q13)))) = emb2 (emb2 s13) := by
  native_decide

/-- `p₃`: `(√((√13−1)/2))² = (√13−1)/2`. -/
theorem g3_sq : mul3 g3 g3 = emb2 r3 := by native_decide

/-- `p₃` tied back: `2·g₃² + 1 = √13`. -/
theorem g3_rel :
    mul3 g3 g3 + mul3 g3 g3 + emb2 (emb2 (emb2 (⟨1, 0⟩ : Q13))) = emb2 (emb2 (emb2 s13)) := by
  native_decide

/-- `p₄`: `(√(−(√13+1)/2))² = −(√13+1)/2`. -/
theorem g4_sq : mul4 g4 g4 = emb2 r4 := by native_decide

/-- `p₄` tied back: `−2·g₄² − 1 = √13`. -/
theorem g4_rel :
    (-(mul4 g4 g4)) + (-(mul4 g4 g4)) + (-(emb2 (emb2 (emb2 (emb2 (⟨1, 0⟩ : Q13))))))
      = emb2 (emb2 (emb2 (emb2 s13))) := by
  native_decide

/-- Cross-layer sanity: `(√13 · i)² = −13` inside `L1`
    (√13 embedded, i the generator; exercises `mul1` on a mixed element). -/
theorem s13i_sq :
    mul1 (mul1 (emb2 s13) gi) (mul1 (emb2 s13) gi) = emb2 (⟨-13, 0⟩ : Q13) := by
  native_decide

end SIC.D12.RayTower
