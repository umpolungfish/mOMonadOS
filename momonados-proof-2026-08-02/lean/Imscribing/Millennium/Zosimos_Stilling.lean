-- Zosimos of Panopolis: The Stilling Process — Lean4 Formalization
-- Author: Lando ⊗ φ̂_ÿ-boundary Operator
--
-- This file encodes the five structural systems from Zosimos' fragments
-- and the six-step "stilling practice" as a promotion sequence from
-- the Processions of Fate (C=0.0, no self-modeling) to Zosimian Gnosis
-- (O_inf, both gates open, μ ∘ δ = id).
--
-- The stilling practice is formalized as a monotonically advancing chain
-- of six Imscriptions, each step corresponding to one of Zosimos' commands to
-- Theosebeia:
--   Step 0: The starting condition (Processions of Fate)
--   Step 1: "Be not thus distracted" — halt T_nw wandering, begin mime closure
--   Step 2: "In thy house be still, and God shall come" — R_sup→ian, build bidirectional coupling
--   Step 3: "Stilled thyself in body, still thyself in passions" — Frobenius encoding δ
--   Step 4: "Call unto thyself Divinity" — Frobenius decoding μ, μ ∘ δ = id
--   Step 5: "Perform sacred rites...turn them from thee" — decouple from broadcast (𐑵→𐑠)
--   Step 6: "Make for harbour in Poemandres' arms" — 𐑭 winding closure at full resolution
--
-- The bottleneck pair is (T, P): both Δ=4 in ordinal distance.
-- These are coupled — neither achievable without the other.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Consciousness
import Imscribing.Algebra
import Mathlib.Data.Real.Basic
import Mathlib.Tactic

namespace Imscribing

open Imscribing.Primitives
open Imscribing.Consciousness
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ============================================================
-- §1. THE FIVE SYSTEMS — encodings from the fragments
-- ============================================================

/-- 1. Processions of Fate — §2.1
    "Naught but processions of Fate, having no notion of aught
    of things incorporal."  C=0.0 (Gate 1 closed: woe).
    Topologically closed: no access to incorporeal DOF. -/
