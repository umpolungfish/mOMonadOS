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
circuit rc [rna]    — sense, antisense, and all three frames
circuit retract     — μ∘δ=id, leg by leg
circuit one [word]  — x86 → IMASM → RNA → IMASM → x86
circuit two [rna]   — RNA → IMASM → x86 → IMASM → wasm → IMASM → AA
```

Both take an argument or fall back to the full alphabet.

## Strands, frames, and what the arcane sequences say

`circuit rc <rna>` prints the sense word, the antisense word, and all three
frames. `.` marks a codon carrying no glyph.

**The antisense of the canonical section is forced, and it is the control-flow
triple.** Reverse complement sends `(p1,p2,p3)` to `(comp p3, comp p2, comp p1)`,
so the antisense reads its FIRST position off the sense strand's wobble. With the
canonical wobble at `U`, and `comp(U) = A`, every antisense codon of a
section word begins with `A`. The only glyphs whose first position is `A` are
`∈`, `∋`, and `⊙`. So:

```
sense      ⊢⊣><⋈⊤∈∋⊙⊥⊞◻
antisense  ∈⊙.∋⊙.∋∈.∋∈⊙
```

Twelve glyphs collapse to three, and the three are fork, fuse, and
self-reference. This is not a coincidence to admire; it is forced by the choice
of canonical wobble, and a different wobble selects a different triple. Wobble
`G` would give every antisense codon a leading `C`, hence `<`, `⋈`, `⊤`.
Whatever the sense strand discards is exactly what the antisense strand puts in
a glyph-bearing position.

**Shine-Dalgarno and its recognizer are a ⊙/silence pair.**

```
AGGAGG…  sense ⊙⊙⊙⊙   antisense ....
CCUCCU…  sense ....    antisense ⊙⊙⊙⊙
```

The ribosome binding site carries nothing but `⊙` on the message strand and
nothing at all on the other; the anti-SD in the small subunit carries the mirror.
The recognition event is one strand speaking `⊙` into a strand that says nothing.

**The palindromic restriction site is a fixed point of the strand involution.**

```
GAAUUC…  sense ◻.◻.   antisense ◻.◻.
```

A true reverse-complement palindrome has an antisense sequence equal to its
sense sequence, so the words coincide. `GAAUUC` lands on `◻`, winding.

**Poly-A is the maximal silence.** Every frame, both strands, no glyph. `AAA` is
diagonal and so is its complement `UUU`, and the homopolymer is the one case
where both sides of the involution sit on the diagonal at once.

**The telomere repeat is silent in its own frame and speaks off it.**

```
UUAGGG…  frame 0 ......   frame 1 ⊣.⊣.⊣   frame 2 ⊙⊥⊙⊥⊙
```

Frame 2 alternates criticality and chirality without interruption. Nothing here
says which frame a non-coding repeat should be read in; the observation is that
the reading is frame-dependent and this repeat has a frame in which it is
entirely mute.

**The hammerhead ribozyme core carries a non-repeating word on both strands.**

```
CUGAUGAGUCCGUGAGGACGAAAC
  sense      <∈⊙.>.⊤.
  antisense  ⊥⊢⊢⊢⊤∋⋈⋈
```

## What is not settled

Whether the canonical wobble should be the unmarked base. It is `U` here because
`N` is the unmarked Belnap value and the section has to be picked somehow, but
nothing yet forces that choice. It is not inert, though: the canonical wobble
selects which triple the antisense strand speaks, so the choice is doing visible
work and deserves a reason rather than a default.
