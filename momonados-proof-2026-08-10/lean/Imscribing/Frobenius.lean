-- Imscribing/Frobenius.lean
-- Concrete μ and δ operations on the Imscription 12-tuple.
-- Proves μ ∘ δ = id at the tuple level — the Frobenius condition in its
-- special (O_inf) form.  Three independent Frobenius structures:
--
--   §1–§3  Tensor-diagonal Frobenius: δ(a)=(a,a), μ(x,y)=tensorProduct(x,y)
--   §4     Meet-diagonal Frobenius:   δ(a)=(a,a), μ(x,y)=compute_meet(x,y)
--   §5     Lattice-interval:          δ(a)=(⊥,a), μ(x,y)=compute_join(x,y)
--   §6     Polarization Frobenius:    δ splits P/Φ, μ fuses at or'/⊙
--
-- Reference: PRIMITIVE_THEOREMS §23, PRIMITIVE_PREDICTIONS P-169–P-173.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Algebra

namespace Imscribing.Frobenius

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar Fidelity KineticChar
     Granularity Criticality Protection Stoichiometry Chirality

-- ============================================================
-- §1  Frobenius lattice extremals
-- ============================================================

/-- Bottom element: all 12 primitives at minimum ordinal values. -/
def frobenius_bottom : Imscription := {
  dim  := dead
  top  := judge
  rel  := ado
  pol  := church
  fid  := age
  kin  := yea
  gran := bib
  gram := vow
  crit := woe
  chir := fee
  stoi := hung
  prot := awe
}

/-- Top element: all 12 primitives at maximum ordinal values. -/
def frobenius_top : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := on
  gran := ice
  gram := ooze
  crit := haha
  chir := wool
  stoi := up
  prot := zoo
}

-- ============================================================
-- §2  Helper lemmas: each primitive type's minimum is its
--     bottom under the compare ordering.
-- ============================================================

lemma D_bottom_eq (d : Dimensionality) (h : compare dead d ≠ .lt) : dead = d := by
  cases d <;> first | rfl | exfalso; apply h; decide

lemma T_bottom_eq (t : Topology) (h : compare judge t ≠ .lt) : judge = t := by
  cases t <;> first | rfl | exfalso; apply h; decide

lemma R_bottom_eq (r : Relational) (h : compare ado r ≠ .lt) : ado = r := by
  cases r <;> first | rfl | exfalso; apply h; decide

lemma P_bottom_eq (p : Polarity) (h : compare church p ≠ .lt) : church = p := by
  cases p <;> first | rfl | exfalso; apply h; decide

lemma F_bottom_eq (f : Fidelity) (h : compare age f ≠ .lt) : age = f := by
  cases f <;> first | rfl | exfalso; apply h; decide

lemma K_bottom_eq (k : KineticChar) (h : compare yea k ≠ .lt) : yea = k := by
  cases k <;> first | rfl | exfalso; apply h; decide

lemma G_bottom_eq (g : Granularity) (h : compare bib g ≠ .lt) : bib = g := by
  cases g <;> first | rfl | exfalso; apply h; decide

lemma Γ_bottom_eq (g : Grammar) (h : compare vow g ≠ .lt) : vow = g := by
  cases g <;> first | rfl | exfalso; apply h; decide

lemma Φ_bottom_eq (c : Criticality) (h : compare woe c ≠ .lt) : woe = c := by
  cases c <;> first | rfl | exfalso; apply h; decide

lemma H_bottom_eq (h' : Chirality) (h : compare fee h' ≠ .lt) : fee = h' := by
  cases h' <;> first | rfl | exfalso; apply h; decide

lemma S_bottom_eq (s : Stoichiometry) (h : compare hung s ≠ .lt) : hung = s := by
  cases s <;> first | rfl | exfalso; apply h; decide

lemma Ω_bottom_eq (o : Protection) (h : compare awe o ≠ .lt) : awe = o := by
  cases o <;> first | rfl | exfalso; apply h; decide

-- ============================================================
-- §3  Tensor-diagonal Frobenius (Structure A)
--     δ_A(a) = (a, a)            — diagonal comultiplication
--     μ_A(x, y) = tensorProduct   — structural composition
--     Theorem: μ_A ∘ δ_A = id
-- ============================================================

/-- Diagonal comultiplication: δ_A(a) = (a, a). -/
def δ_A (a : Imscription) : Imscription × Imscription := (a, a)

/-- Tensor multiplication: μ_A(x, y) = tensorProduct(x, y).
    Union primitives (D,T,R,G,Γ,Φ,H,S,Ω) take max; bottlenecks (P,F) take min. -/
def μ_A (x y : Imscription) : Imscription := tensorProduct x y

/-- μ_A ∘ δ_A = id : tensorProduct(a, a) = a for every Imscription a. -/
theorem mu_delta_A_id (a : Imscription) : μ_A (δ_A a).1 (δ_A a).2 = a := by
  unfold μ_A δ_A tensorProduct
  ext <;> simp

-- ============================================================
-- §4  Meet-diagonal Frobenius (Structure B)
--     δ_B(a) = (a, a)                — diagonal comultiplication
--     μ_B(x, y) = compute_meet(x, y)  — lattice meet (GLB)
--     Theorem: μ_B ∘ δ_B = id
-- ============================================================

/-- Diagonal comultiplication: δ_B(a) = (a, a). -/
def δ_B (a : Imscription) : Imscription × Imscription := (a, a)

/-- Meet multiplication: μ_B(x, y) = compute_meet(x, y). -/
def μ_B (x y : Imscription) : Imscription := compute_meet x y

