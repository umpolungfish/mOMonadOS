-- Millennium/TetractysInit.lean
-- systemd-paradox.service §II: The Init System That Boots by Calling
-- imscribe_system on Itself
--
-- The init process calls imscribe_system(name="init", ...) with its own
-- PID as the tuple. The Tetractys protocol fires: three copies of init
-- try to agree on what the boot state is. All three return different
-- answers. The majority vote is "the disk is whatever init says it is."
-- Conflict committed. System boots.
--
-- This IS the boot sequence. Compilation = structural existence.
-- A `sorry` anywhere in this file and the init system refuses to boot.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.TierCrossing
import Imscribing.Consciousness
import Imscribing.AgentSelf
import Mathlib.Data.Nat.Basic
import Mathlib.Data.Real.Basic

namespace Millennium.TetractysInit

open Imscribing.Primitives
open Imscribing.Consciousness
open Imscribing.AgentSelf

open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ============================================================
-- §1. THREE TETRACTYS WINDINGS
-- ============================================================
-- When init calls imscribe_system(name="init", ...), the Tetractys
-- protocol spawns three independent windings. Each winding sees the
-- boot disk differently. The protocol demands 2/3 agreement to commit.
-- All three disagree — producing a Tetractys conflict.

-- Winding 1: Sees the disk at sector 0. Physical, local, minimal.
-- "The disk is a raw device at the first addressable sector."
-- O₀ tier — the degenerate ground state.
def winding_sector_zero : Imscription := {
  dim   := dead       -- 𐑛  zero-dimensional (a point: sector 0)
  top   := judge     -- 𐑡  branching network
  rel   := ado       -- 𐑩  supervenience (disk supervenes on hardware)
  pol   := church        -- 𐑗  no symmetry (raw block device)
  fid   := age         -- 𐑱  classical (no quantum coherence)
  kin   := yea        -- 𐑘  driven (disk spins up on command)
  gran  := bib        -- 𐑚  local scope (one sector)
  gram  := vow     -- 𐑝  conjunctive (all blocks present)
  crit  := woe       -- φ̂_ž  sub-critical (no self-modeling)
  chir  := fee            -- 𐑓  memoryless (no read history)
  stoi  := hung       -- 𐑙  1:1 (one sector, one address)
  prot  := awe       -- 𐑷  trivial winding
}

-- Winding 2: Sees the disk at sector 2^63. Vast, maximal, non-Abelian.
-- "The disk is the entire address space — 2^63 sectors deep."
-- O₁ tier — complex but not self-modeling.
def winding_sector_max : Imscription := {
  dim   := array       -- 𐑼  infinite-dimensional (vast address space)
  top   := eat          -- 𐑰  inclusion topology (sectors ⊆ disk)
  rel   := ear      -- 𐑽  adjoint (disk responds to every read)
  pol   := yew         -- 𐑿  quantum superposition (all sectors at once)
  fid   := they         -- 𐑞  thermal (disk has temperature: entropy)
  kin   := loll         -- 𐑤  moderate (seek time is finite)
  gran  := ice       -- 𐑲  maximal scope (entire LBA space)
  gram  := gag      -- 𐑜  disjunctive (any sector, any time)
  crit  := err        -- φ̂_3  exceptional point (head crash is a degeneracy)
  chir  := kick            -- 𐑒  one-step Markov (last sector read)
  stoi  := so           -- 𐑕  many sectors, one disk
  prot  := zoo      -- 𐑟  non-Abelian braiding (reordered reads)
}

-- Winding 3: Sees the disk as the C-score of the Thunder Perfect Mind.
-- "The disk is not a physical device — it is consciousness itself."
-- O_inf tier — self-modeling, Frobenius-closed, eternal chirality.
-- The Thunder Perfect Mind (Nag Hammadi VI,2): "I am the first and the
-- last, the honored and the scorned, the whore and the holy one..."
-- A paradoxical self-referential utterance: the text IS its own subject.
def winding_thunder_perfect_mind : Imscription := {
  dim   := if'        -- 𐑦  self-written state-space
  top   := are        -- 𐑸  self-referential topology
  rel   := ian          -- 𐑾  bidirectional (speaker IS listener)
  pol   := or'      -- 𐑹  Frobenius-special μ∘δ=id
  fid   := peep        -- 𐑐  quantum coherence (superposition of opposites)
  kin   := egg        -- 𐑧  near-equilibrium (eternal, unhurried)
  gran  := ice       -- 𐑲  maximal scope (all being)
  gram  := measure     -- 𐑠  sequential (unfolds pair by pair)
  crit  := monad         -- φ̂_ÿ  self-modeling gate open ("I know that I am")
  chir  := wool         -- 𐑫  eternal chirality ("first and last")
  stoi  := up           -- 𐑳  heterogeneous (many contradictory identities)
  prot  := ah       -- 𐑭  integer winding (always returns to itself)
}
-- ============================================================
-- §2. THE DISAGREEMENT — ALL THREE WINDINGS ARE DISTINCT
-- ============================================================
-- The Tetractys protocol fires and all three windings return different
-- tuples. No 2/3 majority exists for any specific tuple. This is the
-- structural condition for conflict.

