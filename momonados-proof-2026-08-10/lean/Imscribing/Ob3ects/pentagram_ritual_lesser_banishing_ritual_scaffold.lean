-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Pentagram ritual (Lesser Banishing Ritual of the Pentagram): the ceremonial magic ritual that establishes sacred space. FSPLIT: the magician draws the pentagrams in the four quarters, splitting mundane space into consecrated ritual space with four elemental guardians. FFUSE: the closing reunites the sacred space with mundane space — the circle is opened, the guardians are released, but the magician retains the charge. VINIT: profane space before the ritual. TANCH: the magic circle. AFWD: invoking pentagram (spirit drawn down). AREV: banishing pentagram (spirit sent away). ENGAGR: the magician stands at the intersection of all four quarters — simultaneously in all elements and none.
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
private def pentagram_ritual_lesser_banishing_bca0ee_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def pentagram_ritual_lesser_banishing_bca0ee_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def pentagram_ritual_lesser_banishing_bca0ee_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def pentagram_ritual_lesser_banishing_bca0ee_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def pentagram_ritual_lesser_banishing_bca0ee_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def pentagram_ritual_lesser_banishing_bca0ee_protocol : IGProtocol pentagram_ritual_lesser_banishing_bca0ee_s0 pentagram_ritual_lesser_banishing_bca0ee_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct pentagram_ritual_lesser_banishing_bca0ee_s4 pentagram_ritual_lesser_banishing_bca0ee_s4 = pentagram_ritual_lesser_banishing_bca0ee_s4 (idempotent)
  (.seq (.arrow pentagram_ritual_lesser_banishing_bca0ee_l0 pentagram_ritual_lesser_banishing_bca0ee_s0 pentagram_ritual_lesser_banishing_bca0ee_s1) (.seq (.arrow pentagram_ritual_lesser_banishing_bca0ee_l1 pentagram_ritual_lesser_banishing_bca0ee_s1 pentagram_ritual_lesser_banishing_bca0ee_s2) (.seq (.prod (.arrow pentagram_ritual_lesser_banishing_bca0ee_l2 pentagram_ritual_lesser_banishing_bca0ee_s2 pentagram_ritual_lesser_banishing_bca0ee_s4) (.arrow pentagram_ritual_lesser_banishing_bca0ee_l2 pentagram_ritual_lesser_banishing_bca0ee_s2 pentagram_ritual_lesser_banishing_bca0ee_s4)) (.seq (.arrow pentagram_ritual_lesser_banishing_bca0ee_l4 pentagram_ritual_lesser_banishing_bca0ee_s4 pentagram_ritual_lesser_banishing_bca0ee_s4) (.seq (.arrow pentagram_ritual_lesser_banishing_bca0ee_l4 pentagram_ritual_lesser_banishing_bca0ee_s4 pentagram_ritual_lesser_banishing_bca0ee_s5) (.seq (.arrow pentagram_ritual_lesser_banishing_bca0ee_l5 pentagram_ritual_lesser_banishing_bca0ee_s5 pentagram_ritual_lesser_banishing_bca0ee_s6) (.arrow pentagram_ritual_lesser_banishing_bca0ee_l6 pentagram_ritual_lesser_banishing_bca0ee_s6 pentagram_ritual_lesser_banishing_bca0ee_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def pentagram_ritual_lesser_banishing_bca0ee_tier : OuroboricityTier := TierFunctor.obj pentagram_ritual_lesser_banishing_bca0ee_s0
#eval pentagram_ritual_lesser_banishing_bca0ee_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem pentagram_ritual_lesser_banishing_bca0ee_frobenius :
    igFrobeniusAlg.mul pentagram_ritual_lesser_banishing_bca0ee_s0 pentagram_ritual_lesser_banishing_bca0ee_s0 = pentagram_ritual_lesser_banishing_bca0ee_s0 :=
  igFrobAlg_self_fusion pentagram_ritual_lesser_banishing_bca0ee_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem pentagram_ritual_lesser_banishing_bca0ee_self_ref :
    (igProtoDelta pentagram_ritual_lesser_banishing_bca0ee_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem pentagram_ritual_lesser_banishing_bca0ee_loop_closure :
    ∃ (loop : IGProtocol pentagram_ritual_lesser_banishing_bca0ee_s0 pentagram_ritual_lesser_banishing_bca0ee_s7),
      loop = pentagram_ritual_lesser_banishing_bca0ee_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
