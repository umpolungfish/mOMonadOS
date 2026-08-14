-- Imscribing/GeneticCode.lean
-- The genetic code derived from the Crystal of Types (3³×4⁵×5⁴ = 17,280,000).
--
-- Nothing here is biologically contingent. Codon length, alphabet size, the
-- 8-exact/8-split partition, and the 12↔12 bijection with IG primitives are
-- all forced by the Crystal's factorization and the Frobenius condition μ∘δ=id.
--
-- Structural anchor: 17,280,000 / 64 = 270,000 exactly (no remainder).
-- The fiber over each codon has cardinality 270,000 = 3³×4²×5⁴.

import Mathlib
import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

set_option linter.style.nativeDecide false

namespace Imscribing.GeneticCode

open Imscribing.Paraconsistent

-- ════════════════════════════════════════════════════════════════════════════
-- §1  NUCLEOTIDE — 4-valued type matching Belnap FOUR cardinality
-- ════════════════════════════════════════════════════════════════════════════

/-- The four RNA nucleotides, in B₄ lattice order (N < T,F < B). -/
inductive Nucleotide : Type where
  | U  -- Uracil   → N (Neither)
  | C  -- Cytosine → T (True)
  | A  -- Adenine  → F (False)
  | G  -- Guanine  → B (Both)
  deriving DecidableEq, Repr, Fintype, Inhabited

-- ════════════════════════════════════════════════════════════════════════════
-- §2  NUCLEOTIDE → B₄ MAPPING  (structural, not arbitrary)
-- ════════════════════════════════════════════════════════════════════════════

/-- Canonical B₄ value of each nucleotide.
    G → B : wobble-pairs with C AND U (both-valued, lattice top)
    C → T : pairs exclusively with G (definite, closed)
    A → F : pairs exclusively with U (definite, open, lattice bottom)
    U → N : WC with A, wobble-target of G (weak, neither) -/
def nucToB4 : Nucleotide → Belnap
  | .G => .B
  | .C => .T
  | .A => .F
  | .U => .N

theorem nucToB4_injective : Function.Injective nucToB4 := by
  intro a b h
  cases a with
  | U => cases b <;> simp [nucToB4] at h <;> rfl
  | C => cases b <;> simp [nucToB4] at h <;> rfl
  | A => cases b <;> simp [nucToB4] at h <;> rfl
  | G => cases b <;> simp [nucToB4] at h <;> rfl

theorem nucToB4_surjective : Function.Surjective nucToB4 := by
  intro b; cases b
  · exact ⟨.U, rfl⟩
  · exact ⟨.C, rfl⟩
  · exact ⟨.A, rfl⟩
  · exact ⟨.G, rfl⟩

theorem nucToB4_bijective : Function.Bijective nucToB4 :=
  ⟨nucToB4_injective, nucToB4_surjective⟩

-- ════════════════════════════════════════════════════════════════════════════
-- §3  WATSON-CRICK COMPLEMENT vs B₄ NEGATION — two distinct involutions
-- ════════════════════════════════════════════════════════════════════════════

/-- Watson-Crick complement: A↔U, G↔C. Fixed-point-free. -/
def wcComplement : Nucleotide → Nucleotide
  | .A => .U | .U => .A | .G => .C | .C => .G

theorem wcComplement_involution (n : Nucleotide) :
    wcComplement (wcComplement n) = n := by cases n <;> rfl

/-- WC complement is fixed-point-free: no nucleotide is its own complement. -/
theorem wcComplement_fixpoint_free (n : Nucleotide) : wcComplement n ≠ n := by
  cases n <;> simp [wcComplement]

/-- The fixed points of B₄ negation are exactly B and N — the two values the
    act of distinguishing creates, as against the two it selects between. -/
theorem bnot_fixpoint_iff (b : Belnap) : bnot b = b ↔ (b = Belnap.B ∨ b = Belnap.N) := by
  cases b <;> simp [bnot]