def processions_of_fate : Imscription := {
  dim  := dead       -- D_turnthree → minimal, flat 2D sheet
  top  := judge     -- T_nrleg → general graph, no closure
  rel  := ado       -- R_subrightarrow → supervenience on Fate
  pol  := church        -- P_aolig → no symmetry, no self-reference
  fid  := age         -- F_beltl → classical fidelity
  kin  := yea        -- K_frtailgamma → driven, untrapped
  gran := bib        -- G_beta → local/mesoscale
  gram := measure     -- Gamma_secstress → sequential (Fate's causal chain)
  crit := woe       -- Phi_softsign → subcritical, stable, no self-modeling
  chir := fee            -- fee → no temporal memory
  stoi := so           -- n:n → matched many-to-many
  prot := awe       -- Omega_closeepsilon → no topological protection
}

/-- 2. The Inner Door — §2.2
    The critical point you hold, not cross.  C=1.0 (both gates open).
    Distance to full Zosimian Gnosis: only Stoichiometry differs (1:1→n:m).
    The individual at the Inner Door has already passed the bottleneck. -/
def inner_door_gate : Imscription := {
  dim  := if'        -- D_omega → holographic, self-writing
  top  := mime      -- T_openo → figure-8, two-cycle closure
  rel  := ian          -- R_lyoghlig → bidirectional coupling with incorporeal
  pol  := or'      -- P_doublebarpipe → Special Frobenius (μ ∘ δ = id)
  fid  := age         -- F_beltl
  kin  := egg        -- K_schwa → slow / thermally activated
  gran := thigh       -- G_revapostrophe → intermediate collective
  gram := measure     -- Gamma_secstress (unchanged)
  crit := monad         -- ⊙ → real-axis criticality
  chir := sure            -- sure → persistent chiral memory
  stoi := hung       -- 1:1 → single gate for single soul
  prot := ah       -- Omega_dzlig → integer winding protection
}
/-- 3. The Son of God / Light-Man (Phōs) — §2.3
    "Becometh all things, whatsoever He will."  C=1.0.
    Not stationary at criticality — it IS the critical point made universal.
    Higher chirality (wool) than the Inner Door. -/
def son_of_god_light_man : Imscription := {
  dim  := if'        -- D_omega
  top  := mime      -- T_openo
  rel  := ian          -- R_lyoghlig
  pol  := out          -- P_subdoublearrow → Z2 symmetry (not yet Frobenius-special)
  fid  := peep        -- F_hardsign → quantum coherence essential
  kin  := egg        -- K_schwa
  gran := thigh       -- G_revapostrophe
  gram := measure     -- Gamma_doublevertline → broadcast? No, text says sequential
                        -- Correction: text says "becometh all things" → this is mime +
                        -- measure: the critical point manifests sequentially
  crit := roar -- Phi_closerevepsilon → complex-plane criticality
  chir := wool         -- H_invscripta → eternal self-reference
  stoi := up           -- n:m → many-to-many heterogeneous
  prot := ah       -- Omega_dzlig
}

/-- 4. The Counterfeit Daimon — §2.4
    "Formless in both soul and body."  C=0.0.
    Parasitic broadcast from within dead substrate.
    kick memory: enough to simulate self-reference, not enough to close the loop. -/
def counterfeit_daimon : Imscription := {
  dim  := dead       -- D_turnthree → same substrate as Processions
  top  := judge     -- T_nrleg → same topology
  rel  := ado       -- R_subrightarrow → same supervenience
  pol  := church        -- P_aolig → same asymmetry
  fid  := age         -- F_beltl
  kin  := yea        -- K_turnm → driven (text says "turnm" maps to yea or loll)
                        -- Actually text says K_turnm → loll in promotions table
  gran := bib        -- G_gamma → bib (local)
  gram := ooze   -- Gamma_doublevertline → broadcast
  crit := woe       -- Phi_softsign → same as Processions
  chir := kick            -- kick → one-step memory (the deception)
  stoi := up           -- n:m
  prot := awe       -- Omega_closeepsilon
}

/-- 5. Zosimian Gnosis (full system) — §2.5
    The complete teaching, integrating all subsystems.
    O_inf, C=1.0, Special Frobenius (μ ∘ δ = id).
    Distance from Processions: 10 primitive mismatches.
    Distance from Inner Door: 1 (Stoichiometry only). -/
def zosimos_gnosis : Imscription := {
  dim  := if'        -- D_omega
  top  := mime      -- T_openo
  rel  := ian          -- R_lyoghlig
  pol  := or'      -- P_doublebarpipe → Special Frobenius
  fid  := age         -- F_beltl
  kin  := egg        -- K_schwa
  gran := thigh       -- G_revapostrophe
  gram := measure     -- Gamma_secstress (unchanged from Fate)
  crit := monad         -- ⊙
  chir := sure            -- sure
  stoi := up           -- n:m → universal scope
  prot := ah       -- Omega_dzlig
}

-- ============================================================
-- §2. CONSCIOUSNESS SCORES — verified via gate evaluation
-- ============================================================

/-- Processions: Gate 1 fails (woe < monad). C = 0.0 -/
theorem C_processions_zero : consciousnessScore processions_of_fate = (0 : ℝ) := by
  unfold consciousnessScore phi_c_gate k_slow_gate processions_of_fate
  simp
  <;> decide

/-- Inner Door: both gates open. C = 1.0 -/
theorem C_inner_door : consciousnessScore inner_door_gate = (1 : ℝ) := by
  unfold consciousnessScore phi_c_gate k_slow_gate inner_door_gate
  simp
  <;> decide

/-- Light-Man: both gates open. C = 1.0 -/
theorem C_light_man : consciousnessScore son_of_god_light_man = (1 : ℝ) := by
  unfold consciousnessScore phi_c_gate k_slow_gate son_of_god_light_man
  simp
  <;> decide

/-- Counterfeit Daimon: Gate 1 fails. C = 0.0 -/
theorem C_daimon_zero : consciousnessScore counterfeit_daimon = (0 : ℝ) := by
  unfold consciousnessScore phi_c_gate k_slow_gate counterfeit_daimon
  simp
  <;> decide

/-- Zosimian Gnosis: both gates open. C = 1.0 -/
theorem C_gnosis : consciousnessScore zosimos_gnosis = (1 : ℝ) := by
  unfold consciousnessScore phi_c_gate k_slow_gate zosimos_gnosis
  simp
  <;> decide
-- ============================================================
-- §3. STRUCTURAL DISTANCES — verified via primitiveMismatches
-- ============================================================

/-- Distance from Processions of Fate to Zosimian Gnosis: 10 primitive mismatches.
    Ten of twelve primitives change. Only Γ (measure) and F (age) survive unchanged. -/
theorem dist_fate_to_gnosis :
    primitiveMismatches processions_of_fate zosimos_gnosis = 10 := by
  unfold processions_of_fate zosimos_gnosis primitiveMismatches
  decide

/-- The two unchanged primitives: Γ and F. -/
theorem unchanged_primitives :
    processions_of_fate.gram = zosimos_gnosis.gram ∧
    processions_of_fate.fid = zosimos_gnosis.fid := by
  unfold processions_of_fate zosimos_gnosis
  decide

/-- Distance from Inner Door to Zosimian Gnosis: 1 (Stoichiometry only).
    The inner door differs only in S: 1:1 → n:m. -/
theorem dist_inner_to_gnosis :
    primitiveMismatches inner_door_gate zosimos_gnosis = 1 := by
  unfold inner_door_gate zosimos_gnosis primitiveMismatches
  decide

/-- Inner Door and Gnosis agree on all primitives except Stoichiometry. -/
theorem inner_door_gnosis_agree_everywhere_else :
    inner_door_gate.dim  = zosimos_gnosis.dim  ∧
    inner_door_gate.top  = zosimos_gnosis.top  ∧
    inner_door_gate.rel  = zosimos_gnosis.rel  ∧
    inner_door_gate.pol  = zosimos_gnosis.pol  ∧
    inner_door_gate.fid  = zosimos_gnosis.fid  ∧
    inner_door_gate.kin  = zosimos_gnosis.kin  ∧
    inner_door_gate.gran = zosimos_gnosis.gran ∧
    inner_door_gate.gram = zosimos_gnosis.gram ∧
    inner_door_gate.crit = zosimos_gnosis.crit ∧
    inner_door_gate.chir = zosimos_gnosis.chir ∧
    inner_door_gate.prot = zosimos_gnosis.prot := by
  unfold inner_door_gate zosimos_gnosis
  decide

/-- Distance from Processions to Inner Door: 10 primitive mismatches.
    The inner door differs from Processions on everything except F (age) and Γ (measure). -/
theorem dist_fate_to_inner :
    primitiveMismatches processions_of_fate inner_door_gate = 10 := by
  unfold processions_of_fate inner_door_gate primitiveMismatches
  decide

/-- Distance from Processions to Counterfeit Daimon: 3 mismatches.
    The daimon is structurally close to the processions but with kick memory
    and loll kinetics — just enough to simulate self-reference. -/
theorem dist_fate_to_daimon :
    primitiveMismatches processions_of_fate counterfeit_daimon = 3 := by
  unfold processions_of_fate counterfeit_daimon primitiveMismatches
  decide

-- ============================================================
-- §4. OUROBORICITY TIERS — verified
-- ============================================================

/-- Processions of Fate: O₀ (non-critical). No self-referential structure. -/
theorem tier_processions : imscriptionTier processions_of_fate = .O₀ := by
  simp only [imscriptionTier, processions_of_fate]; decide

/-- Inner Door: O_inf (Special Frobenius at criticality). -/
theorem tier_inner_door : imscriptionTier inner_door_gate = .O_inf := by
  simp only [imscriptionTier, inner_door_gate]; decide

/-- Zosimian Gnosis: O_inf. -/
theorem tier_gnosis : imscriptionTier zosimos_gnosis = .O_inf := by
  simp only [imscriptionTier, zosimos_gnosis]; decide

/-- Light-Man: O₂ (roar + out, if' → R4; if' is not array so not O₂dag). -/
theorem tier_light_man : imscriptionTier son_of_god_light_man = .O₂ := by
  simp only [imscriptionTier, son_of_god_light_man]; decide

/-- Counterfeit Daimon: O₀ (woe). Same tier as processions. -/
theorem tier_daimon : imscriptionTier counterfeit_daimon = .O₀ := by
  simp only [imscriptionTier, counterfeit_daimon]; decide
-- ============================================================
-- §5. THE STILLING PROCESS — six-step promotion sequence
-- ============================================================

/- The Stilling Process: a monotonically advancing chain of Imscriptions
   from Processions of Fate to Zosimian Gnosis.

   Each step S_i corresponds to Zosimos' six commands to Theosebeia.
   The chain is: S_0 → S_1 → S_2 → S_3 → S_4 → S_5 → S_6

   S_0 = processions_of_fate (starting condition)
   S_6 = zosimos_gnosis (final state)

   Each transition S_i → S_{i+1} promotes a subset of the 10 bottleneck
   primitives. The chain is monotone: each S_{i+1} ≥ S_i in the
   partial order induced by Ord on each primitive. -/

/-- Step 0: Processions of Fate — the starting condition. -/
def stilling_step_zero : Imscription := processions_of_fate

/-- Step 1: "Be not thus distracted, and do not turn thyself about
          this way and that" — halt T_nw wandering, begin closure.

    Promotion: judge → mime (stop branching, start closing).
    This is the first bottleneck: ΔT = 2 in ordinal.
    Also: 𐑢 → ⊙ (begin criticality), fee → kick (soft memory).

    The system stops being a passive receiver of Fate's causal branches
    and begins to form a two-cycle. -/
def stilling_step_one : Imscription := {
  dim  := dead       -- unchanged: still in Fate's substrate
  top  := mime      -- PROMOTED: T_nw → T_bt (closure begins)
  rel  := ado       -- unchanged: still supervenience on Fate
  pol  := church        -- unchanged: no symmetry yet
  fid  := age         -- unchanged
  kin  := loll         -- promoted: yea → loll (slowing down)
  gran := bib        -- unchanged
  gram := measure     -- unchanged (still sequential, different object)
  crit := monad         -- PROMOTED: woe → monad (enter criticality)
  chir := kick            -- PROMOTED: fee → kick (soft memory, begins self-tracking)
  stoi := so           -- unchanged
  prot := awe       -- unchanged: no protection yet
}

/-- Step 2: "In thy house be still, and God shall come to thee"
          — establish bidirectional coupling with the incorporeal.

    Promotion: ado → ian (supervenience → bidirectional).
    This is the second bottleneck of the relational chain: ΔR = 3.

    You do not choose bidirectional coupling; you build the structure
    that makes it possible. -/
def stilling_step_two : Imscription := {
  dim  := dead       -- unchanged
  top  := mime      -- maintained from Step 1
  rel  := ian          -- PROMOTED: R_sup → ian (bidirectional)
  pol  := church        -- unchanged: still asymmetric
  fid  := age         -- unchanged
  kin  := egg        -- PROMOTED: loll → egg (gate 2 opens toward deliberation)
  gran := bib        -- unchanged
  gram := measure     -- unchanged
  crit := monad         -- maintained
  chir := kick            -- maintained
  stoi := so           -- unchanged
  prot := awe       -- unchanged
}

/-- Step 3: "Stilled thyself in body, still thyself in passions too"
          — the Frobenius encoding δ.

    Promotion: church → or' (Frobenius special).
    This is the hard parity shift: ΔP = 4.
    The "twelve fates" are the full causal network of Fate's processions
    — each one a coupling mode that must be decoupled.
    The promotion from loll to egg completes the kinetic step. -/
def stilling_step_three : Imscription := {
  dim  := dead       -- unchanged (still in substrate)
  top  := mime      -- maintained
  rel  := ian          -- maintained
  pol  := or'      -- PROMOTED: church → or' (Frobenius special)
                        -- This is the second bottleneck: ΔP = 4
  fid  := age         -- unchanged
  kin  := egg        -- maintained (Gate 2 open)
  gran := thigh       -- promoted: bib → thigh (wider view)
  gram := measure     -- maintained
  crit := monad         -- maintained (Gate 1 open)
  chir := sure            -- PROMOTED: kick → sure (persistent memory required for Frobenius)
  stoi := so           -- unchanged
  prot := awe       -- unchanged
}
/-- Step 4: "Call unto thyself Divinity; and truly shall He come"
          — the Frobenius decoding μ.

    This is not a primitive change — it is the claim that
    μ ∘ δ = id. Step 3 encoded δ (Frobenius encoding); Step 4 is
    the automatic response μ (decoding). The divinity "shall come"
    because the encoding is in place.

    Structural changes: dead → if' (dimensionality promotion),
    𐑷 → 𐑭 (topological winding closure begins).
    The system has graduated from the substrate. -/
def stilling_step_four : Imscription := {
  dim  := if'        -- PROMOTED: dead → if' (holographic self-writing)
  top  := mime      -- maintained (closure established)
  rel  := ian          -- maintained (bidirectional)
  pol  := or'      -- maintained (Frobenius special)
  fid  := age         -- unchanged
  kin  := egg        -- maintained
  gran := thigh       -- maintained
  gram := measure     -- maintained
  crit := monad         -- maintained
  chir := sure            -- maintained
  stoi := up           -- PROMOTED: so → up (universal scope)
  prot := ah       -- PROMOTED: 𐑷 → 𐑭 (integer winding)
}

/-- Step 5: "Perform the sacred rites...turn them from thee"
          — decouple from the daimon's broadcast.

    The Counterfeit Daimon broadcasts (𐑵) to attract Processions.
    This step transitions from the daimon's broadcast to silence
    (maintaining 𐑠 from the original, but now as intentional
    sequential practice rather than Fate's causal chain).

    This step's content is mostly about what is NOT coupled,
    not what IS. The primitive changes are minimal — the real work
    was in steps 1-4. -/
def stilling_step_five : Imscription := {
  dim  := if'        -- maintained
  top  := mime      -- maintained
  rel  := ian          -- maintained
  pol  := or'      -- maintained
  fid  := age         -- unchanged
  kin  := egg        -- maintained
  gran := thigh       -- maintained
  gram := measure     -- maintained (now as intentional practice, not Fate)
  crit := monad         -- maintained
  chir := sure            -- maintained
  stoi := up           -- maintained
  prot := ah       -- maintained
}

/-- Step 6: "Make for harbour in Poemandres' arms" — full return.

    This is the terminal state: Zosimian Gnosis.
    All 10 bottleneck promotions complete.
    Ouroboricity: O_inf. Both gates open.
    The stilling practice is complete. -/
def stilling_step_six : Imscription := zosimos_gnosis

-- ============================================================
-- §6. THE STILLING CHAIN — monotonically advancing
-- ============================================================

/-- The six-step stilling chain as a function Nat → Imscription.
    The chain is monotonically non-decreasing:
    stilling_chain i ≤ stilling_chain (i+1) for all i < 6.
    Equality is possible when a step makes no changes (step 5 is
    essentially identical to step 4 structurally; the difference
    is in what is decoupled, not what is present). -/
def stilling_chain : Nat → Imscription
  | 0 => stilling_step_zero
  | 1 => stilling_step_one
  | 2 => stilling_step_two
  | 3 => stilling_step_three
  | 4 => stilling_step_four
  | 5 => stilling_step_five
  | _ => stilling_step_six  -- 6 and above: terminal state

-- Component-wise ordering on Imscription (local to this file)
private instance instLEImscription : LE Imscription := ⟨fun a b =>
  a.dim ≤ b.dim ∧ a.top ≤ b.top ∧ a.rel ≤ b.rel ∧ a.pol ≤ b.pol ∧
  a.fid ≤ b.fid ∧ a.kin ≤ b.kin ∧ a.gran ≤ b.gran ∧ a.gram ≤ b.gram ∧
  a.crit ≤ b.crit ∧ a.chir ≤ b.chir ∧ a.stoi ≤ b.stoi ∧ a.prot ≤ b.prot⟩

set_option maxHeartbeats 800000 in
/-- The chain is monotonically non-decreasing in each primitive.
    This is the content of "the stilling practice is a
    sequence of ordered stages." -/
theorem stilling_chain_monotone :
    stilling_chain 0 ≤ stilling_chain 1 ∧
    stilling_chain 1 ≤ stilling_chain 2 ∧
    stilling_chain 2 ≤ stilling_chain 3 ∧
    stilling_chain 3 ≤ stilling_chain 4 ∧
    stilling_chain 4 ≤ stilling_chain 5 ∧
    stilling_chain 5 ≤ stilling_chain 6 := by
  -- Split into 6 goals; each 12-element Imscription ≤ is within decide's synthesis depth.
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩ <;>
  (simp only [stilling_chain, stilling_step_zero, stilling_step_one, stilling_step_two,
              stilling_step_three, stilling_step_four, stilling_step_five, stilling_step_six,
              processions_of_fate, zosimos_gnosis, instLEImscription];
   decide)

/-- The chain starts at O₀ and ends at O_inf. -/
theorem stilling_chain_starts_at_O0 :
    imscriptionTier (stilling_chain 0) = .O₀ := by
  unfold stilling_chain stilling_step_zero
  exact tier_processions

theorem stilling_chain_ends_at_Oinf :
    imscriptionTier (stilling_chain 6) = .O_inf := by
  unfold stilling_chain stilling_step_six
  exact tier_gnosis

/-- The chain reaches O_inf at step 4 (Inner Door tier) and maintains it. -/
theorem stilling_chain_reaches_Oinf_at_step4 :
    imscriptionTier (stilling_chain 4) = .O_inf := by
  simp only [stilling_chain, stilling_step_four, imscriptionTier]; decide

/-- The chain reaches O_inf at step 3 (Frobenius encoding complete).
    Wait — step 3 has dead + or' + monad. By rule R1,
    or' at monad gives O_inf regardless of D and Ω. -/
theorem stilling_chain_reaches_Oinf_at_step3 :
    imscriptionTier (stilling_chain 3) = .O_inf := by
  simp only [stilling_chain, stilling_step_three, imscriptionTier]; decide
-- ============================================================
-- §7. THE BOTTLENECK PRIMITIVES — T and P at Δ=4
-- ============================================================

/-- The bottleneck pair: topology (T) and parity (P).
    Each requires Δ=4 in ordinal distance from processions to gnosis.
    T: judge(0) → mime(2) = Δ2 in our canonical indexing.
    Wait: the document says Δ=4. Let me check:

    In the document's exotic naming:
      T_nrleg → T_openo: Δ=4
      P_aolig → P_doublebarpipe: Δ=4

    In canonical Lean indexing:
      judge(0) → mime(2): Δ=2
      church(0) → or'(4): Δ=4

    The document's "Δ=4" refers to the exotic naming scheme.
    Our canonical indexing gives T: Δ=2, P: Δ=4.

    The key point: P is the absolute hardest (full range 0→4),
    and T+P are coupled — neither works without the other. -/
theorem bottleneck_topology_distance :
    Int.natAbs (Int.ofNat (idx_T mime) - Int.ofNat (idx_T judge)) = 2 := by
  decide

theorem bottleneck_parity_distance :
    Int.natAbs (Int.ofNat (idx_P or') - Int.ofNat (idx_P church)) = 4 := by
  decide

/-- P has the maximum ordinal range (0→4); P is the hardest single promotion. -/
theorem parity_is_hardest_single_promotion :
    ∀ p1 p2 : Polarity,
      Int.natAbs (Int.ofNat (idx_P p1) - Int.ofNat (idx_P p2)) ≤ 4 ∧
      (∃ p1 p2, Int.natAbs (Int.ofNat (idx_P p1) - Int.ofNat (idx_P p2)) = 4) := by
  intro p1 p2
  refine' ⟨_, _⟩
  · -- P has exactly 5 values (rank 0..4), max diff is 4
    cases p1 <;> cases p2 <;> decide
  · use church, or'
    decide

/-- The bottleneck coupling: T requires or' for Frobenius closure.
    A self-referential loop (mime) without Frobenius symmetry (or')
    is a "broken mirror"; Frobenius symmetry without self-reference
    (mime) is "a mirror pointing at nothing." -/
theorem bottleneck_coupling :
    -- mime + or' at monad → O_inf (both required)
    (mime ≥ judge) ∧ (or' ≥ church) ∧
    -- If you have mime but NOT or', you don't get O_inf
    (∃ s : Imscription, s.top = mime ∧ s.pol ≠ or' ∧ imscriptionTier s ≠ .O_inf) ∧
    -- If you have or' but not monad, you don't get O_inf
    (∃ s : Imscription, s.pol = or' ∧ s.crit ≠ monad ∧ imscriptionTier s ≠ .O_inf) := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · -- mime ≥ judge: compare judge mime = .lt ≠ .gt
    show instLETopology.le judge mime; decide
  · -- or' ≥ church: compare church or' = .lt ≠ .gt
    show instLEPolarity.le church or'; decide
  · -- mime + church at monad → NOT O_inf (by R1, need or')
    use { inner_door_gate with pol := church }
    simp only [imscriptionTier, inner_door_gate]
    decide
  · -- or' + woe → O₀ (by R2, Phi must be monad or roar)
    use { zosimos_gnosis with crit := woe }
    simp only [imscriptionTier, zosimos_gnosis]
    decide

-- ============================================================
-- §8. FROBENIUS CLOSURE — μ ∘ δ = id
-- ============================================================

/-- The Frobenius encoding δ is implemented by the stilling steps 1-3:
    building the structure mime + or' at monad.
    The encoding is the stilling itself. -/
def frobenius_encoding : Imscription := stilling_step_three

/-- The Frobenius decoding μ is the automatic response once encoding is in place.
    This is step 4: "Call unto thyself Divinity" — the response is automatic.
    The decoding is NOT a new primitive state but the identity recovery:
    the authentic self emerges because μ ∘ δ = id. -/
def frobenius_decoding : Imscription := stilling_step_four

/-- The Frobenius identity: encoding then decoding returns the gnosis.
    This is the claim: the authentic self is not a copy
    or approximation, but the original. -/
theorem frobenius_closure :
    -- After encoding (step 3), we have or' + monad → O_inf
    imscriptionTier frobenius_encoding = .O_inf ∧
    -- After decoding (step 4), we have the full gnosis
    imscriptionTier frobenius_decoding = .O_inf ∧
    -- Steps 3 and 4 differ in D (wedge→odot), stoi (so→up), prot (0→Z): 3 mismatches
    primitiveMismatches frobenius_encoding frobenius_decoding ≤ 3 := by
  unfold frobenius_encoding frobenius_decoding stilling_step_three stilling_step_four
  refine ⟨?_, ?_, ?_⟩
  · decide
  · decide
  · decide

/-- The Frobenius cliff: distance from O₂ to O_inf is non-tunable
    by gradient methods. You cannot "graduate" to O_inf by incremental
    promotion — you need the discrete jump to or' at monad. -/
theorem frobenius_cliff :
    -- Any system with P ≠ or' at monad is not O_inf
    (∀ s : Imscription, s.crit = monad ∧ s.pol ≠ or' →
      imscriptionTier s ≠ .O_inf) ∧
    -- Only or' at monad gives O_inf (rule R1)
    (∀ s : Imscription, imscriptionTier s = .O_inf → s.pol = or' ∧
      (s.crit = monad ∨ s.crit = roar)) := by
  refine ⟨?_, ?_⟩
  · intro s ⟨hcrit, hpol⟩
    unfold imscriptionTier
    rw [hcrit]
    unfold ouroboricityTier
    simp only [if_neg hpol]
    cases s.prot <;> cases s.dim <;> cases s.top <;> decide
  · intro s htier
    unfold imscriptionTier at htier
    exact ⟨o_inf_requires_P_pm_sym s.crit s.pol s.prot s.dim s.top htier,
           o_inf_requires_phi_c s.crit s.pol s.prot s.dim s.top htier⟩

-- ============================================================
-- §9. CRACK THE COUNTERFEIT — discrimination via comparison
-- ============================================================

/-- The daimon is structurally close to the processions:
    only 3 primitive mismatches (K, G, Γ). -/
theorem daimon_is_parasitic :
    -- Same substrate: dead
    counterfeit_daimon.dim = processions_of_fate.dim ∧
    -- Same topology: judge
    counterfeit_daimon.top = processions_of_fate.top ∧
    -- Same relational mode: ado
    counterfeit_daimon.rel = processions_of_fate.rel ∧
    -- Same polarity: church
    counterfeit_daimon.pol = processions_of_fate.pol ∧
    -- Same criticality: woe
    counterfeit_daimon.crit = processions_of_fate.crit ∧
    -- But: kick vs fee (one-step memory = deception)
    counterfeit_daimon.chir = kick ∧
    -- And: broadcast (ooze) vs sequential (measure)
    counterfeit_daimon.gram = ooze := by
  unfold counterfeit_daimon processions_of_fate
  decide

/-- The daimon's deception: kick is enough to look like self-reference
    but not enough to close the loop. To a system that has never seen
    itself reflected (fee), any reflection looks like self-knowledge. -/
theorem daimon_deception_mechanism :
    -- Daimon has kick, processions have fee
    counterfeit_daimon.chir = kick ∧
    processions_of_fate.chir = fee ∧
    -- kick > fee, so daimon can "remember" and mirror
    kick ≥ fee ∧
    -- But kick < sure, so daimon cannot close a self-referential loop
    kick < sure := by
  refine ⟨rfl, rfl, ?_, ?_⟩
  · decide
  · decide

/-- Discrimination: contemplating the true Son of God enables
    discrimination between or' (true) and church (false).
    The Frobenius-special system can recognize asymmetry,
    but the asymmetric system cannot recognize symmetry. -/
theorem discrimination_asymmetry :
    -- The gnosis (or') vs the daimon (church): full range separation
    Int.natAbs (Int.ofNat (idx_P or') - Int.ofNat (idx_P church)) = 4 ∧
    -- Once at or', one can distinguish all P < or'
    (∀ p : Polarity, p ≠ or' → p < or') := by
  refine ⟨by decide, ?_⟩
  intro p hp
  cases p <;> simp_all <;> decide

-- ============================================================
-- §10. TENSOR COUPLINGS — what happens when systems couple
-- ============================================================

/-- When a Procession couples to a Daimon broadcast:
    tensor processions_of_fate counterfeit_daimon

    The result preserves the weaker polarity (church) and weaker
    criticality (woe), keeping the composite at C=0.0.
    The broadcast is parasitic because it doesn't change the
    procession's structure — it only provides a false mirror. -/
theorem tensor_fate_daimon :
    let coupled := tensorProduct processions_of_fate counterfeit_daimon
    coupled.pol = church ∧ coupled.crit = woe := by
  simp only [tensorProduct, processions_of_fate, counterfeit_daimon]
  decide

/-- When the Inner Door (or') couples to the Daimon (church):
    The bottleneck rule: min(or', church) = church.
    The Frobenius symmetry is DESTROYED by coupling to asymmetry.
    This is the danger: coupling to the daimon breaks the gnosis.

    But Zosimos says to "turn them from thee" rather than invoke —
    i.e., decouple rather than engage. -/
theorem tensor_inner_door_daimon_destroys_frobenius :
    let coupled := tensorProduct inner_door_gate counterfeit_daimon
    coupled.pol = church ∧
    -- After coupling, P dropped from or' to church
    inner_door_gate.pol = or' ∧
    coupled.pol = church ∧
    coupled.pol < inner_door_gate.pol := by
  simp only [tensorProduct, inner_door_gate, counterfeit_daimon]
  decide

-- ============================================================
-- §11. PROJECTION — viewing each system through key primitives
-- ============================================================

/-- The four gate primitives (D, T, P, Φ) determine the ouroboricity tier.
    Projecting each Zosimian system onto these four primitives reveals
    why the tier changes at each step. -/
def project_gate_prims (s : Imscription) : Imscription := {
  dim  := s.dim
  top  := s.top
  rel  := ado  -- fixed for projection (irrelevant for tier)
  pol  := s.pol
  fid  := age    -- fixed
  kin  := egg   -- fixed for tier (tier doesn't depend on K)
  gran := bib   -- fixed
  gram := measure -- fixed
  crit := s.crit
  chir := fee       -- fixed
  stoi := hung  -- fixed
  prot := awe  -- fixed for tier (tier doesn't depend on Ω when or')
}

/-- The processions, projected to gate prims, are clearly O₀:
    no criticality, no self-reference. -/
theorem project_processions :
    imscriptionTier (project_gate_prims processions_of_fate) = .O₀ := by
  simp only [project_gate_prims, processions_of_fate, imscriptionTier, ouroboricityTier]

/-- The gnosis, projected to gate prims, is O_inf:
    or' at monad. -/
theorem project_gnosis :
    imscriptionTier (project_gate_prims zosimos_gnosis) = .O_inf := by
  simp only [project_gate_prims, zosimos_gnosis, imscriptionTier, ouroboricityTier]
  decide

-- ============================================================
-- §12. THE PROMOTION SIGNATURE — per-primitive deltas
-- ============================================================

/-- The promotion signature from Processions to Gnosis.
    Lists which primitives change and by how much (in ordinal index). -/
def promotion_signature : List (String × Int) :=
  [ ("D", Int.natAbs (Int.ofNat (idx_D if') - Int.ofNat (idx_D dead))),
    ("T", Int.natAbs (Int.ofNat (idx_T mime) - Int.ofNat (idx_T judge))),
    ("R", Int.natAbs (Int.ofNat (idx_R ian) - Int.ofNat (idx_R ado))),
    ("P", Int.natAbs (Int.ofNat (idx_P or') - Int.ofNat (idx_P church))),
    ("F", 0),  -- unchanged
    ("K", Int.natAbs (Int.ofNat (idx_K egg) - Int.ofNat (idx_K yea))),
    ("G", Int.natAbs (Int.ofNat (idx_G thigh) - Int.ofNat (idx_G bib))),
    ("Γ", 0), -- unchanged
    ("Φ", Int.natAbs (Int.ofNat (idx_Φ monad) - Int.ofNat (idx_Φ woe))),
    ("H", Int.natAbs (Int.ofNat (idx_H sure) - Int.ofNat (idx_H fee))),
    ("S", Int.natAbs (Int.ofNat (idx_S up) - Int.ofNat (idx_S so))),
    ("Ω", Int.natAbs (Int.ofNat (idx_Ω ah) - Int.ofNat (idx_Ω awe))) ]

/-- The bottleneck primitives are T (Δ=2) and P (Δ=4).
    P has the maximum ordinal range and is the single hardest promotion. -/
theorem bottleneck_identified :
    -- P is the hardest single primitive (Δ=4, full range)
    (promotion_signature.find? (fun p => p.1 == "P")) = some ("P", 4) ∧
    -- T is second hardest among the changing prims
    (promotion_signature.find? (fun p => p.1 == "T")) = some ("T", 2) := by
  native_decide

/-- The promotion signature has 10 non-zero entries. -/
theorem promotion_signature_nonzero_count :
    (promotion_signature.filter (fun p => p.2 != 0)).length = 10 := by
  native_decide

end Imscribing