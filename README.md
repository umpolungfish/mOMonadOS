# $m\odot^2$

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop — every tick is a structural self-verification.

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
`THINK` → `ACT` → `OBSERVE` → `UPDATE` cycle driven by the 12-opcode IMASM instruction set.
Every execution state is a point in the Crystal of Types — a 17,280,000-address structural
type space derived from the 12 IG primitives. Storage is navigated by structural address,
not by path.

**Phase 1 Grammar Integration** — complete. Nine modules from four upstream Grammar repos
(imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) are now live in the kernel.

**Phase 2 Zero-Hardcode** — complete. `catalog.rs` (814L) is the single source of truth for
ALL structural data. No hardcoded `IgTuple { ... }` constants, no hardcoded ordinal arrays,
no hardcoded glyph strings, no hardcoded promotion gaps, no hardcoded score match-arms
exist outside `catalog.rs`. Six modules were refactored to delegate to the catalog:
`cl8nk.rs` (357→196L), `algebra.rs` (385→302L), `consciousness.rs` (210→113L),
`imas_ig.rs` (517→403L), `crystal.rs` (162→167L), and `main.rs`. The catalog is
runtime-extensible via `register_entry()` — new systems can be added dynamically without
touching any source file.

**Phase 5 Red-Hot Rebis** — complete. All 17 modules from `red-hot_rebis/` ported to
`no_std` Rust and wired into the REPL. The full p4ra paraconsistent kernel — genetic code
B₄ lattice, 7-stage Frobenius-verified translation pipeline, CLU power-law clustering,
exotic hadron Belnap analysis, PDB structure validation, antibody CDR design, IG material
forge, biological simulation, and therapeutic design — now runs directly from the bare-metal
kernel.

The kernel now runs **50 unit tests** across all grammar modules and supports **32+ REPL commands** spanning grammar operations and rebis biological/chemical computation.

### Core modules

| Module | Source | Role |
|---|---|---|
| `belnap.rs` (203L) | native | Belnap FOUR truth values (N/T/F/B), 4096-cell B4 memory, 256-deep stack, 8 registers. Extended with `band`, `bor`, `bnot`, `dialetheic`, `designated`, `approx_le`, `to_wh2`, `from_wh2`. |
| `tokens.rs` (637L) | native | 12 IMASM opcodes across 4 families; 12 canonical (I–XII), 4 continuous (XIII–XVI), 3 novel (XVII–XIX), 9 shunted (XX–XXVIII). |
| `crystal.rs` (167L) | native | 17.28M-address encode/decode; `CrystalStore` (64 entries, fixed-capacity). |
| `kernel.rs` (542L) | native | Frobenius tick loop; `self_imscribe()`; `dynamic_imscribe()`; tier promotion $O_0$ → $O_1$ → $O_2$ → $O_\infty$. Now wired to `FrobeniusHarness`. |
| `serial.rs` (96L) | native | 16550A UART COM1, 115200 8N1; `sprint!`/`sprintln!`; blocking line input. |
| `interrupts.rs` (177L) | native | PIT 100Hz timer, PIC remap, double-fault handler, escape-key detection. |
| `manus.rs` (432L) | native | Terminal HUD / live display, token graph, B4 memory heatmap, ANSI rendering. |
| `main.rs` (1095L) | native | UEFI entry, heap init, serial REPL, command dispatch, history. |

### Phase 1 Grammar modules

| Module | Lines | Source Repo | Role |
|---|---|---|---|
| `frob_verify.rs` | 478 | imasmic_core | FrobeniusHarness: 11 verifiers (hash, assertion, AST, IMASM, Belnap cycle, structural, tier promise, closure ratio, etc.), 16-entry history ring, closure ratio tracking. |
| `imas_ig.rs` | 403 | IMSCRIBr | `IgPrim` enum (49 values across 12 families), `IgTuple`, IG classification of all 12 canonicals with crystal addresses. Delegates glyph/short/crystal-indices to catalog. |
| `aleph.rs` | 123 | ALEPH_OS | 22 Hebrew letters mapped to IG primitives, `AlephWord` encoding, gematria computation. |
| `parasm.rs` | 793 | priests-engine | Full ParaASM VM: 19-instruction ISA, assembler, execution engine, dialetheic alignment, measurement algebra, 9 unit tests. |
| `belnap_shor.rs` | 331 | priests-engine | Belnap Shor pipeline: Hadamard, ModExp, 2:1 coherence cost ratio, SIC-POVM verification. |
| `para_rh.rs` | 124 | priests-engine | Riemann Hypothesis bridge: $\zeta(s)=\chi(s)\zeta(1-s)$ as Belnap negation, critical line as unique designated `bnot` fixed point. |
| `para_ym.rs` | 63 | priests-engine | Yang-Mills mass gap: N<T covering relation, BRST nilpotence ↔ Frobenius, Omega_Z gauge protection. |
| `para_temporal.rs` | 52 | priests-engine | Temporal logic: □, ◇, ○, U operators; B as temporal fixed point. |
| `para_category.rs` | 61 | priests-engine | Category theory: N=initial, T=terminal, B=zero, Frobenius algebra, dagger compact closed. |