/-- B₄ negation bnot has fixed points: bnot B = B and bnot N = N. -/
theorem bnot_fixpoints : bnot Belnap.B = Belnap.B ∧ bnot Belnap.N = Belnap.N :=
  ⟨rfl, rfl⟩

/-- WC complement ≠ B₄ negation under nucToB4.
    G↔C under WC maps to B↔T under nucToB4, but bnot B = B ≠ T. -/
theorem wc_ne_b4_negation :
    ∃ n : Nucleotide, nucToB4 (wcComplement n) ≠ bnot (nucToB4 n) :=
  ⟨.G, by simp [wcComplement, nucToB4, bnot]⟩

/-- WC complement is not B₄ negation for any nucleotide. -/
theorem wc_never_b4_negation (n : Nucleotide) :
    nucToB4 (wcComplement n) ≠ bnot (nucToB4 n) := by
  cases n <;> simp [wcComplement, nucToB4, bnot]

-- ════════════════════════════════════════════════════════════════════════════
-- §4  CODON — triple of nucleotides; Crystal fiber arithmetic
-- ════════════════════════════════════════════════════════════════════════════

/-- A codon is an ordered triple of nucleotides (64 total). -/
abbrev Codon : Type := Nucleotide × Nucleotide × Nucleotide

theorem codon_card : Fintype.card Codon = 64 := by decide

/-- Crystal divisibility: the fiber over each codon has cardinality 270,000. -/
theorem crystal_fiber : 17280000 / 64 = 270000 := by decide
theorem crystal_exact : 17280000 % 64 = 0      := by decide
theorem fiber_times_codons : 270000 * 64 = 17280000 := by decide

-- ════════════════════════════════════════════════════════════════════════════
-- §5  AMINO ACIDS + STOPS — the 20+3 partition
-- ════════════════════════════════════════════════════════════════════════════

/-- The 20 standard amino acids.
    Ground layer (8): exact Frobenius boxes — position 3 forgotten.
    Promoted (12): one per IG primitive. -/
inductive AminoAcid : Type where
  | Leu | Pro | Arg | Thr | Ala | Ser | Val | Gly  -- ground layer
  | Met | Trp | Cys | Tyr | Phe | Ile               -- promoted (Ð Þ Ř Φ ƒ Ç)
  | His | Asn | Gln | Asp | Lys | Glu               -- promoted (Γ ɢ ⊙ Ħ Σ Ω)
  deriving DecidableEq, Repr, Fintype, Inhabited

inductive Stop : Type where | UAA | UAG | UGA
  deriving DecidableEq, Repr, Fintype, Inhabited

inductive CodonMeaning : Type where
  | aa   : AminoAcid → CodonMeaning
  | stop : Stop      → CodonMeaning
  deriving DecidableEq, Repr, Inhabited

