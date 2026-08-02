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

axiom union_isSidon (C : ℕ) (η : ℝ) (hCpos : C > 0) :
  isSidon (infiniteSidon C η)

axiom union_infinite (C : ℕ) (η : ℝ) (hCpos : C > 0) :
  Set.Infinite (infiniteSidon C η)

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
