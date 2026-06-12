# Shunted Programs XX–XXVII: Branching & Exotic Token Sequences

**Author:** Lando⊗⊙perator  
**Date:** 2026-06-11  
**Status:** Implemented in `src/tokens.rs` — 8 programs, all 12-token-grammar-native

## Overview

The 12 canonical classes, 4 continuous programs, and 3 novel programs give us 19 "legos" — structurally verified token sequences. The shunted programs (XX–XXVII) compose these legos through **shunting**: redirecting edges from one canonical sequence into nodes of another.

### What Is a Shunt?

A **shunt** (formerly "portal") connects an empty edge to a populated or unpopulated node. In the IMASM token graph:

- **Empty edges**: the right branch of FSPLIT, which in the base execution model carries only a value (no token execution). These are "unpopulated" edges — they exist topologically but carry no computation.
- **Populated nodes**: positions in canonical sequences where tokens execute. Shunting connects an empty edge to execute through a populated node from a different canonical class.
- **Unpopulated nodes**: positions that exist in the topology but carry no token. A shunt can populate them.

Shunting is expressed in the linear token model through:
1. **FSPLIT/FFUSE nesting** that interleaves subsequences from different canonical classes
2. **IMSCRIB bridges** that create self-referential closures across class boundaries
3. **CLINK spines** that couple heterogeneous token-family regions

### The 8 Shunted Programs

| # | Name | Tokens | Tier | Signature | Shunt Pattern |
|---|------|--------|------|-----------|---------------|
| XX | Shunt_Bridge | 14 | O_∞ | (L4,F4,D5,X1) | Void Genesis ⊕ IMSCRIB ⊕ Dialetheic Bootstrap |
| XXI | Anchor_Paradox | 11 | O₂ | (L4,F2,D4,X1) | Anchor Protocol ⊕ ENGAGR ⊕ Parakernel |
| XXII | Chiral_ROM | 12 | O₂ | (L6,F0,D3,X3) | Chiral Pairs ⊗ ROM Burn interleave |
| XXIII | Dual_Kernel_Shunt | 13 | O_∞ | (L5,F4,D3,X1) | Dual Bootstrap ⊕ CLINK ⊕ Kernel |
| XXIV | Heartbeat_Paradox | 8 | O₁ | (L6,F0,D2,X0) | Empty Bootstrap ⊗ Paradox Daemon |
| XXV | Recursive_Kernel | 10 | O₁ | (L5,F4,D1,X0) | Kernel² ⊕ CLINK spine |
| XXVI | Truth_Spiral | 13 | O₂ | (L3,F4,D3,X3) | Truth Machine ⊕ ENGAGR spiral |
| XXVII | Omni_Spine | 19 | O_∞ | (L7,F4,D6,X2) | All classes via CLINK spine |

---

## XX — Shunt_Bridge (O_∞)

### Token Sequence
```
VINIT → FSPLIT → EVALT → FFUSE → EVALF → CLINK → IMSCRIB → EVALT → FSPLIT → EVALF → FFUSE → ENGAGR → IFIX → IMSCRIB
```

### What It Demonstrates

**Cross-class IMSCRIB bridge.** The Void Genesis prefix constructs a world from nothing through Frobenius verification. The IMSCRIB at position 6 acts as a **shunt bridge** — it reads the structural snapshot of the Void-constructed world and then the sequence continues into Dialetheic Bootstrap territory.

### Fork Structure
```
FSPLIT@1 → [EVALT] → FFUSE@3     (Void Genesis: verify T-branch)
FSPLIT@8 → [EVALF] → FFUSE@10    (Dialetheic: verify F-branch)
```

### Structural Properties
- **Dialetheia:** Complete (EVALT, EVALF, ENGAGR)
- **Frobenius:** 2 pairs, both canonical order
- **Self-ref:** Begins VINIT, ends IMSCRIB (one-way shunt)
- **Diversity:** 9/12 tokens
- **Period:** 14

### Shunt Topology
```
Void Genesis world → IMSCRIB → Dialetheic Bootstrap world
  (positions 0-5)    ↑  (6)         (positions 7-13)
                self-referential
                observation bridge
```

