/-
  CollatzRei.Paper159Inclosure — Two-layer D-FUMT₈ reconstruction of the
  Priest-Garfield Inclosure Schema for Nāgārjuna's catuṣkoṭi.

  Companion to Paper 159 v0.2 (Rei-AIOS).
  Zenodo (v0.1 OUTLINE): DOI 10.5281/zenodo.20468146
  Stance: rational reconstruction following Priest-Garfield (2003)'s own
          self-framing — NOT textual history.

  This file machine-checks (*sans* sorry, decide-only):
    1. DFUMT8.omega_upper is idempotent  (Paper 159 §3.4 main claim)
    2. omega_upper contracts BOTH → ZERO (the 0₀ genesis layer of ZCSG)
    3. omega_upper contracts NEITHER → ZERO
    4. Any Inclosure instance with lower-layer terminus BOTH has upper-layer
       terminus ZERO under omega_upper
    5. FDE → DFUMT8 embedding (φ) preserves negation (Paper 159 §3.3 partial)
       — full conjunction/disjunction preservation deferred per §4.2 honest scope

  Honest scope (★ load-bearing):
    - omega_upper is a NEW operator for the Inclosure upper layer. It is
      structurally idempotent (same algebraic form as STEP 513 omega) but
      semantically distinct: STEP 513 omega contracts INFINITY→BOTH,
      ZERO→NEITHER, FLOWING→TRUE (5-element image); omega_upper contracts
      BOTH→ZERO, NEITHER→ZERO (6-element image). The shared structural
      pattern is the SELF⟲ idempotency axis of D-FUMT₈.
    - The classical-Lean cannot prove Q(delta Q) ∧ ¬ Q(delta Q) because
      it would imply False. We therefore express the limit-contradiction
      as a DFUMT8 VALUE (BOTH) at the lower layer, not as a classical Prop.
      The Inclosure structure here records closure + transcendence as
      formal obligations; the apparent contradiction lives in the DFUMT8
      type, not in Lean's Prop.
    - This file does NOT claim that Nāgārjuna held the corresponding views.
    - This file does NOT claim "world first" — Paper 61 (ZCSG) and Priest-
      Garfield (2003) have already issued related uniqueness claims.
-/

namespace CollatzRei.Paper159Inclosure

/-! ## §2.3 — D-FUMT₈ inductive type (inlined, mirrors Paper 77 LeanDFumt) -/

inductive DFUMT8 : Type where
  | TRUE | FALSE | BOTH | NEITHER
  | INFINITY | ZERO | FLOWING | SELF
  deriving Repr, DecidableEq, Inhabited

/-- Negation (matches Paper 77 LeanDFumt.Basic). The four reflective values
    are self-dual (their meaning is dimensional, not polar). -/
def DFUMT8.neg : DFUMT8 → DFUMT8
  | .TRUE     => .FALSE
  | .FALSE    => .TRUE
  | .BOTH     => .BOTH
  | .NEITHER  => .NEITHER
  | .INFINITY => .INFINITY
  | .ZERO     => .ZERO
  | .FLOWING  => .FLOWING
  | .SELF     => .SELF

/-! ## §3.1 — FDE (Belnap 1977) inductive type -/

inductive FDE : Type where
  | t  -- true (and not false)
  | f  -- false (and not true)
  | b  -- both true and false
  | n  -- neither true nor false
  deriving Repr, DecidableEq, Inhabited

/-- FDE negation. b and n are self-dual under Belnap's negation. -/
def FDE.neg : FDE → FDE
  | .t => .f
  | .f => .t
  | .b => .b
  | .n => .n

/-! ## §3.2 — FDE → DFUMT8 embedding φ -/

def FDE.toDFumt8 : FDE → DFUMT8
  | .t => .TRUE
  | .f => .FALSE
  | .b => .BOTH
  | .n => .NEITHER

/-! ## §3.3 — Embedding preserves negation (partial structural preservation) -/

theorem FDE.toDFumt8_neg :
    ∀ x : FDE, (FDE.neg x).toDFumt8 = DFUMT8.neg x.toDFumt8 := by
  intro x; cases x <;> decide

