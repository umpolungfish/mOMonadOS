# Paper 159 — A Two-Layer D-FUMT₈ Reconstruction of Priest-Garfield's Inclosure Schema for Nāgārjuna's Catuṣkoṭi: A Formal Substrate Compatible with Multiple Interpretive Stances

**Version**: v0.2 LEAN-4-BUILT (★ machine-verified — 9 load-bearing theorems *sans* sorry + zero kernel-axiom dependence, see Appendix B)

**Author**: Nobuki Fujimoto (藤本 伸樹), with Claude (Rei-AIOS)
ORCID: 0009-0004-6019-9258 · GitHub: fc0web · note.com: nifty_godwit2635

**Date**: 2026-05-31 (v0.1 OUTLINE → v0.2 LEAN-4-BUILT, same date)

**Companion to**:
- Paper 61 — *Zero-Centered Symbol Grammar (ZCSG)* — [DOI 10.5281/zenodo.15217458](https://doi.org/10.5281/zenodo.15217458) (world-first mathematical encoding of śūnyatā-of-śūnyatā as `0`)
- Paper 77 — *LeanDFumt: an Eight-Valued Logic Library for Lean 4* — substrate of the present formalization

**Status header (★ load-bearing)**:
- ✅ Primary source confirmed: Garfield & Priest 2003 "Nāgārjuna and the Limits of Thought" (Philosophy East and West 53(1): 1-21) — full text read 2026-05-31
- ✅ Pre-existing Rei artifacts surveyed: STEP 476 `nagarjuna-fde-western-engine.ts` (TS, FDE ≅ DFUMT4 isomorphism) + STEP 513 `operator-fixed-point-atlas.ts` (Ω operational fixed-point test) + Paper 77 LeanDFumt substrate
- ✅ **Lean 4 code in §3 BUILT** (2026-05-31): `data/lean4-mathlib/CollatzRei/Paper159Inclosure.lean`, `lake build` exit 0 (1.7s), all 9 load-bearing theorems verified `#print axioms` shows "**does not depend on any axioms**" (no `sorryAx`, no `Classical.choice`, no `propext`, no `Quot.sound`, no `native_decide`). See Appendix B for full audit output. **Stronger than Paper 158** (which used `[propext, Classical.choice, Quot.sound]`).
- ⚠ Honest divergence finding (★ new in v0.2, §3.4): The Paper 159 upper-layer operator (renamed `omega_upper` in the Lean file) is **structurally idempotent but semantically distinct from** STEP 513's `omega`. Both share the SELF⟲ idempotency axis but contract to different fixed-point sets. v0.1 implied (incorrectly) a tighter relationship; this is corrected in v0.2 §3.4.
- ⚠ Honest design finding (★ new in v0.2, §3.5): The literal `Q (delta Q) ∧ ¬ Q (delta Q)` form CANNOT be formalized in classical Lean (it implies `False`). The Lean 4 file therefore expresses the limit-contradiction as a **DFUMT8 VALUE** (BOTH) at the lower layer, not as a classical Prop. The Inclosure structure records closure + transcendence as obligations; the "apparent contradiction" lives in the DFUMT8 type, not in Lean's Prop.
- ⚠ Tillemans 2009/2014/2024 + Siderits + Ferraro + Tao Jiang critic papers **NOT YET READ** (acknowledged in §4.2, citations from secondary triangulation only)
- ⚠ No claim of resolving the Priest-Garfield vs Tillemans scholarly debate (§4.3)
- ⚠ No "world-first" claim (§4.4, Paper 61 ZCSG + Priest-Garfield 2003 already issued such claims — this paper provides formal substrate only)

---

## Abstract

We propose a **two-layer D-FUMT₈ reconstruction** of the Inclosure Schema that Priest and Garfield (2003) apply to Nāgārjuna's catuṣkoṭi. Their original construction terminates the schema at the value BOTH — the dialetheist commitment to a true contradiction at the limits of thought (δ(Ω) is both empty and not empty). Our reconstruction preserves their lower-layer truth-value assignment (Priest's BOTH appears at our object layer) but introduces an **upper-layer modal operator Ω** (idempotent, Ω ∘ Ω = Ω) that maps the lower-layer BOTH to ZERO — the central axis of the ZCSG three-layer notation (Paper 61) corresponding to the "0₀ genesis layer" of emptiness-of-emptiness. The Lean 4 formalization is sketched against the Paper 77 LeanDFumt substrate; full machine verification is left to v0.2. We frame this strictly as **rational reconstruction following Priest-Garfield's own self-framing** (their explicit disavowal of "textual history" claims), and explicitly take no position in the Priest-Garfield vs Tillemans (2009+) scholarly debate. The formal substrate is compatible with multiple readings of catuṣkoṭi.

**Keywords**: D-FUMT₈, catuṣkoṭi, Inclosure Schema, Priest-Garfield, dialetheism, paraconsistent logic, Lean 4, rational reconstruction, śūnyatā, two-layer logic, Rei-AIOS, ZCSG, Nāgārjuna

---

## 1. Introduction

The catuṣkoṭi (四句分別) — Nāgārjuna's fourfold logical figure of affirmation, negation, both, and neither — has long resisted formalization in classical bivalent logic. Priest and Garfield (2003) made the most sustained recent attempt: they applied Priest's Inclosure Schema (Priest 1987, 2002) to two paradoxes they extract from the Mūlamadhyamakakārikā (MMK) — an **ontological paradox** they explicitly name "Nāgārjuna's Paradox" (every thing has one nature, namely no nature) and an **expressibility paradox** (the ultimate truth is that there is no ultimate truth, after Siderits 1989). In both, the limit value δ(Ω) is BOTH — a true contradiction in the sense of Priest's dialetheism.

