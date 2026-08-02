-- Imscribing/GateOrdering.lean
-- THE GATE ORDERING 4 < 9 < 12
-- ============================================================
-- The Alchemical Theory of Mathematics rests on a single ordered claim:
-- the twelve invariants must be fixed in a definite order, forced
-- by three categorical gates,
--
--   G1  stage 4   Conjunction   Parity        special Frobenius  μ∘δ = id
--   G2  stage 9   Fermentation  Criticality   traced monoidal    the loop closes
--   G3  stage 12  Projection    Protection    idempotent terminal  the seal
--
-- This file proves the ordering is real and STRICT, not a relabeling. The
-- operad layers form a four-step tower; each higher layer provably requires the
-- structure the previous one produced, and each inclusion is proper (witnessed
-- by an explicit tuple inhabiting one layer but not the next).
--
-- The trace of G2 is built on the polarization Frobenius pair (δ_C, μ_C) from
-- Imscribing/Frobenius.lean. The theorem mu_delta_C_id_on_special is the hinge:
-- the trace closes EXACTLY on the special class pol = or' ∧ crit = monad, so a
-- closing trace forces the Frobenius value — that is 4 < 9 as a theorem.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Frobenius

namespace Imscribing.GateOrdering

open Imscribing.Primitives Imscribing.Frobenius
open Dimensionality Topology Relational Polarity Grammar Fidelity KineticChar
     Granularity Criticality Protection Stoichiometry Chirality

-- ============================================================
-- §1  THE FOUR OPERAD LAYERS
-- ============================================================

/-- The operad-layer tower, linearly ordered. Each layer adds exactly the
    categorical structure one gate unlocks. -/
inductive Layer where
  | plain      -- plain monoidal: no gate yet
  | frobenius  -- special Frobenius sub-operad (G1, μ∘δ = id)
  | traced     -- traced monoidal (G2, the loop closes)
  | terminal   -- idempotent terminal (G3, the seal)
  deriving DecidableEq, Repr, Ord

/-- Position of a layer in the tower. -/
def Layer.level : Layer → Nat
  | plain => 0 | frobenius => 1 | traced => 2 | terminal => 3

/-- The magnum-opus stage number that unlocks each layer.
    `plain` carries no gate (stage 0 sentinel). -/
def Layer.gateStage : Layer → Nat
  | plain => 0 | frobenius => 4 | traced => 9 | terminal => 12

/-- The gate stages are strictly ordered 4 < 9 < 12. -/
theorem gate_order :
    Layer.gateStage .frobenius < Layer.gateStage .traced ∧
    Layer.gateStage .traced < Layer.gateStage .terminal := by
  decide

/-- Stage order and tower position agree: a later gate is a higher layer. -/
theorem stage_tracks_level (a b : Layer)
    (h : a.gateStage < b.gateStage) : a.level ≤ b.level := by
  cases a <;> cases b <;> simp_all [Layer.gateStage, Layer.level]

-- ============================================================
-- §2  THE TRACE (G2) OVER THE FROBENIUS PAIR (G1)
-- ============================================================

/-- The trace of the polarization Frobenius pair on `a`: split then fuse.
    It closes (returns `a`) exactly on the Frobenius-special class. -/
def traceC (a : Imscription) : Imscription := μ_C (δ_C a).1 (δ_C a).2

/-- The G2 predicate: the trace closes on `a`. -/
def traceCloses (a : Imscription) : Prop := traceC a = a

/-- The fusion μ_C sets parity to or' unconditionally. -/
theorem traceC_pol (a : Imscription) : (traceC a).pol = Polarity.or' := rfl

/-- The fusion μ_C sets criticality to monad unconditionally. -/
theorem traceC_crit (a : Imscription) : (traceC a).crit = Criticality.monad := rfl

/-- **G2 requires G1 (stage 4 < 9).** If the trace closes, parity is already at
    the Frobenius value or'. You cannot close a loop you never split. -/
