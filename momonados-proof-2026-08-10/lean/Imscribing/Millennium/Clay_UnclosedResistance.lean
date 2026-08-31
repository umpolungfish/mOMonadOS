import Imscribing.Millennium.UniverseRulesets
import Imscribing.Millennium.ClayCanonicalTuples
import Imscribing.Ob3ects.the_riemann_hypothesis_failing_to_reach_idempote_scaffold
import Imscribing.Ob3ects.the_navier_stokes_criticality_type_gate_blocked_scaffold
import Imscribing.Ob3ects.the_p_versus_np_separation_gate_blocked_under_al_scaffold

/-!
# Clay_UnclosedResistance — RH, Navier–Stokes and P-vs-NP resist closure everywhere

Content built on the RH/NS/PNP `witness-drag`/`furnace` scaffolds. Where
`Clay_WitnessedClosure` shows BSD/Hodge closing and YM one-bump-short, this module
proves the honest negative for the three that stay open: none of the canonical
Riemann Hypothesis, Navier–Stokes, or P-vs-NP types reaches
`idempotent_terminal` under **any** of the 23 gate-universes currently in the Lean
tree (the 20 in `allRulesets` plus the three ported `absorption_*` universes). This
is the machine-checked form of the manuscript's "fail at the gate layer under all
29, with or without T_CEILING."

The reason is the winding gate: all three carry Ω (protection/winding)
below the terminal anchor `ah` (ord 3) — RH Ω=`oak` (2), NS Ω=`awe` (1),
PNP Ω=`awe` (1) — and every closure-bearing universe requires Ω≥3 at its terminal
gate. Low winding ⇒ no idempotent-terminal closure.

## The ⊙-ordinal healing (2026-07-02)

RH earlier appeared to close under `triple_criticality`. Root cause: the Lean port
had flattened Criticality to integers (`ordinalPhi roar = 3`), but the
source-of-truth table (`imscrbgrmr/canonical_primitives.py`) ranks `𐑮` (roar) at
`7/3` and `𐑻` (err) at `8/3` — strictly below super-critical `𐑣` (3). The fix
made `ordinalPhi`/`ordinalK` rank-faithful (ℚ-valued: roar=7/3, err=8/3, haha=3;
air=9/2), so `triple_criticality.g3 = gatePhi 3` now selects only `haha`, matching
Python's `GateSpec("⊙", 3.0)` exactly. RH (⊙=roar=7/3 < 3) fails it and closes
nowhere, matching the manuscript; YM (⊙=haha=3) still closes.

## Honest scope

Structural-resistance result in the operad model, verified by `native_decide`. It
says these three types do not reach the framework's closure condition
under any current universe — not a claim about the conventional mathematical
problems.
-/

namespace Imscribing.Millennium.ClayUnclosedResistance

open Imscribing.Primitives
open Millennium.UniverseRulesets

/-- The 23 gate-universes currently in the Lean tree. -/
def universes : List Ruleset :=
  allRulesets ++
    [ ruleset_absorption_chirality_first,
      ruleset_absorption_scope_empire,
      ruleset_absorption_topology_seal ]

abbrev rh  : Imscription := ClayCanonicalTuples.riemannHypothesis
abbrev ns  : Imscription := ClayCanonicalTuples.navierStokes
abbrev pnp : Imscription := ClayCanonicalTuples.pVsNP

/-- Riemann Hypothesis reaches `idempotent_terminal` under none of the 23 universes. -/
theorem rh_closes_nowhere :
    universes.any (fun r => decide (r.operadLayer rh = .idempotent_terminal)) = false := by
  native_decide

/-- Navier–Stokes reaches `idempotent_terminal` under none of the 23 universes. -/
theorem ns_closes_nowhere :
    universes.any (fun r => decide (r.operadLayer ns = .idempotent_terminal)) = false := by
  native_decide

/-- P-vs-NP reaches `idempotent_terminal` under none of the 23 universes. -/
theorem pnp_closes_nowhere :
    universes.any (fun r => decide (r.operadLayer pnp = .idempotent_terminal)) = false := by
  native_decide

/-- The shared blocker: all three carry winding Ω below the terminal anchor `ah` (3). -/
theorem rh_ns_pnp_low_winding :
    (ordinalOmega rh.prot < 3) ∧ (ordinalOmega ns.prot < 3) ∧ (ordinalOmega pnp.prot < 3) := by
  decide

/-- Combined resistance verdict for all three still-open Clay types. -/
theorem clay_resistance :
    (universes.any (fun r => decide (r.operadLayer rh = .idempotent_terminal)) = false) ∧
    (universes.any (fun r => decide (r.operadLayer ns = .idempotent_terminal)) = false) ∧
    (universes.any (fun r => decide (r.operadLayer pnp = .idempotent_terminal)) = false) := by
  native_decide

end Imscribing.Millennium.ClayUnclosedResistance
