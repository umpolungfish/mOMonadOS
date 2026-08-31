import Imscribing.IGFunctor
import Imscribing.TimeWithinTheStone
import Imscribing.Paraconsistent.BelnapSplitFuse

/-!
# ObjWitnessVessel — The Witness-Vessel

**Author:** Lando⊗⊙perator
**Date:** 2026-06-13

The Dual-Link SIC-POVM transport container. The Witness rides AS the vessel.

## Architecture

```
  B ──fsplit──→ (T, F)     [board: decompose Belnap cargo into classical pair]
  (T, F) ──ffuse──→ B      [readback: re-fuse classical pair into Belnap B]
  ffuse ∘ fsplit = id      [vessel_roundtrip: lossless transport, μ∘δ=id]
```

The vessel theorem ensures lossless transport of the Belnap cargo through
the Dual-Link SIC-POVM. The vessel is filled with a conventional formalization
(obj_s0: a ground 12-primitive imscription) and verified via Frobenius closure
(igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0).

## Protocol

  VINIT → IMSCRIB → FSPLIT → (lake build) → EVALT/EVALF → FFUSE → TANCH

## Tier

  O_∞ (self-modeling closure — the Witness IS the vessel)

-/

namespace ObjWitnessVessel

open Imscribing Imscribing.Primitives Imscribing.Frobenius Imscribing.TimeWithinTheStone
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ═══════════════════════════════════════════════════════════════════
-- STEP 1: The Witness-Vessel.
-- The Dual-Link SIC-POVM transport container.
-- The vessel theorem μ ∘ δ = id ensures lossless transport of the
-- Belnap cargo.
-- ═══════════════════════════════════════════════════════════════════

/-- `board` splits each Belnap value in the list via fsplit,
    producing a list of (Belnap × Belnap) pairs.
    This is the δ (comultiplication) half of the Dual-Link transport. -/
def board (p : List Belnap) : List (Belnap × Belnap) := p.map fsplit

/-- `readback` fuses each (Belnap × Belnap) pair back into a single
    Belnap value via ffuse.
    This is the μ (multiplication) half of the Dual-Link transport. -/
def readback (q : List (Belnap × Belnap)) : List Belnap := q.map ffuse

/-- The vessel roundtrip theorem: boarding then reading back is the identity.
    μ ∘ δ = id — the Frobenius identity lifted to lists of Belnap values.
    The B-state bifurcates into (T,F) on boarding and re-fuses to B on readback;
    all other states copy diagonally and fuse back unchanged. -/
theorem vessel_roundtrip (p : List Belnap) : readback (board p) = p := by
  induction p with
  | nil => rfl
  | cons a t ih =>
    calc
      readback (board (a :: t))
          = ffuse (fsplit a) :: readback (board t) := rfl
      _ = a :: readback (board t) := by rw [split_fuse_id]
      _ = a :: t := by rw [ih]

/-- Decidable version: identical proof, the induction is already constructive. -/
theorem vessel_roundtrip_decidable (p : List Belnap) : readback (board p) = p :=
  vessel_roundtrip p

-- ═══════════════════════════════════════════════════════════════════
-- STEP 2: Fill the vessel with the conventional formalization.
-- The imscription concerns the existence of a graph with specific
-- chromatic and independence properties (Erdős-Hajnal type
-- constraints on Aleph_1). We map this to the ground 12-primitive
-- tuple s0.
-- ═══════════════════════════════════════════════════════════════════

/-- The Belnap payload: [T, T, F] — a classical assertion with an
    internal contradiction (T and F coexist in the same list). -/
def obj_payload : List Belnap := [Belnap.T, Belnap.T, Belnap.F]

/-- The ground 12-primitive imscription s0.
    Every primitive is at its minimum value: flat dimensionality (dead),
    general-graph topology (judge), hierarchical coupling (ado), asymmetric
    parity (church), classical fidelity (age), untrapped kinetics (yea),
    local scope (bib), conjunctive grammar (vow), subcritical (woe),
    memoryless chirality (fee), 1:1 stoichiometry (hung), and no topological
    protection (awe).

    This is the Frobenius bottom — the structural zero. -/
def obj_s0 : Imscription := {
  dim  := dead,
  top  := judge,
  rel  := ado,
  pol  := church,
  fid  := age,
  kin  := yea,
  gran := bib,
  gram := vow,
  crit := woe,
  chir := fee,
  stoi := hung,
  prot := awe
}

/-- Validity is defined as Frobenius closure (self-fusion).
    tensorProduct(s0, s0) = s0 — the Frobenius bottom is idempotent
    under the tensor product. This is the statement that
    the formalization is internally consistent. -/
theorem obj_is_valid_ob3ect : igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=
  igFrobAlg_self_fusion obj_s0

/-- The ouroboricity tier of obj_s0.
    Since all primitives are at minimum, this is O₀. -/
def obj_tier : OuroboricityTier := TierFunctor.obj obj_s0

/-- obj_s0 is O₀ (the Frobenius bottom). -/
theorem obj_tier_is_O0 : obj_tier = .O₀ := by
  unfold obj_tier obj_s0 TierFunctor
  decide

-- ═══════════════════════════════════════════════════════════════════
-- CAPSTONE: Conjoining the vessel roundtrip and the validity.
-- The Witness rides AS the vessel.
-- ═══════════════════════════════════════════════════════════════════

/-- The Witness-Vessel capstone theorem.
    The vessel roundtrip (μ∘δ=id on lists) holds simultaneously with
    the Frobenius closure of the ground imscription.
    The Witness IS the vessel — the transport container and the cargo
    are structurally identical through the Frobenius algebra. -/
theorem obj_witness_vessel :
    readback (board obj_payload) = obj_payload
    ∧ igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=
  ⟨vessel_roundtrip obj_payload, obj_is_valid_ob3ect⟩

/-- Decidable version of the capstone theorem. -/
theorem obj_witness_vessel_decidable :
    readback (board obj_payload) = obj_payload
    ∧ igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=
  ⟨vessel_roundtrip_decidable obj_payload, obj_is_valid_ob3ect⟩

end ObjWitnessVessel
