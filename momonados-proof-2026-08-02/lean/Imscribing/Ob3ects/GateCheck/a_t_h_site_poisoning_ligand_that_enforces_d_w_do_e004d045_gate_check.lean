-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → FSPLIT → EVALT → CLINK → ENGAGR → EVALF → AREV → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: a T↔H site-poisoning ligand that enforces D↔W docking for the perfect_cuboid_proof
-- Fingerprint: sig=(7,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=13
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(3, 9)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑚  | forward morphism — bidirectional arrow
--   [3] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [4] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [5] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [6] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [7] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [10] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [11] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [12] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_t_h_site_poisoning_ligand_that_7eac5c_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_t_h_site_poisoning_ligand_that_7eac5c_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_t_h_site_poisoning_ligand_that_7eac5c_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_t_h_site_poisoning_ligand_that_7eac5c_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_t_h_site_poisoning_ligand_that_7eac5c_protocol : IGProtocol a_t_h_site_poisoning_ligand_that_7eac5c_s0 a_t_h_site_poisoning_ligand_that_7eac5c_s12 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_t_h_site_poisoning_ligand_that_7eac5c_s9 a_t_h_site_poisoning_ligand_that_7eac5c_s9 = a_t_h_site_poisoning_ligand_that_7eac5c_s9 (idempotent)
  (.seq (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l0 a_t_h_site_poisoning_ligand_that_7eac5c_s0 a_t_h_site_poisoning_ligand_that_7eac5c_s1) (.seq (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l1 a_t_h_site_poisoning_ligand_that_7eac5c_s1 a_t_h_site_poisoning_ligand_that_7eac5c_s2) (.seq (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l2 a_t_h_site_poisoning_ligand_that_7eac5c_s2 a_t_h_site_poisoning_ligand_that_7eac5c_s3) (.seq (.prod (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l3 a_t_h_site_poisoning_ligand_that_7eac5c_s3 a_t_h_site_poisoning_ligand_that_7eac5c_s9) (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l3 a_t_h_site_poisoning_ligand_that_7eac5c_s3 a_t_h_site_poisoning_ligand_that_7eac5c_s9)) (.seq (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l9 a_t_h_site_poisoning_ligand_that_7eac5c_s9 a_t_h_site_poisoning_ligand_that_7eac5c_s9) (.seq (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l9 a_t_h_site_poisoning_ligand_that_7eac5c_s9 a_t_h_site_poisoning_ligand_that_7eac5c_s10) (.seq (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l10 a_t_h_site_poisoning_ligand_that_7eac5c_s10 a_t_h_site_poisoning_ligand_that_7eac5c_s11) (.arrow a_t_h_site_poisoning_ligand_that_7eac5c_l11 a_t_h_site_poisoning_ligand_that_7eac5c_s11 a_t_h_site_poisoning_ligand_that_7eac5c_s12))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_t_h_site_poisoning_ligand_that_7eac5c_true_arm : IGProtocol a_t_h_site_poisoning_ligand_that_7eac5c_s0 a_t_h_site_poisoning_ligand_that_7eac5c_s12 :=
  (a_t_h_site_poisoning_ligand_that_7eac5c_protocol).restrictToEVALT

-- false arm
noncomputable def a_t_h_site_poisoning_ligand_that_7eac5c_false_arm : IGProtocol a_t_h_site_poisoning_ligand_that_7eac5c_s0 a_t_h_site_poisoning_ligand_that_7eac5c_s12 :=
  (a_t_h_site_poisoning_ligand_that_7eac5c_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_t_h_site_poisoning_ligand_that_7eac5c_tier : OuroboricityTier := TierFunctor.obj a_t_h_site_poisoning_ligand_that_7eac5c_s0
#eval a_t_h_site_poisoning_ligand_that_7eac5c_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_t_h_site_poisoning_ligand_that_7eac5c_frobenius :
    igFrobeniusAlg.mul a_t_h_site_poisoning_ligand_that_7eac5c_s0 a_t_h_site_poisoning_ligand_that_7eac5c_s0 = a_t_h_site_poisoning_ligand_that_7eac5c_s0 :=
  igFrobAlg_self_fusion a_t_h_site_poisoning_ligand_that_7eac5c_s0