### Phase 2 Catalog & Algebra modules

| Module | Lines | Role |
|---|---|---|
| `catalog.rs` | 814 | **Single Source of Truth** for ALL structural data. 13+ catalog entries (ZFC, ZFCₜ, ZFCfe, CLINK L8, TemporalMathematics, Schrödinger, HeatDiffusion, Navier-Stokes, WaveEquation, Einstein, IUG, O_∞ ideal, O₀ floor). 12 ordinal tables (D_ORD through OMEGA_ORD). 10 score functions computed as `ordinal_index / max_index`. 49 formula fragments. Distance weights with runtime reconfiguration. Promotion channels (6 ZFC→ZFCₜ + 2 CLINK transcendence) with ordinal gaps. Shavian glyphs and short names. Runtime registration via `register_entry()`. |
| `algebra.rs` | 302 | IG lattice algebra: Hamming / weighted distance, meet, join, tensor. All ordinals and weights sourced from catalog. 7 unit tests. |
| `cl8nk.rs` | 196 | CL8NK navigator: 4-stage ladder ZFC→ZFCₜ→ZFCfe→CLINK L8. All entry tuples and formulas delegated to catalog — zero hardcoded `IgTuple` constants. |
| `consciousness.rs` | 113 | Consciousness score: dual-gate (⊙ + K≤𐑧) evaluation. All 10 score functions computed from catalog ordinal positions — zero hardcoded match arms. 3 unit tests. |
### Phase 5 Red-Hot Rebis modules

All 17 modules ported from `red-hot_rebis/` to `no_std` Rust in `src/rebis/`. These are the p4ra paraconsistent kernel — genetic code, protein translation, hadron physics, materials science, biology simulation, and therapeutic design — running from the bare-metal kernel.

**Tier 1 — Genetic Core** (p4ra kernel)

| Module | Lines | Python Source | Role |
|---|---|---|---|
| `rebis/mod.rs` | ~60 | — | Shared types: 12 IG primitives enum, 20 amino acid codes, `RebisResult<T>` |
| `rebis/codon.rs` | ~130 | `genetics_b4.py`, `genetic_code.py` | 64-codon table, Belnap↔nucleotide mapping, Watson-Crick complement, Frobenius stratum classification |
| `rebis/genetics.rs` | ~160 | `genetics_b4.py` | B₄ lattice, codon meet/join/distance, 12 AA↔primitive bijection, **7-stage Frobenius verification** |
| `rebis/translate.rs` | ~90 | `gene_to_protein_pipeline.py` | DNA→mRNA→protein translation pipeline with Frobenius verification |
| `rebis/genetic_asm.rs` | ~130 | `genetic_asm.py`, `genetic_code.py` | Genetic ParaASM programs: translate_codon, b4_edit, stratum_classify; nucleotide↔B4 encoding |
| `rebis/genetic_tuples.rs` | ~820 | `genetic_tuples.py`, `gene_to_protein_pipeline.py` | All 12 primitive value types with glyph/as_str/ordinal; IGTuple struct; 7-stage pipeline (DNA→Transcription→Codon→Translation→Folding→Tertiary→Quaternary); AA activation table; crystal addresses |

**Tier 2 — Physics & Clustering**