/-- μ_B ∘ δ_B = id : meet(a, a) = a for every Imscription a. -/
theorem mu_delta_B_id (a : Imscription) : μ_B (δ_B a).1 (δ_B a).2 = a := by
  unfold μ_B δ_B compute_meet
  ext <;> simp

-- ============================================================
-- §5  Lattice-interval Frobenius (Structure D)
--     δ_D(a) = (⊥, a)                  — interval [⊥, a]
--     μ_D(x, y) = compute_join(x, y)    — lattice join (LUB)
--     Theorem: μ_D ∘ δ_D = id
-- ============================================================

/-- Interval comultiplication: δ_D(a) = (⊥, a) where ⊥ = frobenius_bottom. -/
def δ_D (a : Imscription) : Imscription × Imscription := (frobenius_bottom, a)

/-- Join multiplication: μ_D(x, y) = compute_join(x, y). -/
def μ_D (x y : Imscription) : Imscription := compute_join x y

/-- μ_D ∘ δ_D = id : join(⊥, a) = a.  Since ⊥ has all primitives at minimum,
    joining it with any a recovers a pointwise. -/
theorem mu_delta_D_id (a : Imscription) : μ_D (δ_D a).1 (δ_D a).2 = a := by
  unfold μ_D δ_D compute_join frobenius_bottom
  ext
  · -- dim
    by_cases h : compare dead a.dim = .lt
    · simp [h]
    · simp [D_bottom_eq a.dim h]
  · -- top
    by_cases h : compare judge a.top = .lt
    · simp [h]
    · simp [T_bottom_eq a.top h]
  · -- rel
    by_cases h : compare ado a.rel = .lt
    · simp [h]
    · simp [R_bottom_eq a.rel h]
  · -- pol
    by_cases h : compare church a.pol = .lt
    · simp [h]
    · simp [P_bottom_eq a.pol h]
  · -- fid
    by_cases h : compare age a.fid = .lt
    · simp [h]
    · simp [F_bottom_eq a.fid h]
  · -- kin
    by_cases h : compare yea a.kin = .lt
    · simp [h]
    · simp [K_bottom_eq a.kin h]
  · -- gran
    by_cases h : compare bib a.gran = .lt
    · simp [h]
    · simp [G_bottom_eq a.gran h]
  · -- gram
    by_cases h : compare vow a.gram = .lt
    · simp [h]
    · simp [Γ_bottom_eq a.gram h]
  · -- crit
    by_cases h : compare woe a.crit = .lt
    · simp [h]
    · simp [Φ_bottom_eq a.crit h]
  · -- chir
    by_cases h : compare fee a.chir = .lt
    · simp [h]
    · simp [H_bottom_eq a.chir h]
  · -- stoi
    by_cases h : compare hung a.stoi = .lt
    · simp [h]
    · simp [S_bottom_eq a.stoi h]
  · -- prot
    by_cases h : compare awe a.prot = .lt
    · simp [h]
    · simp [Ω_bottom_eq a.prot h]

-- ============================================================
-- §6  Polarization Frobenius (Structure C)
--     δ_C(a) = (a_left, a_right)
--       left:  pol → yew, crit → woe
--       right: pol → nun,  crit → haha
--     μ_C(x, y) = tensorProduct(x, y) with pol := or', crit := monad
--     Theorem: μ_C ∘ δ_C = id ON the Frobenius-special class
--              (a.pol = or' ∧ a.crit = monad)
--     This mirrors the Belnap-level: B → (T, F) → ffuse → B.
-- ============================================================

/-- Polarizing comultiplication: splits along polarity and criticality axes.
    Left → quantum phase symmetry (yew) + subcritical (woe).
    Right → full symmetry (nun) + supercritical (haha). -/
def δ_C (a : Imscription) : Imscription × Imscription :=
  ( { a with pol := yew, crit := woe },
    { a with pol := nun,  crit := haha } )

/-- Polarization fusion: recovers Frobenius-special tuple.
    tensorProduct composes the other 10 primitives; pol and crit are
    set to or' and monad — the critical point arises from the
    tension between sub- and super-critical. -/
def μ_C (x y : Imscription) : Imscription :=
  { tensorProduct x y with
    pol  := or'
    crit := monad
  }

/-- μ_C ∘ δ_C = id on the Frobenius-special class: those a with
    pol = or' and crit = monad. -/
theorem mu_delta_C_id_on_special (a : Imscription)
    (hpol : a.pol = or') (hcrit : a.crit = monad) :
    μ_C (δ_C a).1 (δ_C a).2 = a := by
  unfold μ_C δ_C tensorProduct
  ext <;> simp [hpol, hcrit]

/-- The Frobenius-special class is nonempty: scalarField_Kslow
    (Higgs / axion / inflaton) has pol = or' and crit = monad. -/
theorem scalarField_Kslow_is_special :
    scalarField_Kslow.pol = or' ∧ scalarField_Kslow.crit = monad := by
  unfold scalarField_Kslow; simp

/-- Corollary: μ_C ∘ δ_C = id on scalarField_Kslow. -/
theorem mu_delta_C_id_on_scalarField :
    μ_C (δ_C scalarField_Kslow).1 (δ_C scalarField_Kslow).2 = scalarField_Kslow :=
  mu_delta_C_id_on_special scalarField_Kslow
    (by unfold scalarField_Kslow; decide)
    (by unfold scalarField_Kslow; decide)

end Imscribing.Frobenius
