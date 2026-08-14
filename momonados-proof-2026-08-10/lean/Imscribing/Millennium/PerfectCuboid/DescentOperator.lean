-- Imscribing/Millennium/PerfectCuboid/DescentOperator.lean
-- Constructive descent operator for Perfect Cuboid non-existence proof.
-- DISCHARGES: descent, descent_smaller, descent_operator_exists

import Imscribing.Millennium.PerfectCuboid
import Imscribing.Millennium.PerfectCuboid.DescentTheorem
import Mathlib.Data.Nat.Basic
import Mathlib.Tactic

open Millennium.PerfectCuboid

namespace Millennium.PerfectCuboid

-- ====================================================================
-- Algebraic lemmas for halving: substituting p.a=2a, p.b=2b etc. into
-- the cuboid edge identities and simplifying by the factor of 4.
-- ====================================================================

lemma halve_edge_identity_ab {p : Cuboid} {a b d : Nat}
  (ha_eq : p.a = a + a) (hb_eq : p.b = b + b) (hd_eq : p.d = d + d) :
  a * a + b * b = d * d := by
  have h_sub : (a + a) * (a + a) + (b + b) * (b + b) = (d + d) * (d + d) := by
    rw [← ha_eq, ← hb_eq, ← hd_eq, p.h_ab]
  have h1 : (a + a) * (a + a) = 4 * a * a := by ring
  have h2 : (b + b) * (b + b) = 4 * b * b := by ring
  have h3 : (d + d) * (d + d) = 4 * d * d := by ring
  rw [h1, h2, h3] at h_sub
  linarith

lemma halve_edge_identity_ac {p : Cuboid} {a c e : Nat}
  (ha_eq : p.a = a + a) (hc_eq : p.c = c + c) (he_eq : p.e = e + e) :
  a * a + c * c = e * e := by
  have h_sub : (a + a) * (a + a) + (c + c) * (c + c) = (e + e) * (e + e) := by
    rw [← ha_eq, ← hc_eq, ← he_eq, p.h_ac]
  have h1 : (a + a) * (a + a) = 4 * a * a := by ring
  have h2 : (c + c) * (c + c) = 4 * c * c := by ring
  have h3 : (e + e) * (e + e) = 4 * e * e := by ring
  rw [h1, h2, h3] at h_sub
  linarith

lemma halve_edge_identity_bc {p : Cuboid} {b c f : Nat}
  (hb_eq : p.b = b + b) (hc_eq : p.c = c + c) (hf_eq : p.f = f + f) :
  b * b + c * c = f * f := by
  have h_sub : (b + b) * (b + b) + (c + c) * (c + c) = (f + f) * (f + f) := by
    rw [← hb_eq, ← hc_eq, ← hf_eq, p.h_bc]
  have h1 : (b + b) * (b + b) = 4 * b * b := by ring
  have h2 : (c + c) * (c + c) = 4 * c * c := by ring
  have h3 : (f + f) * (f + f) = 4 * f * f := by ring
  rw [h1, h2, h3] at h_sub
  linarith

lemma halve_edge_identity_sp {p : Cuboid} {a b c g : Nat}
  (ha_eq : p.a = a + a) (hb_eq : p.b = b + b) (hc_eq : p.c = c + c) (hg_eq : p.g = g + g) :
  a * a + b * b + c * c = g * g := by
  have h_sub : (a + a) * (a + a) + (b + b) * (b + b) + (c + c) * (c + c) = (g + g) * (g + g) := by
    rw [← ha_eq, ← hb_eq, ← hc_eq, ← hg_eq, p.h_sp]
  have h1 : (a + a) * (a + a) = 4 * a * a := by ring
  have h2 : (b + b) * (b + b) = 4 * b * b := by ring
  have h3 : (c + c) * (c + c) = 4 * c * c := by ring
  have h4 : (g + g) * (g + g) = 4 * g * g := by ring
  rw [h1, h2, h3, h4] at h_sub
  linarith

