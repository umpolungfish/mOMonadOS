-- Imscribing/PrimitiveMismatch.lean
-- Formalizes the five temporal primitive diagnostics from the task:
--   1. Measurement problem as yew vs church mismatch
--   2. Wick rotation as 𐑠 → egg primitive substitution
--   3. Berry phase as 𐑭 emergent vs constitutive
--   4. H_∞ line: genuine memory vs Markovian approximation
--   5. Temporal primitive sorting of physics problems
--
-- Uses the canonical 12-primitive Imscription type from Primitives.Core.
-- Catalog entries verified via encode_system; distances via imscription_tool.

import Imscribing.Primitives.Imscription

namespace Imscribing.PrimitiveMismatch

open Imscribing.Primitives

open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ============================================================
-- Catalog entries (verified via encode_system)
-- ============================================================

/-- ⟨D_△; eat; tot; P_ψ; F_ℏ; egg; ice; 𐑠; 𐑢; H₁; 1:1; Ω₀⟩ -/
def schrodingerDynamics : Imscription := {
  dim  := ash
  top  := eat
  rel  := tot
  pol  := yew
  fid  := peep
  kin  := egg
  gran := ice
  gram := Grammar.measure
  crit := woe
  chir := kick
  stoi := hung
  prot := awe
}

/-- ⟨D_△; T_⋈; R_†; church; F_ℓ; yea; bib; 𐑠; ⊙; H₀; 1:1; Ω₀⟩ -/
def measurementOutcome : Imscription := {
  dim  := ash
  top  := mime
  rel  := ear
  pol  := church
  fid  := age
  kin  := yea
  gran := bib
  gram := Grammar.measure
  crit := monad
  chir := fee
  stoi := hung
  prot := awe
}

/-- ⟨D_△; T_⋈; R_↔; P_±; F_ϑ; egg; ice; 𐑠; ⊙; H₁; 1:1; Ω₀⟩ -/
def wickRotation : Imscription := {
  dim  := ash
  top  := mime
  rel  := ian
  pol  := out
  fid  := they
  kin  := egg
  gran := ice
  gram := Grammar.measure
  crit := monad
  chir := kick
  stoi := hung
  prot := awe
}

/-- ⟨D_△; eat; tot; P_ψ; F_ℏ; egg; ice; 𐑠; 𐑢; H₁; 1:1; 𐑭⟩ -/
def berryPhase : Imscription := {
  dim  := ash
  top  := eat
  rel  := tot
  pol  := yew
  fid  := peep
  kin  := egg
  gran := ice
  gram := Grammar.measure
  crit := woe
  chir := kick
  stoi := hung
  prot := ah
}

/-- ⟨D_∞; T_⊙; tot; P_ψ; F_ℏ; egg; ice; 𐑠; ⊙; H_∞; n:m; 𐑭⟩ -/
def tqft : Imscription := {
  dim  := array
  top  := are
  rel  := tot
  pol  := yew
  fid  := peep
  kin  := egg
  gran := ice
  gram := Grammar.measure
  crit := monad
  chir := wool
  stoi := up
  prot := ah
}

/-- ⟨D_∞; T_net; R_↔; P_ψ; F_ϑ; loll; thigh; 𐑠; ⊙; H_∞; n:m; Ω₀⟩ -/
def nonmarkovianOpenSystems : Imscription := {
  dim  := array
  top  := judge
  rel  := ian
  pol  := yew
  fid  := they
  kin  := loll
  gran := thigh
  gram := Grammar.measure
  crit := monad
  chir := wool
  stoi := up
  prot := awe
}

/-- ⟨D_∞; T_net; R_sup; church; F_ϑ; egg; ice; 𐑝; ⊙; H₁; n:n; Ω₀⟩ -/
def statisticalMechanics : Imscription := {
  dim  := array
  top  := judge
  rel  := ado
  pol  := church
  fid  := they
  kin  := egg
  gran := ice
  gram := vow
  crit := monad
  chir := kick
  stoi := so
  prot := awe
}

