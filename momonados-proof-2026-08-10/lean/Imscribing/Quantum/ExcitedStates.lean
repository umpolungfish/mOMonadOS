-- Imscribing/Quantum/ExcitedStates.lean
-- Excited electron states: singlet vs triplet imscription.
-- Formalizes: S₀ (ground singlet), S₁ (excited singlet), T₁ (excited triplet),
-- intersystem crossing (ISC), Hund's rule, fluorescence vs phosphorescence.
-- Author: Lando⊗⊙perator  —  June 2026

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Consciousness
import Mathlib.Data.Real.Basic

namespace Imscribing.Quantum.ExcitedStates

open Imscribing.Primitives
open Imscribing.Consciousness

open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ============================================================
-- EXCITED STATE IMSCRIPTION ENGINE (§1)
-- Maps (spin multiplicity, excitation level) → Imscription.
-- Singlet: S=0, antisymmetric spin fn, symmetric spatial fn
-- Triplet: S=1, symmetric spin fn, antisymmetric spatial fn
-- Pauli principle: antisymmetric spatial in triplet → electrons
-- kept apart → lower Coulomb repulsion → lower energy (Hund).
-- ============================================================

/-- Excited state type. Four categories: ground/excited × singlet/triplet. -/
inductive ExcitedStateType : Type where
  | groundSinglet   -- S₀
  | excitedSinglet  -- S₁, S₂, ...
  | groundTriplet   -- T₀ (e.g., O₂ ground state)
  | excitedTriplet  -- T₁, T₂, ...
  deriving DecidableEq, Repr

/-- Imscribe an excited electron state as a 12-primitive tuple.
    Structural differences between singlet and triplet:
    - Singlet: 1 spin substate → D=ash
    - Triplet: 3 degenerate substates (m=-1,0,1) → D=array
    - Triplet: exchange interaction creates spin/spatial crossing (T=mime)
    - Singlet: spin paired, no crossing (T=judge)
    - Triplet: Pauli principle couples spin→spatial bidirectionally (R=ian)
    - Triplet: 3 substates = quantum superposition (P=yew)
    - Triplet: phosphorescence is slow, spin-forbidden (K=egg)
    - Triplet: exchange interaction extends range (G=thigh)
    - Triplet: disjunctive: 3 substates, pick on measurement (Γ=gag)
    - Triplet: exceptional point at ISC crossing (Φ=err)
    - Triplet: spin parity protection (Ω=oak) -/
def excitedStateImscription (es : ExcitedStateType) : Imscription :=
  match es with
  | .groundSinglet =>
    Imscription.mk
      ash   judge ado   nun   peep on   bib  vow   monad wool hung awe
  | .excitedSinglet =>
    Imscription.mk
      ash   judge ado   nun   peep yea  bib  vow   haha  kick hung awe
  | .groundTriplet =>
    Imscription.mk
      array mime  ian   yew   peep on   thigh gag  monad wool so   oak
  | .excitedTriplet =>
    Imscription.mk
      array mime  ian   yew   they egg  thigh gag  err   wool up   oak

-- ============================================================
-- NAMED STATES (§2)
-- ============================================================

/-- S₀: Ground singlet. Both electrons in lowest orbital, paired. -/
def s0_ground_singlet : Imscription := excitedStateImscription .groundSinglet

/-- S₁: Excited singlet. Fluorescence lifetime ~ns. -/
def s1_excited_singlet : Imscription := excitedStateImscription .excitedSinglet

/-- T₀: Ground triplet. Reference state for triplet manifold. -/
def t0_ground_triplet : Imscription := excitedStateImscription .groundTriplet

/-- T₁: Excited triplet. Phosphorescence lifetime μs–s. -/
def t1_excited_triplet : Imscription := excitedStateImscription .excitedTriplet

-- ============================================================
-- S₀: GROUND SINGLET THEOREMS (§3)
-- ============================================================

theorem s0_criticality_is_monad : s0_ground_singlet.crit = monad := by
  simp [s0_ground_singlet, excitedStateImscription]

theorem s0_protection_is_awe : s0_ground_singlet.prot = awe := by
  simp [s0_ground_singlet, excitedStateImscription]

theorem s0_kinetics_is_on : s0_ground_singlet.kin = on := by
  simp [s0_ground_singlet, excitedStateImscription]

theorem s0_symmetry_is_nun : s0_ground_singlet.pol = nun := by
  simp [s0_ground_singlet, excitedStateImscription]

