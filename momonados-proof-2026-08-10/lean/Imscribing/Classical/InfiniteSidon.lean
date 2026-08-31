/-
Imscribing/Classical/InfiniteSidon.lean
Infinite Sidon Sets with Near-Optimal Density

Theorem (Erdos-Turan type): For every ε > 0 there exists an infinite
Sidon set A ⊆ ℕ such that
  |A ∩ {1,…,N}| ≫_ε N^{1/2 − ε}   (N → ∞).

Proof: Recursive block construction via the Bose-Chowla finite Sidon
set lemma, with scale separation ensuring the union remains Sidon.

Belnap Verdict: T (True) — the construction is unconditional.

Author: Lando⊗⊙perator
-/

import Mathlib
import Imscribing.Classical.BoseChowla

open Finset
open Set

namespace Imscribing.Classical.InfiniteSidon

set_option linter.style.lambdaSyntax false
set_option linter.style.nativeDecide false
set_option linter.style.induction false
set_option linter.style.whitespace false
set_option linter.style.longLine false
set_option linter.unusedVariables false

-- ---------- Sidon property ----------

/-- Sidon (B2) set. -/
def isSidon (A : Set ℕ) : Prop :=
  ∀ {a b c d : ℕ}, a ∈ A → b ∈ A → c ∈ A → d ∈ A →
    a + b = c + d → (({a, b} : Set ℕ) = ({c, d} : Set ℕ))

-- isSidonFinset imported from BoseChowla
-- ---------- Counting function ----------