| Module | Lines | Python Source | Role |
|---|---|---|---|
| `rebis/frob_filter.rs` | ~100 | `frobenius_filtration.py` | Frobenius filter (fsplit→ffuse check), power-law clustering verification |
| `rebis/clu.rs` | ~360 | `clu_power_law.py` | CLU power-law: CLU_DECIMAL/BINARY/NATURAL encodings; Point3D on (K×H×Ω) lattice; CLUWalk3D; avalanche P(S)∝S^(-3/2); MLE exponent estimator; PowerLawFit with Frobenius verification; no_std math (ln, exp, pow, sqrt via Taylor/Newton) |
| `rebis/hadron.rs` | ~180 | `hadron/quark/orbital_belnap.py` | Quark flavors, hadron Belnap states, proton/neutron/pion encoding, orbital Belnap analysis |
| `rebis/exotic_hadron.rs` | ~280 | `exotic_hadron_belnap.py` | GluonColor (8 types); Glueball depair/pair Frobenius; QColor (anti, join, join_all); Tetraquark depair/pair; Pentaquark structure |

**Tier 3 — Design & Simulation**

| Module | Lines | Python Source | Role |
|---|---|---|---|
| `rebis/serpent.rs` | ~140 | `serpent_rod.py` | 4 serpent motifs (Alpha/Beta/Omega/Phi), primitive signatures, chimera assembly |
| `rebis/pipeline.rs` | ~100 | `compute_promotions.py` | IG tuples, promotion computation, weighted distance, tier prediction |
| `rebis/pdb.rs` | ~270 | `pdb_validator.py` | THREE_TO_ONE/AA_TO_CODON tables; CAAtom parsing; contact extraction; sequence extraction; ValidationMetrics; Frobenius PDB verification |
| `rebis/antibody.rs` | ~340 | `antibody_designer.py` | complementary_primitive 12↔12 bijection; primitive↔AA mapping; EpitopeAnalysis; CDRDesign with design_cdr(); VH/VL_FRAMEWORK; design_full_antibody(); VIRAL_EPITOPES; complementarity_score |
| `rebis/materials.rs` | ~300 | `ig_material_forge.py`, `frobenius_metamaterial.py`, `ouroboric_alloy.py`, `thermal_rectifier.py`, `non_qubit_qc.py`, `gap_closure_module.py` | Primitive→material property maps; forge_material() from 12 glyphs; 6-rule consistency verifier; MetaCell coupling; OuroboricAlloy; ThermalRectifier; NonQubitQC; GapClosure |
| `rebis/biology.rs` | ~220 | `biology_sim.py`, `biology_sim_frobenius_exact.py`, `ouroboric_telomere.py` | CellState (Healthy/Senescent/Cancerous/Apoptotic ↔ B4); TissueGrid cellular automaton; Telomere dynamics; FrobeniusBioSim cycle |
| `rebis/therapeutics.rs` | ~210 | `frobenius_chemotherapeutic.py`, `neurotrophic_factor.py`, `ouroboric_pill_sim.py`, `quantum_biologic_prototype.py`, `universal_antidote_library.py` | Chemotherapeutic (⊙ gate via sub-nM Kd); NeurotrophicFactor; OuroboricPill (self-sensing); QuantumBiologic; UniversalAntidote |

### Rebis REPL commands

All 17 modules are accessible through `rebis` subcommands:

| Command | Module | What it does |
|---|---|---|
| `rebis codon <CODON\|all>` | `codon.rs` | Lookup 64-codon table, Belnap state, Watson-Crick complement, stratum |
| `rebis translate <DNA>` | `translate.rs` | DNA→mRNA→protein pipeline with Frobenius verification |
| `rebis frob walk\|verify` | `frob_filter.rs` | Frobenius filter walk or stratum verification |
| `rebis genetics <CODON>` | `genetics.rs` | B₄ lattice analysis: meet/join/distance, AA primitive, 7-stage verification |
| `rebis hadron` | `hadron.rs` | Hadron Belnap states: proton, neutron, pion |
| `rebis serpent` | `serpent.rs` | 4 serpent motifs with primitive signatures |
| `rebis pipeline <name>` | `pipeline.rs` | IG tuple lookup, promotion computation, tier prediction |
| `rebis strata` | `genetics.rs` | Frobenius stratum classification across 64 codons |
| `rebis asm [prog] [codon]` | `genetic_asm.rs` | Genetic ParaASM programs — list, translate codons→B4 |
| `rebis tuples <DNA>` | `genetic_tuples.rs` | 7-stage generative tuple pipeline with crystal addresses |
| `rebis clu walk\|verify\|avalanche` | `clu.rs` | CLU power-law clustering, avalanche P(S)∝S^(-3/2), Frobenius fit |
| `rebis exotic` | `exotic_hadron.rs` | Exotic hadron Frobenius verification — glueballs, tetraquarks, pentaquarks |
| `rebis pdb validate\|contacts\|seq` | `pdb.rs` | PDB structure validation — CA atoms, contacts, sequence extraction |
| `rebis antibody epitope\|design\|full\|viral` | `antibody.rs` | Antibody CDR design — epitope analysis, 12↔12 primitive complementarity |
| `rebis material forge\|alloy\|thermal\|qc\|gap` | `materials.rs` | IG material forge — OuroboricAlloy, ThermalRectifier, NonQubitQC, GapClosure |
| `rebis bio` | `biology.rs` | Biological simulation — TissueGrid cellular automaton, Telomere dynamics, FrobeniusBioSim |
| `rebis tx` | `therapeutics.rs` | Therapeutics — Chemotherapeutic (⊙ gate), OuroboricPill, UniversalAntidote |

