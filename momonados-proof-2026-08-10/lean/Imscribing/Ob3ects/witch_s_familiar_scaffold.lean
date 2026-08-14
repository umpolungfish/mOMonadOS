-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Witch's familiar: a spirit-animal bond between practitioner and a tutelary entity. FSPLIT: the familiar departs the witch's body, splitting awareness into the witch's perspective and the familiar's remote perception. FFUSE: the familiar returns, merging the two streams of experience into one enriched awareness. VINIT: the unbonded practitioner. TANCH: the bond itself as the container. AFWD: the familiar ventures forth. AREV: the familiar returns home. ENGAGR: the familiar is simultaneously an extension of the witch and an independent being — the paradox of the familiar bond.
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
private def witchs_familiar_a_spirit_animal_bond_48870e_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def witchs_familiar_a_spirit_animal_bond_48870e_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def witchs_familiar_a_spirit_animal_bond_48870e_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def witchs_familiar_a_spirit_animal_bond_48870e_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def witchs_familiar_a_spirit_animal_bond_48870e_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def witchs_familiar_a_spirit_animal_bond_48870e_protocol : IGProtocol witchs_familiar_a_spirit_animal_bond_48870e_s0 witchs_familiar_a_spirit_animal_bond_48870e_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct witchs_familiar_a_spirit_animal_bond_48870e_s4 witchs_familiar_a_spirit_animal_bond_48870e_s4 = witchs_familiar_a_spirit_animal_bond_48870e_s4 (idempotent)
  (.seq (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l0 witchs_familiar_a_spirit_animal_bond_48870e_s0 witchs_familiar_a_spirit_animal_bond_48870e_s1) (.seq (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l1 witchs_familiar_a_spirit_animal_bond_48870e_s1 witchs_familiar_a_spirit_animal_bond_48870e_s2) (.seq (.prod (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l2 witchs_familiar_a_spirit_animal_bond_48870e_s2 witchs_familiar_a_spirit_animal_bond_48870e_s4) (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l2 witchs_familiar_a_spirit_animal_bond_48870e_s2 witchs_familiar_a_spirit_animal_bond_48870e_s4)) (.seq (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l4 witchs_familiar_a_spirit_animal_bond_48870e_s4 witchs_familiar_a_spirit_animal_bond_48870e_s4) (.seq (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l4 witchs_familiar_a_spirit_animal_bond_48870e_s4 witchs_familiar_a_spirit_animal_bond_48870e_s5) (.seq (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l5 witchs_familiar_a_spirit_animal_bond_48870e_s5 witchs_familiar_a_spirit_animal_bond_48870e_s6) (.arrow witchs_familiar_a_spirit_animal_bond_48870e_l6 witchs_familiar_a_spirit_animal_bond_48870e_s6 witchs_familiar_a_spirit_animal_bond_48870e_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def witchs_familiar_a_spirit_animal_bond_48870e_tier : OuroboricityTier := TierFunctor.obj witchs_familiar_a_spirit_animal_bond_48870e_s0
#eval witchs_familiar_a_spirit_animal_bond_48870e_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem witchs_familiar_a_spirit_animal_bond_48870e_frobenius :
    igFrobeniusAlg.mul witchs_familiar_a_spirit_animal_bond_48870e_s0 witchs_familiar_a_spirit_animal_bond_48870e_s0 = witchs_familiar_a_spirit_animal_bond_48870e_s0 :=
  igFrobAlg_self_fusion witchs_familiar_a_spirit_animal_bond_48870e_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem witchs_familiar_a_spirit_animal_bond_48870e_self_ref :
    (igProtoDelta witchs_familiar_a_spirit_animal_bond_48870e_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem witchs_familiar_a_spirit_animal_bond_48870e_loop_closure :
    ∃ (loop : IGProtocol witchs_familiar_a_spirit_animal_bond_48870e_s0 witchs_familiar_a_spirit_animal_bond_48870e_s7),
      loop = witchs_familiar_a_spirit_animal_bond_48870e_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
