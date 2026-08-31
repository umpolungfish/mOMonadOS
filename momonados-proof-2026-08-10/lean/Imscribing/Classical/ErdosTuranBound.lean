/-
Imscribing/Classical/ErdosTuranBound.lean

Erdős–Turán Upper Bound for Sidon Sets.

Theorem: If A ⊆ ℕ is a Sidon set, then |A ∩ {1,…,N}| = O(√N).
Specifically: for a finite Sidon set B ⊆ {1,…,N}, |B| ≤ √(4N) + 1.

Proof (counting argument):
  For a Sidon set B of size k, the k(k+1)/2 unordered sums a+b
  (a,b ∈ B, a ≤ b) are all distinct and lie in [2, 2N]. Hence
  k(k+1)/2 ≤ 2N, giving k ≤ √(4N) + 1.

Author: Lando⊗⊙perator
-/

import Mathlib

open Finset Set
open scoped Classical

namespace Imscribing.Classical.ErdosTuranBound

set_option linter.unusedVariables false
set_option linter.style.openClassical false

-- ── Sidon property ────────────────────────────────────────

/-- B is Sidon (B₂): a+b = c+d with a,b,c,d ∈ B → {a,b} = {c,d}. -/
def isSidonFinset (B : Finset ℕ) : Prop :=
  ∀ {a b c d : ℕ}, a ∈ B → b ∈ B → c ∈ B → d ∈ B →
    a + b = c + d → (({a, b} : Finset ℕ) = ({c, d} : Finset ℕ))

/-- A ⊆ ℕ is Sidon (B₂): a+b = c+d with a,b,c,d ∈ A → {a,b} = {c,d}. -/
def isSidonSet (A : Set ℕ) : Prop :=
  ∀ {a b c d : ℕ}, a ∈ A → b ∈ A → c ∈ A → d ∈ A →
    a + b = c + d → (({a, b} : Set ℕ) = ({c, d} : Set ℕ))

-- ── Finite intersection helper ─────────────────────────────

/-- The finite intersection A ∩ {1,…,N} as a Finset.
    Uses classical decidability for Set membership. -/
noncomputable def finsetInter (A : Set ℕ) (N : ℕ) : Finset ℕ :=
  (Finset.Icc 1 N).filter (· ∈ A)

/-- If A is a Sidon set, then all its finite intersections are Sidon finsets. -/
lemma sidon_set_imp_finset_sidon (A : Set ℕ) (hA : isSidonSet A) (N : ℕ) :
    isSidonFinset (finsetInter A N) := by
  intro a b c d ha hb hc hd hsum
  have haA : a ∈ A := ((Finset.mem_filter.mp ha).2)
  have hbA : b ∈ A := ((Finset.mem_filter.mp hb).2)
  have hcA : c ∈ A := ((Finset.mem_filter.mp hc).2)
  have hdA : d ∈ A := ((Finset.mem_filter.mp hd).2)
  have hset_Set : (({a, b} : Set ℕ) = ({c, d} : Set ℕ)) :=
    hA haA hbA hcA hdA hsum
  apply Finset.Subset.antisymm
  · intro x hx
    have hx_in_set : x ∈ ({a, b} : Set ℕ) := by simpa using hx
    rw [hset_Set] at hx_in_set
    simpa using hx_in_set
  · intro x hx
    have hx_in_set : x ∈ ({c, d} : Set ℕ) := by simpa using hx
    rw [← hset_Set] at hx_in_set
    simpa using hx_in_set

/-- Elements of finsetInter A N are in {1,…,N}. -/
lemma finsetInter_bounds (A : Set ℕ) (N : ℕ) :
    ∀ b ∈ finsetInter A N, 1 ≤ b ∧ b ≤ N := by
  intro b hb
  have hmem := ((Finset.mem_filter.mp hb).1)
  exact ⟨(Finset.mem_Icc.mp hmem).1, (Finset.mem_Icc.mp hmem).2⟩

-- ── Core combinatorial lemma (constructive) ───────────────

