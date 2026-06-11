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
(imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) are now live in the kernel, adding
1,987 lines of verified grammar infrastructure. The kernel now runs 32 unit tests across
all grammar modules and supports 10 new REPL commands.

### Core modules

| Module | Source | Role |
|---|---|---|
| `belnap.rs` (204L) | native | Belnap FOUR truth values (N/T/F/B), 4096-cell B4 memory, 256-deep stack, 8 registers. Extended with `band`, `bor`, `bnot`, `dialetheic`, `designated`, `approx_le`, `to_wh2`, `from_wh2`. |
| `tokens.rs` (360L) | native | 12 IMASM opcodes across 4 families; 12 canonical (I–XII), 4 continuous (XIII–XVI), 3 novel (XVII–XIX). |
| `crystal.rs` (162L) | native | 17.28M-address encode/decode; `CrystalStore` (64 entries, fixed-capacity). |
| `kernel.rs` (542L) | native | Frobenius tick loop; `self_imscribe()`; `dynamic_imscribe()`; tier promotion $O_0$ → $O_1$ → $O_2$ → $O_\infty$. Now wired to `FrobeniusHarness`. |
| `serial.rs` (96L) | native | 16550A UART COM1, 115200 8N1; `sprint!`/`sprintln!`; blocking line input. |
| `interrupts.rs` (177L) | native | PIT 100Hz timer, PIC remap, double-fault handler, escape-key detection. |
| `manus.rs` (432L) | native | Terminal HUD / live display, token graph, B4 memory heatmap, ANSI rendering. |
| `main.rs` (913L) | native | UEFI entry, heap init, serial REPL, command dispatch, history. |

### Phase 1 Grammar modules (new)

| Module | Lines | Source Repo | Role |
|---|---|---|---|
| `frob_verify.rs` | 478 | imasmic_core | FrobeniusHarness: 11 verifiers (hash, assertion, AST, IMASM, Belnap cycle, structural, tier promise, closure ratio, etc.), 16-entry history ring, closure ratio tracking. |
| `imas_ig.rs` | 518 | IMSCRIBr | `IgPrim` enum (49 values across 12 families), `IgTuple`, IG classification of all 12 canonicals with crystal addresses. |
| `aleph.rs` | 123 | ALEPH_OS | 22 Hebrew letters mapped to IG primitives, `AlephWord` encoding, gematria computation. |
| `parasm.rs` | 794 | priests-engine | Full ParaASM VM: 19-instruction ISA, assembler, execution engine, dialetheic alignment, measurement algebra, 9 unit tests. |
| `belnap_shor.rs` | 331 | priests-engine | Belnap Shor pipeline: Hadamard, ModExp, 2:1 coherence cost ratio, SIC-POVM verification. |
| `para_rh.rs` | 124 | priests-engine | Riemann Hypothesis bridge: $\zeta(s)=\chi(s)\zeta(1-s)$ as Belnap negation, critical line as unique designated `bnot` fixed point. |
| `para_ym.rs` | 63 | priests-engine | Yang-Mills mass gap: N<T covering relation, BRST nilpotence ↔ Frobenius, Omega_Z gauge protection. |
| `para_temporal.rs` | 52 | priests-engine | Temporal logic: □, ◇, ○, U operators; B as temporal fixed point. |
| `para_category.rs` | 61 | priests-engine | Category theory: N=initial, T=terminal, B=zero, Frobenius algebra, dagger compact closed. |
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

| # | Name | Type | IgTuple |
|---|---|---|---|
| I | Dialetheic_Bootstrap | Canonical | ⟨𐑼·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩ |
| II | Void_Genesis | Canonical | ⟨𐑛·𐑡·𐑩·𐑗·𐑱·𐑘·𐑚·𐑝·𐑢·𐑓·𐑙·𐑷⟩ |
| III | Anchor_Protocol | Canonical | ⟨𐑨·𐑰·𐑑·𐑿·𐑞·𐑤·𐑔·𐑜·𐑮·𐑒·𐑕·𐑴⟩ |
| IV | Dual_Bootstrap | Canonical | ⟨𐑼·𐑥·𐑽·𐑬·𐑐·𐑪·𐑲·𐑵·𐑻·𐑖·𐑳·𐑟⟩ |
| V | Linear_Chain | Canonical | ⟨𐑨·𐑡·𐑑·𐑗·𐑱·𐑧·𐑚·𐑠·𐑢·𐑒·𐑙·𐑷⟩ |
| VI | Empty_Bootstrap | Canonical | ⟨𐑛·𐑰·𐑩·𐑗·𐑱·𐑪·𐑚·𐑝·𐑢·𐑓·𐑙·𐑷⟩ |
| VII | Parakernel | Canonical | ⟨𐑼·𐑸·𐑾·𐑬·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑴⟩ |
| VIII | Frobenius_Kernel | Canonical | ⟨𐑦·𐑸·𐑾·𐑹·𐑐·𐑧·𐑲·𐑠·⊙·𐑫·𐑳·𐑭⟩ |
| IX | Chiral_Pairs | Canonical | ⟨𐑨·𐑥·𐑽·𐑿·𐑞·𐑤·𐑔·𐑜·𐑮·𐑖·𐑕·𐑴⟩ |
| X | Truth_Machine | Canonical | ⟨𐑨·𐑡·𐑑·𐑗·𐑱·𐑘·𐑚·𐑝·𐑢·𐑒·𐑕·𐑷⟩ |
| XI | Eternal_Return | Canonical | ⟨𐑼·𐑸·𐑽·𐑬·𐑐·𐑧·𐑲·𐑠·𐑮·𐑫·𐑳·𐑭⟩ |
| XII | ROM_Burn | Canonical | ⟨𐑦·𐑥·𐑾·𐑹·𐑐·𐑪·𐑲·𐑵·⊙·𐑫·𐑙·𐑭⟩ |

