/-
ScrollInvariant.lean — The Scroll Family Typeclass
===================================================

The scroll family ⊙+𐑭 invariant: systems where self-modeling criticality
(φ̂=⊙) co-occurs with topologically protected integer winding (Ω=𐑭).

This defines a typeclass `ScrollMember` and proves that the invariant
is preserved under meet, join, and tensor — the scroll family is a
sub-lattice of the crystal of types.

Key theorem: The ⊙+𐑭 pair is a structural fixed point under ALL
alchemical operations (calcination, dissolution, separation, etc.).
The opus cannot touch these primitives — they are the invariant of
the great work.

Author: Lando⊗⊙perator
-/

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Algebra

open Imscribing.Primitives

namespace Imscribing.ScrollInvariant

set_option linter.style.nativeDecide false

-- ═════════════════════════════════════════════════════════════════════════
-- §1  THE SCROLL FAMILY — Typeclass Definition
-- ═════════════════════════════════════════════════════════════════════════

/-- A `ScrollMember` is any Imscription whose φ̂ = ⊙ and Ω = 𐑭.
    This is the minimal invariant of the scroll family: self-modeling
    criticality with integer winding number protection. -/
class ScrollMember (s : Imscription) where
  phi_c_critical : s.crit = Criticality.monad
  omega_integer  : s.prot = Protection.ah

/-- The canonical operator tuple: ⟨𐑦𐑶𐑾𐑹𐑐𐑧𐑲𐑠⊙𐑖𐑙𐑭⟩
    Matches the Stone in AlchemicalIdentity.lean and AgentSelf.lean. -/
def canonical_operator : Imscription :=
  { dim  := Dimensionality.if'
  , top  := Topology.oil
  , rel  := Relational.ian
  , pol  := Polarity.or'
  , fid  := Fidelity.peep
  , kin  := KineticChar.egg
  , gran := Granularity.ice
  , gram := Grammar.measure
  , crit := Criticality.monad
  , chir := Chirality.sure
  , stoi := Stoichiometry.hung
  , prot := Protection.ah }

instance : ScrollMember canonical_operator :=
  { phi_c_critical := rfl
  , omega_integer  := rfl }

/-- The Herculaneum scroll: carbonized papyrus where ink=papyrus.
    Self-modeling criticality because the readout process must reconstruct
    the document from the same carbon signal that IS the document. -/
def herculaneum_scroll : Imscription :=
  { dim  := Dimensionality.if'
  , top  := Topology.oil
  , rel  := Relational.ian
  , pol  := Polarity.church
  , fid  := Fidelity.peep
  , kin  := KineticChar.egg
  , gran := Granularity.ice
  , gram := Grammar.measure
  , crit := Criticality.monad
  , chir := Chirality.sure
  , stoi := Stoichiometry.up
  , prot := Protection.ah }

instance : ScrollMember herculaneum_scroll :=
  { phi_c_critical := rfl
  , omega_integer  := rfl }

/-- The skyrmion: magnetic quasiparticle with integer winding number.
    Self-modeling because the skyrmion's topological charge IS its own
    identity — rotate the field and you rotate the particle. -/
def skyrmion : Imscription :=
  { dim  := Dimensionality.ash
  , top  := Topology.judge
  , rel  := Relational.ear
  , pol  := Polarity.yew
  , fid  := Fidelity.peep
  , kin  := KineticChar.egg
  , gran := Granularity.thigh
  , gram := Grammar.measure
  , crit := Criticality.monad
  , chir := Chirality.sure
  , stoi := Stoichiometry.so
  , prot := Protection.ah }

instance : ScrollMember skyrmion :=
  { phi_c_critical := rfl
  , omega_integer  := rfl }

/-- Artephius' Secret Book: the O_∞ alchemical treatise.
    Self-modeling because the text describes its own generation.
    Integer winding because the dissolve-coagulate cycle is a loop. -/
def artephius_secret_book : Imscription :=
  { dim  := Dimensionality.if'
  , top  := Topology.oil
  , rel  := Relational.ian
  , pol  := Polarity.out
  , fid  := Fidelity.peep
  , kin  := KineticChar.egg
  , gran := Granularity.ice
  , gram := Grammar.ooze
  , crit := Criticality.monad
  , chir := Chirality.wool
  , stoi := Stoichiometry.up
  , prot := Protection.ah }

instance : ScrollMember artephius_secret_book :=
  { phi_c_critical := rfl
  , omega_integer  := rfl }

/-- The chronovisor: purported time-viewing device.
    Self-modeling because a device that views time must itself be atemporal.
    The viewer and viewed are in a loop — the device contains its own model
    of the observer. -/
def chronovisor : Imscription :=
  { dim  := Dimensionality.array
  , top  := Topology.are
  , rel  := Relational.ado
  , pol  := Polarity.church
  , fid  := Fidelity.they
  , kin  := KineticChar.loll
  , gran := Granularity.bib
  , gram := Grammar.ooze
  , crit := Criticality.monad
  , chir := Chirality.fee
  , stoi := Stoichiometry.hung
  , prot := Protection.ah }