/-- Unordered pairs (a,b) with a ≤ b -/
def unorderedPairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun ⟨a, b⟩ => a ≤ b)

/-- Diagonal pairs (a,a) -/
def diagonalPairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun ⟨a, b⟩ => a = b)

/-- Strictly decreasing pairs (a,b) with a > b -/
def reversePairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun ⟨a, b⟩ => a > b)

/-- Sum of a pair -/
def pairSum (p : ℕ × ℕ) : ℕ := p.1 + p.2

/-- Set of all sums of unordered pairs -/
def sumSet (B : Finset ℕ) : Finset ℕ :=
  (unorderedPairs B).image pairSum

/--
Cardinality of unordered pairs: |B|(|B|+1)/2.

Proof: partition B×B into three disjoint sets — unordered pairs U (a≤b),
reverse pairs L (a>b), and the diagonal D (a=b) which is also in U.
The swap map (a,b)↦(b,a) is a bijection U\\D → L, so |U| = |B| + |L|.
Together with |U|+|L| = |B×B| = |B|², we get 2|U| = |B|²+|B|.
-/
lemma card_unorderedPairs (B : Finset ℕ) :
    (unorderedPairs B).card = B.card * (B.card + 1) / 2 := by
  classical
  set U := unorderedPairs B
  set L := reversePairs B
  set D := diagonalPairs B

  have h_disjoint_UL : Disjoint U L := by
    rw [Finset.disjoint_iff_inter_eq_empty]
    ext ⟨a, b⟩
    constructor
    · intro h
      rcases Finset.mem_inter.mp h with ⟨hU, hL⟩
      rcases Finset.mem_filter.mp hU with ⟨_, hle⟩
      rcases Finset.mem_filter.mp hL with ⟨_, hlt⟩
      omega
    · intro h; simp at h

  have h_union_UL : U ∪ L = B ×ˢ B := by
    ext ⟨a, b⟩
    constructor
    · intro h
      rcases Finset.mem_union.mp h with (hU | hL)
      · rcases Finset.mem_filter.mp hU with ⟨hmem, _⟩
        exact hmem
      · rcases Finset.mem_filter.mp hL with ⟨hmem, _⟩
        exact hmem
    · intro h
      rcases Finset.mem_product.mp h with ⟨ha, hb⟩
      by_cases hle : a ≤ b
      · apply Finset.mem_union_left
        apply Finset.mem_filter.mpr
        exact ⟨Finset.mem_product.mpr ⟨ha, hb⟩, hle⟩
      · apply Finset.mem_union_right
        apply Finset.mem_filter.mpr
        have hlt : a > b := by omega
        exact ⟨Finset.mem_product.mpr ⟨ha, hb⟩, hlt⟩

  have h_card_union : U.card + L.card = B.card * B.card := by
    have h_total : (U ∪ L).card = U.card + L.card :=
      Finset.card_union_of_disjoint h_disjoint_UL
    rw [h_union_UL, Finset.card_product] at h_total
    omega

  -- Diagonal: bijection B → D via a ↦ (a,a)
  have h_card_D : D.card = B.card := by
    have h_image : D = B.image (fun a => (a, a)) := by
      ext ⟨a, b⟩
      constructor
      · intro hDmem
        rcases Finset.mem_filter.mp hDmem with ⟨hmem, heq⟩
        rcases Finset.mem_product.mp hmem with ⟨ha, hb⟩
        have heq_ab : a = b := by simpa using heq
        subst heq_ab
        apply Finset.mem_image.mpr
        exact ⟨a, ha, rfl⟩
      · intro hImg
        rcases Finset.mem_image.mp hImg with ⟨a, ha, h_eq⟩
        rcases Prod.mk.inj h_eq with ⟨rfl, rfl⟩
        apply Finset.mem_filter.mpr
        exact ⟨Finset.mem_product.mpr ⟨ha, ha⟩, rfl⟩
    rw [h_image]
    apply Finset.card_image_of_injective B
    intro a₁ a₂ h
    exact (Prod.mk.inj h).1

  have hD_sub_U : D ⊆ U := by
    intro p hp
    rcases p with ⟨x, y⟩
    rcases Finset.mem_filter.mp hp with ⟨hmem, heq⟩
    have heq_xy : x = y := by simpa using heq
    subst heq_xy
    apply Finset.mem_filter.mpr
    exact ⟨hmem, le_rfl⟩

  -- Swap map bijection: U \\ D → L
  have h_card_UD : (U \ D).card = L.card := by
    let swap : ℕ × ℕ → ℕ × ℕ := fun p => (p.2, p.1)
    have h_image : (U \ D).image swap = L := by
      ext ⟨a, b⟩
      constructor
      · intro h
        rcases Finset.mem_image.mp h with ⟨⟨x, y⟩, hxy, hswap⟩
        -- hswap: swap (x,y) = (a,b), i.e., (y,x) = (a,b)
        have hy_eq_a : y = a := by
          have := congrArg Prod.fst hswap
          simpa [swap] using this
        have hx_eq_b : x = b := by
          have := congrArg Prod.snd hswap
          simpa [swap] using this
        subst hy_eq_a; subst hx_eq_b
        rcases Finset.mem_sdiff.mp hxy with ⟨hU, h_notD⟩
        rcases Finset.mem_filter.mp hU with ⟨hmem, hle⟩
        rcases Finset.mem_product.mp hmem with ⟨hx, hy⟩
        -- hle: x ≤ y (from U filter)
        -- h_notD: (x,y) ∉ D, so x ≠ y
        have hne : x ≠ y := by
          intro heq_xy
          apply h_notD
          apply Finset.mem_filter.mpr
          exact ⟨Finset.mem_product.mpr ⟨hx, hy⟩, heq_xy⟩
        have hlt' : y > x := by
          by_contra! hge
          have : x = y := by omega
          exact hne this
        apply Finset.mem_filter.mpr
        exact ⟨Finset.mem_product.mpr ⟨hy, hx⟩, hlt'⟩
      · intro h
        rcases Finset.mem_filter.mp h with ⟨hmem, hlt⟩
        rcases Finset.mem_product.mp hmem with ⟨ha, hb⟩
        -- hlt: a > b
        have hle' : b ≤ a := by omega
        have hne : b ≠ a := by omega
        apply Finset.mem_image.mpr
        refine ⟨(b, a), ?_, rfl⟩
        apply Finset.mem_sdiff.mpr
        constructor
        · apply Finset.mem_filter.mpr
          exact ⟨Finset.mem_product.mpr ⟨hb, ha⟩, hle'⟩
        · intro hD
          rcases Finset.mem_filter.mp hD with ⟨_, heq⟩
          have heq_ba : b = a := by simpa using heq
          exact hne heq_ba
    have h_inj : Function.Injective swap := by
      intro p q h
      rcases Prod.mk.inj h with ⟨h21, h12⟩
      exact Prod.ext h12 h21
    rw [← h_image, Finset.card_image_of_injective _ h_inj]

  have h_card_U : U.card = D.card + (U \ D).card := by
    have h := Finset.card_sdiff_add_card_eq_card hD_sub_U
    omega

  rw [h_card_D, h_card_UD] at h_card_U

  -- Now: U.card = B.card + L.card and U.card + L.card = B.card²
  -- Hence 2·U.card = B.card² + B.card = B.card·(B.card+1)
  have h_mul : U.card * 2 = B.card * (B.card + 1) := by
    calc
      U.card * 2 = U.card + U.card := by omega
      _ = (B.card + L.card) + U.card := by rw [h_card_U]
      _ = B.card + (L.card + U.card) := by omega
      _ = B.card + (U.card + L.card) := by rw [add_comm L.card U.card]
      _ = B.card + (B.card * B.card) := by rw [h_card_union]
      _ = B.card * B.card + B.card := by omega
      _ = B.card * (B.card + 1) := by rw [← Nat.mul_succ]

  have h_final : B.card * (B.card + 1) / 2 = U.card :=
    Nat.div_eq_of_eq_mul_left (by norm_num : 0 < 2) h_mul.symm

  exact h_final.symm

