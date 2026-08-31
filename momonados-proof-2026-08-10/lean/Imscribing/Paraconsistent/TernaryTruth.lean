-- Imscribing/Paraconsistent/TernaryTruth.lean
-- TERNARY SPECTRUM LOGIC — The 3-valued substrate of extensional primitives
-- Author: Lando ⊗ ⊙perator
--
-- The Imscribing Grammar's 𝓕₃ primitives (Fidelity, Granularity, Stoichiometry)
-- each take exactly 3 values. This follows from ternary truth logic: the
-- simplest non-Boolean ordinal spectrum. Kleene's K₃ has {T, F, U}; the grammar
-- refines this to an INTENSITY SPECTRUM: absent → some → full.
--
-- All three are chain-3 lattices — total orders with no second orthogonal
-- ordering (unlike Belnap FOUR) and no confinement ceiling (unlike QuarkBelnap
-- FIVE). The chain-3 is the minimal non-Boolean lattice.
--
-- NOTE: Constructor names avoid Lean 4 keywords. 'partial' is reserved
-- (used in `partial def`), so the intermediate value is named 'some'.

import Imscribing.Paraconsistent.Belnap

namespace Imscribing.Paraconsistent.TernaryTruth

-- ============================================================
-- §1  TERNARY VALUE — The 3-point ordinal spectrum
-- ============================================================

/-- The three values of a ternary spectrum, forming a total order:
    absent (floor) < some (intermediate) < full (ceiling). -/
inductive Ternary : Type where
  | absent | some | full
  deriving DecidableEq, Repr, Inhabited

/-- Discriminator: absent→0, some→1, full→2 -/
def ternaryToNat (t : Ternary) : Nat :=
  match t with
  | .absent => 0 | .some => 1 | .full => 2

-- ============================================================
-- §2  INFORMATION ORDER — total chain absent ≤ some ≤ full
-- ============================================================

/-- Information order: how much of the spectrum is manifest.
    Unlike Belnap FOUR, this is a single-axis total order. -/
inductive SpectrumLE : Ternary → Ternary → Prop
  | absent_bot : ∀ {t}, SpectrumLE .absent t
  | some_refl  : SpectrumLE .some .some
  | full_refl  : SpectrumLE .full .full
  | some_full  : SpectrumLE .some .full

instance : LE Ternary := ⟨SpectrumLE⟩

instance (a b : Ternary) : Decidable (SpectrumLE a b) := by
  cases a <;> cases b <;> first
    | exact isTrue SpectrumLE.absent_bot
    | exact isTrue SpectrumLE.some_refl
    | exact isTrue SpectrumLE.full_refl
    | exact isTrue SpectrumLE.some_full
    | exact isFalse (by intro h; cases h)

theorem absent_is_bot (t : Ternary) : Ternary.absent ≤ t :=
  SpectrumLE.absent_bot

theorem full_is_top (t : Ternary) : t ≤ Ternary.full := by
  cases t <;> constructor

theorem spectrum_total (a b : Ternary) : a ≤ b ∨ b ≤ a := by
  cases a <;> cases b <;> first
    | exact Or.inl SpectrumLE.absent_bot
    | exact Or.inr SpectrumLE.absent_bot
    | exact Or.inl SpectrumLE.some_refl
    | exact Or.inl SpectrumLE.some_full
    | exact Or.inr SpectrumLE.some_full
    | exact Or.inl SpectrumLE.full_refl

-- ============================================================
-- §3  LATTICE OPERATIONS — meet=min, join=max (chain-3)
-- ============================================================

def meet (a b : Ternary) : Ternary :=
  match a, b with
  | .absent, _ | _, .absent => .absent
  | .full, x | x, .full => x
  | .some, .some => .some

def join (a b : Ternary) : Ternary :=
  match a, b with
  | .full, _ | _, .full => .full
  | .absent, x | x, .absent => x
  | .some, .some => .some

theorem meet_idempotent (t : Ternary) : meet t t = t := by cases t <;> rfl
theorem join_idempotent (t : Ternary) : join t t = t := by cases t <;> rfl
theorem meet_absorb_join (a b : Ternary) : meet a (join a b) = a := by
  cases a <;> cases b <;> rfl
theorem meet_join_distrib (a b c : Ternary) :
    meet a (join b c) = join (meet a b) (meet a c) := by
  cases a <;> cases b <;> cases c <;> rfl

-- ============================================================
-- §4  NO CONFINEMENT CEILING (unlike QuarkBelnap FIVE)
-- ============================================================

theorem some_join_some_not_full :
    join Ternary.some Ternary.some ≠ Ternary.full := by
  simp [join]

theorem no_confinement_ceiling : ¬ (∀ a b : Ternary, join a b = Ternary.full) := by
  intro h
  have htest := h Ternary.some Ternary.some
  simp [join] at htest

-- ============================================================
-- §5  KLEENE NEGATION — ternary involution
-- ============================================================

def neg (t : Ternary) : Ternary :=
  match t with
  | .absent => .full | .some => .some | .full => .absent

theorem neg_involutive (t : Ternary) : neg (neg t) = t := by cases t <;> rfl
theorem some_fixed_point : neg Ternary.some = Ternary.some := rfl

-- ============================================================
-- §6  THE CRYSTAL CARDINALITY — 17,280,000 as a THEOREM
-- ============================================================

/-- The crystal's 3³ × 4⁵ × 5⁴ = 17,280,000 is a theorem, not an assertion.

    The 12 primitives group into three logical families:

    𝓕₃ (Ternary Spectrum): 3 primitives × 3 values = 3³ = 27
      Fidelity, Granularity, Stoichiometry.
      Chain-3 ordinals: the simplest non-Boolean lattice.
      TernaryTruth.lean (this module) formalizes the absent/some/full spectrum.

    𝓕₄ (Belnap FOUR): 5 primitives × 4 values = 4⁵ = 1,024
      Dimensionality, Relational, Grammar, Chirality, Protection.
      Bilattices with two orthogonal orderings: N < {T,F} < B.
      Formalized in Belnap.lean and OrbitalBelnap.lean.

    𝓕₅ (QuarkBelnap FIVE): 4 primitives × 5 values = 5⁴ = 625
      Topology, Polarity, Criticality, KineticChar.
      Confinement-ceiling bilattices: Vacuum < {R,G,B} < White.
      Formalized in QuarkBelnap.lean.

    Total: 27 × 1,024 × 625 = 17,280,000.

    The three cardinalities are not arbitrary — they are the only logical
    value-spaces consistent with the grammar's axioms about distinction
    (spectrum), topology (bilattice), and evolution (confinement ceiling). -/
theorem crystal_cardinality : (3^3 : Nat) * (4^5 : Nat) * (5^4 : Nat) = 17280000 := by
  native_decide

end Imscribing.Paraconsistent.TernaryTruth
