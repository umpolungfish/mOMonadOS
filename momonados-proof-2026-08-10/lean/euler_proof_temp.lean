import Mathlib.Tactic
import Mathlib.Data.Nat.Factorization.Basic
import Mathlib.NumberTheory.ArithmeticFunction.Defs
import Mathlib.NumberTheory.ArithmeticFunction.Misc
open Nat ArithmeticFunction

set_option maxRecDepth 1000000

noncomputable def v₂ (n : ℕ) : ℕ := (Nat.factorization n) 2

-- Lemma: sigma splits over factorization product
lemma sigma_factorization_prod (n : ℕ) (hn : n ≠ 0) :
    sigma 1 n = (∏ p ∈ (factorization n).support, sigma 1 (p ^ ((factorization n) p))) := by
  let S := (factorization n).support
  have h_n_prod : (∏ p ∈ S, p ^ ((factorization n) p)) = n := by
    calc
      (∏ p ∈ S, p ^ ((factorization n) p)) = ((factorization n).prod fun p a => p ^ a) := by
        rw [Finsupp.prod_of_support_subset (factorization n) (by simp) (fun p a => p ^ a) (by simp)]
        simp
      _ = n := Nat.factorization_prod_pow_eq_self hn
  have coprime_prime_pow (p q a b : ℕ) (hp : p.Prime) (hq : q.Prime) (hne : p ≠ q) :
      Nat.Coprime (p ^ a) (q ^ b) := by
    refine Nat.Coprime.pow a b ?_
    apply hp.coprime_iff_not_dvd.mpr
    intro hdiv; apply hne; exact (hp.dvd_dvd hq).mp hdiv
  have h_pairwise : (S : Set ℕ).Pairwise (fun p q => Nat.Coprime
      (p ^ ((factorization n) p)) (q ^ ((factorization n) q))) := by
    intro p hp q hq hne
    have hp_prime : p.Prime := by
      have hp_mem : p ∈ n.primeFactors := by simpa using hp
      exact (Nat.mem_primeFactors.mp hp_mem).1
    have hq_prime : q.Prime := by
      have hq_mem : q ∈ n.primeFactors := by simpa using hq
      exact (Nat.mem_primeFactors.mp hq_mem).1
    exact coprime_prime_pow p q ((factorization n) p) ((factorization n) q) hp_prime hq_prime hne
  calc
    sigma 1 n = sigma 1 (∏ p ∈ S, p ^ ((factorization n) p)) := by rw [h_n_prod]
    _ = (∏ p ∈ S, sigma 1 (p ^ ((factorization n) p))) :=
      (isMultiplicative_sigma (k := 1)).map_prod (fun p => p ^ ((factorization n) p)) S h_pairwise

-- Test: does it build?
#check sigma_factorization_prod
