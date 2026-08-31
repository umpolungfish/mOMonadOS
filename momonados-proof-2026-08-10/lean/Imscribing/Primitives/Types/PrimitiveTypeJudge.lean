/-
  Imscribing.Primitives.Types.PrimitiveTypeJudge

  Formalization of the primitive value type 'judge'
  Family: Topology (ordinal 0)
  Surface tokens: heterogeneous graph, local connectivity, adjacency matrix
  Author: Lando⊗⊙perator
-/

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Primitives.TierCrossing
import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing.Primitives.Types
open Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

/-- Surface tokens for 'judge' -/
def the_primitive_type_called_judge_surfaceTokens : List String := ["heterogeneous graph", "local connectivity", "adjacency matrix"]

/-- Boundary condition for 'judge' -/
def the_primitive_type_called_judge_boundaryCondition : String := "The finite set of vertices and edges defining the local graph structure."

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_judge_registerVoid : String := "The uninitialized state where no vertices or edges are present."
def the_primitive_type_called_judge_registerTrue : String := "A state of confirmed local connectivity between vertices."
def the_primitive_type_called_judge_registerFalse : String := "A state of confirmed isolation or absence of an edge."
def the_primitive_type_called_judge_registerBoth : String := "A paradice where a vertex exists in a state of partial or ambiguous connectivity."

/-- Opcode domain-semantic mapping -/
-- AFWD: edge insertion
--   The forward transformation that adds a connection between two vertices.
-- AREV: edge deletion
--   The reverse transformation that removes a connection between two vertices.
-- CLINK: path composition
--   The chaining of local edges to form a path across the graph.
-- ENGAGR: superposition of adjacency
--   A state where two nodes are both connected and disconnected within a non-classical graph.
-- EVALF: disconnected state
--   The negative evaluation that no path exists between two nodes.
-- EVALT: connected state
--   The affirmative evaluation that a path exists between two nodes.
-- FFUSE: vertex contraction
--   The merging of two paths back into a single vertex, preserving the original connectivity.
-- FSPLIT: vertex bifurcation
--   The branching of a single vertex into two distinct adjacent paths.
-- IFIX: adjacency record
--   The permanent fixation of an edge in the graph's immutable history.
-- IMSCRIB: self-loop
--   An edge that connects a vertex to itself, representing identity.
-- TANCH: graph boundary
--   The terminal set of all nodes that constrains the judge topology.
-- VINIT: null graph
--   The state of zero vertices and zero edges before any connectivity is defined.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_judge_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_judge_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_judge_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_judge_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_judge_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_judge_protocol : IGProtocol the_primitive_type_called_judge_s0 the_primitive_type_called_judge_s9 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_judge_l0 the_primitive_type_called_judge_s0 the_primitive_type_called_judge_s1) (.seq (.arrow the_primitive_type_called_judge_l1 the_primitive_type_called_judge_s1 the_primitive_type_called_judge_s2) (.seq (.arrow the_primitive_type_called_judge_l2 the_primitive_type_called_judge_s2 the_primitive_type_called_judge_s3) (.seq (.arrow the_primitive_type_called_judge_l3 the_primitive_type_called_judge_s3 the_primitive_type_called_judge_s4) (.seq (.prod (.arrow the_primitive_type_called_judge_l4 the_primitive_type_called_judge_s4 the_primitive_type_called_judge_s7) (.arrow the_primitive_type_called_judge_l4 the_primitive_type_called_judge_s4 the_primitive_type_called_judge_s7)) (.seq (.arrow the_primitive_type_called_judge_l7 the_primitive_type_called_judge_s7 the_primitive_type_called_judge_s7) (.seq (.arrow the_primitive_type_called_judge_l7 the_primitive_type_called_judge_s7 the_primitive_type_called_judge_s8) (.arrow the_primitive_type_called_judge_l8 the_primitive_type_called_judge_s8 the_primitive_type_called_judge_s9))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_judge_true_arm : IGProtocol the_primitive_type_called_judge_s0 the_primitive_type_called_judge_s9 :=
  (the_primitive_type_called_judge_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_judge_false_arm : IGProtocol the_primitive_type_called_judge_s0 the_primitive_type_called_judge_s9 :=
  (the_primitive_type_called_judge_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_judge_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_judge_s0
theorem the_primitive_type_called_judge_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_judge_s0 the_primitive_type_called_judge_s0 = the_primitive_type_called_judge_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_judge_s0

#eval the_primitive_type_called_judge_tier  -- the Grammar's own verdict on its tier

/-- 'judge' belongs to the Topology family at ordinal 0 -/
def the_primitive_type_called_judge_family : String := "Topology"
def the_primitive_type_called_judge_ordinal : Nat := 0
def the_primitive_type_called_judge_bootstrapSteps : Nat := 10

end Imscribing.Primitives.Types