/-- Core halved cuboid construction. -/
def halve_cuboid_core (p : Cuboid) (a b c d e f g : Nat)
  (ha_eq : p.a = a + a) (hb_eq : p.b = b + b) (hc_eq : p.c = c + c)
  (hd_eq : p.d = d + d) (he_eq : p.e = e + e) (hf_eq : p.f = f + f)
  (hg_eq : p.g = g + g)
  (h_ab : a * a + b * b = d * d) (h_ac : a * a + c * c = e * e)
  (h_bc : b * b + c * c = f * f) (h_sp : a * a + b * b + c * c = g * g) : Cuboid :=
{ a := a, b := b, c := c, d := d, e := e, f := f, g := g,
  ha_pos := by linarith [p.ha_pos],
  hb_pos := by linarith [p.hb_pos],
  hc_pos := by linarith [p.hc_pos],
  hd_pos := by linarith [p.hd_pos],
  he_pos := by linarith [p.he_pos],
  hf_pos := by linarith [p.hf_pos],
  hg_pos := by linarith [p.hg_pos],
  h_ab := h_ab,
  h_ac := h_ac,
  h_bc := h_bc,
  h_sp := h_sp }

/-- Divide a cuboid by 2 when all edges are even. -/
noncomputable def halve_cuboid (p : Cuboid)
  (h : Even p.a ∧ Even p.b ∧ Even p.c ∧ Even p.d ∧ Even p.e ∧ Even p.f ∧ Even p.g) :
  Cuboid :=
let ha := h.1
let hb := h.2.1
let hc := h.2.2.1
let hd := h.2.2.2.1
let he := h.2.2.2.2.1
let hf := h.2.2.2.2.2.1
let hg := h.2.2.2.2.2.2
let a := Classical.choose ha
let b := Classical.choose hb
let c := Classical.choose hc
let d := Classical.choose hd
let e := Classical.choose he
let f := Classical.choose hf
let g := Classical.choose hg
let ha_eq : p.a = a + a := Classical.choose_spec ha
let hb_eq : p.b = b + b := Classical.choose_spec hb
let hc_eq : p.c = c + c := Classical.choose_spec hc
let hd_eq : p.d = d + d := Classical.choose_spec hd
let he_eq : p.e = e + e := Classical.choose_spec he
let hf_eq : p.f = f + f := Classical.choose_spec hf
let hg_eq : p.g = g + g := Classical.choose_spec hg
let h_ab := halve_edge_identity_ab ha_eq hb_eq hd_eq
let h_ac := halve_edge_identity_ac ha_eq hc_eq he_eq
let h_bc := halve_edge_identity_bc hb_eq hc_eq hf_eq
let h_sp := halve_edge_identity_sp ha_eq hb_eq hc_eq hg_eq
halve_cuboid_core p a b c d e f g ha_eq hb_eq hc_eq hd_eq he_eq hf_eq hg_eq h_ab h_ac h_bc h_sp

/-- The g field of halve_cuboid is Classical.choose (h.2.2.2.2.2.2). -/
lemma halve_cuboid_g_eq_choose (p : Cuboid)
  (h : Even p.a ∧ Even p.b ∧ Even p.c ∧ Even p.d ∧ Even p.e ∧ Even p.f ∧ Even p.g) :
  (halve_cuboid p h).g = Classical.choose h.2.2.2.2.2.2 := by
  unfold halve_cuboid
  unfold halve_cuboid_core
  rfl

/-- Halving gives strictly smaller space diagonal. -/
theorem halve_g_smaller (p : Cuboid)
  (h : Even p.a ∧ Even p.b ∧ Even p.c ∧ Even p.d ∧ Even p.e ∧ Even p.f ∧ Even p.g) :
  (halve_cuboid p h).g < p.g := by
  -- Write x for the halved diagonal, Classical.choose of g's evenness. Then
  -- p.g = x + x, the halved cuboid's g is x, and 0 < x because 0 < p.g; so x < 2x.
  set x := Classical.choose h.2.2.2.2.2.2 with hx
  have hg_spec : p.g = x + x := Classical.choose_spec h.2.2.2.2.2.2
  have hg_pos : 0 < x := by
    have h0 : 0 < x + x := by rw [← hg_spec]; exact p.hg_pos
    linarith
  rw [hg_spec, halve_cuboid_g_eq_choose p h, ← hx]
  have : 0 + x < x + x := Nat.add_lt_add_right hg_pos x
  simpa [Nat.zero_add] using this

end Millennium.PerfectCuboid
