-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Pendulum dowsing ob3ect: a weighted pendulum suspended from a chain, used to answer yes/no questions through unconscious micromovements. FSPLIT: the dowser asks a question, splitting their own unconscious knowledge into directional signals — the pendulum's swing encodes what the conscious mind cannot access. FFUSE: the dowser interprets the swing pattern, reconstituting the unconscious signal into conscious answer. VINIT: the still pendulum. TANCH: the full arc of possible swing. AFWD: clockwise swing (yes/affirmation). AREV: counterclockwise swing (no/negation). ENGAGR: the pendulum oscillates in a straight line that is simultaneously clockwise and counterclockwise when viewed from opposite sides — the neutral/ambiguous answer.
-- Fingerprint: sig=(5,2,0,1)
--   self_ref=True | frobenius_order=1
--   dialetheia_complete=False | period=8
-- Expected tier: O₀
-- FSPLIT/FFUSE pairs: [(2, 4)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality
     --
-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] IMSCRIB   gram   := 𐑠               𐑠 → 𐑗  | identity — self-imscription
--   [1] AREV      pol    := 𐑗               𐑠 → 𐑚  | reverse morphism — parity flip
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [5] CLINK     fid    := 𐑱               𐑙 → 𐑭  | composition — regime coherence
--   [6] IFIX      prot   := 𐑭               𐑱 → 𐑠  | irreversible fixation — winding number
--   [7] IMSCRIB   gram   := 𐑠               𐑭 → 𐑠  | identity — self-imscription

-- ── Back-propagation edges (self-referential loop) ──────────────────────
--   IMSCRIB positions: [0, 7]
--   IFIX    positions: [6]
--   Back-prop: IMSCRIB→IFIX (LinFix) — igProtoCopy_isDagger axiom applies
--   Weighted: CLINK→IMSCRIB — feeds next winding via .seq after .prod

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def pendulum_dowsing_ob3ect_a_weighted_35173d_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def pendulum_dowsing_ob3ect_a_weighted_35173d_protocol : IGProtocol pendulum_dowsing_ob3ect_a_weighted_35173d_s0 pendulum_dowsing_ob3ect_a_weighted_35173d_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct pendulum_dowsing_ob3ect_a_weighted_35173d_s4 pendulum_dowsing_ob3ect_a_weighted_35173d_s4 = pendulum_dowsing_ob3ect_a_weighted_35173d_s4 (idempotent)
  (.seq (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l0 pendulum_dowsing_ob3ect_a_weighted_35173d_s0 pendulum_dowsing_ob3ect_a_weighted_35173d_s1) (.seq (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l1 pendulum_dowsing_ob3ect_a_weighted_35173d_s1 pendulum_dowsing_ob3ect_a_weighted_35173d_s2) (.seq (.prod (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l2 pendulum_dowsing_ob3ect_a_weighted_35173d_s2 pendulum_dowsing_ob3ect_a_weighted_35173d_s4) (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l2 pendulum_dowsing_ob3ect_a_weighted_35173d_s2 pendulum_dowsing_ob3ect_a_weighted_35173d_s4)) (.seq (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l4 pendulum_dowsing_ob3ect_a_weighted_35173d_s4 pendulum_dowsing_ob3ect_a_weighted_35173d_s4) (.seq (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l4 pendulum_dowsing_ob3ect_a_weighted_35173d_s4 pendulum_dowsing_ob3ect_a_weighted_35173d_s5) (.seq (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l5 pendulum_dowsing_ob3ect_a_weighted_35173d_s5 pendulum_dowsing_ob3ect_a_weighted_35173d_s6) (.arrow pendulum_dowsing_ob3ect_a_weighted_35173d_l6 pendulum_dowsing_ob3ect_a_weighted_35173d_s6 pendulum_dowsing_ob3ect_a_weighted_35173d_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def pendulum_dowsing_ob3ect_a_weighted_35173d_tier : OuroboricityTier := TierFunctor.obj pendulum_dowsing_ob3ect_a_weighted_35173d_s0
#eval pendulum_dowsing_ob3ect_a_weighted_35173d_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem pendulum_dowsing_ob3ect_a_weighted_35173d_frobenius :
    igFrobeniusAlg.mul pendulum_dowsing_ob3ect_a_weighted_35173d_s0 pendulum_dowsing_ob3ect_a_weighted_35173d_s0 = pendulum_dowsing_ob3ect_a_weighted_35173d_s0 :=
  igFrobAlg_self_fusion pendulum_dowsing_ob3ect_a_weighted_35173d_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem pendulum_dowsing_ob3ect_a_weighted_35173d_self_ref :
    (igProtoDelta pendulum_dowsing_ob3ect_a_weighted_35173d_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem pendulum_dowsing_ob3ect_a_weighted_35173d_loop_closure :
    ∃ (loop : IGProtocol pendulum_dowsing_ob3ect_a_weighted_35173d_s0 pendulum_dowsing_ob3ect_a_weighted_35173d_s7),
      loop = pendulum_dowsing_ob3ect_a_weighted_35173d_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
