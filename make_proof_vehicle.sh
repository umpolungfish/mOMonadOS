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

# Copy Lean formalization from p4rakernel/p4ramill
mkdir -p "$OUT/lean"
cp -r ../p4rakernel/p4ramill/Imscribing "$OUT/lean/"
# Primitives lives under Imscribing/ and is carried by the line above. It used
# to sit beside it, and the stale second copy aborted the whole vehicle build
# with `cannot stat` — the script is `set -e`, so nothing downstream ran.
cp ../p4rakernel/p4ramill/lakefile.toml "$OUT/lean/"
cp ../p4rakernel/p4ramill/lean-toolchain "$OUT/lean/"
cp ../p4rakernel/p4ramill/lake-manifest.json "$OUT/lean/"
cp ../p4rakernel/p4ramill/*.lean "$OUT/lean/"

# The field data the proof rests on, from d12_sic_build. That repository is
# 161 MB; this is the subset a reader needs to check the claims, and it is the
# same set published under ig-docs-public/data/d2048_moduli.
#
# The fiducial travels as the exact extraction, not as numerical seeds: the
# transcripts below are emitted from the kernel itself at build time, and the
# derivation they follow is carried beside them. The .npz/.json optimisation
# output stays out, being a different object from what the extraction produces.
mkdir -p "$OUT/data"
D=../d12_sic_build
cp $D/tower_ramified_4.poly $D/tower_C4.poly $D/tower_C16.poly $D/tower_C32.poly \
   $D/tower_step3_C4.poly $D/tower_step4_C8.poly \
   $D/pin_sunit.txt $D/pin_sunit.gp $D/np_vals.txt \
   $D/ray_class_2048.txt $D/moduli_degrees.txt $D/conductor_convention.txt \
   $D/tower_step*.gp $D/d2048_raytower.gp "$OUT/data/" 2>/dev/null
cp ../ig-docs-public/data/d2048_moduli/README.md "$OUT/data/" 2>/dev/null

# The fiducial, exactly: the unit, its two reciprocal embeddings, the monomial
# by two independent routes, and each radical recovered from its own Gauss sum.
# Emitted by the kernel here so the file and the `d2048 exact` a reader types
# cannot disagree.
cp ../ig-docs/sic_fiducial_extraction_2part_bypass.md "$OUT/data/" 2>/dev/null
echo "  ↳ emitting kernel transcripts"
# The transcript starts at the prompt: forty lines of boot log ahead of the
# answer makes the file look like a log rather than the datum it carries.
transcript() { ./run_hosted_cmds.sh "$1" | sed -n '/^⊙> /,$p' | sed '/^⊙> quit/,$d'; }
transcript "d2048 exact" > "$OUT/data/d2048_exact_extraction.txt"
transcript "d2048 welch" > "$OUT/data/d2048_welch_overlaps.txt"
transcript "d2048 verify" > "$OUT/data/d2048_verify.txt"
# The manuscript the data backs.
mkdir -p "$OUT/paper"
mkdir -p "$OUT/paper/figs"
cp ../ig-docs/manuscripts3/sic_moduli_conductor.tex \
   ../ig-docs/manuscripts3/sic_moduli_conductor.pdf "$OUT/paper/" 2>/dev/null
cp ../ig-docs/manuscripts3/figs/filtration_2048.pdf \
   ../ig-docs/manuscripts3/figs/horn_side.pdf \
   ../ig-docs/manuscripts3/figs/horn_equator.pdf \
   ../ig-docs/manuscripts3/figs/horn_axial.pdf "$OUT/paper/figs/" 2>/dev/null

cat > "$OUT/run.sh" <<'RUNNER'
#!/usr/bin/env bash
# Boot mOMonadOS. Needs qemu-system-x86_64 and nothing else.
set -euo pipefail
cd "$(dirname "$0")"
exec qemu-system-x86_64 -kernel momonados -m 256M -display none \
  -no-reboot -device isa-debug-exit,iobase=0xf4,iosize=4 -serial stdio
RUNNER
chmod +x "$OUT/run.sh"

# A vehicle is a snapshot, and the last time this was forgotten a review
# reported the live kernel as broken on the strength of a two-month-old copy of
# imasm_core carried in here. The stamp goes in first, at the top of the README
# and in a file whose name cannot be skimmed past.
cat > "$OUT/NOT_CANONICAL.md" <<STAMP
# This directory is a snapshot, not the source

Built $(date -u +%Y-%m-%dT%H:%M:%SZ) from the tree at
/home/mrnob0dy666/imsgct/mOMonadOS.

It carries copies — imasm_core among them — that were current on the build date
and drift from the moment the source moves. Read it to RUN the kernel as it
stood. Do not read it to learn what the kernel does now, and never cite a path
inside it as canonical: cite the live tree.
STAMP

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

## Lean Formalization

The formalization files for the math and proofs (including the Stark units, ray class towers, and SIC-POVM identities) are located under the `lean/` directory. These are structured as a Lean 4 Lake package.

## What is in here

    momonados             the kernel, prebuilt; boots under qemu -kernel
    src/                  its full source
    imasm_core/           the IMASM token crate it builds against
    lean/                 the Lean 4 formalisation, p4ramill
    data/                 the field data the claims rest on, with a README
                          mapping each claim to its file
    paper/                the manuscript those claims are made in

Nothing here asks to be taken on trust. The kernel compiles from `src/`, the
Lean elaborates from `lean/`, and every field computation in the paper is
checkable against `data/`.

The fiducial is here as the exact extraction. `data/d2048_exact_extraction.txt`
is the kernel's own output for `d2048 exact`: the Stark unit
(2047+sqrt(4190205))/2 with its minimal polynomial, its two reciprocal real
embeddings, which are the two Galois parts, the monomial with exponents
[-1,3,2] computed by two independent routes, and each radical of the
discriminant recovered from its own Gauss sum, so the unit comes back from the
sums rather than from a seed. `data/sic_fiducial_extraction_2part_bypass.md` is
the derivation, and `data/d2048_welch_overlaps.txt` carries the equiangularity
check. Type `d2048 exact` in the booted kernel and the transcript reproduces.

The .npz and .json output of the numerical optimisation is not included. That
route reaches one attractor from every seed and caps near a fifth of the frame
potential, which is why the extraction goes around it; the files are a different
object from what is being shown here.

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
    insert <word>                  every one-glyph repair for an exposed word

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
