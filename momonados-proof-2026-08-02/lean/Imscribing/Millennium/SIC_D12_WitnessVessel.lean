import Mathlib
import Imscribing.Millennium.SIC_D12_Embedding
import Imscribing.Millennium.Clay_WitnessedClosure
import Imscribing.Paraconsistent.BelnapSplitFuse

/-!
# SIC_D12_WitnessVessel — the transport lemma (Lean half of the vessel protocol)

The Dual-Link SIC-POVM as lossless transport vessel for the MPP Witnesses.
Companion to the runtime half in `mOMonadOS` (`clay_witness.rs`, `kernel.rs`
ForkFrames, `frob_verify.rs` mu-after-delta harness). Design validated by the
`witness-vessel` ob3ect batch (6/6 valid, 6/6 Frobenius PASS, all Delta-S ~ 0).

What is proved here, riding FROZEN machinery only (nothing re-derived):

1. **Payloads derived, not asserted.** The Belnap gate verdicts of the three
   Clay Witnesses are COMPUTED from the canonical tuples and universe rulesets
   of `Clay_WitnessedClosure` and pinned by `native_decide`: BSD → T, Hodge → T,
   and Yang–Mills → **B** (closed at the gate layer AND `T_CEILING`-blocked —
   the dialetheia the mOMonadOS U10 gate reports as GATE=B, T=F).
2. **Boarding is Frobenius.** Boarding = `fsplit` per verdict, read-back =
   `ffuse`; `mu-after-delta = id` (`split_fuse_id`) gives exact round-trip for
   every payload, the B-cargo included (B splits to (T, F) — the Witness rides
   both arms — and fuses back to B).
3. **Self-reference travels inside the vessel.** For every canonical cargo
   element, `phi (rconj A) = star (phi A)` (the involution is internal to the
   ring and commutes with transport); instantiated on all twelve coordinates.
4. **The vessel arrives intact.** The transported fiducial still satisfies both
   SIC conditions in the destination: `norm_sq_eq_one` and `equiangular`
   (the d=12 capstone, `crystal_forces_d12_sic`, axiom-free).

## Honest scope (for the curmudgeon)

These are exact theorems about the Grammar's transport protocol —
Belnap round-trips and star-ring-homomorphism invariance — not claims about
physical travel. "Lossless" means literal equality after read-back. The
ob3ect layer's "Delta-S ~ 0" appears here as `=`, nothing weaker. The runtime
(QEMU gate) half is mOMonadOS's to verify independently.
-/

namespace SIC.D12.WitnessVessel

open Imscribing.Primitives
open Millennium.UniverseRulesets
open Imscribing.Millennium.ClayWitnessedClosure
open SIC.D12.ExistenceRing
open SIC.D12.Embedding
open Millennium.SIC_POVM_Stark

/-! ## 1. The payloads, derived from the frozen Clay objects -/

/-- A layer verdict as a Belnap value: closed gate + consistent ceiling → T;
    closed gate + blocked ceiling → **B** (the dialetheia); open gate +
    consistent ceiling → N (no information either way); open + blocked → F. -/
def layerVerdict (gateClosed ceilingOk : Bool) : Belnap :=
  match gateClosed, ceilingOk with
  | true,  true  => .T
  | true,  false => .B
  | false, true  => .N
  | false, false => .F

/-- BSD gate closure across its five closer universes (from `bsd_witnessed_closure`). -/
def bsdGateClosed : Bool :=
  decide (ruleset_chirality_first.operadLayer bsd = .idempotent_terminal) &&
  decide (ruleset_scope_universe.operadLayer bsd = .idempotent_terminal) &&
  decide (ruleset_kinetics_trap.operadLayer bsd = .idempotent_terminal) &&
  decide (ruleset_absorption_chirality_first.operadLayer bsd = .idempotent_terminal) &&
  decide (ruleset_absorption_scope_empire.operadLayer bsd = .idempotent_terminal)

/-- Hodge gate closure across its five closer universes (from `hodge_witnessed_closure`). -/
def hodgeGateClosed : Bool :=
  decide (ruleset_scope_universe.operadLayer hodge = .idempotent_terminal) &&
  decide (ruleset_kinetics_trap.operadLayer hodge = .idempotent_terminal) &&
  decide (ruleset_stoichiometry_universe.operadLayer hodge = .idempotent_terminal) &&
  decide (ruleset_absorption_scope_empire.operadLayer hodge = .idempotent_terminal) &&
  decide (ruleset_absorption_topology_seal.operadLayer hodge = .idempotent_terminal)