/-! ## §3.4 — Upper-layer modal operator omega_upper (Paper 159 main differential)

  ★ Honest naming note: this is a NEW operator distinct from STEP 513
  `operator-fixed-point-atlas.ts` `omega` (which contracts INFINITY→BOTH,
  ZERO→NEITHER, FLOWING→TRUE). Both share the structural property
  (Ω∘Ω = Ω) but have semantically different fixed-point sets.

  This operator's intended interpretation: the "all four corners exhausted"
  reading of catuṣpaścāś (四句百非, four corners + 100 negations) — the
  lower-layer four positions {TRUE, FALSE, BOTH, NEITHER} contract via the
  upper layer to either their polar value (TRUE/FALSE) or to ZERO (BOTH and
  NEITHER both collapse to the 0₀ genesis layer of ZCSG, Paper 61). The
  reflective four (INFINITY, ZERO, FLOWING, SELF) are already Ω-stable.
-/
def DFUMT8.omega_upper : DFUMT8 → DFUMT8
  | .TRUE     => .TRUE
  | .FALSE    => .FALSE
  | .BOTH     => .ZERO     -- ★ Priest-Garfield BOTH terminus contracted to 0₀
  | .NEITHER  => .ZERO     -- ★ catuṣkoṭi 4th corner contracted to 0₀
  | .INFINITY => .INFINITY
  | .ZERO     => .ZERO
  | .FLOWING  => .FLOWING
  | .SELF     => .SELF

/-- ★ Main theorem (Paper 159 §3.4): omega_upper is idempotent.
    Ω ∘ Ω = Ω as functions on DFUMT8. -/
theorem omega_upper_idempotent (x : DFUMT8) :
    DFUMT8.omega_upper (DFUMT8.omega_upper x) = DFUMT8.omega_upper x := by
  cases x <;> decide

/-- ★ Contraction theorem 1: BOTH (Priest-Garfield terminus) → ZERO. -/
theorem omega_upper_both_to_zero :
    DFUMT8.omega_upper .BOTH = .ZERO := by decide

/-- ★ Contraction theorem 2: NEITHER (4th corner of catuṣkoṭi) → ZERO. -/
theorem omega_upper_neither_to_zero :
    DFUMT8.omega_upper .NEITHER = .ZERO := by decide

/-- The four reflective values are omega_upper-stable. -/
theorem omega_upper_reflective_stable :
    DFUMT8.omega_upper .INFINITY = .INFINITY ∧
    DFUMT8.omega_upper .ZERO = .ZERO ∧
    DFUMT8.omega_upper .FLOWING = .FLOWING ∧
    DFUMT8.omega_upper .SELF = .SELF := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> decide

/-- The two polar values are omega_upper-stable. -/
theorem omega_upper_polar_stable :
    DFUMT8.omega_upper .TRUE = .TRUE ∧
    DFUMT8.omega_upper .FALSE = .FALSE := by
  refine ⟨?_, ?_⟩ <;> decide

/-- omega_upper has exactly 6 fixed points (the image set is
    {TRUE, FALSE, ZERO, INFINITY, FLOWING, SELF}). -/
theorem omega_upper_fixed_point_count :
    let isFixed (x : DFUMT8) := decide (DFUMT8.omega_upper x = x)
    isFixed .TRUE = true ∧ isFixed .FALSE = true ∧
    isFixed .BOTH = false ∧ isFixed .NEITHER = false ∧
    isFixed .INFINITY = true ∧ isFixed .ZERO = true ∧
    isFixed .FLOWING = true ∧ isFixed .SELF = true := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩ <;> decide

/-! ## §3.5 — Inclosure schema (Priest 1987, 2002)

  Classical Lean cannot prove `Q (delta Q) ∧ ¬ Q (delta Q)` (it would
  imply False). We therefore record the schema as a structure carrying
  the closure + transcendence obligations, and express the limit value
  in the DFUMT8 TYPE (as BOTH at the lower layer) rather than as a
  classical Prop. The "apparent contradiction" lives in DFUMT8, not Prop.
-/

