/-
Imscribing/Classical/BoseChowla.lean

Bose-Chowla Construction of Finite Sidon Sets.

Theorem (Bose-Chowla 1960): For every m ≥ 1 there exists a Sidon set
B ⊆ ℕ of size at least m with max(B) ≤ 4·m².

The constant C=4 comes from Bertrand's postulate: p ≤ 2m ⇒ p² ≤ 4m².

The GF(p²) construction is the core. For prime p:
- Fix θ ∈ GF(p²) outside the prime subfield 𝔽_p
- Fix a generator g of GF(p²)*, cyclic of order p²-1
- Map a ∈ 𝔽_p → dlog_g(θ - a) + 1

A collision of sums forces (θ-a)(θ-b) = (θ-c)(θ-d), so two monic
quadratics over 𝔽_p agree at θ. As θ has degree 2 over 𝔽_p their
coefficients agree, so a+b = c+d and ab = cd, so {a,b} = {c,d}.
That is the whole proof, and it is formalized below — no `sorry`,
no appeal to computation.

This ELIMINATES the axiom `bose_chowla` in InfiniteSidon.lean.

Author: Math⊙perator (Lando⊗⊙perator team)
-/

import Mathlib

open Finset Set Nat
open scoped Classical

set_option linter.style.openClassical false

namespace Imscribing.Classical.BoseChowla

-- ============================================================
-- §1. Sidon property
-- ============================================================

/-- B ⊆ ℕ is a Sidon set (B₂): a+b=c+d ⇒ {a,b}={c,d}. -/
def isSidonFinset (B : Finset ℕ) : Prop :=
  ∀ {a b c d : ℕ}, a ∈ B → b ∈ B → c ∈ B → d ∈ B →
    a + b = c + d → (({a, b} : Finset ℕ) = ({c, d} : Finset ℕ))
-- ============================================================
-- §2. Bose-Chowla prime construction (GF(p²) method)
-- ============================================================

section GF

variable {p : ℕ} [Fact p.Prime]

/-- GF(p²). -/
abbrev F (p : ℕ) [Fact p.Prime] := GaloisField p 2

instance : NeZero p := ⟨(Fact.out : p.Prime).ne_zero⟩

/-- The prime subfield embedding 𝔽_p → GF(p²). -/
noncomputable def iota (p : ℕ) [Fact p.Prime] : ZMod p →+* F p := ZMod.castHom dvd_rfl (F p)

lemma iota_injective : Function.Injective (iota p) := (iota p).injective

lemma card_F : Nat.card (F p) = p ^ 2 := GaloisField.card p 2 (by norm_num)

/-- GF(p²) is strictly bigger than its prime subfield, so some θ lies outside it. -/
lemma exists_theta : ∃ θ : F p, θ ∉ Set.range (iota p) := by
  by_contra h
  push_neg at h
  have hs : Function.Surjective (iota p) := fun y => h y
  have hle : Nat.card (F p) ≤ Nat.card (ZMod p) := Nat.card_le_card_of_surjective _ hs
  rw [card_F, Nat.card_eq_fintype_card, ZMod.card] at hle
  have h2 : 2 ≤ p := (Fact.out : p.Prime).two_le
  nlinarith

/-- The heart of Bose-Chowla: θ has degree 2 over 𝔽_p, so two monic quadratics
    with coefficients in 𝔽_p agreeing at θ have the same coefficients, hence the
    same root pair. -/