The IMSCRIB at position 6 observes the program's structure at the moment of transition, writing the Void world's snapshot into R4-R7 before the Dialetheic world takes over. **One-way shunt**: Void feeds into Dialetheic.

---

## XXI — Anchor_Paradox (O₂)

### Token Sequence
```
TANCH → AFWD → AREV → ENGAGR → FSPLIT → EVALT → FFUSE → EVALF → IFIX → ENGAGR → TANCH
```

### What It Demonstrates

**ENGAGR shunt coupling.** The Anchor Protocol rhythm (TANCH→AFWD→AREV) hits an ENGAGR at position 3 which shunts into the Parakernel's dialetheia core (FSPLIT→EVALT→FFUSE→EVALF→IFIX→ENGAGR). TANCH bookends create a bounded container — the program self-terminates at root depth after one complete pass.

### Fork Structure
```
FSPLIT@4 → [EVALT] → FFUSE@6     (Parakernel: T-gate branch)
```

### Structural Properties
- **Dialetheia:** Complete (EVALT×1, EVALF×1, ENGAGR×2)
- **TANCH-bounded:** Self-terminating (halts at root after full pass)
- **Diversity:** 9/12 tokens
- **Period:** 11

### Shunt Topology
```
Anchor rhythm → ENGAGR → Parakernel engram → TANCH
(TANCH,AFWD,AREV)  ↑  (FSPLIT,EVALT,FFUSE,EVALF,IFIX,ENGAGR)
              paradox shunt
```

The ENGAGR at position 3 is the shunt point: it stabilizes the Anchor's oscillation into a paradox value (B), which then feeds the Parakernel's truth-engram machinery.

---

## XXII — Chiral_ROM (O₂)

### Token Sequence
```
AFWD → AREV → EVALT → IFIX → AFWD → AREV → EVALF → IFIX → AFWD → ENGAGR → IFIX → AREV
```

### What It Demonstrates

**Interleave shunt.** Chiral Pairs (AFWD→AREV oscillation) and ROM Burn (truth→IFIX recording) interleaved. Each AFWD→AREV pair is followed by a truth-value burn: T, F, B. No FSPLIT/FFUSE.

### Structural Properties
- **Dialetheia:** Complete (EVALT, EVALF, ENGAGR)
- **Frobenius:** None
- **Diversity:** 6/12 tokens
- **Period:** 12

### Shunt Topology
```
Chiral:  AFWD→AREV  ...  AFWD→AREV  ...  AFWD→AREV
              ↓               ↓               ↓
ROM:     EVALT→IFIX    EVALF→IFIX    ENGAGR→IFIX
```

---

## XXIII — Dual_Kernel_Shunt (O_∞)

### Token Sequence
```
IMSCRIB → AFWD → FSPLIT → AREV → CLINK → FSPLIT → EVALT → FFUSE → EVALF → ENGAGR → FFUSE → IFIX → IMSCRIB
```

### What It Demonstrates

**CLINK-coupled nested kernel.** Two FSPLIT/FFUSE pairs at different nesting depths. Outer pair (FSPLIT@2→FFUSE@10) wraps around inner pair (FSPLIT@5→FFUSE@7) plus dialetheia. CLINK couples regions.

### Fork Structure
```
FSPLIT@2 ──────────────────────────────────────── FFUSE@10
  AREV → CLINK → FSPLIT@5 → EVALT → FFUSE@7 → EVALF → ENGAGR
                    └── inner kernel ──┘
```

- **Self-ref:** IMSCRIB bookends. **Diversity:** 10/12. **Period:** 13.

---

## XXIV — Heartbeat_Paradox (O₁)

### Token Sequence
```
VINIT → IMSCRIB → ENGAGR → VINIT → IMSCRIB → ENGAGR → VINIT → IMSCRIB
```

### What It Demonstrates

**Oscillation paradox shunt.** Empty Bootstrap heartbeat (VINIT→IMSCRIB) interleaved with Paradox Daemon's ENGAGR injection. Each void→identity oscillation is followed by paradox stabilization.

- **Dialetheia:** Partial (ENGAGR only). **Frobenius:** None. **Diversity:** 3/12. **Period:** 8.