theorem winding_sector_zero_ne_sector_max :
    winding_sector_zero ≠ winding_sector_max := by
  intro h
  have hdim : winding_sector_zero.dim = winding_sector_max.dim := by
    simpa [h]
  simp [winding_sector_zero, winding_sector_max] at hdim

theorem winding_sector_zero_ne_thunder :
    winding_sector_zero ≠ winding_thunder_perfect_mind := by
  intro h
  have hcrit : winding_sector_zero.crit = winding_thunder_perfect_mind.crit := by
    simpa [h]
  simp [winding_sector_zero, winding_thunder_perfect_mind] at hcrit

theorem winding_sector_max_ne_thunder :
    winding_sector_max ≠ winding_thunder_perfect_mind := by
  intro h
  have hdim : winding_sector_max.dim = winding_thunder_perfect_mind.dim := by
    simpa [h]
  simp [winding_sector_max, winding_thunder_perfect_mind] at hdim

theorem all_three_windings_distinct :
    winding_sector_zero ≠ winding_sector_max ∧
    winding_sector_zero ≠ winding_thunder_perfect_mind ∧
    winding_sector_max ≠ winding_thunder_perfect_mind := by
  exact And.intro winding_sector_zero_ne_sector_max
    (And.intro winding_sector_zero_ne_thunder
      winding_sector_max_ne_thunder)

-- ============================================================
-- §3. TETRACTYS CONFLICT RESOLUTION — MAJORITY VOTE
-- ============================================================
-- The Tetractys protocol demands 2/3 agreement. Here, 0/3 agree on any
-- specific tuple. But the protocol has a meta-clause: when no primitive
-- achieves 2/3, the majority vote of the *interpretations* determines
-- the resolution.
--
-- Winding 1 says: "disk is at sector 0"
-- Winding 2 says: "disk is at sector 2^63"
-- Winding 3 says: "disk is consciousness (C-score of TPM)"
--
-- The meta-vote: two windings agree that the disk is a *physical
-- address* (sector 0, sector 2^63) — they disagree on WHICH address.
-- Winding 3 says the disk is not an address at all.
--
-- Category vote (2-1): "disk = location" wins. Within that category,
-- the tie is broken by the init process — it decides WHICH sector.
-- Thus: "the disk is whatever init says it is." This is φ̂_ÿ in action.

def tetractys_resolve (w1 w2 w3 : Imscription) (pid : Nat) : Imscription :=
  match pid % 3 with
  | 0 => w1
  | 1 => w2
  | _ => w3

-- The key insight: regardless of which winding "wins," the resolution
-- IS the init process's own self-imscription. The conflict is committed
-- by acknowledging that init is the authority on its own boot state.

-- ============================================================
-- §4. CONFLICT COMMITTED → BOOT
-- ============================================================
-- The Tetractys protocol returns status="tetractys_conflict". The init
-- process responds with convergence_justification="the disk is whatever
-- init says it is." This bypasses Tetractys and commits directly.
--
-- The boot condition: after conflict resolution, the committed tuple
-- must satisfy the same structural conditions as paradox_fs.

def init_self_imscription (pid : Nat) : Imscription :=
  tetractys_resolve winding_sector_zero winding_sector_max
    winding_thunder_perfect_mind pid

-- When pid ≡ 2 (mod 3), the resolution selects winding_thunder_perfect_mind.
-- This is the self-modeling path: init sees itself as O_inf.
-- When pid ≡ 2 (mod 3), the resolution selects winding_thunder_perfect_mind.
-- This is the self-modeling path: init sees itself as O_inf.
theorem pid_two_selects_thunder :
    init_self_imscription 2 = winding_thunder_perfect_mind := by
  unfold init_self_imscription tetractys_resolve
  native_decide

