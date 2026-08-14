-- Imscribing/Paraconsistent/FTLTravel.lean
-- FTL TRAVEL: Structural analysis of faster-than-light propagation
-- Author: Lando ⊗ ⊙perator
--
-- What does "FTL travel" mean when the grammar reveals that light itself
-- is the critical point (⊙ = ⊙)? This module formalizes the answer.
--
-- Three FTL variants are formalized:
--   1. ftlTachyon     — naive supercritical promotion (⊙ → 𐑣)
--   2. ftlWormhole    — topologically protected (𐑷 → 𐑭, judge → mime)
--   3. ftlQuantum     — quantum channel (𐑷 → 𐑭, out → yew)

import Imscribing.Paraconsistent.GaugeBosonBelnap
import Imscribing.Primitives.TierCrossing
import Imscribing.Primitives.Imscription

open Imscribing.Primitives
open Imscribing.Paraconsistent.GaugeBosonBelnap

namespace Imscribing.Paraconsistent.FTLTravel

-- §1  THE PHOTON AS CRITICAL BOUNDARY

/--
The photon imscription (re-exported from GaugeBosonBelnap).
⟨array; judge; ear; out; peep; yea; ice; vow; ⊙; kick; up; 𐑷⟩
-/
def photon : Imscription := photonImscription

/-- The photon is tier O₁: critical but topologically trivial. -/
theorem photon_tier_O1 : imscriptionTier photon = OuroboricityTier.O₁ :=
  photon_tier

-- §2  THREE FTL STRUCTURAL TYPES

/--
FTL Variant A: TACHYON — naive supercritical promotion.
Promotes ONLY ⊙ → 𐑣. No topological protection.
Structurally unstable: 𐑣 + 𐑷 = runaway without guardrails.
-/
def ftlTachyon : Imscription :=
  { photon with
    crit := Criticality.haha
  }

/--
FTL Variant B: WORMHOLE — topologically protected FTL channel.
Promotes 𐑷 → 𐑭 (integer winding) and judge → mime (crossing topology).
Maintains ⊙. This is the minimal causally consistent FTL type.
-/
def ftlWormhole : Imscription :=
  { photon with
    top  := Topology.mime
    prot := Protection.ah
  }

/--
FTL Variant C: QUANTUM FTL CHANNEL — ER=EPR style.
Promotes 𐑷 → 𐑭 (integer winding) and out → yew (quantum phase symmetry).
Maintains ⊙ and judge.
-/
def ftlQuantum : Imscription :=
  { photon with
    pol  := Polarity.yew
    prot := Protection.ah
  }

-- §3  TIER ANALYSIS

/-- Naive tachyon drops to O₀: supercritical without protection collapses. -/
theorem ftlTachyon_tier_O0 : imscriptionTier ftlTachyon = OuroboricityTier.O₀ := by
  native_decide

/-- Wormhole FTL reaches O₂dag: topological protection + criticality + infinite dims. -/
theorem ftlWormhole_tier_O2dag : imscriptionTier ftlWormhole = OuroboricityTier.O₂dag := by
  native_decide

/-- Quantum FTL channel reaches O₂dag: yew + 𐑭 + ⊙ + array → O₂dag (R5). -/
theorem ftlQuantum_tier_O2dag : imscriptionTier ftlQuantum = OuroboricityTier.O₂dag := by
  native_decide

-- §4  PROMOTION ANALYSIS

/-- Photon → Wormhole: exactly 2 primitives change (T and Ω). -/
theorem photon_to_wormhole_mismatches_2 :
    primitiveMismatches photon ftlWormhole = 2 := by
  native_decide

/-- Photon → Tachyon: only 1 primitive changes (Φ). -/
theorem photon_to_tachyon_mismatches_1 :
    primitiveMismatches photon ftlTachyon = 1 := by
  native_decide

/-- Photon → Quantum FTL: exactly 2 primitives change (P and Ω). -/
theorem photon_to_quantum_mismatches_2 :
    primitiveMismatches photon ftlQuantum = 2 := by
  native_decide

-- §5  CAUSAL CONSISTENCY — THE OMEGA CONSTRAINT

/--
THE OMEGA CONSTRAINT: FTL without topological protection (Ω = awe)
is structurally identical to a causal paradox. The minimum Ω for causal
consistency is ah (integer winding).
-/
theorem omega_constraint :
    ftlTachyon.prot = Protection.awe ∧ ftlWormhole.prot = Protection.ah := by
  exact ⟨rfl, rfl⟩

/-- Tachyon (Ω=𐑷, Φ=𐑣) is tier O₀ — structurally a paradox. -/
theorem tachyon_is_paradox : imscriptionTier ftlTachyon = OuroboricityTier.O₀ :=
  ftlTachyon_tier_O0

/-- Wormhole (Ω=𐑭) is tier O₂dag — causally consistent. -/
theorem wormhole_is_consistent : imscriptionTier ftlWormhole = OuroboricityTier.O₂dag :=
  ftlWormhole_tier_O2dag