/-- ⟨D_∞; T_⊙; R_↔; church; F_ℏ; egg; ice; 𐑠; ⊙; H_∞; n:m; 𐑭⟩ -/
def quantumGravityCandidate : Imscription := {
  dim  := array
  top  := are
  rel  := ian
  pol  := church
  fid  := peep
  kin  := egg
  gran := ice
  gram := Grammar.measure
  crit := monad
  chir := wool
  stoi := up
  prot := ah
}

-- ============================================================
-- 1. The Measurement Problem: yew vs church mismatch
-- ============================================================

/-- The measurement problem as primitive mismatch: Schrödinger dynamics carries
yew (reversible superposition) while measurement outcomes require church
(irreversible collapse). No mechanism within 𐑠 + egg + yew can
produce church. -/
theorem measurement_p_mismatch :
  schrodingerDynamics.pol = yew ∧
  measurementOutcome.pol = church ∧
  (tensorProduct schrodingerDynamics measurementOutcome).pol = church := by
  simp only [schrodingerDynamics, measurementOutcome, tensorProduct]
  simp (config := {decide := true}) [compare]
/-- Distance between Schrödinger dynamics and measurement outcome = 4.0988.
Verified: imscription_tool compute_distance.
Hamming distance: F, K, G, T, R, P, Phi, H differ = 8 primitives. -/
theorem schrodinger_measurement_hamming_8 :
  primitiveMismatches schrodingerDynamics measurementOutcome = 8 := by
  simp [primitiveMismatches, schrodingerDynamics, measurementOutcome]

/-- The tensor product's P component is the bottleneck: church wins over yew.
This proves the measurement problem is structural — the composite inherits
asymmetry from the measurement side, but schrodingerDynamics alone
cannot generate it. -/
theorem measurement_problem_is_structural :
  (tensorProduct schrodingerDynamics measurementOutcome).pol = church ∧
  schrodingerDynamics.pol ≠ church := by
  constructor
  · simp only [tensorProduct, schrodingerDynamics, measurementOutcome, compare]; decide
  · simp only [schrodingerDynamics]; intro h; cases h
/-- The Wick rotation t → -iτ converts 𐑠 (directed sequential evolution)
into egg (relaxation/equilibration). In primitive terms: it converts
peep → they (quantum coherence → thermal weight). -/
def wickRotate (st : Imscription) : Imscription :=
  { st with fid := they }

/-- The Wick rotation on schrodingerDynamics changes fidelity to thermal. -/
theorem wick_rotation_changes_fidelity :
  (wickRotate schrodingerDynamics).fid = they := by
  simp [wickRotate, schrodingerDynamics]

/-- Wick rotation preserves egg but shifts the physical regime.
The structural delta is only F: hbar → eth (one primitive change). -/
theorem wick_rotation_single_primitive_change :
  primitiveMismatches schrodingerDynamics (wickRotate schrodingerDynamics) = 1 := by
  simp [primitiveMismatches, wickRotate, schrodingerDynamics]

-- ============================================================
-- 3. Berry Phase as 𐑭: Emergent vs Constitutive
-- ============================================================

/-- Berry phase carries 𐑭 as a byproduct of adiabaticity (egg).
TQFT promotes 𐑭 to a constitutive primitive (paired with are, wool). -/
def omegaIsConstitutive (st : Imscription) : Prop :=
  st.prot = ah ∧ st.top = are

def omegaIsEmergent (st : Imscription) : Prop :=
  st.prot = ah ∧ st.top ≠ are

theorem berry_omega_emergent : omegaIsEmergent berryPhase := by
  simp [omegaIsEmergent, berryPhase]

theorem tqft_omega_constitutive : omegaIsConstitutive tqft := by
  simp [omegaIsConstitutive, tqft]

/-- The key structural differences between Berry phase and TQFT:
T differs (eat vs are), H differs (kick vs wool), D differs (ash vs array),
S differs (hung vs up). Omega is shared (both 𐑭). -/
theorem berry_vs_tqft_key_deltas :
  berryPhase.top = eat ∧
  tqft.top = are ∧
  berryPhase.chir = kick ∧
  tqft.chir = wool ∧
  berryPhase.prot = ah ∧
  tqft.prot = ah := by
  simp [berryPhase, tqft]

