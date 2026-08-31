-- IGProtocol scaffold: VINIT → CLINK → IMSCRIB → FSPLIT → EVALT → EVALF → FFUSE → AFWD → IFIX → TANCH
-- Class: the bipartite sumset cannot stay spread.
-- Fingerprint: sig=(5,2,2,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=10
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 6)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑱  | initial object — ground of distinction
--   [1] CLINK     fid    := 𐑱               𐑼 → 𐑠  | composition — regime coherence
--   [2] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑾  | fuse μ — assembly mode
--   [7] AFWD      rel    := 𐑾               𐑙 → 𐑭  | forward morphism — bidirectional arrow
--   [8] IFIX      prot   := 𐑭               𐑾 → 𐑡  | irreversible fixation — winding number
--   [9] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_bipartite_sumset_cannot_stay_spread_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s3 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s4 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s5 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s6 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_bipartite_sumset_cannot_stay_spread_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_bipartite_sumset_cannot_stay_spread_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_bipartite_sumset_cannot_stay_spread_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_bipartite_sumset_cannot_stay_spread_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_bipartite_sumset_cannot_stay_spread_protocol : IGProtocol the_bipartite_sumset_cannot_stay_spread_s0 the_bipartite_sumset_cannot_stay_spread_s9 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_bipartite_sumset_cannot_stay_spread_s6 the_bipartite_sumset_cannot_stay_spread_s6 = the_bipartite_sumset_cannot_stay_spread_s6 (idempotent)
  (.seq (.arrow the_bipartite_sumset_cannot_stay_spread_l0 the_bipartite_sumset_cannot_stay_spread_s0 the_bipartite_sumset_cannot_stay_spread_s1) (.seq (.arrow the_bipartite_sumset_cannot_stay_spread_l1 the_bipartite_sumset_cannot_stay_spread_s1 the_bipartite_sumset_cannot_stay_spread_s2) (.seq (.arrow the_bipartite_sumset_cannot_stay_spread_l2 the_bipartite_sumset_cannot_stay_spread_s2 the_bipartite_sumset_cannot_stay_spread_s3) (.seq (.prod (.arrow the_bipartite_sumset_cannot_stay_spread_l3 the_bipartite_sumset_cannot_stay_spread_s3 the_bipartite_sumset_cannot_stay_spread_s6) (.arrow the_bipartite_sumset_cannot_stay_spread_l3 the_bipartite_sumset_cannot_stay_spread_s3 the_bipartite_sumset_cannot_stay_spread_s6)) (.seq (.arrow the_bipartite_sumset_cannot_stay_spread_l6 the_bipartite_sumset_cannot_stay_spread_s6 the_bipartite_sumset_cannot_stay_spread_s6) (.seq (.arrow the_bipartite_sumset_cannot_stay_spread_l6 the_bipartite_sumset_cannot_stay_spread_s6 the_bipartite_sumset_cannot_stay_spread_s7) (.seq (.arrow the_bipartite_sumset_cannot_stay_spread_l7 the_bipartite_sumset_cannot_stay_spread_s7 the_bipartite_sumset_cannot_stay_spread_s8) (.arrow the_bipartite_sumset_cannot_stay_spread_l8 the_bipartite_sumset_cannot_stay_spread_s8 the_bipartite_sumset_cannot_stay_spread_s9))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_bipartite_sumset_cannot_stay_spread_true_arm : IGProtocol the_bipartite_sumset_cannot_stay_spread_s0 the_bipartite_sumset_cannot_stay_spread_s9 :=
  (the_bipartite_sumset_cannot_stay_spread_protocol).restrictToEVALT

-- false arm
noncomputable def the_bipartite_sumset_cannot_stay_spread_false_arm : IGProtocol the_bipartite_sumset_cannot_stay_spread_s0 the_bipartite_sumset_cannot_stay_spread_s9 :=
  (the_bipartite_sumset_cannot_stay_spread_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def the_bipartite_sumset_cannot_stay_spread_tier_ground : OuroboricityTier := TierFunctor.obj the_bipartite_sumset_cannot_stay_spread_s0
def the_bipartite_sumset_cannot_stay_spread_tier : OuroboricityTier := TierFunctor.obj the_bipartite_sumset_cannot_stay_spread_s9
#eval the_bipartite_sumset_cannot_stay_spread_tier_ground  -- tier of the ground (pre-transformation)
#eval the_bipartite_sumset_cannot_stay_spread_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_bipartite_sumset_cannot_stay_spread_frobenius :
    igFrobeniusAlg.mul the_bipartite_sumset_cannot_stay_spread_s0 the_bipartite_sumset_cannot_stay_spread_s0 = the_bipartite_sumset_cannot_stay_spread_s0 :=
  igFrobAlg_self_fusion the_bipartite_sumset_cannot_stay_spread_s0
