import Mathlib
import Imscribing.Millennium.SIC_D12_RayTower

/-! # SIC_D12_RayCubic (isolation build: C5 only, recursion limits raised) -/

set_option maxRecDepth 100000
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
def mulC5 : C5 → C5 → C5 := mul3g mul4 p5p p5q

def gc5 : C5 := gen3

theorem gc5_cubic : mulC5 (mulC5 gc5 gc5) gc5 = gc5 + gc5 + gc5 + (1 : C5) := by
  native_decide

end SIC.D12.RayCubic
