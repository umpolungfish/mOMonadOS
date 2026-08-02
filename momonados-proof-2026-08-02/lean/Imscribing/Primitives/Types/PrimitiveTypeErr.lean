/-
  Imscribing.Primitives.Types.PrimitiveTypeErr

  Formalization of the primitive value type 'err'
  Family: Criticality (ordinal 3)
  Surface tokens: exceptional point, eigenvector coalescence, branch point
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

/-- Surface tokens for 'err' -/
def the_primitive_type_called_err_surfaceTokens : List String := ["exceptional point", "eigenvector coalescence", "branch point"]

/-- Boundary condition for 'err' -/
def the_primitive_type_called_err_boundaryCondition : String := "non-Hermitian spectral manifold"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_err_registerVoid : String := "uninitialized parameter space"
def the_primitive_type_called_err_registerTrue : String := "convergent eigenvalue solution"
def the_primitive_type_called_err_registerFalse : String := "divergent eigenvalue solution"
def the_primitive_type_called_err_registerBoth : String := "coalesced state at the square-root branch point"

/-- Opcode domain-semantic mapping -/
-- AFWD: parameter tuning
--   The directed movement through the complex parameter space toward the singularity.
-- AREV: dissipative decay
--   The reverse flow of energy or information out of the system into the environment.
-- CLINK: operator composition
--   The sequential application of perturbations to the system matrix.
-- ENGAGR: exceptional point singularity
--   The state where T and F branches are indistinguishable and simultaneously occupied.
-- EVALF: unstable mode
--   The negative branch where the system exhibits runaway or divergent behavior.
-- EVALT: stable mode
--   The affirmative branch where the system maintains a bounded response.
-- FFUSE: eigenvector coalescence
--   The merging of distinct eigenvectors into a single degenerate state at the exceptional point.
-- FSPLIT: Riemann sheet bifurcation
--   The splitting of the eigenvalue trajectory into distinct topological branches.
-- IFIX: geometric phase record
--   The permanent Berry phase shift recorded after a complete circuit around the singularity.
-- IMSCRIB: self-consistent field
--   The identity of the operator where the output state maps back to the input basis.
-- TANCH: spectral closure
--   The boundary of the parameter space where the eigenvalue spectrum is fully defined.
-- VINIT: unperturbed Hamiltonian
--   The initial state before the introduction of non-Hermitian parameters.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_err_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_primitive_type_called_err_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_err_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_primitive_type_called_err_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_err_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_err_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_err_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_err_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_err_protocol : IGProtocol the_primitive_type_called_err_s0 the_primitive_type_called_err_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_err_l0 the_primitive_type_called_err_s0 the_primitive_type_called_err_s1) (.seq (.arrow the_primitive_type_called_err_l1 the_primitive_type_called_err_s1 the_primitive_type_called_err_s2) (.seq (.arrow the_primitive_type_called_err_l2 the_primitive_type_called_err_s2 the_primitive_type_called_err_s3) (.seq (.prod (.arrow the_primitive_type_called_err_l3 the_primitive_type_called_err_s3 the_primitive_type_called_err_s9) (.arrow the_primitive_type_called_err_l3 the_primitive_type_called_err_s3 the_primitive_type_called_err_s9)) (.seq (.arrow the_primitive_type_called_err_l9 the_primitive_type_called_err_s9 the_primitive_type_called_err_s9) (.seq (.arrow the_primitive_type_called_err_l9 the_primitive_type_called_err_s9 the_primitive_type_called_err_s10) (.arrow the_primitive_type_called_err_l10 the_primitive_type_called_err_s10 the_primitive_type_called_err_s11)))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_err_true_arm : IGProtocol the_primitive_type_called_err_s0 the_primitive_type_called_err_s11 :=
  (the_primitive_type_called_err_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_err_false_arm : IGProtocol the_primitive_type_called_err_s0 the_primitive_type_called_err_s11 :=
  (the_primitive_type_called_err_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_err_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_err_s0
theorem the_primitive_type_called_err_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_err_s0 the_primitive_type_called_err_s0 = the_primitive_type_called_err_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_err_s0

#eval the_primitive_type_called_err_tier  -- the Grammar's own verdict on its tier

/-- 'err' belongs to the Criticality family at ordinal 3 -/
def the_primitive_type_called_err_family : String := "Criticality"
def the_primitive_type_called_err_ordinal : Nat := 3
def the_primitive_type_called_err_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
