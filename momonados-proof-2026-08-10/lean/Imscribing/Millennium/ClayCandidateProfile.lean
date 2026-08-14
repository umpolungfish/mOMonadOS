import Imscribing.Millennium.UniverseRulesets
import Imscribing.Millennium.ClayCanonicalTuples
import Imscribing.Millennium.ClayCandidateTuples
import Imscribing.Millennium.Clay_UnclosedResistance
import Imscribing.Millennium.Clay_WitnessedClosure

/-!
# ClayCandidateProfile — exploratory #eval profile, not a theorem file.

Checks whether the candidate join tuples (rh joined with an exceptional-point
regime, and with Hodge) reach `idempotent_terminal` under any of the same 23
gate-universes that `Clay_UnclosedResistance` proves bare RH closes under none
of. Scratch/exploration only.
-/

namespace Imscribing.Millennium.ClayCandidateProfile

open Imscribing.Primitives
open Millennium.UniverseRulesets
open Imscribing.Millennium.ClayUnclosedResistance (universes)

def profile23 (s : Imscription) : List (String × OperadLayer) :=
  universes.map fun r => (r.name, r.operadLayer s)

def closingUniverses (s : Imscription) : List String :=
  (profile23 s).filterMap fun (n, l) => if l == .idempotent_terminal then some n else none

#eval "== rh (bare, baseline) =="
#eval closingUniverses ClayCanonicalTuples.riemannHypothesis
#eval "== rh_ep_join =="
#eval closingUniverses ClayCandidateTuples.rhEpJoin
#eval "== rh_hc_join =="
#eval closingUniverses ClayCandidateTuples.rhHcJoin
#eval "== meet_rh_hc =="
#eval closingUniverses ClayCandidateTuples.meetRhHc

#eval "== ordinalOmega (winding), rh vs rh_ep_join vs rh_hc_join vs meet_rh_hc =="
#eval ordinalOmega ClayCanonicalTuples.riemannHypothesis.prot
#eval ordinalOmega ClayCandidateTuples.rhEpJoin.prot
#eval ordinalOmega ClayCandidateTuples.rhHcJoin.prot
#eval ordinalOmega ClayCandidateTuples.meetRhHc.prot

#eval "== T_CEILING consistency (only meaningful if it closes somewhere first) =="
#eval Imscribing.Millennium.ClayWitnessedClosure.tCeilingConsistent ClayCandidateTuples.rhEpJoin
#eval Imscribing.Millennium.ClayWitnessedClosure.tCeilingConsistent ClayCandidateTuples.rhHcJoin

end Imscribing.Millennium.ClayCandidateProfile
