# mOMonadOS User Guide

## Boot sequence

```
[BOOT] mOMonadOS — The Self-Imscribing Bare-Metal Kernel
[BOOT] Heap: 4MB @ 0x...
[BOOT] Kernel online — μ∘δ=id
[BOOT] Bootstrap: IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB
[BOOT] Crystal FS: 17280000 addresses

╔══════════════════════════════════════════════════╗
║            m O M o n a d O S                    ║
║    The Self-Imscribing Bare-Metal Kernel         ║
║    Frobenius Core · Belnap FOUR · Crystal FS     ║
╚══════════════════════════════════════════════════╝

Type 'help' for commands.

⊙>
```

The kernel boots with the bootstrap loop loaded and one tick already computed.

---

## REPL commands

### `tick [N]`

Run N kernel ticks (default 1). Each tick is one full THINK→ACT→OBSERVE→UPDATE cycle.

```
⊙> tick
⊙> tick 1000
```

### `run [N]`

Run N additional ticks from the current position. Unlike `tick`, `run` is the continuous
execution path — use it when you want the kernel to evolve without watching each step.

```
⊙> run 10000
```

### `status`

Print kernel state: tick count, cycle count, tier, IP, stack depth, Frobenius check totals,
R0–R7 register values.

### `program`

Show the current program as a token chain with length and instruction pointer.

```
⊙> program
IMSCRIB → EVALT → FSPLIT → EVALF → FFUSE → ENGAGR → IFIX → IMSCRIB
len=8 ip=3
```

### `snapshot`

Show the structural snapshot computed by the last THINK phase.

| Field | Meaning |
|---|---|
| Tier | Ouroboricity: O₀, O₁, O₂, O_∞ |
| sig | Family counts (Logical, Frobenius, Dialetheia, Linear) |
| diversity | Distinct token types present (0–12) |
| self_ref | First token == last token |
| frob_ord | 0=none 1=split→fuse 2=fuse→split |
| dialeth | EVALT ∧ EVALF ∧ ENGAGR all present |
| period | Smallest p such that program repeats with period p |

### `canonical <I–XII>`

Load one of the 12 canonical programs by Roman numeral. Resets IP to 0.

```
⊙> canonical I
⊙> canonical VIII
⊙> canonical XII
```

| # | Name | Program |
|---|---|---|
| I | I_Dialetheic_Bootstrap | IMSCRIB EVALT FSPLIT EVALF FFUSE ENGAGR IFIX IMSCRIB |
| II | II_Void_Genesis | VINIT FSPLIT EVALT FFUSE EVALF CLINK IFIX IMSCRIB |
| III | III_Anchor_Protocol | TANCH AFWD EVALT AREV EVALF CLINK IFIX TANCH |
| IV | IV_Dual_Bootstrap | IMSCRIB AFWD FFUSE FSPLIT AREV CLINK IFIX IMSCRIB |
| V | V_Linear_Chain | IFIX × 8 |
| VI | VI_Empty_Bootstrap | (VINIT IMSCRIB) × 4 |
| VII | VII_Parakernel | ENGAGR AFWD FSPLIT EVALT FFUSE EVALF IFIX ENGAGR |
| VIII | VIII_Frobenius_Kernel | (FSPLIT FFUSE) × 2 |
| IX | IX_Chiral_Pairs | (AFWD AREV) × 4 |
| X | X_Truth_Machine | IMSCRIB FSPLIT EVALT IFIX IMSCRIB FSPLIT EVALF IFIX |
| XI | XI_Eternal_Return | TANCH AFWD AREV TANCH AFWD AREV TANCH AFWD |
| XII | XII_ROM_Burn | EVALT IFIX EVALF IFIX ENGAGR IFIX IMSCRIB IFIX |

### `continuous <1-4>`

Load one of the 4 continuous programs (XIII–XVI). Resets IP to 0.

```
⊙> continuous 1
⊙> continuous 4
```

| # | Name | Tok | Signature |
|---|---|-----|-----------|
| XIII | Heartbeat | 4 | ◊ pulse |
| XIV | Tier_Climber | 9 | O₀→O₁ promotion |
| XV | Frobenius_Oscillator | 4 | μ∘δ oscillation |
| XVI | Paradox_Daemon | 7 | B-stabilized paradox |

