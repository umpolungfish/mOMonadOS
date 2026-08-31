/-
# The parity of the representation function

`r_A(n)` counts ORDERED pairs, and off the diagonal every pair arrives with its
swap. So the count is even unless `n` is twice an element, and the diagonal
contributes at most one pair because `a` is determined by `n = 2a`.

    r(n) is odd  ⟺  n = 2a for some a in A

This needs no asymptotics, no bound on `r`, and no Sidon hypothesis. It is the
same Z₂ involution that `sidon_reps_le_two` uses, read for its parity rather
than for its orbit size.

The consequence for Erdős–Turán is a restriction on where a bounded
representation function can put its mass. A set with `r ≤ C` has

    r(n) ∈ {0, 2, 4, …}

for every `n` outside `{2a : a ∈ A}`, and that exceptional set has exactly
`|A ∩ [0, N/2]|` points in a window, which is `O(√N)` once the counting bound of
`ErdosTuranReduction.lean` is in hand. The odd values are therefore confined to
a sparse set, and the even values carry everything else.
-/

import Mathlib
import Imscribing.Classical.SidonRepresentation

namespace Imscribing.Classical

open Finset

/-- The representations of `n` that are not on the diagonal.

    Equality on `ℕ × ℕ` is decidable, so no classical instance is wanted here;
    supplying one made this filter carry a different `Decidable` than the one
    `card_filter_add_card_filter_not` produces, and the split would not typecheck
    against its own halves. -/
def offDiag (B : Finset ℕ) (n : ℕ) : Finset (ℕ × ℕ) :=
  (reps B n).filter (fun p => ¬ p.1 = p.2)

/-- The representations of `n` that are on the diagonal. -/
def onDiag (B : Finset ℕ) (n : ℕ) : Finset (ℕ × ℕ) :=
  (reps B n).filter (fun p => p.1 = p.2)

lemma card_split (B : Finset ℕ) (n : ℕ) :
    (offDiag B n).card + (onDiag B n).card = (reps B n).card := by
  unfold offDiag onDiag
  rw [add_comm]
  exact Finset.card_filter_add_card_filter_not (s := reps B n) (fun p : ℕ × ℕ => p.1 = p.2)

/-- **The swap halves the off-diagonal.** The involution `(a,b) ↦ (b,a)` carries
    the pairs with `a < b` bijectively onto those with `b < a`, and off the
    diagonal there is no third case. -/
lemma card_offDiag_even (B : Finset ℕ) (n : ℕ) : Even (offDiag B n).card := by
  classical
  set D := offDiag B n with hD
  set L := D.filter (fun p => p.1 < p.2) with hL
  set U := D.filter (fun p => ¬ p.1 < p.2) with hU
  have hsplit : L.card + U.card = D.card :=
    Finset.card_filter_add_card_filter_not (s := D) (fun p : ℕ × ℕ => p.1 < p.2)
  -- swap is a bijection from L to U
  have hLU : L.card = U.card := by
    apply Finset.card_bij (fun p _ => (p.2, p.1))
    · -- maps into U
      intro p hp
      simp only [hU, hL, hD, offDiag, Finset.mem_filter] at hp ⊢
      obtain ⟨⟨hmem, hne⟩, hlt⟩ := hp
      refine ⟨⟨?_, ?_⟩, ?_⟩
      · obtain ⟨h1, h2, h3⟩ := mem_reps.mp hmem
        exact mem_reps.mpr ⟨h2, h1, by omega⟩
      · exact fun h => hne h.symm
      · omega
    · -- injective
      intro p hp q hq h
      have h1 : p.2 = q.2 := congrArg Prod.fst h
      have h2 : p.1 = q.1 := congrArg Prod.snd h
      exact Prod.ext h2 h1
    · -- surjective
      intro q hq
      simp only [hU, hD, offDiag, Finset.mem_filter] at hq
      obtain ⟨⟨hmem, hne⟩, hnlt⟩ := hq
      refine ⟨(q.2, q.1), ?_, rfl⟩
      simp only [hL, hD, offDiag, Finset.mem_filter]
      obtain ⟨h1, h2, h3⟩ := mem_reps.mp hmem
      exact ⟨⟨mem_reps.mpr ⟨h2, h1, by omega⟩, fun h => hne h.symm⟩, by omega⟩
  exact ⟨L.card, by omega⟩