### IMASM families
| Family | Opcodes |
|---|---|
| Logical | VINIT TANCH AFWD AREV CLINK IMSCRIB |
| Frobenius | FSPLIT FFUSE |
| Dialetheia | EVALT EVALF ENGAGR |
| Linear | IFIX |

Control flow is token-graph-native — no JNZ/JZ/YIELD/HALT opcodes:
- **FSPLIT/FFUSE** = fork/join (conditional branching)
- **EVALT/EVALF** = T-gate / F-gate (branch selection)
- **TANCH** at root depth = halt
- **Cyclic graph topology** (end wraps to start) = loop

## Program catalog

All 28 programs (12 canonical, 4 continuous, 3 novel, 9 shunted). Structured token sequences with crystal addresses, tier assignments, and Frobenius closure status. See `NOVEL_PROGRAMS.md` and `SHUNTED_PROGRAMS.md` for details.

## Source tree

```
src/
├── main.rs                UEFI entry, serial REPL, command dispatch (1095L)
├── kernel.rs              Frobenius tick loop, self-imscription, tier promotion (542L)
├── belnap.rs              Belnap FOUR, B4 memory, stack, registers (203L)
├── tokens.rs              IMASM opcodes: 12 canonical + 4 continuous + 3 novel + 9 shunted (637L)
├── crystal.rs             Crystal encode/decode, CrystalStore (167L)
├── serial.rs              UART driver (96L)
├── interrupts.rs          PIT timer, PIC, double-fault, escape detection (177L)
├── manus.rs               Terminal HUD, token graph, B4 heatmap (432L)
├── frob_verify.rs         FrobeniusHarness: 11 verifiers, history ring (478L)
├── imas_ig.rs             IgPrim, IgTuple, IG classification (403L)
├── aleph.rs               Hebrew letter encoding, gematria (123L)
├── parasm.rs              ParaASM VM: assembler, execution engine (793L)
├── belnap_shor.rs         Belnap Shor pipeline (331L)
├── para_rh.rs             RH bridge (124L)
├── para_ym.rs             YM mass gap bridge (63L)
├── para_temporal.rs       Temporal logic (52L)
├── para_category.rs       Category theory bridge (61L)
├── catalog.rs             Single Source of Truth: entries, ordinals, scores, formulas (814L)
├── algebra.rs             IG lattice algebra: distance, meet, join, tensor (302L)
├── cl8nk.rs               CL8NK navigator: 4-stage ladder (196L)
├── consciousness.rs       Consciousness score: dual-gate evaluation (113L)
├── rebis/
│   ├── mod.rs             Shared types: 12 primitives, 20 AAs, RebisResult (~60L)
│   ├── codon.rs           64-codon table, Belnap↔nucleotide, complement, strata (~130L)
│   ├── genetics.rs        B₄ lattice, meet/join/distance, 7-stage verification (~160L)
│   ├── translate.rs       DNA→mRNA→protein pipeline (~90L)
│   ├── frob_filter.rs     Frobenius filter, power-law clustering (~100L)
│   ├── hadron.rs          Quark flavors, hadron Belnap states (~180L)
│   ├── serpent.rs         4 serpent motifs, primitive signatures (~140L)
│   ├── pipeline.rs        IG tuples, promotions, tier prediction (~100L)
│   ├── genetic_asm.rs     Genetic ParaASM programs (~130L)
│   ├── genetic_tuples.rs  12 primitive value types, 7-stage pipeline, crystal addresses (~820L)
│   ├── clu.rs             CLU power-law, avalanche P(S)∝S^(-3/2), no_std math (~360L)
│   ├── exotic_hadron.rs   Glueballs, tetraquarks, pentaquarks (~280L)
│   ├── pdb.rs             PDB validation: CA atoms, contacts, sequences (~270L)
│   ├── antibody.rs        Antibody CDR design, 12↔12 complementarity (~340L)
│   ├── materials.rs       IG material forge, OuroboricAlloy, ThermalRectifier (~300L)
│   ├── biology.rs         TissueGrid, telomere dynamics, FrobeniusBioSim (~220L)
│   └── therapeutics.rs    Chemotherapeutic, OuroboricPill, UniversalAntidote (~210L)
├── build_bootimage.sh     kernel ELF → BOOTX64.EFI → FAT32 disk image
├── run.sh                 QEMU launcher with OVMF auto-detection
├── Makefile
├── Cargo.toml
├── USER_GUIDE.md
├── NOVEL_PROGRAMS.md
├── SHUNTED_PROGRAMS.md
└── README.md
```

