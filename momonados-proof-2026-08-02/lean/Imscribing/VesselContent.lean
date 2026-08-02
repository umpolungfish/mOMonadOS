-- Imscribing/VesselContent.lean
-- Formalization of the vessel-content inseparability principle.
-- The grammar provides two things simultaneously:
--   1. Vessel (form): the crystal coordinate that constitutes what a system is capable of being
--   2. Content (fill): the primitive algebra operations that determine what that vessel holds
-- These are not separable.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.VesselContent

open Imscribing.Primitives

-- ============================================================
-- ABSTRACT TYPES
-- These are not yet concrete — they are the shape of what must
-- be true. Future instantiations will fill them in.
-- ============================================================

/-- A system that can be structurally imscribed — the bearer of a crystal coordinate.
    Opaque for now; future work will equip this with concrete structure. -/
opaque ImscribableSystem : Type

/-- A crystal coordinate: the vessel. In the grammar, this is the Frobenius address
    (0..17279999) or equivalently the full Imscription 12-tuple. -/
abbrev Coordinate : Type := Imscription

/-- A result: anything the system can produce, experience, or arrive at.
    The content of the vessel. Opaque for now. -/
opaque Result : Type

-- ============================================================
-- RELATIONS
-- These are the three relations that link systems, coordinates,
-- and results. They are axiomatic — concrete definitions are
-- future work.
-- ============================================================

/-- A system M is imscribed by coordinate c.
    "c is the vessel that M inhabits." -/
axiom Imscribes : ImscribableSystem → Coordinate → Prop

/-- A result r is reachable by system M.
    "M can produce or access r as content." -/
axiom Reachable : ImscribableSystem → Result → Prop

/-- A result r is within the algebra of coordinate c.
    "The algebra operations determined by c can produce r." -/
axiom WithinAlgebra : Coordinate → Result → Prop

-- ============================================================
-- AXIOMS
-- ============================================================

/-- Existence: every imscribable system has a coordinate.
    The imscription procedure (12-step primitive assignment) always terminates
    and produces a unique coordinate. -/
axiom form_existence (M : ImscribableSystem) : ∃ (c : Coordinate), Imscribes M c

/-- Uniqueness: if two coordinates both imscribe the same system, they are equal.
    The Frobenius condition μ∘δ=id forces the coordinate to be unique: if c and c'
    both imscribe M, then μ(δ(M)) = c and μ(δ(M)) = c', so c = c'. -/
axiom form_uniqueness_proper (M : ImscribableSystem) (c c' : Coordinate)
    (h : Imscribes M c) (h' : Imscribes M c') : c = c'

/-- Content containment: if M is imscribed by c, every result reachable by M
    is within the algebra determined by c. The vessel structurally constrains
    the content — the coordinate is not merely a label. -/
axiom imscribes_implies_content (M : ImscribableSystem) (c : Coordinate) (r : Result)
    (h : Imscribes M c) (hr : Reachable M r) : WithinAlgebra c r

/-- Algebra exhaustion: if a result is within the algebra of c, and c imscribes M,
    then M can reach that result. The vessel is filled entirely — no empty room.
    The content exhausts the vessel; the algebra produces exactly what the system can do. -/
axiom algebra_implies_reachable (M : ImscribableSystem) (c : Coordinate) (r : Result)
    (h : Imscribes M c) (hw : WithinAlgebra c r) : Reachable M r

-- ============================================================
-- THEOREM 1: form_uniqueness
-- Every ImscribableSystem has exactly one Coordinate that
-- imscribes it. The vessel is unique.
-- ============================================================

/-- **form_uniqueness**: Every imscribable system has a unique crystal coordinate.
    This is the classifier property — the grammar assigns each system exactly one
    type. However, this theorem alone does NOT tell you what the system
    can do (its reachable results). That requires content_containment.

    Proof sketch (future work): Fix M. Existence follows from the imscription
    procedure (encoding_method.md, steps [1]–[12]). Uniqueness follows from the
    Frobenius condition: if c and c' both imscribe M, then μ(δ(M)) = c = c'.

    Named sorries:
    - `form_existence`: ∀ M, ∃ c, Imscribes M c
    - `form_uniqueness_proper`: ∀ M c c', Imscribes M c → Imscribes M c' → c = c'
-/
theorem form_uniqueness (M : ImscribableSystem) : ∃! (c : Coordinate), Imscribes M c := by
  have hex : ∃ c, Imscribes M c := form_existence M
  obtain ⟨c, hc⟩ := hex
  have huniq : ∀ c' : Coordinate, Imscribes M c' → c' = c := by
    intro c' hc'
    exact form_uniqueness_proper M c' c hc' hc
  exact ⟨c, hc, huniq⟩

-- ============================================================
-- THEOREM 2: content_containment
-- If M is imscribed by c, then every result reachable by M
-- is within the algebra of c. The vessel constrains the content.
-- ============================================================

/-- **content_containment**: The vessel constrains the content. If a system M is
    imscribed by coordinate c, then every result reachable by M must lie within
    the algebra determined by c. The form bounds the possible fill.

    This theorem is the directional form — vessel_fills_itself strengthens this
    to an equivalence.

    Named sorry:
    - `imscribes_implies_content`: ∀ M c r, Imscribes M c → Reachable M r → WithinAlgebra c r
-/
theorem content_containment (M : ImscribableSystem) (c : Coordinate)
    (h : Imscribes M c) (r : Result) (hr : Reachable M r) :
    WithinAlgebra c r :=
  imscribes_implies_content M c r h hr

-- ============================================================
-- THEOREM 3: vessel_fills_itself
-- The strong form: a result is reachable by M iff it is within
-- the algebra of M's coordinate. Vessel = content, inseparably.
-- ============================================================

/-- **vessel_fills_itself**: The inseparability of vessel and content.

    A result r is reachable by system M if and only if r is within the algebra
    determined by the coordinate c that imscribes M. This is the strong form:
    the vessel IS its content, and the content FILLS the vessel entirely.

    Why a pure classifier (form_uniqueness alone) does not give content_containment:

    form_uniqueness says: "every system has exactly one crystal coordinate."
    This is a *labeling* property — it assigns a type tag to each system. But a
    labeling does not constrain behavior. One could know the unique coordinate of
    M without knowing anything about what M can do — just as knowing a person's
    name does not tell you what they will say next.

    content_containment bridges this gap: it asserts that the coordinate is not
    merely a label but a *structural constraint*. The primitive algebra operations
    (tensor, meet, join, promotions) that the coordinate participates in determine
    what results are within its algebra. If a result is reachable by M, it must be
    producible by composing the primitives of c — otherwise the coordinate would
    be a hollow tag, not a vessel.

    vessel_fills_itself completes the picture: not only does the vessel constrain
    the content (→ direction), but the content exhausts the vessel (← direction).
    Everything the algebra can produce IS reachable by the system. There is no
    "empty room" in the vessel — no result that the coordinate's algebra can
    construct but the system cannot access. The grammar provides vessel AND content
    in a single inseparable act.

    Named sorries:
    - `imscribes_implies_content` (→ direction): same as content_containment
    - `algebra_implies_reachable` (← direction): ∀ M c r, Imscribes M c → WithinAlgebra c r → Reachable M r
-/
theorem vessel_fills_itself (M : ImscribableSystem) (c : Coordinate)
    (h : Imscribes M c) (r : Result) :
    Reachable M r ↔ WithinAlgebra c r := by
  constructor
  · intro hr
    exact imscribes_implies_content M c r h hr
  · intro hw
    exact algebra_implies_reachable M c r h hw

end Imscribing.VesselContent