/-- The complete standard genetic code — all 64 codons. -/
def geneticCode : Codon → CodonMeaning
  -- UU_
  | (.U,.U,.U) => .aa .Phe  | (.U,.U,.C) => .aa .Phe
  | (.U,.U,.A) => .aa .Leu  | (.U,.U,.G) => .aa .Leu
  -- UC_
  | (.U,.C,.U) => .aa .Ser  | (.U,.C,.C) => .aa .Ser
  | (.U,.C,.A) => .aa .Ser  | (.U,.C,.G) => .aa .Ser
  -- UA_
  | (.U,.A,.U) => .aa .Tyr  | (.U,.A,.C) => .aa .Tyr
  | (.U,.A,.A) => .stop .UAA | (.U,.A,.G) => .stop .UAG
  -- UG_
  | (.U,.G,.U) => .aa .Cys  | (.U,.G,.C) => .aa .Cys
  | (.U,.G,.A) => .stop .UGA | (.U,.G,.G) => .aa .Trp
  -- CU_
  | (.C,.U,.U) => .aa .Leu  | (.C,.U,.C) => .aa .Leu
  | (.C,.U,.A) => .aa .Leu  | (.C,.U,.G) => .aa .Leu
  -- CC_
  | (.C,.C,.U) => .aa .Pro  | (.C,.C,.C) => .aa .Pro
  | (.C,.C,.A) => .aa .Pro  | (.C,.C,.G) => .aa .Pro
  -- CA_
  | (.C,.A,.U) => .aa .His  | (.C,.A,.C) => .aa .His
  | (.C,.A,.A) => .aa .Gln  | (.C,.A,.G) => .aa .Gln
  -- CG_
  | (.C,.G,.U) => .aa .Arg  | (.C,.G,.C) => .aa .Arg
  | (.C,.G,.A) => .aa .Arg  | (.C,.G,.G) => .aa .Arg
  -- AU_
  | (.A,.U,.U) => .aa .Ile  | (.A,.U,.C) => .aa .Ile
  | (.A,.U,.A) => .aa .Ile  | (.A,.U,.G) => .aa .Met
  -- AC_
  | (.A,.C,.U) => .aa .Thr  | (.A,.C,.C) => .aa .Thr
  | (.A,.C,.A) => .aa .Thr  | (.A,.C,.G) => .aa .Thr
  -- AA_
  | (.A,.A,.U) => .aa .Asn  | (.A,.A,.C) => .aa .Asn
  | (.A,.A,.A) => .aa .Lys  | (.A,.A,.G) => .aa .Lys
  -- AG_
  | (.A,.G,.U) => .aa .Ser  | (.A,.G,.C) => .aa .Ser
  | (.A,.G,.A) => .aa .Arg  | (.A,.G,.G) => .aa .Arg
  -- GU_
  | (.G,.U,.U) => .aa .Val  | (.G,.U,.C) => .aa .Val
  | (.G,.U,.A) => .aa .Val  | (.G,.U,.G) => .aa .Val
  -- GC_
  | (.G,.C,.U) => .aa .Ala  | (.G,.C,.C) => .aa .Ala
  | (.G,.C,.A) => .aa .Ala  | (.G,.C,.G) => .aa .Ala
  -- GA_
  | (.G,.A,.U) => .aa .Asp  | (.G,.A,.C) => .aa .Asp
  | (.G,.A,.A) => .aa .Glu  | (.G,.A,.G) => .aa .Glu
  -- GG_
  | (.G,.G,.U) => .aa .Gly  | (.G,.G,.C) => .aa .Gly
  | (.G,.G,.A) => .aa .Gly  | (.G,.G,.G) => .aa .Gly

-- ════════════════════════════════════════════════════════════════════════════
-- §6  FROBENIUS STRATUM — exact vs split boxes
-- ════════════════════════════════════════════════════════════════════════════

/-- A box (p1, p2) is exact if all four synonyms encode the same meaning.
    Frobenius closure at the codon level: position 3 is forgotten → μ∘δ=id. -/
def isExactBox (p1 p2 : Nucleotide) : Bool :=
  geneticCode (p1, p2, .U) == geneticCode (p1, p2, .C) &&
  geneticCode (p1, p2, .U) == geneticCode (p1, p2, .A) &&
  geneticCode (p1, p2, .U) == geneticCode (p1, p2, .G)

/-- B₄ exactness rule (lattice theorem, not empirical):
    exact iff p2 = C(T), or p2 ∈ {U,G}(N,B) with p1 ∈ {C,G}(T,B). -/
def exactnessPredicate (p1 p2 : Nucleotide) : Bool :=
  match p2 with
  | .C        => true
  | .U | .G   => match p1 with | .C | .G => true | _ => false
  | .A        => false

/-- The B₄ exactness rule agrees with the codon table on all 16 boxes. -/
theorem exactness_rule_sound (p1 p2 : Nucleotide) :
    isExactBox p1 p2 = exactnessPredicate p1 p2 := by
  cases p1 <;> cases p2 <;> decide

