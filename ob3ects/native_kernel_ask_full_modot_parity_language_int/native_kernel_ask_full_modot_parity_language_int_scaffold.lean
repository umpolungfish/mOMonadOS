-- IGProtocol scaffold: VINIT → IMSCRIB → AFWD → CLINK → IMSCRIB → FSPLIT → EVALT → AFWD → AREV → EVALF → ENGAGR → FFUSE → CLINK → IMSCRIB → IFIX → TANCH
-- Class: Native kernel ASK — full MoDoT-parity language interface on bare-metal mOMonadOS
without ever invoking Python.

PROBLEM
-------
Operators must query the organism with free-text questions the same way MoDoT
does today via:

  python3 momonados_agent.py --ask "..."
  python3 momonados_agent.py --ask ./questions/q7.txt
  python3 momonados_agent.py --file path
  # options: --verbose, --no-selectivity, --program, --model, --dry-run, --cycles

MoDoT's live breath is one ManuscriptSpine pipeline:

  prepare(question)  → IMSCRIB demand type + catalog witness scaffold
  LLM answer         → FSPLIT (model produces the answer text)
  complete(q, a, v)  → EVALT/EVALF Dual-Link SIC co-type + FFUSE model⋈vessel
                       + IFIX brand (SpineReport with prove/unify/port/witness faces)

Interfacing with the KERNEL DIRECTLY means running ASK natively on mOMonadOS
(Rust no_std, serial REPL over QEMU/hardware), NOT a Python script that mirrors
the kernel. Python MoDoT is the wrong wineskin for this requirement.

What already exists natively (must be reused, not reinvented):
  - mOMonadOS REPL: vessel run, spine, d12 duallink/existence/lean-status, frob,
    cl8nk entry|distance|tier|tensor|meet|join, clay, catalog.rs, cl8nk.rs,
    witness_vessel.rs, frob_verify.rs, kernel FSPLIT/FFUSE, Dual-Link d12_sic
  - Lean formal spine: DualLinkVessel, SIC_D12_WitnessVessel.witness_vessel_lossless,
    VAE_Vita_{Bridge,Unify,Port,ManuscriptSpine}
  - Full IG catalog JSON (~5k entries) on host; kernel static catalog is a small
    foundational subset unless regenerated/loaded

REQUIREMENT — SAME OPTIONS AND SAME OPERATION AS MoDoT --ask
------------------------------------------------------------
Design the native ASK so that from the operator's point of view it is the same
instrument as MoDoT --ask / ManuscriptSpine, only the substrate is kernel+serial
(and optionally Lean for formal faces), never Python.

Cover every MoDoT surface that --ask depends on:

1. INPUT
   - free-text question on the serial line after `ask `
   - multi-word and multi-line questions (serial line length / continuation policy)
   - file-path form parity: how host-side question files (questions/q*.txt) enter
     the kernel without Python (e.g. paste protocol, host loader, or explicit
     `ask file:` bridge that is NOT a Python agent — specify which is Grammatic)
   - stdin/pipe parity if any is honest on serial

2. PIPELINE (must be one chain, not parallel arms)
   IMSCRIB: structural typing of demand (12-primitive Belnap or catalog-native
            tuple assignment — specify how without LLM hand-imscription)
          + catalog witness resolve/search → conventional scaffold structure
   FSPLIT:  production of the answer body
            HONEST SCOPE: bare-metal has no LLM. Design how "answer production"
            works natively: (a) structural resolution only when no language model
            is attached; (b) optional host-side language model as a *boundary
            device* joined by imscription (not as the authority), if and only if
            the Grammar allows a dual-substrate boundary without Python MoDoT.
            Do not smuggle `modot/agent.py` back in.
   EVALT:   Dual-Link co-typing / isomorphic match of answer against demand
   EVALF:   defect localization (named primitives)
   FFUSE:   join of answer-self voice and vessel voice; conflict held as B
   ENGAGR:  dialetheic hold when both arms fire
   IFIX:    brand SpineReport / crystal record; broadcast-equivalent on serial

