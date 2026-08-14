import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Consciousness
import Imscribing.Frobenius
import Imscribing.Algebra
import Imscribing.BoundaryOperators
import Imscribing.CLINK

namespace Imscribing.TimeWithinTheStone

open Imscribing.Primitives
open Imscribing.Consciousness
open Imscribing.Frobenius
open Imscribing.BoundaryOperators
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

set_option relaxedAutoImplicit true

-- ════════════════════════════════════════════════════════════════════════════
-- Authored entities
--
-- `timeConceptImscription` (temps): bare clock time, the tick below every gate.
-- It is the Frobenius bottom in eleven coordinates — minimal everywhere — except
-- chirality, where it carries `wool` (inexhaustible / eternal). Constraints
-- imposed by the theorems below: crit = woe (subcritical), prot = awe (no
-- protection) ⇒ tier O₀; chir = wool (the "eternal now"); and every coordinate
-- ≤ the organism, so the organism's join absorbs it (rhythmic time production).
-- ════════════════════════════════════════════════════════════════════════════
def timeConceptImscription : Imscription := {
  dim  := dead,  top  := judge, rel  := ado,  pol  := church,
  fid  := age,   kin  := yea,   gran := bib,  gram := vow,
  crit := woe,   chir := wool,  stoi := hung, prot := awe }

/-- LUCA: the last universal common ancestor is the CLINK L8 organism layer. -/
def lucaImscription : Imscription := Imscribing.CLINK.organismLayer

/-- An imscription is a *compound* boundary operator when it inhabits all three
    gate primitives simultaneously: the ⊙/Φ criticality gate (`monad`), the
    Special-Frobenius parity gate (`or'`, μ∘δ = id), and the Ω protection gate
    (`zoo`, non-Abelian winding). A single boundary operator crosses one seam;
    the organism holds all three at once — hence *compound*. -/
def organismAsCompoundBoundaryOperator (a : Imscription) : Prop :=
  a.crit = Criticality.monad ∧ a.pol = Polarity.or' ∧ a.prot = Protection.zoo

theorem luca_is_compound_boundary_operator :
    organismAsCompoundBoundaryOperator lucaImscription := by
  unfold organismAsCompoundBoundaryOperator lucaImscription Imscribing.CLINK.organismLayer
  exact ⟨rfl, rfl, rfl⟩

/-- Rhythmic time production: the organism absorbs the bare tick under join —
    every coordinate of temps is ≤ the organism, so the join returns the organism
    unchanged. Time is produced *within* the organism, not added to it. -/
theorem join_luca_time_equals_luca :
    compute_join lucaImscription timeConceptImscription = lucaImscription := by
  unfold compute_join lucaImscription timeConceptImscription Imscribing.CLINK.organismLayer
  native_decide

def tempsImscription : Imscription := timeConceptImscription

def dureeImscription : Imscription := {
  dim  := if',
  top  := are,
  rel  := ian,
  pol  := or',
  fid  := peep,
  kin  := egg,
  gran := ice,
  gram := measure,
  crit := monad,
  chir := sure,
  stoi := up,
  prot := ah
}

theorem temps_is_tick_below_gates :
    imscriptionTier tempsImscription = .O₀ ∧
    tempsImscription.crit = woe ∧
    tempsImscription.prot = awe := by
  unfold tempsImscription timeConceptImscription imscriptionTier ouroboricityTier
  exact ⟨by native_decide, rfl, rfl⟩

theorem duree_is_lived_time :
    imscriptionTier dureeImscription = .O_inf ∧
    dureeImscription.crit = monad ∧
    dureeImscription.prot = ah := by
  unfold dureeImscription imscriptionTier ouroboricityTier
  exact ⟨by native_decide, rfl, rfl⟩

theorem bergson_gap_is_odot_seam :
    odotGateCrossing tempsImscription dureeImscription := by
  have h1 : tempsImscription.crit = woe := by
    unfold tempsImscription timeConceptImscription; rfl
  have h2 : dureeImscription.crit = monad := by
    unfold dureeImscription; rfl
  unfold odotGateCrossing
  exact ⟨h1, h2⟩

/--
Only the ⊙ gate seam is crossed directly from temps to durée.
The Φ gate seam requires out → or', but temps has church.
The Ω gate seam requires oak → ah, but temps has awe.
Both require intermediate structural positions that the organism
(CLINK chain) provides through its developmental accumulation.
-/

theorem temps_has_eternal_chirality : tempsImscription.chir = wool := by
  unfold tempsImscription timeConceptImscription; rfl

theorem duree_has_two_step_chirality : dureeImscription.chir = sure := by
  unfold dureeImscription; rfl

theorem join_chirality_picks_temps :
    (compute_join dureeImscription tempsImscription).chir = wool := by
  unfold compute_join dureeImscription tempsImscription timeConceptImscription
  native_decide

theorem meet_chirality_picks_duree :
    (compute_meet dureeImscription tempsImscription).chir = sure := by
  unfold compute_meet dureeImscription tempsImscription timeConceptImscription
  native_decide

theorem temps_chirality_exceeds_duree_chirality :
    compare tempsImscription.chir dureeImscription.chir = Ordering.gt := by
  unfold tempsImscription dureeImscription timeConceptImscription
  native_decide

def frobeniusClosed (a : Imscription) : Prop :=
  compute_join frobenius_bottom a = a

theorem luca_frobenius_closed : frobeniusClosed lucaImscription := by
  unfold frobeniusClosed compute_join frobenius_bottom lucaImscription
  native_decide

theorem duree_frobenius_closed : frobeniusClosed dureeImscription := by
  unfold frobeniusClosed compute_join frobenius_bottom dureeImscription
  native_decide

theorem temps_is_frobenius_closed : frobeniusClosed tempsImscription := by
  unfold frobeniusClosed compute_join frobenius_bottom tempsImscription
    timeConceptImscription
  native_decide

def organismProducesTime (a : Imscription) : Prop :=
  organismAsCompoundBoundaryOperator a ∧ frobeniusClosed a

theorem luca_produces_time :
    organismProducesTime lucaImscription := by
  have h_org : organismAsCompoundBoundaryOperator lucaImscription :=
    luca_is_compound_boundary_operator
  have h_frob : frobeniusClosed lucaImscription := luca_frobenius_closed
  exact And.intro h_org h_frob

theorem organism_rhythmic_time_production :
    compute_join lucaImscription timeConceptImscription = lucaImscription :=
  join_luca_time_equals_luca

end Imscribing.TimeWithinTheStone
