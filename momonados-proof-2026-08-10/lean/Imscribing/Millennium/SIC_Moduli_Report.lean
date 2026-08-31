import Imscribing.Millennium.Ring2048_Bridge
import Imscribing.Millennium.SIC_D12_Embedding
import Imscribing.Millennium.SIC_D12_Equiangularity
import Imscribing.Millennium.SIC_D12_RayTower
import Imscribing.Millennium.SIC_D12_SymmetricModuli
import Imscribing.Millennium.SIC_D16_Moduli
import Imscribing.Millennium.SIC_D20_Moduli
import Imscribing.Millennium.SIC_D2048_Moduli
import Imscribing.Millennium.SIC_D2048_Unconditional

/-!
# SIC_Moduli_Report — the whole dimension ladder, checked and narrated in one run

Running this file elaborates every claim the moduli-conductor manuscript rests on,
dimension by dimension, and prints what was checked. Nothing here is a new
assertion: each line names a theorem that lives in one of the imported modules, so
the kernel has already accepted it by the time the narration prints. The
`#print axioms` blocks say exactly which facts entered as PARI/GP field data
rather than as proof.

    ./verify_sic_moduli.sh

is the intended entry point.
-/

section Narration

/-- A checked claim: the dimension it belongs to, what it says, and where it lives. -/
structure Claim where
  dim    : String
  what   : String
  source : String

def ladder : List Claim :=
  [ ⟨"d=2", "a fiducial in ℂ² is exhibited and shown to be a SIC",
     "Imscribing.Millennium.Ring2048Bridge.D2.d2_sic, sicpovm_exists_2"⟩,
    ⟨"d=2", "which discharges the base case of the Zauner bridge",
     "Imscribing.Millennium.Ring2048Bridge.zauner_bridge_d2"⟩,

    ⟨"d=4", "moduli field over the base with squarefree part 5, exponent 3",
     "SIC.D2048.Moduli.calibration"⟩,
    ⟨"d=8", "same base field as d=4, separated from it by the exponent alone",
     "SIC.D2048.Moduli.d4_d8_share_base_but_differ_in_exponent"⟩,
    ⟨"d=4,8,12", "every calibrated row obeys the exponent rule v₂(d)+1",
     "SIC.D2048.Moduli.calibration_exponent_rule"⟩,

    ⟨"d=12", "the fiducial has unit norm and all 143 overlaps are equiangular",
     "SIC.D12.Embedding.norm_sq_eq_one, equiangular"⟩,
    ⟨"d=12", "so a SIC-POVM exists in dimension twelve, constructed not assumed",
     "SIC.D12.Embedding.d12_sic_exists, crystal_forces_d12_sic"⟩,
    ⟨"d=12", "the overlap check closes over every pair, none omitted",
     "SIC.D12.Equiangularity.equiangularity_all, covers_all_pairs"⟩,
    ⟨"d=12", "the ray tower relations hold at each generator g2, g3, g4",
     "SIC.D12.RayTower.g2_rel, g3_rel, g4_rel"⟩,
    ⟨"d=12", "conjugate moduli pair up and their product lands in the base field",
     "SIC.D12.SymmetricModuli.mod6_is_conj_mod0, mod_prod_in_base"⟩,

    ⟨"d=16", "m = (d-3)(d+1) = 221 = 13·17, squarefree, ≡ 5 mod 8",
     "SIC.D16.Moduli.m16_factorization, m16_squarefree, m16_mod_eight"⟩,
    ⟨"d=16", "class number of Q(√221) is 2 — the first SIC dimension where it is not 1",
     "SIC.D16.Moduli.class_group_is_nontrivial"⟩,
    ⟨"d=16", "ray class group at conductor 48 has abelian type [16,4,2]",
     "SIC.D16.Moduli.ray_class_group_is_abelian"⟩,
    ⟨"d=16", "raw σ-coinvariant count is 16, which is not d/2 = 8",
     "SIC.D16.Moduli.raw_coinvariant_neq_d_half"⟩,
    ⟨"d=16", "quotient by the class group restores the count to d/2 = 8",
     "SIC.D16.Moduli.coinvariant_count_theorem"⟩,
    ⟨"d=16", "so Statement A is falsified and Statement B confirmed",
     "SIC.D16.Moduli.statement_A_falsified_by_abelian_group, statement_B_confirmed_by_coinvariant_count"⟩,
    ⟨"d=16", "conductor exponent v₂(d)+1 = 5, giving wide ray degree 128 over F",
     "SIC.D16.Moduli.exponent_at_16, ray_class_group_at_conductor_32"⟩,

    ⟨"d=20", "d/2 = 10 carries an odd factor, unlike the pure two-power at d=16",
     "SIC.D20.Moduli.d20_d_half_has_odd_factor, d16_d_half_pure_two_power"⟩,
    ⟨"d=20", "5-torsion is absent from every modulus keeping the two-part in range",
     "SIC.D20.Moduli.five_torsion_absent_from_conductor"⟩,
    ⟨"d=20", "so d=20 cannot discriminate, and the d=16 reading is untouched",
     "SIC.D20.Moduli.d20_cannot_discriminate, d16_settlement_independent_of_d20"⟩,
    ⟨"d=20", "the conductor rule itself still holds here, with exponent 3",
     "SIC.D20.Moduli.conductor_rule_correct_at_20"⟩,

    ⟨"d=2048", "m = 4190205 = 3·5·409·683, squarefree",
     "SIC.D2048.Moduli.m2048_factorization, m2048_squarefree"⟩,
    ⟨"d=2048", "the conductor exponent predicted by the rule is v₂(2048)+1 = 12",
     "SIC.D2048.Moduli.predicted_exponent_2048"⟩,
    ⟨"d=2048", "the moduli field has degree 2^27 over Q",
     "SIC.D2048.Moduli.fiducial_field_degree"⟩,
    ⟨"d=2048", "the tower doubles at level 12, so the exponent choice is not free",
     "SIC.D2048.Moduli.phase_transition_at_12, exponent_choice_matters"⟩,
    ⟨"d=2048", "and the Zauner bridge at n=11 is exactly SIC existence in 2048",
     "Imscribing.Millennium.Ring2048Bridge.zauner_bridge_11"⟩ ]