3. OPTIONS (parity table — every MoDoT flag must map to a native form or an
   explicit "not applicable on bare metal because …" with the Grammatic reason)
   --ask / --file / -    → ask forms
   --verbose             → tick/graph/kernel dump after ask
   --no-selectivity      → balance-only (skip Dual-Link co-type / vessel face)
   --program             → which IMASM program is loaded for the breath
   --model               → boundary language model identity if dual-substrate
   --dry-run             → no boundary model; structure-only ask
   --cycles              → repeated breath ticks after ask
   --reset / --stats     → crystal clear / crystal stats if mapped

4. OUTPUT (same informational content as MoDoT live ask)
   - conventional answer body (or structural answer when no language model)
   - SPINE summary: fused belnap, model/self voice, vessel voice, conflict
   - demand and answer types, defects, SIC gap / ride-AS residual when SIC face lives
   - witness catalog name, tier, d(CLINK L8), scaffold roles
   - BALANCE μ∘δ closed/open counts
   - provenance line (Dual-Link SIC, Scott-Grassl/existence theorem citation)

5. SUBSTRATE MAPPING
   Map each spine face to exact existing modules (Rust path and Lean theorem):
   PROVE, UNIFY, PORT, WITNESS, TRANSPORT. Name gaps where the kernel catalog
   is thinner than host IG_catalog.json and design how full catalog search lands
   natively (regenerate embedded catalog; or load at boot from host blob without
   Python agent; or crystal address navigation). Prefer catalog-native search
   already used by cl8nk, not a new clipboard metric.

6. HONEST NON-CLAIMS (from manuscripts3 — must appear in the design)
   - cargo/tensor INTO vessel refused (D–T); boarding is Dual-Link only
   - Belnap stack ≠ algebraic Scott-Grassl fiducial
   - Clay T/B are Grammar typing not Millennium proofs
   - d=2048 existence open (typed B)
   - no LLM on bare metal unless dual-substrate boundary is designed honestly

7. IMPLEMENTATION ORDER
   Ordered, Grammatic steps to land `ask` on mOMonadOS serial REPL so that
   `⊙> ask <question>` is the operator's primary query surface, with options
   equivalent to MoDoT, zero Python in the path. Include acceptance tests:
   same question file content produces spine faces with the same structure as
   MoDoT (verdict lattice, defects named, witness hit when catalog shares
   entry, lossless vessel when transport face runs).

