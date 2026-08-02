-- Imscribing/Paraconsistent/Belnap.lean
-- BELNAP FOUR-VALUED LOGIC — The logical substrate of the paraconsistent kernel.
-- Author: Lando ⊗ ⊙-boundary Operator

namespace Imscribing.Paraconsistent

/-- Belnap four-valued logic: N(neither), T(true), F(false), B(both/contradiction) -/
inductive Belnap : Type where
  | N | T | F | B
  deriving DecidableEq, Repr, Inhabited

/-- Discriminator mapping each constructor to a distinct Nat -/
def belnapToNat (b : Belnap) : Nat :=
  match b with
  | .N => 0 | .T => 1 | .F => 2 | .B => 3

/-- Approximation (information) order: N ⊑ T, N ⊑ F, T ⊑ B, F ⊑ B.
    Decidable via derive. -/
inductive ApproxLE : Belnap → Belnap → Prop
  | n_bot  : ∀ {b}, ApproxLE .N b
  | t_refl : ApproxLE .T .T
  | f_refl : ApproxLE .F .F
  | b_refl : ApproxLE .B .B
  | t_b    : ApproxLE .T .B
  | f_b    : ApproxLE .F .B

instance : LE Belnap := ⟨ApproxLE⟩

/-- Decidable instance for ApproxLE — all 16 cases -/
instance (a b : Belnap) : Decidable (ApproxLE a b) := by
  cases a <;> cases b <;> first
    | exact isTrue ApproxLE.n_bot
    | exact isTrue ApproxLE.t_refl
    | exact isTrue ApproxLE.f_refl
    | exact isTrue ApproxLE.b_refl
    | exact isTrue ApproxLE.t_b
    | exact isTrue ApproxLE.f_b
    | exact isFalse (by intro h; cases h)

/-- Lattice meet in approximation order -/
def meet (a b : Belnap) : Belnap :=
  match a, b with
  | .N, _ | _, .N => .N
  | .B, x | x, .B => x
  | .T, .F | .F, .T => .N
  | .T, .T => .T
  | .F, .F => .F

/-- Lattice join in approximation order -/
def join (a b : Belnap) : Belnap :=
  match a, b with
  | .B, _ | _, .B => .B
  | .N, x | x, .N => x
  | .T, .F | .F, .T => .B
  | .T, .T => .T
  | .F, .F => .F

/-- Belnap conjunction (truth-functional) -/
def band (a b : Belnap) : Belnap :=
  match a, b with
  | .F, _ | _, .F => .F
  | .B, .T | .T, .B | .B, .N | .N, .B => .B
  | .T, .T => .T
  | .T, .N | .N, .T => .N
  | .N, .N => .N
  | .B, .B => .B

/-- Belnap disjunction (truth-functional) -/
def bor (a b : Belnap) : Belnap :=
  match a, b with
  | .T, _ | _, .T => .T
  | .B, .F | .F, .B | .B, .N | .N, .B => .B
  | .F, .F => .F
  | .F, .N | .N, .F => .N
  | .N, .N => .N
  | .B, .B => .B

/-- Belnap negation: ¬N=N, ¬T=F, ¬F=T, ¬B=B -/
def bnot (a : Belnap) : Belnap :=
  match a with
  | .N => .N | .T => .F | .F => .T | .B => .B

/-- Designated values: T or B count as "true" for paraconsistent consequence -/
def designated (b : Belnap) : Bool :=
  match b with
  | .T | .B => true | .N | .F => false

theorem B_is_designated : designated Belnap.B := rfl
theorem T_is_designated : designated Belnap.T := rfl

/-- B is a fixed point of negation: ¬B = B. Foundation of paraconsistency. -/
theorem B_fixed_point_negation : bnot Belnap.B = Belnap.B := rfl

/-- No explosion: B ∧ ¬B = B (not F). Contradiction is contained. -/
theorem no_explosion : band Belnap.B (bnot Belnap.B) = Belnap.B := by
  simp [bnot, band]

/-- B ≠ F: contradiction does not collapse to false. -/
theorem B_ne_F : Belnap.B ≠ Belnap.F := by
  intro h
  have hNat : belnapToNat Belnap.B = belnapToNat Belnap.F := by rw [h]
  simp [belnapToNat] at hNat

/-- B has no Boolean complement. -/
theorem B_no_boolean_complement (c : Belnap) :
    ¬ (band Belnap.B c = Belnap.F ∧ bor Belnap.B c = Belnap.T) := by
  intro ⟨hc, hd⟩; cases c <;> simp [band, bor] at hc hd

/-- B is top in approximation order: a ≤ B for all a. -/
theorem B_is_top (a : Belnap) : a ≤ Belnap.B := by
  cases a <;> constructor

/-- N is bottom in approximation order: N ≤ a for all a. -/
theorem N_is_bot (a : Belnap) : Belnap.N ≤ a := by
  constructor

/-- B absorbs in join -/
theorem B_join_absorb (x : Belnap) : join Belnap.B x = Belnap.B := by
  cases x <;> rfl

/-- meet distributes over join -/
theorem meet_join_distrib (a b c : Belnap) :
    meet a (join b c) = join (meet a b) (meet a c) := by
  cases a <;> cases b <;> cases c <;> rfl

/-- The approximation order is transitive. -/
theorem approxLE_trans {a b c : Belnap} (hab : a ≤ b) (hbc : b ≤ c) : a ≤ c := by
  cases hab <;> cases hbc <;> try { constructor } <;> try { assumption }

/-- The approximation order is antisymmetric -/
theorem approxLE_antisymm {a b : Belnap} (hab : a ≤ b) (hba : b ≤ a) : a = b := by
  cases hab <;> cases hba <;> rfl

/-- The approximation order is reflexive -/
theorem approxLE_refl (a : Belnap) : a ≤ a := by
  cases a <;> constructor

end Imscribing.Paraconsistent