### `novel <1-3>`

Load one of the 3 novel programs (XVII–XIX). Resets IP to 0.

```
⊙> novel 1
⊙> novel 3
```

| # | Name | Tok | Description |
|---|---|-----|-------------|
| XVII | Nested_Fork_Labyrinth | 11 | Deep fork nesting |
| XVIII | Terminal_Sink_Protocol | 8 | Sink-node detection |
| XIX | Mirrorgram | 9 | Self-reflective structure |

### `shunt <0-8>`

Load one of the 9 shunted programs (XX–XXVIII) by index. Resets IP to 0.

```
⊙> shunt 0
⊙> shunt 7
```

| # | Name | Tok | Tier | Description |
|---|---|-----|------|-------------|
| XX | Shunt_Bridge | 14 | O_∞ | Void Genesis ⊕ IMSCRIB ⊕ Dialetheic Bootstrap |
| XXI | Anchor_Paradox | 11 | O₂ | Anchor Protocol ⊕ ENGAGR ⊕ Parakernel |
| XXII | Chiral_ROM | 12 | O₂ | Chiral Pairs ⊗ ROM Burn interleave |
| XXIII | Dual_Kernel_Shunt | 13 | O_∞ | Dual Bootstrap ⊕ CLINK ⊕ Kernel (nested) |
| XXIV | Heartbeat_Paradox | 8 | O₁ | Empty Bootstrap ⊗ Paradox Daemon |
| XXV | Recursive_Kernel | 10 | O₁ | Kernel² ⊕ CLINK spine (stacked) |
| XXVI | Truth_Spiral | 13 | O₂ | Truth Machine ⊕ ENGAGR (Frobenius-complete) |
| XXVII | Omni_Spine | 19 | O_∞ | All classes via CLINK spine (maximal composite) |
| XXVIII | Somatic_Shunt | 11 | O₂ | VP shunt topology — the somatic shunt mechanism |

Six shunt mechanisms are used: **IMSCRIB Bridge**, **ENGAGR Paradox**, **Interleave**, **CLINK Spine**, and **Nested Fork**, and **Somatic Shunt** (the only mechanism instantiated in living tissue). See [SHUNTED_PROGRAMS.md](SHUNTED_PROGRAMS.md) for full token sequences and fork topologies.

---

## Crystal FS

The Crystal of Types is a 17,280,000-address type space. Every address is a
point in the product of 12 primitive value sets:

```
address = Σᵢ (index[i] × stride[i])
strides = [5184000, 1728000, 576000, 144000, 48000, 12000, 4000, 800, 200, 50, 10, 1]
```

Files are located by type, not by path.

### `crystal store <name> [data]`

Store an entry. The kernel automatically:
1. Hashes `name` → selects one of the 12 canonicals (deterministic)
2. Loads that canonical and runs one tick (state change)
3. Derives the 12-primitive address from the resulting snapshot
4. Stores at that address

Same name always maps to the same crystal address. Different names spread across
12 distinct canonical starting points.

```
⊙> crystal store kernel.state
⊙> crystal store notes.md "initial invariants established"
```

Output shows which canonical was loaded, the tick number, and the resulting address + tuple.

### `crystal name <name>`

Retrieve a stored entry by name.

```
⊙> crystal name notes.md
Name:    notes.md
Address: 11538778
Data:    initial invariants established
Canon:   IV_Dual_Bootstrap
```

### `crystal <addr>`

Decode a crystal address to its 12-primitive tuple. If an entry is stored at that address,
it is shown.

```
⊙> crystal 11538778
Address: 11538778
  D: 0   T: 3   R: 2   P: 1   F: 0
  K: 2   G: 2   C: 1   Phi: 1  H: 0
  S: 0   Omega: 3
  Stored: 'notes.md' → 'initial invariants established'
```

### `crystal find`

List all stored entries.

```
⊙> crystal find
3 entries stored:
  [1728000]  farts.txt —
  [11538778] notes.md — initial invariants established
  [2821736]  kernel.state —
```

---

## Memory, registers, stack

### `memory [start] [count]`

Dump B4 memory cells as N/T/F/B. Default: 16 cells from address 0.

