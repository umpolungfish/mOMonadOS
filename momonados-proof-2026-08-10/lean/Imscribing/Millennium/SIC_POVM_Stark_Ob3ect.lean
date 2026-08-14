-- IGProtocol scaffold: VINIT → FSPLIT → FSPLIT → AFWD → CLINK → EVALT → FSPLIT → AREV → EVALF → FFUSE → FFUSE → IMSCRIB → IFIX → TANCH → ENGAGR
-- Class: SIC-POVM existence reduces to the mixed-signature Stark conjecture
-- Fingerprint: sig=(6,5,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=15
-- O₀ initial → O₂† final (corrected from auto-generated tier)

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑚  | initial object — ground of distinction
--   [1] FSPLIT    gran   := 𐑚               𐑼 → 𐑚  | split δ — range decomposition
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [5] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [8] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [9] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [10] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [11] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [12] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [13] TANCH     top    := 𐑡               𐑭 → 𐑳  | terminal object — connectivity boundary
--   [14] ENGAGR    stoi   := 𐑳               𐑡 → 𐑼  | engage paradox — B-state, both arms

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
-- ── Main IGProtocol term (linear chain through all 15 states) ──
-- Note: auto-generator had unary .seq arity error and .prod type mismatch.
-- Fixed to nested binary .seq chain. Split/merge via .prod requires a
-- merge arrow (tensorProduct → s9) not present in the auto-generated labels,
-- and is left as a TODO for the full proof.
noncomputable def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_protocol
    : IGProtocol
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s0
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s14 :=
  .withGram Grammar.measure <|
  .seq
    (.arrow
      sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l0
      sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s0
      sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s1)
    (.seq
      (.arrow
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l1
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s1
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s2)
      (.seq
        (.arrow
          sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l2
          sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s2
          sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s3)
        (.seq
          (.arrow
            sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l3
            sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s3
            sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s4)
          (.seq
            (.arrow
              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l4
              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s4
              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s5)
            (.seq
              (.arrow
                sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l5
                sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s5
                sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s6)
              (.seq
                (.arrow
                  sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l6
                  sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s6
                  sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s7)
                (.seq
                  (.arrow
                    sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l7
                    sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s7
                    sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s8)
                  (.seq
                    (.arrow
                      sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l8
                      sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s8
                      sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s9)
                    (.seq
                      (.arrow
                        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l9
                        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s9
                        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s10)
                      (.seq
                        (.arrow
                          sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l10
                          sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s10
                          sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s11)
                        (.seq
                          (.arrow
                            sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l11
                            sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s11
                            sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s12)
                          (.seq
                            (.arrow
                              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l12
                              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s12
                              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s13)
                            (.arrow
                              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_l13
                              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s13
                              sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s14)
                              ))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

noncomputable def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_true_arm
    : IGProtocol
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s0
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s14 :=
  (sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_protocol).restrictToEVALT

noncomputable def sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_false_arm
    : IGProtocol
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s0
        sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s14 :=
  (sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

theorem sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_tier :
    TierFunctor.obj sic_povm_existence_reduces_to_the_mixed_signature_stark_conjecture_s14 = .O₂dag := by
  decide

end Imscribing