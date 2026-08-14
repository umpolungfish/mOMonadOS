/-
Imscribing/Paraconsistent/BelnapAgreement.lean

The copies of Belnap FOUR agree — checked, not asserted.

Four files in this tree declare the same inductive with the same four
constructors in the same order:

  · `src/Init/Paraconsistent.lean`                  — the kernel's own prelude
  · `Imscribing/Paraconsistent/Paraconsistent.lean` — its p4ramill mirror
  · `Imscribing/Paraconsistent/BelnapSplitFuse.lean` — self-contained by design
  · `Imscribing/Paraconsistent/Belnap.lean`         — the widely imported one

They once disagreed. At the pair {N, B} — the two values a distinction creates
rather than selects, and the only pair the truth order leaves incomparable — the
widely imported copy answered B for conjunction and the other three answered N.
Neither was the meet: the first table failed absorption, so its operations were
not a lattice at all, and the second failed De Morgan, so its negation did not
dualise them. Both are now the meet and join of the truth order

    F < N < T,    F < B < T,    N ∥ B

for which N ∧ B = F and N ∨ B = T, and every law holds. The laws are proved in
`Belnap.lean`; this file checks that the copies still coincide.
-/

import Imscribing.Paraconsistent.Belnap
import Imscribing.Paraconsistent.BelnapSplitFuse

namespace Imscribing.Paraconsistent.Agreement

open Imscribing.Paraconsistent

/-- The constructor-order identification between the two copies in p4ramill. -/
def tr : Imscribing.Paraconsistent.Belnap → _root_.Belnap
  | .N => .N
  | .T => .T
  | .F => .F
  | .B => .B

theorem tr_injective : Function.Injective tr := by
  intro a b h; cases a <;> cases b <;> simp_all [tr]

theorem tr_surjective : Function.Surjective tr := by
  intro b; cases b
  · exact ⟨.N, rfl⟩
  · exact ⟨.T, rfl⟩
  · exact ⟨.F, rfl⟩
  · exact ⟨.B, rfl⟩

/-- Negation agrees on all four values. -/
theorem bnot_agree (a : Imscribing.Paraconsistent.Belnap) :
    tr (bnot a) = _root_.Belnap.bnot (tr a) := by
  cases a <;> rfl

/-- Conjunction agrees on all sixteen pairs — including {N, B}, where the two
    copies once differed. -/
theorem band_agree (a b : Imscribing.Paraconsistent.Belnap) :
    tr (band a b) = _root_.Belnap.band (tr a) (tr b) := by
  cases a <;> cases b <;> rfl

/-- Disjunction agrees on all sixteen pairs. -/
theorem bor_agree (a b : Imscribing.Paraconsistent.Belnap) :
    tr (bor a b) = _root_.Belnap.bor (tr a) (tr b) := by
  cases a <;> cases b <;> rfl

/-- The identification is an isomorphism of the whole structure: it is a
    bijection commuting with all three operations. Any future edit to one copy
    alone breaks this file. -/
theorem copies_isomorphic :
    (Function.Injective tr ∧ Function.Surjective tr) ∧
    (∀ a, tr (bnot a) = _root_.Belnap.bnot (tr a)) ∧
    (∀ a b, tr (band a b) = _root_.Belnap.band (tr a) (tr b)) ∧
    (∀ a b, tr (bor a b) = _root_.Belnap.bor (tr a) (tr b)) :=
  ⟨⟨tr_injective, tr_surjective⟩, bnot_agree, band_agree, bor_agree⟩

/-- The cell that was wrong, now pinned in both copies at once: the created
    pair are lattice complements, meeting at the bottom and joining at the top,
    while negation fixes each of them. -/
theorem created_pair_are_complements :
    band (.N : Imscribing.Paraconsistent.Belnap) .B = .F ∧
    bor (.N : Imscribing.Paraconsistent.Belnap) .B = .T ∧
    _root_.Belnap.band (.N : _root_.Belnap) .B = .F ∧
    _root_.Belnap.bor (.N : _root_.Belnap) .B = .T ∧
    bnot (.N : Imscribing.Paraconsistent.Belnap) = .N ∧
    bnot (.B : Imscribing.Paraconsistent.Belnap) = .B :=
  ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

end Imscribing.Paraconsistent.Agreement
