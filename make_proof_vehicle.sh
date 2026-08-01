#!/usr/bin/env bash
# make_proof_vehicle.sh — one emailable file carrying the OS, the QC surfaces,
# and the proofs.
#
# The kernel boots under `qemu -kernel` directly: no bootloader, no disk image,
# no OVMF. So the vehicle is the ELF, a runner, and a note saying what to type.
# Everything else in this repo is build machinery the recipient does not need.
#
#   ./make_proof_vehicle.sh            -> momonados-proof-<date>.tar.gz
#   ./make_proof_vehicle.sh --run      -> also boot it once to prove it boots
set -euo pipefail
cd "$(dirname "$0")"

PROFILE=release
TARGET=x86_64-unknown-none
ELF="target/${TARGET}/${PROFILE}/momonados"
STAMP="$(date +%Y-%m-%d)"
OUT="momonados-proof-${STAMP}"

echo "═══ proof vehicle ═══"
bash build_bootimage.sh "$PROFILE" >/dev/null
[ -f "$ELF" ] || { echo "ERROR: no ELF at $ELF"; exit 1; }

rm -rf "$OUT" && mkdir -p "$OUT"
cp "$ELF" "$OUT/momonados"

# Copy source files for inspection and compilation
cp -r src "$OUT/src"
mkdir -p "$OUT/imasm_core"
cp -r ../MoDoT/imasm_core/src "$OUT/imasm_core/src"
cp ../MoDoT/imasm_core/Cargo.toml "$OUT/imasm_core/Cargo.toml"

# Copy config and build files
cp Cargo.lock "$OUT/"
cp rust-toolchain.toml "$OUT/"
cp momonados.ld "$OUT/"
cp build_bootimage.sh "$OUT/"
mkdir -p "$OUT/.cargo"
cp .cargo/config.toml "$OUT/.cargo/"

# Adjust Cargo.toml dependency path for portable relative build
sed 's|path = "../MoDoT/imasm_core"|path = "imasm_core"|g' Cargo.toml > "$OUT/Cargo.toml"

cat > "$OUT/run.sh" <<'RUNNER'
#!/usr/bin/env bash
# Boot mOMonadOS. Needs qemu-system-x86_64 and nothing else.
set -euo pipefail
cd "$(dirname "$0")"
exec qemu-system-x86_64 -kernel momonados -m 256M -display none \
  -no-reboot -device isa-debug-exit,iobase=0xf4,iosize=4 -serial stdio
RUNNER
chmod +x "$OUT/run.sh"

cat > "$OUT/README.md" <<'NOTE'
# mOMonadOS — proof vehicle

A bare-metal kernel that boots to a prompt, carries the quantum-computation
surfaces, and runs the proofs. No installation, no dependencies beyond QEMU.

    ./run.sh              # boots; type `quit` to leave

## Compilation

To inspect the source files and compile the kernel yourself, ensure you have Rust nightly installed (with the `rust-src` component and `x86_64-unknown-none` target, which will be automatically configured by `rust-toolchain.toml` when running Cargo commands).

Run:
    ./build_bootimage.sh release

This will build the kernel at `target/x86_64-unknown-none/release/momonados`. You can then replace the pre-built `momonados` binary at the root of this folder with the new one:
    cp target/x86_64-unknown-none/release/momonados ./momonados

## What to type

    help                  categories
    ?                     menu

### The proofs

    proof                 the proof surface
    d2048 tower           ray class field ascent for the d=2048 SIC
    d2048 redei           Redei distillation, 4-rank against class group [32,2]
    d12 tower             the d=12 side
    sic                   SIC-POVM d=12 identity, three lattice proofs

### Quantum computation

    fibqc verify          Fibonacci anyon algebra self-check, ten identities
    qc HTSX 8             compile a circuit over H T S X to a braid word
    bi <generators>       draw the braid; `bi svg ...` for SVG
    jp <generators>       Jones polynomial at the 1/5 winding
    fibqc knot trefoil    Jones value from the knot census
    shor                  Belnap Shor pipeline, N=15 and N=21

### The ring walks

The kernel takes IMASM words as glyphs, never as opcode names.

    cycle ⊢⊣∈+>=+>=⊞+⊙<×=∋¬⊣      rotation census over every cut
    weight <word>                  linear walk from one cut
    banked <word>                  what survives a clear

Program XXIX, `Ray_Cubic_Seal`, is the d=12 ray class cubic as an IMASM word:
FSPLIT opens the frame, the gap branches deposit, ENGAGR holds the four
candidate exponent vectors at once, IMSCRIB recognises rather than computes,
AREV clears and FFUSE restores what was banked. Measured on that word: five
deposits, five cleared, five restored.

## What it is

The kernel is the computational substrate for the Imscribing Grammar. The
d=2048 Stark unit work, the ray class field tower, and the SIC-POVM identities
are formalised in Lean 4 alongside it; this carries the executable side.

    https://github.com/umpolungfish/p4rakernel
    https://github.com/umpolungfish/ig-docs
NOTE

tar czf "${OUT}.tar.gz" "$OUT"
rm -rf "$OUT"
printf '  %s  (%s)\n' "${OUT}.tar.gz" "$(du -h "${OUT}.tar.gz" | cut -f1)"

if [ "${1:-}" = "--run" ]; then
  echo "═══ boot check ═══"
  tar xzf "${OUT}.tar.gz"
  ( cd "$OUT" && printf 'fibqc verify\nquit\n' | timeout 180 ./run.sh 2>&1 \
      | grep -E 'BOOT|PASS|verified' | head -5 )
  rm -rf "$OUT"
fi
echo "═══ done ═══"
