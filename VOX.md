# V⊙x — control-flow closure auditor

Lifts a program's control flow to a twelve-glyph IMASM word and runs the
SIXTEEN_3 verdict over it. `src/vox.rs` holds the classifier and the verdict,
`src/vox_decode.rs` the x86-64 decoder. Reached from the REPL as `vox`.

```
vox verdict <word>    SIXTEEN_3 verdict over a glyph word
vox classify <mn>     which glyph an instruction lifts to
vox lift <path>       decode an ELF and lift its executable sections
```

## What the verdict reads

A word closes at **T**, carries an open fork at **B**, and runs clean and linear
at **N**. Only three glyphs move the verdict: `∈` opens a fork, `∋` fuses one,
`⊣` anchors. Everything else is carried but does not decide.

`⊢` and `∋` are not instructions. `⊢` opens the word; `∋` marks an address with
two or more predecessors. x86 has flat control flow, so both are recovered by
analysing the instruction stream rather than read off any single instruction.
wasm, which has structured control flow, has real opcodes for both.

## The decoder refuses rather than guesses

`decode_one` returns the instruction's length and enough of its shape to name a
mnemonic the classifier already understands. It does not reconstruct registers
or operands, because the glyph does not depend on them.

An opcode it does not know returns `None`. The walk stops there and reports how
far it got, so a partial lift always reads as partial:

```
stopped at +0x20da9a on an opcode the decoder does not know: 49 92 4c 87 e5 …
```

This is the load-bearing decision in the file. A length decoder that guesses a
width goes out of phase with the instruction stream and keeps producing
instructions — wrong ones. The word still verdicts, and the verdict is fiction.
Refusing makes the failure visible; guessing makes it invisible.

Each refusal names the bytes, so extending the table is mechanical. Coverage on
`/bin/ls` went 15% → 41% → 44% → 100% in four rounds, the stops being string
operations, the `0F BA` bit-test group, the whole x87 `D8..DF` escape, and
`xchg` with the accumulator.

## Coverage

100% of bytes decoded, no stops, on `/bin/true`, `/bin/ls`, `/bin/bash`,
`/usr/bin/git`, `/usr/bin/python3`, and on the kernel's own binary — 690,031
instructions across 3.2MB of its `.text`.

Verdicts distribute by shape rather than by size. The `_init` thunk closes at
**T**: it opens a fork and fuses it before the anchor. PLT stubs run **N**,
linear with no fork. A real `.text` read whole sits at **B**, a fork still open
at a terminal, which is what an entire program's control flow looks like when
read as one word.

## Two defects this surfaced

`parse_elf` read `sh_type` at section-header offset `+0`, which is `sh_name`.
Nothing ever matched `SHT_PROGBITS`, so it always returned zero executable
sections and the ELF path had never run. Both the 64-bit and 32-bit field
offsets were shifted by one field.

`compute_merges` looked up predecessors and addresses by linear scan, making the
lift quadratic. It completed on small objects and hung on a real binary.
Predecessor counts now use a `BTreeMap` and address membership a binary search
over the ascending address list; the same 517k-instruction section that hung now
lifts in under two seconds.

## What is not reconstructed

Operand detail. `vox lift` synthesises an operand string only where the glyph
depends on it: a direct branch target, a memory destination, or the absence of
an immediate target that makes a branch indirect. Anything finer would be a
disassembler, and the word does not read it.
