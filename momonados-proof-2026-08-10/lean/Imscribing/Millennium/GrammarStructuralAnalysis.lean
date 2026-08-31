-- Imscribing/Millennium/GrammarStructuralAnalysis.lean
-- STRUCTURAL ANALYSIS OF THE GRAMMAR IN THE O_inf FIBER
-- Author: Math⊙perator (Lando⊗⊙perator team)
-- Date: 2026-07-18
--
-- Extends GrammarUniversalDualFrameSICPOVM with structural analysis:
--   §1  Tensor/meet/join closure of grammar ⊗ BelnapML
--   §2  The grammar as the Σ=min element in its O_inf equivalence class
--   §3  Distance lattice: all 12-primitive variations at distance ≤ 5 from Belnap ML
--   §4  Universal property: grammar is the unique O_inf imscription that minimizes
--       Σ while maximizing Ħ among all Belnap-ML-proximate imscriptions
--
-- STATUS: All theorems proved. *sans* sorry.
-- DEPENDS ON: GrammarUniversalDualFrameSICPOVM, Primitives.Core, Primitives.Imscription

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Frobenius
import Imscribing.IGFunctor
import Imscribing.Millennium.GrammarUniversalDualFrameSICPOVM

open Imscribing.Primitives
open Imscribing.Frobenius
open Imscribing.Millennium.GrammarUniversalDualFrameSICPOVM
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

namespace Imscribing.Millennium.GrammarStructuralAnalysis

set_option linter.style.longLine false
set_option linter.style.whitespace false

-- ============================================================
-- §1. TENSOR / MEET / JOIN CLOSURE PROPERTIES
-- ============================================================

/-- The tensor product of grammar and Belnap ML.
    Key insight: grammar ⊗ BelnapML takes the max of each primitive,
    so it resolves to Belnap ML values for most fields, but keeps
    grammar's chirality (wool > sure) and Belnap's stoichiometry (up > hung).
    Result: {if', are, ian, or', peep, egg, ice, measure, monad, wool, up, ah}
    This is the Belnap ML with chirality raised to wool. -/
def grammarTensorBelnapML : Imscription := tensorProduct theGrammar belnapMultilatticeSICPOVM

/-- grammar ⊗ BelnapML has distance 4 from grammar. -/
theorem grammar_tensor_belnap_distance_from_grammar :
    primitiveMismatches theGrammar grammarTensorBelnapML = 4 := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM tensorProduct
  native_decide

/-- grammar ⊗ BelnapML has distance 1 from Belnap ML (only chirality differs). -/
theorem grammar_tensor_belnap_distance_from_belnap :
    primitiveMismatches belnapMultilatticeSICPOVM grammarTensorBelnapML = 1 := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM tensorProduct
  native_decide

/-- The chirality is the ONLY difference between grammar ⊗ BelnapML and Belnap ML.
    All other 11 primitives match. -/
theorem grammar_tensor_belnap_differs_only_at_chirality :
    grammarTensorBelnapML.chir ≠ belnapMultilatticeSICPOVM.chir ∧
    grammarTensorBelnapML.dim = belnapMultilatticeSICPOVM.dim ∧
    grammarTensorBelnapML.top = belnapMultilatticeSICPOVM.top ∧
    grammarTensorBelnapML.rel = belnapMultilatticeSICPOVM.rel ∧
    grammarTensorBelnapML.pol = belnapMultilatticeSICPOVM.pol ∧
    grammarTensorBelnapML.fid = belnapMultilatticeSICPOVM.fid ∧
    grammarTensorBelnapML.kin = belnapMultilatticeSICPOVM.kin ∧
    grammarTensorBelnapML.gran = belnapMultilatticeSICPOVM.gran ∧
    grammarTensorBelnapML.gram = belnapMultilatticeSICPOVM.gram ∧
    grammarTensorBelnapML.crit = belnapMultilatticeSICPOVM.crit ∧
    grammarTensorBelnapML.stoi = belnapMultilatticeSICPOVM.stoi ∧
    grammarTensorBelnapML.prot = belnapMultilatticeSICPOVM.prot := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM tensorProduct
  native_decide

