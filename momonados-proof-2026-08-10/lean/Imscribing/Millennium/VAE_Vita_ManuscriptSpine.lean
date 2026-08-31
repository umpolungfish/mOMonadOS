/-
  VAE-Vita Manuscript Spine
  =========================
  Binds the formal route (PROVE → UNIFY → PORT) to the manuscripts3 findings
  It does not re-prove SIC existence or the Zauner embedding.

  Manuscripts (ig-docs_lifted/manuscripts3, freeze crystalline-manuscripts3-v1):
    1. SIC-Stark-12th
         hilbert_embedding_equiv_zauner by rfl;
         crystal_forces_d12_sic : SICPOVM_Exists 12 (theorem);
         d=2048 transport closed, existence open as dialetheic B.
    2. Witness Vessel
         ride AS the vessel, not IN cargo;
         board=fsplit, readback=ffuse, μ∘δ=id (propext);
         Clay payloads derived (BSD=T, Hodge=T, YM=B);
         witness_vessel_lossless delivers d=12 fiducial still SIC.
    3. Chrysopoeia 2048
         constructive moduli program; numerical min blocked;
         open: reduced characters / unconditional existence.

  This module packages what is already green:
    - Imscribing.VaeVita.Port.port_kernel_spine
    - SIC.D12.WitnessVessel.witness_vessel_lossless

  Not covered:
    - Cargo/tensor INTO vessel refused (D–T malformation); boarding is Dual-Link.
    - Discrete Belnap stack ≠ algebraic Scott-Grassl fiducial (double cover).
    - Clay T/B are Grammar typing, not Millennium proofs.
    - d=2048 unconditional existence remains open.

  Two Belnap carriers, kept distinct:
    - Imscribing.Paraconsistent.Belnap  (DualLinkVessel / MoDoT)
    - BelnapSplitFuse.Belnap           (WitnessVessel boarding)
  The theorem is their product, not an identification of the two.
-/
import Imscribing.Millennium.VAE_Vita_Port
import Imscribing.Millennium.SIC_D12_WitnessVessel

namespace Imscribing.VaeVita.ManuscriptSpine

open Imscribing.VaeVita.Port
open SIC.D12.WitnessVessel
open Millennium.DualLinkVessel
open Imscribing.VaeVita.Unify
open Millennium.SIC_POVM_Stark
open SIC.D12.Embedding
open SIC.D12.ExistenceRing

/-- Manuscript formal spine: the PORT route conjoined with Witness-Vessel
    lossless transport. The runtime halves (MoDoT vessel.py, mOMonadOS QEMU) are
    outside Lean. -/
theorem manuscript_formal_spine :
    (
      (Imscribing.Frobenius.μ_C
          (Imscribing.Frobenius.δ_C Imscribing.Primitives.scalarField_Kslow).1
          (Imscribing.Frobenius.δ_C Imscribing.Primitives.scalarField_Kslow).2
        = Imscribing.Primitives.scalarField_Kslow)
      ∧ SICPOVM_Exists 12
      ∧ (imscriptionToC12 Imscription.allBoth
          = imscriptionToC12 Imscription.allTrue
            + imscriptionToC12 Imscription.allFalse)
      ∧ (foldCotype Imscription.allTrue Imscription.allTrue = .T)
    )
    ∧
    (
      (∀ p : List Belnap, readback (board p) = p)
      ∧ ymVerdict = .B
      ∧ readback (board mppPayload) = mppPayload
      ∧ (∀ k : Fin 12, phi (rZB k.val) = star (psi k))
      ∧ wh_normSq 12 psi = 1
      ∧ (∀ a b : Fin 12, (a, b) ≠ (0, 0) →
          ((12 : ℝ) + 1) * ‖wh_inner 12 psi (D_ah 12 a b 0 psi)‖ ^ 2 = 1)
    ) := by
  refine ⟨?port, witness_vessel_lossless⟩
  have h := port_kernel_spine
  exact ⟨h.1.1, h.1.2, h.2.1, h.2.2⟩

/-- d=12 SIC existence under the spine. -/
theorem spine_sic_exists : SICPOVM_Exists 12 :=
  port_sic_exists

/-- Dual-Link self-verification (method = object). -/
theorem spine_self_verify :
    foldCotype Imscription.allTrue Imscription.allTrue = .T :=
  port_self_verify

/-- YM Clay verdict is dialetheia B (Witness Vessel cargo). -/
theorem spine_ym_is_B : ymVerdict = .B :=
  ymVerdict_B

/-- Every Belnap payload boards and returns (ride AS, not cargo). -/
theorem spine_roundtrip (p : List Belnap) : readback (board p) = p :=
  roundtrip p

/-- Unify δ is Dual-Link SS4. -/
theorem spine_state_map (im : Imscription) :
    imscriptionToC12 im = imscriptionState im :=
  imscriptionToC12_eq_imscriptionState im

end Imscribing.VaeVita.ManuscriptSpine