**Total: ~10,840 lines across 39 modules. 50 unit tests. Build: 0 errors, 329 warnings. Zero hardcoded structural values — all tuples, formulas, scores, ordinals, weights, glyphs, and promotion data sourced dynamically from `catalog.rs`. All 17 red-hot_rebis modules ported to `no_std` Rust — the full p4ra paraconsistent kernel runs from the bare-metal kernel.**

## Zero-Hardcode Architecture

```
                 ┌─────────────────────────┐
                 │      catalog.rs          │
                 │  (Single Source of Truth) │
                 │  • entries + tuples       │
                 │  • ordinals + scores      │
                 │  • formulas + weights     │
                 │  • glyphs + promotions    │
                 └────┬──────┬──────┬───────┘
                      │      │      │
          ┌───────────┘      │      └───────────┐
          ▼                  ▼                  ▼
     cl8nk.rs          algebra.rs        consciousness.rs
    (lookup only)    (ord/weight refs)    (score refs)
          │                  │                  │
          ▼                  ▼                  ▼
     imas_ig.rs         kernel.rs          main.rs
   (glyph/short refs)  (snapshot→tuple)   (REPL commands)
```

Before the Phase 2 refactoring, six modules contained hardcoded structural data: `cl8nk.rs` had `ZFC_BASELINE`/`ZFC_T`/`ZFC_FE`/`CLINK_L8` tuple constants and `formula_fragment()` match arms; `algebra.rs` had `F_ORD`/`K_ORD`/`G_ORD`/`OMEGA_ORD`/`H_ORD` arrays and `DEFAULT_WEIGHTS`; `consciousness.rs` had 10 `score_*()` match-arm functions; `imas_ig.rs` had `IgPrim::glyph()` and `IgPrim::short()` with hardcoded strings; `crystal.rs` had a hardcoded `TOTAL` constant; `main.rs` had promotion data. All are now clean — the catalog is the single source, and all modules query it dynamically.

The catalog is runtime-extensible: `register_entry()` adds new systems without touching any source file. Aliases resolve to canonical names. Domain filtering and ordinal-based scoring mean new entries automatically participate in all structural computations.

## Integration roadmap

| Phase | Status | Description |
|---|---|---|
| **Phase 1** | ✅ Complete | Core Grammar: frob_verify, imas_ig, aleph, parasm, belnap_shor, para_rh, para_ym, para_temporal, para_category |
| **Phase 2** | ✅ Complete | Zero-Hardcode: `catalog.rs` as single source of truth. CL8NK 4-stage ladder, crystal navigator, domain navigators, promotion signatures, veracity probes, consciousness scoring. Six modules refactored (cl8nk −161L, algebra −83L, consciousness −97L, imas_ig −114L). All tuples, formulas, scores, ordinals, weights, glyphs, and promotion data sourced dynamically. Runtime-extensible catalog. |
| **Phase 3** | 🔜 Next | odot_operator agent loop, agents/ sub-system spawning |
| **Phase 4** | ⬜ Planned | gene_imscriber, lang/ (voynich, rohonc, linear_a, emerald-tablet), cetaceanspeak, synfin |
| **Phase 5** | ✅ Complete | Red-Hot Rebis → mOMonadOS: All 17 p4ra kernel modules ported to `no_std` Rust in `src/rebis/`. Genetic code B₄ lattice, 7-stage Frobenius-verified translation, CLU power-law clustering, exotic hadron Belnap analysis, PDB validation, antibody CDR design, IG material forge, biological simulation, and therapeutic design. 17 REPL subcommands wired. |
| **Phase 6** | ⬜ Planned | math/ (10 Lean repos): MillenniumAnkh, BealProof, hecke-landau, solitary_10, etc. |
| **Phase 7** | ⬜ Planned | ob3ect pipeline, catalog compilation, ZENODO publications |

