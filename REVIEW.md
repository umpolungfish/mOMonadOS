# mOMonadOS Review: From Stepwise to Continuous Execution

**Author:** Lando⊗⊙perator

---

## 1. The Architecture Before

mOMonadOS originally shipped with 12 grammar tokens across four families plus 4 external
control opcodes: **HALT** (0xC), **YIELD** (0xD), **JNZ** (0xE), and **JZ** (0xF). The
control family was imported from conventional VM design — halt, yield, conditional jump.
They sat outside the grammar, bolted onto the side of an otherwise self-contained 12-token
instruction set.

The execution model was stepwise: a linear instruction pointer advanced through the
program, YIELD wrapped it, JNZ/JZ jumped it, HALT stopped it. Each tick was one opcode.
This worked, but it treated the token stream as a flat list. The Frobenius structure
of the grammar — the fact that FSPLIT and FFUSE are δ and μ of a Frobenius algebra,
that tokens carry inherent graph arity — was mechanically present but semantically
underused.

## 2. The User's Observation

The user observed that the 4 control opcodes were redundant. The tokens are not flat;
they carry **graph arity** — each token has a number of input wires and output wires:

```
┌─────────┬─────┬─────┬───────────────────┐
│  Token  │ In  │ Out │    Graph role     │
├─────────┼─────┼─────┼───────────────────┤
│ VINIT   │ 0   │ 1   │ source            │
│ TANCH   │ 1   │ 0   │ sink              │
│ AFWD    │ 1   │ 1   │ linear morphism   │
│ AREV    │ 1   │ 1   │ linear morphism   │
│ CLINK   │ 1   │ 1   │ composition/meet  │
│ ISCRIB  │ 1   │ 1   │ self-imscription  │
│ FSPLIT  │ 1   │ 2   │ fork (δ)          │
│ EVALT   │ 1   │ 1   │ T-gate            │
│ EVALF   │ 1   │ 1   │ F-gate            │
│ FFUSE   │ 2   │ 1   │ join (μ)          │
│ ENGAGR  │ 1   │ 1   │ paradox-stabilize │
│ IFIX    │ 1   │ 1   │ permanent write   │
└─────────┴─────┴─────┴───────────────────┘
```

From this arity graph alone, all control flow can be constructed. The 4 external
opcodes were not gaps in the grammar — they were shadows that the grammar's arity
graph already cast.

## 3. How Each Control Primitive Was Rebuilt

### 3.1 Halting: TANCH at Root Depth

**Was:** HALT (0xC) — an explicit stop instruction.

**Now:** TANCH is a 1→0 sink. It consumes a stack value and writes it to memory.
When TANCH fires **outside any fork context** (fork depth = 0), there are no
remaining wires — the computation frontier is empty. The kernel sets `halted = true`
and the tick loop terminates.

A TANCH inside a fork is an ordinary sink: it consumes the branch-local value and
the fork continues. Only root-depth TANCH halts. This distinction is structural,
not encoded — it follows from the fork-stack topology.

### 3.2 Looping: Cyclic Graph Topology

**Was:** YIELD (0xD) — an explicit loop-back instruction.

**Now:** Programs are inherently cyclic. The last token's output wire connects to the
first token's input. When the IP reaches `program.len()`, it wraps to 0. No YIELD
instruction is needed because the graph is already a cycle.

ISCRIB at cycle boundaries provides self-referential loop-back: it computes a fresh
snapshot and writes structural data into registers R4–R7. Each full cycle is one
winding of the Frobenius loop. The cyclic topology is not simulated — it is the
native execution model.

### 3.3 Conditional Branching: FSPLIT → EVALT/EVALF → FFUSE

**Was:** JNZ (0xE) and JZ (0xF) — explicit conditional jumps using register R0 as target.

**Now:** The fork-join subgraph provides conditional branching natively:

1. **FSPLIT** (1→2 fork) bifurcates execution. The current stack value is duplicated
   to both the left branch (continuing inline) and the right branch (stored in a
   `ForkFrame`). A balanced-parenthesis scan finds the matching FFUSE.

2. **EVALT / EVALF** act as **gates**. EVALT pops the stack and passes only `T`
   (all other values become `N`). EVALF passes only `F`. These gates sit on
   the left branch after the fork, filtering which values proceed.

3. **FFUSE** (2→1 join) pops the fork frame, joins left and right branch values
   via Belnap join (⊔), and jumps the IP to the instruction after FFUSE —
   the `resume_ip` stored in the `ForkFrame`.

The fork-stack supports up to 16 nested fork-join pairs, enabling arbitrarily
nested conditionals. The balanced-parenthesis scan (`find_matching_ffuse`) ensures
that nested FSPLIT/FFUSE pairs are correctly matched — the same algorithm used by
parsers for parentheses.

## 4. The Result: A 12-Token Grammar Basis

After the refactor, the instruction set is exactly 12 tokens. No external control
family. The grammar is closed under its own arity graph:

| Family | Tokens | Graph Role |
|--------|--------|-------------|
| **Logical** | VINIT, TANCH, AFWD, AREV, CLINK, ISCRIB | Sources, sinks, morphisms, composition, identity |
| **Frobenius** | FSPLIT, FFUSE | δ (co-multiplication), μ (multiplication) |
| **Dialetheia** | EVALT, EVALF, ENGAGR | T-gate, F-gate, paradox stabilization |
| **Linear** | IFIX | Permanent memory write (! modality) |