| XIII | Heartbeat | Continuous | ◊ pulse |
| XIV | Tier_Climber | Continuous | O₀→O₁ promotion |
| XV | Frobenius_Oscillator | Continuous | μ∘δ oscillation |
| XVI | Paradox_Daemon | Continuous | B-stabilized paradox |

| XVII | Nested_Fork_Labyrinth | Novel | deep fork nesting |
| XVIII | Terminal_Sink_Protocol | Novel | sink-node detection |
| XIX | Mirrorgram | Novel | self-reflective structure |

See [NOVEL_PROGRAMS.md](NOVEL_PROGRAMS.md) for details on the novel programs.

## REPL commands

### Core commands

| Command | Description |
|---|---|
| `tick [N]` | Run N manual ticks (default 1) |
| `run [N]` | Run N ticks; no arg = continuous (ESC to stop) |
| `watch [N]` | Live terminal HUD, refresh every N ticks (ESC to stop) |
| `graph` | ASCII-art token graph with nesting |
| `heatmap [start] [n]` | B4 memory heatmap with color blocks |
| `timer [N]` | Run N ticks, one per PIT interrupt (ESC to stop) |
| `boot canonical <idx>` | Load canonical + run continuously |
| `boot continuous <idx>` | Load continuous program + run continuously |
| `novel <1-3>` | Load novel program (XVII–XIX) |
| `status` | Kernel status (tick, tier, IP, stack, fork, frob, registers) |
| `program` | Show loaded program + fork depth |
| `snapshot` | Structural snapshot (sig, tier, period, dialetheia, frob_ord) |
| `canonical <I-XII>` | Load canonical program |
| `continuous <1-4>` | Load continuous program |
| `list` | List all programs |
| `crystal <addr>` | Decode crystal address |
| `crystal store <n> [d]` | Store entry in crystal filesystem |
| `crystal name <n>` | Retrieve by name |
| `crystal find` | List stored entries |
| `memory [start] [n]` | Dump B4 memory |
| `registers` | Show R0-R7 |
| `stack` | Stack depth |
| `halt/quit/exit` | Exit |

### Grammar commands (Phase 1)

| Command | Description | Module |
|---|---|---|
| `frob` | FrobeniusHarness status: total checks, closed, open, closure ratio, history ring | `frob_verify.rs` |
| `ig` | Full IG classification of loaded program: 12-primitive tuple + crystal address | `imas_ig.rs` |
| `classify` | List all 12 canonicals with their IgTuple + crystal address | `imas_ig.rs` |
| `aleph <word>` | Encode Hebrew word: letter-by-letter IG primitive mapping + gematria | `aleph.rs` |
| `psm <code>` | ParaASM: assemble + execute (19-instruction ISA, dialetheic alignment, measurement) | `parasm.rs` |
| `shor` | Belnap Shor pipeline: Hadamard → ModExp → SIC-POVM → coherence cost analysis | `belnap_shor.rs` |
| `rh` | Riemann Hypothesis bridge: ζ(s)=χ(s)ζ(1-s) as Belnap negation, critical line as unique bnot fixed point | `para_rh.rs` |
| `ym` | Yang-Mills mass gap: N<T covering, BRST nilpotence ↔ Frobenius, Ω gauge protection | `para_ym.rs` |
| `temp` | Temporal logic operators: □ (always), ◇ (eventually), ○ (next), U (until); B as temporal fixed point | `para_temporal.rs` |
| `cat` | Category theory: N=initial, T=terminal, B=zero object, Frobenius algebra, dagger compact closed | `para_category.rs` |
| `algebra <dist\|meet\|join\|tensor>` | IG lattice algebra: Hamming / weighted distance, meet, join, tensor with ZFC baseline | `algebra.rs` |
| `cl8nk <promotions\|entry>` | CL8NK navigator: ZFC→ZFCₜ→ZFCfe→CLINK L8 ladder, 6 promotion channels, entry lookup (zfc, zfc_t, clink_l8, einstein, IUG, etc.) | `cl8nk.rs` |
| `cscore` | Consciousness score: dual-gate (⊙ + K≤𐑧) evaluation with per-component breakdown | `consciousness.rs` |
## Project structure

