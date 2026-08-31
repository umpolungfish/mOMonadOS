import Mathlib
import Imscribing.Millennium.SIC_D12_RayTower

/-! # SIC_D12_RayCubic — the cubic layer on the degree-32 tower

The generator satisfies θ³ = 3θ + 1, the ray class cubic x³ − 3x − 1.

The relation is not established by multiplying elements of the tower. `mul2`
calls its base multiply five times, so `mul4` is 5⁴ base products and `mul3g` is
thirteen of those; one `mulC5` is roughly eight thousand rational products and
the cube nests one inside another. Measured: a single `mul4` elaborates, a
single `mulC5` overflows the stack, at any recursion limit and with or without
the derived `DecidableEq`. No tactic reaches it, because the term is exponential
by construction.

So nothing here evaluates a product. Zero and one are lifted through the four
`Ext2` layers symbolically — pure rearrangement of the multiplication rule — and
the cube follows by rewriting with those. This is the route the Stark work
already takes: recognise the value from structural position rather than
constructing the field and computing in it.
-/

set_option maxRecDepth 8000
set_option maxSynthPendingDepth 20

namespace SIC.D12.RayCubic

open SIC.D12.RayTower

structure Ext3 (B : Type) where
  a : B
  b : B
  c : B
deriving DecidableEq

instance [Zero B] : Zero (Ext3 B) := ⟨⟨0, 0, 0⟩⟩
instance [Zero B] [One B] : One (Ext3 B) := ⟨⟨1, 0, 0⟩⟩
instance [Add B] : Add (Ext3 B) := ⟨fun x y => ⟨x.a + y.a, x.b + y.b, x.c + y.c⟩⟩
instance [Neg B] : Neg (Ext3 B) := ⟨fun x => ⟨-x.a, -x.b, -x.c⟩⟩

def emb3 [Zero B] (x : B) : Ext3 B := ⟨x, 0, 0⟩
def gen3 [Zero B] [One B] : Ext3 B := ⟨0, 1, 0⟩

def mul3g [Add B] (mB : B → B → B) (p q : B) (x y : Ext3 B) : Ext3 B :=
  let t3 := mB x.b y.c + mB x.c y.b
  let t4 := mB x.c y.c
  ⟨ mB x.a y.a + mB q t3,
    (mB x.a y.b + mB x.b y.a) + mB p t3 + mB q t4,
    (mB x.a y.c + mB x.b y.b + mB x.c y.a) + mB p t4 ⟩

abbrev C5 := Ext3 L4

def toL4 (x : Q13) : L4 := emb2 (emb2 (emb2 (emb2 x)))

def p5p : L4 := toL4 ⟨3, 0⟩
def p5q : L4 := toL4 ⟨1, 0⟩
noncomputable def mulC5 (x y : C5) : C5 := mul3g mul4 p5p p5q x y

def gc5 : C5 := gen3

/-! ## Projections — every one is `rfl`, so nothing is computed. -/

@[simp] theorem q0a : (0 : Q13).a = 0 := rfl
@[simp] theorem q0b : (0 : Q13).b = 0 := rfl
@[simp] theorem q1a : (1 : Q13).a = 1 := rfl
@[simp] theorem q1b : (1 : Q13).b = 0 := rfl
@[simp] theorem qma (x y : Q13) : (x * y).a = x.a * y.a + 13 * (x.b * y.b) := rfl
@[simp] theorem qmb (x y : Q13) : (x * y).b = x.a * y.b + x.b * y.a := rfl
@[simp] theorem qaa (x y : Q13) : (x + y).a = x.a + y.a := rfl
@[simp] theorem qab (x y : Q13) : (x + y).b = x.b + y.b := rfl