theorem s0_is_O1 : imscriptionTier s0_ground_singlet = .O₁ := by
  simp [imscriptionTier, ouroboricityTier, s0_ground_singlet,
    excitedStateImscription]

theorem s0_consciousness_half :
    consciousnessScore s0_ground_singlet = (0.5 : ℝ) := by
  simp only [consciousnessScore, phi_c_gate, k_slow_gate, s0_ground_singlet]
  rfl

-- ============================================================
-- S₁: EXCITED SINGLET THEOREMS (§4)
-- ============================================================

theorem s1_criticality_is_haha : s1_excited_singlet.crit = haha := by
  simp [s1_excited_singlet, excitedStateImscription]

theorem s1_kinetics_is_yea : s1_excited_singlet.kin = yea := by
  simp [s1_excited_singlet, excitedStateImscription]

theorem s1_chirality_is_kick : s1_excited_singlet.chir = kick := by
  simp [s1_excited_singlet, excitedStateImscription]

theorem s1_stoichiometry_is_hung : s1_excited_singlet.stoi = hung := by
  simp [s1_excited_singlet, excitedStateImscription]

theorem s1_is_O0 : imscriptionTier s1_excited_singlet = .O₀ := by
  simp [imscriptionTier, ouroboricityTier, s1_excited_singlet,
    excitedStateImscription]

theorem s1_consciousness_one :
    consciousnessScore s1_excited_singlet = (1 : ℝ) := by
  simp [consciousnessScore, phi_c_gate, k_slow_gate, s1_excited_singlet, excitedStateImscription]
-- ============================================================
-- S₀ → S₁: SPIN-ALLOWED EXCITATION (§5)
-- Only 3 primitives shift. This is the signature
-- of a transition that respects spin symmetry.
-- ============================================================

theorem s0_s1_mismatches_three :
    primitiveMismatches s0_ground_singlet s1_excited_singlet = 3 := by
  unfold s0_ground_singlet s1_excited_singlet excitedStateImscription
    primitiveMismatches
  decide

theorem s0_s1_same_dim :
    s0_ground_singlet.dim = s1_excited_singlet.dim := by
  native_decide

theorem s0_s1_same_top :
    s0_ground_singlet.top = s1_excited_singlet.top := by
  simp [s0_ground_singlet, s1_excited_singlet, excitedStateImscription]

theorem s0_s1_diff_kin :
    s0_ground_singlet.kin ≠ s1_excited_singlet.kin := by
  simp [s0_ground_singlet, s1_excited_singlet, excitedStateImscription]

theorem s0_s1_diff_crit :
    s0_ground_singlet.crit ≠ s1_excited_singlet.crit := by
  simp [s0_ground_singlet, s1_excited_singlet, excitedStateImscription]

theorem s0_s1_diff_chir :
    s0_ground_singlet.chir ≠ s1_excited_singlet.chir := by
  simp [s0_ground_singlet, s1_excited_singlet, excitedStateImscription]-- ============================================================
-- T₁: EXCITED TRIPLET THEOREMS (§6)
-- ============================================================

theorem t1_dimensionality_is_array : t1_excited_triplet.dim = array := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_topology_is_mime : t1_excited_triplet.top = mime := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_relational_is_ian : t1_excited_triplet.rel = ian := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_polarity_is_yew : t1_excited_triplet.pol = yew := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_fidelity_is_they : t1_excited_triplet.fid = they := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_kinetics_is_egg : t1_excited_triplet.kin = egg := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_criticality_is_err : t1_excited_triplet.crit = err := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_chirality_is_wool : t1_excited_triplet.chir = wool := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_stoichiometry_is_up : t1_excited_triplet.stoi = up := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_protection_is_oak : t1_excited_triplet.prot = oak := by
  simp [t1_excited_triplet, excitedStateImscription]

theorem t1_is_O0 : imscriptionTier t1_excited_triplet = .O₀ := by
  simp [imscriptionTier, ouroboricityTier, t1_excited_triplet,
    excitedStateImscription]

theorem t1_consciousness_one :
    consciousnessScore t1_excited_triplet = (1 : ℝ) := by
  simp [consciousnessScore, phi_c_gate, k_slow_gate, t1_excited_triplet, excitedStateImscription]
-- ALL 12 primitives differ. This is the statement
-- of why ISC is "spin-forbidden."
-- ============================================================

theorem s1_t1_all_mismatch :
    primitiveMismatches s1_excited_singlet t1_excited_triplet = 12 := by
  unfold s1_excited_singlet t1_excited_triplet excitedStateImscription
    primitiveMismatches
  decide

