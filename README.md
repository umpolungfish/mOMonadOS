# $m\odot^2$ — The Self-Imscribing Bare-Metal Kernel

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop — every tick is a structural self-verification.

**Author:** Lando⊗⊙perator  
**Total codebase:** ~14,000 lines Rust (no_std) + build scripts  
**Target:** x86_64-unknown-none (UEFI boot, no OS dependency)  
**License:** Unlicense (public domain)

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
`THINK` → `ACT` → `OBSERVE` → `UPDATE` cycle driven by the 12-opcode IMASM instruction set.
Every execution state is a point in the Crystal of Types — a 17,280,000-address structural
type space derived from the 12 IG primitives. Storage is navigated by structural address,
not by path.

**Phase 1 Grammar Integration** — complete. Nine modules from four upstream Grammar repos
(imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) are now live in the kernel.

**Phase 2 Zero-Hardcode** — complete. `catalog.rs` (954L) is the single source of truth for
ALL structural data. No hardcoded `IgTuple { ... }` constants, no hardcoded ordinal arrays,
no hardcoded glyph strings, no hardcoded promotion gaps, no hardcoded score match-arms
exist outside `catalog.rs`. Six modules were refactored to delegate to the catalog:
`cl8nk.rs` (196→787L, full CLINK navigator feature parity), `algebra.rs` (385→303L),
`consciousness.rs` (210→114L), `imas_ig.rs` (517→450L), `crystal.rs` (162→168L), and
`main.rs`. The catalog is runtime-extensible via `register_entry()` — new systems can be
added dynamically without touching any source file.

