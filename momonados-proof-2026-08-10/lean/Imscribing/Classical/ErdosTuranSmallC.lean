/-
# The difference bound, and where C = 2 actually sits

`ErdosTuranBound.lean` bounds a Sidon set by counting its SUMS: the `k(k+1)/2`
distinct sums lie in `[2, 2N]`, giving `k(k+1)/2 ≤ 2N` and so `k ≲ 2√N`.

Counting DIFFERENCES is twice as strong. The `k(k-1)/2` positive differences of a
Sidon set are distinct and lie in `[1, N]`, so

    k(k-1)/2 ≤ N,   equivalently   k² ≤ 2N + k

and `k ≲ √(2N)`. That is the formal form of the measurement: on a `C = 2`
optimum the difference spectrum is perfect, every difference occurring exactly
once, while the sum spectrum only reports `r ≤ 2`. The sum reading throws away a
factor of two, and the difference reading is where it is kept.

## Where this leaves C = 2

It leaves it exactly on the line, which is worth stating precisely because it is
the reason the case is delicate rather than easy.

A basis must cover `[N₀, N]`, and each covered point consumes an unordered pair,
so `k(k+1)/2 ≥ N - N₀ + 1` and `k ≳ √(2N)`. The difference bound gives
`k ≲ √(2N)`. The demand and the bound have the SAME leading constant. They meet
and do not cross.

So `C = 2` is not closed by this, and no sharpening of the sum side would close
it either, since the sum side is the weaker of the two. What would close it is
the sharp Sidon bound `k ≤ √N + O(N^{1/4})`, whose constant is `1` rather than
`√2` and which therefore crosses the covering demand. That bound is Erdős and
Turán's windowed argument, not the one-line difference count, and it is the
named next plank.

The finite search agrees with the arithmetic: at `C = 2` coverage never reaches
the full window at any size measured, and the deficit grows as a rising fraction
of it, which is what sitting on the line and losing looks like.
-/

import Mathlib
import Imscribing.Classical.SidonRepresentation
import Imscribing.Classical.RepresentationParity

namespace Imscribing.Classical

open Finset

/-- The strictly-decreasing ordered pairs: one per positive difference. -/
def diffPairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun p => p.2 < p.1)

/-- The strictly-increasing ordered pairs. -/
def risePairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun p => p.1 < p.2)

/-- The diagonal of `B ×ˢ B`. -/
def diagPairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun p => p.1 = p.2)

lemma card_diagPairs (B : Finset ℕ) : (diagPairs B).card = B.card := by
  classical
  unfold diagPairs
  rw [← Finset.card_image_of_injOn (f := Prod.fst)
        (by intro p hp q hq h
            simp only [Finset.mem_coe, Finset.mem_filter] at hp hq
            exact Prod.ext h (by rw [← hp.2, ← hq.2]; exact h))]
  congr 1
  ext a
  simp only [Finset.mem_image, Finset.mem_filter, Finset.mem_product]
  constructor
  · rintro ⟨p, ⟨⟨h1, _⟩, _⟩, rfl⟩; exact h1
  · intro ha; exact ⟨(a, a), ⟨⟨ha, ha⟩, rfl⟩, rfl⟩

/-- The swap matches the two strict halves. -/
lemma card_rise_eq_diff (B : Finset ℕ) : (risePairs B).card = (diffPairs B).card := by
  classical
  apply Finset.card_bij (fun p _ => (p.2, p.1))
  · intro p hp
    simp only [risePairs, diffPairs, Finset.mem_filter, Finset.mem_product] at hp ⊢
    exact ⟨⟨hp.1.2, hp.1.1⟩, hp.2⟩
  · intro p hp q hq h
    have h1 : p.2 = q.2 := congrArg Prod.fst h
    have h2 : p.1 = q.1 := congrArg Prod.snd h
    exact Prod.ext h2 h1
  · intro q hq
    simp only [diffPairs, Finset.mem_filter, Finset.mem_product] at hq
    refine ⟨(q.2, q.1), ?_, rfl⟩
    simp only [risePairs, Finset.mem_filter, Finset.mem_product]
    exact ⟨⟨hq.1.2, hq.1.1⟩, hq.2⟩

