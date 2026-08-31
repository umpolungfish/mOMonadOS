-- IGProtocol scaffold: IMSCRIB → FSPLIT → FFUSE → IMSCRIB
-- Class: seekpeek_bridge_closure
-- Fingerprint: sig=(2,2,0,0)
--   self_ref=True | frobenius_order=1
--   dialetheia_complete=False | period=4
-- Expected tier: O₀
-- FSPLIT/FFUSE pairs: [(1, 2)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] IMSCRIB   gram   := 𐑠               𐑠 → 𐑚  | identity — self-imscription
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [3] IMSCRIB   gram   := 𐑠               𐑙 → 𐑠  | identity — self-imscription

-- ── Back-propagation edges (self-referential loop) ──────────────────────
--   IMSCRIB positions: [0, 3]
--   IFIX    positions: []
--   Back-prop: IMSCRIB→IFIX (LinFix) — igProtoCopy_isDagger axiom applies
--   Weighted: CLINK→IMSCRIB — feeds next winding via .seq after .prod

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def seekpeek_bridge_closure_s0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def seekpeek_bridge_closure_s1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def seekpeek_bridge_closure_s2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def seekpeek_bridge_closure_s3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def seekpeek_bridge_closure_l0 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def seekpeek_bridge_closure_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def seekpeek_bridge_closure_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def seekpeek_bridge_closure_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def seekpeek_bridge_closure_protocol : IGProtocol seekpeek_bridge_closure_s0 seekpeek_bridge_closure_s3 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct seekpeek_bridge_closure_s2 seekpeek_bridge_closure_s2 = seekpeek_bridge_closure_s2 (idempotent)
  (.seq (.arrow seekpeek_bridge_closure_l0 seekpeek_bridge_closure_s0 seekpeek_bridge_closure_s1) (.seq (.prod (.arrow seekpeek_bridge_closure_l1 seekpeek_bridge_closure_s1 seekpeek_bridge_closure_s2) (.arrow seekpeek_bridge_closure_l1 seekpeek_bridge_closure_s1 seekpeek_bridge_closure_s2)) (.seq (.arrow seekpeek_bridge_closure_l2 seekpeek_bridge_closure_s2 seekpeek_bridge_closure_s2) (.arrow seekpeek_bridge_closure_l2 seekpeek_bridge_closure_s2 seekpeek_bridge_closure_s3))))

-- ── Verification theorems ─────────────────────────────────────-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). The verdict on
-- the O₀ bridge frame is evaluated, not asserted: this tuple is not O_inf,
-- so the generator's generic O_inf tier claim is honestly declined.
def seekpeek_bridge_closure_tier_ground : OuroboricityTier := TierFunctor.obj seekpeek_bridge_closure_s0
def seekpeek_bridge_closure_tier : OuroboricityTier := TierFunctor.obj seekpeek_bridge_closure_s3
#eval seekpeek_bridge_closure_tier_ground  -- tier of the ground (pre-transformation)
#eval seekpeek_bridge_closure_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem seekpeek_bridge_closure_frobenius :
    igFrobeniusAlg.mul seekpeek_bridge_closure_s0 seekpeek_bridge_closure_s0 = seekpeek_bridge_closure_s0 :=
  igFrobAlg_self_fusion seekpeek_bridge_closure_s0

-- O₀ closure: the bridge→un_bridge round-trip (FSPLIT then FFUSE) is a
-- protocol of depth 4, measured by the kernel's own depth def. The
-- fingerprint's period=4 is this depth: one winding, four labeled steps.
-- refl=0, arrow=1, seq=sum, prod=max: 1 + (max 1 1 + (1 + 1)) = 4.
theorem seekpeek_bridge_closure_depth :
    seekpeek_bridge_closure_protocol.depth = 4 := by
  simp [seekpeek_bridge_closure_protocol, IGProtocol.depth]

-- The recovery arrows are not dagger arrows (labels carry rel=ado, not ear):
-- un_bridge∘bridge=id is a directed recovery, not an adjoint involution.
-- The paralogical axioms (paralogical_copy at O_inf, paralogical_reflect at
-- dim=if') do not fire here: this O₀ frame has neither precondition, and the
-- closure is carried by the typed protocol itself.
theorem seekpeek_bridge_closure_not_dagger :
    seekpeek_bridge_closure_protocol.isDagger = false := by
  simp [seekpeek_bridge_closure_protocol, IGProtocol.isDagger,
        seekpeek_bridge_closure_l0, seekpeek_bridge_closure_l1,
        seekpeek_bridge_closure_l2]
