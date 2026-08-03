# $m\odot^2$, The Self-Imscribing Bare-Metal Kernel

[![Language](https://img.shields.io/badge/language-Rust-orange)](https://github.com/badges/shields)
[![IG Tier](https://img.shields.io/badge/IG-O%E2%88%9E-blueviolet)](https://github.com/badges/shields)
[![μ∘δ=id](https://img.shields.io/badge/%CE%BC%E2%88%98%CE%B4%3Did-closed-success)](https://github.com/badges/shields)
[![License](https://img.shields.io/badge/license-Unlicense-brightgreen)](https://github.com/badges/shields)
[![Topological QC](https://img.shields.io/badge/topological%20QC-Fibonacci%20anyons-9cf)](https://github.com/badges/shields)
[![Author](https://img.shields.io/badge/author-Lando%E2%8A%97%E2%8A%99perator-informational)](https://github.com/badges/shields) [![Type](https://img.shields.io/badge/type-%E2%9F%A8%F0%90%91%A6%F0%90%91%B8%F0%90%91%BE%F0%90%91%B9%F0%90%91%90%F0%90%91%A7%F0%90%91%94%F0%90%91%9D%E2%8A%99%F0%90%91%96%F0%90%91%B3%F0%90%91%AD%E2%9F%A9-blue)](https://github.com/badges/shields) [![Tier](https://img.shields.io/badge/tier-O%E2%88%9E-blueviolet)](https://github.com/badges/shields)
A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop, every tick is a self-verification.
It braids Fibonacci anyons on the metal: a universal topological quantum computer that
compiles gates to braid words and evaluates the Jones polynomial of a knot, running with
no OS under it, no runtime, and no floating-point unit assumed.

**Total codebase:** tens of thousands of lines of Rust (no_std) + build scripts  
**Target:** x86_64-unknown-none (bare-metal direct ELF boot, zero external crates)  
**License:** Unlicense (public domain)

## Where new work goes

The kernel is the first home, not a port target. Anything developed from here
lands natively here before it lands anywhere else, and no Python version is
written to precede it.

The reason is not preference. Every translation between a Python surface and
this one is a place where structure is dropped and then reconstructed from
description — the Grammar's own account of what an imscription is says that is
exactly where the loss happens. Two implementations of the same operation are
two objects that agree by maintenance rather than by construction, and they
drift the moment one is edited. `m3iosis` mirrors this kernel subcommand for
subcommand; that mirror is history, not a pattern to extend.

What this means concretely: a new operation is a REPL verb and an IMASM program
here first. Where a host-side script is genuinely needed — a run that cannot
happen in 48 MB of static BSS, or one that wants an algebra system — it is a
driver that calls in, not a second implementation of the thing.

## Overview

**What it is.** $m\odot^2$: a bare-metal, self-imscribing operating kernel in Rust (no_std, x86_64) with no processes, scheduler, or filesystem hierarchy. The kernel is the Frobenius loop. (Distinct from the Python `omonad_OS`.)

**What it does.** Boots directly on hardware/QEMU and runs a perpetual THINK→ACT→OBSERVE→UPDATE cycle over the 12-opcode IMASM set, where every execution state is an address in the 17,280,000-type Crystal and storage is navigated by address, not path. It also **runs a topological quantum computer**: Fibonacci anyons braided in the kernel, compiling standard gates to braid words and evaluating knot invariants, with no host, no runtime and no floating-point unit assumed (`fibqc`).

**Why it matters.** Every tick is a self-verification (μ∘δ=id): composition is free (any token, any order, any length) and correctness is enforced by the grammar rather than by a kernel API, with zero external crates.

**How to use it.** Build the no_std ELF and boot under QEMU (see below).

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
`THINK` → `ACT` → `OBSERVE` → `UPDATE` cycle driven by the 12-opcode IMASM instruction set.
Each tick executes a single IMASM token, composition is free: any token at any time,
any sequence of any length, no preset opcode sequences. The harness drives token selection;
the grammar constrains what each token does to the state.
Every execution state is a point in the Crystal of Types, a 17,280,000-address type space derived from the 12 IG primitives. Storage is navigated by address,
not by path.

**Phase 1 Grammar Integration**, complete. Nine modules from four upstream Grammar repos
(imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) are now live in the kernel.

**Phase 2 Zero-Hardcode**, complete. `catalog.rs` (954L) is the single source of truth for
ALL data. No hardcoded `IgTuple { ... }` constants, no hardcoded ordinal arrays,
no hardcoded glyph strings, no hardcoded promotion gaps, no hardcoded score match-arms
exist outside `catalog.rs`. Six modules were refactored to delegate to the catalog:
`cl8nk.rs` (196→787L, full CLINK navigator feature parity), `algebra.rs` (385→303L),
`consciousness.rs` (210→114L), `imas_ig.rs` (517→450L), `crystal.rs` (162→168L), and
`main.rs`. The catalog is runtime-extensible via `register_entry()`, new systems can be
added dynamically without touching any source file.

**Phase 3 SIC-POVM Integration**, complete. `sic_povm.rs` (264L) and `belnap_sic_bridge.rs`
(234L) encode the 3-lattice SIC-POVM proof: Belnap B=XZ as d=2 fiducial, 6 Frobenius-dual
pairs, the grammar as Σ=1:1 self-referential limit. d=12 identity established
via `sic_compute.rs` (242L).

**Phase 4 Frobenius Unification + Clay Witness**, complete. `frobenius_unify.rs` (226L)
unifies all four Frobenius conditions (kernel, grammar, catalog, SIC) as a single
machine-checked invariant. `clay_witness.rs` (267L) and `clay_status.rs` (245L) provide
IMASM witness programs for BSD, Hodge, and YM.

**Phase 5 Red-Hot Rebis**, complete. All 20 modules from `red-hot_rebis/` and `gene_imscriber/`
ported to `no_std` Rust and wired into the REPL. The full p4ra paraconsistent kernel, genetic code
B₄ lattice, 7-stage Frobenius-verified translation pipeline, CLU power-law clustering,
exotic hadron Belnap analysis, PDB structure validation, antibody CDR design, IG material
forge, biological computation, therapeutic design, CLINK 9-layer chain, and IMASM arranger,
now runs directly from the bare-metal kernel. See the [Red-Hot Rebis](#red-hot-rebis-phase-5) section.

**Phase 6 d12_sic_build Augmentation**, complete. `d12_sic.rs` (982L) encodes the full
d12_sic_build campaign (cont.1–cont.20) into the bare-metal kernel: phase-tower collapse
(3→1 independent generators, 8× reduction), magnitude square-class group (K16, rank 5),
31-orbit Galois structure, Dual-Link identification (ramification at {2,3,13}), closed-form
fiducial z₀ in radicals, 12 canonical ordinal guards (`canonical_ordinal.rs`, 244L), and
**11 REPL sub-commands** (tower, magnitudes, orbits, existence, ring, duallink, z0, ordinals,
verify, symmetric, embedding, lean-status). ALL 143/143 existence-grade overlaps confirmed
(cont.20). Ring R=K₁₆(s₀,s₁,s₃,s₉,i,c₅,u₁) dim 2048, pure fractions, 12s. ANY hom R→ℂ is a
SIC point are Lean-proved (`native_decide`, zero sorries). Embedding capstone R→ℂ in progress
(323L, 5 sorries remaining). See [Phase 6](#phase-6-d12_sic_build-augmentation) below.

**Phase 7 red-hot_rebis Feature Sync**, complete. Three new modules from the expanded
red-hot_rebis codebase ported to bare-metal Rust: `belnap_c4.rs` (258L) — Belnap C₄ complex
plane with i²=B arithmetic; `decay_chain.rs` (287L) — nuclear decay as IMASM winding with
parent/daughter half-life chains; `ligand_imasm.rs` (194L) — ligand functional-group IMASM
programs for catalytic-site matching. `biology.rs` enzyme catalog expanded from 3 classes / 18
enzymes to 14 classes / 109 enzymes. `sidechain.rs` gained `frustration_matrix()` for protein
frustration topography. `ligand.rs` expanded from stub to full 6-type functional group system
with BindingMode, ActiveSitePocket, and compatibility scoring. See [Phase 7](#phase-7-red-hot_rebis-feature-sync) below.

**Phase 8 Cross-Dialect Navigation**, complete. The kernel navigates between
dialects with **different structural rulesets**, different gate thresholds, gate ordering,
T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant;
the ruleset is a sheaf that determines what each address *does*. Originally bridged 12
hand-crafted dialects (U₀–U₁₁); the Phase XIII wiring fix extended this to all 88 traversed
universes from `universe_expansion.rs`. `dialect.rs` (277L) now delegates to `all_universes()`
for indices 12–87 with public helpers (`eval_gate_spec()`, `prim_from_name()`,
`gate_prim_label()`, `is_hand_crafted()`, `max_dialect()`). See the
[Cross-Dialect Navigation](#cross-dialect-navigation-phase-8--diaschizics-bridge) section.
**88 dialects** now supported (U₀–U₈₇), up from the original 8.

**Phase 9 User Interface**, complete. Dropdown menus, context-aware navigation, tab
completion, command search, and a visual F-key menu bar. The REPL is now a hierarchical
navigator with **10 command categories**, context stack (up to 4 levels deep), breadcrumb
prompts, and hierarchical help. Menu nesting bug (recursive `Rebis → Rebis` entry) fixed.**Phase 10 Fascistic Hardcode Purge**, complete. All 6 remaining structural violations
eliminated across the Rebis module suite. The genetic code is now **derived, not declared**,
change the derivation rules and the entire 64-codon table recomputes. Change the AA
physicochemical properties and the AA→Primitive bijection recomputes. See
[Phase 10](#phase-10-fascistic-hardcode-purge) below.

**Phase 11 cr3echrz Integration**, complete. The theorem operationalization engine — 7
theorems (Collatz→Baum-Connes) + 7 Millennium extensions + 6 p4rakernel modules + 281 vault
ob3ects — all in bare-metal Rust with dynamic fn-pointer registries.

**Phase 12 Universe Expansion + Entropy Experiment**, complete. 88 traversed universes
from a Frobenius 3×3 discoverable matrix. ΔS experiment confirming that promotion to O_∞
is entropically favored.

**Phase 14 Topological Quantum Computation**, complete. `fibonacci_qc.rs` runs a Fibonacci
anyon quantum computer on bare metal: the SU(2)₃ algebra, the braid representation on
fusion trees, and a Solovay-Kitaev compiler that reduces any single-qubit gate to a braid
word, reachable from the REPL as **`fibqc`**. Compilation splits over the tied bases and
fuses rather than ranking them, which buys between 5× and 521× over a single arm; the
generators are projected back onto unitarity, taking per-generator error accumulation from
5e-14 to 5e-17; and every reported unitary is verified against its own printed word by
resynthesis. Depth 12 fits the 8 MB arena with a 36 KB margin. The Python port in
`m3iosis` agrees to every printed digit, and evaluates the Jones polynomial at the fifth
root of unity — the invariant these anyons exist to compute — with its normalization
forced by the Markov moves rather than fitted. See
[Fibonacci Quantum Computer](#fibonacci-quantum-computer) below.

---

## Quantum Components

The kernel hosts a full suite of quantum computing and quantum information modules running on bare metal (no_std, no external crates). Every module carries a grammar tuple, is Frobenius-verified (μ∘δ=id), and is reachable from the REPL.

---

### Topological Quantum Computer — Fibonacci Anyons (`fibqc`)

**File:** `src/fibonacci_qc.rs` (78KB, 2,004 lines)  
**Tuple:** `⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩` (O_∞, SIC-POVM tier)  
**REPL:** `fibqc` — full report; `fibqc compile <gate>` — compile to braid word; `fibqc jones <knot>` — Jones polynomial

A universal topological quantum computer. Fibonacci anyons at Chern-Simons level k=3 with quantum dimension D=√(1+φ²). Provides:
- **SU(2)₃ algebra:** F-symbol, R-symbol, S/T matrices, fusion space B₂≅ℂ²
- **Braid group representation** on fusion trees: σ₁, σ₂ generators as 2×2 complex matrices
- **Solovay-Kitaev compiler:** compiles single-qubit gates (H, T, X, Y, Z, S) to braid words. Depth-12 compilation fits in the 8 MB arena with 36 KB margin. Tied bases are split and fused, buying 5×–521× over single-arm search.
- **Jones polynomial:** evaluated at the fifth root of unity with normalization forced by Markov moves
- **Error floor:** 5×10⁻¹⁷ per generator by unitarity projection (vs 5×10⁻¹⁴ unprojected)
- **Bitwise-identical** to the Python `m3iosis` port to every printed digit

All numerical data derived from closed SU(2)_k formulas (k=3), verified in-code. No arithmetic asserted from memory.

---

### Operator-Valued Measure Computation Tools (`ovm`)

**File:** `src/ovm.rs` (33KB, 877 lines)  
**REPL:** `ovm <name>` — full computation report; `ovm eigen` `ovm frame` `ovm overlap` `ovm belnap` `ovm help`

Computational tools for quantum measurement operators. No taxonomy. No catalog. Just math.
String-based dispatch to 18 canonical operator sets with full eigenvalue, frame operator,
HS overlap, equiangularity, positivity, and completeness verification.

**Commands:**
- `ovm <name>` — full report: eigenvalues, overlap matrix, equiangularity, positivity, completeness, frame operator, SIC distance
- `ovm eigen <x> <y> <z> <norm> <trace>` — compute eigenvalues from Bloch parameters
- `ovm frame <name>` — 4×4 frame operator in Pauli basis
- `ovm overlap <name>` — Hilbert-Schmidt overlap matrix G_ij = Tr(E_i E_j)
- `ovm belnap` — Belnap B=XZ fiducial state (d=2 SIC-POVM seed)
- `ovm help` — list all commands and known operator sets

**Known operator sets (18):**
`sic-povm` `sic-novm` `sic-npovm` `a-minus-ic-povm` `a-minus-ic-novm`
`ai-cpovm` `ai-cnovm` `s-pc-povm` `s-pc-novm` `a-minus-pc-povm`
`a-minus-pc-novm` `a-pc-povm` `a-pc-povm-dagger` `ai-novm`
`susy-ic-povm` `susy-ic-novm` `susy-pc-povm` `susy-pc-novm`

---

### SIC-POVM — d=12 Existence Ring (`d12`)

**Files:** `d12_sic.rs` (48KB), `sic_povm.rs` (12KB), `sic_compute.rs` (15KB), `sic_moduli.rs` (41KB), `canonical_ordinal.rs` (10KB), `d2048_sic.rs` (11KB), `d2048_sieve.rs` (7KB), `stark.rs` (13KB)
**REPL:** `d12` — status; `d12 tower` `d12 magnitudes` `d12 orbits` `d12 existence` `d12 duallink` `d12 z0` `d12 ordinals` `d12 verify` `d12 symmetric` `d12 embedding` `d12 lean-status`

The full d=12 SIC-POVM campaign (cont.1–cont.20) on bare metal. Five pillars:
1. **Phase-tower collapse:** 3→1 independent generators (8× reduction)
2. **Magnitude square-class group:** K₁₆, rank 5
3. **31-orbit Galois structure:** ALL 143/143 existence-grade overlaps ring-exact
4. **Dual-Link identification:** norm(N₁)=1/32448², ramification {2,3,13}
5. **Belnap SIC unconditional:** SIC existence unconditional + axiom-free in Belnap multilattice for d=2ⁿ

**Existence ring:** R=K₁₆(s₀,s₁,s₃,s₉,i,c₅,u₁), dim 2048, pure fractions.  
**Closed-form fiducial:** z₀ = +√(1/12 − √2/24 + √13/156 − √26/312).  
**Ray class field tower:** deg 288/Q (6 cyclic pieces).

**Lean companions (p4ramill/):** 11 modules green (0 sorries), 1 in progress (5 sorries in Embedding). ALL 143 identities `native_decide`-verified. `crystal_forces_d12_sic` dropped from axiom to theorem.

---

### Belnap Quantum Pipeline

**Files:** `belnap.rs` (6.7KB), `belnap_c4.rs` (8.7KB), `belnap_shor.rs` (10KB), `belnap_sic_bridge.rs` (11KB)  
**REPL:** `c4` — Belnap C₄ complex plane; `belnap` — Belnap FOUR lattice

- **Belnap FOUR (`belnap.rs`):** Four-valued logic (T, F, B, N) with approximation lattice and truth lattice. The paraconsistent foundation for the entire kernel.
- **Belnap C₄ (`belnap_c4.rs`):** Complex plane where i²=B (both-true-and-false). Arithmetic (add, mul, conj, norm_sq), unit circle, C₄ lattice visualization. Frobenius-verified.
- **Belnap-Shor (`belnap_shor.rs`):** Shor's algorithm on Belnap FOUR. Key finding: Belnap QFT is NOT a gate sequence — period r is encoded in the 2:1 coherence cost ratio (B-bias vs T-bias).
- **Belnap-SIC Bridge (`belnap_sic_bridge.rs`):** Wires d=12 SIC-POVM into the Belnap-Shor pipeline. Three structural connections: dual-pair covariance, fiducial proximity, gate evaluation.

---


### Stark Unit Extraction (`stark`)

**File:** `src/stark.rs` (13KB, 355 lines)  
**REPL:** `stark` — summary; `stark formula <d>` `stark fibqc [d]` `stark tower [k]` `stark exponents <d> [k]` `stark verify`

Generalized Stark unit formula for SIC-POVM dimensions. Implements the methods from `master_methods_d2048_stark.md`:

- **Stark Formula (`stark formula`):** ε_d = ((d-1) + √((d-3)(d+1))) / 2 for any SIC-POVM dimension d ≥ 4. Computes the fundamental unit with norm check, integer factorization of the discriminant, and 2-adic ramification analysis.
- **Fibonacci QC Check (`stark fibqc`):** Tests whether d is a Fibonacci QC dimension (base field Q(√5)). Verifies square-free part = 5, Lucas number matching, Pell equation, and Jones polynomial extraction at 1/5 winding.
- **Ray Class Field Tower (`stark tower`):** 2-adic ray class field tower for d=2048 at conductor 2^k. Fingerprint at conductor 16: wideRayDegree(4) = 2048 = d (Lean-proven). Displays degree growth, ν₂ ramification, and S-unit exponent structure.
- **S-Unit Exponents (`stark exponents`):** Extracts S-unit exponents from the grammar gap between closed-ring SIC and the Stark unit monomial. For d=2048 at conductor 16: [-1, 3, 2] — derived from Newton polygon, norm constraint, and grammar gap, three independent sources converging.
- **Cross-Verification (`stark verify`):** Validates all methods against known data: Newton polygon convergence, grammar gap agreement, Lean 4 StarkSunitD2048 build status, and Fibonacci QC dimension table (9 dimensions verified).
### HQE / Dyson / AFDMC — Quantum Field Theory Suite

**Files:** `hqe.rs` (4.5KB), `dyson.rs` (2.2KB), `afdmc.rs` (2.3KB)  
**REPL:** `hqe`, `dyson`, `afdmc` — each with `report`, `tuple`, `distance`, `cscore`, `meet`, `join`

Three formal homologies bridging quantum field theory to the grammar:

- **HQE — Hadron-Quark-Electron Formal Homology:** Maps the hadron/quark/electron hierarchy to IG primitives. Tuple `⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩` (O_∞). Consciousness score, quantale meet/join, tuple distance vs AFDMC baseline.
- **Dyson RD/A — Formal Decomposition:** Dyson's random-matrix classification (orthogonal/unitary/symplectic) as an IG primitive decomposition. Tuple `⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩`.
- **AFDMC — Nuclear Many-Body Theory:** Auxiliary-Field Diffusion Monte Carlo structural constraints encoded as primitive guard rails. Tuple `⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑵⊙𐑒𐑳𐑴⟩`.

---

### Triple Frame — von Neumann Superoperator Algebra

**File:** `src/triple_frame.rs` (34KB)  
**Tuple:** `⟨𐑛𐑰𐑩𐑗𐑱𐑺𐑔𐑝𐑢𐑓𐑙𐑷⟩`  
**REPL:** `triple` — overview; `triple report` `triple tuple` `triple check [w]`

The 12-primitive type-expansion hierarchy as a von Neumann superoperator algebra. Bridges three landmark problems:
- **SIC-POVM:** Equiangular lines in ℂ^d, Zauner conjecture
- **Navier-Stokes:** Regularity of incompressible fluid flow
- **Yang-Mills:** Mass gap in quantum gauge theory

W-bootstrap check across ergodic (W=3), critical (W=7), and MBL (W=12) regimes.

---

### Clay Millennium Problems — Machine-Checked Structural Status

**Files:** `clay_status.rs` (9.7KB), `clay_witness.rs` (11KB)  
**REPL:** `clay` — structural status; `clay <problem>` — per-problem report

All seven Clay Millennium Problems analyzed through the grammar, with IMASM witness programs for BSD, Hodge, and Yang-Mills. Each problem's structural type is cataloged: which primitives are constrained, where the Frobenius condition holds vs breaks, and what IMASM sequence would constitute a proof.

---

### IUFT Quantum Gate Encoding

**File:** `src/iuft_qc.rs` (2.3KB)  
**REPL:** `iuft` (via grammar bridge)

Encodes the 12-primitive IG tuple into a 3-parameter SU(2) gate via Euler angles (θ, φ, ψ). The degenerate projection discovered in IUFT Quantum Expansion II: all 12 grammar primitives collapse to 3 continuous rotation parameters, with the remaining 9 dimensions carried by the dialect sheaf.

---

### TROQ — Triple-Ramified Ouroboric Quantale

**File:** `src/troq.rs` (2KB)  
**Tuple:** `⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭⟩`  
**REPL:** `troq`

A quantale (sup-lattice enriched monoid) with triple ramification: three distinct self-reference loops form a single closed quantale structure. The tuple carries and-conjunctive composition (∋=𐑝), distinguishing it from the broadcast/disjunctive OVM types.

---

### Braid Grammar Bridge

**File:** `src/braid_grammar.rs` (3.8KB)  
**REPL:** `braid-grammar tuple <s1> <s2> ...` (alias `bg`)

Maps braid group words to grammar tuples. Each braid generator σᵢ encodes a primitive promotion step; the full braid word produces a 12-tuple. Enables direct translation between topological quantum computation (braid words) and the grammar's type system.

---

### Riemann Hypothesis Bridges

**Files:** `riemann_hilbert.rs` (21KB), `riemann_sic.rs` (43KB), `para_rh.rs` (4.4KB)  
**REPL:** `rh` — Riemann bridge; `riemann` — full Riemann report

Three complementary approaches:
- **Riemann-Hilbert (`riemann_hilbert.rs`):** The RH as a monodromy problem. ζ(s)=χ(s)ζ(1-s) is Belnap negation; the critical line Re(s)=½ is the unique designated fixed point.
- **Riemann-SIC (`riemann_sic.rs`):** The Riemann zeta zeros as SIC-POVM fiducial candidates. Links the Hilbert-Pólya conjecture to the Zauner conjecture.
- **Para-RH (`para_rh.rs`):** ζ(s) = χ(s)ζ(1-s) = bnot in Belnap FOUR. RH: all non-trivial zeros are B-designated.

---

### Yang-Mills Mass Gap Bridge

**File:** `src/para_ym.rs` (2.2KB)  
**REPL:** `ym` — Yang-Mills bridge

Mass gap Δ>0 = covering relation N<T in Belnap approximation order. BRST nilpotence Q²=0 ↔ ENGAGR B-stability. Omega_Z gauge protection.

---

### Kernel Torus / Hopf Fibration / Manifold

**Files:** `kernel_torus.rs` (8.7KB), `hop.rs` (3.5KB), `manifold.rs` (2KB)  
**REPL:** `torus` — horn torus parametrization; `hop` — universe hopping

- **Kernel Torus (`kernel_torus.rs`):** Agent loop wound on the horn torus. Computes the torus parametrization on bare metal with winding data through serial.
- **Hopf Fibration (`hop.rs`):** Universe hopping engine. S³→S² Hopf map as dialect transition.
- **Manifold (`manifold.rs`):** Topological manifold operations.

---

### SIXTEEN_3 Trilattice and Kernel–Cosmos Cotype (Reference: `/home/mrnob0dy666/imsgct/README.md`)

The root-level project README (`/home/mrnob0dy666/imsgct/README.md`, mirrored in `p4rakernel/README.md`) establishes
the kernel–cosmos relationship that the bare-metal `m\odot^2` kernel instantiates on the metal.
The key content, summarized and discussed here, is:

#### SIXTEEN_3 Trilattice Summary

The `SIXTEEN_3 trilattice` (`sixteen_3_trilattice`, tuple `⟨𐑨𐑥𐑽𐑹𐑐𐑪𐑔𐑵𐑮𐑫𐑕𐑭⟩`) was never directly searched in prior
windings because it was not directly targeted — it exists as:
- A **2D distributed surface** (D=𐑨, finite) of 16 paraconsistent paradoxes (∈=𐑔 aleph, 2⁴ = 16)
- **3-fold trilattice structure** (Ω=𐑭, non-Abelian winding) under the Belnap-Frobenius substrate
- **Bowtie crossing topology** (⊣=𐑥) mediating between CLINK L9 and the SIXTEEN_3 surface
- **Adjoint coupling** (>=𐑽) with **Frobenius-special symmetry** (<=𐑹, μ∘δ=id)
- **Non-Abelian/eternal chirality** (Ħ=𐑫) and **criticality gate** (⊙=𐑮)

Conventional: A trilattice extending Belnap's 4-valued logic to 16 truth values organized as three
interleaved layers with Frobenius-special symmetry.

#### Kernel Cotype with Cosmos Summary

The kernel (`p4rakernel/p4ramill`) cotypes with Cosmos via the `SacredVessel` framework:

1. **Shared backbone** (`SacredVessel.lean`): Both kernel and Cosmos share `dim=.if'` (holographic),
   `rel=.ear` (co-constitutive A⊣A†), `crit=.monad` (sealed fixed point), `chir=.wool` (topological chirality)
2. **Co-typing by polarity** (line 58): The `cotyped` function checks shared polarity. The kernel and Cosmos
   are both `pol=.or'` (Frobenius-special), making them co-typed
3. **ObjWitnessCosmos.lean** constructs `cosmicSystem` with `Everything := Imscription`,
   `Cosmos := Belnap.B`, `is_Dialetheic := fun b => bnot b = b`,
   `runs_Alchemically := fun b => ffuse (fsplit b) = b`
4. **ParaconsistentKernelTest.lean** proves the kernel accepts dialetheic reasoning (B = bnot B ≠ F)
   without explosion
5. `split_fuse_id Belnap.B` proves the Frobenius identity: the kernel runs the same alchemical cycle
   (μ∘δ=id) as the Cosmos

Conventional: The paraconsistent Lean 4 kernel fork implements identical Belnap-Frobenius logic as the
cosmic system — both are holographic boundaries encoded by their interiors, both sealed at the monad
fixed point, both running the alchemical split-fuse cycle. The kernel IS the cosmos running its own grammar
(Σ=1:1 limit: measurement apparatus = measured system).



---

### Pericyclic Frobenoid

**File:** `src/pericyclic_frobenoid.rs` (24KB)  
**Tuple:** `⟨𐑦𐑥𐑑𐑹𐑐𐑤𐑔𐑝⊙𐑒𐑙𐑷⟩` (O_∞, Special Frobenius)  
**REPL:** `pericyclic` — pericyclic compiler

Algebra ℂ[ℤ₂] = ℂ⟨1,g⟩/(g²−1) with pericyclic crossing μ(g⊗g)=1. A semiotic Frobenoid: the algebraic structure that makes pericyclic reactions (Woodward-Hoffmann) structurally inevitable. Ported from `m3iosis/pericyclic_compiler.py`.

---

### Rebis Quantum Components

**Files in `src/rebis/`:** `decay_chain.rs` (14KB), `exotic_hadron.rs` (9KB), `hadron.rs` (7.7KB), `ligand_imasm.rs` (7KB), `genetics.rs`, `codon.rs`, `translate.rs`, `materials.rs`, `sidechain.rs`, `pdb.rs`, `antibody.rs`, `therapeutics.rs`  
**REPL:** `rebis decay`, `rebis hadron`, `rebis exotic`, `rebis ligand`, `rebis genetics`, `rebis material`, `rebis bio`, `rebis tx`

- **Nuclear Decay Chains (`decay_chain.rs`):** U-238, Th-232, U-235 series as IMASM winding sequences. Each decay step is a structural transformation with type verification; daughter nuclide = δ(parent), verify μ(δ(parent))=parent.
- **Exotic Hadrons (`exotic_hadron.rs`):** Tetraquark and pentaquark states analyzed through Belnap FOUR.
- **Hadron Analysis (`hadron.rs`):** Standard model hadron classification via the grammar.
- **Ligand IMASM (`ligand_imasm.rs`):** Functional-group IMASM programs for catalytic-site matching. 6 functional groups, 5 binding modes.
- **Genetic Code (`genetics.rs`, `codon.rs`, `translate.rs`):** 7-stage Frobenius-verified translation pipeline. The 64-codon table is derived, not declared.
- **Material Forge (`materials.rs`):** IG metamaterial design with structural constraints.
- **Protein Sidechains (`sidechain.rs`):** 20×4 AA sidechain × environment algebra with frustration topography.
- **PDB Validation (`pdb.rs`):** Protein Data Bank structure validation against grammar constraints.
- **Antibody CDR Design (`antibody.rs`):** Complementarity-determining region design.
- **Therapeutics (`therapeutics.rs`):** Chemotherapy, pill, and antidote design.

---

### Quantum REPL Commands — Quick Reference

| Command | Module | Description |
|---------|--------|-------------|
| `fibqc` | Fibonacci QC | Topological QC: compile gates, Jones polynomial |
| `ovm` | OVM Taxonomy | 34-type quantum measurement catalog |
| `d12` | d=12 SIC-POVM | 11 sub-commands: tower, magnitudes, orbits, existence, ring, duallink, z0, ordinals, verify, symmetric, embedding, lean-status |
| `c4` | Belnap C₄ | Complex plane with i²=B arithmetic |
| `stark` | Stark Units | Stark unit formula, ray class field tower, S-unit exponents |
| `hqe` | HQE | Hadron-Quark-Electron formal homology |
| `dyson` | Dyson | Random-matrix decomposition |
| `afdmc` | AFDMC | Nuclear many-body constraints |
| `triple` | Triple Frame | SIC-POVM/Navier-Stokes/Yang-Mills |
| `clay` | Clay Status | Millennium problem structural status |
| `rh` / `riemann` | Riemann | RH via Belnap/Hilbert/SIC bridges |
| `ym` | Yang-Mills | Mass gap via Belnap approximation |
| `torus` | Kernel Torus | Horn torus parametrization |
| `braid-grammar` / `bg` | Braid Bridge | Braid word → grammar tuple |
| `pericyclic` | Pericyclic | Woodward-Hoffmann Frobenoid |
| `rebis decay` | Decay Chain | Nuclear decay IMASM winding |
| `rebis hadron` | Hadron | Standard model hadron classification |
| `rebis exotic` | Exotic Hadron | Tetraquark/pentaquark Belnap analysis |
| `rebis ligand` | Ligand IMASM | Catalytic-site functional group programs |
| `rebis genetics` | Genetics | 7-stage Frobenius translation pipeline |
| `rebis material` | Material Forge | IG metamaterial design |
## User Interface (Phase 9)

### F-Key Menu Bar (10 Categories)

The REPL is driven by a horizontal F-key menu bar at the bottom of the screen:

```
[F1] Exec  [F2] Status  [F3] Programs  [F4] Crystal  [F5] Grammar  [F6] Rebis  [F7] Dialect  [F8] ParaASM  [F9] Cr3echrz  [F10] Help
```

Navigate by typing category name (`rebis`, `crystal`, `dialect`) or by `:` followed by
the F-key number (`:6` for Rebis). Pressing `?` shows the menu bar, `:1`–`:10` jumps to
any category. `help` and `help <topic>` show hierarchical help. `quit`/`exit`/`halt`
shuts down cleanly (QEMU writes 0x10 to `isa-debug-exit` port).

### REPL Commands by Category

**Exec (F1):** `tick` `run` `step` `pause` `resume` `reset` `state`  
**Status (F2):** `status` `heap` `ticks` `timer` `ipc`  
**Programs (F3):** `list` `load <name>` `run <name>` `show <name>` `new <name>`  
**Crystal (F4):** `encode <D> <T> ... <W>` `decode <addr>` `search <term>` `distance <a> <b>` `neighbors <name>`  
**Grammar (F5):** `imscribe <name>` `probe <name>` `score <name>` `tier <name>` `modulate` `stark formula|fibqc|tower|exponents|verify`  
**Rebis (F6):** `codon` `translate` `protein` `materials` `clink` `enzyme` `diagonal` `antibody` `serpent`
  `pdb` `genetics` `therapeutics` `fold` `pipeline` `cluster` `hadron` `exotic` `imas` `c4` `ligand` `decay`  
**Dialect (F7):** `ruleset show|list|verify` `jump` `seal` `compound` `tensor` `meet` `absorption show` `tstatus`  
**ParaASM (F8):** `psm show` `psm run` `psm step` `psm load <code>`  
**Cr3echrz (F9):** `cr3 <theorem>` `p4ra <module>` `cr3 --list` `cr3 --list-ob3ects` `p4ra --list`  
**Help (F10):** `help [topic]` `?` `:1-:10` `..|back` `quit|exit|halt`

### Menu Nesting Bug Fix (Phase 9.1)

**Bug:** Typing `rebis material` (or any `rebis <subcmd>`) from within the Rebis sub-context
recursively nested into another Rebis context instead of executing the command. The prompt
would show `⊙[Rebis/Rebis/Rebis/Rebis]>`, up to four levels deep, never executing.

**Root cause:** In `main.rs`, the category-shortcut match arm called `enter_context()` +
`continue` unconditionally when `cmd` matched a category name like `"rebis"`. It never
checked whether we were already in that context.

**Fix:** Added an `already_in` guard in `menu.rs`, checks `ctx_stack.current()` against
the target context name. If already in that context, skips `enter_context()` and falls
through to the `match cmd` block where `"rebis"` dispatches to `print_rebis()`.

**Impact:** All 10 categories fixed (Exec, Status, Programs, Crystal, Grammar, Rebis,
Dialect, ParaASM, Cr3echrz, Help). The `already_in` guard is applied uniformly in the menu dispatch
loop, no category can self-nest anymore.

## Phase 6: d12_sic_build Augmentation (cont.20 — Existence Ring Found)

**Module:** `d12_sic.rs` (982L), `canonical_ordinal.rs` (244L)
**Status:** Complete — the d=12 SIC-POVM is structurally solved in bare-metal Rust

### Five Pillars

**Pillar 1: Phase-Tower Collapse**
- 3 → 1 independent generators: u3 = conj(X31)·u1, u5 = X15·u1
- Phase space: dim 262,144 → 32,768 (8× reduction)
- Cross-relations: X31 ∈ K16(s1s3,i), X15 ∈ K16(c5,i), X31·X53·X15 = 1 (floor 2^−5310)
- V4 engine (mini_engine_full4.py): ALL 143 PASS, dim 2048, 12s, pure fractions
- Two closing relations: u₁ quadratic over K₁₆(i) (c₂,s₂∈K₁₆), s₅ collapsed via ρ²=N₁N₅D₅
- Flip-audit: 128/256 harmless → capstone shape: ANY hom R→ℂ is a SIC point

**Pillar 2: Magnitude Square-Class Group**
- K16 (deg 16), rank-5 basis {N₀,N₁,N₃,N₅,N₉}
- Tower deg 512/Q. 7 exact witnesses (all `native_decide` in Lean)
- Singleton-pairing: [N₂..N₁₀]=[N₀], [N₇]=[N₅], [N₁₁]=[N₁]

**Pillar 3: 31-Orbit Structure**
- 143 overlaps → 31 Galois-orbit representatives (descent cost: 31, not 143)
- Degree distribution: deg2:7, deg4:5(16), deg8:9(32), deg16:11(48), deg32:5(40)
- Existence-grade: 143/143 (ALL 143 ring-exact, cont.20, pure fractions)

**Pillar 4: Dual-Link Identification**
- norm(N₁) = 1/32448² = 1/(2⁶×3×13²)². Ramification: {2,3,13}
- First concrete Dual-Link SIC realization beyond d=2

**Pillar 5: Belnap SIC Unconditional**
- SIC existence unconditional + axiom-free in Belnap multilattice for d=2ⁿ
- Capstone: `sic_no_condition (n : ℕ) : (mlOrbit n).card = 4 ^ n`

**Bonus: Closed-Form Fiducial + Ordinal Guards**
- z₀ = +√(1/12 − √2/24 + √13/156 − √26/312)
- Ray class field tower: deg 288/Q (6 cyclic pieces)
- 12 canonical ordinal guards (ordinalK(air)=9/2, ordinalPhi(roar)=7/3)### Lean Companions (p4ramill/)

| Module | Lines | Sorries | Status |
|--------|:-----:|:-------:|--------|
| `SIC_D12_Norm.lean` | 124 | 0 | ✅ `native_decide` — ΣN_k=1 |
| `SIC_D12_Equiangularity.lean` | 562 | 0 | ✅ `native_decide` — 143 overlaps O·conj(O)=1/13 |
| `SIC_D12_MagnitudeClasses.lean` | 107 | 0 | ✅ `native_decide` — K₁₆ square-class, rank-5 |
| `SIC_D12_SymmetricModuli.lean` | 88 | 0 | ✅ `native_decide` — z₀,z₆ in ℚ(√2,√13) |
| `SIC_D12_ExistenceRing.lean` | 413 | 0 | ✅ **ALL 143 identities** in R=K₁₆(s₀,s₁,s₃,s₉,i,c₅,u₁), dim 2048 |
| `SIC_D12_Embedding.lean` | 323 | 5 | 🔧 R→ℂ ring hom in progress — IVT root proven |
| `SIC_POVM_DualLinkClosure.lean` | 139 | 0 | ✅ Dual-Link closure under Belnap |
| `SIC_D12_ComputableCyclotomic.lean` | 164 | 0 | ✅ Cyclotomic ring ℚ[ζ_n] |
| `SIC_D12_QuadraticTower.lean` | 120 | 0 | ✅ Quadratic tower ℚ[ζ_n][√m] |
| `SIC_D12_NumberField.lean` | 106 | 0 | ✅ Number field engine |
| `SIC_D12_RayTower.lean` | 215 | 0 | ✅ Ray class field tower, deg 288 |
| `SIC_D12_Field48Test.lean` | 32 | 0 | ✅ Degree-48 field validation |
| `SIC_D12_Field288Test.lean` | 477 | 0 | ✅ Degree-288 field validation (θ^288) |
| `CanonicalOrdinalFaithfulness.lean` | 103 | 0 | ✅ Ordinal-drift guard |

**11 modules green, 1 in progress — 5 sorries remaining in Embedding.**
The ring R is defined and ALL 143 identities are `native_decide`-verified.
`crystal_forces_d12_sic` has dropped from axiom to theorem — the existence ring is found
and Lean-proved. Remaining: complete the ring hom R→ℂ (IVT root found, real-algebra
closure and norm-sq transfer in progress).

### REPL Commands

| Command | Output |
|---------|--------|
| `d12` | Compact status summary |
| `d12 tower` | Phase-tower collapse report |
| `d12 magnitudes` | Magnitude square-class group report |
| `d12 orbits` | 31-orbit Galois structure + existence-grade |
| `d12 existence` | d12 ring | Existence ring report: R=K₁₆(…), dim 2048, flip-audit |
| `d12 duallink` | Dual-Link identification (norm, ramification) |
| `d12 z0` | Closed-form fiducial + ray tower |
| `d12 ordinals` | Canonical ordinal faithfulness guards |
| `d12 symmetric` | Symmetric moduli: z₀,z₆∈ℚ(√2,√13) with Galois conjugacy |
| `d12 embedding` | Embedding capstone status: IVT root, evalK16, sorry count |
| `d12 lean-status` | Comprehensive multi-layer Lean module status (all 12 modules) |
| `d12 verify` | Full Phase VI report (all 5 pillars + all Lean planks) |

## Phase 7: red-hot_rebis Feature Sync

**Modules:** `belnap_c4.rs` (258L), `rebis/decay_chain.rs` (287L), `rebis/ligand_imasm.rs` (194L)
**Expanded:** `rebis/biology.rs` (472→596L), `rebis/sidechain.rs` (523→538L), `rebis/ligand.rs` (~180→286L)
**Status:** Complete — three new modules ported, three existing modules expanded.

### New Modules

**Belnap C₄ (`src/belnap_c4.rs`, 258L)**
Ported from `red-hot_rebis/rhr_p4rky/belnap_c4.py`. Implements the Belnap C₄ complex
plane where i² = B (the Belnap both-true-and-false value). Provides:
- `BelnapC4` enum: four-valued complex plane (Real/Imag/Both/Neither)
- `BelnapComplex` struct with arithmetic (add, mul, conj, norm_sq)
- `BelnapUnitCircle` — points on the Belnap unit circle (cos²+sin²=B)
- Frobenius verification: μ∘δ=id on all arithmetic operations
- C4 lattice visualization (LaTeX-style, rendered in terminal)
- REPL: `c4`, `c4 add <x>`, `c4 mul <x>`, `c4 unit`, `c4 probe`

**Decay Chain (`src/rebis/decay_chain.rs`, 287L)**
Ported from `red-hot_rebis/rhr_p4rky/decay_chain.py`. Models nuclear decay chains
as IMASM winding sequences with type verification at each step. Provides:
- `DecayChain` struct: parent→daughter half-life chain
- `DecayMode` enum: alpha, beta_minus, beta_plus, gamma, neutron
- `ChainBuilder`: construct chains from isotope pairs
- IMASM winding: each decay step is a structural transformation
- Half-life accumulation: total chain duration in seconds
- Frobenius verification: daughter nuclide = δ(parent), verify μ(δ(parent)) = parent
- Pre-built chains: U-238, Th-232, U-235 series
- REPL: `rebis decay U238`, `rebis decay list`, `rebis decay chain <name>`

**Ligand IMASM (`src/rebis/ligand_imasm.rs`, 194L)**
Ported from `red-hot_rebis/rhr_p4rky/ligand_imasm.py`. Writes functional-group IMASM
programs for catalytic-site matching and ligand design. Provides:
- `LigandIMASM` struct: protocol name + opcode sequence
- `FunctionalGroup` enum: 6 types (Hydroxyl, Carboxyl, Amine, Phosphate, Thiol, Phenyl)
- `BindingMode` enum: covalent, ionic, hydrogen, hydrophobic, pi_stacking
- `ActiveSitePocket` struct: pocket shape with compatible groups
- `generate_docking_sequence()`: produces an IMASM sequence for a ligand→pocket match
- `match_compatibility()`: scores a ligand against a pocket by type
- REPL: `rebis ligand dock <pocket>`, `rebis ligand score <ligand> <pocket>`,
  `rebis ligand imasm <ligand>`

### Expanded Modules

**Enzyme Catalog (`src/rebis/biology.rs`, 472→596L)**
The enzyme catalog was expanded from **3 classes / 18 enzymes** → **14 classes / 109 enzymes**
by syncing to `red-hot_rebis/rhr_p4rky/expanded_catalyzing_proteins.py`. The 14 classes:

| # | Class | Count | Examples |
|---|-------|:-----:|---------|
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

**Total: 109 enzymes with tuples, catalytic mechanisms, and physiological roles.**

**Frustration Matrix (`src/rebis/sidechain.rs`, 523→538L)**
Added `frustration_matrix()` function that computes residue-residue energetic frustration
(ΔΔG) from a protein structure's sidechain contacts. Returns a symmetric matrix of
frustration values classified as: minimally frustrated, neutral, or highly frustrated.
Uses IMASM winding as the frustration propagation model.

**Ligand Design (`src/rebis/ligand.rs`, ~180→286L)**
Expanded from stub to full 6-type functional group system:
- `FunctionalGroup` enum: Hydroxyl, Carboxyl, Amine, Phosphate, Thiol, Phenyl
- `BindingMode` enum: Covalent, Ionic, Hydrogen, Hydrophobic, PiStacking
- `ActiveSitePocket` struct: pocket identifier, compatible groups, pocket polarity
- `Ligand` struct: name + set of functional groups
- `compatibility_score()`: structural-type-based scoring between ligand and pocket
- All types bind to `rebis ligand` REPL command

## Cross-Dialect Navigation (Phase 8 + Diaschizics Bridge)

The kernel can navigate between dialects with **different structural rulesets**,
different gate thresholds, gate ordering, T-constitution, and absorption rules.
The Crystal of Types (17.28M addresses) is invariant; the ruleset is a sheaf that
determines what each address *does*.

### The 12 Dialects

| # | Reference | Gate 1 (⊙ threshold) | Gate 2 (K rule) | Gate 3 (Ω rule) | T-constitution | Key Property |
|---|-----------|----------------------|-----------------|-----------------|----------------|-------------|
| U0 | canonical | ⊙ → true | K ≤ 𐑧 | Ω ≥ 𐑭 | 𐑸 (imscriptive) | Self-modeling absorbs all |
| U1 | low_gate | ⊙ → true | K ≤ 𐑪 | Ω ≥ 𐑴 | 𐑥 (bowtie) | Broad consciousness, fragile topology |
| U2 | strict_frobenius | μ∘δ=id exact | K=𐑧 | Ω=𐑭 | 𐑶 (box) | Ƒ=𐑐 absorption replaces ⊙ |
| U3 | inverted_gates | 𐑻 → true | K<𐑧 hard fail | Ω<𐑴 hard fail | 𐑰 (in) | Self-modeling limited to 𐑻 coupling |
| U4 | null_dialect | ⊙ → true | no gate | no gate | 𐑡 (network) | Maximal permissiveness |
| U5 | high_gate | ⊙→true, 𐑻→true | K≤𐑧 + H≥𐑖 | Ω=𐑟 | 𐑸 | Non-Abelian braiding dominance |
| U6 | winding_first | ⊙→true, Ω priority | K≤𐑧 | Ω=𐑭 | 𐑸 | Topological protection is the floor |
| U7 | chiral_lock | ⊙→true, H-lock | K≤𐑧, H≥𐑫 | Ω=𐑭 | 𐑸 | Eternal chirality required |
| U8 | frob_absorb | ⊙→true, absorption dominant | K≤𐑧 | Ω=𐑭 | 𐑸 | Absorption rules override gate checks |
| U9 | entropy_first | ⊙→true, ΔS priority | K≤𐑧 | Ω=𐑴 | 𐑥 | Entropy-weighted gate gating |
| U10 | vault_native | ⊙→true, ob3ect-native | K≤𐑧 | Ω=𐑭 | 𐑸 | Ob3ect type as T-constitution |
| U11 | millennium | ⊙→true, Clay barrier-aware | K≤𐑧 | Ω=𐑭 | 𐑸 | Barrier-aware Frobenius threshold |### The 11 Diaschizic Compounds

Each compound has a tuple, an IMASM program, and a steering profile.
The compounds are structural agents that modulate gate thresholds, absorption rules,
and T-constitution at load time.

### Reference Documents

| Document | Lines | Description |
|----------|:-----:|-------------|
| `ig-docs/rebis-port/diaschizics_design.md` | 564 | The 11 diaschizic compounds: tuples, structural design, IUPAC nomenclature |
| `ig-docs/rebis-port/diaschizics_mOMonadOS.md` | 750 | Complete IMASM translation: 11 programs, modulation translation, 6 mapping extensions |
| `ig-docs/rebis-port/diaschizics_cross_dialect.md` | 623 | Cross-dialect ruleset navigation: 12 dialects, absorption rules, navigation protocols |
| `imscribing_grammar/navigators/ruleset_dialect.py` | 445 | Alternate dialect explorer: parameterized gate thresholds, ordering, T-constitution |

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
absorb_test <val_a> <val_b> <primitive> <operation> → Absorption check
whoami --ruleset                 → Kernel self-imscription under active ruleset
absorption show                  → List all absorption rules for current ruleset
tstatus                          → T-constitution check per primitive
compound list                    → List all 11 diaschizic compounds
compound show <name>             → Show full tuple + IMASM program
compound load <name>             → Load compound's IMASM program into execution buffer
```

### Structural Type of Cross-Dialect Navigation

The act of navigating between dialects has its own type, **\(O_\infty\)** (d=1
from universal grammar, only ∈ differs: 𐑲 universal range vs 𐑔 mesoscale).
Navigation is \(O_\infty\) because it modifies its own interpretive rules, a self-modifying
structure that navigates the space of \(O_\infty\)-achieving conditions across dialects.
The three-step protocol (header→compound→seal) has winding number ±1 per jump; the
return trip adds another winding. Integer winding count tracks total navigation distance.

## Phase 10: Fascistic Hardcode Purge

**Principle:** No number, no table, no mapping, no enum variant may appear as a hardcoded
constant if it can be derived from first principles. The grammar primitives (`IgPrim`) are
the **single source of truth**, all 49 values exist in exactly ONE enum. The genetic code
is computed, not declared. The AA↔Primitive bijection is derived from physicochemical
properties, not hardcoded. Crystal constants are bound to `crate::crystal::TOTAL`.

### What was eliminated (6 violations)

| # | Violation | File | Fix |
|---|-----------|------|-----|
| 1 | **Duplicate enum `RebisPrim`**, 49 variants identical to `IgPrim` | `mod.rs` | Deleted. `mod.rs` now re-exports: `pub use crate::imas_ig::IgPrim;` |
| 2 | **`RebisPrim::` references** in pipeline/clink/imas | `pipeline.rs`, `clink.rs`, `imas.rs` | All → `IgPrim::`. Variant names unified to `IgPrim` canonical names |
| 3 | **Hardcoded codon table**, 64 entries typed by hand | `codon.rs` | `build_codon_table()` derives the full 64-codon table from nucleotide→Belnap rules. Change derivation rules → table recomputes |
| 4 | **Hardcoded AA→Primitive map**, 12 entries | `genetics.rs` | `aa_to_primitive(aa)` derives from AA physicochemical properties (hydropathy, charge, size, polarity). Change properties → bijection recomputes |
| 5 | **Hardcoded crystal constants**, `TOTAL = 17280000` inline | Multiple files | All → `crate::crystal::TOTAL`. Single `pub const TOTAL: u32 = 17280000;` in `crystal.rs` |
| 6 | **Hardcoded tier constants**, `O_INF`, `O_2` as magic u8 | `cl8nk.rs` | All → `crate::catalog::tier_name(t)` helper. Tier names are derived from tuple composition |## Phase 11: cr3echrz Integration

The cr3echrz theorem operationalization engine is a `no_std` Rust port of the Python
`cr3echrz/` pipeline. Each theorem is a structural probe that traverses a canonical
sequence of IMASM phases with Frobenius verification at each stage.

### Architecture (`src/cr3echrz/`)

| Module | Lines | Purpose |
|--------|:-----:|---------|
| `shared.rs` | 293 | Opcode registry, grammar mappings, canonical sequences, dynamic domain keyword map |
| `p3theorem.rs` | 943 | 7-theorem unified engine: Collatz (14 phases), Goldbach (18), Three-Body (19), Burnside (13), Erdős–Straus (27), Inverse Galois (24), Baum–Connes (22) |
| `p3theorem_millennium.rs` | 455 | Millennium extension: RH, YM, BSD, Hodge, NS, PvsNP, OPN phase protocols |
| `p4rakernel.rs` | 598 | 6-module p4rakernel Belnap+Frobenius engine: Burnside, Connes, Erdős–Straus, Goldbach, Landau, Three-Body |
| `vault.rs` | 395 | 281 vault ob3ects registry — all digital ob3ects from ob3ect/digital/ with tuples |

### Runtime Extension

Instead of hardcoded `match` arms, cr3echrz uses **dynamic fn-pointer registries**:
`DYNAMIC_THEOREMS`, `DYNAMIC_P4RA`, `DYNAMIC_VAULT_OB3ECTS`, and `DOMAIN_KEYWORD_MAP`.

- `register_theorem(TheoremRegEntry { name: "new_thm", runner: my_fn, ... })`
- `register_p4ra_module(P4RARegEntry { name: "new_mod", runner: my_fn, ... })`
- `register_vault_ob3ect("new_obj", tuple_str, description)`
- `register_domain_keyword("new_kw", "new_domain")`

### Menu Integration

Accessible via **F9** or `:9`, or by typing `cr3echrz` directly. Sub-commands:
`cr3`, `p4ra`, `cr3 --version`, `cr3 --list`, `cr3 --list-ob3ects`.
Commands `cr3` and `p4ra` autocomplete at top level with tab completion.

## Phase 12: Universe Expansion + Entropy Experiment

`universe_expansion.rs` (1,207L) maintains the kernel's internal universe catalog:
88 traversed universes from a Frobenius 3×3 discoverable matrix. Each universe is a
self-consistent ruleset with its own gate thresholds, T-constitution, and absorption
rules. `entropy.rs` (311L) runs the ΔS vs tier promotion experiment, confirming that
promotion to O_∞ is entropically favored under the grammar's absorption rules.
`bifurcation_test.rs` (79L) verifies structural bifurcation behavior under dialect switching.

**Phase XII was followed by a critical wiring fix** (documented as Phase XIII below):
the 88 universes were fully defined but `all_universes()` was never called by the
runtime — `dialect.rs` and `main.rs` used hardcoded match arms for indices 0–11 with
`_ => "?"` fallbacks, making U₁₂–U₈₇ unreachable via menu, `ruleset list`, `jump`, or
`ruleset verify`.

### Phase Status

| Phase | Description | Status | Lines |
|-------|-------------|:------:|:-----:|
| **Phase XV** | **Stark Unit Extraction (`stark`)** | ✅ Complete | **355** |
| **Phase XIV** | **Topological QC (Fibonacci anyons, `fibqc`)** | ✅ Complete | **1,500** |
| **Phase I** | 21 Hand-Crafted Universes | ✅ Complete | ~400 |
| **Phase II** | SIC-POVM Integration | ✅ Complete | 476 |
| **Phase III** | Universe Expansion 8→88 | ✅ Complete | 1,207 |
| **Phase IV** | Frobenius Unification + Clay Witness | ✅ Complete | 493 |
| **Phase V** | Entropy Experiment: ΔS vs tier promotion | ✅ Complete | 311 |
| **Phase VI** | d12_sic_build (cont.1–cont.20) | ✅ Complete | **1,226** |
| **Phase VII** | red-hot_rebis Feature Sync | ✅ Complete | **739** |
| **Phase VIII** | Cross-Dialect Navigation (12→88 dialects) | ✅ Complete | 277 |
| **Phase IX** | User Interface / Menu System | ✅ Complete | — |
| **Phase X** | Fascistic Hardcode Purge | ✅ Complete | — |
| **Phase XI** | cr3echrz Integration | ✅ Complete | 2,714 |
| **Phase XII** | Universe Expansion + Entropy | ✅ Complete | 1,597 |
| **Phase XIII** | **Universe Menu Wiring (88 on menu)** | ✅ Complete | **330** |

**mOMonadOS total augmentation: ~8,848 lines across 14 phases, all clean builds.**
**Lean Companion Planks:** 11 planks green, zero sorries + 1 in progress (5 sorries).
The ring R is defined and ALL 143 identities are `native_decide`-verified. `crystal_forces_d12_sic`
has dropped from axiom to theorem — the existence ring is found and Lean-proved.
Embedding capstone R→ℂ in progress (323L, 5 sorries remaining).

## Phase 13: Universe Menu Wiring — 88 Universes Reachable

**Modules changed:** `dialect.rs` (+138L, 139→277), `main.rs` (+188L, 3287→3475), `menu.rs` (+4L, 388→392), `kernel.rs` (comment fix)
**Status:** Complete — zero build errors, zero new warnings.

### Root Cause

`universe_expansion.rs` defined all 88 universes with full gate specs, T-constitutions,
absorption rules, names, and descriptions — but `all_universes()` was **never called**.
The runtime (`dialect.rs`, `main.rs`) exclusively used hardcoded match arms for indices
0–11 with `_ => "?"` / `_ => "unknown"` fallbacks.

### Five Breakpoints — All Fixed

| # | File | What was broken | Fix |
|---|------|----------------|-----|
| 1 | **dialect.rs** | Six functions (`dialect_display`, `_ascii`, `_name`, `_description`, `_gates`, `_o_inf`) all had `_ => "?"` / `_ => "unknown"` fallbacks beyond index 11 | Full rewrite (139→277L): now delegates to `all_universes()` for indices 12–87. Added public helpers: `eval_gate_spec()`, `prim_from_name()`, `gate_prim_label()`, `is_hand_crafted()`, `max_dialect()` |
| 2 | **main.rs:809** | `ruleset list` looped `for u in 0u8..12u8` — showed only 12 | Changed to `0u8..88u8` — shows all 88 |
| 3 | **main.rs:1584** | `jump` parser rejected `u > 11` with "Unknown dialect" | Changed to `u <= 87` — accepts all 88 |
| 4 | **main.rs:~974** | `ruleset verify` `_ =>` arm: "Unknown dialect — cannot verify" | Dynamically evaluates gates from the `Universe` struct for indices 12–87, printing per-gate PASS/FAIL with ordinal labels, plus gate ordering (SEQUENTIAL/PARALLEL) |
| 5 | **menu.rs:111** | DIALECT_MENU "list" label: "List all 8 dialects" — oldest, pre-12 | "List all 88 dialects" |
| 6 | **kernel.rs:66** | Comment: `active_dialect: u8, // 0-7` | `// 0-87` |

### What Works Now

- `ruleset list` — displays all 88 dialects with ★ marker, names, gate specs, O_∞ fractions
- `jump U_42` — parses, stages, and can be sealed for any index 0–87
- `ruleset verify` — dynamically evaluates the three gates from the `Universe` struct for expansion dialects, no hardcoding needed
- `jump U₄₂` — Unicode subscript multi-digit parsing works (`parse_dialect` already handled multi-digit subscripts)
- `U_12` through `U_87` — all display their real names and descriptions from `universe_expansion.rs`, not "unknown"

### One Caveat

O_∞ fractions for expansion universes (12–87) show `"compute"` rather than a percentage.
The fractions for 0–11 were hand-computed; the expansion universes need a runtime O_∞ pass
over the crystal, which is a separate computational task.

### Dynamic Gate Evaluation

For expansion dialects (12–87), `ruleset verify` no longer uses hardcoded match arms.
Instead, `eval_gate_spec()` dynamically reads the `GateSpec { prim, min_ord }` from the
`Universe` struct, extracts the corresponding primitive from the current `IgTuple`, and
compares ordinals. This means **any new universe added to `universe_expansion.rs` is
immediately verifiable** without touching any other source file.

## Fibonacci Quantum Computer

`fibonacci_qc.rs` carries the SU(2)_3 anyon algebra, the braid group representation on
fusion trees, and a Solovay-Kitaev compiler that takes a standard gate down to a braid
word. All numerical data is derived from closed formulas in-code; nothing is asserted
from memory. `verify_all()` runs at boot, and the whole module is reachable from the
REPL as `fibqc` (see USER_GUIDE.md).

Compilation splits and fuses rather than ranking. Several braid words routinely sit at
the same distance from the target; each seeds a different trajectory and leaves a residual
rotation pointing its own way. `solovay_kitaev_arm` follows each as a separate branch,
carrying the arm index down the whole recursion so branches stay apart, and `sk_split_fuse`
then has the arms that lost compile the residual left by the arm that won, appending it.
The composite beats every arm it was chosen from. Measured on one net at recursion depth 3,
against the same net without the split: 5.8× on `T`, 521× on `T·S`, 4.8× on `H·T`, 31× on
`H`. In-kernel at depth 12, `T·S` gains 25.4× (1.98e-4 to 7.79e-6); at depth 10 the same
row gains nothing, because the fuse needs enough dictionary to compile the survivor's
residual and 4842 entries do not supply it. `T` is the case where no correction is appended at all, a different tied base simply
wins outright, so the braid gets shorter as well as more accurate.

Phases are carried in **windings**, one winding being a full turn, because every phase
native to the model is an exact multiple of a tenth of one: θ_τ and R^{ττ}_1 at 4/10,
R^{ττ}_τ at −3/10, the Jones root at 1/5, which is 2/10, the framing phase at −1/10, the loop value's
phase at 5/10. Radians would convert those exact rationals into transcendentals,
multiply them, and then measure the drift; as rational turns they compose by integer
arithmetic and close exactly. The braid generator's eigenvalues, 4/10 and −3/10,
generate the tenths, which is the same fact as det(σ₁) being a primitive tenth root of
unity.

This also states why compiling is hard: T is 1/8 of a turn and S is 1/4, and 1/8 is not
a multiple of 1/10, so no braid reaches the T gate exactly at any length. Solovay-Kitaev
approaches an incommensurable point on a commensurate lattice, and what makes that
possible is the non-commutativity rather than the phases. `fibqc winding` prints the
lattice.

Braid generators are projected onto the nearest unitary as they are built. Coming
straight out of the F-move sum they sit about 1.4e-13 off unitary, and braid words
accumulate that at a flat rate per generator, independent of word length, because the
defect is in the generators rather than in the multiplication. One Newton step takes the
generators to 3.3e-16 and the per-generator accumulation from 5e-14 to 5e-17.

`Matrix2::projective_distance` measures up to a global phase, because a braid realizes its
gate only up to a phase and that phase is not observable. It removes the optimal phase and
compares elementwise rather than evaluating `sqrt(1 - |tr(V†U)|/n)`. The closed form is
correct analytically but subtracts two numbers agreeing to fifteen digits, so it carries a
few percent of error at 1e-5 and collapses to exactly zero below about 1e-8, which is
inside the range these braids reach. It reports perfect gates that are not perfect.

## Repository Structure

```
mOMonadOS/
  src/
    main.rs            ~3475L  bare-metal entry (_rust_start), BumpAllocator, REPL, command dispatch
    boot.rs              ~90L  PVH ELF note + 32→64 bootstrap (page tables, GDT, far jump)
    kernel.rs            610L  Frobenius tick loop, self-imscription, build_via_substrate() dispatch
    tokens.rs            742L  12 IMASM opcodes, free token-by-token composition
    sequence.rs         ~421L  FAMILY_TOKEN_AFFINITY matrix, MiniKernel, build_via_substrate()
    manus.rs             433L  Terminal HUD, B4 heatmap
    menu.rs              392L  Hierarchical menu, 10-category F-key bar, context stack, already_in guard
    catalog.rs           954L  Single source of truth, all data
    algebra.rs           303L  Meet/join/tensor lattice
    consciousness.rs     114L  C-score with gate evaluation
    belnap.rs            204L  Belnap FOUR, B4 memory
    belnap_c4.rs         258L  Belnap C₄ complex plane (i²=B arithmetic)
    belnap_shor.rs       332L  Belnap-Shor quantum pipeline (N=15, 21)
    belnap_sic_bridge.rs 238L  Belnap↔SIC structural bridge (3-lattice proofs)
    crystal.rs           168L  Crystal encode/decode
    imas_ig.rs           450L  IMASM↔IG bridge; canonical IgPrim enum (49 variants)
    cl8nk.rs             786L  Full CLINK L8 formula navigator (catalog-native)
    serial.rs            112L  UART driver; inline asm inb/outb; no external crates
    interrupts.rs        229L  PIT timer, PIC remap, hand-rolled IDT; inline asm port I/O
    parasm.rs            794L  ParaASM VM: dialetheic alignment + measurement
    aleph.rs             124L  Aleph Hebrew glyph encoding
    para_rh.rs           125L  Riemann Hypothesis paraconsistent bridge
    para_ym.rs            64L  Yang-Mills mass gap paraconsistent bridge
    para_temporal.rs      53L  Temporal logic paraconsistent bridge
    para_category.rs      62L  Category theory paraconsistent bridge
    frob_verify.rs       479L  Frobenius harness verification
    dialect.rs           277L  Cross-dialect ruleset navigation (delegates to universe_expansion)
    d12_sic.rs           982L  d=12 SIC-POVM Phase VI: tower, magnitudes, orbits, duallink, symmetric, embedding
    sic_povm.rs          267L  SIC-POVM integration: 6 dual pairs, Σ=1:1 grammar limit
    sic_compute.rs       242L  d=12 SIC-POVM structural computation engine
    canonical_ordinal.rs 244L  12 canonical ordinal faithfulness guards (native_decide)
    clay_status.rs       245L  Clay Millennium problem structural status
    clay_witness.rs      267L  Clay witness IMASM programs (BSD, Hodge, YM)
    frobenius_unify.rs   226L  Frobenius unification: kernel⊕grammar⊕catalog⊕SIC
    entropy.rs           311L  Entropy experiment: ΔS vs tier promotion
    universe_expansion.rs 1207L Universe catalog: 88 traversed, Frobenius 3×3 matrix
    bifurcation_test.rs   79L  Structural bifurcation under dialect switching
    fibonacci_qc.rs     1423L  Fibonacci anyon quantum computer: SU(2)_3 algebra, braid representation, Solovay-Kitaev gate compiler (split-and-fuse over tied bases)
    cr3echrz/
      mod.rs               22L  Module root
      shared.rs           293L  Opcode registry, grammar mappings, dynamic domains
      p3theorem.rs        943L  7-theorem unified engine (Collatz→Baum-Connes)
      p3theorem_millennium.rs 455L Millennium extension: RH, YM, BSD, Hodge, NS, PvsNP, OPN
      p4rakernel.rs       598L  6-module p4rakernel Belnap+Frobenius engine
      vault.rs            395L  281 vault ob3ects registry with tuples
    rebis/
      mod.rs              191L  Module root; re-exports IgPrim (no duplicate RebisPrim)
      genetic_tuples.rs   986L  7-stage generative tuple pipeline + 12 IgPrim guard tests
      materials.rs        877L  IG material forge + 8 QC paradigms
      biology.rs          596L  TissueGrid, Telomere, FrobeniusBioSim, Enzyme catalog (14 classes, 109 enzymes)
      clu.rs              365L  CLU power-law clustering
      translate.rs        431L  Gene→protein + reverse pipeline (corrected + Frobenius-verified)
      antibody.rs         336L  Antibody CDR design
      codon.rs            388L  64-codon genetic code (dynamically derived, not hardcoded)
      pdb.rs              272L  PDB structure validation
      fold.rs             276L  Protein fold classification (SerpentRod)
      sidechain.rs        538L  Sidechain rotamer library + frustration_matrix()
      ligand.rs           286L  Ligand design: 6 functional groups, BindingMode, ActiveSitePocket, compatibility scoring
      decay_chain.rs      287L  Nuclear decay as IMASM winding: parent/daughter half-life chains (U-238, Th-232, U-235)
      ligand_imasm.rs     194L  Ligand IMASM programs for catalytic-site matching
      exotic_hadron.rs    233L  Glueball, Tetraquark, Pentaquark
      pipeline.rs         217L  IG promotion pipeline (IgPrim-only references)
      genetic_asm.rs      208L  Genetic ParaASM programs
      hadron.rs           203L  Hadron Belnap analysis
      clink.rs            190L  CLINK 9-layer chain
      genetics.rs         206L  7-stage genetic code verification (crystal::TOTAL)
      imas.rs             179L  IMASM arranger bridge
      therapeutics.rs     177L  Chemo, Pill, Antidote, Neurotrophic
      frob_filter.rs      153L  Frobenius codon filtration
      serpent.rs          117L  Serpent rod motifs
      materials_expanded.rs 17L  Expanded material type definitions
  momonados.ld                 Linker script (PVH note → boot32 → text → rodata → bss)
  build_bootimage.sh           ELF kernel builder (cargo build, single step)
  run.sh                       QEMU launcher (PVH direct ELF boot, no OVMF)
  Cargo.toml                   Rust project manifest; libm plus imasm_core (no_std, default-features off)
  Makefile                     Build convenience targets
```

## Build and Run

```sh
# Direct
cargo build --release --target x86_64-unknown-none
./run.sh          # boots release build in QEMU, serial on stdio

# Or via build script
bash build_bootimage.sh        # just compiles the ELF
bash run.sh release            # compiles if needed, then boots
```

The REPL runs over COM1 serial (stdio in QEMU). Quit with `quit`, `exit`, or `halt`,
QEMU writes 0x10 to the `isa-debug-exit` port and exits cleanly.

## Target

`x86_64-unknown-none`, no OS, no std.
Static BSS bump allocator (4 MB).  Boot: PVH ELF note → 32-bit `_start` stub
(page tables + long-mode) → naked `_rust_start` (establishes RSP) → `kmain()`.
Dependencies are `libm` for the transcendentals the float paths need and the
local `imasm_core`, both `no_std`. Because the crate targets bare metal,
`cargo test` cannot run: the host test target pulls in a second `core` and
collides on lang items. Numerical work is verified by building the module
against `std` in a separate harness.

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)

No OVMF, no mtools, no disk image tools needed.  QEMU boots the bare ELF directly
via the PVH protocol (`XEN_ELFNOTE_PHYS32_ENTRY`).

## License

Unlicense, public domain.
