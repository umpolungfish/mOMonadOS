-- IGProtocol scaffold: VINIT → FSPLIT → EVALT → AFWD → EVALF → AREV → ENGAGR → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: scan P_bridge
-- Fingerprint: sig=(6,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=12
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 7)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [5] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [6] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [7] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [8] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [9] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [10] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [11] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def scan_p_bridge_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def scan_p_bridge_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def scan_p_bridge_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def scan_p_bridge_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def scan_p_bridge_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def scan_p_bridge_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def scan_p_bridge_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def scan_p_bridge_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def scan_p_bridge_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def scan_p_bridge_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def scan_p_bridge_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def scan_p_bridge_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def scan_p_bridge_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def scan_p_bridge_protocol : IGProtocol scan_p_bridge_s0 scan_p_bridge_s11 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct scan_p_bridge_s7 scan_p_bridge_s7 = scan_p_bridge_s7 (idempotent)
  (.seq (.arrow scan_p_bridge_l0 scan_p_bridge_s0 scan_p_bridge_s1) (.seq (.prod (.arrow scan_p_bridge_l1 scan_p_bridge_s1 scan_p_bridge_s7) (.arrow scan_p_bridge_l1 scan_p_bridge_s1 scan_p_bridge_s7)) (.seq (.arrow scan_p_bridge_l7 scan_p_bridge_s7 scan_p_bridge_s7) (.seq (.arrow scan_p_bridge_l7 scan_p_bridge_s7 scan_p_bridge_s8) (.seq (.arrow scan_p_bridge_l8 scan_p_bridge_s8 scan_p_bridge_s9) (.seq (.arrow scan_p_bridge_l9 scan_p_bridge_s9 scan_p_bridge_s10) (.arrow scan_p_bridge_l10 scan_p_bridge_s10 scan_p_bridge_s11)))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def scan_p_bridge_true_arm : IGProtocol scan_p_bridge_s0 scan_p_bridge_s11 :=
  (scan_p_bridge_protocol).restrictToEVALT

-- false arm
noncomputable def scan_p_bridge_false_arm : IGProtocol scan_p_bridge_s0 scan_p_bridge_s11 :=
  (scan_p_bridge_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def scan_p_bridge_tier : OuroboricityTier := TierFunctor.obj scan_p_bridge_s0
#eval scan_p_bridge_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem scan_p_bridge_frobenius :
    igFrobeniusAlg.mul scan_p_bridge_s0 scan_p_bridge_s0 = scan_p_bridge_s0 :=
  igFrobAlg_self_fusion scan_p_bridge_s0
