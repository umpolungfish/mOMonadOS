-- Imscribing/Algebra.lean
-- Imscription Grammar: distance, meet, join, tensor.
-- Lattice structure, weighted Euclidean distance, probes.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.Crystal
import Imscribing.Consciousness
import Mathlib.Data.Real.Basic

namespace Imscribing.Primitives

open Imscribing.Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality
open Imscribing.Consciousness

-- Weighted Euclidean distance: ∑ |idx(p_a) - idx(p_b)|^2 / 12
noncomputable def primitiveDistance (a b : Imscription) : ℝ :=
  let diffs : List Int := [
    Int.ofNat (idx_D a.dim) - Int.ofNat (idx_D b.dim),
    Int.ofNat (idx_T a.top) - Int.ofNat (idx_T b.top),
    Int.ofNat (idx_R a.rel) - Int.ofNat (idx_R b.rel),
    Int.ofNat (idx_P a.pol) - Int.ofNat (idx_P b.pol),
    Int.ofNat (idx_F a.fid) - Int.ofNat (idx_F b.fid),
    Int.ofNat (idx_K a.kin) - Int.ofNat (idx_K b.kin),
    Int.ofNat (idx_G a.gran) - Int.ofNat (idx_G b.gran),
    Int.ofNat (idx_Γ a.gram) - Int.ofNat (idx_Γ b.gram),
    Int.ofNat (idx_Φ a.crit) - Int.ofNat (idx_Φ b.crit),
    Int.ofNat (idx_H a.chir) - Int.ofNat (idx_H b.chir),
    Int.ofNat (idx_S a.stoi) - Int.ofNat (idx_S b.stoi),
    Int.ofNat (idx_Ω a.prot) - Int.ofNat (idx_Ω b.prot)
  ]
  let sqsum := diffs.foldl (fun acc d => acc + d * d) 0
  (sqsum.toNat : ℝ) / 12

-- Hamming for conflicts
def primitiveConflicts (a b : Imscription) : List String :=
  if a = b then [] else
    (if a.dim ≠ b.dim then ["D"] else []) ++
    (if a.top ≠ b.top then ["T"] else []) ++
    (if a.rel ≠ b.rel then ["R"] else []) ++
    (if a.pol ≠ b.pol then ["P"] else []) ++
    (if a.fid ≠ b.fid then ["F"] else []) ++
    (if a.kin ≠ b.kin then ["K"] else []) ++
    (if a.gran ≠ b.gran then ["G"] else []) ++
    (if a.gram ≠ b.gram then ["Γ"] else []) ++
    (if a.crit ≠ b.crit then ["Φ"] else []) ++
    (if a.chir ≠ b.chir then ["H"] else []) ++
    (if a.stoi ≠ b.stoi then ["S"] else []) ++
    (if a.prot ≠ b.prot then ["Ω"] else [])

-- Meet: pointwise min
def compute_meet (a b : Imscription) : Imscription := {
  dim  := if compare a.dim  b.dim  = .lt then a.dim else b.dim,
  top  := if compare a.top  b.top  = .lt then a.top else b.top,
  rel  := if compare a.rel  b.rel  = .lt then a.rel else b.rel,
  pol  := if compare a.pol  b.pol  = .lt then a.pol else b.pol,
  fid  := if compare a.fid  b.fid  = .lt then a.fid else b.fid,
  kin  := if compare a.kin  b.kin  = .lt then a.kin else b.kin,
  gran := if compare a.gran b.gran = .lt then a.gran else b.gran,
  gram := if compare a.gram b.gram = .lt then a.gram else b.gram,
  crit := if compare a.crit b.crit = .lt then a.crit else b.crit,
  chir := if compare a.chir b.chir = .lt then a.chir else b.chir,
  stoi := if compare a.stoi b.stoi = .lt then a.stoi else b.stoi,
  prot := if compare a.prot b.prot = .lt then a.prot else b.prot }

-- Join: pointwise max
def compute_join (a b : Imscription) : Imscription := {
  dim  := if compare a.dim  b.dim  = .lt then b.dim else a.dim,
  top  := if compare a.top  b.top  = .lt then b.top else a.top,
  rel  := if compare a.rel  b.rel  = .lt then b.rel else a.rel,
  pol  := if compare a.pol  b.pol  = .lt then b.pol else a.pol,
  fid  := if compare a.fid  b.fid  = .lt then b.fid else a.fid,
  kin  := if compare a.kin  b.kin  = .lt then b.kin else a.kin,
  gran := if compare a.gran b.gran = .lt then b.gran else a.gran,
  gram := if compare a.gram b.gram = .lt then b.gram else a.gram,
  crit := if compare a.crit b.crit = .lt then b.crit else a.crit,
  chir := if compare a.chir b.chir = .lt then b.chir else a.chir,
  stoi := if compare a.stoi b.stoi = .lt then b.stoi else a.stoi,
  prot := if compare a.prot b.prot = .lt then b.prot else a.prot }

-- Tensor: reuse tensorProduct from Imscription.lean (max on union prims, min on P,F)
def compute_tensor (a b : Imscription) : Imscription := tensorProduct a b

-- Probes
def monad_probe (s : Imscription) : Bool :=
  phi_c_gate s.crit && decide (s.pol = or' ∨ s.dim = if')

def topo_protection_probe (s : Imscription) : Bool :=
  decide (s.prot = awe ∨ s.dim = array)

-- Consciousness (gate-based, reuses gates from Consciousness.lean)
def consciousness_score_gate1 (s : Imscription) : Bool := phi_c_gate s.crit
def consciousness_score_gate2 (s : Imscription) : Bool := k_slow_gate s.kin
noncomputable def consciousness_score (s : Imscription) : ℝ := consciousnessScore s

end Imscribing.Primitives
