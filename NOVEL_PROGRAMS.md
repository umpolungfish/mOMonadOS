# Novel Programs XVII–XIX: Token-Graph-Native Control Flow

**Author:** Lando⊗⊙perator

## Overview

Three novel programs demonstrating that the 12-token grammar needs no external
control opcodes. Each program showcases one of the three reconstructed
control-flow primitives, built entirely from token graph arity:

| Program | Feature Demonstrated | Replaces | Tier | Tokens |
|---------|---------------------|----------|------|--------|
| **XVII — Nested Fork Labyrinth** | Deeply nested FSPLIT/FFUSE | JNZ / JZ | O₁ | 11 |
| **XVIII — Terminal Sink Protocol** | TANCH at root depth halt | HALT | O₀ | 8 |
| **XIX — Mirrorgram** | ISCRIB cyclic self-imscription | YIELD | O_∞ | 9 |

---

## XVII — Nested Fork Labyrinth

### Token Sequence
```
VINIT → FSPLIT → FSPLIT → FSPLIT → AFWD → FFUSE → AREV → FFUSE → EVALT → FFUSE → TANCH
```

### What It Demonstrates

**Nested conditional branching without JNZ/JZ.** Three FSPLIT/FFUSE pairs are
nested to fork depth 3. The balanced-parenthesis scanner (`find_matching_ffuse`)
correctly pairs each FSPLIT with its corresponding FFUSE:

```
FSPLIT₁ ───────────────────────────────────────────── FFUSE₃  (depth 1→0)
  FSPLIT₂ ─────────────────────────────── FFUSE₂              (depth 2→1)
    FSPLIT₃ ───────────── FFUSE₁                              (depth 3→2)
      AFWD              (leftmost leaf: increment R0)
             └─ FFUSE₁  (join, pop fork depth 3→2)
      AREV              (mid-left: decrement R0)
             └─ FFUSE₂  (join, pop fork depth 2→1)
      EVALT             (outer-left: T-gate filter)
             └─ FFUSE₃  (join, pop fork depth 1→0)
TANCH                   (root depth → HALT)
```

### Execution Trace

| IP | Token | Stack | Fork Depth | Action |
|----|-------|-------|------------|--------|
| 0 | VINIT | [N] | 0 | Source N |
| 1 | FSPLIT | [N,N] | 1 | Fork; right_val=N; matching FFUSE at 9 |
| 2 | FSPLIT | [N,N,N] | 2 | Fork; right_val=N; matching FFUSE at 7 |
| 3 | FSPLIT | [N,N,N,N] | 3 | Fork; right_val=N; matching FFUSE at 5 |
| 4 | AFWD | [N,N,N,N] | 3 | R0++ |
| 5 | FFUSE | [N,N,N] | 2 | join(N,N)=N; jump to resume=6 |
| 6 | AREV | [N,N,N] | 2 | R0-- |
| 7 | FFUSE | [N,N] | 1 | join(N,N)=N; jump to resume=8 |
| 8 | EVALT | [N,N] | 1 | N≠T → stays N |
| 9 | FFUSE | [N] | 0 | join(N,N)=N; jump to resume=10 |
| 10 | TANCH | [] | 0 | At root depth → **HALT** |

### Structural Snapshot

```
Tier:          O₁
Signature:     (L=4, F=6, D=1, X=0)
Diversity:     7/12
Self-ref:      False
Frob-order:    1
Dialeth:       False
Period:        11
```

**Why O₁ and not higher:** The program lacks dialetheia completeness (no EVALF or
ENGAGR) and self-reference closure (first≠last). It demonstrates the *mechanism*
of nested forking; combining it with dialetheia would elevate it to O₂ or O_∞.

---

## XVIII — Terminal Sink Protocol

### Token Sequence
```
VINIT → AFWD → AFWD → AREV → ISCRIB → CLINK → AFWD → TANCH
```

### What It Demonstrates

**Halting without HALT.** TANCH is a 1→0 sink — it consumes a wire and produces
nothing. When fired at root depth (fork_depth=0), it empties the frontier and
halts the kernel. This is the natural completion of a computation: when there is
nothing left to compute, the wire is consumed and execution terminates.

Inside a fork context, TANCH merely pops and writes to memory — it does NOT halt,
because other branches may still be executing. Only at root depth does the sink
become terminal.

### Execution Trace