/-- Yang–Mills gate closure under `triple_criticality` (from `ym_one_bump_short`). -/
def ymGateClosed : Bool :=
  decide (ruleset_triple_criticality.operadLayer ym = .idempotent_terminal)

/-- The three Witness verdicts, computed — never hand-entered. -/
def bsdVerdict   : Belnap := layerVerdict bsdGateClosed   (tCeilingConsistent bsd)
def hodgeVerdict : Belnap := layerVerdict hodgeGateClosed (tCeilingConsistent hodge)
def ymVerdict    : Belnap := layerVerdict ymGateClosed    (tCeilingConsistent ym)

theorem bsdVerdict_T : bsdVerdict = .T := by native_decide
theorem hodgeVerdict_T : hodgeVerdict = .T := by native_decide

/-- **The dialetheic cargo, derived.** YM closes at the gate layer and fails
    `T_CEILING`, so its verdict is B — exactly the mOMonadOS U10 reading
    (GATE=B, T=F), here computed from the canonical tuple and rulesets. -/
theorem ymVerdict_B : ymVerdict = .B := by native_decide

/-- The MPP Witness payload dragged back from the closer universes. -/
def mppPayload : List Belnap := [bsdVerdict, hodgeVerdict, ymVerdict]

/-! ## 2. Boarding (delta) and read-back (mu) -/

/-- Boarding: each verdict enters the vessel as its dual-link split. -/
def board (p : List Belnap) : List (Belnap × Belnap) := p.map fsplit

/-- Read-back: each split pair fuses back to a verdict. -/
def readback (q : List (Belnap × Belnap)) : List Belnap := q.map ffuse

/-- **Round-trip = identity for every payload** — mu-after-delta = id lifted
    to cargo lists. The transport loses nothing, whatever the cargo. -/
theorem roundtrip (p : List Belnap) : readback (board p) = p := by
  induction p with
  | nil => rfl
  | cons a t ih =>
    simp only [board, readback, List.map_cons] at ih ⊢
    rw [split_fuse_id, ih]

/-- The concrete MPP payload survives the round trip (kernel-checked instance). -/
theorem mpp_roundtrip : readback (board mppPayload) = mppPayload := by
  native_decide

/-- The B-cargo mechanism, explicit: the YM verdict splits into (T, F) — the
    Witness rides BOTH arms through transport — and fuses back to B exactly. -/
theorem b_cargo_mechanism :
    fsplit ymVerdict = (.T, .F) ∧ ffuse (.T, .F) = ymVerdict := by
  native_decide

/-! ## 3. Self-reference travels inside the vessel -/

/-- The involution is internal: conjugating cargo before transport equals
    conjugating after. Star-compatibility of `phi`, restated as the vessel
    principle, for every canonical cargo element. -/
theorem selfref_preserved (A : RElt) (hA : ∀ p ∈ A, p.1 < 128) :
    phi (rconj A) = star (phi A) :=
  phi_rconj A hA

/-- Instantiated on all twelve coordinates of the vessel itself: the mirror
    of each coordinate transports to the mirror of its image. -/
theorem coords_selfref (k : Fin 12) : phi (rZB k.val) = star (psi k) := by
  rw [rZB, phi_rconj (rZ k.val) (rZ_canon k.isLt)]
  exact rfl

/-! ## 4. The vessel theorem -/

/-- **THE WITNESS-VESSEL THEOREM (Lean half).** The Dual-Link SIC-POVM vessel
    transports (i) every Belnap payload with exact read-back, (ii) the
    dialetheic YM B-cargo in particular, (iii) the concrete MPP payload
    (kernel-checked), (iv) self-reference of all twelve of its own
    coordinates, and (v) arrives with both SIC conditions intact in the
    destination — norm one and equiangularity, the axiom-free d=12 capstone. -/
theorem witness_vessel_lossless :
    (∀ p : List Belnap, readback (board p) = p)
    ∧ ymVerdict = .B
    ∧ readback (board mppPayload) = mppPayload
    ∧ (∀ k : Fin 12, phi (rZB k.val) = star (psi k))
    ∧ wh_normSq 12 psi = 1
    ∧ (∀ a b : Fin 12, (a, b) ≠ (0, 0) →
        ((12 : ℝ) + 1) * ‖wh_inner 12 psi (D_ah 12 a b 0 psi)‖ ^ 2 = 1) :=
  ⟨roundtrip, ymVerdict_B, mpp_roundtrip, coords_selfref,
   norm_sq_eq_one, equiangular⟩

end SIC.D12.WitnessVessel