/-- The diagonal holds at most one pair: `n = a + a` determines `a`. -/
lemma card_onDiag_le_one (B : Finset ℕ) (n : ℕ) : (onDiag B n).card ≤ 1 := by
  classical
  rw [Finset.card_le_one]
  intro p hp q hq
  simp only [onDiag, Finset.mem_filter] at hp hq
  obtain ⟨hp1, hp2⟩ := hp
  obtain ⟨hq1, hq2⟩ := hq
  obtain ⟨_, _, hps⟩ := mem_reps.mp hp1
  obtain ⟨_, _, hqs⟩ := mem_reps.mp hq1
  have : p.1 = q.1 := by omega
  exact Prod.ext this (by omega)

/-- The diagonal is nonempty exactly when `n` is twice an element. -/
lemma onDiag_nonempty_iff (B : Finset ℕ) (n : ℕ) :
    (onDiag B n).Nonempty ↔ ∃ a ∈ B, n = a + a := by
  classical
  constructor
  · rintro ⟨p, hp⟩
    simp only [onDiag, Finset.mem_filter] at hp
    obtain ⟨hmem, heq⟩ := hp
    obtain ⟨h1, h2, h3⟩ := mem_reps.mp hmem
    exact ⟨p.1, h1, by omega⟩
  · rintro ⟨a, ha, rfl⟩
    refine ⟨(a, a), ?_⟩
    simp only [onDiag, Finset.mem_filter]
    exact ⟨mem_reps.mpr ⟨ha, ha, rfl⟩, trivial⟩

/-- **The parity of `r`.**

    `r_A(n)` is odd exactly when `n` is twice an element of `A`. -/
theorem reps_card_odd_iff (B : Finset ℕ) (n : ℕ) :
    Odd (reps B n).card ↔ ∃ a ∈ B, n = a + a := by
  classical
  have hsplit := card_split B n
  obtain ⟨k, hk⟩ := card_offDiag_even B n
  have hle := card_onDiag_le_one B n
  constructor
  · intro hodd
    -- the diagonal must be nonempty, else the total is even
    rcases Nat.eq_zero_or_pos (onDiag B n).card with h0 | hpos
    · exfalso
      obtain ⟨m, hm⟩ := hodd
      omega
    · exact (onDiag_nonempty_iff B n).mp (Finset.card_pos.mp hpos)
  · intro hex
    have hpos : 0 < (onDiag B n).card :=
      Finset.card_pos.mpr ((onDiag_nonempty_iff B n).mpr hex)
    have hone : (onDiag B n).card = 1 := by omega
    exact ⟨k, by omega⟩

/-- **Where a bounded representation function may put odd values.**

    Restated as a membership condition: outside the doubled set, `r` is even.
    A basis with `r ≤ C` therefore takes values in `{0, 2, …}` off `{2a}`. -/
theorem reps_card_even_off_diagonal (B : Finset ℕ) (n : ℕ)
    (h : ¬ ∃ a ∈ B, n = a + a) : Even (reps B n).card := by
  rcases Nat.even_or_odd (reps B n).card with he | ho
  · exact he
  · exact absurd ((reps_card_odd_iff B n).mp ho) h

/-! ## Forcing the count up

    Parity does more than describe. A representation that exists at all and is
    not on the diagonal cannot be alone, because its swap is a second one. So
    for a basis the count off the diagonal starts at two, not at one, and the
    lower estimate of `ErdosTuranReduction.lean` doubles with no constant in
    front of it. -/

