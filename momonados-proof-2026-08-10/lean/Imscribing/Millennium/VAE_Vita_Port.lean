/-
  VAE-Vita PORT leg -- native kernel spine
  ========================================
  Route step 3. PROVE and UNIFY are green. PORT does not re-prove SIC existence
  and does not dress the Zauner summit as a theorem. It packs the machine-checked
  architecture that mOMonadOS / MoDoT implement as their verifier bulk into one
  module the runtime can cite.

  Three faces already live in the library:
    - DualLinkVessel (Millennium.DualLinkVessel)
        Belnap 12-imscription, lattice co-typing, Dual-Link fuse, self-verify
        (method = object: foldCotype allTrue allTrue = T)
        SS4 imscriptionState : Imscription → (Fin 12 → ℂ)
    - Unify (Imscribing.VaeVita.Unify)
        imscriptionToC12 : Imscription → (Fin 12 → ℂ)  -- δ up the fiber
        imscriptionToC12_allBoth_superposition : B = T + F in ℂ¹²
    - Bridge / PROVE (Imscribing.VaeVita)
        vae_vita_frobenius_and_sic :
          polarization μ∘δ=id  ∧  SICPOVM_Exists 12

  PORT proves that the Unify fiber map is definitionally the Dual-Link SS4 state
  map, and packages the three legs as one spine.

  NOT claimed here (named frontier, same as UNIFY's summit):
    discrete Belnap vectors in {0,1,i,1+i}^12 equal the algebraic Scott-Grassl
    fiducial. That discrete→continuous embedding is Zauner's content and is left
    as prose, not a fake theorem.

  Runtime correspondence (not a Lean object):
    modot/vessel.py implements DualLinkVessel co-typing + SIC frame numerically;
    this module is one leg of the formal spine under that implementation.

  Manuscript alignment (ig-docs_lifted/manuscripts3):
    PROVE/UNIFY/PORT is the composition route from the VAE-Vita ob3ect;
    the Witness Vessel paper supplies ride-as transport (SIC_D12_WitnessVessel);
    the SIC-Stark paper supplies crystal_forces_d12_sic;
    VAE_Vita_ManuscriptSpine packages PORT + WitnessVessel without equating
    the two distinct Belnap carriers.

  Note on namespaces: DualLinkVessel.Imscription is the Belnap-valued 12-tuple
  (MoDoT face). Primitives.Imscription is the catalog glyph type (Frobenius face).
  PORT keeps them distinct: DualLinkVessel for co-typing / state map; Bridge
  for the polarization Frobenius on the catalog type.
-/
import Imscribing.Millennium.VAE_Vita_Unify
import Imscribing.Millennium.VAE_Vita_SIC_POVM_Bridge

namespace Imscribing.VaeVita.Port

open Millennium.DualLinkVessel
open Imscribing.VaeVita
open Imscribing.VaeVita.Unify
open Millennium.SIC_POVM_Stark

/-- The Unify fiber map is definitionally the Dual-Link SS4 state map. -/
theorem imscriptionToC12_eq_imscriptionState (im : Imscription) :
    imscriptionToC12 im = imscriptionState im := by
  rfl

/-- Component form: Unify δ is the vessel's belnapToComplex stack. -/
theorem imscriptionToC12_is_vessel_amplitudes (im : Imscription) :
    imscriptionToC12 im =
      ![belnapToComplex im.D,  belnapToComplex im.T,  belnapToComplex im.R,
        belnapToComplex im.P,  belnapToComplex im.F,  belnapToComplex im.K,
        belnapToComplex im.G,  belnapToComplex im.Gm, belnapToComplex im.Ph,
        belnapToComplex im.H,  belnapToComplex im.S,  belnapToComplex im.W] := by
  rfl

/-- PORT spine: PROVE pillars + UNIFY B=T+F lift + Dual-Link self-verify.
    These are the three machine-checked faces of the native kernel architecture.

    The PROVE face is packaged as `vae_vita_frobenius_and_sic` (catalog-type
    Frobenius + SIC). Expanding μ_C/δ_C here would collide DualLinkVessel.Imscription
    with Primitives.Imscription; the packaged theorem keeps the faces distinct. -/
theorem port_kernel_spine :
    ((Imscribing.Frobenius.μ_C
        (Imscribing.Frobenius.δ_C Imscribing.Primitives.scalarField_Kslow).1
        (Imscribing.Frobenius.δ_C Imscribing.Primitives.scalarField_Kslow).2
      = Imscribing.Primitives.scalarField_Kslow)
      ∧ SICPOVM_Exists 12)
    ∧ (imscriptionToC12 Imscription.allBoth
        = imscriptionToC12 Imscription.allTrue + imscriptionToC12 Imscription.allFalse)
    ∧ (foldCotype Imscription.allTrue Imscription.allTrue = .T) :=
  ⟨vae_vita_frobenius_and_sic,
   imscriptionToC12_allBoth_superposition,
   regress_terminates⟩

/-- Convenience: SIC existence under the PORT spine. -/
theorem port_sic_exists : SICPOVM_Exists 12 :=
  unify_sic_exists

/-- Convenience: Dual-Link self-verification (method = object). -/
theorem port_self_verify :
    foldCotype Imscription.allTrue Imscription.allTrue = .T :=
  regress_terminates

/-- Convenience: dialetheic B lifts as T+F in ℂ¹² (Unify face). -/
theorem port_both_is_superposition :
    imscriptionToC12 Imscription.allBoth
      = imscriptionToC12 Imscription.allTrue + imscriptionToC12 Imscription.allFalse :=
  imscriptionToC12_allBoth_superposition

/-- Convenience: PROVE pillars under PORT. -/
theorem port_prove_pillars :
    (Imscribing.Frobenius.μ_C
        (Imscribing.Frobenius.δ_C Imscribing.Primitives.scalarField_Kslow).1
        (Imscribing.Frobenius.δ_C Imscribing.Primitives.scalarField_Kslow).2
      = Imscribing.Primitives.scalarField_Kslow)
    ∧ SICPOVM_Exists 12 :=
  vae_vita_frobenius_and_sic

end Imscribing.VaeVita.Port