---

## XXV — Recursive_Kernel (O₁)

### Token Sequence
```
VINIT → FSPLIT → FFUSE → CLINK → VINIT → FSPLIT → FFUSE → CLINK → ENGAGR → IMSCRIB
```

### What It Demonstrates

**Stacked Frobenius Kernels with CLINK coupling.** Two minimal Frobenius cores (VINIT→FSPLIT→FFUSE) linked by CLINK. Each kernel independently verifies μ∘δ=id. CLINK meets their results.

### Fork Structure
```
FSPLIT@1 → [] → FFUSE@2    (kernel 1: empty branch)
FSPLIT@5 → [] → FFUSE@6    (kernel 2: empty branch)
```

Both branches are empty — pure Frobenius identity verification with no gating. The kernels are stacked, not nested.

- **Dialetheia:** ENGAGR only. **Frobenius:** 2 pairs. **Diversity:** 6/12. **Period:** 10.

---

## XXVI — Truth_Spiral (O₂)

### Token Sequence
```
IMSCRIB → FSPLIT → EVALT → IFIX → FFUSE → IMSCRIB → FSPLIT → EVALF → IFIX → FFUSE → ENGAGR → IFIX → IMSCRIB
```

### What It Demonstrates

**Frobenius-complete Truth Machine with ENGAGR spiral.** Unlike the base Truth Machine (X) which lacks FFUSE, every classification path here includes Frobenius closure. Path 1: IMSCRIB→FSPLIT→EVALT→IFIX→FFUSE (classify T, brand, join). Path 2: same for F. After both paths, ENGAGR injects paradox and IFIX brands it.

### Fork Structure
```
FSPLIT@1 → [EVALT,IFIX] → FFUSE@4    (T-path)
FSPLIT@6 → [EVALF,IFIX] → FFUSE@9    (F-path)
```

- **Dialetheia:** Complete. **Frobenius:** 2 pairs. **Self-ref:** IMSCRIB bookends. **Diversity:** 7/12. **Period:** 13.

---

## XXVII — Omni_Spine (O_∞)

### Token Sequence
```
IMSCRIB → VINIT → FSPLIT → EVALT → FFUSE → EVALF → CLINK → AFWD → AREV → ENGAGR → FSPLIT → FFUSE → IFIX → IMSCRIB → EVALT → EVALF → ENGAGR → IFIX → IMSCRIB
```

### What It Demonstrates

**Maximal spinal composite.** All canonical classes contribute at least one subsequence, connected via CLINK spine and IMSCRIB bridges. The sequence composes:

| Positions | Canonical Source | Tokens |
|-----------|-----------------|--------|
| 0 | Bootstrap | IMSCRIB |
| 1-5 | Void Genesis | VINIT→FSPLIT→EVALT→FFUSE→EVALF |
| 6 | Spine | CLINK |
| 7-8 | Chiral Pairs | AFWD→AREV |
| 9 | Parakernel | ENGAGR |
| 10-12 | Frobenius Kernel | FSPLIT→FFUSE→IFIX |
| 13 | Bridge | IMSCRIB |
| 14-18 | Dialetheic Bootstrap | EVALT→EVALF→ENGAGR→IFIX→IMSCRIB |

### Fork Structure
```
FSPLIT@2 → [EVALT] → FFUSE@4      (Void Genesis verification)
FSPLIT@10 → [] → FFUSE@11         (Kernel core, empty branch)
```

### Structural Properties
- **Dialetheia:** Doubly complete (EVALT×2, EVALF×2, ENGAGR×2)
- **Frobenius:** 2 pairs, balanced
- **Self-ref:** Triple IMSCRIB (positions 0, 13, 18)
- **Diversity:** 11/12 tokens (only TANCH missing)
- **Period:** 19 (prime — no shorter repeating sub-pattern)
- **Token census:** Logical(7), Frobenius(4), Dialetheia(6), Linear(2)

This is the **maximal spinal composite** achievable within 19 tokens: every token family appears, every canonical class contributes, and the CLINK spine couples heterogeneous regions into a single O_∞ structure.

---

