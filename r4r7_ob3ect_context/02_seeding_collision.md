# The collision the widening introduced

The register file is eight wide. R0 through R3 carry the tuple as pairs:
(D,<), (T,Ω), (K,⋈), (H,P). That packing leaves four of the twelve slots
unrepresented: R the adjoint, ∈ the maximal, ⊤, and Σ the one-to-one.

When the file was widened, R and Σ were seeded into R4, and ∈ and ⊤ into R5, so
that the slots closure and self-reference are stated in would be present. But
IMSCRIB writes the four closure witnesses into R4 through R7. So the recovered
slots vote only until the first imscription, and are then overwritten.

The Grammar has already settled that the witnesses belong in R4 through R7 and
that they vote rather than gate. What it has not settled is whether the
recovered slots belong in the register file at all.

## Statement A

The tuple occupies R0 through R3 alone, as four pairs, and R4 through R7 belong
to the closure witnesses, so the adjoint and the one-to-one do not enter the
register file and are carried only by the program the tuple generates.

## Statement B

The adjoint and the one-to-one are seeded into R4 and R5, and the witnesses
overwrite them at the first imscription, so those registers carry tuple
structure before imscription and closure evidence after it.

## What hangs on it

Under A the substrate vote before imscription is blind to R and Σ, and the
register file is cleanly two blocks: what the tuple is, and what the program has
shown. Under B the same registers mean different things at different times, and
a program with no IMSCRIB never converts them, so the vote depends on whether
imscription occurs.