@[simp] theorem e0a {B} [Zero B] : (0 : Ext2 B).a = 0 := rfl
@[simp] theorem e0b {B} [Zero B] : (0 : Ext2 B).b = 0 := rfl
@[simp] theorem e1a {B} [Zero B] [One B] : (1 : Ext2 B).a = 1 := rfl
@[simp] theorem e1b {B} [Zero B] [One B] : (1 : Ext2 B).b = 0 := rfl
@[simp] theorem eaa {B} [Add B] (x y : Ext2 B) : (x + y).a = x.a + y.a := rfl
@[simp] theorem eab {B} [Add B] (x y : Ext2 B) : (x + y).b = x.b + y.b := rfl
@[simp] theorem m2a {B} [Add B] (mB : B → B → B) (r : B) (x y : Ext2 B) :
    (mul2 mB r x y).a = mB x.a y.a + mB r (mB x.b y.b) := rfl
@[simp] theorem m2b {B} [Add B] (mB : B → B → B) (r : B) (x y : Ext2 B) :
    (mul2 mB r x y).b = mB x.a y.b + mB x.b y.a := rfl

@[simp] theorem t0a {B} [Zero B] : (0 : Ext3 B).a = 0 := rfl
@[simp] theorem t0b {B} [Zero B] : (0 : Ext3 B).b = 0 := rfl
@[simp] theorem t0c {B} [Zero B] : (0 : Ext3 B).c = 0 := rfl
@[simp] theorem t1a {B} [Zero B] [One B] : (1 : Ext3 B).a = 1 := rfl
@[simp] theorem t1b {B} [Zero B] [One B] : (1 : Ext3 B).b = 0 := rfl
@[simp] theorem t1c {B} [Zero B] [One B] : (1 : Ext3 B).c = 0 := rfl
@[simp] theorem taa {B} [Add B] (x y : Ext3 B) : (x + y).a = x.a + y.a := rfl
@[simp] theorem tab {B} [Add B] (x y : Ext3 B) : (x + y).b = x.b + y.b := rfl
@[simp] theorem tac {B} [Add B] (x y : Ext3 B) : (x + y).c = x.c + y.c := rfl
@[simp] theorem g3a {B} [Zero B] [One B] : (gen3 : Ext3 B).a = 0 := rfl
@[simp] theorem g3b {B} [Zero B] [One B] : (gen3 : Ext3 B).b = 1 := rfl
@[simp] theorem g3c {B} [Zero B] [One B] : (gen3 : Ext3 B).c = 0 := rfl

/-! ## Extensionality — the frame a deposit is banked in. -/

theorem q_ext {x y : Q13} (ha : x.a = y.a) (hb : x.b = y.b) : x = y := by
  cases x; cases y; simp_all
theorem e_ext {B} {x y : Ext2 B} (ha : x.a = y.a) (hb : x.b = y.b) : x = y := by
  cases x; cases y; simp_all
theorem t_ext {B} {x y : Ext3 B} (ha : x.a = y.a) (hb : x.b = y.b) (hc : x.c = y.c) : x = y := by
  cases x; cases y; simp_all

/-! ## Base: Q13. -/

theorem qzl (z : Q13) : mulQ 0 z = 0 := q_ext (by simp [mulQ]) (by simp [mulQ])
theorem qzr (z : Q13) : mulQ z 0 = 0 := q_ext (by simp [mulQ]) (by simp [mulQ])
theorem qol (z : Q13) : mulQ 1 z = z := q_ext (by simp [mulQ]) (by simp [mulQ])
theorem qor (z : Q13) : mulQ z 1 = z := q_ext (by simp [mulQ]) (by simp [mulQ])
theorem q00 : (0:Q13) + 0 = 0 := q_ext (by simp) (by simp)
theorem qa0 (z : Q13) : z + 0 = z := q_ext (by simp) (by simp)
theorem q0a' (z : Q13) : 0 + z = z := q_ext (by simp) (by simp)

/-! ## One layer up. No product is evaluated: this is the rule rearranged. -/