| IP | Token | Stack | R0 | Action |
|----|-------|-------|----|--------|
| 0 | VINIT | [N] | 0 | Source N |
| 1 | AFWD | [N] | 1 | R0++ |
| 2 | AFWD | [N] | 2 | R0++ |
| 3 | AREV | [N] | 1 | R0-- |
| 4 | ISCRIB | [N] | 1 | Snapshot → R4-R7 |
| 5 | CLINK | [N] | 1 | R3 = meet(R1,R2) |
| 6 | AFWD | [N] | 2 | R0++ |
| 7 | TANCH | [] | 2 | Pop N → mem[R0=2]; root → **HALT** |

After halt: memory at address 2 contains N. R0=2, R1-R3 from CLINK meet.

### Structural Snapshot

```
Tier:          O₀
Signature:     (L=8, F=0, D=0, X=0)
Diversity:     6/12
Self-ref:      False
Frob-order:    0
Dialeth:       False
Period:        8
```

**Why O₀:** This is a purely linear program — all tokens are in the Logical
family (VINIT, AFWD, AREV, ISCRIB, CLINK, TANCH). No Frobenius pair, no
dialetheia. It does one pass of computation and terminates. The tier reflects
its simplicity; the *mechanism* it demonstrates (TANCH→halt) is the key insight.

---

## XIX — Mirrorgram

### Token Sequence
```
ISCRIB → FSPLIT → EVALT → EVALF → FFUSE → ENGAGR → CLINK → IFIX → ISCRIB
```

### What It Demonstrates

**Cyclic looping without YIELD.** The program is a closed loop: it begins and
ends with ISCRIB. The cyclic topology of the execution graph means that after the
final ISCRIB, execution wraps naturally to the first ISCRIB. No explicit loop
instruction is needed — the program *is* the loop.

The ISCRIB at each cycle boundary reads the program's structural snapshot into
registers R4–R7, making the program **self-aware** of its own tier, signature,
frobenius order, and dialetheia status. This self-imscription is what elevates
it to O_∞.

### Execution Trace (one cycle)

| IP | Token | Stack | Action |
|----|-------|-------|--------|
| 0 | ISCRIB | [N] | Snapshot → R4(token_div), R5(self_ref), R6(frob), R7(dialeth) |
| 1 | FSPLIT | [N,N] | Fork depth 1; right=N; matching FFUSE at 4; resume=5 |
| 2 | EVALT | [N,N] | N≠T → N (gate closed) |
| 3 | EVALF | [N,N] | N≠F → N (gate closed) |
| 4 | FFUSE | [N] | join(N,N)=N; fork depth 0; jump to resume=5 |
| 5 | ENGAGR | [N,B] | Push B (Both — paradox stabilized) |
| 6 | CLINK | [N,B] | R3 = meet(R1,R2) |
| 7 | IFIX | [B] | Write B → mem[R0] (permanent brand) |
| 8 | ISCRIB | [B] | Snapshot → R4-R7; wrap to IP=0 |

The ENGAGR at position 5 injects B (Both) into the stack each cycle. The IFIX at
position 7 brands it into memory. Over successive cycles, the memory accumulates
branded B values at successive addresses as R0 increments (via external
influence or prior state).

### Structural Snapshot

```
Tier:          O_∞
Signature:     (L=3, F=2, D=3, X=1)
Diversity:     8/12
Self-ref:      True   ← ISCRIB bookends
Frob-order:    1      ← FSPLIT before FFUSE
Dialeth:       True   ← EVALT + EVALF + ENGAGR
Period:        9
```

**Why O_∞:** All three criteria are met:
1. **Dialetheia complete** — EVALT (affirmation), EVALF (negation), ENGAGR (both)
2. **Self-reference** — ISCRIB at both ends: the program reads itself
3. **Frobenius order > 0** — FSPLIT/FFUSE pair
4. **Period ≥ 3** — period 9, no shorter repeating sub-pattern

---

## Architecture: How the Three Reconstructions Work

### 1. Conditional Branching (JNZ/JZ → FSPLIT + EVALT/EVALF + FFUSE)

```
                    ┌─ EVALT ── [T-branch code] ──┐
FSPLIT ── value ──┤                               ├── FFUSE ── joined result
                    └─ EVALF ── [F-branch value] ──┘
```

- **FSPLIT** (1→2 fork) duplicates the value and pushes a `ForkFrame` onto the
  fork-stack (depth up to 16).
