-- Imscribing/Isomorphisms.lean
-- The chem ↔ math isomorphism, formalized.
--
-- MoDoT's engine runs one structural operation and reads it in two registers at once:
-- as CHEMISTRY (click, cycle, cleave, excite) and as MATHEMATICS (Frobenius fusion,
-- a fixed point μ∘δ=id, a comultiplication section, an idempotent projection). The two
-- are not a result with a gloss bolted on; they are two lossless faces of ONE act. This
-- module makes that precise: for each verb the chemical law and the mathematical law are
-- the SAME Lean proposition, and it holds by the kernel's existing μ∘δ=id theorems.
--
-- Grounded entirely in `Imscribing.Frobenius` (δ_A, μ_A, mu_delta_A_id) and the Core
-- Criticality axis — nothing new is assumed.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Frobenius

namespace Imscribing.Isomorphisms

open Primitives Frobenius
open Criticality

-- ============================================================
-- §1  click / fuse  —  Frobenius fusion μ_A
--   chemically:     two fragments bond on a live conjugate pair; a fragment bonding
--                   with ITSELF is unchanged (self-fusion is idempotent)
--   mathematically: μ ∘ δ = id on the diagonal — the Frobenius multiplication applied
--                   to a comultiplied element returns it
-- ============================================================

/-- The self-fusion of `a`: `μ_A a a` (`a` bonds with itself). -/
def clickSelf (a : Imscription) : Imscription := μ_A a a

/-- The chemical law (self-bond is idempotent) and the mathematical law (μ∘δ = id) are
    definitionally the SAME proposition — `δ_A a = (a, a)`, so `μ_A (δ_A a).1 (δ_A a).2`
    IS `μ_A a a = clickSelf a`. This `Iff.rfl` is the isomorphism itself: one theorem,
    two names. -/
theorem click_chem_iff_math (a : Imscription) :
    (clickSelf a = a) ↔ (μ_A (δ_A a).1 (δ_A a).2 = a) := Iff.rfl

/-- …and it holds, by the kernel's Frobenius identity. -/
theorem click_holds (a : Imscription) : clickSelf a = a := mu_delta_A_id a

-- ============================================================
-- §2  cycle  —  the catalytic fixed point
--   chemically:     a catalyst turns the substrate over and is returned UNCHANGED
--   mathematically: a fixed point of μ∘δ — an idempotent that is the identity on its carrier
--
-- The catalytic return is the SAME operation as `click`, read differently: turnover is
-- self-fusion. So `cycle` and `click` are one act.
-- ============================================================

/-- The catalyst `a` turns over and returns: `μ_A (δ_A a).1 (δ_A a).2`. -/
def catalyticReturn (a : Imscription) : Imscription := μ_A (δ_A a).1 (δ_A a).2

/-- The catalytic cycle IS the self-fusion click — turnover is self-fusion. -/
theorem cycle_is_click (a : Imscription) : catalyticReturn a = clickSelf a := rfl

/-- The catalyst is returned unchanged — the fixed point. -/
theorem cycle_fixed_point (a : Imscription) : catalyticReturn a = a := mu_delta_A_id a

-- ============================================================
-- §3  cleave / homolyze  —  the comultiplication δ_A, section of fusion
--   chemically:     homolytic cleavage splits a bond into fragments; fusing those
--                   fragments back restores the original (fission reverses bonding)
--   mathematically: δ is a section of μ — μ ∘ δ = id, so δ is a right inverse of fusion
-- ============================================================

/-- Cleave `a` into its two fragments: `δ_A a = (a, a)`. -/
def cleave (a : Imscription) : Imscription × Imscription := δ_A a

/-- Fusing the two cleavage fragments restores `a`: cleave is a section of fuse. The
    chemical statement (fission reverses bonding) and the mathematical statement (δ is a
    section of μ) are the same equation. -/
theorem cleave_section (a : Imscription) : μ_A (cleave a).1 (cleave a).2 = a :=
  mu_delta_A_id a

-- ============================================================
-- §4  excite  —  projection onto the exceptional point
--   chemically:     promotion to the excited state, Criticality ⊙ raised to the
--                   exceptional-point resonance; re-exciting an excited state does nothing
--   mathematically: the idempotent projection onto the exceptional-point criticality
--                   `err` (non-Hermitian eigenvector coalescence, the √-branch point)
-- ============================================================

/-- Excite `a`: raise its Criticality coordinate to the exceptional point `err`. -/
def excite (a : Imscription) : Imscription := { a with crit := err }

/-- Excitation reaches the exceptional point. -/
theorem excite_reaches_ep (a : Imscription) : (excite a).crit = err := rfl

/-- Excitation is idempotent: re-exciting an excited state changes nothing (chem), i.e.
    the projection onto `err` is idempotent (math). One equation, both readings. -/
theorem excite_idem (a : Imscription) : excite (excite a) = excite a := rfl

/-- Excitation touches only Criticality — every other coordinate is preserved, so the
    excited state is the SAME object at the exceptional point, not a different one. -/
theorem excite_preserves_off_crit (a : Imscription) :
    (excite a).dim = a.dim ∧ (excite a).top = a.top ∧ (excite a).rel = a.rel ∧
    (excite a).pol = a.pol ∧ (excite a).fid = a.fid ∧ (excite a).kin = a.kin ∧
    (excite a).gran = a.gran ∧ (excite a).gram = a.gram ∧ (excite a).chir = a.chir ∧
    (excite a).stoi = a.stoi ∧ (excite a).prot = a.prot :=
  ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

-- ============================================================
-- §5  The bidirectional key, as data
--
-- The prose glosses MoDoT prints (`── ISOMORPHISM ──`) recorded here beside the Lean
-- operation, so the two registers travel together and the string key stays in sync with
-- the proved content.
-- ============================================================

/-- A verb's two faces: what it means chemically and mathematically. -/
structure TwoFaces where
  verb : String
  chemically : String
  mathematically : String

def clickFaces : TwoFaces :=
  { verb := "click"
    chemically := "two fragments bond on a live conjugate pair; self-fusion is idempotent"
    mathematically := "Frobenius fusion μ∘δ = id on the diagonal (tensorProduct a a = a)" }

def cycleFaces : TwoFaces :=
  { verb := "cycle"
    chemically := "a catalyst turns the substrate over and is returned unchanged"
    mathematically := "a fixed point of μ∘δ — an idempotent, identity on its carrier" }

def cleaveFaces : TwoFaces :=
  { verb := "cleave"
    chemically := "homolytic cleavage into fragments; fusing them back restores the bond"
    mathematically := "δ is a section of μ (right inverse): μ∘δ = id" }

def exciteFaces : TwoFaces :=
  { verb := "excite"
    chemically := "promotion to the excited state — Criticality ⊙ raised to the exceptional point"
    mathematically := "idempotent projection onto exceptional-point criticality err" }

end Imscribing.Isomorphisms