/--
Helper lemma: if two 2-element Finsets are equal and their elements
are sorted (a ≤ b, c ≤ d), then the elements match in order.
-/
lemma pair_eq_of_finset_pair_eq_sorted {a b c d : ℕ}
    (hset : ({a, b} : Finset ℕ) = ({c, d} : Finset ℕ))
    (hab : a ≤ b) (hcd : c ≤ d) : a = c ∧ b = d := by
  have ha_mem : a ∈ ({c, d} : Finset ℕ) := by rw [← hset]; simp
  have hb_mem : b ∈ ({c, d} : Finset ℕ) := by rw [← hset]; simp
  rcases Finset.mem_insert.mp ha_mem with (hac | had)
  · -- a = c
    rcases Finset.mem_insert.mp hb_mem with (hbc | hbd)
    · -- b = c, so a = c = b → {c} = {c,d} → d = c
      have heq_ab : a = b := by rw [hac, hbc]
      have hcard1 : ({c, d} : Finset ℕ).card = 1 := by
        rw [← hset, heq_ab]; simp
      have hc_eq_d : c = d := by
        by_contra! hne
        have hcard2 : ({c, d} : Finset ℕ).card = 2 :=
          Finset.card_insert_of_notMem (by simpa using hne)
        rw [hcard1] at hcard2; omega
      exact ⟨hac, by rw [hbc, hc_eq_d]⟩
    · -- b = d
      have hbd_eq : b = d := Finset.mem_singleton.mp hbd
      exact ⟨hac, hbd_eq⟩
  · -- a = d
    have had_eq : a = d := Finset.mem_singleton.mp had
    rcases Finset.mem_insert.mp hb_mem with (hbc | hbd)
    · -- a = d, b = c, so d ≤ c ≤ d → all equal
      have hle' : d ≤ c := by rw [← had_eq, ← hbc]; exact hab
      have heq_cd : c = d := by omega
      have heq_ab : a = b := by rw [had_eq, hbc, heq_cd]
      refine ⟨?_, ?_⟩
      · rw [had_eq, heq_cd]
      · rw [← heq_ab, had_eq]
    · -- a = d, b = d → {d} = {c,d} → c = d
      have hbd_eq : b = d := Finset.mem_singleton.mp hbd
      have heq_ab : a = b := by rw [had_eq, hbd_eq]
      have hcard1 : ({c, d} : Finset ℕ).card = 1 := by
        rw [← hset, heq_ab]; simp
      have hc_eq_d : c = d := by
        by_contra! hne
        have hcard2 : ({c, d} : Finset ℕ).card = 2 :=
          Finset.card_insert_of_notMem (by simpa using hne)
        rw [hcard1] at hcard2; omega
      exact ⟨by rw [hc_eq_d, had_eq], hbd_eq⟩