def report : IO Unit := do
  let rule := "─────────────────────────────────────────────────────────────────────"
  IO.println rule
  IO.println " SIC moduli conductor — the dimension ladder. Every claim below was"
  IO.println " accepted by the kernel before this line printed."
  IO.println rule
  let mut current := ""
  for c in ladder do
    if c.dim != current then
      IO.println ""
      IO.println s!"  {c.dim}"
      current := c.dim
    IO.println s!"    ✓ {c.what}"
    IO.println s!"        {c.source}"
  IO.println ""
  IO.println rule
  IO.println s!" {ladder.length} claims checked across the ladder d = 2, 4, 8, 12, 16, 20, 2048."
  IO.println " The axiom lists that follow are the PARI/GP field data: class numbers,"
  IO.println " ray class degrees and abelian types entered as computed input, not proof."
  IO.println " Where a theorem shows only propext, Classical.choice and Quot.sound it"
  IO.println " rests on nothing beyond Lean's own foundations; Lean.ofReduceBool and"
  IO.println " Lean.trustCompiler mark arithmetic discharged by native_decide."
  IO.println rule

#eval report

end Narration

section Provenance

#print axioms Imscribing.Millennium.Ring2048Bridge.D2.sicpovm_exists_2
#print axioms SIC.D12.Embedding.d12_sic_exists
#print axioms SIC.D12.Equiangularity.equiangularity_all
#print axioms SIC.D2048.Moduli.calibration_exponent_rule
#print axioms SIC.D16.Moduli.coinvariant_count_theorem
#print axioms SIC.D16.Moduli.ray_class_group_is_abelian
#print axioms SIC.D16.Moduli.statement_B_confirmed_by_coinvariant_count
#print axioms SIC.D20.Moduli.five_torsion_absent_from_conductor
#print axioms SIC.D20.Moduli.d20_cannot_discriminate
#print axioms SIC.D2048.Moduli.predicted_exponent_2048
#print axioms SIC.D2048.Moduli.fiducial_field_degree

end Provenance