theorem lift_zl {B} [Zero B] [Add B] (mB : B → B → B) (r : B)
    (hzl : ∀ z, mB 0 z = 0) (hzr : ∀ z, mB z 0 = 0) (haz : (0:B) + 0 = 0) (y : Ext2 B) :
    mul2 mB r 0 y = 0 := e_ext (by simp [m2a, hzl, hzr, haz]) (by simp [m2b, hzl, haz])
theorem lift_zr {B} [Zero B] [Add B] (mB : B → B → B) (r : B)
    (hzl : ∀ z, mB 0 z = 0) (hzr : ∀ z, mB z 0 = 0) (haz : (0:B) + 0 = 0) (x : Ext2 B) :
    mul2 mB r x 0 = 0 := e_ext (by simp [m2a, hzl, hzr, haz]) (by simp [m2b, hzr, haz])
theorem lift_ol {B} [Zero B] [One B] [Add B] (mB : B → B → B) (r : B)
    (hzl : ∀ z, mB 0 z = 0) (hzr : ∀ z, mB z 0 = 0) (hol : ∀ z, mB 1 z = z)
    (haz : (0:B) + 0 = 0) (hz0 : ∀ z : B, z + 0 = z) (h0z : ∀ z : B, 0 + z = z) (y : Ext2 B) :
    mul2 mB r 1 y = y := e_ext (by simp [m2a, hzl, hzr, hol, hz0]) (by simp [m2b, hzl, hol, hz0, h0z])
theorem lift_or {B} [Zero B] [One B] [Add B] (mB : B → B → B) (r : B)
    (hzl : ∀ z, mB 0 z = 0) (hzr : ∀ z, mB z 0 = 0) (hor : ∀ z, mB z 1 = z)
    (haz : (0:B) + 0 = 0) (hz0 : ∀ z : B, z + 0 = z) (h0z : ∀ z : B, 0 + z = z) (x : Ext2 B) :
    mul2 mB r x 1 = x := e_ext (by simp [m2a, hzl, hzr, hor, hz0]) (by simp [m2b, hzr, hor, hz0, h0z])
theorem e00 {B} [Zero B] [Add B] (h : (0:B)+0 = 0) : (0 : Ext2 B) + 0 = 0 :=
  e_ext (by simp [h]) (by simp [h])
theorem ea0 {B} [Zero B] [Add B] (h : ∀ z : B, z + 0 = z) (z : Ext2 B) : z + 0 = z :=
  e_ext (by simp [h]) (by simp [h])
theorem e0a' {B} [Zero B] [Add B] (h : ∀ z : B, 0 + z = z) (z : Ext2 B) : 0 + z = z :=
  e_ext (by simp [h]) (by simp [h])

/-! ## The four layers, climbed. -/

theorem m1zl (z : L1) : mul1 0 z = 0 := lift_zl mulQ r1 qzl qzr q00 z
theorem m1zr (z : L1) : mul1 z 0 = 0 := lift_zr mulQ r1 qzl qzr q00 z
theorem m1ol (z : L1) : mul1 1 z = z := lift_ol mulQ r1 qzl qzr qol q00 qa0 q0a' z
theorem m1or (z : L1) : mul1 z 1 = z := lift_or mulQ r1 qzl qzr qor q00 qa0 q0a' z
theorem a1_00 : (0:L1) + 0 = 0 := e00 q00
theorem a1_z0 (z : L1) : z + 0 = z := ea0 qa0 z
theorem a1_0z (z : L1) : 0 + z = z := e0a' q0a' z

