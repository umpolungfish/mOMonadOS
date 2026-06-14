# $m\odot^2$

A bare-metal self-imscribing operating kernel. No processes. No scheduler. No filesystem hierarchy.
The kernel IS the Frobenius loop — every tick is a structural self-verification.

## What it is

$m\odot^2$ boots directly on x86_64 hardware (or QEMU) and enters a perpetual
`THINK` → `ACT` → `OBSERVE` → `UPDATE` cycle driven by the 12-opcode IMASM instruction set.
Every execution state is a point in the Crystal of Types — a 17,280,000-address structural
type space derived from the 12 IG primitives. Storage is navigated by structural address,
not by path.

**Phase 1 Grammar Integration** — complete. Nine modules from four upstream Grammar repos
(imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) are now live in the kernel.

**Phase 2 Zero-Hardcode** — complete. `catalog.rs` (814L) is the single source of truth for
ALL structural data. No hardcoded `IgTuple { ... }` constants, no hardcoded ordinal arrays,
no hardcoded glyph strings, no hardcoded promotion gaps, no hardcoded score match-arms
exist outside `catalog.rs`. Six modules were refactored to delegate to the catalog:
`cl8nk.rs` (357→196L), `algebra.rs` (385→302L), `consciousness.rs` (210→113L),
`imas_ig.rs` (517→403L), `crystal.rs` (162→167L), and `main.rs`. The catalog is
runtime-extensible via `register_entry()` — new systems can be added dynamically without
touching any source file.

**Phase 5 Red-Hot Rebis** — complete. All 17 modules from `red-hot_rebis/` ported to
`no_std` Rust and wired into the REPL. The full p4ra paraconsistent kernel — genetic code
B₄ lattice, 7-stage Frobenius-verified translation pipeline, CLU power-law clustering,
exotic hadron Belnap analysis, PDB structure validation, antibody CDR design, IG material
forge, biological simulation, and therapeutic design — now runs directly from the bare-metal
kernel.

