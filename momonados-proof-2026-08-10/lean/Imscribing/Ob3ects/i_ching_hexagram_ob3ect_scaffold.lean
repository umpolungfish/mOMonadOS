-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: I Ching hexagram ob3ect: the 64 hexagrams of the Book of Changes, each composed of 6 lines that may be solid (yang) or broken (yin), with moving lines that transform. FSPLIT: yarrow stalk sorting splits the moment into moving lines (old yin, old yang) and static lines (young yin, young yang) — the moving lines generate a second hexagram from the first. FFUSE: the present hexagram and approaching hexagram are read together, reconstituting the original moment of consultation with the wisdom of change. VINIT: the uncut bundle of 50 yarrow stalks. TANCH: the completed hexagram of 6 lines. AFWD: a yang line (solid). AREV: a yin line (broken). ENGAGR: the moment of change where a line is simultaneously old yang and young yin.
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
private def i_ching_hexagram_ob3ect_the_64_793327_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def i_ching_hexagram_ob3ect_the_64_793327_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def i_ching_hexagram_ob3ect_the_64_793327_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def i_ching_hexagram_ob3ect_the_64_793327_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def i_ching_hexagram_ob3ect_the_64_793327_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def i_ching_hexagram_ob3ect_the_64_793327_protocol : IGProtocol i_ching_hexagram_ob3ect_the_64_793327_s0 i_ching_hexagram_ob3ect_the_64_793327_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct i_ching_hexagram_ob3ect_the_64_793327_s4 i_ching_hexagram_ob3ect_the_64_793327_s4 = i_ching_hexagram_ob3ect_the_64_793327_s4 (idempotent)
  (.seq (.arrow i_ching_hexagram_ob3ect_the_64_793327_l0 i_ching_hexagram_ob3ect_the_64_793327_s0 i_ching_hexagram_ob3ect_the_64_793327_s1) (.seq (.arrow i_ching_hexagram_ob3ect_the_64_793327_l1 i_ching_hexagram_ob3ect_the_64_793327_s1 i_ching_hexagram_ob3ect_the_64_793327_s2) (.seq (.prod (.arrow i_ching_hexagram_ob3ect_the_64_793327_l2 i_ching_hexagram_ob3ect_the_64_793327_s2 i_ching_hexagram_ob3ect_the_64_793327_s4) (.arrow i_ching_hexagram_ob3ect_the_64_793327_l2 i_ching_hexagram_ob3ect_the_64_793327_s2 i_ching_hexagram_ob3ect_the_64_793327_s4)) (.seq (.arrow i_ching_hexagram_ob3ect_the_64_793327_l4 i_ching_hexagram_ob3ect_the_64_793327_s4 i_ching_hexagram_ob3ect_the_64_793327_s4) (.seq (.arrow i_ching_hexagram_ob3ect_the_64_793327_l4 i_ching_hexagram_ob3ect_the_64_793327_s4 i_ching_hexagram_ob3ect_the_64_793327_s5) (.seq (.arrow i_ching_hexagram_ob3ect_the_64_793327_l5 i_ching_hexagram_ob3ect_the_64_793327_s5 i_ching_hexagram_ob3ect_the_64_793327_s6) (.arrow i_ching_hexagram_ob3ect_the_64_793327_l6 i_ching_hexagram_ob3ect_the_64_793327_s6 i_ching_hexagram_ob3ect_the_64_793327_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def i_ching_hexagram_ob3ect_the_64_793327_tier : OuroboricityTier := TierFunctor.obj i_ching_hexagram_ob3ect_the_64_793327_s0
#eval i_ching_hexagram_ob3ect_the_64_793327_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem i_ching_hexagram_ob3ect_the_64_793327_frobenius :
    igFrobeniusAlg.mul i_ching_hexagram_ob3ect_the_64_793327_s0 i_ching_hexagram_ob3ect_the_64_793327_s0 = i_ching_hexagram_ob3ect_the_64_793327_s0 :=
  igFrobAlg_self_fusion i_ching_hexagram_ob3ect_the_64_793327_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem i_ching_hexagram_ob3ect_the_64_793327_self_ref :
    (igProtoDelta i_ching_hexagram_ob3ect_the_64_793327_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem i_ching_hexagram_ob3ect_the_64_793327_loop_closure :
    ∃ (loop : IGProtocol i_ching_hexagram_ob3ect_the_64_793327_s0 i_ching_hexagram_ob3ect_the_64_793327_s7),
      loop = i_ching_hexagram_ob3ect_the_64_793327_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