```
⊙> memory 0 32
N N N N N N N N N N N N N N N N N N N N N N N N N N N N N N N N
```

### `registers`

Show R0–R7 as B4 values.

```
⊙> registers
R0:T R1:N R2:N R3:N R4:B R5:T R6:T R7:T
```

Registers R4–R7 are written by IMSCRIB (self-imscription opcode):
- R4 = token_diversity & 3
- R5 = self_ref (T/F)
- R6 = frobenius_order > 0 (T/F)
- R7 = dialetheia_complete (T/F)

### `stack`

Show current stack depth. The stack holds B4 values pushed by VINIT, EVALT, EVALF,
ENGAGR, FSPLIT.

### `arev [test]`

The ⊥ hop: toggle the kernel's chirality bit so every snapshot is read through
the R1↔R2 mirror. The mirror exchanges the two evidence triples role for role
(dialetheia_complete ↔ atomic_reentry, b_live_ticks ↔ winding_count,
gate_discriminations ↔ bifurcation_revisited) over the shared temporal
substrate; the accumulators themselves are untouched, so hop∘hop = id exactly.
A run sitting at O_inf_dag reads as O_inf through the mirror: the lateral
partner at the same shell, reached by AREV as an operation rather than named
by the classifier.

```
⊙> arev test
═ AREV door experiment ═
replicative loop, 16 ticks, ⊥ = or':
  s0         tier O_inf_dag  R1(dialeth=false b_live=false gates=false)  R2(atomic=true wind=true bifurc=true)
first hop (⊥ flipped) — R1 reads the mirrored evidence:
  s1         tier O_inf      R1(dialeth=true b_live=true gates=true)  R2(atomic=false wind=false bifurc=false)
second hop (⊥ back to or'):
  s2         tier O_inf_dag  R1(...)  R2(...)
hop∘hop = id (raw fields): EXACT
```

---

## Belnap FOUR values

| Value | Meaning |
|---|---|
| N | None — void, absence, the initial object |
| T | True — affirmation |
| F | False — negation |
| B | Both — paradox stabilized (ENGAGR) |

Meet (∧): N<T, N<F, T<B, F<B — N is bottom, B is top.
Join (∨): dual.

---

## Ouroboricity tiers

| Tier | Condition |
|---|---|
| O₀ | No Frobenius pair, no complete dialetheia |
| O₁ | Frobenius pair present OR dialetheia complete |
| O₂ | Frobenius + self-ref + dialetheia complete, period = 2 |
| O_∞ | Frobenius + self-ref + dialetheia complete, period ≥ 3 |

The bootstrap loop (IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB) satisfies
O_∞ from tick 1: Frobenius pair present, self-referential (IMSCRIB first and last),
dialetheia absent but period = 8 ≥ 3. The kernel self-modifies toward O_∞
when it drifts below.

---

## Quit

```
⊙> halt
```

Or Ctrl-A then X in QEMU serial mode.


---

## SIC-POVM Commands

### `sic`

Display the SIC-POVM d=12 identity: 3-lattice proofs (Belnap, crystal, Lean),
6 Frobenius-dual pairs, Σ=1:1 self-referential grammar limit.

```
⊙> sic
SIC-POVM d=12 Structural Identity
  Belnap B=XZ as d=2 fiducial: ✓
  6 Frobenius-dual pairs: ✓
  Grammar as Σ=1:1 limit: d=2.0 (Sigma: 1:1 vs n:m)
  Lean 4 formalization: 4 planks (incl. SIC_D12_ExistenceRing), *sans* sorry
```

### `d12 [subcmd]`

Phase VI d=12 SIC-POVM augmentation (cont.20 — Existence Ring Found). ALL 143 identities exact in R=K₁₆(s₀,s₁,s₃,s₉,i,c₅,u₁), dim 2048, pure fractions, 12s. ANY hom R→ℂ is a SIC point. Lean-proved: `SIC_D12_ExistenceRing.lean` (ALL 143 identities tower-derived, `native_decide`, 8341 jobs green, zero sorries).