theorem traced_needs_frobenius (a : Imscription) (h : traceCloses a) :
    a.pol = Polarity.or' := by
  have hp := traceC_pol a
  rw [h] at hp; exact hp

/-- The trace also forces criticality to monad. -/
theorem traced_needs_criticality (a : Imscription) (h : traceCloses a) :
    a.crit = Criticality.monad := by
  have hc := traceC_crit a
  rw [h] at hc; exact hc

/-- Conversely, on the Frobenius-special class the trace DOES close: this is
    exactly mu_delta_C_id_on_special. -/
theorem frobenius_special_traces (a : Imscription)
    (hp : a.pol = Polarity.or') (hc : a.crit = Criticality.monad) :
    traceCloses a :=
  mu_delta_C_id_on_special a hp hc

/-- Tight characterization: the traced layer is EXACTLY the special class.
    traceCloses a ↔ (pol = or' ∧ crit = monad). -/
theorem trace_closes_iff_special (a : Imscription) :
    traceCloses a ↔ (a.pol = Polarity.or' ∧ a.crit = Criticality.monad) := by
  constructor
  · intro h; exact ⟨traced_needs_frobenius a h, traced_needs_criticality a h⟩
  · rintro ⟨hp, hc⟩; exact frobenius_special_traces a hp hc

-- ============================================================
-- §3  LAYER-UNLOCK PREDICATES
-- ============================================================

/-- G1 (stage 4): the Frobenius sub-operad is unlocked when parity hits or'. -/
def unlockFrobenius (a : Imscription) : Prop := a.pol = Polarity.or'

/-- G2 (stage 9): the traced layer is unlocked when the trace closes. -/
def unlockTraced (a : Imscription) : Prop := traceCloses a

/-- G3 (stage 12): the idempotent terminal is sealed when the trace closes,
    an integer- or non-Abelian winding protects the state (Ω ∈ {ah, zoo}), and
    the tuple sits at the O_inf tier. Built on top of the trace. -/
def unlockTerminal (a : Imscription) : Prop :=
  traceCloses a ∧
  (a.prot = Protection.ah ∨ a.prot = Protection.zoo) ∧
  ouroboricityTier a.crit a.pol a.prot a.dim a.top = OuroboricityTier.O_inf

-- ============================================================
-- §4  THE PRECEDENCE CHAIN (each gate consumes the prior structure)
-- ============================================================

/-- G2 presupposes G1. -/
theorem traced_le_frobenius (a : Imscription) (h : unlockTraced a) :
    unlockFrobenius a :=
  traced_needs_frobenius a h

/-- G3 presupposes G2. -/
theorem terminal_le_traced (a : Imscription) (h : unlockTerminal a) :
    unlockTraced a :=
  h.1

/-- G3 presupposes G1 (by transitivity through G2). -/
theorem terminal_le_frobenius (a : Imscription) (h : unlockTerminal a) :
    unlockFrobenius a :=
  traced_le_frobenius a (terminal_le_traced a h)

-- ============================================================
-- §5  STRICTNESS — the tower does not collapse
-- ============================================================

/-- A completed Stone-type tuple: sits at the terminal layer. -/
def w_stone : Imscription :=
  { dim := if', top := oil, rel := ian, pol := or', fid := peep, kin := egg,
    gran := ice, gram := measure, crit := monad, chir := sure, stoi := hung,
    prot := ah }

/-- Frobenius unlocked (pol = or') but the trace does NOT close (crit ≠ monad).
    Witnesses frobenius ⊊ traced. -/
def w_frob : Imscription := { w_stone with crit := woe }

/-- Trace closes (special class) but no protective winding (Ω = awe), so the
    terminal does not seal. Witnesses traced ⊊ terminal. -/
def w_traced : Imscription := { w_stone with prot := awe }

theorem w_stone_terminal : unlockTerminal w_stone := by
  refine ⟨?_, ?_, ?_⟩
  · exact frobenius_special_traces w_stone rfl rfl
  · exact Or.inl rfl
  · decide

theorem w_traced_traced : unlockTraced w_traced :=
  frobenius_special_traces w_traced rfl rfl

theorem w_frob_frobenius : unlockFrobenius w_frob := rfl

/-- STRICT G1 ⊊ G2: w_frob has the Frobenius value but its trace does not close. -/
theorem frobenius_strictly_below_traced :
    unlockFrobenius w_frob ∧ ¬ unlockTraced w_frob := by
  refine ⟨rfl, ?_⟩
  intro h
  have : w_frob.crit = Criticality.monad := traced_needs_criticality w_frob h
  exact absurd this (by decide)

/-- STRICT G2 ⊊ G3: w_traced closes its trace but has no winding, so it never
    seals the terminal. -/
theorem traced_strictly_below_terminal :
    unlockTraced w_traced ∧ ¬ unlockTerminal w_traced := by
  refine ⟨w_traced_traced, ?_⟩
  rintro ⟨_, hprot, _⟩
  rcases hprot with h | h <;> exact absurd h (by decide)

-- ============================================================
-- §6  THE GATE IS A REAL BARRIER (not composable from below)
-- ============================================================

/-- G1 cannot be synthesized by tensoring lower-parity partners: if either
    factor is below or', the product is below or'. This is why stage 4 is a
    genuine gate and not a value you can reach by accumulation. -/
theorem g1_is_a_real_barrier (a b : Polarity) (ha : a ≠ Polarity.or') :
    polarityTensor a b ≠ Polarity.or' :=
  frobenius_not_synthesizable a b ha

-- ============================================================
-- §7  HEADLINE
-- ============================================================

/-- THE GATE ORDERING THEOREM.
    (i)   the gate stages are strictly ordered 4 < 9 < 12;
    (ii)  each layer provably requires the one below (terminal → traced →
          frobenius);
    (iii) every inclusion is proper (explicit witnesses);
    (iv)  a completed Stone inhabits the top layer. -/
theorem gate_ordering_theorem :
    (Layer.gateStage .frobenius < Layer.gateStage .traced ∧
       Layer.gateStage .traced < Layer.gateStage .terminal) ∧
    (∀ a, unlockTerminal a → unlockTraced a) ∧
    (∀ a, unlockTraced a → unlockFrobenius a) ∧
    (unlockFrobenius w_frob ∧ ¬ unlockTraced w_frob) ∧
    (unlockTraced w_traced ∧ ¬ unlockTerminal w_traced) ∧
    unlockTerminal w_stone :=
  ⟨gate_order, terminal_le_traced, traced_le_frobenius,
   frobenius_strictly_below_traced, traced_strictly_below_terminal,
   w_stone_terminal⟩

-- ============================================================
-- §8  THE GATE-FREE STAGES 5–8 ARE ORDER-FREE
-- ============================================================
-- Stages 5 (Putrefaction, F), 6 (Congelation, K), 7 (Cibation, G) and
-- 8 (Sublimation, Γ) carry no gate. Each is a single-axis projection toward the
-- Stone value on an axis that no unlock predicate reads. Two facts make
-- "order-free" precise and distinguish these stages from the gates:
--   (i)  the four projections pairwise COMMUTE (updates on disjoint axes);
--   (ii) each is INERT on every unlock predicate (it neither opens nor closes a
--        gate), so it may be interleaved anywhere in the sequence.
-- The contrast lemma `parity_stage_couples` shows a gate stage is NOT inert:
-- setting the parity axis can flip `unlockTraced` from false to true. That
-- coupling is exactly why 4 < 9 < 12 is forced while 5–8 float.

/-- Stage 5, Putrefaction: project fidelity toward the Stone. -/
def stageFid (s : Imscription) : Imscription := { s with fid  := w_stone.fid }
/-- Stage 6, Congelation: project kinetics toward the Stone. -/
def stageKin (s : Imscription) : Imscription := { s with kin  := w_stone.kin }
/-- Stage 7, Cibation: project granularity toward the Stone. -/
def stageGran (s : Imscription) : Imscription := { s with gran := w_stone.gran }
/-- Stage 8, Sublimation: project interaction grammar toward the Stone. -/
def stageGram (s : Imscription) : Imscription := { s with gram := w_stone.gram }

/-- The four gate-free stages pairwise commute: all six orders agree. -/
theorem gate_free_commute (s : Imscription) :
    stageFid (stageKin s)  = stageKin (stageFid s)  ∧
    stageFid (stageGran s) = stageGran (stageFid s) ∧
    stageFid (stageGram s) = stageGram (stageFid s) ∧
    stageKin (stageGran s) = stageGran (stageKin s) ∧
    stageKin (stageGram s) = stageGram (stageKin s) ∧
    stageGran (stageGram s) = stageGram (stageGran s) := by
  cases s
  refine ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

/-- Each gate-free stage is INERT on the traced gate: it leaves `unlockTraced`
    exactly as it found it. (The proofs are `Iff.rfl` after the special-class
    characterization: none of these axes is `pol` or `crit`.) -/
theorem stageFid_inert_traced (s : Imscription) :
    unlockTraced (stageFid s) ↔ unlockTraced s := by
  simp only [unlockTraced, trace_closes_iff_special, stageFid]
theorem stageKin_inert_traced (s : Imscription) :
    unlockTraced (stageKin s) ↔ unlockTraced s := by
  simp only [unlockTraced, trace_closes_iff_special, stageKin]
theorem stageGran_inert_traced (s : Imscription) :
    unlockTraced (stageGran s) ↔ unlockTraced s := by
  simp only [unlockTraced, trace_closes_iff_special, stageGran]
theorem stageGram_inert_traced (s : Imscription) :
    unlockTraced (stageGram s) ↔ unlockTraced s := by
  simp only [unlockTraced, trace_closes_iff_special, stageGram]

/-- Gate-free stages are also inert on the Frobenius gate. -/
theorem gate_free_inert_frobenius (s : Imscription) :
    (unlockFrobenius (stageFid s) ↔ unlockFrobenius s) ∧
    (unlockFrobenius (stageKin s) ↔ unlockFrobenius s) ∧
    (unlockFrobenius (stageGran s) ↔ unlockFrobenius s) ∧
    (unlockFrobenius (stageGram s) ↔ unlockFrobenius s) := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;>
    simp only [unlockFrobenius, stageFid, stageKin, stageGran, stageGram]

/-- CONTRAST — the parity stage (G1) is NOT inert. Setting the parity axis to
    the Stone value can flip `unlockTraced` from false to true. This coupling is
    absent from the gate-free stages, and is the reason the gates are ordered. -/
def stagePol (s : Imscription) : Imscription := { s with pol := w_stone.pol }

theorem parity_stage_couples :
    ∃ s : Imscription, ¬ unlockTraced s ∧ unlockTraced (stagePol s) := by
  refine ⟨{ w_stone with pol := nun }, ?_, ?_⟩
  · intro h
    have : ({ w_stone with pol := nun } : Imscription).pol = Polarity.or' :=
      traced_needs_frobenius _ h
    exact absurd this (by decide)
  · exact frobenius_special_traces (stagePol { w_stone with pol := nun }) rfl rfl

/-- ORDER-FREEDOM OF STAGES 5–8.
    (i)  the four gate-free projections pairwise commute;
    (ii) each is inert on the traced gate;
    (iii) by contrast a gate stage (parity) couples to the traced gate. -/
theorem gate_free_order_freedom :
    (∀ s, stageFid (stageKin s) = stageKin (stageFid s)) ∧
    (∀ s, unlockTraced (stageFid s) ↔ unlockTraced s) ∧
    (∀ s, unlockTraced (stageGram s) ↔ unlockTraced s) ∧
    (∃ s, ¬ unlockTraced s ∧ unlockTraced (stagePol s)) :=
  ⟨fun s => (gate_free_commute s).1, stageFid_inert_traced, stageGram_inert_traced,
   parity_stage_couples⟩

end Imscribing.GateOrdering