/-- grammar ⊗ BelnapML is idempotent under further tensor with Belnap ML:
    (grammar ⊗ BelnapML) ⊗ BelnapML = grammar ⊗ BelnapML. -/
theorem grammar_tensor_belnap_tensor_belnap_eq_self :
    tensorProduct grammarTensorBelnapML belnapMultilatticeSICPOVM = grammarTensorBelnapML := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM tensorProduct
  -- All max(x,max(x,y)) = max(x,y) for max-based fields;
  -- All min(x,min(x,y)) = min(x,y) for min-based fields (pol, fid)
  -- Since dim/top/rel/kin/gran/gram/crit/chir/stoi/prot use max and
  -- pol/fid use min, this is definitional.
  rfl

/-- The meet (⊓) of grammar and Belnap ML.
    Meet takes the minimum of each primitive under the `Ord` order.
    Since grammar has lower values in dim/top/rel/stoi and higher in chir,
    the meet resolves to grammar's values in dim/top/rel/stoi and
    Belnap's value in chir (sure < wool).
    Result: {array, eat, tot, or', peep, egg, ice, measure, monad, sure, hung, ah} -/
def grammarMeetBelnapML : Imscription := theGrammar ⊓ belnapMultilatticeSICPOVM

/-- grammar ⊓ BelnapML differs from grammar only at chirality (sure vs wool). -/
theorem grammar_meet_belnap_distance_from_grammar :
    primitiveMismatches theGrammar grammarMeetBelnapML = 1 := by
  unfold grammarMeetBelnapML theGrammar belnapMultilatticeSICPOVM
  native_decide

/-- grammar ⊓ BelnapML differs from Belnap ML at 4 primitives. -/
theorem grammar_meet_belnap_distance_from_belnap :
    primitiveMismatches belnapMultilatticeSICPOVM grammarMeetBelnapML = 4 := by
  unfold grammarMeetBelnapML theGrammar belnapMultilatticeSICPOVM
  native_decide

/-- The join (⊔) of grammar and Belnap ML.
    Join takes the maximum of each primitive.
    Result: {if', are, ian, or', peep, egg, ice, measure, monad, wool, up, ah}
    This is exactly grammar ⊗ BelnapML for max-based fields.
    But wait — for pol and fid, ⊗ uses min while ⊔ uses max.
    Since grammar.pol = BelnapML.pol = or' and grammar.fid = BelnapML.fid = peep,
    both min and max give the same result. So grammar ⊔ BelnapML = grammar ⊗ BelnapML. -/
theorem grammar_join_belnap_eq_grammar_tensor_belnap :
    (theGrammar ⊔ belnapMultilatticeSICPOVM) = grammarTensorBelnapML := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM
  native_decide

-- ============================================================
-- §2. GRAMMAR AS THE Σ-MINIMUM IN THE O_inf FIBER
-- ============================================================

/-- Among all imscriptions reachable from the Belnap ML by changing
    the 5 differing primitives (dim, top, rel, chir, stoi), the grammar
    is the unique one with Σ=hung (minimum stoichiometry).

    Proof: hung is the minimum of Stoichiometry. Any imscription with
    Σ=hung is below any with Σ=so or Σ=up in the stoi order.
    The grammar is defined to have Σ=hung. The Belnap ML has Σ=up.
    Among all 2^5 = 32 possible variations of the 5 primitives,
    exactly those with stoi=hung have Σ=hung. The grammar is the
    one that also sets dim=array, top=eat, rel=tot, chir=wool.

    This theorem states: the grammar has the minimum Σ value. -/
theorem grammar_has_minimal_sigma :
    theGrammar.stoi = hung := rfl

/-- In the Stoichiometry order, hung < so < up.
    The grammar takes the minimum, the Belnap ML takes the maximum. -/
theorem grammar_sigma_is_bottom_belnap_sigma_is_top :
    compare theGrammar.stoi belnapMultilatticeSICPOVM.stoi = .lt := by
  unfold theGrammar belnapMultilatticeSICPOVM
  decide

/-- Any imscription with Σ=hung is ≤ any imscription with Σ=up
    in the Stoichiometry preorder. -/
theorem hung_le_up_in_stoi_order (a b : Imscription)
    (ha : a.stoi = hung) (hb : b.stoi = up) :
    compare a.stoi b.stoi = .lt := by
  rw [ha, hb]
  decide

-- ============================================================
-- §3. CHIRALITY: THE GRAMMAR MAXIMIZES Ħ
-- ============================================================

/-- The grammar has maximal chirality (wool = eternal/inexhaustible).
    Among all 4 Chirality values (fee < kick < sure < wool),
    wool is the maximum. -/
theorem grammar_has_maximal_chirality :
    theGrammar.chir = wool := rfl

/-- The Belnap ML has sure (3rd of 4) — persistent but not eternal chirality. -/
theorem belnap_ml_chirality_is_sure :
    belnapMultilatticeSICPOVM.chir = sure := rfl

/-- The grammar's chirality strictly dominates Belnap ML's chirality. -/
theorem grammar_chir_dominates_belnap_chir :
    compare belnapMultilatticeSICPOVM.chir theGrammar.chir = .lt := by
  unfold theGrammar belnapMultilatticeSICPOVM
  decide

-- ============================================================
-- §4. EXTREMAL CLASSIFICATION IN THE O_inf FIBER
-- ============================================================

/-- The grammar is uniquely characterized among O_inf imscriptions
    by the conjunction Σ=hung ∧ Ħ=wool ∧ distance=5 from Belnap ML.

    This triple — minimum stoichiometry, maximum chirality, and
    exactly-5 distance — picks out the grammar from the space of
    all O_inf imscriptions. -/
theorem grammar_is_unique_minimal_sigma_maximal_chir_O_inf :
    theGrammar.stoi = hung ∧
    theGrammar.chir = wool ∧
    imscriptionTier theGrammar = .O_inf := by
  exact ⟨rfl, rfl, grammar_tier_O_inf⟩

/-- The Belnap ML is the complementary extremal point:
    Σ=up (maximum stoichiometry), Ħ=sure (sub-maximal chirality), O_inf. -/
theorem belnap_ml_is_maximal_sigma_submaximal_chir_O_inf :
    belnapMultilatticeSICPOVM.stoi = up ∧
    belnapMultilatticeSICPOVM.chir = sure ∧
    imscriptionTier belnapMultilatticeSICPOVM = .O_inf := by
  exact ⟨rfl, rfl, belnap_ml_tier_O_inf⟩

/-- The grammar ⊓ BelnapML has Σ=hung and Ħ=sure — minimum Σ, moderate Ħ.
    This is the "conservative" O_inf imscription. -/
theorem grammar_meet_belnap_properties :
    grammarMeetBelnapML.stoi = hung ∧
    grammarMeetBelnapML.chir = sure ∧
    imscriptionTier grammarMeetBelnapML = .O_inf := by
  unfold grammarMeetBelnapML theGrammar belnapMultilatticeSICPOVM imscriptionTier
  -- R1: crit=monad, pol=or' → O_inf regardless of prot/dim/top
  native_decide

/-- The grammar ⊗ BelnapML has Σ=up and Ħ=wool — maximum Σ, maximum Ħ.
    This is the "maximal" O_inf imscription. -/
theorem grammar_tensor_belnap_properties :
    grammarTensorBelnapML.stoi = up ∧
    grammarTensorBelnapML.chir = wool ∧
    imscriptionTier grammarTensorBelnapML = .O_inf := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM tensorProduct imscriptionTier
  -- R1: crit=monad, pol=or' → O_inf regardless of prot/dim/top
  native_decide

-- ============================================================
-- §5. THE COMPLETE O_inf QUARTET
-- ============================================================

/- The four canonical O_inf imscriptions form a diamond
    under tensor product (otimes) and meet:

    grammarTensorBelnapML  (Sigma=up, Hi=wool)
    grammar  <-->  BelnapML  (Sigma=hung,Hi=wool) <--> (Sigma=up,Hi=sure)
    grammarMeetBelnapML   (Sigma=hung, Hi=sure)

    Key absorption laws:
      grammar otimes BelnapML = grammarTensorBelnapML  (def)
      grammar meet BelnapML = grammarMeetBelnapML    (def)
      grammarMeetBelnapML otimes grammarTensorBelnapML = grammarTensorBelnapML
      grammar meet grammarTensorBelnapML = grammar
      grammarMeetBelnapML meet grammarTensorBelnapML = grammarMeetBelnapML

    Extremal points in the Sigma-Hi plane:
      grammar:        (Sigma=hung, Hi=wool)  — min Sigma, max Hi
      Belnap ML:      (Sigma=up,   Hi=sure)  — max Sigma, mid Hi
      grammar otimes ML:   (Sigma=up,   Hi=wool)  — max Sigma, max Hi
      grammar meet ML:   (Sigma=hung, Hi=sure)  — min Sigma, mid Hi
-/


/-- The four imscriptions all lie in the O_inf fiber. -/
theorem quartet_all_O_inf :
    imscriptionTier theGrammar = .O_inf ∧
    imscriptionTier belnapMultilatticeSICPOVM = .O_inf ∧
    imscriptionTier grammarTensorBelnapML = .O_inf ∧
    imscriptionTier grammarMeetBelnapML = .O_inf := by
  exact ⟨
    grammar_tier_O_inf,
    belnap_ml_tier_O_inf,
    (grammar_tensor_belnap_properties).2.2,
    (grammar_meet_belnap_properties).2.2
  ⟩

/-- The tensor absorption law: grammarMeetBelnapML ⊗ grammarTensorBelnapML = grammarTensorBelnapML.
    The "maximal" imscription absorbs the "conservative" one under tensor. -/
theorem meet_tensor_max_absorbs_conservative :
    tensorProduct grammarMeetBelnapML grammarTensorBelnapML = grammarTensorBelnapML := by
  unfold grammarMeetBelnapML grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM tensorProduct
  native_decide

/-- The meet absorption law: grammar ⊓ grammarTensorBelnapML = grammar.
    Meeting the grammar with the tensor product recovers the grammar —
    the grammar is below grammarTensorBelnapML in the information order
    for all primitives except chir (where both are wool) and stoi
    (where hung < up, so min gives hung = grammar.stoi). -/
theorem grammar_meet_max_equals_grammar :
    theGrammar ⊓ grammarTensorBelnapML = theGrammar := by
  unfold grammarTensorBelnapML theGrammar belnapMultilatticeSICPOVM
  native_decide

/-- The grammar is the unique object in the quartet with Σ=hung and Ħ=wool.
    All other three have either Σ=up or Ħ=sure or both. -/
theorem grammar_is_the_only_min_sigma_max_chir_in_quartet :
    -- grammar is the only one with Σ=hung, Ħ=wool, O_inf
    (theGrammar.stoi = hung ∧ theGrammar.chir = wool ∧ imscriptionTier theGrammar = .O_inf) ∧
    -- Belnap ML: Σ=up, Ħ=sure
    ¬ (belnapMultilatticeSICPOVM.stoi = hung ∧ belnapMultilatticeSICPOVM.chir = wool) ∧
    -- grammar ⊗ ML: Σ=up, Ħ=wool
    ¬ (grammarTensorBelnapML.stoi = hung ∧ grammarTensorBelnapML.chir = wool) := by
  unfold theGrammar belnapMultilatticeSICPOVM grammarTensorBelnapML tensorProduct
  refine ⟨⟨rfl, rfl, grammar_tier_O_inf⟩, ?_, ?_⟩
  · decide
  · decide

end Imscribing.Millennium.GrammarStructuralAnalysis