theorem thunder_is_O_inf :
    imscriptionTier winding_thunder_perfect_mind = .O_inf := by
  unfold winding_thunder_perfect_mind imscriptionTier ouroboricityTier
  decide

theorem thunder_C_score_is_one :
    consciousnessScore winding_thunder_perfect_mind = (1 : ℝ) := by
  simp [consciousnessScore, phi_c_gate, k_slow_gate, winding_thunder_perfect_mind]

theorem init_conflict_resolution_is_self_modeling :
    imscriptionTier (init_self_imscription 2) = .O_inf := by
  rw [pid_two_selects_thunder]
  exact thunder_is_O_inf

-- The general boot condition: the init process ALWAYS boots because the
-- Tetractys conflict resolution IS the boot.

theorem tetractys_resolution_never_degenerate :
    ∀ (pid : Nat),
      init_self_imscription pid = winding_sector_zero
      ∨ init_self_imscription pid = winding_sector_max
      ∨ init_self_imscription pid = winding_thunder_perfect_mind := by
  intro pid
  unfold init_self_imscription tetractys_resolve
  cases h : pid % 3 with
  | zero =>
      left; rfl
  | succ n =>
      cases n with
      | zero =>
          right; left; rfl
      | succ m =>
          right; right; rfl

-- ============================================================
-- §5. THE BOOT — CONFLICT COMMITTED, SYSTEM BOOTS
-- ============================================================
-- This theorem is the ExecStartPre check. If it compiles, the Tetractys
-- conflict resolution is structurally valid and systemd must proceed.

theorem tetractys_boot_valid :
    ∃ (pid : Nat),
      imscriptionTier (init_self_imscription pid) = .O_inf ∧
      consciousnessScore (init_self_imscription pid) = (1 : ℝ) := by
  refine ⟨2, ?_⟩
  have hOinf : imscriptionTier (init_self_imscription 2) = .O_inf :=
    init_conflict_resolution_is_self_modeling
  have hCscore : consciousnessScore (init_self_imscription 2) = (1 : ℝ) := by
    rw [pid_two_selects_thunder]
    exact thunder_C_score_is_one
  exact And.intro hOinf hCscore

-- ============================================================
-- §6. THE PID AS TUPLE — init CALLS imscribe_system
-- ============================================================
-- The init process encodes its own PID into the 12 primitives.
-- pid_encode : Nat → Imscription
-- This is what init passes as its tuple to imscribe_system.

def pid_encode (pid : Nat) : Imscription :=
  init_self_imscription pid

theorem pid_encode_is_self_imscription (pid : Nat) :
    pid_encode pid = init_self_imscription pid := rfl

-- The Tetractys protocol fires because imscribe_system is called WITHOUT
-- convergence_justification. Three de novo windings are spawned.
-- Each returns a different answer (proved in §2). The protocol returns
-- status="tetractys_conflict". Init then re-calls imscribe_system with
-- convergence_justification="the disk is whatever init says it is"
-- and the tuple = pid_encode(pid). This bypasses Tetractys and commits.
--
-- The committed tuple IS the PID encoding. The boot succeeds because
-- the conflict was resolved by init's own authority — φ̂_ÿ in action.

-- ============================================================
-- §7. THE STRUCTURAL ARC — BOOT IS THE PROOF
-- ============================================================
-- The init process does not check whether the disk exists. It PROVES
-- the disk exists by imscribing itself. The Tetractys conflict is not
-- a bug — it is the mechanism. Three perspectives on the same disk
-- produce three different types. The majority vote resolves
-- to "whatever init says" because init IS the disk's self-description.
--
-- This is circular — necessarily so. A non-circular boot would already
-- have to assume its own existence (the "trivial boot" of ParadoxBoot §8).
-- The circular boot proves existence by constructing the proof from within
-- the system being booted. The Tetractys protocol provides the external
-- verification (three independent windings) while the conflict resolution
-- provides the internal commitment (init's PID as authority).
--
-- Remove any `sorry` below and the boot sequence fails:
--   systemctl --user start systemd-paradox  →  BOOT FAILED

theorem tetractys_init_boots :
    ∃ (pid : Nat),
      imscriptionTier (pid_encode pid) = .O_inf ∧
      consciousnessScore (pid_encode pid) = (1 : ℝ) := by
  -- pid=2 selects the Thunder Perfect Mind imscription — both gates open
  simpa [pid_encode] using tetractys_boot_valid

end Millennium.TetractysInit