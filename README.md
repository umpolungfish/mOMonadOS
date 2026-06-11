# $m\odot^2$

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop — every tick is a structural self-verification.

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
`THINK` → `ACT` → `OBSERVE` → `UPDATE` cycle driven by the 12-opcode IMASM instruction set.
Every execution state is a point in the Crystal of Types — a 17,280,000-address structural
type space derived from the 12 IG primitives. Storage is navigated by structural address,
not by path.

**Core components:**

| Module | Role |
|---|---|
| `belnap` | Belnap FOUR truth values (N/T/F/B), 4096-cell B4 memory, 256-deep stack, 8 registers |
| `tokens` | 12 IMASM opcodes across 4 families; 12 canonical (I–XII), 4 continuous (XIII–XVI), 3 novel (XVII–XIX) |
| `crystal` | 17.28M-address encode/decode; `CrystalStore` (64 entries, fixed-capacity) |
| `kernel` | Frobenius tick loop; `self_imscribe()`; `dynamic_imscribe()`; tier promotion $O_0$ → $O_1$ → $O_2$ → $O_\infty$ |
| `serial` | 16550A UART COM1, 115200 8N1; `sprint!`/`sprintln!`; blocking line input |
| `interrupts` | PIT 100Hz timer, PIC remap, double-fault handler, escape-key detection |
| `manus` | Terminal HUD / live display, token graph, B4 memory heatmap, ANSI rendering |

**IMASM families:**

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

| # | Name | Type |
|---|---|---|
| I | Dialetheic_Bootstrap | Canonical |
| II | Void_Genesis | Canonical |
| III | Anchor_Protocol | Canonical |
| IV | Dual_Bootstrap | Canonical |
| V | Linear_Chain | Canonical |
| VI | Empty_Bootstrap | Canonical |
| VII | Parakernel | Canonical |
| VIII | Frobenius_Kernel | Canonical |
| IX | Chiral_Pairs | Canonical |
| X | Truth_Machine | Canonical |
| XI | Eternal_Return | Canonical |
| XII | ROM_Burn | Canonical |
| XIII | Heartbeat | Continuous |
| XIV | Tier_Climber | Continuous |
| XV | Frobenius_Oscillator | Continuous |
| XVI | Paradox_Daemon | Continuous |
| XVII | Nested_Fork_Labyrinth | Novel |
| XVIII | Terminal_Sink_Protocol | Novel |
| XIX | Mirrorgram | Novel |

See [NOVEL_PROGRAMS.md](NOVEL_PROGRAMS.md) for details on the novel programs.

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

The REPL runs over COM1 serial (stdio in QEMU). Quit with `halt` or Ctrl-A X.

## REPL commands

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
| `snapshot` | Structural snapshot (sig, tier, period, dialetheia — now dynamic with b_live_ticks) |
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

Full user guide: [USER_GUIDE.md](USER_GUIDE.md)

## Project structure

```
mOMonadOS/
├── src/
│   ├── main.rs        UEFI entry, heap init, serial REPL, history
│   ├── kernel.rs      Frobenius tick loop, self_imscribe(), dynamic_imscribe(), tier promotion
│   ├── tokens.rs      Token enum, Program, 12 canonicals, 4 continuous, 3 novel, signature(), period()
│   ├── belnap.rs      B4, B4Memory (4096 cells), B4Stack (256 deep), B4Registers (8)
│   ├── crystal.rs     encode/decode, indices_from_snapshot(), CrystalStore (64 entries)
│   ├── serial.rs      UART driver, sprint!/sprintln!, read_byte()
│   ├── interrupts.rs  PIT 100Hz timer, PIC remap, IDT, double-fault, escape-key poll
│   └── manus.rs       Live HUD display, token graph ASCII art, B4 memory heatmap
├── build_bootimage.sh   kernel ELF → BOOTX64.EFI → FAT32 disk image
├── run.sh               QEMU launcher with OVMF auto-detection
├── Makefile
├── Cargo.toml
├── USER_GUIDE.md
├── NOVEL_PROGRAMS.md
└── REVIEW.md
```

## Target

`x86_64-unknown-none` — no OS, no std. Heap via `linked_list_allocator` over UEFI physical memory.
Boot via `bootloader_api` 0.11 (same as exOS).

## License

Unlicense — public domain.
