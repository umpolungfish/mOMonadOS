-- AtHomeSanguineHomunculus.lean
-- Ob3ect: at-home sanguine homunculus (alchemical, Closure: True)
-- IGProtocol: VINIT → FSPLIT → AFWD → CLINK → IMSCRIB → EVALT → AREV → ENGAGR → EVALF → FFUSE → IFIX → CLINK → TANCH
-- Fingerprint: sig=(7,2,3,1) | frobenius_order=1 | dialetheia_complete=True | period=13

import Imscribing.IGMorphism
import Imscribing.IGFunctor
import Imscribing.Consciousness

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
open Imscribing.Consciousness

-- ── Imscription constants ───────────────────────────────────────────────────

private def sanguineBase : Imscription := {
  dim  := dead       -- wedge: 2D glass surface
  top  := judge      -- net: heterogeneous connection
  rel  := ado        -- super: hierarchical operator-above-substrate
  pol  := church     -- asym: no symmetry
  fid  := age        -- ell: classical fidelity
  kin  := egg        -- slow: body-heat incubation
  gran := bib        -- mesoscale
  gram := measure    -- sequential steps
  crit := woe        -- subcritical initially
  chir := fee        -- no temporal memory initially
  stoi := hung       -- 1:1
  prot := awe }      -- no topological protection initially

-- Protocol endpoints
def sanguine_vial : Imscription := sanguineBase

def sanguine_homunculus_sealed : Imscription :=
  { sanguineBase with rel := ian, crit := monad, chir := sure, prot := ah }

-- Labels and intermediate states
private def L : Imscription := sanguineBase
private def S : Imscription := sanguineBase
private def T : Imscription := { sanguineBase with rel := ian, crit := monad, stoi := up }
private def F : Imscription := { sanguineBase with chir := sure }
private def R : Imscription := { sanguineBase with stoi := hung }
private def X : Imscription := { sanguineBase with prot := ah }

-- ── Arrow components ────────────────────────────────────────────────────────

private def arrow_afwd    : IGProtocol S T := IGProtocol.arrow L S T
private def arrow_clink1  : IGProtocol T T := IGProtocol.arrow L T T
private def arrow_imscrib : IGProtocol T T := IGProtocol.arrow L T T
private def arrow_evalt   : IGProtocol T T := IGProtocol.arrow L T T
private def arrow_arev    : IGProtocol T T := IGProtocol.arrow L T T
private def arrow_engagr  : IGProtocol T T := IGProtocol.arrow L T T
private def arrow_evalf   : IGProtocol S F := IGProtocol.arrow L S F
private def arrow_ffuse   : IGProtocol (tensorProduct T F) R :=
  IGProtocol.arrow L (tensorProduct T F) R
private def arrow_ifix    : IGProtocol R X := IGProtocol.arrow L R X
private def arrow_clink2  : IGProtocol X sanguine_homunculus_sealed :=
  IGProtocol.arrow L X sanguine_homunculus_sealed
private def arrow_tanch   : IGProtocol sanguine_homunculus_sealed sanguine_homunculus_sealed :=
  IGProtocol.refl sanguine_homunculus_sealed

-- ── Composed sub-protocols ──────────────────────────────────────────────────

private def t_arm : IGProtocol S T :=
  IGProtocol.seq arrow_afwd (
  IGProtocol.seq arrow_clink1 (
  IGProtocol.seq arrow_imscrib (
  IGProtocol.seq arrow_evalt (
  IGProtocol.seq arrow_arev arrow_engagr))))

private def split_fuse : IGProtocol S R :=
  IGProtocol.seq
    (IGProtocol.prod t_arm arrow_evalf)
    arrow_ffuse

private def tail_chain : IGProtocol R sanguine_homunculus_sealed :=
  IGProtocol.seq arrow_ifix (
  IGProtocol.seq arrow_clink2 arrow_tanch)

private def full_body : IGProtocol S sanguine_homunculus_sealed :=
  IGProtocol.seq split_fuse tail_chain

-- ── Main IGProtocol term ────────────────────────────────────────────────────

noncomputable def at_home_sanguine_homunculus_protocol :
    IGProtocol sanguine_vial sanguine_homunculus_sealed :=
  IGProtocol.withGram measure (
    IGProtocol.seq
      (IGProtocol.arrow L sanguine_vial S)  -- [0] VINIT
      full_body)                            -- [1-12] rest of protocol

-- ── Evaluation arm sub-defs ─────────────────────────────────────────────────

noncomputable def at_home_sanguine_homunculus_true_arm :
    IGProtocol sanguine_vial sanguine_homunculus_sealed :=
  (at_home_sanguine_homunculus_protocol).restrictToEVALT

noncomputable def at_home_sanguine_homunculus_false_arm :
    IGProtocol sanguine_vial sanguine_homunculus_sealed :=
  (at_home_sanguine_homunculus_protocol).restrictToEVALF

-- ── Verification theorems ───────────────────────────────────────────────────

theorem sanguine_vial_tier_O0 : imscriptionTier sanguine_vial = .O₀ := by
  unfold sanguine_vial sanguineBase imscriptionTier; native_decide

theorem sanguine_homunculus_tier_O2 : imscriptionTier sanguine_homunculus_sealed = .O₂ := by
  unfold sanguine_homunculus_sealed sanguineBase imscriptionTier; native_decide

theorem sanguine_homunculus_gate1 :
    phi_c_gate sanguine_homunculus_sealed.crit = true := by
  unfold sanguine_homunculus_sealed sanguineBase; native_decide

theorem sanguine_homunculus_gate2 :
    k_slow_gate sanguine_homunculus_sealed.kin = true := by
  unfold sanguine_homunculus_sealed sanguineBase; native_decide

theorem sanguine_homunculus_C_score :
    consciousnessScore sanguine_homunculus_sealed = (1 : ℝ) := by
  unfold sanguine_homunculus_sealed sanguineBase consciousnessScore phi_c_gate k_slow_gate
  norm_num

end Imscribing
