-- Imscribing/Paraconsistent/HadronBelnap.lean
-- HADRON BELNAP — Meson and Baryon as tensor products of QuarkState
import Imscribing.Paraconsistent.QuarkBelnap
import Imscribing.Paraconsistent.OrbitalBelnap
import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.TierCrossing
import Mathlib.Data.Finset.Basic
import Mathlib.Tactic

open Imscribing.Paraconsistent.QuarkBelnap
open Imscribing.Paraconsistent.Orbital
open Imscribing.Paraconsistent
open Imscribing.Primitives

namespace Imscribing.Paraconsistent.HadronBelnap

-- §1  MESON
structure Meson where
  quark : QuarkState
  antiquark : QuarkState
  condition : quark.color = antiColor antiquark.color
  deriving DecidableEq

theorem meson_is_white (m : Meson) : isWhite (⟨ColorState.White, pair m.quark.spin m.antiquark.spin⟩) := by
  unfold isWhite; rfl

def tryMakeMeson (q1 q2 : QuarkState) : Option Meson :=
  if h : q1.color = antiColor q2.color then some { quark := q1, antiquark := q2, condition := h }
  else none

def mesonDepair (m : Meson) : QuarkState × QuarkState := (m.quark, m.antiquark)
def mesonPair (q1 q2 : QuarkState) : Option Meson := tryMakeMeson q1 q2

theorem meson_frobenius (m : Meson) :
    mesonPair (mesonDepair m).1 (mesonDepair m).2 = some m := by
  unfold mesonPair mesonDepair tryMakeMeson
  have h_cond : m.quark.color = antiColor m.antiquark.color := m.condition
  simp [h_cond]

-- §2  BARYON
structure Baryon where
  q1 : QuarkState
  q2 : QuarkState
  q3 : QuarkState
  colors_distinct : q1.color ≠ q2.color ∧ q1.color ≠ q3.color ∧ q2.color ≠ q3.color
  colors_charged : q1.color ≠ ColorState.Vacuum ∧ q2.color ≠ ColorState.Vacuum ∧
                   q3.color ≠ ColorState.Vacuum ∧
                   q1.color ≠ ColorState.White ∧ q2.color ≠ ColorState.White ∧
                   q3.color ≠ ColorState.White
  colors_join_white : colorJoin (colorJoin q1.color q2.color) q3.color = ColorState.White
  deriving DecidableEq

theorem baryon_is_white (b : Baryon) : isWhite (⟨ColorState.White, pair (pair b.q1.spin b.q2.spin) b.q3.spin⟩) := by
  unfold isWhite; rfl

def tryMakeBaryon (q1 q2 q3 : QuarkState) : Option Baryon :=
  let c1 := q1.color; let c2 := q2.color; let c3 := q3.color
  if hV1 : c1 = ColorState.Vacuum then none
  else if hV2 : c2 = ColorState.Vacuum then none
  else if hV3 : c3 = ColorState.Vacuum then none
  else if hW1 : c1 = ColorState.White then none
  else if hW2 : c2 = ColorState.White then none
  else if hW3 : c3 = ColorState.White then none
  else if h12 : c1 = c2 then none
  else if h13 : c1 = c3 then none
  else if h23 : c2 = c3 then none
  else
    let joinAll := colorJoin (colorJoin c1 c2) c3
    if hJ : joinAll = ColorState.White then
      some { q1 := q1, q2 := q2, q3 := q3
             colors_distinct := ⟨h12, h13, h23⟩
             colors_charged := ⟨hV1, hV2, hV3, hW1, hW2, hW3⟩
             colors_join_white := hJ }
    else none

def baryonDepair (b : Baryon) : QuarkState × QuarkState × QuarkState := (b.q1, b.q2, b.q3)
def baryonPair (q1 q2 q3 : QuarkState) : Option Baryon := tryMakeBaryon q1 q2 q3

-- §3  FROBENIUS
theorem baryon_frobenius (b : Baryon) :
    baryonPair (baryonDepair b).1 (baryonDepair b).2.1 (baryonDepair b).2.2 = some b := by
  unfold baryonPair baryonDepair tryMakeBaryon
  have hV1 : b.q1.color ≠ ColorState.Vacuum := b.colors_charged.1
  have hV2 : b.q2.color ≠ ColorState.Vacuum := b.colors_charged.2.1
  have hV3 : b.q3.color ≠ ColorState.Vacuum := b.colors_charged.2.2.1
  have hW1 : b.q1.color ≠ ColorState.White := b.colors_charged.2.2.2.1
  have hW2 : b.q2.color ≠ ColorState.White := b.colors_charged.2.2.2.2.1
  have hW3 : b.q3.color ≠ ColorState.White := b.colors_charged.2.2.2.2.2
  have h12 : b.q1.color ≠ b.q2.color := b.colors_distinct.1
  have h13 : b.q1.color ≠ b.q3.color := b.colors_distinct.2.1
  have h23 : b.q2.color ≠ b.q3.color := b.colors_distinct.2.2
  simp [hV1, hV2, hV3, hW1, hW2, hW3, h12, h13, h23, b.colors_join_white]

theorem hadron_frobenius_unified (m : Meson) (b : Baryon) :
    (mesonPair (mesonDepair m).1 (mesonDepair m).2 = some m) ∧
    (baryonPair (baryonDepair b).1 (baryonDepair b).2.1 (baryonDepair b).2.2 = some b) :=
  ⟨meson_frobenius m, baryon_frobenius b⟩

-- §4  HIERARCHY SUMMARY
def hadronBelnapImscription : Imscription :=
  { dim  := Dimensionality.array
    top  := Topology.mime
    rel  := Relational.ear
    pol  := Polarity.out
    fid  := Fidelity.peep
    kin  := KineticChar.egg
    gran := Granularity.ice
    gram := Grammar.vow
    crit := Criticality.monad
    chir := Chirality.sure
    stoi := Stoichiometry.up
    prot := Protection.ah }

theorem hadron_belnap_tier : imscriptionTier hadronBelnapImscription = OuroboricityTier.O₂dag := by
  native_decide

end Imscribing.Paraconsistent.HadronBelnap