/--
For a Sidon set B, the sum map on unordered pairs is injective:
if (a,b) and (c,d) are unordered pairs (a≤b, c≤d) with a+b=c+d,
then (a,b) = (c,d).
-/
lemma sumSet_card_eq_unorderedPairs_card (B : Finset ℕ) (hSidon : isSidonFinset B) :
    (sumSet B).card = (unorderedPairs B).card := by
  apply Finset.card_image_of_injOn
  intro p hp q hq hsum
  rcases Finset.mem_filter.mp hp with ⟨hmemp, hlep⟩
  rcases Finset.mem_filter.mp hq with ⟨hmemq, hleq⟩
  rcases Finset.mem_product.mp hmemp with ⟨hp1, hp2⟩
  rcases Finset.mem_product.mp hmemq with ⟨hq1, hq2⟩
  dsimp [pairSum] at hsum
  have hset := hSidon hp1 hp2 hq1 hq2 hsum
  rcases pair_eq_of_finset_pair_eq_sorted hset hlep hleq with ⟨h1, h2⟩
  exact Prod.ext h1 h2

/--
Bound: |Icc 2 (2N)| ≤ 2N.

Simple inductive proof: each step from N to N+1 adds at most 2 new
elements (2N+1 and 2N+2) to Icc 2 (2N).
-/
lemma card_Icc_two_to_2N_le_2N (N : ℕ) : ((Finset.Icc 2 (2*N) : Finset ℕ).card : ℕ) ≤ 2*N := by
  induction' N with N IH
  · simp
  · -- Icc 2 (2*(N+1)) ⊆ Icc 2 (2*N) ∪ {2*N+1, 2*N+2}
    have h_sub : (Finset.Icc 2 (2*(N+1)) : Finset ℕ) ⊆
                (Finset.Icc 2 (2*N) : Finset ℕ) ∪ {(2*N+1), (2*N+2)} := by
      intro x hx
      rcases Finset.mem_Icc.mp hx with ⟨hlo, hhi⟩
      by_cases hxle : x ≤ 2*N
      · apply Finset.mem_union_left
        apply Finset.mem_Icc.mpr
        exact ⟨hlo, hxle⟩
      · apply Finset.mem_union_right
        have hx_cases : x = 2*N+1 ∨ x = 2*N+2 := by omega
        rcases hx_cases with (rfl | rfl)
        · simp
        · simp
    have h_card_sub : ((Finset.Icc 2 (2*(N+1)) : Finset ℕ).card : ℕ) ≤
                     (((Finset.Icc 2 (2*N) : Finset ℕ) ∪ {(2*N+1), (2*N+2)}).card : ℕ) :=
      Finset.card_le_card h_sub
    have h_card_union : (((Finset.Icc 2 (2*N) : Finset ℕ) ∪ {(2*N+1), (2*N+2)}).card : ℕ) ≤
                        ((Finset.Icc 2 (2*N) : Finset ℕ).card : ℕ) + 2 := by
      have h_union_le := Finset.card_union_le (Finset.Icc 2 (2*N)) {(2*N+1), (2*N+2)}
      have h_pair_card : (({(2*N+1), (2*N+2)} : Finset ℕ).card : ℕ) ≤ 2 := by
        simp
      omega
    calc
      ((Finset.Icc 2 (2*(N+1)) : Finset ℕ).card : ℕ) ≤
          (((Finset.Icc 2 (2*N) : Finset ℕ) ∪ {(2*N+1), (2*N+2)}).card : ℕ) := h_card_sub
      _ ≤ ((Finset.Icc 2 (2*N) : Finset ℕ).card : ℕ) + 2 := h_card_union
      _ ≤ 2*N + 2 := by omega
      _ = 2*(N+1) := by omega

