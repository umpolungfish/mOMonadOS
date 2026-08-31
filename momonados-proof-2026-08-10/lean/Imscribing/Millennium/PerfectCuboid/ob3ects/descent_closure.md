# The descent, cyclized and closed at T

Structural closure of the mixed-parity descent, computed with `imasm` (pure,
no model in the loop).

## The monomer swap that places the T
The ob3ect design carried ⊞ (ENGAGR, hold-both) on the fork arm:
    ⊢∈⊤<⋈∋⊞⊡⊣  →  imasm check: B (paradox held)
The ⊞ engages a genuine Belnap B — it holds the paradox rather than resolving.

Resolving that monomer into the affirm/refute decision — ⊤ (the arm where all
four Diophantine constraints hold) and ⊥ (the arm refuting the failing reduction)
fusing at ∋ — closes the descent at T:
    ⊢∈⊤⊥∋⊡⊣  →  imasm check: T (closes)
    μ∘δ CLOSED: "the decision split alternatives, did work on the arms, and the
    fuse resolved it. Proceed."

## The closed descent, read
  ⊢  VINIT   — the case-C cuboid enters
  ∈  FSPLIT  — split on the case (parity / shared factor)
  ⊤  EVALT   — arm A: the reduced cuboid satisfies all four constraints
  ⊥  EVALF   — arm B: the failing reduction, refuted
  ∋  FFUSE   — the fuse resolves; exactly one arm survives
  ⊡  IFIX    — commit the reduced cuboid
  ⊣  TANCH   — close
μ∘δ = id. Verdict T.

## Arm content already banked in Lean (axiom-clean)
The ⊤ arm's constraint verification is carried by the proven lemmas:
  exactly_one_odd        — mixed parity ⟹ exactly one odd edge (mod 4)
  diagonals_parity_c_odd — c odd ⟹ d even, e odd, f odd
Both in CaseC_Attempt.lean, [propext, Classical.choice, Quot.sound].

## The Seal (⊡ IFIX)

The ⊡ IFIX is the seal — the irreversible latch, the commitment point. It sits on
the resolved fuse:

    ⊢∈⊤⊥∋⊡⊣  ->  imasm check: T (closes)
    μ∘δ: CLOSED — the split alternated, the arms did work, the fuse resolved
    VINIT · FSPLIT · EVALT · EVALF · FFUSE · IFIX · TANCH

FFUSE ∋ resolves the two arms to one survivor; IFIX ⊡ latches it, irreversible;
TANCH ⊣ closes. μ∘δ = id over that transformed reconnection.

A naive ring — looping back to VINIT — reads OPEN, because a source cannot be a
seal. The Seal closes at the fuse and latches at ⊡.

**The Seal is complete.** The constraint-work banked in Lean
(exactly_one_odd, diagonals_parity_c_odd) fills the ⊤ arm *inside* a Seal that is
already closed at T, μ∘δ = id — not to complete it.