/-- **Off the diagonal, one representation forces two.** -/
theorem two_le_reps_of_nonempty (B : Finset ℕ) (n : ℕ)
    (hne : (reps B n).Nonempty) (hoff : ¬ ∃ a ∈ B, n = a + a) :
    2 ≤ (reps B n).card := by
  have hpos : 0 < (reps B n).card := Finset.card_pos.mpr hne
  have heven : Even (reps B n).card := reps_card_even_off_diagonal B n hoff
  obtain ⟨k, hk⟩ := heven
  omega

/-- **A basis has no representation function bounded by one.**

    With `r ≤ 1`, parity leaves only the doubled set carrying anything: every
    other `n` has an even count of at most one, hence zero, so no `n` off the
    diagonal is represented and the basis property fails at the first such `n`.

    This is the `C = 1` case of Erdős–Turán, and it falls out of the involution
    with no counting at all. -/
theorem no_basis_with_reps_le_one (A : Set ℕ) (N₀ : ℕ)
    (hbasis : ∀ n ≥ N₀, ∃ a ∈ A, ∃ b ∈ A, a + b = n)
    (hbound : ∀ (n : ℕ) (B : Finset ℕ), ↑B ⊆ A → (reps B n).card ≤ 1) :
    False := by
  classical
  -- pick a large n that is odd, so it cannot be twice anything
  obtain ⟨a, ha, b, hb, hab⟩ := hbasis (2 * N₀ + 1) (by omega)
  set n := 2 * N₀ + 1 with hn
  set B : Finset ℕ := {a, b} with hB
  have hBA : (↑B : Set ℕ) ⊆ A := by
    intro x hx
    simp only [hB, Finset.coe_insert, Finset.coe_singleton, Set.mem_insert_iff,
               Set.mem_singleton_iff] at hx
    rcases hx with rfl | rfl
    · exact ha
    · exact hb
  have hmem : (a, b) ∈ reps B n := by
    refine mem_reps.mpr ⟨by simp [hB], by simp [hB], hab⟩
  have hoff : ¬ ∃ c ∈ B, n = c + c := by
    rintro ⟨c, _, hc⟩
    omega
  have := two_le_reps_of_nonempty B n ⟨(a, b), hmem⟩ hoff
  have := hbound n B hBA
  omega

/-- **At `C = 2` the structure is rigid.**

    With `r ≤ 2`, parity leaves `{0, 2}` off the diagonal, so a basis has
    `r(n) = 2` for every large `n` that is not twice an element: EXACTLY one
    unordered pair represents it, never two and never none. The bounded case is
    therefore not a slack condition to be violated somewhere; it pins the whole
    tail to a single value, and a contradiction has that rigidity to attack
    rather than an inequality with room in it. -/
theorem reps_eq_two_of_bound_two (B : Finset ℕ) (n : ℕ)
    (hne : (reps B n).Nonempty) (hoff : ¬ ∃ a ∈ B, n = a + a)
    (hbound : (reps B n).card ≤ 2) :
    (reps B n).card = 2 :=
  le_antisymm hbound (two_le_reps_of_nonempty B n hne hoff)

/-! ## The odd sub-problem

    An odd `n` is never `a + a`, so it has no diagonal representation and its
    count is even. Covering it needs a genuine pair, and since the two summands
    of an odd sum have opposite parity, that pair is MIXED: one even element and
    one odd one.

    So the odd targets are covered by a bipartite sumset `E + O` with no diagonal
    available anywhere in it. Measured on the exact optima, that is where the
    whole deficit sits: at `C = 4` every uncovered integer is odd, without
    exception. -/

/-- **An odd target needs a mixed pair.** Any representation of an odd `n` uses
    one even summand and one odd summand, so the odd targets draw on `E ×ˢ O`
    alone and never on the diagonal. -/
