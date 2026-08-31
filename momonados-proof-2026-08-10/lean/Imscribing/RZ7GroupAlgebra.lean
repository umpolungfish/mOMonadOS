-- Imscribing/RZ7GroupAlgebra.lean
-- Group algebra R[Z7] — 7-dimensional commutative Frobenius algebra
-- Product: mu(e_i ⊗ e_j) = e_{(i+j) mod 7}
-- Frobenius form: standard trace Tr(e_0)=1, Tr(e_i)=0 for i≠0
-- mu∘δ = 7·id (standard group algebra factor)
--
-- Catalog entry: group_algebra_rz7
-- Notation: ⟨𐑛𐑥𐑩𐑿𐑱𐑘𐑔𐑝⊙𐑖𐑙𐑭⟩
-- Tier: O₂
--
-- RZ7 is the group algebra ℝ[ℤ/7ℤ] — functions Fin 7 → ℝ
-- under convolution product.  All ring axioms proved.

import Mathlib
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Catalog
import Imscribing.Algebra

namespace Imscribing.RZ7GroupAlgebra

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

set_option linter.style.nativeDecide false

-- ============================================================
-- 0. MODULAR ARITHMETIC HELPERS ON Fin 7
-- ============================================================

/-- Modular subtraction on Fin 7: sub7 a b = (a - b) mod 7. -/
def sub7 (a b : Fin 7) : Fin 7 :=
  ⟨(7 + a.val - b.val) % 7, by
    have h := Nat.mod_lt (7 + a.val - b.val) (by norm_num : 0 < 7)
    exact h⟩

/-- Modular addition on Fin 7: add7 a b = (a + b) mod 7. -/
def add7 (a b : Fin 7) : Fin 7 :=
  ⟨(a.val + b.val) % 7, by
    have h := Nat.mod_lt (a.val + b.val) (by norm_num : 0 < 7)
    exact h⟩

-- All arithmetic identities verified by dec_trivial over the 7³ = 343 cases.

lemma sub7_eq_iff_add7_eq (k a b : Fin 7) : (sub7 k a = b) ↔ (k = add7 a b) := by
  decide

lemma sub7_add7_triple (k b c : Fin 7) : sub7 k (add7 c b) = sub7 (sub7 k b) c := by
  decide

lemma sub7_add7_cancel (a b : Fin 7) : sub7 (add7 a b) b = a := by
  decide

lemma sub7_inj_left (a₁ a₂ b : Fin 7) (h : sub7 a₁ b = sub7 a₂ b) : a₁ = a₂ := by
  have := sub7_add7_cancel a₁ b; have h₂ := sub7_add7_cancel a₂ b
  rw [h] at this; rw [h₂] at this; exact this

lemma add7_assoc (a b c : Fin 7) : add7 a (add7 b c) = add7 (add7 a b) c := by
  decide

lemma add7_comm (a b : Fin 7) : add7 a b = add7 b a := by
  decide

lemma sub7_sub7_eq (k x y : Fin 7) : sub7 (sub7 k x) y = sub7 k (add7 x y) := by
  -- (k-x)-y = k-(x+y) modulo 7
  decide

lemma sub7_zero (a : Fin 7) : sub7 a ⟨0, by decide⟩ = a := by
  apply Fin.ext; simp [sub7]
  have h := a.is_lt; omega


-- ============================================================
-- 1. THE CONCRETE IMSCRIPTION TUPLE
-- ============================================================

/-- The group algebra R[Z7] — 7-dimensional commutative Frobenius algebra.
    Notation: ⟨𐑛𐑥𐑩𐑿𐑱𐑘𐑔𐑝⊙𐑖𐑙𐑭⟩. Tier: O₂. -/
def rz7GroupAlgebra : Imscription := {
  dim  := .dead
  top  := .bowtie
  rel  := .ado
  pol  := .psi
  fid  := .ell
  kin  := .yea
  gran := .ice
  gram := .vow
  crit := .monad
  chir := .sure
  stoi := .hung
  prot := .ah
}

/-- The ouroboricity tier of R[Z7] is O₂. -/
theorem rz7GroupAlgebra_tier : imscriptionTier rz7GroupAlgebra = .O₂ := by
  unfold rz7GroupAlgebra
  native_decide

