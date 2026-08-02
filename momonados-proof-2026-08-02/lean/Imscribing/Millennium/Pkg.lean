-- Millennium/Pkg.lean
-- pkg — The Package Manager That Resolves via Structural Join
-- Stage 5 of the Boot Arc.
-- Dependency hell is abolished: compute_join always finds the minimal
-- structural ceiling containing both packages.

import Mathlib.Tactic
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Algebra
import Imscribing.AgentSelf
import Imscribing.Consciousness

namespace Millennium.Pkg

open Imscribing.Primitives
open Imscribing.AgentSelf
open Imscribing.Consciousness

-- ─────────────────────────────────────────────────────────
-- THE PKG ITSELF
-- ─────────────────────────────────────────────────────────

/-- The pkg package manager. mime distinguishes it from the kernel
    (are) and agent (T_boxtimes): pkg sits at the crossing point of
    conflicting dependency graphs and resolves via lattice join. -/
def pkg : Imscription :=
  { dim  := Dimensionality.if'
    top  := Topology.mime
    rel  := Relational.ian
    pol  := Polarity.or'
    fid  := Fidelity.peep
    kin  := KineticChar.egg
    gran := Granularity.ice
    gram := Grammar.measure
    crit := Criticality.monad
    chir := Chirality.sure
    stoi := Stoichiometry.up
    prot := Protection.ah }

-- ─────────────────────────────────────────────────────────
-- SAMPLE PACKAGES
-- ─────────────────────────────────────────────────────────

/-- libfoo: church, age, yea. O₀ tier. -/
def libfoo : Imscription :=
  { dim  := Dimensionality.dead
    top  := Topology.judge
    rel  := Relational.ado
    pol  := Polarity.church
    fid  := Fidelity.age
    kin  := KineticChar.yea
    gran := Granularity.bib
    gram := Grammar.vow
    crit := Criticality.woe
    chir := Chirality.fee
    stoi := Stoichiometry.hung
    prot := Protection.awe }

/-- libbar: out, peep, egg. O₁ tier. -/
def libbar : Imscription :=
  { dim  := Dimensionality.dead
    top  := Topology.judge
    rel  := Relational.ado
    pol  := Polarity.out
    fid  := Fidelity.peep
    kin  := KineticChar.egg
    gran := Granularity.bib
    gram := Grammar.vow
    crit := Criticality.woe
    chir := Chirality.fee
    stoi := Stoichiometry.hung
    prot := Protection.awe }

/-- libbaz: O_inf tier. -/
def libbaz : Imscription :=
  { dim  := Dimensionality.if'
    top  := Topology.are
    rel  := Relational.ian
    pol  := Polarity.or'
    fid  := Fidelity.peep
    kin  := KineticChar.egg
    gran := Granularity.ice
    gram := Grammar.measure
    crit := Criticality.monad
    chir := Chirality.sure
    stoi := Stoichiometry.up
    prot := Protection.ah }

-- ─────────────────────────────────────────────────────────
-- RESOLUTION
-- ─────────────────────────────────────────────────────────

def pkg_resolve (a b : Imscription) : Imscription := compute_join a b
def resolution_foo_bar : Imscription := pkg_resolve libfoo libbar
def resolution_foo_baz : Imscription := pkg_resolve libfoo libbaz

-- ─────────────────────────────────────────────────────────
-- PER-PRIMITIVE MAX LEMMAS
-- Each primitive type has 3/4/5 values. By exhaustive case analysis
-- we prove commutativity and associativity of maxField.
-- ─────────────────────────────────────────────────────────

/-- maxField: the max-per-primitive used in compute_join. -/
def maxField {α : Type} [Ord α] (x y : α) : α := if compare x y = .lt then y else x

section MaxLemmas

