-- IGProtocol scaffold: VINIT → AFWD → FSPLIT → CLINK → AREV → EVALT → CLINK → AREV → EVALT → CLINK → AREV → EVALT → FFUSE → FSPLIT → CLINK → AREV → FSPLIT → EVALF → ENGAGR → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: There exists a rectangular box where the three edges, three face diagonals, and the space diagonal are all integers
-- Fingerprint: sig=(12,5,5,1)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=True | period=23
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 12), (16, 19)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑾  | initial object — ground of distinction
--   [1] AFWD      rel    := 𐑾               𐑼 → 𐑚  | forward morphism — bidirectional arrow
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [4] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [7] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [8] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [9] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [10] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [11] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [12] FFUSE     stoi   := 𐑙               𐑙 → 𐑚  | fuse μ — assembly mode
--   [13] FSPLIT    gran   := 𐑚               𐑙 → 𐑱  | split δ — range decomposition
--   [14] CLINK     fid    := 𐑱               𐑚 → 𐑗  | composition — regime coherence
--   [15] AREV      pol    := 𐑗               𐑱 → 𐑚  | reverse morphism — parity flip
--   [16] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [17] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [18] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [19] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [20] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [21] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [22] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def there_exists_a_rectangular_box_where_941fea_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s18 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s19 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s20 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_s21 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def there_exists_a_rectangular_box_where_941fea_s22 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def there_exists_a_rectangular_box_where_941fea_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l18 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l19 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l20 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_exists_a_rectangular_box_where_941fea_l21 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_exists_a_rectangular_box_where_941fea_l22 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def there_exists_a_rectangular_box_where_941fea_protocol : IGProtocol there_exists_a_rectangular_box_where_941fea_s0 there_exists_a_rectangular_box_where_941fea_s22 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct there_exists_a_rectangular_box_where_941fea_s12 there_exists_a_rectangular_box_where_941fea_s12 = there_exists_a_rectangular_box_where_941fea_s12 (idempotent)
  (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l0 there_exists_a_rectangular_box_where_941fea_s0 there_exists_a_rectangular_box_where_941fea_s1) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l1 there_exists_a_rectangular_box_where_941fea_s1 there_exists_a_rectangular_box_where_941fea_s2) (.seq (.prod (.arrow there_exists_a_rectangular_box_where_941fea_l2 there_exists_a_rectangular_box_where_941fea_s2 there_exists_a_rectangular_box_where_941fea_s12) (.arrow there_exists_a_rectangular_box_where_941fea_l2 there_exists_a_rectangular_box_where_941fea_s2 there_exists_a_rectangular_box_where_941fea_s12)) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l12 there_exists_a_rectangular_box_where_941fea_s12 there_exists_a_rectangular_box_where_941fea_s12) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l12 there_exists_a_rectangular_box_where_941fea_s12 there_exists_a_rectangular_box_where_941fea_s13) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l13 there_exists_a_rectangular_box_where_941fea_s13 there_exists_a_rectangular_box_where_941fea_s14) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l14 there_exists_a_rectangular_box_where_941fea_s14 there_exists_a_rectangular_box_where_941fea_s15) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l15 there_exists_a_rectangular_box_where_941fea_s15 there_exists_a_rectangular_box_where_941fea_s16) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l16 there_exists_a_rectangular_box_where_941fea_s16 there_exists_a_rectangular_box_where_941fea_s17) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l17 there_exists_a_rectangular_box_where_941fea_s17 there_exists_a_rectangular_box_where_941fea_s18) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l18 there_exists_a_rectangular_box_where_941fea_s18 there_exists_a_rectangular_box_where_941fea_s19) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l19 there_exists_a_rectangular_box_where_941fea_s19 there_exists_a_rectangular_box_where_941fea_s20) (.seq (.arrow there_exists_a_rectangular_box_where_941fea_l20 there_exists_a_rectangular_box_where_941fea_s20 there_exists_a_rectangular_box_where_941fea_s21) (.arrow there_exists_a_rectangular_box_where_941fea_l21 there_exists_a_rectangular_box_where_941fea_s21 there_exists_a_rectangular_box_where_941fea_s22))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def there_exists_a_rectangular_box_where_941fea_true_arm : IGProtocol there_exists_a_rectangular_box_where_941fea_s0 there_exists_a_rectangular_box_where_941fea_s22 :=
  (there_exists_a_rectangular_box_where_941fea_protocol).restrictToEVALT

-- false arm
noncomputable def there_exists_a_rectangular_box_where_941fea_false_arm : IGProtocol there_exists_a_rectangular_box_where_941fea_s0 there_exists_a_rectangular_box_where_941fea_s22 :=
  (there_exists_a_rectangular_box_where_941fea_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def there_exists_a_rectangular_box_where_941fea_tier_ground : OuroboricityTier := TierFunctor.obj there_exists_a_rectangular_box_where_941fea_s0
def there_exists_a_rectangular_box_where_941fea_tier : OuroboricityTier := TierFunctor.obj there_exists_a_rectangular_box_where_941fea_s22
#eval there_exists_a_rectangular_box_where_941fea_tier_ground  -- tier of the ground (pre-transformation)
#eval there_exists_a_rectangular_box_where_941fea_tier  -- the Grammar's own verdict on the closed object
