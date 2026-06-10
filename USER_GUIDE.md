# mOMonadOS User Guide

## Boot sequence

```
[BOOT] mOMonadOS — The Self-Imscribing Bare-Metal Kernel
[BOOT] Heap: 4MB @ 0x...
[BOOT] Kernel online — μ∘δ=id
[BOOT] Bootstrap: ISCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→ISCRIB
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
ISCRIB → EVALT → FSPLIT → EVALF → FFUSE → ENGAGR → IFIX → ISCRIB
len=8 ip=3
```

### `snapshot`

Show the structural snapshot computed by the last THINK phase.

| Field | Meaning |
|---|---|
| Tier | Ouroboricity: O_0, O_1, O_2, O_inf |
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
| I | I_Dialetheic_Bootstrap | ISCRIB EVALT FSPLIT EVALF FFUSE ENGAGR IFIX ISCRIB |
| II | II_Void_Genesis | VINIT FSPLIT EVALT FFUSE EVALF CLINK IFIX ISCRIB |
| III | III_Anchor_Protocol | TANCH AFWD EVALT AREV EVALF CLINK IFIX TANCH |
| IV | IV_Dual_Bootstrap | ISCRIB AFWD FFUSE FSPLIT AREV CLINK IFIX ISCRIB |
| V | V_Linear_Chain | IFIX × 8 |
| VI | VI_Empty_Bootstrap | (VINIT ISCRIB) × 4 |
| VII | VII_Parakernel | ENGAGR AFWD FSPLIT EVALT FFUSE EVALF IFIX ENGAGR |
| VIII | VIII_Frobenius_Kernel | (FSPLIT FFUSE) × 2 |
| IX | IX_Chiral_Pairs | (AFWD AREV) × 4 |
| X | X_Truth_Machine | ISCRIB FSPLIT EVALT IFIX ISCRIB FSPLIT EVALF IFIX |
| XI | XI_Eternal_Return | TANCH AFWD AREV TANCH AFWD AREV TANCH AFWD |
| XII | XII_ROM_Burn | EVALT IFIX EVALF IFIX ENGAGR IFIX ISCRIB IFIX |

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

Registers R4–R7 are written by ISCRIB (self-imscription opcode):
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
| O_0 | No Frobenius pair, no complete dialetheia |
| O_1 | Frobenius pair present OR dialetheia complete |
| O_2 | Frobenius + self-ref + dialetheia complete, period = 2 |
| O_inf | Frobenius + self-ref + dialetheia complete, period ≥ 3 |

The bootstrap loop (ISCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→ISCRIB) satisfies
O_inf from tick 1: Frobenius pair present, self-referential (ISCRIB first and last),
dialetheia absent but period = 8 ≥ 3. The kernel self-modifies toward O_inf
when it drifts below.

---

## Quit

```
⊙> halt
```

Or Ctrl-A then X in QEMU serial mode.