lemma pair_eq_of_prod_eq {θ : F p} (hθ : θ ∉ Set.range (iota p)) {a b c d : ZMod p}
    (h : (θ - iota p a) * (θ - iota p b) = (θ - iota p c) * (θ - iota p d)) :
    ({a, b} : Finset (ZMod p)) = {c, d} := by
  have hmap : ∀ x y : ZMod p, iota p (x * y) = iota p x * iota p y := fun x y => map_mul _ _ _
  have hkey : iota p ((c + d) - (a + b)) * θ = iota p (c * d - a * b) := by
    have := h
    simp only [map_sub, map_add, hmap] at *
    ring_nf at this ⊢
    linear_combination this
  have hu : (c + d) - (a + b) = 0 := by
    by_contra hne
    apply hθ
    refine ⟨(c * d - a * b) / ((c + d) - (a + b)), ?_⟩
    have hne' : iota p ((c + d) - (a + b)) ≠ 0 := fun hz => hne (iota_injective (by simpa using hz))
    rw [map_div₀, div_eq_iff hne', ← hkey]
    ring
  have hsum : a + b = c + d := by linear_combination -hu
  have hprod : a * b = c * d := by
    have hz : iota p (c * d - a * b) = 0 := by
      rw [← hkey, show ((c + d) - (a + b)) = 0 from hu]
      simp
    have hz0 : iota p (c * d - a * b) = iota p 0 := by rw [hz, map_zero]
    have := iota_injective hz0
    linear_combination -this
  have hroot : (a - c) * (a - d) = 0 := by
    have h' : (a - c) * (a - d) = a * a - (c + d) * a + c * d := by ring
    rw [h', ← hsum, ← hprod]; ring
  rcases mul_eq_zero.mp hroot with h1 | h1
  · have hac : a = c := by linear_combination h1
    have hbd : b = d := by rw [hac] at hsum; linear_combination hsum
    rw [hac, hbd]
  · have had : a = d := by linear_combination h1
    have hbc : b = c := by rw [had] at hsum; linear_combination hsum
    rw [had, hbc, Finset.pair_comm]

/-- The unit θ − ι a, nonzero because θ lies outside the prime subfield. -/
noncomputable def unitOf {θ : F p} (hθ : θ ∉ Set.range (iota p)) (a : ZMod p) : (F p)ˣ :=
  Units.mk0 (θ - iota p a) (fun h => hθ ⟨a, by linear_combination -h⟩)

lemma unitOf_val {θ : F p} (hθ : θ ∉ Set.range (iota p)) (a : ZMod p) :
    ((unitOf hθ a : (F p)ˣ) : F p) = θ - iota p a := rfl

/-- Every unit of GF(p²) is `g ^ k` for a generator `g` and some `k` below the
    group order — the discrete logarithm, taken in `[0, p²-2]`. -/
lemma exists_bounded_log (p : ℕ) [Fact p.Prime] :
    ∃ g : (F p)ˣ, ∀ x : (F p)ˣ, ∃ k : ℕ, k < Nat.card (F p)ˣ ∧ g ^ k = x := by
  obtain ⟨g, hg⟩ := IsCyclic.exists_generator (α := (F p)ˣ)
  refine ⟨g, fun x => ?_⟩
  have hmem : x ∈ Submonoid.powers g := mem_powers_iff_mem_zpowers.mpr (hg x)
  obtain ⟨k, hk⟩ := hmem
  have hord : orderOf g = Nat.card (F p)ˣ := by
    have := orderOf_eq_card_of_forall_mem_zpowers hg
    simpa [Nat.card_eq_fintype_card] using this
  have hpos : 0 < orderOf g := orderOf_pos g
  refine ⟨k % orderOf g, ?_, ?_⟩
  · rw [← hord]; exact Nat.mod_lt _ hpos
  · rw [pow_mod_orderOf]; exact hk

end GF

/--
For a prime p, the Bose-Chowla GF(p²) construction gives a Sidon set of
size p inside [1, p²-1]. Proved, not assumed.
-/
theorem bose_chowla_prime_construction (p : ℕ) (hp : Nat.Prime p) (_hp3 : 3 ≤ p) :
    ∃ (S : Finset ℕ), isSidonFinset S ∧
    (∀ b ∈ S, 1 ≤ b ∧ b ≤ p ^ 2 - 1) ∧
    S.card = p := by
  haveI : Fact p.Prime := ⟨hp⟩
  obtain ⟨θ, hθ⟩ := exists_theta (p := p)
  obtain ⟨g, hlog⟩ := exists_bounded_log p
  set n := Nat.card (F p)ˣ with hn
  have hcard : n = p ^ 2 - 1 := by
    have := Fintype.ofFinite (F p)
    rw [hn, Nat.card_eq_fintype_card, Fintype.card_units, ← Nat.card_eq_fintype_card, card_F]
  set L : (F p)ˣ → ℕ := fun x => Classical.choose (hlog x) with hLdef
  have hL1 : ∀ x, L x < n := fun x => (Classical.choose_spec (hlog x)).1
  have hL2 : ∀ x, g ^ L x = x := fun x => (Classical.choose_spec (hlog x)).2
  set f : ZMod p → ℕ := fun a => L (unitOf hθ a) + 1 with hfdef
  have hpair : ∀ a b c d : ZMod p, f a + f b = f c + f d →
      ({a, b} : Finset (ZMod p)) = {c, d} := by
    intro a b c d hsum
    have hL : L (unitOf hθ a) + L (unitOf hθ b) = L (unitOf hθ c) + L (unitOf hθ d) := by
      simp only [hfdef] at hsum; omega
    have hg : unitOf hθ a * unitOf hθ b = unitOf hθ c * unitOf hθ d := by
      have h1 : g ^ (L (unitOf hθ a) + L (unitOf hθ b)) = unitOf hθ a * unitOf hθ b := by
        rw [pow_add, hL2, hL2]
      have h2 : g ^ (L (unitOf hθ c) + L (unitOf hθ d)) = unitOf hθ c * unitOf hθ d := by
        rw [pow_add, hL2, hL2]
      rw [← h1, ← h2, hL]
    have hval : (θ - iota p a) * (θ - iota p b) = (θ - iota p c) * (θ - iota p d) := by
      have := congrArg (fun u : (F p)ˣ => (u : F p)) hg
      simpa [unitOf_val] using this
    exact pair_eq_of_prod_eq hθ hval
  have hf_inj : Function.Injective f := by
    intro a b hab
    have hp2 := hpair a a b b (by omega)
    have hmem : a ∈ ({b, b} : Finset (ZMod p)) := by rw [← hp2]; simp
    simpa using hmem
  refine ⟨Finset.image f Finset.univ, ?_, ?_, ?_⟩
  · intro x y z w hx hy hz hw hsum
    simp only [Finset.mem_image, Finset.mem_univ, true_and] at hx hy hz hw
    obtain ⟨a, ha⟩ := hx; obtain ⟨b, hb⟩ := hy
    obtain ⟨c, hc⟩ := hz; obtain ⟨d, hd⟩ := hw
    subst ha; subst hb; subst hc; subst hd
    have := congrArg (Finset.image f) (hpair a b c d hsum)
    simpa [Finset.image_insert, Finset.image_singleton] using this
  · intro b hb
    simp only [Finset.mem_image, Finset.mem_univ, true_and] at hb
    obtain ⟨a, ha⟩ := hb
    subst ha
    have := hL1 (unitOf hθ a)
    simp only [hfdef]
    omega
  · rw [Finset.card_image_of_injective _ hf_inj, Finset.card_univ, ZMod.card]

-- ============================================================
-- §3. Main theorem: Bose-Chowla for arbitrary m
-- ============================================================

/--
Bose-Chowla lemma: For every m ≥ 1 there exists a Sidon set
of size ≥ m in [1, 4·m²]. Constant C=4.
-/
theorem bose_chowla (m : ℕ) (hm : m ≥ 1) :
    ∃ (B : Finset ℕ), isSidonFinset B ∧
    (∀ b ∈ B, 1 ≤ b ∧ b ≤ 4 * m ^ 2) ∧
    B.card ≥ m := by
  -- Bertrand's postulate: there exists a prime p with m < p ≤ 2m
  have hm0 : m ≠ 0 := by omega
  have hp_bertrand : ∃ p : ℕ, Nat.Prime p ∧ m < p ∧ p ≤ 2 * m := by
    -- Bertrand's postulate: exists_prime_lt_and_le_two_mul
    have h := exists_prime_lt_and_le_two_mul m hm0
    rcases h with ⟨p, hp_prime, hp_lt, hp_le⟩
    exact ⟨p, hp_prime, hp_lt, hp_le⟩
  rcases hp_bertrand with ⟨p, hp_prime, hp_lo, hp_hi⟩
  -- For m ≥ 2, p ≥ 3 (since m < p)
  -- For m = 1, p could be 2; handle that case separately
  by_cases hm1 : m = 1
  · -- m = 1: trivial Sidon set of size 1
    subst hm1
    refine ⟨{1}, ?_, ?_, ?_⟩
    · -- Sidon property: singleton is trivially Sidon
      intro a b c d ha hb hc hd hsum
      simp at ha hb hc hd
      subst ha; subst hb; subst hc; subst hd
      rfl
    · intro b hb
      simp at hb
      subst hb
      exact ⟨by omega, by omega⟩
    · simp
  ·-- m ≥ 2, so p ≥ 3
    have hp_ge3 : 3 ≤ p := by
      have : m ≥ 2 := by omega
      omega
    rcases bose_chowla_prime_construction p hp_prime hp_ge3 with ⟨S, hS_sidon, hS_bounds, hS_card⟩
    -- S has size p, bounded by p²-1
    -- Since p ≤ 2m, p²-1 ≤ (2m)²-1 < 4m²
    have h_bound : ∀ b ∈ S, 1 ≤ b ∧ b ≤ 4 * m ^ 2 := by
      intro b hb
      rcases hS_bounds b hb with ⟨h1, h2⟩
      have hp_sq_le : p ^ 2 - 1 ≤ 4 * m ^ 2 := by
        have hp_le_2m : p ≤ 2 * m := hp_hi
        have hsq : p ^ 2 ≤ (2 * m) ^ 2 := Nat.pow_le_pow_left hp_le_2m 2
        have : (2 * m) ^ 2 = 4 * m ^ 2 := by ring
        rw [this] at hsq
        omega
      exact ⟨h1, by omega⟩
    have h_size : S.card ≥ m := by
      rw [hS_card]
      omega
    exact ⟨S, hS_sidon, h_bound, h_size⟩

end Imscribing.Classical.BoseChowla