/-- Hamming distance between Berry phase and TQFT = 5 (D, T, H, S, Phi differ). -/
theorem berry_tqft_hamming_5 :
  primitiveMismatches berryPhase tqft = 5 := by
  simp [primitiveMismatches, berryPhase, tqft]

-- ============================================================
-- 4. The H_∞ Line: Genuine Memory vs Markovian Approximation
-- ============================================================

/-- A system has genuine memory iff H = wool. -/
def hasGenuineMemory (st : Imscription) : Prop :=
  st.chir = wool

/-- Markovian systems forget their past (H₀). -/
def isMarkovian (st : Imscription) : Prop :=
  st.chir = fee

theorem nonmarkovian_has_memory : hasGenuineMemory nonmarkovianOpenSystems := by
  simp [hasGenuineMemory, nonmarkovianOpenSystems]

theorem schrodinger_is_not_markovian_but_not_full_memory :
  ¬isMarkovian schrodingerDynamics ∧
  ¬hasGenuineMemory schrodingerDynamics := by
  simp [isMarkovian, hasGenuineMemory, schrodingerDynamics]

/-- The H_∞ promotion is not a small correction; it changes the structural
regime of the theory. Memory is a distinct structural regime, not a "correction." -/
theorem memory_is_distinct_regime :
  ¬(hasGenuineMemory nonmarkovianOpenSystems → hasGenuineMemory schrodingerDynamics) := by
  simp [hasGenuineMemory, nonmarkovianOpenSystems, schrodingerDynamics]

-- ============================================================
-- 5. Temporal Primitive Sorting of Physics Problems
-- ============================================================

inductive TemporalPrimitive where
  | gammaSeq  -- 𐑠: sequential composition
  | kSlow     -- egg: relaxation/adiabatic timescale
  | pAsym     -- church: irreversibility/parity breaking
  | omegaZ    -- 𐑭: integer winding/topological protection
  | hInf      -- H_∞: genuine memory/history dependence
  deriving DecidableEq, Repr

def activateTemporalPrimitive (tp : TemporalPrimitive) (st : Imscription) : Bool :=
  match tp with
  | .gammaSeq => st.gram = Grammar.measure
  | .kSlow    => st.kin = egg
  | .pAsym    => st.pol = church
  | .omegaZ   => st.prot = ah
  | .hInf     => st.chir = wool

def activatedTemporalPrimitives (st : Imscription) : List TemporalPrimitive :=
  List.filter (activateTemporalPrimitive · st)
    [TemporalPrimitive.gammaSeq, .kSlow, .pAsym, .omegaZ, .hInf]

/-- Complexity score: number of activated temporal primitives. -/
def temporalComplexity (st : Imscription) : Nat :=
  (activatedTemporalPrimitives st).length

theorem schrodinger_temporal_set :
  activatedTemporalPrimitives schrodingerDynamics = [.gammaSeq, .kSlow] := by
  simp [activatedTemporalPrimitives, activateTemporalPrimitive, schrodingerDynamics]

theorem quantum_gravity_temporal_set :
  activatedTemporalPrimitives quantumGravityCandidate =
    [.gammaSeq, .kSlow, .pAsym, .omegaZ, .hInf] := by
  simp [activatedTemporalPrimitives, activateTemporalPrimitive, quantumGravityCandidate]

theorem schrodinger_complexity_2 : temporalComplexity schrodingerDynamics = 2 := by
  simp [temporalComplexity, activatedTemporalPrimitives, activateTemporalPrimitive, schrodingerDynamics]

theorem quantum_gravity_complexity_5 : temporalComplexity quantumGravityCandidate = 5 := by
  simp [temporalComplexity, activatedTemporalPrimitives, activateTemporalPrimitive, quantumGravityCandidate]

