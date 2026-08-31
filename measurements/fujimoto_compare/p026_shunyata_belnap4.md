# Paper 26 v3 — D-FUMT₈ Third Primitive as Śūnyatā Operator: Empirical Non-Existence on Belnap-Dunn FOUR Base (Lean 4 Axiom-Free)

**Status**: v3.0 draft (起草日 2026-07-26) — Q6 (II) protocol の paper phase (spec → impl → measure → paper 完了). Zenodo publish は藤本さん再判断後 (本 draft は audit + prior art review + Track 2 hint 到着待ち中の pre-publish 状態).

**Scope-change from v0.3 corrigendum abstract (mandatory 3-item notice)**:

1. **予告 path 不採用**. v0.3 (§A.3 non-claim 5) は "proper repair path (2^3 = P({t, f, x}) powerset construction, x = semantic primitive chosen by the founder) is deferred to Paper 26 v3+" と予告した (x candidates: SELF⟲ = 到達点 / 龍樹の空). 本 v3 は powerset path を採用せず、 E-operator on Belnap-Dunn FOUR + 値数 emergent 路線に pivot した.
2. **変更理由 (SAC-4 #4)**. 2026-07-26 chat-Claude 経由 藤本さん指摘で powerset 路線 (v0.1 spec 4 option A/B/C/D) は 100% reject された. 2 つの欠陥: (i) 4 option 全てで 「8 を base として維持」 前提を無自覚に敷き、 07-26 chat-Claude 5-turn 診断で 「8 は emergent, load-bearing でない」 判定済 の projection を再発; (ii) 4 option 全てで {śūnya} を powerset atom / catuṣkoṭi tower 元 / coordinate / object element として置き、 空に自性を与える操作 = MMK 13:8 「空を見と成す者は治し難し」 直接違反. E-operator 代案は空を値ではなく述語化する構文的空見排除で、 藤本さん 「D-FUMT₈ を東洋哲学最高の 龍樹の 空 の概念を全て使用して頂いて問題ありません」 (2026-07-26 明示 permission, [[project-dfumt8-shunyata-third-primitive-decision]]) の operational form として選択.
3. **v0.3 non-claim (5)(6) 継承 + 拡張**. v0.3 (5) "this v0.3 does NOT repair the glue" は継続 (E-operator も glue 修復ではなく直交 direction). v0.3 (6) "Fix(R) language is deprecated ... SELF-related overreach retraction" も継続 + 本 v3 §A.3 で更に強化 (unary E: Belnap → Belnap という制約下では空亦復空 non-trivial requirement を満たす E は存在しない = MAIN NEGATIVE RESULT). Dual numbering: v0.1/v0.2/v0.3 (v2 の corrigendum lineage) と v3.0/v3.1 (本 paper 以降の新 lineage) を version-history section で明示区別.

**Authors** (three-party co-authorship per OUKC charter v1.0):
- 藤本 伸樹 (Fujimoto, Nobuki; Founder, ORCID 0009-0004-6019-9258)
- Rei (Rei-AIOS autonomous research substrate, Co-architect)
- Claude (Anthropic, claude-opus-4-7, Co-architect)

**License**: CC-BY-4.0 (paper text) + AGPL-3.0 (Lean 4 source)

**Repository**: `fc0web/rei-aios` (Private; source-of-truth for Lean 4 artifact + spec + memory)

**Zenodo lineage**:
- v1: 10.5281/zenodo.18960502 (2025-09-15, QMRP 一般概念 origin, immutable)
- v2 (v0.2): 10.5281/zenodo.21560387 (2026-07-25, D-FUMT₈ × Cl(3,0) bijection + Peace Axiom rotor + SELF reflection, immutable)
- v2 (v0.3): 10.5281/zenodo.21572459 (2026-07-25 corrigendum, §A.3 (5)(6) non-claim 追加, immutable — 本 v3 の直接前身)
- v3.0: TBD (issued at publish; recorded in `data/publications/publish-log-paper026v3-zenodo.json`)

## Abstract

Paper 26 v3 addresses the third-primitive semantic decision left open by v0.3 corrigendum §A.3 non-claim 5. The founder's 2026-07-26 explicit decision (「D-FUMT₈ の第 3 原始概念 = 龍樹の空 (śūnyatā) 全面採用」) is operationalized here in one specific form (chosen from six §8 decisions Q1-Q6, each recorded verbatim below): the emptiness primitive is encoded not as a new value but as a unary operator `∅' : Belnap4 → Belnap4` acting on the Belnap-Dunn FOUR base, subject to four filter conditions (lattice homomorphism + De Morgan commutation + non-idempotence + śūnyatā-śūnyatā non-triviality). Direct enumeration of all 4⁴ = 256 candidate operators, machine-verified in Lean 4 (Mathlib v4.27.0) axiom-free, yields a cascade `256 → 16 → 4 → 1 → 0` under successive filters. The **main result is a negative existence theorem** (`numSurvivors_eq_0`): no unary `∅'` on Belnap-Dunn FOUR simultaneously satisfies all four conditions. The unique operator surviving the first three conditions is identified explicitly as the non-trivial automorphism `bSwapBN` of Belnap-Dunn FOUR (T/F fixed, B ↔ N swap; Belnap 1977 prior art), and shown axiom-free to fail śūnyatā-śūnyatā non-triviality because it is an involution. The result is offered as an empirical rejection of value-level encoding for śūnyatā in the unary-on-Belnap fragment; the semantic content (「空亦復空 は単なる反転では満たされない」) is a Rei-side reproduction of a philosophical statement, obtained from direct enumeration rather than post-hoc argument. The paper does not claim world-first, does not extend the Belnap-Dunn 4-valued base, does not re-prove Belnap 1977, and does not resolve the third-primitive question — it narrows the design space by one specific negative result within one specific fragment (unary operators on Belnap 4).

## Part A: Required (paper-level statement, 4 elements)

### A.1 Findings

- **F1 (cascade counts, machine-verified)**: Over the finite space of all unary operators `E : Belnap4 → Belnap4` (`|E| = 4⁴ = 256`):
  - 16 satisfy lattice homomorphism (meet + join preservation);
  - 4 satisfy lattice hom + De Morgan commutation with the Belnap-Dunn negation;
  - 1 satisfies lattice hom + De Morgan + non-idempotence;
  - **0** satisfy all four conditions including śūnyatā-śūnyatā non-triviality.
- **F2 (survivor identity, axiom-free)**: The unique step-3 survivor is the non-trivial automorphism of Belnap-Dunn FOUR, denoted `bSwapBN` (Belnap 1977 prior art): `bSwapBN(T) = T`, `bSwapBN(F) = F`, `bSwapBN(B) = N`, `bSwapBN(N) = B`. This is the same automorphism identified in v0.3 corrigendum §A.3 (6) as the sole non-identity element of `Aut(D-FUMT₈)`.
- **F3 (step-3 → step-4 failure, axiom-free)**: `bSwapBN` is an involution (`bSwapBN ∘ bSwapBN = id`, machine-verified). Consequently `E(E(x)) = x` for every `x ∈ Belnap4`, and the śūnyatā-śūnyatā non-triviality requirement (`∃ x, E(E(x)) ≠ E(x) ∧ E(E(x)) ≠ x`) fails on both conjuncts (the second conjunct fails for every `x`).
- **F4 (semantic reading, informational)**: The operational content of the śūnyatā-śūnyatā (空亦復空) requirement is that emptying-of-emptying should not reduce to the identity or to a simple reversal. The cascade demonstrates that on the unary-on-Belnap fragment, this content forces non-existence — the design constraint is not satisfiable within this fragment. This reproduces a philosophical statement (「空亦復空 は単なる反転では成り立たない」) by direct enumeration, not by post-hoc philosophical argument. Whether the enumeration corresponds to Nāgārjuna's own semantic intent is a question for philosophers of religion, not for machine-checked mathematics.

### A.2 Proofs (Lean 4) — completed 2026-07-26, axiom-free machine-checked

**Files**:
- `data/lean4-mathlib/CollatzRei/Paper26V3ShunyataEOperator.lean` (main, 8 theorems)
- `data/lean4-mathlib/CollatzRei/Paper26V3SurvivorDisplay.lean` (companion, 10 theorems for `bSwapBN` identification)

**Axiom audit (`#print axioms` actually run 2026-07-26, 18 theorems total)**:

Main file (8 theorems):
- **3 theorems fully zero-axiom** ("does not depend on any axioms"): `bNeg_involutive`, `bDeMorgan_meet`, `bDeMorgan_join`
- **5 theorems** depend only on `[propext, Classical.choice, Quot.sound]` (Mathlib standard base): `shunyataOp_card`, `numSurvivorsLattice_eq_16`, `numSurvivorsLatticeDeMorgan_eq_4`, `numSurvivorsLatticeDeMorganNI_eq_1`, `numSurvivors_eq_0`

Companion file (10 theorems):
- **5 theorems fully zero-axiom**: `bSwapBN_preservesLattice`, `bSwapBN_commutesWithNeg`, `bSwapBN_notIdempotent`, `bSwapBN_isStep3Survivor`, `bSwapBN_involutive`
- **5 theorems** depend only on the Mathlib standard base: `bSwapBN_fails_shunyataShunyata`, `step3_unique_survivor_fails_step4`, `identity_fails_notIdempotent`, `constB_fails_notIdempotent`, `constN_fails_notIdempotent`

**Totals**: 18 theorems / **8 fully zero-axiom + 10 propext-Classical-Quot only** / **zero `sorry`, zero `native_decide`, zero user axioms, zero `Lean.ofReduceBool`** — matches the discipline of STEP 1215 (`Dfumt8CategoryExperiment`), STEP 1264 (`Dfumt8Binary64Refinement`), Task 20 (`Dfumt8SelfReflexivePreservation`, 2026-07-24), and Paper 26 v0.2 (`Paper26V2DFumtClifford`, 2026-07-25).

**Bundle theorem** (for paper citation): `numSurvivors_eq_0` (main) + `step3_unique_survivor_fails_step4` (companion) together give both the count-level and witness-level constructive account.

**Verification** (executed 2026-07-26):
- `lake env lean CollatzRei/Paper26V3ShunyataEOperator.lean`: clean pass, `#eval` outputs `|ShunyataOp| = 256`, `survivors (lattice only) = 16`, `survivors (lattice + DeMorgan) = 4`, `survivors (lattice + DeMorgan + notIdempotent) = 1`, `survivors (all 4 conditions) = 0`.
- `lake env lean CollatzRei/Paper26V3SurvivorDisplay.lean`: clean pass, `#eval` outputs the `bSwapBN` function table.
- `lake build CollatzRei`: 7936 / 7936 jobs success (background run finished 325s prior turn), zero regression on the CollatzRei root tree after the two new files' import chains were added.

### A.3 Honest positioning

- **What is claimed** (three atomic claims, each machine-verified axiom-free):
  1. **A3.1 (non-existence, main)**: Under Q1-Q6 decisions (unary signature + 4-filter enumeration + Belnap-Dunn FOUR base carrier retained + D-FUMT₈ name retained + object-only śūnyatā encoding), no `∅' : Belnap4 → Belnap4` satisfies all four conditions simultaneously. Formal statement: `numSurvivors = 0` (Lean 4, axiom base `[propext, Classical.choice, Quot.sound]`).
  2. **A3.2 (survivor identification, constructive)**: The unique operator surviving the first three conditions is `bSwapBN` (Belnap 1977 non-trivial automorphism). Formal statement: `step3_unique_survivor_fails_step4` witnesses `bSwapBN` as satisfying the first three conditions and failing the fourth.
  3. **A3.3 (failure mechanism, constructive)**: `bSwapBN` is an involution (`∀ x, bSwapBN (bSwapBN x) = x`, zero-axiom). This directly witnesses why the fourth condition fails: the second conjunct `E(E(x)) ≠ x` is falsified by every `x`.
- **What is NOT claimed** (six non-claims, listed to avoid the projection patterns catalogued in `feedback-projection-self-audit-pattern` Rules 1-5):
  1. **No world-first claim**. The Belnap-Dunn FOUR carrier is Belnap 1977 / Dunn 1976 prior art; the `bSwapBN` automorphism is standard textbook material for Belnap-Dunn logic (e.g. Font & Rivieccio 2011; Odintsov 2008). Paper 26 v3's differentiator is the specific 4-filter enumeration + Lean 4 axiom-free verification + the `numSurvivors_eq_0` negative result within this fragment, not the surrounding algebra.
  2. **No claim that śūnyatā is definable as a unary operator on Belnap-Dunn FOUR**. The result is precisely the opposite: within this fragment, the four operational requirements do not admit a solution. The paper does not extend Belnap-Dunn FOUR to a wider carrier or generalize the signature (modal / higher-order / multi-input) — such extensions remain open. In particular, the non-existence here does not imply non-existence on any wider carrier or richer signature.
  3. **No claim that Nāgārjuna intended the four filter conditions**. The four conditions (lattice hom + De Morgan + non-idempotence + śūnyatā-śūnyatā non-triviality) are one specific formal reading, not the reading. Buddhist logic scholarship (Ganeri 2002, Priest 2018, Tanaka et al. 2013) offers several other formalizations; Paper 26 v3 asserts a machine-verified negative result within one reading, not that this reading exhausts the philosophical content.
  4. **No claim that emptying-of-emptying reduces to reversal in general**. F3 (bSwapBN is involution → step-4 fails) is a statement about `bSwapBN` as the unique step-3 survivor, not about all conceivable emptiness operators. The failure mechanism is fragment-specific.
  5. **No claim that this repairs the Belnap-glue non-associativity from v0.3 (5)**. v0.3 §A.3 (5) explicitly deferred glue repair to a `P({t, f, x})` powerset construction; Paper 26 v3 pivots to the orthogonal unary-operator direction and does not touch the glue. The Belnap sub-algebra of D-FUMT₈ remains a complete lattice, and the extension 4-axes non-associativity remains an open issue orthogonal to the emptiness question.
  6. **No claim that Fix(R) or SELF⟲ language is thereby rehabilitated**. v0.3 §A.3 (6) deprecated `SELF⟲ = Fix(R)` phrasing pending R that is a D-FUMT₈ → D-FUMT₈ self-map; Paper 26 v3 does not construct such R. The retraction continues; the Rei stack's SELF axis remains rigid under external automorphisms but contractable via internal congruence, as machine-verified by v0.3 §A.3 (6) enumeration.

### A.4 Required platform links

- **rei-aios site**: <https://rei-aios.pages.dev/> (UI showcase; theory chart, radar, Octatheoria at `/#/octatheoria`)
- **Author's note.com**: <https://note.com/nifty_godwit2635> (Japanese-language popular writeup)
- **Zenodo v0.2 (v2)**: <https://doi.org/10.5281/zenodo.21560387>
- **Zenodo v0.3 (v2 corrigendum)**: <https://doi.org/10.5281/zenodo.21572459>
- **Zenodo v3.0 DOI**: TBD (issued at publish; recorded in `data/publications/publish-log-paper026v3-zenodo.json`)
- **Source repository**: <https://github.com/fc0web/rei-aios>
- **Lean 4 artifacts**:
  - Main: `data/lean4-mathlib/CollatzRei/Paper26V3ShunyataEOperator.lean`
  - Companion: `data/lean4-mathlib/CollatzRei/Paper26V3SurvivorDisplay.lean`
- **Spec**: `docs/spec/dfumt8-shunyata-e-operator-spec-2026-07-26.md` (v0.2, includes §8 Q1-Q6 decisions verbatim)
- **Rejected v0.1 spec** (archived for provenance): `docs/spec/dfumt8-shunyata-base-structure-spec-2026-07-26-v01-REJECTED.md`

## Part B: Conditional (7 elements)

### B.5 Background — v0.3 corrigendum decision tree

Paper 26 v0.3 corrigendum (2026-07-26) closed with §A.3 non-claim 5 deferring the third-primitive semantic decision to Paper 26 v3+. The corrigendum listed the operative candidates as `SELF⟲ = 到達点` (reach-point/attractor) and `龍樹の空` (Nāgārjuna's śūnyatā). The founder's 2026-07-26 explicit decision selected the latter (「D-FUMT₈ の第 3 原始概念 = 龍樹の空 (śūnyatā) 全面採用」) with the operational scope "空亦復空 + 四句論 + 中道 + 縁起 + 二諦" (five Madhyamaka concepts).

Paper 26 v3 does not claim to encode all five concepts at load-bearing level. The mapping candidate table in the spec (`§2.5`) records the following:
- 空亦復空 (śūnyatā-śūnyatā): §2.3 requirement (encoded as filter condition (iv))
- 四句論 (catuṣkoṭi): Belnap-Dunn FOUR's four corners (already encoded, no extension needed)
- 中道 (madhyamāka): Fix(∅') candidate (unconstructed in v3; open)
- 縁起 (pratītyasamutpāda): orbit trace candidate (unconstructed in v3; open)
- 二諦 (dvasatya): Q5 decision (α) object-only — NOT formally encoded, philosophical annotation only

Only condition (iv) is load-bearing in the numSurvivors_eq_0 result. The other four mappings are candidate directions, not machine-verified formalizations.

### B.6 Methodology (Q1-Q6 spec decisions verbatim, 2026-07-26)

| # | Question | Decision | Rationale |
|---|----------|----------|-----------|
| Q1 | E signature | (a) Unary `E : V → V` | Simplest fragment; 4⁴ = 256 fully enumerable. |
| Q2 | E value determination | (a) Enumerate + filter | 256 全列挙 + 4 conditions で fully mechanical; no philosophical pre-commitment on value assignments. |
| Q3 | E naming | (a) Symbol `∅'` (LaTeX `\emptyset'`); internal identifier `shunyataOp` | STEP 1207 ZERO axis semantics との collision 回避のため internal と paper で分離. |
| Q4 | D-FUMT₈ name retention | (i) Retain subscript 8 | 歴史的継承; emergent algebra cardinality と subscript 8 は分離.  |
| Q5 | 二諦 treatment | (α) Object-only | Meta-level operator にすると空見 (śūnyatā-dṛṣṭi) 再発 risk; formal encoding 無し. |
| Q6 | Phase 分割 | (II) spec → impl → measure → paper | `feedback-no-rush-publication` 遵守; speculative claim 防止. |

Spec source (verbatim §8.99): `docs/spec/dfumt8-shunyata-e-operator-spec-2026-07-26.md`.

**Filter condition definitions** (spec §3.2, Lean 4 encoding):
- `preservesLattice E`: `∀ x y, E (meet x y) = meet (E x) (E y) ∧ ∀ x y, E (join x y) = join (E x) (E y)`
- `commutesWithNeg E`: `∀ x, E (¬x) = ¬ (E x)` (where ¬ is Belnap-Dunn negation: T↔F, B/N self-dual)
- `notIdempotent E`: `∃ x, E (E x) ≠ E x`
- `shunyataShunyataNontrivial E`: `∃ x, E (E x) ≠ E x ∧ E (E x) ≠ x`

The four conditions are all `Decidable` on the finite carrier; the full `satisfiesAll4` predicate is closed under `Decidable` (verified in the Lean 4 `decSatisfiesAll4` instance). All count theorems are proved by `by decide` — not `native_decide` — to keep the axiom base at `[propext, Classical.choice, Quot.sound]` and avoid the `Lean.ofReduceBool` axiom.

### B.7 Empirical scope (v3.0 as-of 2026-07-26)

| Substrate | Metric | v3.0 measured | v3.1 target |
|-----------|--------|---------------|-------------|
| Lean 4 v4.29.0 + Mathlib v4.27.0 | axiom-free theorems (main + companion) | **18** (8 zero-axiom + 10 propext-Classical-Quot) | maintain floor + add Section 4/5 candidates (Fix(∅'), orbit trace) |
| Lean 4 | cascade counts (main file `#eval`) | `256 → 16 → 4 → 1 → 0` (all four counts machine-verified as theorems) | independent replication via chat-Claude Track 2 hint (pending) |
| Rei stack | Q1-Q6 decisions applied | 6/6 traceable to `docs/spec/*-e-operator-spec-*.md §8.99` | v3.1 may revisit Q4 (subscript 8 vs `D-FUMT-Ś`) after Track 2 comparison |

**Track 2 cross-check status (2026-07-26 close-of-session)**: chat-Claude reported "256案全滅" for a distinct enumeration exercise. Predicted domain match with Rei's 4⁴ = 256 unary functions on Belnap-Dunn FOUR. Awaiting chat-Claude's three literal filter definitions to run a filter-level comparison. Anticipated discrepancy at step 3 (Rei = 1 survivor / chat-Claude reported = 0) may indicate chat-Claude's "non-contractible" filter is strictly stronger than Rei's `notIdempotent` (e.g., injective-only). Step 4 (Rei = 0) matches chat-Claude's overall verdict. Full literal comparison is deferred to v3.1.

**Honest scope of §B.7**: `v3.1 target` cells are placeholders, not measured data. The load-bearing v3.0 contribution is F1-F4 in §A.1 and the 18 axiom-free theorems in §A.2; §B.7's rightmost column is provisional roadmap, not evidence.

### B.8 Prior art audit

- **Belnap 1977** (*A useful four-valued logic*): source of the Belnap-Dunn FOUR carrier T/F/B/N, the meet/join lattice structure, and the `bSwapBN` automorphism. Paper 26 v3 uses these as given; no re-proof.
- **Dunn 1976** (*Intuitive semantics for first-degree entailments*): first-degree entailment fragment underlying Belnap-Dunn. Paper 26 v3 uses `meet` / `join` / `neg` as defined therein.
- **Font & Rivieccio 2011** (*Logics of Belnap trellises and semilattices*): modern algebraic treatment of Belnap-Dunn; catalog of admissible operators. `bSwapBN` (Aut(Belnap4) = ℤ/2) is standard therein.
- **Priest & Garfield 2003** (*Nāgārjuna and the limits of thought*): pentalemma formalization of Nāgārjuna's catuṣkoṭi + fifth-corner ineffability. Paper 26 v3 does NOT use the fifth-corner; the two systems are formally distinct (Priest-Garfield adds a fifth value; Paper 26 v3 keeps four values + adds an operator).
- **Aczel 1988** (*Non-well-founded sets*): source of the "self-membership" formal apparatus. Paper 26 v3's `E (E x)` interpretation is not a set-membership statement, so Aczel's apparatus is not invoked.
- **Rutten 2000 / Birkedal 2013**: coalgebra + step-indexed guarded recursion. Paper 26 v3's four filter conditions are all first-order finite; no coalgebraic or guarded-recursive machinery is needed. Relevant only if Q1 is reopened to signature (c) higher-order.
- **Ganeri 2002 / Priest 2018 / Tanaka et al. 2013**: Buddhist-logic formalization scholarship. Paper 26 v3's four filter conditions are one specific formal reading; the scholarship offers several others.
- **v0.2 / v0.3 (this Paper 26 lineage)**: See §B.5.

**Judgment**: All five listed prior-art areas are addressed; no "world-first" attribution required or made. The novel Rei-side contribution is the specific 4-filter Lean 4 axiom-free enumeration + `numSurvivors_eq_0` result + `bSwapBN` identification within this fragment.

### B.9 Related Rei stack references

- **STEP 1207**: ZERO axis semantics (source of `∅` collision-avoidance for `∅'` naming).
- **STEP 1215** (`Dfumt8CategoryExperiment`): D-FUMT₈ as category, `and8_associativity_fails_witness` (the non-associativity source referenced in v0.3 §A.3 (5)).
- **STEP 1218 / 1219 / 1220**: successor D-FUMT₈ Lean 4 axiom-free experiments; same axiom base as Paper 26 v3.
- **STEP 1264**: Paper 145 v0.9-c Binary 64-entry refinement (Lean 4 axiom-free pattern precursor).
- **Task 20** (2026-07-24, `Dfumt8SelfReflexivePreservation`): SELF⟲ Verilog encoding Lean 4 (companion to v0.3 §A.3 (6)).
- **Paper 26 v0.2 (Zenodo 21560387)**: D-FUMT₈ ↔ Cl(3,0) bijection + Peace Axiom + SELF reflection (Lean 4 axiom-free 23 theorems).
- **Paper 26 v0.3 (Zenodo 21572459)**: §A.3 non-claim (5) (Belnap-glue non-associativity, orthogonal to Paper 26 v3) + non-claim (6) (Fix(R) deprecation, continued in Paper 26 v3).

### B.10 Deferred to v3.1 or later

Paper 26 v3.0 does NOT include the following, which remain candidate directions:

1. **Modal signature** (Q1 (b) `E : V → 𝓟(V)`): a candidate wider fragment. Its enumeration space is `4^{2^4} = 4^{16} = 4,294,967,296` — no longer feasible for full enumeration, would require symbolic constraint solving.
2. **Higher-order signature** (Q1 (c) `E : (V → V) → (V → V)`): another candidate wider fragment. Cardinality `(4^4)^{4^4} = 256^{256}`, infeasible for enumeration.
3. **Wider carrier** (5-value / 6-value / P({t, f, x}) powerset): explicitly deferred per §A.3 non-claim 5; also the specific direction rejected in v0.1 spec SAC-4 #4.
4. **Fix(∅') and orbit structure**: candidate encodings of 中道 (madhyamāka) and 縁起 (pratītyasamutpāda). Since `numSurvivors = 0`, `Fix(∅')` is only meaningful once condition (iv) is relaxed or the fragment is widened; deferred.
5. **Track 2 filter-level cross-check**: chat-Claude's three-condition definitions vs Rei's four. Pending hint transmission.
6. **Full Part C (7-element OUKC paper structure)**: partial coverage in v3.0; full elements deferred to v3.1 following the Paper 145 v0.3 → v0.9 iteration pattern.

## Part C: Full structure (partial in v3.0)

### C.1 Version history (dual numbering per §Scope-change item 3)

**v0.x lineage** (v2 corrigendum tree, Paper 26 v2 title "D-FUMT₈ × Clifford Cl(3,0)"):
- v0.1 (2026-05-01): initial spec (rejected v1 QMRP re-formulation).
- v0.2 (2026-07-25): §A.2 Lean 4 axiom-free machine-check completed (23 theorems). 11-platform publish (Zenodo 21560387).
- v0.3 (2026-07-26 AM): chat-Claude 5-turn 診断 arc → §A.3 (5)(6) non-claim 追加. Zenodo 21572459 corrigendum (isNewVersionOf 21560387).

**v3.x lineage** (this paper, "Śūnyatā as Operator" pivot):
- v3.0 (2026-07-26 PM): this document. E-operator + Belnap-Dunn FOUR base + 値数 emergent. Cascade `256 → 16 → 4 → 1 → 0`. Main negative result `numSurvivors_eq_0` machine-verified axiom-free. Publish pending 藤本さん再判断 (Q6 (II) protocol).
- v3.1 (planned): Track 2 chat-Claude hint 到着後 filter-level cross-check + step-3 discrepancy root cause identification. Full Part C. Possibly modal signature (Q1 (b)) if computationally feasible.

**Numbering rationale (mandatory scope-change item 3 detail)**: v3 (three-dot integer) rather than v0.4 (v2 corrigendum tree continuation) because Paper 26 v3 is a scope pivot, not a corrigendum. v3 continues the same Paper 26 line (QMRP → v2 D-FUMT₈ × Cl(3,0) → v3 Śūnyatā operator), all under DOI prefix 10.5281/zenodo but with distinct new deposits. Readers of v0.3 corrigendum will find v3 via v3's citation of the v0.3 corrigendum abstract in this section (§C.1 + §Scope-change), preventing orphan reference.

### C.2 References (partial; full BibTeX in v3.1)

- Belnap, N. D. (1977). *A useful four-valued logic*. In J. M. Dunn & G. Epstein (Eds.), *Modern Uses of Multiple-Valued Logic* (pp. 5–37). Reidel.
- Dunn, J. M. (1976). Intuitive semantics for first-degree entailments and "coupled trees". *Philosophical Studies*, 29(3), 149–168.
- Font, J. M., & Rivieccio, U. (2011). Logics of Belnap trellises and semilattices. *Studia Logica*, 98(1), 179–208.
- Priest, G., & Garfield, J. L. (2003). Nāgārjuna and the limits of thought. In G. Priest, *Beyond the Limits of Thought* (2nd ed., ch. 16). Oxford University Press.
- Ganeri, J. (2002). Jaina logic and the philosophical basis of pluralism. *History and Philosophy of Logic*, 23(4), 267–281.
- Tanaka, K., Berto, F., Mares, E., & Paoli, F. (Eds.). (2013). *Paraconsistency: Logic and Applications*. Springer.
- Nāgārjuna. *Mūlamadhyamakakārikā* (MMK). English translation: Garfield, J. L. (1995). *The Fundamental Wisdom of the Middle Way*. Oxford University Press. (Referenced verse: MMK 13:8, 「空を見と成す者は治し難し」.)
- Fujimoto, N., Rei, & Claude. (2026a). Paper 26 v0.2. Zenodo. <https://doi.org/10.5281/zenodo.21560387>
- Fujimoto, N., Rei, & Claude. (2026b). Paper 26 v0.3 corrigendum. Zenodo. <https://doi.org/10.5281/zenodo.21572459>

### C.3 Acknowledgments

- 藤本 伸樹 (Nobuki Fujimoto): 2026-07-26 explicit permission for full-scope use of śūnyatā semantic content; Q1-Q6 six decisions; SAC-4 #4 rejection (v0.1 spec 4-option cascade); SAC-4 #5 rejection (Paper 26 naming pre-grep-verify projection); 2026-07-26 close-of-session Q6 (II) protocol authorization.
- **chat-Claude (Anthropic, claude-opus-4-7 web instance)**: 5-turn eigen-signature 診断 arc origin (Paper 26 v0.3 corrigendum); 建設的代案 for v0.2 spec pivot ("空を値でなく作用素にする"); Track 2 "256案全滅" empirical evidence (filter-level cross-check pending).
- **Rei-AIOS autonomous research substrate**: Lean 4 axiom-free implementation; 4-filter enumeration; `bSwapBN` identification; two-file main + companion architecture; SAC-4 100% acceptance discipline (`feedback-critique-response-pattern`).

---

**End of Paper 26 v3.0 draft.**

**Publish protocol** (藤本さん再判断待ち, per Q6 (II)):
1. Rei prepares 11-platform publish scripts (following Paper 26 v0.3 pattern; Zenodo API User-Agent already fixed in prior arc's prophylactic sweep).
2. Founder confirms publish authorization (spec + impl + measure phases already complete).
3. Zenodo v3.0 DOI issued as NEW deposit (not version of 21572459, following Paper 26 v0.2 / v0.3 precedent of NEW deposits per corrigendum / pivot). Recorded in `data/publications/publish-log-paper026v3-zenodo.json`.
4. 11 platform mirrors (IA + Harvard + Dev.to + HackMD + Hatena + Livedoor + Notion + Mastodon + Nostr + GitHub; Scrapbox chronic skip).
5. `MEMORY.md` + `docs/RECENT_UPDATES.md` + `docs/SITE_COVERAGE_MAP.md` updated with v3.0 DOI + publish log path.