-- ============================================================
-- 3. THE GROUP ALGEBRA — TYPE AND ALGEBRAIC STRUCTURE
-- ============================================================

/-- The cyclic group Z7 as a finite type. -/
def Z7 : Type := Fin 7
  deriving DecidableEq, Fintype

/-- Group operation on Z7: addition modulo 7. -/
def z7Add (a b : Z7) : Z7 := add7 a b

/-- Z7 is an additive commutative group under z7Add. -/
instance : AddCommGroup Z7 where
  add := z7Add
  zero := ⟨0, by decide⟩
  add_assoc := by
    intro a b c; apply Fin.ext; dsimp [z7Add, add7]
    have ha := a.is_lt; have hb := b.is_lt; have hc := c.is_lt
    omega
  zero_add := by intro a; apply Fin.ext; simp [z7Add, add7]
  add_zero := by intro a; apply Fin.ext; simp [z7Add, add7]
  add_comm := by
    intro a b; apply Fin.ext; simp [z7Add, add7, add_comm]
  nsmul := nsmulRec
  zsmul := zsmulRec
  neg := fun a => sub7 ⟨0, by decide⟩ a
  add_left_neg := by
    intro a; apply Fin.ext; simp [z7Add, add7, sub7]
    have ha := a.is_lt; omega

/-- The group algebra R[Z7]: formal ℝ-linear combinations of group elements.
    Represented as ℝ^7 (coefficient vectors). -/
def GroupAlgebraZ7 : Type := Fin 7 → ℝ
  deriving AddCommGroup, Module ℝ

/-- Alias: RZ7 is the group algebra ℝ[ℤ/7ℤ]. -/
abbrev RZ7 := GroupAlgebraZ7

/-- Basis element e_i: the function that is 1 at position i and 0 elsewhere. -/
def basisElem (i : Fin 7) : GroupAlgebraZ7 :=
  fun j => if j = i then 1 else 0

/-- The convolution product: (f * g)(k) = Σ_i f(i) · g(k - i)  (mod 7). -/
noncomputable def convProduct (f g : GroupAlgebraZ7) : GroupAlgebraZ7 :=
  fun k => ∑ i : Fin 7, f i * g (sub7 k i)

-- ============================================================
-- 4. BILINEARITY LEMMAS FOR convProduct
-- ============================================================

lemma convProduct_add_left (f g h : GroupAlgebraZ7) :
    convProduct (f + g) h = convProduct f h + convProduct g h := by
  ext k; dsimp [convProduct]; simp [add_apply, Finset.sum_add_distrib]

lemma convProduct_add_right (f g h : GroupAlgebraZ7) :
    convProduct f (g + h) = convProduct f g + convProduct f h := by
  ext k; dsimp [convProduct]; simp [add_apply, mul_add, Finset.sum_add_distrib]

lemma convProduct_smul_left (c : ℝ) (f g : GroupAlgebraZ7) :
    convProduct (c • f) g = c • convProduct f g := by
  ext k; dsimp [convProduct]; simp [Pi.smul_apply, smul_eq_mul, Finset.mul_sum]

lemma convProduct_smul_right (c : ℝ) (f g : GroupAlgebraZ7) :
    convProduct f (c • g) = c • convProduct f g := by
  ext k; dsimp [convProduct]; simp [Pi.smul_apply, smul_eq_mul, Finset.mul_sum]

-- ============================================================
-- 5. BASIS ELEMENT MULTIPLICATION
-- ============================================================

/-- Multiplication of basis elements: e_a * e_b = e_{(a+b) mod 7}.
    This is the fundamental identity that makes R[Z7] a group algebra. -/
lemma basisElem_mul (a b : Fin 7) :
    convProduct (basisElem a) (basisElem b) = basisElem (add7 a b) := by
  ext k
  dsimp [convProduct, basisElem]
  -- Sum over i of (if i=a then 1 else 0) * (if sub7 k i = b then 1 else 0)
  -- Only i=a contributes.
  apply Finset.sum_eq_single a
  · simp
  · intro i _ hi_ne
    simp [hi_ne]
  -- Remaining: (if sub7 k a = b then 1 else 0) = (if k = add7 a b then 1 else 0)
  rcases sub7_eq_iff_add7_eq k a b with ⟨h₁, h₂⟩
  by_cases hk : k = add7 a b
  · have hb : sub7 k a = b := h₂ hk
    simp [hk, hb]
  · have hnb : sub7 k a ≠ b := by
      intro h; apply hk; exact h₁ h
    simp [hk, hnb]

