-- Imscribing/EulerInvariant.lean
-- Euler Characteristic Invariance under Dialectic Split / Fusion
--
-- The FSPLIT and FFUSE operations on a register-flow graph each preserve
-- the Euler characteristic χ = V − E + F exactly.
--
-- FSPLIT adds one node, two edges, one face: Δχ = +1 − 2 + 1 = 0.
-- FFUSE subtracts the same: Δχ = −1 + 2 − 1 = 0.
-- Together they implement μ ∘ δ = id on the topological invariant.
--
-- Consequence: no sequence of IMASM dialetheia operations can alter
-- the Euler characteristic of the execution graph.  The grammar cannot
-- change the topology it runs on.

import Mathlib.Data.Int.Basic
import Mathlib.Logic.Function.Iterate

namespace Imscribing.EulerInvariant

-- ─────────────────────────────────────────────────────────────────────────────
-- §1  StateGraph
-- ─────────────────────────────────────────────────────────────────────────────

/-- A register-flow graph counted by its topological skeleton. -/
@[ext]
structure StateGraph where
  vertices : ℤ  -- discrete state registers
  edges    : ℤ  -- morphism pathways between registers
  faces    : ℤ  -- closed paraconsistent loops

/-- The Euler characteristic χ = V − E + F. -/
def eulerChar (g : StateGraph) : ℤ :=
  g.vertices - g.edges + g.faces

-- ─────────────────────────────────────────────────────────────────────────────
-- §2  FSPLIT / FFUSE operations
-- ─────────────────────────────────────────────────────────────────────────────

/-- FSPLIT (0x6): dialectic split.
    The incoming register forks into two differential rails;
    the reunion closes a new paradox face. -/
def dialecticSplit (g : StateGraph) : StateGraph :=
  { vertices := g.vertices + 1
    edges    := g.edges    + 2
    faces    := g.faces    + 1 }

/-- FFUSE (0x7): dialectic fusion.  The dual of FSPLIT. -/
def dialecticFuse (g : StateGraph) : StateGraph :=
  { vertices := g.vertices - 1
    edges    := g.edges    - 2
    faces    := g.faces    - 1 }

-- ─────────────────────────────────────────────────────────────────────────────
-- §3  Invariance theorems
-- ─────────────────────────────────────────────────────────────────────────────

/-- χ is preserved under FSPLIT.
    (v+1) − (e+2) + (f+1) = v − e + f  ∎ -/
theorem eulerChar_invariant_split : ∀ g : StateGraph,
    eulerChar (dialecticSplit g) = eulerChar g
  | ⟨v, e, f⟩ => by simp [eulerChar, dialecticSplit]; omega

/-- χ is preserved under FFUSE. -/
theorem eulerChar_invariant_fuse : ∀ g : StateGraph,
    eulerChar (dialecticFuse g) = eulerChar g
  | ⟨v, e, f⟩ => by simp [eulerChar, dialecticFuse]; omega

/-- The Frobenius round-trip: FFUSE ∘ FSPLIT = id on StateGraph. -/
theorem dialecticFuse_split_id : ∀ g : StateGraph,
    dialecticFuse (dialecticSplit g) = g
  | ⟨v, e, f⟩ => by simp [dialecticSplit, dialecticFuse]

/-- FSPLIT ∘ FFUSE = id (the other direction). -/
theorem dialecticSplit_fuse_id : ∀ g : StateGraph,
    dialecticSplit (dialecticFuse g) = g
  | ⟨v, e, f⟩ => by simp [dialecticSplit, dialecticFuse]

-- ─────────────────────────────────────────────────────────────────────────────
-- §4  Iterated invariance
-- ─────────────────────────────────────────────────────────────────────────────

/-- χ is preserved under any number of FSPLIT applications. -/
theorem eulerChar_invariant_nSplits (n : ℕ) (g : StateGraph) :
    eulerChar (dialecticSplit^[n] g) = eulerChar g := by
  induction n with
  | zero      => rfl
  | succ n ih =>
    rw [Function.iterate_succ', Function.comp_apply]
    rw [eulerChar_invariant_split, ih]

/-- χ is preserved under any number of FFUSE applications. -/
theorem eulerChar_invariant_nFuses (n : ℕ) (g : StateGraph) :
    eulerChar (dialecticFuse^[n] g) = eulerChar g := by
  induction n with
  | zero      => rfl
  | succ n ih =>
    rw [Function.iterate_succ', Function.comp_apply]
    rw [eulerChar_invariant_fuse, ih]

-- ─────────────────────────────────────────────────────────────────────────────
-- §5  Corollary: dialetheia opcodes cannot change graph topology
-- ─────────────────────────────────────────────────────────────────────────────

/-- Any finite sequence of FSPLITs followed by FFUSEs preserves χ. -/
theorem imasm_dialetheia_preserves_topology (n m : ℕ) (g : StateGraph) :
    eulerChar (dialecticFuse^[m] (dialecticSplit^[n] g)) = eulerChar g := by
  rw [eulerChar_invariant_nFuses, eulerChar_invariant_nSplits]

end Imscribing.EulerInvariant
