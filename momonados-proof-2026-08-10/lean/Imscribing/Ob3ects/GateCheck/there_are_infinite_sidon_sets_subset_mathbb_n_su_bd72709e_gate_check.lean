-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → IFIX → CLINK → AFWD → FSPLIT → EVALT → AFWD → FFUSE → IFIX → CLINK → FSPLIT → ENGAGR → EVALT → FFUSE → IMSCRIB → TANCH
-- Class: There are infinite Sidon sets \subset \mathbb{N}$ such that\[\lvert A\cap \{1\ldots,N\}\rvert \gg_\epsilon N^{1/2-\epsilon}\]for all $\epsilon>0$
-- Fingerprint: sig=(9,4,3,2)
--   self_ref=False | frobenius_order=3
--   dialetheia_complete=False | period=18
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(6, 9), (12, 15)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑭  | forward morphism — bidirectional arrow
--   [3] IFIX      prot   := 𐑭               𐑾 → 𐑱  | irreversible fixation — winding number
--   [4] CLINK     fid    := 𐑱               𐑭 → 𐑾  | composition — regime coherence
--   [5] AFWD      rel    := 𐑾               𐑱 → 𐑚  | forward morphism — bidirectional arrow
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑭  | fuse μ — assembly mode
--   [10] IFIX      prot   := 𐑭               𐑙 → 𐑱  | irreversible fixation — winding number
--   [11] CLINK     fid    := 𐑱               𐑭 → 𐑚  | composition — regime coherence
--   [12] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [13] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [14] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [15] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [16] IMSCRIB   gram   := 𐑠               𐑙 → 𐑡  | identity — self-imscription
--   [17] TANCH     top    := 𐑡               𐑠 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def there_are_infinite_sidon_sets_subset_f7460e_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_s17 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def there_are_infinite_sidon_sets_subset_f7460e_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l5 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l8 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def there_are_infinite_sidon_sets_subset_f7460e_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def there_are_infinite_sidon_sets_subset_f7460e_l17 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def there_are_infinite_sidon_sets_subset_f7460e_protocol : IGProtocol there_are_infinite_sidon_sets_subset_f7460e_s0 there_are_infinite_sidon_sets_subset_f7460e_s17 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct there_are_infinite_sidon_sets_subset_f7460e_s9 there_are_infinite_sidon_sets_subset_f7460e_s9 = there_are_infinite_sidon_sets_subset_f7460e_s9 (idempotent)
  (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l0 there_are_infinite_sidon_sets_subset_f7460e_s0 there_are_infinite_sidon_sets_subset_f7460e_s1) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l1 there_are_infinite_sidon_sets_subset_f7460e_s1 there_are_infinite_sidon_sets_subset_f7460e_s2) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l2 there_are_infinite_sidon_sets_subset_f7460e_s2 there_are_infinite_sidon_sets_subset_f7460e_s3) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l3 there_are_infinite_sidon_sets_subset_f7460e_s3 there_are_infinite_sidon_sets_subset_f7460e_s4) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l4 there_are_infinite_sidon_sets_subset_f7460e_s4 there_are_infinite_sidon_sets_subset_f7460e_s5) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l5 there_are_infinite_sidon_sets_subset_f7460e_s5 there_are_infinite_sidon_sets_subset_f7460e_s6) (.seq (.prod (.arrow there_are_infinite_sidon_sets_subset_f7460e_l6 there_are_infinite_sidon_sets_subset_f7460e_s6 there_are_infinite_sidon_sets_subset_f7460e_s9) (.arrow there_are_infinite_sidon_sets_subset_f7460e_l6 there_are_infinite_sidon_sets_subset_f7460e_s6 there_are_infinite_sidon_sets_subset_f7460e_s9)) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l9 there_are_infinite_sidon_sets_subset_f7460e_s9 there_are_infinite_sidon_sets_subset_f7460e_s9) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l9 there_are_infinite_sidon_sets_subset_f7460e_s9 there_are_infinite_sidon_sets_subset_f7460e_s10) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l10 there_are_infinite_sidon_sets_subset_f7460e_s10 there_are_infinite_sidon_sets_subset_f7460e_s11) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l11 there_are_infinite_sidon_sets_subset_f7460e_s11 there_are_infinite_sidon_sets_subset_f7460e_s12) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l12 there_are_infinite_sidon_sets_subset_f7460e_s12 there_are_infinite_sidon_sets_subset_f7460e_s13) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l13 there_are_infinite_sidon_sets_subset_f7460e_s13 there_are_infinite_sidon_sets_subset_f7460e_s14) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l14 there_are_infinite_sidon_sets_subset_f7460e_s14 there_are_infinite_sidon_sets_subset_f7460e_s15) (.seq (.arrow there_are_infinite_sidon_sets_subset_f7460e_l15 there_are_infinite_sidon_sets_subset_f7460e_s15 there_are_infinite_sidon_sets_subset_f7460e_s16) (.arrow there_are_infinite_sidon_sets_subset_f7460e_l16 there_are_infinite_sidon_sets_subset_f7460e_s16 there_are_infinite_sidon_sets_subset_f7460e_s17))))))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def there_are_infinite_sidon_sets_subset_f7460e_true_arm : IGProtocol there_are_infinite_sidon_sets_subset_f7460e_s0 there_are_infinite_sidon_sets_subset_f7460e_s17 :=
  (there_are_infinite_sidon_sets_subset_f7460e_protocol).restrictToEVALT

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def there_are_infinite_sidon_sets_subset_f7460e_tier_ground : OuroboricityTier := TierFunctor.obj there_are_infinite_sidon_sets_subset_f7460e_s0
def there_are_infinite_sidon_sets_subset_f7460e_tier : OuroboricityTier := TierFunctor.obj there_are_infinite_sidon_sets_subset_f7460e_s17
#eval there_are_infinite_sidon_sets_subset_f7460e_tier_ground  -- tier of the ground (pre-transformation)
#eval there_are_infinite_sidon_sets_subset_f7460e_tier  -- the Grammar's own verdict on the closed object