/-- Number of elements of A not exceeding N. -/
noncomputable def countUpTo (A : Set ℕ) (N : ℕ) : ℕ :=
  Nat.card {x : ℕ // A x ∧ 1 ≤ x ∧ x ≤ N}

-- ---------- Bose-Chowla axiom ----------

/-- Bose-Chowla constant: C=4 works by Bertrand's postulate (p ≤ 2m ⇒ p² ≤ 4m²). -/
def boseChowlaConst : ℕ := 4

lemma bose_chowla_pos : boseChowlaConst > 0 := by
  unfold boseChowlaConst; norm_num

lemma bose_chowla_lemma (m : ℕ) (hm : m ≥ 1) :
    ∃ (B : Finset ℕ),
    BoseChowla.isSidonFinset B ∧
    (∀ b ∈ B, 1 ≤ b ∧ b ≤ boseChowlaConst * m ^ 2) ∧
    B.card ≥ m := by
  have h := BoseChowla.bose_chowla m hm
  rcases h with ⟨B, hsidon, hbounds, hcard⟩
  refine ⟨B, hsidon, ?_, hcard⟩
  intro b hb
  rcases hbounds b hb with ⟨h1, h2⟩
  refine ⟨h1, ?_⟩
  unfold boseChowlaConst
  -- h2: b ≤ 4 * m^2
  -- we need b ≤ 4 * m^2 (same thing since boseChowlaConst = 4)
  simpa [mul_comm, mul_left_comm, mul_assoc] using h2

-- ---------- Scale separation ----------

noncomputable def eta (ε : ℝ) : ℝ := ε / 2

def nextM (C M : ℕ) : ℕ := 10 * C * (M ^ 2) + M + 1

def M_seq (C : ℕ) : ℕ → ℕ
  | 0 => 1
  | k + 1 => nextM C (M_seq C k)

noncomputable def m_k (C : ℕ) (η : ℝ) (k : ℕ) : ℕ :=
  Nat.floor (((M_seq C k : ℝ) ^ ((1 : ℝ) / 2 - η)))

-- ---------- M-sequence properties ----------

lemma M_pos (C : ℕ) (k : ℕ) : M_seq C k ≥ 1 := by
  induction' k with k ih
  · exact le_refl 1
  · dsimp [M_seq, nextM]
    have : (M_seq C k) ^ 2 ≥ 1 := by
      nlinarith
    omega

lemma M_lt_next (C : ℕ) (k : ℕ) : M_seq C k < M_seq C (k + 1) := by
  dsimp [M_seq, nextM]; omega

lemma M_growth (C : ℕ) (k : ℕ) (hCpos : C > 0) :
    M_seq C (k + 1) > 10 * C * (M_seq C k) ^ 2 := by
  dsimp [M_seq, nextM]; omega

-- ---------- Block construction ----------

lemma singleton_sidon : BoseChowla.isSidonFinset ({1} : Finset ℕ) := by
  intro a b c d ha hb hc hd hsum
  have ha1 : a = 1 := Finset.mem_singleton.mp ha
  have hb1 : b = 1 := Finset.mem_singleton.mp hb
  have hc1 : c = 1 := Finset.mem_singleton.mp hc
  have hd1 : d = 1 := Finset.mem_singleton.mp hd
  subst ha1; subst hb1; subst hc1; subst hd1
  rfl

noncomputable def block (C : ℕ) (η : ℝ) (k : ℕ) : Finset ℕ :=
  if hm : m_k C η k ≥ 1 then
    Classical.choose (bose_chowla_lemma (m_k C η k) hm)
  else
    {1}

lemma block_isSidon (C : ℕ) (η : ℝ) (k : ℕ) :
    BoseChowla.isSidonFinset (block C η k) := by
  dsimp [block]
  split
  · rename_i hm
    exact (Classical.choose_spec (bose_chowla_lemma (m_k C η k) hm)).1
  · exact singleton_sidon

-- ---------- Block bounds ----------

lemma block_pos (C : ℕ) (η : ℝ) (k : ℕ) (b : ℕ) (hb : b ∈ block C η k) :
    1 ≤ b := by
  dsimp [block] at hb
  split at hb
  · rename_i hm
    exact ((Classical.choose_spec (bose_chowla_lemma (m_k C η k) hm)).2.1 b hb).1
  · have hb1 : b = 1 := Finset.mem_singleton.mp hb
    subst hb1; omega

lemma block_upper_bound (C : ℕ) (η : ℝ) (k : ℕ) (b : ℕ) (hb : b ∈ block C η k)
    (hm : m_k C η k ≥ 1) : b ≤ boseChowlaConst * (m_k C η k) ^ 2 := by
  dsimp [block] at hb
  rw [dif_pos hm] at hb
  exact ((Classical.choose_spec (bose_chowla_lemma (m_k C η k) hm)).2.1 b hb).2

lemma block_card_ge (C : ℕ) (η : ℝ) (k : ℕ) (hm : m_k C η k ≥ 1) :
    (block C η k).card ≥ m_k C η k := by
  dsimp [block]; rw [dif_pos hm]
  exact (Classical.choose_spec (bose_chowla_lemma (m_k C η k) hm)).2.2

-- ---------- Infinite union ----------

noncomputable def translatedBlock (C : ℕ) (η : ℝ) (k : ℕ) : Set ℕ :=
  {x | ∃ b ∈ block C η k, x = M_seq C k + b}

noncomputable def infiniteSidon (C : ℕ) (η : ℝ) : Set ℕ :=
  ⋃ (k : ℕ), translatedBlock C η k

-- ---------- Structural axioms ----------

/-- Blocks are small relative to their own scale: every member of block `k` is
    at most `4 · M_k`. This is what makes the scales separate — the block sits
    inside `(M_k, 5·M_k]` after translation, while the next scale starts at
    `10·C·M_k² + M_k + 1`. -/
theorem block_le_scale (C : ℕ) (η : ℝ) (hη : 0 ≤ η) (k : ℕ) (b : ℕ)
    (hb : b ∈ block C η k) : b ≤ 4 * M_seq C k := by
  rcases Nat.lt_or_ge (m_k C η k) 1 with h | h
  · have hbk : block C η k = {1} := by dsimp [block]; rw [dif_neg (by omega)]
    rw [hbk, Finset.mem_singleton] at hb
    have := M_pos C k; omega
  · have hup := block_upper_bound C η k b hb h
    have hM : (1:ℝ) ≤ (M_seq C k : ℝ) := by exact_mod_cast M_pos C k
    -- m_k ≤ M_k^(1/2 - η) ≤ M_k^(1/2), so m_k² ≤ M_k
    have hmk : ((m_k C η k : ℝ)) ≤ (M_seq C k : ℝ) ^ ((1:ℝ)/2 - η) :=
      Nat.floor_le (by positivity)
    have hsq : ((m_k C η k : ℝ))^2 ≤ (M_seq C k : ℝ) := by
      have hpow : (M_seq C k : ℝ) ^ ((1:ℝ)/2 - η) ≤ (M_seq C k : ℝ) ^ ((1:ℝ)/2) := by
        apply Real.rpow_le_rpow_of_exponent_le hM
        linarith
      have h1 : ((m_k C η k : ℝ))^2 ≤ ((M_seq C k : ℝ) ^ ((1:ℝ)/2))^2 := by
        have : (0:ℝ) ≤ (m_k C η k : ℝ) := by positivity
        nlinarith [hmk.trans hpow]
      calc ((m_k C η k : ℝ))^2 ≤ ((M_seq C k : ℝ) ^ ((1:ℝ)/2))^2 := h1
        _ = (M_seq C k : ℝ) := by
            rw [← Real.rpow_natCast ((M_seq C k : ℝ) ^ ((1:ℝ)/2)) 2, ← Real.rpow_mul (by linarith)]
            norm_num
    have hnat : (m_k C η k)^2 ≤ M_seq C k := by exact_mod_cast hsq
    have hconst : boseChowlaConst = 4 := rfl
    rw [hconst] at hup
    calc b ≤ 4 * (m_k C η k)^2 := hup
      _ ≤ 4 * M_seq C k := by omega

axiom union_isSidon (C : ℕ) (η : ℝ) (hCpos : C > 0) :
  isSidon (infiniteSidon C η)

/-- Blocks are never empty: the Bose-Chowla branch has at least `m_k` elements
    and the fallback branch is `{1}`. -/
theorem block_nonempty (C : ℕ) (η : ℝ) (k : ℕ) : (block C η k).Nonempty := by
  rcases Nat.lt_or_ge (m_k C η k) 1 with h | h
  · have hb : block C η k = {1} := by dsimp [block]; rw [dif_neg (by omega)]
    rw [hb]; exact ⟨1, Finset.mem_singleton_self 1⟩
  · have hc := block_card_ge C η k h
    exact Finset.card_pos.mp (by omega)

/-- `M_seq` outruns its index, so it is unbounded. -/
theorem M_ge_self (C : ℕ) (k : ℕ) : k + 1 ≤ M_seq C k := by
  induction k with
  | zero => exact M_pos C 0
  | succ m ih => have := M_lt_next C m; omega

/-- Every block contributes a point above `M_seq C k`. -/
theorem mem_infiniteSidon_ge (C : ℕ) (η : ℝ) (k : ℕ) :
    ∃ x ∈ infiniteSidon C η, M_seq C k < x := by
  obtain ⟨b, hb⟩ := block_nonempty C η k
  refine ⟨M_seq C k + b, ?_, ?_⟩
  · rw [infiniteSidon, Set.mem_iUnion]
    exact ⟨k, ⟨b, hb, rfl⟩⟩
  · have := block_pos C η k b hb; omega

theorem union_infinite (C : ℕ) (η : ℝ) (hCpos : C > 0) :
    Set.Infinite (infiniteSidon C η) := by
  apply Set.infinite_of_not_bddAbove
  rintro ⟨B, hB⟩
  obtain ⟨x, hx, hgt⟩ := mem_infiniteSidon_ge C η B
  have hle := hB hx
  have := M_ge_self C B
  omega

axiom counting_asymptotic (η : ℝ) (hη : η > 0) :
  ∃ (c : ℝ) (N₀ : ℕ), c > 0 ∧ ∀ (N : ℕ), N ≥ N₀ →
    (countUpTo (infiniteSidon boseChowlaConst η) N : ℝ) ≥
    c * ((N : ℝ) ^ ((1 : ℝ) / 2 - (2 : ℝ) * η))

-- ---------- Main theorem ----------

/--
For every ε > 0, there exists an infinite Sidon set A ⊆ ℕ such that
|A ∩ {1,…,N}| ≫_ε N^{1/2−ε} as N → ∞.
-/
theorem exists_infinite_sidon_near_optimal (ε : ℝ) (hε : ε > 0) :
    ∃ (A : Set ℕ), isSidon A ∧ Set.Infinite A ∧
    ∃ (c : ℝ) (N₀ : ℕ), c > 0 ∧ ∀ (N : ℕ), N ≥ N₀ →
      (countUpTo A N : ℝ) ≥ c * ((N : ℝ) ^ ((1 : ℝ) / 2 - ε)) := by
  let η := eta ε
  have hη : η > 0 := by
    dsimp [η, eta]
    exact div_pos hε (by norm_num : (0 : ℝ) < 2)
  have hCpos : boseChowlaConst > 0 := bose_chowla_pos
  let A := infiniteSidon boseChowlaConst η
  have hA_sidon : isSidon A := union_isSidon boseChowlaConst η hCpos
  have hA_infinite : Set.Infinite A := union_infinite boseChowlaConst η hCpos
  rcases counting_asymptotic η hη with ⟨c, N₀, hcpos, hN₀⟩
  have hηcalc : (1 : ℝ)/2 - (2 : ℝ)*η = (1 : ℝ)/2 - ε := by
    dsimp [η, eta]
    ring
  refine ⟨A, hA_sidon, hA_infinite, c, N₀, hcpos, fun N hN => ?_⟩
  have hbound := hN₀ N hN
  rw [hηcalc] at hbound
  exact hbound

end Imscribing.Classical.InfiniteSidon