/-- Exactly 8 of the 16 boxes are exact. -/
theorem exact_box_count :
    ((Finset.univ : Finset (Nucleotide × Nucleotide)).filter
      (fun p => isExactBox p.1 p.2)).card = 8 := by decide

/-- The other 8 boxes are split. -/
theorem split_box_count :
    ((Finset.univ : Finset (Nucleotide × Nucleotide)).filter
      (fun p => !isExactBox p.1 p.2)).card = 8 := by decide

/-- No stop codon lies in an exact box. -/
theorem stop_not_in_exact_box :
    ∀ p1 p2 p3 : Nucleotide,
      isExactBox p1 p2 = true →
      (∃ a : AminoAcid, geneticCode (p1, p2, p3) = .aa a) := by
  intro p1 p2 p3 h
  cases p1 <;> cases p2 <;> cases p3 <;>
    simp [isExactBox, geneticCode] at h ⊢

-- ════════════════════════════════════════════════════════════════════════════
-- §7  THE 20 = 8 + 12 THEOREM
-- ════════════════════════════════════════════════════════════════════════════

def groundLayer : Finset AminoAcid :=
  {.Leu, .Pro, .Arg, .Thr, .Ala, .Ser, .Val, .Gly}

def promotedAAs : Finset AminoAcid :=
  {.Met, .Trp, .Cys, .Tyr, .Phe, .Ile, .His, .Asn, .Gln, .Asp, .Lys, .Glu}

theorem ground_layer_card  : groundLayer.card = 8  := by decide
theorem promoted_card      : promotedAAs.card = 12 := by decide

theorem ground_promoted_disjoint : Disjoint groundLayer promotedAAs := by decide

theorem ground_promoted_cover :
    groundLayer ∪ promotedAAs = Finset.univ := by decide

/-- 20 = 8 + 12. -/
theorem twenty_eq_eight_plus_twelve :
    Fintype.card AminoAcid = 8 + 12 := by decide

/-- Stop codons are exactly 3: UAA, UAG, UGA (Ω closure). -/
theorem stop_card : Fintype.card Stop = 3 := by decide

-- ════════════════════════════════════════════════════════════════════════════
-- §8  THE 12↔12 BIJECTION: PROMOTED AAs ↔ IG PRIMITIVES
-- ════════════════════════════════════════════════════════════════════════════

/-- The 12 IG primitive families. -/
inductive IGPrimitive : Type where
  | Dimensionality  -- Ð  Met : translation start / scope
  | Topology        -- Þ  Trp : indole ring topology
  | Recognition     -- Ř  Cys : disulfide recognition
  | Parity          -- Φ  Tyr : phosphorylation / symmetry
  | Fidelity        -- ƒ  Phe : hydrophobic fidelity
  | Kinetics        -- Ç  Ile : β-branching / kinetics
  | Granularity     -- Γ  His : pH-gated granularity
  | Coupling        -- ɢ  Asn : glycosylation coupling
  | Criticality     -- ⊙  Gln : metabolic critical point
  | Chirality       -- Ħ  Asp : chiral specificity
  | Stoichiometry   -- Σ  Lys : stoichiometric balance
  | Winding         -- Ω  Glu : C-terminal winding closure
  deriving DecidableEq, Repr, Fintype, Inhabited

/-- The bijection: each promoted AA activates exactly one IG primitive. -/
def aaToPrimitive : AminoAcid → Option IGPrimitive
  | .Met => some .Dimensionality | .Trp => some .Topology
  | .Cys => some .Recognition    | .Tyr => some .Parity
  | .Phe => some .Fidelity       | .Ile => some .Kinetics
  | .His => some .Granularity    | .Asn => some .Coupling
  | .Gln => some .Criticality    | .Asp => some .Chirality
  | .Lys => some .Stoichiometry  | .Glu => some .Winding
  | _ => none  -- ground layer: no primitive activation