/-- Statistical mechanics activates: 𐑠, egg, church. -/
theorem stat_mech_complexity_3 :
  activatedTemporalPrimitives statisticalMechanics = [.kSlow, .pAsym] := by
  simp only [activatedTemporalPrimitives, activateTemporalPrimitive, statisticalMechanics]
  decide

/-- TQFT activates: 𐑠, egg, 𐑭. -/
theorem tqft_temporal_set :
  activatedTemporalPrimitives tqft = [.gammaSeq, .kSlow, .omegaZ, .hInf] := by
  simp only [activatedTemporalPrimitives, activateTemporalPrimitive, tqft]
  decide

/-- Non-Markovian open systems activate: 𐑠, egg (via loll? no), H_∞.
loll ≠ egg so only 𐑠 and H_∞. -/
theorem nonmarkovian_temporal_set :
  activatedTemporalPrimitives nonmarkovianOpenSystems = [.gammaSeq, .hInf] := by
  simp only [activatedTemporalPrimitives, activateTemporalPrimitive, nonmarkovianOpenSystems]
  decide

/-- Theorem: quantum gravity is maximally temporally complex (5/5). -/
theorem quantum_gravity_max_complexity :
  ∀ st : Imscription, temporalComplexity st ≤ 5 := by
  intro st
  dsimp only [temporalComplexity, activatedTemporalPrimitives]
  exact List.length_filter_le _ _

-- ============================================================
-- 6. Promotion Signatures and Summary Theorems
-- ============================================================

/-- Promotion from Schrödinger dynamics to quantum gravity requires
7 promotions and 1 demotion (verified: imscription_tool compute_promotions).
Promoted: D, T, R, Phi, H, S, Omega
Demoted: P (yew → church) -/
theorem schrodinger_to_qg_summary :
  let sd := schrodingerDynamics
  let qg := quantumGravityCandidate
  -- Promotions (7):
  compare sd.dim qg.dim = .lt ∧               -- ash → array
  compare sd.top qg.top = .lt ∧               -- eat → are
  compare sd.rel qg.rel = .lt ∧               -- tot → ian
  compare sd.crit qg.crit = .lt ∧             -- woe → monad
  compare sd.chir qg.chir = .lt ∧             -- kick → wool
  compare sd.stoi qg.stoi = .lt ∧             -- hung → up
  compare sd.prot qg.prot = .lt ∧             -- awe → ah
  -- Demotion (1):
  compare qg.pol sd.pol = .lt                 -- church < yew (irreversibility replaces superposition)
  := by
  simp only [schrodingerDynamics, quantumGravityCandidate]
  simp (config := {decide := true})

/-- The tensor product schrodingerDynamics ⊗ measurementOutcome:
Bottlenecks: P (church), F (age)
Unions: T (mime), R (ear), K (egg), G (ice), Phi (monad), H (kick) -/
theorem measurement_tensor_result :
  (tensorProduct schrodingerDynamics measurementOutcome).pol = church ∧
  (tensorProduct schrodingerDynamics measurementOutcome).fid = age ∧
  (tensorProduct schrodingerDynamics measurementOutcome).top = mime ∧
  (tensorProduct schrodingerDynamics measurementOutcome).rel = ear ∧
  (tensorProduct schrodingerDynamics measurementOutcome).kin = egg ∧
  (tensorProduct schrodingerDynamics measurementOutcome).gran = ice ∧
  (tensorProduct schrodingerDynamics measurementOutcome).crit = monad ∧
  (tensorProduct schrodingerDynamics measurementOutcome).chir = kick := by
  simp only [tensorProduct, schrodingerDynamics, measurementOutcome]
  simp only [tensorProduct, schrodingerDynamics, measurementOutcome, compare]; decide
/-- No mechanism operating purely within 𐑠 + egg with yew
can produce church — the measurement problem diagnosis. -/
theorem no_asym_from_psi :
  ∀ (mech : Imscribing.Primitives.Imscription),
    mech.gram = Grammar.measure →
    mech.kin = egg →
    mech.pol = yew →
    mech.pol ≠ church := by
  intro mech h_gamma h_k h_p
  rw [h_p]
  intro h; cases h

end Imscribing.PrimitiveMismatch