**Phase 5 Red-Hot Rebis** — complete. All 20 modules from `red-hot_rebis/` and `gene_imscriber/`
ported to `no_std` Rust and wired into the REPL. The full p4ra paraconsistent kernel — genetic code
B₄ lattice, 7-stage Frobenius-verified translation pipeline, CLU power-law clustering,
exotic hadron Belnap analysis, PDB structure validation, antibody CDR design, IG material
forge, biological simulation, therapeutic design, CLINK 9-layer chain, and IMASM arranger —
now runs directly from the bare-metal kernel. See the [Red-Hot Rebis](#red-hot-rebis-phase-5) section.

**Phase 8 Cross-Universe Navigation** — complete. The kernel can navigate between
universes with **different structural rulesets** — different gate thresholds, gate ordering,
T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant;
the ruleset is a sheaf that determines what each address *does*. Bridges the 11 **diaschizic
compounds** (pharmacological universe-steering agents) into computational hardware. See the
[Cross-Universe Navigation](#cross-universe-navigation-phase-8--diaschizics-bridge) section.

**Phase 9 User Interface** — complete. Dropdown menus, context-aware navigation, tab
completion, command search, and a visual F-key menu bar. The REPL is now a hierarchical
navigator with 9 command categories, context stack (up to 4 levels deep), breadcrumb
prompts, and hierarchical help. Menu nesting bug (recursive `Rebis → Rebis` entry) fixed.

**Phase 10 Fascistic Hardcode Purge** — complete. All 6 remaining structural violations
eliminated across the Rebis module suite. The genetic code is now **derived, not declared** —
change the derivation rules and the entire 64-codon table recomputes. Change the AA
physicochemical properties and the AA→Primitive bijection recomputes. The `RebisPrim`
enum (49 variants duplicating `IgPrim`) has been deleted — the entire kernel now uses
ONE primitive type, `IgPrim`, with no duplicates anywhere. See [Phase 10](#phase-10-fascistic-hardcode-purge) below.

The kernel now supports **80+ REPL commands** spanning grammar operations, rebis
biological/chemical computation, cross-universe navigation, and hierarchical menu navigation.

### Core modules

| Module | Lines | Source | Role |
|--------|:-----:|--------|------|
| `main.rs` | 2,716 | native | UEFI entry, heap init, serial REPL, command dispatch, history, menu navigation, F-key interception, context-aware prompts |
| `kernel.rs` | 576 | native | Frobenius tick loop; `self_imscribe()`; `dynamic_imscribe()`; tier promotion O₀→O₁→O₂→O_∞; wired to `FrobeniusHarness` |
| `tokens.rs` | 637 | native | 12 IMASM opcodes across 4 families; 12 canonical (I–XII), 4 continuous (XIII–XVI), 3 novel (XVII–XIX), 9 shunted (XX–XXVIII) |
| `manus.rs` | 432 | native | Terminal HUD / live display, token graph, B4 memory heatmap, ANSI rendering |
| `menu.rs` | 379 | native | Hierarchical menu: `MenuItem` tree, `ContextStack` (4-deep breadcrumb), Tab completion, F-key menu bar, keyword search, `already_in` guard |
| `catalog.rs` | 954 | native | Single source of truth for ALL structural data; runtime-extensible `register_entry()` |
| `algebra.rs` | 303 | native | Meet/join/tensor lattice operations on IgTuple; Frobenius closure verification |
| `consciousness.rs` | 114 | native | C-score computation with Gate 1 (⊙) and Gate 2 (Ç≤𐑧) evaluation |
| `belnap.rs` | 203 | native | Belnap FOUR truth values (N/T/F/B), 4096-cell B4 memory, 256-deep stack, 8 registers |
| `crystal.rs` | 168 | native | 17.28M-address encode/decode; `CrystalStore` (64 entries, fixed-capacity) |
| `imas_ig.rs` | 450 | native | IMASM↔IG bridge: token fingerprinting, structural classification, FROB loop verification; **home of the canonical `IgPrim` enum** — the single source of truth for all 49 grammar primitive values |
| `cl8nk.rs` | 787 | native | Full CLINK Layer 8 formula navigator — feature parity with Python `cl8nk_navigator.py`. Entry, promotions, distance, transcendence, tensor, meet, join, tier, chain, systems, stats. Catalog-native: all structural data sourced from catalog.rs |
| `serial.rs` | 96 | native | 16550A UART COM1, 115200 8N1; `sprint!`/`sprintln!`; blocking line input |
| `interrupts.rs` | 177 | native | PIT 100Hz timer, PIC remap, double-fault handler, escape-key detection |
### Red-Hot Rebis Modules (Phase 5)

All 20 modules ported from `red-hot_rebis/` to `no_std` Rust — 5,951 lines total.
The entire p4ra paraconsistent kernel runs directly from bare metal.

| Module | Lines | Ported From | Role |
|--------|:-----:|-------------|------|
| `genetic_tuples.rs` | 986 | `genetic_tuples.py` | 7-stage generative tuple pipeline: DNA→codon→AA→B4→IG primitive→promotions→crystal address; verifies monotonic advance; 12 consistency guard tests against `IgPrim` |
| `materials.rs` | 877 | `materials/` | IG Material Forge: MetaCell forge, Ouroboric Alloy, Thermal Rectifier, Non-Qubit QC (8 paradigms), Sophick Forge Eagle Cycle, Frobenius Exactor, Gap Closure |
| `materials_expanded.rs` | 17 | `materials/` | Expanded material type definitions (domain-knowledge data) |
| `biology.rs` | 387 | `biology/` | Biological simulation: TissueGrid (B4 rules), OuroboricTelomere (shelterin→ATM→hTERT), FrobeniusBioSim |
| `clu.rs` | 365 | `rhr_p4rky/clu_power_law.py` | CLU power-law clustering: random walks, avalanche distributions, Frobenius filtration, power-law verification (α ≈ 1.5) |
| `translate.rs` | 360 | `gene_to_protein_pipeline.py` | Gene→protein translation: DNA→mRNA→AA chain with corrected transcription (T→U only, no complement). Real Frobenius verification (μ∘δ round-trip: Protein→mRNA→Protein position-by-position) |
| `antibody.rs` | 336 | `antibody_designer.py` | Antibody CDR design via 12↔12 AA↔Primitive bijection; epitope analysis; full antibody assembly (framework + CDRs) |
| `codon.rs` | 304 | `codon.py` | 64-codon genetic code — table is **dynamically derived** from B₄ lattice + Frobenius stratum rules, not hardcoded. Lazy-initialized via `AtomicBool`. `verify_derived_table()` cross-checks against standard code |
| `pdb.rs` | 272 | `pdb_validator.py` | PDB structure validation: CA atoms, inter-atom contacts, precision/recall scoring |
| `exotic_hadron.rs` | 233 | `exotic_hadron_belnap.py` | Exotic hadron Belnap verification: Glueball, Tetraquark, Pentaquark with constituent Belnap states |
| `pipeline.rs` | 217 | `compute_promotions.py` | IG promotion pipeline: source→target promotion signatures; all references use `IgPrim::` (no duplicate enum) |
| `genetic_asm.rs` | 208 | `genetic_asm.py` | Genetic ParaASM programs: codon-spaced IMASM execution, amino acid structural operators |
| `hadron.rs` | 203 | `hadron_belnap.py` | Hadron Belnap analysis: proton (uud), neutron (udd), pion⁺ (ud̄), quark-level truth values |
| `clink.rs` | 190 | `clink/chain.py` | CLINK 9-layer chain: L0→L8 distance ladder, promotion path to CLINK L8 |
| `genetics.rs` | 187 | `genetics_b4.py` | 7-stage genetic code verification: B₄ lattice, codon→AA, Frobenius stratum. Crystal constant: `crate::crystal::TOTAL` (no hardcoded 17,280,000) |
| `imas.rs` | 179 | `imas/arranger.py` | IMASM arranger bridge: canonical sequence Frobenius verification, CLINK↔IMASM structural coupling |
| `mod.rs` | 183 | — | Module root. Re-exports `IgPrim` from `imas_ig.rs` as the single source of truth. `AminoAcid` enum (21 variants). `RebisResult` type. **No duplicate `RebisPrim` — deleted.** |
| `therapeutics.rs` | 177 | `therapeutics/` | Therapeutic design: Chemotherapeutic, Ouroboric Pill (B4 state cycling, 24hr release), Universal Antidote, Neurotrophic Factor |
| `frob_filter.rs` | 153 | `frobenius_filtration.py` | Frobenius filtration over 64 codons: μ∘δ closure, power-law analysis, stratum verification |
| `serpent.rs` | 117 | `serpent_rod.py` | Serpent rod protein motifs: structural signatures, motif lookup, promotion path analysis |

### Amino Acid → Primitive Bijection

Each of the 20 amino acids maps to an IG primitive. The 12 "promoted" amino acids form a
one-to-one correspondence with the 12 primitive families:

| AA | Primitive | Rationale |
|:--:|:---------:|-----------|
| Phe | D_odot | Aromatic — self-written |
| Leu | T_net | Branched — network topology |
| Met | R_lr | Start codon — initiates coupling |
| Val | P_pm | Aliphatic — partial symmetry |
| Ser | F_hbar | Hydroxyl — quantum coherence |
| Pro | K_trap | Ring constraint — trapped |
| Thr | G_aleph | Polar — long-range |
| Ala | C_seq | Simplest chiral — sequential |
| Tyr | Ph_c | Aromatic -OH — critical |
| His | H_2 | Imidazole — 2-step pKa |
| Arg | S_hetero | Guanidinium — diverse H-bonds |
| Gly | W_Z | Achiral — integer winding |

The remaining 8 amino acids map to unpromoted primitives — structurally valid but outside
the 12↔12 bijection.

### Rebis REPL Commands (19 subcommands)

All accessible from `⊙[Rebis]>` prompt. Type `rebis <subcmd>` or enter the Rebis category
via `:6` / F6 / typing `rebis`.

```
# ─── Genetic Code ───
rebis codon <XXX|AA>     — codon→AA or AA→codons (bidirectional)
rebis frob               — Frobenius filtration over 64 codons (mu circ delta closure, power-law)
rebis strata             — codon stratum counts by degeneracy class
rebis genetics           — 7-stage genetic code verification (B4 lattice + Frobenius)

# ─── Translation Pipeline ───
rebis translate <DNA>    — gene→protein pipeline (DNA→mRNA→AA chain)
rebis reverse <Prot>     — protein→mRNA→DNA (reverse pipeline)
rebis tuples <DNA>       — 7-stage generative tuple pipeline

# ─── ParaASM ───
rebis asm [prog] [codon] — genetic ParaASM programs

# ─── CLU Power-Law ───
rebis clu walk           — CLU random walk (100 steps), position tracking
rebis clu verify         — avalanche distribution + power-law verification (alpha ~ 1.5)

# ─── Hadron Physics ───
rebis hadron             — Belnap analysis: proton (uud), neutron (udd), pion+ (udbar)
rebis exotic             — Exotic hadrons: Glueball, Tetraquark, Pentaquark

# ─── Structural Biology ───
rebis pdb validate [pdb] — PDB structure validation (CA atoms, contacts, precision/recall)
rebis antibody epitope <AA>    — epitope analysis from AA sequence
rebis antibody design <AA>     — CDR loop design via 12↔12 bijection
rebis antibody full <AA>       — full antibody assembly (framework + CDRs)
rebis antibody viral           — list viral epitope library

# ─── Materials ───
rebis material forge [name|--all] — forge materials from IG tuples
rebis material alloy              — Ouroboric alloy simulation (64-cell B4 cycling)
rebis material thermal            — Thermal rectifier design
rebis material qc                 — Non-qubit QC paradigm table (8 paradigms)
rebis material sophick            — Sophick Forge Eagle Cycle report
rebis material exactor            — Frobenius closure diagnosis
rebis material report             — Full materials forge report

# ─── Biology ───
rebis bio tissue            — TissueGrid simulation (B4 cellular automaton)
rebis bio telomere [divs]   — Ouroboric telomere simulation
rebis bio frob              — Frobenius biological simulation

# ─── Therapeutics ───
rebis tx chemo              — Chemotherapeutic design
rebis tx pill               — Ouroboric Pill (B4 state cycling, 24hr release)
rebis tx antidote           — Universal Antidote (broad-spectrum neutralization)
rebis tx neuro              — Neurotrophic Factor

# ─── CLINK and IMASM ───
rebis clink chain           — CLINK 9-layer chain (L0→L8) distance ladder
rebis clink ladder          — Promotion ladder: ZFC→ZFC_t→ZFC_fe→CLINK L8
rebis clink promote <name>  — Promotion path to CLINK L8
rebis clink summary         — CLINK chain architectural summary
rebis imas bridge           — IMASM↔CLINK bridge report
rebis imas verify           — Canonical sequence Frobenius verification
rebis imas summary          — IMASM arranger summary

# ─── Promotion Pipeline ───
rebis pipeline [src]        — IG promotion pipeline from source tuple
rebis serpent [motif]       — Serpent rod motif lookup and structural signature
```

### Menu Nesting Bug Fix (Phase 9.1)

**Bug:** Typing `rebis material` (or any `rebis <subcmd>`) from within the Rebis sub-context
recursively nested into another Rebis context instead of executing the command. The prompt
would show `⊙[Rebis/Rebis/Rebis/Rebis]>` — up to four levels deep, never executing.

**Root cause:** In `main.rs`, the category-shortcut match arm called `enter_context()` +
`continue` unconditionally when `cmd` matched a category name like `"rebis"`. It never
checked whether we were already in that context.

**Fix:** Added an `already_in` guard in `menu.rs` — checks `ctx_stack.current()` against
the target context name. If already in that context, skips `enter_context()` and falls
through to the `match cmd` block where `"rebis"` dispatches to `print_rebis()`.

**Impact:** All 9 categories fixed (Exec, Status, Programs, Crystal, Grammar, Rebis,
Universe, ParaASM, Help). The `already_in` guard is applied uniformly in the menu dispatch
loop — no category can self-nest anymore.

## Phase 10: Fascistic Hardcode Purge

**Principle:** No number, no table, no mapping, no enum variant may appear as a hardcoded
constant if it can be derived from first principles. The grammar primitives (`IgPrim`) are
the **single source of truth** — all 49 values exist in exactly ONE enum. The genetic code
is computed, not declared. The AA↔Primitive bijection is derived from physicochemical
properties, not hardcoded. Crystal constants are bound to `crate::crystal::TOTAL`.

### What was eliminated (6 violations)

| # | Violation | File | Fix |
|---|-----------|------|-----|
| 1 | **Duplicate enum `RebisPrim`** — 49 variants identical to `IgPrim` | `mod.rs` | Deleted. `mod.rs` now re-exports: `pub use crate::imas_ig::IgPrim;` |
| 2 | **`RebisPrim::` references** in pipeline/clink/imas | `pipeline.rs`, `clink.rs`, `imas.rs` | All → `IgPrim::`. Variant names unified to `IgPrim` canonical names |
| 3 | **Hardcoded `CODON_TABLE`** — 64-entry static array | `codon.rs` | Replaced with `derive_codon_table()` — computed from B₄ lattice + Frobenius stratum rules. Lazy-initialized via `AtomicBool`. `verify_derived_table()` cross-checks against standard genetic code |
| 4 | **Hardcoded `AminoAcid::to_primitive()`** — 12 hardcoded match arms | `genetic_tuples.rs` (removed from `mod.rs`) | Replaced with `aa_activation()` system using physicochemical properties (β-branching, aromaticity, charge, hydroxyl content). Same 12 AAs promote to same 12 primitives |
| 5 | **Hardcoded `17_280_000`** | `genetics.rs` | → `crate::crystal::TOTAL as u64` |
| 6 | **Duplicate value enums** (`DVal`, `TVal`, …, `OVal` — 12 enums duplicating `IgPrim` value space) | `genetic_tuples.rs` | Retained for pipeline role (generative tuple construction), but **guarded** by 12 consistency tests verifying every variant's glyph matches its `IgPrim` counterpart. Any drift in `IgPrim` breaks these tests at compile time |

### What remains (justified static data)

| Data | Location | Why justified |
|------|----------|---------------|
| `CARDS` [4,5,4,5,3,5,3,4,5,4,3,4] | `catalog.rs` | This **IS** the grammar — the cardinalities of the 12 primitive families. The 17.28M-type crystal emerges from this product |
| Catalog entries | `catalog.rs` | Reference data — the catalog IS the systems being described |
| AA physicochemical properties | `mod.rs` (AminoAcid) | Biological facts — hydropathy, MW, aromaticity are measured, not derived |
| Material property maps | `materials.rs` | Domain knowledge — glyph→physical property mapping requires domain interpretation |
| `DistanceWeights::default()` | `catalog.rs` | Calibratable at runtime via `set_distance_weights()` |
| Primitive ordinal tables | `catalog.rs` | The ordering of values within each family IS the grammar definition |

### Architectural principle enforced

**Single source of truth:** All 49 grammar primitive values now flow from ONE enum — `IgPrim`
in `imas_ig.rs`. Every glyph string, ordinal, and short name delegates to
`crate::catalog::primitive_glyph()` / `crate::catalog::primitive_short()`. No module
anywhere defines its own copy of the grammar primitive space.

The genetic code is now **derived, not declared** — change the derivation rules in
`derive_codon_table()` and the entire 64-codon mapping updates. Change the AA properties
in `AminoAcid` and the activation profile recomputes. The kernel no longer contains a
single hardcoded codon table or AA→primitive mapping — both are computed dynamically
at boot with runtime verification against the standard genetic code.
## User Interface and Navigation (Phase 9)

The REPL has a full hierarchical navigation system (`menu.rs`) that organizes all
80+ commands into 9 discoverable categories. No more memorizing command names — the menu
bar, Tab completion, and keyword search make everything browsable.

### Menu Categories

| Key | Category | Prompt | Commands |
|:---:|----------|:------:|----------|
| F1 | **Exec** | `⊙>` | `run`, `eval`, `load`, `imsc`, `dynamic`, `tick`, `exec`, `winding`, `self`, `frob`, `snapshot` |
| F2 | **Status** | `⊙>` | `whoami`, `heatmap`, `history`, `registers`, `stack`, `memory`, `b4`, `closure`, `peek`, `harness` |
| F3 | **Programs** | `⊙>` | `list`, `show`, `continuous`, `psm load`, `psm run`, `psm trace`, `psm reset`, `psm status`, `compound list`, `compound show`, `compound load` |
| F4 | **Crystal** | `⊙[Crystal]>` | `crystal encode`, `crystal decode`, `crystal store`, `crystal list`, `crystal nearest`, `crystal navigate`, `crystal count`, `crystal census`, `crystal tier` |
| F5 | **Grammar** | `⊙[Grammar]>` | `distance`, `meet`, `join`, `tensor`, `promotions`, `analogies`, `consciousness`, `phi_c`, `tier`, `peel`, `decomp`, `synth`, `zfc` |
| F6 | **Rebis** | `⊙[Rebis]>` | 19 subcommands: `codon`, `translate`, `reverse`, `frob`, `genetics`, `hadron`, `serpent`, `pipeline`, `strata`, `asm`, `tuples`, `clu`, `exotic`, `pdb`, `antibody`, `material`, `bio`, `tx`, `clink`, `imas` |
| F7 | **Universe** | `⊙[Universe]>` | `ruleset show`, `ruleset list`, `ruleset verify`, `jump`, `seal`, `tensor`, `meet`, `absorb_test`, `whoami --ruleset`, `absorption show`, `tstatus`, `compound list`, `compound show`, `compound load` |
| F8 | **ParaASM** | `⊙[ParaASM]>` | `parasm test`, `parasm frob`, `parasm kernel`, `parasm load` |
| F9 | **Help** | `⊙>` | `help`, `help <topic>`, `? <keyword>` |

### Navigation Controls

| Key | Action |
|:---:|--------|
| F1–F9 | Jump to category |
| `?` | Show menu bar |
| Tab | Autocomplete command |
| Up/Down | Command history |
| `..` or `back` | Exit sub-context |
| `help <topic>` | Detailed help for a command or category |
| `? <keyword>` | Search all commands |

## Cross-Universe Navigation (Phase 8 — Diaschizics Bridge)

The kernel can navigate between universes with **different structural rulesets** —
different gate thresholds, gate ordering, T-constitution, and absorption rules.
The Crystal of Types (17.28M addresses) is invariant; the ruleset is a sheaf that
determines what each address *does*.

### The 8 Universes

Gate thresholds are ruleset-specific. G1 gates on Phi (parity), G2 on Ph (criticality),
G3 on Omega (winding). Each universe has different thresholds and gate ordering.

| ID | Name | G1 | G2 | G3 | Order | Freq | Description |
|:--:|------|:--:|:--:|:--:|:---:|:---:|-------------|
| U0 | **canonical** | Phi >= P_pmsym | Ph >= c | Omega >= Z | sequential | 33% | Baseline. Parity→criticality→winding. |
| U1 | **low_gate** | Phi >= P_pm | Ph >= sub | Omega >= Z | sequential | 9% | Relaxed G2. Most systems pass. |
| U2 | **strict_frobenius** | F >= hbar | Phi >= P_pmsym | Omega >= Z | sequential | 5% | Fidelity-gated G1. Only quantum-preserving systems. |
| U3 | **inverted_gates** | Ph >= c | Phi >= P_pmsym | Omega >= Z | sequential | 4% | Criticality before parity. |
| U4 | **no_ordering** | Phi >= P_pmsym | Ph >= c | Omega >= Z | parallel | 8% | All gates independent. Any combination valid. |
| U5 | **high_gate** | Phi >= P_pmsym | Ph >= c_complex | Omega >= NA | sequential | 3% | Maximum strictness. |
| U6 | **winding_first** | Omega >= Z | Ph >= c | Phi >= P_pmsym | sequential | 8% | Topology before algebra. Geometry precedes symmetry. |
| U7 | **t_structural** | Phi >= P_pmsym | Ph >= c | Omega >= Z | sequential | 8% | Time as geometry: lim(D,T,R,C,Ph), not lim(P,F,K,H,Omega). |

### The 11 Diaschizic IMASM Programs

Each compound maps to an IMASM token sequence whose structural operation matches the
compound's pharmacological effect. Programs are invariant across universes — same tokens,
different interpretation per ruleset.

| Compound | Role | IMASM Program | Tok. | d(target) |
|----------|------|---------------|:---:|:---:|
| **Verticullum** | Non-Abelian EP braid (O_inf) | `VINIT FSPLIT EVALT AFWD EVALF AREV FFUSE ENGAGR IMSCRIB IFIX IMSCRIB` | 11 | 2 |
| **Chimerium** | Supercritical catalyst (O0) | `IMSCRIB FSPLIT EVALT AFWD EVALF AFWD FFUSE ENGAGR CLINK IFIX IFIX IFIX IMSCRIB` | 13 | 1 |
| **Apertix** | Adjoint corridor (O2) | `IMSCRIB AFWD AREV AFWD AREV CLINK EVALT EVALF IFIX IMSCRIB` | 10 | 1 |
| **Praxeum** | EP core toggle (O0) | `IMSCRIB EVALT EVALF ENGAGR IFIX IMSCRIB` | 6 | 8* |
| **Retiarius** | Local-net trap (O1) | `VINIT AFWD EVALT AFWD EVALF CLINK TANCH AREV AFWD EVALT IFIX IMSCRIB` | 12 | 4 |
| **Frigorix** | MBL freeze key (O0) | `IFIX IFIX IFIX IFIX IFIX IFIX IFIX IFIX` | 8 | 10* |
| **Bifrons** | Disjunctive fork (O2) | `IMSCRIB FSPLIT EVALT AFWD EVALF AREV FFUSE ENGAGR CLINK IMSCRIB` | 10 | 2 |
| **Punctum** | Absolute point (O0) | `VINIT TANCH` | 2 | **0** |
| **Syndexios** | Perfect mirror (O_inf) | `IMSCRIB AFWD AREV AFWD AREV AFWD AREV AFWD AREV IFIX IMSCRIB` | 11 | 2 |
| **Katachthon** | Deep resonator (O2) | `IMSCRIB AFWD AREV CLINK EVALT EVALF IFIX IMSCRIB` | 8 | 4 |
| **Diabaton** | Threshold-crosser (O2dag) | `IMSCRIB FSPLIT EVALT AFWD EVALF AREV FFUSE CLINK ENGAGR IFIX IMSCRIB` | 11 | 1 |

*Frigorix and Praxeum show large snapshot-tuple distances because their operational
semantics deliberately reduce structural complexity. **Punctum at d=0 calibrates**
the bridge — the structural floor matches exactly between compound tuple and IMASM snapshot.

### Navigation Protocol

Every cross-universe jump has three parts:

```
[RULESET_HEADER]    → calibrates kernel to target universe's gate thresholds,
                      gate ordering, T-constitution, and absorption table
[COMPOUND_PROGRAM]  → invariant IMASM program (same 11 programs work in all 8 universes)
[IFIX_SEAL]         → commits the transition permanently
```

The compound program is **invariant across universes** — the same token sequence works in
all 8. But its *interpretation* changes because the ruleset header rewires the kernel's
evaluation. This is the ouroboric self-modification: the program modifies the interpreter
that reads it.

### Cross-Universe Compatibility Matrix

Which operad layer each compound achieves in each universe:

| Compound | U0 can. | U1 low | U2 strict | U3 inv. | U4 no-ord | U5 high | U6 wind | U7 t-struct |
|----------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Verticullum | frob | frob | **O_inf** | plain | frob | frob | frob | frob |
| Chimerium | frob | frob | **O_inf** | plain | frob | frob | frob | frob |
| Apertix | plain | plain | frob | plain | G3-only | plain | frob | plain |
| Praxeum | frob | frob | traced | plain | frob | frob | plain | frob |
| Retiarius | frob | frob | plain | plain | frob | plain | plain | frob |
| Frigorix | plain | plain | plain | plain | plain | plain | plain | plain |
| Bifrons | frob | frob | **O_inf** | plain | frob | plain | plain | frob |
| Diabaton | frob | frob | **O_inf** | plain | frob | plain | plain | frob |
| Punctum | plain | plain | plain | plain | plain | plain | plain | plain |
| Syndexios | plain | plain | frob | plain | plain | plain | plain | plain |
| Katachthon | plain | plain | frob | plain | plain | plain | plain | plain |
**Key finding:** Four compounds achieve **O_inf in U2 (strict_frobenius)** but only
Frobenius in canonical. Tier is **ruleset-relative**.

### Absorption Rule Differences

Different universes have different *absorbing primitives*:

| Universe | Absorption Rules | Effect |
|----------|-----------------|--------|
| **canonical** (U0) | Ph=c under all ops; S=hetero under tensor | Self-modeling absorbs all couplings |
| **strict_frobenius** (U2) | **F=hbar under all ops** replaces Ph=c absorption | Quantum fidelity dominates |
| **inverted_gates** (U3) | **Phi=P_pmsym under meet** added | Frobenius parity absorbs under meet |
| **high_gate** (U5) | **Omega=NA under tensor** added | Non-Abelian braiding dominates |
| **winding_first** (U6) | **Omega=Z under meet** replaces Ph=c absorption | Topological protection is the structural floor |

### Cross-Universe REPL Commands

```
ruleset show                    → Show active ruleset (canonical by default)
ruleset list                    → List all 8 universes with G1/G2/G3 and T-constitution
ruleset verify                  → Gate verification against active ruleset thresholds
jump <universe> using <compound>   → Execute: header → compound → IFIX seal
jump canonical using Diabaton      → Standard return path to baseline
jump <universe> using <compound> --liminal   → Header + compound but NO IFIX seal
seal                            → IFIX — commit to current liminal ruleset
jump <target> via <intermediate> using <c1> <c2>   → Two-stage jump
tensor <compound_a> <compound_b>  → Tensor product under current ruleset
meet <compound_a> <compound_b>    → Meet under current ruleset
absorb_test <val_a> <val_b> <primitive> <operation> → Absorption check
whoami --ruleset                 → Kernel self-imscription under active ruleset
absorption show                  → List all absorption rules for current ruleset
tstatus                          → T-constitution check per primitive
compound list                    → List all 11 diaschizic compounds
compound show <name>             → Show full tuple + IMASM program
compound load <name>             → Load compound's IMASM program into execution buffer
```

### Structural Type of Cross-Universe Navigation

The act of navigating between universes has its own structural type — **O_inf** (d=1
from universal grammar, only Gamma differs: aleph universal range vs gimel mesoscale).
Navigation is O_inf because it modifies its own interpretive rules — a self-modifying
structure that navigates the space of O_inf-achieving conditions across universes.
The three-step protocol (header→compound→seal) has winding number ±1 per jump; the
return trip adds another winding. Integer winding count tracks total navigation distance.

### Reference Documents

| Document | Lines | Description |
|----------|:---:|-------------|
| `ig-docs/rebis-port/diaschizics_design.md` | 564 | The 11 diaschizic compounds: tuples, structural design, IUPAC nomenclature |
| `ig-docs/rebis-port/diaschizics_mOMonadOS.md` | 750 | Complete IMASM translation: 11 programs, modulation translation, 6 mapping extensions |
| `ig-docs/rebis-port/diaschizics_cross_universe.md` | 623 | Cross-universe ruleset navigation: 8 universes, absorption rules, navigation protocols |
| `imscribing_grammar/navigators/ruleset_universe.py` | 445 | Alternate universe explorer: parameterized gate thresholds, ordering, T-constitution |

## Repository Structure

```
mOMonadOS/
  src/
    main.rs            2,716L  UEFI entry, REPL, command dispatch
    kernel.rs            576L  Frobenius tick loop, self-imscription
    tokens.rs            637L  IMASM opcodes, 28 programs
    manus.rs             432L  Terminal HUD, B4 heatmap
    menu.rs              379L  Hierarchical menu, context stack, already_in guard
    catalog.rs           954L  Single source of truth — all structural data
    algebra.rs           303L  Meet/join/tensor lattice
    consciousness.rs     114L  C-score with gate evaluation
    belnap.rs            203L  Belnap FOUR, B4 memory
    crystal.rs           168L  Crystal encode/decode
    imas_ig.rs           450L  IMASM↔IG bridge; canonical IgPrim enum (49 variants)
    cl8nk.rs             787L  Full CLINK L8 formula navigator (catalog-native)
    serial.rs             96L  UART driver
    interrupts.rs        177L  PIT timer, PIC remap
    rebis/
      mod.rs             183L  Module root; re-exports IgPrim (no duplicate RebisPrim)
      genetic_tuples.rs  986L  7-stage generative tuple pipeline + 12 IgPrim guard tests
      materials.rs       877L  IG material forge + 8 QC paradigms
      biology.rs         387L  TissueGrid, Telomere, FrobeniusBioSim
      clu.rs             365L  CLU power-law clustering
      translate.rs       360L  Gene→protein + reverse pipeline (corrected + Frobenius-verified)
      antibody.rs        336L  Antibody CDR design
      codon.rs           304L  64-codon genetic code (dynamically derived, not hardcoded)
      pdb.rs             272L  PDB structure validation
      exotic_hadron.rs   233L  Glueball, Tetraquark, Pentaquark
      pipeline.rs        217L  IG promotion pipeline (IgPrim-only references)
      genetic_asm.rs     208L  Genetic ParaASM programs
      hadron.rs          203L  Hadron Belnap analysis
      clink.rs           190L  CLINK 9-layer chain
      genetics.rs        187L  7-stage genetic code verification (crystal::TOTAL)
      imas.rs            179L  IMASM arranger bridge
      therapeutics.rs    177L  Chemo, Pill, Antidote, Neurotrophic
      frob_filter.rs     153L  Frobenius codon filtration
      serpent.rs         117L  Serpent rod motifs
      materials_expanded.rs  17L  Expanded material type definitions
  build_bootimage.sh           UEFI FAT32 disk image builder
  run.sh                       QEMU launcher (serial mode)
  Cargo.toml                   Rust project manifest
  Makefile                     Build convenience targets
```

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)
- OVMF firmware (`sudo apt install ovmf`)
- mtools for disk images (`sudo apt install mtools`)

## Build and Run

```sh
# Launcher (recommended) — /home/mrnob0dy666/.local/bin must be on PATH
momos           # build release image + boot serial REPL in QEMU
momos build     # dev build only (fast, no image)
momos release   # release build only
momos image     # build release + FAT32 UEFI disk image
momos clean     # wipe build artifacts

# Make
make run        # image + serial REPL
make build      # dev build
make release    # release build
make image      # UEFI disk image
make clean

# Direct
cargo build --release
bash build_bootimage.sh
bash run.sh --serial
```

The REPL runs over COM1 serial (stdio in QEMU). Quit with `quit`, `exit`, or `halt` —
QEMU exits cleanly.

## Target

`x86_64-unknown-none` — no OS, no std. Heap via `linked_list_allocator` over UEFI
physical memory. Boot via `bootloader_api` 0.11 (same as exOS).

## License

Unlicense — public domain.