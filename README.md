# $m\odot^2$, The Self-Imscribing Bare-Metal Kernel

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop, every tick is a structural self-verification.

**Author:** Lando⊗⊙perator  
**Total codebase:** ~24,500 lines Rust (no_std) + build scripts  
**Target:** x86_64-unknown-none (bare-metal direct ELF boot, zero external crates)  
**License:** Unlicense (public domain)

## Overview

**What it is.** $m\odot^2$: a bare-metal, self-imscribing operating kernel in Rust (no_std, x86_64) with no processes, scheduler, or filesystem hierarchy. The kernel is the Frobenius loop. (Distinct from the Python `omonad_OS`.)

**What it does.** Boots directly on hardware/QEMU and runs a perpetual THINK→ACT→OBSERVE→UPDATE cycle over the 12-opcode IMASM set, where every execution state is an address in the 17,280,000-type Crystal and storage is navigated by structural address, not path.

**Why it matters.** Every tick is a structural self-verification (μ∘δ=id): composition is free (any token, any order, any length) and correctness is enforced by the grammar rather than by a kernel API, with zero external crates.

**How to use it.** Build the no_std ELF and boot under QEMU (see below).

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
`THINK` → `ACT` → `OBSERVE` → `UPDATE` cycle driven by the 12-opcode IMASM instruction set.
Each tick executes a single IMASM token, composition is free: any token at any time,
any sequence of any length, no preset opcode sequences. The harness drives token selection;
the grammar constrains what each token does to the structural state.
Every execution state is a point in the Crystal of Types, a 17,280,000-address structural
type space derived from the 12 IG primitives. Storage is navigated by structural address,
not by path.

**Phase 1 Grammar Integration**, complete. Nine modules from four upstream Grammar repos
(imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) are now live in the kernel.

**Phase 2 Zero-Hardcode**, complete. `catalog.rs` (954L) is the single source of truth for
ALL structural data. No hardcoded `IgTuple { ... }` constants, no hardcoded ordinal arrays,
no hardcoded glyph strings, no hardcoded promotion gaps, no hardcoded score match-arms
exist outside `catalog.rs`. Six modules were refactored to delegate to the catalog:
`cl8nk.rs` (196→787L, full CLINK navigator feature parity), `algebra.rs` (385→303L),
`consciousness.rs` (210→114L), `imas_ig.rs` (517→450L), `crystal.rs` (162→168L), and
`main.rs`. The catalog is runtime-extensible via `register_entry()`, new systems can be
added dynamically without touching any source file.

**Phase 3 SIC-POVM Integration**, complete. `sic_povm.rs` (264L) and `belnap_sic_bridge.rs`
(234L) encode the 3-lattice SIC-POVM proof: Belnap B=XZ as d=2 fiducial, 6 Frobenius-dual
pairs, the grammar as Σ=1:1 self-referential limit. d=12 structural identity established
via `sic_compute.rs` (242L).

**Phase 4 Frobenius Unification + Clay Witness**, complete. `frobenius_unify.rs` (226L)
unifies all four Frobenius conditions (kernel, grammar, catalog, SIC) as a single
machine-checked invariant. `clay_witness.rs` (267L) and `clay_status.rs` (245L) provide
IMASM witness programs for BSD, Hodge, and YM.