**Phase 8 Cross-Universe Navigation** — complete. The kernel can now navigate between
universes with **different structural rulesets** — different gate thresholds, gate ordering,
T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant;
the ruleset is a sheaf that determines what each address *does*. Bridges the 11 **diaschizic
compounds** (pharmacological universe-steering agents) into computational hardware. See the
[Cross-Universe Navigation](#cross-universe-navigation-phase-8--diaschizics-bridge) section
below.

The kernel now runs **50 unit tests** across all grammar modules and supports **56+ REPL commands** spanning grammar operations, rebis biological/chemical computation, and cross-universe navigation.

### Core modules

| Module | Source | Role |
|---|---|---|
| `belnap.rs` (203L) | native | Belnap FOUR truth values (N/T/F/B), 4096-cell B4 memory, 256-deep stack, 8 registers. Extended with `band`, `bor`, `bnot`, `dialetheic`, `designated`, `approx_le`, `to_wh2`, `from_wh2`. |
| `tokens.rs` (637L) | native | 12 IMASM opcodes across 4 families; 12 canonical (I–XII), 4 continuous (XIII–XVI), 3 novel (XVII–XIX), 9 shunted (XX–XXVIII). |
| `crystal.rs` (167L) | native | 17.28M-address encode/decode; `CrystalStore` (64 entries, fixed-capacity). |
| `kernel.rs` (542L) | native | Frobenius tick loop; `self_imscribe()`; `dynamic_imscribe()`; tier promotion $O_0$ → $O_1$ → $O_2$ → $O_\infty$. Now wired to `FrobeniusHarness`. |
| `serial.rs` (96L) | native | 16550A UART COM1, 115200 8N1; `sprint!`/`sprintln!`; blocking line input. |
| `interrupts.rs` (177L) | native | PIT 100Hz timer, PIC remap, double-fault handler, escape-key detection. |
| `manus.rs` (432L) | native | Terminal HUD / live display, token graph, B4 memory heatmap, ANSI rendering. |
| `main.rs` (1904L) | native | UEFI entry, heap init, serial REPL, command dispatch, history. |

### Phase 1 Grammar modules

| Module | Lines | Source Repo | Role |
|---|---|---|---|
| `frob_verify.rs` | 478 | imasmic_core | FrobeniusHarness: 11 verifiers (hash, assertion, AST, IMASM, Belnap cycle, structural, tier promise, closure ratio, etc.), 16-entry history ring, closure ratio tracking. |
| `imas_ig.rs` | 403 | IMSCRIBr | `IgPrim` enum (49 values across 12 families), `IgTuple`, IG classification of all 12 canonicals with crystal addresses. Delegates glyph/short/crystal-indices to catalog. |
| `aleph.rs` | 123 | ALEPH_OS | 22 Hebrew letters mapped to IG primitives, `AlephWord` encoding, gematria computation. |
| `parasm.rs` | 793 | priests-engine | Full ParaASM VM: 19-instruction ISA, assembler, execution engine, dialetheic alignment, measurement algebra, 9 unit tests. |
| `belnap_shor.rs` | 331 | priests-engine | Belnap Shor pipeline: Hadamard, ModExp, 2:1 coherence cost ratio, SIC-POVM verification. |
| `para_rh.rs` | 124 | priests-engine | Riemann Hypothesis bridge: $\zeta(s)=\chi(s)\zeta(1-s)$ as Belnap negation, critical line as unique designated `bnot` fixed point. |
| `para_ym.rs` | 63 | priests-engine | Yang-Mills mass gap: N<T covering relation, BRST nilpotence ↔ Frobenius, Omega_Z gauge protection. |
| `para_temporal.rs` | 52 | priests-engine | Temporal logic: □, ◇, ○, U operators; B as temporal fixed point. |
| `para_category.rs` | 61 | priests-engine | Category theory: N=initial, T=terminal, B=zero, Frobenius algebra, dagger compact closed. |

### Phase 2 Catalog & Algebra modules

| Module | Lines | Role |
|---|---|---|
| `catalog.rs` | 814 | **Single Source of Truth** for ALL structural data. 13+ catalog entries (ZFC, ZFCₜ, ZFCfe, CLINK L8, TemporalMathematics, Schrödinger, HeatDiffusion, Navier-Stokes, WaveEquation, Einstein, IUG, O_∞ ideal, O₀ floor). 12 ordinal tables (D_ORD through OMEGA_ORD). 10 score functions computed as `ordinal_index / max_index`. 49 formula fragments. Distance weights with runtime reconfiguration. Promotion channels (6 ZFC→ZFCₜ + 2 CLINK transcendence) with ordinal gaps. Shavian glyphs and short names. Runtime registration via `register_entry()`. |
| `algebra.rs` | 302 | IG lattice algebra: Hamming / weighted distance, meet, join, tensor. All ordinals and weights sourced from catalog. 7 unit tests. |
| `cl8nk.rs` | 196 | CL8NK navigator: 4-stage ladder ZFC→ZFCₜ→ZFCfe→CLINK L8. All entry tuples and formulas delegated to catalog — zero hardcoded `IgTuple` constants. |
| `consciousness.rs` | 113 | Consciousness score: dual-gate (⊙ + K≤𐑧) evaluation. All 10 score functions computed from catalog ordinal positions — zero hardcoded match arms. 3 unit tests. |

### Phase 5 Red-Hot Rebis modules

All 17 modules ported from `red-hot_rebis/` to `no_std` Rust in `src/rebis/`:

| Module | Lines | Role |
|---|---|---|
| `rebis/kernel.rs` | 518 | p4ra paraconsistent kernel core — Belnap FOUR state transitions, dialetheic cycles, fixpoint detection |
| `rebis/belnap.rs` | 244 | Belnap FOUR lattice operations: meet, join, negation, conflation, designatedness |
| `rebis/machine.rs` | 384 | ParaASM abstract state machine — 19-instruction ISA with dialetheic alignment |
| `rebis/genetics.rs` | 427 | B₄ genetic lattice: 64 codons, 7-stage Frobenius-verified translation, tuple encoding |
| `rebis/protein.rs` | 312 | Gene-to-protein pipeline: codon→amino acid→PDB backbone→sidechain→validation |
| `rebis/antibody.rs` | 268 | Computational antibody CDR design with Belnap FOUR affinity scoring |
| `rebis/hadron.rs` | 187 | Exotic hadron Belnap FOUR state analysis: quark configurations, tetraquarks, pentaquarks |
| `rebis/orbital.rs` | 143 | Orbital Belnap analysis: electron configuration as Belnap state vector |
| `rebis/materials.rs` | 276 | IG material forge: crystal structure generation, Frobenius metamaterial design |
| `rebis/biology.rs` | 338 | Biological simulation: ouroboric telomere, metabolic flux, population dynamics |
| `rebis/therapeutics.rs` | 294 | Universal antidote library, ouroboric pill simulation, quantum biologic prototypes |
| `rebis/pdb.rs` | 198 | PDB structure validation: backbone φ/ψ angles, Ramachandran, clash detection |
| `rebis/clu.rs` | 156 | CLU power-law clustering: fitness-proportional selection, Frobenius filtration |
| `rebis/frob.rs` | 212 | Frobenius exact design verifier: μ∘δ=id for all p4ra subsystems |
| `rebis/serpent.rs` | 321 | SerpentRod protein design v5: stratified predictor, ch3mpiler bridge |
| `rebis/pipeline.rs` | 176 | Gene-to-protein demo pipeline, MSA analysis bridge |
| `rebis/mod.rs` | 89 | Module declarations, shared types, REPL subcommand routing |

**Total: ~11,650 lines across 39 modules. 50 unit tests. Build: 0 errors, 329 warnings. Zero hardcoded structural values — all tuples, formulas, scores, ordinals, weights, glyphs, and promotion data sourced dynamically from `catalog.rs`. All 17 red-hot_rebis modules ported to `no_std` Rust — the full p4ra paraconsistent kernel runs from the bare-metal kernel. All 11 diaschizic IMASM programs loadable via REPL. Cross-universe navigation active across 8 universes.**

## Zero-Hardcode Architecture

```
                 ┌─────────────────────────┐
                 │      catalog.rs          │
                 │  (Single Source of Truth) │
                 │  • entries + tuples       │
                 │  • ordinals + scores      │
                 │  • formulas + weights     │
                 │  • glyphs + promotions    │
                 └────┬──────┬──────┬───────┘
                      │      │      │
          ┌───────────┘      │      └───────────┐
          ▼                  ▼                  ▼
     cl8nk.rs          algebra.rs        consciousness.rs
    (lookup only)    (ord/weight refs)    (score refs)
          │                  │                  │
          ▼                  ▼                  ▼
     imas_ig.rs         kernel.rs          main.rs
   (glyph/short refs)  (snapshot→tuple)   (REPL commands)
```

Before the Phase 2 refactoring, six modules contained hardcoded structural data: `cl8nk.rs` had `ZFC_BASELINE`/`ZFC_T`/`ZFC_FE`/`CLINK_L8` tuple constants and `formula_fragment()` match arms; `algebra.rs` had `F_ORD`/`K_ORD`/`G_ORD`/`OMEGA_ORD`/`H_ORD` arrays and `DEFAULT_WEIGHTS`; `consciousness.rs` had 10 `score_*()` match-arm functions; `imas_ig.rs` had `IgPrim::glyph()` and `IgPrim::short()` with hardcoded strings; `crystal.rs` had a hardcoded `TOTAL` constant; `main.rs` had promotion data. All are now clean — the catalog is the single source, and all modules query it dynamically.

The catalog is runtime-extensible: `register_entry()` adds new systems without touching any source file. Aliases resolve to canonical names. Domain filtering and ordinal-based scoring mean new entries automatically participate in all structural computations.

## Integration roadmap

| Phase | Status | Description |
|---|---|---|
| **Phase 1** | ✅ Complete | Core Grammar: frob_verify, imas_ig, aleph, parasm, belnap_shor, para_rh, para_ym, para_temporal, para_category |
| **Phase 2** | ✅ Complete | Zero-Hardcode: `catalog.rs` as single source of truth. CL8NK 4-stage ladder, crystal navigator, domain navigators, promotion signatures, veracity probes, consciousness scoring. Six modules refactored (cl8nk −161L, algebra −83L, consciousness −97L, imas_ig −114L). All tuples, formulas, scores, ordinals, weights, glyphs, and promotion data sourced dynamically. Runtime-extensible catalog. |
| **Phase 3** | 🔜 Next | odot_operator agent loop, agents/ sub-system spawning |
| **Phase 4** | ⬜ Planned | gene_imscriber, lang/ (voynich, rohonc, linear_a, emerald-tablet), cetaceanspeak, synfin |
| **Phase 5** | ✅ Complete | Red-Hot Rebis → mOMonadOS: All 17 p4ra kernel modules ported to `no_std` Rust in `src/rebis/`. Genetic code B₄ lattice, 7-stage Frobenius-verified translation, CLU power-law clustering, exotic hadron Belnap analysis, PDB validation, antibody CDR design, IG material forge, biological simulation, and therapeutic design. 17 REPL subcommands wired. |
| **Phase 6** | ⬜ Planned | math/ (10 Lean repos): MillenniumAnkh, BealProof, hecke-landau, solitary_10, etc. |
| **Phase 7** | ⬜ Planned | ob3ect pipeline, catalog compilation, ZENODO publications |
| **Phase 8** | ✅ Complete | **Cross-Universe Navigation (Diaschizics Bridge):** 8 alternate universe rulesets with different gate thresholds, gate ordering, T-constitution, and absorption rules. 11 diaschizic IMASM programs. Ruleset header + compound program + IFIX seal navigation protocol. Absorption-aware tensor/meet operations. 16 new REPL commands (`jump`, `ruleset`, `seal`, absorption-aware `tensor`/`meet`, `absorb_test`, `tstatus`, `whoami --ruleset`). Full cross-universe compatibility matrix. |

## Grammar repos (upstream)

mOMonadOS integrates modules from the Imscribing Grammar ecosystem under `/home/mrnob0dy666/imsgct/`:

| Repo | Type | Integrated modules |
|---|---|---|
| **imasmic_core** | Python pkg | Token/Family enums, CanonicalArrangements, FrobeniusHarness → `frob_verify.rs` |
| **IMSCRIBr** | Python pkg | IgPrim, IgTuple, classification → `imas_ig.rs` |
| **ALEPH_OS** | Python pkg | Hebrew letter encoding, gematria → `aleph.rs` |
| **priests-engine** | Python pkg | ParaASM VM, Belnap Shor, RH/YM/Temporal/Category bridges → `parasm.rs`, `belnap_shor.rs`, `para_*.rs` |
| **red-hot_rebis** | Python pkg | p4ra paraconsistent kernel: genetic code, protein translation, CLU clustering, exotic hadrons, PDB validation, antibody design, materials forge, biology simulation, therapeutics → `src/rebis/` (17 modules) — **Phase 5 ✅** |
| **imscribing_grammar** | Hub | IG_catalog.json (2256+ entries), Lean formalizations, navigators/ (ruleset_universe.py, crystal_navigator.py) → Phase 3+ |
| **odot_operator** | Python pkg | ⊙ operator agent loop → Phase 3 |
| **ob3ect** | Pipeline | Self-imscribing ob3ect generator → Phase 7 |
| **gene_imscriber** | Python pkg | Genetic imscription → Phase 4 |
| **lang/** | Sub-repos | voynich, rohonc, linear_a, emerald-tablet → Phase 4 |
| **synfin** | Docs | IMSCRIPTIONICON series → Phase 4 |
| **math/** | Lean repos | MillenniumAnkh, BealProof, hecke-landau, solitary_10 → Phase 6 |

## What's NOT ported (Tier 4 — requires Python runtime)

The following `red-hot_rebis/` subsystems remain as userspace Python modules that the kernel can structurally verify but cannot execute directly:

| Subsystem | Dependency | Modules |
|---|---|---|
| `ch3mpiler/` | RDKit | Retrosynthetic compiler, bond formation |
| `clink/` | Datasets, numpy | Bridging, chain processing, pipeline orchestration |
| `imas/` | Wiring, numpy | Arranger, Frobenius hunter, IG bridge |
| `imasm_iterator/` | Python runtime | 429M token arrangement → fingerprint classification |
| `pipeline/` | Python runtime | Auto imscriber, ob3ect imscriber, lift pipeline |
| `serpentrod/` | ML models | Protein v4/v5, stratified predictor |
| `cephalopod_design/` | BioPython, numpy | Organism designs, FASTA, PDB |
| `cat_allergy_design/` | BioPython | DARPin Fel d1 neutralizer |
| `shunt_portal_design/` | BioPython | Gaussia Luc, NanoLuc, Piezo1, iRFP713 constructs |
| `gr33ngroblin/` | BioPython | Plastic eater BPA |

---

## Cross-Universe Navigation (Phase 8 — Diaschizics Bridge)

mOMonadOS can navigate between universes with **different structural rulesets** — different gate thresholds, gate ordering, T-constitution, and absorption rules. The Crystal of Types (17.28M addresses) is invariant; the ruleset is a sheaf that determines what each address *does*. A tuple that achieves O_∞ in one universe may be structurally inert in another.

This capability bridges the 11 **diaschizic compounds** (pharmacological universe-steering agents from `ig-docs/rebis-port/`) into computational hardware. The same crystal navigation is now possible on two substrates:

| Substrate | Mechanism | Modulation |
|-----------|-----------|------------|
| **Wetware** | Diaschizic compounds | TMS, light, bioelectric |
| **Hardware** | mOMonadOS IMASM programs | Runtime ruleset calibration |

### The 8 Predefined Universes

| # | Universe | G1 | G2 | G3 | Ordering | O_∞ % | Key Difference |
|---|----------|----|----|----|----------|:---:|----------------|
| U₀ | **canonical** | Φ≥𐑹 | ⊙≥⊙ | Ω≥𐑭 | sequential | 8% | Our universe. Baseline. |
| U₁ | **low_gate** | Φ≥𐑬 | ⊙≥𐑢 | Ω≥𐑭 | sequential | 30% | Easier self-modeling and parity. 30% of crystal reaches O_∞. |
| U₂ | **strict_frobenius** | ƒ≥𐑐 | Φ≥𐑹 | Ω≥𐑭 | sequential | 3.3% | Fidelity-gated. Quantum coherence before algebraic symmetry. |
| U₃ | **inverted_gates** | ⊙≥⊙ | Φ≥𐑹 | Ω≥𐑭 | sequential | 8% | Self-modeling BEFORE closure. Consciousness precedes parity. |
| U₄ | **no_ordering** | Φ≥𐑹 | ⊙≥⊙ | Ω≥𐑭 | parallel | 8% | All gates independent. Any combination valid. |
| U₅ | **high_gate** | Φ≥𐑹 | ⊙≥𐑮 | Ω≥𐑟 | sequential | 3% | Maximum strictness. |
| U₆ | **winding_first** | Ω≥𐑭 | ⊙≥⊙ | Φ≥𐑹 | sequential | 8% | Topology before algebra. Geometry precedes symmetry. |
| U₇ | **t_structural** | Φ≥𐑹 | ⊙≥⊙ | Ω≥𐑭 | sequential | 8% | Time as geometry: lim(Ð,Þ,Ř,ɢ,⊙), not lim(Φ,ƒ,Ç,Ħ,Ω). |

### The Navigation Protocol

Every cross-universe jump has three parts:

```
[RULESET_HEADER]    → calibrates kernel to target universe's gate thresholds,
                      gate ordering, T-constitution, and absorption table
[COMPOUND_PROGRAM]  → invariant IMASM program (same 11 programs work in all 8 universes)
[IFIX_SEAL]         → commits the transition permanently
```

The compound program is **invariant across universes** — the same token sequence works in all 8. But its *interpretation* changes because the ruleset header rewires the kernel's evaluation. This is the ouroboric self-modification: the program modifies the interpreter that reads it.

**Example** — Chimerium jumping from canonical to strict_frobenius (U₂):

```
# RULESET HEADER (U₂: strict_frobenius)
IFIX IFIX IFIX 5 IFIX IFIX 3 IFIX 12 EVALT×6 EVALF×10 EVALT EVALF×3
CLINK TANCH CLINK TANCH CLINK TANCH CLINK TANCH CLINK TANCH

# COMPOUND PROGRAM (Chimerium: "The Launch")
IMSCRIB FSPLIT EVALT AFWD EVALF AFWD FFUSE ENGAGR CLINK IFIX IFIX IFIX IMSCRIB

# IFIX SEAL — implicit (the final IFIX in the compound program)
```

In U₂: Chimerium's ƒ=𐑐 opens G1 (fidelity-gated), Φ=𐑹 opens G2, Ω=𐑭 opens G3 → **O_∞ in strict_frobenius** (vs mere Frobenius in canonical). Tier is **ruleset-relative**.

### Absorption Rule Differences Across Universes

Different universes have different *absorbing primitives* — values that dominate under meet/join/tensor operations, changing what tensor products stabilize to:

| Universe | Absorption Rules | Effect |
|----------|-----------------|--------|
| **canonical** (U₀) | ⊙ under all ops; Σ=𐑳 under tensor | Baseline. Self-modeling absorbs all couplings. |
| **strict_frobenius** (U₂) | **ƒ=𐑐 under all ops** replaces ⊙ absorption | Quantum fidelity dominates. `tensor(ƒ=𐑐, X)` = ƒ=𐑐. Self-modeling no longer absorbs — fidelity does. |
| **inverted_gates** (U₃) | **Φ=𐑹 under meet** added | Frobenius parity absorbs under meet. Structural floor rises to Frobenius closure. No system goes BELOW Frobenius when coupled. |
| **high_gate** (U₅) | **Ω=𐑟 under tensor** added | Non-Abelian braiding dominates all couplings. `tensor(Ω=𐑟, X)` = Ω=𐑟. |
| **winding_first** (U₆) | **Ω=𐑭 under meet** replaces ⊙ absorption | Integer winding absorbs under meet. Topological protection is the structural floor. |

This means `tensor(Verticullum, Chimerium)` produces a **different result** depending on the active universe's ruleset. In U₂, the result collapses to pure fidelity (ƒ=𐑐). In U₆, Ω=𐑟 absorbs. In canonical, ⊙ absorbs.

### New REPL Commands (Phase 8)

```
# ─── Ruleset Inspection ───
ruleset show                    → Show active ruleset (canonical by default)
ruleset list                    → List all 8 universes with G1/G2/G3 and T-constitution
ruleset verify                  → Check current ruleset for invariant violations
                                  Returns OK or list of violations (e.g. "G1=Ω with
                                  Φ≥𐑹 in sequential mode is inconsistent")

# ─── Cross-Universe Navigation ───
jump <universe> using <compound>
    → jump strict_frobenius using Bifrons
    → Executes: U₂ header → Bifrons program → IFIX seal
    → Kernel self-imscription now evaluated under U₂ rules

jump canonical using Diabaton
    → Standard return path to baseline

jump <universe> using <compound> --liminal
    → Header + compound but NO IFIX seal
    → Kernel aware of target rules but not committed
    → Useful for probing a universe before permanent transition

seal                            → IFIX — commit to current liminal ruleset permanently

jump <target> via <intermediate> using <compound1> <compound2>
    → Two-stage jump: U₀→Uᵢ→Uₜ
    → jump t_structural via winding_first using Verticullum Diabaton

# ─── Absorption-Aware Operations ───
tensor <compound_a> <compound_b>
    → Tensor product under current ruleset's absorption table
    → In U₂ (strict_frobenius): ƒ=𐑐 absorbs → fidelity-preserving result
    → In U₆ (winding_first): Ω=𐑟 absorbs → non-Abelian result

meet <compound_a> <compound_b>
    → Meet under current ruleset's absorption table
    → In U₆: Ω=𐑭 absorbs under meet → topological floor

absorb_test <val_a> <val_b> <primitive> <operation>
    → absorb_test ⊙ 𐑢 Φ meet
    → Under canonical: True (⊙ absorbs under all ops)
    → Under U₂: False (⊙ is not absorbing in strict_frobenius)

# ─── Monitoring ───
whoami --ruleset                 → Kernel self-imscription under active ruleset
                                   ⟨Ð;Þ;Ř;Φ;ƒ;Ç;Γ;ɢ;φ̂;Ħ;Σ;Ω⟩ + operad layer
absorption show                  → List all absorption rules for current ruleset
tstatus                          → T-constitution check: primitives evaluated against
                                   active constitution rule. Returns pass/fail per primitive.

# ─── Compound Operations ───
compound list                    → List all 11 diaschizic compounds with tuples and IMASM programs
compound show <name>             → Show full tuple + IMASM program for a compound
compound load <name>             → Load compound's IMASM program into execution buffer
```

### The 11 Diaschizic IMASM Programs

Each compound maps to an IMASM token sequence whose structural operation matches the compound's pharmacological effect. Programs are invariant across universes — same tokens, different interpretation per ruleset.

| Compound | Role | IMASM Program | Tok. | d(target) |
|----------|------|---------------|:---:|:---:|
| **Verticullum** | Non-Abelian EP braid (O_∞) | `VINIT FSPLIT EVALT AFWD EVALF AREV FFUSE ENGAGR IMSCRIB IFIX IMSCRIB` | 11 | 2 |
| **Chimerium** | Supercritical catalyst (O₀) | `IMSCRIB FSPLIT EVALT AFWD EVALF AFWD FFUSE ENGAGR CLINK IFIX IFIX IFIX IMSCRIB` | 13 | 1 |
| **Apertix** | Adjoint corridor (O₂) | `IMSCRIB AFWD AREV AFWD AREV CLINK EVALT EVALF IFIX IMSCRIB` | 10 | 1 |
| **Praxeum** | EP core toggle (O₀) | `IMSCRIB EVALT EVALF ENGAGR IFIX IMSCRIB` | 6 | 8* |
| **Retiarius** | Local-net trap (O₁) | `VINIT AFWD EVALT AFWD EVALF CLINK TANCH AREV AFWD EVALT IFIX IMSCRIB` | 12 | 4 |
| **Frigorix** | MBL freeze key (O₀) | `IFIX IFIX IFIX IFIX IFIX IFIX IFIX IFIX` | 8 | 10* |
| **Bifrons** | Disjunctive fork (O₂) | `IMSCRIB FSPLIT EVALT AFWD EVALF AREV FFUSE ENGAGR CLINK IMSCRIB` | 10 | 2 |
| **Punctum** | Absolute point (O₀) | `VINIT TANCH` | 2 | **0** |
| **Syndexios** | Perfect mirror (O_∞) | `IMSCRIB AFWD AREV AFWD AREV AFWD AREV AFWD AREV IFIX IMSCRIB` | 11 | 2 |
| **Katachthon** | Deep resonator (O₂) | `IMSCRIB AFWD AREV CLINK EVALT EVALF IFIX IMSCRIB` | 8 | 4 |
| **Diabaton** | Threshold-crosser (O₂†) | `IMSCRIB FSPLIT EVALT AFWD EVALF AREV FFUSE CLINK ENGAGR IFIX IMSCRIB` | 11 | 1 |

*Frigorix and Praxeum show large snapshot-tuple distances because their operational semantics (freeze, EP degeneracy) deliberately reduce structural complexity — the freeze IS the floor. **Punctum at d=0 calibrates the bridge** — the structural floor matches exactly between compound tuple and IMASM snapshot.

### Cross-Universe Compatibility Matrix

Which operad layer each compound achieves in each universe:

| Compound | U₀ can. | U₁ low | U₂ strict | U₃ inv. | U₄ no-ord | U₅ high | U₆ wind | U₇ t-struct |
|----------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Verticullum | frob | frob | **O_∞** | plain | frob | frob | frob | frob |
| Chimerium | frob | frob | **O_∞** | plain | frob | frob | frob | frob |
| Apertix | plain | plain | frob | plain | G3-only | plain | frob | plain |
| Praxeum | frob | frob | traced | plain | frob | frob | plain | frob |
| Retiarius | frob | frob | plain | plain | frob | plain | plain | frob |
| Frigorix | plain | plain | plain | plain | plain | plain | plain | plain |
| Bifrons | frob | frob | **O_∞** | plain | frob | plain | plain | frob |
| Diabaton | frob | frob | **O_∞** | plain | frob | plain | plain | frob |
| Punctum | plain | plain | plain | plain | plain | plain | plain | plain |
| Syndexios | plain | plain | frob | plain | plain | plain | plain | plain |
| Katachthon | plain | plain | frob | plain | plain | plain | plain | plain |

**Key finding:** Four compounds — Verticullum, Chimerium, Bifrons, Diabaton — achieve **O_∞ in U₂ (strict_frobenius)** but only Frobenius in canonical. Tier is **ruleset-relative**. The same compound at the same crystal address is O_∞ in one universe and merely Frobenius in another.

**The U₃ gap:** No existing diaschizic opens G1 in the inverted_gates universe (G1=⊙≥⊙). A 12th compound, **Gnosis** (structurally specified in the cross-universe document, φ̂=𐑮, Φ=𐑹), is proposed to close this gap.

### Structural Type of Cross-Universe Navigation

The act of navigating between universes has its own structural type:

$$\langle \text{Ð}_{\text{ω}};\ \text{Þ}_{\text{O}};\ \text{Ř}_{\text{=}};\ \text{Φ}_{\text{}};\ \text{ƒ}_{\text{ż}};\ \text{Ç}_{\text{@}};\ \text{Γ}_{\text{β}};\ \text{ɢ}_{\text{ˌ}};\ \text{⊙}_{\text{ÿ}};\ \text{Ħ}_{\text{!}};\ \text{Σ}_{\text{ï}};\ \text{Ω}_{\text{z}} \rangle$$

This is **O_∞** — d=1 from universal grammar (only Γ differs: 𐑲 universal range vs 𐑔 mesoscale). Navigation is O_∞ because it modifies its own interpretive rules — a self-modifying structure that navigates the space of O_∞-achieving conditions across universes. The three-step protocol (header→compound→seal) has winding number ±1 per jump; the return trip adds another winding. Integer winding count tracks total navigation distance.

### Reference Documents

| Document | Lines | Description |
|----------|:---:|-------------|
| `ig-docs/rebis-port/diaschizics_design.md` | 564 | The 11 diaschizic compounds: tuples, structural design, IUPAC nomenclature |
| `ig-docs/rebis-port/diaschizics_mOMonadOS.md` | 750 | Complete IMASM translation: 11 programs, modulation translation, 6 mapping extensions |
| `ig-docs/rebis-port/diaschizics_cross_universe.md` | 623 | Cross-universe ruleset navigation: 8 universes, absorption rules, navigation protocols |
| `imscribing_grammar/navigators/ruleset_universe.py` | 445 | Alternate universe explorer: parameterized gate thresholds, ordering, T-constitution |

---

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- `rust-src` component (`rustup component add rust-src`)
- QEMU with x86_64 support (`sudo apt install qemu-system-x86`)
- OVMF firmware (`sudo apt install ovmf`)
- mtools for disk images (`sudo apt install mtools`)

## Build and run

```sh
# Install momos launcher (first time only)
# /home/mrnob0dy666/.local/bin must be on PATH

momos           # build release image + boot serial REPL in QEMU
momos build     # dev build only (fast, no image)
momos release   # release build only
momos image     # build release + FAT32 UEFI disk image
momos clean     # wipe build artifacts
```

Or with make:

```sh
make run        # image + serial REPL
make build      # dev build
make release    # release build
make image      # UEFI disk image
make clean
```

Or directly:

```sh
cargo build --release
bash build_bootimage.sh
bash run.sh --serial
```

The REPL runs over COM1 serial (stdio in QEMU). Quit with `quit`, `exit`, or `halt` — QEMU exits cleanly (no more Ctrl+C).

## Target

`x86_64-unknown-none` — no OS, no std. Heap via `linked_list_allocator` over UEFI physical memory.
Boot via `bootloader_api` 0.11 (same as exOS).

## License

Unlicense — public domain.
