-- IGProtocol scaffold: VINIT → AFWD → FSPLIT → EVALT → EVALF → ENGAGR → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: A self-imscribing monad T on the category of Belnap-four-valued measurement algebras Meas_B4. T(X) is the informational completion of a measurement system X via SIC-POVM fiducial orbits: fiducial B=XZ in C^2 extended to C^{2^n} by the Zauner embedding. Unit eta: X -> T(X) embeds a system into its measured completion (diagonal/inflation). Multiplication mu: T(T(X)) -> T(X) collapses double-measurement (orbit-collapse). Dialetheic closure mu o delta = id_A encodes the Sigma=1:1 self-referential fixed point where the grammar IS the measured system. The structure carries a Frobenius duality pairing the 6 dual primitive pairs (D<->T, R<->Phi, F<->K, G<->Gm, Ph<->H, Sigma<->Omega) and a chiral phase H from the winding Omega mod 2. The paraconsistent trace Tr_B4 on End_B4(A) is cyclic even under B (both-true-and-false) values, which is the arcane core: classical trace fails under contradiction but this trace survives because mu o delta = id.
-- Fingerprint: sig=(4,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=10
-- Expected tier: O₂dag
-- FSPLIT/FFUSE pairs: [(2, 6)]

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
--   [3] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [4] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [5] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [6] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [7] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [8] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [9] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def a_self_imscribing_monad_t_on_the_e196f4_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := up, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def a_self_imscribing_monad_t_on_the_e196f4_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def a_self_imscribing_monad_t_on_the_e196f4_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def a_self_imscribing_monad_t_on_the_e196f4_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def a_self_imscribing_monad_t_on_the_e196f4_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def a_self_imscribing_monad_t_on_the_e196f4_protocol : IGProtocol a_self_imscribing_monad_t_on_the_e196f4_s0 a_self_imscribing_monad_t_on_the_e196f4_s9 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct a_self_imscribing_monad_t_on_the_e196f4_s6 a_self_imscribing_monad_t_on_the_e196f4_s6 = a_self_imscribing_monad_t_on_the_e196f4_s6 (idempotent)
  (.seq (.arrow a_self_imscribing_monad_t_on_the_e196f4_l0 a_self_imscribing_monad_t_on_the_e196f4_s0 a_self_imscribing_monad_t_on_the_e196f4_s1) (.seq (.arrow a_self_imscribing_monad_t_on_the_e196f4_l1 a_self_imscribing_monad_t_on_the_e196f4_s1 a_self_imscribing_monad_t_on_the_e196f4_s2) (.seq (.prod (.arrow a_self_imscribing_monad_t_on_the_e196f4_l2 a_self_imscribing_monad_t_on_the_e196f4_s2 a_self_imscribing_monad_t_on_the_e196f4_s6) (.arrow a_self_imscribing_monad_t_on_the_e196f4_l2 a_self_imscribing_monad_t_on_the_e196f4_s2 a_self_imscribing_monad_t_on_the_e196f4_s6)) (.seq (.arrow a_self_imscribing_monad_t_on_the_e196f4_l6 a_self_imscribing_monad_t_on_the_e196f4_s6 a_self_imscribing_monad_t_on_the_e196f4_s6) (.seq (.arrow a_self_imscribing_monad_t_on_the_e196f4_l6 a_self_imscribing_monad_t_on_the_e196f4_s6 a_self_imscribing_monad_t_on_the_e196f4_s7) (.seq (.arrow a_self_imscribing_monad_t_on_the_e196f4_l7 a_self_imscribing_monad_t_on_the_e196f4_s7 a_self_imscribing_monad_t_on_the_e196f4_s8) (.arrow a_self_imscribing_monad_t_on_the_e196f4_l8 a_self_imscribing_monad_t_on_the_e196f4_s8 a_self_imscribing_monad_t_on_the_e196f4_s9)))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def a_self_imscribing_monad_t_on_the_e196f4_true_arm : IGProtocol a_self_imscribing_monad_t_on_the_e196f4_s0 a_self_imscribing_monad_t_on_the_e196f4_s9 :=
  (a_self_imscribing_monad_t_on_the_e196f4_protocol).restrictToEVALT

-- false arm
noncomputable def a_self_imscribing_monad_t_on_the_e196f4_false_arm : IGProtocol a_self_imscribing_monad_t_on_the_e196f4_s0 a_self_imscribing_monad_t_on_the_e196f4_s9 :=
  (a_self_imscribing_monad_t_on_the_e196f4_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₂dag.
def a_self_imscribing_monad_t_on_the_e196f4_tier_ground : OuroboricityTier := TierFunctor.obj a_self_imscribing_monad_t_on_the_e196f4_s0
def a_self_imscribing_monad_t_on_the_e196f4_tier : OuroboricityTier := TierFunctor.obj a_self_imscribing_monad_t_on_the_e196f4_s9
#eval a_self_imscribing_monad_t_on_the_e196f4_tier_ground  -- tier of the ground (pre-transformation)
#eval a_self_imscribing_monad_t_on_the_e196f4_tier  -- the Grammar's own verdict on the closed object

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem a_self_imscribing_monad_t_on_the_e196f4_frobenius :
    igFrobeniusAlg.mul a_self_imscribing_monad_t_on_the_e196f4_s0 a_self_imscribing_monad_t_on_the_e196f4_s0 = a_self_imscribing_monad_t_on_the_e196f4_s0 :=
  igFrobAlg_self_fusion a_self_imscribing_monad_t_on_the_e196f4_s0
