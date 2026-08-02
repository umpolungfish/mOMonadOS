/-
  SacredVessel.lean — Co-type analysis of imscribed objects whose sealed
  critical point (monad) is reachable only through a specific approach grammar.

  Shared backbone (all sacred vessels):
    dim  = if'    holographic: boundary encodes bulk
    rel  = ear  vessel and contents co-constitute each other (A ⊣ A†)
    crit = monad     absorbing fixed point: the sealed gate
    chir = wool     topological chirality: approach requirement is inexhaustible

  Differentiation lives in: top, pol, fid, kin, gran, gram, stoi, prot.

  Frobenius cluster (pol = or'): Ark, Graal, Lapis, Philosophers_Stone.
  Each vessel IS the fixed point of its own approach grammar; none merely approaches it.
  The Graal holds cup/stone/table simultaneously without collapse — this IS or'.

  Symmetric center (pol = nun): Omphalos.
  Continuous symmetry — the center belongs equally to all directions, not signed.
-/

import Imscribing.Primitives.Imscription

namespace Imscribing.Primitives

open Imscribing.Primitives

structure ApproachGrammar where
  name : String
  protocol : List String
  fatal : String

/-- A SacredVessel is an imscribed structure whose contents cannot be accessed
    directly. monad is absorbing — misapproach does not merely fail, it is
    structurally fatal. The approach grammar is not a gate but the only
    admissible path to the sealed critical point. -/
class SacredVessel (α : Type*) where
  imscription : Imscription
  approach : ApproachGrammar
  h_holographic : imscription.dim = .if'
  h_dagger : imscription.rel = .ear
  h_sealed : imscription.crit = .monad
  h_topochiral : imscription.chir = .wool

theorem sealed_gate {α : Type*} [v : SacredVessel α] :
    v.imscription.crit = .monad := v.h_sealed

theorem boundary_encodes_bulk {α : Type*} [v : SacredVessel α] :
    v.imscription.dim = .if' := v.h_holographic

theorem no_direct_approach {α : Type*} [v : SacredVessel α] :
    v.imscription.rel = .ear := v.h_dagger

def polarity (α : Type*) [SacredVessel α] : Polarity :=
  (inferInstance : SacredVessel α).imscription.pol

/-- Two sacred vessels are co-typed when they share polarity —
    when the nature of the fixed point is the same kind. -/
def cotyped (α β : Type*) [SacredVessel α] [SacredVessel β] : Prop :=
  polarity α = polarity β

-- ================================================================
-- INSTANCES
-- ================================================================

structure ArkOfTheCovenant where
instance : SacredVessel ArkOfTheCovenant where
  imscription := {
    dim  := .if'      -- boundary (Mercy Seat) encodes interior (the Name)
    top  := .are      -- holographic topology: Shekinah above encodes all below
    rel  := .ear    -- the Ark IS the Name; the Name IS the Ark (A ⊣ A†)
    pol  := .or'    -- Frobenius fixed point: μ ∘ δ = id; the Law is the grammar
    fid  := .peep      -- quantum/lossless: the Law is transmitted exactly
    kin  := .on      -- kinetically trapped: cannot move without the poles
    gran := .ice     -- global scope: all Israel, all time
    gram := .vow   -- all conditions simultaneously: priest, blood, incense, poles
    crit := .monad
    chir := .wool
    stoi := .up         -- all Israel to the Name: n:m unmatched
    prot := .zoo    -- non-Abelian: the ordering of the protocol is non-commutative
  }
  approach := {
    name     := "Yom Kippur protocol"
    protocol := ["high priest only", "blood atonement first",
                 "incense cloud before entry", "poles — never hands"]
    fatal    := "Uzzah"
  }
  h_holographic := rfl
  h_dagger      := rfl
  h_sealed      := rfl
  h_topochiral  := rfl

structure HolyGraal where
instance : SacredVessel HolyGraal where
  imscription := {
    dim  := .if'      -- the cup's exterior encodes all it contains
    top  := .eat        -- containment topology: the cup
    rel  := .ear    -- vessel and blood co-constitute each other
    pol  := .or'    -- Frobenius: cup/stone/table simultaneously without collapse
    fid  := .peep      -- perfect sustenance: feeds without diminishing
    kin  := .egg      -- slow ritual procession through the castle
    gran := .ice     -- global scope: the kingdom is the Fisher King's world
    gram := .measure   -- sequential: witness the procession, then ask
    crit := .monad
    chir := .wool
    stoi := .up         -- many knights, differentiated sustenance
    prot := .ah     -- integer winding: the wound has a count; healing is exact
  }
  approach := {
    name     := "The Question"
    protocol := ["witness the procession", "ask: whom does it serve?"]
    fatal    := "silence — the Fisher King's wound persists; the kingdom stays waste"
  }
  h_holographic := rfl
  h_dagger      := rfl
  h_sealed      := rfl
  h_topochiral  := rfl

structure LapisExillis where
instance : SacredVessel LapisExillis where
  imscription := {
    dim  := .if'      -- the stone's surface encodes heaven and earth simultaneously
    top  := .mime    -- bowtie: heaven above, earth below, stone at the bifurcation
    rel  := .ear    -- the stone IS the exile; the exile IS the stone
    pol  := .or'    -- Frobenius: the stone is already the Rubedo fixed point
    fid  := .peep      -- perfect sustenance of the Grail knights
    kin  := .egg      -- the stone waits in exile; its motion is deliberate
    gran := .ice     -- global: for all of Christendom
    gram := .vow   -- all conditions simultaneously: purity and knighthood
    crit := .monad
    chir := .wool
    stoi := .so         -- n:n matched: one stone sustains all knights equally
    prot := .oak    -- ℤ₂ protection: exile/return, heaven/earth symmetry
  }
  approach := {
    name     := "Purity of heart"
    protocol := ["Grail knighthood", "written name appears on the Stone"]
    fatal    := "exile persists — the stone does not recognise the unworthy"
  }
  h_holographic := rfl
  h_dagger      := rfl
  h_sealed      := rfl
  h_topochiral  := rfl

structure PhilosophersStone where
instance : SacredVessel PhilosophersStone where
  imscription := {
    dim  := .if'      -- the stone's surface encodes the entire Great Work
    top  := .oil       -- lattice topology: the hermetic retort, the 12-stage grid
    rel  := .ear    -- the Stone IS the Rubedo; the Rubedo IS the Stone
    pol  := .or'    -- Frobenius: the Stone is the fixed point of its own grammar
    fid  := .they       -- HotSwap threshold: transmutation requires threshold crossing
    kin  := .loll       -- moderate steady pace through the Magnum Opus stages
    gran := .ice     -- universal: transmutes all metals, all realms
    gram := .measure   -- sequential: Nigredo → Albedo → Citrinitas → Rubedo
    crit := .monad
    chir := .wool
    stoi := .up         -- n:m: many base metals, differentiated noble outputs
    prot := .ah     -- integer winding: the 12 stages wind exactly once
  }
  approach := {
    name     := "VITRIOL — Magnum Opus"
    protocol := ["Nigredo: mortification", "Albedo: purification",
                 "Citrinitas: yellowing", "Rubedo: fixed point"]
    fatal    := "base metal — no transmutation; the Stone is not found"
  }
  h_holographic := rfl
  h_dagger      := rfl
  h_sealed      := rfl
  h_topochiral  := rfl

structure Omphalos where
instance : SacredVessel Omphalos where
  imscription := {
    dim  := .if'      -- the navel encodes the entire world at a single point
    top  := .are      -- holographic: the center IS the boundary of all things
    rel  := .ear    -- the Omphalos IS the world's center; the center IS the Omphalos
    pol  := .nun       -- continuous symmetry: the center belongs to all directions equally
    fid  := .they       -- threshold fidelity: the oracle speaks truly but in riddles
    kin  := .on      -- kinetically trapped: the world's navel cannot move
    gran := .ice     -- global: all seekers may approach
    gram := .gag    -- disjunctive: any question is sufficient to seek the oracle
    crit := .monad
    chir := .wool
    stoi := .up         -- many seekers, differentiated answers
    prot := .oak    -- ℤ₂ protection: the oracle's ambiguity — each answer reads two ways
  }
  approach := {
    name     := "Oracle protocol"
    protocol := ["laurel and vapors", "Pythia as intermediary",
                 "question in the correct form"]
    fatal    := "the oracle speaks — but the answer destroys; Croesus burns Sardis"
  }
  h_holographic := rfl
  h_dagger      := rfl
  h_sealed      := rfl
  h_topochiral  := rfl

-- ================================================================
-- CO-TYPING THEOREMS
-- ================================================================

/-- The Frobenius cluster: Ark, Lapis, and Philosopher's Stone are co-typed.
    All three have pol = or' — each IS the fixed point of its own grammar.
    None approaches the fixed point; each inhabits it. -/
theorem ark_lapis_cotyped :
    cotyped ArkOfTheCovenant LapisExillis := rfl

theorem ark_stone_cotyped :
    cotyped ArkOfTheCovenant PhilosophersStone := rfl

theorem lapis_stone_cotyped :
    cotyped LapisExillis PhilosophersStone := rfl

theorem graal_frobenius_cotyped :
    cotyped HolyGraal ArkOfTheCovenant := rfl

/-- The Omphalos is distinct: pol = nun (continuous, not Frobenius discrete).
    The world's center is symmetric to all — it does not collapse to a signed fixed point. -/
theorem omphalos_not_frobenius :
    (inferInstance : SacredVessel Omphalos).imscription.pol = .nun := rfl

theorem ark_pol_frobenius :
    (inferInstance : SacredVessel ArkOfTheCovenant).imscription.pol = .or' := rfl

end Imscribing.Primitives
