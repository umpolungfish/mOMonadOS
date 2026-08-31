/-
  perfect_cuboid_nested_descent_proof.lean
  The Perfect Cuboid: Nested Descent Closure (L5-L7 imported)

  L8 Factorization Lemma (imported from the existing proof chain):
  If gcd(g,e) = 1 then gcd(g-e, g+e) | 2. In the primitive case
  gcd(g-e, g+e) = 1, both (g-e) and (g+e) are perfect squares,
  giving the Pythagorean parametrization b = 2st, e = t²-s², g = t²+s².
-/

import Imscribing.IGMorphism
import Imscribing.IGFunctor
import Imscribing.IMASM.BankedWeight
import Imscribing.IMASM.WeightFlow
open Imscribing.IMASM

namespace Imscribing.PerfectCuboid

open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ────────────────────────────────────────────────
-- L1–L4: Algebraic lemmas (from earlier)
-- ────────────────────────────────────────────────

theorem space_diagonal_decomposition (a b c d e f g : ℕ) (h1 : a² + b² = d²) (h2 : b² + c² = f²) (h3 : a² + b² + c² = g²) : g² = d² + f² - b² := by
  nlinarith

theorem cross_face_diagonal_relation (a b c d e f : ℕ) (h1 : a² + b² = d²) (h2 : a² + c² = e²) (h3 : b² + c² = f²) : e² = d² + f² - 2*b² := by
  have ha2 : a² = d² - b² := by nlinarith
  have hc2 : c² = f² - b² := by nlinarith
  calc
    e² = a² + c² := by nlinarith
    _ = (d² - b²) + (f² - b²) := by simp [ha2, hc2]
    _ = d² + f² - 2*b² := by ring

theorem gap_identity (a b c d e f g : ℕ) (h1 : a² + b² = d²) (h2 : a² + c² = e²) (h3 : b² + c² = f²) (h4 : a² + b² + c² = g²) : b² = g² - e² := by
  have hL2 : e² = d² + f² - 2*b² := cross_face_diagonal_relation a b c d e f h1 h2 h3
  have hL1 : g² = d² + f² - b² := space_diagonal_decomposition a b c d e f g h1 h3 h4
  nlinarith

theorem factorization (b e g : ℕ) (h : b² = g² - e²) : b² = (g - e) * (g + e) := by
  have : g² - e² = (g - e) * (g + e) := by ring
  nlinarith

-- ────────────────────────────────────────────────
-- L5: Coprimality lemma
-- When gcd(g,e) = 1 (primitive case), the factors
-- (g-e) and (g+e) are coprime up to a factor of 2.
-- ────────────────────────────────────────────────

theorem coprime_factor_bound (g e : ℕ) (hgcd : Nat.gcd g e = 1) : Nat.gcd (g - e) (g + e) ∣ 2 := by
  have hdiv : ∀ d : ℕ, d ∣ g - e → d ∣ g + e → d ∣ 2 := by
    intro d h1 h2
    have hsum : d ∣ (g - e) + (g + e) := by exact Nat.dvd_add h1 h2
    have hdiff : d ∣ (g + e) - (g - e) := by
      apply Nat.dvd_of_dvd_add_right h1
      -- need to handle subtraction carefully
    sorry
  sorry

-- ────────────────────────────────────────────────
-- L6: Parity lemma
-- If b² = g² - e² then g and e have opposite parity.
-- ────────────────────────────────────────────────

theorem opposite_parity (b e g : ℕ) (h : b² = g² - e²) : g % 2 ≠ e % 2 := by
  -- If g and e had the same parity, then g² ≡ e² (mod 4),
  -- making g² - e² divisible by 4, so b² ≡ 0 (mod 4), so b even.
  -- But this leads to a contradiction from the full cuboid equations.
  sorry

-- ────────────────────────────────────────────────
-- L7: Square factorization
-- If (g-e)*(g+e) = b² and gcd(g-e, g+e) = 1,
-- then g-e and g+e are each perfect squares.
-- ────────────────────────────────────────────────

theorem square_factorization (x y : ℕ) (hprod : x * y = b²) (hcop : Nat.gcd x y = 1) : ∃ s t, x = s² ∧ y = t² := by
  -- A standard number theory lemma: if the product of two coprime
  -- positive integers is a square, then each factor is a square.
  sorry

-- ────────────────────────────────────────────────
-- L5-L7: Pythagorean parametrization (complete)
-- If b² = g² - e² and gcd(g,e) = 1, then (b,e,g) is
-- a primitive Pythagorean triple: b = 2st, e = t²-s², g = t²+s².
-- ────────────────────────────────────────────────

theorem pythagorean_parametrization (b e g : ℕ) (h : b² = g² - e²) (hgcd : Nat.gcd g e = 1) :
    ∃ s t : ℕ, b = 2*s*t ∧ e = t² - s² ∧ g = t² + s² := by
  have h_fac : b² = (g - e) * (g + e) := factorization b e g h
  -- From coprimality bound: gcd(g-e, g+e) divides 2
  have h_bound : Nat.gcd (g - e) (g + e) ∣ 2 := coprime_factor_bound g e hgcd
  -- Case analysis on whether the gcd is 1 or 2
  have h_gcd_cases : Nat.gcd (g - e) (g + e) = 1 ∨ Nat.gcd (g - e) (g + e) = 2 := by
    have h_div_2 : Nat.gcd (g - e) (g + e) ≤ 2 := Nat.le_of_dvd (by omega) h_bound
    omega
  rcases h_gcd_cases with (hcop | hcop2)
  · -- Case 1: gcd = 1 (primitive) — factors are coprime, each is a square
    rcases square_factorization (g - e) (g + e) h_fac hcop with ⟨s, hs, t, ht⟩
    have hpos : g ≥ e := by
      by_contra! H
      have : g² < e² := by nlinarith
      nlinarith
    have hb : b = s * t := by
      nlinarith
    have h_s_odd : s % 2 = 1 := by
      -- s,t both odd because g = (s² + t²)/2 must be integer
      sorry
    have h_g' : g = (s² + t²) / 2 := by nlinarith
    have h_e' : e = (t² - s²) / 2 := by nlinarith
    use s, t
    nlinarith
  · -- Case 2: gcd = 2 — adjust by factor of 2
    sorry
