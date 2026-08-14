/-
  VAE-Vita UNIFY leg -- the coniunctio state map (raising the Djed)
  ================================================================
  Route step 2 (PROVE is done and green). The two faces already live in one
  library:
    - discrete face: Imscribing.Millennium.DualLinkVessel
        (12-Belnap imscription, lattice co-typing, belnapToComplex)
    - continuous face: SIC.D12.Embedding.crystal_forces_d12_sic : SICPOVM_Exists 12
        (the d=12 SIC in ℂ¹²), assembled with the Frobenius closure in
        Imscribing.Millennium.VAE_Vita_SIC_POVM_Bridge.

  UNIFY binds them. `imscriptionToC12` is δ, the lift up the fiber: it carries a
  discrete Belnap imscription to a state in ℂ¹², the same ℂ¹² the SIC fiducial
  lives in. The two loops are threaded through one axis (the Djed / the phase
  fiber). This module provides:
    1. the state map,
    2. the theorem that the dialetheic Both state is the superposition of the two
       links (B = T + F lifted to ℂ¹² -- the still crown held as B is the lock),
    3. the SIC existence re-exported here so both faces sit in one module.

  The remaining embedding. The discrete Belnap imscription carries only
  four values per axis, so its image is the finite set {0, 1, i, 1+i}^12; the exact
  SIC fiducial (algebraic, irrational) is NOT in that image. The real embedding is
  therefore not "a Belnap vector IS a fiducial" but the correspondence between the
  discrete co-typing lattice (foldCotype) and continuous SIC coincidence, realised
  through the ordinal Crystal (17.28M configs) on S^11 that VAE-Vita descends. That
  discrete->continuous embedding is the content of Zauner and is the peak
  of UNIFY.
-/
import Imscribing.Millennium.DualLinkVessel
import Imscribing.Millennium.VAE_Vita_SIC_POVM_Bridge

namespace Imscribing.VaeVita.Unify

open Millennium.DualLinkVessel
open Millennium.SIC_POVM_Stark

/-- δ, the lift up the fiber: a 12-Belnap imscription to a state in ℂ¹², the same
    ℂ¹² the d=12 SIC fiducial lives in. -/
noncomputable def imscriptionToC12 (im : Imscription) : Fin 12 → ℂ :=
  ![belnapToComplex im.D,  belnapToComplex im.T,  belnapToComplex im.R,
    belnapToComplex im.P,  belnapToComplex im.F,  belnapToComplex im.K,
    belnapToComplex im.G,  belnapToComplex im.Gm, belnapToComplex im.Ph,
    belnapToComplex im.H,  belnapToComplex im.S,  belnapToComplex im.W]

/-- The dialetheic Both state is the superposition of the two faces: the all-B
    imscription lifts to the sum of the all-T and all-F states. This is `B = T + F`
    carried up into ℂ¹² -- the still crown held as B is the lock of the two links. -/
theorem imscriptionToC12_allBoth_superposition :
    imscriptionToC12 Imscription.allBoth
      = imscriptionToC12 Imscription.allTrue + imscriptionToC12 Imscription.allFalse := by
  funext k
  fin_cases k <;>
    simp [imscriptionToC12, Imscription.allBoth, Imscription.allTrue,
          Imscription.allFalse, belnapToComplex, Pi.add_apply]

/-- The SIC the state map reaches for lives in the same codomain, ℂ¹². Re-exported
    into the UNIFY module so both faces are one object here. -/
theorem unify_sic_exists : SICPOVM_Exists 12 :=
  SIC.D12.Embedding.crystal_forces_d12_sic

end Imscribing.VaeVita.Unify