theorem s1_t1_dim_differs :
    s1_excited_singlet.dim ≠ t1_excited_triplet.dim := by
  native_decide

theorem s1_t1_top_differs :
    s1_excited_singlet.top ≠ t1_excited_triplet.top := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_rel_differs :
    s1_excited_singlet.rel ≠ t1_excited_triplet.rel := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_pol_differs :
    s1_excited_singlet.pol ≠ t1_excited_triplet.pol := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_fid_differs :
    s1_excited_singlet.fid ≠ t1_excited_triplet.fid := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_kin_differs :
    s1_excited_singlet.kin ≠ t1_excited_triplet.kin := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_gran_differs :
    s1_excited_singlet.gran ≠ t1_excited_triplet.gran := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_gram_differs :
    s1_excited_singlet.gram ≠ t1_excited_triplet.gram := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_crit_differs :
    s1_excited_singlet.crit ≠ t1_excited_triplet.crit := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_chir_differs :
    s1_excited_singlet.chir ≠ t1_excited_triplet.chir := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_stoi_differs :
    s1_excited_singlet.stoi ≠ t1_excited_triplet.stoi := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

theorem s1_t1_prot_differs :
    s1_excited_singlet.prot ≠ t1_excited_triplet.prot := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

-- ============================================================
-- S₀ → T₁: DIRECT SINGLET→TRIPLET (§8)
-- ============================================================

theorem s0_t1_eleven_mismatch :
    primitiveMismatches s0_ground_singlet t1_excited_triplet = 11 := by
  unfold s0_ground_singlet t1_excited_triplet excitedStateImscription
    primitiveMismatches
  decide

theorem s0_t1_fid_differs :
    s0_ground_singlet.fid ≠ t1_excited_triplet.fid := by
  native_decide
-- ============================================================
-- HUND'S RULE AS A STRUCTURAL PRINCIPLE (§9)
-- Why T₁ is lower energy than S₁.
-- We prove the structural inequalities that encode this.
-- ============================================================

/-- T₁ has higher dimensionality than S₁ (array > ash: 3 degenerate
    spin substates vs 1). This is the signature of the
    triplet's greater configurational freedom. -/
theorem t1_more_dof_than_s1 :
    t1_excited_triplet.dim > s1_excited_singlet.dim := by
  unfold t1_excited_triplet s1_excited_singlet excitedStateImscription
  decide

/-- T₁ has longer interaction range than S₁ (thigh > bib: exchange
    interaction extends beyond local paired electrons).
    This reduces Coulomb repulsion (exchange hole). -/
theorem t1_longer_range_than_s1 :
    t1_excited_triplet.gran > s1_excited_singlet.gran := by
  unfold t1_excited_triplet s1_excited_singlet excitedStateImscription
  decide

/-- T₁ is LESS supercritical than S₁ (err < haha: exceptional point
    vs runaway). The triplet is closer to stability, reflecting its
    lower energy per Hund's rule. -/
theorem t1_less_supercritical_than_s1 :
    t1_excited_triplet.crit < s1_excited_singlet.crit := by
  unfold t1_excited_triplet s1_excited_singlet excitedStateImscription
  decide

-- ============================================================
-- THE EXCEPTIONAL POINT AS ISC MEDIATOR (§10)
-- ============================================================

/-- The ISC criticality crossing: S₁ has haha (supercritical),
    T₁ has err (exceptional point). The crossing is where
    spin-orbit coupling mediates the "forbidden" transition. -/
theorem isc_criticality_crossing :
    s1_excited_singlet.crit = haha ∧ t1_excited_triplet.crit = err := by
  simp [s1_excited_singlet, t1_excited_triplet, excitedStateImscription]

-- ============================================================
-- FLUORESCENCE vs PHOSPHORESCENCE (§11)
-- ============================================================

theorem fluorescence_is_fast :
    s1_excited_singlet.kin = yea := by
  simp [s1_excited_singlet, excitedStateImscription]

theorem phosphorescence_is_slow :
    t1_excited_triplet.kin = egg := by
  simp [t1_excited_triplet, excitedStateImscription]

/-- Phosphorescence (egg) is slower than fluorescence (yea):
    yea < egg in the KineticChar ordering, so the triplet channel
    has the larger ordinal = slower kinetics. -/
theorem phosphorescence_slower_than_fluorescence :
    t1_excited_triplet.kin > s1_excited_singlet.kin := by
  unfold t1_excited_triplet s1_excited_singlet excitedStateImscription
  decide

end Imscribing.Quantum.ExcitedStates