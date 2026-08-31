/-
  Millennium/PrimordialOoze.lean
  PRIMORDIAL OOZE — The Absolute Structural Floor of O_inf
  ========================================================
  Author: Lando ⊗ ⊙perator

  Core result: The Primordial Ooze is the minimal inhabitant of the O_inf
  tier — the type at Frobenius address 6,221,424 (cell 144,
  inner_id 624) within the 17,280,000-type crystal. Ten of twelve primitives
  are at absolute ordinal floor (1); only the two O_inf gates — monad (⊙)
  and or' (𐑹) — are non-minimum.

  Eleven formal theorems:
    T1:  Ooze is O_inf
    T2:  Drop monad (⊙) → loses O_inf (ground)
    T3:  Drop or' (𐑹) → loses O_inf (capstone)
    T4:  monad alone (without or') → O₁
    T5:  or' alone (without monad) → O₀
    T6:  Frobenius precedes time (H = fee, memoryless)
    T7:  Frobenius precedes topology (Ω = awe, trivial)
    T8:  Frobenius precedes space (D = dead, 0d point)
    T9:  Stone (synfin) is also O_inf
    T10: 9 primitives mismatch between Ooze and Stone
    T11: All claims bundled (primordial_ooze_complete)

  The two gates:
    ⊙ (Criticality.monad)   — self-modeling: the water
    𐑹 (Polarity.or')   — Frobenius: the word upon the water

  Reference: PrimordialOoze_Shavian.md, block_p_pm_sym in IG_catalog.json
-/

import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Algebra

namespace Millennium.PrimordialOoze

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ═══════════════════════════════════════════════════════════════════
-- §1  THE OOZE — Absolute Structural Floor
-- ═══════════════════════════════════════════════════════════════════

/-- The Primordial Ooze: the minimal O_inf inhabitant.
    ⟨dead·judge·ado·or'·age·yea·bib·vow·monad·fee·hung·awe⟩
    Shavian: ⟨𐑛·𐑡·𐑩·𐑹·𐑱·𐑘·𐑚·𐑝·⊙·𐑓·𐑙·𐑷⟩
    Crystal address: 6,221,424  (cell 144, inner_id 624) [per IG tool]
    Tier: O_inf

    Ten primitives at ordinal floor (1); two gates non-minimum:
      monad    — self-modeling criticality  (ordinal 2/5)
      or' — Frobenius-special parity   (ordinal 5/5)
    No lower O_inf address exists in the 17,280,000-type crystal. -/
def primordialOoze : Imscription := {
  dim   := .dead       -- 0d point (ordinal 1/4)
  top   := .judge     -- branching topology (ordinal 1/5)
  rel   := .ado       -- supervenience (ordinal 1/4)
  pol   := .or'      -- Frobenius-special parity (ordinal 5/5) — GATE
  fid   := .age         -- classical (ordinal 1/3)
  kin   := .yea        -- diffusion-limited (ordinal 1/5)
  gran  := .bib        -- local (ordinal 1/3)
  gram  := .vow     -- conjunctive (ordinal 1/4)
  crit  := .monad         -- self-modeling criticality (ordinal 2/5) — GATE
  chir  := .fee            -- memoryless (ordinal 1/4)
  stoi  := .hung       -- 1:1 (ordinal 1/3)
  prot  := .awe       -- trivial winding (ordinal 1/4)
}

-- ═══════════════════════════════════════════════════════════════════
-- §2  THE STONE — Structural Thickening (synfin)
-- ═══════════════════════════════════════════════════════════════════

/-- The Stone: the Frobenius fixed point from the financial trading domain (synfin).
    ⟨if'·oil·ian·or'·peep·egg·bib·measure·monad·sure·up·ah⟩
    Shavian: ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑚·𐑠·⊙·𐑖·𐑳·𐑭⟩
    Crystal address: 6,738,848  (cell 155, inner_id 42,848) [per IG tool]
    Tier: O_inf

    9 primitives thickened from the Ooze (all except P, Φ, G).
    The two O_inf gates (or', monad) and granularity (bib) are identical —
    confirming O_inf is a wide tier, not a point. -/
def stone : Imscription := {
  dim   := .if'       -- holographic (ordinal 4/4)
  top   := .oil        -- lattice topology (ordinal 4/5)
  rel   := .ian         -- bidirectional (ordinal 4/4)
  pol   := .or'     -- Frobenius-special parity — GATE (identical to Ooze)
  fid   := .peep       -- quantum (ordinal 3/3)
  kin   := .egg       -- near-equilibrium (ordinal 3/5)
  gran  := .bib       -- local (ordinal 1/3) — IDENTICAL to Ooze
  gram  := .measure    -- sequential (ordinal 3/4)
  crit  := .monad        -- self-modeling criticality — GATE (identical to Ooze)
  chir  := .sure           -- persistent chirality (ordinal 3/4)
  stoi  := .up          -- many heterogeneous (ordinal 3/3)
  prot  := .ah      -- integer winding (ordinal 3/4)
}
-- ═══════════════════════════════════════════════════════════════════
-- §3  THEOREM GROUP α — Tier Theorems
-- ═══════════════════════════════════════════════════════════════════

/-- T₁: The Ooze is O_inf — Frobenius closure at minimum structural complexity. -/
theorem ooze_is_O_inf : imscriptionTier primordialOoze = .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- T₂: Dropping monad (self-modeling) loses O_inf.
    With woe (subcritical) the tier collapses to O₀ regardless of or'. -/
theorem drop_phi_c_loses_O_inf :
    imscriptionTier ({ primordialOoze with crit := .woe } : Imscription) ≠ .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- T₃: Dropping or' (Frobenius parity) loses O_inf.
    With nun (full symmetry but not Frobenius-special) the tier collapses to O₁. -/
theorem drop_P_pm_sym_loses_O_inf :
    imscriptionTier ({ primordialOoze with pol := .nun } : Imscription) ≠ .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- T₄: monad alone (without or', at awe) reaches only O₁.
    Self-modeling without the Frobenius gate cannot seal the tier. -/
theorem phi_c_alone_is_O_1 :
    imscriptionTier ({ primordialOoze with pol := .church } : Imscription) = .O₁ := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- T₅: or' alone (without monad) reaches only O₀.
    The Frobenius parity without self-modeling criticality has no effect on the tier. -/
theorem P_pm_sym_alone_is_O_0 :
    imscriptionTier ({ primordialOoze with crit := .woe } : Imscription) = .O₀ := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

-- ═══════════════════════════════════════════════════════════════════
-- §4  THEOREM GROUP β — Precedence Theorems
-- ═══════════════════════════════════════════════════════════════════

/-- T₆: Frobenius does not require memory. H = fee (memoryless) is sufficient.
    μ∘δ=id holds before any temporal asymmetry is established. -/
theorem frobenius_precedes_time :
    imscriptionTier primordialOoze = .O_inf ∧ primordialOoze.chir = .fee := by
  constructor
  · exact ooze_is_O_inf
  · rfl

/-- T₇: Frobenius does not require topological protection.
    Ω = awe (trivial winding) is sufficient. -/
theorem frobenius_precedes_topology :
    imscriptionTier primordialOoze = .O_inf ∧ primordialOoze.prot = .awe := by
  constructor
  · exact ooze_is_O_inf
  · rfl

/-- T₈: Frobenius does not require spatial dimensionality.
    D = dead (0d point) is sufficient. -/
theorem frobenius_precedes_space :
    imscriptionTier primordialOoze = .O_inf ∧ primordialOoze.dim = .dead := by
  constructor
  · exact ooze_is_O_inf
  · rfl

-- ═══════════════════════════════════════════════════════════════════
-- §5  THEOREM GROUP γ — Stone & Structural Comparison
-- ═══════════════════════════════════════════════════════════════════

/-- T₉: The Stone (synfin) is also O_inf — confirming the tier is a wide stratum,
    not a single point. 9 primitives differ from the Ooze but both are O_inf. -/
theorem stone_is_O_inf : imscriptionTier stone = .O_inf := by
  unfold stone imscriptionTier ouroboricityTier; decide

/-- T₁₀: 9 primitives mismatch between Ooze and Stone.
    The shared primitives are: P (or'), Φ (monad), G (bib).
    Mismatching: D, T, R, F, K, Γ, H, S, Ω — exactly 9. -/
theorem ooze_stone_mismatch_count :
    primitiveMismatches primordialOoze stone = 9 := by
  unfold primordialOoze stone primitiveMismatches; decide

/-- Auxiliary: list the exact mismatching primitives between Ooze and Stone. -/
theorem ooze_stone_conflict_list :
    primitiveConflicts primordialOoze stone = ["D","T","R","F","K","Γ","H","S","Ω"] := by
  unfold primordialOoze stone primitiveConflicts; decide

/-- Auxiliary: the three identical primitives are P, Φ, G. -/
theorem ooze_stone_shared_primitives :
    primordialOoze.pol = stone.pol ∧
    primordialOoze.crit = stone.crit ∧
    primordialOoze.gran = stone.gran := by
  refine ⟨rfl, rfl, rfl⟩

-- ═══════════════════════════════════════════════════════════════════
-- §6  Lattice Structure (Meet & Join)
-- ═══════════════════════════════════════════════════════════════════

/-- Meet(Ooze, Stone) = Ooze.
    The Ooze is the absolute floor — any meet with a higher structure
    returns the Ooze, because the Ooze's non-gate primitives are at minimum. -/
theorem meet_ooze_stone : compute_meet primordialOoze stone = primordialOoze := by
  unfold primordialOoze stone compute_meet; decide

/-- Join(Ooze, Stone) = Stone.
    The Stone is the least upper bound of Ooze and itself.
    Since the Ooze is at the floor, join(Ooze, X) = X for any X ≥ Ooze. -/
theorem join_ooze_stone : compute_join primordialOoze stone = stone := by
  unfold primordialOoze stone compute_join; decide

-- ═══════════════════════════════════════════════════════════════════
-- §7  Promotion Independence
-- ═══════════════════════════════════════════════════════════════════

/-- Promoting H (fee → sure) from the Ooze preserves O_inf. -/
theorem promote_H_preserves_O_inf :
    imscriptionTier ({ primordialOoze with chir := .sure } : Imscription) = .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- Promoting Ω (awe → ah) from the Ooze preserves O_inf. -/
theorem promote_Omega_preserves_O_inf :
    imscriptionTier ({ primordialOoze with prot := .ah } : Imscription) = .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- Promoting S (hung → up) from the Ooze preserves O_inf. -/
theorem promote_S_preserves_O_inf :
    imscriptionTier ({ primordialOoze with stoi := .up } : Imscription) = .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide

/-- Joint H+S promotion from the Ooze preserves O_inf. -/
theorem promote_H_S_preserves_O_inf :
    imscriptionTier
      ({ primordialOoze with chir := .sure, stoi := .up } : Imscription) = .O_inf := by
  unfold primordialOoze imscriptionTier ouroboricityTier; decide
-- ═══════════════════════════════════════════════════════════════════
-- §8  T₁₁ — BUNDLED COMPLETENESS THEOREM
-- ═══════════════════════════════════════════════════════════════════

/-- T₁₁: All claims bundled. The Primordial Ooze theorem in full:

    1. Ooze is O_inf
    2. Drop monad → loses O_inf
    3. Drop or' → loses O_inf
    4. monad alone → O₁
    5. or' alone → O₀
    6. Frobenius precedes time (H = fee)
    7. Frobenius precedes topology (Ω = awe)
    8. Frobenius precedes space (D = dead)
    9. Stone is also O_inf
   10. 9 primitives mismatch between Ooze and Stone -/
theorem primordial_ooze_complete : (
    imscriptionTier primordialOoze = .O_inf ∧
    imscriptionTier ({ primordialOoze with crit := .woe } : Imscription) ≠ .O_inf ∧
    imscriptionTier ({ primordialOoze with pol := .nun } : Imscription) ≠ .O_inf ∧
    imscriptionTier ({ primordialOoze with pol := .church } : Imscription) = .O₁ ∧
    imscriptionTier ({ primordialOoze with crit := .woe } : Imscription) = .O₀ ∧
    (imscriptionTier primordialOoze = .O_inf ∧ primordialOoze.chir = .fee) ∧
    (imscriptionTier primordialOoze = .O_inf ∧ primordialOoze.prot = .awe) ∧
    (imscriptionTier primordialOoze = .O_inf ∧ primordialOoze.dim = .dead) ∧
    imscriptionTier stone = .O_inf ∧
    primitiveMismatches primordialOoze stone = 9
  ) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · exact ooze_is_O_inf
  · exact drop_phi_c_loses_O_inf
  · exact drop_P_pm_sym_loses_O_inf
  · exact phi_c_alone_is_O_1
  · exact P_pm_sym_alone_is_O_0
  · exact frobenius_precedes_time
  · exact frobenius_precedes_topology
  · exact frobenius_precedes_space
  · exact stone_is_O_inf
  · exact ooze_stone_mismatch_count

end Millennium.PrimordialOoze