/-- `B ×ˢ B` is the two strict halves and the diagonal. -/
lemma card_product_split (B : Finset ℕ) :
    2 * (diffPairs B).card + B.card = B.card * B.card := by
  classical
  have hsplit : (diffPairs B).card + ((B ×ˢ B).filter (fun p => ¬ p.2 < p.1)).card
      = (B ×ˢ B).card :=
    Finset.card_filter_add_card_filter_not (s := B ×ˢ B) (fun p : ℕ × ℕ => p.2 < p.1)
  have hrest : (B ×ˢ B).filter (fun p : ℕ × ℕ => ¬ p.2 < p.1)
      = risePairs B ∪ diagPairs B := by
    ext p
    simp only [risePairs, diagPairs, Finset.mem_union, Finset.mem_filter]
    constructor
    · rintro ⟨hm, hn⟩
      rcases Nat.lt_or_ge p.1 p.2 with h | h
      · exact Or.inl ⟨hm, h⟩
      · exact Or.inr ⟨hm, by omega⟩
    · rintro (⟨hm, h⟩ | ⟨hm, h⟩) <;> exact ⟨hm, by omega⟩
  have hdisj : Disjoint (risePairs B) (diagPairs B) := by
    rw [Finset.disjoint_left]
    intro p hp hq
    simp only [risePairs, diagPairs, Finset.mem_filter] at hp hq
    omega
  rw [hrest, Finset.card_union_of_disjoint hdisj, card_rise_eq_diff,
      card_diagPairs] at hsplit
  rw [Finset.card_product] at hsplit
  omega

/-- **Distinct differences.** On a Sidon set, `a - b` determines `(a, b)`. -/
lemma diff_injective {B : Finset ℕ} (hB : isSidonFinset B)
    {a b c d : ℕ} (ha : a ∈ B) (hb : b ∈ B) (hc : c ∈ B) (hd : d ∈ B)
    (hab : b < a) (hcd : d < c) (h : a - b = c - d) : a = c ∧ b = d := by
  have hsum : a + d = c + b := by omega
  have := hB ha hd hc hb hsum
  rcases pair_cases this with ⟨h1, h2⟩ | ⟨h1, h2⟩
  · exact ⟨h1.symm, h2⟩
  · omega

/-- **The difference bound.** A Sidon set inside `[0, N]` has at most `N`
    positive differences, hence `k² ≤ 2N + k`.

    Against `erdos_turan_counting_lemma`, which counts sums and reaches
    `k(k+1)/2 ≤ 2N`, this is stronger by a factor of two in `N`. -/
theorem sidon_diff_bound (B : Finset ℕ) (N : ℕ) (hB : isSidonFinset B)
    (hle : ∀ b ∈ B, b ≤ N) :
    B.card * B.card ≤ 2 * N + B.card := by
  classical
  have hinj : (diffPairs B).card ≤ N := by
    have hmap : ∀ p ∈ diffPairs B, p.1 - p.2 ∈ Finset.Icc 1 N := by
      intro p hp
      simp only [diffPairs, Finset.mem_filter, Finset.mem_product] at hp
      have := hle p.1 hp.1.1
      exact Finset.mem_Icc.mpr ⟨by omega, by omega⟩
    have hinj2 : Set.InjOn (fun p : ℕ × ℕ => p.1 - p.2) (diffPairs B) := by
      intro p hp q hq h
      simp only [Finset.mem_coe, diffPairs, Finset.mem_filter, Finset.mem_product] at hp hq
      obtain ⟨e1, e2⟩ :=
        diff_injective hB hp.1.1 hp.1.2 hq.1.1 hq.1.2 hp.2 hq.2 h
      exact Prod.ext e1 e2
    calc (diffPairs B).card
        = ((diffPairs B).image (fun p => p.1 - p.2)).card :=
          (Finset.card_image_of_injOn hinj2).symm
      _ ≤ (Finset.Icc 1 N).card := Finset.card_le_card (by
          intro v hv
          obtain ⟨p, hp, rfl⟩ := Finset.mem_image.mp hv
          exact hmap p hp)
      _ = N := by simp
  have := card_product_split B
  omega

#print axioms sidon_diff_bound
#print axioms card_product_split

end Imscribing.Classical
