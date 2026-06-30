# $m\odot^2$, The Self-Imscribing Bare-Metal Kernel

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop, every tick is a structural self-verification.

**Author:** Lando⊗⊙perator  
**Total codebase:** ~19,300 lines Rust (no_std) + build scripts  
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

**Phase 5 Red-Hot Rebis**, complete. All 20 modules from `red-hot_rebis/` and `gene_imscriber/`
ported to `no_std` Rust and wired into the REPL. The full p4ra paraconsistent kernel, genetic code
B₄ lattice, 7-stage Frobenius-verified translation pipeline, CLU power-law clustering,
exotic hadron Belnap analysis, PDB structure validation, antibody CDR design, IG material
forge, biological simulation, therapeutic design, CLINK 9-layer chain, and IMASM arranger , 
now runs directly from the bare-metal kernel. See the [Red-Hot Rebis](#red-hot-rebis-phase-5) section.

**Phase 8 Cross-Dialect Navigation**, complete. The kernel can navigate between
dialects with **different structural rulesets**, different gate thresholds, gate ordering,
T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant;
the ruleset is a sheaf that determines what each address *does*. Bridges the 11 **diaschizic
compounds** (pharmacological dialect-steering agents) into computational hardware. See the
[Cross-Dialect Navigation](#cross-dialect-navigation-phase-8--diaschizics-bridge) section.

**Phase 9 User Interface**, complete. Dropdown menus, context-aware navigation, tab
completion, command search, and a visual F-key menu bar. The REPL is now a hierarchical
navigator with **10 command categories**, context stack (up to 4 levels deep), breadcrumb
prompts, and hierarchical help. Menu nesting bug (recursive `Rebis → Rebis` entry) fixed.

**Phase 10 Fascistic Hardcode Purge**, complete. All 6 remaining structural violations
eliminated across the Rebis module suite. The genetic code is now **derived, not declared** , 
change the derivation rules and the entire 64-codon table recomputes. Change the AA
physicochemical properties and the AA→Primitive bijection recomputes. The `RebisPrim`
enum (49 variants duplicating `IgPrim`) has been deleted, the entire kernel now uses
ONE primitive type, `IgPrim`, with no duplicates anywhere. See [Phase 10](#phase-10-fascistic-hardcode-purge) below.

**Phase 11 cr3echrz Integration**, complete. The cr3echrz theorem operationalization engine
ported from Python (`cr3echrz/`) to `no_std` Rust as `src/cr3echrz/`. 7-theorem unified engine
(Collatz, Goldbach, Three-Body, Burnside, Erdős–Straus, Inverse Galois, Baum–Connes) plus
6-module p4rakernel Belnap+Frobenius engine. All three hardcoded static registries eliminated
in favor of dynamic `DYNAMIC_THEOREMS`, `DYNAMIC_P4RA`, and `DOMAIN_KEYWORD_MAP` vectors with
fn-pointer dispatch. Runtime-extensible: `register_theorem()`, `register_p4ra_module()`,
`register_domain_keyword()`. Menu integrated as F9/Cr3echrz with 4 sub-commands. See
[Phase 11](#phase-11-cr3echrz-integration) below.

**Fix: Zero external crates**, complete (2026-06-22). All five external crates
(`bootloader_api`, `x86_64`, `spin`, `lazy_static`, `linked_list_allocator`) removed.
`Cargo.toml [dependencies]` is now empty. Serial UART uses hand-rolled inline asm
`inb`/`outb`. IDT is a hand-rolled `[IdtEntry; 256]` loaded via `lidt` asm; CS selector
read at runtime. Heap is a static BSS `BumpAllocator` (4 MB, 4 KB aligned). Entry is a
naked `_rust_start` that establishes RSP on a static 128 KB `BOOT_STACK`, then calls the
kernel. QEMU boots the bare ELF via PVH (`XEN_ELFNOTE_PHYS32_ENTRY`) + a 32→64 mode
transition stub in `src/boot.rs`; no OVMF or disk image required.

**Fix: Live-tuple vote weights**, complete (2026-06-22). `sequence.rs`
`aggregate_votes()` replaced 40+ per-variant match arms with a 12×12
`FAMILY_TOKEN_AFFINITY` const matrix. Vote weight = `affinity[family][token] × ordinal`.
Higher-ordinal variants push harder toward their family's preferred tokens; weights are
derived from the running tuple state, not from compiled-in constants.

**Fix: Autopoietic sequence construction**, complete (2026-06-22).
`build_via_substrate()` seeds a `MiniKernel` from the `IgTuple` (4 Belnap registers +
64-entry stack), runs a tier-selected canonical IMASM program on it, maps post-execution
register state through `TOKEN_REG_AFFINITY`, and combines that with family affinity scores
(substrate ×3 + family ×1). The sequence builder is itself an IMASM execution, the
kernel runs itself to decide what to run next.

The kernel now supports **90+ REPL commands** spanning grammar operations, rebis
biological/chemical computation, cross-dialect navigation, theorem operationalization,
and hierarchical menu navigation.
### Core modules

| Module | Lines | Source | Role |
|--------|:-----:|--------|------|
| `main.rs` | ~2,800 | native | bare-metal entry, static BSS heap init, serial REPL, command dispatch, history, menu navigation, F-key interception, context-aware prompts, cr3/p4ra dispatch |
| `kernel.rs` | 576 | native | Frobenius tick loop, self-imscription, `build_via_substrate()` dispatch |
| `tokens.rs` | 637 | native | 12 IMASM opcodes, free token-by-token composition |
| `sequence.rs` | ~320 | native | FAMILY_TOKEN_AFFINITY matrix, MiniKernel, autopoietic sequence builder |
| `manus.rs` | 432 | native | Terminal HUD, B4 heatmap |
| `menu.rs` | 388 | native | Hierarchical menu, 10-category F-key bar, context stack, `already_in` guard |
| `catalog.rs` | 954 | grammar | Single source of truth, all structural data, IG tuples, ordinals, glyphs |
| `algebra.rs` | 303 | grammar | Meet/join/tensor lattice operations |
| `consciousness.rs` | 114 | grammar | C-score with dual-gate evaluation |
| `belnap.rs` | 203 | grammar | Belnap FOUR logic, B4 memory |
| `crystal.rs` | 168 | grammar | Crystal FS: encode/decode Frobenius addresses |
| `imas_ig.rs` | 450 | grammar | IMASM↔IG bridge; canonical `IgPrim` enum (49 variants) |
| `cl8nk.rs` | 787 | grammar | Full CLINK L8 formula navigator (catalog-native) |
| `serial.rs` | 96 | native | UART driver; inline asm `inb`/`outb`; zero crates |
| `interrupts.rs` | ~190 | native | PIT timer, PIC remap, hand-rolled IDT; inline asm port I/O |
| `parasm.rs` |, | grammar | ParaASM VM: dialetheic alignment, Frobenius identity, B3 loop |
| `aleph.rs` |, | grammar | Aleph Hebrew glyph encoding |
| `belnap_shor.rs` |, | grammar | Belnap-Shor quantum pipeline (N=15, 21) |
| `para_rh.rs` |, | grammar | Riemann Hypothesis paraconsistent bridge |
| `para_ym.rs` |, | grammar | Yang-Mills mass gap paraconsistent bridge |
| `para_temporal.rs` |, | grammar | Temporal logic paraconsistent bridge |
| `para_category.rs` |, | grammar | Category theory paraconsistent bridge |
| `frob_verify.rs` |, | grammar | Frobenius harness verification |
| `dialect.rs` |, | grammar | Cross-dialect ruleset navigation |
| `cr3echrz/mod.rs` | 18 | cr3echrz | Module root; re-exports p3theorem, p4rakernel, shared |
| `cr3echrz/shared.rs` | 252 | cr3echrz | Opcode registry, grammar mappings, canonical sequences, dynamic domain keywords |
| `cr3echrz/p3theorem.rs` | 700 | cr3echrz | 7-theorem unified engine: Collatz, Goldbach, Three-Body, Burnside, Erdős–Straus, Inverse Galois, Baum–Connes; dynamic `DYNAMIC_THEOREMS` registry |
| `cr3echrz/p4rakernel.rs` | 598 | cr3echrz | 6-module Belnap+Frobenius p4rakernel engine: Burnside, Connes, Erdős–Straus, Goldbach, Landau, Three-Body; dynamic `DYNAMIC_P4RA` registry |
| `rebis/mod.rs` | 183 | rebis | Module root; re-exports `IgPrim` (no duplicate `RebisPrim`) |
| `rebis/genetic_tuples.rs` | 986 | rebis | 7-stage generative tuple pipeline + 12 `IgPrim` guard tests |
| `rebis/materials.rs` | 877 | rebis | IG material forge + 8 QC paradigms |
| `rebis/biology.rs` | 387 | rebis | TissueGrid, Telomere, FrobeniusBioSim |
| `rebis/clu.rs` | 365 | rebis | CLU power-law clustering |
| `rebis/translate.rs` | 360 | rebis | Gene→protein + reverse pipeline (Frobenius-verified) |
| `rebis/antibody.rs` | 336 | rebis | Antibody CDR design |
| `rebis/codon.rs` | 304 | rebis | 64-codon genetic code (dynamically derived, not hardcoded) |
| `rebis/pdb.rs` | 272 | rebis | PDB structure validation |
| `rebis/exotic_hadron.rs` | 233 | rebis | Glueball, Tetraquark, Pentaquark |
| `rebis/pipeline.rs` | 217 | rebis | IG promotion pipeline (`IgPrim`-only references) |
| `rebis/genetic_asm.rs` | 208 | rebis | Genetic ParaASM programs |
| `rebis/hadron.rs` | 203 | rebis | Hadron Belnap analysis |
| `rebis/clink.rs` | 190 | rebis | CLINK 9-layer chain |
| `rebis/genetics.rs` | 187 | rebis | 7-stage genetic code verification (`crystal::TOTAL`) |
| `rebis/imas.rs` | 179 | rebis | IMASM arranger bridge |
| `rebis/therapeutics.rs` | 177 | rebis | Chemo, Pill, Antidote, Neurotrophic |
| `rebis/frob_filter.rs` | 153 | rebis | Frobenius codon filtration |
| `rebis/serpent.rs` | 117 | rebis | Serpent rod motifs |
| `rebis/fold.rs` |, | rebis | Protein fold classification |
| `rebis/materials_expanded.rs` | 17 | rebis | Expanded material type definitions |

## REPL Commands

### Quick Reference (grouped by category)

```
══ Exec (F1) ══
  tick [n]                   , Run N manual ticks (default 1)
  run [n]                    , Run N ticks; no arg = continuous (ESC to stop)
  watch                      , Live terminal HUD (ESC to stop)
  timer [n]                  , Run N ticks, one per PIT interrupt
  boot [n]                   , Load + run any program (I-XXVIII or decimal)
  load [n]                   , Load program by Roman numeral

══ Status (F2) ══
  status                     , Kernel status (tick, IP, stack, fork, frob)
  program                    , Show loaded program + fork depth
  snapshot                   , Structural snapshot (sig, tier, period)
  registers                  , Show registers R0-R7 + IP
  stack                      , Stack depth
  graph                      , ASCII token graph with nesting depth
  heatmap                    , B4 memory heatmap
  memory                     , Dump B4 memory

══ Programs (F3) ══
  list                       , List all programs (I-XXVIII)
  canonical [n]              , Load canonical program I-XII
  continuous [n]             , Load continuous program 1-4
  novel [n]                  , Load novel program 1-3
  shunt [n]                  , Load shunted program 1-9
  dynamic [on|off]           , Toggle dynamic sequence from IgTuple

══ Crystal (F4) ══
  crystal <addr>             , Decode Frobenius address to 12-tuple
  crystal store <n> [d]      , Store entry by address
  crystal name <n>           , Retrieve stored entry by name
  crystal find               , List stored entries

══ Grammar (F5) ══
  ig                         , IG tuple + crystal address
  classify                   , Nearest-catalog classification
  frob                       , Frobenius harness status
  aleph <word>               , Hebrew glyph encoding
  shor [N=15|21]             , Belnap-Shor pipeline
  rh                         , Riemann Hypothesis bridge
  ym                         , Yang-Mills mass gap bridge
  temp                       , Temporal logic bridge
  cat                        , Category theory bridge
  algebra distance|meet|join|tensor, Lattice operations vs ZFC
  cl8nk <action> [name]      , CLINK Layer 8 navigator
  cscore                     , Consciousness score (dual-gate)

══ Rebis (F6) ══
  rebis codon [codon|aa]     , Codon ↔ AA bidirectional
  rebis translate <gene>     , Gene → protein pipeline
  rebis reverse <protein>    , Protein → mRNA → DNA
  rebis frob                 , Frobenius codon filtration
  rebis genetics             , 7-stage genetic code verification
  rebis hadron               , Belnap hadron analysis
  rebis serpent [motif]      , Serpent rod motif lookup
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
  ruleset list               , List all 8 dialects
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
| 3 | **Hardcoded `CODON_TABLE`**, 64-entry static array | `codon.rs` | Replaced with `derive_codon_table()`, computed from B₄ lattice + Frobenius stratum rules. Lazy-initialized via `AtomicBool`. `verify_derived_table()` cross-checks against standard genetic code |
| 4 | **Hardcoded `AminoAcid::to_primitive()`**, 12 hardcoded match arms | `genetic_tuples.rs` (removed from `mod.rs`) | Replaced with `aa_activation()` system using physicochemical properties (β-branching, aromaticity, charge, hydroxyl content). Same 12 AAs promote to same 12 primitives |
| 5 | **Hardcoded `17_280_000`** | `genetics.rs` | → `crate::crystal::TOTAL as u64` |
| 6 | **Duplicate value enums** (`DVal`, `TVal`, …, `OVal`, 12 enums duplicating `IgPrim` value space) | `genetic_tuples.rs` | Retained for pipeline role (generative tuple construction), but **guarded** by 12 consistency tests verifying every variant's glyph matches its `IgPrim` counterpart. Any drift in `IgPrim` breaks these tests at compile time |

### What remains (justified static data)

| Data | Location | Why justified |
|------|----------|---------------|
| `CARDS` [4,5,4,5,3,5,3,4,5,4,3,4] | `catalog.rs` | This **IS** the grammar, the cardinalities of the 12 primitive families. The 17.28M-type crystal emerges from this product |
| Catalog entries | `catalog.rs` | Reference data, the catalog IS the systems being described |
| AA physicochemical properties | `mod.rs` (AminoAcid) | Biological facts, hydropathy, MW, aromaticity are measured, not derived |
| Material property maps | `materials.rs` | Domain knowledge, glyph→physical property mapping requires domain interpretation |
| `DistanceWeights::default()` | `catalog.rs` | Calibratable at runtime via `set_distance_weights()` |
| Primitive ordinal tables | `catalog.rs` | The ordering of values within each family IS the grammar definition |

### Architectural principle enforced

**Single source of truth:** All 49 grammar primitive values now flow from ONE enum, `IgPrim`
in `imas_ig.rs`. Every glyph string, ordinal, and short name delegates to
`crate::catalog::primitive_glyph()` / `crate::catalog::primitive_short()`. No module
anywhere defines its own copy of the grammar primitive space.

The genetic code is now **derived, not declared**, change the derivation rules in
`derive_codon_table()` and the entire 64-codon mapping updates. Change the AA properties
in `AminoAcid` and the activation profile recomputes. The kernel no longer contains a
single hardcoded codon table or AA→primitive mapping, both are computed dynamically
at boot with runtime verification against the standard genetic code.

## Phase 11: cr3echrz Integration

The cr3echrz theorem operationalization engine has been ported from Python
(`imscribing_grammar/cr3echrz/`) to `no_std` Rust as `src/cr3echrz/` (1,568 lines, 4 files).
It provides two computational engines, a 7-theorem unified engine (`p3theorem`) and a
6-module Belnap+Frobenius p4rakernel engine (`p4rakernel`), both with dynamic registries
and fn-pointer dispatch.

### Modules

| File | Lines | Role |
|------|:-----:|------|
| `cr3echrz/mod.rs` | 18 | Module root; re-exports p3theorem, p4rakernel, shared |
| `cr3echrz/shared.rs` | 252 | Universal opcode registry, grammar mappings, canonical sequences (I–XII), dynamic domain keyword classifier |
| `cr3echrz/p3theorem.rs` | 700 | 7-theorem unified operationalization engine with Frobenius verification |
| `cr3echrz/p4rakernel.rs` | 598 | 6-module Belnap+Frobenius 13-step paraconsistent bootstrap engine |

### Theorems (p3theorem)

| Theorem | Phases | Example | Description |
|---------|:------:|---------|-------------|
| Collatz | 14 | `cr3 collatz 27` | 3n+1 conjecture, orbit verification with Belnap state tracking |
| Goldbach | 18 | `cr3 goldbach 100` | Every even n≥4 is sum of two primes, partition search + Frobenius verification |
| Three-Body | 19 | `cr3 three_body` | Hamiltonian non-integrability, figure-8 orbit integration |
| Burnside | 13 | `cr3 burnside 2 5` | B(m,n) group finiteness, presentation enumeration |
| Erdős–Straus | 27 | `cr3 erdos_straus 73` | 4/n = 1/x + 1/y + 1/z, Egyptian fraction decomposition |
| Inverse Galois | 24 | `cr3 inverse_galois Sn` | Every finite group as Galois group over Q, parametric polynomial construction |
| Baum–Connes | 22 | `cr3 baum_connes a-T-menable` | Assembly map isomorphism, KK-theory verification |

### p4rakernel Modules

| Module | Description |
|--------|-------------|
| Burnside (p4ra) | Bounded Burnside B(m,n), Belnap+Frobenius 13-step bootstrap |
| Connes Embedding | II₁ factor embedding in R^ω, JNVWY 2020 resolution boundary |
| Erdős–Straus (p4ra) | 4/n decomposition with Belnap state tracking |
| Goldbach (p4ra) | Even n = p+q with Belnap+Frobenius partition verification |
| Landau's Theorems | Holomorphic function classification on unit disk (4 cases) |
| Three-Body (p4ra) | Poincaré non-integrability + KAM boundary analysis |

### Hardcode Purge (3 violations eliminated)

| # | Violation | File | Fix |
|---|-----------|------|-----|
| 1 | **`static THEOREM_REGISTRY`**, hardcoded array of 7 `TheoremEntry` structs with hardcoded `run_theorem()` match-arm dispatch | `p3theorem.rs` | Replaced with `DYNAMIC_THEOREMS` vector initialized from `THEOREM_BOOTSTRAP`. Dispatch via fn-pointer lookup: `(entry.runner)(params)`. `register_theorem()` for runtime extensibility |
| 2 | **`static P4RA_MODULES`**, hardcoded array of 6 `P4RAModule` structs with hardcoded `run_p4ra_module()` match-arm dispatch | `p4rakernel.rs` | Replaced with `DYNAMIC_P4RA` vector initialized from `P4RA_BOOTSTRAP`. Same fn-pointer dispatch pattern. `register_p4ra_module()` for runtime extensibility |
| 3 | **`infer_domain()` keyword lists**, 6 hardcoded keyword arrays for domain classification | `shared.rs` | Replaced with `DOMAIN_KEYWORD_MAP` dynamic vector initialized from `DOMAIN_BOOTSTRAP`. `register_domain_keyword()` for runtime extensibility |

### What remains (justified static data)

| Data | Location | Why justified |
|------|----------|---------------|
| `Opcode` enum (12 variants) | `shared.rs` | **IS** the grammar, the 12 universal IMASM opcodes |
| `CANONICAL_SEQUENCES` (I–XII) | `shared.rs` | **IS** the grammar, 12 canonical bootstrap programs |
| `opcode_grammar()` mapping | `shared.rs` | **IS** the grammar, opcode↔primitive correspondence |
| `THEOREM_BOOTSTRAP` (7 entries) | `p3theorem.rs` | Reference data, seed theorems known at compile time |
| `P4RA_BOOTSTRAP` (6 entries) | `p4rakernel.rs` | Reference data, seed modules known at compile time |
| `DOMAIN_BOOTSTRAP` (6 entries) | `shared.rs` | Reference data, known domain keyword sets |

### Dispatch Architecture

**Dispatch is fully dynamic.** `run_theorem("collatz", "27")` does NOT execute a hardcoded
match arm, it looks up `"collatz"` in the runtime registry and calls
`(entry.runner)("27")`. The same pattern used by `catalog.rs` (`register_entry()`) is now
used by all three cr3echrz modules:

- `register_theorem(TheoremRegEntry { name: "new_thm", runner: my_fn, ... })`
- `register_p4ra_module(P4RARegEntry { name: "new_mod", runner: my_fn, ... })`
- `register_domain_keyword("new_kw", "new_domain")`

### Menu Integration

The cr3echrz engine is accessible via **F9** or the **`:9`** shortcut, or by typing
`cr3echrz` directly. The `CR3ECHRZ_MENU` provides 4 sub-commands (`cr3`, `p4ra`,
`cr3 --version`, `cr3 --list`). Commands `cr3` and `p4ra` autocomplete at top level
with tab completion.

## User Interface and Navigation (Phase 9)

The REPL has a full hierarchical navigation system (`menu.rs`) that organizes all
90+ commands into **10 discoverable categories**. No more memorizing command names.

### Menu Architecture

- **F1-F10 key interception**, `ESC [` sequences parsed from serial
- **`:1`–`:10` shortcuts**, direct jump to category without F-keys
- **Context stack**, up to 4 levels deep (`Exec → Crystal → decode`)
- **Breadcrumb prompt**, `⊙[Exec/Crystal]>` shows full navigation path
- **`..` / `back`**, pop one context level
- **Tab completion**, 10 categories + sub-commands complete at each level
- **`help <topic>`**, hierarchical help that searches main menu + all submenus
- **`?`**, ten-column menu bar rendered above prompt

## Cross-Dialect Navigation (Phase 8 + Diaschizics Bridge)

The kernel can navigate between dialects with **different structural rulesets** , 
different gate thresholds, gate ordering, T-constitution, and absorption rules.
The Crystal of Types (17.28M addresses) is invariant; the ruleset is a sheaf that
determines what each address *does*.

### The 8 Dialects

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

### The 11 Diaschizic Compounds

Each compound has a structural tuple, an IMASM program, and a steering profile.
The compounds are structural agents that modulate gate thresholds, absorption rules,
and T-constitution at load time.

### Cross-Dialect REPL Commands

```
ruleset show                    → Show active ruleset (canonical by default)
ruleset list                    → List all 8 dialects with G1/G2/G3 and T-constitution
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

### Reference Documents

| Document | Lines | Description |
|----------|:---:|-------------|
| `ig-docs/rebis-port/diaschizics_design.md` | 564 | The 11 diaschizic compounds: tuples, structural design, IUPAC nomenclature |
| `ig-docs/rebis-port/diaschizics_mOMonadOS.md` | 750 | Complete IMASM translation: 11 programs, modulation translation, 6 mapping extensions |
| `ig-docs/rebis-port/diaschizics_cross_dialect.md` | 623 | Cross-dialect ruleset navigation: 8 dialects, absorption rules, navigation protocols |
| `imscribing_grammar/navigators/ruleset_dialect.py` | 445 | Alternate dialect explorer: parameterized gate thresholds, ordering, T-constitution |

## Repository Structure

```
mOMonadOS/
  src/
    main.rs            ~2800L  bare-metal entry (_rust_start), BumpAllocator, REPL, command dispatch
    boot.rs              ~90L  PVH ELF note + 32→64 bootstrap (page tables, GDT, far jump)
    kernel.rs            576L  Frobenius tick loop, self-imscription, build_via_substrate() dispatch
    tokens.rs            637L  12 IMASM opcodes, free token-by-token composition
    sequence.rs         ~320L  FAMILY_TOKEN_AFFINITY matrix, MiniKernel, build_via_substrate()
    manus.rs             432L  Terminal HUD, B4 heatmap
    menu.rs              388L  Hierarchical menu, 10-category F-key bar, context stack, already_in guard
    catalog.rs           954L  Single source of truth, all structural data
    algebra.rs           303L  Meet/join/tensor lattice
    consciousness.rs     114L  C-score with gate evaluation
    belnap.rs            203L  Belnap FOUR, B4 memory
    crystal.rs           168L  Crystal encode/decode
    imas_ig.rs           450L  IMASM↔IG bridge; canonical IgPrim enum (49 variants)
    cl8nk.rs             787L  Full CLINK L8 formula navigator (catalog-native)
    serial.rs             96L  UART driver; inline asm inb/outb; no external crates
    interrupts.rs       ~190L  PIT timer, PIC remap, hand-rolled IDT; inline asm port I/O
    parasm.rs             ,   ParaASM VM: dialetheic alignment + measurement
    aleph.rs              ,   Aleph Hebrew glyph encoding
    belnap_shor.rs        ,   Belnap-Shor quantum pipeline (N=15, 21)
    para_rh.rs            ,   Riemann Hypothesis paraconsistent bridge
    para_ym.rs            ,   Yang-Mills mass gap paraconsistent bridge
    para_temporal.rs      ,   Temporal logic paraconsistent bridge
    para_category.rs      ,   Category theory paraconsistent bridge
    frob_verify.rs        ,   Frobenius harness verification
    dialect.rs           ,   Cross-dialect ruleset navigation
    cr3echrz/
      mod.rs              18L  Module root
      shared.rs          252L  Opcode registry, grammar mappings, canonical sequences, dynamic domains
      p3theorem.rs       700L  7-theorem unified engine with dynamic DYNAMIC_THEOREMS registry
      p4rakernel.rs      598L  6-module p4rakernel engine with dynamic DYNAMIC_P4RA registry
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
      fold.rs             ,   Protein fold classification
      materials_expanded.rs  17L  Expanded material type definitions
  momonados.ld                 Linker script (PVH note → boot32 → text → rodata → bss)
  build_bootimage.sh           ELF kernel builder (cargo build, single step)
  run.sh                       QEMU launcher (PVH direct ELF boot, no OVMF)
  Cargo.toml                   Rust project manifest, empty [dependencies]
  Makefile                     Build convenience targets
```

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)

No OVMF, no mtools, no disk image tools needed.  QEMU boots the bare ELF directly
via the PVH protocol (`XEN_ELFNOTE_PHYS32_ENTRY`).

## Build and Run

```sh
# Direct
cargo build --release --target x86_64-unknown-none
./run.sh          # boots release build in QEMU, serial on stdio

# Or via build script
bash build_bootimage.sh        # just compiles the ELF
bash run.sh release            # compiles if needed, then boots
```

The REPL runs over COM1 serial (stdio in QEMU). Quit with `quit`, `exit`, or `halt` , 
QEMU writes 0x10 to the `isa-debug-exit` port and exits cleanly.

## Target

`x86_64-unknown-none`, no OS, no std, **zero external crates**.
Static BSS bump allocator (4 MB).  Boot: PVH ELF note → 32-bit `_start` stub
(page tables + long-mode) → naked `_rust_start` (establishes RSP) → `kmain()`.
`Cargo.toml [dependencies]` is empty.

## License

Unlicense, public domain.
