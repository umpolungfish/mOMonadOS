-- Imscribing/Paraconsistent/ExoticHadronBelnap.lean
-- EXOTIC HADRON BELNAP — Tetraquarks, Pentaquarks, Glueballs, Hybrids
-- Author: Lando ⊗ ⊙perator
import Imscribing.Paraconsistent.QuarkBelnap
import Imscribing.Paraconsistent.HadronBelnap
import Imscribing.Paraconsistent.OrbitalBelnap
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.TierCrossing
import Mathlib.Data.Finset.Basic
import Mathlib.Tactic

open Imscribing.Paraconsistent.QuarkBelnap
open Imscribing.Paraconsistent.Orbital
open Imscribing.Paraconsistent
open Imscribing.Primitives

namespace Imscribing.Paraconsistent.ExoticHadronBelnap

-- §1  GLUEBALL
inductive GluconColor : Type where
  | rg | rb | gr | gb | br | bg | rrdd | bbrd
  deriving DecidableEq, Repr, Inhabited

structure Glueball where
  gluons : Finset GluconColor
  gluonCount_at_least_two : gluons.card >= 2
  deriving DecidableEq

def glueballDepair (g : Glueball) : Glueball × Glueball := (g, g)

def glueballPair (g1 g2 : Glueball) : Option Glueball :=
  let merged := g1.gluons ∪ g2.gluons
  if h : merged.card >= 2 then
    some { gluons := merged, gluonCount_at_least_two := h }
  else none

theorem glueball_frobenius (g : Glueball) :
    glueballPair (glueballDepair g).1 (glueballDepair g).2 = some g := by
  unfold glueballDepair glueballPair; simp [g.gluonCount_at_least_two]

-- §2  TETRAQUARK
structure Tetraquark where
  q1 : QuarkState
  q2 : QuarkState
  aq1 : QuarkState
  aq2 : QuarkState
  anti_condition1 : aq1.color = antiColor q1.color
  anti_condition2 : aq2.color = antiColor q2.color
  total_white : colorJoin (colorJoin q1.color q2.color)
                           (colorJoin aq1.color aq2.color) = ColorState.White
  deriving DecidableEq

def tetraquarkDepair (t : Tetraquark) : (QuarkState × QuarkState) × (QuarkState × QuarkState) :=
  ((t.q1, t.aq1), (t.q2, t.aq2))

def tetraquarkPair (p1 p2 : QuarkState × QuarkState) : Option Tetraquark :=
  let (q1, aq1) := p1; let (q2, aq2) := p2
  if h1 : aq1.color = antiColor q1.color then
    if h2 : aq2.color = antiColor q2.color then
      let total := colorJoin (colorJoin q1.color q2.color) (colorJoin aq1.color aq2.color)
      if h3 : total = ColorState.White then
        some { q1 := q1, q2 := q2, aq1 := aq1, aq2 := aq2
               anti_condition1 := h1, anti_condition2 := h2, total_white := h3 }
      else none
    else none
  else none

theorem tetraquark_frobenius (t : Tetraquark) :
    tetraquarkPair (tetraquarkDepair t).1 (tetraquarkDepair t).2 = some t := by
  unfold tetraquarkDepair tetraquarkPair
  have total_eq : (colorJoin (colorJoin t.q1.color t.q2.color)
    (colorJoin (antiColor t.q1.color) (antiColor t.q2.color))) = ColorState.White := by
    calc
      colorJoin (colorJoin t.q1.color t.q2.color) (colorJoin (antiColor t.q1.color) (antiColor t.q2.color))
          = colorJoin (colorJoin t.q1.color t.q2.color) (colorJoin t.aq1.color t.aq2.color) := by
            rw [t.anti_condition1, t.anti_condition2]
      _ = ColorState.White := t.total_white
  simp [total_eq, t.anti_condition1, t.anti_condition2]

-- §3  PENTAQUARK
structure Pentaquark where
  q1 : QuarkState
  q2 : QuarkState
  q3 : QuarkState
  q4 : QuarkState
  aq1 : QuarkState
  total_white : colorJoin (colorJoin (colorJoin (colorJoin q1.color q2.color) q3.color) q4.color)
                           aq1.color = ColorState.White
  deriving DecidableEq

-- §4  UNIFIED HADRON TYPE
inductive AnyHadron : Type where
  | meson     (m : HadronBelnap.Meson) : AnyHadron
  | baryon    (b : HadronBelnap.Baryon) : AnyHadron
  | tetraquark (t : Tetraquark) : AnyHadron
  | pentaquark (p : Pentaquark) : AnyHadron
  | glueball   (g : Glueball) : AnyHadron
  deriving DecidableEq

-- §5  STRUCTURAL IMSCRIPTION
def exoticHadronImscription : Imscription :=
  { dim  := Dimensionality.array,    top  := Topology.mime
  , rel  := Relational.ear,      pol  := Polarity.out
  , fid  := Fidelity.peep,          kin  := KineticChar.egg
  , gran := Granularity.ice,      gram := Grammar.vow
  , crit := Criticality.monad,         chir := Chirality.sure
  , stoi := Stoichiometry.up,        prot := Protection.ah }

theorem exotic_hadron_tier : imscriptionTier exoticHadronImscription = OuroboricityTier.O₂dag := by
  native_decide

end Imscribing.Paraconsistent.ExoticHadronBelnap
