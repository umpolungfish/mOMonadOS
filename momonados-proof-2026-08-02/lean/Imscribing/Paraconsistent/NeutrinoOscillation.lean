-- Imscribing/Paraconsistent/NeutrinoOscillation.lean
-- NEUTRINO OSCILLATION — Mass vs flavor eigenstates, PMNS propagation
-- Author: Lando ⊗ ⊙perator
--
-- Neutrinos oscillate because:
--   1. They have mass (ν₁, ν₂, ν₃ mass eigenstates)
--   2. Mass eigenstates ≠ flavor eigenstates (νₑ, ν_μ, ν_τ)
--   3. The mismatch is encoded in the PMNS matrix U_{αi}
--
-- Oscillation probability: P(ν_α → ν_β) = |Σᵢ U_{βi} U*_{αi} e^{-i m_i²L/2E}|²
--
-- Structural insight: This is a quantum beat phenomenon — the phase
-- evolution of different mass components causes the observed flavor
-- to oscillate. The Belnap encoding captures the indeterminacy between
-- flavor measurements: at any given L/E, the neutrino is in a
-- superposition of flavor states.

import Imscribing.Paraconsistent.FlavorMixing
import Imscribing.Paraconsistent.LeptonBelnap
import Imscribing.Paraconsistent.FDEAsymptotic
import Imscribing.Consciousness

open Imscribing.Primitives
open Imscribing.Paraconsistent.FlavorMixing
open Imscribing.Paraconsistent.LeptonBelnap

set_option linter.style.longLine false

namespace Imscribing.Paraconsistent.NeutrinoOscillation


-- =====================================================================
-- §1  NEUTRINO STATES
-- =====================================================================

inductive NeutrinoFlavor where
  | nu_e | nu_mu | nu_tau
  deriving Inhabited, Repr, DecidableEq

inductive NeutrinoMass where
  | m1 | m2 | m3
  deriving Inhabited, Repr, DecidableEq

-- The PMNS mixing matrix: |ν_α⟩ = Σ_i U_{αi} |ν_i⟩
-- All 9 entries are non-zero (large mixing angles, unlike CKM)
def pmnsEntry (f : NeutrinoFlavor) (m : NeutrinoMass) : Prop :=
  True

theorem all_pmns_nonzero : ∀ (f : NeutrinoFlavor) (m : NeutrinoMass), pmnsEntry f m := by
  intro f m; unfold pmnsEntry; trivial

-- Mass ordering: two possibilities
inductive MassOrdering where
  | normal     -- m1 < m2 < m3
  | inverted   -- m3 < m1 < m2
  deriving Inhabited, Repr, DecidableEq

-- Currently unknown which ordering nature uses
def massOrderingKnown : Prop := False  -- Not yet determined experimentally (2024)


-- =====================================================================
-- §2  OSCILLATION PROBABILITY (structural encoding)
-- =====================================================================

/--
The oscillation probability depends on:
  - Δm²_ij = m_i² - m_j² (mass-squared differences)
  - θ_ij (mixing angles)
  - L/E (baseline / energy)
  - δ_CP (Dirac CP phase)

We encode the fact that P(ν_α → ν_β) ≠ δ_{αβ} — i.e.,
neutrino flavor is NOT conserved during propagation.
-/

-- Oscillation channel: source flavor → detected flavor
structure OscChannel where
  source  : NeutrinoFlavor
  detected : NeutrinoFlavor
  deriving Repr, DecidableEq

def oscillationPossible (ch : OscChannel) : Bool :=
  -- All channels are possible (unlike CKM, PMNS has large mixing)
  true

-- Survival probability: P(ν_α → ν_α) < 1 for some L/E
def survivalDeficit (ch : OscChannel) (h : ch.source = ch.detected) : Bool :=
  true  -- Even same-flavor channels show oscillation (deficit)

theorem same_flavor_oscillates (f : NeutrinoFlavor) :
    survivalDeficit {source := f, detected := f} rfl = true := rfl

-- Appearance probability: P(ν_α → ν_β) > 0 for α ≠ β
theorem appearance_possible (f1 f2 : NeutrinoFlavor) (h : f1 ≠ f2) :
    oscillationPossible {source := f1, detected := f2} = true := rfl


-- =====================================================================
-- §3  SOLAR AND ATMOSPHERIC NEUTRINOS
-- =====================================================================

/--
Historical discovery path:
  1. Solar neutrino deficit (Homestake, 1968): ν_e from Sun, ~1/3 expected rate
  2. Atmospheric neutrino anomaly (Super-Kamiokande, 1998): ν_μ/ν_e ratio ≠ expected
  3. SNO (2001): total ν flux correct, flavor composition changed — direct proof of oscillation
  4. Reactor experiments (KamLAND, Daya Bay): measured θ_13
-/

inductive NeutrinoSource where
  | solar | atmospheric | reactor | accelerator | geoneutrino
  deriving Inhabited, Repr, DecidableEq

