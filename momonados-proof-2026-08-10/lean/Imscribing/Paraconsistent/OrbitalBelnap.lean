-- Imscribing/Paraconsistent/OrbitalBelnap.lean
-- Electron orbital occupancy is isomorphic to Belnap FOUR as a bilattice.
--
-- The four states {empty, spinUp, spinDown, paired} carry two natural orderings:
--   information order (occupancy): empty < spinUp, spinDown < paired
--   truth order (chirality):       spinDown < empty, paired < spinUp
-- Both match the Belnap bilattice exactly under:
--   empty → N, spinUp → T, spinDown → F, paired → B
--
-- The Pauli exclusion principle is the anti-extensionality ceiling on B:
-- no state above `paired` in the information order exists.
--
-- The pairing/depairing morphisms (Cooper pair) satisfy pair ∘ depair = id:
-- the Frobenius condition holds at the orbital level.
--
-- Governing primitives: Ħ (Chirality) determines the T/F labeling of spin;
-- Φ (Parity) enforces the two-electron ceiling.

import Imscribing.Paraconsistent.Belnap
import Mathlib.Logic.Function.Defs

namespace Imscribing.Paraconsistent.Orbital

open Imscribing.Paraconsistent

-- ════════════════════════════════════════════════════════════════════════════
-- §1  ORBITAL STATE
-- ════════════════════════════════════════════════════════════════════════════

/-- The four occupancy states of an atomic orbital. -/
inductive OrbitalState : Type where
  | empty     -- no electrons       → N (neither)
  | spinUp    -- one electron ↑     → T (true,  positive Ħ-chirality)
  | spinDown  -- one electron ↓     → F (false, negative Ħ-chirality)
  | paired    -- two electrons ↑↓   → B (both,  Φ-sealed at lattice top)
  deriving DecidableEq, Repr, Inhabited

-- ════════════════════════════════════════════════════════════════════════════
-- §2  ORBITAL → B₄ MAPPING
-- ════════════════════════════════════════════════════════════════════════════

def orbToB4 : OrbitalState → Belnap
  | .empty    => .N
  | .spinUp   => .T
  | .spinDown => .F
  | .paired   => .B

theorem orbToB4_injective : Function.Injective orbToB4 := by
  intro a b h; cases a <;> cases b <;> simp [orbToB4] at h ⊢

theorem orbToB4_surjective : Function.Surjective orbToB4 := by
  intro b; cases b
  · exact ⟨.empty,    rfl⟩
  · exact ⟨.spinUp,   rfl⟩
  · exact ⟨.spinDown, rfl⟩
  · exact ⟨.paired,   rfl⟩

theorem orbToB4_bijective : Function.Bijective orbToB4 :=
  ⟨orbToB4_injective, orbToB4_surjective⟩

-- ════════════════════════════════════════════════════════════════════════════
-- §3  INFORMATION ORDER (occupancy / approximation)
--     empty ⊑ spinUp, spinDown ⊑ paired — mirrors N ⊑ T, F ⊑ B
-- ════════════════════════════════════════════════════════════════════════════

/-- Information order: how much occupancy information is present. -/
inductive OccupancyLE : OrbitalState → OrbitalState → Prop
  | empty_bot   : ∀ {s}, OccupancyLE .empty s
  | su_refl     : OccupancyLE .spinUp   .spinUp
  | sd_refl     : OccupancyLE .spinDown .spinDown
  | paired_refl : OccupancyLE .paired   .paired
  | su_paired   : OccupancyLE .spinUp   .paired
  | sd_paired   : OccupancyLE .spinDown .paired

instance : LE OrbitalState := ⟨OccupancyLE⟩

instance (a b : OrbitalState) : Decidable (OccupancyLE a b) := by
  cases a <;> cases b <;> first
    | exact isTrue OccupancyLE.empty_bot
    | exact isTrue OccupancyLE.su_refl
    | exact isTrue OccupancyLE.sd_refl
    | exact isTrue OccupancyLE.paired_refl
    | exact isTrue OccupancyLE.su_paired
    | exact isTrue OccupancyLE.sd_paired
    | exact isFalse (by intro h; cases h)

/-- orbToB4 preserves the information order. -/
theorem orbToB4_mono {a b : OrbitalState} (h : a ≤ b) : orbToB4 a ≤ orbToB4 b := by
  cases h <;> simp only [orbToB4] <;> constructor

/-- orbToB4 reflects the information order. -/
theorem orbToB4_reflect {a b : OrbitalState} (h : orbToB4 a ≤ orbToB4 b) : a ≤ b := by
  cases a <;> cases b <;> simp only [orbToB4] at h <;> cases h <;> constructor

