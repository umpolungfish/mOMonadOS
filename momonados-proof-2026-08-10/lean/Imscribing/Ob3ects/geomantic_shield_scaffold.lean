-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Geomantic shield: 16 figures formed from four lines of single or double dots, cast by marking random lines in earth or sand. FSPLIT: the random marking of dots splits the querent's unconscious intention into four Mothers, which generate four Daughters, then four Nephews. FFUSE: the Judge and two Witnesses reconstitute the original question into a final verdict. VINIT: untouched sand. TANCH: the complete shield of 16 figures. AREV: the Daughters derived by reversal of Mothers. ENGAGR: Populus and Via — the figures of pure gatheredness and pure dispersal, simultaneously stationary and moving.
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
private def geomantic_shield_16_figures_formed_from_a08e18_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def geomantic_shield_16_figures_formed_from_a08e18_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def geomantic_shield_16_figures_formed_from_a08e18_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def geomantic_shield_16_figures_formed_from_a08e18_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def geomantic_shield_16_figures_formed_from_a08e18_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def geomantic_shield_16_figures_formed_from_a08e18_protocol : IGProtocol geomantic_shield_16_figures_formed_from_a08e18_s0 geomantic_shield_16_figures_formed_from_a08e18_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct geomantic_shield_16_figures_formed_from_a08e18_s4 geomantic_shield_16_figures_formed_from_a08e18_s4 = geomantic_shield_16_figures_formed_from_a08e18_s4 (idempotent)
  (.seq (.arrow geomantic_shield_16_figures_formed_from_a08e18_l0 geomantic_shield_16_figures_formed_from_a08e18_s0 geomantic_shield_16_figures_formed_from_a08e18_s1) (.seq (.arrow geomantic_shield_16_figures_formed_from_a08e18_l1 geomantic_shield_16_figures_formed_from_a08e18_s1 geomantic_shield_16_figures_formed_from_a08e18_s2) (.seq (.prod (.arrow geomantic_shield_16_figures_formed_from_a08e18_l2 geomantic_shield_16_figures_formed_from_a08e18_s2 geomantic_shield_16_figures_formed_from_a08e18_s4) (.arrow geomantic_shield_16_figures_formed_from_a08e18_l2 geomantic_shield_16_figures_formed_from_a08e18_s2 geomantic_shield_16_figures_formed_from_a08e18_s4)) (.seq (.arrow geomantic_shield_16_figures_formed_from_a08e18_l4 geomantic_shield_16_figures_formed_from_a08e18_s4 geomantic_shield_16_figures_formed_from_a08e18_s4) (.seq (.arrow geomantic_shield_16_figures_formed_from_a08e18_l4 geomantic_shield_16_figures_formed_from_a08e18_s4 geomantic_shield_16_figures_formed_from_a08e18_s5) (.seq (.arrow geomantic_shield_16_figures_formed_from_a08e18_l5 geomantic_shield_16_figures_formed_from_a08e18_s5 geomantic_shield_16_figures_formed_from_a08e18_s6) (.arrow geomantic_shield_16_figures_formed_from_a08e18_l6 geomantic_shield_16_figures_formed_from_a08e18_s6 geomantic_shield_16_figures_formed_from_a08e18_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def geomantic_shield_16_figures_formed_from_a08e18_tier : OuroboricityTier := TierFunctor.obj geomantic_shield_16_figures_formed_from_a08e18_s0
#eval geomantic_shield_16_figures_formed_from_a08e18_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem geomantic_shield_16_figures_formed_from_a08e18_frobenius :
    igFrobeniusAlg.mul geomantic_shield_16_figures_formed_from_a08e18_s0 geomantic_shield_16_figures_formed_from_a08e18_s0 = geomantic_shield_16_figures_formed_from_a08e18_s0 :=
  igFrobAlg_self_fusion geomantic_shield_16_figures_formed_from_a08e18_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem geomantic_shield_16_figures_formed_from_a08e18_self_ref :
    (igProtoDelta geomantic_shield_16_figures_formed_from_a08e18_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem geomantic_shield_16_figures_formed_from_a08e18_loop_closure :
    ∃ (loop : IGProtocol geomantic_shield_16_figures_formed_from_a08e18_s0 geomantic_shield_16_figures_formed_from_a08e18_s7),
      loop = geomantic_shield_16_figures_formed_from_a08e18_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