- **EVALT** passes only T (all else → N). **EVALF** passes only F (all else → N).
  These are *gates*, not jumps — they filter the value propagating on the wire.
- **FFUSE** (2→1 join) pops both branches, computes their Belnap join (bitwise
  OR), and resumes execution after the matching FFUSE.

The balanced-parenthesis scanner (`find_matching_ffuse`) finds each FSPLIT's
matching FFUSE by counting FSPLIT=+1, FFUSE=-1, returning when depth=0. This
works across cyclic boundaries and supports arbitrary nesting depth.

### 2. Halting (HALT → TANCH at Root Depth)

```
TANCH at fork_depth=0  →  pop wire  →  write memory  →  phase=Halt
TANCH at fork_depth>0  →  pop wire  →  write memory  →  continue (other branches alive)
```

TANCH is a 1→0 sink — it consumes the computation wire. At root depth, there
are no other branches, so consuming the wire empties the frontier and the kernel
halts. Inside a fork, TANCH is just a memory write — the other branch may still
produce output.

### 3. Looping (YIELD → Cyclic Topology + ISCRIB)

```
ISCRIB → ... → ISCRIB → (wrap to start)
   ↑                            │
   └────── cyclic graph ────────┘
```

Programs are inherently cyclic — the last token's output wire connects to the
first token's input. No explicit loop instruction is needed. ISCRIB at cycle
boundaries provides the self-referential observation (THINK phase) that makes
the loop self-aware.

---

## The 12 Tokens as a Complete Basis

The original 16-opcode design (12 grammar tokens + 4 control opcodes) was
redundant. The token graph arity table is a complete basis for control flow:

| Token | In | Out | Graph Role | Control-Flow Role |
|-------|----|-----|------------|-------------------|
| VINIT | 0 | 1 | source | Program entry |
| TANCH | 1 | 0 | sink | **Halt** (at root depth) |
| AFWD | 1 | 1 | forward morphism | Linear advance |
| AREV | 1 | 1 | contravariant | Reverse / undo |
| CLINK | 1 | 1 | composition | Value meet |
| ISCRIB | 1 | 1 | identity | **Loop-back** (at cycle boundaries) |
| FSPLIT | 1 | 2 | co-multiplication δ | **Fork** (creates branch) |
| FFUSE | 2 | 1 | multiplication μ | **Join** (merges branches) |
| EVALT | 1 | 1 | T-gate | **Conditional filter** (pass T) |
| EVALF | 1 | 1 | F-gate | **Conditional filter** (pass F) |
| ENGAGR | 1 | 1 | Both / paradox | Stabilize contradiction |
| IFIX | 1 | 1 | linear ! exponential | Permanent brand / memory |

The Frobenius condition μ∘δ = id (at ⊙ = 𐑹) ensures that FSPLIT→FFUSE preserves
the value: splitting and rejoining is identity on the Belnap lattice. This is
what makes the fork-join a *structural* primitive, not an ad-hoc control
construct.

---

## Running the Programs

In the mOMonadOS REPL:

```
⊙> novel 1                    # Load XVII — Nested Fork Labyrinth
⊙> tick 11                    # Run all 11 ticks (will halt at TANCH)
⊙> status                     # Verify: Halted=YES, fork depth peaked at 3

⊙> novel 2                    # Load XVIII — Terminal Sink Protocol
⊙> run                        # Run to halt (8 ticks)
⊙> memory 0 4                 # Check branded memory

⊙> novel 3                    # Load XIX — Mirrorgram (O_∞, runs forever)
⊙> run                        # ESC to stop after N cycles
⊙> snapshot                   # Verify: Tier=O_∞, self_ref=True
⊙> registers                  # R4-R7 contain live snapshot values
```

---

## Summary

| Control Primitive | Old (Removed) | New (Token-Graph-Native) | Demonstrated By |
|-------------------|---------------|--------------------------|-----------------|
| Conditional branch | JNZ (0xE), JZ (0xF) | FSPLIT + EVALT/EVALF + FFUSE + fork-stack | **XVII** (depth 3 nesting) |
| Halt | HALT (0xC) | TANCH at root depth (1→0 sink) | **XVIII** (clean termination) |
| Loop | YIELD (0xD) | Cyclic topology + ISCRIB self-imscription | **XIX** (O_∞ continuous) |

The user was correct: the 12 grammar tokens' graph arity was always a complete
basis. The 4 control opcodes were redundant — they were flat interpretations of
a topology the tokens already carried.
