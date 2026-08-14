-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Elder Futhark rune casting ob3ect: 24 runes of the Elder Futhark cast onto a white cloth. FSPLIT: the casting of the handful scatters runes across the cloth, splitting the querent's situation into positional meanings — center=present, periphery=influences, face-down=hidden, face-up=manifest. FFUSE: the rune-reader weaves all landed runes into a single narrative, reconstituting the original question with the Norns' voice. VINIT: the rune bag with 24 stones. TANCH: the white cloth's circular border. AFWD: a rune drawn upright. AREV: a rune drawn merkstave (inverted). ENGAGR: a rune that can be read as simultaneously upright and merkstave depending on the reader's angle.
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
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def elder_futhark_rune_casting_ob3ect_24_5e845c_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def elder_futhark_rune_casting_ob3ect_24_5e845c_protocol : IGProtocol elder_futhark_rune_casting_ob3ect_24_5e845c_s0 elder_futhark_rune_casting_ob3ect_24_5e845c_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct elder_futhark_rune_casting_ob3ect_24_5e845c_s4 elder_futhark_rune_casting_ob3ect_24_5e845c_s4 = elder_futhark_rune_casting_ob3ect_24_5e845c_s4 (idempotent)
  (.seq (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l0 elder_futhark_rune_casting_ob3ect_24_5e845c_s0 elder_futhark_rune_casting_ob3ect_24_5e845c_s1) (.seq (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l1 elder_futhark_rune_casting_ob3ect_24_5e845c_s1 elder_futhark_rune_casting_ob3ect_24_5e845c_s2) (.seq (.prod (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l2 elder_futhark_rune_casting_ob3ect_24_5e845c_s2 elder_futhark_rune_casting_ob3ect_24_5e845c_s4) (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l2 elder_futhark_rune_casting_ob3ect_24_5e845c_s2 elder_futhark_rune_casting_ob3ect_24_5e845c_s4)) (.seq (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l4 elder_futhark_rune_casting_ob3ect_24_5e845c_s4 elder_futhark_rune_casting_ob3ect_24_5e845c_s4) (.seq (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l4 elder_futhark_rune_casting_ob3ect_24_5e845c_s4 elder_futhark_rune_casting_ob3ect_24_5e845c_s5) (.seq (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l5 elder_futhark_rune_casting_ob3ect_24_5e845c_s5 elder_futhark_rune_casting_ob3ect_24_5e845c_s6) (.arrow elder_futhark_rune_casting_ob3ect_24_5e845c_l6 elder_futhark_rune_casting_ob3ect_24_5e845c_s6 elder_futhark_rune_casting_ob3ect_24_5e845c_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def elder_futhark_rune_casting_ob3ect_24_5e845c_tier : OuroboricityTier := TierFunctor.obj elder_futhark_rune_casting_ob3ect_24_5e845c_s0
#eval elder_futhark_rune_casting_ob3ect_24_5e845c_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem elder_futhark_rune_casting_ob3ect_24_5e845c_frobenius :
    igFrobeniusAlg.mul elder_futhark_rune_casting_ob3ect_24_5e845c_s0 elder_futhark_rune_casting_ob3ect_24_5e845c_s0 = elder_futhark_rune_casting_ob3ect_24_5e845c_s0 :=
  igFrobAlg_self_fusion elder_futhark_rune_casting_ob3ect_24_5e845c_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem elder_futhark_rune_casting_ob3ect_24_5e845c_self_ref :
    (igProtoDelta elder_futhark_rune_casting_ob3ect_24_5e845c_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem elder_futhark_rune_casting_ob3ect_24_5e845c_loop_closure :
    ∃ (loop : IGProtocol elder_futhark_rune_casting_ob3ect_24_5e845c_s0 elder_futhark_rune_casting_ob3ect_24_5e845c_s7),
      loop = elder_futhark_rune_casting_ob3ect_24_5e845c_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