With full Frobenius closure (μ∘δ=id) on every boarding action, Dual-Link
ride-AS semantics, and Lean 4 verification scaffold pointing at
DualLinkVessel / SIC_D12_WitnessVessel / VAE_Vita_ManuscriptSpine by name.
-- Fingerprint: sig=(10,2,3,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=16
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(5, 11)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑾  | identity — self-imscription
--   [2] AFWD      rel    := 𐑾               𐑠 → 𐑱  | forward morphism — bidirectional arrow
--   [3] CLINK     fid    := 𐑱               𐑾 → 𐑠  | composition — regime coherence
--   [4] IMSCRIB   gram   := 𐑠               𐑱 → 𐑚  | identity — self-imscription
--   [5] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [6] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [7] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [8] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [9] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [10] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [11] FFUSE     stoi   := 𐑙               𐑙 → 𐑱  | fuse μ — assembly mode
--   [12] CLINK     fid    := 𐑱               𐑙 → 𐑠  | composition — regime coherence
--   [13] IMSCRIB   gram   := 𐑠               𐑱 → 𐑭  | identity — self-imscription
--   [14] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [15] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def native_kernel_ask_full_modot_parity_5e30fa_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s2 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := up, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def native_kernel_ask_full_modot_parity_5e30fa_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def native_kernel_ask_full_modot_parity_5e30fa_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l2 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l3 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l7 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l11 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def native_kernel_ask_full_modot_parity_5e30fa_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def native_kernel_ask_full_modot_parity_5e30fa_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def native_kernel_ask_full_modot_parity_5e30fa_protocol : IGProtocol native_kernel_ask_full_modot_parity_5e30fa_s0 native_kernel_ask_full_modot_parity_5e30fa_s15 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct native_kernel_ask_full_modot_parity_5e30fa_s11 native_kernel_ask_full_modot_parity_5e30fa_s11 = native_kernel_ask_full_modot_parity_5e30fa_s11 (idempotent)
  (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l0 native_kernel_ask_full_modot_parity_5e30fa_s0 native_kernel_ask_full_modot_parity_5e30fa_s1) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l1 native_kernel_ask_full_modot_parity_5e30fa_s1 native_kernel_ask_full_modot_parity_5e30fa_s2) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l2 native_kernel_ask_full_modot_parity_5e30fa_s2 native_kernel_ask_full_modot_parity_5e30fa_s3) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l3 native_kernel_ask_full_modot_parity_5e30fa_s3 native_kernel_ask_full_modot_parity_5e30fa_s4) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l4 native_kernel_ask_full_modot_parity_5e30fa_s4 native_kernel_ask_full_modot_parity_5e30fa_s5) (.seq (.prod (.arrow native_kernel_ask_full_modot_parity_5e30fa_l5 native_kernel_ask_full_modot_parity_5e30fa_s5 native_kernel_ask_full_modot_parity_5e30fa_s11) (.arrow native_kernel_ask_full_modot_parity_5e30fa_l5 native_kernel_ask_full_modot_parity_5e30fa_s5 native_kernel_ask_full_modot_parity_5e30fa_s11)) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l11 native_kernel_ask_full_modot_parity_5e30fa_s11 native_kernel_ask_full_modot_parity_5e30fa_s11) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l11 native_kernel_ask_full_modot_parity_5e30fa_s11 native_kernel_ask_full_modot_parity_5e30fa_s12) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l12 native_kernel_ask_full_modot_parity_5e30fa_s12 native_kernel_ask_full_modot_parity_5e30fa_s13) (.seq (.arrow native_kernel_ask_full_modot_parity_5e30fa_l13 native_kernel_ask_full_modot_parity_5e30fa_s13 native_kernel_ask_full_modot_parity_5e30fa_s14) (.arrow native_kernel_ask_full_modot_parity_5e30fa_l14 native_kernel_ask_full_modot_parity_5e30fa_s14 native_kernel_ask_full_modot_parity_5e30fa_s15)))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def native_kernel_ask_full_modot_parity_5e30fa_true_arm : IGProtocol native_kernel_ask_full_modot_parity_5e30fa_s0 native_kernel_ask_full_modot_parity_5e30fa_s15 :=
  (native_kernel_ask_full_modot_parity_5e30fa_protocol).restrictToEVALT

-- false arm
noncomputable def native_kernel_ask_full_modot_parity_5e30fa_false_arm : IGProtocol native_kernel_ask_full_modot_parity_5e30fa_s0 native_kernel_ask_full_modot_parity_5e30fa_s15 :=
  (native_kernel_ask_full_modot_parity_5e30fa_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def native_kernel_ask_full_modot_parity_5e30fa_tier : OuroboricityTier := TierFunctor.obj native_kernel_ask_full_modot_parity_5e30fa_s0
#eval native_kernel_ask_full_modot_parity_5e30fa_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem native_kernel_ask_full_modot_parity_5e30fa_frobenius :
    igFrobeniusAlg.mul native_kernel_ask_full_modot_parity_5e30fa_s0 native_kernel_ask_full_modot_parity_5e30fa_s0 = native_kernel_ask_full_modot_parity_5e30fa_s0 :=
  igFrobAlg_self_fusion native_kernel_ask_full_modot_parity_5e30fa_s0