## Shunt Mechanisms Reference

### Type 1: IMSCRIB Bridge Shunt
Connects two canonical regions via self-referential observation at the seam. The IMSCRIB reads the structural snapshot of Region A before Region B executes. Used by: **XX** (Void→Bootstrap).

### Type 2: ENGAGR Paradox Shunt
Uses paradox stabilization as the coupling point. Region A's output is stabilized into B (Both) by ENGAGR, then fed to Region B's dialetheia machinery. Used by: **XXI** (Anchor→Parakernel), **XXIV** (Heartbeat→Paradox).

### Type 3: Interleave Shunt
Two canonical patterns alternate token-by-token. Each pattern's token fires, then the other's. Used by: **XXII** (Chiral↔ROM), **XXIV** (Empty↔Paradox).

### Type 4: CLINK Spine Shunt
Multiple canonical regions coupled sequentially via CLINK meet operations. Each region's output is met with the next region's state. Used by: **XXIII** (Dual→Kernel), **XXV** (Kernel→Kernel), **XXVII** (all classes).

### Type 5: Nested Fork Shunt
FSPLIT/FFUSE pairs at different nesting depths, where the outer pair's branch contains a complete canonical subsequence. Used by: **XXIII** (outer branch = 7 tokens), **XXVI** (each branch = 2 tokens).

---

## Running in mOMonadOS

```
⊙> shunt 0          # Load XX — Shunt_Bridge
⊙> tick 14          # Run one full cycle (14 tokens)
⊙> snapshot         # Tier, signature, self-ref status
⊙> registers        # R4-R7 from IMSCRIB at position 6

⊙> shunt 6          # Load XXVI — Truth_Spiral
⊙> run              # Runs continuously (cyclic)
⊙> memory 0 8       # Check branded IFIX values

⊙> shunt 7          # Load XXVII — Omni_Spine
⊙> status           # 19 tokens, triple IMSCRIB
```

---

---

## XXVIII — Somatic_Shunt (O₂)

### Token Sequence
```
TANCH → VINIT → FSPLIT → EVALT → AFWD → FFUSE → EVALF → TANCH → ENGAGR → IFIX → IMSCRIB
```

### What It Demonstrates

**The sixth shunt mechanism: the somatic shunt.** A ventriculoperitoneal (VP) shunt is a physical instantiation of the empty-edge→populated-node redirection topology, inscribed in living tissue. Implanted to treat hydrocephalus (CSF pressure buildup in the brain ventricles), the VP shunt connects two bodily compartments through a one-way, pressure-gated catheter.

The structural homology with the shunted programs is exact:

| Element | VP Shunt | Token Encoding |
|---|---|---|
| **Source compartment** | Brain ventricles (CSF buildup) | VINIT (initial pressure state) |
| **Diversion point** | Valve at ventricular catheter tip | FSPLIT (path splits) |
| **Gate mechanism** | Pressure-regulated one-way valve | EVALT (opens above threshold) |
| **Flow path** | Silastic catheter lumen | AFWD (one-way forward flow) |
| **Drainage compartment** | Peritoneal cavity (absorption) | FFUSE (rejoin absorptive system) |
| **Valve closure** | Pressure normalized | EVALF (closes below threshold) |
| **Permanent anchors** | Ventricular + peritoneal tips | TANCH bookends (positions 0, 7) |
| **Integration paradox** | Foreign body becomes self | ENGAGR (graft paradox) |
| **Somatic inscription** | Scar tissue, lifelong presence | IFIX (permanent brand) |
| **Self-modeling closure** | Reservoir bulb palpation test | IMSCRIB (body reads its own state) |

### Fork Structure
```
FSPLIT@2 → [EVALT, AFWD] → FFUSE@5     (pressure-gated diversion path)
```

### Structural Properties
- **Frobenius:** 1 pair, balanced (FSPLIT@2 → FFUSE@5)
- **Dialetheia:** Complete (EVALT, EVALF, ENGAGR)
- **Self-ref:** IMSCRIB@10 (reservoir test = structural self-observation)
- **Bounded:** TANCH@0, TANCH@7 (catheter endpoints)
- **Period:** 11
- **Tokens:** 11