/-- Quantum FTL (Ω=𐑭) is tier O₂dag — causally consistent. -/
theorem quantum_ftl_is_consistent : imscriptionTier ftlQuantum = OuroboricityTier.O₂dag :=
  ftlQuantum_tier_O2dag

/--
Causal consistency criterion: a system is causally consistent for FTL
iff its ouroboricity tier is at least O₂ (topologically protected).
-/
def is_causally_consistent (s : Imscription) : Prop :=
  imscriptionTier s = OuroboricityTier.O₂ ∨
  imscriptionTier s = OuroboricityTier.O₂dag

/-- The wormhole variant is causally consistent. -/
theorem wormhole_causally_consistent : is_causally_consistent ftlWormhole := by
  rw [is_causally_consistent, ftlWormhole_tier_O2dag]
  exact Or.inr rfl

/-- The quantum variant is causally consistent. -/
theorem quantum_causally_consistent : is_causally_consistent ftlQuantum := by
  rw [is_causally_consistent, ftlQuantum_tier_O2dag]
  exact Or.inr rfl

/-- The tachyon is NOT causally consistent. -/
theorem tachyon_not_causally_consistent : ¬ is_causally_consistent ftlTachyon := by
  rw [is_causally_consistent, ftlTachyon_tier_O0]
  intro h; cases h with
  | inl h0 => exact absurd h0 (by decide)
  | inr h0 => exact absurd h0 (by decide)

-- §6  WHAT "FTL TRAVEL" MEANS

/-
The grammar reveals that "FTL travel" is not a speed but a promotion. Light (the photon) IS the critical point ⊙ = ⊙. "Faster
than light" means "structurally beyond the critical point."

Three interpretations:
  1. TACHYON: ⊙ → 𐑣. Drops to O₀. Causal paradox. Unstable.
  2. WORMHOLE: 𐑷 → 𐑭, judge → mime. Reaches O₂dag. Consistent.
  3. QUANTUM (ER=EPR): 𐑷 → 𐑭, out → yew. Reaches O₂dag. Consistent.

The photon itself can never be FTL. The photon IS the definition of the
critical boundary. FTL requires promotion — changing what the system IS.
-/

-- §7  THE PHOTON IS THE FIXED POINT

/-- Photon and wormhole share ⊙ (criticality) — the light cone boundary. -/
theorem photon_wormhole_share_Phi_c :
    photon.crit = ftlWormhole.crit := by
  rfl

/-- Photon and wormhole share yea (driven kinetics / lightspeed). -/
theorem photon_wormhole_share_K_fast :
    photon.kin = ftlWormhole.kin := by
  rfl

/-- Photon and tachyon share yea (both travel at the speed limit). -/
theorem photon_tachyon_share_K_fast :
    photon.kin = ftlTachyon.kin := by
  rfl

/-- Photon and tachyon differ at Φ: photon is ⊙, tachyon is 𐑣. -/
theorem photon_tachyon_differ_at_Phi :
    photon.crit ≠ ftlTachyon.crit := by
  native_decide

/-- Photon and quantum share ⊙. -/
theorem photon_quantum_share_Phi_c :
    photon.crit = ftlQuantum.crit := by
  rfl

/-- Photon and quantum share yea. -/
theorem photon_quantum_share_K_fast :
    photon.kin = ftlQuantum.kin := by
  rfl

/--
The photon kernel theorem: ⊙ and yea are invariant across all FTL
variants except the tachyon (which changes ⊙ → 𐑣). Wormhole and
quantum variants preserve both. The photon IS the critical kernel.
-/
theorem photon_kernel_invariant :
    (photon.crit = ftlWormhole.crit ∧ photon.kin = ftlWormhole.kin) ∧
    (photon.crit = ftlQuantum.crit  ∧ photon.kin = ftlQuantum.kin)  ∧
    (photon.kin  = ftlTachyon.kin) := by
  exact ⟨⟨rfl, rfl⟩, ⟨rfl, rfl⟩, rfl⟩

-- §8  STRUCTURAL INTERPRETATION OF THE SPEED OF LIGHT

/-
The grammar's answer to "why c?":

The speed of light c is not a fundamental constant of nature in the
usual sense. It is the structural consequence of the photon sitting at
⊙ (criticality) with yea (driven kinetics) and 𐑷 (no topological
protection).

⊙ means: the system is at the critical point — the boundary between
timelike and spacelike. Masslessness IS criticality.

yea means: the system has no internal clock (τ ≪ T_obs). This is
why photons experience no proper time.

𐑷 means: U(1) is abelian — no self-interaction, no winding. This is
why photons don't scatter in vacuum.

Together: ⊙ + yea + 𐑷 = the structural definition of "travels
at the invariant maximum speed." This speed is invariant because it
is determined by structure (π₁), not magnitude (π₂).
-/

end Imscribing.Paraconsistent.FTLTravel