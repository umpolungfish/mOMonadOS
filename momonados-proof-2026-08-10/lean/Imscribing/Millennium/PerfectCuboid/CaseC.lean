-- Imscribing/Millennium/PerfectCuboid/CaseC.lean
--
-- Consolidation of the descent axioms into ONE, precisely named.
--
-- Grammar workflow (this file is the conventional decomposition of a Grammar
-- construction; see the companion note). The descent-operator obligation was
-- imscribed and its closure checked with the structural verbs: the split→work→
-- fuse protocol closes (⊢∈⊤⊥∋⊡⊣ ⟹ T), so the descent ARCHITECTURE is sound. The
-- kernel closure verdict on the whole theorem, however, is OPEN: it stands on
-- `descent_operator_exists`. The catalog reading locates the cut on the ⊥
-- (chirality) axis — the promotion 𐑖 (two-step, bounded descent: cases A and B)
-- → 𐑫 (inexhaustible descent: case C). That promotion is the single unclosed
-- step, and this file names it exactly.
--
-- What is PROVEN here, from the existing proven lemmas:
--   • g even  ⟹  all seven entries even  (all_seven_even_of_g_even)
--   • For any cuboid: it either descends (g even, halve) or is impossible
--     (all-pairwise-coprime, case B), OR it lands in the one residual case C.
--
-- What remains ONE axiom, stated precisely (no longer opaque):
--   • case_mixed_parity_impossible — g odd and some edge-pair shares a factor.
--     This is the genuine open content, equivalent to the perfect-cuboid
--     conjecture on this branch. It replaces the three original descent axioms;
--     the main non-existence theorem now rests on it alone.

import Imscribing.Millennium.PerfectCuboid
import Imscribing.Millennium.PerfectCuboid.DescentTheorem
import Imscribing.Millennium.PerfectCuboid.DescentOperator

open Millennium.PerfectCuboid
open Millennium.PerfectCuboid.DescentTheorem

set_option linter.style.longLine false

namespace Millennium.PerfectCuboid.CaseC

/- ====================================================================
   g even forces every entry even.
   all_edges_even_when_g_even gives a,b,c. The face diagonals follow:
   d² = a²+b² ≡ 0 (mod 4) with a,b even, so d is even; likewise e, f.
   ==================================================================== -/

lemma all_seven_even_of_g_even (p : Cuboid) (hg : Even p.g) :
    Even p.a ∧ Even p.b ∧ Even p.c ∧ Even p.d ∧ Even p.e ∧ Even p.f ∧ Even p.g := by
  obtain ⟨ha, hb, hc⟩ := all_edges_even_when_g_even p hg
  -- a,b even ⟹ a²+b² ≡ 0 (mod 4); with the face equation, d² ≡ 0, so d even.
  have hsq0 (x : Nat) (hx : Even x) : x * x % 4 = 0 := by
    obtain ⟨k, hk⟩ := hx
    have : (k + k) * (k + k) = 4 * (k * k) := by ring
    rw [hk, this]; omega
  have hd : Even p.d := by
    have : p.d * p.d % 4 = 0 := by
      rw [← p.h_ab]; have := hsq0 p.a ha; have := hsq0 p.b hb; omega
    exact even_of_sq_mod_four_zero this
  have he : Even p.e := by
    have : p.e * p.e % 4 = 0 := by
      rw [← p.h_ac]; have := hsq0 p.a ha; have := hsq0 p.c hc; omega
    exact even_of_sq_mod_four_zero this
  have hf : Even p.f := by
    have : p.f * p.f % 4 = 0 := by
      rw [← p.h_bc]; have := hsq0 p.b hb; have := hsq0 p.c hc; omega
    exact even_of_sq_mod_four_zero this
  exact ⟨ha, hb, hc, hd, he, hf, hg⟩

/- ====================================================================
   THE ONE RESIDUAL AXIOM — case C, stated precisely.

   g odd (so case A, the halving descent, does not apply) AND at least one
   edge-pair shares a common factor (so case B, the all-coprime parity
   contradiction, does not apply). This is the mixed-parity case whose
   sub-analysis (two-squares parametrization g = u²+v² = p²+q², the
   difference-of-squares factorization of c², the gcd sub-cases) remains open.
   It carries exactly the content of the perfect-cuboid conjecture on this
   branch, and it is the ⊥ 𐑖→𐑫 promotion the Grammar located.
   ==================================================================== -/

axiom case_mixed_parity_impossible (p : Cuboid)
    (hg_odd : ¬ Even p.g)
    (hshare : Nat.gcd p.a p.b ≠ 1 ∨ Nat.gcd p.a p.c ≠ 1 ∨ Nat.gcd p.b p.c ≠ 1) :
    False

/- ====================================================================
   descent_operator_exists, now a THEOREM.

   Complete case split: either every entry is even (halve, a genuine smaller
   cuboid), or all three pairs are coprime (case B: impossible), or g is odd
   with a shared pair (case C: the residual axiom). In the two impossible
   branches a smaller cuboid exists vacuously.
   ==================================================================== -/

theorem descent_or_impossible (p : Cuboid) : ∃ q : Cuboid, q.g < p.g := by
  by_cases hcop : Nat.gcd p.a p.b = 1 ∧ Nat.gcd p.a p.c = 1 ∧ Nat.gcd p.b p.c = 1
  · exact (case_all_gcd_one_impossible p hcop.1 hcop.2.1 hcop.2.2).elim
  · by_cases hge : Even p.g
    · exact ⟨halve_cuboid p (all_seven_even_of_g_even p hge),
             halve_g_smaller p (all_seven_even_of_g_even p hge)⟩
    · have hshare : Nat.gcd p.a p.b ≠ 1 ∨ Nat.gcd p.a p.c ≠ 1 ∨ Nat.gcd p.b p.c ≠ 1 := by
        by_contra h
        push_neg at h
        exact hcop ⟨h.1, h.2.1, h.2.2⟩
      exact (case_mixed_parity_impossible p hge hshare).elim

/- ====================================================================
   The perfect cuboid cannot exist — proved from descent_or_impossible by
   well-ordering, resting on the single residual axiom above.
   ==================================================================== -/

theorem no_perfect_cuboid : ¬ ∃ (_ : Cuboid), True := by
  intro h
  obtain ⟨p, _⟩ := h
  have chain : ∀ n : Nat, ∃ q : Cuboid, q.g + n ≤ p.g := by
    intro n
    induction n with
    | zero => exact ⟨p, by omega⟩
    | succ k ih =>
      obtain ⟨q, hq⟩ := ih
      obtain ⟨q', hq'⟩ := descent_or_impossible q
      exact ⟨q', by omega⟩
  obtain ⟨q, hq⟩ := chain (p.g + 1)
  omega

#print axioms descent_or_impossible
#print axioms no_perfect_cuboid

end Millennium.PerfectCuboid.CaseC
