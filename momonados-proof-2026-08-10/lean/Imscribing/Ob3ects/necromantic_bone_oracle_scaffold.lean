-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Necromantic bone oracle: casting ancestral bones, teeth, and grave-earth tokens for divination with the dead. FSPLIT: the bones are cast onto the ancestor cloth, splitting the question into positional meanings — skull fragment=ancestor's voice, tooth=warning, finger bone=direction, grave-earth=the answer lies buried. FFUSE: the necromancer reads all bones together, reconstituting the dead's answer from the scattered fragments. VINIT: the uncast bag of bones. TANCH: the ancestor cloth as the boundary between living and dead. AFWD: bone falling face-up (the dead speak clearly). AREV: bone falling face-down (the dead are silent). ENGAGR: a bone that lands on edge — the dead speak and are silent simultaneously.
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
private def necromantic_bone_oracle_casting_c736fd_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def necromantic_bone_oracle_casting_c736fd_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def necromantic_bone_oracle_casting_c736fd_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def necromantic_bone_oracle_casting_c736fd_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def necromantic_bone_oracle_casting_c736fd_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def necromantic_bone_oracle_casting_c736fd_protocol : IGProtocol necromantic_bone_oracle_casting_c736fd_s0 necromantic_bone_oracle_casting_c736fd_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct necromantic_bone_oracle_casting_c736fd_s4 necromantic_bone_oracle_casting_c736fd_s4 = necromantic_bone_oracle_casting_c736fd_s4 (idempotent)
  (.seq (.arrow necromantic_bone_oracle_casting_c736fd_l0 necromantic_bone_oracle_casting_c736fd_s0 necromantic_bone_oracle_casting_c736fd_s1) (.seq (.arrow necromantic_bone_oracle_casting_c736fd_l1 necromantic_bone_oracle_casting_c736fd_s1 necromantic_bone_oracle_casting_c736fd_s2) (.seq (.prod (.arrow necromantic_bone_oracle_casting_c736fd_l2 necromantic_bone_oracle_casting_c736fd_s2 necromantic_bone_oracle_casting_c736fd_s4) (.arrow necromantic_bone_oracle_casting_c736fd_l2 necromantic_bone_oracle_casting_c736fd_s2 necromantic_bone_oracle_casting_c736fd_s4)) (.seq (.arrow necromantic_bone_oracle_casting_c736fd_l4 necromantic_bone_oracle_casting_c736fd_s4 necromantic_bone_oracle_casting_c736fd_s4) (.seq (.arrow necromantic_bone_oracle_casting_c736fd_l4 necromantic_bone_oracle_casting_c736fd_s4 necromantic_bone_oracle_casting_c736fd_s5) (.seq (.arrow necromantic_bone_oracle_casting_c736fd_l5 necromantic_bone_oracle_casting_c736fd_s5 necromantic_bone_oracle_casting_c736fd_s6) (.arrow necromantic_bone_oracle_casting_c736fd_l6 necromantic_bone_oracle_casting_c736fd_s6 necromantic_bone_oracle_casting_c736fd_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def necromantic_bone_oracle_casting_c736fd_tier : OuroboricityTier := TierFunctor.obj necromantic_bone_oracle_casting_c736fd_s0
#eval necromantic_bone_oracle_casting_c736fd_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem necromantic_bone_oracle_casting_c736fd_frobenius :
    igFrobeniusAlg.mul necromantic_bone_oracle_casting_c736fd_s0 necromantic_bone_oracle_casting_c736fd_s0 = necromantic_bone_oracle_casting_c736fd_s0 :=
  igFrobAlg_self_fusion necromantic_bone_oracle_casting_c736fd_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem necromantic_bone_oracle_casting_c736fd_self_ref :
    (igProtoDelta necromantic_bone_oracle_casting_c736fd_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem necromantic_bone_oracle_casting_c736fd_loop_closure :
    ∃ (loop : IGProtocol necromantic_bone_oracle_casting_c736fd_s0 necromantic_bone_oracle_casting_c736fd_s7),
      loop = necromantic_bone_oracle_casting_c736fd_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
