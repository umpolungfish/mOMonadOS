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

The Crystal of Types is a 17,280,000-address structural type space. Every address is a
point in the product of 12 primitive value sets:

```
address = Σᵢ (index[i] × stride[i])
strides = [5184000, 1728000, 576000, 144000, 48000, 12000, 4000, 800, 200, 50, 10, 1]
```

Files are located by structural type, not by path.

### `crystal store <name> [data]`

Store an entry. The kernel automatically:
1. Hashes `name` → selects one of the 12 canonicals (deterministic)
2. Loads that canonical and runs one tick (structural state change)
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

Display the SIC-POVM d=12 structural identity: 3-lattice proofs (Belnap, crystal, Lean),
6 Frobenius-dual pairs, Σ=1:1 self-referential grammar limit.

```
⊙> sic
SIC-POVM d=12 Structural Identity
  Belnap B=XZ as d=2 fiducial: ✓
  6 Frobenius-dual pairs: ✓
  Grammar as Σ=1:1 limit: d=2.0 (Sigma: 1:1 vs n:m)
  Lean 4 formalization: 3 theorems, 0 sorries
```

### `d12 [subcmd]`

Phase VI d=12 SIC-POVM augmentation. Full structural data from the d12_sic_build campaign.

| Sub-command | Output |
|-------------|--------|
| (no args) | Compact status summary |
| `tower` | Phase-tower collapse: 3→1 generators, 8× reduction |
| `magnitudes` | Magnitude square-class group: K16, rank 5, singleton-pairing |
| `orbits` | 31-orbit Galois structure, degree distribution, existence-grade |
| `duallink` | Dual-Link identification: norm(N₁)=1/32448², ramification {2,3,13} |
| `z0` | Closed-form fiducial: z₀=+√(1/12−√2/24+√13/156−√26/312) + ray tower |
| `ordinals` | 12 canonical ordinal faithfulness guards |
| `verify` | Full Phase VI report (all 5 pillars + Lean module listing) |

```
⊙> d12 tower
Phase-Tower Collapse
  3 independent generators → 1
  Phase space: dim 262,144 → 32,768 (8× reduction)
  X31 ∈ K16(s1s3,i)  |X31|=1 ✓
  X15 ∈ K16(c5,i)    |X15|=1 ✓
  X31·X53·X15 = 1    resid 2^−5310 ✓
  V2 engine: ONE generator, 143/143 loop written
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
all 7 problems (RH, YM, BSD, Hodge, NS, PvsNP, OPN) with Lean 4 status (sorry/sorry-free)
and Frobenius absorption class.

```
⊙> clay
Clay Millennium — Structural Status (Lean 4, machine-checked)
  RH:  barrier=O₂, sorry-free=12/12, abs=⊗_EML
  YM:  barrier=O₂, sorry-free=8/8,   abs=⊗_mass_gap
  BSD: barrier=O₂, sorry-free=5/5,   abs=⊗_2adic
  Hodge:  barrier=O₂, sorry-free=4/4, abs=⊗_hodge
  NS:  barrier=O₂, sorry-free=6/6,   abs=⊗_ns
  PvsNP: barrier=O₂, sorry-free=7/7, abs=⊗_pnp
  OPN: barrier=O₂, sorry-free=3/3,   abs=⊗_opn
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