/--
Erdős–Turán counting lemma:
If B ⊆ {1,…,N} is Sidon then |B|(|B|+1)/2 ≤ 2N.

Proof: construct the set of unordered sums S = {a+b | a,b ∈ B, a ≤ b}.
By the Sidon property, all k(k+1)/2 sums are distinct, so |S| = k(k+1)/2.
Each sum lies in [2, 2N], so |S| ≤ |Icc 2 (2N)| ≤ 2N.
-/
theorem erdos_turan_counting_lemma (B : Finset ℕ) (N : ℕ)
    (hSidon : isSidonFinset B) (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) :
    B.card * (B.card + 1) / 2 ≤ 2 * N := by
  have h_card_sumSet : (sumSet B).card = (unorderedPairs B).card :=
    sumSet_card_eq_unorderedPairs_card B hSidon
  have h_card_unordered : (unorderedPairs B).card = B.card * (B.card + 1) / 2 :=
    card_unorderedPairs B
  have h_subset : sumSet B ⊆ Finset.Icc 2 (2*N) := by
    intro s hs
    rcases Finset.mem_image.mp hs with ⟨⟨a, b⟩, hp, rfl⟩
    rcases Finset.mem_filter.mp hp with ⟨hmem, hle⟩
    rcases Finset.mem_product.mp hmem with ⟨ha, hb⟩
    rcases hB a ha with ⟨ha1, haN⟩
    rcases hB b hb with ⟨hb1, hbN⟩
    apply Finset.mem_Icc.mpr
    have hlow : 2 ≤ a + b := by omega
    have hhigh : a + b ≤ 2 * N := by omega
    exact ⟨hlow, hhigh⟩
  have h_card_Icc : ((Finset.Icc 2 (2*N) : Finset ℕ).card : ℕ) ≤ 2*N :=
    card_Icc_two_to_2N_le_2N N
  calc
    B.card * (B.card + 1) / 2 = (unorderedPairs B).card := by rw [h_card_unordered]
    _ = (sumSet B).card := by rw [h_card_sumSet]
    _ ≤ ((Finset.Icc 2 (2*N) : Finset ℕ).card : ℕ) :=
      Finset.card_le_card h_subset
    _ ≤ 2 * N := h_card_Icc