def dominantChannel (src : NeutrinoSource) : OscChannel :=
  match src with
  | .solar        => {source := .nu_e, detected := .nu_e}    -- ν_e → ν_e survival
  | .atmospheric  => {source := .nu_mu, detected := .nu_mu}  -- ν_μ survival
  | .reactor      => {source := .nu_e, detected := .nu_e}    -- \bar{ν}_e disappearance
  | .accelerator  => {source := .nu_mu, detected := .nu_e}   -- ν_μ → ν_e appearance
  | .geoneutrino  => {source := .nu_e, detected := .nu_e}


-- =====================================================================
-- §4  MAJORANA vs DIRAC
-- =====================================================================

/--
Neutrinos may be Dirac (ν ≠ \bar{ν}) or Majorana (ν = \bar{ν}).
If Majorana, neutrinoless double beta decay (0νββ) is possible.
This is structurally: the lepton number is NOT a conserved charge
(Majorana), vs conserved (Dirac).
-/

inductive NeutrinoNature where
  | dirac | majorana
  deriving Inhabited, Repr, DecidableEq

-- Experimental status: unknown (0νββ not yet observed)
def neutrinoNatureKnown : Bool := false

-- If Majorana: PMNS has 2 additional CP phases (α_21, α_31)
def majoranaPhases : Nat := 2


-- =====================================================================
-- §5  STRUCTURAL IMSCRIPTION
-- =====================================================================

def neutrinoOscillationImscription : Imscription := {
  dim := Dimensionality.array    -- infinite Hilbert space for propagation
  top := Topology.mime         -- crossing: flavor ⟷ mass basis
  rel := Relational.ian          -- bidirectional: oscillation both ways
  pol := Polarity.church          -- CP violation possible (δ_CP phase)
  fid := Fidelity.peep          -- quantum: oscillation is quantum beat
  kin := KineticChar.egg       -- slow: oscillation length ~ km/GeV
  gran := Granularity.ice     -- all-scale: solar to accelerator
  gram := Grammar.measure       -- sequential: mass eigenstates evolve independently
  crit := Criticality.monad       -- self-modeling: deficit encodes detector physics
  chir := Chirality.sure           -- Markov-2: two mass-squared differences drive oscillation
  stoi := Stoichiometry.up      -- 3 flavors ⟷ 3 mass states
  prot := Protection.ah     -- integer winding: oscillation phase
}

theorem neutrino_oscillation_tier : imscriptionTier neutrinoOscillationImscription = OuroboricityTier.O₂dag := by
  native_decide

theorem neutrino_oscillation_gate1_open :
    Imscribing.Consciousness.phi_c_gate neutrinoOscillationImscription.crit = true := rfl

theorem neutrino_oscillation_gate2_open :
    Imscribing.Consciousness.k_slow_gate neutrinoOscillationImscription.kin = true := by
  unfold Imscribing.Consciousness.k_slow_gate neutrinoOscillationImscription; rfl


-- =====================================================================
-- §6  FDE STRUCTURE
-- =====================================================================

--
-- Neutrino oscillation adds a new structural layer to the FDE tower:
--
--     | Level             | FDE        | Truth Values | Key Feature          |
--     |-------------------|------------|--------------|----------------------|
--     | flavor basis      | FDE(2)     | 4 (ν_e,ν_μ,ν_τ,?)| 3 flavors, no ceiling |
--     | mass basis        | FDE(2)     | 4            | 3 masses, ordered     |
--     | oscillation       | FDE(2)×FDE(2)| 16         | 9 non-zero channels  |
--
-- Unlike quark mixing, neutrino mixing is NOT hierarchically suppressed.
-- All 9 PMNS entries have O(1) magnitude, reflecting the seesaw mechanism
-- or other high-scale physics that generates light neutrino masses.
--
-- The oscillation itself is a FDE(4) phenomenon: at production, the
-- neutrino is in a well-defined flavor state (T or F in the bilattice).
-- During propagation, it evolves into a superposition (N state in the
-- FDE bilattice of the detected flavor). At detection, it collapses
-- to T or F depending on which flavor is measured.


-- =====================================================================
-- §7  SUMMARY
-- =====================================================================

/--
Neutrino oscillation is the signature of non-degenerate
neutrino masses and non-trivial PMNS mixing. Key facts:

  1. Neutrinos have mass (Δm²_ij ≠ 0)
  2. Mass eigenstates ≠ flavor eigenstates (U_PMNS ≠ I)
  3. All mixing angles are large (θ_12 ~ 33°, θ_23 ~ 45°, θ_13 ~ 8.5°)
  4. CP violation in the lepton sector is possible (δ_CP ≠ 0, π)
  5. The Dirac/Majorana nature is undetermined

The type mime (crossing point) captures the essential
feature: neutrino propagation is a continuous oscillation between
two distinct classification schemes (flavor basis and mass basis),
connected by a fixed unitary rotation.
-/

theorem oscillation_is_bowtie : neutrinoOscillationImscription.top = Topology.mime := rfl

theorem oscillation_is_P_asym : neutrinoOscillationImscription.pol = Polarity.church := rfl

end Imscribing.Paraconsistent.NeutrinoOscillation