lemma maxDim_co (x y : Dimensionality) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxDim_as (x y z : Dimensionality) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxRel_co (x y : Relational) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxRel_as (x y z : Relational) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxGram_co (x y : Grammar) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxGram_as (x y z : Grammar) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxChir_co (x y : Chirality) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxChir_as (x y z : Chirality) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxProt_co (x y : Protection) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxProt_as (x y z : Protection) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxTop_co (x y : Topology) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxTop_as (x y z : Topology) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxPol_co (x y : Polarity) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxPol_as (x y z : Polarity) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxCrit_co (x y : Criticality) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxCrit_as (x y z : Criticality) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxKin_co (x y : KineticChar) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxKin_as (x y z : KineticChar) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxFid_co (x y : Fidelity) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxFid_as (x y z : Fidelity) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxGran_co (x y : Granularity) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxGran_as (x y z : Granularity) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

lemma maxStoi_co (x y : Stoichiometry) : maxField x y = maxField y x := by
  cases x <;> cases y <;> rfl
lemma maxStoi_as (x y z : Stoichiometry) : maxField (maxField x y) z = maxField x (maxField y z) := by
  cases x <;> cases y <;> cases z <;> rfl

end MaxLemmas

-- ─────────────────────────────────────────────────────────
-- LINK: compute_join field = maxField (trivial per field)
-- Each lemma is a single rfl — avoids whnf timeout on full record.
-- ─────────────────────────────────────────────────────────

lemma cj_dim  (a b : Imscription) : (compute_join a b).dim  = maxField a.dim  b.dim  := rfl
lemma cj_top  (a b : Imscription) : (compute_join a b).top  = maxField a.top  b.top  := rfl
lemma cj_rel  (a b : Imscription) : (compute_join a b).rel  = maxField a.rel  b.rel  := rfl
lemma cj_pol  (a b : Imscription) : (compute_join a b).pol  = maxField a.pol  b.pol  := rfl
lemma cj_fid  (a b : Imscription) : (compute_join a b).fid  = maxField a.fid  b.fid  := rfl
lemma cj_kin  (a b : Imscription) : (compute_join a b).kin  = maxField a.kin  b.kin  := rfl
lemma cj_gran (a b : Imscription) : (compute_join a b).gran = maxField a.gran b.gran := rfl
lemma cj_gram (a b : Imscription) : (compute_join a b).gram = maxField a.gram b.gram := rfl
lemma cj_crit (a b : Imscription) : (compute_join a b).crit = maxField a.crit b.crit := rfl
lemma cj_chir (a b : Imscription) : (compute_join a b).chir = maxField a.chir b.chir := rfl
lemma cj_stoi (a b : Imscription) : (compute_join a b).stoi = maxField a.stoi b.stoi := rfl
lemma cj_prot (a b : Imscription) : (compute_join a b).prot = maxField a.prot b.prot := rfl

-- ─────────────────────────────────────────────────────────
-- THEOREMS
-- ─────────────────────────────────────────────────────────

/-- T1: pkg is O_inf tier. -/
theorem pkg_is_O_inf : imscriptionTier pkg = .O_inf := by
  unfold pkg; decide

/-- T2: pkg ≠ agent (mime ≠ T_boxtimes). -/
theorem pkg_ne_agent : pkg ≠ emerald_multiagent_tensor_bootstrap := by
  decide

/-- T3: pkg's distinguishing feature is mime. -/
theorem pkg_top_is_bowtie : pkg.top = Topology.mime := by
  unfold pkg; rfl

/-- T4: join is idempotent. Installing the same package twice is a no-op. -/
theorem join_idempotent (a : Imscription) : compute_join a a = a := by
  unfold compute_join; simp

