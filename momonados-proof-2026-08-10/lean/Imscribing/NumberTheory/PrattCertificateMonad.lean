/-
Imscribing/NumberTheory/PrattCertificateMonad.lean
Pratt Certificate Monad in Lean 4 — buildable core.
Catalog: ⟨Ð=𐑼; Þ=𐑡; Ř=𐑑; Φ=𐑗; ƒ=𐑞; Ç=𐑧; Γ=𐑲; ɢ=𐑠; ⊙=⊙; Ħ=𐑫; Σ=𐑳; Ω=𐑭⟩
Ob3ect:  ⟨Ð=𐑦; Þ=𐑰; Ř=𐑽; Φ=𐑬; ƒ=𐑐; Ç=𐑧; Γ=𐑔; ɢ=𐑜; ⊙=⊙; Ħ=𐑫; Σ=𐑳; Ω=𐑭⟩
-/

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.IGFunctor
import Mathlib

namespace Imscribing.NumberTheory.PrattCertificateMonad
open Imscribing.Primitives
open Imscribing
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ============================================================
-- §1. PRATT CERTIFICATE — TYPE DEFINITIONS
-- ============================================================

inductive PrattTree : ℕ → Type where
  | leaf : PrattTree 2
  | node (p : ℕ) (hp : Nat.Prime p) (hpg2 : p > 2) :
      (factors : List ℕ) →
      (hfactors_prod : factors.prod = p - 1) →
      (hfactors_prime : ∀ r ∈ factors, Nat.Prime r) →
      (children : ∀ (r : ℕ), r ∈ factors → PrattTree r) →
      PrattTree p

structure PrattCertificate (p : ℕ) where
  tree : PrattTree p
  generators : List (ℕ × ℕ)

axiom pratt_theorem (p : ℕ) (hp : Nat.Prime p) : Nonempty (PrattCertificate p)

-- ============================================================
-- §2. DEPTH (CLASS NUMBER)
-- ============================================================

def listMax : List ℕ → ℕ
  | [] => 0
  | x :: xs => List.foldl (fun a b => max a b) x xs

def PrattTree.depth {p : ℕ} : PrattTree p → ℕ
  | .leaf => 1
  | .node _ _ _ factors _ _ children =>
    1 + listMax ((factors.attach).map (fun ⟨r, hr⟩ => (children r hr).depth))

-- ============================================================
-- §3. CLASS HIERARCHY
-- ============================================================