```
mOMonadOS/
├── src/
│   ├── main.rs              UEFI entry, heap init, serial REPL, command dispatch, history
│   ├── kernel.rs            Frobenius tick loop, self_imscribe(), dynamic_imscribe(), tier promotion
│   ├── tokens.rs            Token enum, Program, 12 canonicals, 4 continuous, 3 novel, signature(), period()
│   ├── belnap.rs            B4, B4Memory (4096 cells), B4Stack (256 deep), B4Registers (8)
│   ├── crystal.rs           encode/decode, indices_from_snapshot(), CrystalStore (64 entries)
│   ├── serial.rs            UART driver, sprint!/sprintln!, read_byte()
│   ├── interrupts.rs        PIT 100Hz timer, PIC remap, IDT, double-fault, escape-key poll
│   ├── manus.rs             Live HUD display, token graph ASCII art, B4 memory heatmap
│   │
│   ├── frob_verify.rs       FrobeniusHarness: 11 verifiers, 16-entry history ring, closure ratio
│   ├── imas_ig.rs           IgPrim (49 values, 12 families), IgTuple, canonical classification
│   ├── aleph.rs             22 Hebrew letters → IG primitives, AlephWord, gematria
│   ├── parasm.rs            ParaASM VM: 19-instruction ISA, assembler, execution, 9 unit tests
│   ├── belnap_shor.rs       Belnap Shor pipeline: Hadamard, ModExp, SIC-POVM, coherence cost
│   ├── para_rh.rs           RH bridge: Belnap negation, critical line as bnot fixed point
│   ├── para_ym.rs           YM mass gap: N<T covering, BRST ↔ Frobenius
│   ├── para_temporal.rs     Temporal logic: □◇○U operators, B as temporal fixed point
│   └── para_category.rs     Category theory: N/T/B objects, Frobenius algebra, dagger compact
├── build_bootimage.sh       kernel ELF → BOOTX64.EFI → FAT32 disk image
├── run.sh                   QEMU launcher with OVMF auto-detection
├── Makefile
├── Cargo.toml
├── USER_GUIDE.md
├── NOVEL_PROGRAMS.md
└── README.md
```

**Total: 6,478 lines across 20 modules. 32 unit tests. Build: 0 errors.**

## Integration roadmap

| Phase | Status | Description |
|---|---|---|
| **Phase 1** | ✅ Complete | Core Grammar: frob_verify, imas_ig, aleph, parasm, belnap_shor, para_rh, para_ym, para_temporal, para_category |
| **Phase 2** | ✅ Complete | imscribing_grammar navigators (30+ modules): ZFCₜ, CL8NK, crystal navigator, domain navigators, promotion signatures, veracity probes |
| **Phase 3** | 🔜 Next | odot_operator agent loop, agents/ sub-system spawning |
| **Phase 4** | ⬜ Planned | gene_imscriber, lang/ (voynich, rohonc, linear_a, emerald-tablet), cetaceanspeak, synfin |
| **Phase 5** | ⬜ Planned | red-hot_rebis/clink/, rionrebis/ deep structure |
| **Phase 6** | ⬜ Planned | math/ (10 Lean repos): MillenniumAnkh, BealProof, hecke-landau, solitary_10, etc. |
| **Phase 7** | ⬜ Planned | ob3ect pipeline, catalog compilation, ZENODO publications |

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

## Grammar repos (upstream)

mOMonadOS integrates modules from the Imscribing Grammar ecosystem under `/home/mrnob0dy666/imsgct/`:

| Repo | Type | Integrated modules |
|---|---|---|
| **imasmic_core** | Python pkg | Token/Family enums, CanonicalArrangements, FrobeniusHarness → `frob_verify.rs` |
| **IMSCRIBr** | Python pkg | IgPrim, IgTuple, classification → `imas_ig.rs` |
| **ALEPH_OS** | Python pkg | Hebrew letter encoding, gematria → `aleph.rs` |
| **priests-engine** | Python pkg | ParaASM VM, Belnap Shor, RH/YM/Temporal/Category bridges → `parasm.rs`, `belnap_shor.rs`, `para_*.rs` |
| **imscribing_grammar** | Hub | IG_catalog.json (2256+ entries), Lean formalizations → Phase 2 |
| **odot_operator** | Python pkg | ⊙ operator agent loop → Phase 3 |
| **ob3ect** | Pipeline | Self-imscribing ob3ect generator → Phase 7 |
| **gene_imscriber** | Python pkg | Genetic imscription → Phase 4 |
| **lang/** | Sub-repos | voynich, rohonc, linear_a, emerald-tablet → Phase 4 |
| **synfin** | Docs | IMSCRIPTIONICON series → Phase 4 |
| **math/** | Lean repos | MillenniumAnkh, BealProof, hecke-landau, solitary_10 → Phase 6 |

## Target

`x86_64-unknown-none` — no OS, no std. Heap via `linked_list_allocator` over UEFI physical memory.
Boot via `bootloader_api` 0.11 (same as exOS).

## License

Unlicense — public domain.