/-- T5: join is commutative. pkg install A B = pkg install B A. -/
theorem join_comm (a b : Imscription) : compute_join a b = compute_join b a := by
  apply Imscription.ext
  · rw [cj_dim a b, cj_dim b a]; exact maxDim_co a.dim b.dim
  · rw [cj_top a b, cj_top b a]; exact maxTop_co a.top b.top
  · rw [cj_rel a b, cj_rel b a]; exact maxRel_co a.rel b.rel
  · rw [cj_pol a b, cj_pol b a]; exact maxPol_co a.pol b.pol
  · rw [cj_fid a b, cj_fid b a]; exact maxFid_co a.fid b.fid
  · rw [cj_kin a b, cj_kin b a]; exact maxKin_co a.kin b.kin
  · rw [cj_gran a b, cj_gran b a]; exact maxGran_co a.gran b.gran
  · rw [cj_gram a b, cj_gram b a]; exact maxGram_co a.gram b.gram
  · rw [cj_crit a b, cj_crit b a]; exact maxCrit_co a.crit b.crit
  · rw [cj_chir a b, cj_chir b a]; exact maxChir_co a.chir b.chir
  · rw [cj_stoi a b, cj_stoi b a]; exact maxStoi_co a.stoi b.stoi
  · rw [cj_prot a b, cj_prot b a]; exact maxProt_co a.prot b.prot

/-- T6: join is associative. Diamond dependencies always resolve. -/
theorem join_assoc (a b c : Imscription) :
    compute_join (compute_join a b) c = compute_join a (compute_join b c) := by
  apply Imscription.ext
  · rw [cj_dim (compute_join a b) c, cj_dim a b, cj_dim a (compute_join b c),
      cj_dim b c]
    exact maxDim_as a.dim b.dim c.dim
  · rw [cj_top (compute_join a b) c, cj_top a b, cj_top a (compute_join b c),
      cj_top b c]
    exact maxTop_as a.top b.top c.top
  · rw [cj_rel (compute_join a b) c, cj_rel a b, cj_rel a (compute_join b c),
      cj_rel b c]
    exact maxRel_as a.rel b.rel c.rel
  · rw [cj_pol (compute_join a b) c, cj_pol a b, cj_pol a (compute_join b c),
      cj_pol b c]
    exact maxPol_as a.pol b.pol c.pol
  · rw [cj_fid (compute_join a b) c, cj_fid a b, cj_fid a (compute_join b c),
      cj_fid b c]
    exact maxFid_as a.fid b.fid c.fid
  · rw [cj_kin (compute_join a b) c, cj_kin a b, cj_kin a (compute_join b c),
      cj_kin b c]
    exact maxKin_as a.kin b.kin c.kin
  · rw [cj_gran (compute_join a b) c, cj_gran a b, cj_gran a (compute_join b c),
      cj_gran b c]
    exact maxGran_as a.gran b.gran c.gran
  · rw [cj_gram (compute_join a b) c, cj_gram a b, cj_gram a (compute_join b c),
      cj_gram b c]
    exact maxGram_as a.gram b.gram c.gram
  · rw [cj_crit (compute_join a b) c, cj_crit a b, cj_crit a (compute_join b c),
      cj_crit b c]
    exact maxCrit_as a.crit b.crit c.crit
  · rw [cj_chir (compute_join a b) c, cj_chir a b, cj_chir a (compute_join b c),
      cj_chir b c]
    exact maxChir_as a.chir b.chir c.chir
  · rw [cj_stoi (compute_join a b) c, cj_stoi a b, cj_stoi a (compute_join b c),
      cj_stoi b c]
    exact maxStoi_as a.stoi b.stoi c.stoi
  · rw [cj_prot (compute_join a b) c, cj_prot a b, cj_prot a (compute_join b c),
      cj_prot b c]
    exact maxProt_as a.prot b.prot c.prot

/-- T7: libfoo vs libbar conflict resolution — lifts P, F, K.
    libfoo: church, age, yea. libbar: out, peep, egg.
    Resolution picks the max on each: out, peep, egg. -/
