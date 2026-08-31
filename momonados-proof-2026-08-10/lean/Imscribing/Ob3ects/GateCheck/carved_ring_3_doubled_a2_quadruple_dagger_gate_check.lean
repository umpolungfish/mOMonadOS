-- IGProtocol scaffold: VINIT → FSPLIT → AFWD → AFWD → EVALT → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: carved ring 3 doubled a2 quadruple dagger
-- Fingerprint: sig=(5,2,1,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=False | period=9
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(1, 5)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [2] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [6] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [7] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [8] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def carved_ring_3_doubled_a2_quadruple_f9f14f_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def carved_ring_3_doubled_a2_quadruple_f9f14f_protocol : IGProtocol carved_ring_3_doubled_a2_quadruple_f9f14f_s0 carved_ring_3_doubled_a2_quadruple_f9f14f_s8 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct carved_ring_3_doubled_a2_quadruple_f9f14f_s5 carved_ring_3_doubled_a2_quadruple_f9f14f_s5 = carved_ring_3_doubled_a2_quadruple_f9f14f_s5 (idempotent)
  (.seq (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l0 carved_ring_3_doubled_a2_quadruple_f9f14f_s0 carved_ring_3_doubled_a2_quadruple_f9f14f_s1) (.seq (.prod (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l1 carved_ring_3_doubled_a2_quadruple_f9f14f_s1 carved_ring_3_doubled_a2_quadruple_f9f14f_s5) (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l1 carved_ring_3_doubled_a2_quadruple_f9f14f_s1 carved_ring_3_doubled_a2_quadruple_f9f14f_s5)) (.seq (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l5 carved_ring_3_doubled_a2_quadruple_f9f14f_s5 carved_ring_3_doubled_a2_quadruple_f9f14f_s5) (.seq (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l5 carved_ring_3_doubled_a2_quadruple_f9f14f_s5 carved_ring_3_doubled_a2_quadruple_f9f14f_s6) (.seq (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l6 carved_ring_3_doubled_a2_quadruple_f9f14f_s6 carved_ring_3_doubled_a2_quadruple_f9f14f_s7) (.arrow carved_ring_3_doubled_a2_quadruple_f9f14f_l7 carved_ring_3_doubled_a2_quadruple_f9f14f_s7 carved_ring_3_doubled_a2_quadruple_f9f14f_s8))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def carved_ring_3_doubled_a2_quadruple_f9f14f_true_arm : IGProtocol carved_ring_3_doubled_a2_quadruple_f9f14f_s0 carved_ring_3_doubled_a2_quadruple_f9f14f_s8 :=
  (carved_ring_3_doubled_a2_quadruple_f9f14f_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def carved_ring_3_doubled_a2_quadruple_f9f14f_tier : OuroboricityTier := TierFunctor.obj carved_ring_3_doubled_a2_quadruple_f9f14f_s0
#eval carved_ring_3_doubled_a2_quadruple_f9f14f_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem carved_ring_3_doubled_a2_quadruple_f9f14f_frobenius :
    igFrobeniusAlg.mul carved_ring_3_doubled_a2_quadruple_f9f14f_s0 carved_ring_3_doubled_a2_quadruple_f9f14f_s0 = carved_ring_3_doubled_a2_quadruple_f9f14f_s0 :=
  igFrobAlg_self_fusion carved_ring_3_doubled_a2_quadruple_f9f14f_s0