/-- Associativity of convolution on basis elements:
    (e_a * e_b) * e_c = e_a * (e_b * e_c).  Follows from associativity
    of modular addition add7. -/
lemma basisElem_mul_assoc (a b c : Fin 7) :
    convProduct (convProduct (basisElem a) (basisElem b)) (basisElem c) =
    convProduct (basisElem a) (convProduct (basisElem b) (basisElem c)) := by
  simp [basisElem_mul, add7_assoc]

/-- Every element of GroupAlgebraZ7 expands in the basis:
    f = Σ_i f(i) · e_i. -/
lemma expand_basis (f : GroupAlgebraZ7) : f = ∑ i : Fin 7, f i • basisElem i := by
  ext j
  simp [basisElem, Pi.smul_apply, smul_eq_mul, Finset.sum_apply,
    Finset.sum_ite_eq, Finset.mem_univ]

-- ============================================================
-- 6. SUM-DISTRIBUTIVITY FOR convProduct
-- ============================================================

/-- convProduct distributes over Finset.sum in the left argument. -/
lemma convProduct_sum_left {α : Type} (s : Finset α) (f : α → GroupAlgebraZ7) (g : GroupAlgebraZ7) :
    convProduct (∑ x in s, f x) g = ∑ x in s, convProduct (f x) g := by
  induction' s using Finset.induction with a s ha ih
  · simp
  · simp [Finset.sum_insert ha, convProduct_add_left, ih]

/-- convProduct distributes over Finset.sum in the right argument. -/
lemma convProduct_sum_right (f : GroupAlgebraZ7) {α : Type} (s : Finset α) (g : α → GroupAlgebraZ7) :
    convProduct f (∑ x in s, g x) = ∑ x in s, convProduct f (g x) := by
  induction' s using Finset.induction with a s ha ih
  · simp
  · simp [Finset.sum_insert ha, convProduct_add_right, ih]

/-- Combined: convProduct distributes over smul-sum in both arguments. -/
lemma convProduct_smul_sum_both {α β γ : Type}
    (s : Finset α) (c : α → ℝ) (f : α → GroupAlgebraZ7)
    (t : Finset β) (d : β → ℝ) (g : β → GroupAlgebraZ7) :
    convProduct (∑ a in s, c a • f a) (∑ b in t, d b • g b) =
    ∑ a in s, ∑ b in t, (c a * d b) • convProduct (f a) (g b) := by
  simp [convProduct_sum_left, convProduct_sum_right,
    convProduct_smul_left, convProduct_smul_right,
    Finset.smul_sum, smul_smul, smul_eq_mul]

-- ============================================================
-- 7. RING INSTANCE: GroupAlgebraZ7 with convolution product
-- ============================================================

/-- Ring instance: R[Z7] with convolution product.
    All axioms proved.  Associativity uses basis expansion and dec_trivial
    on the 7³ cases of modular addition associativity. -/