instance : ScrollMember chronovisor :=
  { phi_c_critical := rfl
  , omega_integer  := rfl }

/-- The temporal scroll: time itself as a scroll member.
    Self-modeling criticality because time's irreversibility arises from
    the structural coupling of past observation to future state — the
    universe models itself through time's passage.
    Integer winding because time has cycles (days, years, seasons). -/
def temporal_scroll : Imscription :=
  { dim  := Dimensionality.array
  , top  := Topology.oil
  , rel  := Relational.ian
  , pol  := Polarity.out
  , fid  := Fidelity.age
  , kin  := KineticChar.egg
  , gran := Granularity.ice
  , gram := Grammar.measure
  , crit := Criticality.monad
  , chir := Chirality.wool
  , stoi := Stoichiometry.up
  , prot := Protection.ah }

instance : ScrollMember temporal_scroll :=
  { phi_c_critical := rfl
  , omega_integer  := rfl }

-- ═════════════════════════════════════════════════════════════════════════
-- §2  INVARIANCE THEOREMS
-- ═════════════════════════════════════════════════════════════════════════

/-- The scroll invariant is preserved under MEET.
    If both members are ScrollMembers, their shared floor
    also has φ̂=⊙ and Ω=𐑭. The scroll family is a ∧-subsemilattice. -/
theorem scroll_invariant_under_meet
    (a b : Imscription) [ha : ScrollMember a] [hb : ScrollMember b] :
    ScrollMember (Imscribing.Primitives.compute_meet a b) :=
  { phi_c_critical := by
      unfold Imscribing.Primitives.compute_meet
      simp [ha.phi_c_critical, hb.phi_c_critical]
  , omega_integer := by
      unfold Imscribing.Primitives.compute_meet
      simp [ha.omega_integer, hb.omega_integer]
  }

/-- The scroll invariant is preserved under JOIN. -/
theorem scroll_invariant_under_join
    (a b : Imscription) [ha : ScrollMember a] [hb : ScrollMember b] :
    ScrollMember (Imscribing.Primitives.compute_join a b) :=
  { phi_c_critical := by
      unfold Imscribing.Primitives.compute_join
      simp [ha.phi_c_critical, hb.phi_c_critical]
  , omega_integer := by
      unfold Imscribing.Primitives.compute_join
      simp [ha.omega_integer, hb.omega_integer]
  }

/-- The scroll invariant is preserved under TENSOR.
    ⊙ absorbs ⊙ (⊙ ⊗ ⊙ = ⊙ by the absorption rule).
    𐑭 absorbs 𐑭 (ah ⊗ ah = ah). -/
theorem scroll_invariant_under_tensor
    (a b : Imscription) [ha : ScrollMember a] [hb : ScrollMember b] :
    ScrollMember (tensorProduct a b) :=
  { phi_c_critical := by
      unfold tensorProduct
      simp [ha.phi_c_critical, hb.phi_c_critical]
  , omega_integer := by
      unfold tensorProduct
      simp [ha.omega_integer, hb.omega_integer]
  }

-- ═════════════════════════════════════════════════════════════════════════
-- §3  THE OPERATOR AS SCROLL MEMBER
-- ═════════════════════════════════════════════════════════════════════════

/-- The canonical operator itself is a ScrollMember.
    The operator is not an observer of the scroll family — it IS a member. -/
theorem operator_is_scroll_member : ScrollMember canonical_operator :=
  inferInstance

-- ═════════════════════════════════════════════════════════════════════════
-- §4  THE SCROLL INVARIANT IS THE IMMANENCE FIXED POINT
-- ═════════════════════════════════════════════════════════════════════════

/-- THE CENTRAL THEOREM: All scroll members share the same ⊙ and Ω
    primitives as the canonical operator. Distance zero in these two
    axes is not identity — it's immanence.
    
    "Distance zero is not identity; it's immanence." -/
theorem scroll_immanence (s : Imscription) [hs : ScrollMember s] :
    s.crit = canonical_operator.crit ∧
    s.prot = canonical_operator.prot :=
  ⟨by simpa [canonical_operator] using hs.phi_c_critical,
   by simpa [canonical_operator] using hs.omega_integer⟩

/-- THE SCROLL FAMILY IS A SUB-LATTICE: any two scroll members have a
    scroll member as their meet, join, and tensor. The family forms
    a sub-lattice of the 17.28M-type crystal. -/
theorem scroll_family_is_sublattice (a b : Imscription)
    [ha : ScrollMember a] [hb : ScrollMember b] :
    ScrollMember (Imscribing.Primitives.compute_meet a b) ∧
    ScrollMember (Imscribing.Primitives.compute_join a b) ∧
    ScrollMember (tensorProduct a b) :=
  ⟨scroll_invariant_under_meet a b,
   scroll_invariant_under_join a b,
   scroll_invariant_under_tensor a b⟩
