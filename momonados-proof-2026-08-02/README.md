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

The numerical fiducials from the working repository are deliberately absent.
`d2048 next` records the numerical seeds as dead at residual 3.87e-3 and says
plainly not to polish the fiducial; they are not part of what is proved, and
including them would invite a reader to treat them as evidence.

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
