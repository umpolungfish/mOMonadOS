/-
  Imscribing.Primitives.Types.PrimitiveTypeIan

  Formalization of the primitive value type 'ian'
  Family: Relational (ordinal 3)
  Surface tokens: lateral exchange, peer protocol, symmetric link
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

/-- Surface tokens for 'ian' -/
def the_primitive_type_called_ian_surfaceTokens : List String := ["lateral exchange", "peer protocol", "symmetric link"]

/-- Boundary condition for 'ian' -/
def the_primitive_type_called_ian_boundaryCondition : String := "The Relational Mode interface for lateral peer-to-peer exchange."

/-- Belnap FOUR register descriptions -/
def the_primitive_type_called_ian_registerVoid : String := "uninitialized peer state"
def the_primitive_type_called_ian_registerTrue : String := "active symmetric synchronization"
def the_primitive_type_called_ian_registerFalse : String := "connection timeout or parity error"
def the_primitive_type_called_ian_registerBoth : String := "simultaneous bidirectional data flow"

/-- Opcode domain-semantic mapping -/
-- AFWD: outbound request
--   A directed transmission from one peer toward another.
-- AREV: inbound response
--   The return transmission that completes the lateral handshake.
-- CLINK: session binding
--   The sequential chaining of request and response into a stable connection.
-- ENGAGR: asynchronous buffer
--   A state where data is both sent and not yet acknowledged, held in transit.
-- EVALF: desynchronized state
--   The failure state where peer data views diverge.
-- EVALT: synchronized state
--   The affirmative state where both peers share a consistent data view.
-- FFUSE: symmetric link
--   The reconstitution of transmit and receive paths into a unified lateral relation.
-- FSPLIT: duplex channel
--   The bifurcation of a single connection into simultaneous transmit and receive paths.
-- IFIX: transaction log
--   The permanent append-only record of the lateral exchange.
-- IMSCRIB: peer identity
--   The self-recognition of a node as a valid participant in the ian mode.
-- TANCH: ian protocol boundary
--   The formal definition of the lateral relational mode that contains all peer exchanges.
-- VINIT: unlinked node
--   The state of a potential peer before any lateral connection is established.

-- Stage Imscriptions (per-node cumulative)
private def the_primitive_type_called_ian_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def the_primitive_type_called_ian_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }
private def the_primitive_type_called_ian_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := ah }

-- Label Imscriptions (per-node delta)
private def the_primitive_type_called_ian_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_primitive_type_called_ian_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_primitive_type_called_ian_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_primitive_type_called_ian_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- Main IGProtocol term
noncomputable def the_primitive_type_called_ian_protocol : IGProtocol the_primitive_type_called_ian_s0 the_primitive_type_called_ian_s11 :=
  .withGram Grammar.measure <|
  (.seq (.arrow the_primitive_type_called_ian_l0 the_primitive_type_called_ian_s0 the_primitive_type_called_ian_s1) (.seq (.arrow the_primitive_type_called_ian_l1 the_primitive_type_called_ian_s1 the_primitive_type_called_ian_s2) (.seq (.arrow the_primitive_type_called_ian_l2 the_primitive_type_called_ian_s2 the_primitive_type_called_ian_s3) (.seq (.prod (.arrow the_primitive_type_called_ian_l3 the_primitive_type_called_ian_s3 the_primitive_type_called_ian_s7) (.arrow the_primitive_type_called_ian_l3 the_primitive_type_called_ian_s3 the_primitive_type_called_ian_s7)) (.seq (.arrow the_primitive_type_called_ian_l7 the_primitive_type_called_ian_s7 the_primitive_type_called_ian_s7) (.seq (.arrow the_primitive_type_called_ian_l7 the_primitive_type_called_ian_s7 the_primitive_type_called_ian_s8) (.seq (.arrow the_primitive_type_called_ian_l8 the_primitive_type_called_ian_s8 the_primitive_type_called_ian_s9) (.seq (.arrow the_primitive_type_called_ian_l9 the_primitive_type_called_ian_s9 the_primitive_type_called_ian_s10) (.arrow the_primitive_type_called_ian_l10 the_primitive_type_called_ian_s10 the_primitive_type_called_ian_s11)))))))))

-- Evaluation arm sub-defs
noncomputable def the_primitive_type_called_ian_true_arm : IGProtocol the_primitive_type_called_ian_s0 the_primitive_type_called_ian_s11 :=
  (the_primitive_type_called_ian_protocol).restrictToEVALT
noncomputable def the_primitive_type_called_ian_false_arm : IGProtocol the_primitive_type_called_ian_s0 the_primitive_type_called_ian_s11 :=
  (the_primitive_type_called_ian_protocol).restrictToEVALF

-- Verification theorems
def the_primitive_type_called_ian_tier : OuroboricityTier := TierFunctor.obj the_primitive_type_called_ian_s0
theorem the_primitive_type_called_ian_frobenius :
    igFrobeniusAlg.mul the_primitive_type_called_ian_s0 the_primitive_type_called_ian_s0 = the_primitive_type_called_ian_s0 :=
  igFrobAlg_self_fusion the_primitive_type_called_ian_s0

#eval the_primitive_type_called_ian_tier  -- the Grammar's own verdict on its tier

/-- 'ian' belongs to the Relational family at ordinal 3 -/
def the_primitive_type_called_ian_family : String := "Relational"
def the_primitive_type_called_ian_ordinal : Nat := 3
def the_primitive_type_called_ian_bootstrapSteps : Nat := 12

end Imscribing.Primitives.Types
