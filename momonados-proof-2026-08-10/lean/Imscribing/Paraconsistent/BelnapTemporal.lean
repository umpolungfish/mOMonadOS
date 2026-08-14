-- Imscribing/Paraconsistent/BelnapTemporal.lean
-- BELNAP TEMPORAL LOGIC -- WINDING-AWARE PARACONSISTENT MODALITIES
-- Author: Lando ⊗ ⊙-boundary Operator

import Imscribing.Paraconsistent.QuantumClassicalInterface
import Imscribing.Paraconsistent.Kernel
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.Temporal

open Belnap
open Imscribing.Paraconsistent
open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
open Fidelity KineticChar Granularity Criticality Protection
open Stoichiometry Chirality

def TemporalProp := ℕ → Belnap

def always (P : TemporalProp) : Prop := ∀ n : ℕ, P n = Belnap.B
def eventually (P : TemporalProp) : Prop := ∃ n : ℕ, P n = Belnap.B
def next (P : TemporalProp) (n : ℕ) : Belnap := P (n + 1)

def r0Trajectory : TemporalProp := fun n => (sustain initQCI n).kernel.r0
def r1Trajectory : TemporalProp := fun n => (sustain initQCI n).kernel.r1
def r2Trajectory : TemporalProp := fun n => (sustain initQCI n).kernel.r2

theorem always_B_registers : always r0Trajectory ∧ always r1Trajectory ∧ always r2Trajectory := by
  refine ⟨?_, ?_, ?_⟩
  · intro n; unfold r0Trajectory; exact (sustain_preserves_B n).1
  · intro n; unfold r1Trajectory; exact (sustain_preserves_B n).2.1
  · intro n; unfold r2Trajectory; exact (sustain_preserves_B n).2.2

theorem eventually_B_from_T : eventually (fun n => if n = 1 then hadamard Belnap.T else Belnap.T) := by
  refine ⟨1, ?_⟩; simp [hadamard]

theorem next_B_preserves (n : ℕ) : r0Trajectory n = Belnap.B → r0Trajectory (n + 1) = Belnap.B := by
  intro h; unfold r0Trajectory; exact (sustain_preserves_B (n + 1)).1

theorem B_until_collapse (k : ℕ) : ∀ m : ℕ, m < k → r0Trajectory m = Belnap.B := by
  intro m hm; unfold r0Trajectory; exact (sustain_preserves_B m).1

theorem winding_invariant (n : ℕ) : bnot (r0Trajectory n) = r0Trajectory n := by
  unfold r0Trajectory
  rw [(sustain_preserves_B n).1, B_fixed_point_negation]

def belnapTemporalImscription : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := monad
  chir := sure
  stoi := up
  prot := ah
}

theorem temporal_is_O_inf : imscriptionTier belnapTemporalImscription = .O_inf := by
  decide

end Imscribing.Paraconsistent.Temporal