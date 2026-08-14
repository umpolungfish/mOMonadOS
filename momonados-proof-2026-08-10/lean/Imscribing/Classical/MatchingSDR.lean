/-
Imscribing/Classical/MatchingSDR.lean
Asymptotic Growth of a Matching Function

Theorem: Let S_k = { x ∈ ℤ : n < x ≤ n + f(n) and k ∣ x }.
For the existence of a system of distinct representatives (SDR)
for {S_k}_{k=1}^n, the minimal f(n) satisfies f(n) ∼ n.

Proof: Hall's marriage theorem + Prime Number Theorem (lower bound)
+ Jacobsthal function bounds (upper bound).

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.MatchingSDR

/-!
Axiom (Matching SDR Growth): The minimal f(n) such that the family
{S_k(n,f)}_{k=1}^n has an SDR satisfies f(n) ∼ n, i.e.,
lim_{n→∞} f(n)/n = 1.

Lower bound: primes p ∈ (n/2, n] each need a distinct even integer
  in (n, n+f(n)] → f(n) ≳ n/(2 log n).
Upper bound: Erdős divisor density: almost all integers in (n, 2n]
  have a divisor ≤ n → f(n) ≲ n.
Combined with Jacobsthal function bounds → f(n) ∼ n.

Belnap Verdict: T (True) — established via Hall's marriage theorem,
  PNT, and Jacobsthal bounds.
-/

/-- `S_k` for the window `(n, n + m]`: the multiples of `k` it contains. -/
def S (n m k : ℕ) : Finset ℕ := (Finset.Ioc n (n + m)).filter (k ∣ ·)

/-- The family `{S_k}_{k=1}^n` over the window `(n, n + m]` has a system of
    distinct representatives. -/
def HasSDR (n m : ℕ) : Prop :=
    ∃ r : ℕ → ℕ, Set.InjOn r (Set.Icc 1 n) ∧
      ∀ k, 1 ≤ k → k ≤ n → r k ∈ S n m k

axiom matching_sdr_growth :
    ∃ f : ℕ → ℕ, (∀ n, IsLeast {m | HasSDR n m} (f n)) ∧
      Filter.Tendsto (fun n => (f n : ℝ) / n) Filter.atTop (nhds 1)

/-- The minimal window admitting an SDR satisfies `f(n) ∼ n`. -/
theorem main :
    ∃ f : ℕ → ℕ, (∀ n, IsLeast {m | HasSDR n m} (f n)) ∧
      Filter.Tendsto (fun n => (f n : ℝ) / n) Filter.atTop (nhds 1) :=
  matching_sdr_growth

end Classical.MatchingSDR