theorem odd_needs_mixed (B : Finset ℕ) (n : ℕ) (hodd : ¬ 2 ∣ n)
    {a b : ℕ} (h : (a, b) ∈ reps B n) :
    (2 ∣ a ∧ ¬ 2 ∣ b) ∨ (¬ 2 ∣ a ∧ 2 ∣ b) := by
  obtain ⟨_, _, hsum⟩ := mem_reps.mp h
  simp only [Nat.dvd_iff_mod_eq_zero] at *
  omega

/-- The diagonal never carries an odd target. -/
theorem onDiag_empty_of_odd (B : Finset ℕ) (n : ℕ) (hodd : ¬ 2 ∣ n) :
    onDiag B n = ∅ := by
  classical
  rw [← Finset.not_nonempty_iff_eq_empty]
  intro hne
  obtain ⟨a, _, ha⟩ := (onDiag_nonempty_iff B n).mp hne
  exact hodd ⟨a, by omega⟩

/-! ## The criterion, stated exactly

    The measurements localize the whole question onto one quantity: the size of
    the mixed sumset inside the window. These two theorems are that criterion in
    its formal form. What is NOT proved is that the quantity must fail; that is
    the conjecture, and it is now a statement about `E + O` alone. -/

/-- The even and odd parts of a finite set. -/
def evenPart (B : Finset ℕ) : Finset ℕ := B.filter (fun a => 2 ∣ a)

/-- The odd part. -/
def oddPart (B : Finset ℕ) : Finset ℕ := B.filter (fun a => ¬ 2 ∣ a)

/-- **The mixed sumset must cover the odd window.**

    Every odd `n` that is represented at all is a sum of one even element and one
    odd element, so a basis with bounded representation function forces the
    bipartite sumset `evenPart + oddPart` to contain every odd `n` past `N₀`.

    This is the covering demand the measurements track, with no counting in it. -/
theorem odd_mem_mixed_sumset (B : Finset ℕ) (n : ℕ) (hodd : ¬ 2 ∣ n)
    (hne : (reps B n).Nonempty) :
    ∃ e ∈ evenPart B, ∃ o ∈ oddPart B, e + o = n := by
  obtain ⟨⟨a, b⟩, hab⟩ := hne
  obtain ⟨ha, hb, hsum⟩ := mem_reps.mp hab
  rcases odd_needs_mixed B n hodd hab with ⟨hae, hbo⟩ | ⟨hao, hbe⟩
  · exact ⟨a, Finset.mem_filter.mpr ⟨ha, hae⟩, b, Finset.mem_filter.mpr ⟨hb, hbo⟩, hsum⟩
  · exact ⟨b, Finset.mem_filter.mpr ⟨hb, hbe⟩, a, Finset.mem_filter.mpr ⟨ha, hao⟩,
      by omega⟩

/-- **The multiplicity on an odd target is halved.**

    `r(n)` counts ordered pairs and an odd `n` has no diagonal, so its
    representations come in swap pairs: the number of UNORDERED mixed pairs
    summing to an odd `n` is exactly `r(n)/2`, hence at most `C/2` under a bound
    of `C`. That halving is why the bipartite problem carries `C/2` where the
    original carries `C`. -/
theorem odd_reps_card_eq_two_mul (B : Finset ℕ) (n : ℕ) (hodd : ¬ 2 ∣ n) :
    ∃ h : ℕ, (reps B n).card = 2 * h := by
  obtain ⟨k, hk⟩ := reps_card_even_off_diagonal B n (by
    rintro ⟨a, _, rfl⟩
    exact hodd ⟨a, by omega⟩)
  exact ⟨k, by omega⟩

#print axioms odd_mem_mixed_sumset
#print axioms odd_reps_card_eq_two_mul
#print axioms odd_needs_mixed
#print axioms reps_card_odd_iff
#print axioms reps_card_even_off_diagonal
#print axioms two_le_reps_of_nonempty
#print axioms no_basis_with_reps_le_one
#print axioms reps_eq_two_of_bound_two

end Imscribing.Classical