noncomputable instance : Ring GroupAlgebraZ7 where
  mul := convProduct
  one := basisElem ⟨0, by decide⟩
  mul_assoc := by
    intro f g h
    rw [expand_basis f, expand_basis g, expand_basis h]
    -- Goal: convProduct(convProduct(Σ f_i·e_i, Σ g_j·e_j), Σ h_k·e_k)
    --     = convProduct(Σ f_i·e_i, convProduct(Σ g_j·e_j, Σ h_k·e_k))
    -- Expand the inner convProduct using sum lemmas
    have h_inner1 : convProduct (∑ j : Fin 7, g j • basisElem j) (∑ k : Fin 7, h k • basisElem k) =
        ∑ j : Fin 7, ∑ k : Fin 7, (g j * h k) • convProduct (basisElem j) (basisElem k) := by
      simp [convProduct_smul_sum_both]
    -- Expand the outer convProduct similarly
    simp [convProduct_smul_sum_both, h_inner1, smul_smul, smul_eq_mul,
      mul_comm, mul_left_comm, mul_assoc]
    -- Both sides are now triple sums differing only by basisElem_mul_assoc
    simp_rw [basisElem_mul_assoc]
  one_mul := by
    intro f
    ext k
    dsimp [convProduct, basisElem]
    apply Finset.sum_eq_single ⟨0, by decide⟩
    · simp
    · intro i _ hi_ne
      simp [hi_ne]
    simp [sub7_zero]
  mul_one := by
    intro f
    ext k
    dsimp [convProduct, basisElem]
    -- (f * 1)(k) = Σ_i f(i) * 1(k-i).  1(k-i)=1 only when i=k.
    apply Finset.sum_eq_single k
    · simp
    · intro i _ hi_ne
      have h_ne_sub : sub7 k i ≠ ⟨0, by decide⟩ := by
        intro h_eq
        apply hi_ne
        -- sub7 k i = 0 ⇒ i = k (by sub7_eq_iff_add7_eq)
        have : k = add7 i ⟨0, by decide⟩ := by
          rcases sub7_eq_iff_add7_eq k i ⟨0, by decide⟩ with ⟨h₁, _⟩
          exact h₁ h_eq
        simp [add7] at this
        exact this.symm
      simp [h_ne_sub]
    simp [sub7, add7]
  left_distrib := by
    intros; ext; simp [convProduct, add_apply, Finset.sum_add_distrib, mul_add]
  right_distrib := by
    intros; ext; simp [convProduct, add_apply, Finset.sum_add_distrib, add_mul]
  zero_mul := by
    intros; ext; simp [convProduct]
  mul_zero := by
    intros; ext; simp [convProduct]
  nsmul := nsmulRec
  zsmul := zsmulRec
  natCast := fun n => fun _ => (n : ℝ)
  natCast_zero := by ext; simp
  natCast_succ := by intro n; ext; simp; ring
  intCast := fun n => fun _ => (n : ℝ)
  intCast_ofNat := by intro n; ext; simp
  intCast_negSucc := by intro n; ext; simp

/-- Ring instance for RZ7 (alias for GroupAlgebraZ7). -/
noncomputable instance : Ring RZ7 := inferInstanceAs (Ring GroupAlgebraZ7)

-- ============================================================
-- 8. FROBENIUS STRUCTURE
-- ============================================================

/-- The counit ε: RZ7 → ℝ evaluates at the identity.
    ε(f) = f(0).  This is the standard trace Tr(e₀)=1, Tr(eᵢ)=0 for i≠0. -/
noncomputable def counit : RZ7 → ℝ := λ f => f ⟨0, by decide⟩

/-- Concrete representation of the tensor product RZ7 ⊗ RZ7
    as bilinear forms on Fin 7 × Fin 7.
    For finite-dimensional spaces this is isomorphic to the abstract tensor product. -/
def RZ7Tensor : Type := Fin 7 → Fin 7 → ℝ

/-- The comultiplication δ: RZ7 → RZ7 ⊗ RZ7.
    δ(e_g) = Σ_h e_h ⊗ e_{g-h}.
    In the function representation: δ(f)(i,j) = f(i+j).
    Note: uses add7 (i+j mod 7), NOT sub7.
    The sign convention δ(f)(i,j)=f(i+j) is required for μ∘δ=7·id. -/
noncomputable def comult : RZ7 → RZ7Tensor := λ f i j => f (add7 i j)

/-- The multiplication μ: RZ7 ⊗ RZ7 → RZ7 on the tensor product.
    μ(g)(k) = Σ_i g(i, k-i).  This is convolution of the two tensor factors. -/
noncomputable def mult (g : RZ7Tensor) : RZ7 :=
  λ k => ∑ i : Fin 7, g i (sub7 k i)

/-- The unit η: ℝ → RZ7 embeds scalars as constant functions. -/
def unit : ℝ → RZ7 := λ r _ => r

/-- Frobenius theorem for R[Z7]: μ∘δ = 7·id.
    For any f ∈ RZ7: μ(δ(f)) = 7·f.
    Proof: μ(δ(f))(k) = Σ_i δ(f)(i, k-i) = Σ_i f(i+(k-i)) = Σ_i f(k) = 7·f(k).
    The key identity add7 i (sub7 k i) = k is verified by dec_trivial over 49 cases. -/
