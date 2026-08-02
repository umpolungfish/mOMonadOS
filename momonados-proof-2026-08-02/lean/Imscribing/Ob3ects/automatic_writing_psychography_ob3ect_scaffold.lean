-- IGProtocol scaffold: IMSCRIB → AREV → FSPLIT → AFWD → FFUSE → CLINK → IFIX → IMSCRIB
-- Class: Automatic writing / psychography ob3ect: the spiritualist practice where a medium's hand writes messages from spirits, the subconscious, or the collective unconscious without conscious control. FSPLIT: the medium enters trance, splitting consciousness into the observing self (watching the hand move) and the writing self (the hand producing text autonomously). FFUSE: the medium reads the written text, reconstituting the two streams — the message integrates into conscious awareness. VINIT: the blank page and still hand. TANCH: the filled page. AFWD: the pen moves forward across the page. AREV: the pen retraces or crosses out — spirit editing. ENGAGR: the writing is simultaneously from the medium (their hand wrote it) and not from the medium (they did not consciously author it).
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
private def automatic_writing_psychography_ob3ect_362a82_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_s3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_s4 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_s5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_s6 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def automatic_writing_psychography_ob3ect_362a82_s7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
  --
-- ── Label Imscriptions (per-node delta) ─────────────────────
private def automatic_writing_psychography_ob3ect_362a82_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def automatic_writing_psychography_ob3ect_362a82_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def automatic_writing_psychography_ob3ect_362a82_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
  --
-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def automatic_writing_psychography_ob3ect_362a82_protocol : IGProtocol automatic_writing_psychography_ob3ect_362a82_s0 automatic_writing_psychography_ob3ect_362a82_s7 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct automatic_writing_psychography_ob3ect_362a82_s4 automatic_writing_psychography_ob3ect_362a82_s4 = automatic_writing_psychography_ob3ect_362a82_s4 (idempotent)
  (.seq (.arrow automatic_writing_psychography_ob3ect_362a82_l0 automatic_writing_psychography_ob3ect_362a82_s0 automatic_writing_psychography_ob3ect_362a82_s1) (.seq (.arrow automatic_writing_psychography_ob3ect_362a82_l1 automatic_writing_psychography_ob3ect_362a82_s1 automatic_writing_psychography_ob3ect_362a82_s2) (.seq (.prod (.arrow automatic_writing_psychography_ob3ect_362a82_l2 automatic_writing_psychography_ob3ect_362a82_s2 automatic_writing_psychography_ob3ect_362a82_s4) (.arrow automatic_writing_psychography_ob3ect_362a82_l2 automatic_writing_psychography_ob3ect_362a82_s2 automatic_writing_psychography_ob3ect_362a82_s4)) (.seq (.arrow automatic_writing_psychography_ob3ect_362a82_l4 automatic_writing_psychography_ob3ect_362a82_s4 automatic_writing_psychography_ob3ect_362a82_s4) (.seq (.arrow automatic_writing_psychography_ob3ect_362a82_l4 automatic_writing_psychography_ob3ect_362a82_s4 automatic_writing_psychography_ob3ect_362a82_s5) (.seq (.arrow automatic_writing_psychography_ob3ect_362a82_l5 automatic_writing_psychography_ob3ect_362a82_s5 automatic_writing_psychography_ob3ect_362a82_s6) (.arrow automatic_writing_psychography_ob3ect_362a82_l6 automatic_writing_psychography_ob3ect_362a82_s6 automatic_writing_psychography_ob3ect_362a82_s7)))))))
  --
-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₀.
def automatic_writing_psychography_ob3ect_362a82_tier : OuroboricityTier := TierFunctor.obj automatic_writing_psychography_ob3ect_362a82_s0
#eval automatic_writing_psychography_ob3ect_362a82_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem automatic_writing_psychography_ob3ect_362a82_frobenius :
    igFrobeniusAlg.mul automatic_writing_psychography_ob3ect_362a82_s0 automatic_writing_psychography_ob3ect_362a82_s0 = automatic_writing_psychography_ob3ect_362a82_s0 :=
  igFrobAlg_self_fusion automatic_writing_psychography_ob3ect_362a82_s0
  --
-- Self-reference: Δ is a dagger and μ = Δ†
theorem automatic_writing_psychography_ob3ect_362a82_self_ref :
    (igProtoDelta automatic_writing_psychography_ob3ect_362a82_s0 (by decide)).isDagger = true ∧
    igProtoMu_depth (paralogical_dagger (by decide)) = 1 := by
  constructor
  · exact igProtoCopy_isDagger
  · exact igProtoMu_depth
  --
-- Loop closure: period=8, depth=1
theorem automatic_writing_psychography_ob3ect_362a82_loop_closure :
    ∃ (loop : IGProtocol automatic_writing_psychography_ob3ect_362a82_s0 automatic_writing_psychography_ob3ect_362a82_s7),
      loop = automatic_writing_psychography_ob3ect_362a82_protocol ∧
      loop.period = 8 ∧ loop.depth = 1 := by
  exact ⟨_, rfl, by decide, by decide⟩
  --
-- igProtoCopy_isDagger licenses IMSCRIB→IFIX burn
-- CLINK→IMSCRIB weighted edge: .seq continuation