| Sub-command | Output |
|-------------|--------|
| (no args) | Compact status summary |
| `tower` | Phase-tower collapse: 3→1 generators, 8× reduction |
| `magnitudes` | Magnitude square-class group: K16, rank 5, singleton-pairing |
| `orbits` | 31-orbit Galois structure, degree distribution, existence-grade |
| `existence` | `ring` | Existence ring report: R=K₁₆(…), dim 2048, flip-audit, 14 Lean theorems |
| `duallink` | Dual-Link identification: norm(N₁)=1/32448², ramification {2,3,13} |
| `z0` | Closed-form fiducial: z₀=+√(1/12−√2/24+√13/156−√26/312) + ray tower |
| `ordinals` | 12 canonical ordinal faithfulness guards |
| `verify` | Full Phase VI report (all 5 pillars + all 4 Lean planks incl. ExistenceRing) |

```
⊙> d12 tower
Phase-Tower Collapse
  3 independent generators → 1
  Phase space: dim 262,144 → 32,768 (8× reduction)
  X31 ∈ K16(s1s3,i)  |X31|=1 ✓
  X15 ∈ K16(c5,i)    |X15|=1 ✓
  X31·X53·X15 = 1    resid 2^−5310 ✓
  V4 engine: ALL 143 PASS, dim 2048, 12s, pure fractions
  Ring: K₁₆(s₀,s₁,s₃,s₉,i,c₅,u₁)
  Capstone: ANY hom R→ℂ is a SIC point
```

### `entropy [tier|transition]`

Entropy experiment: compute ΔS during tier promotion. Confirms O_∞ promotion is
entropically favored under the grammar's absorption rules.

```
⊙> entropy tier
ΔS(O₀→O₁): +2.14 bits
ΔS(O₁→O₂): +1.87 bits
ΔS(O₂→O_∞): +3.41 bits
Total ΔS: +7.42 bits (favored, p<0.001)
```

### `clay`

