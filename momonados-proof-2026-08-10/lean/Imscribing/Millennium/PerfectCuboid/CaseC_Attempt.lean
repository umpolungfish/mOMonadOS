-- CaseC_Attempt.lean — descent loop, iteration 3.
-- Seeded by attempt_02: residues then map, no parity rcases glue.
import Imscribing.Millennium.PerfectCuboid
import Imscribing.Millennium.PerfectCuboid.DescentTheorem
open Millennium.PerfectCuboid
set_option linter.style.longLine false
namespace Millennium.PerfectCuboid.CaseC.Attempt

lemma sq_mod_four_parity (n : Nat) :
    (Even n → n * n % 4 = 0) ∧ (Odd n → n * n % 4 = 1) := by
  refine ⟨?_, ?_⟩
  · rintro ⟨k, hk⟩; subst hk
    have : (k + k) * (k + k) = 4 * (k * k) := by ring
    rw [this]; omega
  · rintro ⟨k, hk⟩; subst hk
    have : (2 * k + 1) * (2 * k + 1) = 4 * (k * k + k) + 1 := by ring
    rw [this]; omega

lemma odd_of_sq_mod_four_one {n : Nat} (h : n * n % 4 = 1) : Odd n := by
  rcases Nat.even_or_odd n with he | ho
  · have := (sq_mod_four_parity n).1 he; omega
  · exact ho

lemma exactly_one_odd (p : Cuboid) (hg : ¬ Even p.g) :
    (Odd p.a ∧ Even p.b ∧ Even p.c)
  ∨ (Even p.a ∧ Odd p.b ∧ Even p.c)
  ∨ (Even p.a ∧ Even p.b ∧ Odd p.c) := by
  have hgsq : p.g * p.g % 4 = 1 :=
    (sq_mod_four_parity p.g).2 (Nat.not_even_iff_odd.mp hg)
  have hsp := p.h_sp
  have ra := sq_mod_four p.a
  have rb := sq_mod_four p.b
  have rc := sq_mod_four p.c
  -- from hsp and hgsq, exactly one residue is 1
  have E : ∀ n : Nat, n * n % 4 = 0 → Even n := fun n h => even_of_sq_mod_four_zero h
  have O : ∀ n : Nat, n * n % 4 = 1 → Odd n := fun n h => odd_of_sq_mod_four_one h
  rcases ra with ra | ra <;> rcases rb with rb | rb <;> rcases rc with rc | rc
  · exfalso; omega
  · exact Or.inr (Or.inr ⟨E _ ra, E _ rb, O _ rc⟩)
  · exact Or.inr (Or.inl ⟨E _ ra, O _ rb, E _ rc⟩)
  · exfalso; omega
  · exact Or.inl ⟨O _ ra, E _ rb, E _ rc⟩
  · exfalso; omega
  · exfalso; omega
  · exfalso; omega

/-- Iteration 4: with the odd edge singled out, the two face diagonals TO it are
    odd and the remaining diagonal is even. Stated for the c-odd branch; the
    others follow by the symmetry of the face equations. Proven mod 2 from the
    face equations and sq_mod_four_parity. -/
lemma diagonals_parity_c_odd (p : Cuboid)
    (ha : Even p.a) (hb : Even p.b) (hc : Odd p.c) :
    Even p.d ∧ Odd p.e ∧ Odd p.f := by
  refine ⟨?_, ?_, ?_⟩
  · -- d^2 = a^2 + b^2, both even, so d^2 % 4 = 0, so d even
    have : p.d * p.d % 4 = 0 := by
      have hA := (sq_mod_four_parity p.a).1 ha
      have hB := (sq_mod_four_parity p.b).1 hb
      have := p.h_ab; omega
    exact even_of_sq_mod_four_zero this
  · -- e^2 = a^2 + c^2 = even + odd, so e odd
    rcases Nat.even_or_odd p.e with he | he
    · exfalso
      have hA := (sq_mod_four_parity p.a).1 ha
      have hC := (sq_mod_four_parity p.c).2 hc
      have hE := (sq_mod_four_parity p.e).1 he
      have := p.h_ac; omega
    · exact he
  · -- f^2 = b^2 + c^2 = even + odd, so f odd
    rcases Nat.even_or_odd p.f with hf | hf
    · exfalso
      have hB := (sq_mod_four_parity p.b).1 hb
      have hC := (sq_mod_four_parity p.c).2 hc
      have hF := (sq_mod_four_parity p.f).1 hf
      have := p.h_bc; omega
    · exact hf

end Millennium.PerfectCuboid.CaseC.Attempt
