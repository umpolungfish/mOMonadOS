# Non-Forking Razor — crystallized census

Kernel-branch proof for the manuscript `ig-docs/nonforking_razor.tex`.

`census.sh` reproduces the paper's Table 1 against the hosted kernel: ten razor
words (five conjectures × affirm/negate) plus the two crystal decodes. It asserts
every word derives to its expected crystal, banks OK, closes vox T, and classifies
to ⊞; and that the descent crystal 3444190 and the two-fork-pair 16404190 differ
only at the ⊢ mark (0 vs 3).

Run: `./crystallized/nonforking_razor/census.sh` (exit 0 = PASS).

Result: RH, Collatz, BSD, Hodge, NS — every negation reconnects to the descent
crystal 3444190 (parity or', Frobenius self-dual), holds, μ∘δ=id. Four of the five
negations are the byte-identical word ⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣. Only Collatz's affirmation
lands on the fork (16404190), drawing the ⊢ distinction the others leave implicit.