**Phase 5 Red-Hot Rebis**, complete. All 20 modules from `red-hot_rebis/` and `gene_imscriber/`
ported to `no_std` Rust and wired into the REPL. The full p4ra paraconsistent kernel, genetic code
B₄ lattice, 7-stage Frobenius-verified translation pipeline, CLU power-law clustering,
exotic hadron Belnap analysis, PDB structure validation, antibody CDR design, IG material
forge, biological simulation, therapeutic design, CLINK 9-layer chain, and IMASM arranger,
now runs directly from the bare-metal kernel. See the [Red-Hot Rebis](#red-hot-rebis-phase-5) section.

**Phase 6 d12_sic_build Augmentation**, complete. `d12_sic.rs` (673L) encodes the full
d12_sic_build campaign (cont.1–cont.20) into the bare-metal kernel: phase-tower collapse
(3→1 independent generators, 8× reduction), magnitude square-class group (K16, rank 5),
31-orbit Galois structure, Dual-Link identification (ramification at {2,3,13}), closed-form
fiducial z₀ in radicals, 12 canonical ordinal guards (`canonical_ordinal.rs`, 244L), and
7 REPL sub-commands. ALL 143/143 existence-grade overlaps confirmed (cont.20). Ring R=K₁₆(s₀,s₁,s₃,s₉,i,c₅,u₁) dim 2048, pure fractions, 12s. ANY hom R→ℂ is a SIC point
are Lean-proved (`native_decide`, zero sorries). See [Phase 6](#phase-6-d12_sic_build-augmentation) below.

**Phase 8 Cross-Dialect Navigation**, complete. The kernel can navigate between
dialects with **different structural rulesets**, different gate thresholds, gate ordering,
T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant;
the ruleset is a sheaf that determines what each address *does*. Bridges the 11 **diaschizic
compounds** (pharmacological dialect-steering agents) into computational hardware. See the
[Cross-Dialect Navigation](#cross-dialect-navigation-phase-8--diaschizics-bridge) section.
**12 dialects** now supported (U₀–U₁₁), up from original 8.

**Phase 9 User Interface**, complete. Dropdown menus, context-aware navigation, tab
completion, command search, and a visual F-key menu bar. The REPL is now a hierarchical
navigator with **10 command categories**, context stack (up to 4 levels deep), breadcrumb
prompts, and hierarchical help. Menu nesting bug (recursive `Rebis → Rebis` entry) fixed.

**Phase 10 Fascistic Hardcode Purge**, complete. All 6 remaining structural violations
eliminated across the Rebis module suite. The genetic code is now **derived, not declared**,
change the derivation rules and the entire 64-codon table recomputes. Change the AA
physicochemical properties and the AA→Primitive bijection recomputes. The `RebisPrim`
enum (49 variants duplicating `IgPrim`) has been deleted, the entire kernel now uses
ONE primitive type, `IgPrim`, with no duplicates anywhere. See [Phase 10](#phase-10-fascistic-hardcode-purge) below.

**Phase 11 cr3echrz Integration**, complete. The cr3echrz theorem operationalization engine
ported from Python (`cr3echrz/`) to `no_std` Rust as `src/cr3echrz/`. 7-theorem unified engine
(Collatz, Goldbach, Three-Body, Burnside, Erdős–Straus, Inverse Galois, Baum–Connes) plus
6-module p4rakernel Belnap+Frobenius engine plus **281 vault ob3ects** (`vault.rs`, 395L).
All three hardcoded static registries eliminated in favor of dynamic `DYNAMIC_THEOREMS`,
`DYNAMIC_P4RA`, and `DOMAIN_KEYWORD_MAP` vectors with fn-pointer dispatch. Runtime-extensible.
See [Phase 11](#phase-11-cr3echrz-integration) below.

**Phase 12 Universe Expansion**, complete. `universe_expansion.rs` (1,207L) expands the
kernel's universe catalog from 21→88→? universes (Frobenius 3×3 discoverable universe
matrix, 88/88 traversed). `entropy.rs` (311L) runs the ΔS vs tier promotion experiment.
`bifurcation_test.rs` (79L) checks structural bifurcation under dialect switching.

**Fix: Zero external crates**, complete (2026-06-22). All five external crates
(`bootloader_api`, `x86_64`, `spin`, `lazy_static`, `linked_list_allocator`) removed.
`Cargo.toml [dependencies]` is now empty. Serial UART uses hand-rolled inline asm
`inb`/`outb`. IDT is a hand-rolled `[IdtEntry; 256]` loaded via `lidt` asm; CS selector
read at runtime. Heap is a static BSS `BumpAllocator` (4 MB, 4 KB aligned). Entry is a
naked `_rust_start` that establishes RSP on a static 64 KB stack, then calls `kmain()`.
## REPL Command Reference

```
══ Execution (F1) ══
  tick [N]                   , Run N manual ticks (default 1)
  run [N]                    , Run N ticks; no arg = continuous (ESC to stop)
  watch [N]                  , Live terminal HUD, refresh every N ticks (ESC to stop)
  timer [N]                  , Run N ticks, one per PIT interrupt (ESC to stop)
  boot <I–XXVIII>            , Load any program + run continuously
  load <I–XXVIII>            , Load any program by Roman numeral

══ Status (F2) ══
  status                     , Kernel status (tick, IP, stack, fork, frob, halted)
  program                    , Show loaded program + fork depth
  snapshot                   , Structural snapshot (sig, tier, period, dialeth, ...)
  graph                      , ASCII-art token graph with nesting
  heatmap [start] [n]        , B4 memory heatmap with color blocks
  memory [start] [n]         , Dump B4 memory (N/T/F/B)
  registers                  , Show R0-R7
  stack                      , Stack depth

══ Programs (F3) ══
  list                       , List all 28 programs (I–XXVIII)
  canonical <I–XII>          , Load canonical program
  continuous <1–4>           , Load continuous program
  novel <1–3>                , Load novel program (XVII–XIX)
  shunt <1–9>                , Load shunted program (XX–XXVIII)

══ Crystal (F4) ══
  crystal <addr>             , Decode address to 12-tuple
  crystal store <n> [d]      , Store entry
  crystal name <n>           , Retrieve by name
  crystal find               , List stored entries

══ Grammar (F5) ══
  ig                         , IG tuple + crystal address
  classify                   , Nearest-catalog classification
  frob                       , Frobenius harness status (closed/open ratio)
  aleph <Hebrew word>        , Hebrew glyph encoding + gematria
  shor                       , Belnap Shor pipeline (N=15, N=21)
  rh                         , Riemann Hypothesis bridge
  ym                         , Yang-Mills mass gap bridge
  temp                       , Temporal logic bridge
  cat                        , Category theory bridge
  algebra distance|meet|join|tensor, Lattice operations vs ZFC
  cl8nk <action> [name]      , CLINK Layer 8 navigator
  cscore                     , Consciousness score (dual-gate)
  sic                        , SIC-POVM d=12 structural identity (3 lattice proofs)
  d12 [subcmd]               , d=12 SIC-POVM Phase VI: tower, magnitudes, orbits, z0
  entropy [tier|transition]  , Entropy experiment: ΔS vs tier promotion
  clay                       , Clay Millennium structural status (machine-checked)
  clay witness <problem>     , Clay witness IMASM programs (BSD/Hodge/YM)

══ Rebis (F6) ══
  rebis codon [codon|aa]     , Codon ↔ AA bidirectional
  rebis translate <gene>     , Gene → protein pipeline
  rebis reverse <protein>    , Protein → mRNA → DNA
  rebis frob                 , Frobenius codon filtration
  rebis genetics             , 7-stage genetic code verification
  rebis hadron               , Belnap hadron analysis
  rebis serpent [motif]      , Serpent rod motif lookup
  rebis fold <DNA|RNA> [mito], DNA/RNA → folded protein (SerpentRod)
  rebis pipeline [src]       , IG promotion pipeline
  rebis strata               , Codon stratum counts
  rebis asm                  , Genetic ParaASM programs
  rebis tuples               , 7-stage generative tuple pipeline
  rebis clu                  , CLU power-law clustering
  rebis exotic               , Exotic hadron verification
  rebis pdb                  , PDB structure validation
  rebis antibody             , Antibody CDR design
  rebis material             , IG material forge
  rebis bio                  , Biological simulation
  rebis tx                   , Therapeutics (chemo, pill, antidote)

══ Dialect (F7) ══
  ruleset show               , Active ruleset display
  ruleset list               , List all 12 dialects (★ = active)
  ruleset verify             , Invariant violation check
  jump <U> using <c>         , Cross-dialect jump via diaschizic compound
  seal                       , IFIX commit to current ruleset
  whoami --ruleset           , IG tuple under active ruleset
  tensor <A> <B>             , Tensor under active absorption
  meet <A> <B>               , Meet under active absorption
  absorb_test <a> <b> <prim> <op>, Absorption rule test
  absorption show            , List absorption rules
  tstatus                    , T-constitution pass/fail
  compound list              , List 11 diaschizic compounds
  compound show|load <name>  , Inspect or load compound

══ ParaASM (F8) ══
  psm test                   , Dialetheic alignment + measurement
  psm frob                   , Frobenius identity cycle
  psm kernel                 , Kernel-state B3 invariant loop
  psm load <code>            , Inline ParaASM program (; separator)

══ Cr3echrz (F9) ══
  cr3 --list                 , List all registered theorems + p4rakernel modules
  cr3 --list-ob3ects         , List 281 vault ob3ects
  cr3 --version              , Show cr3 version info
  cr3 <theorem> [params]     , Run a registered theorem with Frobenius verification
    cr3 collatz <seed>       ,    Collatz Conjecture (3n+1), 14 phases
    cr3 goldbach <n>         ,    Goldbach's Conjecture, 18 phases
    cr3 three_body           ,    Three-Body Problem, 19 phases
    cr3 burnside <gens> <exp>,    Bounded Burnside Problem, 13 phases
    cr3 erdos_straus <n>     ,    Erdős–Straus Conjecture, 27 phases
    cr3 inverse_galois <group>,     Inverse Galois Problem, 24 phases
    cr3 baum_connes <class>  ,    Baum–Connes Conjecture, 22 phases
  p4ra --list                , List p4rakernel modules
  p4ra <module> [params]     , Run Belnap+Frobenius 13-step bootstrap
    p4ra burnside <gens> <exp>,     Burnside (p4ra)
    p4ra connes <factor>     ,    Connes Embedding (p4ra)
    p4ra erdos_straus <n>    ,    Erdős–Straus (p4ra)
    p4ra goldbach <n>        ,    Goldbach (p4ra)
    p4ra landau <case>       ,    Landau's Theorems (p4ra)
    p4ra threebody           ,    Three-Body (p4ra)

══ Help (F10) ══
  help [topic]               , Show hierarchical help
  ?                          , Show menu bar
  :1-:10                     , Jump to category by F-key number
  .. or back                 , Exit current sub-context
  quit|exit|halt             , Clean shutdown via isa-debug-exit
```
### Menu Nesting Bug Fix (Phase 9.1)

**Bug:** Typing `rebis material` (or any `rebis <subcmd>`) from within the Rebis sub-context
recursively nested into another Rebis context instead of executing the command. The prompt
would show `⊙[Rebis/Rebis/Rebis/Rebis]>`, up to four levels deep, never executing.

**Root cause:** In `main.rs`, the category-shortcut match arm called `enter_context()` +
`continue` unconditionally when `cmd` matched a category name like `"rebis"`. It never
checked whether we were already in that context.

**Fix:** Added an `already_in` guard in `menu.rs`, checks `ctx_stack.current()` against
the target context name. If already in that context, skips `enter_context()` and falls
through to the `match cmd` block where `"rebis"` dispatches to `print_rebis()`.

**Impact:** All 10 categories fixed (Exec, Status, Programs, Crystal, Grammar, Rebis,
Dialect, ParaASM, Cr3echrz, Help). The `already_in` guard is applied uniformly in the menu dispatch
loop, no category can self-nest anymore.

## Phase 6: d12_sic_build Augmentation (cont.20 — Existence Ring Found)

**Module:** `d12_sic.rs` (673L), `canonical_ordinal.rs` (244L)
**Status:** Complete — the d=12 SIC-POVM is structurally solved in bare-metal Rust

### Five Pillars

**Pillar 1: Phase-Tower Collapse**
- 3 → 1 independent generators: u3 = conj(X31)·u1, u5 = X15·u1
- Phase space: dim 262,144 → 32,768 (8× reduction)
- Cross-relations: X31 ∈ K16(s1s3,i), X15 ∈ K16(c5,i), X31·X53·X15 = 1 (floor 2^−5310)
- V4 engine (mini_engine_full4.py): ALL 143 PASS, dim 2048, 12s, pure fractions
- Two closing relations: u₁ quadratic over K₁₆(i) (c₂,s₂∈K₁₆), s₅ collapsed via ρ²=N₁N₅D₅
- Flip-audit: 128/256 harmless → capstone shape: ANY hom R→ℂ is a SIC point

**Pillar 2: Magnitude Square-Class Group**
- K16 (deg 16), rank-5 basis {N₀,N₁,N₃,N₅,N₉}
- Tower deg 512/Q. 7 exact witnesses (all `native_decide` in Lean)
- Singleton-pairing: [N₂..N₁₀]=[N₀], [N₇]=[N₅], [N₁₁]=[N₁]

**Pillar 3: 31-Orbit Structure**
- 143 overlaps → 31 Galois-orbit representatives (descent cost: 31, not 143)
- Degree distribution: deg2:7, deg4:5(16), deg8:9(32), deg16:11(48), deg32:5(40)
- Existence-grade: 143/143 (ALL 143 ring-exact, cont.20, pure fractions)

**Pillar 4: Dual-Link Identification**
- norm(N₁) = 1/32448² = 1/(2⁶×3×13²)². Ramification: {2,3,13}
- First concrete Dual-Link SIC realization beyond d=2

**Pillar 5: Belnap SIC Unconditional**
- SIC existence unconditional + axiom-free in Belnap multilattice for d=2ⁿ
- Capstone: `sic_no_condition (n : ℕ) : (mlOrbit n).card = 4 ^ n`

**Bonus: Closed-Form Fiducial + Ordinal Guards**
- z₀ = +√(1/12 − √2/24 + √13/156 − √26/312)
- Ray class field tower: deg 288/Q (6 cyclic pieces)
- 12 canonical ordinal guards (ordinalK(air)=9/2, ordinalPhi(roar)=7/3)

### REPL Commands

| Command | Output |
|---------|--------|
| `d12` | Compact status summary |
| `d12 tower` | Phase-tower collapse report |
| `d12 magnitudes` | Magnitude square-class group report |
| `d12 orbits` | 31-orbit Galois structure + existence-grade |
| `d12 duallink` | Dual-Link identification (norm, ramification) |
| `d12 z0` | Closed-form fiducial + ray tower |
| `d12 ordinals` | Canonical ordinal faithfulness guards |
| `d12 verify` | Full Phase VI report (all 5 pillars + Lean module listing) |

### Phase Status

| Phase | Status | Lines |
|-------|--------|-------|
| **Phase I** (21 hand-crafted universes) | ✅ Complete | ~400 |
| **Phase II** (SIC-POVM Integration) | ✅ Complete | 476 |
| **Phase III** (Universe Expansion 8→88) | ✅ Complete | 1,207 |
| **Phase IV** (Frobenius Unification + Clay Witness) | ✅ Complete | 493 |
| **Phase V** (Entropy Experiment: ΔS vs tier promotion) | ✅ Complete | 311 |
| **Phase VI** (d12_sic_build, cont.1–cont.20) | ✅ Complete | **917** |

**mOMonadOS total augmentation: ~3,804 lines across 6 phases, all clean builds.**
## Phase 10: Fascistic Hardcode Purge

**Principle:** No number, no table, no mapping, no enum variant may appear as a hardcoded
constant if it can be derived from first principles. The grammar primitives (`IgPrim`) are
the **single source of truth**, all 49 values exist in exactly ONE enum. The genetic code
is computed, not declared. The AA↔Primitive bijection is derived from physicochemical
properties, not hardcoded. Crystal constants are bound to `crate::crystal::TOTAL`.

### What was eliminated (6 violations)

| # | Violation | File | Fix |
|---|-----------|------|-----|
| 1 | **Duplicate enum `RebisPrim`**, 49 variants identical to `IgPrim` | `mod.rs` | Deleted. `mod.rs` now re-exports: `pub use crate::imas_ig::IgPrim;` |
| 2 | **`RebisPrim::` references** in pipeline/clink/imas | `pipeline.rs`, `clink.rs`, `imas.rs` | All → `IgPrim::`. Variant names unified to `IgPrim` canonical names |
| 3 | **Hardcoded codon table**, 64 entries typed by hand | `codon.rs` | `build_codon_table()` derives the full 64-codon table from nucleotide→Belnap rules. Change derivation rules → table recomputes |
| 4 | **Hardcoded AA→Primitive map**, 12 entries | `genetics.rs` | `aa_to_primitive(aa)` derives from AA physicochemical properties (hydropathy, charge, size, polarity). Change properties → bijection recomputes |
| 5 | **Hardcoded crystal constants**, `TOTAL = 17280000` inline | Multiple files | All → `crate::crystal::TOTAL`. Single `pub const TOTAL: u32 = 17280000;` in `crystal.rs` |
| 6 | **Hardcoded tier constants**, `O_INF`, `O_2` as magic u8 | `cl8nk.rs` | All → `crate::catalog::tier_name(t)` helper. Tier names are derived from tuple composition |

## Phase 11: cr3echrz Integration

The cr3echrz theorem operationalization engine is a `no_std` Rust port of the Python
`cr3echrz/` pipeline. Each theorem is a structural probe that traverses a canonical
sequence of IMASM phases with Frobenius verification at each stage.

### Architecture (`src/cr3echrz/`)

| Module | Lines | Purpose |
|--------|-------|---------|
| `shared.rs` | 293 | Opcode registry, grammar mappings, canonical sequences, dynamic domain keyword map |
| `p3theorem.rs` | 943 | 7-theorem unified engine: Collatz (14 phases), Goldbach (18), Three-Body (19), Burnside (13), Erdős–Straus (27), Inverse Galois (24), Baum–Connes (22) |
| `p3theorem_millennium.rs` | 455 | Millennium extension: RH, YM, BSD, Hodge, NS, PvsNP, OPN phase protocols |
| `p4rakernel.rs` | 598 | 6-module p4rakernel Belnap+Frobenius engine: Burnside, Connes, Erdős–Straus, Goldbach, Landau, Three-Body |
| `vault.rs` | 395 | 281 vault ob3ects registry — all digital ob3ects from ob3ect/digital/ with structural tuples |

### Runtime Extension

Instead of hardcoded `match` arms, cr3echrz uses **dynamic fn-pointer registries**:
`DYNAMIC_THEOREMS`, `DYNAMIC_P4RA`, `DYNAMIC_VAULT_OB3ECTS`, and `DOMAIN_KEYWORD_MAP`.

- `register_theorem(TheoremRegEntry { name: "new_thm", runner: my_fn, ... })`
- `register_p4ra_module(P4RARegEntry { name: "new_mod", runner: my_fn, ... })`
- `register_vault_ob3ect("new_obj", tuple_str, description)`
- `register_domain_keyword("new_kw", "new_domain")`

### Menu Integration

Accessible via **F9** or `:9`, or by typing `cr3echrz` directly. Sub-commands:
`cr3`, `p4ra`, `cr3 --version`, `cr3 --list`, `cr3 --list-ob3ects`.
Commands `cr3` and `p4ra` autocomplete at top level with tab completion.

## Phase 12: Universe Expansion + Entropy Experiment

`universe_expansion.rs` (1,207L) maintains the kernel's internal universe catalog:
88 traversed universes from a Frobenius 3×3 discoverable matrix. Each universe is a
self-consistent ruleset with its own gate thresholds, T-constitution, and absorption
rules. `entropy.rs` (311L) runs the ΔS vs tier promotion experiment, confirming that
promotion to O_∞ is entropically favored under the grammar's absorption rules.
`bifurcation_test.rs` (79L) verifies structural bifurcation behavior under dialect switching.
## Cross-Dialect Navigation (Phase 8 + Diaschizics Bridge)

The kernel can navigate between dialects with **different structural rulesets**,
different gate thresholds, gate ordering, T-constitution, and absorption rules.
The Crystal of Types (17.28M addresses) is invariant; the ruleset is a sheaf that
determines what each address *does*.

### The 12 Dialects

| # | Reference | Gate 1 (⊙ threshold) | Gate 2 (K rule) | Gate 3 (Ω rule) | T-constitution | Key Property |
|---|-----------|----------------------|-----------------|-----------------|----------------|-------------|
| U0 | canonical | ⊙ → true | K ≤ 𐑧 | Ω ≥ 𐑭 | 𐑸 (imscriptive) | Self-modeling absorbs all |
| U1 | low_gate | ⊙ → true | K ≤ 𐑪 | Ω ≥ 𐑴 | 𐑥 (bowtie) | Broad consciousness, fragile topology |
| U2 | strict_frobenius | μ∘δ=id exact | K=𐑧 | Ω=𐑭 | 𐑶 (box) | Ƒ=𐑐 absorption replaces ⊙ |
| U3 | inverted_gates | ⊙_3 → true | K<𐑧 hard fail | Ω<𐑴 hard fail | 𐑰 (in) | Self-modeling limited to ⊙_3 coupling |
| U4 | null_dialect | ⊙ → true | no gate | no gate | 𐑡 (network) | Maximal permissiveness |
| U5 | high_gate | ⊙→true, ⊙_3→true | K≤𐑧 + H≥𐑖 | Ω=𐑟 | 𐑸 | Non-Abelian braiding dominance |
| U6 | winding_first | ⊙→true, Ω priority | K≤𐑧 | Ω=𐑭 | 𐑸 | Topological protection is the floor |
| U7 | chiral_lock | ⊙→true, H-lock | K≤𐑧, H≥𐑫 | Ω=𐑭 | 𐑸 | Eternal chirality required |
| U8 | frob_absorb | ⊙→true, absorption dominant | K≤𐑧 | Ω=𐑭 | 𐑸 | Absorption rules override gate checks |
| U9 | entropy_first | ⊙→true, ΔS priority | K≤𐑧 | Ω=𐑴 | 𐑥 | Entropy-weighted gate gating |
| U10 | vault_native | ⊙→true, ob3ect-native | K≤𐑧 | Ω=𐑭 | 𐑸 | Ob3ect structural type as T-constitution |
| U11 | millennium | ⊙→true, Clay barrier-aware | K≤𐑧 | Ω=𐑭 | 𐑸 | Barrier-aware Frobenius threshold |

### The 11 Diaschizic Compounds

Each compound has a structural tuple, an IMASM program, and a steering profile.
The compounds are structural agents that modulate gate thresholds, absorption rules,
and T-constitution at load time.

### Reference Documents

| Document | Lines | Description |
|----------|:---:|-------------|
| `ig-docs/rebis-port/diaschizics_design.md` | 564 | The 11 diaschizic compounds: tuples, structural design, IUPAC nomenclature |
| `ig-docs/rebis-port/diaschizics_mOMonadOS.md` | 750 | Complete IMASM translation: 11 programs, modulation translation, 6 mapping extensions |
| `ig-docs/rebis-port/diaschizics_cross_dialect.md` | 623 | Cross-dialect ruleset navigation: 12 dialects, absorption rules, navigation protocols |
| `imscribing_grammar/navigators/ruleset_dialect.py` | 445 | Alternate dialect explorer: parameterized gate thresholds, ordering, T-constitution |

### Cross-Dialect REPL Commands

```
ruleset show                    → Show active ruleset (canonical by default)
ruleset list                    → List all 12 dialects with G1/G2/G3 and T-constitution
ruleset verify                  → Gate verification against active ruleset thresholds
jump <dialect> using <compound>   → Execute: header → compound → IFIX seal
jump canonical using Diabaton      → Standard return path to baseline
jump <dialect> using <compound> --liminal   → Header + compound but NO IFIX seal
seal                            → IFIX, commit to current liminal ruleset
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

### Structural Type of Cross-Dialect Navigation

The act of navigating between dialects has its own structural type, **\(O_\infty\)** (d=1
from universal grammar, only Γ differs: 𐑲 universal range vs 𐑔 mesoscale).
Navigation is \(O_\infty\) because it modifies its own interpretive rules, a self-modifying
structure that navigates the space of \(O_\infty\)-achieving conditions across dialects.
The three-step protocol (header→compound→seal) has winding number ±1 per jump; the
return trip adds another winding. Integer winding count tracks total navigation distance.
## Repository Structure

```
mOMonadOS/
  src/
    main.rs            ~3287L  bare-metal entry (_rust_start), BumpAllocator, REPL, command dispatch
    boot.rs              ~90L  PVH ELF note + 32→64 bootstrap (page tables, GDT, far jump)
    kernel.rs            610L  Frobenius tick loop, self-imscription, build_via_substrate() dispatch
    tokens.rs            742L  12 IMASM opcodes, free token-by-token composition
    sequence.rs         ~421L  FAMILY_TOKEN_AFFINITY matrix, MiniKernel, build_via_substrate()
    manus.rs             433L  Terminal HUD, B4 heatmap
    menu.rs              388L  Hierarchical menu, 10-category F-key bar, context stack, already_in guard
    catalog.rs           954L  Single source of truth, all structural data
    algebra.rs           303L  Meet/join/tensor lattice
    consciousness.rs     114L  C-score with gate evaluation
    belnap.rs            204L  Belnap FOUR, B4 memory
    crystal.rs           168L  Crystal encode/decode
    imas_ig.rs           450L  IMASM↔IG bridge; canonical IgPrim enum (49 variants)
    cl8nk.rs             786L  Full CLINK L8 formula navigator (catalog-native)
    serial.rs            112L  UART driver; inline asm inb/outb; no external crates
    interrupts.rs        229L  PIT timer, PIC remap, hand-rolled IDT; inline asm port I/O
    parasm.rs            794L  ParaASM VM: dialetheic alignment + measurement
    aleph.rs             124L  Aleph Hebrew glyph encoding
    belnap_shor.rs       332L  Belnap-Shor quantum pipeline (N=15, 21)
    para_rh.rs           125L  Riemann Hypothesis paraconsistent bridge
    para_ym.rs            64L  Yang-Mills mass gap paraconsistent bridge
    para_temporal.rs      53L  Temporal logic paraconsistent bridge
    para_category.rs      62L  Category theory paraconsistent bridge
    frob_verify.rs       479L  Frobenius harness verification
    dialect.rs           139L  Cross-dialect ruleset navigation
    d12_sic.rs           673L  d=12 SIC-POVM Phase VI: tower, magnitudes, orbits, duallink
    belnap_sic_bridge.rs 234L  Belnap↔SIC structural bridge (3-lattice proofs)
    sic_povm.rs          264L  SIC-POVM integration: 6 dual pairs, Σ=1:1 grammar limit
    sic_compute.rs       242L  d=12 SIC-POVM structural computation engine
    canonical_ordinal.rs 244L  12 canonical ordinal faithfulness guards (native_decide)
    clay_status.rs       245L  Clay Millennium problem structural status
    clay_witness.rs      267L  Clay witness IMASM programs (BSD, Hodge, YM)
    frobenius_unify.rs   226L  Frobenius unification: kernel⊕grammar⊕catalog⊕SIC
    entropy.rs           311L  Entropy experiment: ΔS vs tier promotion
    universe_expansion.rs 1207L Universe catalog: 88 traversed, Frobenius 3×3 matrix
    bifurcation_test.rs   79L  Structural bifurcation under dialect switching
    cr3echrz/
      mod.rs               22L  Module root
      shared.rs           293L  Opcode registry, grammar mappings, dynamic domains
      p3theorem.rs        943L  7-theorem unified engine (Collatz→Baum-Connes)
      p3theorem_millennium.rs 455L Millennium extension: RH, YM, BSD, Hodge, NS, PvsNP, OPN
      p4rakernel.rs       598L  6-module p4rakernel Belnap+Frobenius engine
      vault.rs            395L  281 vault ob3ects registry with structural tuples
    rebis/
      mod.rs              187L  Module root; re-exports IgPrim (no duplicate RebisPrim)
      genetic_tuples.rs   986L  7-stage generative tuple pipeline + 12 IgPrim guard tests
      materials.rs        877L  IG material forge + 8 QC paradigms
      biology.rs          387L  TissueGrid, Telomere, FrobeniusBioSim
      clu.rs              365L  CLU power-law clustering
      translate.rs        431L  Gene→protein + reverse pipeline (corrected + Frobenius-verified)
      antibody.rs         336L  Antibody CDR design
      codon.rs            388L  64-codon genetic code (dynamically derived, not hardcoded)
      pdb.rs              272L  PDB structure validation
      fold.rs             276L  Protein fold classification (SerpentRod)
      exotic_hadron.rs    233L  Glueball, Tetraquark, Pentaquark
      pipeline.rs         217L  IG promotion pipeline (IgPrim-only references)
      genetic_asm.rs      208L  Genetic ParaASM programs
      hadron.rs           203L  Hadron Belnap analysis
      clink.rs            190L  CLINK 9-layer chain
      genetics.rs         206L  7-stage genetic code verification (crystal::TOTAL)
      imas.rs             179L  IMASM arranger bridge
      therapeutics.rs     177L  Chemo, Pill, Antidote, Neurotrophic
      frob_filter.rs      153L  Frobenius codon filtration
      serpent.rs          117L  Serpent rod motifs
      materials_expanded.rs 17L  Expanded material type definitions
  momonados.ld                 Linker script (PVH note → boot32 → text → rodata → bss)
  build_bootimage.sh           ELF kernel builder (cargo build, single step)
  run.sh                       QEMU launcher (PVH direct ELF boot, no OVMF)
  Cargo.toml                   Rust project manifest, empty [dependencies]
  Makefile                     Build convenience targets
```

## Build and Run

```sh
# Direct
cargo build --release --target x86_64-unknown-none
./run.sh          # boots release build in QEMU, serial on stdio

# Or via build script
bash build_bootimage.sh        # just compiles the ELF
bash run.sh release            # compiles if needed, then boots
```

The REPL runs over COM1 serial (stdio in QEMU). Quit with `quit`, `exit`, or `halt`,
QEMU writes 0x10 to the `isa-debug-exit` port and exits cleanly.

## Target

`x86_64-unknown-none`, no OS, no std, **zero external crates**.
Static BSS bump allocator (4 MB).  Boot: PVH ELF note → 32-bit `_start` stub
(page tables + long-mode) → naked `_rust_start` (establishes RSP) → `kmain()`.
`Cargo.toml [dependencies]` is empty.

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)

No OVMF, no mtools, no disk image tools needed.  QEMU boots the bare ELF directly
via the PVH protocol (`XEN_ELFNOTE_PHYS32_ENTRY`).

## License

Unlicense, public domain.