The present paper proposes a **two-layer reconstruction** of this schema using the eight-valued D-FUMT₈ logic of the Rei-AIOS project (Fujimoto 2026, Paper 77). The lower layer assigns the same truth value BOTH to δ(Ω) that Priest-Garfield assign — preserving their analysis as a special case at the object level. An **upper-layer modal operator** Ω (with idempotency Ω ∘ Ω = Ω) is then applied to the lower-layer values: BOTH and NEITHER both contract to ZERO, the central axis of the Zero-Centered Symbol Grammar (ZCSG, Paper 61) corresponding to the **0₀ genesis layer**. The "all standpoints exhausted" reading of Nāgārjuna's catuṣpaścāś (四句百非, "four corners and one hundred negations") is thereby formally captured as an idempotent stable point.

This paper builds on three pre-existing Rei-AIOS artifacts:
1. **STEP 476 `nagarjuna-fde-western-engine.ts`**: a TypeScript engine demonstrating the structural isomorphism catuṣkoṭi ≅ FDE (Belnap 1977) ≅ D-FUMT₈ base 4 values
2. **STEP 513 `operator-fixed-point-atlas.ts`**: a TypeScript enumeration of fixed points for the operators Ω, Φ, Ψ, NOT and their compositions (320 fixed-point checks), which includes the operational test Ω ∘ Ω = Ω
3. **Paper 77 LeanDFumt**: the open-source Lean 4 library providing the inductive type `DFUMT8` with negation, conjunction, disjunction, and the `Decidable` infrastructure used here

This paper's contribution is to **promote** the catuṣkoṭi ≅ FDE ≅ DFUMT4 isomorphism (currently TypeScript-only) and the Ω idempotency stipulation (currently a SEED_KERNEL axiom plus an operational test) to **machine-verified Lean 4 theorems**, and to **add the upper-layer modal structure** that distinguishes our reconstruction from Priest-Garfield's BOTH-only terminus.

**Honest scope (★ load-bearing throughout)**:
- We follow Priest-Garfield's own self-framing of their work as "rational reconstruction" — not textual history (Garfield & Priest 2003: 2, "we do not claim that Nāgārjuna himself had explicit views about logic ... This is, hence, not textual history but rational reconstruction").
- We take no position in the dialetheist (Priest-Garfield, Deguchi-Garfield-Priest 2008 / 2013) versus weak-dialetheist (Tillemans 2009, 2024) debate, nor in the broader scholarly discussion (Siderits, Ferraro, Tao Jiang).
- We do not claim that Nāgārjuna was a dialetheist; we provide a formal substrate compatible with multiple interpretive stances.
- We do not claim "world first" status: Paper 61 (ZCSG) and Priest-Garfield (2003, who name "Nāgārjuna's Paradox" and write that the ontological paradox "to our knowledge is found nowhere else") have already issued related uniqueness claims. This paper supplies formal substrate only.

---

## 2. Background

### 2.1 Catuṣkoṭi: positive and negative uses (after Garfield & Priest 2003: 13-14)

Classical Indian logic and rhetoric regards a proposition as defining a logical space of four corners (*koṭi*): the proposition (true), its negation (false), both, and neither. The catuṣkoṭi receives two distinct uses in the MMK:

- **Positive tetralemma** — conventional perspective, e.g. MMK XVIII:6 ("That there is a self has been taught, And the doctrine of no-self, By the buddhas, as well as the Doctrine of neither self nor nonself"). The four corners are accepted at the conventional (*saṃvṛti*) level as distinct standpoints.
- **Negative tetralemma** — ultimate perspective, e.g. MMK XXII:11 ("'Empty' should not be asserted. 'Nonempty' should not be asserted. Neither both nor neither should be asserted. These are used only nominally"). At the ultimate (*paramārtha*) level, all four corners are negated.

Priest and Garfield's analysis focuses on this *negative* use — at the ultimate level, catuṣkoṭi exhibits limit phenomena that they then formalize via the Inclosure Schema.

### 2.2 Inclosure Schema (Priest 1987, 2002)

Priest's Inclosure Schema isolates the general structure of self-referential paradoxes. Given properties φ and ψ and an operator δ:

| Condition | Statement |
|---|---|
| (i) Transcendence | ∀X ⊆ Ω, ψ(X) → δ(X) ∉ X |
| (ii) Closure | ∀X ⊆ Ω, ψ(X) → δ(X) ∈ Ω |

Applying δ to Ω itself yields δ(Ω) ∈ Ω ∧ δ(Ω) ∉ Ω — the limit contradiction. Priest 1987/2002 catalogs Russell's paradox, the Burali-Forti paradox, the Liar, and others as instances. Priest-Garfield 2003 add Nāgārjuna's two paradoxes:

- **Ontological** ("Nāgārjuna's Paradox"): φ(χ) = "χ is empty"; ψ(X) = "X is a set of things with some common nature"; δ(X) = "the nature of things in X". Closure: since all things are empty, δ(X) ∈ Ω. Transcendence: δ(X) has no nature (because emptiness is exactly the absence of nature), so δ(X) ∉ X. Limit: δ(Ω) — emptiness itself — both is and is not empty.
- **Expressibility**: φ(χ) = "χ is an ultimate truth"; ψ(X) = "X is definable"; δ(X) = the sentence "there is nothing which is in D" (where D refers to X). The limit truth — "there is no ultimate truth" (Siderits 1989) — both is and is not an ultimate truth.

Priest-Garfield assign the value **BOTH** (true-and-false) to δ(Ω) in both cases.

### 2.3 D-FUMT₈ and the LeanDFumt substrate (Paper 77)

D-FUMT₈ is an eight-valued logic with constructors {TRUE, FALSE, BOTH, NEITHER, INFINITY, ZERO, FLOWING, SELF}. The base four values {TRUE, FALSE, BOTH, NEITHER} are isomorphic to Belnap's FDE (Anderson and Belnap 1975, Belnap 1977; structural isomorphism verified in STEP 476). The remaining four values extend the system with dimensional / reflective truth statuses. Paper 77 supplies the Lean 4 inductive type `DFUMT8` together with `neg`, `and`, `or`, `implies`, and the `Decidable` instance, all proved as 29 zero-sorry theorems via `decide`. The library is Mathlib-free, builds in approximately 5 seconds, and is released under Apache-2.0 (`github.com/fc0web/lean-d-fumt8`).