abbrev Prime : Type := { p : ℕ // Nat.Prime p }

noncomputable def classNumber (p : Prime) : ℕ :=
  (Classical.choice (pratt_theorem p.val p.property)).tree.depth

def classHierarchy (r : ℕ) : Set Prime := {p | classNumber p = r}

theorem classHierarchy_cover : (⋃ (r : ℕ), classHierarchy r) = Set.univ := by
  ext p; simp [classHierarchy]

axiom classOne_is_two (p : Prime) : classNumber p = 1 ↔ p.val = 2

-- ============================================================
-- §4. IMSCRIPTIVE EMBEDDING
-- ============================================================

def prattCertificateMonad_imscription : Imscription :=
  { dim   := Dimensionality.array
  , top   := Topology.judge
  , rel   := Relational.tot
  , pol   := Polarity.church
  , fid   := Fidelity.they
  , kin   := KineticChar.egg
  , gran  := Granularity.bib
  , gram  := Grammar.measure
  , crit  := Criticality.monad
  , chir  := Chirality.wool
  , stoi  := Stoichiometry.up
  , prot  := Protection.ah
  }

def prattCertificateMonad_ob3ect_imscription : Imscription :=
  { dim   := Dimensionality.if'
  , top   := Topology.eat
  , rel   := Relational.ear
  , pol   := Polarity.out
  , fid   := Fidelity.peep
  , kin   := KineticChar.egg
  , gran  := Granularity.ice
  , gram  := Grammar.gag
  , crit  := Criticality.monad
  , chir  := Chirality.wool
  , stoi  := Stoichiometry.up
  , prot  := Protection.ah
  }

theorem catalog_vs_ob3ect_differ :
    prattCertificateMonad_imscription ≠ prattCertificateMonad_ob3ect_imscription := by
  intro h
  have hdim := congrArg Imscription.dim h
  simp [prattCertificateMonad_imscription, prattCertificateMonad_ob3ect_imscription] at hdim

theorem catalog_ob3ect_agree_on_core :
    prattCertificateMonad_imscription.crit = prattCertificateMonad_ob3ect_imscription.crit ∧
    prattCertificateMonad_imscription.chir = prattCertificateMonad_ob3ect_imscription.chir ∧
    prattCertificateMonad_imscription.stoi = prattCertificateMonad_ob3ect_imscription.stoi ∧
    prattCertificateMonad_imscription.prot = prattCertificateMonad_ob3ect_imscription.prot ∧
    prattCertificateMonad_imscription.kin = prattCertificateMonad_ob3ect_imscription.kin := by
  simp [prattCertificateMonad_imscription, prattCertificateMonad_ob3ect_imscription]

-- ============================================================
-- §5. TIER THEOREMS
-- ============================================================

theorem prattCertificateMonad_tier :
    imscriptionTier prattCertificateMonad_imscription = .O₂dag := by
  unfold imscriptionTier prattCertificateMonad_imscription
  decide

theorem prattCertificateMonad_ob3ect_tier :
    imscriptionTier prattCertificateMonad_ob3ect_imscription = .O₂ := by
  unfold imscriptionTier prattCertificateMonad_ob3ect_imscription
  decide

-- ============================================================
-- §6. AT CRITICALITY
-- ============================================================

theorem prattCertificateMonad_at_criticality :
    prattCertificateMonad_imscription.crit = Criticality.monad := rfl

-- ============================================================
-- §7. DISTANCE TO GOLDBACH
-- ============================================================

def goldbach_imscription : Imscription :=
  { dim := Dimensionality.array
  , top := Topology.are
  , rel := Relational.ian
  , pol := Polarity.or'
  , fid := Fidelity.peep
  , kin := KineticChar.egg
  , gran := Granularity.bib
  , gram := Grammar.measure
  , crit := Criticality.monad
  , chir := Chirality.sure
  , stoi := Stoichiometry.hung
  , prot := Protection.ah
  }

theorem pratt_goldbach_mismatches :
    primitiveMismatches prattCertificateMonad_imscription goldbach_imscription = 6 := by
  unfold prattCertificateMonad_imscription goldbach_imscription primitiveMismatches
  decide

theorem pratt_ob3ect_goldbach_mismatches :
    primitiveMismatches prattCertificateMonad_ob3ect_imscription goldbach_imscription = 8 := by
  unfold prattCertificateMonad_ob3ect_imscription goldbach_imscription primitiveMismatches
  decide

-- ============================================================
-- §8. INVARIANT CORE
-- ============================================================

theorem pratt_invariant_core :
    prattCertificateMonad_imscription.crit = Criticality.monad ∧
    prattCertificateMonad_imscription.chir = Chirality.wool ∧
    prattCertificateMonad_imscription.stoi = Stoichiometry.up ∧
    prattCertificateMonad_imscription.prot = Protection.ah ∧
    prattCertificateMonad_imscription.kin = KineticChar.egg := by
  simp [prattCertificateMonad_imscription]

-- ============================================================
-- §9. NEW OB3ECT TUPLE (--entry prime_class_hierarchy_pratt, laguna-s-2.1)
-- ============================================================

/-- The ob3ect tuple generated by auto.py --entry prime_class_hierarchy_pratt
    using poolside/laguna-s-2.1. Differs from the original ob3ect tuple only
    in chirality: Ħ=𐑖 (two-step) vs Ħ=𐑫 (eternal). -/
def pratt_entry_ob3ect_imscription : Imscription :=
  { dim   := Dimensionality.if'
  , top   := Topology.eat
  , rel   := Relational.ear
  , pol   := Polarity.out
  , fid   := Fidelity.peep
  , kin   := KineticChar.egg
  , gran  := Granularity.ice
  , gram  := Grammar.gag
  , crit  := Criticality.monad
  , chir  := Chirality.sure
  , stoi  := Stoichiometry.up
  , prot  := Protection.ah
  }

/-- The new ob3ect differs from the original ob3ect ONLY in chirality -/
theorem entry_ob3ect_differs_from_original_ob3ect :
    pratt_entry_ob3ect_imscription ≠ prattCertificateMonad_ob3ect_imscription := by
  intro h
  have hchir := congrArg Imscription.chir h
  simp [pratt_entry_ob3ect_imscription, prattCertificateMonad_ob3ect_imscription] at hchir

/-- Yet both ob3ects and the catalog share the invariant core (crit, stoi, prot, kin) -/
theorem entry_ob3ect_invariant_core_matches :
    pratt_entry_ob3ect_imscription.crit = prattCertificateMonad_imscription.crit ∧
    pratt_entry_ob3ect_imscription.stoi = prattCertificateMonad_imscription.stoi ∧
    pratt_entry_ob3ect_imscription.prot = prattCertificateMonad_imscription.prot ∧
    pratt_entry_ob3ect_imscription.kin  = prattCertificateMonad_imscription.kin := by
  simp [pratt_entry_ob3ect_imscription, prattCertificateMonad_imscription]

/-- The new ob3ect differs from the catalog on exactly 8 primitives -/
theorem entry_ob3ect_catalog_distance :
    primitiveMismatches pratt_entry_ob3ect_imscription prattCertificateMonad_imscription = 8 := by
  unfold pratt_entry_ob3ect_imscription prattCertificateMonad_imscription primitiveMismatches
  decide

/-- The new ob3ect differs from the original ob3ect on exactly 1 primitive (chir) -/
theorem entry_ob3ect_vs_original_distance :
    primitiveMismatches pratt_entry_ob3ect_imscription prattCertificateMonad_ob3ect_imscription = 1 := by
  unfold pratt_entry_ob3ect_imscription prattCertificateMonad_ob3ect_imscription primitiveMismatches
  decide

-- ============================================================
-- §10. OB3ECT PROTOCOL INTEGRATION
-- ============================================================

/-- Invariant core is preserved under the IGProtocol Frobenius self-fusion -/
theorem invariant_core_frobenius_stable (a : Imscription) :
    igFrobeniusAlg.mul a a = a :=
  igFrobAlg_self_fusion a

/-- Criticality is a monad: the IGProtocol preserves the ⊙ gate -/
theorem crit_preserved_under_self_fusion (a : Imscription) (h : a.crit = Criticality.monad) :
    (igFrobeniusAlg.mul a a).crit = Criticality.monad := by
  rw [igFrobAlg_self_fusion a]
  exact h

-- ============================================================
-- §11. PROTOCOL SOUNDNESS: CRITICALITY MONAD LADDER
-- ============================================================

/-- All three ob3ect tuples (catalog, original ob3ect, entry ob3ect) share
    the exact same criticality: monad. This is the Pratt certificate monad's
    structural essence. -/
theorem all_tuples_share_monad_criticality :
    prattCertificateMonad_imscription.crit = Criticality.monad ∧
    prattCertificateMonad_ob3ect_imscription.crit = Criticality.monad ∧
    pratt_entry_ob3ect_imscription.crit = Criticality.monad := by
  simp [prattCertificateMonad_imscription, prattCertificateMonad_ob3ect_imscription,
        pratt_entry_ob3ect_imscription]

/-- The Frobenius algebra on Imscription is idempotent for monad-critical
    objects: μ(a,a) = a when crit=monad. -/
theorem monad_critical_idempotent (a : Imscription) (_hcrit : a.crit = Criticality.monad) :
    igFrobeniusAlg.mul a a = a :=
  igFrobAlg_self_fusion a

/-- Tier stability: all three tuples are at or above O₂.
    Catalog: O₂†, both ob3ects: O₂ -/
theorem all_tuples_tier_at_least_O2 :
    (imscriptionTier prattCertificateMonad_imscription =
     Imscribing.Primitives.OuroboricityTier.O₂dag ∨
     imscriptionTier prattCertificateMonad_imscription =
     Imscribing.Primitives.OuroboricityTier.O₂) ∧
    imscriptionTier prattCertificateMonad_ob3ect_imscription =
      Imscribing.Primitives.OuroboricityTier.O₂ ∧
    imscriptionTier pratt_entry_ob3ect_imscription =
      Imscribing.Primitives.OuroboricityTier.O₂ := by
  have hcat : imscriptionTier prattCertificateMonad_imscription =
      Imscribing.Primitives.OuroboricityTier.O₂dag := prattCertificateMonad_tier
  have hobj : imscriptionTier prattCertificateMonad_ob3ect_imscription =
      Imscribing.Primitives.OuroboricityTier.O₂ := prattCertificateMonad_ob3ect_tier
  have hentry : imscriptionTier pratt_entry_ob3ect_imscription =
      Imscribing.Primitives.OuroboricityTier.O₂ := by
    unfold imscriptionTier pratt_entry_ob3ect_imscription
    decide
  exact ⟨Or.inl hcat, hobj, hentry⟩

end Imscribing.NumberTheory.PrattCertificateMonad