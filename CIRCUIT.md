# Circuit — substrate round trips through IMASM

Two circuits, both routed through the twelve-glyph alphabet:

```
x86 → IMASM → RNA → IMASM → x86
RNA → IMASM → x86 → IMASM → wasm → IMASM → AA
```

`src/circuit.rs`, reached from the REPL as `circuit`.

## What is being claimed

Not that a binary returns byte-identical. It cannot. Every substrate leg is
many-to-one: `vox`'s classifier ends in a catch-all, so distinct instructions
carry the same glyph and the map has a fiber and no inverse. Assume identity on
the outer composite and the degeneracy count refutes it directly, sixty-four
codons against twelve glyphs.

What closes is the word. Each leg is a retraction, `μ∘δ = id` on glyphs, while
`δ∘μ` is idempotent rather than identity. The identity holds exactly on the
image of `δ` — the canonical section — and nowhere else. That is the whole
result, and both circuits are instruments for reading it.

## The RNA leg is enumeration, not assignment

Four bases give sixteen ordered pairs at the first two codon positions. Four are
diagonal, twelve are not, and twelve is the size of the alphabet. So the
off-diagonal pairs **are** the glyphs and nothing was chosen. The third position
is the wobble and carries no glyph, which is the same statement `rebis/codon.rs`
already makes about the exact stratum.

The diagonal codons are the part of codon space the alphabet does not reach.
They carry no glyph and cannot enter a circuit.

## The two machine substrates differ, and the difference is structural

```
RNA   μ∘δ=id on ⊢⊣><⋈⊤∈∋⊙⊥⊞◻
x86   μ∘δ=id on ⊣><⋈⊤∈⊙⊥⊞◻      not expressible ⊢∋
wasm  μ∘δ=id on ⊢⊣><⋈⊤∈∋⊙⊥⊞◻
```

wasm carries structured control flow, so `block` and `end` are real opcodes and
all twelve glyphs have a representative. x86 is flat: `⊢` opens a word and `∋`
marks a merge, and neither is an instruction. Both are recovered by analysis of
the instruction stream rather than read off any single instruction, which is
what the lifter in `vox.rs` does when it inserts `⊢` at the start and `∋` at
every address with two or more predecessors.

So x86 realizes ten of twelve directly and re-derives the other two. The word
still closes, because re-derivation puts them back where they were.

## Circuit one closes

```
in   ⊢⊣><⋈⊤∈∋⊙⊥⊞◻
rna  UCUUAUUGUCUUCAUCGUAUUACUAGUGUUGCUGAU
out  ⊢⊣><⋈⊤∈∋⊙⊥⊞◻
```

## Circuit two closes on the section and fails off it

On the canonical section, direct translation and the routed chain agree exactly:
the detour through two machine substrates is invisible.

On arbitrary RNA it is not invisible, and the way it fails is the point:

```
AUG  ∈  jne  if  Ile   off-section: AUU is the canonical codon
GCC  ⊞  xor  i32.add  Ala   off-section: GCU is the canonical codon
UGC  >  call  call  Cys   off-section: UGU is the canonical codon
ACG  ∋  —  end  Thr   off-section: ACU is the canonical codon

direct  Met-Ala-His-Cys-Thr
routed  Ile-Ala-His-Cys-Thr
```

Four codons sat off the section and `δ∘μ` moved all four, but only one changed
the protein. `GCC → GCU` stays Ala, `UGC → UGU` stays Cys, `ACG → ACU` stays
Thr. `AUG → AUU` goes Met to Ile.

The circuit is invisible exactly where the genetic code's own third-position
degeneracy absorbs the move, and visible exactly where that degeneracy breaks.
Met is a singleton box, so its third position carries information, and that
information is what the wobble discards. The routed protein and the direct
protein differ at precisely the codons where the code itself refuses to be
degenerate.

## Commands

```
circuit table       — every glyph across every substrate
circuit retract     — μ∘δ=id, leg by leg
circuit one [word]  — x86 → IMASM → RNA → IMASM → x86
circuit two [rna]   — RNA → IMASM → x86 → IMASM → wasm → IMASM → AA
```

Both take an argument or fall back to the full alphabet.

## What is not settled

Whether the canonical wobble should be the unmarked base. It is `U` here because
`N` is the unmarked Belnap value and the section has to be picked somehow, but
nothing yet forces that choice, and a different section would move which codons
sit off it without changing any of the structure above.