/-- orbToB4 is an order isomorphism for the information order. -/
theorem orbToB4_orderIso (a b : OrbitalState) : a ≤ b ↔ orbToB4 a ≤ orbToB4 b :=
  ⟨orbToB4_mono, orbToB4_reflect⟩

-- ════════════════════════════════════════════════════════════════════════════
-- §4  PAULI EXCLUSION = ANTI-EXTENSIONALITY CEILING ON B
-- ════════════════════════════════════════════════════════════════════════════

/-- `paired` is the information-order maximum. -/
theorem paired_is_top (s : OrbitalState) : s ≤ OrbitalState.paired := by
  cases s <;> constructor

/-- Pauli exclusion: nothing lies strictly above `paired` in the information order.
    A third electron cannot be added; the B-ceiling is closed. -/
theorem pauli_exclusion {s : OrbitalState} (h : OrbitalState.paired ≤ s) :
    s = OrbitalState.paired := by
  cases h; rfl

-- ════════════════════════════════════════════════════════════════════════════
-- §5  TRUTH ORDER (chirality / Ħ-axis)
--     spinDown <_t {empty, paired} <_t spinUp — mirrors F <_t N, B <_t T
-- ════════════════════════════════════════════════════════════════════════════

/-- Truth order: how much positive Ħ-chirality is asserted. -/
inductive ChiralityLE : OrbitalState → OrbitalState → Prop
  | sd_bot      : ∀ {s}, ChiralityLE .spinDown s
  | empty_refl  : ChiralityLE .empty   .empty
  | paired_refl : ChiralityLE .paired  .paired
  | su_refl     : ChiralityLE .spinUp  .spinUp
  | empty_su    : ChiralityLE .empty   .spinUp
  | paired_su   : ChiralityLE .paired  .spinUp

/-- `spinDown` (F) is the truth-order minimum. -/
theorem spinDown_is_truth_bot (s : OrbitalState) : ChiralityLE OrbitalState.spinDown s :=
  ChiralityLE.sd_bot

/-- `spinUp` (T) is the truth-order maximum. -/
theorem spinUp_is_truth_top (s : OrbitalState) : ChiralityLE s OrbitalState.spinUp := by
  cases s <;> constructor

/-- `empty` (N) and `paired` (B) are incomparable in the truth order. -/
theorem empty_paired_truth_incomparable :
    ¬ ChiralityLE OrbitalState.empty OrbitalState.paired ∧
    ¬ ChiralityLE OrbitalState.paired OrbitalState.empty := by
  constructor <;> (intro h; cases h)

-- ════════════════════════════════════════════════════════════════════════════
-- §6  FROBENIUS: PAIRING / DEPAIRING (Cooper pair morphisms)
--     depair : OrbitalState → OrbitalState × OrbitalState   (δ)
--     pair   : OrbitalState → OrbitalState → OrbitalState   (μ)
--     pair ∘ depair = id   — the Frobenius identity at the orbital level
-- ════════════════════════════════════════════════════════════════════════════

/-- Depairing (δ): resolve an orbital into its two spin components. -/
def depair : OrbitalState → OrbitalState × OrbitalState
  | .paired   => (.spinUp, .spinDown)
  | .spinUp   => (.spinUp, .empty)
  | .spinDown => (.empty,  .spinDown)
  | .empty    => (.empty,  .empty)

/-- Pairing (μ): combine two spin components into an orbital.
    Opposite spins fill the orbital; same-spin is Pauli-blocked (keeps first). -/
def pair : OrbitalState → OrbitalState → OrbitalState
  | .paired,   _          => .paired
  | _,         .paired    => .paired
  | .spinUp,   .spinDown  => .paired
  | .spinDown, .spinUp    => .paired
  | .empty,    .empty     => .empty
  | .spinUp,   .spinUp    => .spinUp
  | .spinUp,   .empty     => .spinUp
  | .empty,    .spinUp    => .spinUp
  | .spinDown, .spinDown  => .spinDown
  | .spinDown, .empty     => .spinDown
  | .empty,    .spinDown  => .spinDown

/-- Frobenius identity: pair ∘ depair = id for all orbital states.
    For `paired`: Cooper-pair depairing and re-pairing recovers the filled orbital. -/
theorem pair_depair_id (s : OrbitalState) :
    pair (depair s).1 (depair s).2 = s := by
  cases s <;> rfl

end Imscribing.Paraconsistent.Orbital
