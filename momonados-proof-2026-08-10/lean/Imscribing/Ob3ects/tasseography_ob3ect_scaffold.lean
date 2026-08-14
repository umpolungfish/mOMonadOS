-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Tasseography ob3ect: tea leaf reading — loose tea leaves swirled in a cup, then inverted onto a saucer, leaving patterns on the cup walls. FSPLIT: the inverting of the cup splits the wet leaves into scattered clusters and trails across the cup's interior surface. FFUSE: the reader synthesizes the scattered leaf formations into a coherent narrative — the rim=present, middle=near future, bottom=distant future. VINIT: the empty cup. TANCH: the cup's rim as boundary. AFWD: a leaf cluster near the rim (imminent event). AREV: a leaf cluster at the bottom (distant, delayed). ENGAGR: a leaf formation that can be read as two contradictory symbols simultaneously (e.g., a bird and a snake from the same cluster).
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
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def tasseography_ob3ect_tea_leaf_reading_e9d119_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def tasseography_ob3ect_tea_leaf_reading_e9d119_protocol : IGProtocol tasseography_ob3ect_tea_leaf_reading_e9d119_s0 tasseography_ob3ect_tea_leaf_reading_e9d119_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct tasseography_ob3ect_tea_leaf_reading_e9d119_s4 tasseography_ob3ect_tea_leaf_reading_e9d119_s4 = tasseography_ob3ect_tea_leaf_reading_e9d119_s4 (idempotent)
  (.seq (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l0 tasseography_ob3ect_tea_leaf_reading_e9d119_s0 tasseography_ob3ect_tea_leaf_reading_e9d119_s1) (.seq (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l1 tasseography_ob3ect_tea_leaf_reading_e9d119_s1 tasseography_ob3ect_tea_leaf_reading_e9d119_s2) (.seq (.prod (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l2 tasseography_ob3ect_tea_leaf_reading_e9d119_s2 tasseography_ob3ect_tea_leaf_reading_e9d119_s4) (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l2 tasseography_ob3ect_tea_leaf_reading_e9d119_s2 tasseography_ob3ect_tea_leaf_reading_e9d119_s4)) (.seq (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l4 tasseography_ob3ect_tea_leaf_reading_e9d119_s4 tasseography_ob3ect_tea_leaf_reading_e9d119_s4) (.seq (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l4 tasseography_ob3ect_tea_leaf_reading_e9d119_s4 tasseography_ob3ect_tea_leaf_reading_e9d119_s5) (.seq (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l5 tasseography_ob3ect_tea_leaf_reading_e9d119_s5 tasseography_ob3ect_tea_leaf_reading_e9d119_s6) (.arrow tasseography_ob3ect_tea_leaf_reading_e9d119_l6 tasseography_ob3ect_tea_leaf_reading_e9d119_s6 tasseography_ob3ect_tea_leaf_reading_e9d119_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def tasseography_ob3ect_tea_leaf_reading_e9d119_tier : OuroboricityTier := TierFunctor.obj tasseography_ob3ect_tea_leaf_reading_e9d119_s0
#eval tasseography_ob3ect_tea_leaf_reading_e9d119_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem tasseography_ob3ect_tea_leaf_reading_e9d119_frobenius :
    igFrobeniusAlg.mul tasseography_ob3ect_tea_leaf_reading_e9d119_s0 tasseography_ob3ect_tea_leaf_reading_e9d119_s0 = tasseography_ob3ect_tea_leaf_reading_e9d119_s0 :=
  igFrobAlg_self_fusion tasseography_ob3ect_tea_leaf_reading_e9d119_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem tasseography_ob3ect_tea_leaf_reading_e9d119_self_ref :
    (igProtoDelta tasseography_ob3ect_tea_leaf_reading_e9d119_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem tasseography_ob3ect_tea_leaf_reading_e9d119_loop_closure :
    ∃ (loop : IGProtocol tasseography_ob3ect_tea_leaf_reading_e9d119_s0 tasseography_ob3ect_tea_leaf_reading_e9d119_s7),
      loop = tasseography_ob3ect_tea_leaf_reading_e9d119_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