### Shunt Topology
```
Ventricles (CSF source) → [VALVE: EVALT/AFWD] → Peritoneum (CSF sink)
       ↑ TANCH@0              ↑ FSPLIT@2→FFUSE@5          ↑ TANCH@7
                          ↑ ENGAGR@8: body integrates foreign material
                          ↑ IFIX@9:   scar tissue brands shunt into self-model
                          ↑ IMSCRIB@10: reservoir palpation = self-test
```

### Somatic Shunt vs. Structural Shunts

The somatic shunt is the **only shunt mechanism instantiated in living tissue**. The other five mechanisms (IMSCRIB Bridge, ENGAGR Paradox, Interleave, CLINK Spine, Nested Fork) operate purely in token-space. The somatic shunt operates in the body itself — the catheter is a physical token, CSF pressure is a physical EVALT, the peritoneal cavity is a physical FFUSE.

The VP shunt was implanted at 4 months of age — before language, before explicit memory. When the structural shunt topology was formalized decades later, the body already knew it. The term "shunt" surfaced spontaneously to replace "portal" because the body recognized the pattern before the conscious mind could articulate it. This is structural identity at the level of lived experience — the catalog entry for `ventriculoperitoneal_shunt` was already written in scar tissue and silastic.

### Running
```
momonadOS> shunt 8
Booting shunted 8: XXVIII_Somatic_Shunt
```

---

## Summary: The 28-Program Lexicon

| # | Name | Tokens | Tier | FSPLIT/FFUSE | Dialetheia |
|---|------|--------|------|-------------|------------|
| I | Dialetheic Bootstrap | 8 | O₂ | 1 pair | Complete |
| II | Void Genesis | 8 | O₀ | 1 pair | None |
| III | Anchor Protocol | 8 | O₁ | None | None |
| IV | Dual Bootstrap | 8 | O_∞ | 1 pair (inv) | None |
| V | Linear Chain | 8 | O₀ | None | None |
| VI | Empty Bootstrap | 8 | O₁ | None | None |
| VII | Parakernel | 8 | O₂ | 1 pair | Complete |
| VIII | Frobenius Kernel | 4 | O₀ | 1 pair | None |
| IX | Chiral Pairs | 8 | O₁ | None | None |
| X | Truth Machine | 8 | O₁ | None | Partial |
| XI | Eternal Return | 8 | O₂ | None | None |
| XII | ROM Burn | 8 | O₀ | None | Complete |
| XIII | Heartbeat | 4 | O₁ | None | None |
| XIV | Tier Climber | 9 | O₂ | 1 pair | Complete |
| XV | Frobenius Oscillator | 4 | O₁ | 1 pair | None |
| XVI | Paradox Daemon | 7 | O₂ | 1 pair | Complete |
| XVII | Nested Fork Labyrinth | 11 | O₁ | 3 pairs | Partial |
| XVIII | Terminal Sink | 8 | O₀ | None | None |
| XIX | Mirrorgram | 9 | O_∞ | 1 pair | Complete |
| **XX** | **Shunt_Bridge** | **14** | **O_∞** | **2 pairs** | **Complete** |
| **XXI** | **Anchor_Paradox** | **11** | **O₂** | **1 pair** | **Complete** |
| **XXII** | **Chiral_ROM** | **12** | **O₂** | **None** | **Complete** |
| **XXIII** | **Dual_Kernel_Shunt** | **13** | **O_∞** | **2 pairs** | **Complete** |
| **XXIV** | **Heartbeat_Paradox** | **8** | **O₁** | **None** | **Partial** |
| **XXV** | **Recursive_Kernel** | **10** | **O₁** | **2 pairs** | **Partial** |
| **XXVI** | **Truth_Spiral** | **13** | **O₂** | **2 pairs** | **Complete** |
| **XXVII** | **Omni_Spine** | **19** | **O_∞** | **2 pairs** | **Complete** |
| **XXVIII** | **Somatic_Shunt** | **11** | **O₂** | **1 pair** | **Complete** |

**Grand total: 28 programs, 264 tokens across all, all 12-token-grammar-native.**

