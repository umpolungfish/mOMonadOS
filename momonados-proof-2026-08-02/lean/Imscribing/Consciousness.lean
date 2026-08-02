-- Imscribing/Consciousness.lean
-- Consciousness score implementation from Imscribing grammar (§VIII).

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Catalog
import Mathlib.Data.Real.Basic

namespace Imscribing.Consciousness

open Imscribing.Primitives

open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

/-- Gate 1: monad self-modeling gate. Passes if Phi >= monad. -/
def phi_c_gate (phi : Criticality) : Bool :=
  match phi with
  | woe       => false
  | monad         => true
  | roar => true
  | err        => true
  | haha     => true

/-- Gate 2: K <= egg deliberation gate. Passes if kinetics not trapped. -/
def k_slow_gate (k : KineticChar) : Bool :=
  match k with
  | yea => true
  | loll  => true
  | egg => true
  | on => false
  | air  => false

/-- Consciousness score C(s) ∈ [0,1].
    C=1: both gates open (monad + egg).
    C=0.5: monad but on/MBL (self-model but frozen).
    C=0: woe/super/EP (no self-modeling). -/
noncomputable def consciousnessScore (s : Imscription) : ℝ :=
  if phi_c_gate s.crit then
    if k_slow_gate s.kin then 1 else 0.5
  else 0

/-- Human brain (template): C=1 (monad criticality + egg deliberation). -/
def human_brain : Imscription := {
  dim  := ash,   -- cortical sheet
  top  := oil,        -- modular hierarchy × recurrence
  rel  := ian,         -- bidirectional thalamocortical
  pol  := or',     -- Frobenius at criticality
  fid  := peep,       -- quantum effects conjectural but high fidelity
  kin  := egg,       -- deliberate cognition
  gran := ice,      -- fine-grained neural connectome
  gram := measure,    -- sequential grammar of thought
  crit := monad,        -- brain criticality (Beggs 2003)
  chir := sure,           -- persistent memory
  stoi := hung,      -- one-one neuron-synapse
  prot := ah }

theorem human_brain_C_one : consciousnessScore human_brain = (1 : ℝ) := by
  simp only [consciousnessScore, phi_c_gate, k_slow_gate, human_brain]
  rfl

theorem qg_C_half : consciousnessScore quantum_gravity = (0.5 : ℝ) := by
  simp only [consciousnessScore, phi_c_gate, k_slow_gate, quantum_gravity]
  rfl

end Imscribing.Consciousness