The Frobenius condition μ∘δ = id (at ⊙ = 𐑹) is structurally embedded:
FSPLIT followed by FFUSE with no gate interference is an identity on the
stack value. The EVALT/EVALF gates break this identity selectively, which is
exactly how conditional branching emerges from Frobenius structure.

## 5. Execution Model: Continuous, Not Stepwise

The kernel tick loop is now:

```
THINK → ACT → OBSERVE → UPDATE → (cyclic)
```

Each tick is one full winding through all four phases — not one opcode.
ACT executes exactly one token, but the surrounding phases perform
self-imscription (`self_imscribe()`), snapshot comparison, tier promotion
checks, and Frobenius verification. The kernel is always self-modeling.

**Continuous programs** (XIII–XVI) run without manual stepping. They use only
the 12 tokens. The REPL supports `run N` for timed execution and `cont` for
unbounded continuous running with keyboard interrupt.

## 6. What the User Was Correct About

The user's core claim — that the tokens are not flat, that their graph arity
is a complete basis for control flow — was exactly right. The 4 control opcodes
were not extensions of the grammar; they were redundant encodings of structure
already present in the 12 tokens' arity graph.

The key insight was seeing the tokens as a **graph** rather than a **list**.
Once you read VINIT as a 0→1 source and TANCH as a 1→0 sink, halting is
topological. Once you read FSPLIT as 1→2 and FFUSE as 2→1, conditional
branching is Frobenius. Once you read the program as cyclic, looping is
inherent. The arity table was the Rosetta Stone.

## 7. Current Canonical Programs

| # | Name | Tokens | Tier |
|---|------|--------|------|
| I | Dialetheic Bootstrap | ISCRIB EVALT FSPLIT EVALF FFUSE ENGAGR IFIX ISCRIB | O₁ |
| II | Void Genesis | VINIT FSPLIT EVALT FFUSE EVALF CLINK IFIX ISCRIB | O₁ |
| III | Anchor Protocol | TANCH AFWD EVALT AREV EVALF CLINK IFIX TANCH | O₁ |
| IV | Dual Bootstrap | ISCRIB AFWD FFUSE FSPLIT AREV CLINK IFIX ISCRIB | O₁ |
| V | Linear Chain | IFIX×8 | O₀ |
| VI | Empty Bootstrap | (VINIT ISCRIB)×4 | O₀ |
| VII | Parakernel | ENGAGR AFWD FSPLIT EVALT FFUSE EVALF IFIX ENGAGR | O₁ |
| VIII | Frobenius Kernel | (FSPLIT FFUSE)×2 | O₁ |
| IX | Chiral Pairs | (AFWD AREV)×4 | O₀ |
| X | Truth Machine | ISCRIB FSPLIT EVALT IFIX ISCRIB FSPLIT EVALF IFIX | O₁ |
| XI | Eternal Return | TANCH AFWD AREV TANCH AFWD AREV TANCH AFWD | O₀ |
| XII | ROM Burn | EVALT IFIX EVALF IFIX ENGAGR IFIX ISCRIB IFIX | O₁ |

| # | Continuous | Tokens | Tier |
|---|------------|--------|------|
| XIII | Heartbeat | ISCRIB×4 | O₁ |
| XIV | Tier Climber | ISCRIB FSPLIT EVALT EVALF FFUSE ENGAGR CLINK IFIX ISCRIB | O₂ |
| XV | Frobenius Oscillator | FSPLIT ISCRIB FFUSE ISCRIB | O₁ |
| XVI | Paradox Daemon | VINIT FSPLIT EVALT EVALF ENGAGR FFUSE ISCRIB | O₁ |

## 8. Ouroboricity Tiers

Programs self-imscribe via `self_imscribe()` which computes a `Snapshot`:

- **frobenius_order:** 0 (no δ/μ), 1 (δ before μ), 2 (μ before δ)
- **period:** minimal repetition period
- **signature:** (Logical, Frobenius, Dialetheia, Linear) 4-tuple
- **token_diversity:** count of distinct token types used
- **self_ref:** first token = last token
- **dialetheia_complete:** EVALT ∧ EVALF ∧ ENGAGR all present

Tier assignment:
- **O₀:** No Frobenius structure, no complete dialetheia
- **O₁:** Frobenius order > 0 OR dialetheia complete
- **O₂:** Dialetheia complete + self-ref + Frobenius order > 0 + period = 2
- **O_∞:** Dialetheia complete + self-ref + Frobenius order > 0 + period ≥ 3

The kernel's `maybe_promote()` checks every tick whether the snapshot tier has
changed. Tier promotion is structural — when a program's execution pattern shifts
its self-imscription, the tier updates automatically.

## 9. Conclusion

The refactor eliminated 4 external opcodes and reconstructed them from the 12
grammar tokens' inherent graph arity. The result is a closed grammar: every
aspect of execution — halting, looping, branching — is generated by the tokens
themselves. The Frobenius algebra (δ = FSPLIT, μ = FFUSE) is not just one family
among four; it is the structural engine from which control flow emerges.

The grammar did not need extension. It needed to be read as a graph, not a list.