-- ── Quadratic bound helper ────────────────────────────────

/--
If x ≥ 0 and x² + x ≤ C, then x ≤ (√(1+4C) - 1) / 2.
-/
lemma quadratic_bound {x C : ℝ} (hx : 0 ≤ x) (h : x ^ 2 + x ≤ C) :
    x ≤ (Real.sqrt (1 + 4*C) - 1) / 2 := by
  have h_nonneg_2x1 : 0 ≤ 2*x + 1 := by nlinarith
  have h_nonneg_C : 0 ≤ 1 + 4*C := by nlinarith
  have h_sq : (2*x + 1)^2 ≤ 1 + 4*C := by nlinarith
  have h_sqrt_ineq : 2*x + 1 ≤ Real.sqrt (1 + 4*C) := by
    have h_sqrt_sq := Real.sqrt_le_sqrt h_sq
    have h_left : Real.sqrt ((2*x + 1)^2) = 2*x + 1 := Real.sqrt_sq h_nonneg_2x1
    rw [h_left] at h_sqrt_sq
    exact h_sqrt_sq
  nlinarith

-- ── Final comparison lemma ─────────────────────────────────

/--
For all N ≥ 0: (√(16N+5) - 1)/2 ≤ √(4N) + 1.
-/
lemma sqrt_comparison {N : ℝ} (hN : 0 ≤ N) :
    (Real.sqrt (16*N + 5) - 1) / 2 ≤ Real.sqrt (4*N) + 1 := by
  have h_sqrt4N : Real.sqrt (4*N) = 2 * Real.sqrt N := by
    rw [Real.sqrt_mul (by norm_num : (0:ℝ) ≤ 4), show Real.sqrt (4 : ℝ) = 2 by norm_num]
  rw [h_sqrt4N]
  have h_rhs_nonneg : 0 ≤ 4*Real.sqrt N + 3 := by
    nlinarith [Real.sqrt_nonneg N]
  have h_sq_sqrtN : (Real.sqrt N)^2 = N := Real.sq_sqrt hN
  have h_nonneg_large : 0 ≤ 16*N + 5 := by nlinarith
  have h_sq_sqrt_large : (Real.sqrt (16*N + 5))^2 = 16*N + 5 :=
    Real.sq_sqrt h_nonneg_large
  have h_sq_ineq : (Real.sqrt (16*N + 5))^2 ≤ (4*Real.sqrt N + 3)^2 := by
    calc
      (Real.sqrt (16*N + 5))^2 = 16*N + 5 := h_sq_sqrt_large
      _ ≤ 16*N + 24*Real.sqrt N + 9 := by
        nlinarith [Real.sqrt_nonneg N]
      _ = 16*(Real.sqrt N)^2 + 24*Real.sqrt N + 9 := by rw [h_sq_sqrtN]
      _ = (4*Real.sqrt N + 3)^2 := by ring
  have h_lhs_nonneg : 0 ≤ Real.sqrt (16*N + 5) := Real.sqrt_nonneg _
  have h_sqrt_ineq' : Real.sqrt (16*N + 5) ≤ 4*Real.sqrt N + 3 := by
    have h_sqrt_sq := Real.sqrt_le_sqrt h_sq_ineq
    have h_left : Real.sqrt ((Real.sqrt (16*N + 5))^2) = Real.sqrt (16*N + 5) :=
      Real.sqrt_sq h_lhs_nonneg
    have h_right : Real.sqrt ((4*Real.sqrt N + 3)^2) = 4*Real.sqrt N + 3 :=
      Real.sqrt_sq h_rhs_nonneg
    rw [h_left, h_right] at h_sqrt_sq
    exact h_sqrt_sq
  nlinarith