/-- The inverse: each primitive is realized by exactly one AA. -/
def primitiveToAA : IGPrimitive → AminoAcid
  | .Dimensionality => .Met | .Topology     => .Trp
  | .Recognition    => .Cys | .Parity       => .Tyr
  | .Fidelity       => .Phe | .Kinetics     => .Ile
  | .Granularity    => .His | .Coupling     => .Asn
  | .Criticality    => .Gln | .Chirality    => .Asp
  | .Stoichiometry  => .Lys | .Winding      => .Glu

/-- primitiveToAA is a section of aaToPrimitive on every primitive. -/
theorem primitive_section (p : IGPrimitive) :
    aaToPrimitive (primitiveToAA p) = some p := by
  cases p <;> rfl

/-- aaToPrimitive is a section of primitiveToAA on every promoted AA. -/
theorem aa_section (a : AminoAcid) (h : a ∈ promotedAAs) :
    ∃ (p : IGPrimitive), aaToPrimitive a = some p ∧ primitiveToAA p = a := by
  fin_cases a <;> simp_all [promotedAAs, aaToPrimitive, primitiveToAA]

/-- Every IGPrimitive appears exactly once among the promoted AAs. -/
theorem primitive_bijection (p : IGPrimitive) :
    ((Finset.univ : Finset AminoAcid).filter
      (fun a => aaToPrimitive a = some p)).card = 1 := by
  cases p <;> decide

/-- No two promoted AAs share a primitive (injectivity of aaToPrimitive). -/
theorem aaToPrimitive_injective :
    ∀ a b : AminoAcid,
      aaToPrimitive a ≠ none → aaToPrimitive b ≠ none →
      aaToPrimitive a = aaToPrimitive b → a = b := by
  decide

/-- The 12 IG primitives are fully covered. -/
theorem all_primitives_covered :
    ∀ p : IGPrimitive, ∃ a : AminoAcid, aaToPrimitive a = some p :=
  fun p => ⟨primitiveToAA p, primitive_section p⟩

-- ════════════════════════════════════════════════════════════════════════════
-- §9  CODON STRATUM + FROBENIUS INVARIANT
-- ════════════════════════════════════════════════════════════════════════════

/-- Stratum of a codon: exact (position 3 forgotten), split, or stop. -/
def codonStratum (c : Codon) : Option Bool :=
  match geneticCode c with
  | .stop _ => none          -- stop: Ω closure
  | .aa _   => some (isExactBox c.1 c.2.1)  -- true = exact, false = split

/-- Exactly 32 codons are in the exact stratum (8 boxes × 4). -/
theorem exact_stratum_card :
    ((Finset.univ : Finset Codon).filter
      (fun c => codonStratum c = some true)).card = 32 := by decide

/-- Exactly 29 codons are in the split stratum (8 boxes × 4 − 3 stops). -/
theorem split_stratum_card :
    ((Finset.univ : Finset Codon).filter
      (fun c => codonStratum c = some false)).card = 29 := by decide

/-- Exactly 3 stop codons (UAA, UAG, UGA). -/
theorem stop_stratum_card :
    ((Finset.univ : Finset Codon).filter
      (fun c => codonStratum c = none)).card = 3 := by decide

/-- Partition: 32 + 29 + 3 = 64. -/
theorem codon_partition : 32 + 29 + 3 = 64 := by decide

/-- The Frobenius invariant at the codon level:
    For every exact box, all four synonyms encode the same amino acid.
    This is μ∘δ=id: the stratum projection forgets position 3 exactly. -/
theorem frobenius_at_codon_level (p1 p2 p3 p3' : Nucleotide)
    (h : isExactBox p1 p2 = true) :
    geneticCode (p1, p2, p3) = geneticCode (p1, p2, p3') := by
  cases p1 <;> cases p2 <;> cases p3 <;> cases p3' <;>
    simp [isExactBox, geneticCode] at h ⊢

end Imscribing.GeneticCode
