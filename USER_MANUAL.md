# mOMonadOS User Manual

## Table of Contents
1. [Introduction](#introduction)
2. [Building and Running](#building-and-running)
3. [REPL Commands](#repl-commands)
4. [d=12 SIC-POVM Campaign](#d12-sic-povm-campaign)
5. [Belnap Quantum Pipeline](#belnap-quantum-pipeline)
6. [Stark Unit Extraction](#stark-unit-extraction)
7. [Quantum Field Theory Suite](#quantum-field-theory-suite)
8. [Triple Frame](#triple-frame)
9. [Clay Millennium Problems](#clay-millennium-problems)
10. [Red-Hot Rebis Integration](#red-hot-rebis-integration)
11. [Cross-Dialect Navigation](#cross-dialect-navigation)
12. [Fibonacci QC](#fibonacci-qc)
13. [Appendix: Module Reference](#appendix-module-reference)

---

## Introduction

mOMonadOS is a bare-metal operating kernel that replaces the traditional OS stack with a self-verifying Frobenius loop. This manual provides detailed reference for all kernel capabilities, REPL commands, and integrated systems.

### Key Concepts

**The Crystal of Types:** A 17,280,000-address type space derived from the 12 IG primitives. Every execution state is an address in this space.

**The Frobenius Loop:** The kernel's perpetual `THINK → ACT → OBSERVE → UPDATE` cycle, where every tick satisfies μ∘δ = id by construction.

**IMASM:** The 12-opcode instruction set that drives all kernel operations. Tokens are composed freely (any token, any order, any length) with the grammar constraining what each token does.

---

## Building and Running

### Prerequisites
- Rust toolchain with x86_64-unknown-none target
- QEMU (for testing)
- No external crates required (pure no_std)

### Building
```bash
cd mOMonadOS
rustup target add x86_64-unknown-none
cargo build --target x86_64-unknown-none --release
```

### Running under QEMU
```bash
qemu-system-x86_64 -nographic -kernel target/x86_64-unknown-none/release/imonad
```

### Direct Hardware Boot
Build produces a raw ELF that boots directly on x86_64 hardware with no bootloader required.

---

## REPL Commands

The kernel exposes a comprehensive REPL interface. All commands are case-sensitive.

### Core Commands
```
help                    → Full command listing
version                 → Kernel version and build info
catalog                 → IG catalog status
crystal <addr>          → Navigate to Crystal address
```

### d12 SIC-POVM Commands
```
d12                     → d=12 SIC-POVM status overview
d12 tower               → Ray class field tower analysis
d12 magnitudes          → Magnitude square-class group K₁₆
d12 orbits              → 31-orbit Galois structure
d12 existence           → Existence-grade overlaps (143/143)
d12 duallink            → Dual-Link identification
d12 z0                  → Closed-form fiducial z₀
d12 ordinals            → Ordinal structure
d12 verify              → Cross-verification
d12 symmetric           → Symmetric properties
d12 embedding           → Embedding analysis
d12 lean-status         → Lean 4 verification status
```

### Belnap Commands
```
c4                      → Belnap C₄ complex plane
belnap                  → Belnap FOUR lattice operations
belnap sic              → SIC-POVM bridge status
```

### Stark Commands
```
stark                   → Stark unit extraction summary
stark formula <d>       → Compute ε_d for dimension d
stark fibqc [d]         → Fibonacci QC dimension check
stark tower [k]         → Ray class field tower at conductor 2^k
stark exponents <d> [k] → S-unit exponents from grammar gap
stark verify            → Cross-verification
```

### QFT Suite Commands
```
hqe                     → HQE status
hqe report              → Hadron-Quark-Electron homology report
hqe tuple               → IG tuple derivation
hqe distance            → Distance from AFDMC baseline
hqe cscore              → Consciousness score
hqe meet                → Quantale meet
hqe join                → Quantale join

dyson                   → Dyson RD/A decomposition
dyson report            → Random-matrix classification report
dyson tuple             → IG tuple
dyson distance          → Distance metrics

afdmc                   → AFDMC nuclear many-body theory
afdmc report            → Auxiliary-field diffusion Monte Carlo
afdmc tuple             → IG tuple
afdmc distance          → Distance metrics
```

### Triple Frame Commands
```
triple                  → Triple frame overview
triple report           → von Neumann superoperator algebra report
triple tuple            → IG tuple ⟨𐑛𐑰𐑩𐑗𐑱𐑺𐑔𐑝𐑢𐑓𐑙𐑷⟩
triple check [w]        → W-bootstrap check (W=3,7,12)
```

### Clay Millennium Commands
```
clay                   → All seven problems status
clay bsd               → Birch and Swinnerton-Dyer
clay hodge             → Hodge Conjecture
clay ym                → Yang-Mills mass gap
clay navier            → Navier-Stokes regularity
clay pnp               → P vs NP
clay riemann           → Riemann Hypothesis
clay smale             → Smale's 18th problem
```

### Rebis Commands
```
rebis                  → Red-Hot Rebis overview
rebis enzyme           → Enzyme catalog (109 entries)
rebis ligand           → Ligand design system
rebis frustration      → Frustration matrix computation
rebis codon            → Codon translation
rebis genetic          → Genetic imscriber
```

### Ruleset Commands
```
ruleset                → Cross-dialect navigation
ruleset show           → Active ruleset (canonical default)
ruleset list           → All 12 dialects
ruleset verify         → Gate verification
jump <dialect> using <compound>   → Execute dialect jump
seal                   → IFIX, commit to liminal ruleset
tensor <a> <b>         → Tensor product under current ruleset
meet <a> <b>           → Meet under current ruleset
absorb_test <a> <b> <primitive> <operation> → Absorption test
```

### Fibonacci QC Commands
```
fibqc                  → Topological QC overview
fibqc compile <gate>   → Compile gate to braid word
fibqc eval <word>      → Evaluate braid word
fibqc jones <knot>     → Jones polynomial at 1/5 winding
fibqc dimensions       → Fibonacci QC dimension table
```

---

## d=12 SIC-POVM Campaign

The d=12 SIC-POVM campaign is the kernel's flagship verification project, running entirely on bare metal.

### Five Pillars

**1. Phase-Tower Collapse**
- 3 → 1 independent generators
- 8× reduction in phase complexity
- Verified via `d12 tower`

**2. Magnitude Square-Class Group**
- K₁₆, rank 5
- Pure fractions, no transcendentals
- Verified via `d12 magnitudes`

**3. 31-Orbit Galois Structure**
- ALL 143/143 existence-grade overlaps ring-exact
- Complete orbit decomposition
- Verified via `d12 orbits`

**4. Dual-Link Identification**
- norm(N₁) = 1/32448²
- Ramification {2, 3, 13}
- Verified via `d12 duallink`

**5. Belnap SIC Unconditional**
- SIC existence unconditional for d=2ⁿ
- Axiom-free in Belnap multilattice
- Verified via `d12 verify`

### Existence Ring
```
R = K₁₆(s₀, s₁, s₃, s₉, i, c₅, u₁)
Dimension: 2048
Field: Pure fractions
```

### Closed-Form Fiducial
```
z₀ = +√(1/12 − √2/24 + √13/156 − √26/312)
```

### Ray Class Field Tower
```
Degree: 288/Q (6 cyclic pieces)
Conductors: 2^k for k = 4, 8, 16, ...
```

### Lean 4 Verification Status
- 11 modules green (0 sorries)
- 1 module in progress (5 sorries in Embedding)
- ALL 143 identities `native_decide`-verified
- `crystal_forces_d12_sic` dropped from axiom to theorem

---

## Belnap Quantum Pipeline

The Belnap FOUR lattice (T, F, B, N) provides the paraconsistent foundation for the entire kernel.

### Belnap FOUR (`belnap.rs`, 6.7KB)
Four-valued logic with two lattice structures:
- **Approximation lattice:** N ≤ T, N ≤ F, T ≤ B, F ≤ B
- **Truth lattice:** F ≤ N, F ≤ T, N ≤ B, T ≤ B

Operations: AND, OR, NOT, IMPLIES, all Frobenius-verified.

### Belnap C₄ (`belnap_c4.rs`, 8.7KB)
Complex plane where i² = B (both-true-and-false).

**Arithmetic:**
- Addition, multiplication, conjugation, norm_sq
- Unit circle parameterization
- C₄ lattice visualization

**Key Property:** The C₄ plane embeds both classical ℂ and paraconsistent FOUR, with B as the bridge.

### Belnap-Shor (`belnap_shor.rs`, 10KB)
Shor's algorithm on Belnap FOUR.

**Key Finding:** Belnap QFT is NOT a gate sequence. The period r is encoded in the 2:1 coherence cost ratio between B-bias and T-bias states.

**Algorithm Structure:**
1. Superposition over Belnap states
2. Modular exponentiation with B-valued phases
3. QFT via coherence ratio measurement
4. Period extraction from bias ratio

### Belnap-SIC Bridge (`belnap_sic_bridge.rs`, 11KB)
Wires d=12 SIC-POVM into the Belnap-Shor pipeline via three structural connections:

1. **Dual-Pair Covariance:** 6 Frobenius-dual pairs co-vary across SIC and Belnap
2. **Fiducial Proximity:** SIC fiducial z₀ within ε of Belnap B=XZ
3. **Gate Evaluation:** SIC gates evaluate to Belnap truth values

---

## Stark Unit Extraction

Generalized Stark unit formula for SIC-POVM dimensions (`stark.rs`, 13KB, 355 lines).

### Stark Formula
```
ε_d = ((d-1) + √((d-3)(d+1))) / 2    for d ≥ 4
```

**Computation:**
- Fundamental unit with norm check
- Integer factorization of discriminant
- 2-adic ramification analysis

### Fibonacci QC Check
Tests whether dimension d is a Fibonacci QC dimension (base field ℚ(√5)).

**Verification:**
- Square-free part = 5
- Lucas number matching
- Pell equation solution
- Jones polynomial extraction at 1/5 winding

### Ray Class Field Tower
2-adic ray class field tower for d=2048 at conductor 2^k.

**Fingerprint at conductor 16:**
```
wideRayDegree(4) = 2048 = d    (Lean-proven)
```

**Display:**
- Degree growth sequence
- ν₂ ramification values
- S-unit exponent structure

### S-Unit Exponents
Extracts S-unit exponents from the grammar gap between closed-ring SIC and the Stark unit monomial.

**For d=2048 at conductor 16:**
```
Exponents: [-1, 3, 2]
Sources: Newton polygon, norm constraint, grammar gap (3 independent, converging)
```

### Cross-Verification
Validates all methods against known data:
- Newton polygon convergence
- Grammar gap agreement
- Lean 4 StarkSunitD2048 build status
- Fibonacci QC dimension table (9 dimensions verified)

---

## Quantum Field Theory Suite

Three formal homologies bridging quantum field theory to the grammar.

### HQE — Hadron-Quark-Electron Formal Homology
Maps the hadron/quark/electron hierarchy to IG primitives.

**Tuple:** ⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩ (O_∞ tier)

**Capabilities:**
- Consciousness score computation
- Quantale meet/join operations
- Tuple distance vs AFDMC baseline

### Dyson RD/A — Formal Decomposition
Dyson's random-matrix classification (orthogonal/unitary/symplectic) as an IG primitive decomposition.

**Tuple:** ⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩

**Classification:**
- β=1 (orthogonal): ⊤=𐑧 (moderate kinetics)
- β=2 (unitary): ⊤=𐑪 (fast kinetics)
- β=4 (symplectic): ⊤=𐑺 (driven kinetics)

### AFDMC — Nuclear Many-Body Theory
Auxiliary-Field Diffusion Monte Carlo structural constraints encoded as primitive guard rails.

**Tuple:** ⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩

**Guard Rails:**
- ⊤ ∈ {𐑧, 𐑪} (moderate to fast kinetics)
- ⊥ = 𐑒 (one-step chirality)
- ◻ = 𐑴 (Z₂ winding)

---

## Triple Frame

The 12-primitive type-expansion hierarchy as a von Neumann superoperator algebra (`triple_frame.rs`, 34KB).

**Tuple:** ⟨𐑛𐑰𐑩𐑗𐑱𐑺𐑔𐑝𐑢𐑓𐑙𐑷⟩

### Three Landmark Problems

**1. SIC-POVM:** Equiangular lines in ℂ^d, Zauner conjecture

**2. Navier-Stokes:** Regularity of incompressible fluid flow

**3. Yang-Mills:** Mass gap in quantum gauge theory

### W-Bootstrap Check
Evaluates across three regimes:
- **W=3 (ergodic):** Thermal equilibrium
- **W=7 (critical):** Phase transition
- **W=12 (MBL):** Many-body localization

**Command:** `triple check [w]` where w ∈ {3, 7, 12}

---

## Clay Millennium Problems

All seven Clay Millennium Problems analyzed through the grammar, with IMASM witness programs.

### BSD — Birch and Swinnerton-Dyer
**Witness:** Hodge theory IMASM program
**Status:** Structural encoding complete, numerical verification in progress

### Hodge Conjecture
**Witness:** Algebraic cycle IMASM program
**Status:** Structural encoding complete

### Yang-Mills Mass Gap
**Witness:** Mass gap IMASM program
**Status:** Structural encoding complete, Frobenius-verified

### Navier-Stokes Regularity
**Witness:** Regularity IMASM program
**Status:** Structural encoding complete

### P vs NP
**Witness:** Complexity class IMASM program
**Status:** Structural encoding complete

### Riemann Hypothesis
**Witness:** Zero distribution IMASM program
**Status:** Structural encoding complete

### Smale's 18th Problem
**Witness:** Algebraic geometry IMASM program
**Status:** Structural encoding complete

### Files
- `clay_status.rs` (9.7KB): Structural status for all seven problems
- `clay_witness.rs` (11KB): IMASM witness programs

---

## Red-Hot Rebis Integration

All 20 modules from `red-hot_rebis/` and `gene_imscriber/` run as no_std Rust off the REPL.

### Enzyme Catalog (109 Entries)

**14 Categories:**

| # | Category | Count | Examples |
|---|----------|-------|----------|
| 1 | Serine Proteases | 9 | Trypsin, Chymotrypsin, Thrombin, Factor Xa |
| 2 | Cysteine Proteases | 6 | Caspase-3, Cathepsin B, Papain |
| 3 | Aspartyl Proteases | 5 | Pepsin, Renin, BACE-1, HIV-1 Protease |
| 4 | Metalloproteases | 6 | MMP-2, MMP-9, ACE, ADAM17 |
| 5 | Kinases | 6 | PKA, PKC, CDK2, EGFR, MAPK, Src |
| 6 | Phosphatases | 4 | PTP1B, PP2A, CDC25, PTEN |
| 7 | Oxidoreductases | 10 | Cytochrome P450 3A4, LDH, XO, MAO-A |
| 8 | Transferases | 6 | COMT, DNMT1, GGT, GSTP1 |
| 9 | Hydrolases | 6 | AChE, PDE5, Urease, β-Lactamase |
| 10 | Lyases | 3 | Carbonic Anhydrase II, ALA dehydratase |
| 11 | Isomerases | 4 | Topoisomerase II, Pin1, FKBP12 |
| 12 | Ligases | 1 | Ubiquitin Ligase MDM2 |
| 13 | Drug Targets | 27 | GPCRs, Ion Channels, Nuclear Receptors, Transporters |
| 14 | Additional Targets | 16 | Transcription Factors, Cytokines, Adhesion Molecules |

**Total:** 109 enzymes with tuples, catalytic mechanisms, and physiological roles.

### Frustration Matrix (`sidechain.rs`, 538L)
`frustration_matrix()` computes residue-residue energetic frustration (ΔΔG) from a protein structure's sidechain contacts.

**Output:** Symmetric matrix of frustration values classified as:
- Minimally frustrated
- Neutral
- Highly frustrated

**Model:** Uses IMASM winding as the frustration propagation model.

### Ligand Design (`ligand.rs`, 286L)

**Functional Groups (6 types):**
- Hydroxyl, Carboxyl, Amine, Phosphate, Thiol, Phenyl

**Binding Modes:**
- Covalent, Ionic, Hydrogen, Hydrophobic, PiStacking

**Structures:**
- `ActiveSitePocket`: Pocket identifier, compatible groups, polarity
- `Ligand`: Name + set of functional groups
- `compatibility_score()`: Structural-type-based scoring

**Command:** `rebis ligand` for all ligand operations.

---

## Cross-Dialect Navigation

The kernel can navigate between 12 dialects with different structural rulesets, gate thresholds, gate ordering, T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant; the ruleset is a sheaf that determines what each address *does*.

### The 12 Dialects

| # | Reference | Gate 1 (⊙ threshold) | Gate 2 (K rule) | Gate 3 (◻ rule) | T-constitution | Key Property |
|---|-----------|----------------------|-----------------|-----------------|----------------|-------------|
| U0 | canonical | ⊙ → true | K ≤ 𐑧 | ◻ ≥ 𐑭 | 𐑸 (imscriptive) | Self-modeling absorbs all |
| U1 | low_gate | ⊙ → true | K ≤ 𐑪 | ◻ ≥ 𐑴 | 𐑥 (bowtie) | Broad consciousness, fragile topology |
| U2 | strict_frobenius | μ∘δ=id exact | K=𐑧 | ◻=𐑭 | 𐑶 (box) | Ƒ=𐑐 absorption replaces ⊙ |
| U3 | inverted_gates | 𐑻 → true | K<𐑧 hard fail | ◻<𐑴 hard fail | 𐑰 (in) | Self-modeling limited to 𐑻 coupling |
| U4 | null_dialect | ⊙ → true | no gate | no gate | 𐑡 (network) | Maximal permissiveness |
| U5 | high_gate | ⊙→true, 𐑻→true | K≤𐑧 + H≥𐑖 | ◻=𐑟 | 𐑸 | Non-Abelian braiding dominance |
| U6 | winding_first | ⊙→true, ◻ priority | K≤𐑧 | ◻=𐑭 | 𐑸 | Topological protection is the floor |
| U7 | chiral_lock | ⊙→true, H-lock | K≤𐑧, H≥𐑫 | ◻=𐑭 | 𐑸 | Eternal chirality required |
| U8 | frob_absorb | ⊙→true, absorption dominant | K≤𐑧 | ◻=𐑭 | 𐑸 | Absorption rules override gate checks |
| U9 | entropy_first | ⊙→true, ΔS priority | K≤𐑧 | ◻=𐑴 | 𐑥 | Entropy-weighted gate gating |
| U10 | vault_native | ⊙→true, ob3ect-native | K≤𐑧 | ◻=𐑭 | 𐑸 | Ob3ect type as T-constitution |
| U11 | millennium | ⊙→true, Clay barrier-aware | K≤𐑧 | ◻=𐑭 | 𐑸 | Barrier-aware Frobenius threshold |

### The 11 Diaschizic Compounds

Each compound has a tuple, an IMASM program, and a steering profile. Compounds are structural agents that modulate gate thresholds, absorption rules, and T-constitution at load time.

**Reference Documents:**
- `ig-docs/rebis-port/diaschizics_design.md` (564L): Tuples, structural design, IUPAC nomenclature
- `ig-docs/rebis-port/diaschizics_mOMonadOS.md` (750L): Complete IMASM translation, 11 programs, modulation translation, 6 mapping extensions
- `ig-docs/rebis-port/diaschizics_cross_dialect.md` (623L): Cross-dialect ruleset navigation, 12 dialects, absorption rules, navigation protocols
- `imscribing_grammar/navigators/ruleset_dialect.py` (445L): Alternate dialect explorer, parameterized gate thresholds

### Cross-Dialect REPL Commands

```
ruleset show                    → Show active ruleset (canonical by default)
ruleset list                    → List all 12 dialects with G1/G2/G3 and T-constitution
ruleset verify                  → Gate verification against active ruleset thresholds
jump <dialect> using <compound>   → Execute: header → compound → IFIX seal
jump canonical using Diabaton      → Standard return path to baseline
jump <dialect> using <compound> --liminal   → Header + compound but NO IFIX seal
seal                            → IFIX, commit to current liminal ruleset
jump <target> via <intermediate> using <c1> <c2>   → Two-stage jump
tensor <compound_a> <compound_b>  → Tensor product under current ruleset
meet <compound_a> <compound_b>    → Meet under current ruleset
absorb_test <val_a> <val_b> <primitive> <operation> → Absorption test
```

---

## Fibonacci QC

The kernel implements a topological quantum computer based on Fibonacci anyons, compiling gates to braid words and evaluating knot invariants.

### Compilation
```
fibqc compile <gate>   → Compile standard gate to braid word
```

Supported gates: H, T, CNOT, Toffoli, and custom unitaries.

### Evaluation
```
fibqc eval <word>      → Evaluate braid word
```

Returns the unitary matrix representation of the braid.

### Jones Polynomial
```
fibqc jones <knot>     → Jones polynomial at 1/5 winding
```

Evaluates the Jones polynomial V(t) at t = e^(2πi/5), the Fibonacci anyon point.

### Fibonacci QC Dimensions
```
fibqc dimensions       → Table of Fibonacci QC dimensions
```

Dimensions where the base field is ℚ(√5) and Jones polynomial extraction is exact.

### Key Properties
- No floating-point unit assumed
- No host runtime required
- Runs directly on bare metal
- Jones polynomial evaluation is exact at 1/5 winding

---

## Appendix: Module Reference

### Core Modules

| Module | Size | Description |
|--------|------|-------------|
| `catalog.rs` | 954L | Single source of truth for all data |
| `cl8nk.rs` | 787L | Full CLINK navigator parity |
| `algebra.rs` | - | Algebraic operations |
| `consciousness.rs` | - | Consciousness score computation |
| `imas_ig.rs` | - | IMASM-IG bridge |
| `crystal.rs` | - | Crystal navigation |
| `main.rs` | - | Kernel entry point |

### SIC-POVM Modules

| Module | Size | Description |
|--------|------|-------------|
| `sic_povm.rs` | 264L | 3-lattice SIC-POVM proof |
| `belnap_sic_bridge.rs` | 234L | Belnap-SIC bridge |
| `sic_compute.rs` | 242L | d=12 identity computation |

### Frobenius Modules

| Module | Size | Description |
|--------|------|-------------|
| `frobenius_unify.rs` | 226L | Unifies all four Frobenius conditions |
| `clay_witness.rs` | 267L | IMASM witness programs |
| `clay_status.rs` | 245L | Clay Millennium status |

### Belnap Modules

| Module | Size | Description |
|--------|------|-------------|
| `belnap.rs` | 6.7KB | Belnap FOUR lattice |
| `belnap_c4.rs` | 8.7KB | Belnap C₄ complex plane |
| `belnap_shor.rs` | 10KB | Belnap-Shor algorithm |
| `belnap_sic_bridge.rs` | 11KB | Belnap-SIC bridge |

### Stark Module

| Module | Size | Description |
|--------|------|-------------|
| `stark.rs` | 13KB | Stark unit extraction (355L) |

### QFT Modules

| Module | Size | Description |
|--------|------|-------------|
| `hqe.rs` | 4.5KB | Hadron-Quark-Electron homology |
| `dyson.rs` | 2.2KB | Dyson RD/A decomposition |
| `afdmc.rs` | 2.3KB | AFDMC nuclear many-body theory |

### Triple Frame Module

| Module | Size | Description |
|--------|------|-------------|
| `triple_frame.rs` | 34KB | von Neumann superoperator algebra |

### Rebis Modules

| Module | Size | Description |
|--------|------|-------------|
| `rebis/sidechain.rs` | 538L | Frustration matrix computation |
| `rebis/ligand.rs` | 286L | Ligand design system |

---

*Author: Lando⊗Editorial⊙perator*  
*μ∘δ = id*