### 2.4 ZCSG three-layer notation (Paper 61)

The Zero-Centered Symbol Grammar (Fujimoto 2026, Paper 61, DOI [10.5281/zenodo.15217458](https://doi.org/10.5281/zenodo.15217458)) introduces a three-layer notation for emptiness:

| Symbol | Dimension | Nāgārjuna interpretation | Mathematical entity | D-FUMT₈ |
|---|---|---|---|---|
| o0 | −1 | emptiness-before-emptiness, unnameable | empty set with reduced homology H̃₋₁(∅) = ℤ | NEITHER |
| **0** | **0** | **śūnyatā-of-śūnyatā (pure origin) — the 0₀ genesis layer** | **fixed point, origin** | **ZERO** |
| 0o | +1 | dependent co-arising | dimension-bearing object | BOTH |

The **central axis 0** is the formal symbol Paper 61 introduces for "śūnyatā(śūnyatā) = śūnyatā" — emptiness applied to itself as a fixed point. This paper's upper-layer Ω is structurally a contraction map from dimension +1 (BOTH) to dimension 0 (the ZCSG central axis); the linkage is exhibited in §3.8.

---

## 3. Lean 4 Formalization (★ BUILT — machine-verified zero-sorry, zero-axiom-dependence)

All Lean 4 fragments below are **machine-verified** in `data/lean4-mathlib/CollatzRei/Paper159Inclosure.lean` against Lean 4 v4.27.0 + Mathlib v4.27.0 (Mathlib is loaded by the host project but not used by this file — the formalization is Mathlib-free in spirit; see §4.5). `lake build CollatzRei.Paper159Inclosure` exits 0 in ~1.7 seconds. All 9 load-bearing theorems pass `#print axioms` with "does not depend on any axioms" — i.e. they hold under the empty axiom set (purely constructive `decide` reasoning on a finite inductive type, no Classical reasoning required). See Appendix B for the full audit transcript.

Pinned commit: `4f5c50020f8ba0e589f8d6ebe278195ee812a1b7` (the commit immediately before the v0.2 update).

### 3.1 FDE inductive type

```lean
inductive FDE : Type where
  | t : FDE  -- true (and not false)
  | f : FDE  -- false (and not true)
  | b : FDE  -- both true and false
  | n : FDE  -- neither true nor false
  deriving Repr, DecidableEq, Inhabited
```

### 3.2 FDE-to-DFUMT8 embedding φ

```lean
def phi : FDE → DFUMT8
  | .t => .TRUE
  | .f => .FALSE
  | .b => .BOTH
  | .n => .NEITHER
```

### 3.3 Structural preservation theorems (lower layer) — partial machine verification

The TypeScript proofs in STEP 476 (`verifyNOT`, `verifyAND`, `verifyOR`, `verifyLattice`) are lifted to Lean 4 incrementally. **v0.2 covers negation preservation** (machine-verified); conjunction/disjunction/lattice preservation is deferred to v0.3 (the and/or truth tables are larger and have multiple non-equivalent semantics in the literature — Belnap bilattice meet/join vs Paper 77 LeanDFumt's operational meet — see honest scope note).

**v0.2 BUILT** (in `Paper159Inclosure.lean`):

```lean
def FDE.neg : FDE → FDE
  | .t => .f | .f => .t | .b => .b | .n => .n

theorem FDE.toDFumt8_neg : ∀ x : FDE, (FDE.neg x).toDFumt8 = DFUMT8.neg x.toDFumt8 := by
  intro x; cases x <;> decide
```

`#print axioms FDE.toDFumt8_neg` → "does not depend on any axioms".

**v0.3 DEFERRED**: `phi_and_preserves` / `phi_or_preserves` / lattice order preservation. Honest scope reason: Belnap's original 1977 bilattice gives a specific meet/join under the truth ordering, while Paper 77 LeanDFumt's `DFUMT8.and` / `DFUMT8.or` follow an operational meet (FALSE absorbs, NEITHER propagates) — these are NOT identical on the four-corner subset. Establishing the embedding requires either (a) re-aligning Paper 77 and/or FDE definitions to match, or (b) parameterizing the embedding by which lattice convention is in force. Both are non-trivial editorial choices that should be made deliberately rather than rushed.

### 3.4 Upper-layer modal operator `omega_upper` with idempotency (★ load-bearing — main differential)

**Pre-existing Rei artifacts** (no Lean 4 formalization of any version of this property prior to v0.2):
- SEED_KERNEL theory `dfumt-idempotency` (`src/axiom-os/seed-kernel.ts` line 239): stipulates "Ω(Ω(x)) → Ω(x) stability" as axiom.
- STEP 513 `operator-fixed-point-atlas.ts` lines 86-93: operational TypeScript test of a fixed-point structure for an operator named `omega`.

**★ Honest divergence finding (new in v0.2)**: STEP 513's `omega` has a **different concrete definition** from the operator Paper 159 needs. STEP 513 omega contracts `INFINITY → BOTH`, `ZERO → NEITHER`, `FLOWING → TRUE`, and is identity on `{TRUE, FALSE, BOTH, NEITHER, SELF}` (5 fixed points); the Paper 159 upper-layer operator contracts `BOTH → ZERO`, `NEITHER → ZERO`, and is identity on `{TRUE, FALSE, INFINITY, ZERO, FLOWING, SELF}` (6 fixed points). Both share the structural property `Ω ∘ Ω = Ω` (the SELF⟲ idempotency axis of D-FUMT₈), but their semantics — what they contract to what — differ.

To avoid silent name-clash, the Lean file names the Paper 159 operator **`omega_upper`**. STEP 513's operator (named `omega` in TypeScript) is not formalized in this file.

**v0.2 BUILT** (in `Paper159Inclosure.lean`):

```lean
def DFUMT8.omega_upper : DFUMT8 → DFUMT8
  | .TRUE     => .TRUE
  | .FALSE    => .FALSE
  | .BOTH     => .ZERO     -- ★ Priest-Garfield BOTH terminus contracted to 0₀
  | .NEITHER  => .ZERO     -- ★ catuṣkoṭi 4th corner contracted to 0₀
  | .INFINITY => .INFINITY
  | .ZERO     => .ZERO
  | .FLOWING  => .FLOWING
  | .SELF     => .SELF

theorem omega_upper_idempotent (x : DFUMT8) :
    DFUMT8.omega_upper (DFUMT8.omega_upper x) = DFUMT8.omega_upper x := by
  cases x <;> decide
```

`#print axioms omega_upper_idempotent` → "does not depend on any axioms".

Additional machine-verified theorems in the same file (all axiom-free):

- `omega_upper_both_to_zero : DFUMT8.omega_upper .BOTH = .ZERO`
- `omega_upper_neither_to_zero : DFUMT8.omega_upper .NEITHER = .ZERO`
- `omega_upper_reflective_stable` (4-conjunct: INFINITY, ZERO, FLOWING, SELF all fixed)
- `omega_upper_polar_stable` (2-conjunct: TRUE, FALSE all fixed)
- `omega_upper_fixed_point_count` (8-conjunct: exactly 6 of 8 values are fixed; only BOTH and NEITHER move)

### 3.5 Inclosure Schema (★ design decision — classical-Lean adaptation)

★ Honest design resolution (new in v0.2): the literal `Q (delta Q) ∧ ¬ Q (delta Q)` form is a paraconsistent statement; classical Lean 4 rejects it as `False`. v0.1 flagged this as a "known design issue, flagged for v0.2" — v0.2 resolves it as follows.

We split the schema into two parts:

1. The **classical Lean structure** records closure + transcendence as formal obligations on `Q`, `ψ`, `δ`. No "contradiction theorem" is asserted at the Prop level.
2. The **apparent limit-contradiction** is expressed in the **DFUMT8 TYPE** (as the value `BOTH` at the lower layer), not in Lean's `Prop`. The "apparent contradiction" lives in the eight-valued type system, where `BOTH` is a first-class value distinct from both `TRUE` and `FALSE`.

**v0.2 BUILT** (in `Paper159Inclosure.lean`):

```lean
structure Inclosure (α : Type) where
  Q : α → Prop                          -- the totality Ω as a predicate
  psi : (α → Prop) → Prop               -- "definability" / "set has common nature"
  delta : (α → Prop) → α                -- the diagonalizer
  closure : ∀ X : α → Prop,
    (∀ x, X x → Q x) → psi X → Q (delta X)
  transcendence : ∀ X : α → Prop,
    (∀ x, X x → Q x) → psi X → ¬ X (delta X)
```

No `inclosure_limit_contradiction` theorem (which would imply `False` classically). Instead:

```lean
def Inclosure.limitLowerLayer {α : Type} (_I : Inclosure α) : DFUMT8 := .BOTH

def Inclosure.limitUpperLayer {α : Type} (I : Inclosure α) : DFUMT8 :=
  DFUMT8.omega_upper I.limitLowerLayer
```

This is a **design choice** in the rational-reconstruction stance: we encode Priest-Garfield's BOTH terminus as a DFUMT8 value rather than as a classical Prop, then study its behavior under `omega_upper`. The schema's structural obligations (closure, transcendence) remain in `Prop`; only the **terminus assignment** is in `DFUMT8`. This avoids both (a) needing to make Lean paraconsistent (which would require a custom logic core, far outside scope) and (b) silently letting `False` enter the load-bearing proof chain.

### 3.5.1 Two-layer schema visualization (★ Rei extension of Garfield-Priest 2003 Fig. 1)

The following figure summarizes the two-layer architecture. The lower half reproduces Garfield & Priest (2003, p. 17, Fig. 1) — the original Inclosure visualization — with the δ(Ω) terminus made explicit as a DFUMT₈ value `BOTH`. The upper half is the Paper 159 differential: an idempotent operator `omega_upper` contracts `BOTH` (Priest-Garfield's dialetheist terminus) and `NEITHER` (catuṣkoṭi's 4th corner) to the ZCSG genesis layer `0₀ = ZERO` (Paper 61). The right-side panel enumerates `omega_upper`'s action on all eight D-FUMT₈ values: six of eight are fixed; only `BOTH` and `NEITHER` move.

![Two-Layer D-FUMT₈ Reconstruction of the Inclosure Schema](figures/paper-159-fig1-two-layer-inclosure.svg)

*Fig. 1 (Rei extension). Lower layer reproduces Garfield & Priest (2003) Fig. 1 verbatim with the BOTH terminus made explicit as a DFUMT₈ value. Upper layer is the Paper 159 differential: idempotent operator `Ω_upper` contracts BOTH and NEITHER to the ZCSG genesis layer `0₀ = ZERO` (Paper 61). All connecting structure (Ω_upper definition, idempotency proof, contraction theorems) is machine-verified in Lean 4 zero-axiom-dependence — see §3.4 + Appendix B. SVG source: `papers/figures/paper-159-fig1-two-layer-inclosure.svg`. The lower-layer visualization style and labels (Ω, ψ, δ, φ) follow Priest-Garfield 2003; we claim no novelty for that portion. The upper-layer addition (red dashed arrow, Ω_upper box, 0₀ disc, D-FUMT₈ legend panel) is Rei-AIOS.*

### 3.5.2 Rei-native original mandala (★ NOT derived from Priest-Garfield Fig. 1)

The following figure is a **separate, original composition** in Rei's own ZCSG-native visual vocabulary. It is not a derivative of Garfield-Priest 2003 Fig. 1 (that role belongs to Fig. 1 above). Fig. 2 expresses the same two-layer structure from Rei's own perspective: a mandala with `0₀` at the center, the eight D-FUMT₈ values as petals, the Ω_upper contractions of BOTH and NEITHER shown as red dashed inward arrows, the six fixed values shown with faint self-loops, and an outer dashed cycle suggesting the recursive flow `空 → 自己参照 → 創発 → 再帰的自己変容` (★ explicitly labeled as **PAPER 2 TERRITORY** — not formalized in v0.2). The right-side panels record the three interpretive-stance compatibility (P-G dialetheist / Tillemans weak / non-dialetheist), an honest status legend distinguishing verified / proposed / Paper 2 territory, the complete Ω_upper action table, and a hint pointing to the separate sages-axes lens (STEP 1188) where the same eight-axis structure is read across multiple civilizational traditions.

![Rei Mandala: D-FUMT₈ Two-Layer with 0₀ Genesis Axis](figures/paper-159-fig2-rei-mandala.svg)

*Fig. 2 (Rei original mandala). Central `0₀` medallion = ZCSG central axis (Paper 61). Eight petals = D-FUMT₈ values (Paper 77 substrate). Red dashed Ω_upper arrows (★ Paper 159 §3.4 differential): BOTH (Priest-Garfield terminus) and NEITHER (catuṣkoṭi 4th corner) contract to `0₀`; six of eight values fixed (faint self-loops on TRUE / FALSE / INFINITY / FLOWING / ZERO / SELF — SELF marked with the ⟲ idempotency symbol). Outer dashed ring = chat-Claude / ChatGPT cycle `空 → 自己参照 → 創発 → 再帰的自己変容` (★ PAPER 2 TERRITORY, proposed, NOT formalized in v0.2). Right panels: three interpretive stances on one substrate / honest status legend / Ω_upper full action / sages-axes lens hint. SVG source: `papers/figures/paper-159-fig2-rei-mandala.svg`. Both Fig. 1 (derivative of Priest-Garfield) and Fig. 2 (Rei-native original) express the same two-layer architecture: Fig. 1 in Priest-Garfield's set-theoretic idiom, Fig. 2 in Rei's mandala / ZCSG idiom. The honest pairing makes the derivative vs original split explicit.*

### 3.6 Nāgārjuna's Paradox as an Inclosure instance (lower-layer assignment: BOTH)

Following Garfield & Priest (2003: 17-18), we instantiate the Inclosure schema with φ(χ) = "χ is empty", ψ(X) = "X is a set of things with some common nature", δ(X) = "the nature of things in X". The lower-layer assignment of δ(Ω) is BOTH (the value Priest-Garfield assign): emptiness has the nature of being empty, but as it is itself empty, it has no nature.

### 3.7 Expressibility Paradox as an Inclosure instance (lower-layer assignment: BOTH)

Following Garfield & Priest (2003: 17): φ(χ) = "χ is an ultimate truth", ψ(X) = "X is definable", δ(X) = the sentence asserting that X has no member. The lower-layer assignment of δ(Ω) is again BOTH.

### 3.8 Upper-layer Ω contracts BOTH to ZERO — main differential (★ machine-verified)

The main formal differential of Paper 159 against Priest-Garfield's BOTH-only terminus: the upper layer `omega_upper` contracts the lower-layer BOTH to ZERO, the ZCSG 0₀ genesis layer (Paper 61). This holds for ANY Inclosure instance whose lower-layer assignment is BOTH — including Nāgārjuna's Paradox (§3.6) and the Expressibility Paradox (§3.7).

**v0.2 BUILT** (in `Paper159Inclosure.lean`):

```lean
theorem Inclosure.limit_upper_is_zero {α : Type} (I : Inclosure α) :
    I.limitUpperLayer = .ZERO := by
  unfold Inclosure.limitUpperLayer Inclosure.limitLowerLayer
  decide

theorem Inclosure.limit_upper_idempotent {α : Type} (I : Inclosure α) :
    DFUMT8.omega_upper I.limitUpperLayer = I.limitUpperLayer := by
  rw [Inclosure.limit_upper_is_zero]
  decide

-- Direct sanity-check examples
example : DFUMT8.omega_upper .BOTH = .ZERO := by decide
example : DFUMT8.omega_upper (DFUMT8.omega_upper .BOTH) = .ZERO := by decide
example : DFUMT8.omega_upper .NEITHER = .ZERO := by decide
example : DFUMT8.omega_upper (DFUMT8.omega_upper .NEITHER) = .ZERO := by decide
```

`#print axioms Inclosure.limit_upper_is_zero` → "does not depend on any axioms".

The full semantic characterization of the genesis layer ZERO (= ZCSG `0`) — including its relation to the empty set's reduced homology H̃₋₁(∅) = ℤ from Paper 61 §2.2, the "self-reference → recursive self-transformation" question raised in the chat-Claude / ChatGPT design discussion (Acknowledgments), and the question "why does a system without self generate self" (the natural next problem after the present formal substrate) — is deferred to a planned Paper 2 on the ontology of 0₀.

### 3.9 Compatibility with multiple interpretive stances (★ new in v0.2)

The formal substrate built in §3.4–3.8 is compatible with at least three readings of catuṣkoṭi. The Lean 4 file includes `example` statements for each:

```lean
-- (P-G) Dialetheist reading (Priest-Garfield 2003 / Deguchi-Garfield-Priest):
--       use only the LOWER layer; δ(Ω) = BOTH is endorsed.
example : (.BOTH : DFUMT8) = .BOTH := rfl

-- (T) Weak dialetheist (Tillemans 2009, 2024):
--     use the upper layer to contract BOTH to ZERO, so conjoined contradictions
--     φ ∧ ¬φ are NOT endorsed at the modal layer.
example : DFUMT8.omega_upper .BOTH = .ZERO := by decide

-- (¬D) Non-dialetheist:
--      assign δ(Ω) = NEITHER at the lower layer instead of BOTH. The upper
--      layer still contracts NEITHER to ZERO.
example : DFUMT8.omega_upper .NEITHER = .ZERO := by decide
```

None of these readings is endorsed by the present paper; the substrate accepts each by varying the lower-layer assignment function. This is the formal expression of the non-commitment stance declared in §4.3.

---

## 4. Honest Framing and Scope (★ load-bearing)

### 4.1 Rational reconstruction stance

Garfield and Priest (2003: 2) explicitly disavow textual-history claims:

> "Finally, we do not claim that Nāgārjuna himself had explicit views about logic, or about the limits of thought. We do, however, think that if he did, he had the views we are about to sketch. **This is, hence, not textual history but rational reconstruction.**"

We adopt the same stance. The Lean 4 formalization proposed here is a formal substrate compatible with Priest-Garfield's reading; it does not assert that Nāgārjuna held the corresponding views.

### 4.2 Scholarly debate disclosure

The Priest-Garfield dialetheist reading has been debated for two decades. Notably:

- Tillemans (1999): broadly aligned with Priest-Garfield (paraconsistent at the ultimate level, classical at the conventional level — Priest-Garfield 2003 footnote 2 explicitly agree with Tillemans on this point).
- Tillemans (2009, 2014): later argues for a "weak dialetheist" reading of Nāgārjuna, holding that Nāgārjuna does not accept conjoined contradictions φ ∧ ¬φ.
- Deguchi, Garfield & Priest (2008, 2013): defend and elaborate the dialetheist reading.
- Siderits, Ferraro, Tao Jiang: various critical engagements.
- A 2024 paper in *Asian Philosophy* offers an updated critique of Priest-Garfield's use of catuṣkoṭi.

**We have not yet read these critic papers directly.** Citations above are from secondary triangulation (web search snippets and one verified primary source: Garfield & Priest 2003). Their detailed evaluation is deferred to v0.2.

### 4.3 Non-commitment to interpretive position

We take no position in the dialetheist / weak-dialetheist / non-dialetheist debate. The formal substrate proposed here is, by design, compatible with each: Priest-Garfield's reading is preserved as the lower-layer assignment of BOTH; Tillemans's worry that conjoined contradictions are not endorsed can be addressed via the upper-layer contraction (the lower-layer BOTH does not survive Ω application); non-dialetheist readings that reject the BOTH assignment altogether can use the lower layer with only TRUE / FALSE / NEITHER.

### 4.4 No "world first" claim

Two prior uniqueness claims should be acknowledged and respected:

- **Paper 61 ZCSG** (Fujimoto 2026): claims "world-first mathematical encoding of śūnyatā-of-śūnyatā" as the symbol `0`.
- **Priest-Garfield 2003** (p. 18): name the ontological paradox "Nāgārjuna's Paradox" and state that, "to our knowledge, [it] is found nowhere else."

The present paper avoids any "world first" framing. The contribution is the two-layer reconstruction plus Lean 4 substrate, both of which presuppose and extend those prior claims rather than competing with them.

### 4.5 Library-only, Mathlib-free in spirit, `decide`-only (★ stronger than expected)

We inherit Paper 77's design discipline: all proofs by `decide` (no `native_decide` axiom used), propositional fragment on finite inductive types. The host project (`data/lean4-mathlib/`) requires Mathlib, but `Paper159Inclosure.lean` itself does NOT import Mathlib — it is Mathlib-free as a file. Build time: 1.7 seconds via `lake build CollatzRei.Paper159Inclosure`.

★ Result is **stronger than initially planned**: all 9 load-bearing theorems show `#print axioms ... does not depend on any axioms` (no `propext`, no `Classical.choice`, no `Quot.sound`, no `native_decide`). This is the strongest possible axiom-purity result. Paper 158 (Collatz exit layer) by contrast uses `[propext, Classical.choice, Quot.sound]`.

### 4.6 Ω idempotency: honest provenance disclosure (★ updated in v0.2)

The idempotency Ω ∘ Ω = Ω is *stipulated* in the Rei SEED_KERNEL as the axiom `dfumt-idempotency` and is *operationally tested* in STEP 513 (TypeScript). v0.2 of this paper produces the **first Lean 4 machine-verified statement** of this property in the Rei stack — but as a `decide`-only proof of an operationally specific operator `omega_upper`, NOT as a general statement about all idempotent operators on D-FUMT₈. The general claim "any operator Ω with Ω ∘ Ω = Ω is well-behaved" is not formalized; v0.2 verifies one concrete operator instance.

★ As noted in §3.4, STEP 513's `omega` and this paper's `omega_upper` are **distinct operators** sharing the structural idempotency property. v0.1 implied an identity that does not hold. v0.2 corrects this and re-states the relationship as "shared structural pattern, distinct semantics".

### 4.7 Upper-layer scope limitations

The upper-layer operator Ω is defined here only on D-FUMT₈ values. A full semantic theory of the genesis layer ZERO — including its identification with the ZCSG central axis `0`, its connection to the reduced homology of the empty set, and its role as the convergence point of catuṣkoṭi negations — is **explicitly deferred to a planned Paper 2** on the ontology of 0₀.

---

## 5. Discussion

### 5.1 Relation to STEP 476 (TypeScript engine)

STEP 476 `nagarjuna-fde-western-engine.ts` (654 lines) supplies the operational TypeScript proofs of catuṣkoṭi ≅ FDE ≅ DFUMT4 isomorphism: `verifyNOT`, `verifyAND` (16 cases), `verifyOR` (16 cases), `verifyLattice`. The present paper proposes to lift these to Lean 4 zero-sorry theorems via `decide`, thereby promoting machine-checked algebraic proofs of the same content.

### 5.2 Heidegger comparison (after Priest-Garfield 2003: 16)

Priest-Garfield identify Heidegger as the closest Western parallel to Nāgārjuna's ontological insight, while noting that Heidegger does not follow the identification of the two truths (conventional and ultimate). We adopt the same comparison and the same limitation.

### 5.3 Two-layer architecture as the main differential

Priest-Garfield's Inclosure terminates at BOTH — the dialetheist commitment to a true contradiction. Our two-layer reconstruction preserves BOTH at the lower layer (so Priest-Garfield's analysis is embedded as a special case) but adds an upper-layer Ω that contracts BOTH to ZERO. The strategy is analogous to QMRP (Paper 26 candidate) reframing Shannon's bound as a special case at finite N. The differential is twofold:

1. **Structural**: separating object-level truth values from meta-level modal operators, formalizing the "all standpoints exhausted" reading of catuṣpaścāś as an idempotent stable point rather than as a singleton truth value.
2. **Methodological**: machine verification via Lean 4 `decide`, which Priest 1987/2002 and Priest-Garfield 2003 do not pursue.

### 5.4 Connection to ZCSG (Paper 61)

ZCSG (Paper 61) introduces the central symbol `0` for śūnyatā-of-śūnyatā, with dimension 0, distinguishing it from o0 (dimension −1, NEITHER) and 0o (dimension +1, BOTH). The upper-layer Ω of the present paper is structurally a contraction map from dimension +1 to dimension 0, i.e. from 0o to `0`. The two papers thus approach the same structure from complementary angles: ZCSG from notation, the present paper from Lean 4 formalization.

### 5.5 What this paper does NOT claim

- That Nāgārjuna was a dialetheist (or was not).
- A resolution to the Priest-Garfield / Tillemans / Siderits debate.
- Any extension to Mathlib's standard logic.
- Any new philosophical thesis beyond rational-reconstruction formal substrate.
- A tier ranking of philosophers (a methodologically dubious framing we explicitly avoid).
- "Four-step termination" of catuṣkoṭi negations — we adopt the cleaner idempotent stable-point reading consistent with catuṣpaścāś (four corners and one hundred negations) tradition.
- Prior machine verification of Ω idempotency in the Rei stack (it has been axiomatically stipulated and operationally tested only).
- Full semantic theory of the genesis layer ZERO (deferred to a planned Paper 2).

---

## 6. Conclusion

This v0.2 LEAN-4-BUILT paper supplies a two-layer D-FUMT₈ reconstruction of Priest-Garfield's Inclosure Schema for Nāgārjuna's catuṣkoṭi. The lower layer (FDE embedding, BOTH at limit) preserves Priest-Garfield's analysis as a special case. The upper layer (idempotent modal operator `omega_upper` contracting BOTH and NEITHER to ZERO) supplies the differential that Priest-Garfield's BOTH-only terminus does not offer, and connects to the central axis `0` of the ZCSG three-layer notation (Paper 61). All 9 load-bearing theorems are machine-verified in Lean 4 with `decide`-only proofs that depend on **no axioms** (no `propext`, no `Classical.choice`, no `Quot.sound`, no `native_decide`); see Appendix B. The paper is positioned as **rational reconstruction following Priest-Garfield's own self-framing**, compatible with at least three interpretive stances of catuṣkoṭi, and explicitly defers the genesis-layer ontology to a planned Paper 2.

Two honest findings emerged during the v0.1 → v0.2 build:

1. **`omega_upper` is distinct from STEP 513's `omega`** (§3.4). Both are idempotent but contract to different fixed-point sets; v0.1 implied a closer relationship than holds. v0.2 corrects this by renaming the Paper 159 operator and stating the relationship as "shared structural pattern, distinct semantics".
2. **The literal limit-contradiction cannot be formalized in classical Lean** (§3.5). v0.2 resolves this by encoding Priest-Garfield's BOTH terminus as a DFUMT8 VALUE rather than as a classical Prop, leaving Lean's `Prop` consistent.

The Tillemans 2009/2014/2024 + Siderits + Ferraro + Tao Jiang critic papers remain **NOT YET READ** (§4.2 unchanged). Their direct evaluation is deferred to v0.3.

---

## Acknowledgments

The two-layer architecture in §3.4 / §3.8 was articulated in a chat session with Claude (Anthropic) on 2026-05-31, building on the design discussion in an earlier session on the same date concerning the BOTH-vs-NEITHER reading of catuṣkoṭi negation. The articulation of the lower-layer = Priest, upper-layer = Ω with idempotency design was that session's central contribution.

The full text of Garfield & Priest (2003) was accessed via the open Smith ScholarWorks repository on 2026-05-31; this paper would have been substantially weaker without that primary source. We thank both authors for making the original work openly available.

---

## References

1. Anderson, A. R., & Belnap, N. D. (1975). *Entailment: The Logic of Relevance and Necessity, Vol. I*. Princeton University Press.
2. Belnap, N. D. (1977). A useful four-valued logic. In J. M. Dunn & G. Epstein (Eds.), *Modern Uses of Multiple-Valued Logic* (pp. 5–37). Reidel.
3. Deguchi, Y., Garfield, J. L., & Priest, G. (2008). The way of the dialetheist: Contradictions in Buddhism. *Philosophy East and West*, 58(3), 395–402.
4. Deguchi, Y., Garfield, J. L., & Priest, G. (2013). How we think Mādhyamikas think: A response to Tom Tillemans. *Philosophy East and West*, 63(3), 426–435.
5. Fujimoto, N. (2026). *Zero-Centered Symbol Grammar (ZCSG): A World-First Dimensional Encoding of Nāgārjuna's Śūnyatā-of-Śūnyatā*. **Paper 61**, [DOI 10.5281/zenodo.15217458](https://doi.org/10.5281/zenodo.15217458).
6. Fujimoto, N. (2026). *LeanDFumt: An Open-Source Eight-Valued Logic Library for Lean 4*. **Paper 77**. GitHub: <https://github.com/fc0web/lean-d-fumt8>.
7. Garfield, J. L. (Trans.) (1995). *The Fundamental Wisdom of the Middle Way: Nāgārjuna's Mūlamadhyamakakārikā*. Oxford University Press.
8. Garfield, J. L., & Priest, G. (2003). Nāgārjuna and the limits of thought. *Philosophy East and West*, 53(1), 1–21. [JSTOR 1400052](https://www.jstor.org/stable/1400052). Open-access copy: Smith ScholarWorks.
9. Hayes, R. (1994). Nāgārjuna's appeal. *Journal of Indian Philosophy*, 22, 299–378.
10. Nāgārjuna (c. 150 CE). *Mūlamadhyamakakārikā*. (See Garfield 1995 for English translation.)
11. Priest, G. (1987). *In Contradiction: A Study of the Transconsistent*. Martinus Nijhoff (2nd ed. Oxford, 2006).
12. Priest, G. (2002). *Beyond the Limits of Thought* (2nd ed.). Oxford University Press.
13. Siderits, M. (1989). Thinking on empty: Madhyamika anti-realism and canons of rationality. In S. Biderman & B.-A. Scharfstein (Eds.), *Rationality in Question: On Eastern and Western Views of Rationality*. Brill.
14. Tillemans, T. J. F. (1999). Is Nāgārjuna's logic deviant or non-classical? In *Scripture, Logic, Language: Essays on Dharmakīrti and His Tibetan Successors*. Wisdom Publications.
15. The Lean 4 development team (2024–2026). *Lean 4 v4.27.0*. <https://lean-lang.org>.

Additional critic papers (Tillemans 2009 / 2014 / 2024, Siderits later work, Ferraro 2013, Tao Jiang) acknowledged in §4.2 but not yet directly consulted; full citation list will be completed in v0.2.

---

## Version history

- **v0.1** (2026-05-31, morning): Initial OUTLINE + SKELETON release. Lean 4 code PROPOSED, not yet built. Honest scope as stated. Released for archival continuity. Zenodo DOI [10.5281/zenodo.20468146](https://doi.org/10.5281/zenodo.20468146), 11/11 platform broadcast (commit `4f5c5002`).
- **v0.2** (2026-05-31, afternoon): **Lean 4 BUILT** — 9 load-bearing theorems machine-verified with zero axiom dependence (stronger than expected; even purer than Paper 158). Two honest findings: (a) `omega_upper` rename + STEP 513 semantic divergence note (§3.4); (b) classical-Lean limit-contradiction adaptation via DFUMT8-VALUE encoding (§3.5). FDE negation preservation machine-verified; conjunction/disjunction/lattice preservation deferred to v0.3 per honest scope (multiple lattice conventions in literature). Critic papers (Tillemans 2009+, Siderits, Ferraro, Tao Jiang) STILL NOT YET READ — unchanged from v0.1. **Two figures added** (both SVG): Fig. 1 (Rei extension of Garfield-Priest 2003 Fig. 1) in §3.5.1, and Fig. 2 (Rei-native original mandala, NOT derived from Priest-Garfield) in §3.5.2. Sources: `papers/figures/paper-159-fig1-two-layer-inclosure.svg` and `papers/figures/paper-159-fig2-rei-mandala.svg`. Pinned commit (pre-v0.2): `4f5c50020f8ba0e589f8d6ebe278195ee812a1b7`.
- **v0.3** (planned, no timeline commitment): Direct reading of Tillemans 2009/2014/2024 + Siderits + Ferraro critic papers. FDE and/or/lattice preservation theorems (after editorial choice of which lattice convention to align). Possible Paper 2 design seed (genesis-layer ZERO ontology) extracted into separate document.

---

## Appendix B — Lean 4 axiom audit (v0.2 BUILT verification)

**Source file**: `data/lean4-mathlib/CollatzRei/Paper159Inclosure.lean` (~200 lines)
**Audit file**: `data/lean4-mathlib/CollatzRei/Paper159InclosurePrintAxioms.lean` (`#print axioms` for each load-bearing theorem)
**Lean version**: v4.27.0 (host project uses Mathlib v4.27.0; this file imports no Mathlib)
**Build command**: `lake build CollatzRei.Paper159Inclosure` (1.7s, exit 0)
**Pinned commit (pre-v0.2)**: `4f5c50020f8ba0e589f8d6ebe278195ee812a1b7`

### `#print axioms` output (exact transcript)

```
'CollatzRei.Paper159Inclosure.omega_upper_idempotent' does not depend on any axioms
'CollatzRei.Paper159Inclosure.omega_upper_both_to_zero' does not depend on any axioms
'CollatzRei.Paper159Inclosure.omega_upper_neither_to_zero' does not depend on any axioms
'CollatzRei.Paper159Inclosure.omega_upper_reflective_stable' does not depend on any axioms
'CollatzRei.Paper159Inclosure.omega_upper_polar_stable' does not depend on any axioms
'CollatzRei.Paper159Inclosure.omega_upper_fixed_point_count' does not depend on any axioms
'CollatzRei.Paper159Inclosure.FDE.toDFumt8_neg' does not depend on any axioms
'CollatzRei.Paper159Inclosure.Inclosure.limit_upper_is_zero' does not depend on any axioms
'CollatzRei.Paper159Inclosure.Inclosure.limit_upper_idempotent' does not depend on any axioms
```

### Interpretation

"Does not depend on any axioms" means the proof requires **none** of:
- `sorryAx` (would indicate an unfinished proof)
- `propext` (propositional extensionality)
- `Classical.choice` (the axiom of choice)
- `Quot.sound` (quotient soundness)
- `native_decide` (computed kernel reduction)
- any user-introduced axiom

The proofs go through purely by `decide` on finite inductive types. This is the strongest possible axiom-purity result in Lean 4.

### Comparison with Paper 158

Paper 158 (Collatz exit layer) achieved zero-sorry but the load-bearing theorems show `[propext, Classical.choice, Quot.sound]`. Paper 159 v0.2 is **strictly purer**: no Classical reasoning is even invoked. This is a consequence of working entirely on a finite inductive type (DFUMT8 with 8 constructors) rather than on ℕ with division and induction.

### Sample reproduction commands (cygwin/PowerShell)

```sh
cd C:/Users/user/rei-aios/data/lean4-mathlib
lake build CollatzRei.Paper159Inclosure              # produces .olean
lake env lean CollatzRei/Paper159InclosurePrintAxioms.lean  # prints audit
```

---

*Rei-AIOS Project. Peace Axiom #196: immutable = true. License: CC-BY 4.0 (text), Apache-2.0 (any associated Lean 4 code).*
