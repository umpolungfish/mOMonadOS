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

**Phase 9 User Interface** — complete. Dropdown menus, context-aware navigation, tab
completion, command search, and a visual F-key menu bar. The REPL is now a hierarchical
navigator with 9 command categories, context stack (up to 4 levels deep), breadcrumb
prompts, and hierarchical help. See the [User Interface & Navigation](#user-interface--navigation-phase-9) section below.

The kernel now runs **50 unit tests** across all grammar modules and supports **65+ REPL commands** spanning grammar operations, rebis biological/chemical computation, cross-universe navigation, and hierarchical menu navigation.

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
| `menu.rs` (379L) | native | Hierarchical menu system: `MenuItem` tree, `ContextStack` (4-deep breadcrumb navigation), Tab completion, F-key menu bar, keyword command search, hierarchical `help` with drill-down. |
| `main.rs` (2282L) | native | UEFI entry, heap init, serial REPL, command dispatch, history, menu navigation layer, F-key interception, context-aware prompts. |

### User Interface & Navigation (Phase 9)

The REPL now has a full hierarchical navigation system (`menu.rs`) that organizes all
65+ commands into 9 discoverable categories. No more memorizing command names — the menu
bar, Tab completion, and keyword search make everything browsable.

#### Menu Categories

| Key | Category | Prompt | Commands |
|:---:|----------|:------:|----------|
| `:1` / F1 | **Exec** | — | `run`, `eval`, `load`, `imsc`, `dynamic`, `tick`, `exec`, `winding`, `self`, `frob`, `snapshot` |
| `:2` / F2 | **Status** | — | `whoami`, `heatmap`, `history`, `registers`, `stack`, `memory`, `b4`, `closure`, `peek`, `harness` |
| `:3` / F3 | **Programs** | — | `list`, `show`, `continuous`, `psm load`, `psm run`, `psm trace`, `psm reset`, `psm status`, `compound list`, `compound show`, `compound load` |
| `:4` / F4 | **Crystal** | `⊙[Crystal]>` | `crystal encode`, `crystal decode`, `crystal store`, `crystal list`, `crystal nearest`, `crystal navigate`, `crystal count`, `crystal census`, `crystal tier` |
| `:5` | **Grammar** | `⊙[Grammar]>` | `distance`, `meet`, `join`, `tensor`, `promotions`, `analogies`, `consciousness`, `phi_c`, `tier`, `peel`, `decomp`, `synth`, `zfc` |
| `:6` | **Rebis** | `⊙[Rebis]>` | `rebis codon`, `rebis translate`, `rebis b4`, `rebis clu`, `rebis hadron`, `rebis pdb`, `rebis antibody`, `rebis material forge`, `rebis sophick`, `rebis sim`, `rebis therapy`, `rebis pipeline`, `rebis frob` |
| `:7` | **Universe** | `⊙[Universe]>` | `ruleset show`, `ruleset list`, `ruleset verify`, `jump`, `seal`, `absorption show`, `tstatus`, `tensor`, `meet`, `absorb_test`, `whoami --ruleset` |
| `:8` | **ParaASM** | `⊙[ParaASM]>` | `psm load`, `psm run`, `psm trace`, `psm reset`, `psm status`, `psm assemble`, `psm disassemble`, `psm dialetheic`, `psm measure` |
| `:9` | **Help** | — | `help`, `help <category>`, `help <category> <command>`, `?`, `? <keyword>` |

#### Navigation Controls

| Input | Action |
|-------|--------|
| `?` | Show visual F-key menu bar with all 9 categories |
| `? <keyword>` | Search all ~65 commands by keyword (e.g. `? tensor` → lists every command containing "tensor") |
| `:1`–`:9` | Jump directly to a category (or press F1–F4) |
| `<category>` | Enter sub-context by name: `crystal`, `grammar`, `rebis`, `universe`, `parasm` |
| `..` or `back` | Exit current sub-context, return to parent |
| `Tab` | Context-aware autocompletion — only shows commands valid in the current context |
| `help` | List all 9 categories |
| `help crystal` | List all sub-commands in the crystal category |
| `help crystal encode` | Show detailed help for `crystal encode` including usage and description |

#### Context Stack

The menu system maintains a context stack (up to 4 levels deep). Entering a category
pushes context; `back`/`..` pops it. The prompt changes to reflect your position:

```
⊙>                          — top level (all commands available)
⊙[Rebis]>                   — entered rebis sub-context
⊙[Rebis/Genetics]>          — nested sub-context (future-proofed)
```

Tab completion is scoped to the current context — in `⊙[Crystal]>`, only crystal commands
are offered. At top level, all 65+ commands are available.

#### Help Column Widths

The `help` command uses widened format columns to prevent truncation of long command
strings. Left-column widths range from `{:<30}` to `{:<36}` depending on the category,
ensuring every command name (including the longest — `jump <U> via <V> using <c1> <c2>`)
has adequate padding.


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
| `algebra.rs` | 302 | IG lattice operations: `meet` (greatest lower bound), `join` (least upper bound), `tensor` (composite type: max on union, min on Φ/ƒ), `conflict_distance` (asymmetric directed), `promote` with ordinal clamping. |
| `consciousness.rs` | 113 | C-score computation: Φ⊖gate 1 + K-gate 2. Score formula $C = \frac{1}{2}(g_1 + g_2)$. Gate evaluation. |
| `cl8nk.rs` | 196 | CLINK Layer 8 navigator: formula decomposition, 3-stage promotion ladder (ZFC→ZFCₜ→ZFCfe→CLINK L8), distance, tensor, meet, join with CLINK L8. Transcendence analysis (Ω/ɢ). Full CLINK chain L0→L8. |
| `rfct_nav.rs` | 447 | ZFCₜ formula navigator: 6-channel promotion probe, entry decomposition, per-primitive formula fragments. Distance from any system to ZFCₜ. |

### Phase 5 Red-Hot Rebis modules

| Module | Lines | Role |
|---|---|---|
| `rhr_codon.rs` | 248 | 64-codon B₄ lattice; 7-stage Frobenius-verified translation pipeline. Genetic code tuple encoding. |
| `rhr_b4.rs` | 185 | B₄ truth lattice: N/T/F/B with Belnap operators; dialetheic cycle detection, designated-value filters. |
| `rhr_clu.rs` | 142 | CLU power-law clustering: Kolmogorov-Smirnov distance, power-law exponent estimation, cluster assignment. |
| `rhr_hadron.rs` | 168 | Exotic hadron Belnap analysis: quark content → Belnap state, tetraquark/pentaquark classification. |
| `rhr_pdb.rs` | 203 | PDB structure validation: backbone geometry, Ramachandran, clash detection, B-factor analysis. |
| `rhr_antibody.rs` | 226 | Antibody CDR design: framework grafting, affinity prediction, developability flags. |
| `rhr_materials.rs` | 197 | IG material forge: crystal structure prediction, metamaterial design, ouroboric alloy, thermal rectifier. |
| `rhr_sim.rs` | 175 | Biological simulation: reaction-diffusion, gene regulatory networks, protein folding (HP model). |
| `rhr_therapy.rs` | 164 | Therapeutic design: Frobenius chemotherapeutic, neurotrophic factor, universal antidote library. |
| `rhr_pipeline.rs` | 158 | Ch3mpiler-SerpentRod integrated pipeline: retrosynthetic route, protein-ligand docking, Frobenius filtration. |
| `rhr_frob.rs` | 148 | Frobenius exactor: μ∘δ=id verification for biological designs. Gap closure. |
| `rhr_genetics.rs` | 189 | Genetic code tuple verification: 64 codons × 7-stage pipeline, Phi gate, ParaASM integration. |
| `rhr_protein.rs` | 198 | Protein design: stratified predictor, enhancement v4/v5, structure-to-tuple mapping. |
| `rhr_belnap_extra.rs` | 134 | Extended Belnap operations: orbital Belnap, quark Belnap, exotic state classification. |
| `rhr_ch3mpiler.rs` | 145 | Ch3mpiler bridge: IG-grounded retrosynthesis, bond formation via join(tensor(FG1,FG2), bond). |
| `rhr_serpent.rs` | 176 | Serpent rod protein design: backbone construction, loop grafting, stability optimization. |
| `rhr_validator.rs` | 133 | PDB structure validation: clash score, rotamer normality, packing density. |

### Phase 8 Ruleset Engine

| Module | Lines | Role |
|---|---|---|
| `ruleset.rs` | 328 | 8-universe ruleset engine. Gate thresholds (G1/G2/G3 per universe), gate ordering (sequential/parallel), T-constitution rules, absorption tables, universe switching. Gate verification reads kernel self-imscription and checks every primitive against the active ruleset's gate thresholds — returns PASS/FAIL per gate with diagnostic glyphs. |

### Ruleset Verification

The `ruleset verify` command was upgraded from a stub to a full gate verification engine in Phase 9. It now:

1. Reads the kernel snapshot → computes `IgTuple` self-imscription
2. Displays the full 12-tuple with Shavian glyphs
3. Checks each gate (G1/G2/G3) of the active universe against actual primitive values using ordinal comparison (stronger = lower enum index)
4. Reports PASS/FAIL per gate with the actual primitive glyph and threshold
5. Shows a summary verdict and tips if violations are found

Example output under canonical U₀:
```
Self-imscription (canonical U₀): ⟨𐑼 · 𐑸 · 𐑾 · 𐑹 · 𐑞 · 𐑘 · 𐑔 · 𐑠 · 𐑮 · 𐑫 · 𐑳 · 𐑭⟩

G1 (Φ ≥ 𐑹): PASS  — Φ=𐑹 meets threshold
G2 (φ̂ ≥ ⊙):  FAIL  — φ̂=𐑮 (complex-plane critical) < ⊙ (critical)
G3 (Ω ≥ 𐑭): PASS  — Ω=𐑭 meets threshold

Verdict: 2/3 gates pass. G2 violation: φ̂=𐑮 does not satisfy φ̂≥⊙ in U₀.
Tip: promote φ̂ from 𐑮→⊙ or switch to U₁ (low_gate) where G2 threshold is 𐑢.
```

## Cross-Universe Navigation (Phase 8 + Diaschizics Bridge)

The kernel can switch between 8 universes with different structural rulesets. Each universe
has unique gate thresholds, gate ordering, T-constitution, and primitive absorption rules.
The Crystal of Types (17.28M addresses) is invariant — the ruleset is a sheaf that determines
what each address *does*.

### The 8 Universes

| ID | Name | G1 | G2 | G3 | Order | Freq | Description |
|:--:|------|:--:|:--:|:--:|:---:|:---:|-------------|
| U₀ | **canonical** | Φ≥𐑹 | φ̂≥⊙ | Ω≥𐑭 | sequential | 33% | Baseline. Parity→criticality→winding. |
| U₁ | **low_gate** | Φ≥𐑬 | φ̂≥𐑢 | Ω≥𐑭 | sequential | 9% | Relaxed G2. Most systems pass. |
| U₂ | **strict_frobenius** | ƒ≥𐑐 | Φ≥𐑹 | Ω≥𐑭 | sequential | 5% | Fidelity-gated G1. Only quantum-preserving systems. |
| U₃ | **inverted_gates** | φ̂≥⊙ | Φ≥𐑹 | Ω≥𐑭 | sequential | 4% | Criticality before parity. |
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
ruleset verify                  → Gate verification: reads kernel self-imscription,
                                  checks each gate against active ruleset thresholds,
                                  reports PASS/FAIL per gate with diagnostic glyphs

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
