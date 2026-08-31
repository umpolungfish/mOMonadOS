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
  | .N, .B | .B, .N => .F
  | .T, x => x
  | x, .T => x
  | .N, .N => .N
  | .B, .B => .B

/-- Belnap disjunction (truth-functional) -/
def bor (a b : Belnap) : Belnap :=
  match a, b with
  | .T, _ | _, .T => .T
  | .N, .B | .B, .N => .T
  | .F, x => x
  | x, .F => x
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

/-- **B's Boolean complement is exactly N.** In the truth lattice the created
    pair are each other's complement: their meet is bottom and their join is
    top. Nothing else complements B. -/
theorem B_complement_iff (c : Belnap) :
    (band Belnap.B c = Belnap.F ∧ bor Belnap.B c = Belnap.T) ↔ c = Belnap.N := by
  cases c <;> simp [band, bor]

/-- The complement is **not** the negation. Negation fixes B and N; the lattice
    complement exchanges them. The two involutions on the created pair are
    different maps, which is why a contradiction is not a falsehood. -/
theorem complement_ne_negation :
    bnot Belnap.B = Belnap.B ∧ bnot Belnap.N = Belnap.N ∧
    (band Belnap.B Belnap.N = Belnap.F ∧ bor Belnap.B Belnap.N = Belnap.T) :=
  ⟨rfl, rfl, rfl, rfl⟩

/-! ### The laws that fix the table

`band` and `bor` are the meet and join of the truth order F < N < T, F < B < T,
in which N and B are incomparable. The four laws below pin that table down: a
pair of binary operations satisfying commutativity, associativity, idempotence
and absorption *is* a lattice, and De Morgan says the negation dualises it.
Together they leave no freedom at the incomparable pair — which is exactly where
this file and the kernel prelude once disagreed, each in a way that broke one of
these laws. -/

theorem band_comm (a b : Belnap) : band a b = band b a := by cases a <;> cases b <;> rfl
theorem bor_comm (a b : Belnap) : bor a b = bor b a := by cases a <;> cases b <;> rfl

theorem band_assoc (a b c : Belnap) : band (band a b) c = band a (band b c) := by
  cases a <;> cases b <;> cases c <;> rfl
theorem bor_assoc (a b c : Belnap) : bor (bor a b) c = bor a (bor b c) := by
  cases a <;> cases b <;> cases c <;> rfl

theorem band_idem (a : Belnap) : band a a = a := by cases a <;> rfl
theorem bor_idem (a : Belnap) : bor a a = a := by cases a <;> rfl

/-- Absorption — the law that makes the pair a lattice rather than merely two
    commutative associative operations. -/
theorem band_absorb (a b : Belnap) : band a (bor a b) = a := by
  cases a <;> cases b <;> rfl
theorem bor_absorb (a b : Belnap) : bor a (band a b) = a := by
  cases a <;> cases b <;> rfl

/-- De Morgan — the law that makes negation the lattice's dualising involution. -/
theorem de_morgan_and (a b : Belnap) : bnot (band a b) = bor (bnot a) (bnot b) := by
  cases a <;> cases b <;> rfl
theorem de_morgan_or (a b : Belnap) : bnot (bor a b) = band (bnot a) (bnot b) := by
  cases a <;> cases b <;> rfl

/-- The lattice is distributive. -/
theorem band_distrib_bor (a b c : Belnap) :
    band a (bor b c) = bor (band a b) (band a c) := by
  cases a <;> cases b <;> cases c <;> rfl
theorem bor_distrib_band (a b c : Belnap) :
    bor a (band b c) = band (bor a b) (bor a c) := by
  cases a <;> cases b <;> cases c <;> rfl

/-- F is the bottom of the truth order and T the top. -/
theorem band_F (a : Belnap) : band a Belnap.F = Belnap.F := by cases a <;> rfl
theorem bor_T (a : Belnap) : bor a Belnap.T = Belnap.T := by cases a <;> rfl
theorem band_T (a : Belnap) : band a Belnap.T = a := by cases a <;> rfl
theorem bor_F (a : Belnap) : bor a Belnap.F = a := by cases a <;> rfl

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
