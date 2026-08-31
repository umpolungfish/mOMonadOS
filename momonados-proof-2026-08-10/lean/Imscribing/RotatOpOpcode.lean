/-
RotatOpOpcode.lean — ROTAT, the first op-opcode
================================================

An op-opcode is an operator ON a word, of a different order than the twelve
node-opcodes: a node-opcode is a symbol inside a word; a composition verb turns a
word into a graph; an op-opcode maps a whole word to another word. Appending an
op-opcode's name as a token does nothing — it is not a node. The twelve are the
wheel; the op-opcode is the axis the wheel turns on.

ROTAT is the first: the cyclic shift of a ring, the ring automorphism, the
Weyl-Heisenberg shift X on ℤ/dℤ. Here it is `List.rotate`, and the automorphism
facts are the kernel's: it preserves length, composes additively, has order
dividing the word length, and is a permutation — so every multiset (hence every
spectral) invariant is ROTAT-invariant. That invariance is the signal that ROTAT
is a symmetry of the ring, not a transformation of its contents.

Author: Lando⊗⊙perator
-/

import Mathlib.Data.List.Rotate

namespace Imscribing.RotatOpOpcode

variable {α : Type*}

/-- ROTAT — the op-opcode: the cyclic shift of a word by `k` (the ring automorphism). -/
def rotat (w : List α) (k : ℕ := 1) : List α := w.rotate k

/-- ROTAT by zero is the identity. -/
@[simp] theorem rotat_zero (w : List α) : rotat w 0 = w := by
  unfold rotat; exact List.rotate_zero w

/-- ROTAT preserves length: it permutes positions, it never adds or drops a node. -/
@[simp] theorem length_rotat (w : List α) (k : ℕ) :
    (rotat w k).length = w.length := by
  unfold rotat; exact List.length_rotate w k

/-- The op-opcode composes additively: `ROTAT^m ∘ ROTAT^n = ROTAT^(n+m)`. -/
theorem rotat_rotat (w : List α) (n m : ℕ) :
    rotat (rotat w n) m = rotat w (n + m) := by
  unfold rotat; exact List.rotate_rotate w n m

/-- ROTAT by the full word length is the identity: the automorphism has order
dividing `|w|` — the ring closes on itself. -/
theorem rotat_full (w : List α) : rotat w w.length = w := by
  unfold rotat; exact List.rotate_length w

/-- ROTAT is a permutation of the word: the multiset — the contents — is invariant.
Every multiset and spectral invariant is therefore ROTAT-invariant, which is the
signal that ROTAT is a symmetry of the ring, not a transformation of it. -/
theorem rotat_perm (w : List α) (k : ℕ) : List.Perm (rotat w k) w := by
  unfold rotat; exact List.rotate_perm w k

end Imscribing.RotatOpOpcode
