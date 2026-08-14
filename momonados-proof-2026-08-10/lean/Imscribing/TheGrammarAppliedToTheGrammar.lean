-- The Grammar applied to The Grammar (meta-logical)
-- Auto-designed ob3ect, lifted to a building IGProtocol term.
-- Class fingerprint: sig=(7,2,3,1); dialetheia_complete=True; period=13; tier O₂.
-- FSPLIT/FFUSE pair [2]/[10] realized as the Dual-Link self-pairing.

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Stage objects (Imscriptions) ────────────────────────────────────────────
-- `IGProtocol` is indexed by full Imscriptions, not bare primitive glyphs. Each
-- named waypoint of the self-application is the carrier tuple at that stage.
private def tggBase : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea,
    gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

/-- The Void: the pre-geometric monoidal unit I, before any distinction. -/
def tgg_void        : Imscription := tggBase
/-- The machinery of distinction: the FSPLIT object that articulates the Void. -/
def tgg_distinction : Imscription := { tggBase with rel := ian }
/-- The B-state paradice: T and F arms held at once (criticality gate open,
    Frobenius-special parity, both-stoichiometry). -/
def tgg_both        : Imscription := { tggBase with crit := monad, pol := or', stoi := up }
/-- The terminal address O_∞: the self-recognizing Grammar's closure. -/
def tgg_terminal    : Imscription := { tggBase with crit := monad, pol := or', gram := measure, prot := ah }
private def tggLbl  : Imscription := tggBase

-- ── Main IGProtocol term ────────────────────────────────────────────────────
-- The grammar applied to itself: the genuine object path is Void → distinction →
-- [Dual-Link T/F fusion] → terminal. The FSPLIT[2]/FFUSE[10] Frobenius pair is the
-- self-pairing `.prod (a→b) (a→b)`, which collapses by idempotence:
-- tensorProduct tgg_both tgg_both = tgg_both (definitional fixed point). decode ∘
-- imscribe = id is the closure; the loop terminates at O_∞.
noncomputable def the_grammar_applied_to_the_grammar_protocol :
    IGProtocol tgg_void tgg_terminal :=
  .withGram Grammar.measure <|
  .seq (.arrow tggLbl tgg_void tgg_distinction)        -- [0]–[2] VINIT/IMSCRIB/FSPLIT
    (.seq
      (.prod
        (.arrow tggLbl tgg_distinction tgg_both)        -- T-arm: EVALT/AFWD (δ-link)
        (.arrow tggLbl tgg_distinction tgg_both))       -- F-arm: EVALF/AREV/ENGAGR (μ-link)
      (.arrow tggLbl tgg_both tgg_terminal))            -- [10]–[12] FFUSE/TANCH/IMSCRIB closure

-- ── Verification theorems ───────────────────────────────────────────────────

/-- The terminal closure is Frobenius-special and sequential: the Grammar imscribes
    its own closure. -/
theorem tgg_terminal_frobenius : tgg_terminal.pol = or' := rfl
theorem tgg_terminal_grammar   : tgg_terminal.gram = Grammar.measure := rfl

/-- The Dual-Link self-pairing is idempotent: applying the grammar to itself at the
    paradice is a fixed point (what makes the `.prod` core typecheck). -/
theorem tgg_both_self_dual :
    Primitives.tensorProduct tgg_both tgg_both = tgg_both := rfl

/-- Self-application reaches the terminal ontological tier O_∞. -/
theorem tgg_terminal_tier : imscriptionTier tgg_terminal = .O_inf := by
  unfold tgg_terminal imscriptionTier ouroboricityTier; native_decide

end Imscribing