-- ── Main bound ────────────────────────────────────────────

/--
Erdős–Turán upper bound: For any finite Sidon set B ⊆ {1,…,N},
|B| ≤ √(4N) + 1.
-/
theorem erdos_turan_bound_finset (B : Finset ℕ) (N : ℕ)
    (hSidon : isSidonFinset B) (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) :
    (B.card : ℝ) ≤ Real.sqrt (4 * (N : ℝ)) + 1 := by
  by_cases hk0 : B.card = 0
  · have hcard0 : (B.card : ℝ) = 0 := by exact_mod_cast hk0
    rw [hcard0]
    have hN_nonneg : (0 : ℝ) ≤ (N : ℝ) := by exact_mod_cast Nat.zero_le N
    have hsqrt_nonneg : 0 ≤ Real.sqrt (4*(N : ℝ)) := Real.sqrt_nonneg _
    nlinarith
  have hk_pos : B.card ≥ 1 := by omega
  have h_ineq_nat : B.card * (B.card + 1) / 2 ≤ 2 * N :=
    erdos_turan_counting_lemma B N hSidon hB
  have h_mul_nat : B.card * (B.card + 1) ≤ 4 * N + 1 := by omega
  have hk_nonneg : (0 : ℝ) ≤ (B.card : ℝ) := by exact_mod_cast Nat.zero_le B.card
  have hN_nonneg : (0 : ℝ) ≤ (N : ℝ) := by exact_mod_cast Nat.zero_le N
  have h_mul_real : (B.card : ℝ) * ((B.card : ℝ) + 1) ≤ 4 * (N : ℝ) + 1 := by
    exact_mod_cast h_mul_nat
  have h_sq_bound : (B.card : ℝ)^2 + (B.card : ℝ) ≤ 4*(N : ℝ) + 1 := by
    nlinarith
  have h_sqrt_bound : (B.card : ℝ) ≤ (Real.sqrt (16*(N : ℝ) + 5) - 1) / 2 := by
    have h := quadratic_bound hk_nonneg h_sq_bound
    rw [show (1 : ℝ) + 4*(4*(N:ℝ)+1) = 16*(N:ℝ) + 5 by ring] at h
    exact h
  have h_final : (Real.sqrt (16*(N : ℝ) + 5) - 1) / 2 ≤ Real.sqrt (4*(N : ℝ)) + 1 :=
    sqrt_comparison hN_nonneg
  nlinarith

-- ── Set version of the bound ───────────────────────────────

/--
For any Sidon set A ⊆ ℕ and any N: |A∩{1,…,N}| ≤ √(4N) + 1.
-/
theorem erdos_turan_bound_set (A : Set ℕ) (hA : isSidonSet A) (N : ℕ) :
    ((finsetInter A N).card : ℝ) ≤ Real.sqrt (4 * (N : ℝ)) + 1 :=
  erdos_turan_bound_finset (finsetInter A N) N
    (sidon_set_imp_finset_sidon A hA N) (finsetInter_bounds A N)

-- ── Asymptotic bound: pointwise ratio estimate ─────────────

