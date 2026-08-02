-- IGProtocol scaffold: VINIT → AFWD → CLINK → CLINK → CLINK → IMSCRIB → FSPLIT → EVALT → IFIX → AREV → FFUSE → FSPLIT → EVALF → IFIX → AREV → FFUSE → ENGAGR → TANCH
-- Class: There exists no rectangular boxes where the three edges, three face diagonals, and the space diagonal are all integers
-- Fingerprint: sig=(9,4,3,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=18
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(6, 10), (11, 15)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑱  | forward morphism — bidirectional arrow
--   [2] CLINK     fid    := 𐑱               𐑾 → 𐑱  | composition — regime coherence
--   [3] CLINK     fid    := 𐑱               𐑱 → 𐑱  | composition — regime coherence
--   [4] CLINK     fid    := 𐑱               𐑱 → 𐑠  | composition — regime coherence
--   [5] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [9] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [10] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [11] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [12] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [13] IFIX      prot   := 𐑭               𐑚 → 𐑙  | irreversible fixation — winding number
--   [14] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [15] FFUSE     stoi   := 𐑙               𐑙 → 𐑳  | fuse μ — assembly mode
--   [16] ENGAGR    stoi   := 𐑳               𐑙 → 𐑡  | engage paradox — B-state, both arms
--   [17] TANCH     top    := 𐑡               𐑳 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def there_exists_no_rectangular_boxes_where_34a8ec_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def there_exists_no_rectangular_boxes_where_34a8ec_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_exists_no_rectangular_boxes_where_34a8ec_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def there_exists_no_rectangular_boxes_where_34a8ec_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def there_exists_no_rectangular_boxes_where_34a8ec_protocol : IGProtocol there_exists_no_rectangular_boxes_where_34a8ec_s0 there_exists_no_rectangular_boxes_where_34a8ec_s17 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct there_exists_no_rectangular_boxes_where_34a8ec_s10 there_exists_no_rectangular_boxes_where_34a8ec_s10 = there_exists_no_rectangular_boxes_where_34a8ec_s10 (idempotent)
  (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l0 there_exists_no_rectangular_boxes_where_34a8ec_s0 there_exists_no_rectangular_boxes_where_34a8ec_s1) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l1 there_exists_no_rectangular_boxes_where_34a8ec_s1 there_exists_no_rectangular_boxes_where_34a8ec_s2) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l2 there_exists_no_rectangular_boxes_where_34a8ec_s2 there_exists_no_rectangular_boxes_where_34a8ec_s3) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l3 there_exists_no_rectangular_boxes_where_34a8ec_s3 there_exists_no_rectangular_boxes_where_34a8ec_s4) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l4 there_exists_no_rectangular_boxes_where_34a8ec_s4 there_exists_no_rectangular_boxes_where_34a8ec_s5) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l5 there_exists_no_rectangular_boxes_where_34a8ec_s5 there_exists_no_rectangular_boxes_where_34a8ec_s6) (.seq (.prod (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l6 there_exists_no_rectangular_boxes_where_34a8ec_s6 there_exists_no_rectangular_boxes_where_34a8ec_s10) (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l6 there_exists_no_rectangular_boxes_where_34a8ec_s6 there_exists_no_rectangular_boxes_where_34a8ec_s10)) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l10 there_exists_no_rectangular_boxes_where_34a8ec_s10 there_exists_no_rectangular_boxes_where_34a8ec_s10) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l10 there_exists_no_rectangular_boxes_where_34a8ec_s10 there_exists_no_rectangular_boxes_where_34a8ec_s11) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l11 there_exists_no_rectangular_boxes_where_34a8ec_s11 there_exists_no_rectangular_boxes_where_34a8ec_s12) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l12 there_exists_no_rectangular_boxes_where_34a8ec_s12 there_exists_no_rectangular_boxes_where_34a8ec_s13) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l13 there_exists_no_rectangular_boxes_where_34a8ec_s13 there_exists_no_rectangular_boxes_where_34a8ec_s14) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l14 there_exists_no_rectangular_boxes_where_34a8ec_s14 there_exists_no_rectangular_boxes_where_34a8ec_s15) (.seq (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l15 there_exists_no_rectangular_boxes_where_34a8ec_s15 there_exists_no_rectangular_boxes_where_34a8ec_s16) (.arrow there_exists_no_rectangular_boxes_where_34a8ec_l16 there_exists_no_rectangular_boxes_where_34a8ec_s16 there_exists_no_rectangular_boxes_where_34a8ec_s17)))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def there_exists_no_rectangular_boxes_where_34a8ec_true_arm : IGProtocol there_exists_no_rectangular_boxes_where_34a8ec_s0 there_exists_no_rectangular_boxes_where_34a8ec_s17 :=
  (there_exists_no_rectangular_boxes_where_34a8ec_protocol).restrictToEVALT

-- false arm
noncomputable def there_exists_no_rectangular_boxes_where_34a8ec_false_arm : IGProtocol there_exists_no_rectangular_boxes_where_34a8ec_s0 there_exists_no_rectangular_boxes_where_34a8ec_s17 :=
  (there_exists_no_rectangular_boxes_where_34a8ec_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def there_exists_no_rectangular_boxes_where_34a8ec_tier_ground : OuroboricityTier := TierFunctor.obj there_exists_no_rectangular_boxes_where_34a8ec_s0
def there_exists_no_rectangular_boxes_where_34a8ec_tier : OuroboricityTier := TierFunctor.obj there_exists_no_rectangular_boxes_where_34a8ec_s17
#eval there_exists_no_rectangular_boxes_where_34a8ec_tier_ground  -- tier of the ground (pre-transformation)
#eval there_exists_no_rectangular_boxes_where_34a8ec_tier  -- the Grammar's own verdict on the closed object
