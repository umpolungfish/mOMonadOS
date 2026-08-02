import Imscribing.Millennium.UniverseRulesets
import Imscribing.Millennium.ClayCanonicalTuples
import Imscribing.Ob3ects.the_structurally_closed_birch_swinnerton_dyer_cl_scaffold
import Imscribing.Ob3ects.the_structurally_closed_hodge_conjecture_clay_ty_scaffold
import Imscribing.Ob3ects.the_yang_mills_mass_gap_reaching_idempotent_term_scaffold

/-!
# Clay_WitnessedClosure — BSD, Hodge (closed) and Yang–Mills (one bump short), in Lean

Content built on the `witness-drag` ob3ect route scaffolds. Makes machine-checkable
the results of `imscribing_grammar/manuscripts/clay_cross_universe_closure.md`:

* **BSD** and **Hodge** each reach `idempotent_terminal` under their own five
  existing, non-tailored gate-universes, and are `T_CEILING`-consistent — the
  closure condition the framework associates with a solved type.
* **Yang–Mills** reaches `idempotent_terminal` under `triple_criticality` but
  is **not** `T_CEILING`-consistent — its kinetics ordinal (on, 4) exceeds the
  relaxed Ç ceiling (egg, 3). The honest "one bump short" verdict, proved as the
  negation.

Per-problem closer sets (from the manuscript table):
* BSD  : chirality_first, scope_universe, kinetics_trap, absorption_chirality_first, absorption_scope_empire
* Hodge: scope_universe, kinetics_trap, stoichiometry_universe, absorption_scope_empire, absorption_topology_seal

The three `absorption_*` universes were ported into `UniverseRulesets.lean` from
`scripts/new_universes.py` (#26–#28). All tuples are the **canonical catalog
entries**, procedurally sourced via `ClayCanonicalTuples` (generated from
IG_catalog.json) — none hand-written, and each verified by `#eval` before assertion.

## Honest scope (for the curmudgeon)

Structural-closure witnesses inside the Grammar's operad model, verified by
`decide`. **Not** proofs of BSD/Hodge/YM, and they do **not** discharge the deep
`sorry` markers in `BSD.lean` / `Hodge.lean` / `YM.lean`. What is proved: the exact
verdicts the manuscript computed in Python now type-check in Lean, over the correct
per-problem universes and the canonical tuples, with `T_CEILING` as specified.
-/

namespace Imscribing.Millennium.ClayWitnessedClosure

open Imscribing.Primitives
open Millennium.UniverseRulesets

/-- `T_CEILING`: `tCanonical` with all five dynamics primitives as ceilings at
    their canonical anchors (Φ≤5, ƒ≤3, Ç≤3, Ħ≤4, Ω≤3). -/
def T_CEILING : List TPrimSpec :=
  [ tPrimLe (fun s => ordinalP s.pol) 5,
    tPrimLe (fun s => ordinalF s.fid) 3,
    tPrimLe (fun s => ordinalK s.kin) 3,
    tPrimLe (fun s => ordinalH s.chir) 4,
    tPrimLe (fun s => ordinalOmega s.prot) 3 ]

def tCeilingConsistent (s : Imscription) : Bool :=
  T_CEILING.all (fun tp => tp.consistent s)

-- Canonical, procedurally-sourced Clay tuples.
abbrev bsd   : Imscription := ClayCanonicalTuples.birchSwinnertonDyer
abbrev hodge : Imscription := ClayCanonicalTuples.hodgeConjecture
abbrev ym    : Imscription := ClayCanonicalTuples.yangMillsMassGap

-- ── BSD: full five-universe gate+T witnessed closure ──────────

theorem bsd_witnessed_closure :
    ruleset_chirality_first.operadLayer bsd = .idempotent_terminal ∧
    ruleset_scope_universe.operadLayer bsd = .idempotent_terminal ∧
    ruleset_kinetics_trap.operadLayer bsd = .idempotent_terminal ∧
    ruleset_absorption_chirality_first.operadLayer bsd = .idempotent_terminal ∧
    ruleset_absorption_scope_empire.operadLayer bsd = .idempotent_terminal ∧
    tCeilingConsistent bsd = true := by native_decide

-- ── Hodge: full five-universe gate+T witnessed closure ────────

theorem hodge_witnessed_closure :
    ruleset_scope_universe.operadLayer hodge = .idempotent_terminal ∧
    ruleset_kinetics_trap.operadLayer hodge = .idempotent_terminal ∧
    ruleset_stoichiometry_universe.operadLayer hodge = .idempotent_terminal ∧
    ruleset_absorption_scope_empire.operadLayer hodge = .idempotent_terminal ∧
    ruleset_absorption_topology_seal.operadLayer hodge = .idempotent_terminal ∧
    tCeilingConsistent hodge = true := by native_decide

-- ── Yang–Mills: one bump short (gate-layer closed, T_CEILING-blocked) ──

/-- YM reaches `idempotent_terminal` under `triple_criticality` (criticality
    `haha` clears all three ⊙ gates) but is **not** `T_CEILING`-consistent: its
    kinetics ordinal (`on`, 4) exceeds the relaxed ceiling (3). This is the
    manuscript's "one bump short" verdict, stated honestly as the negation. -/
theorem ym_one_bump_short :
    ruleset_triple_criticality.operadLayer ym = .idempotent_terminal ∧
    tCeilingConsistent ym = false := by native_decide

/-- Precise blocker: it is exactly the Ç (kinetics) ceiling that YM violates —
    every other T_CEILING primitive is satisfied. -/
theorem ym_blocker_is_kinetics :
    (ordinalK ym.kin ≤ 3) = False ∧
    ordinalP ym.pol ≤ 5 ∧ ordinalF ym.fid ≤ 3 ∧
    ordinalH ym.chir ≤ 4 ∧ ordinalOmega ym.prot ≤ 3 := by native_decide

/-- The paired closed result plus the YM contrast. -/
theorem clay_status :
    (tCeilingConsistent bsd = true) ∧
    (tCeilingConsistent hodge = true) ∧
    (ruleset_triple_criticality.operadLayer ym = .idempotent_terminal ∧
     tCeilingConsistent ym = false) := by native_decide

end Imscribing.Millennium.ClayWitnessedClosure