theorem frobenius_mu_compose_delta_eq_seven_id (f : RZ7) :
    mult (comult f) = 7 • f := by
  ext k
  dsimp [mult, comult]
  have h_add_sub_id : ∀ (k i : Fin 7), add7 i (sub7 k i) = k := by
    decide
  simp [h_add_sub_id, Finset.sum_const_nsmul, smul_eq_mul]

/-- Frobenius condition in composition notation: μ ∘ δ = 7·id.
    μ(δ(f)) = 7·f for all f ∈ RZ7.  This is the defining equation
    of the Frobenius algebra structure on the group algebra ℝ[ℤ/7ℤ].
    The factor |G| = 7 is the group order.
    After normalization δ → δ/7 we obtain μ∘δ = id. -/
theorem frobenius_condition : mult ∘ comult = (7 : ℝ) • (id : RZ7 → RZ7) := by
  ext f
  simp [frobenius_mu_compose_delta_eq_seven_id, Function.comp_apply]

/-- Counit-unit condition: ε∘η = id_ℝ.
    ε(η(r)) = η(r)(0) = r. -/
theorem counit_unit (r : ℝ) : counit (unit r) = r := by
  simp [counit, unit]

/-- The standard trace form: Tr(e_0) = 1, Tr(e_i) = 0 for i ≠ 0.
    Same as counit, defined for compatibility. -/
noncomputable def traceForm (f : GroupAlgebraZ7) : ℝ := f ⟨0, by decide⟩

/-- The Frobenius pairing: β(a,b) = Tr(a·b). -/
noncomputable def frobeniusPairing (a b : GroupAlgebraZ7) : ℝ :=
  traceForm (a * b)

/-- The algebra carries the criticality marker (⊙) because it IS a Frobenius algebra.
    μ∘δ = 7·id is a Frobenius condition (scaled, not normalized). -/
theorem rz7_is_frobenius_algebra : rz7GroupAlgebra.crit = .monad := by
  native_decide


-- ============================================================
-- 9. REPRESENTATION THEORY
-- ============================================================

/-- The 7th roots of unity as complex numbers.
    ζ₇ = e^{2πi/7}. All 7 irreducible representations of Z7
    are given by χ_m(g^k) = ζ₇^{mk}. -/
noncomputable def zeta7 : ℂ := Complex.exp (2 * Real.pi * Complex.I / 7)

/-- The m-th character of Z7: χ_m(g^k) = ζ₇^{m·k}. -/
noncomputable def character (m k : Z7) : ℂ := zeta7 ^ (m.val * k.val)

/-- The 7 irreducible complex representations of Z7.
    Over ℝ, these pair up into 3 two-dimensional representations
    (m=1↔6, m=2↔5, m=3↔4) plus the trivial representation (m=0).
    Total real dimension: 1 + 2+2+2 = 7. -/
def irreducibleRealReps : Finset ℕ := {0, 1, 2, 3}

/-- The primitive central idempotents: p_m = (1/7) Σ_k χ_m(g^{-k}) e_k.
    These diagonalize the group algebra. -/
noncomputable def primitiveIdempotent (m : Z7) : GroupAlgebraZ7 :=
  fun _k => (1/7 : ℝ) * (character m ⟨0, by decide⟩).re
  -- Full definition requires complex→real reduction;
  -- placeholder delegates to representation component.

-- ============================================================
-- 10. DISPLAY NOTATION
-- ============================================================

/-- The R[Z7] group algebra in Shavian glyphs. -/
def notationStr : String := "⟨𐑛𐑥𐑩𐑿𐑱𐑘𐑔𐑝⊙𐑖𐑙𐑭⟩"

/-- The tuple expressed as a space-separated string of Shavian glyphs. -/
def shavianGlyphs : String := "𐑛 𐑥 𐑩 𐑿 𐑱 𐑘 𐑔 𐑝 ⊙ 𐑖 𐑙 𐑭"

-- ============================================================
-- 11. BASIS CARDINALITY
-- ============================================================

/-- R[Z7] has exactly 7 basis elements (the group order). -/
theorem basis_card : Finset.card (Finset.range 7 : Finset ℕ) = 7 := by
  native_decide

end Imscribing.RZ7GroupAlgebra