theorem m2zl (z : L2) : mul2' 0 z = 0 := lift_zl mul1 r2 m1zl m1zr a1_00 z
theorem m2zr (z : L2) : mul2' z 0 = 0 := lift_zr mul1 r2 m1zl m1zr a1_00 z
theorem m2ol (z : L2) : mul2' 1 z = z := lift_ol mul1 r2 m1zl m1zr m1ol a1_00 a1_z0 a1_0z z
theorem m2or (z : L2) : mul2' z 1 = z := lift_or mul1 r2 m1zl m1zr m1or a1_00 a1_z0 a1_0z z
theorem a2_00 : (0:L2) + 0 = 0 := e00 a1_00
theorem a2_z0 (z : L2) : z + 0 = z := ea0 a1_z0 z
theorem a2_0z (z : L2) : 0 + z = z := e0a' a1_0z z

theorem m3zl (z : L3) : mul3 0 z = 0 := lift_zl mul2' r3 m2zl m2zr a2_00 z
theorem m3zr (z : L3) : mul3 z 0 = 0 := lift_zr mul2' r3 m2zl m2zr a2_00 z
theorem m3ol (z : L3) : mul3 1 z = z := lift_ol mul2' r3 m2zl m2zr m2ol a2_00 a2_z0 a2_0z z
theorem m3or (z : L3) : mul3 z 1 = z := lift_or mul2' r3 m2zl m2zr m2or a2_00 a2_z0 a2_0z z
theorem a3_00 : (0:L3) + 0 = 0 := e00 a2_00
theorem a3_z0 (z : L3) : z + 0 = z := ea0 a2_z0 z
theorem a3_0z (z : L3) : 0 + z = z := e0a' a2_0z z

theorem m4zl (z : L4) : mul4 0 z = 0 := lift_zl mul3 r4 m3zl m3zr a3_00 z
theorem m4zr (z : L4) : mul4 z 0 = 0 := lift_zr mul3 r4 m3zl m3zr a3_00 z
theorem m4ol (z : L4) : mul4 1 z = z := lift_ol mul3 r4 m3zl m3zr m3ol a3_00 a3_z0 a3_0z z
theorem m4or (z : L4) : mul4 z 1 = z := lift_or mul3 r4 m3zl m3zr m3or a3_00 a3_z0 a3_0z z
theorem a4_z0 (z : L4) : z + 0 = z := ea0 a3_z0 z
theorem a4_0z (z : L4) : 0 + z = z := e0a' a3_0z z

theorem z3 {B} [Zero B] [Add B] (h : (0:B) + 0 = 0) : (0:B) = 0 + 0 + 0 := by
  rw [h, h]

/-! ## The two embedding facts: the coefficients of the cubic. -/

theorem q_is_one : p5q = (1 : L4) := rfl
theorem p_is_three : p5p = (1 : L4) + 1 + 1 := by
  refine e_ext (e_ext (e_ext (e_ext (q_ext ?_ ?_) ?_) ?_) ?_) ?_ <;>
    first
      | (show (3:ℚ) = _; norm_num)
      | (show (0:ℚ) = _; norm_num)
      | exact z3 q00
      | exact z3 a1_00
      | exact z3 a2_00
      | exact z3 a3_00
      | (simp [p5p, toL4, emb2]; norm_num)

/-! ## The cubic. Every `mul4` below is discharged by a lifted lemma. -/

theorem gc5_sq : mulC5 gc5 gc5 = (⟨0, 0, 1⟩ : C5) :=
  t_ext (by simp [mulC5, mul3g, gc5, m4zl, m4zr, m4ol, a4_z0, a4_0z])
        (by simp [mulC5, mul3g, gc5, m4zl, m4zr, m4ol, a4_z0, a4_0z])
        (by simp [mulC5, mul3g, gc5, m4zl, m4zr, m4ol, a4_z0, a4_0z])

theorem gc5_cubic : mulC5 (mulC5 gc5 gc5) gc5 = gc5 + gc5 + gc5 + (1 : C5) := by
  rw [gc5_sq]
  refine t_ext ?_ ?_ ?_ <;>
    simp [mulC5, mul3g, gc5, m4zl, m4zr, m4ol, m4or, a4_z0, a4_0z, q_is_one, p_is_three]

end SIC.D12.RayCubic