## Grammar repos (upstream)

mOMonadOS integrates modules from the Imscribing Grammar ecosystem under `/home/mrnob0dy666/imsgct/`:

| Repo | Type | Integrated modules |
|---|---|---|
| **imasmic_core** | Python pkg | Token/Family enums, CanonicalArrangements, FrobeniusHarness → `frob_verify.rs` |
| **IMSCRIBr** | Python pkg | IgPrim, IgTuple, classification → `imas_ig.rs` |
| **ALEPH_OS** | Python pkg | Hebrew letter encoding, gematria → `aleph.rs` |
| **priests-engine** | Python pkg | ParaASM VM, Belnap Shor, RH/YM/Temporal/Category bridges → `parasm.rs`, `belnap_shor.rs`, `para_*.rs` |
| **red-hot_rebis** | Python pkg | p4ra paraconsistent kernel: genetic code, protein translation, CLU clustering, exotic hadrons, PDB validation, antibody design, materials forge, biology simulation, therapeutics → `src/rebis/` (17 modules) — **Phase 5 ✅** |
| **imscribing_grammar** | Hub | IG_catalog.json (2256+ entries), Lean formalizations → Phase 3+ |
| **odot_operator** | Python pkg | ⊙ operator agent loop → Phase 3 |
| **ob3ect** | Pipeline | Self-imscribing ob3ect generator → Phase 7 |
| **gene_imscriber** | Python pkg | Genetic imscription → Phase 4 |
| **lang/** | Sub-repos | voynich, rohonc, linear_a, emerald-tablet → Phase 4 |
| **synfin** | Docs | IMSCRIPTIONICON series → Phase 4 |
| **math/** | Lean repos | MillenniumAnkh, BealProof, hecke-landau, solitary_10 → Phase 6 |

## What's NOT ported (Tier 4 — requires Python runtime)

The following `red-hot_rebis/` subsystems remain as userspace Python modules that the kernel can structurally verify but cannot execute directly:

| Subsystem | Dependency | Modules |
|---|---|---|
| `ch3mpiler/` | RDKit | Retrosynthetic compiler, bond formation |
| `clink/` | Datasets, numpy | Bridging, chain processing, pipeline orchestration |
| `imas/` | Wiring, numpy | Arranger, Frobenius hunter, IG bridge |
| `imasm_iterator/` | Python runtime | 429M token arrangement → fingerprint classification |
| `pipeline/` | Python runtime | Auto imscriber, ob3ect imscriber, lift pipeline |
| `serpentrod/` | ML models | Protein v4/v5, stratified predictor |
| `cephalopod_design/` | BioPython, numpy | Organism designs, FASTA, PDB |
| `cat_allergy_design/` | BioPython | DARPin Fel d1 neutralizer |
| `shunt_portal_design/` | BioPython | Gaussia Luc, NanoLuc, Piezo1, iRFP713 constructs |
| `gr33ngroblin/` | BioPython | Plastic eater BPA |

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)
- OVMF firmware (`sudo apt install ovmf`)
- mtools for disk images (`sudo apt install mtools`)

## Build and run

```sh
# Install momos launcher (first time only)
# /home/mrnob0dy666/.local/bin must be on PATH

momos           # build release image + boot serial REPL in QEMU
momos build     # dev build only (fast, no image)
momos release   # release build only
momos image     # build release + FAT32 UEFI disk image
momos clean     # wipe build artifacts
```

Or with make:

```sh
make run        # image + serial REPL
make build      # dev build
make release    # release build
make image      # UEFI disk image
make clean
```

Or directly:

```sh
cargo build --release
bash build_bootimage.sh
bash run.sh --serial
```

The REPL runs over COM1 serial (stdio in QEMU). Quit with `quit`, `exit`, or `halt` — QEMU exits cleanly (no more Ctrl+C).

## Target

`x86_64-unknown-none` — no OS, no std. Heap via `linked_list_allocator` over UEFI physical memory.
Boot via `bootloader_api` 0.11 (same as exOS).

## License

Unlicense — public domain.
