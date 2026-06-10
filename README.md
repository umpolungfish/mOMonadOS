# m$\odot$MonadOS

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop — every tick is a structural self-verification.

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
THINK → ACT → OBSERVE → UPDATE cycle driven by the 12-opcode IMASM instruction set.
Every execution state is a point in the Crystal of Types — a 17,280,000-address structural
type space derived from the 12 IG primitives. Storage is navigated by structural address,
not by path.

**Core components:**

| Module | Role |
|---|---|
| `belnap` | Belnap FOUR truth values (N/T/F/B), 4096-cell B4 memory, stack, registers |
| `tokens` | 12 IMASM opcodes across 4 families; 12 canonical programs (I–XII) |
| `crystal` | 17.28M-address encode/decode; `CrystalStore` (64 entries, fixed-capacity) |
| `kernel` | Frobenius tick loop; `self_imscribe()`; tier promotion $O_0$→$O_1$→$O_2$→$O_\infty$ |
| `serial` | 16550A UART COM1, 115200 8N1; `sprint!`/`sprintln!`; blocking line input |

**IMASM families:**

| Family | Opcodes |
|---|---|
| Logical | VINIT TANCH AFWD AREV CLINK ISCRIB |
| Frobenius | FSPLIT FFUSE |
| Dialetheia | EVALT EVALF ENGAGR |
| Linear | IFIX |

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)
- OVMF firmware (`sudo apt install ovmf`)
- mtools for disk images (`sudo apt install mtools`)

## Build and run

```sh
# Install momos launcher (first time only)
# ~/.local/bin must be on PATH

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

## Project structure

```
mOMonadOS/
├── src/
│   ├── main.rs       UEFI entry, heap init, serial REPL
│   ├── kernel.rs     Frobenius tick loop, self_imscribe(), tier promotion
│   ├── tokens.rs     Token enum, Program, 12 canonicals, signature(), period()
│   ├── belnap.rs     B4, B4Memory, B4Stack, B4Registers
│   ├── crystal.rs    encode/decode, indices_from_snapshot(), CrystalStore
│   └── serial.rs     UART driver, sprint!/sprintln!, read_byte()
├── build_bootimage.sh  kernel ELF → BOOTX64.EFI → FAT32 disk image
├── run.sh              QEMU launcher with OVMF auto-detection
├── Makefile
└── Cargo.toml
```

## Target

`x86_64-unknown-none` — no OS, no std. Heap via `linked_list_allocator` over UEFI physical memory.
Boot via `bootloader_api` 0.11 (same as exOS).

## License

Unlicense — public domain.
