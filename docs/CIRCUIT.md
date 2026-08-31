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

## The RNA leg is read off the code, not assigned

Nothing in the glyph map is chosen here. A codon translates to an amino acid
through the live table. An amino acid carries a primitive exactly when every one
of its codons sits in the split stratum — the twelve promoted amino acids, as
against the eight ground-layer ones. A primitive belongs to exactly one axis, and
the axis is the glyph.

```
⊢ Met   ⊣ Trp   > Cys   < Tyr   ⋈ Phe   ⊤ Ile
∈ His   ∋ Asn   ⊙ Gln   ⊥ Asp   ⊞ Lys   ⊡ Glu
```

This is the correspondence `GeneticCode.lean` states as `aaToPrimitive` and
`primitiveToAA`, with `promoted_card = 12`, `ground_promoted_disjoint` and
`twenty_eq_eight_plus_twelve` behind it. The eight ground-layer amino acids —
Leu, Pro, Arg, Thr, Ala, Ser, Val, Gly — carry no primitive and no glyph, and
neither do the stops. Roughly half of codon space is silent to the alphabet.

δ takes a glyph to the first codon in enumeration order that carries it, which is
a section picked by order rather than by taste:

```
AUG UGG UGU UAU UUU AUU CAU AAU CAA GAU AAA GAA
```

That string is a real RNA that spells the alphabet and translates to the twelve
primitives in order.

## The two codon→primitive derivations agree

`AminoAcid::to_primitive` in `src/rebis/mod.rs` derives a primitive value from
the codon box, and it lands on the same twelve axes as the correspondence above:
Tyr on `<`, His on `∈`, Asn on `∋`, with no amino acid doubled on `⊙`. `circuit
drift` checks the two derivations against each other and reports agreement.

## The two machine substrates differ, and the difference is structural

```
RNA   μ∘δ=id on ⊢⊣><⋈⊤∈∋⊙⊥⊞⊡
x86   μ∘δ=id on ⊣><⋈⊤∈⊙⊥⊞⊡      not expressible ⊢∋
wasm  μ∘δ=id on ⊢⊣><⋈⊤∈∋⊙⊥⊞⊡
```

wasm carries structured control flow, so `block` and `end` are real opcodes and
all twelve glyphs have a representative. x86 is flat: `⊢` opens a word and `∋`
marks a merge, and neither is an instruction. Both are recovered by analysis of
the instruction stream rather than read off any single instruction, which is
what the lifter in `vox.rs` does when it inserts `⊢` at the start and `∋` at
every address with two or more predecessors.

So x86 realizes ten of twelve directly and re-derives the other two. The word
still closes, because re-derivation puts them back where they were.

## Both circuits close on the section

```
in   ⊢⊣><⋈⊤∈∋⊙⊥⊞⊡
rna  AUGUGGUGUUAUUUUAUUCAUAAUCAAGAUAAAGAA
out  ⊢⊣><⋈⊤∈∋⊙⊥⊞⊡
```

```
direct  Met-Trp-Cys-Tyr-Phe-Ile-His-Asn-Gln-Asp-Lys-Glu
routed  Met-Trp-Cys-Tyr-Phe-Ile-His-Asn-Gln-Asp-Lys-Glu
```

The detour through two machine substrates is invisible on the section, and the
retraction `μ∘δ = id` holds on all twelve glyphs for RNA and for wasm.

Off the section it fails, and the mechanism is the one `SerpentRod.lean` now
states as `frobenius_serpent_rod_exact` and `frobenius_serpent_rod_fails_split`:
forgetting the third position is lossless on the exact stratum and lossy on the
split stratum, and the twelve primitives live entirely on the split stratum. The
Frobenius holds on the ground layer and fails exactly where the Grammar's
primitives are.

## Commands

```
circuit table       — every glyph across every substrate
circuit rc [rna]    — sense, antisense, and all three frames
circuit drift       — to_primitive against the canonical correspondence
circuit retract     — μ∘δ=id, leg by leg
circuit one [word]  — x86 → IMASM → RNA → IMASM → x86
circuit two [rna]   — RNA → IMASM → x86 → IMASM → wasm → IMASM → AA
```

Both take an argument or fall back to the full alphabet.

## Strands, frames, and what the sequences say

`circuit rc <rna>` prints the sense word, the antisense word, and all three
frames. `.` marks a codon carrying no glyph.

The sense glyph is read off the amino acid, so it depends on all three codon
positions; the antisense reads the reverse complement, which puts the sense
strand's third position first. The two strands are not independent readings of
the same content and neither determines the other.

**The CAG repeat is a run of pure criticality.** `CAG` is Gln, and Gln is
Criticality.

```
CAGCAGCAG…  sense ⊙⊙⊙⊙⊙   antisense .....
```

The expansion that causes polyglutamine disease is, in this alphabet, unbounded
repetition of a single glyph, and the glyph is the self-referential one. The
complement strand says nothing: `CUG` is Leu, ground layer.

**Poly-A is not silent — it is constant.**

```
AAAAAA…  sense ⊞⊞⊞⊞ in every frame   antisense ⋈⋈⋈⋈
```

`AAA` is Lys, Stoichiometry; `UUU` is Phe, Fidelity. A homopolymer reads the
same in all three frames, so the poly-A tail is a constant ⊞ signal whose
complement is a constant ⋈.

**The palindromic restriction site is a fixed point of the strand involution.**

```
GAAUUC…  sense ⊡⋈⊡⋈   antisense ⊡⋈⊡⋈
```

A reverse-complement palindrome is its own antisense, so the words coincide.
`GAA` is Glu, Winding; `UUU` is Phe, Fidelity.

**Much of the genome is mute to the alphabet.** The telomere repeat `UUAGGG`,
the Shine-Dalgarno sequence `AGGAGG` and its recognizer `CCUCCU` are silent in
every frame on both strands: Leu, Gly and Arg are all ground layer. Silence here
is not absence of function, it is absence of a primitive — the eight ground-layer
amino acids are structural scaffold, and sequences built from them carry no
glyph.

## Both open questions, closed

**The section choice is free.** δ takes a glyph to the first codon in
enumeration order that carries it, and that looked arbitrary. It is not load-
bearing: μ routes through the amino acid, so `μ∘δ = id` holds for *every* codon
carrying the glyph, not merely the representative δ picks.
`section_choice_is_free` checks this exhaustively, and `circuit census` reports
it. The section is determined up to a choice that changes nothing.

**The ground layer is silent by construction, not by omission.** Codon space
classifies with no residue:

```
23  carry a primitive (the twelve promoted acids)
38  scaffold (the eight ground-layer acids, no primitive)
 3  stop
64  total
```

`codon_role` is total — every codon is a primitive, scaffold, or stop — and
`strand_word` now prints `·` for scaffold and `|` for stop rather than one
anonymous dot for both. The eight ground-layer amino acids are exactly the
four-fold degenerate ones, and `GeneticCode.lean` proves they carry no primitive
(`ground_promoted_disjoint`, `aaToPrimitive` returning none). Silence here is the
theorem `20 = 8 + 12`, not a gap in the map.

The degeneracy per glyph is in `circuit table`. Met and Trp have one codon each,
Ile has three, the rest two — twenty-three in all.