/--
For any Sidon set A ⊆ ℕ and any N ≥ 1:
|A∩{1,…,N}| / √N ≤ 2 + 1/√N.
-/
theorem erdos_turan_ratio_bound (A : Set ℕ) (hA : isSidonSet A) (N : ℕ) (hN : N ≥ 1) :
    ((finsetInter A N).card : ℝ) / Real.sqrt (N : ℝ) ≤ 2 + 1 / Real.sqrt (N : ℝ) := by
  have hcard := erdos_turan_bound_set A hA N
  -- hcard: card ≤ √(4N) + 1 = 2√N + 1
  have h_sqrt_4N : Real.sqrt (4 * (N : ℝ)) = 2 * Real.sqrt (N : ℝ) := by
    rw [Real.sqrt_mul (show 0 ≤ (4 : ℝ) from by norm_num),
      show Real.sqrt (4 : ℝ) = 2 by norm_num]
  rw [h_sqrt_4N] at hcard
  -- hcard: card ≤ 2√N + 1
  have hNpos : 0 < N := Nat.pos_of_ne_zero (by omega)
  have hNpos' : (0 : ℝ) < (N : ℝ) := by exact_mod_cast hNpos
  have h_sqrt_pos : Real.sqrt (N : ℝ) > 0 := Real.sqrt_pos.mpr hNpos'
  have h_inv_nonneg : 0 ≤ (Real.sqrt (N : ℝ))⁻¹ :=
    inv_nonneg.mpr (Real.sqrt_nonneg _)
  calc
    ((finsetInter A N).card : ℝ) / Real.sqrt (N : ℝ) =
        ((finsetInter A N).card : ℝ) * (Real.sqrt (N : ℝ))⁻¹ := by rw [div_eq_mul_inv]
    _ ≤ (2 * Real.sqrt (N : ℝ) + 1) * (Real.sqrt (N : ℝ))⁻¹ :=
      mul_le_mul_of_nonneg_right hcard h_inv_nonneg
    _ = (2 * Real.sqrt (N : ℝ) + 1) / Real.sqrt (N : ℝ) := by rw [div_eq_mul_inv]
    _ = 2 + 1 / Real.sqrt (N : ℝ) := by
      field_simp [h_sqrt_pos.ne.symm]

-- ── O(√N) bound with explicit constant ────────────────────

/--
For any Sidon set A ⊆ ℕ and any N ≥ 1:
|A∩{1,…,N}| ≤ 3√N.

This is the clean "big-O" formulation: A(N) = O(√N).
-/
theorem erdos_turan_O_sqrtN (A : Set ℕ) (hA : isSidonSet A) (N : ℕ) (hN : N ≥ 1) :
    ((finsetInter A N).card : ℝ) ≤ 3 * Real.sqrt (N : ℝ) := by
  have h_ratio := erdos_turan_ratio_bound A hA N hN
  have hNpos : 0 < N := Nat.pos_of_ne_zero (by omega)
  have hNpos' : (0 : ℝ) < (N : ℝ) := by exact_mod_cast hNpos
  have h_sqrt_pos : Real.sqrt (N : ℝ) > 0 := Real.sqrt_pos.mpr hNpos'
  -- h_ratio: card/√N ≤ 2 + 1/√N
  -- Since √N ≥ 1 (because N ≥ 1), we have 1/√N ≤ 1, so 2 + 1/√N ≤ 3
  -- Hence card/√N ≤ 3, so card ≤ 3√N
  have h_sqrt_ge_one : Real.sqrt (N : ℝ) ≥ 1 := by
    have hN1 : (1 : ℝ) ≤ (N : ℝ) := by exact_mod_cast hN
    calc
      Real.sqrt (N : ℝ) ≥ Real.sqrt (1 : ℝ) := Real.sqrt_le_sqrt hN1
      _ = 1 := by norm_num
  have h_inv_le_one : 1 / Real.sqrt (N : ℝ) ≤ 1 :=
    (div_le_one h_sqrt_pos).mpr h_sqrt_ge_one
  have h_card_div_three : ((finsetInter A N).card : ℝ) / Real.sqrt (N : ℝ) ≤ 3 := by
    nlinarith
  -- Multiply both sides by √N > 0: card = (card/√N)*√N ≤ 3*√N
  calc
    ((finsetInter A N).card : ℝ) =
        (((finsetInter A N).card : ℝ) / Real.sqrt (N : ℝ)) * Real.sqrt (N : ℝ) := by
      field_simp [h_sqrt_pos.ne.symm]
    _ ≤ 3 * Real.sqrt (N : ℝ) :=
      mul_le_mul_of_nonneg_right h_card_div_three (Real.sqrt_nonneg _)

end Imscribing.Classical.ErdosTuranBound
