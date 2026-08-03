# Context: how the closure witnesses enter the substrate vote

## The object

mOMonadOS carries two evaluators. The kernel is the full IMASM machine: an
eight-wide B4 register file, a B4 memory, a fork stack, cyclic programs. The
MiniKernel in `sequence.rs` is a substrate evaluator seeded from an IG tuple; it
runs a canonical program and reads its post-execution registers to derive token
scores, and those scores drive program synthesis through `build_via_substrate`.

## What was just corrected

The MiniKernel had drifted from the kernel in four places. Three are now fixed.

1. FSPLIT/FFUSE. The kernel records the split-time value in a fork frame and
   FFUSE joins the linear left with the value belonging to its matching split.
   The MiniKernel joined whatever was second on the stack and consumed two items
   where the kernel consumes one. Both the fused value and the arity were wrong.
   Eighteen of the thirty-four canonical programs produced a different final
   stack under the correction.

2. Register width. The file is eight wide in the kernel, because the layer is
   CL8NK, CLINK Layer 8, the Organism. The MiniKernel had four, packing pairs of
   tuple slots into single B4 values: (D,<), (T,Ω), (K,⋈), (H,P). That drops four
   of the twelve slots, and the four dropped were R, ∈, ⊤ and Σ. R is the adjoint
   slot and Σ is the one-to-one self-referential slot, so the packing removed
   exactly what closure and self-reference are stated in. The file is now eight
   wide, with R4 and R5 seeded from the recovered slots.

3. IMSCRIB and IFIX. In the kernel IMSCRIB writes four values into R4 through R7:
   token diversity, self-reference, Frobenius order, dialetheia completeness.
   Those are the closure witnesses. With no R4-R7 the MiniKernel's IMSCRIB wrote
   none of them, which is not a simplification of the operation but its deletion.
   IFIX stores to memory at the address in R0; with no memory the MiniKernel
   folded the store into a register join and lost the address. Both now behave as
   the kernel does, with the witnesses computed by `self_imscribe` on the running
   program, which is legitimate because all four are static in the program.

## The open question

`register_scores` votes through a twelve-by-four affinity table over R0-R3:

```
                R0  R1  R2  R3          R0 = D x Phi   R1 = T x Omega
    VINIT    [   2,  0,  0,  1 ]        R2 = K x f     R3 = H x P
    TANCH    [   0,  0,  2,  0 ]
    AFWD     [   1,  2,  2,  0 ]        score[token] = sum over reg of
    AREV     [   0,  1,  1,  2 ]            affinity[token][reg] * b4_score(reg)
    CLINK    [   2,  1,  0,  0 ]
    IMSCRIB  [   0,  0,  0,  3 ]        b4_score: N -> 0, T or F -> 1, B -> 2
    FSPLIT   [   0,  2,  0,  0 ]
    FFUSE    [   0,  2,  0,  1 ]
    EVALT    [   1,  0,  2,  0 ]
    EVALF    [   1,  0,  2,  0 ]
    ENGAGR   [   2,  0,  0,  2 ]
    IFIX     [   0,  0,  3,  1 ]
```

R4 through R7 are now written and unread. Giving them a vote is not a mechanical
extension, because the witnesses are not a fifth and sixth pair of tuple slots.
In the kernel these same four values determine the tier of the snapshot. Two
readings are available and they differ in kind, not in degree.

## The two statements

**Statement A.** The closure witnesses in R4 through R7 enter the substrate vote
as four further affinity columns, weighting tokens in the same manner as R0
through R3, so that a program's diversity, self-reference, Frobenius order and
dialetheia completeness each pull the next token toward the primitives they
favour.

**Statement B.** The closure witnesses in R4 through R7 gate the substrate vote,
qualifying the result computed from R0 through R3 rather than adding to it, so
that they determine whether and how the vote is admitted rather than contributing
weight to it, as they determine tier in the kernel.

## What hangs on it

The substrate vote selects tokens, the tokens become programs, and the programs
are what the kernel then evaluates for closure. If the witnesses vote, closure
evidence feeds back as preference. If they gate, closure evidence conditions
admission. The first makes closure one influence among several; the second makes
it a precondition. Both are implementable; only one is what the Grammar says.