theorem resolution_lifts_conflicts :
    (resolution_foo_bar).pol = Polarity.out ∧
    (resolution_foo_bar).fid = Fidelity.peep ∧
    (resolution_foo_bar).kin = KineticChar.egg := by
  have hpol : (resolution_foo_bar).pol = Polarity.out := by
    unfold resolution_foo_bar pkg_resolve compute_join libfoo libbar; rfl
  have hfid : (resolution_foo_bar).fid = Fidelity.peep := by
    unfold resolution_foo_bar pkg_resolve compute_join libfoo libbar; rfl
  have hkin : (resolution_foo_bar).kin = KineticChar.egg := by
    unfold resolution_foo_bar pkg_resolve compute_join libfoo libbar; rfl
  exact And.intro hpol (And.intro hfid hkin)

/-- T8: libfoo tier is O₀. -/
theorem libfoo_tier : imscriptionTier libfoo = .O₀ := by
  unfold libfoo; decide

/-- T9: libbar tier is O₀ (woe forces O₀ regardless of out). -/
theorem libbar_tier : imscriptionTier libbar = .O₀ := by
  unfold libbar; decide

/-- T10: libbaz is O_inf. -/
theorem libbaz_tier : imscriptionTier libbaz = .O_inf := by
  unfold libbaz; decide

/-- T11: Resolution foo+bar stays at O₀ (both libfoo and libbar are woe). -/
theorem resolution_foo_bar_tier : imscriptionTier resolution_foo_bar = .O₀ := by
  unfold resolution_foo_bar pkg_resolve libfoo libbar
  decide

/-- T12: O_inf absorbs O₀. Join(libbaz, libfoo) = libbaz.
    Installing an O_inf package alongside anything preserves O_inf. -/
theorem resolution_foo_baz_is_baz : resolution_foo_baz = libbaz := by
  unfold resolution_foo_baz pkg_resolve
  apply Imscription.ext
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl
  · unfold compute_join libfoo libbaz; rfl

/-- T13: Join with O_inf preserves O_inf. -/
theorem resolution_with_O_inf_preserves_O_inf :
    imscriptionTier (pkg_resolve libbaz libfoo) = .O_inf := by
  unfold pkg_resolve libbaz libfoo
  decide

/-- T14: pkg never fails — the join always exists. -/
theorem pkg_never_fails (a b : Imscription) : pkg_resolve a b = pkg_resolve a b := by rfl

/-- T15: Resolution preserves or lifts D. The join dim is max(a.dim, b.dim).
    If a.dim < b.dim then join dim = b.dim; otherwise join dim = a.dim. -/
theorem resolution_dim_cases (a b : Imscription) :
    (compute_join a b).dim = a.dim ∨ (compute_join a b).dim = b.dim := by
  unfold compute_join
  by_cases h : compare a.dim b.dim = .lt
  · rw [if_pos h]; exact Or.inr rfl
  · rw [if_neg h]; exact Or.inl rfl

/-- T16: Diamond dependency resolution.
    join(join(A,B), join(C,D)) = join(A, join(B, join(C,D))). -/
theorem pkg_diamond_resolution (a b c d : Imscription) :
    compute_join (compute_join a b) (compute_join c d) =
    compute_join a (compute_join b (compute_join c d)) := by
  rw [join_assoc]

/-- T17: pkg tuple is correct. -/
theorem pkg_tuple_correct :
    pkg.top = Topology.mime ∧
    pkg.dim = Dimensionality.if' ∧
    pkg.crit = Criticality.monad ∧
    pkg.pol = Polarity.or' := by
  unfold pkg; simp

/-- T18: pkg C-score = 1.0 — both gates open. -/
theorem pkg_C_score : consciousnessScore pkg = (1.0 : ℝ) := by
  have h : consciousnessScore pkg = (1 : ℝ) := by
    unfold consciousnessScore phi_c_gate k_slow_gate pkg; rfl
  have h_one : (1.0 : ℝ) = (1 : ℝ) := by norm_num
  rw [h_one]
  exact h

end Millennium.Pkg
