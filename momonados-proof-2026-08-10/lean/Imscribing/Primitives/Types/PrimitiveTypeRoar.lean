/-
  Imscribing.Primitives.Types.PrimitiveTypeRoar

  Formalization of the primitive value type 'roar'
  Family: Criticality (ordinal 2)
  Surface tokens: complex-axis criticality, analytic continuation, Lee-Yang edge
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

/-- Surface tokens for 'roar' -/
def the_primitive_type_called_roar_surfaceTokens : List String := ["complex-axis criticality", "analytic continuation", "Lee-Yang edge"]

/-- Boundary condition for 'roar' -/
def the_primitive_type_called_roar_boundaryCondition : String := "The complex plane of the renormalization group flow"

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_roar_registerVoid : String := "pre-critical vacuum without defined scaling"
def the_primitive_type_called_roar_registerTrue : String := "stable analytic continuation along the roar axis"
def the_primitive_type_called_roar_registerFalse : String := "instability or divergence at the Lee-Yang edge"
def the_primitive_type_called_roar_registerBoth : String := "non-Hermitian degeneracy at an exceptional point"

/-- Opcode domain-semantic mapping -/
-- AFWD: analytic continuation
--   The forward transformation from the real axis into the complex parameter space.
-- AREV: Hermitian projection
--   The descent from the complex plane back to the observable real-axis physics.
-- CLINK: RG step composition
--   The sequential chaining of renormalization group transformations.
-- ENGAGR: exceptional point coalescence
--   The paradice where two distinct eigenvectors become simultaneously identical and non-orthogonal.
-- EVALF: divergent singularity
--   The failure state where the continuation hits a non-removable pole.
-- EVALT: convergent series
--   The affirmative state where the analytic continuation remains well-defined.
-- FFUSE: monodromy integration
--   The reconstitution of the global partition function from its branched components.
-- FSPLIT: complex bifurcation
--   The branching of the partition function into distinct Riemann sheets.
-- IFIX: critical exponent record
--   The permanent fixation of the universal scaling dimensions in the ROM of the theory.
-- IMSCRIB: fixed point identity
--   The self-recognition of the system at the critical point where it maps to itself.
-- TANCH: Lee-Yang edge singularity
--   The terminal boundary where the analytic continuation of the partition function terminates.
-- VINIT: unperturbed Hamiltonian
--   The initial state before the introduction of complex parameters or coupling constants.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_roar_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_s1 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_roar_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_roar_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_roar_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }
private def the_primitive_type_called_roar_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_roar_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l1 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_roar_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_roar_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_roar_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_roar_protocol : IGProtocol the_primitive_type_called_roar_s0 the_primitive_type_called_roar_s10 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_roar_l0 the_primitive_type_called_roar_s0 the_primitive_type_called_roar_s1) (.seq (.arrow the_primitive_type_called_roar_l1 the_primitive_type_called_roar_s1 the_primitive_type_called_roar_s2) (.seq (.arrow the_primitive_type_called_roar_l2 the_primitive_type_called_roar_s2 the_primitive_type_called_roar_s3) (.seq (.prod (.arrow the_primitive_type_called_roar_l3 the_primitive_type_called_roar_s3 the_primitive_type_called_roar_s7) (.arrow the_primitive_type_called_roar_l3 the_primitive_type_called_roar_s3 the_primitive_type_called_roar_s7)) (.seq (.arrow the_primitive_type_called_roar_l7 the_primitive_type_called_roar_s7 the_primitive_type_called_roar_s7) (.seq (.arrow the_primitive_type_called_roar_l7 the_primitive_type_called_roar_s7 the_primitive_type_called_roar_s8) (.seq (.arrow the_primitive_type_called_roar_l8 the_primitive_type_called_roar_s8 the_primitive_type_called_roar_s9) (.arrow the_primitive_type_called_roar_l9 the_primitive_type_called_roar_s9 the_primitive_type_called_roar_s10))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_roar_true_arm : IGProtocol the_primitive_type_called_roar_s0 the_primitive_type_called_roar_s10 :=
  (the_primitive_type_called_roar_protocol).restrictToEVALT

-- Verification theorems
def the_primitive_type_called_roar_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_roar_s0
theorem the_primitive_type_called_roar_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_roar_s0 the_primitive_type_called_roar_s0 = the_primitive_type_called_roar_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_roar_s0

#eval the_primitive_type_called_roar_tier  -- the Grammar's own verdict on its tier

/-- 'roar' belongs to the Criticality family at ordinal 2 -/
def the_primitive_type_called_roar_family : String := "Criticality"
def the_primitive_type_called_roar_ordinal : Nat := 2
def the_primitive_type_called_roar_bootstrapSteps : Nat := 11

end Imscribing.Primitives.Types