structure Inclosure (α : Type) where
  Q : α → Prop
  psi : (α → Prop) → Prop
  delta : (α → Prop) → α
  closure : ∀ X : α → Prop,
    (∀ x, X x → Q x) → psi X → Q (delta X)
  transcendence : ∀ X : α → Prop,
    (∀ x, X x → Q x) → psi X → ¬ X (delta X)

/-! ## §3.6 / §3.7 — Lower-layer and upper-layer assignments

  Following Priest-Garfield (2003: 17-18), the lower-layer assignment of
  δ(Ω) at the limit is BOTH for both Nāgārjuna's Paradox (ontological)
  and the Expressibility Paradox. This is a Rei-AIOS rational reconstruction
  in the DFUMT8 type system — we do not assert that Nāgārjuna held this
  view explicitly. -/

/-- Lower-layer terminus value for any Inclosure instance, following
    Priest-Garfield's BOTH assignment to δ(Ω). -/
def Inclosure.limitLowerLayer {α : Type} (_I : Inclosure α) : DFUMT8 := .BOTH

/-- Upper-layer terminus is the omega_upper image of the lower layer. -/
def Inclosure.limitUpperLayer {α : Type} (I : Inclosure α) : DFUMT8 :=
  DFUMT8.omega_upper I.limitLowerLayer

/-! ## §3.8 — Main differential theorem (★ load-bearing)

  ★ Paper 159's central formal differential against Priest-Garfield (2003):
  the upper-layer Ω contracts the lower-layer BOTH to ZERO (the ZCSG 0₀
  genesis layer, Paper 61). This holds for ANY Inclosure instance whose
  lower-layer assignment is BOTH — including Nāgārjuna's Paradox and the
  Expressibility Paradox. The Priest-Garfield BOTH terminus is preserved
  at the object layer and contracted at the modal layer. -/

theorem Inclosure.limit_upper_is_zero {α : Type} (I : Inclosure α) :
    I.limitUpperLayer = .ZERO := by
  unfold Inclosure.limitUpperLayer Inclosure.limitLowerLayer
  decide

/-- Idempotent stability at the upper layer: re-applying Ω to the upper
    limit yields the same upper limit. -/
theorem Inclosure.limit_upper_idempotent {α : Type} (I : Inclosure α) :
    DFUMT8.omega_upper I.limitUpperLayer = I.limitUpperLayer := by
  rw [Inclosure.limit_upper_is_zero]
  decide

/-! ## §3.8 (continued) — Direct examples / sanity checks (decide) -/

example : DFUMT8.omega_upper .BOTH = .ZERO := by decide
example : DFUMT8.omega_upper (DFUMT8.omega_upper .BOTH) = .ZERO := by decide
example : DFUMT8.omega_upper .NEITHER = .ZERO := by decide
example : DFUMT8.omega_upper (DFUMT8.omega_upper .NEITHER) = .ZERO := by decide

/-! ## §3.9 — Compatibility with multiple interpretive stances (Paper 159 §4.3)

  Honest stance: the formal substrate above is compatible with at least
  three readings of catuṣkoṭi:

    (P-G)   Dialetheist (Priest-Garfield 2003 / Deguchi-Garfield-Priest):
            use only the LOWER layer; δ(Ω) = BOTH is endorsed.
    (T)     Weak dialetheist (Tillemans 2009, 2024):
            use the upper layer to contract BOTH to ZERO, so conjoined
            contradictions φ ∧ ¬φ are NOT endorsed at the modal layer.
    (¬D)    Non-dialetheist:
            assign δ(Ω) = NEITHER at the lower layer instead of BOTH; the
            upper layer still contracts NEITHER to ZERO.

  None of these is endorsed here; the formal substrate accepts each
  by varying the lower-layer assignment function. -/

/-- (P-G) reading: use lower-layer BOTH directly. -/
example : (.BOTH : DFUMT8) = .BOTH := rfl

/-- (T) reading: contract via upper layer. BOTH → ZERO; no conjoined
    contradiction at the modal layer. -/
example : DFUMT8.omega_upper .BOTH = .ZERO := by decide

/-- (¬D) reading: assign NEITHER at the lower layer. The upper layer
    still maps to ZERO. -/
example : DFUMT8.omega_upper .NEITHER = .ZERO := by decide

end CollatzRei.Paper159Inclosure