Clay Millennium structural status: displays the machine-checked barrier taxonomy for
all 7 problems (RH, YM, BSD, Hodge, NS, PvsNP, OPN) with Lean 4 status (sorry/*sans* sorry)
and Frobenius absorption class.

```
⊙> clay
Clay Millennium — Structural Status (Lean 4, machine-checked)
  RH:  barrier=O₂, *sans* sorry=12/12, abs=⊗_EML
  YM:  barrier=O₂, *sans* sorry=8/8,   abs=⊗_mass_gap
  BSD: barrier=O₂, *sans* sorry=5/5,   abs=⊗_2adic
  Hodge:  barrier=O₂, *sans* sorry=4/4, abs=⊗_hodge
  NS:  barrier=O₂, *sans* sorry=6/6,   abs=⊗_ns
  PvsNP: barrier=O₂, *sans* sorry=7/7, abs=⊗_pnp
  OPN: barrier=O₂, *sans* sorry=3/3,   abs=⊗_opn
```

### `clay witness <problem>`

Load IMASM witness program for BSD, Hodge, or YM. The witness program traverses the
structural barrier via Frobenius-dual pairs.

```
⊙> clay witness bsd
BSD Witness: FSPLIT→EVALT→FFUSE→EVALF→CLINK→IFIX (6 tok, O₂)
  Frobenius pair: ✓  Dialetheia: partial  Barrier: 2-adic structural

⊙> clay witness hodge
Hodge Witness: IMSCRIB→FSPLIT→EVALT→IFIX→IMSCRIB (5 tok, O_∞)
  Frobenius pair: ✓  Self-ref: ✓  Period: 5 ≥ 3

⊙> clay witness ym
YM Witness: AFWD→FSPLIT→EVALT→AREV→FFUSE→EVALF→CLINK→IFIX (8 tok, O_∞)
  Frobenius pair: ✓  Self-ref: ✗  Dialetheia: complete  Period: 8 ≥ 3
```

---


## Fibonacci Anyon Quantum Computer

`fibonacci_qc.rs` carries the SU(2)₃ anyon algebra, the braid representation on fusion
trees, and a Solovay-Kitaev compiler that reduces a standard gate to a braid word.
Fibonacci anyons are computationally universal, and the fusion space V_n has dimension
F_{n-1}, so four anyons carry one qubit.

### `fibqc verify`

Runs the algebra self-check: F unitary, the pentagon form, the Yang-Baxter braid
relation, spin-statistics, S unitary, charge conjugation, the TQFT identities, the
Verlinde formula, and the Artin relations up to eight strands.

The pentagon entry checks more than `F² = I`. That identity alone is satisfied by four
numbers that merely square to the identity, so the check also verifies what the
pentagon forces: that F is real and symmetric with nonvanishing off-diagonal,
anti-diagonal in the sense d = −a, and normalized to a² + b² = 1. Together these pin
the entries.

### `fibqc compile <gates> [depth]`

Compiles a circuit over `H`, `T`, `S`, `X` to a braid word.

```
⊙> fibqc compile T S
Building gate net (depth 10, SK recursion 3)...
  net: 4842 entries, 1704 KB (heap 2909 of 49152 KB)
  single arm : error 7.260095e-4  length 1068
  split+fused: error 7.260095e-4  length 1068
  gain       : 1.0x
  unitary    : true
  word check : PASS (residual 5.20e-14)
  heap peak  : 2922 of 49152 KB
```

The circuit compiles as ONE unitary rather than gate by gate, so the approximation
error is incurred once instead of accumulating across the gates.

### The two depths

Two trailing integers set the gate net depth and then the Solovay-Kitaev recursion
depth, as in `fibqc compile T S 12 3`. They default to 10 and 3. One integer sets the
net depth alone, and `fibqc compile T S 10 0` asks the net directly with no recursion
on top, which is the only setting whose error is monotone in net size.

Neither depth is capped. The net grows until it is within a third of the arena and
then stops where it is, and the recursion asks the same question one level at a time:
the word a level returns is five sub-words long, so it grows by roughly a factor of
five per level, its size is known before any of it is allocated, and a level that will
not fit is declined where the level below it is already a complete answer. What comes
back is the deepest thing that fit, and the run says so rather than reporting the
depth that was asked for.

Depth costs arena, not correctness. Recursion 9 on a small net reaches an error near
4e-10 with a word of 1.8 million generators and peaks at 18 MB of the 48 MB arena.

### Where the recursion stops paying

The net sets how deep the recursion is worth running, and the point is sharp. On the
47-entry net that a net depth of 3 builds, recursion 7 gives 6.4e-8, recursion 8 gives
2.7e-9, and recursion 9 gives 3.9e-10. Recursions 10 and 11 give 3.9e-10: the same
error, the same word to the generator, and the same high-water mark to the kilobyte.
The levels above 9 are not merely a poor trade. They contribute nothing at all.

What happens there is worth knowing, because it is the free reduction doing it. A level
appends its two commutator factors and their inverses around the base word, and once
the residual falls below what the net can resolve, those four factors cancel each other
completely. The level returns its own base, byte for byte. Without free reduction the
same level would return a word five times longer for no accuracy whatever, and the
level above that twenty-five times longer, and the arena would be what stopped it
rather than arithmetic. With it, asking for a depth past the net's resolution is free:
it costs no memory and no error.

So a run whose error will not come down wants a larger net, not a deeper recursion. The
flat pair of rows is the same signal here as in the two error lines below: the
dictionary is the limit.

The words themselves are freely reduced at every level, since the concatenation is
where cancelling pairs are created and each one the level above would otherwise copy
five more times. Free reduction leaves the unitary exactly where it was, so the errors
are unchanged and only the lengths come down.

The compile scopes itself with `heap_mark`/`heap_reset`, so a second invocation starts
from the same place, and it reports its own high-water mark. Each level of the
recursion scopes itself the same way: the sub-words it built are dead the moment it
returns, so its result is moved down over them and the bump pointer follows. Peak use
tracks the path down the recursion rather than the whole tree, which is the difference
between a few megabytes and the whole arena.

### What the two error lines mean

Several braid words routinely sit at the same distance from the target. Each seeds a
different trajectory and leaves a residual rotation pointing its own way. `single arm`
is what one of them achieves alone; `split+fused` follows every tied word as a separate
branch and then has the arms that lost compile the residual left by the arm that won,
appending it, so the composite beats every arm it was chosen from.

Depth matters to whether the fuse fires at all. `T S` gains nothing at depth 10 and
25.4× at depth 12, because compiling the survivor's residual needs more dictionary than
4842 entries supply. A flat row means dictionary-limited, not converged.

### `word check`

The reported unitary is verified against its own printed word by resynthesizing the
word from scratch, agreeing to about 1e-13 on a short word and to about 1e-10 on a
word of a million generators, where the residual is the rounding of the walk itself.
The walk folds the generators through 2x2 registers rather than building a matrix per
generator, so checking a long word costs no arena and the check cannot be the thing
that runs the kernel out. The determinant identity
`det(braid) = det(σ₁)^(sum of exponents)` is NOT used: `det(σ₁)` is a primitive tenth
root of unity, so that test passes by chance one time in ten, and it sees only the sum
of the exponents, so every permutation of a word passes it.

### Drawing the word: `draw`, `svg`, `loop`

A word in front of the gates picks a form: `qc draw H T` puts the strand diagram in
the terminal, `qc svg H T` emits the flat diagram as an SVG document, and
`qc loop H T` emits the closed braid. `bi` takes the same three over a word typed
straight in, as in `bi loop 1 2 -1 -2 1 2`. Everything is written to the serial line,
so redirect it to a file to keep it.

The flat form is the braid as it is written: strands down the page, one crossing per
row, folded into columns so a compiled circuit is a figure rather than a mile of paper.
It is the form to read a specific crossing out of, and it carries the crossing index
beside every row.

The closed form is the braid as a link. Closing a braid means bending the page into a
cylinder so every strand's foot meets its own head, and `loop` draws that cylinder end
on: the strand positions are concentric tracks, one crossing owns one angular slot,
and reading once around the ring is reading the word once through. Nothing is added to
close it, because a circle is already closed. The strands of a permutation cycle join
into one curve on their own, and each such curve is one component of the link, which
is why colour there follows the component rather than the strand. The hole in the
middle is the braid axis, and the caption sits in it.

The ring grows with the crossing count rather than packing them tighter, so the
crossings stay the same size to read at any length. A word too long for the arena to
hold the whole document is drawn as far as it fits and says so, in the caption and in
the `desc`, as the closure of the crossings it drew.

Both SVG forms carry their own stylesheet and answer either theme: the gap in an under
strand is cut by overstroking it in the background colour, and a viewer in a dark theme
would otherwise see those gaps filled with white.

### `fibqc jones <strands> <generators...>` and `fibqc knot [name]`

Jones polynomial of the braid closure at 1/5 of a winding (t = e^{2πi/5} in radian notation). This is the invariant Fibonacci
anyons exist to compute: SU(2)₃ Chern-Simons *is* that evaluation, so the braid
representation performs it rather than simulating it.

```
⊙> fibqc knot trefoil
trefoil — closure of a 2-strand braid
  braid      : 2 strands, 3 crossings, writhe 3
  V at t = 1/5 winding : -0.809017 -1.314328i
  |V|        : 1.543362
  chirality  : SEPARATED from mirror (mirror value is the conjugate)
```

The value's own phase is reported in windings too, which makes the chirality verdict
readable rather than asserted: the knots that cannot be told from their mirror sit at
exactly 1/2 and 0, the only self-inverse windings, while a separated one lands
off-lattice.

`fibqc knot` with no argument lists the census. `fibqc jones 2 1 1 1` takes an arbitrary
braid, strand count first.

The two normalization constants are forced rather than fitted: requiring the unknot to
evaluate to 1 in its three Markov presentations is two constraints in two unknowns, and
it returns the framing phase e^{−iπ/5} and the loop value −φ, which are the constants
the theory names. Verified against exact values for the trefoil, its mirror, the
figure-eight and the cinquefoil.

The chirality verdict is decidable from the value alone, and necessarily: the Jones
polynomial has integer coefficients and mirroring acts by t → t⁻¹, which on the unit
circle is conjugation, so V(K*) = conj(V(K)) and a mirror pair separates exactly when
the imaginary part is nonzero. **A real value means the invariant cannot see the
chirality at this root, not that the knot is amphichiral.** The cinquefoil is chiral and
evaluates real.

One root is not a complete invariant, and the command says so when it matters: 8₁₉
evaluates to 1 exactly as the unknot does, because t³ + t⁵ − t⁸ collapses to t³ + 1 − t³
under t⁵ = 1.

σ₁ here is the negative crossing in the standard Jones orientation, and the word is
mirrored internally to compensate.

### `fibqc winding`

Prints the model's phase lattice in windings, where one winding is a full turn.

Every phase native to the model is an exact multiple of a **tenth of a turn**:

| constant | winding | tenths |
|----------|---------|--------|
| θ_τ topological spin | 2/5 | 4 |
| R^{ττ}_1 | 2/5 | 4 |
| R^{ττ}_τ | −3/10 | −3 |
| t Jones root | 1/5 | 2 |
| α framing phase | −1/10 | −1 |
| −φ loop value (phase of) | 1/2 | 5 |
| modular T diagonal | 0, 2/5 | 0, 4 |
| F eigenvalues | 0, 1/2 | 0, 5 |

The braid generator's two eigenvalues are 4/10 and −3/10, and those generate the
tenths. That is the same fact as det(σ₁) being a primitive tenth root of unity, which
is why that determinant makes such a weak checksum: it is the lattice, and everything
lives on it.

**Why compilation is hard, in one line.** The gates that are not multiples of a tenth
are exactly the ones not native to the model: T is 1/8 of a turn and S is 1/4. Since
1/8 is not a multiple of 1/10, no braid reaches the T gate exactly at any length.
Solovay-Kitaev exists to approach an incommensurable point on a commensurate lattice,
and what makes the approach possible is the non-commutativity of the generators, not
their phases.

**Why some chirality cannot be seen.** The only self-inverse windings are 0 and 1/2,
which are precisely the real values. A knot invariant landing there is one the mirror
cannot be told from, and `Winding::is_self_inverse` is the criterion the kernel uses,
taken from the lattice rather than from a tolerance on an imaginary part.

Holding phases this way is also why the arithmetic is exact. Radians turn these
rationals into transcendentals, multiply them, and then measure the drift; as rational
turns they compose by integer arithmetic. `α^writhe` is one exact scaling rather than
`writhe` complex multiplications, and the loop value splits into an exact half-turn
phase times a real magnitude, so the phase accumulates nothing. The unknot returns
exactly zero error and the period-ten closure of the T(2,n) family holds to 1e-15.

### Note on piping

The serial console drops the first character of the first line, so
`printf "fibqc verify\n" | ./run.sh` arrives as `ibqc`. Send a bare newline first.


## Lattice cycling and weight flow

### `cycle <word>`

Walks an IMASM word around its ROTAT orbit. A word is a ring and ROTAT is the
cyclic shift, so every rotation is the same object: the verdict and the topology
hold across the whole orbit, and the **final register does not**. That makes the
phase the only handle on where a word comes to rest, and the report ends with
the map from cut to landing register.

```
⊙> cycle ⊢⊙=>◇+×<⊞●×¬⊣
  landing register by cut:
    A      at k = 0, 1, 2, 3, 4, 12
    Ftf    at k = 5, 6, 7, 8
    F      at k = 9, 10
    T      at k = 11
```

◇ and ● are accepted for ∈ and ∋, since a word copied out of a bootstrap report
is in the 12-op alphabet while the machine reads the trilattice one.

### `weight <word>`

The machine holds each open fork as a set and closes it with a union. Union is
idempotent, so a finished walk knows which base values were touched and nothing
else: not how many times, not by which arm, not whether a value reached the end
or was destroyed and restored on the way. This counts.

Weight banked in a frame survives a clear that empties the register; weight left
in the open does not. The lift of OR to weights is max rather than sum, so the
fuse **restores** what a clear destroyed instead of adding to it, and at weights
zero and one the accounting reduces to the set semantics exactly.

Two movements carry no weight and are reported because they are otherwise
invisible in a final register. **SEED**: AFWD and IMSCRIB put T into an empty
register directly, so a walk can land in T having carried nothing. **INERT**:
after IFIX every token but IFIX and IMSCRIB is a no-op.

```
⊙> weight ¬⊣⊢⊙=>◇+×<⊞●×
     4 ⊙  SEED T into an empty register, no weight
  final    : T
  surviving: none
  deposits 0  cleared 0  restored 0  seeded 1  inert 11
```

That word is the k=11 cut above. It starts with ¬, which fixes the register, so
eleven of thirteen steps are inert and the T is placed by the seed rule rather
than carried by anything. Nothing moved because nothing ran.
